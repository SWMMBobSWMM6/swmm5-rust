# swmm5-gpkg

**Run a SWMM 5 model with swmm5-rust and get a GeoPackage of its nodes, links and subcatchments with their results**, for RustGIS or any GIS that reads GeoPackage (RustGIS and GDAL were tried; see below). The file is written by [terrio](https://github.com/opengeos/terrio), which is pure Rust: no GDAL, no SQLite library.

```
swmm5-gpkg run    model.inp --epsg 3435              # runs the model, writes model.rpt, model.out, model.gpkg
swmm5-gpkg export model.inp results.out -o map.gpkg  # from results you already have, any engine
```

```
cargo build --release -p swmm5-gpkg     # the first build compiles terrio and takes several minutes
```

This crate is left out of the workspace's default members, so a plain `cargo build` builds the engine as before and never touches terrio.

## What is in the file

| Table | Rows | Holds |
|---|---|---|
| `nodes` | points | `id`, `type`, `invert_elev`, `full_depth`, `max_*` and `max_*_time` for depth, head, volume, lateral inflow, total inflow, flooding and each pollutant, `hours_flooded`, and the `rpt_*` columns |
| `links` | lines: upstream node, the `[VERTICES]` points, downstream node | `id`, `type`, `from_node`, `to_node`, `length`, `full_depth`, `max_*` and `max_*_time` for flow, depth, velocity, volume, capacity and each pollutant, `min_flow` (below zero: it ran backwards), and the `rpt_*` columns |
| `subcatchments` | polygons | `id`, `rain_gage`, `outlet`, `area`, `max_*` and `max_*_time` for rainfall, snow depth, losses, runoff, groundwater and each pollutant |
| `swmm_run` | key and value | the model, the engine version, flow units, report step, periods, first and last report, and how to read the numbers |
| `swmm_fields` | one row per column | the unit and meaning of every attribute column |

Values are in the model's own units (the `.out` says which: CFS and feet, LPS and metres, and so on); `swmm_fields` gives the unit of each column.

### Two kinds of peak

They are different numbers and the file keeps them apart.

* **`max_*`** is the highest value among the reporting periods saved in the `.out`: one value per report step. Any engine's `.out` gives these, so `export` fills them.
* **`rpt_*`** is the engine's own running maximum over every routing step, the number its `.rpt` summary tables print. A `.out` does not hold it. `run` fills these columns by reading the engine's memory after the last step, before the engine prints and frees them; nothing is parsed from the report text. `export` leaves them empty.

On Greenville in US units (5-minute report step, 10-second routing step) 25 of 172 nodes peak higher over the routing steps than in the saved periods. The largest gap is JCT-148: 33.25 ft among the saved periods, 41.48 ft in the report. A map coloured by `max_depth` would understate that node by 8 feet; `rpt_max_depth` does not.

### Coordinates

A SWMM input file does not say what its coordinates are. Give `--epsg N` if you know; without it the layers carry no reference system (terrio records GDAL's "Undefined SRS").

RustGIS 0.7.0 reads a layer with no reference system as longitude and latitude, which is wrong for a model in feet or metres. For RustGIS, give `--epsg`.

### Times

`*_time` columns are ISO 8601 text on the model's clock (`1997-01-01T14:05:00`). They are not GeoPackage DATETIME, because the standard requires a DATETIME to be UTC and a SWMM model has no time zone. As text they still sort in time order.

### What is left out, and said

* A node with no `[COORDINATES]` line, a link with an end node that has none, and a subcatchment with fewer than three `[POLYGONS]` points are not drawn. `swmm5-gpkg` prints each by name, and `swmm_run` records the counts.
* An object the `[REPORT]` section left out of the `.out` is drawn with `has_results` false and empty result columns.
* If the `.out` names an object the `.inp` does not have, the two are not the same model and nothing is written.

## Opened in RustGIS

`tools/open_in_rustgis.py` loads the file in RustGIS 0.7.0 itself (headless, `pip install rustgis`), styles the links by `rpt_max_flow` and the nodes by `rpt_max_depth`, and renders a PNG. On Greenville with `--epsg 3435` RustGIS reports 30 polygons, 172 points and 235 polylines and places them at 41.77 N, 88.15 W, which is Naperville, Illinois.

RustGIS 0.7.0 does not load GeoPackage attribute tables, so `swmm_run` and `swmm_fields` are not shown there (it says so in a notice). GDAL lists them, and QGIS reads GeoPackage through GDAL.

## What has been checked

`tools/check_gpkg.py` reads the GeoPackage with Python's `sqlite3` and decodes the geometry blobs itself, so nothing in the check comes from terrio or from this crate. Five models, 19 November 2026 (the two Greenville models, EPA's Example 1, `PUMP_STATION`, `Inlet Pipe test`):

| Against | What | Result |
|---|---|---|
| the `.inp`, parsed in the check script | 451 node points, 575 link lines with their vertices, 68 subcatchment rings; node and link types, end nodes | exactly equal |
| swmm-toolkit 0.17.0 (OWA's C reader) | 6,155 `max_*` values and the time of each | exactly equal |
| a published run record (`swmm-run-record-repo/demo/greenville/swmm5-rust/record.json`, same `.out` SHA-256) | 5,364 peak values and times | exactly equal |
| the `.rpt` "Reported Max Depth" column, which the engine keeps over saved periods | `max_depth` of 451 nodes | agree to the digits printed |
| the `.rpt` Node Depth, Node Inflow, Node Flooding and Link Flow Summary tables | 4,437 printed numbers for the `rpt_*` columns | agree to the digits printed |
| GDAL 3.12.4 (pyogrio) | opens every layer: same counts, geometry types and reference system | yes |
| terrio's GeoPackage validator, strict, full check (`cargo test`) | the file written from the test model | valid, no warnings |
| RustGIS 0.7.0 | loads the three layers, styles and renders them | yes |

`run` drives the engine through `swmm_open`, `swmm_start`, `swmm_step`, `swmm_end` and `swmm_close`, the calls `swmm_run` makes. On all five models the `.out` it writes is byte-identical to `runswmm-rs`'s, and the `.rpt` is identical apart from the three clock lines.

Negative controls, each on its own copy of the file: one `max_flow` multiplied by 1.0000001, one time of peak moved by a minute, one `rpt_max_depth` raised by 0.02 ft. Each makes the check fail.

## What this does not show

* **The `.inp` map reader is small and new.** It reads `[COORDINATES]`, `[VERTICES]`, `[POLYGONS]`, `[MAP]` and the first three columns of the node, link and subcatchment sections. It is checked against a second parser in the check script on five models, not against a corpus.
* **`rpt_*` covers part of the report.** Node depth, HGL, lateral and total inflow, flooding rate and hours; link flow, velocity and depth ratio. Surcharge, storage, outfall and pump summaries, and the subcatchment runoff summary, are not carried.
* **Summaries only.** There is one row per object, with peaks. Time series for animation are in the `.out`; `swmm-out` reads them, and nothing here writes them to the GeoPackage yet.
* **The reference system is whatever you say it is.** `--epsg` is written as given. Nothing checks that the model's coordinates are in it.
* **SWMM 6 has not been tried.** `export` should read any `.out` in SWMM 5's layout; only swmm5-rust's were tested.

## AI use

This crate was written with Claude, Anthropic's AI model (Claude Fable 5.2), in one session on 19 November 2026, at the request of Robert (Bob) Dickinson, who asked for swmm5-rust results in a GeoPackage through terrio and is responsible for the repository. Claude wrote the input-map reader, the exporter, the engine run loop that captures the maxima, the tests, the check scripts and this text, and ran the checks. The checks are mechanical (exact comparison with swmm-toolkit and a published run record, digit comparison with the engine's report tables, byte comparison of the engine's files, terrio's validator, GDAL, RustGIS); they are not the AI's judgment. No person has yet reviewed the code line by line, and no person has opened the GeoPackage in the RustGIS desktop application.
