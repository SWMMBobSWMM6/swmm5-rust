// node test for engine_core.js: the in-memory WASI layer must give the same bytes as
// node's own WASI (the gate's runner), for all three engines, legacy and handle APIs,
// and the many-at-once runner must match solo runs.
//   node test_core.js WASM_DIR MODEL.inp [MODEL2.inp ...]
const fs = require('fs'), path = require('path'), crypto = require('crypto');
const C = require('./engine_core.js');
const dir = process.argv[2];
const models = process.argv.slice(3);
const mod = n => new WebAssembly.Module(fs.readFileSync(path.join(dir, n)));
const M = { epa: mod('swmm524_epa.wasm'), cplus: mod('swmm5cplus.wasm'), rust: mod('swmm5rs.wasm') };
const sha = b => b ? crypto.createHash('sha256').update(b).digest('hex').slice(0, 16) : '(none)';
const CLOCK = /Analysis begun on|Analysis ended on|Total elapsed time/;
const norm = b => b ? Buffer.from(b).toString('latin1').split('\n').filter(l => !CLOCK.test(l)).join('\n') : '';
let fail = 0;
const check = (ok, msg) => { console.log((ok ? 'ok   ' : 'FAIL ') + msg); if (!ok) fail++; };
const texts = [];
for (const m of models) {
  const name = path.basename(m), files = { [name]: fs.readFileSync(m) };
  texts.push(fs.readFileSync(m));
  const r = {};
  for (const k of ['epa', 'cplus', 'rust']) {
    r[k + '/legacy'] = C.runLegacy(M[k], name, files);
    if (k !== 'epa') r[k + '/handle'] = C.runHandle(M[k], name, files);
  }
  const ref = r['epa/legacy'];
  for (const [k, x] of Object.entries(r)) {
    check(x.kind === k.split('/')[0], `${name} ${k}: engine detected as ${x.kind}`);
    check(sha(x.out) === sha(ref.out) && norm(x.rpt) === norm(ref.rpt),
      `${name} ${k}: rc ${x.rc}${x.trap ? ' ' + x.trap : ''}, .out ${sha(x.out)}, ${(x.out || []).length} bytes, .rpt ${norm(x.rpt).length} chars, ${x.ms.toFixed(0)} ms, version ${x.version}`);
  }
  // against node's own WASI: the gate runner's output, if present next to the model
  const gold = m.replace(/\.inp$/i, '.node.out');
  if (fs.existsSync(gold)) check(sha(fs.readFileSync(gold)) === sha(ref.out), `${name}: MemWasi .out equals node WASI .out`);
}
if (models.length > 1) {
  const solo = models.map((m, i) => C.runLegacy(M.epa, 'solo.inp', { 'solo.inp': texts[i] }));
  let prog = 0;
  const mr = C.runMulti(M.rust, texts.map(t => ({ inp: t })), () => prog++, 200);
  check(mr.live === models.length && mr.after === 0, `rust many-at-once: ${mr.live} handles open together, ${mr.after} left after, ${mr.rounds} rounds, ${prog} progress calls`);
  mr.res.forEach((x, i) => check(sha(x.out) === sha(solo[i].out) && norm(x.rpt) === norm(solo[i].rpt),
    `rust many-at-once model ${i}: ${x.steps} steps, .out ${sha(x.out)} vs solo EPA ${sha(solo[i].out)}`));
  const mc = C.runMulti(M.cplus, texts.map(t => ({ inp: t })), null, 1e9);
  mc.res.forEach((x, i) => check(sha(x.out) === sha(solo[i].out), `cplus many-at-once model ${i}: .out ${sha(x.out)}`));
}
console.log(fail ? `${fail} FAILED` : 'all passed');
process.exit(fail ? 1 : 0);
