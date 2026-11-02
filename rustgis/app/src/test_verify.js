// The Verify tab's checker on the six demos: the plugin (through the node host) against EPA's
// report (EPA's wasm engine), field by field. Prints the counts, the planted-defect self-test,
// and what the page's previous checker said on the same layers.
//   node test_verify.js > test_verify.json
const fs = require('fs');
const { runPlugin, toolRequest } = require('/root/work/host/rghost.js');
const core = require('/root/work/swmm5-rust/app/engine_core.js');
const V = require('./verify.js');
const plugin = fs.readFileSync('/root/work/swmm5-rust/rustgis/dist/plugin.wasm');
const epa = new WebAssembly.Module(fs.readFileSync('/root/work/epawasm/swmm524_epa.wasm'));
const DEMOS = [['jason_dorn_storage_example.inp', ''], ['swmm_example.inp', ''], ['Example_LID_RB.inp', ''], ['controlExample.inp', ''], ['user2.inp', ''], ['Greenville_all_SWMM5_Features.inp', '3443']];
// the page's checker as it was before this fix (app.js checkAttrs), kept to show what it missed
function oldRows(rpt, title) { const L = rpt.split('\n'); const i = L.findIndex(l => l.trim() === title); const out = {}; if (i < 0) return out; let d = 0;
  for (const l of L.slice(i + 1)) { const t = l.trim(); if (d === 0 && t.startsWith('No ')) return out; if (/^-{3,}$/.test(t)) { d++; if (d > 2) break; continue; }
    if (d === 2) { if (!t) break; const tok = t.split(/\s+/); if (tok[0] !== 'System') out[tok[0]] = tok; } } return out; }
const TABLES = { subcatchments: ['Subcatchment Runoff Summary'], nodes: ['Node Depth Summary', 'Node Inflow Summary', 'Node Surcharge Summary', 'Node Flooding Summary', 'Storage Volume Summary', 'Outfall Loading Summary'], links: ['Link Flow Summary', 'Conduit Surcharge Summary', 'Pumping Summary'] };
const INPUTS = /^(id|type|rain_gage|outlet|area_(ac|ha)|pct_imperv|width_(ft|m)|slope_pct|from_node|to_node|length_(ft|m)|roughness|shape|geom1_(ft|m)|barrels|invert_(ft|m)|full_depth_(ft|m)|format|interval|source|veloc_over_50)$/;
const kindOf = n => / subcatchments$/.test(n) ? 'subcatchments' : / links$/.test(n) ? 'links' : / nodes$/.test(n) ? 'nodes' : / rain gages$/.test(n) ? 'gages' : 'table';
function oldCheck(rpt, layers) { let agree = 0, dis = 0; const why = { string: 0, sci: 0, other: 0 };
  for (const L of layers) { const kinds = TABLES[kindOf(L.name)]; if (!kinds) continue; const T = kinds.map(t => oldRows(rpt, t));
    for (const f of L.fc.features) { const p = f.properties, id = p.id;
      for (const [k, v] of Object.entries(p)) { if (INPUTS.test(k) || v == null) continue; const hrs = /_hr$/.test(k) && /^t_max/.test(k);
        const ok = T.some(rows => { const tok = rows[id]; if (!tok) return false;
          if (hrs) { for (let j = 1; j + 1 < tok.length; j++) if (/^\d+$/.test(tok[j]) && /^\d\d:\d\d$/.test(tok[j + 1])) { const [hh, mm] = tok[j + 1].split(':').map(Number); if (Math.abs(Math.round((+tok[j] * 24 + hh + mm / 60) * 1e4) / 1e4 - v) < 1e-9) return true; } return false; }
          return tok.some(x => /^-?[\d.]+$/.test(x) && Math.abs(parseFloat(x) - v) <= 1e-9 * Math.max(1, Math.abs(v))); });
        if (ok) agree++; else { dis++; if (typeof v === 'string') why.string++; else if (T.some(r => r[id] && r[id].some(x => /e[-+]\d/.test(x) && Math.abs(parseFloat(x) - v) <= 1e-9 * Math.max(1, Math.abs(v))))) why.sci++; else why.other++; } } } }
  return { agree, dis, why }; }
const out = [];
for (const [f, epsg] of DEMOS) {
  const req = toolRequest('run-swmm', '/root/work/runs/' + f, epsg ? { epsg } : {});
  const r = runPlugin(plugin, req); const ok = r.ok;
  const layers = ok.layers.map(l => ({ name: l.name, fc: l.geojson }));
  const tables = ok.tables.map(t => ({ name: t.name, fc: t.geojson }));
  const e = core.runLegacy(epa, 'm.inp', { 'm.inp': fs.readFileSync('/root/work/runs/' + f) });
  const rpt = Buffer.from(e.rpt).toString('latin1');
  const now = V.check(rpt, [...layers, ...tables]), layersOnly = V.check(rpt, layers);
  const st = V.selfTest(rpt, [...layers, ...tables]);
  const old = oldCheck(rpt, layers);
  // the old checker on a planted swap of average and maximum depth
  const sw = JSON.parse(JSON.stringify(layers)); const nl = sw.find(l => / nodes$/.test(l.name));
  const node = nl && nl.fc.features.find(x => x.properties.avg_depth_ft !== undefined && x.properties.avg_depth_ft !== x.properties.max_depth_ft);
  if (node) node.properties.max_depth_ft = node.properties.avg_depth_ft;
  const oldSwap = node ? oldCheck(rpt, sw) : null;
  out.push({ model: f, now: { agree: now.agree, dis: now.dis, missing: now.missing, bad: now.bad.slice(0, 3) }, layers_only: { agree: layersOnly.agree, dis: layersOnly.dis, missing: layersOnly.missing },
    selftest: st, old, old_on_swap: oldSwap && { dis: oldSwap.dis, caught: oldSwap.dis > old.dis } });
  for (const L of layers) fs.writeFileSync(`/root/work/app/vtest/${f.slice(0, -4)}.${L.name.split(' ').slice(1).join('_')}.geojson`, JSON.stringify(L.fc));
  fs.writeFileSync(`/root/work/app/vtest/${f.slice(0, -4)}.rpt`, Buffer.from(e.rpt));
}
console.log(JSON.stringify(out, null, 1));
