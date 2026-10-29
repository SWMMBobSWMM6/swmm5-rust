Rust's standard library panics on wasm32-wasip1 for `std::env::temp_dir()` and `std::process::id()`; a native-only gate will not see it.

Nine models that open a scratch file (rain interface files, hotstart files) passed the native gate and trapped in WebAssembly with
"no filesystem on wasm" and then "unsupported". The fix was `cfg!(target_os = "wasi")` branches in `getTempFileName()` and `mkstemp()`
(commit "WASI: scratch files in the current directory, no process id"). Run the WebAssembly gate on the full set, not a sample, before
publishing a wasm engine; the trap message comes through stderr (MemWasi captures it as `panic` in `engine_core.js`).
