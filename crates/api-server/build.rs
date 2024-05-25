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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::str;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    name: String,
    private: Option<bool>,
    version: String,
    #[serde(rename = "type")]
    type_: Option<String>,
    scripts: HashMap<String, String>,
    dependencies: HashMap<String, String>,
    devDependencies: HashMap<String, String>,
}

impl PackageJson {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            private: None,
            version: String::new(),
            type_: None,
            scripts: HashMap::new(),
            dependencies: HashMap::new(),
            devDependencies: HashMap::new(),
        }
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }
}
fn main() {
    println!("cargo:rerun-if-changed=webconsole");

    let data = fs::read_to_string("./webconsole/package.json").unwrap();
    let mut package: PackageJson = serde_json::from_str(&data).unwrap();

    package.set_version(std::env::var("MAIN_PKG_VERSION").unwrap_or(std::env::var("CARGO_PKG_VERSION").unwrap()).as_str());

    let serialized = serde_json::to_string_pretty(&package).unwrap();
    fs::write("./webconsole/package.json", serialized).unwrap();

    let output = Command::new("npm")
        .current_dir("webconsole")
        .arg("install")
        .output()
        .expect("Failed to execute command");
    assert!(
        output.status.success(),
        "Failed to install npm dependencies: {}{}",
        str::from_utf8(&output.stdout).unwrap_or(""),
        str::from_utf8(&output.stderr).unwrap_or("")
    );

    let output = Command::new("npm")
        .current_dir("webconsole")
        .arg("run")
        .arg("build")
        .output()
        .expect("Failed to execute command");
    assert!(
        output.status.success(),
        "Failed to build webconsole: {}{}",
        str::from_utf8(&output.stdout).unwrap_or(""),
        str::from_utf8(&output.stderr).unwrap_or("")
    );
}
