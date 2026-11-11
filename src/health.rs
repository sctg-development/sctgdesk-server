//! `/livez` and `/readyz` of hbbr, on `HEALTH_PORT` (off when unset).
//!
//! hbbr has no HTTP server: this is a minimal one, on its own thread. `/livez` answers 200
//! as long as the process runs; `/readyz` answers 200 when the relay port accepts a TCP
//! connection, 503 otherwise.
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

pub fn serve(health_port: u16, relay_port: u16) {
    let listener = match TcpListener::bind((Ipv4Addr::UNSPECIFIED, health_port)) {
        Ok(l) => l,
        Err(e) => {
            hbb_common::log::error!("cannot listen on HEALTH_PORT {health_port}: {e}");
            return;
        }
    };
    hbb_common::log::info!("hbbr health probes on port {health_port}");
    for stream in listener.incoming().flatten() {
        // one thread per probe: a stalled client must not block the next probe
        std::thread::spawn(move || answer(stream, relay_port));
    }
}

fn answer(mut stream: TcpStream, relay_port: u16) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
    // a request can arrive in several segments: read up to the end of the headers
    let mut buf = [0u8; 1024];
    let mut n = 0;
    while n < buf.len() && !buf[..n].windows(4).any(|w| w == b"\r\n\r\n") {
        match stream.read(&mut buf[n..]) {
            Ok(0) | Err(_) => break,
            Ok(read) => n += read,
        }
    }
    let request = String::from_utf8_lossy(&buf[..n]);
    let path = request.split_whitespace().nth(1).unwrap_or("");
    let (status, body) = match path {
        "/livez" => ("200 OK", "ok"),
        "/readyz" => {
            let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, relay_port));
            if TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_ok() {
                ("200 OK", "ok")
            } else {
                ("503 Service Unavailable", "relay port not accepting")
            }
        }
        _ => ("404 Not Found", "not found"),
    };
    let _ = write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
}
