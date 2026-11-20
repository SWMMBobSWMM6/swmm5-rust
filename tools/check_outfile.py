#!/usr/bin/env python3
"""Check the swmm-out reader (crates/outfile) against three things it did not come from:

  A  swmm-toolkit (OWA's C output library, from PyPI): every value of every period, bit for bit
  B  swmmrr/outfile.py (the run-record tool's Python reader): every value, bit for bit
  C  the engine's own time series tables in the .rpt (written when no .out name is given):
     every printed number and every time stamp, at the precision printed

RUNS_DIR holds NAME.out files written by an engine given three file names, and RUNS_DIR/tables/NAME.rpt
written by the same engine given two (no .out name), which is when it prints the tables.
SWMMRR_DIR is a checkout of SWMMBobSWMM6/swmm-run-record-repo. Needs: pip install swmm-toolkit numpy

usage: check_outfile.py SWMMOUT_BINARY RUNS_DIR [SWMMRR_DIR]
"""
import csv, io, os, re, subprocess, sys
import numpy as np

SWMMOUT, RUNS = sys.argv[1], sys.argv[2]
SWMMRR = sys.argv[3] if len(sys.argv) > 3 else None
csv.field_size_limit(1 << 30)


def dump(path):
    """{(kind, id, var): float32 array}, dates (float64 array) from `swmmout dump`."""
    text = subprocess.run([SWMMOUT, 'dump', path], capture_output=True, text=True, check=True).stdout
    rows = csv.reader(io.StringIO(text))
    head = next(rows)
    dates = np.array([float(x) for x in head[3:]], dtype=np.float64)
    series, order = {}, []
    for r in rows:
        key = (r[0], r[1], r[2])
        series[key] = np.array([np.float32(x) for x in r[3:]], dtype=np.float32)
        order.append(key)
    return dates, series, order


def same_bits(a, b):
    a = np.asarray(a, dtype=np.float32)
    b = np.asarray(b, dtype=np.float32)
    return a.shape == b.shape and np.array_equal(a.view(np.uint32), b.view(np.uint32))


def check_toolkit(path, dates, series, order):
    from swmm.toolkit import output, shared_enum as e
    h = output.init()
    output.open(h, path)
    nsub, nnode, nlink, _, npoll = output.get_proj_size(h)
    nper = output.get_times(h, e.Time.NUM_PERIODS)
    assert nper == len(dates), (nper, len(dates))
    kinds = [('subcatchment', nsub, 8 + npoll, e.ElementType.SUBCATCH, output.get_subcatch_attribute, e.SubcatchAttribute),
             ('node', nnode, 6 + npoll, e.ElementType.NODE, output.get_node_attribute, e.NodeAttribute),
             ('link', nlink, 5 + npoll, e.ElementType.LINK, output.get_link_attribute, e.LinkAttribute)]
    compared = 0
    bad = []
    # names, in order
    mine = {k: [] for k in ('subcatchment', 'node', 'link')}
    for kind, oid, var in order:
        if kind in mine and (not mine[kind] or mine[kind][-1] != oid):
            mine[kind].append(oid)
    for kind, n, nvar, et, getter, enum in kinds:
        names = [output.get_elem_name(h, et, i) for i in range(n)]
        if names != mine[kind]:
            bad.append('%s names differ' % kind)
            continue
        varnames = [key[2] for key in order if key[0] == kind and key[1] == (names[0] if names else None)]
        if n and len(varnames) != nvar:
            bad.append('%s variable count %d vs %d' % (kind, len(varnames), nvar))
            continue
        base = list(enum)[-1].value      # POLLUT_CONC_0
        for v in range(nvar):
            # the first pollutant has an enum member; later ones are that value + j
            attr = enum(v) if v <= base else v
            grid = np.empty((nper, n), dtype=np.float32)
            for p in range(nper):
                grid[p] = getter(h, p, attr)
            for i, name in enumerate(names):
                if not same_bits(series[(kind, name, varnames[v])], grid[:, i]):
                    bad.append('%s %s %s' % (kind, name, varnames[v]))
                compared += nper
    sysvars = [key[2] for key in order if key[0] == 'system']
    grid = np.array([output.get_system_result(h, p, 0) for p in range(nper)], dtype=np.float32).reshape(nper, -1)
    for v, name in enumerate(sysvars):
        if not same_bits(series[('system', '', name)], grid[:, v]):
            bad.append('system %s' % name)
        compared += nper
    # dates: the toolkit builds them from the start date and step
    start = output.get_start_date(h)
    step = output.get_times(h, e.Time.REPORT_STEP)
    nominal = start + (np.arange(nper) + 1) * step / 86400.0
    date_gap = float(np.max(np.abs(nominal - dates)) * 86400.0) if nper else 0.0
    output.close(h)
    return compared, bad, date_gap


