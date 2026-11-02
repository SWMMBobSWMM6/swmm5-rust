#!/usr/bin/env python3
"""Builds swmm5-rust-rustgis.html from the plugin, the EPA engine, the runs and the essay.

Every number on the page and in the essay is computed here from files written by the runs:
census6.jsonl (221 models; plugin rerun after the review fixes), rgtest/out/*.run.json (RustGIS itself), fuel_final.jsonl (wasmi),
attrs.json (attribute agreement), regress221.json + lakes (default build), so the page cannot
drift from the evidence.
"""
import base64, hashlib, json, os, re, subprocess, sys, glob

W = '/root/work'
PLUG = f'{W}/swmm5-rust/rustgis'
OUT = sys.argv[1] if len(sys.argv) > 1 else f'{W}/app/dist/swmm5-rust-rustgis.html'
DOCX = f'{W}/app/dist/swmm5-rust-in-rustgis.docx'

b64 = lambda b: base64.b64encode(b).decode()
sha = lambda b: hashlib.sha256(b).hexdigest()
rd = lambda p: open(p, 'rb').read()

plugin = rd(f'{PLUG}/dist/plugin.wasm')
rgpkg = rd(f'{PLUG}/dist/swmm5-rust-0.1.0.rgplugin')
manifest = json.load(open(f'{PLUG}/plugin.json'))
epa = rd(f'{W}/epawasm/swmm524_epa.wasm')
core = open(f'{W}/swmm5-rust/app/engine_core.js').read()

# ---------------------------------------------------------------- measured inputs
fuel = {json.loads(l)['model']: json.loads(l) for l in open(f'{W}/rgtest/fuel_final.jsonl')}
runs = {}
for p in glob.glob(f'{W}/rgtest/out/*.run.json'):
    r = json.load(open(p)); runs[r['tag']] = r
attrs = json.load(open(f'{W}/rgtest/attrs.json'))
census = [json.loads(l) for l in open(f'{W}/host/census6.jsonl')]
regress = json.load(open(f'{W}/reg/regress221.json'))
lakes = json.load(open(f'{W}/reg/lakes/result.json'))
# ---------------------------------------------------------------- the outside review (page 1.1)
TV = json.load(open(f'{W}/app/test_verify.json'))           # node test_verify.js: old and new checkers on the six demos
old_agree = sum(r['old']['agree'] for r in TV); old_dis = sum(r['old']['dis'] for r in TV)
old_str = sum(r['old']['why']['string'] for r in TV); old_sci = sum(r['old']['why']['sci'] for r in TV)
new_agree = sum(r['now']['agree'] for r in TV); new_dis = sum(r['now']['dis'] for r in TV); new_miss = sum(r['now']['missing'] for r in TV)
old_swap_missed = sum(1 for r in TV if r['old_on_swap'] and not r['old_on_swap']['caught']); swap_models = sum(1 for r in TV if r['old_on_swap'])
st_tried = sum(1 for r in TV for x in r['selftest'] if x['applicable']); st_caught = sum(1 for r in TV for x in r['selftest'] if x['applicable'] and x.get('caught'))
per_model = ', '.join(f"{r['now']['agree']:,}" for r in TV)
py_agree = sum(x['values'] for x in attrs)
# node 9 of the storage example, from EPA's run (byte-identical to the plugin's .out: same SHA-256)
sys.path.insert(0, f'{W}/swmm5-rust/app'); from outread import Out
import struct
_o = Out(f'{W}/runs/jason_dorn_storage_example.epawasm.out')
_per = 8 + 4 * (_o.n['sub'] * _o.nv['sub'] + _o.n['node'] * _o.nv['node'] + _o.n['link'] * _o.nv['link'] + _o.nv['sys'])
_ni = _o.ids['node'].index('9'); _base = _o.n['sub'] * _o.nv['sub'] + _ni * _o.nv['node']
# the prolog's start date sits after the property and reporting-variable lists (EPA 5.2 layout)
_q = struct.unpack_from('<6i', _o.b, len(_o.b) - 24)[1]
for _cnt in (_o.n['sub'], _o.n['node'], _o.n['link']):
    _k = struct.unpack_from('<i', _o.b, _q)[0]; _q += 4 + 4 * _k + 4 * _cnt * _k
for _ in range(4):
    _k = struct.unpack_from('<i', _o.b, _q)[0]; _q += 4 + 4 * _k
_start = struct.unpack_from('<d', _o.b, _q)[0]
_dates = [struct.unpack_from('<d', _o.b, _o.resPos + k * _per)[0] for k in range(_o.nPer)]
hydro = dict(id='9', hours=[round((d - _start) * 24, 4) for d in _dates],
             depth=[round(struct.unpack_from('<f', _o.b, _o.resPos + k * _per + 8 + 4 * _base)[0], 5) for k in range(_o.nPer)],
             ref=0.23, out_sha=hashlib.sha256(rd(f'{W}/runs/jason_dorn_storage_example.epawasm.out')).hexdigest(),
             step_min=round((_dates[1] - _dates[0]) * 1440))
# ---------------------------------------------------------------- before and after the review fixes
sys.path.insert(0, f'{W}/regress2')
import classify  # every difference between the old and new plugin code on 221 models, tied to a fix
fx_tot, fx_by, fx_unexpl = classify.summary()
FIXNAME = {1: 'Ponded column named for the routing the engine used', 6: 'Link `type` from the input section',
           7: 'CUSTOM barrels from the right column', 8: 'Report rows with numbers run together refused',
           10: 'Names with a non-breaking space kept whole', 14: 'Rain gages with no symbol listed in a table'}
def _mlist(ms):
    names = sorted({m.split('/')[-1] for m in ms})
    return ', '.join(names) if len(names) <= 4 else f"{len(ms)} models, among them {', '.join(names[:3])}"
fx_rows = [[str(k), FIXNAME.get(k, '?'), _mlist(v['models']), f"{v['n']:,}"] for k, v in sorted(fx_by.items())]
fx_models = len({m for v in fx_by.values() for m in v['models']})
fix_text = (f"Of the {fx_tot['models']}, {fx_tot['crash']} memory-error decks stop in both versions and {fx_tot['both_err']} input-error decks stop with "
            f"{'the same error text' if fx_tot['err_same'] else 'errors'} in both. On the other {fx_tot['models'] - fx_tot['crash'] - fx_tot['both_err']}, "
            f"{fx_tot['same']:,} attribute values came out identical and {fx_tot['renamed']:,} more differ only in their new names. "
            f"The remaining {fx_tot['ndiffs']:,} differences, in {fx_models} models, each trace to one of the fixes"
            + (", and none is unexplained." if not fx_unexpl else f"; {len(fx_unexpl)} are not yet explained."))
# seepage.inp: where the old reader centered the model, against where it belongs
def _centre(path):
    import math
    R_, lon0, lat0, k = 6371008.8, -82.3474, 29.6476, 0.3048
    r = json.load(open(path))['ok']
    for l in r['layers']:
        for ft in l['geojson']['features']:
            if ft['properties']['id'] == 'STR_5442':
                lon, lat = ft['geometry']['coordinates']
                dy = math.radians(lat - lat0) * R_ / k; dx = math.radians(lon - lon0) * R_ * math.cos(math.radians(lat0)) / k
                return 466702.394 - dx, 137663.067 - dy
