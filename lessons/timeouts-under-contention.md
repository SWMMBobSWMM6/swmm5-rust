A fixed per-model timeout turns into false "inconclusive" results when several gates share two cores; rerun timeouts alone with a long cap.

The FILLED_CIRCULAR gate ran beside the WebAssembly gate on a two-core machine. Large NCIMM models that EPA finished in 270 s hit the
300 s cap in the Rust run. Those are reruns, not results: rerun them alone, one job, with a cap of an hour, before quoting the set.

Background jobs in this environment are stopped after two hours, so a long rerun has to be resumable: `gate.py` skips models
already in its results file, and a stopped run loses only the models in flight. The one-hour-cap rerun was stopped once at
10 of 16 and resumed; the in-flight `3363_Nodes.inp` started over.
