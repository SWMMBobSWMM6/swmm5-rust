#!/usr/bin/env python3
"""Rust signatures for the engine's cross-module functions, as stubs.

Rules (exceptions in SIG below):
  * the model is the first parameter, `prj: &mut Project` (C SWMM names many
    locals `p`, so the project is `prj`); functions that touch no model
    state take no project;
  * `char* tok[], int ntoks` -> `tok: &[Vec<u8>], ntoks: i32`;
  * an input `char*` -> `&[u8]`; a scalar out-parameter `T*` -> `&mut T`;
    an array `T x[]` -> `&[T]` or `&mut [T]`;
  * a pointer to a struct -> `&T` / `&mut T`;
  * a callback -> `&mut dyn FnMut(..)`; its `void*` context is captured.
"""
import re, json, sys, os, collections
D = sys.argv[1]; OUT = sys.argv[2]
src = {f[:-3]: open(os.path.join(D, f)).read() for f in os.listdir(D) if f.endswith('.rs')}
defs = {}
for m, s in src.items():
    for mm in re.finditer(r'#\[no_mangle\]\npub unsafe extern "C" fn (\w+)\((.*?)\)( -> ([^{]+))?\s*\{', s, flags=re.S):
        defs[mm.group(1)] = (m, mm.group(2), (mm.group(4) or '').strip())
users = collections.defaultdict(set)
for mod, s in src.items():
    for blk in re.findall(r'extern "C" \{(.*?)\n\}', s, flags=re.S):
        for n in re.findall(r'^\s*(?:pub )?fn (\w+)\(', blk, flags=re.M):
            if n in defs and defs[n][0] != mod: users[n].add(mod)

