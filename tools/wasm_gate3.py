#!/usr/bin/env python3
"""WebAssembly parity for three engines, byte for byte.

  wasm_gate3.py REF_EXE EPA_WASM CPLUS_WASM RS_WASM WASM_RUN_MJS CORPUS LIST OUT.jsonl

For each model: native EPA 5.2.4 (the gate's reference), then EPA 5.2.4,
swmm5-cplus and swmm5-rust compiled to wasm32-wasi and run in node.
Recorded per model:
  rs_out/rs_rpt    swmm5-rust wasm == EPA wasm   (the claim: same toolchain family, C vs Rust)
  cp_out/cp_rpt    swmm5-cplus wasm == EPA wasm  (the earlier claim, rechecked)
  plat_out/plat_rpt EPA wasm == EPA native        (platform control: same C, two libcs)
  rs_native_out    swmm5-rust wasm == EPA native
Resumes from OUT.jsonl.
"""
import sys, os, subprocess, json, tempfile, hashlib
ref, epw, cpw, rsw, mjs, corpus, lst, outp = [os.path.abspath(a) for a in sys.argv[1:9]]
CLOCK = (b'Analysis begun on', b'Analysis ended on', b'Total elapsed time')
def norm(b): return b'\n'.join(l for l in b.split(b'\n') if not any(c in l for c in CLOCK))
def rd(p): return open(p, 'rb').read() if os.path.exists(p) else b''
models = [l.strip() for l in open(lst) if l.strip()]
models = [m[len('corpus1729/'):] if m.startswith('corpus1729/') else m for m in models]
done = set()
if os.path.exists(outp):
    done = {json.loads(l)['inp'] for l in open(outp)}
os.makedirs('/tmp/claude-0', exist_ok=True)
with open(outp, 'a') as fo:
    for k, m in enumerate(models):
        if m in done: continue
        p = os.path.join(corpus, m)
        n = os.path.basename(p)[:-4]
        rec = dict(inp=m)
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            try:
                r = subprocess.run([ref, p, d + '/ref.rpt', d + '/ref.out'], cwd=os.path.dirname(p),
                                   capture_output=True, timeout=600)
                rec['rc_ref'] = r.returncode
                for tag, w in (('epa', epw), ('cp', cpw), ('rs', rsw)):
                    r = subprocess.run(['node', mjs, w, p, d + '/' + tag], capture_output=True, timeout=1800)
                    o = r.stdout.decode('latin-1'); i = o.rfind('{"name"')
                    try: rec['run_' + tag] = json.loads(o[i:].strip().splitlines()[0])
                    except Exception: rec['run_' + tag] = dict(rc='node exit %d' % r.returncode, stderr=r.stderr.decode()[-300:])
            except subprocess.TimeoutExpired:
                rec['timeout'] = True
                fo.write(json.dumps(rec) + '\n'); fo.flush(); continue
            ro, rr = rd(d + '/ref.out'), rd(d + '/ref.rpt')
            eo, er = rd(f'{d}/epa/{n}.out'), rd(f'{d}/epa/{n}.rpt')
            co, cr = rd(f'{d}/cp/{n}.out'), rd(f'{d}/cp/{n}.rpt')
            so, sr = rd(f'{d}/rs/{n}.out'), rd(f'{d}/rs/{n}.rpt')
            rec.update(out_bytes=len(eo), rpt_bytes=len(er),
                       rs_out=(so == eo), rs_rpt=(norm(sr) == norm(er)),
                       cp_out=(co == eo), cp_rpt=(norm(cr) == norm(er)),
                       plat_out=(eo == ro), plat_rpt=(norm(er) == norm(rr)),
                       rs_native_out=(so == ro), rs_native_rpt=(norm(sr) == norm(rr)),
                       sha=hashlib.sha256(eo).hexdigest()[:16])
        fo.write(json.dumps(rec) + '\n'); fo.flush()
        if not (rec['rs_out'] and rec['rs_rpt']):
            print('RS WASM DIFF', m, {x: rec[x] for x in ('rs_out', 'rs_rpt', 'cp_out', 'cp_rpt')},
                  rec.get('run_epa', {}).get('rc'), rec.get('run_rs', {}).get('rc'), flush=True)
        print(f'{k + 1}/{len(models)}', flush=True)
print('done')
