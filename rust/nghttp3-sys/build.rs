use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.join("../..");

    println!("cargo:rerun-if-changed={}", repo_root.join("CMakeLists.txt").display());
    println!("cargo:rerun-if-changed={}", repo_root.join("CMakeOptions.txt").display());
    println!("cargo:rerun-if-changed={}", repo_root.join("lib").display());

    let dst = cmake::Config::new(&repo_root)
        .define("ENABLE_LIB_ONLY", "ON")
        .define("ENABLE_STATIC_LIB", "ON")
        .define("ENABLE_SHARED_LIB", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .profile("Release")
        .build();

    emit_link_searches(&dst);
    println!("cargo:rustc-link-lib=static=nghttp3");

    let public_header = repo_root.join("lib/includes/nghttp3/nghttp3.h");
    let source_include = repo_root.join("lib/includes");
    let installed_include = dst.join("include");

    let bindings = bindgen::Builder::default()
        .header(public_header.to_string_lossy())
        .clang_arg(format!("-I{}", source_include.display()))
        .clang_arg(format!("-I{}", installed_include.display()))
        .allowlist_function("nghttp3_.*")
        .allowlist_type("nghttp3_.*")
        .allowlist_var("NGHTTP3_.*")
        .derive_default(true)
        .generate_comments(true)
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate nghttp3 bindings");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write generated nghttp3 bindings");
}

fn emit_link_searches(dst: &Path) {
    for dir in [dst.join("lib"), dst.join("lib64"), dst.join("build/lib")] {
        if dir.exists() {
            println!("cargo:rustc-link-search=native={}", dir.display());
        }
    }
}
