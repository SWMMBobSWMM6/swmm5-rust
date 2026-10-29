# swmm5-rust

**EPA SWMM 5.2.4 translated into safe Rust, byte-identical to EPA's engine.**

The engine crate carries `#![forbid(unsafe_code)]`: the compiler checks every array index and every reference, and an `unsafe` block anywhere in it is a build error. It is held to EPA SWMM 5.2.4's output: on every gated model where EPA's own behaviour is defined, the binary `.out` file is identical byte for byte and the `.rpt` report is identical byte for byte after removing the three lines that print the wall clock (see [Results](#results) for which models, and for the few runs still inconclusive).

| | |
|---|---|
| Reference | EPA SWMM 5.2.4, [USEPA/Stormwater-Management-Model](https://github.com/USEPA/Stormwater-Management-Model/tree/v5.2.4) tag `v5.2.4` |
| Safe engine | `crates/engine`: 63 files, about 56,000 lines, no `unsafe` |
| Unsafe code | in the engine and its interface, `crates/capi` only: 7 small blocks where C or JavaScript callers hand in pointers. The test harnesses that call glibc or the c2rust baseline for comparison use `unsafe` too; they do not ship |
| Gates | see [Results](#results) |
| WebAssembly | `wasm32-wasip1`, runs in the app beside EPA's C and swmm5-cplus |
| App | `dist/swmm5-rust.html`: one file, three engines, with a Doc tab (the essay is also `dist/swmm5-rust-doc.docx`). Built by `app/build.py` |

## Why

C lets a program read the eleventh element of a ten-element array; the result is "undefined", and the program usually carries on with whatever it read. Decks from the OpenSWMM 2026 code review make EPA 5.2.4 do exactly that (see [docs/DEFECTS.md](docs/DEFECTS.md), RS-5 to RS-10). Natively some run on and some crash with an empty report; in WebAssembly the answers change again. swmm5-rust's answer does not change with the platform: it stops at the bad index with a message naming the line, or writes the error report where C's bad access comes only on cleanup (RS-9), or, at the one place the translators chose a value, reads 0.0 where C reads far outside a table (RS-10).

## How it was built

1. **Baseline.** [c2rust](https://github.com/immunant/c2rust) 0.22.1 turned EPA's C into Rust statement for statement (`crates/c2rust-baseline`, 978 functions, every one `unsafe`). `tools/c2rust_stable.py` makes it build on stable Rust; `tools/c2rust_casts.py` routes its float-to-integer casts through `ccast` (below). It matches EPA on all 221 smoke and coverage models (the six memory-error decks included: a statement-for-statement copy goes wrong where the C goes wrong) and never ships: it is the reference the safe engine was checked against.
2. **Skeleton.** Data types, one `Project` structure that owns the whole model (every global, and every module's file-scope statics in a state structure of its own), and a stub for every function.
3. **Translation.** The 54 modules, in ten parallel batches, all under one rule book: [TRANSLATING.md](TRANSLATING.md). Literal translation, same names, same statement order, arithmetic bit for bit, strings as bytes, linked lists as vectors in walk order.
4. **Integration and gates.** `tools/gate.py` (native) and `tools/wasm_gate3.py` (WebAssembly, three engines) compare outputs byte for byte. `crates/difftest/src/bin/lockstep.rs` steps the c2rust baseline and the safe engine side by side and reports the first state variable whose bits differ.

Two support modules make byte identity possible:

* `ccast`: float-to-integer conversion as x86-64 does it. In C an out-of-range conversion is undefined; x86 gives `i32::MIN`, Rust's `as` saturates. gage.c line 251 converts about 8.6e14 for any one-point rain series, and the difference decides whether WARNING 09 is written.
* `cio`: `printf`, `sscanf`, `strtod`, `atoi` and `FILE` in safe Rust, checked against glibc on about 11 million inputs (`crates/engine/tests/cio_vs_libc.rs`).

## Results

<!-- RESULTS -->
Written by `app/build.py` from `results/`; the app's Verify tab and the Doc essay are built from the same files.

| Gate (EPA 5.2.4 as reference) | Models | Identical | Undefined in C | Different | Inconclusive |
|---|---|---|---|---|---|
| Native: smoke and coverage set | 221 | 215 | 6 | 0 | 0 |
| Native: every corpus model with FILLED_CIRCULAR | 68 | 59 | 0 | 0 | 9 |
| Native: corpus set (3,759 models) | 3,759 | 3,738 | 15 | 0 | 6 |
| Native, engine as it stands: the 221 set and every other gated model with control rules | 415 | 409 | 6 | 0 | 0 |
| WebAssembly: swmm5-rust vs EPA, same libc | 221 | 215 | 6 | 0 | 0 |
| WebAssembly: swmm5-cplus vs EPA (for reference) | 221 | 215 | 6 | 0 | 0 |
| Platform control: EPA wasm vs EPA native | 221 | 194 | 6 | 21 | 0 |

"Identical" means the `.out` file byte for byte and the `.rpt` byte for byte after removing the three wall-clock lines. "Undefined in C": EPA reads or writes memory it does not own (an explicit list of decks, plus any model that crashes EPA with a signal); see docs/DEFECTS.md. "Inconclusive": a run hit the time or memory cap.

Inconclusive: `OWA_EXTRAN/longTS_extran1.inp`, `OpenSWMM_Code_Review_2026/4-io/IO-02-time-options-accept-nan-inf-huge/IO-02_end-time-1e30.inp` (IO-02 in the code review: END_TIME 1e30 hours puts the end 4.2e28 days after the start, so EPA's own run does not end; not rerun), `SWMM5_NCIMM/1220_Nodes.inp`, `SWMM5_NCIMM/12334_Nodes.inp`, `SWMM5_NCIMM/2348_Nodes.inp`, `SWMM5_NCIMM/235_Subs_RDII.inp`, `SWMM5_NCIMM/23865_Nodes.inp`, `SWMM5_NCIMM/3363_Nodes.inp`, `SWMM5_NCIMM/4190_Nodes.inp`, `SWMM5_NCIMM/669_Subs_4500_Nodes.inp`, `SWMM5_NCIMM/6715_Nodes_400_Subs.inp`, `SWMM5_NCIMM/8117_Nodes.inp`, `SWMM5_NCIMM/8394_H&H_Elements.inp`, `SWMM5_NCIMM/indy_small.inp`, `Simon_EPA/Session38_100_Control_Rules.inp`

Native run time over 12 models (`tools/bench3.py`, CPU seconds, median of 5 interleaved runs of EPA, C++, Rust, EPA): swmm5-rust 1.12× EPA at the median, swmm5-cplus 0.94×; on ten of the twelve it is slower by more than EPA differs from itself on the same model; on two the difference is inside that noise (Rust).

Instruction counts (callgrind, `tools/profile.py`, results/profile.json): swmm5-rust executes 23% more on `manifold_pumps.inp`, 20% more on `46_Subs.inp`, 26% more on `10_Subs_Continuous.inp`, 11% more on `Peter Klaver_WQ_Tracer.inp` than EPA. On `manifold_pumps.inp` 37 percent of the extra is in xsect.c's `lookup()` (35.5 instructions a call in EPA, 62.1 in swmm5-rust, 22,640,867 calls in each).

WebAssembly run time in node over 12 models (`app/bench_wasm.js`): swmm5-rust 1.36× EPA at the median, swmm5-cplus 0.99×; on all twelve it is slower by more than EPA differs from itself on the same model (Rust).
<!-- /RESULTS -->

## Defects found

[docs/DEFECTS.md](docs/DEFECTS.md) has the register (RS-1 to RS-10) with the C as released, and says where the OpenSWMM 2026 code review or Bob's earlier swmm5-rs port found the same fault first. Four are reproduced on purpose so the output still matches EPA: a storage index used as a node index (RS-1), an LID error message that prints its own template (RS-2), the FILLED_CIRCULAR round trip that edits a link's geometry by one bit (RS-3), and out-of-range float-to-int conversions (RS-4). [docs/UNDEFINED.md](docs/UNDEFINED.md) lists every other place the translators noted that the C is undefined, and what the Rust does there.

### Departures from the C

* `getAbsolutePath()`: C copies up to PATH_MAX bytes into the 260-byte `InpDir`; the Rust truncates to the buffer. (EPA's own runswmm aborts under glibc fortify on relative paths, so the gate passes absolute ones.)
* Scratch files: C's Linux branch creates `swmmXXXXXX` in the current directory. The Rust uses the TEMPDIR option if set, as EPA's Windows branch does, otherwise the system temporary directory, and on WASI the current directory. The name never reaches the outputs.
* Running out of memory: where C checks an allocation and reports ERROR 101 when it fails, a Rust `Vec` that cannot be allocated stops the program with "memory allocation failed" and no report. Given the memory, the two agree: `Hydrology/sewer_rdii_problem.inp` asks both engines for 3.4 GB of RDII rainfall history, so under the gate's 3 GiB address-space cap EPA reported ERROR 101 and the Rust stopped; rerun without the cap (`results/rs_memcap_rerun.jsonl`) the outputs are byte-identical.
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
