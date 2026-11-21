use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    hbb_common::gen_version();
    // Get the current version from the environment
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    // Build a temporary file path
    let tmp_dir = env::var("TMP")
        .or_else(|_| env::var("TEMP"))
        .or_else(|_| env::var("TMPDIR"))
        .unwrap_or_else(|_| "/tmp".to_string());
    let mut path = PathBuf::from(tmp_dir);
    path.push("version-8659B48F-5726-433D-BEC2-C7042FE9D93B.txt");

    // write the version to the file
    let mut file = File::create(&path).unwrap();
    file.write_fmt(format_args!("{}", version)).unwrap();

    // Pass also the version to the build script
    println!("cargo:rustc-env=MAIN_PKG_VERSION={}", version);
}
