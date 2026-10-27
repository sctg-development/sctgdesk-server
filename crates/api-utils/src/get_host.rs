// Copyright (c) 2024 Ronan LE MEILLAT for SCTG Development
//
// This file is part of the SCTGDesk project.
//
// SCTGDesk is free software: you can redistribute it and/or modify
// it under the terms of the Affero General Public License version 3 as
// published by the Free Software Foundation.
//
// SCTGDesk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Affero General Public License for more details.
//
// You should have received a copy of the Affero General Public License
// along with SCTGDesk. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
use std::collections::HashMap;

pub fn get_host(headers: HashMap<String, String>) -> String {
    // Default to http
    let mut proto = "http".to_string(); 

    // Check if the headers contain the X-Forwarded-Proto header
    if let Some(proto_in_headers) = headers.get("x-forwarded-proto") {
        proto = proto_in_headers.to_string();
    }

    // Check if the headers contain the X-Forwarded-Host header
    if let Some(host_in_headers) = headers.get("x-forwarded-host") {
        return format!("{}://{}", proto, host_in_headers);
    }

    // Default to the host header
    if let Some(host) = headers.get("host") {
        return format!("{}://{}",proto, host.to_string());
    }

    "".to_string()
}