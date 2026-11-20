#!/usr/bin/env python3
"""Check a GeoPackage written by swmm5-gpkg against things it did not come from.

  1  structure, read with Python's sqlite3 (not terrio): the GeoPackage tables, contents, reference systems
  2  geometry, decoded here from the GeoPackage blobs: every node point, link line and subcatchment
     ring against the numbers in the .inp
  3  attributes against swmm-toolkit (OWA's C reader): the peak of every variable of every object,
     exactly, and the time of each peak
  4  attributes against a published swmm-run-record record.json, when one is given
  5  GDAL (pyogrio) opens every layer and sees the same counts, types and reference system
  6  the .rpt summary tables: how far the saved-period peaks sit below the all-routing-step maxima

Needs: pip install swmm-toolkit numpy pyogrio

usage: check_gpkg.py FILE.gpkg MODEL.inp RESULTS.out [--record record.json] [--rpt MODEL.rpt] [--epsg N]
"""
import json, re, sqlite3, struct, sys
import numpy as np

args = sys.argv[1:]
def opt(name):
    if name in args:
        i = args.index(name)
        v = args[i + 1]
        del args[i:i + 2]
        return v
    return None
record, rpt, epsg = opt('--record'), opt('--rpt'), opt('--epsg')
gpkg, inp, out = args
fail = []
def check(ok, msg):
    print(('  ok   ' if ok else '  FAIL ') + msg)
    if not ok:
        fail.append(msg)

# --- 1 structure ---------------------------------------------------------------
db = sqlite3.connect(gpkg)
app_id = db.execute('pragma application_id').fetchone()[0]
check(app_id == 0x47504B47, 'application_id is GPKG (%#x)' % app_id)
check(db.execute('pragma integrity_check').fetchone()[0] == 'ok', 'sqlite integrity_check')
contents = {r[0]: r for r in db.execute('select table_name, data_type, srs_id, min_x, min_y, max_x, max_y from gpkg_contents')}
check(set(contents) == {'nodes', 'links', 'subcatchments', 'swmm_run', 'swmm_fields'}, 'contents: %s' % sorted(contents))
check([contents[t][1] for t in ('nodes', 'links', 'subcatchments', 'swmm_run', 'swmm_fields')] == ['features'] * 3 + ['attributes'] * 2, 'three feature tables, two attribute tables')
geomcols = {r[0]: r for r in db.execute('select table_name, column_name, geometry_type_name, srs_id from gpkg_geometry_columns')}
check([geomcols[t][2] for t in ('nodes', 'links', 'subcatchments')] == ['POINT', 'LINESTRING', 'POLYGON'], 'geometry types POINT, LINESTRING, POLYGON')
want_srs = int(epsg) if epsg else 99999      # 99999: the "Undefined SRS" entry GDAL writes and reads as "no reference system"
check(all(geomcols[t][3] == want_srs for t in geomcols), 'srs_id %d on every layer%s' % (want_srs, '' if epsg else ' (Undefined SRS)'))
check(db.execute('select count(*) from gpkg_spatial_ref_sys where srs_id=?', (want_srs,)).fetchone()[0] == 1, 'that srs_id is defined in gpkg_spatial_ref_sys')
run = dict(db.execute('select key, value from swmm_run where key != "note"'))
fields = {(l, f): u for l, f, u in db.execute('select layer, field, unit from swmm_fields')}

# --- the .inp, read here independently -------------------------------------------
def inp_sections(path):
    sec, data = None, {}
    for line in open(path, encoding='latin-1'):
        line = line.split(';')[0].strip()
        if not line:
            continue
        if line.startswith('['):
            sec = line.strip('[]').upper()
            continue
        data.setdefault(sec, []).append(re.findall(r'"[^"]*"|\S+', line))
    return data
S = inp_sections(inp)
unq = lambda s: s.strip('"')
coords = {unq(t[0]): (float(t[1]), float(t[2])) for t in S.get('COORDINATES', [])}
verts, polys = {}, {}
for t in S.get('VERTICES', []):
    verts.setdefault(unq(t[0]), []).append((float(t[1]), float(t[2])))
for t in S.get('POLYGONS', []):
    polys.setdefault(unq(t[0]), []).append((float(t[1]), float(t[2])))
