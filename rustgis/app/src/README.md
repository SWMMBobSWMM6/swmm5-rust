# Page sources

`build.py` builds `../swmm5-rust-rustgis.html` (standalone, with doctype, charset and viewport) and an
artifact copy (bare, for claude.ai) from `template.html`, `app.js` and these inlined modules:

* `verify.js`: the Verify tab's field-by-field attribute check against EPA's report, with its
  planted-defect self-test. `verify_attrs.py` is the separately written Python checker behind the
  page's static numbers; the two agree model by model on the six demos.
* `../../../app/rpt_compare.js` (the swmm5-rust app's own `.out` reader, `parseOut`): the time series.
* `../../../app/engine_core.js`: the in-memory WASI host that runs EPA SWMM 5.2.4 (wasm32-wasi) for Verify.

`build.py` also reads the evidence it reports (census, RustGIS runs, fuel, attribute checks) from the
paths at its top; those files are the build session's and are not in this repository, so the
numbers on the page and in the Doc are what those runs measured.

Tests:

* `node test_verify.js`: old and new checkers on the six demos, with planted defects.
* `python3 test_verify.py`: the Python checker's planted-defect test.
* `node test_page.js page.html outdir`: the page headless: runs, cancel, superseded runs, an external
  rainfall file named by a Windows path (`fixtures/`), Verify, the run bundle, an 8,593-row table,
  keyboard tabs, both themes, phone width.
