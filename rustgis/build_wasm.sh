#!/bin/bash
# swmm5-rust as a RustGIS plugin module: wasm32-unknown-unknown, no imports but RustGIS's three.
#
# rustup's prebuilt wasm32 standard library could not be downloaded on the build machine, so std
# is compiled from source: Ubuntu 24.04's rustc-1.91 + rust-1.91-src, with the `dlmalloc`
# dependency that Debian's packaging strips (std's allocator on wasm32-unknown-unknown) restored
# in a private copy of the library sources, linked by the system wasm-ld. With rustup and its
# wasm32-unknown-unknown target installed, plain
#   cargo build --release --target wasm32-unknown-unknown --lib
# does the same.
#
# Needs (Ubuntu 24.04): rustc-1.91 cargo-1.91 rust-1.91-src lld
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
SRC=/opt/rustsrc191u/library
if [ ! -d "$SRC" ]; then
  mkdir -p /opt/rustsrc191u && cp -r /usr/src/rustc-1.91.1/library /opt/rustsrc191u/
  python3 - "$SRC/std/Cargo.toml" <<'PY'
import sys
p = sys.argv[1]; s = open(p).read()
if 'dlmalloc' not in s:
    add = ("\n[target.'cfg(any(all(target_family = \"wasm\", target_os = \"unknown\"), target_os = \"xous\", target_os = \"vexos\", all(target_vendor = \"fortanix\", target_env = \"sgx\")))'.dependencies]\n"
           "dlmalloc = { version = \"0.2.10\", features = ['rustc-dep-of-std'] }\n")
    k = s.index('\n[dev-dependencies]'); s = s[:k] + '\n' + add + s[k:]
    open(p, 'w').write(s)
PY
fi
cd "$HERE"
export RUSTC=/usr/bin/rustc-1.91
export CARGO_TARGET_DIR="$HERE/target/wasm191"
# (source paths in panic messages are written relative to the repository)
export RUSTFLAGS="-C linker=wasm-ld --remap-path-prefix=$(cd "$HERE/.." && pwd)=swmm5-rust --remap-path-prefix=$HOME/.cargo/registry/src=crates.io --remap-path-prefix=$HOME/.cargo/git/checkouts=git"
export __CARGO_TESTS_ONLY_SRC_ROOT=$SRC
export RUSTC_BOOTSTRAP=1
/usr/bin/cargo-1.91 build --release --lib --target wasm32-unknown-unknown -Zbuild-std=std,panic_abort
mkdir -p "$HERE/dist"
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/swmm5_rust_rustgis.wasm" "$HERE/dist/plugin.wasm"
ls -la "$HERE/dist/plugin.wasm"
