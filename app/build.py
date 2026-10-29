#!/usr/bin/env python3
"""Assemble swmm5-rust.html (one file) and doc/out/essay.json from measured results.

Everything the page states as a number is computed here from the repo's results/
files, the source trees, or a run made during the build.
"""
import os, re, json, math, base64, hashlib, collections, datetime, html, subprocess, sys, glob, tempfile, statistics
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, 'doc')); sys.path.insert(0, HERE)
import figures as F
import essay as E
from outread import Out

REPO = os.path.dirname(HERE)
RES = f'{REPO}/results'
CORPUS = '/home/claude/work/corpus1729'
EPA_SRC = '/home/claude/work/epa524/src/solver'
CPP_SRC = '/home/claude/swmmbobswmm6/swmm5-cplus/src/engine'
RS_SRC = f'{REPO}/crates/engine/src'
C2R_SRC = f'{REPO}/crates/c2rust-baseline/src/engine'
W = f'{REPO}/build/wasm'
TMP = '/tmp/claude-0/appbuild'; os.makedirs(TMP, exist_ok=True)
REF = f'{REPO}/ref/runswmm'; RSBIN = f'{REPO}/ref/runswmm-rs-gate'

def jl(p): return [json.loads(l) for l in open(p)] if os.path.exists(p) else []
def J(p): return json.load(open(p)) if os.path.exists(p) else None
def rd(p, enc='latin-1'): return open(p, encoding=enc).read()
def lines_of(paths): return sum(sum(1 for _ in open(p, encoding='latin-1')) for p in paths)

# ------------------------------------------------------------------ source census
c_files = sorted(glob.glob(f'{EPA_SRC}/*.c'))
c_all = c_files + sorted(glob.glob(f'{EPA_SRC}/*.h')) + glob.glob(f'{EPA_SRC}/*.dat')
rs_files = sorted(glob.glob(f'{RS_SRC}/*.rs'))
c2r_files = sorted(glob.glob(f'{C2R_SRC}/*.rs'))
def c2r_census():
    tot = uns = 0; statics = 0
    for f in c2r_files:
        s = rd(f)
        for m in re.finditer(r'^\s*(pub\s+)?(unsafe\s+)?(extern\s+"C"\s+)?fn\s+(\w+)\s*\(', s, re.M):
            j = s.find('{', m.end()); k = s.find(';', m.end())
            if k != -1 and (j == -1 or k < j): continue
            tot += 1; uns += bool(m.group(2))
        inext = False; depth = 0
        for ln in s.split('\n'):
            st = ln.strip()
            if re.match(r'(unsafe\s+)?extern\s+"C"\s*\{', st): inext = True; depth = 0
            if inext:
                depth += ln.count('{') - ln.count('}')
                if depth <= 0: inext = False
                continue
            if re.match(r'(pub\s+)?static\s+mut\s', st): statics += 1
    return tot, uns, statics
c2r_fns, c2r_unsafe, c2r_statics = c2r_census()
rs_fns = sum(len(re.findall(r'^\s*(pub(\([a-z]+\))?\s+)?fn\s+\w+', rd(f), re.M)) for f in rs_files)
rs_unsafe_tokens = sum(len(re.findall(r'\bunsafe\s*(\{|fn|impl)', rd(f))) for f in rs_files)
assert rs_unsafe_tokens == 0, 'unsafe found in the engine'
capi = rd(f'{REPO}/crates/capi/src/lib.rs')
pat = lambda p: sum(len(re.findall(p, rd(f))) for f in rs_files if not f.endswith(('ccast.rs', 'cio.rs', 'macros.rs')))
ST = dict(c_files=len(c_files), c_lines=lines_of(c_all), c_lines_c=lines_of(c_files), c_other=len(c_all) - len(c_files), c2r_lines=lines_of(c2r_files), c2r_fns=c2r_fns, c2r_unsafe=c2r_unsafe,
          c2r_statics=c2r_statics, rs_lines=lines_of(rs_files), rs_files=len(rs_files), rs_fns=rs_fns,
          capi_lines=capi.count('\n'), capi_unsafe=len(re.findall(r'unsafe\s*\{', capi)),
          casts=pat(r'\bf(64|32)_to_i(8|32|64)\('), prepends=pat(r'\.insert\(0,'), takes=pat(r'mem::take\('),
          cprintf=pat(r'cprintf!\('), closures=pat(r'&mut dyn FnMut'), xs_get=pat(r'\blinkXsect_\w+\('),
          project_fields=len(re.findall(r'^\s*pub \w+:', rd(f'{RS_SRC}/globals.rs'), re.M)),
          project_mods=len(re.findall(r'^\s*pub \w+: crate::\w+::\w+State', rd(f'{RS_SRC}/globals.rs'), re.M)))
ST['project_globals'] = ST['project_fields'] - ST['project_mods']
assert c2r_fns == c2r_unsafe

# ------------------------------------------------------------------ gates
UB = {'IO-23_no-gages.inp', 'CRASH-02_xsect-before-weir.inp', 'CRASH-03_empty-curve.inp', 'CRASH-04_quoted-shape.inp',
      'CRASH-08_expr-bad-option.inp', 'CRASH-08_expr-duplicate-id.inp',
      # corpus decks checked one by one: the same C fault as a deck above
      'CRASH-02_orifice-bad-node.inp',   # CRASH-02's other deck: ERROR 209, then Conduit[0] of an empty array is written (link.c:190)
      }
def classify(r):
    n = r['inp'].split('/')[-1]
    if str(r.get('rc_ref')) == 'timeout' or str(r.get('rc_test')) == 'timeout': return 'inc'
    if r['out_same'] and r['rpt_same']: return 'same'
    if n in UB: return 'ub'
    # EPA killed by a signal (SIGSEGV, SIGABRT): its C touched memory it does not own, so there is no EPA answer.
    # Listed by name on the Verify tab; a Rust panic where EPA ran normally is NOT in this class.
    if isinstance(r.get('rc_ref'), int) and r['rc_ref'] < 0: return 'ub'
    return 'diff'
def fold(rows):
    fo = collections.OrderedDict()
    for r in sorted(rows, key=lambda r: r['inp']):
        k = r['inp'].split('/')[0]
        f = fo.setdefault(k, dict(name=k, n=0, same=0, inc=0, diff=0, ub=0))
        f['n'] += 1; f[classify(r)] += 1
    return list(fo.values())
def tally(rows):
    c = collections.Counter(classify(r) for r in rows)
    return dict(n=len(rows), same=c['same'], ub=c['ub'], diff=c['diff'], inc=c['inc'])
