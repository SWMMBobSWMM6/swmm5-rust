# Two descriptions agree: every result attribute a layer holds is checked against the number EPA
# SWMM 5.2.4 printed in its report, found by this script's own reading of the .rpt (written
# separately from the plugin's Rust reader).
#
# Field by field: each attribute name maps to one table, one column and one unit, so a value
# that sits in the right row but under the wrong name (average depth filed as maximum depth)
# disagrees. A result attribute with no column is a disagreement; a column with no attribute is
# counted as missing. Numbers may be in scientific notation; rpt_type is compared as text.
import json, sys, os, re

NUM = re.compile(r'^[-+]?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?$')
def num(s):
    return float(s) if NUM.match(s) else None
def hrs(d, hm):
    m = re.match(r'^(\d+):(\d\d)$', hm)
    if not re.match(r'^\d+$', d) or not m: return None
    return round((int(d) * 24 + int(m.group(1)) + int(m.group(2)) / 60) * 1e4) / 1e4

def option(rpt, name):
    m = re.search(r'^\s*' + re.escape(name) + r' \.+ (\S+)', rpt, re.M)
    return m.group(1) if m else None

def table(rpt, title):
    """(heading lines, data rows as cell lists, System row cells or None), or None if absent."""
    L = rpt.split('\n')
    try: i = next(k for k, l in enumerate(L) if l.strip() == title)
    except StopIteration: return None
    head, rows, system, d = [], [], None, 0
    for l in L[i + 1:]:
        t = l.strip()
        if d == 0 and t.startswith('No '): return head, rows, None
        if re.fullmatch(r'-{3,}', t):
            d += 1; continue
        if d == 1: head.append(t)
        elif d == 2:
            if not t: break
            rows.append(t.split())
        elif d == 3:
            if t.startswith('System'): system = t.split()
            break
    return head, rows, system

NODE_TYPES = {'JUNCTION', 'OUTFALL', 'STORAGE', 'DIVIDER'}
LINK_TYPES = {'CONDUIT', 'CHANNEL', 'DUMMY', 'PUMP', 'ORIFICE', 'WEIR', 'OUTLET'}

def split(cells, n):
    """name, the last n cells; None if the row is shorter."""
    if len(cells) < n + 1: return None
    return ' '.join(cells[:len(cells) - n]), cells[len(cells) - n:]

def nums(cells):
    v = [num(c) for c in cells]
    return None if any(x is None for x in v) else v

