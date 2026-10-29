A gate's resource caps can turn into false differences, not only false timeouts: when a run under a cap disagrees, rerun it without the cap before calling it a difference.

The native gate runs each engine with a 3 GiB address-space cap. `Hydrology/sewer_rdii_problem.inp` asks for 3.4 GB of RDII
rainfall-history arrays (`pastRain`, `pastMonth`, sized by `getMaxPeriods()` in rdii.c) in both engines. Under the cap EPA's
checked `calloc` failed and it wrote "ERROR 101: memory allocation error"; the Rust's `vec!` failed and the program aborted with
"memory allocation failed" (exit -6). The gate recorded a difference. `ulimit -v` reproduced it; `valgrind --tool=massif`
named the allocation; rerun without the cap (`gate.py --mem 0`), the outputs are byte-identical. What remains is a real but
narrow departure, documented in the README: out of memory, C reports and Rust stops.

Why 3.4 GB: `getMaxPeriods()` in rdii.c keeps `tBase / rainInterval + 1` past rainfall values per unit hydrograph, and
`getRainInterval()` starts from the wet step. The model has 613 hydrographs whose long-term response (T = 900 h, K = 10) has a
9,900-hour base; at its 1-minute wet step that is 594,001 periods each, 8 bytes in `pastRain` and 1 in `pastMonth`, for a run
that lasts 4 days. Of the model's 1,839 unit-hydrograph rows only 60 have a nonzero response ratio R; `getMaxPeriods()` sizes the history
from `tBase` alone, so most of that memory holds rainfall that is later multiplied by zero.