RERUN = {r['inp']: r for r in jl(f'{RES}/rs_timeouts_rerun.jsonl')}   # timeouts rerun alone with a long cap
def merged(rows): return [RERUN.get(r['inp'], r) if classify(r) == 'inc' else r for r in rows]
g221 = merged(jl(f'{RES}/rs_221_v2.jsonl')); T221 = tally(g221)
gfc = merged(jl(f'{RES}/rs_filled68.jsonl')); TFC = tally(gfc)
gco = merged(jl(f'{RES}/rs_corpus.jsonl')); TCO = tally(gco)
UB_CORPUS = sorted({r['inp'].split('/')[-1] for r in gco if classify(r) == 'ub'})
DIFF_LIST = sorted({r['inp'] for r in g221 + gfc + gco if classify(r) == 'diff'})
INC_LIST = sorted({r['inp'] for r in g221 + gfc + gco if classify(r) == 'inc'})
N_FC = sum(1 for l in open(f'{RES}/filled68.txt') if l.strip())
N_CO = sum(1 for l in open(f'{RES}/corpus_set.txt') if l.strip())
diffs_all = [r['inp'] for r in g221 + gfc + gco if classify(r) == 'diff']
c2r_221 = jl(f'{RES}/c2rust_casts_221.jsonl'); c2r_cov = jl(f'{RES}/c2rust_cov.jsonl')
C2R_SAME = sum(1 for r in c2r_221 if r['out_same'] and r['rpt_same'])
C2R_W09 = sum(1 for r in c2r_cov if not r['rpt_same'] and 'WARNING 09' in str(r.get('rpt_first_diff')))
wz_all = [r for r in jl(f'{RES}/wasm3_221.jsonl') if not r.get('timeout')]
W_UB = sum(1 for r in wz_all if r['inp'].split('/')[-1] in UB)
WN_ALL = len(wz_all)
wz = [r for r in wz_all if r['inp'].split('/')[-1] not in UB]       # undefined-behaviour decks are counted apart
WN = len(wz)
WS = dict(rs=sum(1 for r in wz if r['rs_out'] and r['rs_rpt']), cp=sum(1 for r in wz if r['cp_out'] and r['cp_rpt']),
          plat=sum(1 for r in wz if r['plat_out'] and r['plat_rpt']), rsn=sum(1 for r in wz if r['rs_native_out'] and r['rs_native_rpt']))
w_rs_bad = [r['inp'].split('/')[-1] for r in wz if not (r['rs_out'] and r['rs_rpt'])]
w_plat_bad = [r['inp'].split('/')[-1] for r in wz if not (r['plat_out'] and r['plat_rpt'])]
w_rs_bad_notub = [n for n in w_rs_bad if n not in UB]
fprefix = J(f'{RES}/filled_prefix.json')

# ------------------------------------------------------------------ runs made now: storage order, decks
def run_native(exe, inp, tag):
    rpt, out = f'{TMP}/{tag}.rpt', f'{TMP}/{tag}.out'
    for p in (rpt, out):
        if os.path.exists(p): os.remove(p)
    r = subprocess.run([exe, inp, rpt, out], cwd=os.path.dirname(inp), capture_output=True, timeout=300)
    return r.returncode, rpt, out
SO = {}
for v in 'AB':
    inp = f'{REPO}/tests/models/storage_order_{v}.inp'
    rc, rpt, out = run_native(REF, inp, 'so' + v)
    rc2, rpt2, out2 = run_native(RSBIN, inp, 'sr' + v)
    o = Out(out)
    SO[v] = dict(out=o, same=open(out, 'rb').read() == open(out2, 'rb').read())
hrs = [(k + 1) * SO['A']['out'].step / 3600 for k in range(SO['A']['out'].nPer)]
ser = dict(hours=hrs)
for v in 'AB':
    o = SO[v]['out']
    for s in ('S1', 'S2'):
        ser[f'{v}_{s}'] = o.series('node', o.index('node', s), 0)
S2_eq_S1_A = ser['A_S2'] == ser['A_S1']
SO_MAXD_A = max(abs(a - b) for a, b in zip(ser['A_S1'], ser['A_S2']))
SO_MAXD_B = max(abs(a - b) for a, b in zip(ser['B_S1'], ser['B_S2']))
SO_PEAK = {k: max(ser[k]) for k in ('A_S1', 'A_S2', 'B_S1', 'B_S2')}
SO_SAME = SO['A']['same'] and SO['B']['same']
DECK_PATHS = {
    'IO-23_no-gages.inp': '4-io/IO-23-unit-hydrograph-without-gage/IO-23_no-gages.inp',
    'CRASH-02_xsect-before-weir.inp': '5-crashes/CRASH-02-xsection-before-link-heap-overflow/CRASH-02_xsect-before-weir.inp',
    'CRASH-03_empty-curve.inp': '5-crashes/CRASH-03-curve-without-points/CRASH-03_empty-curve.inp',
    'CRASH-04_quoted-shape.inp': '5-crashes/CRASH-04-curve-id-only-line-count-pass/CRASH-04_quoted-shape.inp',
    'CRASH-08_expr-bad-option.inp': '5-crashes/CRASH-08-rule-expression-freed-after-input-error/CRASH-08_expr-bad-option.inp',
    'CRASH-08_expr-duplicate-id.inp': '5-crashes/CRASH-08-rule-expression-freed-after-input-error/CRASH-08_expr-duplicate-id.inp'}
DECK_WHAT = {'IO-23_no-gages.inp': 'unit hydrograph, no rain gage: reads Gage[0] of an empty array',
             'CRASH-02_xsect-before-weir.inp': '[XSECTIONS] before its link: writes past the Conduit array',
             'CRASH-03_empty-curve.inp': 'curve with no points: follows a NULL pointer',
             'CRASH-04_quoted-shape.inp': 'quoted "Shape": writes past a zero-length Shape array',
             'CRASH-08_expr-bad-option.inp': 'bad option + EXPRESSION: walks a NULL array on close',
             'CRASH-08_expr-duplicate-id.inp': 'duplicate ID + EXPRESSION: walks a NULL array on close'}
dk_abs = [f'{CORPUS}/OpenSWMM_Code_Review_2026/{p}' for p in DECK_PATHS.values()]
wasm_runs = [json.loads(l) for l in subprocess.run(['node', f'{HERE}/run_decks.js', W] + dk_abs + [f'{REPO}/tests/models/lid_error_message.inp'],
                                                   capture_output=True, text=True, timeout=600).stdout.splitlines() if l.startswith('{')]
WR = {(r['model'], r['engine']): r for r in wasm_runs}
deck_rows = []
SIG = {-11: 'segfault', 139: 'segfault', -6: 'abort', 134: 'abort'}
for n, p in DECK_PATHS.items():
    inp = f'{CORPUS}/OpenSWMM_Code_Review_2026/{p}'
    rce, rpte, oute = run_native(REF, inp, 'de')
    rcr, rptr, outr = run_native(RSBIN, inp, 'dr')
    sz = lambda q: os.path.getsize(q) if os.path.exists(q) else 0
    def nat(rc, rpt, rust):
        if rc in SIG: return (f'{SIG[rc]}|report {sz(rpt)} bytes', 'bad')
        if rust and rc == 101: return ('panic, names|the line', 'stop')
        if rc == 0 and 'ERROR' not in rd(rpt): return (f'runs on|rc 0', 'ub')
        return (f'error report|rc {rc}', 'ok')
    def was(r, rust):
        if r['trap']:
            return ('panic, names|the line', 'stop') if rust and r['panic'] else ('trap: ' + ('out of|bounds' if 'bounds' in r['trap'] else r['trap'][:12]), 'bad')
        if r['err']: return (f'error report|rc {r["rc"]}', 'ok')
        return (f'runs on|rc {r["rc"]}', 'ub')
    deck_rows.append(dict(name=n.replace('.inp', ''), what=DECK_WHAT[n],
                          epa_native=nat(rce, rpte, False), rs_native=nat(rcr, rptr, True),
                          epa_wasm=was(WR[(n, 'epa')], False), rs_wasm=was(WR[(n, 'rust')], True),
                          panic=WR[(n, 'rust')]['panic'], text=rd(inp)))
