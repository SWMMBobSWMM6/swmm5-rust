# results/

Every number in the app and the essay is computed by `app/build.py` from these files.

## Model lists

| File | What |
|---|---|
| `smoke.txt`, `coverage.txt` | 40 smoke models and 187 coverage models (every cross-section shape, routing method and feature found in the corpus, error decks, the OpenSWMM 2026 code-review decks) |
| `wasm_list.txt` | the 221 distinct models of smoke + coverage: the "smoke and coverage set" |
| `filled68.txt` | every corpus model with a FILLED_CIRCULAR conduit |
| `corpus_set.txt` | the 3,759 models swmm5-cplus was gated on: every corpus model under 125 KB plus a seeded sample of 72 larger ones |
| `tier0.txt` ... `tier5.txt` | the 221 split by the features they use, simplest first (used while integrating) |
| `bench_list.txt` | the 12 models timed (the swmm5-cplus speed-study models plus four EPA examples) |

The corpus is [SWMMBobSWMM6/1729-SWMM5-Models-2030](https://github.com/SWMMBobSWMM6/1729-SWMM5-Models-2030).

## Gates (one JSON line per model)

Native gate records (`tools/gate.py`): `rc_ref`/`rc_test` (exit code, negative = signal, `timeout`), `t_ref`/`t_test` (seconds), `out_same`, `rpt_same` (after removing the three wall-clock lines), `out_sha`.

| File | Reference | Engine tested | |
|---|---|---|---|
| `c2rust_smoke.jsonl`, `c2rust_cov.jsonl` | EPA 5.2.4 | c2rust baseline before ccast | 12 coverage reports differ by WARNING 09 (gage.c line 251) |
| `c2rust_casts_221.jsonl` | EPA 5.2.4 | c2rust baseline with ccast | 221/221 |
| `rs_221_v1.jsonl` | EPA 5.2.4 | swmm5-rust, first integrated build | 215/221 |
| `rs_221_v2.jsonl` | EPA 5.2.4 | swmm5-rust, gated binary (`engine_commit.txt`) | 215/221; the 6 others are memory-error decks |
| `rs_filled68_beforefix.jsonl` | EPA 5.2.4 | swmm5-rust before the FILLED_CIRCULAR fix | partial; one model differed |
| `rs_filled68.jsonl` | EPA 5.2.4 | gated binary | the FILLED_CIRCULAR set |
| `rs_corpus.jsonl` | EPA 5.2.4 | gated binary | the 3,759-model corpus set |
| `rs_timeouts_rerun.jsonl` | EPA 5.2.4 | the engine as it stands (`ref/runswmm-rs-head`) | models that hit the 300 s cap while gates shared the machine, rerun two at a time on an otherwise idle machine with a one-hour cap |

WebAssembly gate records (`tools/wasm_gate3.py`): `rs_out`/`rs_rpt` (swmm5-rust wasm vs EPA wasm), `cp_out`/`cp_rpt` (swmm5-cplus wasm vs EPA wasm), `plat_out`/`plat_rpt` (EPA wasm vs EPA native: the platform control), `rs_native_out`/`rs_native_rpt` (swmm5-rust wasm vs EPA native), and the run record of each engine.

| File | |
|---|---|
| `wasm3_221_beforewasifix.jsonl` | first run; found the WASI panics (`std::env::temp_dir`, `std::process::id`) on 9 models |
| `wasm3_221.jsonl` | rerun with the fixed engine (`wasm_engine_commit.txt`) |

## Other evidence

| File | |
|---|---|
| `binaries.sha256` | SHA-256 of every binary and wasm module in `ref/` (check with `sha256sum -c`): EPA and swmm5-cplus native (rebuilt byte for byte by `tools/build_refs.sh`), swmm5-rust native as gated (`runswmm-rs-gate`, d37bbc5) and as it stands (`runswmm-rs-head`), the three WebAssembly engines, `swmm5rs-gate.wasm` (the e88b4e0 run) and `swmm5rs-final.wasm` (the final run, ecbd2df). Earlier builds of `swmm5rs-gate.wasm` were overwritten and are not kept |
| `engine_commit.txt`, `wasm_engine_commit.txt` | commits the gated native binary (d37bbc5) and the gated wasm module (ecbd2df, the engine as it stands) were built from. Between the two, the engine changed in one place that runs natively: a control rule that reads LINK VELOCITY now calls the same function C calls (2dcf313); the rest is WASI-only code and removed `mut`s |
| `head_check.txt`, `rs_head_check.jsonl` | the native binary built from the engine as it stands, gated on the 221 set and every other gated model with control rules, to cover that change; the ten rule models that hit the time cap in the gated runs are in the timeouts rerun, which uses the same binary |
| `filled_prefix.json` | the FILLED_CIRCULAR drift before the fix: 13 of 7,848 .out values and one time of maximum depth |
| `lockstep_prefix_filled.log` | lockstep on the pre-fix engine: first difference at step 29, conduit C0, new depth, last bit |
| `lockstep_samples.log` | lockstep on the current engine: no state difference on three sample models |
| `unit_tests.log` | cio against glibc (8.2 million printf float conversions and more) and the difftest suite (14 tests) |
| `bench_native.json`, `bench_wasm.json` | timing: interleaved EPA, C++, Rust, EPA runs with the EPA-against-EPA noise floor (`tools/bench3.py`, `app/bench_wasm.js`) |
| `profile.json`, `profile_work/` | instruction counts per function (own and inclusive), calls per function and every caller-to-callee call count, EPA against swmm5-rust, from callgrind (`tools/profile.py`); the `.cg` files are callgrind's own output |
| `lookup_disasm.txt` | `xsect.c`'s `lookup()` as compiled in both binaries: EPA's one `cvttsd2si` against the Rust's twelve instructions for the same conversion, and the Rust's table-read checks |
| `memcap_models.txt`, `rs_memcap_rerun.jsonl` | models that ask for more than the gate's 3 GiB address-space cap, rerun without it (`gate.py --mem 0`): under the cap EPA reports ERROR 101 and the Rust stops with "memory allocation failed" |
| `nocap_models.txt`, `rs_nocap_rerun.jsonl` | models EPA itself cannot finish inside the one-hour cap, run with no time cap, EPA and the engine as it stands side by side, one core each (`gate.py --timeout 0 --pair`) |
| `rs_interface_serial.jsonl` | every gated model that SAVEs or USEs an interface file, rerun one at a time: run in parallel, two models can share one file in their folder (the 1000-year models do); these records replace the parallel ones |

The C and C++ reference binaries in `ref/` are rebuilt byte for byte by `tools/build_refs.sh`, which records their compiler flags.
