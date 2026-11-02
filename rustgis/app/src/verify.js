// verify.js -- the Verify tab's attribute check. Reads EPA SWMM 5.2.4's report on its own
// (separately from the plugin's Rust reader) and checks every result attribute field by field:
// each attribute name maps to one table, one column and one unit. A value in the right row
// under the wrong name disagrees; a result attribute with no column disagrees; a column the
// layer lacks is missing. Numbers may be in scientific notation; rpt_type is compared as text.
// Used by the page (inlined at build time) and by test_verify.js under node.
(function (root) {
  'use strict';
  const NUM = /^[-+]?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?$/;
  const num = s => NUM.test(s) ? parseFloat(s) : undefined;
  function hrs(d, hm) {
    const m = /^(\d+):(\d\d)$/.exec(hm || '');
    if (!/^\d+$/.test(d || '') || !m) return undefined;
    return Math.round((+d * 24 + +m[1] + +m[2] / 60) * 1e4) / 1e4;
  }
  function option(rpt, name) {
    const m = new RegExp('^\\s*' + name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') + ' \\.+ (\\S+)', 'm').exec(rpt);
    return m ? m[1] : null;
  }
  // { head: heading lines, rows: cell arrays, system: System row cells | null } or null
  function table(rpt, title) {
    const L = rpt.split('\n'); const i = L.findIndex(l => l.trim() === title); if (i < 0) return null;
    const head = [], rows = []; let system = null, d = 0;
    for (const l of L.slice(i + 1)) {
      const t = l.trim();
      if (d === 0 && t.startsWith('No ')) return { head, rows, system: null };
      if (/^-{3,}$/.test(t)) { d++; continue; }
      if (d === 1) head.push(t);
      else if (d === 2) { if (!t) break; rows.push(t.split(/\s+/)); }
      else if (d === 3) { if (t.startsWith('System')) system = t.split(/\s+/); break; }
    }
    return { head, rows, system };
  }
  const NODE_TYPES = ['JUNCTION', 'OUTFALL', 'STORAGE', 'DIVIDER'];
  const LINK_TYPES = ['CONDUIT', 'CHANNEL', 'DUMMY', 'PUMP', 'ORIFICE', 'WEIR', 'OUTLET'];
  const split = (c, n) => c.length < n + 1 ? null : [c.slice(0, c.length - n).join(' '), c.slice(c.length - n)];
  const nums = c => { const v = c.map(num); return v.some(x => x === undefined) ? null : v; };

  // { subcatchments: {name: {attr: value}}, nodes: {...}, links: {...} }
  function expected(rpt) {
    const fu = (option(rpt, 'Flow Units') || 'CFS').toUpperCase(), us = ['CFS', 'GPM', 'MGD'].includes(fu), q = fu.toLowerCase();
    const [L, DIN, V6, V3, VEL, BAL] = us ? ['ft', 'in', 'MG', '1000ft3', 'fps', 'gal'] : ['m', 'mm', 'ML', '1000m3', 'mps', 'ltr'];
    const E = { subcatchments: {}, nodes: {}, links: {} };
    const put = (k, name, kv) => { E[k][name] = Object.assign(E[k][name] || {}, kv); };
    let t;
    t = table(rpt, 'Subcatchment Runoff Summary');
    for (const c of t ? t.rows : []) { const s = split(c, 10), v = s && nums(s[1]);
      if (v) put('subcatchments', s[0], { ['precip_' + DIN]: v[0], ['runon_' + DIN]: v[1], ['evap_' + DIN]: v[2], ['infil_' + DIN]: v[3],
        ['imperv_runoff_' + DIN]: v[4], ['perv_runoff_' + DIN]: v[5], ['runoff_' + DIN]: v[6], ['runoff_vol_' + V6]: v[7], ['peak_runoff_' + q]: v[8], runoff_coeff: v[9] }); }
    t = table(rpt, 'Node Depth Summary');
    for (const c of t ? t.rows : []) { const s = split(c, 7); if (!s || !NODE_TYPES.includes(s[1][0])) continue;
      const x = s[1], v = nums([x[1], x[2], x[3], x[6]]), h = hrs(x[4], x[5]);
      if (v && h !== undefined) put('nodes', s[0], { ['avg_depth_' + L]: v[0], ['max_depth_' + L]: v[1], ['max_hgl_' + L]: v[2], t_max_depth_hr: h, ['rpt_max_depth_' + L]: v[3] }); }
    t = table(rpt, 'Node Inflow Summary');
    for (const c of t ? t.rows : []) { const unit = /^[A-Za-z]+$/.test(c[c.length - 1]); const s = split(c, unit ? 9 : 8); if (!s || !NODE_TYPES.includes(s[1][0])) continue;
      const x = s[1], v = nums([x[1], x[2], x[5], x[6], x[7]]), h = hrs(x[3], x[4]);
      if (v && h !== undefined) put('nodes', s[0], { ['max_lat_inflow_' + q]: v[0], ['max_total_inflow_' + q]: v[1], t_max_inflow_hr: h,
        ['lat_inflow_vol_' + V6]: v[2], ['total_inflow_vol_' + V6]: v[3], flow_bal_err_pct: unit ? null : v[4], ['flow_bal_err_' + BAL]: unit ? v[4] : null }); }
    t = table(rpt, 'Node Surcharge Summary');
    for (const c of t ? t.rows : []) { const s = split(c, 4); if (!s || !NODE_TYPES.includes(s[1][0])) continue;
      const v = nums(s[1].slice(1)); if (v) put('nodes', s[0], { surcharge_hr: v[0], ['max_above_crown_' + L]: v[1], ['min_below_rim_' + L]: v[2] }); }
    t = table(rpt, 'Node Flooding Summary');
    if (t) {
      // the last column is a ponded depth (Feet/Meters) or a ponded volume (1000 ft3/m3): read it off the heading
      const depth = t.head.some(h => /\b(Feet|Meters)$/.test(h));
      for (const c of t.rows) { const s = split(c, 6); if (!s) continue; const x = s[1], v = nums([x[0], x[1], x[4], x[5]]), h = hrs(x[2], x[3]);
        if (v && h !== undefined) put('nodes', s[0], { flood_hr: v[0], ['max_flood_rate_' + q]: v[1], t_max_flood_hr: h, ['flood_vol_' + V6]: v[2],
          [depth ? 'max_ponded_depth_' + L : 'max_ponded_vol_' + V3]: v[3] }); }
    }
    t = table(rpt, 'Storage Volume Summary');
    for (const c of t ? t.rows : []) { const s = split(c, 9); if (!s) continue; const x = s[1], v = nums([x[0], x[1], x[2], x[3], x[4], x[5], x[8]]), h = hrs(x[6], x[7]);
      if (v && h !== undefined) put('nodes', s[0], { ['avg_vol_' + V3]: v[0], avg_pct_full: v[1], evap_pct_loss: v[2], exfil_pct_loss: v[3], ['max_vol_' + V3]: v[4],
        max_pct_full: v[5], t_max_vol_hr: h, ['max_outflow_' + q]: v[6] }); }
    t = table(rpt, 'Outfall Loading Summary');
    if (t && t.system) { const n = t.system.length - 1;            // the System row has one cell per column
      for (const c of t.rows) { const s = split(c, n), v = s && nums(s[1]);
        if (v) put('nodes', s[0], { flow_freq_pct: v[0], ['avg_flow_' + q]: v[1], ['max_flow_' + q]: v[2], ['total_vol_' + V6]: v[3] }); } }
    t = table(rpt, 'Link Flow Summary');
    for (const c of t ? t.rows : []) {
      let p = -1; for (let k = 1; k + 3 < c.length; k++) if (LINK_TYPES.includes(c[k]) && num(c[k + 1]) !== undefined && hrs(c[k + 2], c[k + 3]) !== undefined) { p = k; break; }
      if (p < 0) continue;
      const ty = c[p], rest = c.slice(p + 4), kv = { rpt_type: ty, ['max_flow_' + q]: num(c[p + 1]), t_max_flow_hr: hrs(c[p + 2], c[p + 3]) };
      let ok = true;
      if (ty === 'CONDUIT' || ty === 'CHANNEL') {
        if (rest.length < 2 || rest.length > 3) ok = false;
        else { if (rest[0] === '>50.00') { kv['max_veloc_' + VEL] = null; kv.veloc_over_50 = true; } else kv['max_veloc_' + VEL] = num(rest[0]);
          kv.max_full_flow = num(rest[1]); if (rest.length === 3) kv.max_full_depth = num(rest[2]); }
      } else if (ty === 'PUMP') { if (rest.length > 1) ok = false; else if (rest.length) kv.max_full_flow = num(rest[0]); }
      else if (ty === 'DUMMY' || ty === 'OUTLET') ok = !rest.length;
      else { if (rest.length > 1) ok = false; else if (rest.length) kv.max_full_depth = num(rest[0]); }
      if (ok && !Object.entries(kv).some(([k, v]) => v === undefined)) put('links', c.slice(0, p).join(' '), kv);
    }
    t = table(rpt, 'Conduit Surcharge Summary');
    for (const c of t ? t.rows : []) { const s = split(c, 5), v = s && nums(s[1]);
      if (v) put('links', s[0], { hrs_full_both: v[0], hrs_full_up: v[1], hrs_full_dn: v[2], hrs_above_full_normal: v[3], hrs_capacity_ltd: v[4] }); }
    t = table(rpt, 'Pumping Summary');
    for (const c of t ? t.rows : []) { const s = split(c, 9), v = s && nums(s[1]);
      if (v) put('links', s[0], { pct_utilized: v[0], startups: v[1], ['min_flow_' + q]: v[2], ['avg_flow_' + q]: v[3], ['max_pump_flow_' + q]: v[4],
        ['pumped_vol_' + V6]: v[5], power_kwh: v[6], pct_off_curve_low: v[7], pct_off_curve_high: v[8] }); }
    return E;
  }

  // attributes that come from the .inp, not the report
  const INPUT = /^(id|type|rain_gage|outlet|area_(ac|ha)|pct_imperv|width_(ft|m)|slope_pct|from_node|to_node|length_(ft|m)|roughness|shape|geom1_(ft|m)|barrels|invert_(ft|m)|full_depth_(ft|m)|format|interval|source)$/;
  function same(a, b) {
    if (a === null || b === null || a === undefined || b === undefined) return (a === null || a === undefined) && (b === null || b === undefined);
    if (typeof a === 'boolean' || typeof b === 'boolean') return a === b;
    if (typeof a === 'string' || typeof b === 'string') return a === b;
    return Math.abs(a - b) <= 1e-9 * Math.max(1, Math.abs(a));
  }
  // features of one kind against the report's expectations for that kind
  function checkFeatures(Ek, feats) {
    let agree = 0, dis = 0, missing = 0; const bad = [];
    const upper = new Map(Object.keys(Ek).map(k => [k.toUpperCase(), k]));
    for (const f of feats) {
      const p = f.properties || {}, id = p.id;
      const exp = Ek[id] || Ek[upper.get(String(id).toUpperCase())] || {};
      for (const [k, v] of Object.entries(p)) {
        if (INPUT.test(k)) continue;
        if (k in exp) { if (same(exp[k], v)) agree++; else { dis++; bad.push({ id, k, v, exp: exp[k] }); } }
        else if (v !== null && v !== undefined) { dis++; bad.push({ id, k, v, exp: 'no such column for this object in EPA’s report' }); }
      }
      for (const k of Object.keys(exp)) if (!(k in p)) { missing++; bad.push({ id, k, v: 'absent', exp: exp[k] }); }
    }
    return { agree, dis, missing, bad };
  }
  const KIND = n => / subcatchments( \(|$)/.test(n) ? 'subcatchments' : / links( \(|$)/.test(n) ? 'links' : / nodes( \(|$)/.test(n) ? 'nodes' : null;
  // layers: [{name, fc}] -- drawn layers and the "no coordinates" tables alike
  function check(rpt, layers) {
    const E = expected(rpt), res = []; let agree = 0, dis = 0, missing = 0; const bad = [];
    for (const L of layers) {
      const k = KIND(L.name); if (!k) continue;
      const r = checkFeatures(E[k], L.fc.features);
      res.push({ layer: L.name, n: L.fc.features.length, a: r.agree, d: r.dis, m: r.missing });
      agree += r.agree; dis += r.dis; missing += r.missing; for (const b of r.bad) if (bad.length < 12) bad.push({ layer: L.name, ...b });
    }
    return { res, agree, dis, missing, bad };
  }
  // planted defects: each must be caught, or the check proves nothing
  function selfTest(rpt, layers) {
    const clone = o => JSON.parse(JSON.stringify(o));
    const find = (kind, pred) => { for (const L of layers) if (KIND(L.name) === kind) { const i = L.fc.features.findIndex(f => pred(f.properties || {})); if (i >= 0) return [L, i]; } return null; };
    const plants = [
      ['maximum depth replaced by average depth', () => find('nodes', p => typeof p.avg_depth_ft === 'number' ? p.avg_depth_ft !== p.max_depth_ft : typeof p.avg_depth_m === 'number' && p.avg_depth_m !== p.max_depth_m),
        p => { const u = 'avg_depth_ft' in p ? 'ft' : 'm'; p['max_depth_' + u] = p['avg_depth_' + u]; }],
      ['a link’s report type changed', () => find('links', p => typeof p.rpt_type === 'string'), p => { p.rpt_type = p.rpt_type === 'WEIR' ? 'CONDUIT' : 'WEIR'; }],
      ['a unit relabeled (feet as meters)', () => find('nodes', p => typeof p.max_depth_ft === 'number' || typeof p.max_depth_m === 'number'),
        p => { if ('max_depth_ft' in p) { p.max_depth_m = p.max_depth_ft; delete p.max_depth_ft; } else { p.max_depth_ft = p.max_depth_m; delete p.max_depth_m; } }],
      ['a peak runoff 0.1% high', () => find('subcatchments', p => Object.keys(p).some(k => /^peak_runoff_/.test(k) && typeof p[k] === 'number')),
        p => { const k = Object.keys(p).find(k => /^peak_runoff_/.test(k)); p[k] = p[k] * 1.001 + 0.001; }],
      ['time of maximum depth one minute late', () => find('nodes', p => typeof p.t_max_depth_hr === 'number'), p => { p.t_max_depth_hr += 1 / 60; }],
      ['a result left out', () => find('nodes', p => typeof p.max_hgl_ft === 'number' || typeof p.max_hgl_m === 'number'), p => { delete p['max_hgl_ft' in p ? 'max_hgl_ft' : 'max_hgl_m']; }],
    ];
    const out = [];
    for (const [what, locate, plant] of plants) {
      const at = locate(); if (!at) { out.push({ what, applicable: false }); continue; }
      const [L, i] = at, copy = layers.map(l => l === L ? { name: l.name, fc: clone(l.fc) } : l);
      plant(copy.find(l => l.name === L.name).fc.features[i].properties);
      const r = check(rpt, copy.filter(l => l.name === L.name));
      out.push({ what, applicable: true, caught: r.dis + r.missing > 0, id: L.fc.features[i].properties.id });
    }
    return out;
  }
  const API = { expected, check, checkFeatures, selfTest, option, table };
  if (typeof module !== 'undefined' && module.exports) module.exports = API; else root.VERIFY = API;
})(typeof self !== 'undefined' ? self : this);