inp_links = {}
for sec in ('CONDUITS', 'PUMPS', 'ORIFICES', 'WEIRS', 'OUTLETS'):
    for t in S.get(sec, []):
        inp_links[unq(t[0])] = (unq(t[1]), unq(t[2]), sec[:-1])
inp_nodes = {}
for sec, kind in (('JUNCTIONS', 'JUNCTION'), ('OUTFALLS', 'OUTFALL'), ('STORAGE', 'STORAGE'), ('DIVIDERS', 'DIVIDER')):
    for t in S.get(sec, []):
        inp_nodes[unq(t[0])] = kind

# --- 2 geometry ------------------------------------------------------------------
def geom(blob):
    """Coordinates of a GeoPackage geometry blob: header (magic, version, flags, srs, envelope) then WKB."""
    assert blob[:2] == b'GP'
    flags = blob[3]
    env = [0, 32, 48, 48, 64][(flags >> 1) & 7]
    srs = struct.unpack_from('<i' if flags & 1 else '>i', blob, 4)[0]
    w = blob[8 + env:]
    e = '<' if w[0] == 1 else '>'
    t = struct.unpack_from(e + 'I', w, 1)[0]
    if t == 1:
        return srs, [struct.unpack_from(e + '2d', w, 5)]
    if t == 2:
        n = struct.unpack_from(e + 'I', w, 5)[0]
        return srs, [struct.unpack_from(e + '2d', w, 9 + 16 * i) for i in range(n)]
    if t == 3:
        rings, n = struct.unpack_from(e + '2I', w, 5)
        assert rings == 1
        return srs, [struct.unpack_from(e + '2d', w, 13 + 16 * i) for i in range(n)]
    raise ValueError(t)

gcol = {t: geomcols[t][1] for t in geomcols}
rows = {t: {r[0]: r[1] for r in db.execute('select id, "%s" from "%s"' % (gcol[t], t))} for t in gcol}
bad = [i for i, b in rows['nodes'].items() if geom(b)[1] != [coords[i]]]
check(not bad and len(rows['nodes']) == len([n for n in inp_nodes if n in coords]), '%d node points equal the [COORDINATES] numbers exactly' % len(rows['nodes']))
bad = []
for i, b in rows['links'].items():
    a, z, _ = inp_links[i]
    if geom(b)[1] != [coords[a]] + verts.get(i, []) + [coords[z]]:
        bad.append(i)
check(not bad and len(rows['links']) == len([l for l, (a, z, _) in inp_links.items() if a in coords and z in coords]),
      '%d link lines run from-node, [VERTICES], to-node exactly (%d interior vertices)' % (len(rows['links']), sum(len(verts.get(i, [])) for i in rows['links'])))
bad = []
for i, b in rows['subcatchments'].items():
    ring = list(polys[i])
    if ring[0] == ring[-1]:
        ring.pop()
    if geom(b)[1] != ring + [ring[0]]:
        bad.append(i)
check(not bad, '%d subcatchment rings equal the [POLYGONS] numbers, closed' % len(rows['subcatchments']))
check(all(geom(b)[0] == want_srs for t in rows for b in rows[t].values()), 'every geometry blob carries srs_id %d' % want_srs)

# --- 3 attributes against swmm-toolkit ---------------------------------------------
from swmm.toolkit import output, shared_enum as e
h = output.init(); output.open(h, out)
nsub, nnode, nlink, _, npoll = output.get_proj_size(h)
nper = output.get_times(h, e.Time.NUM_PERIODS)
start, step = output.get_start_date(h), output.get_times(h, e.Time.REPORT_STEP)
import datetime
def stamp(k):
    # the toolkit's clock: start date + (k + 1) report steps, in days since 30 Dec 1899
    days = start + (k + 1) * step / 86400.0
    t = datetime.datetime(1899, 12, 30) + datetime.timedelta(seconds=round(days * 86400.0))
    return t.strftime('%Y-%m-%dT%H:%M:%S')
kinds = [('subcatchments', 'subcatchment', nsub, e.ElementType.SUBCATCH, output.get_subcatch_attribute, e.SubcatchAttribute),
         ('nodes', 'node', nnode, e.ElementType.NODE, output.get_node_attribute, e.NodeAttribute),
         ('links', 'link', nlink, e.ElementType.LINK, output.get_link_attribute, e.LinkAttribute)]
