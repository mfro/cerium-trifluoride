use std::env;
use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn find_all_headers<P1: AsRef<Path>, P2: AsRef<Path>>(
    path: P1,
    prefix: P2,
    headers: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    for entry in path.as_ref().read_dir()? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            headers.push(prefix.as_ref().join(entry.file_name()));
        }
    }

    Ok(())
}

fn main() {
    // println!("cargo:rerun-if-env-changed=CEF_ROOT");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path: &Path = out_dir.as_ref();
    let wrapper_path = out_path.join("wrapper.h");

    let cef_root = std::env::var("CEF_ROOT")
        .expect("environment variable CEF_ROOT must be set to the target CEF distribution");

    let cef_path: &Path = cef_root.as_ref();
    let mut capi_header_paths = vec![];
    find_all_headers(
        &cef_path.join("include/capi"),
        "include/capi",
        &mut capi_header_paths,
    )
    .unwrap();
    find_all_headers(
        cef_path.join("include/capi/views"),
        "include/capi/views",
        &mut capi_header_paths,
    )
    .unwrap();

    {
        let mut wrapper = BufWriter::new(File::create(&wrapper_path).unwrap());
        for path in &capi_header_paths {
            writeln!(wrapper, "#include \"{}\"", &path.to_str().unwrap()).unwrap();
        }
    }

    println!("cargo:rustc-link-lib=dylib={}", "libcef");
    println!(
        "cargo:rustc-link-search=native={}",
        cef_path.join("Release").to_str().unwrap()
    );

    let bindings = bindgen::Builder::default()
        .header(wrapper_path.to_str().unwrap())
        .derive_default(true)
        .generate_comments(false)
        .whitelist_type("cef_.*")
        .whitelist_var("cef_.*")
        .whitelist_function("cef_.*")
        .whitelist_type("CEF_.*")
        .whitelist_var("CEF_.*")
        .whitelist_function("CEF_.*")
        .clang_arg(format!("-I{}", &cef_root))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // let bindings = bindings.to_string();

    // let utf8 = bindings.contains("CEF_STRING_TYPE_UTF8");
    // let utf16 = bindings.contains("CEF_STRING_TYPE_UTF16");
    // let wide = bindings.contains("CEF_STRING_TYPE_WIDE");

    // if utf8 {
    //     assert!(!utf16 && !wide, "invalid CEF_STRING_TYPE configuration");
    //     println!("cargo:rustc-cfg=cef_string_type_utf8");
    // } else if utf16 {
    //     assert!(!utf8 && !wide, "invalid CEF_STRING_TYPE configuration");
    //     println!("cargo:rustc-cfg=cef_string_type_utf16");
    // } else if wide {
    //     assert!(!utf8 && !utf16, "invalid CEF_STRING_TYPE configuration");
    //     println!("cargo:rustc-cfg=cef_string_type_utfwide");
    // } else {
    //     panic!("invalid CEF_STRING_TYPE configuration");
    // }
}
