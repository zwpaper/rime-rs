extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=dylib=rime");
    println!("cargo:rustc-link-search=native=./lib/");

    env::set_var(
        "LLVM_CONFIG_PATH",
        "/usr/local/Cellar/llvm/10.0.0_3/bin/llvm-config",
    );
    env::set_var("OUT_DIR", ".");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./lib/rime_api.h")
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
        .write_to_file(out_path.join("./src/rime/ffi.rs"))
        .expect("Couldn't write bindings!");
}
