//! The math library. EPA SWMM's WebAssembly build (and swmm5-rust's wasm32-wasip1 build, which
//! matches it) gets pow, exp, log, sin... from wasi-libc. On wasm32-unknown-unknown, Rust would
//! otherwise use the Rust port of musl's libm inside compiler-builtins, whose last bits differ on
//! some inputs, enough to change a run's results. So the plugin links wasi-libc's libc.a, from
//! which the linker takes only the members it needs (the math functions, and errno); they are
//! pure computation and import nothing, so the module still imports only RustGIS's functions.
//!
//! WASI_LIBC_DIR points at the folder holding libc.a (default: Ubuntu's wasi-libc package).
fn main() {
    println!("cargo:rerun-if-env-changed=WASI_LIBC_DIR");
    let target = std::env::var("TARGET").unwrap_or_default();
    if target != "wasm32-unknown-unknown" {
        return;
    }
    let dir = std::env::var("WASI_LIBC_DIR").unwrap_or_else(|_| "/usr/lib/wasm32-wasi".into());
    if !std::path::Path::new(&dir).join("libc.a").is_file() {
        panic!("wasi-libc's libc.a not found in {dir} (install wasi-libc, or set WASI_LIBC_DIR)");
    }
    println!("cargo:rustc-link-search=native={dir}");
    println!("cargo:rustc-link-lib=static=c");
}
