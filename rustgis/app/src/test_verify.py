# Planted defects: the checker must catch each one. A checker that passes everything passes this
# only by accident, so a pass here is evidence the field-by-field mapping is doing work.
import json, copy, sys
sys.path.insert(0, '/root/work/rgtest'); import verify_attrs as V
rpt = open('/root/work/runs/jason_dorn_storage_example.epawasm.rpt', encoding='latin-1').read()
E = V.expected(rpt)
nodes = json.load(open('/root/work/rgtest/out/jason_dorn_storage_example.nodes.geojson'))['features']
links = json.load(open('/root/work/rgtest/out/jason_dorn_storage_example.links.geojson'))['features']
subs = json.load(open('/root/work/rgtest/out/jason_dorn_storage_example.subcatchments.geojson'))['features']
base = [V.check_features(E['node'], nodes), V.check_features(E['link'], links), V.check_features(E['subcatch'], subs)]
print('clean:', [(b[0], b[1], b[2]) for b in base])
assert all(b[1] == 0 and b[2] == 0 for b in base)
def caught(kind, feats, plant, what):
    f = copy.deepcopy(feats); plant(f)
    ok, bad, miss, bl = V.check_features(E[kind], f)
    print(f'{"caught" if bad + miss else "MISSED"}: {what}  -> {bl[:1]}')
    return bad + miss > 0
def swap_avg_max(f):
    x = next(n for n in f if n['properties'].get('avg_depth_ft') != n['properties'].get('max_depth_ft'))
    p = x['properties']; p['max_depth_ft'] = p['avg_depth_ft']
def wrong_type(f): f[0]['properties']['rpt_type'] = 'WEIR' if f[0]['properties']['rpt_type'] != 'WEIR' else 'CONDUIT'
def wrong_unit(f):
    p = f[0]['properties']; p['max_depth_m'] = p.pop('max_depth_ft')
def nudged(f):
    p = f[0]['properties']; p['peak_runoff_cfs'] = p['peak_runoff_cfs'] * 1.001 + 0.001
def dropped(f): f[0]['properties'].pop('max_hgl_ft')
def time_shift(f): f[0]['properties']['t_max_depth_hr'] += 1 / 60
r = [caught('node', nodes, swap_avg_max, 'maximum depth replaced by average depth'),
     caught('link', links, wrong_type, 'link report type changed'),
     caught('node', nodes, wrong_unit, 'max_depth_ft relabeled max_depth_m'),
     caught('subcatch', subs, nudged, 'peak runoff off by 0.1%'),
     caught('node', nodes, dropped, 'max_hgl_ft left out'),
     caught('node', nodes, time_shift, 'time of maximum depth one minute late')]
print(f'{sum(r)} of {len(r)} planted defects caught'); assert all(r)
# the old checker, for the record
sys.path.insert(0, '/root/work/rgtest'); import verify_attrs_v1 as V1
import os, tempfile
f = copy.deepcopy({'type': 'FeatureCollection', 'features': nodes}); swap_avg_max(f['features'])
p = tempfile.mktemp(suffix='.geojson'); json.dump(f, open(p, 'w'))
ok, bad, bl, n = V1.check('/root/work/runs/jason_dorn_storage_example.epawasm.rpt', p, 'node')
print('old checker on the swapped depth:', ok, 'agree,', bad, 'disagree')
