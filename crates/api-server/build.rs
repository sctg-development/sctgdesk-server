use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=webconsole");

    let output = Command::new("npm")
        .current_dir("webconsole")
        .arg("install")
        .output()
        .expect("Failed to execute command");
    assert!(
        output.status.success(),
        "Failed to install npm dependencies"
    );

    let output = Command::new("npm")
        .current_dir("webconsole")
        .arg("run")
        .arg("build")
        .output()
        .expect("Failed to execute command");
    assert!(output.status.success(), "Failed to build webconsole");
}