def expected(rpt):
    """{kind: {object name: {attribute: value}}} from EPA's report, one entry per column."""
    fu = (option(rpt, 'Flow Units') or 'CFS').upper()
    us = fu in ('CFS', 'GPM', 'MGD'); q = fu.lower()
    L, DIN, V6, V3, VEL, BAL = ('ft', 'in', 'MG', '1000ft3', 'fps', 'gal') if us else ('m', 'mm', 'ML', '1000m3', 'mps', 'ltr')
    E = {'subcatch': {}, 'node': {}, 'link': {}}
    def put(kind, name, kv):
        E[kind].setdefault(name, {}).update(kv)

    t = table(rpt, 'Subcatchment Runoff Summary')
    for c in (t[1] if t else []):
        s = split(c, 10); v = s and nums(s[1])
        if v: put('subcatch', s[0], {f'precip_{DIN}': v[0], f'runon_{DIN}': v[1], f'evap_{DIN}': v[2], f'infil_{DIN}': v[3],
            f'imperv_runoff_{DIN}': v[4], f'perv_runoff_{DIN}': v[5], f'runoff_{DIN}': v[6], f'runoff_vol_{V6}': v[7],
            f'peak_runoff_{q}': v[8], 'runoff_coeff': v[9]})
    t = table(rpt, 'Node Depth Summary')
    for c in (t[1] if t else []):
        s = split(c, 7)
        if not s or s[1][0] not in NODE_TYPES: continue
        x = s[1]; v = nums([x[1], x[2], x[3], x[6]]); h = hrs(x[4], x[5])
        if v and h is not None: put('node', s[0], {f'avg_depth_{L}': v[0], f'max_depth_{L}': v[1], f'max_hgl_{L}': v[2],
            't_max_depth_hr': h, f'rpt_max_depth_{L}': v[3]})
    t = table(rpt, 'Node Inflow Summary')
    for c in (t[1] if t else []):
        unit = bool(c) and c[-1].isalpha()
        s = split(c, 9 if unit else 8)
        if not s or s[1][0] not in NODE_TYPES: continue
        x = s[1]; v = nums([x[1], x[2], x[5], x[6], x[7]]); h = hrs(x[3], x[4])
        if v and h is not None: put('node', s[0], {f'max_lat_inflow_{q}': v[0], f'max_total_inflow_{q}': v[1], 't_max_inflow_hr': h,
            f'lat_inflow_vol_{V6}': v[2], f'total_inflow_vol_{V6}': v[3],
            'flow_bal_err_pct': None if unit else v[4], f'flow_bal_err_{BAL}': v[4] if unit else None})
    t = table(rpt, 'Node Surcharge Summary')
    for c in (t[1] if t else []):
        s = split(c, 4)
        if not s or s[1][0] not in NODE_TYPES: continue
        v = nums(s[1][1:])
        if v: put('node', s[0], {'surcharge_hr': v[0], f'max_above_crown_{L}': v[1], f'min_below_rim_{L}': v[2]})
    t = table(rpt, 'Node Flooding Summary')
    if t:
        # the last column is a ponded depth (Feet/Meters) or a ponded volume (1000 ft3/m3): read it off the heading
        depth = any(h.split() and h.split()[-1] in ('Feet', 'Meters') for h in t[0])
        for c in t[1]:
            s = split(c, 6)
            if not s: continue
            x = s[1]; v = nums([x[0], x[1], x[4], x[5]]); h = hrs(x[2], x[3])
            if v and h is not None: put('node', s[0], {'flood_hr': v[0], f'max_flood_rate_{q}': v[1], 't_max_flood_hr': h,
                f'flood_vol_{V6}': v[2], (f'max_ponded_depth_{L}' if depth else f'max_ponded_vol_{V3}'): v[3]})
    t = table(rpt, 'Storage Volume Summary')
    for c in (t[1] if t else []):
        s = split(c, 9)
        if not s: continue
        x = s[1]; v = nums([x[0], x[1], x[2], x[3], x[4], x[5], x[8]]); h = hrs(x[6], x[7])
        if v and h is not None: put('node', s[0], {f'avg_vol_{V3}': v[0], 'avg_pct_full': v[1], 'evap_pct_loss': v[2],
            'exfil_pct_loss': v[3], f'max_vol_{V3}': v[4], 'max_pct_full': v[5], 't_max_vol_hr': h, f'max_outflow_{q}': v[6]})
    t = table(rpt, 'Outfall Loading Summary')
    if t and t[2]:
        n = len(t[2]) - 1                      # the System row has one cell per column
        for c in t[1]:
            s = split(c, n); v = s and nums(s[1])
            if v: put('node', s[0], {'flow_freq_pct': v[0], f'avg_flow_{q}': v[1], f'max_flow_{q}': v[2], f'total_vol_{V6}': v[3]})
    t = table(rpt, 'Link Flow Summary')
    for c in (t[1] if t else []):
        p = next((k for k in range(1, len(c) - 3) if c[k] in LINK_TYPES and num(c[k + 1]) is not None and hrs(c[k + 2], c[k + 3]) is not None), None)
        if p is None: continue
        ty, rest = c[p], c[p + 4:]
        kv = {'rpt_type': ty, f'max_flow_{q}': num(c[p + 1]), 't_max_flow_hr': hrs(c[p + 2], c[p + 3])}
        ok = True
        if ty in ('CONDUIT', 'CHANNEL'):
            if not 2 <= len(rest) <= 3: ok = False
            else:
                if rest[0] == '>50.00': kv[f'max_veloc_{VEL}'] = None; kv['veloc_over_50'] = True
                else: kv[f'max_veloc_{VEL}'] = num(rest[0])
                kv['max_full_flow'] = num(rest[1])
                if len(rest) == 3: kv['max_full_depth'] = num(rest[2])
        elif ty == 'PUMP':
            if len(rest) > 1: ok = False
            elif rest: kv['max_full_flow'] = num(rest[0])
        elif ty in ('DUMMY', 'OUTLET'):
            ok = not rest
        else:
            if len(rest) > 1: ok = False
            elif rest: kv['max_full_depth'] = num(rest[0])
        if ok and not any(v is None for k, v in kv.items() if k != f'max_veloc_{VEL}'):
            put('link', ' '.join(c[:p]), kv)
    t = table(rpt, 'Conduit Surcharge Summary')
    for c in (t[1] if t else []):
        s = split(c, 5); v = s and nums(s[1])
        if v: put('link', s[0], {'hrs_full_both': v[0], 'hrs_full_up': v[1], 'hrs_full_dn': v[2], 'hrs_above_full_normal': v[3], 'hrs_capacity_ltd': v[4]})
    t = table(rpt, 'Pumping Summary')
    for c in (t[1] if t else []):
        s = split(c, 9); v = s and nums(s[1])
        if v: put('link', s[0], {'pct_utilized': v[0], 'startups': v[1], f'min_flow_{q}': v[2], f'avg_flow_{q}': v[3],
            f'max_pump_flow_{q}': v[4], f'pumped_vol_{V6}': v[5], 'power_kwh': v[6], 'pct_off_curve_low': v[7], 'pct_off_curve_high': v[8]})
    return E

