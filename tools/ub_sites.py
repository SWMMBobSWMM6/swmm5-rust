#!/usr/bin/env python3
"""List the places where the translation notes that EPA's C is undefined, reads
past an array, or dereferences NULL, with the Rust comment that says what the
safe engine does instead. Writes docs/UNDEFINED.md."""
import re, glob, os
HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PAT = re.compile(r'undefined|dereferences? (a )?NULL|C would (overrun|crash|read|dereference|use|have)(?! received)|C crashes|reads? (beyond|past)|overrun|indeterminate|uninitiali[sz]ed', re.I)
rows = []
for f in sorted(glob.glob(f'{HERE}/crates/engine/src/*.rs')):
    name = os.path.basename(f)
    if name in ('ccast.rs', 'error.rs'): continue
    L = open(f, encoding='utf-8').read().split('\n')
    k = 0
    while k < len(L):
        ln = L[k]
        if '//' in ln and not ln.strip().startswith(('//!', '///')) and PAT.search(ln.split('//', 1)[1]):
            # the whole comment block around the hit
            a = k
            while a > 0 and L[a - 1].strip().startswith('//') and not L[a - 1].strip().startswith(('//!', '///', '//=')): a -= 1
            b = k
            while b + 1 < len(L) and L[b + 1].strip().startswith('//'): b += 1
            text = ' '.join(x.strip().lstrip('/').lstrip('!').strip() for x in L[a:b + 1])
            rows.append((name, a + 1, text))
            k = b + 1
        else:
            k += 1
with open(f'{HERE}/docs/UNDEFINED.md', 'w') as fo:
    fo.write('# Where EPA\'s C is undefined, and what swmm5-rust does\n\n'
             'Collected by `tools/ub_sites.py` from the translation\'s own comments: every place a translator noted that the C reads '
             'past an array, follows a NULL pointer, reads an uninitialized value, or is otherwise undefined. Safe Rust cannot do '
             'any of these, so each site says what the engine does instead. Most cannot be reached by a valid input file.\n\n'
             f'{len(rows)} sites.\n\n| File | Line | Note |\n|---|---|---|\n')
    for n, l, t in rows:
        fo.write(f'| `{n}` | {l} | {t.replace("|", "/")} |\n')
print(len(rows), 'sites')
