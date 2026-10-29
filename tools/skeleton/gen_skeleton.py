#!/usr/bin/env python3
"""Write the engine skeleton: one stub file per C module, globals.rs, prelude.rs, lib.rs.

Each module file gets its state struct (the C file's statics; the fields
other modules reach are pre-declared so every module compiles alone), and a
stub for every function other modules call, with the signature agreed in
sigs.json. Translators replace the `todo!()` bodies and add the module's
private functions and remaining state.
"""
import json, sys, os, re
SIGS, OUTDIR = sys.argv[1], sys.argv[2]
sigs = json.load(open(SIGS))
R_OVERRIDE = {'UCF', 'getDateTime', 'link_getLength', 'link_getVelocity', 'link_getFroude', 'link_getYcrit',
  'link_getYnorm', 'link_getPower', 'subcatch_getFracPerv', 'subcatch_getDepth', 'subcatch_getStorage',
  'lid_getPervArea', 'lid_getFlowToPerv', 'lid_getStoredVolume', 'lid_getDrainFlow', 'gwater_getVolume',
  'snow_getSnowCover', 'iface_getIfaceFlow', 'iface_getIfaceNode', 'iface_getIfaceQual', 'massbal_getStepFlowError',
  'street_getExtentFilled', 'surfqual_getWtdWashoff', 'subcatch_getWtdOutflow', 'inlet_capturedFlow',
  'gage_getPastRain', 'link_getFullState'}
FIX = {
 'toposort_sortLinks': 'prj: &mut Project, sortedLinks: &mut [i32]',
 'treatmnt_setInflow': 'prj: &mut Project, qIn: f64, wIn: &[f64]',
 'link_getFullState': 'a1: f64, a2: f64, aFull: f64',
}
MODULES = ['climate', 'controls', 'culvert', 'datetime', 'dwflow', 'dynwave', 'error', 'exfil', 'findroot',
  'flowrout', 'forcmain', 'gage', 'gwater', 'hash', 'hotstart', 'iface', 'infil', 'inflow', 'inlet', 'input',
  'inputrpt', 'keywords', 'kinwave', 'landuse', 'lid', 'lidproc', 'link', 'massbal', 'mathexpr', 'node',
  'odesolve', 'output', 'project', 'qualrout', 'rain', 'rdii', 'report', 'roadway', 'routing', 'runoff',
  'shape', 'snow', 'stats', 'statsrpt', 'street', 'subcatch', 'surfqual', 'swmm5', 'table', 'toposort',
  'transect', 'treatmnt', 'xsect']