LID = {k: WR[('lid_error_message.inp', k)] for k in ('epa', 'cplus', 'rust')}

# ------------------------------------------------------------------ timing (if measured)
bench_n = J(f'{RES}/bench_native.json'); bench_w = J(f'{RES}/bench_wasm.json')
def geo(v): return math.exp(sum(math.log(x) for x in v) / len(v))

# ------------------------------------------------------------------ numbers for text
def pct(a, b): return f'{100 * a / b:.1f}%' if b else '—'
NUM = dict(
    C_FILES=ST['c_files'], C_LINES=f"{ST['c_lines']:,}", C_LINES_C=f"{ST['c_lines_c']:,}", C_OTHER=ST['c_other'], CASTS_OTHER=ST['casts'] - 1, C2R_LINES=f"{ST['c2r_lines']:,}", C2R_FNS=f"{ST['c2r_fns']:,}",
    C2R_STATICS=f"{ST['c2r_statics']:,}", RS_LINES=f"{ST['rs_lines']:,}", RS_FNS=f"{ST['rs_fns']:,}", RS_FILES=ST['rs_files'],
    CAPI_LINES=ST['capi_lines'], CAPI_UNSAFE=ST['capi_unsafe'], CASTS=ST['casts'], PREPENDS=ST['prepends'], TAKES=ST['takes'],
    CPRINTF=f"{ST['cprintf']:,}", CLOSURES=ST['closures'], XS_GET=ST['xs_get'], PROJECT_FIELDS=ST['project_fields'], PROJECT_GLOBALS=ST['project_globals'], PROJECT_MODS=ST['project_mods'],
    N221=T221['n'], S221=T221['same'], UB221=T221['ub'], D221=T221['diff'],
    NFC=N_FC, DFC=TFC['n'], SFC=TFC['same'], NCO=f'{N_CO:,}', DCO=f"{TCO['n']:,}", SCO=f"{TCO['same']:,}", UBCO=TCO['ub'], DIFFCO=TCO['diff'], INCCO=TCO['inc'],
    C2R_SAME=C2R_SAME, C2R_W09=C2R_W09,
    WN=WN, WRS=WS['rs'], WCP=WS['cp'], WPLAT=WS['plat'], WRSN=WS['rsn'], WPLAT_BAD=WN - WS['plat'], W_UB=W_UB, WN_ALL=WN_ALL,
    WASM_STATE=('' if WN_ALL >= 221 else f' (the run had reached {WN_ALL} of the 221 when this was built)'),
    RUSTC_NATIVE='1.97', RUSTC_WASM='1.91',
    FP_DIFF=fprefix['differ'] if fprefix else 0, FP_VALS=f"{fprefix['values']:,}" if fprefix else '', FP_MAX=f"{fprefix['max_abs']:.1e}" if fprefix else '',
    SO_PA=f"{SO_PEAK['A_S2']:.2f}", SO_PB=f"{SO_PEAK['B_S2']:.2f}", SO_P1=f"{SO_PEAK['A_S1']:.2f}",
    SO_DIFF=f"{SO_PEAK['B_S2'] - SO_PEAK['A_S2']:.2f}", SO_MAXD_A=f"{SO_MAXD_A:.1e}", SO_MAXD_B=f"{SO_MAXD_B:.2f}",
)
GATE_CO_STATE = ('' if TCO['n'] >= N_CO else ' (still running when this was built)')
NUM['CO_STATE'] = GATE_CO_STATE
NUM['FC_RESULT'] = (f"{TFC['same']} of {TFC['n']} identical" + (f", {TFC['inc']} inconclusive (both runs hit the time cap)" if TFC['inc'] else '') + (f", {TFC['diff']} different" if TFC['diff'] else ', none different')
                    + ('' if TFC['n'] >= N_FC else f" ({TFC['n']} of {N_FC} run when this was built)"))
NUM['CO_DONE'] = 'all' if TCO['n'] >= N_CO else f"{TCO['n']:,} of the"
NUM['CORPUS_RESULT'] = (f"all {N_CO:,} run, {TCO['same']:,} identical" + (f", {TCO['ub']} undefined in C" if TCO['ub'] else '') + (f", {TCO['diff']} different" if TCO['diff'] else '') + (f", {TCO['inc']} inconclusive at the time cap" if TCO['inc'] else '')
                        if TCO['n'] >= N_CO else f"{TCO['n']:,} run when this was built, {TCO['same']:,} of them identical" if TCO['n'] else "still running when this was built")
if bench_n:
    NUM.update(dict(T_RS=f"{bench_n['summary']['rs_median']:.2f}", T_CP=f"{bench_n['summary']['cp_median']:.2f}",
                    T_FLOOR=f"±{100 * bench_n['summary']['floor_dev']:.0f}%", T_REPS=bench_n['summary']['reps'], T_N=bench_n['summary']['models']))
if bench_w:
    NUM.update(dict(TW_RS=f"{bench_w['summary']['rs_median']:.2f}", TW_CP=f"{bench_w['summary']['cp_median']:.2f}",
                    TW_FLOOR=f"±{100 * bench_w['summary']['floor_dev']:.0f}%"))
if not bench_n: NUM['NO_TIMING'] = ''
def sub(s):
    return re.sub(r'\{([A-Z0-9_]+)\}', lambda m: str(NUM[m.group(1)]) if m.group(1) in NUM else m.group(0), s)

# ------------------------------------------------------------------ code: C, C++, Rust side by side
def func(path, sig):
    """the function whose definition line contains sig, from that line to its closing brace"""
    L = rd(path).split('\n')
    i = next(k for k, l in enumerate(L) if sig in l and not l.rstrip().endswith(';'))
    depth = 0; started = False
    for k in range(i, len(L)):
        depth += L[k].count('{') - L[k].count('}')
        if '{' in L[k]: started = True
        if started and depth <= 0: return '\n'.join(L[i:k + 1]), i + 1
    raise SystemExit('unterminated ' + sig)
def excerpt(path, anchor, before, after):
    L = rd(path).split('\n')
    i = next(k for k, l in enumerate(L) if anchor in l)
    return '\n'.join(L[max(0, i - before):i + after + 1]), max(0, i - before) + 1
def ref(path, start, text):
    return dict(file=os.path.basename(path), line=start, text=text)
