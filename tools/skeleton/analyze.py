#!/usr/bin/env python3
"""Which engine functions read global state, which write it (transitively).

Reads the c2rust baseline. A global is a `static mut` that some function
assigns (a table that is never assigned is a constant, not state). A
function writes state if it assigns a global, takes `&raw mut`/`&mut` of one,
does file I/O, or calls a function that writes. Output: JSON {fn: mode}
with mode in pure | read | write, plus call graph and owning module.
"""
import re, os, json, sys
D = sys.argv[1]
src = {f[:-3]: open(os.path.join(D, f)).read() for f in os.listdir(D) if f.endswith('.rs')}
FILEIO = {'fprintf','fwrite','fputs','fputc','fflush','fseek','fseeko','ftell','fgets','fread','feof','rewind','fclose','fopen','remove','sprintf','snprintf','printf','puts','fgetc','getc','mkstemp','time','clock'}
# global definitions
globs = {}
for m, s in src.items():
    for n in re.findall(r'^(?:pub )?static mut (\w+):', s, flags=re.M):
        globs.setdefault(n, m)
# function bodies (definitions only)
fns = {}
for m, s in src.items():
    for mm in re.finditer(r'^(?:#\[no_mangle\]\n)?(?:pub )?unsafe extern "C" fn (\w+)\((.*?)\)[^{]*\{', s, flags=re.M | re.S):
        name = mm.group(1)
        # body: brace matching
        i = mm.end(); depth = 1
        while depth:
            c = s[i]
            if c == '{': depth += 1
            elif c == '}': depth -= 1
            i += 1
        body = s[mm.end():i-1]
        fns[(m, name)] = body
names_by = {}
for (m, n) in fns: names_by.setdefault(n, []).append(m)
ident = re.compile(r'\b[A-Za-z_]\w*\b')
assigned = set()
for (m, n), b in fns.items():
    for line in b.split(';'):
        # assignment: something op= something, LHS before the first '=' not part of ==,<=,>=,!=
        mm = re.search(r'(?<![=!<>])(\+|-|\*|/|%|\||&|\^|<<|>>)?=(?!=)', line)
        if mm:
            lhs = line[:mm.start()]
            for g in ident.findall(lhs):
                if g in globs: assigned.add(g)
        for g in re.findall(r'&(?:raw )?mut \(?\*?(\w+)', line):
            if g in globs: assigned.add(g)
state = set(g for g in assigned if not re.search(r'(Words|KeyWords|Words2|Txt|Word)$', g))
info = {}
for (m, n), b in fns.items():
    ids = set(ident.findall(b))
    reads = ids & state
    calls = set(x for x in re.findall(r'\b(\w+)\(', b))
    w = False
    for line in b.split(';'):
        mm = re.search(r'(?<![=!<>])(\+|-|\*|/|%|\||&|\^|<<|>>)?=(?!=)', line)
        if mm and set(ident.findall(line[:mm.start()])) & state: w = True
        if re.search(r'&(?:raw )?mut \(?\*?(\w+)', line) and set(re.findall(r'&(?:raw )?mut \(?\*?(\w+)', line)) & state: w = True
    if calls & FILEIO and reads: w = True
    info[(m, n)] = dict(mod=m, reads=bool(reads), writes=w, calls=sorted(c for c in calls if c in names_by))
# propagate
changed = True
def resolve(caller_mod, c):
    ms = names_by[c]
    return (caller_mod, c) if caller_mod in ms else (ms[0], c)
while changed:
    changed = False
    for k, v in info.items():
        for c in v['calls']:
            t = info[resolve(v['mod'], c)]
            if t['writes'] and not v['writes']: v['writes'] = True; changed = True
            if t['reads'] and not v['reads']: v['reads'] = True; changed = True
out = {f"{k[0]}::{k[1]}": ('write' if v['writes'] else 'read' if v['reads'] else 'pure') for k, v in info.items()}
json.dump(dict(mode=out, state=sorted(state), consts=sorted(set(globs) - state)), open(sys.argv[2], 'w'), indent=0)
import collections
print(collections.Counter(out.values()), len(state), 'state globals', len(set(globs)-state), 'constant statics')
