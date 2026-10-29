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
    static mut UnitSystem: ::core::ffi::c_int;
    static mut Link: *mut TLink;
    static mut Weir: *mut TWeir;
}
pub type LinkType = ::core::ffi::c_uint;
pub const OUTLET: LinkType = 4;
pub const WEIR: LinkType = 3;
pub const ORIFICE: LinkType = 2;
pub const PUMP: LinkType = 1;
pub const CONDUIT: LinkType = 0;
pub type UnitsType = ::core::ffi::c_uint;
pub const SI: UnitsType = 1;
pub const US: UnitsType = 0;
pub type FlowClassType = ::core::ffi::c_uint;
pub const ALL_FULL: FlowClassType = 10;
pub const DN_FULL: FlowClassType = 9;
pub const UP_FULL: FlowClassType = 8;
pub const MAX_FLOW_CLASSES: FlowClassType = 7;
pub const DN_CRITICAL: FlowClassType = 6;
pub const UP_CRITICAL: FlowClassType = 5;
pub const SUPCRITICAL: FlowClassType = 4;
pub const SUBCRITICAL: FlowClassType = 3;
pub const DN_DRY: FlowClassType = 2;
pub const UP_DRY: FlowClassType = 1;
pub const DRY: FlowClassType = 0;
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
pub struct TWeir {
    pub type_0: ::core::ffi::c_int,
    pub cDisch1: ::core::ffi::c_double,
    pub cDisch2: ::core::ffi::c_double,
    pub endCon: ::core::ffi::c_double,
    pub canSurcharge: ::core::ffi::c_int,
    pub roadWidth: ::core::ffi::c_double,
    pub roadSurface: ::core::ffi::c_int,
    pub cdCurve: ::core::ffi::c_int,
    pub cSurcharge: ::core::ffi::c_double,
    pub length: ::core::ffi::c_double,
    pub slope: ::core::ffi::c_double,
    pub surfArea: ::core::ffi::c_double,
}
pub const PAVED: RoadSurface = 1;
pub type RoadSurface = ::core::ffi::c_uint;
pub const GRAVEL: RoadSurface = 2;
static mut N_Cr_Low_Paved: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut Cr_Low_Paved: [[::core::ffi::c_double; 2]; 4] = [
    [0.0f64, 2.85f64],
    [0.2f64, 2.95f64],
    [0.7f64, 3.03f64],
    [4.0f64, 3.05f64],
];
static mut N_Cr_Low_Gravel: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut Cr_Low_Gravel: [[::core::ffi::c_double; 2]; 8] = [
    [0.0f64, 2.5f64],
    [0.5f64, 2.7f64],
    [1.0f64, 2.8f64],
    [1.5f64, 2.9f64],
    [2.0f64, 2.98f64],
    [2.5f64, 3.02f64],
    [3.0f64, 3.03f64],
    [4.0f64, 3.05f64],
];
static mut N_Cr_High_Paved: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut Cr_High_Paved: [[::core::ffi::c_double; 2]; 2] =
    [[0.15f64, 3.05f64], [0.25f64, 3.10f64]];
static mut N_Cr_High_Gravel: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut Cr_High_Gravel: [[::core::ffi::c_double; 2]; 2] =
    [[0.15f64, 2.95f64], [0.30f64, 3.10f64]];