# fields of a module's state that other modules use (C: extern references)
SHARED = {
 'stats': ['SubcatchStats: Vec<TSubcatchStats>', 'NodeStats: Vec<TNodeStats>', 'LinkStats: Vec<TLinkStats>',
           'StorageStats: Vec<TStorageStats>', 'OutfallStats: Vec<TOutfallStats>', 'PumpStats: Vec<TPumpStats>',
           'MaxOutfallFlow: f64', 'RoutingTimeSpan: f64'],
 'massbal': ['StepFlowTotals: TRoutingTotals', 'StepQualTotals: Vec<TRoutingTotals>', 'NodeInflow: Vec<f64>',
             'NodeOutflow: Vec<f64>'],
 'subcatch': ['Vevap: f64', 'Vpevap: f64', 'Vinfil: f64', 'Vinflow: f64', 'Voutflow: f64', 'VlidIn: f64',
              'VlidInfil: f64', 'VlidOut: f64', 'VlidDrain: f64', 'VlidReturn: f64'],
 'runoff': ['HasWetLids: i8', 'OutflowLoad: Vec<f64>'],
 'output': ['SubcatchResults: Vec<f32>', 'NodeResults: Vec<f32>', 'LinkResults: Vec<f32>'],
 'project': ['Htable: Vec<HTtable>'],
}
EXTRA = {
 'error': '''#[derive(Clone)]
pub struct ErrorState {
    /// error.c: `char ErrString[256]`
    pub ErrString: [u8; 256],
}
impl Default for ErrorState {
    fn default() -> Self {
        ErrorState { ErrString: [0; 256] }
    }
}
''',
 'mathexpr': '''/// One term of a tokenized expression in postfix order (C: `struct ExprNode`,
/// whose prev/next links become the order of this vector).
#[derive(Clone, Copy, Debug, Default)]
pub struct ExprNode {
    pub opcode: i32,
    pub ivar: i32,
    pub fvalue: f64,
}
/// C: `typedef struct ExprNode MathExpr;` a list of terms.
pub type MathExpr = Vec<ExprNode>;

#[derive(Default)]
pub struct MathexprState {}
''',
 'hash': '''/// One entry of a hash table bucket (C: `struct HTentry`; the bucket's
/// linked list becomes a vector in the same order).
#[derive(Clone, Default)]
pub struct HTentry {
    pub key: Vec<u8>,
    pub data: i32,
}
/// C: `typedef struct HTentry *HTtable;`, an array of HTMAXSIZE buckets.
pub type HTtable = Vec<Vec<HTentry>>;

#[derive(Default)]
pub struct HashState {}
''',
}
def camel(m): return m[0].upper() + m[1:]
for m in MODULES:
    if m == 'keywords':
        continue
    path = os.path.join(OUTDIR, m + '.rs')
    lines = [f'//! {m}.c of EPA SWMM 5.2.4, in safe Rust.', '//!',
             f'//! TRANSLATION STUB: bodies are `todo!()` until {m}.c is translated.', '',
             'use crate::prelude::*;', '']
    if m in EXTRA:
        lines.append(EXTRA[m])
    else:
        lines.append('/// The file-scope variables of ' + m + '.c.')
        lines.append('#[derive(Default)]')
        lines.append(f'pub struct {camel(m)}State {{')
        for f in SHARED.get(m, []):
            lines.append(f'    pub {f},')
        lines.append('}')
        lines.append('')
    for n, ps, r, users in sigs.get(m, []):
        if n in FIX: ps = FIX[n]
        if n in R_OVERRIDE: ps = ps.replace('prj: &mut Project', 'prj: &Project')
        ret = f' -> {r}' if r else ''
        lines.append(f'/// Called by: {", ".join(users) if users else "(entry point)"}.')
        lines.append(f'pub fn {n}({ps}){ret} {{')
        lines.append(f'    todo!("{n}")')
        lines.append('}')
        lines.append('')
    open(path, 'w').write('\n'.join(lines))