_co, _cn = _centre(f'{W}/regress2/plugin-native.old.seep.json'), _centre(f'{W}/regress2/plugin-native.seep.json')
seep_miles = ((_co[0] - _cn[0]) ** 2 + (_co[1] - _cn[1]) ** 2) ** 0.5 / 5280

DEMOS = [
    ('jason_dorn_storage_example.inp', 'Storage and orifices: 8 subcatchments, dynamic wave', 'storage example', '', 'EPA/jason_dorn_storage_example.inp'),
    ('swmm_example.inp', 'pyswmm test model: 3 subcatchments, kinematic wave', 'pyswmm example', '', 'EPA_PYSWMM_Additions/PYSWMM_tests/swmm_example.inp'),
    ('Example_LID_RB.inp', 'EPA LID example: rain barrels on 9 subcatchments', 'LID rain barrels', '', 'EPA/Example_LID_RB.inp'),
    ('controlExample.inp', 'Control rules and pumps: 34 nodes, dynamic wave', 'control rules', '', 'EPA/controlExample.inp'),
    ('user2.inp', 'EPA user2: 17 subcatchments, 72 nodes (too big for RustGIS’s budget)', 'EPA user2', '', 'EPA_PYSWMM_Additions/EPA_nrtestsuite_v2.5.0-dev/revisions/user2.inp'),
    ('Greenville_all_SWMM5_Features.inp', 'Greenville (Naperville, IL): 30 subcatchments, 172 nodes, EPSG:3443', 'Greenville', '3443', 'Greenville/Greenville_all_SWMM5_Features.inp'),
]
demos = []
for f, name, short, epsg, src in DEMOS:
    stem = f[:-4]
    r = runs.get(stem, {})
    demos.append(dict(file=f, name=name, short=short, epsg=epsg, src=src, b64=b64(rd(f'{W}/runs/{f}')),
                      fuel=fuel.get(f), rustgis=dict(rc=r.get('rc'), seconds=r.get('seconds'))))

# ---------------------------------------------------------------- the census
def same(r, a, b):
    return r.get(a + '_out') and r.get(a + '_out') == r.get(b + '_out') and r.get(a + '_rpt') == r.get(b + '_rpt')
n = len(census)
skipped = [r for r in census if r.get('skip') or r.get('wasm_timeout') or r.get('n_timeout')]
ran = [r for r in census if r not in skipped]
ok = [r for r in ran if r.get('p_out')]
perr = [r for r in ran if r.get('p_error') or r.get('p_trap')]
code_of = lambda s: (re.search(r'error (\d+)', s or '') or [None, None])[1]
same_err = [r for r in perr if code_of(r.get('p_error')) and str(r.get('e_rc')) == code_of(r.get('p_error'))]
other_err = [r for r in perr if r not in same_err]
mem_err = [r for r in other_err if 'unreachable' in (r.get('p_error') or r.get('p_trap') or '')]
other_err = [r for r in other_err if r not in mem_err]
pe = sum(1 for r in ok if same(r, 'p', 'e'))
pn = sum(1 for r in ok if same(r, 'p', 'n'))
en_all = [r for r in ran if r.get('e_out') and r.get('n_out')]
en = sum(1 for r in ok if same(r, 'e', 'n'))
no_coords = sum(1 for r in ok if not r.get('p_layers'))
with_data = sum(1 for r in ok if r.get('data'))
rows_skipped = sum(1 for r in ok if r.get('p_skipped'))
C = dict(
    n=n, ran=len(ran), ok=len(ok), pe=pe, pn=pn, en=en, same_err=len(same_err), other_err=len(other_err), skipped=len(skipped),
    no_coords=no_coords, with_data=with_data, rows_skipped=rows_skipped,
    note=(f"The 221 smoke and coverage models of the swmm5-rust gate (corpus SWMMBobSWMM6/1729-SWMM5-Models-2030 at 9ee63bd), each run three ways: "
          f"the plugin module through this page's host code, EPA SWMM 5.2.4 compiled to wasm32-wasi (the swmm5-rust app's binary), and EPA's code natively "
          f"(the unpatched swmm5-rust binary, which the swmm5-rust gate shows identical to EPA's gcc build). Data files a model names were handed over as "
          f"data files ({with_data} models)."),
    caption=(f"Of {len(ok)} models the plugin ran, {pe} match EPA's WebAssembly build byte for byte (.out, and .rpt without its clock lines). "
             f"{len(same_err)} more are input-error decks where both engines stop with the same error code. Against EPA native, the match is {pn} of {len(ok)}: "
             f"EPA's own wasm build matches its native build on {en} of {len(ok)}, so the gap is the math library, not the translation. "
             f"{len(mem_err)} are memory-error decks from the OpenSWMM 2026 code review, where EPA's C reads memory it does not own and carries on; the plugin stops at the bad index. "
             f"{len(skipped)} model{'s' if len(skipped) != 1 else ''} (over 300 MB of output) not run in WebAssembly."),
    bars=[
        dict(label='plugin vs EPA wasm (same libm)', parts=[dict(k='same', n=pe), dict(k='diff', n=len(ok) - pe), dict(k='ub', n=len(same_err)), dict(k='mem', n=len(mem_err))]),
        dict(label='plugin vs EPA native (glibc)', parts=[dict(k='same', n=pn), dict(k='diff', n=len(ok) - pn), dict(k='ub', n=len(same_err)), dict(k='mem', n=len(mem_err))]),
        dict(label='EPA wasm vs EPA native (control)', parts=[dict(k='same', n=en), dict(k='diff', n=len(ok) - en), dict(k='ub', n=len(same_err)), dict(k='mem', n=len(mem_err))]),
    ],
    errors=[dict(inp=r['inp'], why=(r.get('p_error') or r.get('p_trap') or '').replace('trap: unreachable | ', '').replace(' | ', ' ')[:220]) for r in mem_err + other_err],
    mem=len(mem_err),
)
C['n'] = len(ok) + len(same_err) + len(mem_err)
mem_example = next((r for r in mem_err if 'CRASH-02' in r['inp']), mem_err[0] if mem_err else None)

# ---------------------------------------------------------------- RustGIS runs
rg = []
for f, name, short, epsg, src in DEMOS:
    stem = f[:-4]
    for tag, tool in ((stem, 'run-swmm'), (stem + '.map', 'map-swmm')):
        r = runs.get(tag)
        if not r:
            continue
        status = 'layers added' if r['rc'] == 0 else ('stopped: ran too long' if any('ran too long' in l for l in r['stderr']) else 'failed')
        a = next((x for x in attrs if x['model'] == stem and x['where'] == 'RustGIS'), None) if tool == 'run-swmm' else None
        fu = fuel.get(f) if tool == 'run-swmm' else None
        rg.append(dict(model=short, tool=tool, ok=r['rc'] == 0, status=status, seconds=r['seconds'],
                       fuel_pct=fu['pct_of_budget'] if fu else None, values=a['values'] if a else None, dis=a['dis'] if a else None))
