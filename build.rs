use cmake::Config;
use std::env;
use std::path::PathBuf;

/// Resolve the PaPILO source directory.
///
/// Priority:
///   1. `PAPILO_SRC` environment variable (used for local development against a
///      checkout of PaPILO that is being edited in place).
///   2. The bundled `papilo` git submodule next to this crate.
fn papilo_source_dir() -> PathBuf {
    if let Ok(dir) = env::var("PAPILO_SRC") {
        let p = PathBuf::from(dir);
        assert!(
            p.join("src/papilopresolve.cpp").exists(),
            "PAPILO_SRC={} does not contain src/papilopresolve.cpp",
            p.display()
        );
        return p;
    }

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let submodule = manifest.join("papilo");
    assert!(
        submodule.join("src/papilopresolve.cpp").exists(),
        "PaPILO source not found. Either set PAPILO_SRC to a PaPILO checkout that \
         contains src/papilopresolve.cpp, or initialize the submodule with \
         `git submodule update --init`."
    );
    submodule
}

fn main() {
    let papilo_src = papilo_source_dir();
    let target = env::var("TARGET").unwrap();
    let apple = target.contains("apple");
    let linux = target.contains("linux");
    let mingw = target.contains("pc-windows-gnu");

    let profile = env::var("PROFILE").unwrap();
    let build_type = if profile == "release" {
        "Release"
    } else {
        "Debug"
    };

    // Rebuild if the C API or its implementation change.
    println!(
        "cargo:rerun-if-changed={}",
        papilo_src.join("src/papilopresolve.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        papilo_src.join("src/papilopresolve.cpp").display()
    );
    println!("cargo:rerun-if-env-changed=PAPILO_SRC");

    // Build only the presolve C API shared library. It depends only on the
    // header-only PaPILO core, so SCIP, SoPlex, GMP and TBB are all disabled and
    // the standalone executable / test suite are not built.
    let dst = Config::new(&papilo_src)
        .profile(build_type)
        .define("SCIP", "off")
        .define("SOPLEX", "off")
        .define("GMP", "off")
        .define("TBB", "off")
        .define("LUSOL", "off")
        .build_target("papilopresolve")
        .build_arg("-j4")
        .build();

    // With `build_target`, cmake does not run the install step, so the build
    // artifacts live under `<OUT_DIR>/build`.
    let build_dir = dst.join("build");
    let lib_dir = build_dir.join("lib");
    let export_header_dir = build_dir.join("binaries");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=papilopresolve");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());

    if apple {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if linux || mingw {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    println!("cargo:lib_dir={}", lib_dir.display());

    // generate bindings against the presolve C API header
    let bindings = bindgen::Builder::default()
        .header(papilo_src.join("src/papilopresolve.h").to_str().unwrap())
        .clang_arg(format!("-I{}", export_header_dir.display()))
        .allowlist_function("papilo_.*")
        .allowlist_type("PAPILO_.*|Papilo_.*")
        .allowlist_var("PAPILO_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
