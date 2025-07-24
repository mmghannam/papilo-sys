use std::env;
use cmake::Config;
use std::path::PathBuf;

fn build_and_link_papilo() -> PathBuf {
    let profile = env::var("PROFILE").unwrap();
    let build_type = if profile == "release" { "Release" } else { "Debug" };

    // build papilo library with cmake
    let dst = Config::new("papilo")
        .profile(build_type)
        .define("SCIP", "off")
        .define("SOPLEX", "off")
        .define("TBB", "Off")
        .define("LUSOL", "Off")
        .build();

    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=papilo-core");

    let target = env::var("TARGET").unwrap();
    let apple = target.contains("apple");
    let linux = target.contains("linux");
    let mingw = target.contains("pc-windows-gnu");
    if apple {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if linux || mingw {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    dst
}

fn main() {
    let dst = build_and_link_papilo();

    // generate bindings
    let bindings = bindgen::Builder::default()
        .header("papilo/src/papilolib.h")
        .clang_arg(format!("-I{}/include", dst.display()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}