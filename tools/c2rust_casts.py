#!/usr/bin/env python3
"""Route every float-to-integer `as` cast in the c2rust output through ccast.

  c2rust_casts.py CLIPPY_JSON ENGINE_DIR

The cast sites come from the compiler, not from a pattern: clippy's
cast_possible_truncation lint reports each `expr as T` whose source is a
float, with its exact span. Each span is rewritten to crate::ccast::f64_to_T(expr).
"""
import json, sys, collections, os
FN = {('f64','i32'):'f64_to_i32', ('f64','i64'):'f64_to_i64', ('f64','i8'):'f64_to_i8', ('f32','i64'):'f32_to_i64'}
edits = collections.defaultdict(list)
for l in open(sys.argv[1]):
    try: m = json.loads(l)
    except Exception: continue
    if m.get('reason') != 'compiler-message': continue
    msg = m['message']
    if 'cast_possible_truncation' not in str(msg.get('code')): continue
    t = msg['message']           # casting `f64` to `i32` may truncate the value
    a, b = t.split('`')[1], t.split('`')[3]
    if (a, b) not in FN: continue
    sp = [s for s in msg['spans'] if s['is_primary']][0]
    edits[sp['file_name']].append((sp['byte_start'], sp['byte_end'], FN[(a, b)]))
n = 0
for f, es in edits.items():
    data = open(f, 'rb').read()
    for s, e, fn in sorted(set(es), reverse=True):
        expr = data[s:e].decode()
        k = expr.rindex(' as ')
        inner = expr[:k].strip()
        data = data[:s] + f'crate::ccast::{fn}({inner})'.encode() + data[e:]
        n += 1
    open(f, 'wb').write(data)
print(n, 'casts rewritten in', len(edits), 'files')