def F_(path, sig): t, s = func(path, sig); return ref(path, s, t)
def X_(path, a, b, c): t, s = excerpt(path, a, b, c); return ref(path, s, t)
code = [
    dict(id='filled', title='FILLED_CIRCULAR: the round trip through a pointer',
         why='C edits the link\'s own cross-section through the pointer, and the subtraction does not undo the addition in the last bit. The Rust function takes &mut TXsect; every caller hands it the link\'s section (linkXsect_* getters) so the bit is lost where EPA loses it.',
         c=F_(f'{EPA_SRC}/xsect.c', 'double filled_circ_getAofY(TXsect* xsect, double y)'),
         cpp=F_(f'{CPP_SRC}/xsect_geometry.cpp', 'double filled_circ_getAofY(TXsect* xsect, double y)'),
         rs=ref(f'{RS_SRC}/xsect.rs', func(f'{RS_SRC}/xsect.rs', 'fn filled_circ_getAofY')[1],
                func(f'{RS_SRC}/xsect.rs', 'fn filled_circ_getAofY')[0] + '\n\n// and every link lookup goes through a getter that writes the copy back:\n' +
                excerpt(f'{RS_SRC}/xsect.rs', 'macro_rules! link_xsect_getter', 0, 6)[0])),
    dict(id='storage', title='storage_getVolDiff: a storage index used as a node index',
         why='k is the storage unit\'s index; storage_getVolume() and storage_getSurfArea() take a node index and look up Node[k].subIndex. When node k is not that storage unit, the root finder solves the wrong unit\'s volume curve. Reproduced, and flagged in a comment.',
         c=F_(f'{EPA_SRC}/node.c', 'void  storage_getVolDiff(double y'),
         cpp=F_(f'{CPP_SRC}/node.cpp', 'void  Project::storage_getVolDiff'),
         rs=F_(f'{RS_SRC}/node.rs', 'fn storage_getVolDiff')),
    dict(id='cast', title='gage.c line 251: the number that would not fit',
         why='For a one-point series dxMin stays at BIG (1e10 days); times 86,400 it does not fit an int. C leaves the result undefined, x86 gives INT_MIN, and the warning test depends on which. ccast::f64_to_i32 gives x86\'s answer.',
         c=X_(f'{EPA_SRC}/gage.c', 'gageInterval = (int)(floor(Tseries[k].dxMin*SECperDAY + 0.5));', 5, 9),
         cpp=X_(f'{CPP_SRC}/gage.cpp', 'gageInterval = (int)(floor(Tseries[k].dxMin*SECperDAY + 0.5));', 5, 9),
         rs=ref(f'{RS_SRC}/gage.rs', excerpt(f'{RS_SRC}/gage.rs', 'gageInterval = f64_to_i32(', 5, 9)[1],
                excerpt(f'{RS_SRC}/gage.rs', 'gageInterval = f64_to_i32(', 5, 9)[0] + '\n\n// ccast.rs\n' + func(f'{RS_SRC}/ccast.rs', 'pub fn f64_to_i32')[0])),
    dict(id='list', title='A linked list becomes a vector, in walk order',
         why='C prepends each new external inflow to the node\'s list; the routing step then sums them in that order. Rust keeps a Vec and inserts at the front, so the sums are taken in the same order.',
         c=X_(f'{EPA_SRC}/inflow.c', 'inflow->next = Node[j].extInflow;', 14, 3),
         cpp=X_(f'{CPP_SRC}/inflow.cpp', 'inflow->next = Node[j].extInflow;', 14, 3),
         rs=X_(f'{RS_SRC}/inflow.rs', 'prj.Node[ju].extInflow.insert(0, TExtInflow::default());', 12, 8)),
    dict(id='msg', title='report_writeErrorMsg: one buffer, two jobs',
         why='lid.c puts the LID name in the global Msg and passes Msg as s. error_getMsg(code, Msg) then copies the message template over Msg, so fprintf prints the template where the name belongs. Rust has no aliasing, so lid.rs passes the template itself, and the report reads exactly as EPA\'s.',
         c=ref(f'{EPA_SRC}/report.c', func(f'{EPA_SRC}/report.c', 'void report_writeErrorMsg(int code, char* s)')[1],
               func(f'{EPA_SRC}/report.c', 'void report_writeErrorMsg(int code, char* s)')[0] + '\n\n// lid.c, the caller\n' + excerpt(f'{EPA_SRC}/lid.c', 'sstrcat(Msg, ERR_DRAIN_HEADS, MAXMSG);', 2, 1)[0]),
         cpp=F_(f'{CPP_SRC}/report.cpp', 'void Project::report_writeErrorMsg(int code, char* s)'),
         rs=F_(f'{RS_SRC}/report.rs', 'pub fn report_writeErrorMsg')),
    dict(id='delete', title='controls_delete: the loop that walks a NULL array',
         why='After an input error found while counting, Expression is still NULL but ExpressionCount is not 0, and C dereferences it on close (CRASH-08). The Rust array is an empty Vec; get_mut returns None and nothing is touched. This is one place the translation cannot do what C does, because what C does is undefined.',
         c=F_(f'{EPA_SRC}/controls.c', 'void controls_delete(void)'),
         cpp=F_(f'{CPP_SRC}/controls.cpp', 'void Project::controls_delete(void)'),
         rs=F_(f'{RS_SRC}/controls.rs', 'pub fn controls_delete')),
]

# ------------------------------------------------------------------ representation table
repr_rows = [
    ['global variable (<code>Node</code>, <code>Nobjects</code>, ...)', 'field of <code>Project</code>: <code>prj.Node</code>', f"{ST['project_globals']} fields"],
    ['file-scope <code>static</code> in a module', 'field of that module\'s state: <code>prj.dynwave.Omega</code>', f"{ST['project_mods']} module states"],
    ['<code>char*</code> name, <code>char s[N]</code>', '<code>Vec&lt;u8&gt;</code>, <code>[u8; N]</code> with C\'s terminator rules', 'bytes, never UTF-8'],
    ['<code>malloc</code>/<code>calloc</code> array', '<code>Vec&lt;T&gt;</code> filled with <code>T::default()</code> (zeros, as calloc)', 'freed by dropping'],
    ['linked list', '<code>Vec&lt;T&gt;</code> in the order C walks it', f"{ST['prepends']} prepends become <code>insert(0, ..)</code>"],
    ['pointer that may be NULL', '<code>Option&lt;T&gt;</code> or <code>Option&lt;usize&gt;</code>', 'the NULL check is the type'],
    ['table cursor <code>thisEntry</code>', '<code>Option&lt;usize&gt;</code> into <code>entries</code>', 'CRASH-03 stops here'],
    ['function pointer + <code>void*</code>', '<code>&amp;mut dyn FnMut</code> closure', f"{ST['closures']} callback parameters"],
    ['<code>(int)x</code> on a double', '<code>ccast::f64_to_i32(x)</code>: x86\'s answer when x does not fit', f"{ST['casts']} conversions"],
    ['<code>fprintf</code>/<code>sprintf</code>', '<code>cprintf!</code>/<code>cformat!</code> in safe Rust, checked against glibc', f"{ST['cprintf']} report writes"],
    ['<code>FILE*</code>', '<code>Option&lt;CFile&gt;</code>: buffered, closes when dropped', 'C\'s fopen modes'],
    ['<code>&amp;Link[j].xsect</code> passed for editing', 'copy, call, write back (<code>linkXsect_*</code>)', f"{ST['xs_get']} call sites"],
]
tiles_inside = [[f"{ST['rs_lines']:,}", f"lines of safe Rust in {ST['rs_files']} files; {ST['rs_fns']:,} functions; the crate forbids unsafe"],
                ['0', f"unsafe blocks in the engine, against {ST['c2r_fns']:,} unsafe functions in the c2rust translation"],
                [str(ST['project_fields']), f"fields of Project: {ST['project_globals']} globals and {ST['project_mods']} module states holding every file-scope static"],
                [f"{ST['casts']}", 'float-to-integer conversions routed through ccast to match x86']]

