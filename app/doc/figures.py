#!/usr/bin/env python3
"""Figures for the swmm5-rust Doc tab and .docx, drawn from measured data.

Every colour is a CSS custom property (--f-* for figure ink and faces, --epa,
--cpp and --rs for the three engines), so the app themes the figures and they
repaint with the theme toggle. svg_for_docx() substitutes the light-theme
values when the same SVG is rasterised for Word.
"""
import math, html

FONT = "IBM Plex Sans, Helvetica, Arial, sans-serif"
MONO = "IBM Plex Mono, Menlo, Consolas, monospace"
LIGHT = {'--f-ink': '#1b2733', '--f-dim': '#5a6878', '--f-line': '#cfd8e3', '--f-face': '#ffffff',
         '--f-soft': '#eef3f8', '--f-a': '#1f6fd1', '--f-b': '#a15c00', '--f-c': '#1d7a45',
         '--f-d': '#c2352b', '--f-e': '#6b4fbb', '--f-on': '#ffffff',
         '--epa': '#a15c00', '--cpp': '#1f6fd1', '--rs': '#b8326f'}

def esc(s): return html.escape(str(s))

def svg(w, h, body, label):
    return (f'<svg viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg" role="img" '
            f'aria-label="{esc(label)}" font-family="{FONT}" style="max-width:100%;height:auto">'
            f'<rect x="0" y="0" width="{w}" height="{h}" fill="var(--f-face)"/>{body}</svg>')

def text(x, y, s, size=12, fill='var(--f-ink)', anchor='start', weight=400, family=None, extra=''):
    fam = f' font-family="{family}"' if family else ''
    return (f'<text x="{x:.1f}" y="{y:.1f}" font-size="{size}" fill="{fill}" text-anchor="{anchor}" '
            f'font-weight="{weight}"{fam} {extra}>{esc(s)}</text>')

def svg_for_docx(s):
    for k, v in LIGHT.items():
        s = s.replace(f'var({k})', v)
    return s.replace('style="max-width:100%;height:auto"', '')

ARROW = ('<defs><marker id="{id}" markerWidth="9" markerHeight="9" refX="8" refY="4.5" orient="auto">'
         '<path d="M0,0 L9,4.5 L0,9 z" fill="var(--f-dim)"/></marker></defs>')

def legend(b, x, y, items, size=11):
    for col, name in items:
        b.append(f'<rect x="{x}" y="{y - 9}" width="11" height="11" fill="{col}" rx="2"/>')
        b.append(text(x + 16, y, name, size, 'var(--f-dim)'))
        x += 16 + len(name) * size * 0.56 + 20
    return x

# ----------------------------------------------------------------- F1 the route
def fig_pipeline(st):
    W, H = 820, 300
    b = [ARROW.format(id='ap1'), text(24, 30, 'From C to safe Rust: two translations, and the output bytes judge both', 14, weight=600)]
    boxes = [('EPA SWMM 5.2.4', f'{st["c_files"]} .c files', f'{st["c_lines_c"]:,} lines of C'),
             ('c2rust', 'mechanical, unsafe', f'{st["c2r_fns"]:,} unsafe fns'),
             ('skeleton', 'types, Project,', 'stubs, rule book'),
             ('54 modules', 'translated one by one', 'in 10 parallel batches'),
             ('swmm5-rust', f'{st["rs_lines"]:,} lines', '0 unsafe')]
    x = 24; w = 136; gap = 20; y = 62
    for i, (t1, t2, t3) in enumerate(boxes):
        col = 'var(--f-d)' if i == 1 else 'var(--rs)' if i == 4 else 'var(--f-a)'
        b.append(f'<rect x="{x}" y="{y}" width="{w}" height="74" rx="8" fill="var(--f-soft)" stroke="{col}" stroke-width="1.5"/>')
        b.append(text(x + w / 2, y + 24, t1, 13, anchor='middle', weight=600))
        b.append(text(x + w / 2, y + 44, t2, 11, 'var(--f-dim)', 'middle'))
        b.append(text(x + w / 2, y + 60, t3, 11, 'var(--f-dim)', 'middle'))
        if i < len(boxes) - 1:
            b.append(f'<path d="M{x + w + 2} {y + 37} h{gap - 6}" stroke="var(--f-dim)" stroke-width="1.6" marker-end="url(#ap1)"/>')
        x += w + gap
    # gates under c2rust and under the result
    def gate(cx, label1, label2, ok):
        gy = 180
        b.append(f'<rect x="{cx - 92}" y="{gy}" width="184" height="58" rx="8" fill="var(--f-soft)" stroke="var(--f-c)" stroke-width="1.6"/>')
        b.append(text(cx, gy + 23, label1, 12, anchor='middle', weight=600))
        b.append(text(cx, gy + 41, label2, 11, 'var(--f-dim)', 'middle'))
        b.append(f'<path d="M{cx} {y + 76} v{gy - y - 80}" stroke="var(--f-dim)" stroke-width="1.6" marker-end="url(#ap1)"/>')
    gate(24 + 1 * (w + gap) + w / 2, 'gate: c2rust vs EPA', st['c2r_gate'], True)
    gate(24 + 4 * (w + gap) + w / 2 - 30, 'gate: Rust vs EPA', st['rs_gate'], True)
    b.append(text(24, 266, 'The c2rust crate is the C, statement for statement, with every pointer kept. It is the reference the safe translation', 11, 'var(--f-dim)'))
    b.append(text(24, 282, 'was checked against step by step, and it never ships. Every byte of output either matches EPA or is explained.', 11, 'var(--f-dim)'))
    return svg(W, H, ''.join(b), 'The translation pipeline')