names_of = {'subcatchments': ['rainfall', 'snow_depth', 'evaporation_loss', 'infiltration_loss', 'runoff', 'groundwater_flow', 'groundwater_elevation', 'soil_moisture'],
            'nodes': ['depth', 'head', 'volume', 'lateral_inflow', 'total_inflow', 'flooding'],
            'links': ['flow', 'depth', 'velocity', 'volume', 'capacity']}
polls = [output.get_elem_name(h, e.ElementType.POLLUT, i) for i in range(npoll)]
compared = 0
bad = []
for table, kind, n, et, getter, enum in kinds:
    ids = [output.get_elem_name(h, et, i) for i in range(n)]
    base = list(enum)[-1].value
    cols = names_of[table] + ['quality_' + re.sub(r'[^a-z0-9]+', '_', p.lower()).strip('_') for p in polls]
    cur = db.execute('select * from "%s"' % table)
    head = [d[0] for d in cur.description]
    got = {r[head.index('id')]: dict(zip(head, r)) for r in cur}
    for v, col in enumerate(cols):
        grid = np.array([getter(h, p, enum(v) if v <= base else v) for p in range(nper)], dtype=np.float32).reshape(nper, n)
        for i, oid in enumerate(ids):
            if oid not in got:
                continue                      # no geometry in the .inp, not drawn
            s = grid[:, i]
            k = int(np.argmax(s))             # first occurrence of the maximum
            g = got[oid]
            # the stored double must be exactly the saved single-precision value, widened
            if g['max_' + col] != float(s[k]) or g['max_' + col + '_time'] != stamp(k):
                bad.append((table, oid, col, g['max_' + col], float(s[k]), g['max_' + col + '_time'], stamp(k)))
            compared += 1
            if table == 'links' and col == 'flow':
                k2 = int(np.argmin(s))
                if g['min_flow'] != float(s[k2]) or g['min_flow_time'] != stamp(k2):
                    bad.append((table, oid, 'min_flow', g['min_flow'], float(s[k2])))
            if table == 'nodes' and col == 'flooding':
                hours = float(np.count_nonzero(s > 0)) * step / 3600.0
                if abs(g['hours_flooded'] - hours) > 1e-9:
                    bad.append((table, oid, 'hours_flooded', g['hours_flooded'], hours))
check(not bad, '%d peaks (the exact stored value, and the time of peak) equal swmm-toolkit%s' % (compared, '' if not bad else ' -- first: %s' % (bad[0],)))
output.close(h)
check(run.get('periods') == str(nper) and run.get('report_step_seconds') == str(step), 'swmm_run: %s periods of %s s' % (run.get('periods'), run.get('report_step_seconds')))

# properties and types against the .inp
bad = []
for (i, t, a, z) in db.execute('select id, type, from_node, to_node from links'):
    if (a, z, t) != inp_links[i]:
        bad.append(i)
for (i, t) in db.execute('select id, type from nodes'):
    if t != inp_nodes[i]:
        bad.append(i)
check(not bad, 'node types and link types / end nodes equal the .inp sections they sit in (%d nodes, %d links)' % (len(rows['nodes']), len(rows['links'])))
types = dict(db.execute('select type, count(*) from nodes group by type'))
ltypes = dict(db.execute('select type, count(*) from links group by type'))
print('       node types %s, link types %s' % (types, ltypes))

# --- 4 a published run record --------------------------------------------------------
if record:
    R = json.load(open(record))
    n = 0
    bad = []
    type_gap = {}
    for table, key in (('subcatchments', 'subcatchments'), ('nodes', 'nodes'), ('links', 'links')):
        cur = db.execute('select * from "%s"' % table)
        head = [d[0] for d in cur.description]
        got = {r[head.index('id')]: dict(zip(head, r)) for r in cur}
        for oid, peaks in R['peaks'][key].items():
            for k, v in peaks.items():
                if k == 'type':
                    # the record's object types come from its own reader; counted apart
                    if got[oid][k] != v:
                        type_gap[(table, got[oid][k], v)] = type_gap.get((table, got[oid][k], v), 0) + 1
                    continue
                if k in got[oid]:
                    if got[oid][k] != v:
                        bad.append((table, oid, k, got[oid][k], v))
                    n += 1
    check(not bad and n > 0, '%d values equal the peaks in the published run record (%s, .out sha256 %s...)%s' % (
        n, R['engine']['id'], R['files']['out']['sha256'][:12], '' if not bad else ' -- first: %s' % (bad[0],)))
    print('       the record\'s own node_types: %s ; link_types: %s' % (R['objects'].get('node_types'), R['objects'].get('link_types')))
    for (table, here, there), c in sorted(type_gap.items()):
        print('       type differs for %d %s: %s here and in the .inp, %s in the record' % (c, table, here, there))

