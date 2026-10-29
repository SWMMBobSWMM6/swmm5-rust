#!/usr/bin/env python3
"""Make c2rust's output build on stable Rust, and nothing else.

  c2rust_stable.py SRC_DIR DST_DIR

* drops #![feature(extern_types, raw_ref_op)]: &raw is stable since Rust 1.82;
* turns each opaque `pub type X;` (glibc FILE internals, SWMM's TInlet, which
  most modules only ever see through a pointer) into a zero-sized #[repr(C)]
  struct, which is what an incomplete C type is to a module that never
  dereferences it.
No statement of the engine is touched.
"""
import os, re, sys
src, dst = sys.argv[1], sys.argv[2]
os.makedirs(dst, exist_ok=True)
mods = []
for f in sorted(os.listdir(src)):
    if not f.endswith('.rs'): continue
    s = open(os.path.join(src, f)).read()
    s = re.sub(r'^#!\[feature\([^)]*\)\]\n', '', s, flags=re.M)
    opaque = re.findall(r'^\s*pub type (\w+);\n', s, flags=re.M)
    s = re.sub(r'^\s*pub type \w+;\n', '', s, flags=re.M)
    if opaque:
        s += '\n' + ''.join(f'#[repr(C)]\npub struct {t} {{\n    _opaque: [u8; 0],\n}}\n' for t in opaque)
    open(os.path.join(dst, f), 'w').write(s)
    mods.append(f[:-3])
print(len(mods), 'modules')
