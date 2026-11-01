# swmm5-rust in RustGIS

A [RustGIS](https://github.com/opengeos/RustGIS) WebAssembly plugin that runs **swmm5-rust**, EPA SWMM 5.2.4 translated into safe Rust, inside RustGIS's plugin sandbox. It adds the model's subcatchments, links, nodes and rain gages as layers whose attributes are the engine's own summary results. It is one module for the RustGIS desktop app, `rustgis-cli` and the web app at [web.rustgis.app](https://web.rustgis.app); it has been run on `rustgis-cli` (see "Tested in").

| | |
|---|---|
| Package | `dist/swmm5-rust-0.1.0.rgplugin` (plugin.json + plugin.wasm) |
| Module | `wasm32-unknown-unknown`, imports only RustGIS's `call`, `result` and `log` |
| Tools | `run-swmm` (run and map), `map-swmm` (map only); command `about` |
| Ribbon | a **SWMM5 Rust** tab |
| Permission | `files` (to read the .inp and any data files you pick) |
| Tested in | `rustgis-cli` 0.5.0 built from source (a5603d1). The desktop and web apps load plugins through the same host code but were not run here. |
| Registry checks | pass (`scripts/package.py` runs opengeos/rustgis-plugins `scripts/build.py`); category Hydrology |

## How it fits in a box with no files

RustGIS runs plugins in wasmi with no files, no clock, no network, a 512 MB memory cap and a fuel budget per call. The engine reads and writes through one type, `cio::CFile`. The new engine feature **`memfs`** (off by default) swaps that type for an in-memory table of named byte vectors, gives the scratch-file namer a fixed seed, passes file names through unchanged, and reads a host-set clock. RustGIS's clock reads zero, so in the report "Analysis begun on" and "Analysis ended on" read 1 January 1970 and "Total elapsed time" reads "< 1 sec". The default build never compiles `memfs.rs`. The feature is not additive: it replaces disk I/O, so never build the command-line engine or the C interface with it (`--all-features` would).

Engine change: `crates/engine/{Cargo.toml, src/cio.rs, src/lib.rs, src/report.rs, src/swmm5.rs}` (+55 −7 lines) and the new `crates/engine/src/memfs.rs`.

Math library: the module links wasi-libc's `libc.a` (`build.rs`), from which the linker takes only what it needs; its link map lists `pow`, `exp`, `log`, `log10`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `round`, `scalbn`, `fmin`/`fmax` and their helpers, `memcmp`, and an empty destructor hook. Without it, Rust's own libm port changes the last bits of `pow`/`exp` and with them some results (swmm_example.inp: C1's maximum velocity 5.38 ft/s against EPA's 5.40). With it, the plugin is byte-identical to EPA SWMM 5.2.4 compiled to WebAssembly with wasi-libc.

## What a run returns

* Layers `<model> subcatchments` (polygons), `<model> links`, `<model> nodes`, `<model> rain gages`, with input attributes and the results from the report's summary tables (Subcatchment Runoff, Node Depth, Node Inflow, Node Surcharge, Node Flooding, Storage Volume, Outfall Loading, Link Flow, Conduit Surcharge, Pumping). Field names carry units: `max_depth_ft`, `peak_runoff_cfs`, `max_vol_1000ft3` (thousands of cubic feet, as the report prints them; `1000m3` in SI).
* A link's `type` is its input section (CONDUIT, PUMP, ORIFICE, WEIR, OUTLET); a run adds `rpt_type`, the report's word for it (CHANNEL for an irregular conduit, DUMMY for a dummy cross section or an outlet).
* Times (`t_max_depth_hr` and the like) are hours from the start of the reporting period (REPORT_START), to the minute the report printed. `hrs_above_full_normal` is the report's "Hours Above Full Normal Flow".
* Objects with nothing to draw (no coordinates, no polygon, no gage symbol) as tables, with the same attributes.
* Messages: continuity errors as reported, the SHA-256 of the output file and of the report (wall-clock lines removed), which data file stood in for which name in the .inp, files the model wrote, any report row not read, and where the coordinates were placed.
* Optionally the report as a table.

Names follow SWMM's own rules: tokens are split on space, tab, carriage return and line feed only (a quoted name may hold spaces), and names match without regard to case. A report row is read from the right, so a name with spaces is kept whole. Every cell must be the number its column promises; a row that is not (two wide numbers run together) is left out whole, the messages count such rows by table and name the first ten.

On the code-review decks where EPA's C reads memory it does not own, the engine stops at the bad index instead. The plugin logs the panic line (for example `panicked at swmm5-rust/crates/engine/src/link.rs:142:20: index out of bounds`) before the module aborts. RustGIS reports only that the tool stopped with an error: it drops what a failed tool logged. Hosts that keep the log, such as the companion HTML page, show the line.

Coordinates: with an EPSG code the layers keep the model's coordinates and declare the system in GeoJSON's `crs` member, and RustGIS reprojects them. The code is checked against the table RustGIS 0.5.0 projects with (crs-definitions 0.6 plus RustGIS's own additions, in `src/epsg.rs`): an unknown code, or one of Esri's six-digit codes, is refused with a reason, because RustGIS would read the coordinates as longitude/latitude without saying so. `[MAP] UNITS Degrees` is read as longitude/latitude. Otherwise the model is drawn around an anchor (default 29.6476 N, 82.3474 W, the University of Florida) on a sphere of radius 6,371,008.8 m: near true size, within about 0.3% of WGS84 at that latitude.

Data files: up to three files the model names (rainfall, inflow, hotstart). Each is found by its own name, in any case, whatever folder the .inp gives. Two data files with the same name are refused, since the model could read only one of them; so is a data file named like the engine's own `model.inp`, `model.rpt` or `model.out`.

## Limits

* A run must fit RustGIS's fuel budget (3×10^10 per tool call). Measured: the 34-node control-rules example uses 56%; EPA's user2 (72 nodes) needs 226% and is stopped. `map-swmm` always fits.
* Summary results only; the output file is hashed, not returned.
* The sandbox has no clock.
* With a known EPSG code that is not the model's, RustGIS draws the layers in the wrong place, and leaves out features it cannot convert without saying so.

## Build

```sh
./build_wasm.sh                          # dist/plugin.wasm (see the script for the toolchain notes)
python3 scripts/package.py --no-build    # dist/swmm5-rust-0.1.0.rgplugin, with the registry's checks
cargo test --release --lib               # the .inp and .rpt readers, the EPSG and data-file checks
cargo run --release --bin plugin-native -- model.inp [--data rain.dat]   # the same code natively (reply on stderr)
```

`build_wasm.sh` compiles std for `wasm32-unknown-unknown` from Ubuntu 24.04's `rust-1.91-src` (restoring the `dlmalloc` dependency Debian strips) because rustup's prebuilt target could not be downloaded on the build machine. With rustup, `cargo build --release --lib --target wasm32-unknown-unknown` does the same (wasi-libc is still needed for the math library; `WASI_LIBC_DIR` points at its `libc.a`).

## Install

The package is attached to the release `rustgis-plugin-v0.1.0` of this repository (SHA-256 `d9559b733678387faa218c871417940919d2225ea1830b274b8fb8340437e9fb`); `scripts/package.py --url <release asset URL>` writes its registry entry to `dist/swmm5-rust.json`.

* Web app: open web.rustgis.app, start from the World Atlas sample (a tool needs a layer on the map), drag the .rgplugin onto the map or use Plugins › Plugin Manager › Install from a file.
* Desktop: the same Plugin Manager, or drop the file on the window.
* Command line: `rustgis-cli run --sample --plugin swmm5-rust-0.1.0.rgplugin --cmd 'gp.run={"tool":"plugin:swmm5-rust/run-swmm","params":{"model":"model.inp"}}'`

## The page and the write-up

* `app/swmm5-rust-rustgis.html`: one self-contained page that runs the same `plugin.wasm` through a JavaScript implementation of RustGIS's plugin interface, checks it against EPA SWMM 5.2.4 for WebAssembly, and offers the .rgplugin, CSV and GeoJSON for download. Its Doc tab is the write-up.
* `doc/swmm5-rust-in-rustgis.docx`: the same write-up as a Word document, with the review's defect register and the measurements.

## AI use

Built with Claude (Anthropic, Claude Opus 5.5) on 31 October and 1 November 2026 at the request of Robert (Bob) Dickinson, SWMM Enablement. Claude wrote the `memfs` feature and the engine edits, this crate, the packaging changes, the build script, and ran the checks: the in-memory engine against the disk engine, the default build before and after the patch on 221 models, the plugin against EPA's WebAssembly and native builds on the same 221 models, runs inside `rustgis-cli` 0.5.0 itself, and an independent reading of EPA's report for every result attribute. A second Claude agent reviewed the code and the claims and found 17 problems, all fixed before release (among them a ponded depth labeled as a volume on models that spell dynamic wave `DW`, and unit names off by a factor of a million). The checks are mechanical, not the AI's judgment. No person has yet reviewed this code line by line.

## License

MIT, as swmm5-rust. EPA SWMM is public domain. `scripts/package.py` is from opengeos/rustgis-plugin-template (MIT). The RustGIS plugin SDK is MIT. `src/epsg.rs` lists codes from crs-definitions (MIT/Apache-2.0) and RustGIS (MIT).
