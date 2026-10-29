// Run models through the three wasm engines with engine_core.js; print one JSON line per run.
//   node run_decks.js WASM_DIR MODEL.inp ...
const fs = require('fs'), path = require('path'), crypto = require('crypto');
const C = require('./engine_core.js');
const dir = process.argv[2];
const mod = n => new WebAssembly.Module(fs.readFileSync(path.join(dir, n)));
const M = { epa: mod('swmm524_epa.wasm'), cplus: mod('swmm5cplus.wasm'), rust: mod('swmm5rs.wasm') };
const sha = b => b && b.length ? crypto.createHash('sha256').update(b).digest('hex').slice(0, 16) : '';
for (const m of process.argv.slice(3)) {
  const name = path.basename(m), files = { [name]: fs.readFileSync(m) };
  for (const k of ['epa', 'cplus', 'rust']) {
    let r;
    try { r = C.runLegacy(M[k], name, files); } catch (e) { r = { rc: 'throw', trap: String(e.message) }; }
    const rpt = r.rpt ? Buffer.from(r.rpt).toString('latin1') : '';
    const err = (rpt.match(/ERROR[^\n]*/) || [''])[0].trim();
    console.log(JSON.stringify({ model: name, engine: k, rc: r.rc, trap: r.trap || '', panic: r.panic || '',
      rpt: (r.rpt || []).length, out: (r.out || []).length, out_sha: sha(r.out), err }));
  }
}
