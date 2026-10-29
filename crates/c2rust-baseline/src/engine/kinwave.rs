#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn link_getLength(link: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn link_getLossRate(
        link: ::core::ffi::c_int,
        routeModel: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        tstep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn link_getFullState(
        a1: ::core::ffi::c_double,
        a2: ::core::ffi::c_double,
        aFull: ::core::ffi::c_double,
    ) -> ::core::ffi::c_char;
    fn xsect_getAmax(xsect: *mut TXsect) -> ::core::ffi::c_double;
    fn xsect_getSofA(xsect: *mut TXsect, area: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getAofS(xsect: *mut TXsect, sFactor: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getdSdA(xsect: *mut TXsect, area: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn findroot_Newton(
        x1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        rts: *mut ::core::ffi::c_double,
        xacc: ::core::ffi::c_double,
        func: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        p: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
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
pub const ERR_KINWAVE: ErrorType = 103;
pub type ErrorType = ::core::ffi::c_uint;
pub const MAXERRMSG: ErrorType = 1000;
pub const ERR_API_TIME_PERIOD: ErrorType = 509;
pub const ERR_API_PROPERTY_VALUE: ErrorType = 508;
pub const ERR_API_PROPERTY_TYPE: ErrorType = 507;
pub const ERR_API_OBJECT_NAME: ErrorType = 506;
pub const ERR_API_OBJECT_INDEX: ErrorType = 505;
pub const ERR_API_OBJECT_TYPE: ErrorType = 504;
pub const ERR_API_NOT_ENDED: ErrorType = 503;
pub const ERR_API_NOT_STARTED: ErrorType = 502;
pub const ERR_API_NOT_OPEN: ErrorType = 501;
pub const ERR_SYSTEM: ErrorType = 500;
pub const ERR_TABLE_FILE_READ: ErrorType = 363;
pub const ERR_TABLE_FILE_OPEN: ErrorType = 361;
pub const ERR_ROUTING_FILE_NAMES: ErrorType = 357;
pub const ERR_ROUTING_FILE_NOMATCH: ErrorType = 355;
pub const ERR_ROUTING_FILE_FORMAT: ErrorType = 353;
pub const ERR_ROUTING_FILE_OPEN: ErrorType = 351;
pub const ERR_RDII_FILE_FORMAT: ErrorType = 345;
pub const ERR_RDII_FILE_OPEN: ErrorType = 343;
pub const ERR_RDII_FILE_SCRATCH: ErrorType = 341;
pub const ERR_CLIMATE_END_OF_FILE: ErrorType = 339;
pub const ERR_CLIMATE_FILE_READ: ErrorType = 338;
pub const ERR_CLIMATE_FILE_OPEN: ErrorType = 337;
pub const ERR_NO_CLIMATE_FILE: ErrorType = 336;
pub const ERR_HOTSTART_FILE_READ: ErrorType = 335;
pub const ERR_HOTSTART_FILE_FORMAT: ErrorType = 333;
pub const ERR_HOTSTART_FILE_OPEN: ErrorType = 331;
pub const ERR_RUNOFF_FILE_READ: ErrorType = 329;
pub const ERR_RUNOFF_FILE_END: ErrorType = 327;
pub const ERR_RUNOFF_FILE_FORMAT: ErrorType = 325;
pub const ERR_RUNOFF_FILE_OPEN: ErrorType = 323;
pub const ERR_RAIN_FILE_GAGE: ErrorType = 321;
pub const ERR_RAIN_IFACE_FORMAT: ErrorType = 320;
pub const ERR_RAIN_FILE_FORMAT: ErrorType = 319;
pub const ERR_RAIN_FILE_SEQUENCE: ErrorType = 318;
pub const ERR_RAIN_FILE_DATA: ErrorType = 317;
pub const ERR_RAIN_FILE_OPEN: ErrorType = 315;
pub const ERR_RAIN_FILE_SCRATCH: ErrorType = 313;
pub const ERR_OUT_READ: ErrorType = 311;
pub const ERR_OUT_WRITE: ErrorType = 309;
pub const ERR_OUT_SIZE: ErrorType = 308;
pub const ERR_OUT_FILE: ErrorType = 307;
pub const ERR_RPT_FILE: ErrorType = 305;
pub const ERR_INP_FILE: ErrorType = 303;
pub const ERR_FILE_NAME: ErrorType = 301;
pub const ERR_INFIL_PARAMS: ErrorType = 235;
pub const ERR_MATH_EXPR: ErrorType = 233;
pub const ERR_TRANSECT_NO_DEPTH: ErrorType = 231;
pub const ERR_TRANSECT_OVERBANK: ErrorType = 229;
pub const ERR_TRANSECT_MANNING: ErrorType = 227;
pub const ERR_TRANSECT_TOO_MANY: ErrorType = 225;
pub const ERR_TRANSECT_TOO_FEW: ErrorType = 223;
pub const ERR_TRANSECT_SEQUENCE: ErrorType = 221;
pub const ERR_TRANSECT_UNKNOWN: ErrorType = 219;
pub const ERR_RULE: ErrorType = 217;
pub const ERR_DATETIME: ErrorType = 213;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_DUP_NAME: ErrorType = 207;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_LINE_LENGTH: ErrorType = 201;
pub const ERR_INPUT: ErrorType = 200;
pub const ERR_REPORT_STEP: ErrorType = 195;
pub const ERR_REPORT_DATE: ErrorType = 193;
pub const ERR_START_DATE: ErrorType = 191;
pub const ERR_LID_CAPTURE_AREA: ErrorType = 188;
pub const ERR_LID_AREAS: ErrorType = 187;
pub const ERR_LID_PARAMS: ErrorType = 185;
pub const ERR_LID_LAYER: ErrorType = 184;
pub const ERR_LID_TYPE: ErrorType = 183;
pub const ERR_SNOWPACK_PARAMS: ErrorType = 182;
pub const ERR_SNOWMELT_PARAMS: ErrorType = 181;
pub const ERR_TIMESERIES_SEQUENCE: ErrorType = 173;
pub const ERR_CURVE_SEQUENCE: ErrorType = 171;
pub const ERR_CYCLIC_TREATMENT: ErrorType = 161;
pub const ERR_RAIN_GAGE_INTERVAL: ErrorType = 159;
pub const ERR_RAIN_GAGE_TSERIES: ErrorType = 158;
pub const ERR_RAIN_GAGE_FORMAT: ErrorType = 157;
pub const ERR_RAIN_FILE_CONFLICT: ErrorType = 156;
pub const ERR_RDII_AREA: ErrorType = 155;
pub const ERR_UNITHYD_RATIOS: ErrorType = 153;
pub const ERR_UNITHYD_TIMES: ErrorType = 151;
pub const ERR_NO_OUTLETS: ErrorType = 145;
pub const ERR_REGULATOR_SHAPE: ErrorType = 143;
pub const ERR_OUTFALL: ErrorType = 141;
pub const ERR_STORAGE_VOLUME: ErrorType = 140;
pub const ERR_REGULATOR: ErrorType = 139;
pub const ERR_NODE_DEPTH: ErrorType = 138;
pub const ERR_WEIR_DIVIDER: ErrorType = 137;
pub const ERR_DIVIDER_LINK: ErrorType = 136;
pub const ERR_DIVIDER: ErrorType = 135;
pub const ERR_DUMMY_LINK: ErrorType = 134;
pub const ERR_MULTI_OUTLET: ErrorType = 133;
pub const ERR_LOOP: ErrorType = 131;
pub const ERR_PUMP_LIMITS: ErrorType = 122;
pub const ERR_NO_CURVE: ErrorType = 121;
pub const ERR_XSECT: ErrorType = 119;
pub const ERR_NO_XSECT: ErrorType = 117;
pub const ERR_SLOPE: ErrorType = 115;
pub const ERR_BARRELS: ErrorType = 114;
pub const ERR_ROUGHNESS: ErrorType = 113;
pub const ERR_ELEV_DROP: ErrorType = 112;
pub const ERR_LENGTH: ErrorType = 111;
pub const ERR_GROUND_ELEV: ErrorType = 110;
pub const ERR_AQUIFER_PARAMS: ErrorType = 109;
pub const ERR_SUBCATCH_OUTLET: ErrorType = 108;
pub const ERR_TIMESTEP: ErrorType = 107;
pub const ERR_ODE_SOLVER: ErrorType = 105;
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NONE: ErrorType = 0;
static mut WX: ::core::ffi::c_double = 0.6f64;
static mut WT: ::core::ffi::c_double = 0.6f64;
static mut EPSIL: ::core::ffi::c_double = 0.001f64;
static mut Beta1: ::core::ffi::c_double = 0.;
static mut C1: ::core::ffi::c_double = 0.;
static mut C2: ::core::ffi::c_double = 0.;
static mut Afull: ::core::ffi::c_double = 0.;
static mut Qfull: ::core::ffi::c_double = 0.;
static mut pXsect: *mut TXsect = ::core::ptr::null::<TXsect>() as *mut TXsect;
#[no_mangle]
pub unsafe extern "C" fn kinwave_execute(
    mut j: ::core::ffi::c_int,
    mut qinflow: *mut ::core::ffi::c_double,
    mut qoutflow: *mut ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut dxdt: ::core::ffi::c_double = 0.;
    let mut dq: ::core::ffi::c_double = 0.;
    let mut ain: ::core::ffi::c_double = 0.;
    let mut aout: ::core::ffi::c_double = 0.;
    let mut qin: ::core::ffi::c_double = 0.;
    let mut qout: ::core::ffi::c_double = 0.;
    let mut a1: ::core::ffi::c_double = 0.;
    let mut a2: ::core::ffi::c_double = 0.;
    let mut q1: ::core::ffi::c_double = 0.;
    let mut q2: ::core::ffi::c_double = 0.;
    let mut q3: ::core::ffi::c_double = 0.;
    *qoutflow = *qinflow;
    if (*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int {
        return result;
    }
    if (*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int {
        return result;
    }
    pXsect = &raw mut (*Link.offset(j as isize)).xsect;
    Qfull = (*Link.offset(j as isize)).qFull;
    Afull = (*Link.offset(j as isize)).xsect.aFull;
    k = (*Link.offset(j as isize)).subIndex;
    Beta1 = (*Conduit.offset(k as isize)).beta / Qfull;
    q1 = (*Conduit.offset(k as isize)).q1 / Qfull;
    q2 = (*Conduit.offset(k as isize)).q2 / Qfull;
    qin = *qinflow
        / (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int as ::core::ffi::c_double
        / Qfull;
    q3 = link_getLossRate(j, KW as ::core::ffi::c_int, qin * Qfull, tStep) / Qfull;
    a1 = (*Conduit.offset(k as isize)).a1 / Afull;
    a2 = (*Conduit.offset(k as isize)).a2 / Afull;
    if qin >= 1.0f64 {
        ain = 1.0f64;
    } else {
        ain = xsect_getAofS(pXsect, qin / Beta1) / Afull;
    }
    if qin <= TINY && q2 <= TINY {
        qout = 0.0f64;
        aout = 0.0f64;
    } else {
        dxdt = link_getLength(j) / tStep * Afull / Qfull;
        dq = q2 - q1;
        C1 = dxdt * WT / WX;
        C2 = (1.0f64 - WT) * (ain - a1);
        C2 = C2 - WT * a2;
        C2 = C2 * dxdt / WX;
        C2 = C2 + (1.0f64 - WX) / WX * dq - qin;
        C2 = C2 + q3 / WX;
        aout = a2;
        result = solveContinuity(qin, ain, &raw mut aout);
        if result == -(1 as ::core::ffi::c_int) {
            report_writeErrorMsg(
                ERR_KINWAVE as ::core::ffi::c_int,
                (*Link.offset(j as isize)).ID,
            );
            return 1 as ::core::ffi::c_int;
        }
        if result <= 0 as ::core::ffi::c_int {
            result = 1 as ::core::ffi::c_int;
        }
        qout = Beta1 * xsect_getSofA(pXsect, aout * Afull);
        if qin > 1.0f64 {
            qin = 1.0f64;
        }
    }
    (*Conduit.offset(k as isize)).q1 = qin * Qfull;
    (*Conduit.offset(k as isize)).a1 = ain * Afull;
    (*Conduit.offset(k as isize)).q2 = qout * Qfull;
    (*Conduit.offset(k as isize)).a2 = aout * Afull;
    (*Conduit.offset(k as isize)).fullState = link_getFullState(
        (*Conduit.offset(k as isize)).a1,
        (*Conduit.offset(k as isize)).a2,
        Afull,
    );
    *qinflow = (*Conduit.offset(k as isize)).q1
        * (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int as ::core::ffi::c_double;
    *qoutflow = (*Conduit.offset(k as isize)).q2
        * (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int as ::core::ffi::c_double;
    return result;
}
unsafe extern "C" fn solveContinuity(
    mut qin: ::core::ffi::c_double,
    mut ain: ::core::ffi::c_double,
    mut aout: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut aLo: ::core::ffi::c_double = 0.;
    let mut aHi: ::core::ffi::c_double = 0.;
    let mut aTmp: ::core::ffi::c_double = 0.;
    let mut fLo: ::core::ffi::c_double = 0.;
    let mut fHi: ::core::ffi::c_double = 0.;
    let mut tol: ::core::ffi::c_double = EPSIL;
    aHi = 1.0f64;
    fHi = 1.0f64 + C1 + C2;
    aLo = xsect_getAmax(pXsect) / Afull;
    if aLo < aHi {
        fLo = Beta1 * (*pXsect).sMax + C1 * aLo + C2;
    } else {
        fLo = fHi;
    }
    if fHi * fLo > 0.0f64 {
        aHi = aLo;
        fHi = fLo;
        aLo = 0.0f64;
        fLo = C2;
    }
    if fHi * fLo <= 0.0f64 {
        if *aout < aLo || *aout > aHi {
            *aout = 0.5f64 * (aLo + aHi);
        }
        if fLo > fHi {
            aTmp = aLo;
            aLo = aHi;
            aHi = aTmp;
        }
        n = findroot_Newton(
            aLo,
            aHi,
            aout,
            tol,
            Some(
                evalContinuity
                    as unsafe extern "C" fn(
                        ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            NULL,
        );
        if n <= 0 as ::core::ffi::c_int {
            n = -(1 as ::core::ffi::c_int);
        }
    } else if fLo < 0.0f64 {
        if qin > 1.0f64 {
            *aout = ain;
        } else {
            *aout = 1.0f64;
        }
        n = -(2 as ::core::ffi::c_int);
    } else if fLo > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *aout = 0.0f64;
        n = -(3 as ::core::ffi::c_int);
    } else {
        n = -(1 as ::core::ffi::c_int);
    }
    return n;
}
unsafe extern "C" fn evalContinuity(
    mut a: ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
    mut df: *mut ::core::ffi::c_double,
    mut p: *mut ::core::ffi::c_void,
) {
    *f = Beta1 * xsect_getSofA(pXsect, a * Afull) + C1 * a + C2;
    *df = Beta1 * Afull * xsect_getdSdA(pXsect, a * Afull) + C1;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const TINY: ::core::ffi::c_double = 1.0E-6f64;

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
