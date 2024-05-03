use std::collections::HashMap;

pub fn get_host(headers: HashMap<String, String>) -> String {
    if let Some(host) = headers.get("host") {
        return format!("http://{}", host.to_string());
    }

    if let (Some(proto), Some(host)) = (headers.get("X-Forwarded-Proto"), headers.get("X-Forwarded-Host")) {
        return format!("{}://{}", proto, host);
    }

    "".to_string()
}