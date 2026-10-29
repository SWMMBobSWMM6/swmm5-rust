#!/bin/bash
# The two C/C++ reference binaries the gates and the timing ran, rebuilt from source.
# Both rebuilds reproduce the binaries in ref/ byte for byte (SHA-256 in results/binaries.sha256),
# which is how the flags below are known rather than remembered. GCC 13.3.0 (Ubuntu 24.04).
#
#   EPA SWMM 5.2.4   USEPA/Stormwater-Management-Model tag v5.2.4 (7952ca8)
#   swmm5-cplus      SWMMBobSWMM6/swmm5-cplus 4f8e1ae
#
#   tools/build_refs.sh EPA_CHECKOUT CPLUS_CHECKOUT OUTDIR
set -e
EPA=$1; CP=$2; OUT=$3
mkdir -p "$OUT"
# EPA: gcc -O2, no LTO, no OpenMP, Ubuntu's default fortify (runswmm then needs absolute paths)
gcc -O2 -I"$EPA/src/solver/include" -I"$EPA/src/solver" -o "$OUT/runswmm" "$EPA"/src/solver/*.c "$EPA/src/run/main.c" -lm
# swmm5-cplus: its own build script, -O2 -flto, EPA's main.c compiled as C
( cd "$CP" && MAINC=src/run/main.c bash tools/build_native.sh src/engine src/api "$OUT/cplus" "-O2 -flto" )
cp "$OUT/cplus/runswmm_cp" "$OUT/runswmm-cplus"
# swmm5-rust: cargo build --release (opt-level 2, no LTO) at results/engine_commit.txt
sha256sum "$OUT/runswmm" "$OUT/runswmm-cplus"
