use clap::App;
mod common;
mod relay_server;
use flexi_logger::*;
use hbb_common::{config::RELAY_PORT, ResultType};
use relay_server::*;
mod health;
mod json_log;
mod version;

/// As a Windows service (`hbbr --windows-service`, see docs/windows.md) the program talks to the service manager and
/// runs `run_relay` as its body; otherwise it is the usual console program.
fn main() -> ResultType<()> {
    if hbbs::windows_service::requested() {
        return hbbs::windows_service::run("hbbr", run_relay);
    }
    run_relay()
}

fn run_relay() -> ResultType<()> {
    let logger = Logger::try_with_env_or_str("info")?;
    // a service has no console: its log goes to files (rotated) in the data directory
    let logger = match hbbs::windows_service::log_dir() {
        Some(dir) => logger
            .log_to_file(FileSpec::default().directory(dir).basename("hbbr").suppress_timestamp())
            .rotate(Criterion::Size(10_000_000), Naming::Numbers, Cleanup::KeepLogFiles(5))
            .append(),
        None => logger.log_to_stdout(),
    };
    let _logger = logger
        .format(json_log::format_or(opt_format))
        .write_mode(WriteMode::Async)
        .start()?;
    let args = format!(
        "-p, --port=[NUMBER(default={RELAY_PORT})] 'Sets the listening port'
        -k, --key=[KEY] 'Only allow the client with the same key'
        , --windows-service 'Runs hbbr as a Windows service (put it in the binPath of sc.exe create, not for a console)'
        ",
    );
    let matches = App::new("hbbr")
        .version(version::VERSION)
        .author("Purslane Ltd. <info@rustdesk.com>")
        .about("RustDesk Relay Server")
        .args_from_usage(&args)
        .get_matches();
    if let Ok(v) = ini::Ini::load_from_file(".env") {
        if let Some(section) = v.section(None::<String>) {
            section.iter().for_each(|(k, v)| std::env::set_var(k, v));
        }
    }
    let mut port = RELAY_PORT;
    if let Ok(v) = std::env::var("PORT") {
        let v: i32 = v.parse().unwrap_or_default();
        if v > 0 {
            port = v + 1;
        }
    }
    if let Ok(health_port) = std::env::var("HEALTH_PORT") {
        if let Ok(health_port) = health_port.parse::<u16>() {
            let relay_port = matches
                .value_of("port")
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or(port as u16);
            std::thread::spawn(move || health::serve(health_port, relay_port));
        }
    }
    start(
        matches.value_of("port").unwrap_or(&port.to_string()),
        matches
            .value_of("key")
            .unwrap_or(&std::env::var("KEY").unwrap_or_default()),
    )?;
    Ok(())
}