static mut N_Kt_Paved: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
static mut Kt_Paved: [[::core::ffi::c_double; 2]; 9] = [
    [0.8f64, 1.0f64],
    [0.85f64, 0.98f64],
    [0.90f64, 0.92f64],
    [0.93f64, 0.85f64],
    [0.95f64, 0.80f64],
    [0.97f64, 0.70f64],
    [0.98f64, 0.60f64],
    [0.99f64, 0.50f64],
    [1.00f64, 0.40f64],
];
static mut N_Kt_Gravel: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut Kt_Gravel: [[::core::ffi::c_double; 2]; 12] = [
    [0.75f64, 1.00f64],
    [0.80f64, 0.985f64],
    [0.83f64, 0.97f64],
    [0.86f64, 0.93f64],
    [0.89f64, 0.90f64],
    [0.90f64, 0.87f64],
    [0.92f64, 0.80f64],
    [0.94f64, 0.70f64],
    [0.96f64, 0.60f64],
    [0.98f64, 0.50f64],
    [0.99f64, 0.40f64],
    [1.00f64, 0.24f64],
];
#[no_mangle]
pub unsafe extern "C" fn roadway_getInflow(
    mut j: ::core::ffi::c_int,
    mut dir: ::core::ffi::c_double,
    mut hRoad: ::core::ffi::c_double,
    mut h1: ::core::ffi::c_double,
    mut h2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut roadSurf: ::core::ffi::c_int = 0;
    let mut useVariableCd: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_double = 0.;
    let mut roadWidth: ::core::ffi::c_double = 0.;
    let mut hWr: ::core::ffi::c_double = 0.;
    let mut ht: ::core::ffi::c_double = 0.;
    let mut cD: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.0f64;
    let mut dqdh: ::core::ffi::c_double = 0.0f64;
    if (*Link.offset(j as isize)).type_0 != WEIR as ::core::ffi::c_int {
        return 0.0f64;
    }
    k = (*Link.offset(j as isize)).subIndex;
    roadWidth = (*Weir.offset(k as isize)).roadWidth;
    roadSurf = (*Weir.offset(k as isize)).roadSurface;
    cD = (*Weir.offset(k as isize)).cDisch1;
    if UnitSystem == SI as ::core::ffi::c_int {
        cD = cD / 0.552f64;
    }
    useVariableCd = FALSE;
    if roadWidth > 0.0f64 && roadSurf >= 1 as ::core::ffi::c_int {
        useVariableCd = TRUE;
    }
    hWr = h1 - hRoad;
    ht = h2 - hRoad;
    if hWr > FUDGE {
        if useVariableCd != 0 {
            cD = getCd(hWr, ht, roadWidth, roadSurf);
        }
        length = (*Link.offset(j as isize)).xsect.wMax;
        q = cD * length * pow(hWr, 1.5f64);
        dqdh = 1.5f64 * q / hWr;
    }
    (*Link.offset(j as isize)).dqdh = dqdh;
    (*Link.offset(j as isize)).newDepth = if h1 - hRoad >= 0.0f64 {
        h1 - hRoad
    } else {
        0.0f64
    };
    (*Link.offset(j as isize)).flowClass = SUBCRITICAL as ::core::ffi::c_int;
    if hRoad > h2 {
        if dir == 1.0f64 {
            (*Link.offset(j as isize)).flowClass = DN_CRITICAL as ::core::ffi::c_int;
        } else {
            (*Link.offset(j as isize)).flowClass = UP_CRITICAL as ::core::ffi::c_int;
        }
    }
    return dir * q;
}
unsafe extern "C" fn getCd(
    mut hWr: ::core::ffi::c_double,
    mut ht: ::core::ffi::c_double,
    mut roadWidth: ::core::ffi::c_double,
    mut roadSurf: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut kT: ::core::ffi::c_double = 1.0f64;
    let mut hL: ::core::ffi::c_double = 0.;
    let mut htH: ::core::ffi::c_double = 0.;
    let mut cR: ::core::ffi::c_double = 0.;
    if hWr <= 0.0f64 {
        return 0.0f64;
    }
    hL = hWr / roadWidth;
    if hL <= 0.15f64 {
        if roadSurf == PAVED as ::core::ffi::c_int {
            cR = getY(
                hWr,
                &raw const Cr_Low_Paved as *const [::core::ffi::c_double; 2],
                N_Cr_Low_Paved,
            );
        } else {
            cR = getY(
                hWr,
                &raw const Cr_Low_Gravel as *const [::core::ffi::c_double; 2],
                N_Cr_Low_Gravel,
            );
        }
    } else if roadSurf == PAVED as ::core::ffi::c_int {
        cR = getY(
            hL,
            &raw const Cr_High_Paved as *const [::core::ffi::c_double; 2],
            N_Cr_High_Paved,
        );
    } else {
        cR = getY(
            hL,
            &raw const Cr_High_Gravel as *const [::core::ffi::c_double; 2],
            N_Cr_High_Gravel,
        );
    }
    if ht > 0.0f64 {
        htH = ht / hWr;
        if roadSurf == PAVED as ::core::ffi::c_int {
            kT = getY(
                htH,
                &raw const Kt_Paved as *const [::core::ffi::c_double; 2],
                N_Kt_Paved,
            );
        } else {
            kT = getY(
                htH,
                &raw const Kt_Gravel as *const [::core::ffi::c_double; 2],
                N_Kt_Gravel,
            );
        }
    }
    return cR * kT;
}
unsafe extern "C" fn getY(
    mut x: ::core::ffi::c_double,
    mut table: *const [::core::ffi::c_double; 2],
    n: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut dx: ::core::ffi::c_double = 0.;
    let mut dy: ::core::ffi::c_double = 0.;
    if x <= (*table.offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] {
        return (*table.offset(0 as ::core::ffi::c_int as isize))[1 as ::core::ffi::c_int as usize];
    }
    if x >= (*table.offset((n - 1 as ::core::ffi::c_int) as isize))
        [0 as ::core::ffi::c_int as usize]
    {
        return (*table.offset((n - 1 as ::core::ffi::c_int) as isize))
            [1 as ::core::ffi::c_int as usize];
    }
    i = 1 as ::core::ffi::c_int;
    while i < n {
        if x <= (*table.offset(i as isize))[0 as ::core::ffi::c_int as usize] {
            x1 = (*table.offset((i - 1 as ::core::ffi::c_int) as isize))
                [0 as ::core::ffi::c_int as usize];
            dx = (*table.offset(i as isize))[0 as ::core::ffi::c_int as usize] - x1;
            y1 = (*table.offset((i - 1 as ::core::ffi::c_int) as isize))
                [1 as ::core::ffi::c_int as usize];
            dy = (*table.offset(i as isize))[1 as ::core::ffi::c_int as usize] - y1;
            return y1 + (x - x1) * dy / dx;
        }
        i += 1;
    }
    return (*table.offset((n - 1 as ::core::ffi::c_int) as isize))
        [1 as ::core::ffi::c_int as usize];
}
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FUDGE: ::core::ffi::c_double = 0.0001f64;

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