# ----------------------------------------------------------------- F2 where unsafe lives
def fig_unsafe(st):
    W, H = 820, 290
    L, R = 250, 780
    b = [text(24, 30, 'Where the unchecked code is', 14, weight=600),
         text(24, 48, 'Bar length: lines of source. Dark part: code the compiler cannot check for memory safety.', 11, 'var(--f-dim)')]
    rows = [('EPA SWMM 5.2.4 (C)', st['c_lines'], st['c_lines'], f'all of it: C has no safe subset ({st["c_files"]} .c files and {st["c_other"]} headers and tables)', 'var(--epa)'),
            ('c2rust translation', st['c2r_lines'], st['c2r_lines'], f'{st["c2r_fns"]:,} of {st["c2r_fns"]:,} functions unsafe; {st["c2r_statics"]:,} static mut', 'var(--f-d)'),
            ('swmm5-rust engine', st['rs_lines'], 0, '#![forbid(unsafe_code)]: none, and none allowed', 'var(--rs)'),
            ('C interface for callers', st['capi_lines'], st['capi_lines'], f'{st["capi_unsafe"]} unsafe blocks: pointers in, slices out', 'var(--f-e)')]
    mx = max(r[1] for r in rows)
    X = lambda v: (R - L) * v / mx
    for i, (name, n, u, note, col) in enumerate(rows):
        y = 74 + i * 50
        b.append(text(24, y + 15, name, 12, weight=600))
        b.append(text(24, y + 31, f'{n:,} lines', 11, 'var(--f-dim)'))
        b.append(f'<rect x="{L}" y="{y + 2}" width="{max(X(n), 2):.1f}" height="20" rx="3" fill="var(--f-soft)" stroke="{col}" stroke-width="1.4"/>')
        if u:
            b.append(f'<rect x="{L}" y="{y + 2}" width="{max(X(u), 3):.1f}" height="20" rx="3" fill="{col}"/>')
        b.append(text(L, y + 38, note, 11, 'var(--f-dim)'))
    b.append(text(24, 280, 'The interface is the one place a C or JavaScript caller hands the engine raw pointers; everything behind it is checked.', 11, 'var(--f-dim)'))
    return svg(W, H, ''.join(b), 'Unsafe code by source tree')