# ------------------------------------------------------------------ defect register
def cx(path, anchor, b, a): return excerpt(path, anchor, b, a)[0]
defects = [
    dict(id='RS-1', also='Reported earlier as NUM-55 in the OpenSWMM 2026 code review, which also found the second fault on the same lines: in metric models the solve mixes user units and feet, and depths come out about ten times too small. Fixed in the OpenSWMM fork (commit 3d09a10a); EPA 5.2.4 has it. The review\'s decks NUM-55_cms and NUM-55_cfs-junction-first are in the gate and match EPA.',  title='storage_getVolDiff() passes a storage index where a node index belongs', status='reproduced', statusKind='hi',
         body=f'For FUNCTIONAL (with a Newton solve), CONICAL and PYRAMIDAL storage under kinematic-wave or steady routing, the depth found for a given volume uses the storage curve of whatever unit node k points at. Same network in two section orders: peak depth in S2 is {NUM["SO_PA"]} ft with the outfall listed before the storage units and {NUM["SO_PB"]} ft with the storage units first. Try it on this tab.',
         code=cx(f'{EPA_SRC}/node.c', '*f = storage_getVolume(k, y) - storageVol->v;', 2, 1), where='node.c, lines 923-925'),
    dict(id='RS-2', also='Not among the OpenSWMM 2026 code review\'s entries; we found no earlier report of it.',  title='LID parameter errors print the message template instead of the LID name', status='reproduced', statusKind='hi',
         body='lid.c builds "&lt;LID name&gt; &lt;layer&gt;" in the global Msg and passes Msg; report_writeErrorMsg() then reuses Msg for the template. The deck tests/models/lid_error_message.inp gets <code>ERROR 185: invalid parameter value for LID</code> followed by the template again, ending <code>%s..</code>, in all three engines.',
         code=cx(f'{EPA_SRC}/report.c', 'fprintf(Frpt.file, error_getMsg(code, Msg), s);', 3, 0), where='report.c line 1433; lid.c lines 984, 999, 1011, 1028, 1041, 1155, 1170 and 1177'),
    dict(id='RS-3', also='Not among the OpenSWMM 2026 code review\'s entries; we found no earlier report of it. It is harmless to the physics (one bit), but it is state that C changes as a side effect of reading.',  title='FILLED_CIRCULAR geometry edits the link\'s full depth and area as it reads them', status='reproduced', statusKind='hi',
         body=f'The add-then-subtract round trip can leave yFull or aFull one bit lower, and C keeps the change in Link[j].xsect. A first Rust translation that worked on copies differed from EPA in {NUM["FP_DIFF"]} of {NUM["FP_VALS"]} .out values (largest {NUM["FP_MAX"]}) and in one time of maximum depth, 01:04 against EPA\'s 01:06. Fixed by writing the copy back.' + ' The independent review found one more read that used a copy, the link velocity a control rule asks for; it now goes through the same path (tests/models/filled_circ_control.inp).',
         code=cx(f'{EPA_SRC}/xsect.c', 'xsect->yFull += xsect->yBot;', 3, 12), where='xsect.c, filled_circ_* functions'),
    dict(id='RS-4', also='Reported as CRASH-21 in the OpenSWMM 2026 code review, and met earlier by Bob Dickinson\'s c2rust port, swmm5-rs, where it was the cause of 25 reports differing from EPA\'s by one WARNING 09 line.',  title='Out-of-range float-to-int conversions are undefined in C', status='x86 behaviour reproduced', statusKind='hi',
         body=f'gage.c line 251 converts about 8.6e14 to int for any one-point rain series. x86 gives -2,147,483,648, Rust\'s <code>as</code> gives +2,147,483,647, and the difference decides whether WARNING 09 is written. It changed {NUM["C2R_W09"]} reports in the c2rust translation until every conversion went through ccast.',
         code=cx(f'{EPA_SRC}/gage.c', 'gageInterval = (int)(floor(Tseries[k].dxMin*SECperDAY + 0.5));', 0, 4), where=f'gage.c line 251 (and {ST["casts"] - 1} other conversions)'),
    dict(id='RS-5', also='IO-23 in the OpenSWMM 2026 code review.',  title='IO-23: a unit hydrograph group with no rain gage reads Gage[0] of an empty array', status='Rust stops; C reads on', statusKind='warn',
         body='With no gages in the project, EPA reads past the end of the gage array and runs to completion on whatever it finds. swmm5-rust stops at the index with <code>' + html.escape(deck_rows[0]['panic'].split('\n')[0]) + '</code>.',
         code=cx(f'{EPA_SRC}/rdii.c', 'if ( Gage[g].coGage >= 0 )', 3, 3), where='rdii.c line 1062'),
    dict(id='RS-6', also='CRASH-02 in the OpenSWMM 2026 code review.',  title='CRASH-02: an [XSECTIONS] row before its link\'s row writes past the Conduit array', status='Rust stops; C writes on', statusKind='warn',
         body='Link[j].type is still the 0 calloc left, and CONDUIT is 0, so a regulator-only model writes Conduit[0] of a zero-length array. EPA runs on; swmm5-rust stops at the index.',
         code=cx(f'{EPA_SRC}/link.c', 'if ( Link[j].type == CONDUIT ) Conduit[Link[j].subIndex].barrels = 1;', 1, 0), where='link.c line 190'),
    dict(id='RS-7', also='CRASH-03 in the OpenSWMM 2026 code review.',  title='CRASH-03: a curve with no points is dereferenced through a NULL cursor', status='Rust stops; C crashes natively', statusKind='warn',
         body='Native EPA segfaults with an empty report. In WebAssembly address 0 is ordinary memory, so EPA runs on there and writes a report. swmm5-rust stops with a message naming table_getNextEntry.',
         code=cx(f'{EPA_SRC}/table.c', 'entry = table->thisEntry->next;', 2, 2), where='table.c line 382'),
    dict(id='RS-8', also='CRASH-04 in the OpenSWMM 2026 code review.',  title='CRASH-04: a quoted "Shape" leaves the Shape array empty, and project_validate() writes into it', status='Rust stops; C aborts', statusKind='warn',
         body='The counting pass compares the token with its quotes; the reading pass strips them. Native EPA aborts on heap corruption; the WebAssembly build traps in node, and in the headless Chromium used to test this page the same access crashed the whole renderer, so the page runs this deck only in swmm5-rust. swmm5-rust stops at the index.',
         code=cx(f'{EPA_SRC}/project.c', 'Shape[j].curve = i;', 4, 2), where='project.c line 227'),
    dict(id='RS-9', also='CRASH-08 in the OpenSWMM 2026 code review.',  title='CRASH-08: controls_delete() walks a NULL Expression array after an early input error', status='Rust reports the error', statusKind='ok',
         body='Native EPA segfaults on close and leaves a 0-byte report. In WebAssembly EPA survives and writes the ERROR 205/207 report, byte-identical to swmm5-rust\'s. The Rust loop finds an empty Vec and touches nothing.',
         code=cx(f'{EPA_SRC}/controls.c', 'mathexpr_delete(Expression[i].expression);', 2, 4), where='controls.c line 313'),
    dict(id='RS-10', also='CRASH-07 in the OpenSWMM 2026 code review.', title='CRASH-07: a NaN or overflowing ratio indexes a cross-section table far outside its bounds', status='Rust reads 0.0; C crashes natively', statusKind='warn',
         body='lookup() converts x/delta to an int with no check; a NaN or a huge ratio gives INT_MIN, and C reads 16 GB below the table. Native EPA segfaults. In WebAssembly the read lands in the module\'s own memory and EPA runs on. This is the one place the translators chose a value instead of stopping: an out-of-range read gives 0.0, because for a NaN ratio the result is NaN whatever is read. On CRASH-07_tiny-diameter all three WebAssembly engines then agree byte for byte; on CRASH-07_nan-diameter EPA\'s WebAssembly build reads something else and differs. The register says so rather than claim a match.',
         code=cx(f'{EPA_SRC}/xsect.c', 'i = (int)(x / delta);', 2, 4), where='xsect.c line 1488 (and 1441, 1651)'),
]