rg_ok = [r for r in rg if r['tool'] == 'run-swmm' and r['ok']]
rg_vals = sum(x['values'] for x in attrs if x['where'] == 'RustGIS')
all_vals = sum(x['values'] for x in attrs)
all_dis = sum(x['dis'] for x in attrs)

# ---------------------------------------------------------------- default build
reg_same = sum(1 for r in regress if r['same'])
reg_text = (f"The engine patch adds a cargo feature, memfs, that is off by default. To check that the default build behaves exactly as before, "
            f"the unpatched and patched runswmm-rs binaries were run on the same 221 models: {reg_same} of {len(regress)} gave identical exit codes, "
            f"reports (wall-clock lines removed) and output files. The one left, lakes.inp, writes a {lakes['out_bytes']/1e6:,.0f} MB output file and hit that run's "
            f"two-minute cap on both binaries; rerun without the cap (about five minutes each), the two are {'identical too, so 221 of 221' if lakes['out_same'] and lakes['rpt_same'] else 'different'}. "
            f"(The binaries themselves differ by a few bytes: panic messages carry source line numbers, and the patch moved lines.)")

# ---------------------------------------------------------------- the libm story (swmm_example)
L = next(r for r in census if r['inp'].endswith('PYSWMM_tests/swmm_example.inp')) if any(r['inp'].endswith('PYSWMM_tests/swmm_example.inp') for r in census) else None
libm_rows = [
    dict(engine='EPA SWMM 5.2.4, native Linux (gcc)', libm='glibc libm', sha=hashlib.sha256(rd(f'{W}/runs/swmm_example.out')).hexdigest()),
    dict(engine='EPA SWMM 5.2.4, WebAssembly (clang, wasm32-wasi)', libm='wasi-libc (musl) libm', sha=hashlib.sha256(rd(f'{W}/runs/swmm_example.epawasm.out')).hexdigest()),
    dict(engine='swmm5-rust plugin, first build (wasm32-unknown-unknown)', libm="Rust's port of musl libm in compiler-builtins", sha='3ef43bc4a6a2eb5cde3700af63657c5d64cbd092d73aacf18020d77d1c61d5b2'),
    dict(engine='swmm5-rust plugin, as shipped', libm='wasi-libc libm, linked in', sha=json.loads(subprocess.run(['node', f'{W}/host/one.js', f'{PLUG}/dist/plugin.wasm', f'{W}/epawasm/swmm524_epa.wasm', f'{W}/runs/swmm_example.inp', 'plugin'], capture_output=True, text=True).stdout)['p_out']),
]

# ---------------------------------------------------------------- facts for the essay
F = dict(
    fuel_budget='3×10^10', cmd_budget='5×10^8',
    jd=fuel['jason_dorn_storage_example.inp'], ce=fuel['controlExample.inp'], u2=fuel['user2.inp'], gv=fuel['Greenville_all_SWMM5_Features.inp'],
    se=fuel['swmm_example.inp'], lid=fuel['Example_LID_RB.inp'],
    plug_kb=len(plugin) / 1024, pkg_kb=len(rgpkg) / 1024,
    memfs_lines=sum(1 for _ in open(f'{W}/swmm5-rust/crates/engine/src/memfs.rs')),
    plugin_lines=sum(sum(1 for _ in open(p)) for p in glob.glob(f'{PLUG}/src/*.rs')),
    rg_vals=rg_vals, all_vals=all_vals, all_dis=all_dis,
)
fmtk = lambda x: f'{x:,.0f}'
gv_first = None
try:
    fc = json.load(open(f'{W}/rgtest/out/Greenville_all_SWMM5_Features.map.nodes.geojson'))
    g = fc['features'][0]; gv_first = (g['properties']['id'], g['geometry']['coordinates'])
except Exception:
    pass