P = 'prj: &mut Project'; R = 'prj: &Project'
# Exceptions, written as full Rust parameter lists and return types.
SIG = {
 # hash.c: the table is passed in, no model state
 'HTcreate': ('', 'HTtable'), 'HTinsert': ('ht: &mut HTtable, key: &[u8], data: i32', 'i32'),
 'HTfind': ('ht: &HTtable, key: &[u8]', 'i32'), 'HTfindKey': ('ht: &HTtable, key: &[u8]', 'Option<Vec<u8>>'),
 'HTfree': ('ht: &mut HTtable', ''), 'hash': ('str: &[u8]', 'u32'), 'samestr': ('s1: &[u8], s2: &[u8]', 'i32'),
 # mempool.c is gone: names are owned Vec<u8>
 'AllocInit': None, 'Alloc': None, 'AllocSetPool': None, 'AllocReset': None, 'AllocFreePool': None,
 # mathexpr.c
 'mathexpr_create': ('formula: &[u8], getVar: &mut dyn FnMut(&[u8]) -> i32', 'Option<MathExpr>'),
 'mathexpr_eval': ('expr: &MathExpr, getVariableValue: &mut dyn FnMut(i32) -> f64', 'f64'),
 'mathexpr_delete': ('expr: Option<MathExpr>', ''),
 # findroot.c / odesolve.c: callbacks capture what C passed as void*
 'findroot_Newton': ('x1: f64, x2: f64, rts: &mut f64, xacc: f64, func: &mut dyn FnMut(f64, &mut f64, &mut f64)', 'i32'),
 'findroot_Ridder': ('x1: f64, x2: f64, xacc: f64, func: &mut dyn FnMut(f64) -> f64', 'f64'),
 'odesolve_open': ('maxn: i32', 'i32'), 'odesolve_close': ('', ''),
 'odesolve_integrate': ('ystart: &mut [f64], n: i32, x1: f64, x2: f64, eps: f64, h1: f64, derivs: &mut dyn FnMut(f64, &[f64], &mut [f64])', 'i32'),
 # input.c parsing helpers: pure
 'findmatch': ('s: &[u8], keyword: &[&[u8]]', 'i32'), 'match': ('str: &[u8], substr: &[u8]', 'i32'),
 'getDouble': ('s: &[u8], y: &mut f64', 'i32'), 'getInt': ('s: &[u8], y: &mut i32', 'i32'),
 'getFloat': ('s: &[u8], y: &mut f32', 'i32'),
 # swmm5.c string helpers: pure; file-name helpers work on a TFile-style buffer
 'strcomp': ('s1: &[u8], s2: &[u8]', 'i32'), 'sstrncpy': ('dest: &mut [u8], src: &[u8], n: usize', 'usize'),
 'sstrcat': ('dest: &mut [u8], src: &[u8], size: usize', 'usize'), 'isRelativePath': ('fname: &[u8]', 'i32'),
 'addAbsolutePath': (P + ', fname: &mut [u8]', ''), 'getTempFileName': (P + ', fname: &mut [u8]', ''),
 'getElapsedTime': (R + ', aDate: DateTime, days: &mut i32, hrs: &mut i32, mins: &mut i32', ''),
 # error.c
 'error_getMsg': ('errCode: i32, msg: &mut Vec<u8>', ''),
 'error_setInpError': (P + ', errcode: i32, s: &[u8]', 'i32'),
 # datetime.c: pure except where DateFormat (datetime state) is used
 'datetime_encodeDate': ('year: i32, month: i32, day: i32', 'DateTime'),
 'datetime_encodeTime': ('hour: i32, minute: i32, second: f64', 'DateTime'),
 'datetime_decodeDate': ('date: DateTime, year: &mut i32, month: &mut i32, day: &mut i32', ''),
 'datetime_decodeTime': ('time: DateTime, h: &mut i32, m: &mut i32, s: &mut i32', ''),
 'datetime_dateToStr': (R + ', date: DateTime, s: &mut Vec<u8>', ''),
 'datetime_timeToStr': ('time: DateTime, s: &mut Vec<u8>', ''),
 'datetime_strToDate': (R + ', s: &[u8], d: &mut DateTime', 'i32'),
 'datetime_strToTime': ('s: &[u8], t: &mut DateTime', 'i32'),
 'datetime_findMonth': ('month: &[u8]', 'i32'),
 'datetime_setDateFormat': (P + ', fmt: i32', ''),
 'datetime_getTimeStamp': (R + ', fmt: i32, aDate: DateTime, stampSize: i32, timeStamp: &mut Vec<u8>', ''),
 'datetime_dayOfYear': ('date: DateTime', 'i32'), 'datetime_dayOfWeek': ('date: DateTime', 'i32'),
 'datetime_hourOfDay': ('date: DateTime', 'i32'), 'datetime_monthOfYear': ('date: DateTime', 'i32'),
 'datetime_daysPerMonth': ('year: i32, month: i32', 'i32'),
 'datetime_addSeconds': ('date1: DateTime, seconds: f64', 'DateTime'),
 'datetime_addDays': ('date1: DateTime, date2: DateTime', 'DateTime'),
 'datetime_timeDiff': ('date1: DateTime, date2: DateTime', 'i64'),
 'divMod': ('n: i32, d: i32, result: &mut i32, remainder: &mut i32', ''),
 # table.c: a table carries its own cursor and file; no model state
 'table_init': ('table: &mut TTable', ''), 'table_deleteEntries': ('table: &mut TTable', ''),
 'table_addEntry': ('table: &mut TTable, x: f64, y: f64', 'i32'),
 'table_getFirstEntry': ('table: &mut TTable, x: &mut f64, y: &mut f64', 'i32'),
 'table_getNextEntry': ('table: &mut TTable, x: &mut f64, y: &mut f64', 'i32'),
 'table_validate': ('table: &mut TTable', 'i32'),
 'table_interpolate': ('x: f64, x1: f64, y1: f64, x2: f64, y2: f64', 'f64'),
 'table_lookup': ('table: &mut TTable, x: f64', 'f64'), 'table_lookupEx': ('table: &mut TTable, x: f64', 'f64'),
 'table_intervalLookup': ('table: &mut TTable, x: f64', 'f64'), 'table_inverseLookup': ('table: &mut TTable, y: f64', 'f64'),
 'table_getSlope': ('table: &mut TTable, x: f64', 'f64'), 'table_getMaxY': ('table: &mut TTable, x: f64', 'f64'),
 'table_getStorageVolume': ('table: &mut TTable, x: f64', 'f64'), 'table_getStorageDepth': ('table: &mut TTable, v: f64', 'f64'),
 'table_tseriesInit': ('table: &mut TTable', ''), 'table_tseriesLookup': ('table: &mut TTable, x: f64, extend: i8', 'f64'),
 # xsect.c: geometry reads Transect/Street/Shape, so a shared &Project
 'xsect_isOpen': ('type_: i32', 'i32'),
 'xsect_getAmax': (R + ', xsect: &TXsect', 'f64'),
 'xsect_getSofA': (R + ', xsect: &TXsect, area: f64', 'f64'), 'xsect_getYofA': (R + ', xsect: &TXsect, a: f64', 'f64'),
 'xsect_getRofA': (R + ', xsect: &TXsect, a: f64', 'f64'), 'xsect_getAofS': (R + ', xsect: &TXsect, sFactor: f64', 'f64'),
 'xsect_getdSdA': (R + ', xsect: &TXsect, a: f64', 'f64'), 'xsect_getAofY': (R + ', xsect: &TXsect, y: f64', 'f64'),
 'xsect_getRofY': (R + ', xsect: &TXsect, y: f64', 'f64'), 'xsect_getWofY': (R + ', xsect: &TXsect, y: f64', 'f64'),
 'xsect_getYcrit': (R + ', xsect: &TXsect, q: f64', 'f64'),
 'xsect_setParams': (R + ', xsect: &mut TXsect, type_: i32, p: &[f64], ucf: f64', 'i32'),
 'xsect_setIrregXsectParams': (R + ', xsect: &mut TXsect', ''),
 'xsect_setCustomXsectParams': (R + ', xsect: &mut TXsect', ''),
 'xsect_setStreetXsectParams': (R + ', xsect: &mut TXsect', ''),
 # shape.c: Atotal/Ptotal are working values of one call
 'shape_validate': ('shape: &mut TShape, curve: &mut TTable', 'i32'),
 # infil.c Green-Ampt kernel: InfilFactor and Fumax live in the infil state
 'grnampt_setParams': ('infil: &mut TGrnAmpt, p: &[f64]', 'i32'),
 'grnampt_initState': ('infil: &mut TGrnAmpt', ''),
 'grnampt_getInfil': ('ist: &mut crate::infil::InfilState, infil: &mut TGrnAmpt, tstep: f64, irate: f64, depth: f64, modelType: i32', 'f64'),
 'grnampt_getParams': (R + ', j: i32, p: &mut [f64]', ''),
 'infil_getState': (R + ', j: i32, x: &mut [f64]', ''), 'infil_setState': (P + ', j: i32, x: &[f64]', ''),
 'exfil_getLoss': ('ist: &mut crate::infil::InfilState, exfil: &mut TExfil, tStep: f64, depth: f64, area: f64', 'f64'),
 # lidproc.c: the caller takes the unit and process out of prj.lid (std::mem::take) and puts them back
 'lidproc_initWaterBalance': ('lidUnit: &mut TLidUnit, initVol: f64', ''),
 'lidproc_getOutflow': (P + ', lidUnit: &mut TLidUnit, lidProc: &TLidProc, inflow: f64, evap: f64, infil: f64, maxInfil: f64, tStep: f64, lidEvap: &mut f64, lidInfil: &mut f64, lidDrain: &mut f64', 'f64'),
 'lidproc_saveResults': (P + ', lidUnit: &mut TLidUnit, ucfRainfall: f64, ucfRainDepth: f64', ''),
 # report.c: struct arguments are copies of model state (they are all Copy)
 'report_writeTseriesErrorMsg': (P + ', code: i32, i: i32', ''),
 'report_writeRainStats': (P + ', i: i32, r: &TRainStats', ''),
 'report_writeMaxStats': (P + ', maxMassBalErrs: &[TMaxStats], maxCourantCrit: &[TMaxStats], nMaxStats: i32', ''),
 'report_writeMaxFlowTurns': (P + ', flowTurns: &[TMaxStats], nMaxStats: i32', ''),
 'report_writeNonconvergedStats': (P + ', maxNonconverged: &[TMaxStats], nMaxStats: i32', ''),
 'iface_saveOutletResults': (P + ', reportDate: DateTime', ''),
 'project_findID': (R + ', type_: i32, id: &[u8]', 'Option<Vec<u8>>'),
 'project_createMatrix': ('nrows: i32, ncols: i32', 'Vec<Vec<f64>>'),
 'project_freeMatrix': ('a: Vec<Vec<f64>>', ''),
 'project_open': (P + ', f1: &[u8], f2: &[u8], f3: &[u8]', ''),
 'project_readOption': (P + ', s1: &[u8], s2: &[u8]', 'i32'),
 'project_findObject': (R + ', type_: i32, id: &[u8]', 'i32'),
 'project_addObject': (P + ', type_: i32, id: &[u8], n: i32', 'i32'),
 'massbal_getSysFlows': (R + ', f: f64, sysFlows: &mut [f64]', ''),
 'flowrout_execute': (P + ', links: &[i32], routingModel: i32, tStep: f64', 'i32'),
 'inflow_getDwfInflow': (R + ', inflow: &TDwfInflow, month: i32, day: i32, hour: i32', 'f64'),
 'inflow_initDwfInflow': ('patterns: &[TPattern], inflow: &mut TDwfInflow', ''),
 'inflow_getExtInflow': (P + ', inflow: &TExtInflow, aDate: DateTime', 'f64'),
 'landuse_getCoPollutLoad': (R + ', p: i32, washoff: &[f64]', 'f64'),
 'landuse_getInitBuildup': (R + ', landFactor: &mut [TLandFactor], initBuildup: &[f64], area: f64, curb: f64', ''),
 'landuse_getWashoffLoad': (P + ', i: i32, p: i32, area: f64, landFactor: &mut [TLandFactor], runoff: f64, vOutflow: f64', 'f64'),
 'lid_addDrainLoads': (R + ', j: i32, c: &mut [f64], tStep: f64', ''),
 'gwater_getState': (R + ', j: i32, x: &mut [f64]', ''), 'gwater_setState': (P + ', j: i32, x: &[f64]', ''),
 'snow_getState': (R + ', i: i32, j: i32, x: &mut [f64]', ''), 'snow_setState': (P + ', i: i32, j: i32, x: &[f64]', ''),
 'snow_getSnowMelt': (P + ', j: i32, rainfall: f64, snowfall: f64, tStep: f64, netPrecip: &mut [f64]', 'f64'),
 'node_getResults': (P + ', j: i32, wt: f64, x: &mut [f32]', ''),
 'link_getResults': (P + ', j: i32, wt: f64, x: &mut [f32]', ''),
 'subcatch_getResults': (P + ', j: i32, wt: f64, x: &mut [f32]', ''),
 'report_writeLine': (P + ', line: &[u8]', ''),
}
def map_type(t, name, fn):
    t = t.strip().replace('::core::ffi::c_', '').replace('mut ', '').replace('const ', '')
    base = {'int': 'i32', 'double': 'f64', 'float': 'f32', 'long': 'i64', 'char': 'i8', 'DateTime': 'DateTime',
            'size_t': 'usize', 'uint': 'u32'}
    if t in base: return base[t]
    if t == '**char': return 'TOK'
    if t == '*char': return '&[u8]'
    if t.startswith('*'):
        inner = map_type(t[1:], name, fn)
        arrays = {'x', 'values', 'c', 'sysFlows', 'p', 'ystart', 'washoff', 'links'}
        if name in arrays: return f'&mut [{inner}]'
        return f'&mut {inner}'
    if t.startswith('Option<'): return 'CALLBACK'
    return t
