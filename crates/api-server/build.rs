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
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

/// Runs `npm <args>` in `dir`, panicking with the captured output on failure.
fn npm(dir: &Path, args: &[&str], what: &str) {
    let output = if cfg!(target_os = "windows") {
        let line = format!("npm {}", args.join(" "));
        Command::new("cmd.exe").current_dir(dir).args(["/C", &line]).output()
    } else {
        Command::new("npm").current_dir(dir).args(args).output()
    }
    .expect("Failed to execute npm");
    assert!(
        output.status.success(),
        "Failed to {}: {}{}",
        what,
        str::from_utf8(&output.stdout).unwrap_or(""),
        str::from_utf8(&output.stderr).unwrap_or("")
    );
}

/// `include_dir!` and `include_str!` need these files at compile time. When the web build is
/// skipped, create empty placeholders (the directories are git-ignored) unless a real build exists.
fn ensure_placeholder(path: &Path, content: &str) {
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=SCTGDESK_SKIP_WEB_BUILD");
    let web: PathBuf = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../web");
    let (console, rapidoc) = (web.join("console"), web.join("rapidoc"));
    for watched in [
        console.join("src"),
        console.join("index.html"),
        console.join("package.json"),
        rapidoc.join("index.ts"),
        rapidoc.join("package.json"),
    ] {
        println!("cargo:rerun-if-changed={}", watched.display());
    }

    // SCTGDESK_SKIP_WEB_BUILD=1 builds the server without Node.js; the embedded web console
    // and API documentation are then placeholders.
    if env::var_os("SCTGDESK_SKIP_WEB_BUILD").is_some() {
        ensure_placeholder(
            &console.join("dist/index.html"),
            "<!doctype html><title>SCTGDesk</title><p>Web console not built (SCTGDESK_SKIP_WEB_BUILD).",
        );
        ensure_placeholder(&rapidoc.join("dist/sctgdesk-server.min.js"), "");
        ensure_placeholder(&rapidoc.join("dist/sctgdesk-server.min.js.map"), "");
        return;
    }

    npm(&console, &["install", "--force"], "install npm dependencies of web/console");
    npm(&console, &["run", "build"], "build web/console");
    npm(&rapidoc, &["install", "--force"], "install npm dependencies of web/rapidoc");
    npm(&rapidoc, &["run", "build"], "build web/rapidoc");
}