# ------------------------------------------------------------------ figures
gate_note = (f'The 221 smoke and coverage models: {T221["same"]} byte-identical; {T221["ub"]} are decks where EPA\'s C reads or writes memory it does not own; '
             f'{T221["diff"]} differ otherwise.')
wasm_rows = [('swmm5-rust vs EPA', 'both wasm32-wasi, node', WS['rs'], 'var(--rs)'),
             ('swmm5-cplus vs EPA', 'both wasm32-wasi, node', WS['cp'], 'var(--cpp)'),
             ('EPA wasm vs EPA native', 'platform control: same C, two libcs', WS['plat'], 'var(--epa)'),
             ('swmm5-rust wasm vs EPA native', 'across platform and language', WS['rsn'], 'var(--f-e)')]
cio_rows = [('printf, floating point', 200_000 * 41, '41 formats SWMM uses, random and edge values'),
            ('printf, integers and strings', 100_000 * 25 + 6 * 14 + 2, '25 integer formats, 14 string widths'),
            ('strtod', 300_000, 'number parsing, including partial and malformed text'),
            ('atoi', 300_000, 'integer parsing, the same strings')]
FIGS = dict(pipeline=F.fig_pipeline(dict(ST, c2r_gate=f'{C2R_SAME}/221 identical (after ccast)', rs_gate=f'{T221["same"]}/221 + {T221["ub"]} undefined in C')),
            unsafe=F.fig_unsafe(ST), cast=F.fig_cast(), ulp=F.fig_ulp(), storage=F.fig_storage(dict(ser, maxd_a=SO_MAXD_A)),
            gate=F.fig_gate(fold(g221), gate_note), wasm=F.fig_wasm(wasm_rows, WN or 1, W_UB), memdecks=F.fig_memdecks(deck_rows),
            borrow=F.fig_borrow(), lists=F.fig_lists(), cio=F.fig_cio(cio_rows))
if gco:
    FIGS['corpus'] = F.fig_gate(fold(gco), f'{TCO["n"]:,} of {N_CO:,} corpus models{GATE_CO_STATE}: {TCO["same"]:,} identical, {TCO["ub"]} undefined in C, {TCO["diff"]} different, {TCO["inc"]} inconclusive. Log scale.',
                                title='The corpus gate: models under 125 KB plus a seeded sample of larger ones, EPA 5.2.4 against swmm5-rust')
if bench_n:
    FIGS['timing'] = F.fig_timing([dict(label=f'swmm5-rust / EPA, median {bench_n["summary"]["rs_median"]:.2f}', color='var(--rs)', rows=[(r['model'], r['rs'], r['floor']) for r in bench_n['rows']]),
                                   dict(label=f'swmm5-cplus / EPA, median {bench_n["summary"]["cp_median"]:.2f}', color='var(--cpp)', rows=[(r['model'], r['cp'], r['floor']) for r in bench_n['rows']])],
                                  'Native run time against EPA 5.2.4 (all three built with optimization level 2)',
                                  f'Median of {bench_n["summary"]["reps"]} interleaved runs per model: EPA, C++, Rust, EPA again. Grey: the EPA-against-EPA noise floor.')
if bench_w:
    FIGS['timing_wasm'] = F.fig_timing([dict(label=f'swmm5-rust / EPA, median {bench_w["summary"]["rs_median"]:.2f}', color='var(--rs)', rows=[(r['model'], r['rs'], r['floor']) for r in bench_w['rows']]),
                                        dict(label=f'swmm5-cplus / EPA, median {bench_w["summary"]["cp_median"]:.2f}', color='var(--cpp)', rows=[(r['model'], r['cp'], r['floor']) for r in bench_w['rows']])],
                                       'In the browser\'s engine: WebAssembly run time against EPA 5.2.4 (node, V8)',
                                       f'Median of {bench_w["summary"]["reps"]} interleaved runs per model in one process. Grey: the EPA-against-EPA noise floor.')

# ------------------------------------------------------------------ essay -> html + json
def code_html(t): return '<pre class="code"><code>' + html.escape(t) + '</code></pre>'
parts = [f'<h1>{html.escape(E.TITLE)}</h1><p class="dek">{html.escape(E.SUBTITLE)}</p>']
toc = []; essay_json = dict(title=E.TITLE, subtitle=E.SUBTITLE, blocks=[]); fign = 0; missing = []
for b in E.BLOCKS:
    k = b[0]
    if k == 'h2':
        parts.append(f'<h2 id="{b[2]}">{html.escape(b[1])}</h2>'); toc.append([b[2], b[1]])
        essay_json['blocks'].append(dict(kind='h2', text=b[1]))
    elif k == 'p' or k.startswith('p?'):
        if k.startswith('p?') and k[2:] not in NUM: continue      # a paragraph that needs a measurement not yet made
        parts.append('<p>' + sub(b[1]) + '</p>'); essay_json['blocks'].append(dict(kind='p', html=sub(b[1])))
    elif k == 'fig':
        if b[1] not in FIGS: missing.append(b[1]); continue
        fign += 1
        cap = f'Figure {fign}. ' + sub(b[2])
        parts.append(f'<figure class="fig"><div class="frame">{FIGS[b[1]]}</div><figcaption>{cap}</figcaption></figure>')
        essay_json['blocks'].append(dict(kind='fig', key=b[1], caption=cap))
    elif k == 'code':
        t = b[2]
        if t.startswith('@@FUNC:'):                 # a function quoted verbatim from EPA's source
            fn, sig = t[7:].split('|', 1); t = func(f'{EPA_SRC}/{fn}', sig)[0]
        parts.append(code_html(t)); essay_json['blocks'].append(dict(kind='code', text=t))
    elif k == 'table':
        rows = [[sub(c) for c in r] for r in b[2]]
        parts.append('<div class="tablewrap"><table class="t"><thead><tr>' + ''.join(f'<th>{h}</th>' for h in b[1]) +
                     '</tr></thead><tbody>' + ''.join('<tr>' + ''.join(f'<td>{c}</td>' for c in r) + '</tr>' for r in rows) + '</tbody></table></div>')
        essay_json['blocks'].append(dict(kind='table', head=b[1], rows=rows))
    elif k in ('ul', 'ol'):
        items = [sub(x) for x in b[1]]
        parts.append(f'<{k}>' + ''.join(f'<li>{x}</li>' for x in items) + f'</{k}>')
        essay_json['blocks'].append(dict(kind=k, items=items))
