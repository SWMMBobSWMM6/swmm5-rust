#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut ForceMainEqn: ::core::ffi::c_int;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
}
pub type ForceMainType = ::core::ffi::c_uint;
pub const D_W: ForceMainType = 1;
pub const H_W: ForceMainType = 0;
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
static mut VISCOS: ::core::ffi::c_double = 1.1E-5f64;
#[no_mangle]
pub unsafe extern "C" fn forcemain_getEquivN(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut xsect: TXsect = (*Link.offset(j as isize)).xsect;
    let mut f: ::core::ffi::c_double = 0.;
    let mut d: ::core::ffi::c_double = xsect.yFull;
    match ForceMainEqn {
        0 => {
            return 1.067f64 / xsect.rBot * pow(d / (*Conduit.offset(k as isize)).slope, 0.04f64);
        }
        1 => {
            f = forcemain_getFricFactor(xsect.rBot, d / 4.0f64, 1.0e12f64);
            return sqrt(f / 185.0f64) * pow(d, 1.0f64 / 6.0f64);
        }
        _ => {}
    }
    return (*Conduit.offset(k as isize)).roughness;
}
#[no_mangle]
pub unsafe extern "C" fn forcemain_getRoughFactor(
    mut j: ::core::ffi::c_int,
    mut lengthFactor: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut xsect: TXsect = (*Link.offset(j as isize)).xsect;
    let mut r: ::core::ffi::c_double = 0.;
    match ForceMainEqn {
        0 => {
            r = 1.318f64 * xsect.rBot * pow(lengthFactor, 0.54f64);
            return GRAVITY / pow(r, 1.852f64);
        }
        1 => return 1.0f64 / 8.0f64 / lengthFactor,
        _ => {}
    }
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn forcemain_getFricSlope(
    mut j: ::core::ffi::c_int,
    mut v: ::core::ffi::c_double,
    mut hrad: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut re: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut xsect: TXsect = (*Link.offset(j as isize)).xsect;
    match ForceMainEqn {
        0 => return xsect.sBot * pow(v, 0.852f64) / pow(hrad, 1.1667f64),
        1 => {
            re = forcemain_getReynolds(v, hrad);
            f = forcemain_getFricFactor(xsect.rBot, hrad, re);
            return f * xsect.sBot * v / hrad;
        }
        _ => {}
    }
    return 0.0f64;
}
unsafe extern "C" fn forcemain_getReynolds(
    mut v: ::core::ffi::c_double,
    mut hrad: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return 4.0f64 * hrad * v / VISCOS;
}
unsafe extern "C" fn forcemain_getFricFactor(
    mut e: ::core::ffi::c_double,
    mut hrad: ::core::ffi::c_double,
    mut re: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut f: ::core::ffi::c_double = 0.;
    if re < 10.0f64 {
        re = 10.0f64;
    }
    if re <= 2000.0f64 {
        f = 64.0f64 / re;
    } else if re < 4000.0f64 {
        f = forcemain_getFricFactor(e, hrad, 4000.0f64);
        f = 0.032f64 + (f - 0.032f64) * (re - 2000.0f64) / 2000.0f64;
    } else {
        f = e / 3.7f64 / (4.0f64 * hrad);
        if re < 1.0e10f64 {
            f += 5.74f64 / pow(re, 0.9f64);
        }
        f = log10(f);
        f = 0.25f64 / f / f;
    }
    return f;
}
pub const GRAVITY: ::core::ffi::c_double = 32.2f64;

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