# input attributes (from the .inp, not the report)
INPUT = re.compile(r'^(id|type|rain_gage|outlet|area_(ac|ha)|pct_imperv|width_(ft|m)|slope_pct|from_node|to_node|length_(ft|m)|roughness|shape|geom1_(ft|m)|barrels|invert_(ft|m)|full_depth_(ft|m)|format|interval|source)$')

def same(a, b):
    if a is None or b is None: return a is None and b is None
    if isinstance(a, bool) or isinstance(b, bool): return a is b
    if isinstance(a, str) or isinstance(b, str): return a == b
    return abs(a - b) <= 1e-9 * max(1.0, abs(a))

def check_features(E_kind, feats):
    ok = bad = missing = 0; badlist = []
    upper = {k.upper(): k for k in E_kind}
    for f in feats:
        p = f['properties']; fid = p['id']
        exp = E_kind.get(fid) or E_kind.get(upper.get(str(fid).upper(), ''), {})
        for k, v in p.items():
            if INPUT.match(k): continue
            if k in exp:
                if same(exp[k], v): ok += 1
                else: bad += 1; badlist.append((fid, k, v, exp[k]))
            elif v is not None:
                # (a null where the report has no column is an empty cell: RustGIS's exporter gives
                # every feature every field of its layer)
                bad += 1; badlist.append((fid, k, v, 'no such column for this object in the report'))
        for k in exp:
            if k not in p: missing += 1; badlist.append((fid, k, 'absent', exp[k]))
    return ok, bad, missing, badlist

def check(rpt_path, layer_path, kind):
    rpt = open(rpt_path, encoding='latin-1').read(); fc = json.load(open(layer_path))
    ok, bad, missing, bl = check_features(expected(rpt)[kind], fc['features'])
    return ok, bad + missing, bl[:8], len(fc['features'])

if __name__ == '__main__':
    tag, rpt = sys.argv[1], sys.argv[2]; d = os.path.dirname(os.path.abspath(sys.argv[3])) if len(sys.argv) > 3 else '/root/work/rgtest/out'
    tot = [0, 0]
    for kind, name in (('subcatch', 'subcatchments'), ('link', 'links'), ('node', 'nodes')):
        p = f'{d}/{tag}.{name}.geojson'
        if not os.path.exists(p): continue
        ok, bad, bl, n = check(rpt, p, kind); tot[0] += ok; tot[1] += bad
        print(f'{tag:34s} {name:14s} {n:5d} features  {ok:6d} values agree  {bad} disagree or missing', bl if bad else '')
    print(f'{tag:34s} TOTAL {tot[0]} agree, {tot[1]} disagree or missing')
