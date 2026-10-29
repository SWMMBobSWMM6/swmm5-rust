A fixed per-model timeout turns into false "inconclusive" results when several gates share two cores; rerun timeouts alone with a long cap.

The FILLED_CIRCULAR gate ran beside the WebAssembly gate on a two-core machine. Large NCIMM models that EPA finished in 270 s hit the
300 s cap in the Rust run. Those are reruns, not results: rerun them alone, one job, with a cap of an hour, before quoting the set.
