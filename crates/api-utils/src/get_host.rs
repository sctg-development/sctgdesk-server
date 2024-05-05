use std::collections::HashMap;

pub fn get_host(headers: HashMap<String, String>) -> String {
    // Default to http
    let mut proto = "http".to_string(); 

    // Check if the headers contain the X-Forwarded-Proto header
    if let Some(proto_in_headers) = headers.get("X-Forwarded-Proto") {
        proto = proto_in_headers.to_string();
    }

    // Check if the headers contain the X-Forwarded-Host header
    if let Some(host_in_headers) = headers.get("X-Forwarded-Host") {
        return format!("{}://{}", proto, host_in_headers);
    }

    // Default to the host header
    if let Some(host) = headers.get("host") {
        return format!("{}://{}",proto, host.to_string());
    }

    "".to_string()
}