# ----------------------------------------------------------------- F3 the number that would not fit
def fig_cast():
    W, H = 820, 330
    L, R, T, B = 90, 560, 70, 270
    lo, hi = 1.0e9, 4.0e9          # x: the double being converted
    ylo, yhi = -2.3e9, 2.3e9       # y: the int that comes out
    X = lambda v: L + (R - L) * (v - lo) / (hi - lo)
    Y = lambda v: B - (B - T) * (v - ylo) / (yhi - ylo)
    I32 = 2147483647
    b = [text(24, 28, 'One conversion, three answers: (int)x for a double x past the largest int', 14, weight=600),
         text(24, 46, 'In C the result is undefined; each machine gives its own. Rust\'s "as" saturates. The engine must match x86.', 11, 'var(--f-dim)')]
    for v in [-2e9, -1e9, 0, 1e9, 2e9]:
        b.append(f'<line x1="{L}" y1="{Y(v):.1f}" x2="{R}" y2="{Y(v):.1f}" stroke="var(--f-line)" stroke-width="{1.4 if v == 0 else 0.7}"/>')
        b.append(text(L - 8, Y(v) + 4, f'{v / 1e9:+.0f}e9' if v else '0', 10, 'var(--f-dim)', 'end'))
    for v in [1e9, 2e9, 3e9, 4e9]:
        b.append(text(X(v), B + 18, f'{v / 1e9:.0f}e9', 10, 'var(--f-dim)', 'middle'))
    b.append(f'<line x1="{X(I32):.1f}" y1="{T - 8}" x2="{X(I32):.1f}" y2="{B}" stroke="var(--f-dim)" stroke-dasharray="3 3"/>')
    b.append(text(X(I32) + 4, T - 12, '2³¹ − 1 = 2,147,483,647', 10, 'var(--f-dim)'))
    b.append(text((L + R) / 2, B + 36, 'x, the double being converted', 11, 'var(--f-dim)', 'middle'))
    # x86 cvttsd2si: identity then INT_MIN
    b.append(f'<path d="M{X(lo):.1f} {Y(lo):.1f} L{X(I32):.1f} {Y(I32):.1f}" stroke="var(--f-ink)" stroke-width="2.4" fill="none"/>')
    b.append(f'<path d="M{X(I32):.1f} {Y(-I32 - 1):.1f} L{X(hi):.1f} {Y(-I32 - 1):.1f}" stroke="var(--epa)" stroke-width="3" fill="none"/>')
    b.append(f'<path d="M{X(I32):.1f} {Y(I32):.1f} L{X(hi):.1f} {Y(I32):.1f}" stroke="var(--f-d)" stroke-width="3" stroke-dasharray="7 5" fill="none"/>')
    lx = 590
    for i, (col, dash, t1, t2) in enumerate([
            ('var(--epa)', '', 'x86-64 (cvttsd2si), and the', 'wasm build: −2,147,483,648'),
            ('var(--f-d)', '7 5', 'Rust "as", ARM64:', 'saturate at +2,147,483,647'),
            ('var(--rs)', '2 4', 'swmm5-rust ccast::f64_to_i32:', 'x86\'s answer, on purpose')]):
        y = 92 + i * 52
        b.append(f'<line x1="{lx}" y1="{y}" x2="{lx + 30}" y2="{y}" stroke="{col}" stroke-width="3" {"stroke-dasharray=" + chr(34) + dash + chr(34) if dash else ""}/>')
        b.append(text(lx + 38, y + 4, t1, 11))
        b.append(text(lx + 38, y + 20, t2, 11, 'var(--f-dim)'))
    b.append(f'<path d="M{X(I32):.1f} {Y(-I32 - 1) - 6:.1f} L{X(hi):.1f} {Y(-I32 - 1) - 6:.1f}" stroke="var(--rs)" stroke-width="2" stroke-dasharray="2 4" fill="none"/>')
    b.append(text(24, 330, 'gage.c line 251: a one-point rainfall series makes Tseries[k].dxMin*SECperDAY about 8.6e14. On x86 the interval becomes negative and no', 11, 'var(--f-dim)'))
    b.append(text(24, 346, 'warning is written; a saturating cast makes it huge, and WARNING 09 appears. 12 of the 221 test reports changed until ccast.', 11, 'var(--f-dim)'))
    return svg(W, H + 28, ''.join(b), 'Float to integer conversion on three machines')

