// wasm_run.mjs -- run one model through a SWMM wasm32-wasi reactor in node.
//   node wasm_run.mjs <engine.wasm> <model.inp> <outdir>
// Works for EPA 5.2.4 (C), swmm5-cplus and swmm5-rust: all export swmm_run;
// strings are passed with malloc (C) or swmm5rs_malloc (Rust).
import { WASI } from 'node:wasi';
import fs from 'node:fs';
import path from 'node:path';
process.removeAllListeners('warning');
const [wasmPath, inp, outdir] = process.argv.slice(2);
fs.mkdirSync(outdir, { recursive: true });
const name = path.basename(inp).replace(/\.inp$/i, '');
const wasi = new WASI({ version: 'preview1', args: [], env: {},
  preopens: { '/': outdir, '/model': path.dirname(path.resolve(inp)) } });
const inst = new WebAssembly.Instance(new WebAssembly.Module(fs.readFileSync(wasmPath)), wasi.getImportObject());
wasi.initialize(inst);
const ex = inst.exports;
const malloc = ex.malloc || ex.swmm5rs_malloc;
const enc = new TextEncoder();
const cstr = (s) => { const b = enc.encode(s + '\0'); const p = malloc(b.length);
  new Uint8Array(ex.memory.buffer, p, b.length).set(b); return p; };
const t0 = performance.now();
let rc;
try { rc = ex.swmm_run(cstr('/model/' + path.basename(inp)), cstr('/' + name + '.rpt'), cstr('/' + name + '.out')); }
catch (e) { rc = 'trap: ' + e.message; }
console.log(JSON.stringify({ name, rc, ms: +(performance.now() - t0).toFixed(1) }));
