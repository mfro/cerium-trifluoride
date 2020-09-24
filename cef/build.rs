use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../tools/make_rust_wrapper.py");
    // println!("cargo:rerun-if-env-changed=CEF_ROOT");

    std::env::var("CEF_ROOT")
        .expect("environment variable CEF_ROOT must be set to the target CEF distribution");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let result = Command::new("python")
        .arg("../tools/make_rust_wrapper.py")
        .arg("--output-dir")
        .arg(&out_path)
        .output()
        .unwrap();

    if !result.status.success() {
        println!("{}", String::from_utf8_lossy(&result.stdout));
        println!("{}", String::from_utf8_lossy(&result.stderr));
        panic!("exit status: {}", result.status)
    }
}
