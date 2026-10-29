// Interleaved WebAssembly timing of EPA 5.2.4, swmm5-cplus and swmm5-rust in node (V8),
// through the same engine_core.js the app uses, with an EPA-against-EPA noise floor.
//   node bench_wasm.js WASM_DIR CORPUS LIST OUT.json [reps]
const fs = require('fs'), path = require('path'), crypto = require('crypto');
const C = require('./engine_core.js');
const [dir, corpus, list, outp, repsArg] = process.argv.slice(2);
const reps = +(repsArg || 5);
const mod = n => new WebAssembly.Module(fs.readFileSync(path.join(dir, n)));
const M = { epa: mod('swmm524_epa.wasm'), cp: mod('swmm5cplus.wasm'), rs: mod('swmm5rs.wasm') };
const models = fs.readFileSync(list, 'utf8').split('\n').map(s => s.trim()).filter(Boolean);
const sha = b => b ? crypto.createHash('sha256').update(b).digest('hex') : '';
const CLOCK = /Analysis begun on|Analysis ended on|Total elapsed time/;
const norm = b => b ? Buffer.from(b).toString('latin1').split('\n').filter(l => !CLOCK.test(l)).join('\n') : '';
const median = a => { const s = [...a].sort((x, y) => x - y), n = s.length; return n % 2 ? s[(n - 1) / 2] : (s[n / 2 - 1] + s[n / 2]) / 2; };
const res = {}, same = {};
// rep -1 is a warm-up (V8 tiers code up from its baseline compiler as it runs) and the identity check; it is not timed
for (let rep = -1; rep < reps; rep++) {
  for (const m of models) {
    const p = path.join(corpus, m), name = path.basename(p), files = { [name]: fs.readFileSync(p) };
    res[m] = res[m] || { epa: [], cp: [], rs: [], epa2: [] };
    const o = {};
    for (const [k, eng] of [['epa', 'epa'], ['cp', 'cp'], ['rs', 'rs'], ['epa2', 'epa']]) {
      const r = eng === 'epa' ? C.runLegacy(M.epa, name, files) : C.runHandle(M[eng], name, files);
      if (rep >= 0) res[m][k].push(r.ms);
      if (rep === -1) o[k] = sha(r.out) + '|' + sha(Buffer.from(norm(r.rpt)));
    }
    if (rep === -1) same[m] = { rs: o.rs === o.epa, cp: o.cp === o.epa, epa2: o.epa2 === o.epa };
  }
  console.log(rep < 0 ? 'warm-up done' : 'rep ' + (rep + 1) + ' done');
}
const rows = models.map(m => {
  const a = median(res[m].epa), a2 = median(res[m].epa2), c = median(res[m].cp), r = median(res[m].rs);
  return { model: path.basename(m).replace(/\.inp$/, ''), path: m, epa: a, epa2: a2, cp_t: c, rs_t: r, rs: r / a, cp: c / a, floor: a2 / a, same: same[m] };
});
const geo = v => Math.exp(v.reduce((s, x) => s + Math.log(x), 0) / v.length);
const summary = { rs_median: median(rows.map(r => r.rs)), cp_median: median(rows.map(r => r.cp)), rs_geo: geo(rows.map(r => r.rs)), cp_geo: geo(rows.map(r => r.cp)),
  floor_dev: Math.max(...rows.map(r => Math.abs(r.floor - 1))), reps, models: rows.length, all_identical: rows.every(r => r.same.rs && r.same.cp),
  measure: 'wall milliseconds in one node process (V8 ' + process.versions.v8 + '), median of reps' };
fs.writeFileSync(outp, JSON.stringify({ rows, summary, raw: res }, null, 1));
rows.forEach(r => console.log(r.model.slice(0, 34).padEnd(34), 'EPA', r.epa.toFixed(0).padStart(7), 'ms  C++/EPA', r.cp.toFixed(3), ' Rust/EPA', r.rs.toFixed(3), ' floor', r.floor.toFixed(3), JSON.stringify(r.same)));
console.log(JSON.stringify(summary, null, 1));
