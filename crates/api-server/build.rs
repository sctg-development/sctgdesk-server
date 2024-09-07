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
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str;

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

    // Construit le chemin du fichier dans le répertoire temporaire
    let tmp_dir = env::var("TMP")
        .or_else(|_| env::var("TEMP"))
        .or_else(|_| env::var("TMPDIR"))
        .unwrap_or_else(|_| "/tmp".to_string());
    let mut path = PathBuf::from(tmp_dir);
    path.push("version-8659B48F-5726-433D-BEC2-C7042FE9D93B.txt");
    // Lit la version à partir du fichier
    let version =
        fs::read_to_string(&path).unwrap_or_else(|_| env::var("CARGO_PKG_VERSION").unwrap());

    package.set_version(&version);

    let serialized = serde_json::to_string_pretty(&package).unwrap();
    fs::write("./webconsole/package.json", serialized).unwrap();

    let is_windows = cfg!(target_os = "windows");

    let (command, install_args, build_args) = if is_windows {
        ("cmd.exe", &["/C", "npm install --force"], &["/C", "npm run build"])
    } else {
        ("npm", &["install", "--force"], &["run", "build"])
    };

    let output = Command::new(command)
        .current_dir("webconsole")
        .args(install_args)
        .output()
        .expect("Failed to execute command");
    assert!(
        output.status.success(),
        "Failed to install npm dependencies: {}{}",
        str::from_utf8(&output.stdout).unwrap_or(""),
        str::from_utf8(&output.stderr).unwrap_or("")
    );

    let output = Command::new(command)
        .current_dir("webconsole")
        .args(build_args)
        .output()
        .expect("Failed to execute command");
    assert!(
        output.status.success(),
        "Failed to build webconsole: {}{}",
        str::from_utf8(&output.stdout).unwrap_or(""),
        str::from_utf8(&output.stderr).unwrap_or("")
    );
}
