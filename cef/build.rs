use std::path::PathBuf;
use std::process::Command;
use std::{env, fs::File, io::prelude::*, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=../tools/make_rust_wrapper.py");
    // println!("cargo:rerun-if-env-changed=CEF_ROOT");

    let cef_root = std::env::var("CEF_ROOT")
        .expect("environment variable CEF_ROOT must be set to the target CEF distribution");
    let cef_path: &Path = cef_root.as_ref();

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

    let cef_string_header = cef_path.join("include/internal/cef_string.h");
    let mut cef_string = vec![];
    File::open(cef_string_header)
        .unwrap()
        .read_to_end(&mut cef_string)
        .unwrap();
    let cef_string = String::from_utf8(cef_string).unwrap();

    let utf8 = cef_string.contains("\n#define CEF_STRING_TYPE_UTF8");
    let utf16 = cef_string.contains("\n#define CEF_STRING_TYPE_UTF16");
    let wide = cef_string.contains("\n#define CEF_STRING_TYPE_WIDE");

    if utf8 {
        assert!(!utf16 && !wide, "invalid CEF_STRING_TYPE configuration");
        println!("cargo:rustc-cfg=cef_string_type_utf8");
    } else if utf16 {
        assert!(!utf8 && !wide, "invalid CEF_STRING_TYPE configuration");
        println!("cargo:rustc-cfg=cef_string_type_utf16");
    } else if wide {
        assert!(!utf8 && !utf16, "invalid CEF_STRING_TYPE configuration");
        println!("cargo:rustc-cfg=cef_string_type_utfwide");
    } else {
        panic!("invalid CEF_STRING_TYPE configuration");
    }
}
