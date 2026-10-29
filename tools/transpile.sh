#!/bin/bash
# Regenerate crates/c2rust-baseline/src/engine from EPA SWMM 5.2.4.
#   tools/transpile.sh EPA_CHECKOUT   (a clone of USEPA/Stormwater-Management-Model at tag v5.2.4)
# Needs c2rust 0.22.1 (cargo install --locked c2rust; LLVM/libclang 18 dev files).
set -e
EPA=$(realpath "$1"); HERE=$(cd "$(dirname "$0")/.." && pwd)
W=$(mktemp -d); cp -r "$EPA/src/solver" "$W/solver"; cd "$W/solver"
python3 - <<'PY'
import json,glob,os
d=os.getcwd()
json.dump([{"directory":d,"file":f,"arguments":["clang","-O2","-Iinclude","-c",f]} for f in sorted(glob.glob("*.c"))],
          open("compile_commands.json","w"),indent=1)
PY
c2rust transpile --output-dir "$W/out" compile_commands.json
rm -rf "$HERE/crates/c2rust-baseline/src/engine"
python3 "$HERE/tools/c2rust_stable.py" "$W/out/src" "$HERE/crates/c2rust-baseline/src/engine"
cd "$HERE"
cargo clippy --release -p swmm5-c2rust --message-format=json -- -A clippy::all -W clippy::cast_possible_truncation 2>/dev/null > "$W/casts.json" || true
python3 tools/c2rust_casts.py "$W/casts.json" .
cargo build --release -p swmm5-c2rust