# ----------------------------------------------------------------- F4 one bit
def fig_ulp():
    W, H = 820, 300
    b = [ARROW.format(id='ap4'), text(24, 28, 'A round trip that does not come home: FILLED_CIRCULAR, 0.91 ft pipe with 0.41 ft of sediment', 14, weight=600),
         text(24, 46, 'filled_circ_getAofY() adds the sediment back to the link\'s full depth, computes, and subtracts it again, through a pointer to the link.', 11, 'var(--f-dim)')]
    rows = [('yFull = 0.91 − 0.41', '0.5', '0x3FE0000000000000', 'set once, in xsect_setParams'),
            ('yFull += yBot', '0.9099999999999999', '0x3FED1EB851EB851E', 'not 0.91: 0.91 has no exact binary form'),
            ('yFull −= yBot', '0.49999999999999994', '0x3FDFFFFFFFFFFFFF', 'one bit below where it started'),
            ('next call: += then −=', '0.49999999999999994', '0x3FDFFFFFFFFFFFFF', 'stable from here on')]
    for i, (op, val, bits, note) in enumerate(rows):
        y = 78 + i * 46
        col = 'var(--f-d)' if i == 2 else 'var(--f-ink)'
        b.append(f'<rect x="24" y="{y - 16}" width="772" height="38" rx="6" fill="var(--f-soft)" stroke="{"var(--f-d)" if i == 2 else "var(--f-line)"}"/>')
        b.append(text(36, y + 7, op, 12, weight=600, family=MONO))
        b.append(text(250, y + 7, val, 12, col, weight=600 if i == 2 else 400, family=MONO))
        b.append(text(440, y + 7, bits, 11, 'var(--f-dim)', family=MONO))
        b.append(text(600, y + 7, note, 11, 'var(--f-dim)'))
    b.append(text(24, 270, 'C changes Link[j].xsect itself, so the next time step sees the new value. A Rust copy would not, and the output would drift from', 11, 'var(--f-dim)'))
    b.append(text(24, 286, 'EPA\'s by that bit. In swmm5-rust the edit is written back to the link, so the bit is lost exactly where EPA loses it.', 11, 'var(--f-dim)'))
    return svg(W, H, ''.join(b), 'Floating-point round trip in FILLED_CIRCULAR geometry')

# ----------------------------------------------------------------- F5 storage order
def fig_storage(ser):
    """ser: dict with hours, A_S1, A_S2, B_S1, B_S2 (node depth, ft)."""
    W, H = 820, 360
    L, R, T, B = 70, 600, 70, 300
    hrs = ser['hours']; x1 = 6.0
    n = sum(1 for h in hrs if h <= x1 + 1e-9)
    ymax = max(max(ser[k][:n]) for k in ('A_S1', 'A_S2', 'B_S1', 'B_S2'))
    ymax = math.ceil(ymax / 2) * 2
    X = lambda v: L + (R - L) * v / x1
    Y = lambda v: B - (B - T) * v / ymax
    b = [text(24, 28, 'Same network, two section orders: depth in storage unit S2 (pyramid) and S1 (funnel)', 14, weight=600),
         text(24, 46, 'Kinematic-wave routing, identical inflows. EPA 5.2.4; swmm5-rust and swmm5-cplus write the same bytes for each order.', 11, 'var(--f-dim)')]
    for v in range(0, ymax + 1, 2):
        b.append(f'<line x1="{L}" y1="{Y(v):.1f}" x2="{R}" y2="{Y(v):.1f}" stroke="var(--f-line)" stroke-width="0.8"/>')
        b.append(text(L - 8, Y(v) + 4, str(v), 10, 'var(--f-dim)', 'end'))
    for v in range(0, 7):
        b.append(text(X(v), B + 16, str(v), 10, 'var(--f-dim)', 'middle'))
    b.append(text((L + R) / 2, B + 34, 'hours', 11, 'var(--f-dim)', 'middle'))
    b.append(text(18, (T + B) / 2, 'depth, ft', 11, 'var(--f-dim)', 'middle', extra=f'transform="rotate(-90 18 {(T + B) / 2})"'))
    def line(key, col, w, dash=''):
        pts = ' '.join(f'{X(hrs[k]):.1f},{Y(ser[key][k]):.1f}' for k in range(n))
        b.append(f'<polyline points="{pts}" fill="none" stroke="{col}" stroke-width="{w}" {"stroke-dasharray=" + chr(34) + dash + chr(34) if dash else ""} stroke-linejoin="round"/>')
    line('B_S2', 'var(--f-c)', 3)
    line('B_S1', 'var(--f-dim)', 2)
    line('A_S2', 'var(--f-d)', 2.2, '7 5')
    pa, pb = max(ser['A_S2']), max(ser['B_S2'])
    lx = 620
    for i, (col, dash, t1, t2) in enumerate([
            ('var(--f-c)', '', 'S2, storage first (order B)', f'peak {pb:.2f} ft: its own shape'),
            ('var(--f-d)', '7 5', 'S2, outfall listed first (A)', f'peak {pa:.2f} ft: S1\'s shape'),
            ('var(--f-dim)', '', 'S1, either order', 'the same in both')]):
        y = 92 + i * 50
        b.append(f'<line x1="{lx}" y1="{y}" x2="{lx + 28}" y2="{y}" stroke="{col}" stroke-width="3" {"stroke-dasharray=" + chr(34) + dash + chr(34) if dash else ""}/>')
        b.append(text(lx + 34, y + 4, t1, 11))
        b.append(text(lx + 34, y + 20, t2, 11, 'var(--f-dim)'))
    b.append(text(24, 344, f'In order A the red S2 curve lies on S1\'s, within {ser["maxd_a"]:.1e} ft: storage_getVolDiff() hands Node[k] the storage index k, and node 1 is the outfall, whose subIndex points at S1.', 11, 'var(--f-dim)'))
    return svg(W, H, ''.join(b), 'Storage depth in two section orders')