# globals.rs
G = '''//! The model: every global variable of globals.h and every file-scope
//! variable of the 54 C files, as fields of one struct. Two `Project`s share
//! nothing, so any number of models can run in one process.

use crate::prelude::*;

pub struct Project {
    // ---- files (globals.h) ----
    pub Finp: TFile,
    pub Fout: TFile,
    pub Frpt: TFile,
    pub Fclimate: TFile,
    pub Frain: TFile,
    pub Frunoff: TFile,
    pub Frdii: TFile,
    pub Fhotstart1: TFile,
    pub Fhotstart2: TFile,
    pub Finflows: TFile,
    pub Foutflows: TFile,
    // ---- counts and messages ----
    pub Nperiods: i64,
    pub TotalStepCount: i64,
    pub ReportStepCount: i64,
    pub NonConvergeCount: i64,
    pub Msg: [u8; 1025],
    pub ErrorMsg: [u8; 1025],
    pub Title: [[u8; 1025]; 3],
    pub TempDir: [u8; 260],
    pub InpDir: [u8; 260],
    pub RptFlags: TRptFlags,
    pub NumEvents: i32,
    pub NormalFlowLtd: i32,
    pub SlopeWeighting: i32,
    pub Compatibility: i32,
    pub SkipSteadyState: i32,
    pub IgnoreRainfall: i32,
    pub IgnoreRDII: i32,
    pub IgnoreSnowmelt: i32,
    pub IgnoreGwater: i32,
    pub IgnoreRouting: i32,
    pub IgnoreQuality: i32,
    pub ErrorCode: i32,
    pub Warnings: i32,
    pub WetStep: i32,
    pub DryStep: i32,
    pub ReportStep: i32,
    pub RuleStep: i32,
    pub SweepStart: i32,
    pub SweepEnd: i32,
    pub MaxTrials: i32,
    pub NumThreads: i32,
    pub Nobjects: [i32; 18],
    pub Nnodes: [i32; 5],
    pub Nlinks: [i32; 5],
    pub UnitSystem: i32,
    pub FlowUnits: i32,
    pub InfilModel: i32,
    pub RouteModel: i32,
    pub ForceMainEqn: i32,
    pub LinkOffsets: i32,
    pub SurchargeMethod: i32,
    pub AllowPonding: i32,
    pub InertDamping: i32,
    pub SysFlowTol: f64,
    pub QualError: f64,
    pub LatFlowTol: f64,
    pub RouteStep: f64,
    pub HeadTol: f64,
    pub MinRouteStep: f64,
    pub LengtheningStep: f64,
    pub StartDryDays: f64,
    pub CourantFactor: f64,
    pub MinSurfArea: f64,
    pub MinSlope: f64,
    pub RunoffError: f64,
    pub GwaterError: f64,
    pub FlowError: f64,
    pub CrownCutoff: f64,
    pub StartDate: DateTime,
    pub StartTime: DateTime,
    pub StartDateTime: DateTime,
    pub EndDate: DateTime,
    pub EndTime: DateTime,
    pub EndDateTime: DateTime,
    pub ReportStartDate: DateTime,
    pub ReportStartTime: DateTime,
    pub ReportStart: DateTime,
    pub TotalDuration: f64,
    pub ElapsedTime: f64,
    pub ReportTime: f64,
    pub OldRunoffTime: f64,
    pub NewRunoffTime: f64,
    pub OldRoutingTime: f64,
    pub NewRoutingTime: f64,
    pub Temp: TTemp,
    pub Evap: TEvap,
    pub Wind: TWind,
    pub Snow: TSnow,
    pub Adjust: TAdjust,
    // ---- object arrays (C: malloc'd arrays indexed 0..Nobjects[type]-1) ----
    pub Snowmelt: Vec<TSnowmelt>,
    pub Gage: Vec<TGage>,
    pub Subcatch: Vec<TSubcatch>,
    pub Aquifer: Vec<TAquifer>,
    pub UnitHyd: Vec<TUnitHyd>,
    pub Node: Vec<TNode>,
    pub Outfall: Vec<TOutfall>,
    pub Divider: Vec<TDivider>,
    pub Storage: Vec<TStorage>,
    pub Link: Vec<TLink>,
    pub Conduit: Vec<TConduit>,
    pub Pump: Vec<TPump>,
    pub Orifice: Vec<TOrifice>,
    pub Weir: Vec<TWeir>,
    pub Outlet: Vec<TOutlet>,
    pub Pollut: Vec<TPollut>,
    pub Landuse: Vec<TLanduse>,
    pub Pattern: Vec<TPattern>,
    pub Curve: Vec<TTable>,
    pub Tseries: Vec<TTable>,
    pub Transect: Vec<TTransect>,
    pub Street: Vec<TStreet>,
    pub Shape: Vec<TShape>,
    pub Event: Vec<TEvent>,
    // ---- file-scope variables of each C module ----
'''
for m in MODULES:
    if m == 'keywords': continue
    G += f'    pub {m}: crate::{m}::{camel(m)}State,\n'
