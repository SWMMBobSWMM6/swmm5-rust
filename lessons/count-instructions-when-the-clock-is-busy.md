When the machine is too busy for a stopwatch, count instructions instead: callgrind's counts do not depend on load, and its call counts check the translation too.

Two cores were busy with gates for hours, so run times measured then would have been noise. `tools/profile.py` runs EPA and
swmm5-rust under valgrind's callgrind and compares instruction counts function by function. Four things came out of it that a
stopwatch could not have given:

* Where the extra work is. On `manifold_pumps.inp`, 37% of swmm5-rust's extra instructions are in `xsect.c`'s 34-line `lookup()`:
  35.5 instructions a call in EPA, 62.1 in the Rust, the same 22,640,867 calls in each. The Rust converts float to int the x86 way
  (`ccast`) and checks every table read.
* A cost that is not safety: the Rust output writer allocated a small vector per object per report step (2.18 million allocations
  against EPA's 6,834 on a continuous model).
* A fidelity check for free: on the four profiled models, 204 of the 215 engine functions both builds keep are called the same
  number of times. The other eleven are small functions one compiler inlined at some call sites and the other did not; the
  caller lists show it (for `link_getYcrit`, the function it wraps is called 1,314,659 times in each). The first version of this
  count looked only at the 60 busiest functions and reported one exception; an independent fact-check found the other ten. Count
  over everything, and explain every difference before writing about it.
* Where the lookup's extra instructions are: 12 instructions for C's single `cvttsd2si` (Rust's saturating `as` plus `ccast`
  putting x86's answer back), so about two-fifths is the price of copying C's conversion exactly, not of bounds checks.

An instruction count is not a run time (no cache misses, no branch mispredictions). Quote the stopwatch for speed; use the counts
to say where the time goes.
