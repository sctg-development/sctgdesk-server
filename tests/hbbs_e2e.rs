//! End-to-end checks of the `hbbs` binary: the server is started for real and spoken to the way
//! the official client does (docs/testing.md).
//!
//! They guard against regressions that unit tests cannot see, all met in production:
//! - one request with a malformed bearer token used to abort the whole process,
//! - `--logged-in-only` left the client waiting forever when the API server was unreachable,
//! - the key exchange must stay compatible with the official client (docs/key-exchange.md).
//!
//! hbbs listens on fixed ports (21115, 21116, 21118), so the tests run one after the other and
//! need those ports to be free: stop a local hbbs before running them.
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use hbb_common::{
    bytes::Bytes,
    config::{CONNECT_TIMEOUT, READ_TIMEOUT},
    protobuf::Message,
    rendezvous_proto::*,
    socket_client,
    sodiumoxide::crypto::{box_, secretbox, sign},
    tcp::{kx_version_for, KxTranscript, KX_PARAMS_DOMAIN},
    timeout, ResultType, Stream,
};

const RENDEZVOUS_PORT: u16 = 21116;
const STARTUP_TIMEOUT: Duration = Duration::from_secs(90);

/// The ports are fixed: only one server at a time.
static ONE_SERVER_AT_A_TIME: Mutex<()> = Mutex::new(());

struct Server {
    child: Child,
    dir: PathBuf,
    api_port: u16,
}

impl Server {
    /// Starts hbbs in an empty directory (it creates its key pair and database there).
    fn start(extra_args: &[&str], envs: &[(&str, &str)]) -> Server {
        for port in [21115, 21116, 21118] {
            assert!(
                TcpListener::bind(("0.0.0.0", port)).is_ok(),
                "port {port} is in use: stop the local hbbs before running these tests"
            );
        }
        let api_port = TcpListener::bind(("127.0.0.1", 0))
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let dir =
            std::env::temp_dir().join(format!("hbbs-e2e-{}-{}", std::process::id(), api_port));
        std::fs::create_dir_all(&dir).unwrap();
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_hbbs"));
        cmd.arg(format!("--api-port={api_port}"))
            .args(extra_args)
            .current_dir(&dir)
            .env_remove("API_SERVER")
            .env_remove("DB_URL")
            .stdout(std::fs::File::create(dir.join("hbbs.log")).unwrap())
            .stderr(Stdio::null());
        for (k, v) in envs {
            cmd.env(k, v);
        }
        let child = cmd.spawn().expect("cannot start hbbs");
        let mut server = Server {
            child,
            dir,
            api_port,
        };
        server.wait_until_ready();
        server
    }

