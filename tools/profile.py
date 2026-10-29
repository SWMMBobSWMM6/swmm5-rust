#!/usr/bin/env python3
"""Count instructions, not seconds: callgrind on EPA 5.2.4 and swmm5-rust, per function.

An instruction count does not depend on what else the machine is doing, so it can be taken
while the gates run. It is not a run time (cache misses and branch mispredictions are not
counted), but it says where the extra work is.

  python3 tools/profile.py EPA_BIN RS_BIN CORPUS WORKDIR OUT.json model.inp [model.inp ...]

Callgrind output is kept in WORKDIR as <stem>.epa.cg / <stem>.rs.cg and reused if present.
For every function: instructions (its own and inclusive), calls; and every caller -> callee edge with its count.
"""
import json, os, re, subprocess, sys, collections, filecmp

epa, rs, corpus, work, outp, *models = sys.argv[1:]
os.makedirs(work, exist_ok=True)

def run(binary, inp, tag):
    cg = os.path.join(work, tag + '.cg')
    if not os.path.exists(cg):
        subprocess.run(['nice', '-n', '19', 'valgrind', '--tool=callgrind', '--callgrind-out-file=' + cg,
                        binary, inp, os.path.join(work, tag + '.rpt'), os.path.join(work, tag + '.out')],
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
    txt = subprocess.run(['callgrind_annotate', '--threshold=100', cg], capture_output=True, text=True).stdout
    txt = subprocess.run(['c++filt'], input=txt, capture_output=True, text=True).stdout
    total = int(re.search(r'([\d,]+) \(100\.0%\)\s+PROGRAM TOTALS', txt).group(1).replace(',', ''))
    fns = collections.Counter(); libs = set()
    for l in txt.splitlines():
        m = re.match(r'\s*([\d,]+) \(\s*[\d.]+%\)\s+(\S.*?):(.+?) \[(.+?)\]', l)
        if not m: continue
        name = m.group(3).strip()
        name = re.sub(r'^swmm5_rust::\w+::', '', name)      # module path
        name = re.sub(r'_mut$', '', name)                    # the Rust's &mut twins of xsect getters
        if 'libm' in m.group(4): name = 'libm:' + name.split('_fma')[0].lstrip('_')
        if '.so' in m.group(4): libs.add(name)
        fns[name] += int(m.group(1).replace(',', ''))
    # inclusive cost (the function and everything it calls)
    itxt = subprocess.run(['callgrind_annotate', '--inclusive=yes', '--threshold=100', cg], capture_output=True, text=True).stdout
    itxt = subprocess.run(['c++filt'], input=itxt, capture_output=True, text=True).stdout
    incl = collections.Counter()
    for l in itxt.splitlines():
        m = re.match(r'\s*([\d,]+) \(\s*[\d.]+%\)\s+(\S.*?):(.+?) \[(.+?)\]', l)
        if m: incl[re.sub(r'_mut$', '', re.sub(r'^swmm5_rust::\w+::', '', m.group(3).strip()))] += int(m.group(1).replace(',', ''))
    # calls into each function: the "(Nx)" counts on the caller lines of its block in the caller tree
    tree = subprocess.run(['callgrind_annotate', '--tree=caller', '--threshold=100', cg], capture_output=True, text=True).stdout
    tree = subprocess.run(['c++filt'], input=tree, capture_output=True, text=True).stdout
    calls = collections.Counter(); edges = collections.Counter()
    nm = lambda x: re.sub(r'_mut$', '', re.sub(r'^swmm5_rust::\w+::', '', x.strip()))
    for blk in re.split(r'\n\s*\n', tree):
        me = re.search(r'\*\s+\S*?:(.+?) \[', blk)
        if not me: continue
        name = nm(me.group(1))
        for caller, n in re.findall(r'^\s*[\d,]+ \(.*?\)\s+<\s+\S*?:(.+?) \(([\d,]+)x\)', blk, re.M):
            calls[name] += int(n.replace(',', '')); edges[(nm(caller), name)] += int(n.replace(',', ''))
    fns = {k: (v, calls.get(k, 0)) for k, v in fns.items()}
    return total, fns, libs, incl, edges

res = []
for m in models:
    inp = os.path.join(corpus, m)
    stem = re.sub(r'\W+', '_', os.path.basename(m)[:-4])
    te, fe, le, ie, ee = run(epa, inp, stem + '.epa')
    tr, fr, lr, ir, er = run(rs, inp, stem + '.rs')
    same = filecmp.cmp(os.path.join(work, stem + '.epa.out'), os.path.join(work, stem + '.rs.out'), shallow=False)
    keys = set(fe) | set(fr)
    z = (0, 0)
    rows = sorted(({'fn': k, 'epa': fe.get(k, z)[0], 'rs': fr.get(k, z)[0], 'epa_calls': fe.get(k, z)[1], 'rs_calls': fr.get(k, z)[1],
                    'epa_incl': ie.get(k, 0), 'rs_incl': ir.get(k, 0), 'lib': k in le or k in lr} for k in keys),
                  key=lambda r: -max(r['epa'], r['rs']))
    # every caller -> callee edge with its call count, in each build
    ek = set(ee) | set(er)
    calls = sorted(([a, b, ee.get((a, b), 0), er.get((a, b), 0)] for a, b in ek), key=lambda e: -max(e[2], e[3]))
    res.append(dict(model=m, epa_total=te, rs_total=tr, ratio=tr / te, out_identical=same, rows=rows, edges=calls))
    print(f'{m}: EPA {te:,}  Rust {tr:,}  ratio {tr / te:.3f}  .out identical {same}')
json.dump(dict(measure='callgrind instruction counts (Ir), valgrind ' +
               subprocess.run(['valgrind', '--version'], capture_output=True, text=True).stdout.strip(),
               epa=os.path.basename(epa), rs=os.path.basename(rs), models=res), open(outp, 'w'), indent=1)
