use std::path::Path;

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

    //TODO bindgen
}