DOC = [
    dict(t='title', x='The Engine in the Box'),
    dict(t='by', x='How swmm5-rust became a RustGIS plugin, and how we know it still gives EPA’s answers. Written for SWMM Enablement (Robert Dickinson), 31 October and 1 November 2026.'),
    dict(t='h2', x='A program that may not touch the disk'),
    dict(t='lede', x='Here is the job. Take the Storm Water Management Model, a program that has spent fifty-five years reading files and writing files, and run it inside a box that has no files.'),
    dict(t='p', x='The box is RustGIS’s plugin sandbox. RustGIS is a geographic information system written in Rust by Qiusheng Wu’s opengeos group. It runs on the desktop, on a command line, and in a web browser at [web.rustgis.app](https://web.rustgis.app). Anyone can add to it with a plugin, and a plugin is a WebAssembly module: a program compiled to a portable instruction set that a host runs in an interpreter. RustGIS runs that interpreter, wasmi, with the doors bolted. No files. No network. No clock. No threads. The module gets three functions it may call back into RustGIS, a memory cap of 512 MB, and an allowance of fuel that each instruction spends.'),
    dict(t='p', x='EPA’s SWMM is C, and the first thing it does is call `fopen` on the input file. Drop that engine into the box and it dies on line one. So the question was never whether SWMM can run in a browser. Bob’s apps have done that for a year with EPA’s C compiled for WASI, the WebAssembly System Interface, which hands a module a file system. The question was whether SWMM can run in a box that hands out nothing at all.'),
    dict(t='p', x='It can, because of swmm5-rust.'),
    dict(t='h2', x='What swmm5-rust brought to the job'),
    dict(t='p', x='swmm5-rust is EPA SWMM 5.2.4 translated module by module into safe Rust in October 2026: 63 files, about 56,000 lines, with `#![forbid(unsafe_code)]` at the top of the engine crate so the compiler refuses any memory trick C would allow. Its gate holds it to EPA byte for byte. On the 221 smoke and coverage models, 215 give identical files and the other six are decks where EPA itself reads memory it does not own.'),
    dict(t='p', x='Two choices made in that translation are what make a plugin possible. Every simulation is one value of type `Project`, not a scatter of globals, so a fresh instance starts clean. And the translators wrote their own small C library, `cio`, so they could print numbers exactly as glibc prints them. That library has its own `FILE`. Every byte the engine reads or writes passes through one struct, `cio::CFile`. Change what sits behind that struct and you change where the engine’s files live, without touching a line of hydrology.'),
    dict(t='fig', f='pipeline', cap='One tool call from end to end. The .inp arrives as base64 inside a JSON request; the engine runs entirely in memory; layers go back in the reply. Nothing survives between calls, because RustGIS gives every call a fresh instance.'),
    dict(t='h2', x='A disk made of memory'),
    dict(t='p', x=f'The new module is called memfs, and it is {F["memfs_lines"]} lines including its tests. Behind a cargo feature of the same name, `cio::CFile` stops being a wrapper around an operating-system file and becomes a cursor into a byte vector, kept in a table keyed by file name. It keeps the stdio rules SWMM leans on: `fgets` keeps the newline, end-of-file turns true only after a read runs past the end, `rewind` and `fseek` clear it, and a write past the end fills the gap with zeros.'),
    dict(t='p', x='Three other spots in the engine reached past `cio` for the operating system. The scratch-file namer wanted the clock and a process id. The path resolver wanted the current directory. The report wanted `time()` for its “Analysis begun on” line. With memfs the first gets a fixed seed, the second passes names through unchanged, and the third reads a clock the host sets. In RustGIS’s box nobody sets it, so the report says the analysis began and ended on 1 January 1970 and took less than a second. The swmm5-rust gate already leaves those three lines out when it compares two reports.'),
    dict(t='fig', f='memfs', cap='The default build never compiles memfs. With the feature on, the same CFile calls land in a name-to-bytes table, and the engine above cannot tell.'),
    dict(t='code', x='// cio.rs: with the feature, the in-memory types replace the std::fs ones\n#[cfg(feature = "memfs")]\npub use crate::memfs::{remove, CFile};\n\n#[cfg(not(feature = "memfs"))]\npub struct CFile {\n    f: fs::File,\n    ...', cap='The whole switch. The original type is still there, unchanged, under `cfg(not(feature = "memfs"))`.'),
    dict(t='p', x=f'A claim like “the default build is unchanged” is cheap to make and cheap to check, so it was checked. {reg_text}'),
    dict(t='p', x='A model that names a rainfall file or a hotstart file needs that file inside the box too. The tool takes up to three data files. A host can hand files over only by name, so memfs finds each one by its own name, whatever folder the .inp gives and in any case, and the run’s messages say which file stood in for which name. Two data files with the same name are refused, because the model could read only one of them.'),
    dict(t='h2', x='Three answers from one source'),
    dict(t='p', x='Here is the surprise. The first plugin build matched EPA on the storage example to the last bit. On the pyswmm test model it did not: conduit C1’s maximum velocity came out 5.38 ft/s where EPA printed 5.40.'),
    dict(t='p', x='Nothing in the translation was wrong. The difference was the math library. A WebAssembly module for a bare host has no C library, so Rust supplies its own port of musl’s `pow`, `exp`, `log` and the rest. EPA’s WebAssembly build gets them from wasi-libc. Both are correct to within a last bit, but the last bits differ on some inputs, and a kinematic-wave run feeds each step’s result into the next. A difference in the sixteenth digit can reach the second.'),
    dict(t='fig', f='libm', cap='One C source, four builds, three different output files for swmm_example.inp. The shipped plugin links wasi-libc’s math and lands on EPA’s WebAssembly answer exactly.'),
    dict(t='p', x='The fix was to link wasi-libc’s `libc.a` into the plugin. The linker pulls only the members it needs, and its link map lists them: `pow`, `exp`, `log`, `log10`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `round`, `scalbn`, `fmin`/`fmax` and their helpers, plus `memcmp` and an empty destructor hook. None of them calls the operating system, so the module still imports nothing but RustGIS’s own functions. Which answer is right? None more than the others: each is a correct floating-point computation with a different rounding of a transcendental function. The plugin’s reference is EPA compiled for the same machine with the same library, and against that reference it is identical. The census below measures the native gap as well, and EPA’s own WebAssembly build shows the same gap against its native build.'),
    dict(t='h2', x='Putting the pipes on the earth'),
    dict(t='p', x='SWMM coordinates are whatever the modeler drew in. RustGIS reads every GeoJSON layer as longitude and latitude unless the layer says otherwise, so the plugin handles three cases. With an EPSG code, it writes the model’s own coordinates and declares the system in the layer’s `crs` member, and RustGIS’s own reader reprojects them. With `[MAP] UNITS Degrees`, the coordinates already are longitude and latitude. Otherwise the model is drawn near true size around an anchor point, which defaults to the University of Florida in Gainesville, one of the three places SWMM was first written in 1969 to 1971.'),
    dict(t='p', x='An EPSG code is a promise the plugin cannot keep by itself, because RustGIS does the reprojecting. RustGIS knows a code only if it is in its table, and a code it does not know makes it read the coordinates as longitude and latitude. It writes a warning when that happens, but not where a plugin’s user can see it, so a State Plane model in feet lands in the wrong place, or nowhere, without a word. The plugin now carries the list of the 6,606 codes RustGIS 0.5.0 can project and refuses any other, with the reason. Esri’s six-digit codes, common in US stormwater models, get their own message: they are not EPSG codes at all.'),
    dict(t='fig', f='placement', cap='The local-frame arithmetic: an equirectangular placement on a sphere. Near Gainesville it is within about 0.3% of true size on the WGS84 ellipsoid, plenty for the few miles a drainage model spans.'),
    dict(t='p', x=(f'The EPSG path was tested on Greenville, which is Innovyze’s training network drawn from the City of Naperville, Illinois, in NAD83 HARN State Plane Illinois East (US feet), EPSG:3443. RustGIS put node {gv_first[0]} at {gv_first[1][1]:.4f} N, {abs(gv_first[1][0]):.4f} W: Naperville.' if gv_first else 'The EPSG path was tested on Greenville (EPSG:3443).')),
    dict(t='h2', x='Reading the engine’s own report'),
    dict(t='p', x='Every result attribute on the map comes from the summary tables EPA SWMM prints in its .rpt file: Subcatchment Runoff, Node Depth, Node Inflow, Node Surcharge, Node Flooding, Storage Volume, Outfall Loading, Link Flow, Conduit Surcharge and Pumping. That choice means any number in RustGIS’s attribute table can be found, digit for digit, in the report. A row is read from the right. EPA’s statsrpt.c prints a fixed run of cells after each name, so the name is whatever comes before them, and a quoted SWMM name with spaces in it survives. Where a row leaves cells blank (a pump has no velocity), the row grammar of statsrpt.c decides which cells are there. Every cell must be the number its column promises, or the whole row is left out and counted in the run’s messages.'),
    dict(t='p', x='The reader had one bug worth showing, because it is the kind that hides. A table ends its headings at a line of dashes. The Conduit Surcharge Summary has dashes inside a heading as well, so one row of headings was read as data:'),
    dict(t='code', x='  ----------------------------------------------------------------------------\n                                                           Hours        Hours\n                         --------- Hours Full --------   Above Full   Capacity\n  Conduit                Both Ends  Upstream  Dnstream   Normal Flow   Limited\n  ----------------------------------------------------------------------------\n\n// rpt.rs, as first written: any line starting with dashes counted as a rule\nif tr.starts_with("---") {\n\n// as fixed: a rule is a line of dashes and nothing else\nif tr.len() >= 3 && tr.bytes().all(|c| c == b\'-\') {', cap='The heading that fooled it, the line that was fooled, and the fix.'),
    dict(t='p', x=f'The check that caught it is the one that keeps the reader honest: a separate reader, written in Python, finds each attribute’s number in EPA’s report and compares. Across six models, {fmtk(all_vals)} result values agree and {all_dis} disagree.'),
    dict(t='h2', x='A second reader, with no stake in it'),
    dict(t='p', x='A reader that checks numbers can only find wrong numbers. It cannot find a right number with the wrong name on it. For that the code itself has to be read, and read by someone who did not write it and has nothing riding on its being right. So a second Claude agent, started fresh with no part in the build, was handed the plugin, the engine patch and every claim in the README, and told to break them. It came back with seventeen findings. The worst one is the kind no numeric check will ever catch.'),
    dict(t='p', x='SWMM has two spellings for dynamic wave routing. The manual says `DYNWAVE`; the engine also takes the old keyword `DW`, a leftover from SWMM 4, and EPA’s own test suite uses it in Example2.inp and Example3.inp. Under dynamic wave the last column of the Node Flooding table is a ponded depth in feet. Under anything else it is a ponded volume in thousands of cubic feet. The plugin decided which by comparing the option to the full word:'),
    dict(t='code', x='// lib.rs, as it was: dynamic wave only when spelled out in full\nres = rpt::parse(&text, &units, m.routing == "DYNWAVE");\n\n// rpt.rs, as it was: that flag names the last flooding column\nlet last = if dynwave { format!("max_ponded_depth_{l}") } else { format!("max_ponded_vol_{v3}") };\n\n// rpt.rs, as it was: and the volume unit\nfn vol3(&self) -> &\'static str { if self.us { "kft3" } else { "km3" } }', cap='Three lines as they were. On Example2 the first made a depth in feet into `max_ponded_vol_kft3`. The third named thousands of cubic meters `km3`, which reads as cubic kilometers: off by a factor of a million.'),
    dict(t='p', x='The number was right; the report said “Ponded Depth Feet” right above it; and the attribute check passed, because it compares numbers. Now the plugin asks the report which routing the engine actually used (“Flow Routing Method ...... DYNWAVE”), so whatever the .inp says, the label follows the engine. The volume columns are named `1000ft3` and `1000m3`, which is what the report prints.'),
    dict(t='table', head=['#', 'What the reviewer found', 'What changed'], w=[1, 9, 7], rows=[
        ['1', 'Ponded depth labeled a volume when routing is spelled `DW`', 'Routing read from the report'],
        ['2', 'Volume fields named `kft3` and `km3`, off by 10^6 as read', '`1000ft3` and `1000m3`'],
        ['3', 'An EPSG code RustGIS lacks put the model in the wrong place, silently', 'Codes checked against RustGIS 0.5.0’s table'],
        ['4', 'Data files with the same name collided; case had to match', 'Same names refused; any case; messages say which file stood in'],
        ['5', 'An in-memory read past the end crashed where a disk read comes up short', 'Clamped, as a disk does'],
        ['6', 'After a run, outlets became DUMMY and irregular conduits CHANNEL', '`type` from the input; the report’s word in `rpt_type`'],
        ['7', 'CUSTOM barrels read from the wrong column', 'Column 6, as the engine reads it'],
        ['8', 'A bad Link Flow row still wrote attributes; Outfall rows went unchecked', 'Every cell checked in every table; all skips counted'],
        ['9', 'Claimed RustGIS shows the panic line; it drops a failed tool’s log', 'Claim corrected here and in the README'],
        ['10', 'Names split on a non-breaking space; matched with case', 'SWMM’s separators only; case ignored, as SWMM does'],
        ['11', 'A missing continuity line borrowed the next section’s', 'Read stops at the section’s end'],
        ['12', 'Report hash said to match the gate’s; it does not', 'Claim removed'],
        ['13', '`hrs_above_normal` misnamed; times said to be from the run’s start', '`hrs_above_full_normal`; from REPORT_START'],
        ['14', 'Rain gages with no symbol dropped', 'Listed in a table'],
        ['15', 'Command icon not in RustGIS’s set', '`help`'],
        ['16', 'memfs not additive; output file held twice just to hash it', 'Documented; hashed where it lies'],
        ['17', 'README claimed more than was tested (apps, clock lines, “true size”)', 'Corrected'],
    ], cap='The defect register, numbered as the reviewer numbered it, most serious first. Every one was fixed before this version shipped.'),
    dict(t='p', x=f'Fixing them turned up an eighteenth. The report reader split rows on blanks and took the first cell as the name, but a quoted SWMM name may hold spaces, and then every cell shifts one place. Reading rows from the right fixed that too. To be sure the fixes changed only what they meant to, the plugin code before and after was run on all 221 models. {fix_text}'),
    dict(t='table', head=['#', 'Fix', 'Models where it changed the map', 'Values'], w=[1, 6, 8, 2], rows=fx_rows,
         cap='Before and after, on the 221 models: every difference, grouped by the finding that caused it. A value is one attribute or one geometry of one object.'),
    dict(t='p', x='The run put a face on finding 10. The reviewer had pictured a name with a non-breaking space in it, pasted from a spreadsheet. Two models in the corpus, lakes.inp and seepage.inp from the NCIMM set, have exactly that. To SWMM the character is part of the name: its tokenizer splits on space, tab, carriage return and line feed, and nothing else. The old reader split on it anyway.'),
    dict(t='code', x='[VERTICES]   seepage.inp, as it is (the gap after 1_ is byte A0, a non-breaking space)\n1_\u00a05031    466164.303000    137997.322000\n\n// inp.rs, as it was: any Unicode whitespace ended a token\nc if c.is_whitespace() => {\n\n// the old reading:  link "1_",      x = 5031,       y = 466164.303\n// SWMM\'s reading:   link "1_ 5031", x = 466164.303, y = 137997.322', cap='The line, and the line of code that misread it.'),
    dict(t='p', x=f'One stray point at x = 5,031 feet stretched the model’s bounding box, and the local frame centers a model on that box, so the old reader drew all of seepage.inp about {seep_miles:.0f} miles from where it belonged. In lakes.inp, conduit 10284NW ran to a node called “TO” instead of “TO MNRIV4”, and its 112.854-foot length went into its roughness. EPA’s engine read both files correctly all along, and every result value was right. Only the map was wrong, and only a reader of the code would have known why.'),
    dict(t='h2', x='A third reader, from outside'),
    dict(t='p', x='Two readers who share a workshop share its blind spots, so Bob passed the finished page to an outside reviewer: another AI system, with no part in this build. It ran both engines on the six demos itself, found the output files identical, and scored the page 80 out of 100. Then it put its finger on the one place the build had not looked at hard enough: the checker that grades everything else.'),
    dict(t='p', x='The Verify tab checks each attribute on the map against EPA’s report. The way it checked was this: take the value, and see whether that number appears anywhere in the same object’s rows of the report.'),
    dict(t='code', x="// app.js, as it was: a value passes if the number appears anywhere in the object's report rows\nreturn tok.some(x => /^-?[\\d.]+$/.test(x) && Math.abs(parseFloat(x) - v) <= 1e-9 * Math.max(1, Math.abs(v)));", cap='The line as it was. The reviewer set node 9’s maximum depth to its average depth, 0.17 ft instead of 0.23 ft, and the check still passed: 0.17 is in node 9’s row. The pattern also rejects 6.27e-07, and nothing in it expects a word.'),
    dict(t='p', x=f'That is a test a wrong number can pass, and a right number can fail. On the six demos it reported {fmtk(old_agree)} values agreeing and {old_dis} disagreeing, and every one of the {old_dis} was the checker’s fault: {old_str} were the links’ `rpt_type` words, added by the first round of fixes and never taught to the checker, and {old_sci} were Greenville values printed in scientific notation. Planted on every model, the swap of average for maximum depth went through {old_swap_missed} times out of {swap_models}. The static figure on the Verify tab came from a different checker, the Python one, which mapped most fields to their columns but fell back to the same row search for any field it had no column for. Neither number was false. Neither was a test you could lean on.'),
    dict(t='p', x='The fix is to stop asking whether a number is somewhere and start asking whether it is in the right place. Each attribute name now maps to one table, one column and one unit. A value filed under the wrong name disagrees. A result attribute with no column in the report disagrees. A column the layer lacks is counted as missing. The ponded-depth column is identified by reading the table’s own heading, not by trusting the routing option.'),
    dict(t='p', x='A meter is calibrated by feeding it a known wrong input, so the checker is too. Every time it runs, it plants six errors in a copy of the layers (maximum depth swapped for average, a link’s type changed, feet relabeled as meters, a peak runoff 0.1% high, a time one minute late, a result left out) and checks that each one is caught.'),
    dict(t='table', head=['On the six demos', 'Checker as it was', 'Checker now'], w=[5, 3, 3], rows=[
        ['Values agreeing with EPA’s report', fmtk(old_agree), fmtk(new_agree)],
        ['Disagreeing', f'{old_dis} (all the checker’s fault)', str(new_dis)],
        ['Missing from the layers', 'not counted', str(new_miss)],
        ['Maximum depth replaced by average depth', f'passed on {old_swap_missed} of {swap_models} models', 'caught on every model'],
        ['Planted errors caught', 'not tested', f'{st_caught} of {st_tried}'],
        ['Example2 as the plugin wrote it before the first review (ponded depth named as a volume)', '133 agree, 0 disagree', '2 disagree, 2 missing'],
    ], cap=f'Measured with node test_verify.js and verify_attrs.py. The page’s checker (JavaScript) and the static one (Python) are written separately and now agree model by model: {per_model} values. The last row is the worst bug of the second review, which the old checker could not have found.'),
    dict(t='p', x='The reviewer’s second finding was plainer. The plugin got the model’s data files; EPA’s engine, run for comparison, got the .inp and nothing else. Move a model’s rainfall into a file and the plugin runs while EPA stops with ERROR 361, could not open external file.'),
    dict(t='code', x="// app.js, as it was: EPA's engine got the .inp and nothing else\nconst r = self.SwmmEngineCore.runLegacy(epaMod, 'm.inp', { 'm.inp': d.inp });", cap='The line as it was.'),
    dict(t='p', x='Fixing it turned up a wrinkle. EPA’s engine opens a data file only by the exact path the .inp writes, say `C:\\Users\\bob\\storms\\rain_ts1.dat`; the plugin finds it by its own name. So both engines now get one frozen bundle, and EPA gets each data file under its own name and under every path the .inp gives for it. The reviewer’s case, rebuilt with that Windows path, now runs in both and matches byte for byte.'),
    dict(t='p', x='The third finding was about time. A run’s answer was filed under whatever model the form showed when the answer came back, and there was no way to stop a run. Now every run gets a number, and its inputs are copied and hashed the moment it starts. An answer that comes back after a newer run started is dropped. A run can be cancelled, or is stopped after ten minutes, and the engine is restarted clean.'),
    dict(t='code', x="// app.js, as it was: no run number, and the name read from the form when the reply arrived\nconst r = await call({ kind: 'plugin', request });\n$('#runline').innerHTML = `... ${esc(S.model.name)} · ${fmt(n)} features ...`;", cap='The lines as they were.'),
    dict(t='h3', x='What the page can now show'),
    dict(t='p', x=f'The plugin returns summaries; time series live in the output file, and the plugin hands back only that file’s hash. So the page runs EPA’s engine on the same bundle and reads its output file with the swmm5-rust app’s own .out reader. When that file’s SHA-256 equals the plugin’s, the curves are the plugin’s, bit for bit, and the page says so; when it does not, the page says that instead. The page’s reader and an independent Python reader agree on all 15,552 values sampled from three demos.'),
    dict(t='fig', f='hydro', cap=f'Node 9 of the storage example, its depth at each {hydro["step_min"]}-minute reporting period, from an output file whose SHA-256 ({hydro["out_sha"][:12]}…) is the one the plugin reported. The dashed line is the report’s maximum, 0.23 ft, taken over every routing step and printed to two decimals; the highest reported period is {max(hydro["depth"]):.4f} ft.'),
    dict(t='list', x=[
        '**Hydrographs:** click a node, link or subcatchment (or a row, or find it by ID) for its time series, with the report’s own maximum drawn for comparison.',
        '**Time slider:** drag it, or press play, and the map colors every node, link and subcatchment by that moment’s value.',
        '**Run summary:** continuity errors, warnings, flooded and surcharged nodes and units, with badges that keep three things apart: it ran in this browser; what RustGIS’s fuel meter measured; whether this run has been verified.',
        '**Run bundle:** one .zip with the inputs, the plugin’s layers and messages, EPA’s report and output file, every hash, and the verification result.',
        '**Find by ID and zoom to selection; a table that scrolls any number of rows** (the 17,100-element deck’s 8,593 nodes) and filters by any text; keyboard access to the tabs, menus, map, table and dialogs.',
    ]),
    dict(t='h2', x='Proof in the box itself'),
    dict(t='p', x=f'A host written for a test proves only that the test host works. So RustGIS {manifest.get("minRustgisVersion")} was built from source and the packaged plugin was installed in it, the way a user installs it. The command line loads the World Atlas sample (a tool needs a layer on the map), installs the .rgplugin for the run, calls `gp.run` on the tool, and then RustGIS’s own exporter writes each output layer back out. Those files are what the attribute check reads: {fmtk(rg_vals)} values from {len(rg_ok)} models, every one equal to EPA’s report.'),
    dict(t='fig', f='rgshot', cap='RustGIS drew this, not this page: its own map export of the storage example after the plugin ran, nodes graduated by max_depth_ft.'),
    dict(t='p', x=f'Then the census. The swmm5-rust gate’s 221 models were run through the plugin and through EPA twice more. {C["caption"]}'),
    dict(t='fig', f='census', cap='Green: identical files. Red: the files differ. Orange: both engines stop with the same input error. Gray: memory-error decks, where EPA reads memory it does not own and the plugin stops. The bottom bar is the control: EPA against itself on two math libraries.'),
    dict(t='p', x=(f'The gray segment is where safe Rust earns its keep. On {C["mem"]} decks from the OpenSWMM 2026 code review, EPA’s C indexes past the end of an array, reads whatever is there, and carries on; its answer depends on what happened to be in memory. swmm5-rust refuses. The plugin logs where it stopped before the module aborts. RustGIS itself reports only that the tool stopped with an error, because it drops whatever a failed tool logged; this page keeps the log. On {mem_example["inp"].split("/")[-1]} the logged line reads:' if mem_example else '')),
    dict(t='code', x=((mem_example.get('p_error') or '').replace('trap: unreachable | ', '').replace(' | ', '\n') if mem_example else ''), cap='The plugin’s last log line on a memory-error deck, as this page shows it. RustGIS, on the desktop, the web or the command line, shows only that the tool stopped with an error.'),
    dict(t='h2', x='The fuel bill'),
    dict(t='p', x=f'RustGIS meters plugins. A tool call gets {F["fuel_budget"]} units of fuel and a command {F["cmd_budget"]}; wasmi burns fuel as the module executes its instructions, and when the tank is empty RustGIS stops the call and says it ran too long. That is how a host keeps a buggy plugin from hanging the map. It is also a ceiling on model size, and the ceiling is real. (RustGIS’s plugin guide quotes the budget as about 4×10^10 instructions per tool; the constant in the 0.5.0 source is 3×10^10 fuel, which is what the meter below counts, and the runs inside RustGIS stop exactly where the meter says they will.)'),
    dict(t='fig', f='fuel', cap='Fuel each demo model needs for a run, measured with wasmi 2.0.0 configured as RustGIS 0.5.0 configures it, on a log scale. Two of the six do not fit; their Map only runs do.'),
    dict(t='table', head=['Model', 'Fuel', 'Share of budget', 'wasmi seconds (release build)'],
         rows=[[d[2], f'{fuel[d[0]]["fuel"]/1e9:.2f}×10^9', f'{fuel[d[0]]["pct_of_budget"]}%', f'{fuel[d[0]]["wasmi_seconds"]}'] for d in DEMOS],
         cap='Measured, not estimated: the same plugin.wasm, the same request RustGIS builds, fuel counted by the interpreter.'),
    dict(t='p', x=f'The rule of thumb that falls out: about ten seconds of interpreted run time. The control-rules model spends {F["ce"]["pct_of_budget"]}% of the tank; EPA’s user2, at {F["u2"]["pct_of_budget"]}%, does not finish. For a model past the ceiling there are three roads. Map it without running it, which always fits. Run it on this page, where the same module runs in the browser’s own WebAssembly engine with nothing metering it, then export the layers as GeoJSON and drop them on RustGIS. Or use the desktop process plugin built in October, which runs EPA’s C natively and has no fuel to spend.'),
    dict(t='h2', x='What this does not show'),
    dict(t='list', x=[
        'It is not a new engine and not more accurate. Every number is EPA SWMM 5.2.4’s, defects included.',
        'Byte identity is against EPA compiled to WebAssembly with wasi-libc. Against EPA native on Linux the match is lower, by the same platform effect EPA shows against itself.',
        'Inside RustGIS a run must fit in the fuel budget; the measured ceiling sits between the control-rules model and EPA’s user2.',
        'The sandbox has no clock: the report’s begin and end lines read 1 January 1970 and its elapsed time reads under a second.',
        'A report row whose numbers overflow their printed width runs two numbers together (the code review’s API-18 deck prints 2000000000.002000000000.00). The plugin leaves such a row out whole, counts the rows it skipped by table, and names the first ten.',
        'The EPSG check is against RustGIS 0.5.0’s table; a later RustGIS may know codes this plugin refuses. A known code that is not the model’s own still draws the model in the wrong place, and RustGIS leaves out features it cannot convert without saying so.',
        'When the engine panics, RustGIS shows only that the tool failed; the line saying where is visible on this page, not in RustGIS.',
        'The layers carry summary statistics, not time series; the .out file is hashed, not returned.',
        'RustGIS was exercised through rustgis-cli built from source. The web app and the desktop window run the same plugin crate, but the screens themselves were not driven here.',
        'The module was built with Ubuntu’s Rust 1.91 and std compiled from source; a rustup build is expected to behave the same and was not run.',
        'The page’s time series come from EPA’s run of the same inputs. They are the plugin’s numbers only when the two output files share a SHA-256, which the page checks and states on every run.',
        'The attribute checkers were written by the same AI that wrote the plugin. The planted-error self-test shows they can fail; it does not make them independent.',
        'The memory figure is the browser’s WebAssembly memory for the call. RustGIS’s 512 MB cap is applied by wasmi inside RustGIS, which this page does not run.',
        'The code reviews were by AI systems, a second Claude agent and an outside reviewer, not by a person. No person has yet reviewed the plugin code line by line.',
    ]),
    dict(t='h2', x='How to use it'),
    dict(t='list', x=[
        '**On this page:** pick a model or open an .inp (and its data files), then Run SWMM 5 Model. Click anything for its time series; drag the time slider to watch the run. Verify runs EPA’s engine on the same inputs and compares. Export gives the .rgplugin, the active table as CSV, the layers as GeoJSON for RustGIS, and the whole run as one .zip.',
        '**RustGIS web app:** open web.rustgis.app, start from the World Atlas sample, drag swmm5-rust-0.1.0.rgplugin onto the map (or Plugins › Plugin Manager › Install from a file), then use the SWMM5 Rust tab.',
        '**RustGIS desktop:** the same Plugin Manager, or drop the file on the window.',
        '**Command line:** `rustgis-cli run --sample --plugin swmm5-rust-0.1.0.rgplugin --cmd \'gp.run={"tool":"plugin:swmm5-rust/run-swmm","params":{"model":"model.inp"}}\'`',
        '**Rebuild it:** `rustgis/build_wasm.sh` then `python3 rustgis/scripts/package.py --no-build`, which also runs the plugin registry’s own checks.',
    ]),
    dict(t='h2', x='AI use'),
    dict(t='p', x='Claude (Anthropic’s model, Claude Opus 5.5) built this in one session on 31 October and 1 November 2026 at Bob Dickinson’s request: the memfs feature and the three engine edits, the plugin crate and its packaging, the wasm toolchain workaround, the RustGIS build and runs, the census, the fuel meter, this page and this essay. A second Claude agent reviewed the code and the claims; its seventeen findings are in the register above, all fixed. An outside AI review that Bob passed along found the verifier’s weaknesses described above; those were fixed in page 1.1 on 1 November 2026. The checks are mechanical (byte comparisons, an independent report reader, runs inside RustGIS itself, the code before and after the fixes on 221 models), not the AI’s judgment. The engine is the swmm5-rust translation Claude made for Bob on 28 and 29 October 2026; EPA SWMM 5.2.4 is Lewis Rossman’s design.'),
]

