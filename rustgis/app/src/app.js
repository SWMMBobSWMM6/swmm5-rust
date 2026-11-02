'use strict';
const $ = s => document.querySelector(s), $$ = s => [...document.querySelectorAll(s)];
const A = window.APP;
const esc = s => String(s).replace(/[&<>"]/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
const fmt = (v, d) => v == null || v === '' ? '' : typeof v === 'number' ? (Number.isInteger(v) ? v.toLocaleString('en-US') : v.toLocaleString('en-US', { maximumFractionDigits: d == null ? 4 : d })) : String(v);
function toast(t, ms) { const d = document.createElement('div'); d.className = 'toast'; d.textContent = t; document.body.appendChild(d); setTimeout(() => d.remove(), ms || 3200); }

// ------------------------------------------------------------------ theme (dark-mode-default kit)
const CHART_THEME = {
  light: { face: '#ffffff', frame: '#e3e6ea', grid: '#eef0f3', tick: '#5b6472', axis: '#374151', stroke: '#ffffff',
    poly: 'rgba(37,99,235,0.10)', polyLine: '#64748b', link: '#334155', node: '#1f2937', gage: '#b84a2c', sel: '#d97706',
    rust: '#b84a2c', water: '#0e7490', ok: '#16a34a', bad: '#dc2626', budget: '#b45309', bar: '#2563eb', bar2: '#0e7490', muted: '#94a3b8',
    ramp: ['#c7d2fe', '#93c5fd', '#3b82f6', '#1d4ed8', '#172554'], ink: '#111827' },
  dark: { face: '#0f1a2c', frame: '#2a3752', grid: '#1b2740', tick: '#94a3b8', axis: '#c3cee0', stroke: '#0f1a2c',
    poly: 'rgba(96,165,250,0.10)', polyLine: '#5b6b88', link: '#9fb0c8', node: '#dbe4f0', gage: '#f0905a', sel: '#fbbf24',
    rust: '#f0905a', water: '#5ec8e5', ok: '#4ade80', bad: '#f87171', budget: '#fbbf24', bar: '#60a5fa', bar2: '#5ec8e5', muted: '#64748b',
    ramp: ['#475569', '#2f7fb5', '#22b8c8', '#8fdc7a', '#fde047'], ink: '#f1f5f9' }
};
let THEME = 'dark', THEME_PREF = 'dark';
const TH = () => CHART_THEME[THEME === 'dark' ? 'dark' : 'light'];
function prefersDark() { try { return window.matchMedia('(prefers-color-scheme: dark)').matches; } catch (e) { return true; } }
function applyTheme(pref) {
  THEME_PREF = (pref === 'light' || pref === 'auto') ? pref : 'dark';
  THEME = THEME_PREF === 'auto' ? (prefersDark() ? 'dark' : 'light') : THEME_PREF;
  document.documentElement.setAttribute('data-theme', THEME);
  $$('#theme-toggle button').forEach(b => b.classList.toggle('on', b.dataset.t === THEME_PREF));
  try { localStorage.setItem('swmm5rust_rustgis_theme', THEME_PREF); } catch (e) {}
}
function repaintAll() { drawMap(); renderLegend(); renderStaticFigures(); renderEssay(); renderGates(); }
$$('#theme-toggle button').forEach(b => b.addEventListener('click', () => { applyTheme(b.dataset.t); repaintAll(); }));
try { const mq = window.matchMedia('(prefers-color-scheme: dark)'); const f = () => { if (THEME_PREF === 'auto') { applyTheme('auto'); repaintAll(); } };
  mq.addEventListener ? mq.addEventListener('change', f) : mq.addListener(f); } catch (e) {}
(function () { let p = 'dark'; try { const v = localStorage.getItem('swmm5rust_rustgis_theme'); if (v === 'light' || v === 'auto' || v === 'dark') p = v; } catch (e) {} applyTheme(p); })();

// ------------------------------------------------------------------ bytes
function b64bytes(id) { const s = document.getElementById(id).textContent.trim(); const bin = atob(s); const u = new Uint8Array(bin.length); for (let i = 0; i < bin.length; i++) u[i] = bin.charCodeAt(i); return u; }
function bytesB64(u) { let s = ''; for (let i = 0; i < u.length; i += 0x8000) s += String.fromCharCode.apply(null, u.subarray(i, i + 0x8000)); return btoa(s); }
const latin1 = u => { let s = ''; for (let i = 0; i < u.length; i += 0x8000) s += String.fromCharCode.apply(null, u.subarray(i, i + 0x8000)); return s; };
const PLUGIN = b64bytes('b64-plugin'), EPA = b64bytes('b64-epa');
// SHA-256 (FIPS 180-4), for hashes that must match the plugin's own
function sha256(bytes) {
  const K = new Uint32Array([0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2]);
  const H = new Uint32Array([0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19]);
  const n = bytes.length, nb = ((n + 9 + 63) >> 6) << 6, m = new Uint8Array(nb); m.set(bytes); m[n] = 0x80;
  const dv = new DataView(m.buffer); dv.setUint32(nb - 4, (n * 8) >>> 0); dv.setUint32(nb - 8, Math.floor(n / 0x20000000));
  const W = new Uint32Array(64);
  for (let o = 0; o < nb; o += 64) {
    for (let i = 0; i < 16; i++) W[i] = dv.getUint32(o + 4 * i);
    for (let i = 16; i < 64; i++) { const a = W[i - 15], b = W[i - 2];
      W[i] = (((a >>> 7 | a << 25) ^ (a >>> 18 | a << 14) ^ a >>> 3) + W[i - 7] + ((b >>> 17 | b << 15) ^ (b >>> 19 | b << 13) ^ b >>> 10) + W[i - 16]) >>> 0; }
    let [a, b, c, d, e, f, g, h] = H;
    for (let i = 0; i < 64; i++) {
      const t1 = (h + ((e >>> 6 | e << 26) ^ (e >>> 11 | e << 21) ^ (e >>> 25 | e << 7)) + ((e & f) ^ (~e & g)) + K[i] + W[i]) >>> 0;
      const t2 = (((a >>> 2 | a << 30) ^ (a >>> 13 | a << 19) ^ (a >>> 22 | a << 10)) + ((a & b) ^ (a & c) ^ (b & c))) >>> 0;
      h = g; g = f; f = e; e = (d + t1) >>> 0; d = c; c = b; b = a; a = (t1 + t2) >>> 0;
    }
    H[0] += a; H[1] += b; H[2] += c; H[3] += d; H[4] += e; H[5] += f; H[6] += g; H[7] += h;
  }
  return [...H].map(x => x.toString(16).padStart(8, '0')).join('');
}
const CLOCK = ['Analysis begun on', 'Analysis ended on', 'Total elapsed time'];
const withoutClock = u => { const s = latin1(u).split('\n').filter(l => !CLOCK.some(c => l.includes(c))).join('\n'); const o = new Uint8Array(s.length); for (let i = 0; i < s.length; i++) o[i] = s.charCodeAt(i); return o; };

// ------------------------------------------------------------------ the RustGIS host side, in a worker
const HOST_SRC = `
function runPlugin(mod, request) {
  let mem = null, pending = null; const logs = [];
  const enc = new TextEncoder(), dec = new TextDecoder();
  const imports = { rustgis: {
    call(ptr, len) {
      const req = JSON.parse(dec.decode(new Uint8Array(mem.buffer, ptr, len)));
      // A tool makes no host requests (RustGIS: "tools can't use ..."); commands may notify.
      const reply = req.op === 'host.notify' ? { ok: null } : { error: "tools can't use '" + req.op + "': they work on the layers they're given" };
      pending = enc.encode(JSON.stringify(reply)); return pending.length;
    },
    result(ptr) { new Uint8Array(mem.buffer, ptr, pending.length).set(pending); pending = null; },
    log(ptr, len) { logs.push(dec.decode(new Uint8Array(mem.buffer, ptr, len))); }
  }};
  const inst = new WebAssembly.Instance(mod, imports);       // a fresh instance per call, as RustGIS does
  mem = inst.exports.memory;
  const bytes = enc.encode(JSON.stringify(request));
  const ptr = inst.exports.rustgis_alloc(bytes.length);
  new Uint8Array(mem.buffer, ptr, bytes.length).set(bytes);
  let r;
  try { r = BigInt.asUintN(64, inst.exports.rustgis_call(ptr, bytes.length)); }
  catch (e) { return { trap: String(e && e.message || e), logs, memoryMB: mem.buffer.byteLength / 1048576 }; }  // RustGIS reports a trap as an error
  const rp = Number(r >> 32n), rl = Number(r & 0xffffffffn);
  return { reply: JSON.parse(dec.decode(new Uint8Array(mem.buffer, rp, rl))), logs, memoryMB: mem.buffer.byteLength / 1048576 };
}
let pluginMod = null, epaMod = null, ready = null;
onmessage = async (e) => { const d = e.data;
  try {
    if (d.kind === 'init') { ready = (async () => { pluginMod = await WebAssembly.compile(d.plugin); epaMod = await WebAssembly.compile(d.epa); })(); await ready; postMessage({ id: d.id, ok: true }); return; }
    await ready;
    if (d.kind === 'plugin') { const t0 = performance.now(); const r = runPlugin(pluginMod, d.request); postMessage({ id: d.id, ms: performance.now() - t0, ...r }); }
    else if (d.kind === 'epa') { const t0 = performance.now(); const r = self.SwmmEngineCore.runLegacy(epaMod, 'm.inp', d.files);
      postMessage({ id: d.id, ms: performance.now() - t0, rc: r.rc, trap: r.trap, rpt: r.rpt, out: r.out }); }
  } catch (err) { postMessage({ id: d.id, error: String(err && err.message || err) }); }
};`;
let worker = null, seq = 0, workerReady = null; const waiting = new Map();
const RUN_LIMIT_MS = 10 * 60 * 1000;   // the page stops a run (plugin or EPA) after ten minutes
function startWorker() {
  const src = document.getElementById('src-core').textContent + '\n' + HOST_SRC;
  worker = new Worker(URL.createObjectURL(new Blob([src], { type: 'text/javascript' })));
  worker.onmessage = e => { const w = waiting.get(e.data.id); if (!w) return; waiting.delete(e.data.id); e.data.error ? w.rej(new Error(e.data.error)) : w.res(e.data); };
  worker.onerror = e => { e.preventDefault && e.preventDefault(); restartWorker('the engine stopped: ' + (e.message || 'worker error')); };
  const id = ++seq;
  workerReady = new Promise((res, rej) => { waiting.set(id, { res, rej }); worker.postMessage({ kind: 'init', id, plugin: PLUGIN.slice().buffer, epa: EPA.slice().buffer }); });
  return workerReady;
}
// One job at a time on the worker. A job that outlives RUN_LIMIT_MS, or is cancelled, takes the
// worker with it: terminating is the only way to stop WebAssembly mid-call. A fresh worker
// compiles the two modules again (about a second) and takes the next job.
async function call(msg) {
  await workerReady;
  return new Promise((res, rej) => {
    const id = ++seq; let timer = null;
    const fin = f => v => { clearTimeout(timer); waiting.delete(id); f(v); };
    waiting.set(id, { res: fin(res), rej: fin(rej) });
    timer = setTimeout(() => restartWorker(`stopped after ${RUN_LIMIT_MS / 60000} minutes, the page's limit`), RUN_LIMIT_MS);
    worker.postMessage({ ...msg, id });
  });
}
function restartWorker(why) {
  try { worker.terminate(); } catch (e) {}
  const err = Object.assign(new Error(why), { stopped: true });
  const ws = [...waiting.values()]; waiting.clear(); ws.forEach(w => w.rej(err));
  return startWorker();
}

// ------------------------------------------------------------------ state
// S.run is the run on the map: its input bundle (copied and hashed when the run started), the
// plugin's reply, and, once asked for, EPA's run of the same bundle. Nothing about a run is read
// from the form after it starts, so a reply can never be filed under the wrong model.
const S = { data: [], model: null, layers: [], tables: [], messages: [], active: null, sym: {}, sel: null, view: null, planar: false,
  run: null, pending: null, dirty: false, verify: null, time: { on: false, k: 0, vars: { nodes: 0, links: 0, subcatchments: 4 }, ranges: {} } };
let RUN_SEQ = 0;
const demoSel = $('#demo');
A.demos.forEach((d, i) => { const o = document.createElement('option'); o.value = i; o.textContent = d.name; demoSel.appendChild(o); });
const userOpt = document.createElement('option'); userOpt.value = 'user'; userOpt.textContent = 'Your file (choose below)'; userOpt.disabled = true; demoSel.appendChild(userOpt);
function demoBytes(d) { if (!d._bytes) { const bin = atob(d.b64); d._bytes = new Uint8Array(bin.length); for (let i = 0; i < bin.length; i++) d._bytes[i] = bin.charCodeAt(i); } return d._bytes; }
function setModelFromDemo(i) {
  const d = A.demos[i]; S.model = { name: d.file, bytes: demoBytes(d), demo: d };
  $('#epsg').value = d.epsg || ''; $('#alon').value = A.anchor[0]; $('#alat').value = A.anchor[1];
  fuelNote();
}
function fuelNote() {
  const d = S.model && S.model.demo, el = $('#fuelnote');
  if (!d || !d.fuel) { el.hidden = false; el.className = 'box panel';
    el.innerHTML = 'Runs here have no fuel limit. Inside RustGIS 0.5.0 a tool call gets 3×10<sup>10</sup> fuel, burned as the module executes; this page cannot meter your file, so a large model may stop there with <i>it ran too long</i>. Map only always fits.'; return; }
  const f = d.fuel; el.hidden = false; el.className = 'box ' + (f.fits ? 'ok' : 'hi');
  el.innerHTML = f.fits ? `In RustGIS this run uses <b>${(f.fuel / 1e9).toFixed(2)}×10<sup>9</sup></b> fuel, <b>${f.pct_of_budget}%</b> of a tool call's budget (measured in wasmi 2.0.0, configured as RustGIS 0.5.0).`
    : `This run needs <b>${(f.fuel / 1e9).toFixed(1)}×10<sup>9</sup></b> fuel, <b>${f.pct_of_budget}%</b> of RustGIS's per-call budget: RustGIS stops it with <i>it ran too long</i>. It runs here, where nothing meters it; <b>Map only</b> fits in RustGIS.`;
}
demoSel.addEventListener('change', () => { if (demoSel.value !== 'user') { setModelFromDemo(+demoSel.value); S.data = []; $('#datafiles').value = ''; renderDataList(); runTool('run-swmm'); } });
$('#datafiles').addEventListener('change', async e => {
  S.data = await Promise.all([...e.target.files].slice(0, 3).map(async f => ({ name: f.name, bytes: new Uint8Array(await f.arrayBuffer()) })));
  if (e.target.files.length > 3) toast('The tool takes three data files; the first three were kept.');
  renderDataList(); markDirty();
});
function renderDataList() { $('#datalist').textContent = S.data.length ? 'Given: ' + S.data.map(d => `${d.name} (${fmt(d.bytes.length)} bytes)`).join(', ') : ''; }
$('#file').addEventListener('change', async e => {
  const f = e.target.files[0]; if (!f) return;
  S.model = { name: f.name, bytes: new Uint8Array(await f.arrayBuffer()), demo: null };
  userOpt.disabled = false; userOpt.textContent = 'Your file: ' + f.name; demoSel.value = 'user'; $('#epsg').value = ''; fuelNote(); runTool('run-swmm');
});
function markDirty() { if (S.run && !S.pending) { S.dirty = true; renderSummary(); renderVerifyIdle(); } }
['#epsg', '#alon', '#alat'].forEach(s => $(s).addEventListener('input', markDirty));
$('#report').addEventListener('change', markDirty);

// ------------------------------------------------------------------ running a tool
// The inputs of one run, frozen: the model and its data files copied and hashed, the form's values.
function makeBundle(tool) {
  const copy = (name, bytes) => Object.freeze({ name, bytes: bytes.slice(), size: bytes.length, sha: sha256(bytes) });
  return Object.freeze({ tool, model: copy(S.model.name, S.model.bytes), demo: S.model.demo ? S.model.demo.file : null,
    data: Object.freeze(tool === 'run-swmm' ? S.data.slice(0, 3).map(d => copy(d.name, d.bytes)) : []),
    params: Object.freeze({ epsg: $('#epsg').value.trim(), anchor_lon: +$('#alon').value, anchor_lat: +$('#alat').value, report: tool === 'run-swmm' && $('#report').checked }) });
}
function requestOf(b) {
  const params = { model: b.model.name, epsg: b.params.epsg, anchor_lon: b.params.anchor_lon, anchor_lat: b.params.anchor_lat };
  if (b.tool === 'run-swmm') params.report = b.params.report;
  const request = { abi: 1, kind: 'tool', id: b.tool, params, files: { model: { name: b.model.name, data: bytesB64(b.model.bytes) } }, layers: {}, lang: 'en' };
  b.data.forEach((d, i) => { params['data' + (i + 1)] = d.name; request.files['data' + (i + 1)] = { name: d.name, data: bytesB64(d.bytes) }; });
  return request;
}
function showRunning(p) {
  $('#runline').innerHTML = `<span class="spin"></span> ${p.tool === 'run-swmm' ? 'Running' : 'Mapping'} ${esc(p.bundle.model.name)} in the plugin module… <span class="mono" id="elapsed">0.0 s</span> <button class="btn small" id="cancelbtn">Cancel</button>`;
  $('#cancelbtn').addEventListener('click', () => cancelRun('cancelled'));
  p.timer = setInterval(() => { const el = $('#elapsed'); if (el && S.pending === p) el.textContent = ((performance.now() - p.started) / 1000).toFixed(1) + ' s'; }, 200);
}
// Stop the job in flight (a plugin run, or EPA's run for Verify or the time series).
function cancelRun(why) {
  const p = S.pending, busy = p || S.epaBusy;
  if (!busy) return;
  if (p) { clearInterval(p.timer); S.pending = null; }
  restartWorker(why === 'superseded' ? 'superseded by a newer run' : 'cancelled');
  if (p && why === 'cancelled') {
    $('#runline').innerHTML = `<span class="chip warn">cancelled</span> ${esc(p.bundle.model.name)} after ${((performance.now() - p.started) / 1000).toFixed(1)} s. ${S.run ? `The map still shows run #${S.run.id} (${esc(S.run.bundle.model.name)}).` : ''}`;
    $('#runbtn').disabled = false; $('#mapbtn').disabled = false;
  }
}
async function runTool(tool) {
  if (!S.model) return;
  cancelRun('superseded');                                   // a new run replaces the one in flight
  const bundle = makeBundle(tool), p = { id: ++RUN_SEQ, tool, bundle, started: performance.now() };
  S.pending = p; showRunning(p);
  let r = null, err = null;
  try { r = await call({ kind: 'plugin', request: requestOf(bundle) }); } catch (e) { err = e; }
  if (S.pending !== p) return;                               // cancelled or superseded: its answer is dropped
  clearInterval(p.timer); S.pending = null;
  const run = { id: p.id, tool, bundle, request: requestOf(bundle), finished: new Date(), ms: r ? r.ms : performance.now() - p.started,
    mem: r ? r.memoryMB : null, logs: r ? r.logs : [], reply: null, epa: null, series: null };
  if (err) run.reply = { error: err.message };
  else { if (r.trap) r.reply = { error: 'The plugin stopped with an error (' + r.trap + ').' + (r.logs.length ? '\n' + r.logs.join('\n') : '') }; run.reply = r.reply; }
  S.run = run; S.verify = null; S.dirty = false; S.time.on = false; S.time.k = 0;
  if (run.reply.error) { S.layers = []; S.tables = []; S.messages = run.reply.error.split('\n');
    $('#runline').innerHTML = `<span class="chip warn">${err && err.stopped ? 'stopped' : 'error'}</span> ${esc(run.reply.error.split('\n')[0])}`; }
  else {
    const ok = run.reply.ok;
    S.layers = ok.layers.map(l => ({ name: l.name, fc: l.geojson, kind: kindOf(l.name), visible: true }));
    S.tables = ok.tables.map(t => ({ name: t.name, fc: t.geojson, kind: 'table', visible: false }));
    S.messages = ok.messages;
    S.planar = !!(ok.layers[0] && ok.layers[0].geojson.crs);
    const rptT = S.tables.find(t => / report$/.test(t.name));
    S.rptText = rptT ? rptT.fc.features.map(f => f.properties.text).join('\n') : '';
    S.sym = {}; for (const L of S.layers) S.sym[L.name] = defaultField(L);
    S.active = (S.layers.find(l => l.kind === 'nodes') || S.layers[0] || S.tables[0] || {}).name || null;
    S.sel = null; S.view = null;
    const n = S.layers.reduce((a, l) => a + l.fc.features.length, 0);
    $('#runline').innerHTML = `<span class="chip ok">${tool === 'run-swmm' ? 'ran' : 'mapped'}</span> run #${run.id} · ${esc(bundle.model.name)} · ${fmt(n)} features · ${fmt(run.ms, 0)} ms in your browser`;
  }
  $('#runbtn').disabled = false; $('#mapbtn').disabled = false;
  GIF.caption(`${tool === 'run-swmm' ? 'Ran' : 'Mapped'} ${bundle.model.name} with the swmm5-rust plugin`);
  renderLayers(); renderMessages(); renderTable(); drawMap(); renderLegend(); renderPluginTab(); renderVerifyIdle(); renderSummary(); rebuildFind(); renderSeries();
  // time series come from EPA's run of the same bundle; for a quick model, fetch them now
  if (tool === 'run-swmm' && !run.reply.error && run.ms < 2000) loadSeries(run, true);
}
const kindOf = n => / subcatchments$/.test(n) ? 'subcatchments' : / links$/.test(n) ? 'links' : / nodes$/.test(n) ? 'nodes' : / rain gages$/.test(n) ? 'gages' : 'table';
function numericFields(L) {
  const seen = new Map();
  for (const f of L.fc.features) for (const [k, v] of Object.entries(f.properties || {})) if (typeof v === 'number' && k !== 'line') seen.set(k, true);
  return [...seen.keys()];
}
function defaultField(L) {
  const fs = numericFields(L), pick = re => fs.find(k => re.test(k));
  if (L.kind === 'nodes') return pick(/^max_depth_/) || pick(/^flood_hr$/) || pick(/^invert_/) || '';
  if (L.kind === 'links') return pick(/^max_full_flow$/) || pick(/^max_flow_/) || pick(/^geom1_/) || '';
  if (L.kind === 'subcatchments') return pick(/^runoff_(in|mm)$/) || pick(/^pct_imperv$/) || '';
  return '';
}
$('#runbtn').addEventListener('click', () => runTool('run-swmm'));
$('#mapbtn').addEventListener('click', () => runTool('map-swmm'));

// ------------------------------------------------------------------ layers, messages, table
function renderLayers() {
  const ul = $('#layers'); ul.innerHTML = '';
  if (!S.layers.length && !S.tables.length) { ul.innerHTML = '<li style="cursor:default;color:var(--ink-faint)">No layers yet.</li>'; return; }
  for (const L of [...S.layers, ...S.tables]) {
    const li = document.createElement('li'); if (L.name === S.active) li.classList.add('active');
    li.tabIndex = 0; li.setAttribute('role', 'button'); li.setAttribute('aria-pressed', L.name === S.active ? 'true' : 'false');
    const isLayer = L.kind !== 'table';
    li.innerHTML = `${isLayer ? `<input type="checkbox" ${L.visible ? 'checked' : ''} aria-label="Show ${esc(L.name)}">` : '<span class="mono" style="font-size:11px;color:var(--ink-faint)">tbl</span>'}<span style="min-width:0;overflow-wrap:anywhere">${esc(L.name)}</span><span class="n">${fmt(L.fc.features.length)}</span>`;
    if (isLayer) {
      const fs = numericFields(L);
      const sym = document.createElement('div'); sym.className = 'sym';
      sym.innerHTML = `<span>color by</span><select aria-label="Field to color ${esc(L.name)} by"><option value="">single color</option>${fs.map(k => `<option ${S.sym[L.name] === k ? 'selected' : ''}>${esc(k)}</option>`).join('')}</select>`;
      sym.querySelector('select').addEventListener('change', e => { S.sym[L.name] = e.target.value; S.active = L.name; renderLayers(); renderTable(); drawMap(); renderLegend(); GIF.caption(`${L.name}: colored by ${e.target.value || 'a single color'}`); });
      sym.addEventListener('click', e => e.stopPropagation()); sym.addEventListener('keydown', e => e.stopPropagation());
      li.appendChild(sym);
      li.querySelector('input').addEventListener('click', e => { e.stopPropagation(); L.visible = e.target.checked; drawMap(); });
      li.querySelector('input').addEventListener('keydown', e => e.stopPropagation());
    }
    const pickLayer = () => { S.active = L.name; renderLayers(); renderTable(); renderLegend(); const n = [...$$('#layers li')].find(x => x.textContent.includes(L.name)); if (n) n.focus(); };
    li.addEventListener('click', pickLayer);
    li.addEventListener('keydown', e => { if (e.target === li && (e.key === 'Enter' || e.key === ' ')) { e.preventDefault(); pickLayer(); } });
    ul.appendChild(li);
  }
}
function renderMessages() { $('#msgs').innerHTML = S.messages.map(m => `<li>${esc(m)}</li>`).join('') || '<li>Nothing yet.</li>'; }
let sortKey = null, sortDir = 1;
function activeLayer() { return [...S.layers, ...S.tables].find(l => l.name === S.active) || null; }
function columns(L) { const cols = []; const seen = new Set(); for (const f of L.fc.features) for (const k of Object.keys(f.properties || {})) if (!seen.has(k)) { seen.add(k); cols.push(k); } return cols; }
// The attribute table draws only the rows in view (plus a margin), so a layer of any size scrolls
// whole: no cutoff. Rows can be filtered by any text in them and sorted by any column.
const TV = { L: null, cols: [], rows: [], rowH: 28, first: -1, last: -1 };
function renderTable() {
  const L = activeLayer(), t = $('#attr');
  $('#tabname').textContent = L ? L.name : '';
  if (!L) { TV.L = null; TV.rows = []; $('#tabcount').textContent = ''; t.innerHTML = '<tbody><tr><td class="t" style="color:var(--ink-faint)">Run a model to fill the table.</td></tr></tbody>'; return; }
  const cols = columns(L), q = $('#tabfilter').value.trim().toLowerCase();
  let rows = L.fc.features.map((f, i) => ({ i, p: f.properties || {} }));
  if (q) rows = rows.filter(r => cols.some(k => r.p[k] != null && String(r.p[k]).toLowerCase().includes(q)));
  if (sortKey && cols.includes(sortKey)) rows.sort((a, b) => { const x = a.p[sortKey], y = b.p[sortKey]; if (x == null) return 1; if (y == null) return -1; return (x > y ? 1 : x < y ? -1 : 0) * sortDir; });
  Object.assign(TV, { L, cols, rows, first: -1, last: -1, num: new Set(cols.filter(k => L.fc.features.some(f => typeof (f.properties || {})[k] === 'number'))) });
  $('#tabcount').textContent = q ? `${fmt(rows.length)} of ${fmt(L.fc.features.length)} rows` : `${fmt(L.fc.features.length)} rows`;
  t.innerHTML = `<thead><tr>${cols.map(k => `<th class="${TV.num.has(k) ? '' : 't'}" data-k="${esc(k)}" tabindex="0" aria-sort="${sortKey === k ? (sortDir > 0 ? 'ascending' : 'descending') : 'none'}">${esc(k)}${sortKey === k ? (sortDir > 0 ? ' ▲' : ' ▼') : ''}</th>`).join('')}</tr></thead><tbody></tbody>`;
  t.querySelectorAll('th').forEach(th => { const go = () => { const k = th.dataset.k; if (sortKey === k) sortDir = -sortDir; else { sortKey = k; sortDir = 1; } renderTable(); };
    th.addEventListener('click', go); th.addEventListener('keydown', e => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); go(); } }); });
  paintRows(true);
}
function paintRows(force) {
  const t = $('#attr'), tb = t.tBodies[0], sc = $('#tablescroll'); if (!tb || !TV.L) return;
  const n = TV.rows.length, top = sc.scrollTop, h = Math.max(sc.clientHeight, 360);   // (the box is 340 px at most; before rows exist it is short)
  const first = Math.max(0, Math.floor(top / TV.rowH) - 12), last = Math.min(n, Math.ceil((top + h) / TV.rowH) + 12);
  if (!force && first === TV.first && last === TV.last) return;
  TV.first = first; TV.last = last;
  const ncol = TV.cols.length, sel = S.sel && S.sel.layer === TV.L.name ? S.sel.i : -1;
  const spacer = px => px > 0 ? `<tr class="spacer" aria-hidden="true"><td colspan="${ncol}" style="height:${px}px;padding:0;border:0"></td></tr>` : '';
  tb.innerHTML = spacer(first * TV.rowH) + TV.rows.slice(first, last).map(r => `<tr data-i="${r.i}" class="${r.i === sel ? 'sel' : ''}">${TV.cols.map(k => { const v = r.p[k]; return `<td class="${typeof v === 'number' ? '' : 't'}">${esc(fmt(v))}</td>`; }).join('')}</tr>`).join('') + spacer((n - last) * TV.rowH);
  if (!n) tb.innerHTML = `<tr><td class="t" colspan="${ncol}" style="color:var(--ink-faint)">No rows match.</td></tr>`;
  const tr = tb.querySelector('tr[data-i]'); if (tr) { const rh = tr.getBoundingClientRect().height; if (rh > 10 && Math.abs(rh - TV.rowH) > 0.5) { TV.rowH = rh; paintRows(true); } }
}
let paintQueued = false;
$('#tablescroll').addEventListener('scroll', () => { if (paintQueued) return; paintQueued = true; requestAnimationFrame(() => { paintQueued = false; paintRows(); }); });
$('#attr').addEventListener('click', e => { const tr = e.target.closest('tr[data-i]'); if (!tr || !TV.L) return; selectFeature(TV.L, +tr.dataset.i, { scroll: false }); });
$('#tabfilter').addEventListener('input', () => { $('#tablescroll').scrollTop = 0; renderTable(); });
// arrow keys move the selection through the table's rows, in the order shown
$('#tablescroll').addEventListener('keydown', e => {
  if (!TV.L || !TV.rows.length || (e.key !== 'ArrowDown' && e.key !== 'ArrowUp' && e.key !== 'Enter')) return;
  e.preventDefault();
  let pos = S.sel && S.sel.layer === TV.L.name ? TV.rows.findIndex(r => r.i === S.sel.i) : -1;
  if (e.key === 'Enter') { if (pos >= 0) zoomToSelection(); return; }
  pos = Math.max(0, Math.min(TV.rows.length - 1, pos + (e.key === 'ArrowDown' ? 1 : -1)));
  selectFeature(TV.L, TV.rows[pos].i, { scroll: true });
});
function scrollToRow(i) {
  const pos = TV.rows.findIndex(r => r.i === i); if (pos < 0) return; const sc = $('#tablescroll');
  const y = pos * TV.rowH; if (y < sc.scrollTop + 30 || y > sc.scrollTop + sc.clientHeight - 2 * TV.rowH) sc.scrollTop = Math.max(0, y - sc.clientHeight / 2);
  paintRows(true);
}
function csvOf(L) {
  const cols = columns(L); const q = v => { if (v == null) return ''; const s = String(v); return /[",\n]/.test(s) ? '"' + s.replace(/"/g, '""') + '"' : s; };
  return [cols.join(','), ...L.fc.features.map(f => cols.map(k => q((f.properties || {})[k])).join(','))].join('\n') + '\n';
}

// ------------------------------------------------------------------ the map
const cv = $('#map'), ctx = cv.getContext('2d');
const R = 6378137;
const proj = c => S.planar ? [c[0], c[1]] : [c[0] * Math.PI / 180 * R, Math.log(Math.tan(Math.PI / 4 + c[1] * Math.PI / 360)) * R];
function bounds() {
  let b = [Infinity, Infinity, -Infinity, -Infinity];
  const add = c => { const p = proj(c); b[0] = Math.min(b[0], p[0]); b[1] = Math.min(b[1], p[1]); b[2] = Math.max(b[2], p[0]); b[3] = Math.max(b[3], p[1]); };
  for (const L of S.layers) for (const f of L.fc.features) eachCoord(f.geometry, add);
  return b[0] === Infinity ? null : b;
}
function eachCoord(g, fn) { if (!g) return; if (g.type === 'Point') fn(g.coordinates); else if (g.type === 'LineString') g.coordinates.forEach(fn); else if (g.type === 'Polygon') g.coordinates.forEach(r => r.forEach(fn)); }
function size() { const r = cv.getBoundingClientRect(), dpr = window.devicePixelRatio || 1; return { w: r.width, h: r.height, dpr }; }
function fit() {
  const b = bounds(), { w, h } = size(); if (!b) { S.view = null; return; }
  const bw = Math.max(b[2] - b[0], 1e-6), bh = Math.max(b[3] - b[1], 1e-6);
  S.view = { cx: (b[0] + b[2]) / 2, cy: (b[1] + b[3]) / 2, s: 0.86 * Math.min(w / bw, h / bh) };
}
const toScreen = (p, w, h) => [w / 2 + (p[0] - S.view.cx) * S.view.s, h / 2 - (p[1] - S.view.cy) * S.view.s];
function classes(L) {
  const tc = timeClasses(L); if (tc) return tc;
  const k = S.sym[L.name]; if (!k) return null;
  const vals = L.fc.features.map(f => (f.properties || {})[k]).filter(v => typeof v === 'number').sort((a, b) => a - b);
  if (!vals.length) return null;
  const uniq = [...new Set(vals)];
  let br;
  if (uniq.length <= 5) br = uniq;
  else { br = []; for (let i = 1; i <= 5; i++) br.push(vals[Math.min(vals.length - 1, Math.ceil(i * vals.length / 5) - 1)]); br = [...new Set(br)]; }
  return { field: k, breaks: br, min: vals[0] };
}
function colourFor(cl, v) { const ramp = TH().ramp; if (v == null || typeof v !== 'number') return null; let i = cl.breaks.findIndex(b => v <= b); if (i < 0) i = cl.breaks.length - 1; const idx = cl.breaks.length === 1 ? 4 : Math.round(i * 4 / (cl.breaks.length - 1)); return ramp[idx]; }
function drawMap() {
  const { w, h, dpr } = size(); if (!w) return;
  if (cv.width !== Math.round(w * dpr) || cv.height !== Math.round(h * dpr)) { cv.width = Math.round(w * dpr); cv.height = Math.round(h * dpr); }
  const T = TH(); ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.fillStyle = T.face; ctx.fillRect(0, 0, w, h);
  if (!S.layers.length) { ctx.fillStyle = T.tick; ctx.font = '14px Barlow, sans-serif'; ctx.textAlign = 'center'; ctx.fillText(S.messages.length ? 'Nothing to draw.' : 'Loading the plugin…', w / 2, h / 2); return; }
  if (!S.view) fit();
  // grid
  ctx.strokeStyle = T.grid; ctx.lineWidth = 1; const gs = niceStep(120 / S.view.s);
  const x0 = S.view.cx - w / 2 / S.view.s, y0 = S.view.cy - h / 2 / S.view.s;
  for (let x = Math.floor(x0 / gs) * gs; x < x0 + w / S.view.s; x += gs) { const sx = toScreen([x, 0], w, h)[0]; ctx.beginPath(); ctx.moveTo(sx, 0); ctx.lineTo(sx, h); ctx.stroke(); }
  for (let y = Math.floor(y0 / gs) * gs; y < y0 + h / S.view.s; y += gs) { const sy = toScreen([0, y], w, h)[1]; ctx.beginPath(); ctx.moveTo(0, sy); ctx.lineTo(w, sy); ctx.stroke(); }
  const order = ['subcatchments', 'links', 'nodes', 'gages'];
  for (const kind of order) for (const L of S.layers.filter(l => l.kind === kind && l.visible)) {
    const cl = classes(L);
    L.fc.features.forEach((f, i) => {
      const g = f.geometry; if (!g) return; const v = cl ? valueOf(L, f, cl) : null; const c = cl ? colourFor(cl, v) : null;
      const sel = S.sel && S.sel.layer === L.name && S.sel.i === i;
      if (g.type === 'Polygon') {
        ctx.beginPath(); g.coordinates.forEach(r => r.forEach((q, j) => { const p = toScreen(proj(q), w, h); j ? ctx.lineTo(p[0], p[1]) : ctx.moveTo(p[0], p[1]); })); ctx.closePath();
        ctx.fillStyle = c ? c + (THEME === 'dark' ? '99' : '70') : T.poly; ctx.fill(); ctx.strokeStyle = sel ? T.sel : T.polyLine; ctx.lineWidth = sel ? 3 : 1; ctx.stroke();
      } else if (g.type === 'LineString') {
        ctx.beginPath(); g.coordinates.forEach((q, j) => { const p = toScreen(proj(q), w, h); j ? ctx.lineTo(p[0], p[1]) : ctx.moveTo(p[0], p[1]); });
        ctx.strokeStyle = sel ? T.sel : (c || T.link); ctx.lineWidth = sel ? 5 : (c ? 3 : 2); ctx.lineCap = 'round'; ctx.stroke();
      } else if (g.type === 'Point') {
        const p = toScreen(proj(g.coordinates), w, h); const t = (f.properties || {}).type || '';
        ctx.fillStyle = sel ? T.sel : (kind === 'gages' ? T.gage : (c || T.node)); ctx.strokeStyle = T.stroke; ctx.lineWidth = 1.5;
        ctx.beginPath(); const r = sel ? 7 : 4.5;
        if (kind === 'gages') { ctx.moveTo(p[0], p[1] - 8); ctx.lineTo(p[0] + 7, p[1] + 5); ctx.lineTo(p[0] - 7, p[1] + 5); ctx.closePath(); }
        else if (t === 'OUTFALL') { ctx.moveTo(p[0], p[1] + r + 2); ctx.lineTo(p[0] + r + 2, p[1] - r); ctx.lineTo(p[0] - r - 2, p[1] - r); ctx.closePath(); }
        else if (t === 'STORAGE') ctx.rect(p[0] - r - 1, p[1] - r - 1, 2 * r + 2, 2 * r + 2);
        else if (t === 'DIVIDER') { ctx.moveTo(p[0], p[1] - r - 2); ctx.lineTo(p[0] + r + 2, p[1]); ctx.lineTo(p[0], p[1] + r + 2); ctx.lineTo(p[0] - r - 2, p[1]); ctx.closePath(); }
        else ctx.arc(p[0], p[1], r, 0, 2 * Math.PI);
        ctx.fill(); ctx.stroke();
      }
    });
  }
  // scale bar (US customary on Web Mercator; map units on a declared EPSG system)
  const lat = S.planar ? 0 : (2 * Math.atan(Math.exp(S.view.cy / R)) - Math.PI / 2);
  const unitPerPx = 1 / S.view.s; let label, len;
  if (S.planar) { const st = niceStep(unitPerPx * 120); len = st / unitPerPx; label = `${fmt(st)} map units (EPSG:${esc($('#epsg').value || '?')})`; }
  else { const ftPerPx = unitPerPx * Math.cos(lat) / 0.3048; const st = niceStep(ftPerPx * 120); len = st / ftPerPx; label = st >= 5280 ? `${fmt(st / 5280, 2)} mi` : `${fmt(st)} ft`; }
  ctx.fillStyle = T.tick; ctx.strokeStyle = T.axis; ctx.lineWidth = 2; const bx = w - len - 18, by = h - 18;
  ctx.beginPath(); ctx.moveTo(bx, by); ctx.lineTo(bx + len, by); ctx.moveTo(bx, by - 5); ctx.lineTo(bx, by); ctx.moveTo(bx + len, by - 5); ctx.lineTo(bx + len, by); ctx.stroke();
  ctx.font = '11px "JetBrains Mono", monospace'; ctx.textAlign = 'right'; ctx.fillText(label, w - 18, by - 8);
}
function niceStep(x) { const p = Math.pow(10, Math.floor(Math.log10(x))); const m = x / p; return (m < 1.5 ? 1 : m < 3.5 ? 2 : m < 7.5 ? 5 : 10) * p; }
function renderLegend() {
  const el = $('#legend'), L = activeLayer();
  const lay = L && L.kind !== 'table' ? L : S.layers.find(l => S.sym[l.name]);
  const cl = lay && classes(lay);
  if (!cl) { el.hidden = true; return; }
  const ramp = TH().ramp; el.hidden = false;
  let lo = cl.min;
  el.innerHTML = `<div class="ttl">${esc(lay.name.split(' ').slice(-1)[0])}: ${esc(cl.label || cl.field)}</div>` + cl.breaks.map((b, i) => { const idx = cl.breaks.length === 1 ? 4 : Math.round(i * 4 / (cl.breaks.length - 1)); const s = `<div class="item"><span class="sw" style="background:${ramp[idx]}"></span>${fmt(i === 0 ? lo : cl.breaks[i - 1], 3)} – ${fmt(b, 3)}</div>`; return s; }).join('');
}
// pan / zoom / pick
let drag = null; const ptrs = new Map();
cv.addEventListener('pointerdown', e => { cv.setPointerCapture(e.pointerId); ptrs.set(e.pointerId, [e.offsetX, e.offsetY]); drag = { x: e.offsetX, y: e.offsetY, moved: false, d0: null }; cv.style.cursor = 'grabbing'; });
cv.addEventListener('pointermove', e => {
  if (!ptrs.has(e.pointerId) || !S.view) return; const prev = ptrs.get(e.pointerId); ptrs.set(e.pointerId, [e.offsetX, e.offsetY]);
  if (ptrs.size === 2) { const [a, b] = [...ptrs.values()]; const d = Math.hypot(a[0] - b[0], a[1] - b[1]); if (drag.d0) zoomAt((a[0] + b[0]) / 2, (a[1] + b[1]) / 2, d / drag.d0); drag.d0 = d; drag.moved = true; return; }
  const dx = e.offsetX - prev[0], dy = e.offsetY - prev[1]; if (Math.abs(e.offsetX - drag.x) + Math.abs(e.offsetY - drag.y) > 3) drag.moved = true;
  S.view.cx -= dx / S.view.s; S.view.cy += dy / S.view.s; drawMap();
});
cv.addEventListener('pointerup', e => { ptrs.delete(e.pointerId); cv.style.cursor = 'grab'; if (drag && !drag.moved) pick(e.offsetX, e.offsetY); if (!ptrs.size) drag = null; });
cv.addEventListener('wheel', e => { e.preventDefault(); zoomAt(e.offsetX, e.offsetY, Math.exp(-e.deltaY * 0.0015)); }, { passive: false });
function zoomAt(x, y, k) { if (!S.view) return; const { w, h } = size(); const wx = S.view.cx + (x - w / 2) / S.view.s, wy = S.view.cy - (y - h / 2) / S.view.s; S.view.s *= k; S.view.cx = wx - (x - w / 2) / S.view.s; S.view.cy = wy + (y - h / 2) / S.view.s; drawMap(); }
$('#zin').addEventListener('click', () => { const { w, h } = size(); zoomAt(w / 2, h / 2, 1.5); });
$('#zout').addEventListener('click', () => { const { w, h } = size(); zoomAt(w / 2, h / 2, 1 / 1.5); });
$('#zfit').addEventListener('click', () => { fit(); drawMap(); });
function segDist(p, a, b) { const dx = b[0] - a[0], dy = b[1] - a[1]; const t = Math.max(0, Math.min(1, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / (dx * dx + dy * dy || 1))); return Math.hypot(p[0] - a[0] - t * dx, p[1] - a[1] - t * dy); }
function inPoly(p, ring) { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const a = ring[i], b = ring[j]; if ((a[1] > p[1]) !== (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]) c = !c; } return c; }
function pick(x, y) {
  const { w, h } = size(); const P = [x, y]; let best = null;
  for (const kind of ['gages', 'nodes', 'links', 'subcatchments']) {
    for (const L of S.layers.filter(l => l.kind === kind && l.visible)) L.fc.features.forEach((f, i) => {
      const g = f.geometry; if (!g) return;
      if (g.type === 'Point') { const d = Math.hypot(...toScreen(proj(g.coordinates), w, h).map((v, k) => v - P[k])); if (d < 9 && (!best || d < best.d)) best = { L, i, d }; }
      else if (g.type === 'LineString') { const pts = g.coordinates.map(q => toScreen(proj(q), w, h)); for (let k = 1; k < pts.length; k++) { const d = segDist(P, pts[k - 1], pts[k]); if (d < 6 && (!best || d < best.d)) best = { L, i, d: d + 0.5 }; } }
      else if (g.type === 'Polygon' && !best) { const ring = g.coordinates[0].map(q => toScreen(proj(q), w, h)); if (inPoly(P, ring)) best = { L, i, d: 99 }; }
    });
    if (best && best.d < 99) break;
  }
  if (!best) { S.sel = null; drawMap(); paintRows(true); renderSeriesChart(); return; }
  selectFeature(best.L, best.i, { scroll: true });
}
// one selection for the map, the table and the hydrograph
function selectFeature(L, i, o) {
  S.sel = { layer: L.name, i }; if (S.active !== L.name) { S.active = L.name; renderLayers(); renderTable(); renderLegend(); }
  if (o && o.scroll) scrollToRow(i); else paintRows(true);
  drawMap(); renderSeriesChart();
  GIF.caption(`${L.name}: ${(L.fc.features[i].properties || {}).id}`);
}
function featureBox(g) { let b = [Infinity, Infinity, -Infinity, -Infinity]; eachCoord(g, c => { const p = proj(c); b = [Math.min(b[0], p[0]), Math.min(b[1], p[1]), Math.max(b[2], p[0]), Math.max(b[3], p[1])]; }); return b[0] === Infinity ? null : b; }
function zoomToSelection() {
  const L = S.sel && [...S.layers, ...S.tables].find(l => l.name === S.sel.layer); if (!L) return toast('Select a feature first (click it, or find it by ID).');
  const g = L.fc.features[S.sel.i].geometry; if (!g) return toast(`${L.fc.features[S.sel.i].properties.id} has no coordinates: it is listed in the table "${L.name}".`);
  if (!S.view) fit(); const b = featureBox(g), { w, h } = size(); const all = bounds();
  const span = Math.max(b[2] - b[0], b[3] - b[1]), whole = all ? Math.max(all[2] - all[0], all[3] - all[1]) : span;
  const target = Math.max(span * 3, whole / 25, 1e-6);           // the feature and its neighbors, not one pixel's worth
  S.view = { cx: (b[0] + b[2]) / 2, cy: (b[1] + b[3]) / 2, s: Math.min(w, h) / target };
  drawMap();
}
$('#zsel').addEventListener('click', zoomToSelection);
// keyboard on the map: arrows pan, + and - zoom, 0 fits the model, Z zooms to the selection
cv.addEventListener('keydown', e => {
  if (!S.view) return; const { w, h } = size(), step = 60 / S.view.s; let used = true;
  if (e.key === 'ArrowLeft') S.view.cx -= step; else if (e.key === 'ArrowRight') S.view.cx += step;
  else if (e.key === 'ArrowUp') S.view.cy += step; else if (e.key === 'ArrowDown') S.view.cy -= step;
  else if (e.key === '+' || e.key === '=') zoomAt(w / 2, h / 2, 1.4); else if (e.key === '-' || e.key === '_') zoomAt(w / 2, h / 2, 1 / 1.4);
  else if (e.key === '0') fit(); else if (e.key === 'z' || e.key === 'Z') zoomToSelection(); else used = false;
  if (used) { e.preventDefault(); drawMap(); }
});
// find a feature by ID, across every layer and table
function rebuildFind() {
  const seen = new Set(), opts = [];
  for (const L of [...S.layers, ...S.tables]) { if (/ report$/.test(L.name)) continue; for (const f of L.fc.features) { const id = (f.properties || {}).id; if (id == null || seen.has(id)) continue; seen.add(id); if (opts.length < 5000) opts.push(`<option value="${esc(id)}">${esc(L.name.replace(/^.*? (subcatchments|links|nodes|rain gages)/, '$1'))}</option>`); } }
  $('#findlist').innerHTML = opts.join('');
}
function findId(q) {
  q = String(q || '').trim(); if (!q) return;
  const all = [...S.layers, ...S.tables].filter(L => !/ report$/.test(L.name)), lq = q.toLowerCase();
  let hit = null;
  for (const pass of [x => x === q, x => x.toLowerCase() === lq, x => x.toLowerCase().startsWith(lq), x => x.toLowerCase().includes(lq)]) {
    for (const L of all) { const i = L.fc.features.findIndex(f => (f.properties || {}).id != null && pass(String(f.properties.id))); if (i >= 0) { hit = [L, i]; break; } }
    if (hit) break;
  }
  if (!hit) return toast(`No feature with ID "${q}".`);
  const [L, i] = hit; if (L.kind !== 'table' && !L.visible) { L.visible = true; renderLayers(); }
  selectFeature(L, i, { scroll: true }); if (L.fc.features[i].geometry) zoomToSelection(); else toast(`${L.fc.features[i].properties.id} has no coordinates: its row is in "${L.name}".`);
}
$('#find').addEventListener('keydown', e => { if (e.key === 'Enter') { e.preventDefault(); findId(e.target.value); } });
$('#find').addEventListener('change', e => findId(e.target.value));
new ResizeObserver(() => drawMap()).observe(cv);

// ------------------------------------------------------------------ time series and the time slider
// The plugin returns summaries; the time series are in the output file. They are read from EPA's
// run of the same input bundle (the run Verify makes), with swmm5-rust's own .out reader
// (rpt_compare.js, SWCMP.parseOut). The plugin reports its .out's SHA-256; when EPA's matches it,
// these are the plugin's numbers to the bit, and the page says so. When it does not, it says that.
const FLOW_UNITS = ['CFS', 'GPM', 'MGD', 'CMS', 'LPS', 'MLD'];
const OK = { subcatchments: 'sub', nodes: 'node', links: 'link' };
const objKind = L => OK[L.kind] || (/ subcatchments \(/.test(L.name) ? 'sub' : / links \(/.test(L.name) ? 'link' : / nodes \(/.test(L.name) ? 'node' : null);
function varName(kind, v, pa) { const n = SWCMP.VARS[kind]; return n[v] || (pa.ids.pol[v - n.length] !== undefined ? pa.ids.pol[v - n.length] + (kind === 'sub' ? ' washoff' : ' concentration') : 'variable ' + v); }
function varUnit(kind, v, pa) {
  const us = pa.flowUnits < 3, q = FLOW_UNITS[pa.flowUnits] || 'flow', len = us ? 'ft' : 'm', vol = us ? 'ft³' : 'm³';
  const U = { sub: [us ? 'in/hr' : 'mm/hr', us ? 'in' : 'mm', us ? 'in/day' : 'mm/day', us ? 'in/hr' : 'mm/hr', q, q, len, 'fraction'],
    node: [len, len, vol, q, q, q], link: [q, len, us ? 'ft/s' : 'm/s', vol, 'fraction full'] };
  return U[kind][v] || (kind === 'sub' ? 'washoff' : 'concentration');
}
const nVars = (kind, pa) => kind === 'sub' ? pa.nSubV : kind === 'node' ? pa.nNodeV : pa.nLinkV;
function swmmDate(d) { const t = new Date(Math.round((d - 25569) * 86400000)), z = n => String(n).padStart(2, '0'); return `${z(t.getUTCMonth() + 1)}/${z(t.getUTCDate())}/${t.getUTCFullYear()} ${z(t.getUTCHours())}:${z(t.getUTCMinutes())}`; }
async function loadSeries(run, quiet) {
  if (!ranOk(run) || run.series || run.seriesJob) return;
  run.seriesJob = true; run.seriesError = null; if (S.run === run) renderSeries();
  try {
    const e = await epaFor(run);
    if (e.rc !== 0) throw new Error(`EPA's engine stopped on these inputs (${epaErrors(e)[0] || 'code ' + e.rc}), so there is no output file to read`);
    const pa = SWCMP.parseOut(e.out); if (!pa || !pa.nPer) throw new Error("EPA's run wrote no readable output file");
    const idx = {}; for (const k of ['sub', 'node', 'link']) idx[k] = new Map(pa.ids[k].map((id, i) => [id, i]));
    const date = new Float64Array(pa.nPer), hours = new Float64Array(pa.nPer);
    for (let k = 0; k < pa.nPer; k++) { date[k] = pa.dv.getFloat64(pa.outPos + k * pa.bpp, true); hours[k] = (date[k] - pa.start) * 24; }
    run.series = { pa, idx, date, hours, same: e.outSha === hashOf(run, /^output file/), outSha: e.outSha, cache: new Map() };
  } catch (err) { run.seriesError = err.message; if (!quiet) toast(err.message); }
  run.seriesJob = false;
  if (S.run === run) { S.time.ranges = {}; if (run.series) S.time.k = Math.min(S.time.k, run.series.pa.nPer - 1); renderSeries(); renderSummary(); drawMap(); renderLegend(); }
}
// every object's series for one variable, and its range over the whole run
function column(kind, v) {
  const sr = S.run.series, key = kind + ':' + v; let c = sr.cache.get(key);
  if (!c) { const n = sr.pa.ids[kind].length, arr = []; let lo = Infinity, hi = -Infinity;
    const mag = kind === 'link' && v === 0;          // a link's flow is colored by its size, either direction
    for (let i = 0; i < n; i++) { const a = sr.pa.series(kind, i, v); if (mag) for (let k = 0; k < a.length; k++) a[k] = Math.abs(a[k]); arr.push(a); for (let k = 0; k < a.length; k++) { if (a[k] < lo) lo = a[k]; if (a[k] > hi) hi = a[k]; } }
    c = { arr, lo, hi, mag }; sr.cache.set(key, c); }
  return c;
}
function timeOn() { return S.time.on && S.run && S.run.series; }
function timeClasses(L) {
  if (!timeOn()) return null; const kind = objKind(L); if (!kind || L.kind === 'table') return null;
  const v = S.time.vars[L.kind]; if (v == null || v < 0) return null;
  const sr = S.run.series, c = column(kind, v); if (!(c.hi > c.lo)) return { time: true, kind, v, c, breaks: [c.hi], min: c.lo, label: label(kind, v) };
  const br = []; for (let i = 1; i <= 5; i++) br.push(c.lo + (c.hi - c.lo) * i / 5);
  return { time: true, kind, v, c, breaks: br, min: c.lo, label: label(kind, v) };
  function label(kind, v) { return `${c.mag ? '|flow|' : varName(kind, v, sr.pa)} (${varUnit(kind, v, sr.pa)}) at ${swmmDate(sr.date[S.time.k])}`; }
}
function valueOf(L, f, cl) {
  if (!cl.time) return (f.properties || {})[cl.field];
  const i = S.run.series.idx[cl.kind].get(String((f.properties || {}).id)); return i === undefined ? null : cl.c.arr[i][S.time.k];
}
let playTimer = null;
function stopPlay() { if (playTimer) { clearInterval(playTimer); playTimer = null; } const b = $('#tsplay'); if (b) { b.textContent = '▶'; b.setAttribute('aria-label', 'Play'); } }
function onTime() {
  const sr = S.run && S.run.series; if (!sr) return;
  const sl = $('#tslider'); if (sl) sl.value = S.time.k;
  const tl = $('#tstime'); if (tl) tl.textContent = `${swmmDate(sr.date[S.time.k])} · ${fmt(sr.hours[S.time.k], 2)} h from the report start · period ${S.time.k + 1} of ${fmt(sr.pa.nPer)}`;
  if (S.time.on) { drawMap(); renderLegend(); }
  renderSeriesChart();
}
function renderSeries() {
  const card = $('#tscard'), run = S.run; stopPlay();
  if (!ranOk(run)) { card.hidden = true; return; } card.hidden = false;
  const body = $('#tsbody');
  if (run.seriesJob) { body.innerHTML = `<div class="box panel"><span class="spin"></span> Reading the time series: running EPA SWMM 5.2.4 on run #${run.id}'s inputs…</div>`; return; }
  if (!run.series) {
    body.innerHTML = `${run.seriesError ? `<div class="box warn">${esc(run.seriesError)}</div>` : ''}<div class="box panel" style="margin-top:${run.seriesError ? 8 : 0}px">The plugin returns summary results. For hydrographs and the time slider, the page runs EPA's engine on run #${run.id}'s inputs and reads its output file; Verify checks that file against the plugin's, bit for bit. <button class="btn small" id="tsload">Load time series</button></div>`;
    $('#tsload').addEventListener('click', () => loadSeries(run, false)); return;
  }
  const sr = run.series, pa = sr.pa;
  const kinds = [['nodes', 'node'], ['links', 'link'], ['subcatchments', 'sub']].filter(([K, k]) => S.layers.some(L => L.kind === K) && pa.ids[k].length);
  const sel = (K, k) => `<label class="tsvar">${K}<select data-k="${K}" aria-label="Color ${K} by">${Array.from({ length: nVars(k, pa) }, (_, v) => `<option value="${v}" ${S.time.vars[K] === v ? 'selected' : ''}>${esc(varName(k, v, pa))}</option>`).join('')}</select></label>`;
  body.innerHTML = `<div class="box ${sr.same ? 'ok' : 'warn'}" style="font-size:12.5px">${sr.same
      ? `Read from EPA's run of run #${run.id}'s inputs. Its output file's SHA-256 (${sr.outSha.slice(0, 12)}…) equals the one the plugin reported, so these are the plugin's numbers, bit for bit.`
      : `Read from EPA's run of run #${run.id}'s inputs. Its output file does <b>not</b> match the plugin's (SHA-256 ${sr.outSha.slice(0, 12)}… against ${String(hashOf(run, /^output file/) || '').slice(0, 12)}…): these are EPA's numbers, not the plugin's. See Verify.`}</div>
    <div class="tsrow"><button class="btn small" id="tsplay" aria-label="Play">▶</button>
      <input type="range" id="tslider" min="0" max="${pa.nPer - 1}" value="${S.time.k}" aria-label="Reporting period" style="flex:1 1 200px;min-width:0">
      <label class="chk"><input type="checkbox" id="tson" ${S.time.on ? 'checked' : ''}> color the map by this moment</label></div>
    <div class="tsrow mono" id="tstime" style="font-size:12px;color:var(--ink-dim)"></div>
    <div class="tsrow">${kinds.map(([K, k]) => sel(K, k)).join('')}</div>
    <div id="tschart" style="margin-top:8px"></div>`;
  $('#tsplay').addEventListener('click', () => {
    if (playTimer) return stopPlay();
    S.time.on = true; $('#tson').checked = true; const step = Math.max(1, Math.ceil(pa.nPer / 240));
    $('#tsplay').textContent = '❚❚'; $('#tsplay').setAttribute('aria-label', 'Pause');
    playTimer = setInterval(() => { S.time.k = (S.time.k + step) % pa.nPer; onTime(); }, 60);
  });
  $('#tslider').addEventListener('input', e => { S.time.k = +e.target.value; if (!S.time.on) { S.time.on = true; $('#tson').checked = true; } onTime(); });
  $('#tson').addEventListener('change', e => { S.time.on = e.target.checked; drawMap(); renderLegend(); });
  $$('#tsbody select[data-k]').forEach(x => x.addEventListener('change', e => { S.time.vars[e.target.dataset.k] = +e.target.value; S.time.on = true; $('#tson').checked = true; drawMap(); renderLegend(); }));
  onTime();
}
// the selected object's hydrograph, with the moment on the slider marked
let chartVar = { node: 0, link: 0, sub: 4 };
function renderSeriesChart() {
  const el = $('#tschart'); if (!el) return; const run = S.run, sr = run && run.series; if (!sr) { el.innerHTML = ''; return; }
  const L = S.sel && [...S.layers, ...S.tables].find(l => l.name === S.sel.layer); const kind = L && objKind(L);
  if (!L || !kind) { el.innerHTML = '<p style="margin:0;font-size:13px;color:var(--ink-dim)">Click a node, link or subcatchment on the map, a row in the table, or find one by ID, to see its time series here.</p>'; return; }
  const f = L.fc.features[S.sel.i], id = String(f.properties.id), i = sr.idx[kind].get(id), pa = sr.pa;
  if (i === undefined) { el.innerHTML = `<p style="margin:0;font-size:13px;color:var(--ink-dim)">${esc(id)} is not in the output file.</p>`; return; }
  const v = Math.min(chartVar[kind], nVars(kind, pa) - 1), y = pa.series(kind, i, v), unit = varUnit(kind, v, pa), T = TH();
  // the report's own maximum for the same quantity (over every routing step, not just reporting periods)
  const us = pa.flowUnits < 3, q = (FLOW_UNITS[pa.flowUnits] || '').toLowerCase(), len = us ? 'ft' : 'm';
  const refKey = { 'node:0': 'max_depth_' + len, 'node:4': 'max_total_inflow_' + q, 'link:0': 'max_flow_' + q, 'sub:4': 'peak_runoff_' + q }[kind + ':' + v];
  const ref = refKey && typeof f.properties[refKey] === 'number' ? f.properties[refKey] : null;
  const W = Math.max(300, el.clientWidth || 640), H = 230, l = 56, r = 14, t = 14, b = 34, x0 = sr.hours[0], x1 = sr.hours[pa.nPer - 1] || x0 + 1;
  let lo = Infinity, hi = -Infinity, kmax = 0; for (let k = 0; k < y.length; k++) { if (y[k] < lo) lo = y[k]; if (y[k] > hi) { hi = y[k]; kmax = k; } }
  if (ref != null) hi = Math.max(hi, ref);
  if (lo > 0) lo = 0; if (!(hi > lo)) hi = lo + 1;
  const X = h => l + (h - x0) / (x1 - x0 || 1) * (W - l - r), Y = val => t + (1 - (val - lo) / (hi - lo)) * (H - t - b);
  const ys = niceStep((hi - lo) / 4), xs = niceStep((x1 - x0) / 6);
  let svg = `<svg viewBox="0 0 ${W} ${H}" width="100%" role="img" aria-label="${esc(id)} ${esc(varName(kind, v, pa))} over time"><rect width="${W}" height="${H}" fill="${T.face}"/>`;
  for (let g = Math.ceil(lo / ys) * ys; g <= hi + 1e-12; g += ys) svg += `<line x1="${l}" x2="${W - r}" y1="${Y(g)}" y2="${Y(g)}" stroke="${T.grid}"/><text x="${l - 6}" y="${Y(g) + 4}" fill="${T.tick}" font-size="11" text-anchor="end" font-family="JetBrains Mono, monospace">${fmt(+g.toPrecision(6), 3)}</text>`;
  for (let g = Math.ceil(x0 / xs) * xs; g <= x1 + 1e-9; g += xs) svg += `<line x1="${X(g)}" x2="${X(g)}" y1="${t}" y2="${H - b}" stroke="${T.grid}"/><text x="${X(g)}" y="${H - b + 15}" fill="${T.tick}" font-size="11" text-anchor="middle" font-family="JetBrains Mono, monospace">${fmt(g, 1)}</text>`;
  svg += `<text x="${(l + W - r) / 2}" y="${H - 4}" fill="${T.axis}" font-size="11.5" text-anchor="middle" font-family="Barlow, sans-serif">hours from the report start</text>`;
  svg += `<text x="12" y="${(t + H - b) / 2}" fill="${T.axis}" font-size="11.5" text-anchor="middle" font-family="Barlow, sans-serif" transform="rotate(-90 12 ${(t + H - b) / 2})">${esc(unit)}</text>`;
  if (ref != null) svg += `<line x1="${l}" x2="${W - r}" y1="${Y(ref)}" y2="${Y(ref)}" stroke="${T.rust}" stroke-dasharray="5 4"/><text x="${W - r - 4}" y="${Y(ref) - 5}" fill="${T.rust}" font-size="11" text-anchor="end" font-family="Barlow, sans-serif">${esc(refKey)} from the report: ${fmt(ref, 3)}</text>`;
  let d = ''; const stride = Math.max(1, Math.floor(y.length / 1600));
  for (let k = 0; k < y.length; k += stride) d += (k ? 'L' : 'M') + X(sr.hours[k]).toFixed(1) + ' ' + Y(y[k]).toFixed(1);
  svg += `<path d="${d}" fill="none" stroke="${T.water}" stroke-width="2"/>`;
  const kx = X(sr.hours[S.time.k]); svg += `<line x1="${kx}" x2="${kx}" y1="${t}" y2="${H - b}" stroke="${T.sel}" stroke-width="1.5"/><circle cx="${kx}" cy="${Y(y[S.time.k])}" r="4" fill="${T.sel}"/>`;
  svg += '</svg>';
  el.innerHTML = `<div class="tsrow" style="justify-content:space-between"><b>${esc(id)}</b><label class="tsvar">series<select id="tsvarsel" aria-label="Variable to plot">${Array.from({ length: nVars(kind, pa) }, (_, k) => `<option value="${k}" ${k === v ? 'selected' : ''}>${esc(varName(kind, k, pa))}</option>`).join('')}</select></label></div>
    <div class="svgbox" style="overflow-x:auto">${svg}</div>
    <p style="margin:4px 0 0;font-size:12.5px;color:var(--ink-dim)">At the marker: <b>${fmt(y[S.time.k], 4)} ${esc(unit)}</b>. Highest reporting-period value ${fmt(y[kmax], 4)} ${esc(unit)} at ${fmt(sr.hours[kmax], 2)} h.${ref != null ? ' The dashed line is the report’s maximum: taken over every routing step, not just the reporting periods, and printed to the report’s precision, so the two can differ in the last place.' : ''}</p>`;
  $('#tsvarsel').addEventListener('change', e => { chartVar[kind] = +e.target.value; renderSeriesChart(); });
}

// ------------------------------------------------------------------ run summary and badges
function renderSummary() {
  const el = $('#runsum'), run = S.run; if (!run) { el.hidden = true; return; } el.hidden = false;
  const b = run.bundle, msgs = run.reply && run.reply.ok ? run.reply.ok.messages : [], chips = [], facts = [];
  const fu = (/^\s*FLOW_UNITS\s+(\S+)/mi.exec(latin1(b.model.bytes)) || [, 'CFS'])[1].toUpperCase();
  const us = ['CFS', 'GPM', 'MGD'].includes(fu);
  if (run.reply.error) chips.push(`<span class="chip warn">browser: ${esc(run.reply.error.split('\n')[0].slice(0, 80))}</span>`);
  else chips.push(`<span class="chip ok">ran in this browser · ${fmt(run.ms, 0)} ms</span>`);
  const demo = b.demo && A.demos.find(d => d.file === b.demo), fu2 = demo && demo.fuel;
  if (b.tool === 'map-swmm') chips.push('<span class="chip ok">RustGIS: Map only always fits</span>');
  else if (fu2) chips.push(fu2.fits ? `<span class="chip ok">RustGIS: ${fu2.pct_of_budget}% of the fuel budget (measured)</span>` : `<span class="chip hi">RustGIS: needs ${fu2.pct_of_budget}% of the fuel budget (measured), so RustGIS stops it; Map only fits</span>`);
  else chips.push('<span class="chip panel">RustGIS fuel: not measured for this file</span>');
  if (run.mem != null) chips.push(`<span class="chip ${run.mem > 512 ? 'hi' : 'panel'}">memory ${fmt(run.mem, 1)} MB (RustGIS's cap: 512 MB)</span>`);
  const v = S.verify && S.verify.runId === run.id ? S.verify : null;
  if (b.tool !== 'run-swmm' || run.reply.error) {}
  else if (!v) chips.push('<button class="chip panel linkchip" data-go="verify">not verified yet: Verify ›</button>');
  else if (v.e.rc !== 0) chips.push('<span class="chip warn">verify: EPA\'s engine stopped</span>');
  else { const clean = v.same && !v.at.dis && !v.at.missing; chips.push(`<button class="chip ${clean ? 'ok' : 'warn'} linkchip" data-go="verify">${clean ? `verified: identical files, ${fmt(v.at.agree)} values agree` : 'verify: differences'}</button>`); }
  if (S.dirty) chips.push('<span class="chip hi">the form has changed since this run</span>');
  if (!run.reply.error) {
    const ce = msgs.find(m => /^continuity error/.test(m)); if (ce) facts.push(['continuity error (%)', ce.replace(/^continuity error \(%\) as reported: /, '')]);
    const wn = msgs.find(m => /WARNING lines/.test(m)); facts.push(['warnings', wn ? wn.split(' ')[0] : '0']);
    const nodes = [...S.layers, ...S.tables].filter(L => objKind(L) === 'node').flatMap(L => L.fc.features.map(f => f.properties || {}));
    if (b.tool === 'run-swmm') { facts.push(['flooded nodes', fmt(nodes.filter(p => p.flood_hr > 0).length)]); facts.push(['surcharged nodes', fmt(nodes.filter(p => p.surcharge_hr > 0).length)]); }
    const sk = msgs.find(m => /report rows not read/.test(m)); if (sk) facts.push(['report rows not read', sk.split(' ')[0]]);
  }
  facts.push(['units', `${fu} · ${us ? 'US customary' : 'SI'}`]);
  el.innerHTML = `<div class="sumhead"><b>Run #${run.id}</b> <span class="mono">${esc(b.tool)}</span> · ${esc(b.model.name)} <span class="mono" title="SHA-256 of the input file">${b.model.sha.slice(0, 10)}…</span>${b.data.length ? ' + ' + b.data.map(d => esc(d.name)).join(', ') : ''}</div>
    <div class="badges">${chips.join('')}</div>
    ${facts.length ? `<div class="facts">${facts.map(([k, x]) => `<span><span class="k">${esc(k)}</span> ${esc(x)}</span>`).join('')}</div>` : ''}`;
  el.querySelectorAll('[data-go]').forEach(x => x.addEventListener('click', () => showTab(x.dataset.go)));
}

// ------------------------------------------------------------------ plugin tab
function elide(v) {
  return JSON.stringify(v, (k, x) => typeof x === 'string' && x.length > 120 ? x.slice(0, 72) + `… (${fmt(x.length)} characters)` : x, 1)
    .replace(/\[\s*(-?[\d.e+-]+),\s*(-?[\d.e+-]+)\s*\]/g, '[$1, $2]');
}
function trimReply(r) {
  if (!r || !r.ok) return r;
  const ok = r.ok; const cut = fc => ({ ...fc, features: fc.features.slice(0, 2).concat(fc.features.length > 2 ? [`… ${fc.features.length - 2} more features`] : []) });
  return { ok: { layers: ok.layers.map(l => ({ name: l.name, geojson: cut(l.geojson) })), tables: ok.tables.map(t => ({ name: t.name, geojson: cut(t.geojson) })), messages: ok.messages } };
}
function renderPluginTab() {
  $('#reqjson').textContent = S.run ? elide(S.run.request) : 'Run a model first.';
  $('#repjson').textContent = S.run && S.run.reply ? elide(trimReply(S.run.reply)) : 'Run a model first.';
}
function staticPlugin() {
  $('#manifest').textContent = JSON.stringify(A.manifest, null, 2);
  $('#clicmd').textContent = `rustgis-cli run --sample --plugin swmm5-rust-0.1.0.rgplugin --cmd 'gp.run={"tool":"plugin:swmm5-rust/run-swmm","params":{"model":"model.inp"}}'`;
  const P = A.prov;
  $('#pluginfacts').innerHTML = [['package', 'swmm5-rust-0.1.0.rgplugin, ' + fmt(P.rgplugin_bytes) + ' bytes'], ['package SHA-256', `<span class="mono">${P.rgplugin_sha}</span>`],
    ['module', 'plugin.wasm, ' + fmt(P.plugin_bytes) + ' bytes, imports only RustGIS\u2019s call, result and log'], ['module SHA-256', `<span class="mono">${P.plugin_sha}</span>`],
    ['permissions', 'files (to read the .inp you pick)'], ['tested in', 'RustGIS ' + P.rustgis + ' (rustgis-cli, built from source)'],
    ['registry checks', 'passed (opengeos/rustgis-plugins scripts/build.py)']].map(([k, v]) => `<div>${k}</div><div>${v}</div>`).join('');
}

// ------------------------------------------------------------------ EPA's run of the same bundle
// The files EPA's engine gets: the .inp as m.inp, and each data file under its own name and under
// every name the .inp gives for it (C:\rain\storm.dat, ..\storm.dat), so EPA opens it where the
// .inp says. The plugin finds a data file by its own name in any case; EPA's file system is exact.
function epaFiles(b) {
  const files = { 'm.inp': b.model.bytes.slice() };
  const text = latin1(b.model.bytes), toks = [];
  for (const line of text.split('\n')) { const body = line.split(';')[0]; const re = /"([^"\n]*)"|([^\s"]+)/g; let m; while ((m = re.exec(body))) toks.push(m[1] !== undefined ? m[1] : m[2]); }
  for (const d of b.data) {
    files[d.name] = d.bytes.slice();
    for (const t of toks) { const base = t.split(/[\\/]/).pop(); if (base.toLowerCase() === d.name.toLowerCase() && t !== d.name) files[t.replace(/^\/+/, '')] = d.bytes.slice(); }
  }
  return files;
}
async function epaFor(run) {
  if (run.epa) return run.epa;
  if (run.epaJob) return run.epaJob;
  S.epaBusy = run.id;
  run.epaJob = call({ kind: 'epa', files: epaFiles(run.bundle) }).then(r => {
    const rpt = new Uint8Array(r.rpt || []), out = new Uint8Array(r.out || []);
    run.epa = { rc: r.rc, trap: r.trap, ms: r.ms, rpt, out, outSha: sha256(out), rptSha: sha256(withoutClock(rpt)), files: Object.keys(epaFiles(run.bundle)) };
    return run.epa;
  }).finally(() => { if (S.epaBusy === run.id) S.epaBusy = null; run.epaJob = null; });
  return run.epaJob;
}
const hashOf = (run, re) => { const msgs = run && run.reply && run.reply.ok ? run.reply.ok.messages : []; const m = (msgs.find(x => re.test(x)) || '').match(/SHA-256 ([0-9a-f]{64})/); return m ? m[1] : null; };
const epaErrors = e => latin1(e.rpt).split('\n').filter(l => /^\s*ERROR/.test(l)).slice(0, 6).map(l => l.trim());

// ------------------------------------------------------------------ verify tab
const ranOk = run => run && run.tool === 'run-swmm' && run.reply && run.reply.ok;
function renderVerifyIdle() {
  const run = S.run, el = $('#verifyout');
  if (S.verify && run && S.verify.runId === run.id) { renderVerify(); return; }
  el.innerHTML = ranOk(run) ? `<div class="box panel">Ready to check run #${run.id}: <b>${esc(run.bundle.model.name)}</b>${run.bundle.data.length ? ' with ' + run.bundle.data.map(d => esc(d.name)).join(', ') : ''}.${S.dirty ? ' The form has changed since this run; the check uses the inputs run #' + run.id + ' was given.' : ''}</div>`
    : `<div class="box panel">Run a model on the Map tab first (Run SWMM 5 Model, not Map only).</div>`;
}
function bundleRows(b) {
  const row = (what, f) => `<tr><td>${what}</td><td>${esc(f.name)}</td><td class="r">${fmt(f.size)}</td><td class="mono">${f.sha.slice(0, 16)}…</td></tr>`;
  return `<div style="overflow-x:auto"><table class="plain"><thead><tr><th>Input</th><th>File</th><th class="r">Bytes</th><th>SHA-256</th></tr></thead><tbody>${row('model', b.model)}${b.data.map(d => row('data file', d)).join('')}</tbody></table></div>
    <p style="font-size:12.5px;color:var(--ink-dim);margin:6px 0 0">Settings: EPSG ${esc(b.params.epsg || 'none')}, anchor ${b.params.anchor_lat}, ${b.params.anchor_lon}${b.params.report ? ', report table on' : ''}. Both engines were given these bytes, frozen when run #${S.run.id} started.</p>`;
}
async function runVerify() {
  const run = S.run, out = $('#verifyout');
  if (!ranOk(run)) { renderVerifyIdle(); return; }
  $('#verifybtn').disabled = true;
  out.innerHTML = `<span class="spin"></span> Running EPA SWMM 5.2.4 (wasm32-wasi) on run #${run.id}'s inputs… <button class="btn small" id="vcancel">Cancel</button>`;
  $('#vcancel').addEventListener('click', () => cancelRun('cancelled'));
  try {
    const e = await epaFor(run);
    if (S.run !== run) return;
    const pOut = hashOf(run, /^output file/), pRpt = hashOf(run, /^report:/);
    const same = e.outSha === pOut && e.rptSha === pRpt;
    const layers = [...S.layers, ...S.tables].map(l => ({ name: l.name, fc: l.fc }));
    const rptText = latin1(e.rpt);
    const at = e.rc === 0 ? VERIFY.check(rptText, layers) : null, st = at ? VERIFY.selfTest(rptText, layers) : [];
    S.verify = { runId: run.id, same, at, st, pOut, pRpt, e };
    renderVerify(); renderSummary();
    if (!run.series) loadSeries(run, true);
    GIF.caption(same ? 'Verify: identical to EPA SWMM 5.2.4, byte for byte' : 'Verify: files differ');
  } catch (err) { if (S.run === run) out.innerHTML = `<div class="box warn">EPA's run did not finish: ${esc(err.message)}</div>`; }
  $('#verifybtn').disabled = false;
}
function renderVerify() {
  const v = S.verify, run = S.run, out = $('#verifyout'); if (!v || !run || v.runId !== run.id) return renderVerifyIdle();
  const e = v.e, chip = ok => `<span class="chip ${ok ? 'ok' : 'warn'}">${ok ? 'identical' : 'different'}</span>`;
  if (e.rc !== 0) {
    out.innerHTML = `<div class="box warn"><b>EPA's engine stopped</b> (code ${esc(e.rc)}) on run #${run.id}'s inputs, while the plugin ran.</div><pre class="code" style="margin-top:8px">${esc(epaErrors(e).join('\n') || e.trap || '')}</pre><div style="margin-top:10px">${bundleRows(run.bundle)}</div>`; return;
  }
  const at = v.at, clean = v.same && !at.dis && !at.missing, caught = v.st.filter(x => x.applicable && x.caught).length, tried = v.st.filter(x => x.applicable).length;
  out.innerHTML = `<div class="box ${clean ? 'ok' : 'warn'}"><b>${v.same ? 'Byte for byte the same.' : 'The files differ.'}</b> ${v.same ? `EPA SWMM 5.2.4 and the swmm5-rust plugin wrote the same output file and the same report (wall-clock lines aside) for run #${run.id}, ${esc(run.bundle.model.name)}.` : 'See the hashes below; a difference here is worth a defect report.'}${S.dirty ? ' (The form has changed since this run; this check is of run #' + run.id + '\u2019s inputs.)' : ''}</div>
    <div style="margin-top:10px">${bundleRows(run.bundle)}</div>
    <div style="overflow-x:auto"><table class="plain" style="margin-top:10px"><thead><tr><th>File</th><th>EPA SWMM 5.2.4 (C, wasm32-wasi)</th><th>swmm5-rust plugin</th><th></th></tr></thead><tbody>
    <tr><td>output (.out), ${fmt(e.out.length)} bytes</td><td class="mono">${e.outSha.slice(0, 16)}…</td><td class="mono">${(v.pOut || '').slice(0, 16)}…</td><td>${chip(e.outSha === v.pOut)}</td></tr>
    <tr><td>report (.rpt), clock lines removed</td><td class="mono">${e.rptSha.slice(0, 16)}…</td><td class="mono">${(v.pRpt || '').slice(0, 16)}…</td><td>${chip(e.rptSha === v.pRpt)}</td></tr></tbody></table></div>
    <p style="margin:12px 0 6px"><b>${fmt(at.agree)}</b> result attributes agree with EPA's report, column by column; <b>${fmt(at.dis)}</b> disagree and <b>${fmt(at.missing)}</b> are missing.</p>
    <div style="overflow-x:auto"><table class="plain"><thead><tr><th>Layer or table</th><th class="r">Features</th><th class="r">Agree</th><th class="r">Disagree</th><th class="r">Missing</th></tr></thead><tbody>${at.res.map(x => `<tr><td>${esc(x.layer)}</td><td class="r">${fmt(x.n)}</td><td class="r">${fmt(x.a)}</td><td class="r">${fmt(x.d)}</td><td class="r">${fmt(x.m)}</td></tr>`).join('')}</tbody></table></div>
    ${at.bad.length ? `<pre class="code" style="margin-top:8px">${esc(at.bad.map(b => `${b.layer} ${b.id} ${b.k} = ${JSON.stringify(b.v)}; EPA: ${JSON.stringify(b.exp)}`).join('\n'))}</pre>` : ''}
    <div class="box ${caught === tried ? 'panel' : 'warn'}" style="margin-top:10px"><b>Checker self-test: ${caught} of ${tried} planted errors caught.</b> Each is planted in a copy of these layers and the check run again; a checker that passed everything would fail here.
      <ul style="margin:6px 0 0;padding-left:20px">${v.st.map(x => `<li>${esc(x.what)}${x.applicable ? (x.caught ? ' — caught' : ' — <b>missed</b>') + ` (on ${esc(x.id)})` : ' — nothing in this model to plant it on'}</li>`).join('')}</ul></div>
    <p style="font-size:12.5px;color:var(--ink-dim);margin:10px 0 0">EPA's run took ${fmt(e.ms, 0)} ms here; the plugin's took ${fmt(run.ms, 0)} ms (it also builds the layers). Both ran in this browser's WebAssembly engine, so neither is metered; RustGIS interprets the plugin with a fuel budget.</p>`;
}
$('#verifybtn').addEventListener('click', runVerify);

// ------------------------------------------------------------------ charts (shared by Verify and Doc)
function svgFuel(width) {
  const T = TH(), rows = A.demos.filter(d => d.fuel).map(d => ({ name: d.short, f: d.fuel.fuel, fits: d.fuel.fits }));
  const W = width || 760, H = 60 + rows.length * 34, L = 170, Rr = 24, budget = 3e10;
  const lo = 1e8, hi = 2e11, x = v => L + (Math.log10(v) - Math.log10(lo)) / (Math.log10(hi) - Math.log10(lo)) * (W - L - Rr);
  let s = `<svg viewBox="0 0 ${W} ${H}" width="100%" style="max-width:${W}px" role="img" aria-label="Fuel each demo model needs, against RustGIS's budget"><rect x="0" y="0" width="${W}" height="${H}" fill="${T.face}"/>`;
  for (let e = 8; e <= 11; e++) { const X = x(Math.pow(10, e)); s += `<line x1="${X}" y1="22" x2="${X}" y2="${H - 26}" stroke="${T.grid}"/><text x="${X}" y="${H - 10}" fill="${T.tick}" font-size="11" text-anchor="middle" font-family="JetBrains Mono, monospace">10^${e}</text>`; }
  rows.forEach((r, i) => { const y = 30 + i * 34; s += `<text x="${L - 10}" y="${y + 15}" fill="${T.axis}" font-size="12.5" text-anchor="end" font-family="Barlow, sans-serif">${esc(r.name)}</text><rect x="${L}" y="${y + 3}" width="${Math.max(2, x(r.f) - L)}" height="18" rx="3" fill="${r.fits ? T.bar : T.rust}"/><text x="${x(r.f) + 6}" y="${y + 16}" fill="${T.tick}" font-size="11" font-family="JetBrains Mono, monospace">${(r.f / 1e9).toFixed(r.f < 1e9 ? 2 : 1)}e9</text>`; });
  const B = x(budget); s += `<line x1="${B}" y1="16" x2="${B}" y2="${H - 26}" stroke="${T.budget}" stroke-width="2" stroke-dasharray="5 4"/><text x="${B - 6}" y="16" fill="${T.budget}" font-size="11.5" text-anchor="end" font-family="Barlow, sans-serif">RustGIS budget per tool call, 3×10^10</text>`;
  return s + '</svg>';
}
function svgCensus(width) {
  const T = TH(), C = A.census; const W = width || 760, rows = C.bars, H = 40 + rows.length * 40, L = 250, Rr = 20;
  const x = v => L + v / C.n * (W - L - Rr);
  let s = `<svg viewBox="0 0 ${W} ${H}" width="100%" style="max-width:${W}px" role="img" aria-label="Census of the 221-model set"><rect x="0" y="0" width="${W}" height="${H}" fill="${T.face}"/>`;
  rows.forEach((r, i) => { const y = 20 + i * 40; let X = L;
    s += `<text x="${L - 10}" y="${y + 17}" fill="${T.axis}" font-size="12.5" text-anchor="end" font-family="Barlow, sans-serif">${esc(r.label)}</text>`;
    r.parts.forEach(p => { const wdt = x(p.n) - L; if (wdt <= 0) return; const col = p.k === 'same' ? T.ok : p.k === 'diff' ? T.rust : p.k === 'ub' ? T.budget : T.muted; s += `<rect x="${X}" y="${y + 4}" width="${wdt}" height="20" fill="${col}"/>`; if (wdt > 26) s += `<text x="${X + wdt / 2}" y="${y + 18}" fill="${T.face}" font-size="11" font-weight="600" text-anchor="middle" font-family="JetBrains Mono, monospace">${p.n}</text>`; X += wdt; }); });
  return s + '</svg>';
}
function svgABI(width) {
  const T = TH(), W = width || 760, H = 300;
  const box = (x, y, w, h, title, sub, col) => `<rect x="${x}" y="${y}" width="${w}" height="${h}" rx="12" fill="${T.face}" stroke="${col}" stroke-width="2"/><text x="${x + 14}" y="${y + 26}" fill="${col}" font-size="17" font-weight="700" font-family="Barlow Condensed, sans-serif">${title}</text>${sub.map((t, i) => `<text x="${x + 14}" y="${y + 50 + i * 19}" fill="${T.axis}" font-size="12.5" font-family="Barlow, sans-serif">${t}</text>`).join('')}`;
  const arrow = (x1, y, x2, label, col) => `<line x1="${x1}" y1="${y}" x2="${x2}" y2="${y}" stroke="${col}" stroke-width="2"/><path d="M${x2} ${y} l${x2 > x1 ? -9 : 9} -5 v10 z" fill="${col}"/><text x="${(x1 + x2) / 2}" y="${y - 7}" fill="${T.axis}" font-size="12" text-anchor="middle" font-family="JetBrains Mono, monospace">${label}</text>`;
  let s = `<svg viewBox="0 0 ${W} ${H}" width="100%" style="max-width:${W}px" role="img" aria-label="The RustGIS plugin interface"><rect x="0" y="0" width="${W}" height="${H}" fill="${T.face}"/>`;
  s += box(16, 30, 250, 240, 'RustGIS (host)', ['desktop, rustgis-cli or web app', 'wasmi interpreter', 'fuel: 3×10^10 per tool call', 'memory: 512 MB', 'a fresh instance per call', 'this page plays this part'], T.rust);
  s += box(W - 266, 30, 250, 240, 'plugin.wasm', ['swmm5-rust engine (safe Rust)', 'memfs: files in memory', '.rpt tables → attributes', '.inp geometry → GeoJSON', 'exports: memory, rustgis_alloc,', '  rustgis_call'], T.water);
  s += arrow(272, 92, W - 272, 'rustgis_alloc(len) → ptr', T.axis);
  s += arrow(272, 132, W - 272, 'request JSON: tool, params, files{model}', T.axis);
  s += arrow(W - 272, 172, 272, 'reply: {ok:{layers, tables, messages}}', T.axis);
  s += arrow(W - 272, 212, 272, 'log: the line a stopped run leaves', T.muted);
  s += `<text x="${W / 2}" y="258" fill="${T.tick}" font-size="11.5" text-anchor="middle" font-family="Barlow, sans-serif">no files · no clock · no network · no threads</text>`;
  return s + '</svg>';
}
function svgMemfs(width) {
  const T = TH(), W = width || 760, H = 230;
  const b = (x, y, w, h, t, sub, col) => `<rect x="${x}" y="${y}" width="${w}" height="${h}" rx="10" fill="${T.face}" stroke="${col}" stroke-width="1.6"/><text x="${x + w / 2}" y="${y + 22}" fill="${col}" font-size="14.5" font-weight="700" text-anchor="middle" font-family="Barlow Condensed, sans-serif">${t}</text>${sub.map((u, i) => `<text x="${x + w / 2}" y="${y + 42 + i * 16}" fill="${T.axis}" font-size="11.5" text-anchor="middle" font-family="JetBrains Mono, monospace">${u}</text>`).join('')}`;
  let s = `<svg viewBox="0 0 ${W} ${H}" width="100%" style="max-width:${W}px" role="img" aria-label="How memfs stands in for the disk"><rect x="0" y="0" width="${W}" height="${H}" fill="${T.face}"/>`;
  s += b(20, 20, 210, 84, 'swmm5-rust engine', ['input.rs, report.rs', 'output.rs, hotstart.rs'], T.water);
  s += b(275, 20, 210, 84, 'cio::CFile', ['fopen fgets fprintf', 'fread fwrite fseek'], T.axis);
  s += b(530, 20, 210, 84, 'default build', ['std::fs::File', '(unchanged)'], T.muted);
  s += b(530, 126, 210, 84, 'feature "memfs"', ['BTreeMap<name, bytes>', 'clock() = 0'], T.rust);
  s += b(275, 126, 210, 84, 'the plugin', ['put("model.inp")', 'get("model.rpt", ".out")'], T.rust);
  s += `<path d="M230 62 H275 M485 62 H530" stroke="${T.axis}" stroke-width="1.6"/><path d="M485 84 C505 84 505 168 530 168" fill="none" stroke="${T.rust}" stroke-width="1.6"/><path d="M530 168 H485" stroke="${T.rust}" stroke-width="1.6"/>`;
  return s + '</svg>';
}
function svgPipeline(width) {
  const T = TH(), W = width || 760, H = 150, steps = [['.inp', 'your model', T.axis], ['swmm_run()', 'safe Rust, in memory', T.water], ['.rpt + .out', 'EPA\'s own tables', T.water], ['layers', 'GeoJSON + attributes', T.rust], ['RustGIS', 'map, table, tools', T.rust]];
  const bw = (W - 40 - 4 * 24) / 5;
  let s = `<svg viewBox="0 0 ${W} ${H}" width="100%" style="max-width:${W}px" role="img" aria-label="From .inp to RustGIS layers"><rect x="0" y="0" width="${W}" height="${H}" fill="${T.face}"/>`;
  steps.forEach((st, i) => { const x = 20 + i * (bw + 24); s += `<rect x="${x}" y="34" width="${bw}" height="70" rx="10" fill="${T.face}" stroke="${st[2]}" stroke-width="1.8"/><text x="${x + bw / 2}" y="64" fill="${st[2]}" font-size="16" font-weight="700" text-anchor="middle" font-family="Barlow Condensed, sans-serif">${st[0]}</text><text x="${x + bw / 2}" y="86" fill="${T.tick}" font-size="11" text-anchor="middle" font-family="Barlow, sans-serif">${st[1]}</text>`;
    if (i < 4) s += `<path d="M${x + bw + 4} 69 h14 l-5 -4 m5 4 l-5 4" fill="none" stroke="${T.axis}" stroke-width="1.6"/>`; });
  s += `<text x="${W / 2}" y="132" fill="${T.tick}" font-size="11.5" text-anchor="middle" font-family="Barlow, sans-serif">one tool call: request in, reply out; nothing persists between calls</text>`;
  return s + '</svg>';
}
function svgPlacement(width) {
  const T = TH(), W = width || 760, H = 330;
  let s = `<svg viewBox="0 0 ${W} ${H}" width="100%" style="max-width:${W}px" role="img" aria-label="How a local model frame is placed on the earth"><rect x="0" y="0" width="${W}" height="${H}" fill="${T.face}"/>`;
  s += `<rect x="24" y="40" width="210" height="170" fill="none" stroke="${T.muted}" stroke-dasharray="4 3"/><text x="24" y="30" fill="${T.water}" font-size="15" font-weight="700" font-family="Barlow Condensed, sans-serif">the model's own frame (feet)</text>`;
  s += `<circle cx="110" cy="140" r="4" fill="${T.water}"/><text x="40" y="160" fill="${T.axis}" font-size="12" font-family="JetBrains Mono, monospace">(cx, cy)</text><circle cx="190" cy="80" r="4" fill="${T.node}"/><text x="150" y="70" fill="${T.axis}" font-size="12" font-family="JetBrains Mono, monospace">(x, y)</text>`;
  s += `<path d="M110 140 L190 140 L190 80" fill="none" stroke="${T.rust}" stroke-width="1.6"/><text x="135" y="156" fill="${T.rust}" font-size="11.5" font-family="JetBrains Mono, monospace">x−cx</text><text x="196" y="114" fill="${T.rust}" font-size="11.5" font-family="JetBrains Mono, monospace">y−cy</text>`;
  s += `<text x="24" y="228" fill="${T.tick}" font-size="11.5" font-family="Barlow, sans-serif">center = middle of everything drawn</text>`;
  s += `<path d="M252 125 h48 l-7 -5 m7 5 l-7 5" fill="none" stroke="${T.axis}" stroke-width="1.8"/>`;
  const gx = 320, gw = W - gx - 24;
  s += `<path d="M${gx} 60 Q${gx + gw / 2} 30 ${gx + gw} 60 L${gx + gw - 14} 200 Q${gx + gw / 2} 228 ${gx + 14} 200 Z" fill="none" stroke="${T.muted}"/><text x="${gx}" y="30" fill="${T.rust}" font-size="15" font-weight="700" font-family="Barlow Condensed, sans-serif">longitude / latitude, near true size</text>`;
  s += `<circle cx="${gx + gw / 2 - 30}" cy="134" r="4" fill="${T.rust}"/><text x="${gx + gw / 2 - 22}" y="130" fill="${T.axis}" font-size="12" font-family="JetBrains Mono, monospace">(lon0, lat0)</text><circle cx="${gx + gw / 2 + 50}" cy="78" r="4" fill="${T.node}"/><text x="${gx + gw / 2 + 58}" y="74" fill="${T.axis}" font-size="12" font-family="JetBrains Mono, monospace">(lon, lat)</text>`;
  s += `<rect x="20" y="244" width="${W - 40}" height="74" rx="8" fill="${T.face}" stroke="${T.frame}"/>`;
  s += `<text x="34" y="266" fill="${T.axis}" font-size="12.5" font-family="JetBrains Mono, monospace">lat = lat0 + (y − cy)·k / R</text><text x="${Math.round(W * 0.42)}" y="266" fill="${T.axis}" font-size="12.5" font-family="JetBrains Mono, monospace">lon = lon0 + (x − cx)·k / (R·cos lat0)</text>`;
  s += `<text x="34" y="286" fill="${T.tick}" font-size="11.5" font-family="JetBrains Mono, monospace">k = 0.3048 m per ft (1 m per m)</text><text x="${Math.round(W * 0.42)}" y="286" fill="${T.tick}" font-size="11.5" font-family="JetBrains Mono, monospace">R = 6,371,008.8 m · anchor (−82.3474, 29.6476)</text>`;
  s += `<text x="34" y="306" fill="${T.tick}" font-size="11.5" font-family="Barlow, sans-serif">With an EPSG code there is no arithmetic: the plugin checks RustGIS knows the code, and RustGIS reprojects.</text>`;
  return s + '</svg>';
}
function svgLibm(width) {
  const T = TH(), W = width || 760, rows = A.libm.rows, H = 70 + rows.length * 46;
  let s = `<svg viewBox="0 0 ${W} ${H}" width="100%" style="max-width:${W}px" role="img" aria-label="One C source, three math libraries, three answers"><rect x="0" y="0" width="${W}" height="${H}" fill="${T.face}"/>`;
  s += `<text x="20" y="28" fill="${T.axis}" font-size="13" font-family="Barlow, sans-serif">${esc(A.libm.model)}: the .out file's SHA-256, by engine build and the math library it uses</text>`;
  const groups = {}; let gi = 0; const cols = [T.bar, T.ok, T.budget, T.rust];
  rows.forEach((r, i) => { if (!(r.sha in groups)) groups[r.sha] = cols[gi++ % cols.length]; const y = 46 + i * 46, c = groups[r.sha];
    s += `<rect x="20" y="${y}" width="${W - 40}" height="36" rx="8" fill="${T.face}" stroke="${c}" stroke-width="1.8"/>`;
    s += `<text x="34" y="${y + 15}" fill="${T.axis}" font-size="13" font-weight="600" font-family="Barlow, sans-serif">${esc(r.engine)}</text><text x="34" y="${y + 30}" fill="${T.tick}" font-size="11.5" font-family="Barlow, sans-serif">${esc(r.libm)}</text>`;
    s += `<text x="${W - 34}" y="${y + 23}" fill="${c}" font-size="13" font-weight="600" text-anchor="end" font-family="JetBrains Mono, monospace">${r.sha.slice(0, 16)}…</text>`; });
  return s + '</svg>';
}
// node 9's depth from the storage example's output file (the Doc's hydrograph figure)
function svgHydro(width) {
  const T = TH(), H0 = A.hydro, W = width || 760, H = 280, l = 60, r = 16, t = 18, b = 40, x0 = 0, x1 = H0.hours[H0.hours.length - 1];
  const hi = Math.max(H0.ref, ...H0.depth) * 1.08, X = h => l + (h - x0) / (x1 - x0) * (W - l - r), Y = v => t + (1 - v / hi) * (H - t - b);
  let s = `<svg viewBox="0 0 ${W} ${H}" width="100%" style="max-width:${W}px" role="img" aria-label="Depth at node 9 over the storage example's run"><rect x="0" y="0" width="${W}" height="${H}" fill="${T.face}"/>`;
  for (let g = 0; g <= hi + 1e-9; g += 0.05) s += `<line x1="${l}" x2="${W - r}" y1="${Y(g)}" y2="${Y(g)}" stroke="${T.grid}"/><text x="${l - 8}" y="${Y(g) + 4}" fill="${T.tick}" font-size="12" text-anchor="end" font-family="JetBrains Mono, monospace">${g.toFixed(2)}</text>`;
  for (let g = 0; g <= x1 + 1e-9; g += 4) s += `<line x1="${X(g)}" x2="${X(g)}" y1="${t}" y2="${H - b}" stroke="${T.grid}"/><text x="${X(g)}" y="${H - b + 16}" fill="${T.tick}" font-size="12" text-anchor="middle" font-family="JetBrains Mono, monospace">${g}</text>`;
  s += `<text x="${(l + W - r) / 2}" y="${H - 6}" fill="${T.axis}" font-size="13" text-anchor="middle" font-family="Barlow, sans-serif">hours from the report start</text><text x="16" y="${(t + H - b) / 2}" fill="${T.axis}" font-size="13" text-anchor="middle" font-family="Barlow, sans-serif" transform="rotate(-90 16 ${(t + H - b) / 2})">depth (ft)</text>`;
  s += `<line x1="${l}" x2="${W - r}" y1="${Y(H0.ref)}" y2="${Y(H0.ref)}" stroke="${T.rust}" stroke-dasharray="6 4" stroke-width="1.5"/><text x="${W - r - 6}" y="${Y(H0.ref) - 6}" fill="${T.rust}" font-size="12.5" text-anchor="end" font-family="Barlow, sans-serif">the report's maximum depth: 0.23 ft</text>`;
  s += `<path d="${H0.hours.map((h, k) => (k ? 'L' : 'M') + X(h).toFixed(1) + ' ' + Y(H0.depth[k]).toFixed(1)).join('')}" fill="none" stroke="${T.water}" stroke-width="2.5"/>`;
  return s + `<text x="${l + 8}" y="${t + 14}" fill="${T.water}" font-size="14" font-weight="700" font-family="Barlow Condensed, sans-serif">node 9, depth at each reporting period</text></svg>`;
}
const FIGS = { abi: svgABI, memfs: svgMemfs, pipeline: svgPipeline, fuel: svgFuel, census: svgCensus, placement: svgPlacement, libm: svgLibm, hydro: svgHydro };
function renderStaticFigures() { $('#fig-abi').innerHTML = svgABI(); }

// ------------------------------------------------------------------ gates (Verify tab, static)
function renderGates() {
  const C = A.census, G = A.rgruns, at = A.attrs, rg = A.regress;
  $('#gates').innerHTML = `
  <div class="stat">${A.headline.map(h => `<div class="s"><div class="v">${esc(h.v)}</div><div class="d">${esc(h.d)}</div></div>`).join('')}</div>
  <div><h3 style="font-size:18px;margin-bottom:6px">Inside RustGIS itself</h3>
  <p style="margin:0 0 8px;font-size:13.5px">RustGIS ${esc(A.prov.rustgis)} built from source (rustgis-cli, debug build), World Atlas sample loaded, the .rgplugin installed for the run, <code>gp.run</code> on the tool, then RustGIS's own exporter wrote each layer back out as GeoJSON. The attribute check reads those files.</p>
  <div style="overflow-x:auto"><table class="plain"><thead><tr><th>Model</th><th>Tool</th><th>In RustGIS</th><th class="r">Fuel used</th><th class="r">Values checked</th></tr></thead><tbody>
  ${G.map(r => `<tr><td>${esc(r.model)}</td><td class="mono">${esc(r.tool)}</td><td>${r.ok ? '<span class="chip ok">layers added</span>' : '<span class="chip hi">' + esc(r.status) + '</span>'}</td><td class="r">${r.fuel_pct != null ? r.fuel_pct + '%' : ''}</td><td class="r">${r.values != null ? fmt(r.values) + ' agree, ' + r.dis + ' not' : ''}</td></tr>`).join('')}
  </tbody></table></div></div>
  <figure class="fig"><div class="svgbox">${svgFuel()}</div><figcaption>Fuel a <code>run-swmm</code> call needs, measured with wasmi 2.0.0 configured as RustGIS 0.5.0 configures it. Past the dashed line RustGIS stops the tool with <i>it ran too long</i>; the same two runs fit as <code>map-swmm</code>.</figcaption></figure>
  <div><h3 style="font-size:18px;margin-bottom:6px">The 221-model census</h3>
  <p style="margin:0 0 8px;font-size:13.5px">${esc(C.note)}</p>
  <figure class="fig"><div class="svgbox">${svgCensus()}</div><figcaption>${esc(C.caption)}</figcaption></figure>
  ${C.errors.length ? `<details style="margin-top:6px"><summary style="cursor:pointer;font-size:13px">Models the plugin could not run (${C.errors.length})</summary><div style="overflow-x:auto"><table class="plain" style="margin-top:6px">${C.errors.map(e => `<tr><td class="mono" style="font-size:11.5px">${esc(e.inp)}</td><td style="font-size:12.5px">${esc(e.why)}</td></tr>`).join('')}</table></div></details>` : ''}
  </div>
  <div><h3 style="font-size:18px;margin-bottom:6px">Two descriptions agree</h3>
  <div style="overflow-x:auto"><table class="plain"><thead><tr><th>Model</th><th>Layers from</th><th class="r">Result values</th><th class="r">Agree with EPA's report</th></tr></thead><tbody>${at.map(r => `<tr><td>${esc(r.model)}</td><td>${esc(r.where)}</td><td class="r">${fmt(r.values + r.dis)}</td><td class="r">${fmt(r.values)}</td></tr>`).join('')}</tbody></table></div></div>
  <div><h3 style="font-size:18px;margin-bottom:6px">The engine's default build is unchanged</h3><p style="margin:0;font-size:13.5px">${esc(rg.text)}</p></div>`;
}

// ------------------------------------------------------------------ doc and about
const inline = s => esc(s).replace(/`([^`]+)`/g, '<code>$1</code>').replace(/\*\*([^*]+)\*\*/g, '<b>$1</b>').replace(/\*([^*]+)\*/g, '<i>$1</i>').replace(/\[([^\]]+)\]\((https?:[^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener">$1</a>');
function renderEssay() {
  const out = [];
  for (const b of A.doc) {
    if (b.t === 'title') out.push(`<h1 style="font-size:40px;line-height:1.05;margin:4px 0 6px">${inline(b.x)}</h1>`);
    else if (b.t === 'by') out.push(`<p style="color:var(--ink-faint);font-size:13px;margin:0 0 14px">${inline(b.x)}</p>`);
    else if (b.t === 'h2') out.push(`<h2>${inline(b.x)}</h2>`);
    else if (b.t === 'h3') out.push(`<h3>${inline(b.x)}</h3>`);
    else if (b.t === 'lede') out.push(`<p class="lede">${inline(b.x)}</p>`);
    else if (b.t === 'p') out.push(`<p>${inline(b.x)}</p>`);
    else if (b.t === 'list') out.push(`<ul>${b.x.map(i => `<li>${inline(i)}</li>`).join('')}</ul>`);
    else if (b.t === 'code') out.push(`<pre class="code">${esc(b.x)}</pre>${b.cap ? `<p style="font-size:12.5px;color:var(--ink-dim);margin-top:4px">${inline(b.cap)}</p>` : ''}`);
    else if (b.t === 'table') out.push(`<div style="overflow-x:auto"><table class="plain">${b.w ? `<colgroup>${b.w.map(x => `<col style="width:${(100 * x / b.w.reduce((p, q) => p + q, 0)).toFixed(1)}%">`).join('')}</colgroup>` : ''}<thead><tr>${b.head.map(h => `<th>${inline(h)}</th>`).join('')}</tr></thead><tbody>${b.rows.map(r => `<tr>${r.map(c => `<td>${inline(String(c))}</td>`).join('')}</tr>`).join('')}</tbody></table></div>${b.cap ? `<p style="font-size:12.5px;color:var(--ink-dim);margin-top:4px">${inline(b.cap)}</p>` : ''}`);
    else if (b.t === 'fig') out.push(`<figure class="fig" data-fig="${b.f}"><div class="svgbox">${b.f === 'rgshot' ? `<img src="${A.rgshot}" alt="RustGIS's own rendering of the plugin's layers" style="width:100%;border-radius:10px;border:1px solid var(--line)">` : FIGS[b.f]()}</div><figcaption>${inline(b.cap)}</figcaption></figure>`);
  }
  $('#essay').innerHTML = out.join('');
  const hs = $$('#essay h2'); hs.forEach((h, i) => { h.id = 'doc-s' + (i + 1); });
  $('#toc').innerHTML = hs.map((h, i) => `<a href="#doc-s${i + 1}">${i + 1}. ${esc(h.textContent)}</a>`).join('');
  $$('#toc a').forEach(a => a.addEventListener('click', e => { e.preventDefault(); const t = document.getElementById(a.getAttribute('href').slice(1)); if (t) t.scrollIntoView({ block: 'start' }); }));
}
function renderAbout() {
  const P = A.prov;
  $('#about').innerHTML = `
  <div class="card"><h2 style="font-size:24px">About</h2><p style="margin:8px 0">${inline(A.about.what)}</p></div>
  <div class="card"><div class="label">AI use</div><p style="margin:6px 0 0">${inline(A.about.ai)}</p></div>
  <div class="card"><div class="label">Credits and licenses</div><ul style="margin:6px 0 0;padding-left:20px">${A.about.credits.map(c => `<li>${inline(c)}</li>`).join('')}</ul></div>
  <div class="card"><div class="label">Provenance</div><div class="kv" style="margin-top:8px">${P.rows.map(([k, v]) => `<div>${esc(k)}</div><div class="mono">${esc(v)}</div>`).join('')}</div></div>
  <div class="box hi"><b>What this does not show.</b> ${inline(A.about.limits)}</div>`;
  $('#foot').innerHTML = `<div>${inline(A.about.foot)}</div><div><b>What this does not show:</b> ${inline(A.about.footLimits)}</div>`;
}

// ------------------------------------------------------------------ export (downloads capability, or a plain link off claude.ai)
let DL = null; (async () => { try { if (window.claude && window.claude.use) DL = await window.claude.use('downloads'); } catch (e) { DL = null; } })();
async function save(filename, data, inArtifactName) {
  if (DL) {
    try { await DL.save({ filename: inArtifactName || filename, data }); toast('Saved ' + (inArtifactName || filename)); return; }
    catch (e) { if (e && e.code === 'declined') return; if (e && e.code !== 'unavailable' && e.code !== 'not_granted') { toast('Not saved: ' + (e.message || e.code)); return; } }
  }
  const blob = data instanceof Blob ? data : new Blob([data]); const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = filename; document.body.appendChild(a); a.click(); setTimeout(() => { URL.revokeObjectURL(a.href); a.remove(); }, 2000);
}
// a zip archive with stored (uncompressed) entries
function zipStore(files) {
  const crcT = new Uint32Array(256); for (let n = 0; n < 256; n++) { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; crcT[n] = c >>> 0; }
  const crc = d => { let c = 0xffffffff; for (let i = 0; i < d.length; i++) c = crcT[(c ^ d[i]) & 255] ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
  const parts = [], central = []; let off = 0; const enc = new TextEncoder();
  const u16 = v => [v & 255, v >> 8 & 255], u32 = v => [v & 255, v >> 8 & 255, v >> 16 & 255, v >>> 24 & 255];
  for (const f of files) { const nm = enc.encode(f.name), c = crc(f.data), sz = f.data.length;
    const head = new Uint8Array([0x50, 0x4b, 3, 4, ...u16(20), ...u16(0), ...u16(0), ...u16(0), ...u16(0x21), ...u32(c), ...u32(sz), ...u32(sz), ...u16(nm.length), ...u16(0)]);
    parts.push(head, nm, f.data);
    central.push(new Uint8Array([0x50, 0x4b, 1, 2, ...u16(20), ...u16(20), ...u16(0), ...u16(0), ...u16(0), ...u16(0x21), ...u32(c), ...u32(sz), ...u32(sz), ...u16(nm.length), ...u16(0), ...u16(0), ...u16(0), ...u16(0), ...u32(0), ...u32(off)]), nm);
    off += head.length + nm.length + sz; }
  const cdSize = central.reduce((a, b) => a + b.length, 0);
  const end = new Uint8Array([0x50, 0x4b, 5, 6, 0, 0, 0, 0, ...u16(files.length), ...u16(files.length), ...u32(cdSize), ...u32(off), 0, 0]);
  const all = [...parts, ...central, end], out = new Uint8Array(all.reduce((a, b) => a + b.length, 0)); let p = 0; for (const b of all) { out.set(b, p); p += b.length; } return out;
}
function stem() { const n = S.run ? S.run.bundle.model.name : S.model ? S.model.name : 'model'; return n.replace(/\.[^.]+$/, ''); }
const X = {
  plugin: () => save('swmm5-rust-0.1.0.rgplugin', new Blob([b64bytes('b64-rgplugin')]), 'swmm5-rust-0.1.0.rgplugin.zip'),
  csv: () => { const L = activeLayer(); if (!L) return toast('Run a model first.'); save(L.name.replace(/\s+/g, '_') + '.csv', csvOf(L)); },
  // one GeoJSON file per layer in a zip: RustGIS opens it as one layer per file (polygons, lines and points apart)
  geojson: () => { if (!S.layers.length) return toast('Run a model first.'); const enc = new TextEncoder();
    const files = [...S.layers, ...S.tables].map(L => ({ name: L.name.replace(/[^A-Za-z0-9_.-]+/g, '_') + '.geojson', data: enc.encode(JSON.stringify(L.fc)) }));
    save(stem() + '_swmm5-rust_layers.zip', new Blob([zipStore(files)], { type: 'application/zip' })); },
  inp: () => { const m = S.run ? S.run.bundle.model : S.model; if (!m) return; save(m.name, new Blob([m.bytes]), stem() + '.inp.txt'); },
  rpt: () => { if (!S.rptText) return toast('Tick "Add the report (.rpt) as a table" and run again.'); save(stem() + '.rpt', S.rptText, stem() + '.rpt.txt'); },
  docx: () => save('swmm5-rust-in-rustgis.docx', new Blob([b64bytes('b64-docx')])),
  bundle: () => {
    const run = S.run; if (!run) return toast('Run a model first.');
    const enc = new TextEncoder(), b = run.bundle, files = [], safe = n => n.replace(/[^A-Za-z0-9_.-]+/g, '_');
    files.push({ name: 'inputs/' + safe(b.model.name), data: b.model.bytes });
    b.data.forEach(d => files.push({ name: 'inputs/' + safe(d.name), data: d.bytes }));
    const ok = run.reply && run.reply.ok;
    if (ok) {
      for (const L of [...ok.layers, ...ok.tables]) if (!/ report$/.test(L.name)) files.push({ name: 'plugin/' + safe(L.name) + '.geojson', data: enc.encode(JSON.stringify(L.geojson)) });
      if (S.rptText) files.push({ name: 'plugin/report.rpt', data: enc.encode(S.rptText) });
    }
    files.push({ name: 'plugin/messages.txt', data: enc.encode((S.messages || []).join('\n') + '\n') });
    if (run.epa) { files.push({ name: 'epa/model.rpt', data: run.epa.rpt }); files.push({ name: 'epa/model.out', data: run.epa.out }); }
    const v = S.verify && S.verify.runId === run.id ? S.verify : null;
    const manifest = {
      what: 'one run of the swmm5-rust RustGIS plugin, with everything needed to repeat and check it',
      page: A.pageVersion, plugin: { package: 'swmm5-rust-0.1.0.rgplugin', module_sha256: A.prov.plugin_sha, rgplugin_sha256: A.prov.rgplugin_sha },
      run: { id: run.id, tool: b.tool, finished: run.finished.toISOString(), browser_ms: Math.round(run.ms), wasm_memory_mb: run.mem, error: run.reply.error || null },
      inputs: { model: { name: b.model.name, bytes: b.model.size, sha256: b.model.sha }, data_files: b.data.map(d => ({ name: d.name, bytes: d.size, sha256: d.sha })), settings: b.params },
      plugin_outputs: { out_sha256: hashOf(run, /^output file/), rpt_sha256_without_clock_lines: hashOf(run, /^report:/), layers: ok ? ok.layers.map(l => l.name) : [], tables: ok ? ok.tables.map(t => t.name) : [] },
      epa_run: run.epa ? { engine: 'EPA SWMM 5.2.4, wasm32-wasi', exit: run.epa.rc, out_sha256: run.epa.outSha, rpt_sha256_without_clock_lines: run.epa.rptSha, files_given: run.epa.files } : null,
      verification: v ? { identical_files: v.same, attributes: { agree: v.at.agree, disagree: v.at.dis, missing: v.at.missing }, self_test: v.st } : 'not run',
      rustgis: b.tool === 'map-swmm' ? 'map-swmm always fits RustGIS\'s fuel budget' : (b.demo && (A.demos.find(d => d.file === b.demo) || {}).fuel) || 'fuel not measured for this file',
      note: 'epa/model.out is the output file of EPA\'s run of these same inputs; when its SHA-256 equals plugin_outputs.out_sha256 it is byte for byte the plugin\'s output file.' };
    files.unshift({ name: 'run.json', data: enc.encode(JSON.stringify(manifest, null, 2)) });
    save(`${stem()}_run${run.id}_bundle.zip`, new Blob([zipStore(files)], { type: 'application/zip' }));
  },
};
function menuOpen(open) { $('#exportmenu').hidden = !open; $('#exportbtn').setAttribute('aria-expanded', open ? 'true' : 'false'); if (open) $('#exportmenu button').focus(); }
$('#exportbtn').addEventListener('click', e => { e.stopPropagation(); menuOpen($('#exportmenu').hidden); });
document.addEventListener('click', () => { if (!$('#exportmenu').hidden) menuOpen(false); });
$$('#exportmenu button').forEach(b => b.addEventListener('click', () => { menuOpen(false); X[b.dataset.x](); }));
$('#exportmenu').addEventListener('keydown', e => {
  const items = $$('#exportmenu button'), i = items.indexOf(document.activeElement);
  if (e.key === 'ArrowDown' || e.key === 'ArrowUp') { e.preventDefault(); items[(i + (e.key === 'ArrowDown' ? 1 : items.length - 1)) % items.length].focus(); }
  else if (e.key === 'Escape') { e.preventDefault(); menuOpen(false); $('#exportbtn').focus(); }
  else if (e.key === 'Tab') menuOpen(false);
});
$('#csvbtn').addEventListener('click', X.csv); $('#getplugin').addEventListener('click', X.plugin); $('#docxbtn').addEventListener('click', X.docx);

// ------------------------------------------------------------------ tabs, help, tour
function showTab(v) {
  $$('#tabs button').forEach(b => { const on = b.dataset.v === v; b.classList.toggle('on', on); b.setAttribute('aria-selected', on ? 'true' : 'false'); b.tabIndex = on ? 0 : -1; });
  $$('.view').forEach(s => s.hidden = s.id !== 'v-' + v); if (v === 'map') { drawMap(); paintRows(true); renderSeriesChart(); }
  GIF.caption({ map: 'Map', plugin: 'Plugin: the request and reply', verify: 'Verify against EPA', doc: 'Doc', about: 'About' }[v]);
}
$$('#tabs button').forEach(b => b.addEventListener('click', () => showTab(b.dataset.v)));
// arrow keys move along the tabs (WAI-ARIA tabs pattern)
$('#tabs').addEventListener('keydown', e => {
  const tabs = $$('#tabs button'), i = tabs.indexOf(document.activeElement); if (i < 0) return;
  let j = null; if (e.key === 'ArrowRight') j = (i + 1) % tabs.length; else if (e.key === 'ArrowLeft') j = (i + tabs.length - 1) % tabs.length; else if (e.key === 'Home') j = 0; else if (e.key === 'End') j = tabs.length - 1;
  if (j === null) return; e.preventDefault(); tabs[j].focus(); showTab(tabs[j].dataset.v);
});
// the help dialog keeps focus inside itself, closes on Escape and hands focus back
let helpReturn = null;
function openHelp() { helpReturn = document.activeElement; $('#help').hidden = false; $('#tourbtn').focus(); }
function closeHelp() { $('#help').hidden = true; if (helpReturn && helpReturn.focus && document.contains(helpReturn)) helpReturn.focus(); else $('#helpbtn').focus(); }
$('#helpbtn').addEventListener('click', openHelp);
$('#helpclose').addEventListener('click', closeHelp);
$('#help').addEventListener('keydown', e => {
  if (e.key === 'Escape') { e.preventDefault(); closeHelp(); return; }
  if (e.key !== 'Tab') return;
  const f = [...$('#help').querySelectorAll('button, a[href], input, select, [tabindex]:not([tabindex="-1"])')].filter(x => !x.disabled);
  if (!f.length) return; const first = f[0], last = f[f.length - 1];
  if (e.shiftKey && document.activeElement === first) { e.preventDefault(); last.focus(); } else if (!e.shiftKey && document.activeElement === last) { e.preventDefault(); first.focus(); }
});
$('#help').addEventListener('click', e => { if (e.target === $('#help')) closeHelp(); });
const TOUR = [
  ['#demo', 'Pick a model', 'Six embedded models, from a three-subcatchment test to the Greenville (Naperville) network, or open your own .inp below.'],
  ['#runbtn', 'Run it in the plugin', 'This sends RustGIS\'s own request to plugin.wasm: the file, the EPSG code and the anchor point. Map only skips the engine.'],
  ['#mapwrap', 'Read the map', 'Drag to pan, wheel or pinch to zoom, click a feature to find its row. Colors come from the field chosen for each layer.'],
  ['#layercard', 'Choose what colors it', 'Each layer has a "color by" list of its numeric attributes: the engine\'s summary results after a run.'],
  ['#runsum', 'Read the run at a glance', 'Continuity, warnings, flooded and surcharged nodes, and what is measured about RustGIS: fuel, memory, and whether this run has been verified.'],
  ['#tscard', 'Watch it happen', 'Click a node or link for its hydrograph; drag the time slider (or press play) to color the map by any moment of the run.'],
  ['#msgcard', 'See what RustGIS would log', 'Continuity errors as reported, file hashes, and where the coordinates were placed.'],
  ['#tabs', 'Check it, install it', 'Verify runs EPA\'s own engine on the same file; Plugin shows the wire and the install steps; Doc explains the build.'],
  ['#helpbtn', 'Help is here', 'This button reopens the instructions and the tour.']];
let tourI = -1;
function tourStep() {
  if (tourI < 0 || tourI >= TOUR.length) { $('#tourbox').hidden = true; $('#tourring').hidden = true; tourI = -1; return; }
  const [sel, h, t] = TOUR[tourI]; if (sel !== '#tabs' && sel !== '#helpbtn') showTab('map'); const el = $(sel); el.scrollIntoView({ block: 'center', behavior: 'instant' });
  const r = el.getBoundingClientRect(), ring = $('#tourring'), box = $('#tourbox');
  ring.hidden = false; Object.assign(ring.style, { left: r.left - 4 + 'px', top: r.top - 4 + 'px', width: r.width + 8 + 'px', height: r.height + 8 + 'px' });
  $('#tourttl').textContent = h; $('#tourtxt').textContent = t; $('#tourn').textContent = `${tourI + 1} / ${TOUR.length}`; $('#tournext').textContent = tourI === TOUR.length - 1 ? 'Done' : 'Next';
  box.hidden = false; $('#tournext').focus({ preventScroll: true }); const bw = Math.min(340, window.innerWidth - 32); let left = Math.min(Math.max(16, r.left), window.innerWidth - bw - 16); let top = r.bottom + 12; if (top + 170 > window.innerHeight) top = Math.max(16, r.top - 180);
  Object.assign(box.style, { left: left + 'px', top: top + 'px', width: bw + 'px' });
}
$('#tourbtn').addEventListener('click', () => { $('#help').hidden = true; tourI = 0; tourStep(); });
$('#tourbox').addEventListener('keydown', e => { if (e.key === 'Escape') { e.preventDefault(); tourI = TOUR.length; tourStep(); $('#helpbtn').focus(); } });
$('#tournext').addEventListener('click', () => { tourI++; tourStep(); });
$('#tourskip').addEventListener('click', () => { tourI = TOUR.length; tourStep(); $('#helpbtn').focus(); });

// ------------------------------------------------------------------ Make a GIF: the live map, captioned, encoded here
const GIF = (() => {
  let cap = '', rec = null;
  function caption(t) { if (t) cap = t; }
  function frame() {
    const T = TH(), sw = Math.min(720, cv.width), sh = Math.round(cv.height * sw / cv.width), c = document.createElement('canvas'); c.width = sw; c.height = sh + 30;
    const g = c.getContext('2d'); g.fillStyle = T.face; g.fillRect(0, 0, c.width, c.height); g.drawImage(cv, 0, 0, sw, sh);
    g.fillStyle = THEME === 'dark' ? '#0b1220' : '#1e3a5f'; g.fillRect(0, sh, sw, 30); g.fillStyle = '#ffffff'; g.font = '14px Barlow, sans-serif'; g.fillText(cap.slice(0, 90), 10, sh + 20);
    return g.getImageData(0, 0, c.width, c.height);
  }
  function encode(frames, delay) {
    const w = frames[0].width, h = frames[0].height, out = [];
    // a 6x6x6 colour cube, plus the theme's own colours exactly (the map face, the ramps, the marks)
    const pal = []; for (let r = 0; r < 6; r++) for (let g = 0; g < 6; g++) for (let b = 0; b < 6; b++) pal.push([r * 51, g * 51, b * 51]);
    const exact = new Map(), T = TH(), hex = h => [parseInt(h.slice(1, 3), 16), parseInt(h.slice(3, 5), 16), parseInt(h.slice(5, 7), 16)];
    [T.face, T.grid, T.tick, T.axis, T.link, T.node, T.gage, T.sel, T.polyLine, ...T.ramp, '#0b1220', '#1e3a5f', '#ffffff', ...CHART_THEME.dark.ramp, ...CHART_THEME.light.ramp]
      .filter(h => /^#[0-9a-f]{6}$/i.test(h)).forEach(h => { const c = hex(h), k = c[0] << 16 | c[1] << 8 | c[2]; if (!exact.has(k) && pal.length < 256) { exact.set(k, pal.length); pal.push(c); } });
    while (pal.length < 256) pal.push([0, 0, 0]);
    const put = (...b) => out.push(...b), word = v => put(v & 255, v >> 8);
    put(71, 73, 70, 56, 57, 97); word(w); word(h); put(0xf7, 0, 0); pal.forEach(p => put(p[0], p[1], p[2]));
    put(0x21, 0xff, 11, ...[...'NETSCAPE2.0'].map(c => c.charCodeAt(0)), 3, 1, 0, 0, 0);
    for (const im of frames) {
      const d = im.data, idx = new Uint8Array(w * h);
      for (let i = 0, j = 0; i < idx.length; i++, j += 4) { const k = d[j] << 16 | d[j + 1] << 8 | d[j + 2], e = exact.get(k); idx[i] = e !== undefined ? e : Math.round(d[j] / 51) * 36 + Math.round(d[j + 1] / 51) * 6 + Math.round(d[j + 2] / 51); }
      put(0x21, 0xf9, 4, 0, delay & 255, delay >> 8, 0, 0); put(0x2c); word(0); word(0); word(w); word(h); put(0);
      // LZW, minimum code size 8
      put(8); const bytes = []; let cur = 0, nb = 0; const emit = (code, size) => { cur |= code << nb; nb += size; while (nb >= 8) { bytes.push(cur & 255); cur >>= 8; nb -= 8; } };
      let dict = new Map(), size = 9, next = 258; emit(256, size); let prefix = idx[0];
      for (let i = 1; i < idx.length; i++) { const k = prefix * 256 + idx[i]; const hit = dict.get(k);
        if (hit !== undefined) { prefix = hit; continue; }
        emit(prefix, size); if (next >= (1 << size) && size < 12) size++; if (next < 4096) dict.set(k, next++); else { emit(256, size); dict = new Map(); size = 9; next = 258; }
        prefix = idx[i]; }
      emit(prefix, size); emit(257, size); if (nb > 0) bytes.push(cur & 255);
      for (let i = 0; i < bytes.length; i += 255) { const n = Math.min(255, bytes.length - i); put(n); for (let k = 0; k < n; k++) out.push(bytes[i + k]); } put(0);
    }
    put(0x3b); return new Uint8Array(out);
  }
  async function tour() {
    const btn = $('#gifbtn'); btn.disabled = true; btn.textContent = 'Recording…'; showTab('map');
    const frames = [], snap = (t, n) => { caption(t); drawMap(); for (let i = 0; i < (n || 3); i++) frames.push(frame()); };
    const wait = ms => new Promise(r => setTimeout(r, ms));
    try {
      fit(); snap(`${S.model ? S.model.name : 'model'}: run in the swmm5-rust RustGIS plugin`, 4);
      const nodes = S.layers.find(l => l.kind === 'nodes'), links = S.layers.find(l => l.kind === 'links'), subs = S.layers.find(l => l.kind === 'subcatchments');
      if (subs) { S.sym[subs.name] = defaultField(subs); snap(`Subcatchments colored by ${S.sym[subs.name] || 'a single color'}`); }
      if (links) { S.sym[links.name] = defaultField(links); snap(`Links colored by ${S.sym[links.name] || 'a single color'}`); }
      if (nodes) { S.sym[nodes.name] = defaultField(nodes); snap(`Nodes colored by ${S.sym[nodes.name] || 'a single color'}`); }
      const { w, h } = size(); for (let i = 0; i < 4; i++) { zoomAt(w / 2, h / 2, 1.25); await wait(30); snap('Zooming in: every attribute is a number from EPA SWMM 5.2.4\'s report', 1); }
      for (let i = 0; i < 4; i++) { zoomAt(w / 2, h / 2, 0.8); await wait(30); snap('Same bytes as EPA\'s engine, inside RustGIS\'s sandbox', 1); }
      if (S.run && S.run.series) {
        const was = { on: S.time.on, k: S.time.k }, n = S.run.series.pa.nPer; S.time.on = true;
        for (let j = 0; j <= 16; j++) { S.time.k = Math.min(n - 1, Math.round(j * (n - 1) / 16)); snap(`Time slider: ${swmmDate(S.run.series.date[S.time.k])}`, 1); }
        Object.assign(S.time, was);
      }
      if (nodes) { const i = nodes.fc.features.reduce((m, f, k, a) => ((f.properties[S.sym[nodes.name]] || 0) > (a[m].properties[S.sym[nodes.name]] || 0) ? k : m), 0); S.sel = { layer: nodes.name, i }; snap(`Deepest node: ${nodes.fc.features[i].properties.id}`, 4); }
      renderLayers(); renderTable(); renderLegend();
      btn.textContent = 'Encoding…'; await wait(20);
      const gif = encode(frames, 50);
      await save(stem() + '-swmm5-rust-rustgis.gif', new Blob([gif], { type: 'image/gif' }));
    } catch (e) { toast('GIF failed: ' + e.message); }
    btn.disabled = false; btn.textContent = 'Make a GIF';
  }
  return { caption, tour };
})();
$('#gifbtn').addEventListener('click', () => GIF.tour());

// ------------------------------------------------------------------ boot
renderAbout(); staticPlugin(); renderStaticFigures(); renderEssay(); renderGates(); renderLayers(); renderMessages(); renderTable(); renderPluginTab(); renderVerifyIdle(); renderSummary(); renderSeries();
let firstVisit = true; try { firstVisit = !localStorage.getItem('swmm5rust_rustgis_seen'); localStorage.setItem('swmm5rust_rustgis_seen', '1'); } catch (e) {}
if (firstVisit) openHelp();
setModelFromDemo(0);
startWorker().then(() => runTool('run-swmm')).catch(e => { $('#runline').innerHTML = `<span class="chip warn">error</span> The plugin could not start here: ${esc(e.message)}`; });
window.__app = { S, runTool, runVerify, showTab, sha256, svgFuel, svgCensus, svgABI, svgMemfs, svgPipeline, loadSeries, findId, cancelRun, selectFeature, X };
