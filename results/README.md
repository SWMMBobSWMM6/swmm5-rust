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
| `rs_timeouts_rerun.jsonl` | EPA 5.2.4 | gated binary | models that hit the 300 s cap while gates shared the machine, rerun alone with a long cap |

WebAssembly gate records (`tools/wasm_gate3.py`): `rs_out`/`rs_rpt` (swmm5-rust wasm vs EPA wasm), `cp_out`/`cp_rpt` (swmm5-cplus wasm vs EPA wasm), `plat_out`/`plat_rpt` (EPA wasm vs EPA native: the platform control), `rs_native_out`/`rs_native_rpt` (swmm5-rust wasm vs EPA native), and the run record of each engine.

| File | |
|---|---|
| `wasm3_221_beforewasifix.jsonl` | first run; found the WASI panics (`std::env::temp_dir`, `std::process::id`) on 9 models |
| `wasm3_221.jsonl` | rerun with the fixed engine (`wasm_engine_commit.txt`) |

## Other evidence

| File | |
|---|---|
| `binaries.sha256` | SHA-256 of every binary and wasm module the gates ran |
| `engine_commit.txt`, `wasm_engine_commit.txt` | commits the gated native binary and wasm module were built from; later commits change only WASI code paths (native) or nothing in the engine |
| `filled_prefix.json` | the FILLED_CIRCULAR drift before the fix: 13 of 7,848 .out values and one time of maximum depth |
| `lockstep_prefix_filled.log` | lockstep on the pre-fix engine: first difference at step 29, conduit C0, new depth, last bit |
| `lockstep_samples.log` | lockstep on the current engine: no state difference on three sample models |
| `unit_tests.log` | cio against glibc (8.2 million printf float conversions and more) and the difftest suite (14 tests) |
| `bench_native.json`, `bench_wasm.json` | timing: interleaved EPA, C++, Rust, EPA runs with the EPA-against-EPA noise floor |
