#!/bin/bash
# swmm5-rust -> wasm32-wasip1 reactor (crates/capi as a cdylib).
#
# This machine cannot download Rust's prebuilt wasm32-wasip1 standard library,
# so std is compiled from source: Ubuntu's rustc-1.91 + rust-1.91-src, with the
# `wasi` dependency that Debian's packaging strips restored in a private copy
# of the library sources, linked by the system wasm-ld against Ubuntu's
# wasi-libc (the same libc.a the EPA and swmm5-cplus wasm builds use).
#
# Needs (Ubuntu 24.04): rustc-1.91 cargo-1.91 rust-1.91-src wasi-libc lld
set -e
HERE=$(cd "$(dirname "$0")/.." && pwd)
OUT=${1:-$HERE/build/wasm}
SRC=/opt/rustsrc191/library
if [ ! -d "$SRC" ]; then
  mkdir -p /opt/rustsrc191 && cp -r /usr/src/rustc-1.91.1/library /opt/rustsrc191/
  python3 - "$SRC/std/Cargo.toml" <<'PY'
import sys
p = sys.argv[1]; s = open(p).read()
if 'target_os = "wasi"' not in s:
    add = "\n[target.'cfg(target_os = \"wasi\")'.dependencies]\nwasi = { version = \"0.11.0\", features = ['rustc-dep-of-std'], default-features = false }\n"
    k = s.index('\n[features]'); s = s[:k] + '\n' + add + s[k:]
    open(p, 'w').write(s)
PY
fi
cd "$HERE"
export RUSTC=/usr/bin/rustc-1.91
export CARGO_TARGET_DIR="$HERE/target/wasm191"
export RUSTFLAGS="-C linker=wasm-ld -C link-self-contained=no -L /usr/lib/wasm32-wasi -C link-arg=--export=swmm5rs_malloc -C link-arg=--export=swmm5rs_free"
export __CARGO_TESTS_ONLY_SRC_ROOT=$SRC
export RUSTC_BOOTSTRAP=1
/usr/bin/cargo-1.91 build --release -p swmm5-rust-capi --target wasm32-wasip1 \
  -Zbuild-std=std,panic_abort --config 'profile.release.panic="abort"' \
  --config 'profile.release.opt-level=2' --config 'profile.release.lto=true' --config 'profile.release.codegen-units=1'
mkdir -p "$OUT"
cp "$CARGO_TARGET_DIR/wasm32-wasip1/release/swmm5rs.wasm" "$OUT/swmm5rs.wasm"
ls -la "$OUT/swmm5rs.wasm"