# --- 5 GDAL ---------------------------------------------------------------------------
try:
    import pyogrio
    layers = {l[0]: l[1] for l in pyogrio.list_layers(gpkg)}
    ok = True
    for t, want in (('nodes', 'Point'), ('links', 'LineString'), ('subcatchments', 'Polygon')):
        info = pyogrio.read_info(gpkg, layer=t)
        crs = info['crs']
        ok &= layers[t] == want and info['features'] == len(rows[t])
        ok &= (crs is None or 'ndefined' in str(crs) or 'nknown' in str(crs)) if not epsg else (str(crs) == 'EPSG:%s' % epsg)
        dt = [str(d) for f, d in zip(info['fields'], info['dtypes']) if f.endswith('_time')]
        ok &= all(d == 'object' for d in dt)          # ISO 8601 text on the model clock, by design
    check(ok, 'GDAL %s opens nodes/links/subcatchments: same counts, geometry types, reference system %s, *_time columns as text' % (
        pyogrio.__gdal_version_string__, crs))
    for t in ('swmm_run', 'swmm_fields'):
        check(t in layers, 'GDAL lists the attribute table %s' % t)
except ImportError:
    print('  --   pyogrio not installed: GDAL check skipped')

# --- 6 the .rpt summary tables -----------------------------------------------------------
if rpt:
    text = open(rpt, encoding='latin-1').read().split('\n')
    def table(title):
        """Rows (token lists) of a summary table, by its title line."""
        hits = [k for k, l in enumerate(text) if l.strip() == title]
        if not hits:
            return []
        j = hits[0] + 2
        dashes = 0
        while dashes < 2:
            if j >= len(text) or re.match(r'^\s*\*{6,}\s*$', text[j]):
                return []                    # the next table's title: this one has no rows ("No nodes were flooded.")
            dashes += bool(re.match(r'^\s*-{10,}\s*$', text[j]))
            j += 1
        rows_ = []
        while j < len(text) and text[j].strip():
            rows_.append(text[j].split())
            j += 1
        return rows_
    def near(txt, val):
        """Does a number printed with n decimals equal val rounded to n decimals? (half a last digit, plus float noise)"""
        dec = len(txt.split('.')[1]) if '.' in txt else 0
        return abs(float(txt) - val) <= 0.5 * 10 ** -dec + 1e-9 * max(1.0, abs(val))
    o = {t[0].upper(): t[1:] for t in S.get('OPTIONS', [])}
    # the report counts "Time of Max Occurrence" from the report start (swmm5.c getElapsedTime)
    md = [int(x) for x in o.get('REPORT_START_DATE', o['START_DATE'])[0].split('/')]
    hms = [int(x) for x in (o.get('REPORT_START_TIME', o.get('START_TIME', ['0:0:0']))[0].split(':') + ['0', '0'])[:3]]
    t0 = datetime.datetime(md[2], md[0], md[1], *hms)
    def elapsed_ok(iso_time, days, hhmm):
        if iso_time is None:                 # the engine never set the date: the report prints day 0, 00:00
            return (days, hhmm) == ('0', '00:00')
        d = (datetime.datetime.strptime(iso_time, '%Y-%m-%dT%H:%M:%S') - t0).total_seconds()
        if d <= 0:
            return (days, hhmm) == ('0', '00:00')
        want = int(days) * 86400 + int(hhmm[:2]) * 3600 + int(hhmm[3:]) * 60
        return -1 <= d - want < 61           # the report drops the seconds

    cur = db.execute('select * from nodes'); head = [d[0] for d in cur.description]
    N = {r[head.index('id')]: dict(zip(head, r)) for r in cur}
    cur = db.execute('select * from links'); head = [d[0] for d in cur.description]
    L = {r[head.index('id')]: dict(zip(head, r)) for r in cur}
    have_rpt = any(v['rpt_max_depth'] is not None for v in N.values())

    depth = table('Node Depth Summary')
    bad = [r[0] for r in depth if r[0] in N and N[r[0]]['max_depth'] is not None and not near(r[7], N[r[0]]['max_depth'])]
    check(not bad, 'saved-period max_depth equals the .rpt "Reported Max Depth" column for %d nodes%s' % (len(depth), '' if not bad else ' -- %s' % bad[:5]))
    if have_rpt:
        n = 0; bad = []
        for r in depth:
            g = N.get(r[0])
            if g is None:
                continue
            ok = near(r[3], g['rpt_max_depth']) and near(r[4], g['rpt_max_hgl']) and elapsed_ok(g['rpt_max_depth_time'], r[5], r[6])
            n += 3
            if not ok:
                bad.append((r[0], r[3:7], g['rpt_max_depth'], g['rpt_max_hgl'], g['rpt_max_depth_time']))
        for r in table('Node Inflow Summary'):
            g = N.get(r[0])
            if g is None:
                continue
            n += 2
            if not (near(r[2], g['rpt_max_lateral_inflow']) and near(r[3], g['rpt_max_total_inflow'])):
                bad.append((r[0], r[2:4], g['rpt_max_lateral_inflow'], g['rpt_max_total_inflow']))
        flooded = table('Node Flooding Summary')
        listed = set()
        for r in flooded:
            g = N.get(r[0])
            if g is None:
                continue
            listed.add(r[0])
            n += 2
            if not (near(r[1], max(0.01, g['rpt_hours_flooded'])) and near(r[2], g['rpt_max_flooding'])):
                bad.append((r[0], r[1:3], g['rpt_hours_flooded'], g['rpt_max_flooding']))
        # a node the report does not list as flooded must have no flooded time (outfalls are never listed)
        unlisted = [i for i, g in N.items() if i not in listed and g['rpt_hours_flooded'] > 0 and g['type'] != 'OUTFALL']
        check(not bad and not unlisted, 'rpt_* node columns equal the Node Depth, Inflow and Flooding Summary tables: %d printed numbers (%d nodes flooded)%s' % (
            n, len(listed), '' if not (bad or unlisted) else ' -- %s %s' % (bad[:2], unlisted[:5])))
        n = 0; bad = []
        for r in table('Link Flow Summary'):
            g = L.get(r[0])
            if g is None:
                continue
            ok = near(r[2], g['rpt_max_flow']) and elapsed_ok(g['rpt_max_flow_time'], r[3], r[4])
            n += 2
            extra = r[5:]
            if r[1] in ('CONDUIT', 'CHANNEL') and len(extra) == 3:
                ok &= (extra[0] == '>50.00' and g['rpt_max_velocity'] > 50.0) or (extra[0] != '>50.00' and near(extra[0], g['rpt_max_velocity']))
                ok &= near(extra[2], g['rpt_max_depth_ratio'])
                n += 2
            elif r[1] in ('PUMP', 'DUMMY', 'OUTLET'):
                ok &= g['rpt_max_velocity'] is None and g['rpt_max_depth_ratio'] is None
            elif len(extra) == 1:              # orifice or weir with a full depth
                ok &= g['rpt_max_velocity'] is None and near(extra[0], g['rpt_max_depth_ratio'])
                n += 1
            elif len(extra) == 0:              # bottom orifice, or no full depth
                ok &= g['rpt_max_velocity'] is None and g['rpt_max_depth_ratio'] is None
            else:
                ok = False
            if not ok:
                bad.append((r, g['rpt_max_flow'], g['rpt_max_flow_time'], g['rpt_max_velocity'], g['rpt_max_depth_ratio']))
        check(not bad, 'rpt_* link columns equal the Link Flow Summary table: %d printed numbers%s' % (n, '' if not bad else ' -- %s' % bad[:2]))
        gap = [(i, g['max_depth'], g['rpt_max_depth']) for i, g in N.items() if g['max_depth'] is not None and g['rpt_max_depth'] - g['max_depth'] > 0.005]
        worst = max(gap, key=lambda t: t[2] - t[1]) if gap else None
        print('       %d of %d nodes peak higher over every routing step than in the saved periods%s' % (
            len(gap), len(N), '' if not worst else '; largest gap %s: saved max_depth %.3f, rpt_max_depth %.3f' % worst))
    else:
        print('       rpt_* columns are empty in this file (it came from `export`)')

print('RESULT:', 'FAILED %d check(s)' % len(fail) if fail else 'all checks passed')
sys.exit(1 if fail else 0)
