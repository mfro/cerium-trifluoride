## Rust CEF bindings

Rust language bindings and minimal wrapper around the CEF C API. The goal of this library is to enable use of CEF without directly using unsafe or extern code and no more.

### Repository Layout

`tools` contains the code generation tool, based heavily on and using the `cef_parser.py` and related CEF tooling. It generates Rust wrapper code from the output of the CEF C++ parser, similar to the `make_capi_header.py` tool in the CEF repository. It has also been extended to parse basic `struct` and `enum` definitions in `include/internal/cef_types.h`.

`cef-sys` is a `rust-bindgen`-generated crate. It is pretty minimal templated code from the basic recommended `bindgen` structure. It invokes `tools/make_rust_wrapper.py` to generate a list of header files to feed to `bindgen`.

`cef` contains the Rust wrappers. The base CEF wrappers are implemented here, such as refcounting and string types, and the auto-generated wrappers are generated and included from the build script by invoking `tools/make_rust_wrapper.py`.

`cef-test` contains a sample of how the `cef` crate can be used.

### Build Instructions

- Download a minimal or standard binary distribuion from the [CEF build site](http://opensource.spotify.com/cefbuilds/index.html)
- Unpack the archive
- Set the `CEF_ROOT` environment variable to the resulting directory
- Create a temporary directory in the crate you are trying to run (eg `cef-test/release`)
- Copy the contents of `$CEF_ROOT/Release` and `$CEF_ROOT/Resources` into the temporary directory
- From within the build directory, run `cargo run`. If you get a cargo error that says "(never executed)", do `cargo run` from the crate root once first (it should crash) before running it from within the temporary directory.

`windows64` is the only target this has been tested on.

[CEF Repository](https://bitbucket.org/chromiumembedded/cef)
