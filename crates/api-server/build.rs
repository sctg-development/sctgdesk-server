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
use flate2::{write::GzEncoder, Compression};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

/// Runs `npm <args>` in `dir`, panicking with the captured output on failure.
fn npm(dir: &Path, args: &[&str], what: &str) {
    let output = if cfg!(target_os = "windows") {
        let line = format!("npm {}", args.join(" "));
        Command::new("cmd.exe")
            .current_dir(dir)
            .args(["/C", &line])
            .output()
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

/// Compresses `source` to `target` (gzip, best compression), creating the folders. The gzip header holds no
/// date, so the same input always gives the same bytes.
fn gzip_file(source: &Path, target: &Path, transform: impl Fn(Vec<u8>) -> Vec<u8>) {
    let data = transform(
        fs::read(source).unwrap_or_else(|e| panic!("cannot read {}: {e}", source.display())),
    );
    fs::create_dir_all(target.parent().unwrap()).unwrap();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&data).unwrap();
    fs::write(target, encoder.finish().unwrap()).unwrap();
}

/// The files of `dir` and its sub-folders.
fn files_below(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
    {
        let path = entry.unwrap().path();
        if path.is_dir() {
            found.extend(files_below(&path));
        } else {
            found.push(path);
        }
    }
    found
}

/// Makes the compressed copy of the embedded web assets in `OUT_DIR`, which is what the server embeds.
///
/// The web console (`ui/<path>.gz`) and the JavaScript of the API documentation (`rapidoc.min.js.gz`) are about a
/// third of their size once compressed, and they weigh a lot in a binary meant for a router. The server sends the
/// compressed bytes as they are to the browsers that accept gzip (all of them) and decompresses for the others.
/// The source map of the API documentation is not embedded at all: it is 5 MB that only the browser developer tools
/// ask for, and the reference to it is removed from the script so that they do not even try.
fn compress_web_assets(console: &Path, rapidoc: &Path) {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    // a folder that is not wiped would keep the files of a former build of the console (names carry a hash)
    let ui = out.join("ui");
    let _ = fs::remove_dir_all(&ui);
    let dist = console.join("dist");
    for file in files_below(&dist) {
        let relative = file.strip_prefix(&dist).unwrap();
        let mut target = ui.join(relative).into_os_string();
        target.push(".gz");
        gzip_file(&file, Path::new(&target), |data| data);
    }
    gzip_file(
        &rapidoc.join("dist/sctgdesk-server.min.js"),
        &out.join("rapidoc.min.js.gz"),
        |data| {
            // drop the "//# sourceMappingURL=..." comment that ends the file: the map is not served
            let text = String::from_utf8_lossy(&data).into_owned();
            match text.rfind("//# sourceMappingURL=") {
                Some(at) if !text[at..].trim_end().contains('\n') => {
                    text[..at].trim_end().as_bytes().to_vec()
                }
                _ => data,
            }
        },
    );
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
        // the built assets, which are compressed below (also when they were built by hand)
        console.join("dist"),
        rapidoc.join("dist/sctgdesk-server.min.js"),
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
        compress_web_assets(&console, &rapidoc);
        return;
    }

    npm(
        &console,
        &["install", "--force"],
        "install npm dependencies of web/console",
    );
    npm(&console, &["run", "build"], "build web/console");
    npm(
        &rapidoc,
        &["install", "--force"],
        "install npm dependencies of web/rapidoc",
    );
    npm(&rapidoc, &["run", "build"], "build web/rapidoc");
    compress_web_assets(&console, &rapidoc);
}
