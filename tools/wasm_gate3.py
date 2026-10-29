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
import sys, os, subprocess, json, tempfile, hashlib, shutil
ref, epw, cpw, rsw, mjs, corpus, lst, outp = [os.path.abspath(a) for a in sys.argv[1:9]]
CLOCK = (b'Analysis begun on', b'Analysis ended on', b'Total elapsed time')
def digest(p, normalize=False):
    # streamed, so a 1.6 GB .out (the 1000-year simulations) never has to fit in memory
    h = hashlib.sha256(); n = 0
    if not os.path.exists(p): return h.hexdigest(), 0
    with open(p, 'rb') as f:
        if normalize:
            for line in f:
                n += len(line)
                if not any(c in line for c in CLOCK): h.update(line)
        else:
            while True:
                b = f.read(1 << 20)
                if not b: break
                n += len(b); h.update(b)
    return h.hexdigest(), n
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
                H = {}
                H['ref'] = (digest(d + '/ref.out'), digest(d + '/ref.rpt', True))
                for q in ('/ref.out', '/ref.rpt'):
                    if os.path.exists(d + q): os.remove(d + q)
                for tag, w in (('epa', epw), ('cp', cpw), ('rs', rsw)):
                    r = subprocess.run(['node', mjs, w, p, d + '/' + tag], capture_output=True, timeout=1800)
                    o = r.stdout.decode('latin-1'); i = o.rfind('{"name"')
                    try: rec['run_' + tag] = json.loads(o[i:].strip().splitlines()[0])
                    except Exception: rec['run_' + tag] = dict(rc='node exit %d' % r.returncode, stderr=r.stderr.decode()[-300:])
                    H[tag] = (digest(f'{d}/{tag}/{n}.out'), digest(f'{d}/{tag}/{n}.rpt', True))
                    shutil.rmtree(f'{d}/{tag}', ignore_errors=True)      # one engine's files on disk at a time
            except subprocess.TimeoutExpired:
                rec['timeout'] = True
                fo.write(json.dumps(rec) + '\n'); fo.flush(); continue
            (ro, rr), (eo, er), (co, cr), (so, sr) = H['ref'], H['epa'], H['cp'], H['rs']
            rec.update(out_bytes=eo[1], rpt_bytes=er[1],
                       rs_out=(so[0] == eo[0]), rs_rpt=(sr[0] == er[0]),
                       cp_out=(co[0] == eo[0]), cp_rpt=(cr[0] == er[0]),
                       plat_out=(eo[0] == ro[0]), plat_rpt=(er[0] == rr[0]),
                       rs_native_out=(so[0] == ro[0]), rs_native_rpt=(sr[0] == rr[0]),
                       sha=eo[0][:16])
        fo.write(json.dumps(rec) + '\n'); fo.flush()
        if not (rec['rs_out'] and rec['rs_rpt']):
            print('RS WASM DIFF', m, {x: rec[x] for x in ('rs_out', 'rs_rpt', 'cp_out', 'cp_rpt')},
                  rec.get('run_epa', {}).get('rc'), rec.get('run_rs', {}).get('rc'), flush=True)
        print(f'{k + 1}/{len(models)}', flush=True)
print('done')
