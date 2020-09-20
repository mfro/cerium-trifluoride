use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../tools/make_rust_wrapper.py");
    println!("cargo:rerun-if-env-changed=CEF_ROOT");

    let cef_path = std::env::var("CEF_ROOT")
        .expect("environment variable CEF_ROOT must be set to the target CEF distribution");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let header_path = out_path.join("wrapper.h");

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

    assert!(header_path.exists());

    println!("cargo:rustc-link-lib=dylib={}", "libcef");
    println!(
        "cargo:rustc-link-search=native={}",
        AsRef::<Path>::as_ref(&cef_path)
            .join("Release")
            .to_str()
            .unwrap()
    );

    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .derive_default(true)
        .generate_comments(false)
        .whitelist_type("cef_.*")
        .whitelist_function("cef_.*")
        .clang_arg(format!("-I{}", &cef_path))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