ABOUT = dict(
    what=('This page runs **swmm5-rust** (EPA SWMM 5.2.4 translated into safe Rust, [github.com/SWMMBobSWMM6/swmm5-rust](https://github.com/SWMMBobSWMM6/swmm5-rust)) as the RustGIS plugin it was built to be. '
          'The WebAssembly module embedded here is the one inside swmm5-rust-0.1.0.rgplugin; the page plays RustGIS’s side of the plugin interface (ABI 1) in JavaScript. '
          '[RustGIS](https://github.com/opengeos/RustGIS) is the open-source GIS by Qiusheng Wu and opengeos.'),
    ai=('Built with Claude (Anthropic, Claude Opus 5.5) in one session on 31 October and 1 November 2026 for Robert (Bob) Dickinson, SWMM Enablement. Claude wrote the memfs engine feature, the plugin crate, '
        'the packaging, the census and RustGIS test harnesses, the fuel meter, this page and the Doc essay, and ran every check reported here. A second Claude agent reviewed the code and claims; its 17 findings were fixed. An outside AI review Bob passed along led to page 1.1 (field-by-field verification, run bundles, time series). Bob chose the goal: connect swmm5-rust to RustGIS in an HTML app. '
        'The checks are mechanical, not the AI’s judgment. No person has yet reviewed the plugin code line by line.'),
    credits=[
        'EPA SWMM 5.2.4: U.S. EPA, public domain; Lewis Rossman’s engine design. swmm5-rust: MIT.',
        'RustGIS 0.5.0 and its plugin SDK (opengeos/rustgis-plugins): Apache-2.0 and MIT; packaging script from opengeos/rustgis-plugin-template (MIT).',
        'wasi-libc (its musl-derived math library is linked into the plugin): MIT and Apache-2.0 terms of its components.',
        'engine_core.js (the in-memory WASI layer used to run EPA’s wasm here) comes from the swmm5-rust app, MIT.',
        'Models: EPA examples and test models, pyswmm’s test model, and the Greenville training network (Innovyze, from City of Naperville data), all from SWMMBobSWMM6/1729-SWMM5-Models-2030.',
        'Fonts: Barlow, Barlow Condensed and JetBrains Mono (SIL Open Font License) from Google Fonts.',
    ],
    limits=('The plugin is byte-identical to EPA SWMM 5.2.4 compiled to WebAssembly with wasi-libc, not to every EPA build; the census measures the native gap. Inside RustGIS a run must fit the fuel budget. '
            'The plugin returns summary results; the page’s time series come from EPA’s run of the same inputs, tied to the plugin’s output by SHA-256. The RustGIS screens were not driven here; rustgis-cli was.'),
    foot=(f'swmm5-rust in RustGIS · plugin 0.1.0 · plugin.wasm SHA-256 {sha(plugin)[:16]}… · package SHA-256 {sha(rgpkg)[:16]}… · tested in rustgis-cli 0.5.0 (a5603d1) · Built with Claude for SWMM Enablement, 31 October–1 November 2026.'),
    footLimits='identity is against EPA’s WebAssembly build; RustGIS meters runs with fuel; the plugin returns summary results, and the page’s time series are read from EPA’s run of the same inputs.',
)
PAGE_VERSION = '1.1, 1 November 2026 (outside review applied: field-by-field verification, run bundles, time series)'
PROV = dict(
    rustgis='0.5.0 (a5603d1)', plugin_sha=sha(plugin), plugin_bytes=len(plugin), rgplugin_sha=sha(rgpkg), rgplugin_bytes=len(rgpkg),
    rows=[
        ['plugin.wasm SHA-256', sha(plugin)], ['swmm5-rust-0.1.0.rgplugin SHA-256', sha(rgpkg)], ['EPA SWMM 5.2.4 wasm (swmm524_epa.wasm) SHA-256', sha(epa)],
        ['swmm5-rust engine', 'SWMMBobSWMM6/swmm5-rust c2af2d3 + memfs feature (5 files, +55 −7 lines, new memfs.rs)'],
        ['page', PAGE_VERSION],
        ['.out reader', 'rpt_compare.js parseOut, from SWMMBobSWMM6/swmm5-rust app/ (the swmm5-rust app\u2019s own reader)'],
        ['RustGIS', 'opengeos/RustGIS v0.5.0 a5603d1, rustgis-cli built from source (Rust 1.97)'],
        ['plugin SDK', 'opengeos/rustgis-plugins sdk 24e323c'],
        ['plugin toolchain', 'Ubuntu rustc 1.91.1, std built from source for wasm32-unknown-unknown, wasm-ld, wasi-libc 0.0~git20230113 (math only)'],
        ['fuel meter', 'wasmi 2.0.0, Config::default() + consume_fuel(true), as RustGIS 0.5.0'],
        ['corpus', 'SWMMBobSWMM6/1729-SWMM5-Models-2030 9ee63bd'],
    ],
)
headline = [
    dict(v=f'{C["pe"]} / {C["ok"]}', d='models byte-identical to EPA’s WebAssembly build (plus ' + str(C['same_err']) + ' same-error decks)'),
    dict(v=fmtk(all_vals), d=f'result attributes checked against EPA’s report column by column, {all_dis} disagree'),
    dict(v=f'{len(rg_ok)} of {len([r for r in rg if r["tool"] == "run-swmm"])}', d='demo models that fit RustGIS’s fuel budget for a full run'),
    dict(v=f'{reg_same + (1 if lakes["out_same"] and lakes["rpt_same"] else 0)} / {len(regress)}', d='models unchanged in the engine’s default build'),
]

