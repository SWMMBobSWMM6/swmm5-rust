When rustup cannot download the wasm32-wasip1 standard library, build std from Ubuntu's Rust sources.

`tools/build_wasm.sh`: rustc-1.91 + rust-1.91-src from apt, a private copy of `library/` with the `wasi` dependency restored in
`std/Cargo.toml` (Debian strips it), `-Zbuild-std=std,panic_abort` with `RUSTC_BOOTSTRAP=1`, linked by the system `wasm-ld` against
Ubuntu's wasi-libc (`-C link-self-contained=no -L /usr/lib/wasm32-wasi`). The result is a reactor with no `_start` and no `_initialize`;
node needs `wasi.initialize(inst)`, and the browser needs a WASI layer with 19 calls (MemWasi in `app/engine_core.js`).
