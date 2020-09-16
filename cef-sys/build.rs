extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let cef_path = std::env::var("CEF_PATH").expect("CEF_PATH must be specified");
    let cef_path = PathBuf::from(cef_path);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=dylib={}", "libcef");
    println!(
        "cargo:rustc-link-search=native={}",
        cef_path.join("Release").to_str().unwrap()
    );

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_default(true)
        .generate_comments(true)
        .whitelist_type("cef_.*")
        .whitelist_function("cef_.*")
        .clang_arg(format!("-I{}", cef_path.to_str().unwrap()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
