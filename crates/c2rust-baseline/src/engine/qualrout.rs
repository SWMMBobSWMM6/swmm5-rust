#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn inlet_capturedFlow(link: ::core::ffi::c_int) -> ::core::ffi::c_double;
    static mut RouteModel: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut Node: *mut TNode;
    static mut Storage: *mut TStorage;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Pollut: *mut TPollut;
    fn treatmnt_treat(
        node: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        v: ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
    );
    fn treatmnt_setInflow(qIn: ::core::ffi::c_double, wIn: *mut ::core::ffi::c_double);
    fn massbal_addReactedMass(pollut: ::core::ffi::c_int, mass: ::core::ffi::c_double);
    fn massbal_addSeepageLoss(pollut: ::core::ffi::c_int, seepLoss: ::core::ffi::c_double);
    fn massbal_addToFinalStorage(pollut: ::core::ffi::c_int, mass: ::core::ffi::c_double);
}
pub type ObjectType = ::core::ffi::c_uint;
pub const MAX_OBJ_TYPES: ObjectType = 18;
pub const INLET: ObjectType = 17;
pub const STREET: ObjectType = 16;
pub const LID: ObjectType = 15;
pub const SHAPE: ObjectType = 14;
pub const SNOWMELT: ObjectType = 13;
pub const UNITHYD: ObjectType = 12;
pub const AQUIFER: ObjectType = 11;
pub const TRANSECT: ObjectType = 10;
pub const CONTROL: ObjectType = 9;
pub const TSERIES: ObjectType = 8;
pub const CURVE: ObjectType = 7;
pub const TIMEPATTERN: ObjectType = 6;
pub const LANDUSE: ObjectType = 5;
pub const POLLUT: ObjectType = 4;
pub const LINK: ObjectType = 3;
pub const NODE: ObjectType = 2;
pub const SUBCATCH: ObjectType = 1;
pub const GAGE: ObjectType = 0;
pub type NodeType = ::core::ffi::c_uint;
pub const DIVIDER: NodeType = 3;
pub const STORAGE: NodeType = 2;
pub const OUTFALL: NodeType = 1;
pub const JUNCTION: NodeType = 0;
pub type LinkType = ::core::ffi::c_uint;
pub const OUTLET: LinkType = 4;
pub const WEIR: LinkType = 3;
pub const ORIFICE: LinkType = 2;
pub const PUMP: LinkType = 1;
pub const CONDUIT: LinkType = 0;
pub type XsectType = ::core::ffi::c_uint;
pub const STREET_XSECT: XsectType = 25;
pub const FORCE_MAIN: XsectType = 24;
pub const CUSTOM: XsectType = 23;
pub const IRREGULAR: XsectType = 22;
pub const SEMICIRCULAR: XsectType = 21;
pub const BASKETHANDLE: XsectType = 20;
pub const SEMIELLIPTICAL: XsectType = 19;
pub const CATENARY: XsectType = 18;
pub const GOTHIC: XsectType = 17;
pub const HORSESHOE: XsectType = 16;
pub const EGGSHAPED: XsectType = 15;
pub const ARCH: XsectType = 14;
pub const VERT_ELLIPSE: XsectType = 13;
pub const HORIZ_ELLIPSE: XsectType = 12;
pub const MOD_BASKET: XsectType = 11;
pub const RECT_ROUND: XsectType = 10;
pub const RECT_TRIANG: XsectType = 9;
pub const POWERFUNC: XsectType = 8;
pub const PARABOLIC: XsectType = 7;
pub const TRIANGULAR: XsectType = 6;
pub const TRAPEZOIDAL: XsectType = 5;
pub const RECT_OPEN: XsectType = 4;
pub const RECT_CLOSED: XsectType = 3;
pub const FILLED_CIRCULAR: XsectType = 2;
pub const CIRCULAR: XsectType = 1;
pub const DUMMY: XsectType = 0;
pub type RouteModelType = ::core::ffi::c_uint;
pub const DW: RouteModelType = 4;
pub const EKW: RouteModelType = 3;
pub const KW: RouteModelType = 2;
pub const SF: RouteModelType = 1;
pub const NO_ROUTING: RouteModelType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprNode {
    pub opcode: ::core::ffi::c_int,
    pub ivar: ::core::ffi::c_int,
    pub fvalue: ::core::ffi::c_double,
    pub prev: *mut ExprNode,
    pub next: *mut ExprNode,
}
pub type MathExpr = ExprNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TGrnAmpt {
    pub S: ::core::ffi::c_double,
    pub Ks: ::core::ffi::c_double,
    pub IMDmax: ::core::ffi::c_double,
    pub IMD: ::core::ffi::c_double,
    pub F: ::core::ffi::c_double,
    pub Fu: ::core::ffi::c_double,
    pub Lu: ::core::ffi::c_double,
    pub T: ::core::ffi::c_double,
    pub Sat: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TExfil {
    pub btmExfil: *mut TGrnAmpt,
    pub bankExfil: *mut TGrnAmpt,
    pub btmArea: ::core::ffi::c_double,
    pub bankMinDepth: ::core::ffi::c_double,
    pub bankMaxDepth: ::core::ffi::c_double,
    pub bankMaxArea: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtInflow {
    pub param: ::core::ffi::c_int,
    pub type_0: ::core::ffi::c_int,
    pub tSeries: ::core::ffi::c_int,
    pub basePat: ::core::ffi::c_int,
    pub cFactor: ::core::ffi::c_double,
    pub baseline: ::core::ffi::c_double,
    pub sFactor: ::core::ffi::c_double,
    pub next: *mut ExtInflow,
}
pub type TExtInflow = ExtInflow;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DwfInflow {
    pub param: ::core::ffi::c_int,
    pub avgValue: ::core::ffi::c_double,
    pub patterns: [::core::ffi::c_int; 4],
    pub next: *mut DwfInflow,
}
pub type TDwfInflow = DwfInflow;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TRdiiInflow {
    pub unitHyd: ::core::ffi::c_int,
    pub area: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTreatment {
    pub treatType: ::core::ffi::c_int,
    pub equation: *mut MathExpr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TNode {
    pub ID: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub subIndex: ::core::ffi::c_int,
    pub rptFlag: ::core::ffi::c_int,
    pub invertElev: ::core::ffi::c_double,
    pub initDepth: ::core::ffi::c_double,
    pub fullDepth: ::core::ffi::c_double,
    pub surDepth: ::core::ffi::c_double,
    pub pondedArea: ::core::ffi::c_double,
    pub extInflow: *mut TExtInflow,
    pub dwfInflow: *mut TDwfInflow,
    pub rdiiInflow: *mut TRdiiInflow,
    pub treatment: *mut TTreatment,
    pub degree: ::core::ffi::c_int,
    pub inlet: ::core::ffi::c_int,
    pub updated: ::core::ffi::c_char,
    pub crownElev: ::core::ffi::c_double,
    pub inflow: ::core::ffi::c_double,
    pub outflow: ::core::ffi::c_double,
    pub losses: ::core::ffi::c_double,
    pub oldVolume: ::core::ffi::c_double,
    pub newVolume: ::core::ffi::c_double,
    pub fullVolume: ::core::ffi::c_double,
    pub overflow: ::core::ffi::c_double,
    pub oldDepth: ::core::ffi::c_double,
    pub newDepth: ::core::ffi::c_double,
    pub oldLatFlow: ::core::ffi::c_double,
    pub newLatFlow: ::core::ffi::c_double,
    pub oldQual: *mut ::core::ffi::c_double,
    pub newQual: *mut ::core::ffi::c_double,
    pub oldFlowInflow: ::core::ffi::c_double,
    pub oldNetInflow: ::core::ffi::c_double,
    pub qualInflow: ::core::ffi::c_double,
    pub apiExtInflow: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TStorage {
    pub fEvap: ::core::ffi::c_double,
    pub a0: ::core::ffi::c_double,
    pub a1: ::core::ffi::c_double,
    pub a2: ::core::ffi::c_double,
    pub aCurve: ::core::ffi::c_int,
    pub shape: ::core::ffi::c_int,
    pub exfil: *mut TExfil,
    pub hrt: ::core::ffi::c_double,
    pub evapLoss: ::core::ffi::c_double,
    pub exfilLoss: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TXsect {
    pub type_0: ::core::ffi::c_int,
    pub culvertCode: ::core::ffi::c_int,
    pub transect: ::core::ffi::c_int,
    pub yFull: ::core::ffi::c_double,
    pub wMax: ::core::ffi::c_double,
    pub ywMax: ::core::ffi::c_double,
    pub aFull: ::core::ffi::c_double,
    pub rFull: ::core::ffi::c_double,
    pub sFull: ::core::ffi::c_double,
    pub sMax: ::core::ffi::c_double,
    pub yBot: ::core::ffi::c_double,
    pub aBot: ::core::ffi::c_double,
    pub sBot: ::core::ffi::c_double,
    pub rBot: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLink {
    pub ID: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub subIndex: ::core::ffi::c_int,
    pub rptFlag: ::core::ffi::c_int,
    pub node1: ::core::ffi::c_int,
    pub node2: ::core::ffi::c_int,
    pub offset1: ::core::ffi::c_double,
    pub offset2: ::core::ffi::c_double,
    pub xsect: TXsect,
    pub q0: ::core::ffi::c_double,
    pub qLimit: ::core::ffi::c_double,
    pub cLossInlet: ::core::ffi::c_double,
    pub cLossOutlet: ::core::ffi::c_double,
    pub cLossAvg: ::core::ffi::c_double,
    pub seepRate: ::core::ffi::c_double,
    pub hasFlapGate: ::core::ffi::c_int,
    pub inlet: *mut TInlet,
    pub oldFlow: ::core::ffi::c_double,
    pub newFlow: ::core::ffi::c_double,
    pub oldDepth: ::core::ffi::c_double,
    pub newDepth: ::core::ffi::c_double,
    pub oldVolume: ::core::ffi::c_double,
    pub newVolume: ::core::ffi::c_double,
    pub surfArea1: ::core::ffi::c_double,
    pub surfArea2: ::core::ffi::c_double,
    pub qFull: ::core::ffi::c_double,
    pub setting: ::core::ffi::c_double,
    pub targetSetting: ::core::ffi::c_double,
    pub timeLastSet: ::core::ffi::c_double,
    pub froude: ::core::ffi::c_double,
    pub oldQual: *mut ::core::ffi::c_double,
    pub newQual: *mut ::core::ffi::c_double,
    pub totalLoad: *mut ::core::ffi::c_double,
    pub flowClass: ::core::ffi::c_int,
    pub dqdh: ::core::ffi::c_double,
    pub direction: ::core::ffi::c_schar,
    pub bypassed: ::core::ffi::c_char,
    pub normalFlow: ::core::ffi::c_char,
    pub inletControl: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TConduit {
    pub length: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub barrels: ::core::ffi::c_char,
    pub modLength: ::core::ffi::c_double,
    pub roughFactor: ::core::ffi::c_double,
    pub slope: ::core::ffi::c_double,
    pub beta: ::core::ffi::c_double,
    pub qMax: ::core::ffi::c_double,
    pub a1: ::core::ffi::c_double,
    pub a2: ::core::ffi::c_double,
    pub q1: ::core::ffi::c_double,
    pub q2: ::core::ffi::c_double,
    pub q1Old: ::core::ffi::c_double,
    pub q2Old: ::core::ffi::c_double,
    pub evapLossRate: ::core::ffi::c_double,
    pub seepLossRate: ::core::ffi::c_double,
    pub capacityLimited: ::core::ffi::c_char,
    pub superCritical: ::core::ffi::c_char,
    pub hasLosses: ::core::ffi::c_char,
    pub fullState: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPollut {
    pub ID: *mut ::core::ffi::c_char,
    pub units: ::core::ffi::c_int,
    pub mcf: ::core::ffi::c_double,
    pub dwfConcen: ::core::ffi::c_double,
    pub pptConcen: ::core::ffi::c_double,
    pub gwConcen: ::core::ffi::c_double,
    pub rdiiConcen: ::core::ffi::c_double,
    pub initConcen: ::core::ffi::c_double,
    pub kDecay: ::core::ffi::c_double,
    pub coPollut: ::core::ffi::c_int,
    pub coFraction: ::core::ffi::c_double,
    pub snowOnly: ::core::ffi::c_int,
}
pub const ZERO: ::core::ffi::c_double = 1.0E-10f64;
static mut ZeroVolume: ::core::ffi::c_double = 0.0353147f64;
static mut ZeroDepth: ::core::ffi::c_double = 0.003281f64;
#[no_mangle]
pub unsafe extern "C" fn qualrout_init() {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut isWet: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        isWet = ((*Node.offset(i as isize)).newDepth > ZeroDepth) as ::core::ffi::c_int;
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            c = 0.0f64;
            if isWet != 0 {
                c = (*Pollut.offset(p as isize)).initConcen;
            }
            *(*Node.offset(i as isize)).oldQual.offset(p as isize) = c;
            *(*Node.offset(i as isize)).newQual.offset(p as isize) = c;
            p += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        isWet = ((*Link.offset(i as isize)).newDepth > ZeroDepth) as ::core::ffi::c_int;
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            c = 0.0f64;
            if isWet != 0 {
                c = (*Pollut.offset(p as isize)).initConcen;
            }
            *(*Link.offset(i as isize)).oldQual.offset(p as isize) = c;
            *(*Link.offset(i as isize)).newQual.offset(p as isize) = c;
            p += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn qualrout_execute(mut tStep: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut qIn: ::core::ffi::c_double = 0.;
    let mut vAvg: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        findLinkMassFlow(i, tStep);
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(j as isize)).qualInflow = (*Node.offset(j as isize)).inflow;
        qIn = (*Node.offset(j as isize)).qualInflow;
        vAvg =
            ((*Node.offset(j as isize)).oldVolume + (*Node.offset(j as isize)).newVolume) / 2.0f64;
        if !(*Node.offset(j as isize)).treatment.is_null() {
            if qIn < ZERO {
                qIn = 0.0f64;
            }
            treatmnt_setInflow(
                qIn,
                (*Node.offset(j as isize)).newQual as *mut ::core::ffi::c_double,
            );
        }
        if (*Node.offset(j as isize)).type_0 == STORAGE as ::core::ffi::c_int
            || (*Node.offset(j as isize)).oldVolume > ZeroVolume
        {
            findStorageQual(j, tStep);
        } else {
            findNodeQual(j);
        }
        if !(*Node.offset(j as isize)).treatment.is_null() {
            treatmnt_treat(j, qIn, vAvg, tStep);
        }
        j += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        findLinkQual(i, tStep);
        i += 1;
    }
}
unsafe extern "C" fn getMixedQual(
    mut c: ::core::ffi::c_double,
    mut v1: ::core::ffi::c_double,
    mut wIn: ::core::ffi::c_double,
    mut qIn: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut vIn: ::core::ffi::c_double = 0.;
    let mut cIn: ::core::ffi::c_double = 0.;
    let mut cMax: ::core::ffi::c_double = 0.;
    if qIn <= ZERO {
        return c;
    }
    vIn = qIn * tStep;
    cIn = wIn * tStep / vIn;
    cMax = if c >= cIn { c } else { cIn };
    c = (c * v1 + wIn * tStep) / (v1 + vIn);
    c = if c <= cMax { c } else { cMax };
    c = if c >= 0.0f64 { c } else { 0.0f64 };
    return c;
}
unsafe extern "C" fn findLinkMassFlow(mut i: ::core::ffi::c_int, mut tStep: ::core::ffi::c_double) {
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut qLink: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    qLink = (*Link.offset(i as isize)).newFlow;
    j = (*Link.offset(i as isize)).node2;
    if qLink < 0.0f64 {
        j = (*Link.offset(i as isize)).node1;
    }
    qLink = fabs(qLink);
    if RouteModel != DW as ::core::ffi::c_int {
        qLink -= inlet_capturedFlow(i);
    }
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        w = qLink * *(*Link.offset(i as isize)).oldQual.offset(p as isize);
        *(*Node.offset(j as isize)).newQual.offset(p as isize) += w;
        *(*Link.offset(i as isize)).totalLoad.offset(p as isize) += w * tStep;
        p += 1;
    }
    (*Node.offset(j as isize)).qualInflow += qLink;
}
unsafe extern "C" fn findNodeQual(mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    let mut qNode: ::core::ffi::c_double = 0.;
    qNode = (*Node.offset(j as isize)).qualInflow;
    if qNode > ZERO {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *(*Node.offset(j as isize)).newQual.offset(p as isize) /= qNode;
            p += 1;
        }
    } else {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            if (*Node.offset(j as isize)).newDepth > ZeroDepth {
                *(*Node.offset(j as isize)).newQual.offset(p as isize) =
                    *(*Node.offset(j as isize)).oldQual.offset(p as isize);
            } else {
                *(*Node.offset(j as isize)).newQual.offset(p as isize) = 0.0f64;
            }
            p += 1;
        }
    };
}
unsafe extern "C" fn findLinkQual(mut i: ::core::ffi::c_int, mut tStep: ::core::ffi::c_double) {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut wIn: ::core::ffi::c_double = 0.;
    let mut qIn: ::core::ffi::c_double = 0.;
    let mut qSeep: ::core::ffi::c_double = 0.;
    let mut v1: ::core::ffi::c_double = 0.;
    let mut v2: ::core::ffi::c_double = 0.;
    let mut c1: ::core::ffi::c_double = 0.;
    let mut c2: ::core::ffi::c_double = 0.;
    let mut vEvap: ::core::ffi::c_double = 0.;
    let mut vLosses: ::core::ffi::c_double = 0.;
    let mut fEvap: ::core::ffi::c_double = 0.;
    let mut barrels: ::core::ffi::c_double = 0.;
    j = (*Link.offset(i as isize)).node1;
    if (*Link.offset(i as isize)).newFlow < 0.0f64 {
        j = (*Link.offset(i as isize)).node2;
    }
    if (*Link.offset(i as isize)).type_0 != CONDUIT as ::core::ffi::c_int
        || (*Link.offset(i as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int
    {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *(*Link.offset(i as isize)).newQual.offset(p as isize) =
                *(*Node.offset(j as isize)).newQual.offset(p as isize);
            p += 1;
        }
        return;
    }
    k = (*Link.offset(i as isize)).subIndex;
    barrels = (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_double;
    qIn = fabs((*Conduit.offset(k as isize)).q1) * barrels;
    qSeep = (*Conduit.offset(k as isize)).seepLossRate * barrels;
    vEvap = (*Conduit.offset(k as isize)).evapLossRate * barrels * tStep;
    v1 = (*Link.offset(i as isize)).oldVolume;
    v2 = (*Link.offset(i as isize)).newVolume;
    vLosses = qSeep * tStep + vEvap;
    fEvap = 1.0f64;
    if vEvap > 0.0f64 && v1 > ZeroVolume {
        fEvap += vEvap / v1;
    }
    if RouteModel == SF as ::core::ffi::c_int {
        findSFLinkQual(i, qSeep, fEvap, tStep);
        return;
    }
    if RouteModel == DW as ::core::ffi::c_int {
        qIn = qIn + (v2 + vLosses - v1) / tStep;
        qIn = if qIn >= 0.0f64 { qIn } else { 0.0f64 };
    }
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        c1 = *(*Link.offset(i as isize)).oldQual.offset(p as isize);
        massbal_addSeepageLoss(p, qSeep * c1);
        c1 *= fEvap;
        c2 = getReactedQual(p, c1, v1, tStep);
        wIn = *(*Node.offset(j as isize)).newQual.offset(p as isize) * qIn;
        c2 = getMixedQual(c2, v1, wIn, qIn, tStep);
        if v2 < ZeroVolume || (*Link.offset(i as isize)).newDepth <= ZeroDepth {
            massbal_addToFinalStorage(p, c2 * v2);
            c2 = 0.0f64;
        }
        *(*Link.offset(i as isize)).newQual.offset(p as isize) = c2;
        p += 1;
    }
}
unsafe extern "C" fn findSFLinkQual(
    mut i: ::core::ffi::c_int,
    mut qSeep: ::core::ffi::c_double,
    mut fEvap: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut j: ::core::ffi::c_int = (*Link.offset(i as isize)).node1;
    let mut p: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_double = 0.;
    let mut c2: ::core::ffi::c_double = 0.;
    let mut lossRate: ::core::ffi::c_double = 0.;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        c1 = *(*Node.offset(j as isize)).newQual.offset(p as isize);
        massbal_addSeepageLoss(p, qSeep * c1);
        c1 *= fEvap;
        c2 = c1;
        if (*Pollut.offset(p as isize)).kDecay > 0.0f64 {
            c2 = c1 * exp(-(*Pollut.offset(p as isize)).kDecay * tStep);
            c2 = if 0.0f64 >= c2 { 0.0f64 } else { c2 };
            lossRate = (c1 - c2) * (*Link.offset(i as isize)).newFlow;
            massbal_addReactedMass(p, lossRate);
        }
        *(*Link.offset(i as isize)).newQual.offset(p as isize) = c2;
        p += 1;
    }
}
unsafe extern "C" fn findStorageQual(mut j: ::core::ffi::c_int, mut tStep: ::core::ffi::c_double) {
    let mut p: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut qIn: ::core::ffi::c_double = 0.;
    let mut wIn: ::core::ffi::c_double = 0.;
    let mut v1: ::core::ffi::c_double = 0.;
    let mut c1: ::core::ffi::c_double = 0.;
    let mut c2: ::core::ffi::c_double = 0.;
    let mut qExfil: ::core::ffi::c_double = 0.0f64;
    let mut vEvap: ::core::ffi::c_double = 0.0f64;
    let mut fEvap: ::core::ffi::c_double = 1.0f64;
    qIn = (*Node.offset(j as isize)).qualInflow;
    v1 = (*Node.offset(j as isize)).oldVolume;
    if (*Node.offset(j as isize)).type_0 == STORAGE as ::core::ffi::c_int {
        updateHRT(j, (*Node.offset(j as isize)).oldVolume, qIn, tStep);
        k = (*Node.offset(j as isize)).subIndex;
        qExfil = (*Storage.offset(k as isize)).exfilLoss / tStep;
        vEvap = (*Storage.offset(k as isize)).evapLoss;
        if vEvap > 0.0f64 && v1 > ZeroVolume {
            fEvap += vEvap / v1;
        }
    }
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        c1 = *(*Node.offset(j as isize)).oldQual.offset(p as isize);
        massbal_addSeepageLoss(p, qExfil * c1);
        c1 *= fEvap;
        if (*Node.offset(j as isize)).treatment.is_null()
            || (*(*Node.offset(j as isize)).treatment.offset(p as isize))
                .equation
                .is_null()
        {
            c1 = getReactedQual(p, c1, v1, tStep);
        }
        wIn = *(*Node.offset(j as isize)).newQual.offset(p as isize);
        c2 = getMixedQual(c1, v1, wIn, qIn, tStep);
        if ((*Node.offset(j as isize)).newVolume <= ZeroVolume
            || (*Node.offset(j as isize)).newDepth <= ZeroDepth)
            && qIn <= ZERO
        {
            massbal_addToFinalStorage(p, c2 * (*Node.offset(j as isize)).newVolume);
            c2 = 0.0f64;
        }
        *(*Node.offset(j as isize)).newQual.offset(p as isize) = c2;
        p += 1;
    }
}
unsafe extern "C" fn updateHRT(
    mut j: ::core::ffi::c_int,
    mut v: ::core::ffi::c_double,
    mut q: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut k: ::core::ffi::c_int = (*Node.offset(j as isize)).subIndex;
    let mut hrt: ::core::ffi::c_double = (*Storage.offset(k as isize)).hrt;
    if v < ZERO {
        hrt = 0.0f64;
    } else {
        hrt = (hrt + tStep) * v / (v + q * tStep);
    }
    (*Storage.offset(k as isize)).hrt = if hrt >= 0.0f64 { hrt } else { 0.0f64 };
}
unsafe extern "C" fn getReactedQual(
    mut p: ::core::ffi::c_int,
    mut c: ::core::ffi::c_double,
    mut v1: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut c2: ::core::ffi::c_double = 0.;
    let mut lossRate: ::core::ffi::c_double = 0.;
    let mut kDecay: ::core::ffi::c_double = (*Pollut.offset(p as isize)).kDecay;
    if kDecay == 0.0f64 {
        return c;
    }
    c2 = c * (1.0f64 - kDecay * tStep);
    c2 = if 0.0f64 >= c2 { 0.0f64 } else { c2 };
    lossRate = (c - c2) * v1 / tStep;
    massbal_addReactedMass(p, lossRate);
    return c2;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