essay_html = '\n'.join(parts)
left = sorted(set(re.findall(r'\{([A-Z0-9_]+)\}', essay_html)))
os.makedirs(f'{HERE}/doc/out', exist_ok=True)
json.dump(essay_json, open(f'{HERE}/doc/out/essay.json', 'w'), indent=1)
for key, s in FIGS.items():
    open(f'{HERE}/doc/out/fig_{key}.svg', 'w').write(F.svg_for_docx(s))

# ------------------------------------------------------------------ samples
def opt(text, key):
    m = re.search(r'^\s*' + key + r'\s+(\S+)', text, re.M | re.I); return m.group(1) if m else None
def parse_dt(d, t):
    for fmt in ('%m/%d/%Y %H:%M:%S', '%m/%d/%Y %H:%M'):
        try: return datetime.datetime.strptime(f'{d} {t}', fmt)
        except Exception: pass
    return None
SAMPLE_SPEC = [
    ('exam79', f'{CORPUS}/EPA/exam79.inp', 'EXTRAN Example 79, tidal boundary'),
    ('extran1_egg', f'{CORPUS}/EPA/extran1_egg.inp', 'EXTRAN Example 1, egg-shaped conduits'),
    ('extran1_irregular', f'{CORPUS}/EPA/extran1_irregular.inp', 'EXTRAN Example 1, irregular transects'),
    ('controlExample', f'{CORPUS}/EPA/controlExample.inp', 'Control rules on a pump'),
    ('small_pump_fm_model', f'{CORPUS}/EPA/small_pump_fm_model.inp', 'Pump into a force main'),
    ('siphon_larger_SW5', f'{CORPUS}/EPA/siphon_larger_SW5.inp', 'Surcharged 2-ft siphon'),
    ('transect_example', f'{CORPUS}/EPA/transect_example.inp', 'Natural channel transects'),
    ('filled_circ_drift', f'{REPO}/tests/models/filled_circ_drift.inp', 'FILLED_CIRCULAR sizes that lose a bit (RS-3)'),
    ('storage_order_A', f'{REPO}/tests/models/storage_order_A.inp', 'Storage index as node index, order A (RS-1)'),
    ('storage_order_B', f'{REPO}/tests/models/storage_order_B.inp', 'Storage index as node index, order B (RS-1)'),
    ('lid_error_message', f'{REPO}/tests/models/lid_error_message.inp', 'LID error message template (RS-2)'),
]
samples = []
for key, p, name in SAMPLE_SPEC:
    t = rd(p)
    a = parse_dt(opt(t, 'START_DATE'), opt(t, 'START_TIME') or '00:00:00')
    b = parse_dt(opt(t, 'END_DATE'), opt(t, 'END_TIME') or '00:00:00')
    days = (b - a).total_seconds() / 86400 if a and b else 1
    samples.append(dict(key=key, file=key + '.inp', name=name, units=opt(t, 'FLOW_UNITS') or 'CFS', days=days, text=t))

# ------------------------------------------------------------------ verify
parity = ('<b>Parity contract.</b> swmm5-rust matches EPA SWMM 5.2.4 built from the official v5.2.4 tag: the binary <code>.out</code> file byte for byte, '
          'and the <code>.rpt</code> file byte for byte after removing the three wall-clock lines (analysis begun, ended, elapsed). '
          'Native: EPA compiled by GCC 13.3 at <code>-O2</code>, swmm5-rust by Rust 1.97 at <code>opt-level=2</code>, on x86-64 Linux; the gate passes absolute paths because EPA\'s own <code>getAbsolutePath()</code> overruns its buffer on relative ones. '
          'WebAssembly: EPA and swmm5-cplus by Clang 18 with wasi-libc, swmm5-rust by Rust 1.91 for wasm32-wasip1 linked against the same wasi-libc. '
          'Where EPA\'s C reads or writes memory it does not own, its output is undefined; those decks are counted apart, never as a match.')
verify_tiles = [[f'{T221["same"]} / {T221["n"] - T221["ub"]}', f'smoke and coverage set, native, byte-identical; {T221["ub"]} more are undefined in C'],
                [f'{TFC["same"]} / {TFC["n"]}', f'corpus models with FILLED_CIRCULAR conduits{"" if TFC["n"] >= N_FC else " (of " + str(N_FC) + ", running)"}'],
                [f'{TCO["same"]:,} / {TCO["n"] - TCO["ub"] - TCO["inc"]:,}', f'corpus set, native{"" if TCO["n"] >= N_CO else f" ({TCO[chr(110)]:,} of {N_CO:,} run so far)"}'],
                [f'{WS["rs"]} / {WN}', f'WebAssembly: swmm5-rust against EPA, same libc; {W_UB} undefined decks apart' + ('' if WN_ALL >= 221 else ' (running)')]]
controls = [
    'Null control: each engine run twice on the same input writes the same bytes, so identical output is reported as identical and the comparison stops there.',
    f'Positive control: before the FILLED_CIRCULAR fix the gate caught a difference of one float bit in {NUM["FP_DIFF"]} of {NUM["FP_VALS"]} .out values. The instrument sees one bit.',
    f'Translation control: the c2rust crate, EPA\'s C transliterated statement for statement, matched EPA on {C2R_SAME} of 221 models once its float-to-int casts went through ccast. The lockstep tool steps it beside the safe engine and compares every node, link and subcatchment state after every routing step: no difference on the sample models (results/lockstep_samples.log); on the engine before the FILLED_CIRCULAR fix it stops at step 29, conduit C0, new depth, last bit.',
    f'Platform control: EPA\'s C against itself, native (GCC, glibc) against WebAssembly (Clang, wasi-libc), agrees on {WS["plat"]} of {WN} models with defined behaviour. The source is the same, so where it disagrees the platform is the cause; which library functions differ was not traced.',
    f'Undefined-behaviour control: the six memory-error decks are run in four builds (EPA and Rust, native and wasm). EPA\'s own answer changes with the platform on {sum(1 for r in deck_rows if r["epa_native"][1] != r["epa_wasm"][1])} of them, which is what undefined means; swmm5-rust\'s changes on {(lambda n: "none" if n == 0 else n)(sum(1 for r in deck_rows if r["rs_native"][1] != r["rs_wasm"][1]))}.',
    f'Library control: printf, strtod, atoi and sscanf rewritten in safe Rust were compared with glibc on about {sum(r[1] for r in cio_rows) / 1e6:.0f} million inputs.',
    'Section-order control: the RS-1 network is run in two section orders by all three engines; each engine agrees with EPA on each order, and the orders disagree with each other exactly as EPA does.',
    'Timing control: EPA, C++, Rust, then EPA again, interleaved; the second EPA run sets the noise floor quoted beside every ratio.',
]
limits = ('<b>What this does not show.</b> swmm5-rust is not a new engine and not more accurate: every number is EPA 5.2.4\'s, defects included, and RS-1 to RS-4 are reproduced on purpose. '
          'It does not reproduce EPA where EPA\'s own behaviour is undefined (RS-5 to RS-9): there it stops or reports, and C\'s answer depends on memory. '
          + ('' if TCO['n'] >= N_CO else f'The corpus gate had run {TCO["n"]:,} of {N_CO:,} models when this page was built. ')
          + 'Safe Rust rules out memory errors, not wrong arithmetic; the arithmetic is checked only by the gate, on the models the gate ran. '
          'It is not faster than the C. The C interface is the one unsafe part, and the translation has not been reviewed line by line by a person.')
