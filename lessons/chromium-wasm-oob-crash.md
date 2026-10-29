An out-of-bounds access in C compiled to WebAssembly crashed the whole headless Chromium renderer, while node trapped cleanly.

CRASH-04 (quoted "Shape" curve) makes EPA's C and swmm5-cplus write past a zero-length array. In node both trap with "memory access out
of bounds". In headless Chromium (Playwright, this container) the same run, in a Worker or on the main thread, killed the page
("Target crashed"), with and without `--js-flags=--no-wasm-trap-handler`. The app therefore runs that deck only in swmm5-rust and shows
the node result for the C engines. Not tested in desktop Chrome or Firefox; treat any known-UB deck as able to take the tab down.
