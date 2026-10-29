#!/usr/bin/env python3
"""Interleaved native timing of EPA 5.2.4, swmm5-cplus and swmm5-rust, with a noise floor.

  bench3.py EPA_EXE CPLUS_EXE RUST_EXE CORPUS LIST OUT.json [--reps N]

Each rep runs, for every model: EPA, C++, Rust, then EPA again. Times are the
child's user+system CPU seconds (os.wait4), so other load on the machine moves
them less than wall time; wall time is kept too. The noise floor of a model is
the ratio of its two EPA medians. The first rep also checks that the three
.out files are byte-identical and the .rpt files equal minus wall-clock lines.
"""
import sys, os, json, time, statistics, argparse, tempfile, subprocess, hashlib
ap = argparse.ArgumentParser()
for a in ('epa', 'cp', 'rs', 'corpus', 'lst', 'out'): ap.add_argument(a)
ap.add_argument('--reps', type=int, default=5)
x = ap.parse_args()
EXE = dict(epa=os.path.abspath(x.epa), cp=os.path.abspath(x.cp), rs=os.path.abspath(x.rs))
models = [l.strip() for l in open(x.lst) if l.strip()]
CLOCK = (b'Analysis begun on', b'Analysis ended on', b'Total elapsed time')
def norm(b): return b'\n'.join(l for l in b.split(b'\n') if not any(c in l for c in CLOCK))
os.makedirs('/tmp/claude-0', exist_ok=True)
def run(exe, inp, keep=False):
    with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
        t0 = time.perf_counter()
        p = subprocess.Popen([exe, inp, d + '/m.rpt', d + '/m.out'], cwd=os.path.dirname(inp),
                             stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        _, status, ru = os.wait4(p.pid, 0)
        wall = time.perf_counter() - t0
        out = None
        if keep:
            rd = lambda q: open(q, 'rb').read() if os.path.exists(q) else b''
            out = (hashlib.sha256(rd(d + '/m.out')).hexdigest(), hashlib.sha256(norm(rd(d + '/m.rpt'))).hexdigest())
        return ru.ru_utime + ru.ru_stime, wall, out
res = {m: {k: [] for k in ('epa', 'cp', 'rs', 'epa2')} for m in models}
same = {}
for rep in range(x.reps):
    for m in models:
        inp = os.path.join(x.corpus, m)
        outs = {}
        for k, exe in (('epa', 'epa'), ('cp', 'cp'), ('rs', 'rs'), ('epa2', 'epa')):
            cpu, wall, o = run(EXE[exe], inp, keep=(rep == 0))
            res[m][k].append(dict(cpu=cpu, wall=wall)); outs[k] = o
        if rep == 0:
            same[m] = dict(rs=outs['rs'] == outs['epa'], cp=outs['cp'] == outs['epa'], epa2=outs['epa2'] == outs['epa'])
    print('rep', rep + 1, 'done', flush=True)
rows = []
med = lambda m, k, f='cpu': statistics.median(v[f] for v in res[m][k])
for m in models:
    a, a2, c, r = med(m, 'epa'), med(m, 'epa2'), med(m, 'cp'), med(m, 'rs')
    rows.append(dict(model=os.path.basename(m).replace('.inp', ''), path=m, epa=a, epa2=a2, cp_t=c, rs_t=r,
                     rs=r / a, cp=c / a, floor=a2 / a, wall_rs=med(m, 'rs', 'wall') / med(m, 'epa', 'wall'), same=same[m]))
geo = lambda v: __import__('math').exp(sum(__import__('math').log(t) for t in v) / len(v))
summary = dict(rs_median=statistics.median(r['rs'] for r in rows), cp_median=statistics.median(r['cp'] for r in rows),
               rs_geo=geo([r['rs'] for r in rows]), cp_geo=geo([r['cp'] for r in rows]),
               floor_dev=max(abs(r['floor'] - 1) for r in rows), reps=x.reps, models=len(rows),
               all_identical=all(r['same']['rs'] and r['same']['cp'] for r in rows), measure='child user+system CPU seconds, median of reps')
json.dump(dict(rows=rows, summary=summary, raw=res), open(x.out, 'w'), indent=1)
for r in rows:
    print(f"{r['model'][:34]:34s} EPA {r['epa']:7.3f}s  C++/EPA {r['cp']:.3f}  Rust/EPA {r['rs']:.3f}  floor {r['floor']:.3f}  same {r['same']}")
print(json.dumps(summary, indent=1))