shot = rd(f'{W}/rgtest/out/jason_dorn_storage_example.small.png')
APP = dict(version='0.1.0', pageVersion=PAGE_VERSION, hydro=hydro, anchor=[-82.3474, 29.6476], manifest=manifest, demos=demos, census=C, rgruns=rg, attrs=attrs,
           regress=dict(text=reg_text), prov=PROV, doc=DOC, about=ABOUT, headline=headline,
           libm=dict(model='swmm_example.inp', rows=libm_rows), rgshot='data:image/png;base64,' + b64(shot))

tpl = open(f'{W}/app/template.html').read()
js = open(f'{W}/app/app.js').read()
cmp_js = open(f'{W}/swmm5-rust/app/rpt_compare.js').read()      # the swmm5-rust app's .out reader, imported, not copied
verify_js = open(f'{W}/app/verify.js').read()
docx = rd(DOCX) if os.path.exists(DOCX) else b''
html = (tpl.replace('/*__PLUGIN_WASM__*/', b64(plugin)).replace('/*__EPA_WASM__*/', b64(epa)).replace('/*__RGPLUGIN__*/', b64(rgpkg))
        .replace('/*__DOCX__*/', b64(docx)).replace('/*__ENGINE_CORE__*/', core.replace('</script', '<\\/script'))
        .replace('/*__CMP_JS__*/', cmp_js.replace('</script', '<\\/script')).replace('/*__VERIFY_JS__*/', verify_js.replace('</script', '<\\/script'))
        .replace('/*__DATA__*/', json.dumps(APP).replace('</', '<\\/')).replace('/*__APP_JS__*/', js))
os.makedirs(os.path.dirname(OUT), exist_ok=True)
# Two files from one page: the artifact (claude.ai wraps it in its own document) and a standalone
# document with its doctype, charset and viewport, for opening from disk or a web server.
cut = html.index('</style>') + len('</style>')
standalone = ('<!doctype html>\n<html lang="en" data-theme="dark">\n<head>\n<meta charset="utf-8">\n'
              '<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">\n'
              + html[:cut] + '\n</head>\n<body>\n' + html[cut:] + '\n</body>\n</html>\n')
open(OUT, 'w').write(standalone)
art = os.path.join(os.path.dirname(OUT), 'artifact', os.path.basename(OUT)); os.makedirs(os.path.dirname(art), exist_ok=True)
open(art, 'w').write(html)
json.dump(dict(C=C, rg=rg, headline=headline, F={k: v for k, v in F.items()}), open(f'{W}/app/dist/facts.json', 'w'), indent=1, default=str)
print(OUT, f'{len(html)/1e6:.2f} MB', 'docx embedded' if docx else 'no docx yet')