gate_csv = 'set,folder,models,identical,undefined_in_C,different,inconclusive\n' + ''.join(
    f"{nm},{f['name']},{f['n']},{f['same']},{f['ub']},{f['diff']},{f['inc']}\n" for nm, rows in (('smoke+coverage', g221), ('filled_circular', gfc), ('corpus', gco)) if rows for f in fold(rows))

def sha_file(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
commit = subprocess.run(['git', '-C', REPO, 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()
gate_commit = (open(f'{RES}/engine_commit.txt').read().strip() if os.path.exists(f'{RES}/engine_commit.txt') else commit)
prov = [['engine commit (gated binaries)', gate_commit], ['page built from commit', commit],
        ['swmm5rs.wasm sha256', sha_file(f'{W}/swmm5rs.wasm')], ['swmm5cplus.wasm sha256', sha_file(f'{W}/swmm5cplus.wasm')],
        ['swmm524_epa.wasm sha256', sha_file(f'{W}/swmm524_epa.wasm')],
        ['EPA reference', 'USEPA/Stormwater-Management-Model tag v5.2.4'],
        ['native toolchains', 'GCC 13.3.0 -O2 (EPA); Rust 1.97.0 opt-level 2 (swmm5-rust)'],
        ['wasm toolchains', 'Clang 18.1.3 + wasi-libc (EPA, swmm5-cplus); Rust 1.91 wasm32-wasip1, std built from source, wasm-ld + the same wasi-libc'],
        ['in-browser file system', 'MemWasi: 19 WASI calls, in memory (this page)'],
        ['built', datetime.date.today().isoformat()]]

data = dict(samples=samples, figs=FIGS, insideTiles=tiles_inside, repr=repr_rows, code=code, defects=defects,
            decks=[dict(name=r['name'], what=r['what'], text=r['text'],
                        skip=(['epa', 'cpp'] if r['name'] == 'CRASH-04_quoted-shape' else []),
                        recorded={'epa': WR[(r['name'] + '.inp', 'epa')], 'cpp': WR[(r['name'] + '.inp', 'cplus')]}) for r in deck_rows],
            deckMatrix=[{k: v for k, v in r.items() if k != 'text'} for r in deck_rows],
            lid={k: dict(err=v['err'], rc=v['rc'], out_sha=v['out_sha']) for k, v in LID.items()},
            verify=dict(parity=parity, tiles=verify_tiles, controls=controls, limits=limits, csv=gate_csv,
                        wasmBad=w_rs_bad, platBad=w_plat_bad, ubCorpus=UB_CORPUS, diffs=DIFF_LIST, inc=INC_LIST),
            prov=prov, commit=gate_commit, essayHtml=essay_html, toc=toc, stats=ST)

DOCX = f'{HERE}/doc/out/swmm5-rust-doc.docx'
tpl = open(f'{HERE}/app_template.html').read()
out = (tpl.replace('/*__WASM_RS__*/', base64.b64encode(open(f'{W}/swmm5rs.wasm', 'rb').read()).decode())
          .replace('/*__WASM_CPLUS__*/', base64.b64encode(open(f'{W}/swmm5cplus.wasm', 'rb').read()).decode())
          .replace('/*__WASM_EPA__*/', base64.b64encode(open(f'{W}/swmm524_epa.wasm', 'rb').read()).decode())
          .replace('/*__ENGINE_CORE__*/', open(f'{HERE}/engine_core.js').read())
          .replace('/*__CMP__*/', open(f'{HERE}/rpt_compare.js').read())
          .replace('/*__DOCX__*/', base64.b64encode(open(DOCX, 'rb').read()).decode() if os.path.exists(DOCX) else '')
          .replace('/*__DATA__*/', json.dumps(data).replace('</', '<\\/')))
os.makedirs(f'{REPO}/build/app', exist_ok=True)
open(f'{REPO}/build/app/swmm5-rust.html', 'w').write(out)
print('wrote', len(out) // 1024, 'KB |', '221:', T221, '| filled:', TFC, '| corpus:', TCO, '| wasm:', WN, WS, '| diffs:', diffs_all[:5],
      '| storage S2==S1 in A:', S2_eq_S1_A, 'engines agree:', SO_SAME, '| missing figs:', missing, '| unfilled:', left, '| docx:', os.path.exists(DOCX))

# ------------------------------------------------------------------ docs/DEFECTS.md, from the same register
def md(h): return html.unescape(re.sub(r'</?code>', '`', re.sub(r'<(?!/?code)[^>]+>', '', h)))
os.makedirs(f'{REPO}/docs', exist_ok=True)
with open(f'{REPO}/docs/DEFECTS.md', 'w') as fo:
    fo.write('# Defect register\n\nWhat the translation of EPA SWMM 5.2.4 into safe Rust found in the C. '
             'Generated by `app/build.py` from the same list the app\'s Inside tab shows; every code excerpt is taken from '
             'the v5.2.4 source at build time.\n\n'
             '* **RS-1 to RS-4** are reproduced on purpose: the output must match EPA byte for byte, defects included.\n'
             '* **RS-5 to RS-10** are undefined behaviour in C (out-of-bounds or NULL accesses). EPA\'s output there depends on memory '
             'and on the platform, so there is nothing to reproduce; swmm5-rust stops with a message naming the line, or, for RS-9, writes '
             'the error report, or, for RS-10, reads 0.0 where C reads far outside a table. The gate counts these decks apart.\n\n'
             'Other places where C\'s behaviour is undefined, and what the Rust does there, are listed in [UNDEFINED.md](UNDEFINED.md).\n\n')
    for d in defects:
        fo.write(f"## {d['id']}: {md(d['title'])}\n\n*Status: {d['status']}. Where: {d['where']}.*\n\n{md(d['body'])}\n\n{md(d.get('also', ''))}\n\n```c\n{d['code']}\n```\n\n")
print('wrote docs/DEFECTS.md')