G += '''}

impl Default for Project {
    fn default() -> Self {
        Project {
            Finp: Default::default(), Fout: Default::default(), Frpt: Default::default(),
            Fclimate: Default::default(), Frain: Default::default(), Frunoff: Default::default(),
            Frdii: Default::default(), Fhotstart1: Default::default(), Fhotstart2: Default::default(),
            Finflows: Default::default(), Foutflows: Default::default(),
            Nperiods: 0, TotalStepCount: 0, ReportStepCount: 0, NonConvergeCount: 0,
            Msg: [0; 1025], ErrorMsg: [0; 1025], Title: [[0; 1025]; 3], TempDir: [0; 260], InpDir: [0; 260],
            RptFlags: Default::default(), NumEvents: 0, NormalFlowLtd: 0, SlopeWeighting: 0, Compatibility: 0,
            SkipSteadyState: 0, IgnoreRainfall: 0, IgnoreRDII: 0, IgnoreSnowmelt: 0, IgnoreGwater: 0,
            IgnoreRouting: 0, IgnoreQuality: 0, ErrorCode: 0, Warnings: 0, WetStep: 0, DryStep: 0,
            ReportStep: 0, RuleStep: 0, SweepStart: 0, SweepEnd: 0, MaxTrials: 0, NumThreads: 0,
            Nobjects: [0; 18], Nnodes: [0; 5], Nlinks: [0; 5], UnitSystem: 0, FlowUnits: 0, InfilModel: 0,
            RouteModel: 0, ForceMainEqn: 0, LinkOffsets: 0, SurchargeMethod: 0, AllowPonding: 0,
            InertDamping: 0, SysFlowTol: 0.0, QualError: 0.0, LatFlowTol: 0.0, RouteStep: 0.0, HeadTol: 0.0,
            MinRouteStep: 0.0, LengtheningStep: 0.0, StartDryDays: 0.0, CourantFactor: 0.0,
            MinSurfArea: 0.0, MinSlope: 0.0, RunoffError: 0.0, GwaterError: 0.0, FlowError: 0.0,
            CrownCutoff: 0.0, StartDate: 0.0, StartTime: 0.0, StartDateTime: 0.0, EndDate: 0.0,
            EndTime: 0.0, EndDateTime: 0.0, ReportStartDate: 0.0, ReportStartTime: 0.0, ReportStart: 0.0,
            TotalDuration: 0.0, ElapsedTime: 0.0, ReportTime: 0.0, OldRunoffTime: 0.0, NewRunoffTime: 0.0,
            OldRoutingTime: 0.0, NewRoutingTime: 0.0, Temp: Default::default(), Evap: Default::default(),
            Wind: Default::default(), Snow: Default::default(), Adjust: Default::default(),
            Snowmelt: Vec::new(), Gage: Vec::new(), Subcatch: Vec::new(), Aquifer: Vec::new(),
            UnitHyd: Vec::new(), Node: Vec::new(), Outfall: Vec::new(), Divider: Vec::new(),
            Storage: Vec::new(), Link: Vec::new(), Conduit: Vec::new(), Pump: Vec::new(),
            Orifice: Vec::new(), Weir: Vec::new(), Outlet: Vec::new(), Pollut: Vec::new(),
            Landuse: Vec::new(), Pattern: Vec::new(), Curve: Vec::new(), Tseries: Vec::new(),
            Transect: Vec::new(), Street: Vec::new(), Shape: Vec::new(), Event: Vec::new(),
'''
for m in MODULES:
    if m == 'keywords': continue
    G += f'            {m}: Default::default(),\n'
G += '''        }
    }
}
'''
open(os.path.join(OUTDIR, 'globals.rs'), 'w').write(G)
# prelude
P = '''//! What every module of the engine sees: the constants, the types, the
//! model, the C library shims, the macros of macros.h, and every module's
//! public functions by their C names (C has one global namespace).

pub use crate::ccast::*;
pub use crate::cio::{self, CFile, Scan};
pub use crate::consts::*;
pub use crate::enums::*;
pub use crate::globals::Project;
pub use crate::keywords::*;
pub use crate::macros::*;
pub use crate::objects::*;
pub use crate::text::*;
pub use crate::{cformat, cprintf};
'''
for m in MODULES:
    if m == 'keywords': continue
    P += f'pub use crate::{m}::*;\n'
open(os.path.join(OUTDIR, 'prelude.rs'), 'w').write(P)
L = '''//! # swmm5-rust
//!
//! EPA SWMM 5.2.4 in safe Rust. Every simulation is a value of type
//! [`Project`]; nothing is global, and the crate contains no `unsafe` code.
//! Function and field names are those of EPA's C, so the source can be
//! read side by side with it.
#![forbid(unsafe_code)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![allow(clippy::all)]
// during translation only; removed once every module is in
#![allow(unused_variables, unused_mut, dead_code, unused_imports, unreachable_code)]

pub mod ccast;
pub mod cio;
pub mod consts;
pub mod enums;
pub mod globals;
pub mod keywords;
pub mod macros;
pub mod objects;
pub mod prelude;
pub mod text;

'''
for m in MODULES:
    if m == 'keywords': continue
    L += f'pub mod {m};\n'
L += '\npub use globals::Project;\n'
open(os.path.join(OUTDIR, 'lib.rs'), 'w').write(L)
print('ok')