def check_swmmrr(path, dates, series, order):
    sys.path.insert(0, SWMMRR)
    from swmmrr.outfile import OutFile
    f = OutFile(path)
    flat = np.array([f.period_values(k) for k in range(f.nperiods)], dtype=np.float32)
    mine = np.array([series[k] for k in order], dtype=np.float32).T if order else np.zeros((0, 0), np.float32)
    ok = same_bits(mine, flat) and np.array_equal(np.array(f.dates()), dates)
    return flat.size, ok


NUM = re.compile(r'^-?\d+\.\d+$|^-?\d+$')


def check_rpt(rpt, dates_iso, series):
    """Compare every number of every time series table with the reader's value."""
    lines = open(rpt, encoding='latin-1').read().split('\n')
    compared = worst = 0
    bad, skipped = [], set()
    stamps_ok = True
    i = 0
    colmap = {'subcatchment': {'Precip.': 'rainfall', 'Runoff': 'runoff'},
              'node': {'Inflow': 'total_inflow', 'Flooding': 'flooding', 'Depth': 'depth', 'Head': 'head'},
              'link': {'Flow': 'flow', 'Velocity': 'velocity', 'Depth': 'depth', 'Capacity/': 'capacity'}}
    while i < len(lines):
        m = re.match(r'\s*<<< (Subcatchment|Node|Link) (.*) >>>\s*$', lines[i])
        if not m:
            i += 1
            continue
        kind, oid = m.group(1).lower(), m.group(2)
        # the column names are on the first header line; a subcatchment table also has "Date Time" there.
        # Names with a blank in them (Snow Depth, GW Elev., GW Flow) split in two and are not compared;
        # they come after Precip., Losses and Runoff, so those three still line up.
        h1 = lines[i + 2]
        cols = h1.split()[2:] if kind == 'subcatchment' else h1.split()
        j = i + 5
        k = 0
        while j < len(lines) and lines[j].strip():
            parts = lines[j].split()
            stamp = '%s-%s-%sT%s' % (parts[0][6:10], parts[0][0:2], parts[0][3:5], parts[1])
            if stamp != dates_iso[k]:
                stamps_ok = False
            for name, txt in zip(cols, parts[2:]):
                var = colmap[kind].get(name)
                if var is None:
                    var = 'quality:' + name if (kind, oid, 'quality:' + name) in series else None
                if var is None:
                    skipped.add(name)
                    continue
                val = float(series[(kind, oid, var)][k])
                dec = len(txt.split('.')[1]) if '.' in txt else 0
                err = abs(float(txt) - val)
                tol = 0.5 * 10 ** -dec + 1e-6 * max(1.0, abs(val))
                worst = max(worst, err / (10 ** -dec))
                if err > tol:
                    bad.append((kind, oid, var, k, txt, val))
                compared += 1
            j += 1
            k += 1
        if k != len(dates_iso):
            bad.append((kind, oid, 'rows', k, len(dates_iso), None))
        i = j
    return compared, bad, sorted(skipped), stamps_ok, worst


def iso_stamps(path):
    text = subprocess.run([SWMMOUT, 'series', path, 'sys', 'rainfall'], capture_output=True, text=True, check=True).stdout
    return [l.split(',')[0] for l in text.strip().split('\n')[1:]]


total_a = total_b = total_c = 0
fail = False
for name in sorted(os.listdir(RUNS)):
    if not name.endswith('.out') or name == 'part.out':
        continue
    path = os.path.join(RUNS, name)
    dates, series, order = dump(path)
    n, bad, gap = check_toolkit(path, dates, series, order)
    total_a += n
    line = '%-46s A toolkit: %9d values %s (dates within %.1e s of start+k*step)' % (name, n, 'identical' if not bad else 'DIFFERENT %s' % bad[:3], gap)
    fail |= bool(bad)
    if SWMMRR:
        nb, ok = check_swmmrr(path, dates, series, order)
        total_b += nb
        line += ' | B swmmrr: %s' % ('identical' if ok else 'DIFFERENT')
        fail |= not ok
    rpt = os.path.join(RUNS, 'tables', name[:-4] + '.rpt')
    if os.path.exists(rpt) and len(dates):
        nc, badc, skipped, stamps_ok, worst = check_rpt(rpt, iso_stamps(path), series)
        total_c += nc
        line += ' | C rpt tables: %d numbers %s, stamps %s, worst %.2f of a last digit' % (
            nc, 'agree' if not badc else 'DISAGREE %s' % badc[:3], 'agree' if stamps_ok else 'DISAGREE', worst)
        if skipped:
            line += ' (columns not compared: %s)' % ', '.join(skipped)
        fail |= bool(badc) or not stamps_ok
    print(line)
print('totals: A %d values, B %d values, C %d printed numbers' % (total_a, total_b, total_c))
sys.exit(1 if fail else 0)
