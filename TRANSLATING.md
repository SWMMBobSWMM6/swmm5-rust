# Translating a SWMM module into safe Rust

swmm5-rust is EPA SWMM 5.2.4 in safe Rust. Its one success test: for every
model, the `.out` file is byte-identical to EPA's, and the `.rpt` file is
byte-identical apart from the three wall-clock lines. Nothing is improved,
nothing is fixed. A translation is correct when it does exactly what the C
does, bugs included.

## Where things are

| | |
|---|---|
| EPA's C (the source of truth) | `/home/claude/work/epa524/src/solver/*.c, *.h` |
| The same C translated by c2rust (statement for statement, unsafe) | `crates/c2rust-baseline/src/engine/<module>.rs` |
| The safe engine | `crates/engine/src/` |
| Shared types (objects.h etc.) | `crates/engine/src/objects.rs` |
| The model (all globals + each module's state) | `crates/engine/src/globals.rs` (`struct Project`) |
| Constants / enums / text.h / keyword tables | `consts.rs`, `enums.rs`, `text.rs`, `keywords.rs` |
| macros.h | `macros.rs` (`MIN`, `MAX`, `ABS`, `SGN`, `SQR`, `SIGN`, `LOG10`, `UCHAR`, `MOD`) |
| printf/sscanf/strtod/atoi/FILE/string.h | `cio.rs` (checked against glibc on ~11 million inputs) |
| float-to-int conversion | `ccast.rs` |
| Everything above, in scope | `use crate::prelude::*;` |

Read the C file first, then the c2rust file when the C is unclear (it shows
every implicit conversion and every macro expanded).

## The rules

### 1. Translate literally

* Same functions, same names, same parameters in the same order, same local
  variable names, same statements in the same order, same loops, same
  `if`/`else` structure, same early returns. Keep each function's header
  comment block and the inline comments.
* C struct field names are kept (`Node[j].newDepth`). The C field `type`
  is `type_`.
* Every function that touches model state takes the model first:
  `prj: &mut Project` (or `prj: &Project` if it only reads). The name is
  `prj` because C SWMM uses `p` for pollutant indexes. A global `X` is
  `prj.X`; a file-scope static `S` of module `m` is `prj.m.S`.
* `#ifdef _WIN32`, `#ifdef _OPENMP` and similar: take the branch a Linux
  GCC build without OpenMP takes (that is the reference build).

### 2. Arithmetic must be bit-identical

* `int` → `i32`, `long` → `i64`, `double` → `f64`, `float` → `f32`,
  a `char` holding a number → `i8`, `DateTime` → `f64`, `size_t` → `usize`.
  Index with `x[j as usize]`.
* Keep every expression's operand order and grouping: `a*b*c` is `(a*b)*c`,
  and that is not `a*(b*c)` in floating point. Do not simplify, factor,
  or reorder. Do not replace `x/2.0` with `x*0.5`.
* Where C converts implicitly, write the conversion: `int` in a double
  expression → `as f64` at the same point C converts. Integer division stays
  integer division (`i/2` with ints truncates, as in C).
* `float` arithmetic: C on x86-64 evaluates `float op float` in float, and
  promotes to double when either side is double. Do the same with `f32`/`f64`.
  Assigning a double to a float is `as f32` (round to nearest, as C).
* **Every float-to-integer conversion goes through `ccast`**:
  `f64_to_i32(x)`, `f64_to_i64(x)`, `f64_to_i8(x)`, `f32_to_i64(x)`. Never
  write `as i32` on a float: Rust saturates where x86 gives `i32::MIN`, and
  this already changed 12 EPA test reports in the baseline. This includes C's
  implicit conversions (`int n = x;` with `x` double).
* Math: `pow(a,b)` → `a.powf(b)`, `sqrt` → `.sqrt()`, `exp`, `log` →
  `.ln()`, `log10` → `.log10()`, `fabs` → `.abs()`, `floor`, `ceil`,
  `fmod(a,b)` → `a % b`, `sin`/`cos`/`tan`/`atan`/`asin`/`acos`, `atan2(y,x)`
  → `y.atan2(x)`. These call the same libm as C. Do not use `powi`,
  `mul_add`, `hypot` or `exp_m1` unless C called the matching function.
* Macros come from `macros.rs`. A macro argument with a side effect
  (`MAX(x, f())`, where C may call `f` twice) needs care: write out what C
  does.
* `&&` and `||` short-circuit in both languages; keep the same order of tests.
* C's `CALL(x)` macro is `if prj.ErrorCode <= 0 { prj.ErrorCode = x; }`
  (x is not evaluated once there is an error).

### 3. Strings are bytes

* Names, tokens and lines are `&[u8]` / `Vec<u8>`, never `str`. A fixed C
  buffer `char s[N]` is `[u8; N]` with C's terminator semantics; `cio`
  has `strlen, cstr, strcpy, sstrncpy, strcat, strcmp, strncmp, strstr,
  strchr, strrchr, toupper, isspace, isdigit`.
* `sprintf(buf, fmt, ...)` → `let s = cformat!(b"fmt", a, b);`
  `fprintf(F.file, fmt, ...)` → `cprintf!(prj.F, b"fmt", a, b);`
  (TFile has `write_bytes`). Copy each format string byte for byte from the
  C, or use the `text.rs` constant C used (`FMT20`, `WARN03`...). Arguments
  are typed: pass `f64` for `%f`/`%e`/`%g`, an integer for `%d`/`%ld`, bytes
  for `%s`, a `u8` for `%c`. A mismatch panics (in C it would be undefined).
* `sscanf` → `cio::sscanf(input, b"fmt", &mut [Scan::Int(&mut a), ...])`;
  `strtod` → `cio::strtod(s)` (value, bytes consumed); `atof`, `atoi`, `strtol`.
* Keyword tables are slices without C's NULL terminator: loop over
  `table.iter()` or `0..table.len()` where C looped to NULL.

### 4. Files

* `TFile.file` is `Option<CFile>`. `fopen(name, mode)` → `CFile::open(&name, mode)`;
  `fclose` → `f.file = None` (dropping flushes); `fgets(buf, n, f)` →
  `f.fgets(n)` (returns the line with its newline, None at EOF);
  `fread`/`fwrite` → `read_bytes`/`write_bytes`; `fseek(f, pos, SEEK_SET)` →
  `fseek_set(pos)`; `ftell`, `rewind`, `feof`, `fflush` as named.
* Binary values are written little-endian (`x.to_le_bytes()`), which is what
  C's `fwrite` of an `int`/`float`/`double` writes on x86-64 and wasm32.
* `remove(name)` → `cio::remove(name)`.

### 5. Pointers became owners

`objects.rs` says how each pointer field is represented. In short:

* `char* ID` → `Vec<u8>`; `double*` arrays → `Vec<f64>`.
* A pointer that can be NULL → `Option<...>`; `x == NULL` → `x.is_none()`.
* **A linked list → `Vec` in the order C walks it** (element 0 is the head).
  If C prepends (`item->next = head; head = item;`) use `v.insert(0, item)`;
  if C appends, `v.push(item)`. Iteration order changes floating-point sums,
  so this matters.
* `TTable`'s points are `entries: Vec<TTableEntry>`; its cursor `thisEntry`
  is `Option<usize>` into it.
* `malloc`/`calloc(n, sizeof(T))` → `vec![T::default(); n]` (calloc zeroes,
  and `Default` is all zeros). `free`/`FREE` → drop, `clear()` or `= Vec::new()`.
  `MEMCHECK` and out-of-memory branches disappear (Rust aborts instead).
* A module-scope pointer to "the current object" (`theLidUnit`, `pXsect`,
  `theSubarea`) becomes an index, a copied value, or a parameter.

### 6. Borrowing: how to call with `prj` and part of `prj`

The compiler refuses `f(prj, &mut prj.Link[j].xsect)`. In order of preference:

1. Pass an index instead of a reference when the callee can look it up.
2. Copy: `let xs = prj.Link[j].xsect; f(prj, &xs);` (TXsect, TExtInflow,
   the totals and stats structs are `Copy`). If the callee modifies it,
   write it back after.
3. Take and restore: `let mut v = std::mem::take(&mut prj.Subcatch[j].newQual);
   f(prj, j, &mut v); prj.Subcatch[j].newQual = v;` — only when the callee
   does not read that same field through `prj` (check!).
4. Borrow disjoint fields directly in one call: `g(&mut prj.infil, &mut prj.Storage[k].exfil)`.

Never hold a reference into `prj` across a call that takes `prj`.

### 7. Callbacks

A C function pointer with a `void*` context becomes `&mut dyn FnMut(...)`; the
closure captures what the context carried. `findroot_Ridder`,
`findroot_Newton`, `odesolve_integrate`, `mathexpr_create/eval` are declared
this way in their stubs.

### 8. What C leaves undefined

Reproduce what EPA's x86-64 GCC build does. An uninitialized local that is
read before it is written: say so in a comment and use what the code path
gives (usually it is written first on every path; check). An out-of-bounds
read in C: say so in a comment and mirror it if it can affect output.
Signed overflow: Rust release builds wrap, as x86 does.

### 9. Scope of your work

* Edit only the files of your modules. Do not edit `objects.rs`,
  `globals.rs`, `prelude.rs`, `lib.rs` or another module's file. If a shared
  type or another module's signature blocks you, work around it locally and
  list the change you need in your report.
* Implement the public functions with the signatures in your stubs. If one
  cannot work, change it and list the change (who calls it is noted above
  each stub).
* Add your module's private functions (non-`pub`, same names as C) and the
  rest of its file-scope state to its `<Module>State` struct. Constants that
  C defined in the `.c` file are private `const`s in your file. Do not make
  anything `pub` that C kept `static`, except state fields other modules use.
* No `unsafe`, anywhere (the crate forbids it).
* `cargo check -p swmm5-rust` must pass with your modules in place and the
  others still stubs. Do not leave `todo!()` in your modules.
