use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for libraries in the specified directory.
    println!("cargo:rustc-link-search=asi_sdk/ASI_linux_mac_SDK_V1.29/lib/armv8");

    // Tell cargo to tell rustc to link the library statically.
    println!("cargo:rustc-link-lib=static=ASICamera2");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // Tell cargo to invalidate the built crate whenever the wrapper changes.
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/asi_sdk_bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("asi_sdk_bindings.rs"))
        .expect("Couldn't write bindings!");
}
