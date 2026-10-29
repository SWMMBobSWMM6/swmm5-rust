A copy satisfies the borrow checker and can still be wrong: check whether the C function changes its argument through the pointer.

EPA's `filled_circ_*` geometry functions add the sediment depth to `xsect->yFull`, compute, and subtract it again. The round trip can leave
`yFull` one bit lower, and because C writes through a pointer to `Link[j].xsect`, the link keeps the new value. The first Rust translation
passed copies (the easy answer to "cannot borrow `prj` as mutable more than once") and lost the edit: 13 of 7,848 `.out` values and one
time of maximum depth (01:04 in the Rust against EPA's 01:06) differed on `tests/models/filled_circ_drift.inp`. Fix: copy, call, write back
(`linkXsect_*` getters in `xsect.rs`).

How to catch it next time: when translating a C function that takes `T*`, grep its body for assignments through the pointer before
deciding between a copy and a write-back. The gate catches it only if a test model hits the sizes that drift.
