#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn findroot_Ridder(
        x1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        xacc: ::core::ffi::c_double,
        func: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_double,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_double,
        >,
        p: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_double;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    fn xsect_getAofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getWofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type LinkType = ::core::ffi::c_uint;
pub const OUTLET: LinkType = 4;
pub const WEIR: LinkType = 3;
pub const ORIFICE: LinkType = 2;
pub const PUMP: LinkType = 1;
pub const CONDUIT: LinkType = 0;
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
pub struct TCulvert {
    pub yFull: ::core::ffi::c_double,
    pub scf: ::core::ffi::c_double,
    pub dQdH: ::core::ffi::c_double,
    pub qc: ::core::ffi::c_double,
    pub kk: ::core::ffi::c_double,
    pub mm: ::core::ffi::c_double,
    pub ad: ::core::ffi::c_double,
    pub hPlus: ::core::ffi::c_double,
    pub xsect: *mut TXsect,
}
pub const FORM: CulvertParam = 0;
pub const M: CulvertParam = 2;
pub const K: CulvertParam = 1;
pub const C: CulvertParam = 3;
pub const Y: CulvertParam = 4;
pub type CulvertParam = ::core::ffi::c_uint;
static mut MAX_CULVERT_CODE: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
static mut Params: [[::core::ffi::c_double; 5]; 58] = [
    [0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.00f64],
    [1.0f64, 0.0098f64, 2.00f64, 0.0398f64, 0.67f64],
    [1.0f64, 0.0018f64, 2.00f64, 0.0292f64, 0.74f64],
    [1.0f64, 0.0045f64, 2.00f64, 0.0317f64, 0.69f64],
    [1.0f64, 0.0078f64, 2.00f64, 0.0379f64, 0.69f64],
    [1.0f64, 0.0210f64, 1.33f64, 0.0463f64, 0.75f64],
    [1.0f64, 0.0340f64, 1.50f64, 0.0553f64, 0.54f64],
    [1.0f64, 0.0018f64, 2.50f64, 0.0300f64, 0.74f64],
    [1.0f64, 0.0018f64, 2.50f64, 0.0243f64, 0.83f64],
    [1.0f64, 0.026f64, 1.0f64, 0.0347f64, 0.81f64],
    [1.0f64, 0.061f64, 0.75f64, 0.0400f64, 0.80f64],
    [1.0f64, 0.061f64, 0.75f64, 0.0423f64, 0.82f64],
    [2.0f64, 0.510f64, 0.667f64, 0.0309f64, 0.80f64],
    [2.0f64, 0.486f64, 0.667f64, 0.0249f64, 0.83f64],
    [2.0f64, 0.515f64, 0.667f64, 0.0375f64, 0.79f64],
    [2.0f64, 0.495f64, 0.667f64, 0.0314f64, 0.82f64],
    [2.0f64, 0.486f64, 0.667f64, 0.0252f64, 0.865f64],
    [2.0f64, 0.545f64, 0.667f64, 0.04505f64, 0.73f64],
    [2.0f64, 0.533f64, 0.667f64, 0.0425f64, 0.705f64],
    [2.0f64, 0.522f64, 0.667f64, 0.0402f64, 0.68f64],
    [2.0f64, 0.498f64, 0.667f64, 0.0327f64, 0.75f64],
    [2.0f64, 0.497f64, 0.667f64, 0.0339f64, 0.803f64],
    [2.0f64, 0.493f64, 0.667f64, 0.0361f64, 0.806f64],
    [2.0f64, 0.495f64, 0.667f64, 0.0386f64, 0.71f64],
    [2.0f64, 0.497f64, 0.667f64, 0.0302f64, 0.835f64],
    [2.0f64, 0.495f64, 0.667f64, 0.0252f64, 0.881f64],
    [2.0f64, 0.493f64, 0.667f64, 0.0227f64, 0.887f64],
    [1.0f64, 0.0083f64, 2.00f64, 0.0379f64, 0.69f64],
    [1.0f64, 0.0145f64, 1.75f64, 0.0419f64, 0.64f64],
    [1.0f64, 0.0340f64, 1.50f64, 0.0496f64, 0.57f64],
    [1.0f64, 0.0100f64, 2.00f64, 0.0398f64, 0.67f64],
    [1.0f64, 0.0018f64, 2.50f64, 0.0292f64, 0.74f64],
    [1.0f64, 0.0045f64, 2.00f64, 0.0317f64, 0.69f64],
    [1.0f64, 0.0100f64, 2.00f64, 0.0398f64, 0.67f64],
    [1.0f64, 0.0018f64, 2.50f64, 0.0292f64, 0.74f64],
    [1.0f64, 0.0095f64, 2.00f64, 0.0317f64, 0.69f64],
    [1.0f64, 0.0083f64, 2.00f64, 0.0379f64, 0.69f64],
    [1.0f64, 0.0300f64, 1.00f64, 0.0463f64, 0.75f64],
    [1.0f64, 0.0340f64, 1.50f64, 0.0496f64, 0.57f64],
    [1.0f64, 0.0300f64, 1.50f64, 0.0496f64, 0.57f64],
    [1.0f64, 0.0088f64, 2.00f64, 0.0368f64, 0.68f64],
    [1.0f64, 0.0030f64, 2.00f64, 0.0269f64, 0.77f64],
    [1.0f64, 0.0300f64, 1.50f64, 0.0496f64, 0.57f64],
    [1.0f64, 0.0088f64, 2.00f64, 0.0368f64, 0.68f64],
    [1.0f64, 0.0030f64, 2.00f64, 0.0269f64, 0.77f64],
    [1.0f64, 0.0083f64, 2.00f64, 0.0379f64, 0.69f64],
    [1.0f64, 0.0300f64, 1.00f64, 0.0473f64, 0.75f64],
    [1.0f64, 0.0340f64, 1.50f64, 0.0496f64, 0.57f64],
    [2.0f64, 0.534f64, 0.555f64, 0.0196f64, 0.90f64],
    [2.0f64, 0.519f64, 0.640f64, 0.0210f64, 0.90f64],
    [2.0f64, 0.536f64, 0.622f64, 0.0368f64, 0.83f64],
    [2.0f64, 0.5035f64, 0.719f64, 0.0478f64, 0.80f64],
    [2.0f64, 0.547f64, 0.800f64, 0.0598f64, 0.75f64],
    [2.0f64, 0.475f64, 0.667f64, 0.0179f64, 0.97f64],
    [2.0f64, 0.560f64, 0.667f64, 0.0446f64, 0.85f64],
    [2.0f64, 0.560f64, 0.667f64, 0.0378f64, 0.87f64],
    [2.0f64, 0.500f64, 0.667f64, 0.0446f64, 0.65f64],
    [2.0f64, 0.500f64, 0.667f64, 0.0378f64, 0.71f64],
];
#[no_mangle]
pub unsafe extern "C" fn culvert_getInflow(
    mut j: ::core::ffi::c_int,
    mut q0: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut code: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut condition: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut culvert: TCulvert = TCulvert {
        yFull: 0.,
        scf: 0.,
        dQdH: 0.,
        qc: 0.,
        kk: 0.,
        mm: 0.,
        ad: 0.,
        hPlus: 0.,
        xsect: ::core::ptr::null_mut::<TXsect>(),
    };
    if (*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int {
        return q0;
    }
    culvert.xsect = &raw mut (*Link.offset(j as isize)).xsect;
    code = (*culvert.xsect).culvertCode;
    if code <= 0 as ::core::ffi::c_int || code > MAX_CULVERT_CODE {
        return q0;
    }
    k = (*Link.offset(j as isize)).subIndex;
    culvert.yFull = (*culvert.xsect).yFull;
    culvert.ad = (*culvert.xsect).aFull * sqrt(culvert.yFull);
    match code {
        5 | 37 | 46 => {
            culvert.scf = -7.0f64 * (*Conduit.offset(k as isize)).slope;
        }
        _ => {
            culvert.scf = 0.5f64 * (*Conduit.offset(k as isize)).slope;
        }
    }
    y = h
        - ((*Node.offset((*Link.offset(j as isize)).node1 as isize)).invertElev
            + (*Link.offset(j as isize)).offset1);
    y2 = culvert.yFull
        * (16.0f64 * Params[code as usize][C as ::core::ffi::c_int as usize]
            + Params[code as usize][Y as ::core::ffi::c_int as usize]
            - culvert.scf);
    if y >= y2 {
        q = getSubmergedFlow(code, y, &raw mut culvert);
        condition = 2 as ::core::ffi::c_int;
    } else {
        y1 = 0.95f64 * culvert.yFull;
        if y <= y1 {
            q = getUnsubmergedFlow(code, y, &raw mut culvert);
            condition = 1 as ::core::ffi::c_int;
        } else {
            q = getTransitionFlow(code, y, y1, y2, &raw mut culvert);
            condition = 0 as ::core::ffi::c_int;
        }
    }
    if q < q0 {
        (*Link.offset(j as isize)).inletControl = TRUE as ::core::ffi::c_char;
        (*Link.offset(j as isize)).dqdh = culvert.dQdH;
        return q;
    } else {
        return q0;
    };
}
unsafe extern "C" fn getUnsubmergedFlow(
    mut code: ::core::ffi::c_int,
    mut h: ::core::ffi::c_double,
    mut culvert: *mut TCulvert,
) -> ::core::ffi::c_double {
    let mut arg: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    (*culvert).kk = Params[code as usize][K as ::core::ffi::c_int as usize];
    (*culvert).mm = Params[code as usize][M as ::core::ffi::c_int as usize];
    arg = h / (*culvert).yFull / (*culvert).kk;
    if Params[code as usize][FORM as ::core::ffi::c_int as usize] == 1.0f64 {
        q = getForm1Flow(h, culvert);
    } else {
        q = (*culvert).ad * pow(arg, 1.0f64 / (*culvert).mm);
    }
    (*culvert).dQdH = q / h / (*culvert).mm;
    return q;
}
unsafe extern "C" fn getSubmergedFlow(
    mut code: ::core::ffi::c_int,
    mut h: ::core::ffi::c_double,
    mut culvert: *mut TCulvert,
) -> ::core::ffi::c_double {
    let mut cc: ::core::ffi::c_double = Params[code as usize][C as ::core::ffi::c_int as usize];
    let mut yy: ::core::ffi::c_double = Params[code as usize][Y as ::core::ffi::c_int as usize];
    let mut arg: ::core::ffi::c_double = (h / (*culvert).yFull - yy + (*culvert).scf) / cc;
    let mut q: ::core::ffi::c_double = 0.;
    if arg <= 0.0f64 {
        (*culvert).dQdH = 0.0f64;
        return BIG;
    }
    q = sqrt(arg) * (*culvert).ad;
    (*culvert).dQdH = 0.5f64 * q / arg / (*culvert).yFull / cc;
    return q;
}
unsafe extern "C" fn getTransitionFlow(
    mut code: ::core::ffi::c_int,
    mut h: ::core::ffi::c_double,
    mut h1: ::core::ffi::c_double,
    mut h2: ::core::ffi::c_double,
    mut culvert: *mut TCulvert,
) -> ::core::ffi::c_double {
    let mut q1: ::core::ffi::c_double = getUnsubmergedFlow(code, h1, culvert);
    let mut q2: ::core::ffi::c_double = getSubmergedFlow(code, h2, culvert);
    let mut q: ::core::ffi::c_double = q1 + (q2 - q1) * (h - h1) / (h2 - h1);
    (*culvert).dQdH = (q2 - q1) / (h2 - h1);
    return q;
}
unsafe extern "C" fn getForm1Flow(
    mut h: ::core::ffi::c_double,
    mut culvert: *mut TCulvert,
) -> ::core::ffi::c_double {
    (*culvert).hPlus = h / (*culvert).yFull + (*culvert).scf;
    findroot_Ridder(
        0.01f64 * h,
        h,
        0.001f64,
        Some(
            form1Eqn
                as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_double,
        ),
        culvert as *mut ::core::ffi::c_void,
    );
    return (*culvert).qc;
}
unsafe extern "C" fn form1Eqn(
    mut yc: ::core::ffi::c_double,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    let mut ac: ::core::ffi::c_double = 0.;
    let mut wc: ::core::ffi::c_double = 0.;
    let mut yh: ::core::ffi::c_double = 0.;
    let mut culvert: *mut TCulvert = p as *mut TCulvert;
    ac = xsect_getAofY((*culvert).xsect, yc);
    wc = xsect_getWofY((*culvert).xsect, yc);
    yh = ac / wc;
    (*culvert).qc = ac * sqrt(GRAVITY * yh);
    return (*culvert).hPlus
        - yc / (*culvert).yFull
        - yh / 2.0f64 / (*culvert).yFull
        - (*culvert).kk * pow((*culvert).qc / (*culvert).ad, (*culvert).mm);
}
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BIG: ::core::ffi::c_double = 1.0E10f64;
pub const GRAVITY: ::core::ffi::c_double = 32.2f64;

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