# ----------------------------------------------------------------- F6 native gate by folder
def fig_gate(folders, title_note, title='The native gate: every model run by EPA 5.2.4 and swmm5-rust, outputs compared byte for byte'):
    W = 820; rh = 16; x0 = 280; x1 = 640
    H = 80 + rh * len(folders) + 40
    b = [text(24, 28, title, 14, weight=600), text(24, 46, title_note, 11, 'var(--f-dim)')]
    mx = max(f['n'] for f in folders)
    sc = lambda v: (x1 - x0) * math.log10(1 + v) / math.log10(1 + mx)
    for i, f in enumerate(folders):
        y = 66 + i * rh
        b.append(text(24, y + 11, f['name'][:38], 10, family=MONO))
        w = sc(f['n'])
        b.append(f'<rect x="{x0}" y="{y + 2}" width="{w:.1f}" height="11" fill="var(--f-c)" rx="1.5"/>')
        bad = f['diff'] + f.get('ub', 0)
        if f.get('ub'):
            b.append(f'<rect x="{x0}" y="{y + 2}" width="{max(sc(f["ub"]), 3):.1f}" height="11" fill="var(--f-e)"/>')
        if f['diff']:
            b.append(f'<rect x="{x0 + (sc(f["ub"]) if f.get("ub") else 0):.1f}" y="{y + 2}" width="{max(sc(f["diff"]), 3):.1f}" height="11" fill="var(--f-d)"/>')
        if f.get('inc'):
            b.append(f'<rect x="{x0 + sc(f["n"] - f["inc"]):.1f}" y="{y + 2}" width="{max(w - sc(f["n"] - f["inc"]), 3):.1f}" height="11" fill="var(--f-b)" rx="1.5"/>')
        extra = ', '.join(([f'{f["ub"]} undefined in C'] if f.get('ub') else []) + ([f'{f["diff"]} different'] if f['diff'] else []) + ([f'{f["inc"]} inconclusive'] if f.get('inc') else []))
        b.append(text(x0 + w + 6, y + 11, f'{f["same"]}/{f["n"]}' + (f'  ({extra})' if extra else ''), 10, 'var(--f-dim)'))
    yb = 66 + rh * len(folders) + 22
    legend(b, 24, yb, [('var(--f-c)', 'byte-identical .out and .rpt'), ('var(--f-e)', 'EPA reads or writes memory it does not own'),
                       ('var(--f-d)', 'different'), ('var(--f-b)', 'inconclusive (both hit the time cap)')])
    return svg(W, H, ''.join(b), 'Native gate results by folder')

