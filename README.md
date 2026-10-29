# swmm5-rust

**EPA SWMM 5.2.4 translated into safe Rust, byte-identical to EPA's engine.**

The engine crate carries `#![forbid(unsafe_code)]`: the compiler checks every array index and every reference, and an `unsafe` block anywhere in it is a build error. Nothing it computes differs from EPA SWMM 5.2.4. For every model where EPA's own behaviour is defined, the binary `.out` file is identical byte for byte and the `.rpt` report is identical byte for byte after removing the three lines that print the wall clock.

| | |
|---|---|
| Reference | EPA SWMM 5.2.4, [USEPA/Stormwater-Management-Model](https://github.com/USEPA/Stormwater-Management-Model/tree/v5.2.4) tag `v5.2.4` |
| Safe engine | `crates/engine`: 63 files, about 56,000 lines, no `unsafe` |
| Unsafe code | `crates/capi` only: 7 small blocks where C or JavaScript callers hand in pointers |
| Native gate | see [Results](#results) |
| WebAssembly | `wasm32-wasip1`, runs in the app beside EPA's C and swmm5-cplus |
| App | `dist/swmm5-rust.html`: one file, three engines, with a Doc tab (also `dist/swmm5-rust-doc.docx`) |

## Why

C lets a program read the eleventh element of a ten-element array; the result is "undefined", and the program usually carries on with whatever it read. Six decks from the OpenSWMM 2026 code review make EPA 5.2.4 do exactly that (see [docs/DEFECTS.md](docs/DEFECTS.md), RS-5 to RS-9). Natively some run on and some crash with an empty report; in WebAssembly the answers change again. swmm5-rust stops at the bad index with a message naming the line, on every platform.

## How it was built

1. **Baseline.** [c2rust](https://github.com/immunant/c2rust) 0.22.1 turned EPA's C into Rust statement for statement (`crates/c2rust-baseline`, 978 functions, every one `unsafe`). `tools/c2rust_stable.py` makes it build on stable Rust; `tools/c2rust_casts.py` routes its float-to-integer casts through `ccast` (below). It matches EPA on all 221 smoke and coverage models and never ships: it is the reference the safe engine was checked against.
2. **Skeleton.** Data types, one `Project` structure that owns the whole model (every global, and every module's file-scope statics in a state structure of its own), and a stub for every function.
3. **Translation.** The 54 modules, in ten parallel batches, all under one rule book: [TRANSLATING.md](TRANSLATING.md). Literal translation, same names, same statement order, arithmetic bit for bit, strings as bytes, linked lists as vectors in walk order.
4. **Integration and gates.** `tools/gate.py` (native) and `tools/wasm_gate3.py` (WebAssembly, three engines) compare outputs byte for byte. `crates/difftest/src/bin/lockstep.rs` steps the c2rust baseline and the safe engine side by side and reports the first state variable whose bits differ.

Two support modules make byte identity possible:

* `ccast`: float-to-integer conversion as x86-64 does it. In C an out-of-range conversion is undefined; x86 gives `i32::MIN`, Rust's `as` saturates. gage.c line 251 converts about 8.6e14 for any one-point rain series, and the difference decides whether WARNING 09 is written.
* `cio`: `printf`, `sscanf`, `strtod`, `atoi` and `FILE` in safe Rust, checked against glibc on about 11 million inputs (`crates/engine/tests/cio_vs_libc.rs`).

## Results

Filled in from `results/` when the gates finish; the app's Verify tab and the Doc essay are built from the same files.

## Defects found

[docs/DEFECTS.md](docs/DEFECTS.md) has the register (RS-1 to RS-9) with the C as released. Four are reproduced on purpose so the output still matches EPA: a storage index used as a node index (RS-1), an LID error message that prints its own template (RS-2), the FILLED_CIRCULAR round trip that edits a link's geometry by one bit (RS-3), and out-of-range float-to-int conversions (RS-4). [docs/UNDEFINED.md](docs/UNDEFINED.md) lists every other place the translators noted that the C is undefined, and what the Rust does there.

### Deliberate departures that cannot change the output

* `getAbsolutePath()`: C copies up to PATH_MAX bytes into the 260-byte `InpDir`; the Rust truncates to the buffer. (EPA's own runswmm aborts under glibc fortify on relative paths, so the gate passes absolute ones.)
* Scratch files: C's Linux branch creates `swmmXXXXXX` in the current directory. The Rust uses the TEMPDIR option if set, as EPA's Windows branch does, otherwise the system temporary directory, and on WASI the current directory. The name never reaches the outputs.
* `controls_delete()` after an early input error (CRASH-08): C walks a NULL array and crashes; the Rust finds an empty vector and writes the error report.

## Building

```sh
cargo build --release                      # native: target/release/runswmm-rs
target/release/runswmm-rs model.inp model.rpt model.out
tools/build_wasm.sh                        # build/wasm/swmm5rs.wasm (see the script for toolchain notes)
cargo test -p swmm5-rust                   # cio against glibc
cd app && python3 build.py                 # the app, from results/ and the three wasm engines
```

The C interface (`crates/capi`) exports EPA's `swmm_*` functions and a handle-based twin of each (`swmm5rs_create`, `swmm5rs_open`, `swmm5rs_step`, ...), so one process or one WebAssembly instance can hold any number of models.

## AI use

This repository was produced with Claude, Anthropic's AI model (Claude Opus 5.5), in one session on 28 and 29 October 2026, at the request of Robert (Bob) Dickinson, who chose the goal (safe Rust, byte-identical to EPA 5.2.4) and supplies the model corpus ([SWMMBobSWMM6/1729-SWMM5-Models-2030](https://github.com/SWMMBobSWMM6/1729-SWMM5-Models-2030)). Claude ran c2rust and wrote the passes over its output, the skeleton, the rule book and the integration; ran the 54-module translation as ten batches of parallel agents; wrote `ccast`, `cio` and their tests, the C interface, the WebAssembly build, the gates, the lockstep tool, the app and the essay. The work was checked mechanically by the byte-for-byte gates, not by the AI's judgment. It has not yet been reviewed line by line by a person.

## License and credit

EPA SWMM is a work of the U.S. Environmental Protection Agency, in the public domain. Version 5.2.4's engine is Lewis Rossman's design; this translation keeps every algorithm as he wrote it.
