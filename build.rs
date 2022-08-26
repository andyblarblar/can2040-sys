use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=can-CMakeLists.txt");

    //Copy custom cmakelists to the can2040 submodule
    if !Path::exists("can2040/CMakeLists.txt".as_ref()) {
        std::fs::copy("can-CMakeLists.txt", "can2040/CMakeLists.txt").expect("File copy failed!");
    }

    //Compile and link can2040
    let mut conf = cmake::Config::new("can2040");
    let mut dst = conf.build();
    dst.push("build");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=can2040");

    let bindings = bindgen::Builder::default()
        .header("can2040/src/can2040.h")
        .ctypes_prefix("cty")
        .use_core()
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