# ----------------------------------------------------------------- F7 wasm agreement
def fig_wasm(rows, n, ub=0):
    """rows: list of (label, sublabel, same_count, color)"""
    W, H = 820, 110 + 46 * len(rows)
    L, R = 330, 760
    b = [text(24, 28, f'In the browser: {n} models through three engines compiled to WebAssembly', 14, weight=600),
         text(24, 46, 'Each bar: models whose .out matches byte for byte and whose .rpt matches line for line (wall-clock lines removed)'
              + (f'; the {ub} decks undefined in C are left out.' if ub else '.'), 11, 'var(--f-dim)')]
    X = lambda v: (R - L) * v / n
    for i, (lab, sub, same, col) in enumerate(rows):
        y = 70 + i * 46
        b.append(text(24, y + 13, lab, 12, weight=600))
        b.append(text(24, y + 29, sub, 11, 'var(--f-dim)'))
        b.append(f'<rect x="{L}" y="{y}" width="{R - L}" height="18" rx="3" fill="var(--f-soft)" stroke="var(--f-line)"/>')
        b.append(f'<rect x="{L}" y="{y}" width="{max(X(same), 2):.1f}" height="18" rx="3" fill="{col}"/>')
        b.append(text(L, y + 33, f'{same} of {n}', 11, 'var(--f-dim)'))
    b.append(text(24, H - 14, 'The platform row is EPA\'s own C against itself, GCC and glibc against Clang and wasi-libc: the same source, so a difference there is the platform\'s.', 11, 'var(--f-dim)'))
    return svg(W, H, ''.join(b), 'WebAssembly parity of three engines')

# ----------------------------------------------------------------- F8 six decks
def fig_memdecks(rows):
    """rows: list of dict(name, what, epa_native, epa_wasm, rs_native, rs_wasm) each a (text, kind) kind in ok/bad/ub."""
    W = 900; rh = 50; H = 96 + rh * len(rows) + 48
    cols = [('EPA, native x86', 'epa_native'), ('EPA, wasm', 'epa_wasm'), ('swmm5-rust, native', 'rs_native'), ('swmm5-rust, wasm', 'rs_wasm')]
    cx = [372, 504, 636, 768]
    b = [text(24, 28, 'Six decks where EPA\'s C touches memory it does not own', 14, weight=600),
         text(24, 46, 'From the OpenSWMM 2026 code review. C\'s answer depends on what happens to be in memory; safe Rust stops at the first bad index.', 11, 'var(--f-dim)')]
    for (lab, _), x in zip(cols, cx):
        b.append(text(x + 55, 76, lab, 11, 'var(--f-dim)', 'middle', 600))
    fill = {'ok': 'var(--f-c)', 'bad': 'var(--f-d)', 'ub': 'var(--f-b)', 'stop': 'var(--rs)'}
    for i, r in enumerate(rows):
        y = 92 + i * rh
        b.append(f'<line x1="24" y1="{y - 6}" x2="{W - 24}" y2="{y - 6}" stroke="var(--f-line)" stroke-width="0.8"/>')
        b.append(text(24, y + 12, r['name'], 11, weight=600, family=MONO))
        what = r['what'].split(': ', 1)
        for j, wl in enumerate(what):
            b.append(text(24, y + 27 + 12 * j, wl + (':' if j == 0 and len(what) > 1 else ''), 10, 'var(--f-dim)'))
        for (lab, key), x in zip(cols, cx):
            t, k = r[key]
            b.append(f'<rect x="{x}" y="{y}" width="110" height="30" rx="5" fill="var(--f-soft)" stroke="{fill[k]}" stroke-width="1.6"/>')
            parts = t.split('|')
            for j, p in enumerate(parts):
                b.append(text(x + 55, y + (19 if len(parts) == 1 else 13 + 12 * j), p, 10, 'var(--f-ink)', 'middle'))
    yb = 92 + rh * len(rows) + 14
    legend(b, 24, yb, [('var(--f-b)', 'runs on, output depends on memory contents'), ('var(--f-d)', 'killed by the operating system or the VM')], 10)
    legend(b, 24, yb + 18, [('var(--rs)', 'stops with a message naming the line'), ('var(--f-c)', 'proper error report')], 10)
    return svg(W, H, ''.join(b), 'Memory-error decks in four builds')

