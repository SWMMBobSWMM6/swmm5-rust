# swmm-out

**A Rust reader for the binary results (`.out`) file of EPA SWMM 5.** No dependencies, no `unsafe`.

It reads what the file holds: the engine version, the flow units, the names of the reported subcatchments, nodes, links and pollutants, their saved properties (type, invert, depth, length, area), the reporting variables with units, the date stored with every period, and every value. It gives a time series, a snapshot of one moment, or the peaks of everything in one pass.

```rust
use swmm_out::{iso, Kind, OutFile};

let mut f = OutFile::open("model.out")?;
println!("{} {} periods of {} s", f.flow_units(), f.n_periods, f.report_step);

let depth = f.series_by_name(Kind::Node, "J1", "depth")?;   // Vec<f32>, as stored
let dates = f.dates()?;                                     // the time base, one date per value
println!("{} {}", iso(dates[0]), depth[0]);

let peaks = f.stats(Kind::Link)?;                           // peaks[link][variable]
```

From the command line:

```
swmmout info   model.out                 what the file holds
swmmout json   model.out                 the same, as JSON
swmmout series model.out node J1 depth   one time series as CSV
swmmout series model.out sys runoff
swmmout stats  model.out link            peaks of every variable of every link as CSV
swmmout dump   model.out                 every value, one series per line
```

Variable names are the ones the [swmm-run-record](https://github.com/SWMMBobSWMM6/swmm-run-record-repo) JSON uses (`depth`, `total_inflow`, `flooding`, `quality:TSS`, ...), so a peak here and a peak there carry the same key.

## How it reads the file

The layout is in the crate documentation (`src/lib.rs`), taken from `crates/engine/src/output.rs`, which is held byte for byte to EPA 5.2.4's `output.c`. Four choices are worth knowing:

* **Every count comes from the file.** The number of properties and of variables of each kind is read, not assumed, so a file with more variables than SWMM 5.2 writes still reads. A code this reader has no name for is called `code_N`.
* **The type code is an integer.** The engine writes each node's and link's type with `fwrite(&k, sizeof(INT4), ...)` and the other properties as `REAL4`. Read as a float, type 1 (OUTFALL) is 1.4e-45, which truncates to 0 (JUNCTION).
* **The file is walked from byte 0; the closing record is a check, not a map.** EPA writes the three section positions as the low 32 bits of a 64-bit offset, so above 4 GiB they cannot be used to seek. The walked positions are compared with them modulo 2^32, and a mismatch is an error.
* **A run that did not finish is refused unless asked for.** `OutFile::open` wants the closing record. `OutFile::open_partial` (`--partial`) reads the periods that were written in full and sets `complete` to false.

Dates are days since 30 December 1899. `iso()` decodes them as the engine's `datetime.c` does, so a stamp reads as the report file prints it.

## What has been checked

`tools/check_outfile.py` compares the reader with three things it did not come from. On ten `.out` files written by swmm5-rust on 19 November 2026 (this repository's gates hold that engine byte for byte to EPA 5.2.4; EPA's own binary was not rerun for this check):

| Against | What | Result |
|---|---|---|
| swmm-toolkit 0.17.0 (OWA's C output library) | every value of every period: 9,135,216 | identical bits |
| `swmmrr/outfile.py` (the run-record tool's reader) | the same 9,135,216 values, and the dates | identical |
| the engine's own time series tables in the `.rpt` | 649,824 printed numbers and every time stamp, five models | agree to the digits printed |

The models: the two Greenville models (CFS and LPS; 172 nodes, 235 links, 30 subcatchments; pumps, weirs, orifices, an outlet, storage, a divider), EPA's Example 1 (one pollutant), `PUMP_STATION` (GPM), `Inlet Pipe test` (LPS), and four of this repository's gate decks. The largest file is 29.6 MB, 3,000 periods.

Negative controls, so the checks are known to fail when they should: every series shifted by one period (153 series flagged by the toolkit check, 28,421 numbers by the report check), node depth swapped with head (12 series, 4,320 numbers), one bit flipped in one value (that series flagged).

`cargo test -p swmm-out` runs a small model (`tests/data/tiny.inp`) with the engine and checks the reader against what the input fixes: a steady 1 cfs inflow, 1 in/hr on one impervious acre (at most 1.0083 cfs), a 5 cfs pump, the names, types and properties, a file cut mid-period, and a closing record that lies.

## What this does not show

* **Only version 52004 files were read.** SWMM 5.1 writes the same layout and should read; SWMM 5.0 used some variable codes differently and its names here would be wrong. OpenSWMM 6 stamps 60000 on this layout; no such file was tested.
* **No file above 4 GiB was read.** The reader is written for it; it has not been tried.
* **Agreement with swmm-toolkit is agreement between two readers of one layout**, both written from EPA's `output.c`. The report tables are the check that does not share that origin: the engine printed those numbers itself.
* **Peaks are of saved periods.** `stats()` sees one value per report step. The engine's summary tables are kept over every routing step and can be higher (see `crates/gpkg`).
* **Not built for WebAssembly in this session.** The crate uses only `std` and `OutFile::from_bytes` takes a buffer, so it is expected to build with `tools/build_wasm.sh`'s toolchain; that has not been done.

## One parser

Bob's apps have read `.out` files with four separate JavaScript parsers, and there are two Python readers (`app/outread.py` here and `swmmrr/outfile.py` in the run-record tool). This crate is a candidate for the single one: compiled to WebAssembly it could serve the browser apps, and it already serves native Rust. Until that is decided it is a seventh reader, held to the checks above.

## AI use

This crate was written with Claude, Anthropic's AI model (Claude Fable 5.2), in one session on 19 November 2026, at the request of Robert (Bob) Dickinson, who asked for a Rust `.out` reader and is responsible for the repository. Claude wrote the reader, the command-line tool, the tests, the check scripts and this text, and ran the checks. The checks are mechanical (bit comparison with swmm-toolkit and with `swmmrr`, digit comparison with the engine's report tables, negative controls); they are not the AI's judgment. No person has yet reviewed the code line by line.
