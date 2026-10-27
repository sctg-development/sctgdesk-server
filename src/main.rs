// https://tools.ietf.org/rfc/rfc5128.txt
// https://blog.csdn.net/bytxl/article/details/44344855

use flexi_logger::*;
use hbb_common::{bail, config::RENDEZVOUS_PORT, ResultType};
use hbbs::{common::*, *};
use rocket::{
    config::LogLevel,
    data::{Limits, ToByteUnit},
};

use std::thread;
use sctgdesk_api_server::build_rocket;

const RMEM: usize = 0;
const API_PORT: i32 = 21114;

fn get_rocket_log_level() -> LogLevel {
    let log_level_env = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    match log_level_env.as_str() {
        "off" => return LogLevel::Off,
        "error" => return LogLevel::Critical,
        "warn" => return LogLevel::Normal,
        "info" => return LogLevel::Normal,
        "debug" => return LogLevel::Debug,
        "trace" => return LogLevel::Debug,
        _ => return LogLevel::Off
        
    }
}
#[rocket::main]
async fn start_rocket() -> ResultType<()> {
    let port = get_arg_or("api-port", API_PORT.to_string()).parse::<i32>()?;
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", port))
        .merge(("log_level", get_rocket_log_level()))
        .merge(("secret_key", "wJq+s/xvwZjmMX3ev0p4gQTs9Ej5wt0brsk3ZGhoBTg="))
        .merge(("ident",  format!("SCTGDeskServer/{}", env!("CARGO_PKG_VERSION"))))
        .merge(("limits", Limits::new().limit("json", 2.mebibytes())));
    let _rocket = build_rocket(figment).await.ignite().await?.launch().await;
    Ok(())
}

fn log_format(
    write: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let file = record.file().unwrap_or("unknown");
    let line = record.line().unwrap_or(0);
    let file = file.rsplitn(2, '/').next().unwrap_or(file); // Obtenez seulement le nom du fichier, pas le chemin complet
    let timestamp = now.now().to_string();
    write!(
        write,
        "{} [{}] {}:{} - {}",
        timestamp,
        record.level(),
        file,
        line,
        record.args()
    )
}

fn main() -> ResultType<()> {
    let _logger = Logger::try_with_env_or_str("info")?
        .log_to_stdout()
        .format(log_format)
        .write_mode(WriteMode::Async)
        .start()?;
    let args = format!(
        "-c --config=[FILE] +takes_value 'Sets a custom config file'
        -a, --api-port=[NUMBER(default={API_PORT})] 'Sets the listening port of API server'
        -p, --port=[NUMBER(default={RENDEZVOUS_PORT})] 'Sets the listening port'
        -s, --serial=[NUMBER(default=0)] 'Sets configure update serial number'
        -R, --rendezvous-servers=[HOSTS] 'Sets rendezvous servers, separated by comma'
        -u, --software-url=[URL] 'Sets download url of RustDesk software of newest version'
        -r, --relay-servers=[HOST] 'Sets the default relay servers, separated by comma'
        -M, --rmem=[NUMBER(default={RMEM})] 'Sets UDP recv buffer size, set system rmem_max first, e.g., sudo sysctl -w net.core.rmem_max=52428800. vi /etc/sysctl.conf, net.core.rmem_max=52428800, sudo sysctl –p'
        , --mask=[MASK] 'Determine if the connection comes from LAN, e.g. 192.168.0.0/16'
        -k, --key=[KEY] 'Only allow the client with the same key'
        , --logged-in-only 'Only allow logged in user to control'
        , --require-kx-v1 'Refuse TCP key exchange version 0 (clients older than the signed key exchange); can also be set with REQUIRE_KX_V1=Y'",
    );
    init_args(&args, "hbbs", "RustDesk ID/Rendezvous Server");
    let port = get_arg_or("port", RENDEZVOUS_PORT.to_string()).parse::<i32>()?;
    if port < 3 {
        bail!("Invalid port");
    }
    let rmem = get_arg("rmem").parse::<usize>().unwrap_or(RMEM);
    let serial: i32 = get_arg("serial").parse().unwrap_or(0);

    std::env::set_var("MAIN_PKG_VERSION", env!("CARGO_PKG_VERSION"));
    let handle = thread::spawn(|| {
        let rt = rocket::tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = sctgdesk_api_server::ApiState::new_with_db("db_v2.sqlite3").await;
        });
    });
    handle.join().unwrap();
    let rocket_thread = thread::spawn(|| {
        let _ = start_rocket();
    });

    RendezvousServer::start(port, serial, &get_arg_or("key", "-".to_owned()), rmem)?;
    let _ = rocket_thread.join();
    Ok(())
}