out = collections.defaultdict(list)
todo = []
for n in sorted(defs):
    m, params, ret = defs[n]
    if not users[n] and n not in SIG: continue
    if n in SIG:
        if SIG[n] is None: continue
        ps, r = SIG[n]
    else:
        plist = [x.strip() for x in re.sub(r'\s+', ' ', params).split(',') if x.strip()]
        rs = []
        skip = False
        for i, pp in enumerate(plist):
            name, t = pp.split(':', 1)
            name = name.replace('mut ', '').strip()
            if name == 'type_0': name = 'type_'
            mt = map_type(t, name, n)
            if mt == 'TOK':
                rs.append(f'{name}: &[Vec<u8>]')
            elif mt == 'CALLBACK':
                todo.append(n); skip = True
            else:
                if mt.startswith('&mut TMaxStats'): mt = '&[TMaxStats]'
                if mt.startswith('&mut T') and name in ('totals', 'timeStepStats', 'QualTotals', 'r'):
                    mt = '&' + mt[5:]
                rs.append(f'{name}: {mt}')
        ps = ', '.join([P] + rs)
        r = map_type(ret, '', n) if ret else ''
        if r == '&[u8]': r = 'Vec<u8>'
    out[m].append((n, ps, r, sorted(users[n])))
json.dump({m: v for m, v in out.items()}, open(OUT, 'w'), indent=1)
print(sum(len(v) for v in out.values()), 'signatures;', 'callbacks unmapped:', todo)
