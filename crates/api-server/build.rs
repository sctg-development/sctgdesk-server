use std::process::Command;
use std::str;

fn main() {
    println!("cargo:rerun-if-changed=webconsole");

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