#!/usr/bin/env python3
"""Open a GeoPackage written by swmm5-gpkg in RustGIS itself (headless, in-process: `pip install rustgis`),
list what RustGIS sees, style the network by its results and render a PNG.

usage: open_in_rustgis.py FILE.gpkg OUT.png [--links FIELD] [--nodes FIELD]
"""
import sys, rustgis

args = sys.argv[1:]
def opt(name, default):
    if name in args:
        i = args.index(name); v = args[i + 1]; del args[i:i + 2]; return v
    return default
link_field, node_field = opt('--links', 'rpt_max_flow'), opt('--nodes', 'rpt_max_depth')
gpkg, png = args

s = rustgis.Session()
s.add_data(gpkg)
for text, _ in s.notices():
    print('notice:', text)
print('RustGIS', rustgis.__version__, 'sees:')
for l in s:
    print('  %-14s %-9s %4d features, %2d fields' % (l.name, l.type, l.count, len(l.fields)))
by = {l.name: l for l in s}
subs, links, nodes = by['subcatchments'], by['links'], by['nodes']
s.run('map.basemap', name='blank')
s.run('layer.move', layer='subcatchments', to=99)            # under the network
s.run('layer.color', layer=subs.id, color='#DCE6F2')
s.run('layer.outline', layer=subs.id, color='#8FA6C4', width=0.6)
links.style('graduated', field=link_field, classes=5, method='quantile')
nodes.style('proportional', field=node_field)
s.zoom_to(links)
s.to_png(png, width=1500, height=900)
print('styled links by %s, nodes by %s; wrote %s' % (link_field, node_field, png))
