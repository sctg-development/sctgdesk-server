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
    sodiumoxide::crypto::{box_, secretbox, sign},
    socket_client,
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
        let dir = std::env::temp_dir().join(format!(
            "hbbs-e2e-{}-{}",
            std::process::id(),
            api_port
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_hbbs"));
        cmd.arg(format!("--api-port={api_port}"))
            .args(extra_args)
            .current_dir(&dir)
            .env_remove("API_SERVER")
            .env_remove("DB_URL")
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        for (k, v) in envs {
            cmd.env(k, v);
        }
        let child = cmd.spawn().expect("cannot start hbbs");
        let mut server = Server { child, dir, api_port };
        server.wait_until_ready();
        server
    }

    fn wait_until_ready(&mut self) {
        let start = Instant::now();
        let ready = |p: u16| TcpStream::connect(("127.0.0.1", p)).is_ok();
        while !(ready(RENDEZVOUS_PORT) && ready(self.api_port) && self.dir.join("id_ed25519.pub").exists())
        {
            assert!(
                self.child.try_wait().unwrap().is_none(),
                "hbbs exited while starting"
            );
            assert!(start.elapsed() < STARTUP_TIMEOUT, "hbbs did not start in time");
            std::thread::sleep(Duration::from_millis(200));
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

    /// POST to the API with the given bearer token, returns the HTTP status.
    fn post_with_bearer(&self, path: &str, bearer: &str) -> u16 {
        let mut stream = TcpStream::connect(("127.0.0.1", self.api_port)).unwrap();
        stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        write!(
            stream,
            "POST {path} HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {bearer}\r\n\
             Content-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}"
        )
        .unwrap();
        let mut answer = Vec::new();
        let _ = stream.read_to_end(&mut answer);
        let text = String::from_utf8_lossy(&answer);
        text.split_whitespace()
            .nth(1)
            .and_then(|code| code.parse().ok())
            .unwrap_or_else(|| panic!("no HTTP answer for bearer {bearer:?}"))
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
    let _lock = ONE_SERVER_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&[], &[]);
    let mut conn = connect_and_exchange_keys(&server.public_key()).await.unwrap();
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
    let _lock = ONE_SERVER_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
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
    let _lock = ONE_SERVER_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let server = Server::start(&["--logged-in-only"], &[("API_SERVER", "http://127.0.0.1:1")]);
    let answer = punch_hole(&server.public_key(), "abc").await;
    assert!(
        answer.other_failure.to_lowercase().contains("unreachable"),
        "unexpected answer: {answer:?}"
    );
}

#[hbb_common::tokio::test(crate = "hbb_common::tokio")]
async fn logged_in_only_checks_tokens_on_the_configured_api_port() {
    let _lock = ONE_SERVER_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    // no API_SERVER: it must follow --api-port, not the default 21114
    let server = Server::start(&["--logged-in-only"], &[]);
    let answer = punch_hole(&server.public_key(), "abc").await;
    assert!(
        answer.other_failure.to_lowercase().contains("session expired"),
        "unexpected answer: {answer:?}"
    );
}
