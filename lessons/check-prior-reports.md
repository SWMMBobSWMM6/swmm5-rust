Before calling a defect new, search the OpenSWMM 2026 code review in the corpus (and Bob's earlier comparisons) for the function name.

The storage-index defect (RS-1) found during translation was already NUM-55 in `1729-SWMM5-Models-2030/OpenSWMM_Code_Review_2026`,
with a second fault on the same lines (metric units) that the translation had not noticed. The gage.c:251 cast (RS-4) is CRASH-21
there and was the cause of Bob's 25-report difference in his earlier swmm5-rs port. A `grep -rl <function> --include=README.md` over
the review folder takes a second. docs/DEFECTS.md now carries an "also reported as" line for each entry.
