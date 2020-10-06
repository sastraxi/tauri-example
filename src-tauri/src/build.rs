extern crate bindgen;

#[cfg(windows)]
extern crate winres;

use std::env;
use std::path::PathBuf;

#[cfg(windows)]
fn main() {
    if std::path::Path::new("icons/icon.ico").exists() {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.compile().expect("Unable to find visual studio tools");
    } else {
        panic!("No Icon.ico found. Please add one or check the path");
    }
    panic!("Windows opentok library not built yet; exiting. :(")
}

#[cfg(not(windows))]
fn main() {
    let library_path = "opentok/libopentok_linux_llvm_x86_64";
    let wrapper_path = "opentok/wrapper-linux-x86_64.h";

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!(r"cargo:rustc-link-search={}/lib/", library_path);
    println!("cargo:rustc-link-lib=opentok");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", wrapper_path);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(wrapper_path)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate opentok bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write opentok bindings!");
}