    fn wait_until_ready(&mut self) {
        let start = Instant::now();
        let ready = |p: u16| TcpStream::connect(("127.0.0.1", p)).is_ok();
        while !(ready(RENDEZVOUS_PORT)
            && ready(self.api_port)
            && self.dir.join("id_ed25519.pub").exists())
        {
            assert!(
                self.child.try_wait().unwrap().is_none(),
                "hbbs exited while starting"
            );
            assert!(
                start.elapsed() < STARTUP_TIMEOUT,
                "hbbs did not start in time"
            );
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    /// The lines hbbs logged with the message `what`, waiting up to 5 s for `n` of them (the logger is asynchronous).
    fn log_lines(&self, what: &str, n: usize) -> Vec<String> {
        let start = Instant::now();
        loop {
            let lines: Vec<String> = std::fs::read_to_string(self.dir.join("hbbs.log"))
                .unwrap_or_default()
                .lines()
                .filter(|l| l.contains(what))
                .map(str::to_owned)
                .collect();
            if lines.len() >= n || start.elapsed() > Duration::from_secs(5) {
                return lines;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// The public key a client has to be configured with.
    fn public_key(&self) -> String {
        std::fs::read_to_string(self.dir.join("id_ed25519.pub"))
            .unwrap()
            .trim()
            .to_owned()
    }

    fn is_running(&mut self) -> bool {
        self.child.try_wait().unwrap().is_none()
    }

    /// One HTTP request to the API: (status, body).
    fn http(&self, method: &str, path: &str, bearer: Option<&str>, body: &str) -> (u16, String) {
        let mut stream = TcpStream::connect(("127.0.0.1", self.api_port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(10)))
            .unwrap();
        let auth = bearer
            .map(|b| format!("Authorization: Bearer {b}\r\n"))
            .unwrap_or_default();
        write!(
            stream,
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\n{auth}Content-Type: application/json\r\n\
             Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
        let mut answer = Vec::new();
        let _ = stream.read_to_end(&mut answer);
        let text = String::from_utf8_lossy(&answer).into_owned();
        let status = text
            .split_whitespace()
            .nth(1)
            .and_then(|code| code.parse().ok())
            .unwrap_or_else(|| panic!("no HTTP answer for {method} {path}"));
        let body = text.split("\r\n\r\n").nth(1).unwrap_or_default().to_owned();
        (status, body)
    }

    /// GET with the given bearer token, returns the HTTP status and the raw body (binary).
    fn get_bytes(&self, path: &str, bearer: Option<&str>) -> (u16, Vec<u8>) {
        let mut stream = TcpStream::connect(("127.0.0.1", self.api_port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(20)))
            .unwrap();
        let auth = bearer
            .map(|b| format!("Authorization: Bearer {b}\r\n"))
            .unwrap_or_default();
        write!(
            stream,
            "GET {path} HTTP/1.1\r\nHost: localhost\r\n{auth}Connection: close\r\n\r\n"
        )
        .unwrap();
        let mut answer = Vec::new();
        let _ = stream.read_to_end(&mut answer);
        let split = answer
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .unwrap_or_else(|| panic!("no HTTP answer for GET {path}"));
        let head = String::from_utf8_lossy(&answer[..split]).into_owned();
        let status = head
            .split_whitespace()
            .nth(1)
            .and_then(|code| code.parse().ok())
            .unwrap();
        (status, answer[split + 4..].to_vec())
    }

    /// POST to the API with the given bearer token, returns the HTTP status.
    fn post_with_bearer(&self, path: &str, bearer: &str) -> u16 {
        self.http("POST", path, Some(bearer), r#"{"id":"E2E","uuid":"ZTJl"}"#)
            .0
    }

    /// Stops hbbs and starts it again on the same directory (same database and key pair).
    fn restart(&mut self, extra_args: &[&str]) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.child = Command::new(env!("CARGO_BIN_EXE_hbbs"))
            .arg(format!("--api-port={}", self.api_port))
            .args(extra_args)
            .current_dir(&self.dir)
            .env_remove("API_SERVER")
            .env_remove("DB_URL")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("cannot restart hbbs");
        self.wait_until_ready();
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

// ---- the key exchange of the official client (rustdesk src/common.rs), kept as it is there ----

fn get_pk(pk: &[u8]) -> Option<[u8; 32]> {
    (pk.len() == 32).then(|| {
        let mut t = [0u8; 32];
        t.copy_from_slice(pk);
        t
    })
}

fn create_symmetric_key_msg(their_pk_b: [u8; 32]) -> (Bytes, Bytes, secretbox::Key) {
    let their_pk_b = box_::PublicKey(their_pk_b);
    let (our_pk_b, out_sk_b) = box_::gen_keypair();
    let key = secretbox::gen_key();
    let nonce = box_::Nonce([0u8; box_::NONCEBYTES]);
    let sealed_key = box_::seal(&key.0, &nonce, &their_pk_b, &out_sk_b);
    (Vec::from(our_pk_b.0).into(), sealed_key.into(), key)
}

/// Connects to the rendezvous server and performs the key exchange as the client does.
async fn connect_and_exchange_keys(public_key: &str) -> ResultType<Stream> {
    use base64::Engine;
    let rs_pk = base64::engine::general_purpose::STANDARD
        .decode(public_key)
        .ok()
        .and_then(|pk| get_pk(&pk))
        .map(sign::PublicKey)
        .ok_or_else(|| hbb_common::anyhow::anyhow!("invalid public key"))?;
    let mut conn =
        socket_client::connect_tcp(format!("127.0.0.1:{RENDEZVOUS_PORT}"), CONNECT_TIMEOUT).await?;
    let first = timeout(READ_TIMEOUT, conn.next())
        .await?
        .ok_or_else(|| hbb_common::anyhow::anyhow!("no first message"))??;
    let Some(rendezvous_message::Union::KeyExchange(ex)) =
        RendezvousMessage::parse_from_bytes(&first)?.union
    else {
        hbb_common::bail!("the first message is not a KeyExchange");
    };
    if ex.keys.len() != 1 {
        hbb_common::bail!("invalid key exchange message");
    }
    let their_pk_b = sign::verify(&ex.keys[0], &rs_pk)
        .map_err(|_| hbb_common::anyhow::anyhow!("signature mismatch in key exchange"))?;
    let their_pk_b = get_pk(&their_pk_b)
        .ok_or_else(|| hbb_common::anyhow::anyhow!("wrong public key length"))?;
    // bit 255 set: the server requires the signed parameters (version 1)
    if their_pk_b[31] & 0x80 != 0 || !ex.signed_params.is_empty() {
        let params = sign::verify(&ex.signed_params, &rs_pk)
            .ok()
            .and_then(|signed| {
                let params = signed.strip_prefix(KX_PARAMS_DOMAIN)?;
                KxParams::parse_from_bytes(params).ok()
            })
            .ok_or_else(|| hbb_common::anyhow::anyhow!("missing or invalid signed parameters"))?;
        assert!(
            params.pk[..] == their_pk_b[..] && params.version == ex.version,
            "key exchange version or public key does not match its signature"
        );
    }
    let (asymmetric_value, symmetric_value, key) = create_symmetric_key_msg(their_pk_b);
    let picked = kx_version_for(ex.version);
    let mut out = RendezvousMessage::new();
    out.set_key_exchange(KeyExchange {
        keys: vec![asymmetric_value.clone(), symmetric_value],
        version: picked,
        ..Default::default()
    });
    timeout(CONNECT_TIMEOUT, conn.send(&out)).await??;
    conn.set_negotiated_key(
        key,
        true,
        &KxTranscript {
            initiator_pk: &asymmetric_value,
            responder_pk: &their_pk_b,
            advertised: ex.version,
            picked,
        },
    )?;
    Ok(conn)
}

/// Sends a punch hole request with `token` and returns its answer within 15 s.
async fn punch_hole(public_key: &str, token: &str) -> PunchHoleResponse {
    let mut conn = connect_and_exchange_keys(public_key).await.unwrap();
    let mut msg = RendezvousMessage::new();
    msg.set_punch_hole_request(PunchHoleRequest {
        id: "123456789".into(),
        token: token.into(),
        licence_key: public_key.into(),
        ..Default::default()
    });
    conn.send(&msg).await.unwrap();
    let answer = conn
        .next_timeout(15_000)
        .await
        .expect("no answer to the punch hole request within 15 s: the client would wait forever")
        .unwrap();
    match RendezvousMessage::parse_from_bytes(&answer).unwrap().union {
        Some(rendezvous_message::Union::PunchHoleResponse(r)) => r,
        other => panic!("expected a PunchHoleResponse, got {other:?}"),
    }
}

// ---- the tests ----

#[hbb_common::tokio::test(crate = "hbb_common::tokio")]
async fn key_exchange_works_with_the_official_client_logic() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&[], &[]);
    let mut conn = connect_and_exchange_keys(&server.public_key())
        .await
        .unwrap();
    // the server must read what the client encrypts and answer in kind
    let mut msg = RendezvousMessage::new();
    msg.set_test_nat_request(TestNatRequest::default());
    conn.send(&msg).await.unwrap();
    let answer = conn.next_timeout(5_000).await.expect("no answer").unwrap();
    assert!(matches!(
        RendezvousMessage::parse_from_bytes(&answer).unwrap().union,
        Some(rendezvous_message::Union::TestNatResponse(_))
    ));
}

#[test]
fn a_malformed_bearer_token_does_not_stop_the_server() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let mut server = Server::start(&[], &[]);
    // the API server runs inside hbbs: a panic there used to abort the rendezvous server too
    for token in ["x", "!!!!", "AAAA", &"A".repeat(200)] {
        let status = server.post_with_bearer("/api/currentUser", token);
        assert_eq!(status, 401, "bearer {token:?}");
        assert!(server.is_running(), "hbbs died after bearer {token:?}");
    }
    assert!(TcpStream::connect(("127.0.0.1", RENDEZVOUS_PORT)).is_ok());
}

#[hbb_common::tokio::test(crate = "hbb_common::tokio")]
async fn logged_in_only_answers_when_the_api_server_is_unreachable() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(
        &["--logged-in-only"],
        &[("API_SERVER", "http://127.0.0.1:1")],
    );
    let answer = punch_hole(&server.public_key(), "abc").await;
    assert!(
        answer.other_failure.to_lowercase().contains("unreachable"),
        "unexpected answer: {answer:?}"
    );
}

#[hbb_common::tokio::test(crate = "hbb_common::tokio")]
async fn logged_in_only_checks_tokens_on_the_configured_api_port() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    // no API_SERVER: it must follow --api-port, not the default 21114
    let server = Server::start(&["--logged-in-only"], &[]);
    let answer = punch_hole(&server.public_key(), "abc").await;
    assert!(
        answer
            .other_failure
            .to_lowercase()
            .contains("session expired"),
        "unexpected answer: {answer:?}"
    );
}

// ---- the history of the connections, with the protocol of the RustDesk clients (issue #29) ----

fn audit_json(body: &str) -> serde_json::Value {
    serde_json::from_str(body).unwrap_or_else(|e| panic!("not JSON ({e}): {body}"))
}

fn audit_login(server: &Server, name: &str, password: &str) -> String {
    let body =
        format!(r#"{{"username":"{name}","password":"{password}","id":"CTRL","uuid":"ZTJl"}}"#);
    let (status, answer) = server.http("POST", "/api/login", None, &body);
    assert_eq!(status, 200, "login of {name}: {answer}");
    audit_json(&answer)["access_token"]
        .as_str()
        .unwrap()
        .to_owned()
}

/// The device posts the connection, the controlling client finds the record, leaves a note, and the administrator
/// and the user find it in the history.
#[hbb_common::tokio::test(crate = "hbb_common::tokio")]
async fn a_connection_is_recorded_with_its_note() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&[], &[]);
    let admin = audit_login(&server, "admin", "Hello,world!");
    for user in ["alice", "bob"] {
        let body = format!(
            r#"{{"name":"{user}","password":"pw-{user}-123","confirm-password":"pw-{user}-123","email":"{user}@x.org","is_admin":false,"group_name":"Default"}}"#
        );
        assert_eq!(server.http("POST", "/api/user", Some(&admin), &body).0, 200);
    }
    let alice = audit_login(&server, "alice", "pw-alice-123");
    let bob = audit_login(&server, "bob", "pw-bob-123");
    // a device registered with the server (a real one registers over the rendezvous protocol)
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite://{}/db_v2.sqlite3", server.dir.display()))
        .await
        .expect("the database of hbbs");
    sqlx::query(
        "INSERT INTO peer (guid, id, uuid, pk, info) VALUES (randomblob(16), 'DEV1', randomblob(16), randomblob(32), '{\"hostname\":\"print-server\"}')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let post = |body: &str| server.http("POST", "/api/audit/conn", None, body);
    let history = |token: &str, query: &str| {
        let (status, body) = server.http(
            "GET",
            &format!("/api/audit/conn?current=1&pageSize=50{query}"),
            Some(token),
            "",
        );
        assert_eq!(status, 200, "{body}");
        audit_json(&body)
    };
    // the session id of a client is a u64
    let session = u64::MAX;

    // what the controlled device sends: the connection, then who controls, then the end
    assert_eq!(post(&format!(r#"{{"id":"DEV1","uuid":"dXVpZA==","conn_id":1,"session_id":{session},"nonce":"n1","action":"new","ip":"10.0.0.5"}}"#)).0, 200);
    // a retried post is applied once
    assert_eq!(post(&format!(r#"{{"id":"DEV1","uuid":"dXVpZA==","conn_id":1,"session_id":{session},"nonce":"n1","action":"new","ip":"10.0.0.5"}}"#)).0, 200);
    assert_eq!(post(&format!(r#"{{"id":"DEV1","uuid":"dXVpZA==","conn_id":1,"session_id":{session},"nonce":"n2","peer":["CTRL","Alice PC"],"type":0}}"#)).0, 200);
    // an unknown device, and a device that sends nothing useful, leave no trace
    assert_eq!(
        post(&format!(
            r#"{{"id":"NOBODY","conn_id":1,"session_id":{session},"nonce":"n3","action":"new"}}"#
        ))
        .0,
        200
    );
    assert_eq!(post("{}").0, 200);
    // what is recorded is logged at info, once: not the retry, not the unknown device
    let logged = server.log_lines("audit conn: device", 2);
    assert_eq!(logged.len(), 2, "{logged:?}");
    assert!(
        logged[0].contains(r#"device "DEV1""#)
            && logged[0].contains(r#"action Some("new")"#)
            && logged[0].contains(r#"ip Some("10.0.0.5")"#),
        "{logged:?}"
    );
    assert!(
        logged[1].contains(r#"peer Some(["CTRL", "Alice PC"])"#),
        "{logged:?}"
    );
    let all = history(&admin, "");
    assert_eq!(all["total"], 1, "{all}");
    let record = &all["data"][0];
    assert_eq!(
        (record["device_id"].as_str(), record["device_name"].as_str()),
        (Some("DEV1"), Some("print-server"))
    );
    assert_eq!(
        (
            record["controller_id"].as_str(),
            record["controller_name"].as_str(),
            record["ip"].as_str()
        ),
        (Some("CTRL"), Some("Alice PC"), Some("10.0.0.5"))
    );
    assert!(
        record["ended_at"].is_null() && record["user"].is_null(),
        "{record}"
    );

    // the controlling client asks for the guid of the session (with its token), then leaves a note at the end
    let path = format!("/api/audit/conn/active?id=DEV1&session_id={session}&conn_type=0");
    assert_eq!(
        server.http("GET", &path, None, "").0,
        401,
        "a token is needed"
    );
    let (status, answer) = server.http("GET", &path, Some(&alice), "");
    assert_eq!(status, 200, "{answer}");
    let guid = audit_json(&answer)
        .as_str()
        .expect("the guid as a JSON string")
        .to_owned();
    assert_eq!(
        audit_json(
            &server
                .http(
                    "GET",
                    &format!("/api/audit/conn/active?id=DEV1&session_id=1&conn_type=0"),
                    Some(&alice),
                    ""
                )
                .1
        ),
        serde_json::Value::Null
    );
    // it is alice's now: nobody else gets it, nor sets its note
    assert_eq!(
        audit_json(&server.http("GET", &path, Some(&bob), "").1),
        serde_json::Value::Null
    );
    let note = |token: &str, text: &str| {
        server
            .http(
                "PUT",
                "/api/audit",
                Some(token),
                &format!(r#"{{"guid":"{guid}","note":"{text}"}}"#),
            )
            .0
    };
    assert_eq!(note(&bob, "not mine"), 403);
    assert_eq!(note(&alice, "replaced the toner, 20 minutes"), 200);
    assert_eq!(
        history(&admin, "")["data"][0]["note"],
        "replaced the toner, 20 minutes"
    );
    assert_eq!(history(&admin, "")["data"][0]["user"], "alice");

    // the end of the connection
    assert_eq!(
        post(&format!(
            r#"{{"id":"DEV1","conn_id":1,"session_id":{session},"nonce":"n4","action":"close"}}"#
        ))
        .0,
        200
    );
    assert!(!history(&admin, "")["data"][0]["ended_at"].is_null());

    // search, and who sees what
    assert_eq!(history(&admin, "&q=toner")["total"], 1);
    assert_eq!(
        history(&admin, "&q=print-server")["total"],
        1,
        "the name of the device"
    );
    assert_eq!(history(&admin, "&q=alice")["total"], 1, "the account");
    assert_eq!(history(&admin, "&q=nothing-like-this")["total"], 0);
    assert_eq!(history(&admin, "&from=2999-01-01")["total"], 0);
    assert_eq!(
        history(&admin, "&from=2000-01-01&to=2999-01-01")["total"],
        1
    );
    assert_eq!(history(&alice, "")["total"], 1, "alice sees her connection");
    assert_eq!(history(&bob, "")["total"], 0, "bob does not see it");
    // an administrator may set any note; the posted one of the controlling client (with the session) also works
    assert_eq!(note(&admin, "checked"), 200);
    assert_eq!(
        post(&format!(
            r#"{{"id":"DEV1","session_id":{session},"note":"from the client"}}"#
        ))
        .0,
        200
    );
    assert_eq!(history(&admin, "")["data"][0]["note"], "from the client");
}

/// A login of the API server still works after hbbs restarted, until the user logs out.
#[test]
fn a_session_survives_a_restart() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let mut server = Server::start(&[], &[]);
    // the first start creates the admin account (default password "Hello,world!")
    let login = r#"{"username":"admin","password":"Hello,world!","id":"E2E","uuid":"ZTJl"}"#;
    let (status, body) = server.http("POST", "/api/login", None, login);
    assert_eq!(status, 200, "login: {body}");
    let token = body
        .split("\"access_token\":\"")
        .nth(1)
        .and_then(|t| t.split('"').next())
        .expect("an access token")
        .to_owned();
    assert_eq!(server.post_with_bearer("/api/currentUser", &token), 200);
    // the client treats any "error" key, even false, as a failure
    let (_, user) = server.http(
        "POST",
        "/api/currentUser",
        Some(&token),
        r#"{"id":"E2E","uuid":"ZTJl"}"#,
    );
    assert!(
        user.contains("\"name\"") && !user.contains("\"error\""),
        "currentUser body: {user}"
    );

    server.restart(&[]);
    assert_eq!(
        server.post_with_bearer("/api/currentUser", &token),
        200,
        "the session was lost with the restart"
    );
    // the page of device groups the client asks for after login
    let (status, body) = server.http(
        "GET",
        "/api/device-group/accessible?current=1&pageSize=100",
        Some(&token),
        "",
    );
    assert_eq!(status, 200, "device groups: {body}");
    assert!(
        body.contains("\"total\"") && body.contains("\"data\""),
        "{body}"
    );

    let (status, _) = server.http(
        "POST",
        "/api/logout",
        Some(&token),
        r#"{"id":"E2E","uuid":"ZTJl"}"#,
    );
    assert_eq!(status, 200);
    server.restart(&[]);
    assert_eq!(
        server.post_with_bearer("/api/currentUser", &token),
        401,
        "a logged out session came back"
    );
}

// ---- access control of the API (issues #18, #21, #27) ----

fn json(body: &str) -> serde_json::Value {
    serde_json::from_str(body).unwrap_or_else(|e| panic!("not JSON ({e}): {body}"))
}

/// Logs in on the device `device` and returns the access token.
fn login(server: &Server, name: &str, password: &str, device: &str) -> String {
    let body =
        format!(r#"{{"username":"{name}","password":"{password}","id":"{device}","uuid":"ZTJl"}}"#);
    let (status, answer) = server.http("POST", "/api/login", None, &body);
    assert_eq!(status, 200, "login of {name}: {answer}");
    json(&answer)["access_token"].as_str().unwrap().to_owned()
}

fn names(value: &serde_json::Value, key: &str) -> Vec<String> {
    let mut names: Vec<String> = value["data"]
        .as_array()
        .unwrap_or_else(|| panic!("no data in {value}"))
        .iter()
        .map(|item| item[key].as_str().unwrap_or_default().to_owned())
        .collect();
    names.sort();
    names
}

/// Before the fix every logged-in user could read and change any address book, saw all the users, the groups and the
/// devices, was shown full rights on a read-only address book, and could make itself an administrator.
#[hbb_common::tokio::test(crate = "hbb_common::tokio")]
async fn a_user_only_gets_what_its_group_gives_access_to() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&[], &[]);
    let admin = login(&server, "admin", "Hello,world!", "ADMIN");
    let post = |path: &str, token: &str, body: &str| server.http("POST", path, Some(token), body);
    let get = |path: &str, token: &str| server.http("GET", path, Some(token), "");

    // three groups; A may connect to B, nobody to C
    for group in ["grpA", "grpB", "grpC"] {
        let body = format!(
            r#"{{"name":"{group}","note":"","allowed_outgoings":[],"allowed_incomings":[]}}"#
        );
        assert_eq!(post("/api/group", &admin, &body).0, 200);
    }
    let (_, groups) = get("/api/groups?current=1&pageSize=100", &admin);
    let guid_of = |name: &str| -> String {
        json(&groups)["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|g| g["name"] == name)
            .unwrap_or_else(|| panic!("no group {name}"))["guid"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    let (guid_a, guid_b) = (guid_of("grpA"), guid_of("grpB"));
    let update = format!(
        r#"{{"guid":"{guid_a}","name":"grpA","note":"","allowed_outgoings":["{guid_b}"],"allowed_incomings":[]}}"#
    );
    let (status, answer) = server.http("PUT", "/api/group", Some(&admin), &update);
    assert_eq!(status, 200, "{answer}");
    // the access is stored and given back, from both sides
    let (_, group_a) = get(&format!("/api/group/{guid_a}"), &admin);
    assert_eq!(
        json(&group_a)["access_to"],
        json(&format!(r#"["{guid_b}"]"#)),
        "{group_a}"
    );
    let (_, group_b) = get(&format!("/api/group/{guid_b}"), &admin);
    assert_eq!(
        json(&group_b)["accessed_from"],
        json(&format!(r#"["{guid_a}"]"#)),
        "{group_b}"
    );

    for (user, group) in [("alice", "grpA"), ("bob", "grpB"), ("carol", "grpC")] {
        let body = format!(
            r#"{{"name":"{user}","password":"pw-{user}-123","confirm-password":"pw-{user}-123","email":"{user}@x.org","is_admin":false,"group_name":"{group}"}}"#
        );
        assert_eq!(post("/api/user", &admin, &body).0, 200);
    }

    // devices: one per user, registered by hand (a real one registers over the rendezvous protocol)
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite://{}/db_v2.sqlite3", server.dir.display()))
        .await
        .expect("the database of hbbs");
    for peer in ["PEER_A", "PEER_B", "PEER_C", "PEER_X"] {
        sqlx::query(
            "INSERT INTO peer (guid, id, uuid, pk, info) VALUES (randomblob(16), ?, randomblob(16), randomblob(32), '{}')",
        )
        .bind(peer)
        .execute(&pool)
        .await
        .unwrap();
    }
    // logging in on a device makes it the user's (PEER_X is nobody's)
    let alice = login(&server, "alice", "pw-alice-123", "PEER_A");
    let bob = login(&server, "bob", "pw-bob-123", "PEER_B");
    let carol = login(&server, "carol", "pw-carol-123", "PEER_C");

    // #27: users, groups and devices
    let (_, users) = get("/api/users?current=1&pageSize=50", &alice);
    assert_eq!(
        names(&json(&users), "name"),
        ["alice", "bob"],
        "alice sees her group and the one she may reach: {users}"
    );
    let (_, users) = get("/api/users?current=1&pageSize=50", &bob);
    assert_eq!(names(&json(&users), "name"), ["bob"], "{users}");
    let (_, users) = get("/api/users?current=1&pageSize=50", &carol);
    assert_eq!(names(&json(&users), "name"), ["carol"], "{users}");
    let (_, users) = get("/api/users?current=1&pageSize=50", &admin);
    assert_eq!(
        names(&json(&users), "name").len(),
        4,
        "the administrator sees everybody: {users}"
    );

    let groups_of = |token: &str| {
        let (status, body) = get("/api/device-group/accessible?current=1&pageSize=100", token);
        assert_eq!(status, 200, "{body}");
        let value = json(&body);
        assert_eq!(
            value["total"],
            value["data"].as_array().unwrap().len(),
            "{body}"
        );
        names(&value, "name")
    };
    assert_eq!(groups_of(&alice), ["grpA", "grpB"]);
    assert_eq!(groups_of(&bob), ["grpB"]);
    assert_eq!(groups_of(&admin), ["Default", "grpA", "grpB", "grpC"]);

    let peers_of = |token: &str| {
        let (status, body) = get("/api/peers", token);
        assert_eq!(status, 200, "{body}");
        names(&json(&body), "id")
    };
    assert_eq!(peers_of(&alice), ["PEER_A", "PEER_B"]);
    assert_eq!(peers_of(&bob), ["PEER_B"]);
    assert_eq!(peers_of(&carol), ["PEER_C"]);
    assert_eq!(peers_of(&admin), ["PEER_A", "PEER_B", "PEER_C", "PEER_X"]);
    // statistics are for the administrator
    assert_ne!(get("/api/peers/count/all", &alice).0, 200);
    assert_ne!(get("/api/peers/cpus", &alice).0, 200);
    assert_eq!(get("/api/peers/count/all", &admin).0, 200);

    // an address book shared with grpA, read only
    let (_, book) = post(
        "/api/ab/shared/add",
        &admin,
        r#"{"name":"secret","note":null}"#,
    );
    let ab = json(&book)["data"][0]["guid"].as_str().unwrap().to_owned();
    let rule = |rule: u32| format!(r#"{{"guid":"{ab}","group":"{guid_a}","rule":{rule}}}"#);
    assert_eq!(post("/api/ab/rule", &admin, &rule(1)).0, 200);
    let peer = r#"{"id":"123456789","alias":"CONFIDENTIAL","username":"root","password":"s3cret"}"#;
    assert_eq!(post(&format!("/api/ab/peer/add/{ab}"), &admin, peer).0, 200);

    // #18: the right the client is told is the right of the rule, and it is enforced
    let rule_of = |token: &str| {
        let (_, body) = post("/api/ab/shared/profiles", token, "");
        let value = json(&body);
        value["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|b| b["name"] == "secret")
            .map(|b| b["rule"].as_u64().unwrap())
    };
    assert_eq!(rule_of(&alice), Some(1));
    assert_eq!(rule_of(&bob), None, "bob has no rule on this address book");
    let list = format!("/api/ab/peers?current=1&pageSize=100&ab={ab}");
    assert_eq!(post(&list, &alice, "").0, 200, "a read-only user reads");
    assert_eq!(
        post(
            &format!("/api/ab/peer/add/{ab}"),
            &alice,
            r#"{"id":"666","alias":"x"}"#
        )
        .0,
        403
    );
    assert_eq!(
        server
            .http(
                "DELETE",
                &format!("/api/ab/peer/{ab}"),
                Some(&alice),
                r#"["123456789"]"#
            )
            .0,
        403
    );
    assert_eq!(
        post(
            &format!("/api/ab/tag/add/{ab}"),
            &alice,
            r#"{"name":"t","color":1}"#
        )
        .0,
        403
    );
    assert_eq!(post(&list, &admin, "").0, 200);
    let (_, peers) = post(&list, &admin, "");
    assert_eq!(json(&peers)["total"], 1, "nothing was changed: {peers}");

    // #21: no rule, no access, whatever the guid is known
    assert_eq!(post(&list, &bob, "").0, 403);
    assert_eq!(
        post(
            &format!("/api/ab/peer/add/{ab}"),
            &bob,
            r#"{"id":"666","alias":"x"}"#
        )
        .0,
        403
    );
    assert_eq!(post(&format!("/api/ab/tags/{ab}"), &bob, "").0, 403);
    assert_eq!(
        get(
            &format!("/api/ab/rules?current=1&pageSize=10&ab={ab}"),
            &bob
        )
        .0,
        403
    );
    // nor does a rule of read-write give the right to hand out rights
    assert_eq!(post("/api/ab/rule", &alice, &rule(3)).0 == 200, false);

    // read and write
    assert_eq!(post("/api/ab/rule", &admin, &rule(2)).0, 200);
    assert_eq!(rule_of(&alice), Some(2));
    assert_eq!(
        post(
            &format!("/api/ab/peer/add/{ab}"),
            &alice,
            r#"{"id":"777","alias":"added"}"#
        )
        .0,
        200
    );
    let (_, peers) = post(&list, &alice, "");
    assert_eq!(json(&peers)["total"], 2, "{peers}");
    // the personal address book is its owner's
    let (_, personal) = post("/api/ab/personal", &bob, "");
    let personal = json(&personal)["guid"].as_str().unwrap().to_owned();
    assert_eq!(
        post(
            &format!("/api/ab/peers?current=1&pageSize=100&ab={personal}"),
            &bob,
            ""
        )
        .0,
        200
    );
    assert_eq!(
        post(
            &format!("/api/ab/peers?current=1&pageSize=100&ab={personal}"),
            &alice,
            ""
        )
        .0,
        403
    );

    // a user cannot make itself an administrator
    let (_, users) = get("/api/users?current=1&pageSize=50", &alice);
    let alice_guid = json(&users)["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|u| u["name"] == "alice")
        .unwrap()["guid"]
        .as_str()
        .unwrap()
        .to_owned();
    let promote = format!(r#"{{"uuid":"{alice_guid}","is_admin":true,"status":0}}"#);
    let _ = server.http("PUT", "/api/user", Some(&alice), &promote);
    let (_, again) = server.http(
        "POST",
        "/api/login",
        None,
        r#"{"username":"alice","password":"pw-alice-123","id":"PEER_A","uuid":"ZTJl"}"#,
    );
    assert_eq!(
        json(&again)["user"]["admin"],
        false,
        "alice promoted herself: {again}"
    );
    assert_eq!(get("/api/user-list?current=1&pageSize=10", &alice).0, 401);
}

#[test]
fn a_failed_login_answers_a_json_error() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&[], &[]);
    let (status, body) = server.http(
        "POST",
        "/api/login",
        None,
        r#"{"username":"admin","password":"wrong","id":"E2E","uuid":"ZTJl"}"#,
    );
    assert_eq!(status, 401);
    // the clients jsonDecode this body: an empty one makes them crash
    assert!(
        json(&body)["error"].as_str().is_some_and(|e| !e.is_empty()),
        "body: {body:?}"
    );
}

#[test]
fn the_secret_key_is_generated() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&[], &[]);
    // generated at the first start, not a constant of the source
    let key = std::fs::read_to_string(server.dir.join("secret_key.txt")).unwrap();
    assert!(key.trim().len() >= 64, "secret_key.txt: {key:?}");
    assert_ne!(key.trim(), "wJq+s/xvwZjmMX3ev0p4gQTs9Ej5wt0brsk3ZGhoBTg=");
}

#[test]
fn the_probes_answer() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&[], &[]);
    for path in ["/livez", "/readyz"] {
        let (status, body) = server.http("GET", path, None, "");
        assert_eq!((status, body.as_str()), (200, "ok"), "{path}");
    }
}

#[test]
fn hbbr_answers_its_probes() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let get = |port: u16, path: &str| -> String {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        s.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        write!(s, "GET {path} HTTP/1.1\r\nHost: x\r\n\r\n").unwrap();
        let mut out = String::new();
        let _ = s.read_to_string(&mut out);
        out.lines().next().unwrap_or("").to_owned()
    };
    let free = |_: ()| {
        TcpListener::bind(("127.0.0.1", 0))
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    };
    let (relay, health) = (free(()), free(()));
    let dir = std::env::temp_dir().join(format!("hbbr-e2e-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_hbbr"))
        .arg(format!("--port={relay}"))
        .current_dir(&dir)
        .env("HEALTH_PORT", health.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let start = Instant::now();
    while TcpStream::connect(("127.0.0.1", health)).is_err()
        || TcpStream::connect(("127.0.0.1", relay)).is_err()
    {
        assert!(start.elapsed() < Duration::from_secs(30), "hbbr did not start");
        std::thread::sleep(Duration::from_millis(100));
    }
    assert_eq!(get(health, "/livez"), "HTTP/1.1 200 OK");
    assert_eq!(get(health, "/readyz"), "HTTP/1.1 200 OK");
    assert_eq!(get(health, "/nope"), "HTTP/1.1 404 Not Found");
    let _ = child.kill();
    let _ = child.wait();
    let _ = std::fs::remove_dir_all(&dir);
}


// ---- the backup downloaded by an administrator (--allow-remote-backup) ----

#[hbb_common::tokio::test(crate = "hbb_common::tokio")]
async fn the_backup_is_refused_without_the_flag() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&[], &[]);
    let admin = audit_login(&server, "admin", "Hello,world!");
    // an administrator, but the flag is not set: the endpoint does not exist
    assert_eq!(server.get_bytes("/api/backup", Some(&admin)).0, 404);
    // documented in the OpenAPI description whatever the flag is, with the 404 explained
    let (status, spec) = server.get_bytes("/openapi.json", None);
    assert_eq!(status, 200);
    let spec: serde_json::Value = serde_json::from_slice(&spec).unwrap();
    let op = &spec["paths"]["/api/backup"]["get"];
    assert_eq!(op["tags"][0], "backup", "{op}");
    assert!(op["responses"]["200"]["content"]["application/gzip"].is_object());
    assert!(op["responses"]["404"].is_object());
}

#[hbb_common::tokio::test(crate = "hbb_common::tokio")]
async fn an_administrator_downloads_a_backup() {
    let _lock = ONE_SERVER_AT_A_TIME
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&["--allow-remote-backup"], &[]);
    let admin = audit_login(&server, "admin", "Hello,world!");
    let body = r#"{"name":"alice","password":"pw-alice-123","confirm-password":"pw-alice-123","email":"alice@x.org","is_admin":false,"group_name":"Default"}"#;
    assert_eq!(server.http("POST", "/api/user", Some(&admin), body).0, 200);
    let alice = audit_login(&server, "alice", "pw-alice-123");

    // neither anonymous nor a plain user
    assert_eq!(server.get_bytes("/api/backup", None).0, 401);
    assert_eq!(server.get_bytes("/api/backup", Some(&alice)).0, 401);

    let (status, bytes) = server.get_bytes("/api/backup", Some(&admin));
    assert_eq!(status, 200);
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(&bytes[..]));
    let mut files = std::collections::HashMap::new();
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let name = entry.path().unwrap().to_string_lossy().into_owned();
        let mut data = Vec::new();
        entry.read_to_end(&mut data).unwrap();
        files.insert(name, data);
    }
    let db = &files["sctgdesk-backup/db_v2.sqlite3"];
    assert!(db.starts_with(b"SQLite format 3\0"), "not a SQLite file");
    // the copy is consistent: it holds the user created above, without the -wal file of the server
    assert!(
        db.windows(5).any(|w| w == b"alice"),
        "the user created before the backup is not in the copy"
    );
    assert!(files.contains_key("sctgdesk-backup/id_ed25519"));
    assert!(files.contains_key("sctgdesk-backup/id_ed25519.pub"));
    assert!(files.contains_key("sctgdesk-backup/MANIFEST.txt"));
}