# ----------------------------------------------------------------- F9 borrow patterns
def fig_borrow():
    W, H = 820, 346
    b = [text(24, 28, 'The compiler\'s one objection, and the four ways around it', 14, weight=600),
         text(24, 46, 'f(prj, &mut prj.Link[j].xsect) is refused: a function cannot hold the whole model and a piece of it, both writable, at once.', 11, 'var(--f-dim)')]
    pats = [('1  pass an index', 'f(prj, j)', 'the callee looks the link up itself', 'most functions: C already passed j'),
            ('2  copy (and write back)', 'let xs = prj.Link[j].xsect;\nf(prj, &xs);', 'TXsect, inflows, totals are Copy', 'FILLED_CIRCULAR needs the write-back'),
            ('3  take and restore', 'let q = mem::take(&mut ...);\nf(prj, &mut q); ... = q;', 'for vectors: no copy at all', '18 places; callee must not read it'),
            ('4  split the borrow', 'g(&mut prj.infil,\n  &mut prj.Storage[k].exfil)', 'two disjoint fields', 'when no whole-model access needed')]
    for i, (t, code, why, note) in enumerate(pats):
        x = 24 + (i % 2) * 398; y = 70 + (i // 2) * 128
        b.append(f'<rect x="{x}" y="{y}" width="380" height="114" rx="8" fill="var(--f-soft)" stroke="var(--rs)" stroke-width="1.2"/>')
        b.append(text(x + 14, y + 22, t, 13, weight=600))
        for j, ln in enumerate(code.split('\n')):
            b.append(text(x + 14, y + 44 + 15 * j, ln.replace(' ', '\u00a0'), 11, 'var(--f-ink)', family=MONO))
        b.append(text(x + 14, y + 84, why, 11, 'var(--f-dim)'))
        b.append(text(x + 14, y + 100, note, 11, 'var(--f-dim)'))
    b.append(text(24, 336, 'Never hold a reference into the model across a call that takes the model. That one rule, enforced by the compiler, is what C cannot promise.', 11, 'var(--f-dim)'))
    return svg(W, H, ''.join(b), 'Borrowing patterns')

# ----------------------------------------------------------------- F10 linked list to Vec
def fig_lists():
    W, H = 820, 250
    b = [ARROW.format(id='ap10'), text(24, 28, 'A linked list becomes a vector in the order C walks it', 14, weight=600),
         text(24, 46, 'Node[j].extInflow: inflow.c prepends each new inflow, so the last one read is visited first. Sums are taken in that order.', 11, 'var(--f-dim)')]
    # C list
    b.append(text(24, 84, 'C', 12, weight=600))
    xs = [140, 300, 460]
    labs = ['FLOW (3rd read)', 'BOD (2nd read)', 'TSS (1st read)']
    b.append(text(50, 104, 'head', 11, 'var(--f-dim)', family=MONO))
    b.append(f'<path d="M86 100 H{xs[0] - 4}" stroke="var(--f-dim)" stroke-width="1.5" marker-end="url(#ap10)"/>')
    for i, (x, l) in enumerate(zip(xs, labs)):
        b.append(f'<rect x="{x}" y="86" width="130" height="28" rx="5" fill="var(--f-soft)" stroke="var(--epa)"/>')
        b.append(text(x + 65, 104, l, 11, anchor='middle'))
        if i < 2:
            b.append(f'<path d="M{x + 132} 100 H{xs[i + 1] - 4}" stroke="var(--f-dim)" stroke-width="1.5" marker-end="url(#ap10)"/>')
    b.append(text(xs[2] + 140, 104, '→ NULL', 11, 'var(--f-dim)', family=MONO))
    b.append(text(24, 150, 'Rust', 12, weight=600))
    for i, (x, l) in enumerate(zip(xs, labs)):
        b.append(f'<rect x="{x}" y="132" width="130" height="28" rx="5" fill="var(--f-soft)" stroke="var(--rs)"/>')
        b.append(text(x + 65, 150, f'[{i}] ' + l, 11, anchor='middle'))
    b.append(text(xs[2] + 140, 150, 'Vec<TExtInflow>', 11, 'var(--f-dim)', family=MONO))
    b.append(text(24, 196, 'each new item:  C  item->next = head; head = item;      Rust  v.insert(0, item);', 12, family=MONO))
    b.append(text(24, 226, 'Floating-point addition is not associative, so visiting the same inflows in another order can change the last bit of a node\'s inflow.', 11, 'var(--f-dim)'))
    return svg(W, H, ''.join(b), 'Linked list to vector')

# ----------------------------------------------------------------- F11 cio checks
def fig_cio(rows):
    """rows: list of (label, count, note)"""
    W, H = 820, 90 + 44 * len(rows)
    L, R = 300, 640
    mx = max(r[1] for r in rows)
    X = lambda v: (R - L) * math.log10(v) / math.log10(mx)
    b = [text(24, 28, 'C\'s text routines rewritten in safe Rust, and checked against glibc itself', 14, weight=600),
         text(24, 46, 'Each check: the same format or text given to glibc and to swmm5-rust\'s cio module; the bytes must be equal. Log scale.', 11, 'var(--f-dim)')]
    for i, (lab, n, note) in enumerate(rows):
        y = 66 + i * 44
        b.append(text(24, y + 14, lab, 12, weight=600))
        b.append(text(24, y + 30, note, 10, 'var(--f-dim)'))
        b.append(f'<rect x="{L}" y="{y + 2}" width="{max(X(n), 3):.1f}" height="18" rx="3" fill="var(--rs)"/>')
        b.append(text(L + max(X(n), 3) + 8, y + 15, f'{n:,} identical', 11, 'var(--f-dim)'))
    return svg(W, H, ''.join(b), 'cio checks against glibc')

# ----------------------------------------------------------------- F12 timing
def fig_timing(groups, title, sub):
    """groups: list of dict(label, color, rows=[(model, ratio, floor)]) -- ratio = engine time / EPA time."""
    models = [r[0] for r in groups[0]['rows']]
    W = 820; rh = 22; H = 110 + rh * len(models) + 40
    L, R, T = 280, 780, 70
    allv = [r[1] for g in groups for r in g['rows']] + [r[2] for g in groups for r in g['rows']] + [1.0]
    lo = math.floor(min(allv) * 10) / 10 - 0.05; hi = math.ceil(max(allv) * 10) / 10 + 0.05
    X = lambda v: L + (R - L) * (v - lo) / (hi - lo)
    B = T + rh * len(models)
    b = [text(24, 28, title, 14, weight=600), text(24, 46, sub, 11, 'var(--f-dim)')]
    step = 0.1 if hi - lo <= 1.2 else 0.25 if hi - lo <= 3 else 0.5
    v = math.ceil(lo / step) * step
    while v <= hi + 1e-9:
        b.append(f'<line x1="{X(v):.1f}" y1="{T - 6}" x2="{X(v):.1f}" y2="{B + 4}" stroke="var(--f-line)" stroke-width="{1.8 if abs(v - 1) < 1e-9 else 0.7}"/>')
        b.append(text(X(v), B + 18, f'{v:.2f}', 10, 'var(--f-dim)', 'middle'))
        v += step
    for i, m in enumerate(models):
        y = T + i * rh + rh / 2
        b.append(text(24, y + 4, m[:36], 10, family=MONO))
        fl = groups[0]['rows'][i][2]
        f0, f1 = sorted([1.0, fl])
        b.append(f'<rect x="{X(f0):.1f}" y="{y - 6}" width="{max(2, X(f1) - X(f0)):.1f}" height="12" fill="var(--f-soft)" stroke="var(--f-line)"/>')
        for g in groups:
            r = g['rows'][i]
            b.append(f'<circle cx="{X(r[1]):.1f}" cy="{y}" r="5" fill="{g["color"]}"/>')
    legend(b, 24, B + 40, [(g['color'], g['label']) for g in groups] + [('var(--f-soft)', 'noise floor: EPA timed twice')])
    return svg(W, H, ''.join(b), 'Run time relative to EPA')
