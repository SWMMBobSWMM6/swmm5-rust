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
    fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn datetime_monthOfYear(date: DateTime) -> ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut OldRunoffTime: ::core::ffi::c_double;
    static mut Evap: TEvap;
    static mut Adjust: TAdjust;
    static mut Subcatch: *mut TSubcatch;
    static mut Pattern: *mut TPattern;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut InfilModelWords: [*mut ::core::ffi::c_char; 0];
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
}
pub type size_t = usize;
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
pub type ConversionType = ::core::ffi::c_uint;
pub const FLOW: ConversionType = 10;
pub const GWFLOW: ConversionType = 9;
pub const MASS: ConversionType = 8;
pub const TEMPERATURE: ConversionType = 7;
pub const WINDSPEED: ConversionType = 6;
pub const VOLUME: ConversionType = 5;
pub const LANDAREA: ConversionType = 4;
pub const LENGTH: ConversionType = 3;
pub const EVAPRATE: ConversionType = 2;
pub const RAINDEPTH: ConversionType = 1;
pub const RAINFALL: ConversionType = 0;
pub type PatternType = ::core::ffi::c_uint;
pub const WEEKEND_PATTERN: PatternType = 3;
pub const HOURLY_PATTERN: PatternType = 2;
pub const DAILY_PATTERN: PatternType = 1;
pub const MONTHLY_PATTERN: PatternType = 0;
pub type DateTime = ::core::ffi::c_double;
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
pub type InfilType = ::core::ffi::c_uint;
pub const CURVE_NUMBER: InfilType = 4;
pub const MOD_GREEN_AMPT: InfilType = 3;
pub const GREEN_AMPT: InfilType = 2;
pub const MOD_HORTON: InfilType = 1;
pub const HORTON: InfilType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct THorton {
    pub f0: ::core::ffi::c_double,
    pub fmin: ::core::ffi::c_double,
    pub Fmax: ::core::ffi::c_double,
    pub decay: ::core::ffi::c_double,
    pub regen: ::core::ffi::c_double,
    pub tp: ::core::ffi::c_double,
    pub Fe: ::core::ffi::c_double,
}
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
pub struct TCurveNum {
    pub Smax: ::core::ffi::c_double,
    pub regen: ::core::ffi::c_double,
    pub Tmax: ::core::ffi::c_double,
    pub S: ::core::ffi::c_double,
    pub F: ::core::ffi::c_double,
    pub P: ::core::ffi::c_double,
    pub T: ::core::ffi::c_double,
    pub Se: ::core::ffi::c_double,
    pub f: ::core::ffi::c_double,
}
pub const ERR_MEMORY: ErrorType = 101;
#[derive(Copy, Clone)]
#[repr(C)]
pub union TInfil {
    pub horton: THorton,
    pub grnAmpt: TGrnAmpt,
    pub curveNum: TCurveNum,
}
pub const ERR_INFIL_PARAMS: ErrorType = 235;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSubcatch {
    pub ID: *mut ::core::ffi::c_char,
    pub rptFlag: ::core::ffi::c_int,
    pub gage: ::core::ffi::c_int,
    pub outNode: ::core::ffi::c_int,
    pub outSubcatch: ::core::ffi::c_int,
    pub infilModel: ::core::ffi::c_int,
    pub infil: ::core::ffi::c_int,
    pub subArea: [TSubarea; 3],
    pub width: ::core::ffi::c_double,
    pub area: ::core::ffi::c_double,
    pub fracImperv: ::core::ffi::c_double,
    pub slope: ::core::ffi::c_double,
    pub curbLength: ::core::ffi::c_double,
    pub initBuildup: *mut ::core::ffi::c_double,
    pub landFactor: *mut TLandFactor,
    pub groundwater: *mut TGroundwater,
    pub gwLatFlowExpr: *mut MathExpr,
    pub gwDeepFlowExpr: *mut MathExpr,
    pub snowpack: *mut TSnowpack,
    pub nPervPattern: ::core::ffi::c_int,
    pub dStorePattern: ::core::ffi::c_int,
    pub infilPattern: ::core::ffi::c_int,
    pub lidArea: ::core::ffi::c_double,
    pub rainfall: ::core::ffi::c_double,
    pub evapLoss: ::core::ffi::c_double,
    pub infilLoss: ::core::ffi::c_double,
    pub runon: ::core::ffi::c_double,
    pub oldRunoff: ::core::ffi::c_double,
    pub newRunoff: ::core::ffi::c_double,
    pub oldSnowDepth: ::core::ffi::c_double,
    pub newSnowDepth: ::core::ffi::c_double,
    pub oldQual: *mut ::core::ffi::c_double,
    pub newQual: *mut ::core::ffi::c_double,
    pub pondedQual: *mut ::core::ffi::c_double,
    pub totalLoad: *mut ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSnowpack {
    pub snowmeltIndex: ::core::ffi::c_int,
    pub fArea: [::core::ffi::c_double; 3],
    pub wsnow: [::core::ffi::c_double; 3],
    pub fw: [::core::ffi::c_double; 3],
    pub coldc: [::core::ffi::c_double; 3],
    pub ati: [::core::ffi::c_double; 3],
    pub sba: [::core::ffi::c_double; 3],
    pub awe: [::core::ffi::c_double; 3],
    pub sbws: [::core::ffi::c_double; 3],
    pub imelt: [::core::ffi::c_double; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TGroundwater {
    pub aquifer: ::core::ffi::c_int,
    pub node: ::core::ffi::c_int,
    pub surfElev: ::core::ffi::c_double,
    pub a1: ::core::ffi::c_double,
    pub b1: ::core::ffi::c_double,
    pub a2: ::core::ffi::c_double,
    pub b2: ::core::ffi::c_double,
    pub a3: ::core::ffi::c_double,
    pub fixedDepth: ::core::ffi::c_double,
    pub nodeElev: ::core::ffi::c_double,
    pub bottomElev: ::core::ffi::c_double,
    pub waterTableElev: ::core::ffi::c_double,
    pub upperMoisture: ::core::ffi::c_double,
    pub theta: ::core::ffi::c_double,
    pub lowerDepth: ::core::ffi::c_double,
    pub oldFlow: ::core::ffi::c_double,
    pub newFlow: ::core::ffi::c_double,
    pub evapLoss: ::core::ffi::c_double,
    pub maxInfilVol: ::core::ffi::c_double,
    pub stats: TGWaterStats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TGWaterStats {
    pub infil: ::core::ffi::c_double,
    pub evap: ::core::ffi::c_double,
    pub latFlow: ::core::ffi::c_double,
    pub deepFlow: ::core::ffi::c_double,
    pub avgUpperMoist: ::core::ffi::c_double,
    pub finalUpperMoist: ::core::ffi::c_double,
    pub avgWaterTable: ::core::ffi::c_double,
    pub finalWaterTable: ::core::ffi::c_double,
    pub maxFlow: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLandFactor {
    pub fraction: ::core::ffi::c_double,
    pub buildup: *mut ::core::ffi::c_double,
    pub lastSwept: DateTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSubarea {
    pub routeTo: ::core::ffi::c_int,
    pub fOutlet: ::core::ffi::c_double,
    pub N: ::core::ffi::c_double,
    pub fArea: ::core::ffi::c_double,
    pub dStore: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub inflow: ::core::ffi::c_double,
    pub runoff: ::core::ffi::c_double,
    pub depth: ::core::ffi::c_double,
}
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_NAME: ErrorType = 209;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPattern {
    pub ID: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub factor: [::core::ffi::c_double; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAdjust {
    pub temp: [::core::ffi::c_double; 12],
    pub evap: [::core::ffi::c_double; 12],
    pub rain: [::core::ffi::c_double; 12],
    pub hydcon: [::core::ffi::c_double; 12],
    pub rainFactor: ::core::ffi::c_double,
    pub hydconFactor: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEvap {
    pub type_0: ::core::ffi::c_int,
    pub tSeries: ::core::ffi::c_int,
    pub monthlyEvap: [::core::ffi::c_double; 12],
    pub panCoeff: [::core::ffi::c_double; 12],
    pub recoveryPattern: ::core::ffi::c_int,
    pub dryOnly: ::core::ffi::c_int,
    pub rate: ::core::ffi::c_double,
    pub recoveryFactor: ::core::ffi::c_double,
}
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
pub const ERR_DUP_NAME: ErrorType = 207;
pub const ERR_KEYWORD: ErrorType = 205;
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
pub const ERR_KINWAVE: ErrorType = 103;
pub const ERR_NONE: ErrorType = 0;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TINY: ::core::ffi::c_double = 1.0E-6f64;
pub const ZERO: ::core::ffi::c_double = 1.0E-10f64;
pub const MIN_TOTAL_DEPTH: ::core::ffi::c_double = 0.004167f64;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Infil: *mut TInfil = ::core::ptr::null::<TInfil>() as *mut TInfil;
static mut Fumax: ::core::ffi::c_double = 0.;
static mut InfilFactor: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn infil_create(mut n: ::core::ffi::c_int) {
    Infil = calloc(n as size_t, ::core::mem::size_of::<TInfil>() as size_t) as *mut TInfil;
    if Infil.is_null() {
        ErrorCode = ERR_MEMORY as ::core::ffi::c_int;
    }
    InfilFactor = 1.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn infil_delete() {
    if !Infil.is_null() {
        free(Infil as *mut ::core::ffi::c_void);
        Infil = ::core::ptr::null_mut::<TInfil>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn infil_readParams(
    mut m: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 5] = [0.; 5];
    j = project_findObject(
        SUBCATCH as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    i = findmatch(
        *tok.offset((ntoks - 1 as ::core::ffi::c_int) as isize),
        &raw mut InfilModelWords as *mut *mut ::core::ffi::c_char,
    );
    if i >= 0 as ::core::ffi::c_int {
        m = i;
        ntoks -= 1;
    }
    if m == HORTON as ::core::ffi::c_int {
        n = 5 as ::core::ffi::c_int;
    } else if m == MOD_HORTON as ::core::ffi::c_int {
        n = 5 as ::core::ffi::c_int;
    } else if m == GREEN_AMPT as ::core::ffi::c_int {
        n = 4 as ::core::ffi::c_int;
    } else if m == MOD_GREEN_AMPT as ::core::ffi::c_int {
        n = 4 as ::core::ffi::c_int;
    } else if m == CURVE_NUMBER as ::core::ffi::c_int {
        n = 4 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    }
    if ntoks < n {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        x[i as usize] = 0.0f64;
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < n {
        if getDouble(
            *tok.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(i as isize));
        }
        i += 1;
    }
    if (m == HORTON as ::core::ffi::c_int || m == MOD_HORTON as ::core::ffi::c_int) && ntoks > n {
        if getDouble(
            *tok.offset(n as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((n - 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(n as isize));
        }
    }
    (*Subcatch.offset(j as isize)).infil = j;
    (*Subcatch.offset(j as isize)).infilModel = m;
    match m {
        0 | 1 => {
            status = horton_setParams(
                &raw mut (*Infil.offset(j as isize)).horton,
                &raw mut x as *mut ::core::ffi::c_double,
            );
        }
        2 | 3 => {
            status = grnampt_setParams(
                &raw mut (*Infil.offset(j as isize)).grnAmpt,
                &raw mut x as *mut ::core::ffi::c_double,
            );
        }
        4 => {
            status = curvenum_setParams(
                &raw mut (*Infil.offset(j as isize)).curveNum,
                &raw mut x as *mut ::core::ffi::c_double,
            );
        }
        _ => {
            status = TRUE;
        }
    }
    if status == 0 {
        return error_setInpError(
            ERR_INFIL_PARAMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn infil_initState(mut j: ::core::ffi::c_int) {
    match (*Subcatch.offset(j as isize)).infilModel {
        0 | 1 => {
            horton_initState(&raw mut (*Infil.offset(j as isize)).horton);
        }
        2 | 3 => {
            grnampt_initState(&raw mut (*Infil.offset(j as isize)).grnAmpt);
        }
        4 => {
            curvenum_initState(&raw mut (*Infil.offset(j as isize)).curveNum);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn infil_getState(
    mut j: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    match (*Subcatch.offset(j as isize)).infilModel {
        0 | 1 => {
            horton_getState(&raw mut (*Infil.offset(j as isize)).horton, x);
        }
        2 | 3 => {
            grnampt_getState(&raw mut (*Infil.offset(j as isize)).grnAmpt, x);
        }
        4 => {
            curvenum_getState(&raw mut (*Infil.offset(j as isize)).curveNum, x);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn infil_setState(
    mut j: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    match (*Subcatch.offset(j as isize)).infilModel {
        0 | 1 => {
            horton_setState(&raw mut (*Infil.offset(j as isize)).horton, x);
        }
        2 | 3 => {
            grnampt_setState(&raw mut (*Infil.offset(j as isize)).grnAmpt, x);
        }
        4 => {
            curvenum_setState(&raw mut (*Infil.offset(j as isize)).curveNum, x);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn infil_setInfilFactor(mut j: ::core::ffi::c_int) {
    let mut m: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    InfilFactor = Adjust.hydconFactor;
    if j >= 0 as ::core::ffi::c_int {
        p = (*Subcatch.offset(j as isize)).infilPattern;
        if p >= 0 as ::core::ffi::c_int
            && (*Pattern.offset(p as isize)).type_0 == MONTHLY_PATTERN as ::core::ffi::c_int
        {
            m = datetime_monthOfYear(getDateTime(OldRunoffTime)) - 1 as ::core::ffi::c_int;
            InfilFactor = (*Pattern.offset(p as isize)).factor[m as usize];
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn infil_getInfil(
    mut j: ::core::ffi::c_int,
    mut tstep: ::core::ffi::c_double,
    mut rainfall: ::core::ffi::c_double,
    mut runon: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    match (*Subcatch.offset(j as isize)).infilModel {
        0 => {
            return horton_getInfil(
                &raw mut (*Infil.offset(j as isize)).horton,
                tstep,
                rainfall + runon,
                depth,
            );
        }
        1 => {
            return modHorton_getInfil(
                &raw mut (*Infil.offset(j as isize)).horton,
                tstep,
                rainfall + runon,
                depth,
            );
        }
        2 | 3 => {
            return grnampt_getInfil(
                &raw mut (*Infil.offset(j as isize)).grnAmpt,
                tstep,
                rainfall + runon,
                depth,
                (*Subcatch.offset(j as isize)).infilModel,
            );
        }
        4 => {
            depth += runon * tstep;
            return curvenum_getInfil(
                &raw mut (*Infil.offset(j as isize)).curveNum,
                tstep,
                rainfall,
                depth,
            );
        }
        _ => return 0.0f64,
    };
}
unsafe extern "C" fn horton_setParams(
    mut infil: *mut THorton,
    mut p: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    k = 0 as ::core::ffi::c_int;
    while k < 5 as ::core::ffi::c_int {
        if *p.offset(k as isize) < 0.0f64 {
            return FALSE;
        }
        k += 1;
    }
    (*infil).f0 = *p.offset(0 as ::core::ffi::c_int as isize) / UCF(RAINFALL as ::core::ffi::c_int);
    (*infil).fmin =
        *p.offset(1 as ::core::ffi::c_int as isize) / UCF(RAINFALL as ::core::ffi::c_int);
    (*infil).decay = *p.offset(2 as ::core::ffi::c_int as isize) / 3600.0f64;
    if *p.offset(3 as ::core::ffi::c_int as isize) == 0.0f64 {
        *p.offset(3 as ::core::ffi::c_int as isize) = TINY;
    }
    (*infil).regen = -log(1.0f64 - 0.98f64)
        / *p.offset(3 as ::core::ffi::c_int as isize)
        / SECperDAY as ::core::ffi::c_double;
    (*infil).Fmax =
        *p.offset(4 as ::core::ffi::c_int as isize) / UCF(RAINDEPTH as ::core::ffi::c_int);
    if (*infil).f0 < (*infil).fmin {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn horton_initState(mut infil: *mut THorton) {
    (*infil).tp = 0.0f64;
    (*infil).Fe = 0.0f64;
}
unsafe extern "C" fn horton_getState(mut infil: *mut THorton, mut x: *mut ::core::ffi::c_double) {
    *x.offset(0 as ::core::ffi::c_int as isize) = (*infil).tp;
    *x.offset(1 as ::core::ffi::c_int as isize) = (*infil).Fe;
}
unsafe extern "C" fn horton_setState(mut infil: *mut THorton, mut x: *mut ::core::ffi::c_double) {
    (*infil).tp = *x.offset(0 as ::core::ffi::c_int as isize);
    (*infil).Fe = *x.offset(1 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn horton_getInfil(
    mut infil: *mut THorton,
    mut tstep: ::core::ffi::c_double,
    mut irate: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut iter: ::core::ffi::c_int = 0;
    let mut fa: ::core::ffi::c_double = 0.;
    let mut fp: ::core::ffi::c_double = 0.0f64;
    let mut Fp: ::core::ffi::c_double = 0.;
    let mut F1: ::core::ffi::c_double = 0.;
    let mut t1: ::core::ffi::c_double = 0.;
    let mut tlim: ::core::ffi::c_double = 0.;
    let mut ex: ::core::ffi::c_double = 0.;
    let mut kt: ::core::ffi::c_double = 0.;
    let mut FF: ::core::ffi::c_double = 0.;
    let mut FF1: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    let mut f0: ::core::ffi::c_double = (*infil).f0 * InfilFactor;
    let mut fmin: ::core::ffi::c_double = (*infil).fmin * InfilFactor;
    let mut Fmax: ::core::ffi::c_double = (*infil).Fmax;
    let mut tp: ::core::ffi::c_double = (*infil).tp;
    let mut df: ::core::ffi::c_double = f0 - fmin;
    let mut kd: ::core::ffi::c_double = (*infil).decay;
    let mut kr: ::core::ffi::c_double = (*infil).regen * Evap.recoveryFactor;
    if df < 0.0f64 || kd < 0.0f64 || kr < 0.0f64 {
        return 0.0f64;
    }
    if df == 0.0f64 || kd == 0.0f64 {
        fp = f0;
        fa = irate + depth / tstep;
        if fp > fa {
            fp = fa;
        }
        return if 0.0f64 >= fp { 0.0f64 } else { fp };
    }
    fa = irate + depth / tstep;
    if fa > ZERO {
        t1 = tp + tstep;
        tlim = 16.0f64 / kd;
        if tp >= tlim {
            Fp = fmin * tp + df / kd;
            F1 = Fp + fmin * tstep;
        } else {
            Fp = fmin * tp + df / kd * (1.0f64 - exp(-kd * tp));
            F1 = fmin * t1 + df / kd * (1.0f64 - exp(-kd * t1));
        }
        fp = (F1 - Fp) / tstep;
        fp = if fp >= fmin { fp } else { fmin };
        if fp > fa {
            fp = fa;
        }
        if t1 > tlim {
            tp = t1;
        } else if fp < fa {
            tp = t1;
        } else {
            F1 = Fp + fp * tstep;
            tp = tp + tstep / 2.0f64;
            iter = 1 as ::core::ffi::c_int;
            while iter <= 20 as ::core::ffi::c_int {
                kt = if 60.0f64 <= kd * tp { 60.0f64 } else { kd * tp };
                ex = exp(-kt);
                FF = fmin * tp + df / kd * (1.0f64 - ex) - F1;
                FF1 = fmin + df * ex;
                r = FF / FF1;
                tp = tp - r;
                if fabs(r) <= 0.001f64 * tstep {
                    break;
                }
                iter += 1;
            }
        }
        if Fmax > 0.0f64 {
            if (*infil).Fe + fp * tstep > Fmax {
                fp = (Fmax - (*infil).Fe) / tstep;
            }
            fp = if fp >= 0.0f64 { fp } else { 0.0f64 };
            (*infil).Fe += fp * tstep;
        }
    } else if kr > 0.0f64 {
        r = exp(-kr * tstep);
        tp = 1.0f64 - exp(-kd * tp);
        tp = -log(1.0f64 - r * tp) / kd;
        if Fmax > 0.0f64 {
            (*infil).Fe = fmin * tp + df / kd * (1.0f64 - exp(-kd * tp));
        }
    }
    (*infil).tp = tp;
    return fp;
}
unsafe extern "C" fn modHorton_getInfil(
    mut infil: *mut THorton,
    mut tstep: ::core::ffi::c_double,
    mut irate: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut f: ::core::ffi::c_double = 0.0f64;
    let mut fp: ::core::ffi::c_double = 0.;
    let mut fa: ::core::ffi::c_double = 0.;
    let mut f0: ::core::ffi::c_double = (*infil).f0 * InfilFactor;
    let mut fmin: ::core::ffi::c_double = (*infil).fmin * InfilFactor;
    let mut df: ::core::ffi::c_double = f0 - fmin;
    let mut kd: ::core::ffi::c_double = (*infil).decay;
    let mut kr: ::core::ffi::c_double = (*infil).regen * Evap.recoveryFactor;
    if df < 0.0f64 || kd < 0.0f64 || kr < 0.0f64 {
        return 0.0f64;
    }
    if df == 0.0f64 || kd == 0.0f64 {
        fp = f0;
        fa = irate + depth / tstep;
        if fp > fa {
            fp = fa;
        }
        return if 0.0f64 >= fp { 0.0f64 } else { fp };
    }
    fa = irate + depth / tstep;
    if fa > ZERO {
        if (*infil).Fmax > 0.0f64 && (*infil).Fe >= (*infil).Fmax {
            return 0.0f64;
        }
        fp = f0 - kd * (*infil).Fe;
        fp = if fp >= fmin { fp } else { fmin };
        f = if fa <= fp { fa } else { fp };
        (*infil).Fe += (if f - fmin >= 0.0f64 { f - fmin } else { 0.0f64 }) * tstep;
        if (*infil).Fmax > 0.0f64 {
            (*infil).Fe = if (*infil).Fe >= (*infil).Fmax {
                (*infil).Fe
            } else {
                (*infil).Fmax
            };
        }
    } else if kr > 0.0f64 {
        (*infil).Fe *= exp(-kr * tstep);
        (*infil).Fe = if (*infil).Fe >= 0.0f64 {
            (*infil).Fe
        } else {
            0.0f64
        };
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn grnampt_getParams(
    mut j: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_double,
) {
    *p.offset(0 as ::core::ffi::c_int as isize) =
        (*Infil.offset(j as isize)).grnAmpt.S * UCF(RAINDEPTH as ::core::ffi::c_int);
    *p.offset(1 as ::core::ffi::c_int as isize) =
        (*Infil.offset(j as isize)).grnAmpt.Ks * UCF(RAINFALL as ::core::ffi::c_int);
    *p.offset(2 as ::core::ffi::c_int as isize) = (*Infil.offset(j as isize)).grnAmpt.IMDmax;
}
#[no_mangle]
pub unsafe extern "C" fn grnampt_setParams(
    mut infil: *mut TGrnAmpt,
    mut p: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ksat: ::core::ffi::c_double = 0.;
    if *p.offset(0 as ::core::ffi::c_int as isize) < 0.0f64
        || *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64
        || *p.offset(2 as ::core::ffi::c_int as isize) < 0.0f64
        || *p.offset(2 as ::core::ffi::c_int as isize) > 1.0f64
    {
        return FALSE;
    }
    (*infil).S = *p.offset(0 as ::core::ffi::c_int as isize) / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*infil).Ks = *p.offset(1 as ::core::ffi::c_int as isize) / UCF(RAINFALL as ::core::ffi::c_int);
    (*infil).IMDmax = *p.offset(2 as ::core::ffi::c_int as isize);
    ksat = (*infil).Ks * 12.0f64 * 3600.0f64;
    (*infil).Lu = 4.0f64 * sqrt(ksat) / 12.0f64;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn grnampt_initState(mut infil: *mut TGrnAmpt) {
    if infil.is_null() {
        return;
    }
    (*infil).IMD = (*infil).IMDmax;
    (*infil).Fu = 0.0f64;
    (*infil).F = 0.0f64;
    (*infil).Sat = FALSE as ::core::ffi::c_char;
    (*infil).T = 0.0f64;
}
unsafe extern "C" fn grnampt_getState(mut infil: *mut TGrnAmpt, mut x: *mut ::core::ffi::c_double) {
    *x.offset(0 as ::core::ffi::c_int as isize) = (*infil).IMD;
    *x.offset(1 as ::core::ffi::c_int as isize) = (*infil).F;
    *x.offset(2 as ::core::ffi::c_int as isize) = (*infil).Fu;
    *x.offset(3 as ::core::ffi::c_int as isize) = (*infil).Sat as ::core::ffi::c_double;
    *x.offset(4 as ::core::ffi::c_int as isize) = (*infil).T;
}
unsafe extern "C" fn grnampt_setState(mut infil: *mut TGrnAmpt, mut x: *mut ::core::ffi::c_double) {
    (*infil).IMD = *x.offset(0 as ::core::ffi::c_int as isize);
    (*infil).F = *x.offset(1 as ::core::ffi::c_int as isize);
    (*infil).Fu = *x.offset(2 as ::core::ffi::c_int as isize);
    (*infil).Sat = crate::ccast::f64_to_i8(*x.offset(3 as ::core::ffi::c_int as isize));
    (*infil).T = *x.offset(4 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn grnampt_getInfil(
    mut infil: *mut TGrnAmpt,
    mut tstep: ::core::ffi::c_double,
    mut irate: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
    mut modelType: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    Fumax = (*infil).IMDmax * (*infil).Lu * sqrt(InfilFactor);
    (*infil).T -= tstep;
    if (*infil).Sat != 0 {
        return grnampt_getSatInfil(infil, tstep, irate, depth);
    } else {
        return grnampt_getUnsatInfil(infil, tstep, irate, depth, modelType);
    };
}
unsafe extern "C" fn grnampt_getUnsatInfil(
    mut infil: *mut TGrnAmpt,
    mut tstep: ::core::ffi::c_double,
    mut irate: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
    mut modelType: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut ia: ::core::ffi::c_double = 0.;
    let mut c1: ::core::ffi::c_double = 0.;
    let mut F2: ::core::ffi::c_double = 0.;
    let mut dF: ::core::ffi::c_double = 0.;
    let mut Fs: ::core::ffi::c_double = 0.;
    let mut kr: ::core::ffi::c_double = 0.;
    let mut ts: ::core::ffi::c_double = 0.;
    let mut ks: ::core::ffi::c_double = (*infil).Ks * InfilFactor;
    let mut lu: ::core::ffi::c_double = (*infil).Lu * sqrt(InfilFactor);
    ia = irate + depth / tstep;
    if ia < ZERO {
        ia = 0.0f64;
    }
    if ia == 0.0f64 {
        if (*infil).Fu <= 0.0f64 {
            return 0.0f64;
        }
        kr = lu / 90000.0f64 * Evap.recoveryFactor;
        dF = kr * Fumax * tstep;
        (*infil).F -= dF;
        (*infil).Fu -= dF;
        if (*infil).Fu <= 0.0f64 {
            (*infil).Fu = 0.0f64;
            (*infil).F = 0.0f64;
            (*infil).IMD = (*infil).IMDmax;
            return 0.0f64;
        }
        if (*infil).T <= 0.0f64 {
            (*infil).IMD = (Fumax - (*infil).Fu) / lu;
            (*infil).F = 0.0f64;
        }
        return 0.0f64;
    }
    if ia <= ks {
        dF = ia * tstep;
        (*infil).F += dF;
        (*infil).Fu += dF;
        (*infil).Fu = if (*infil).Fu <= Fumax {
            (*infil).Fu
        } else {
            Fumax
        };
        if modelType == GREEN_AMPT as ::core::ffi::c_int && (*infil).T <= 0.0f64 {
            (*infil).IMD = (Fumax - (*infil).Fu) / lu;
            (*infil).F = 0.0f64;
        }
        return ia;
    }
    (*infil).T = 5400.0f64 / lu / Evap.recoveryFactor;
    Fs = ks * ((*infil).S + depth) * (*infil).IMD / (ia - ks);
    if (*infil).F > Fs {
        (*infil).Sat = TRUE as ::core::ffi::c_char;
        return grnampt_getSatInfil(infil, tstep, irate, depth);
    }
    if (*infil).F + ia * tstep < Fs {
        dF = ia * tstep;
        (*infil).F += dF;
        (*infil).Fu += dF;
        (*infil).Fu = if (*infil).Fu <= Fumax {
            (*infil).Fu
        } else {
            Fumax
        };
        return ia;
    }
    ts = tstep - (Fs - (*infil).F) / ia;
    if ts <= 0.0f64 {
        ts = 0.0f64;
    }
    c1 = ((*infil).S + depth) * (*infil).IMD;
    F2 = grnampt_getF2(Fs, c1, ks, ts);
    if F2 > Fs + ia * ts {
        F2 = Fs + ia * ts;
    }
    dF = F2 - (*infil).F;
    (*infil).F = F2;
    (*infil).Fu += dF;
    (*infil).Fu = if (*infil).Fu <= Fumax {
        (*infil).Fu
    } else {
        Fumax
    };
    (*infil).Sat = TRUE as ::core::ffi::c_char;
    return dF / tstep;
}
unsafe extern "C" fn grnampt_getSatInfil(
    mut infil: *mut TGrnAmpt,
    mut tstep: ::core::ffi::c_double,
    mut irate: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut ia: ::core::ffi::c_double = 0.;
    let mut c1: ::core::ffi::c_double = 0.;
    let mut dF: ::core::ffi::c_double = 0.;
    let mut F2: ::core::ffi::c_double = 0.;
    let mut ks: ::core::ffi::c_double = (*infil).Ks * InfilFactor;
    let mut lu: ::core::ffi::c_double = (*infil).Lu * sqrt(InfilFactor);
    ia = irate + depth / tstep;
    if ia < ZERO {
        return 0.0f64;
    }
    (*infil).T = 5400.0f64 / lu / Evap.recoveryFactor;
    c1 = ((*infil).S + depth) * (*infil).IMD;
    F2 = grnampt_getF2((*infil).F, c1, ks, tstep);
    dF = F2 - (*infil).F;
    if dF > ia * tstep {
        dF = ia * tstep;
        (*infil).Sat = FALSE as ::core::ffi::c_char;
    }
    (*infil).F += dF;
    (*infil).Fu += dF;
    (*infil).Fu = if (*infil).Fu <= Fumax {
        (*infil).Fu
    } else {
        Fumax
    };
    return dF / tstep;
}
unsafe extern "C" fn grnampt_getF2(
    mut f1: ::core::ffi::c_double,
    mut c1: ::core::ffi::c_double,
    mut ks: ::core::ffi::c_double,
    mut ts: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut f2: ::core::ffi::c_double = f1;
    let mut f2min: ::core::ffi::c_double = 0.;
    let mut df2: ::core::ffi::c_double = 0.;
    let mut c2: ::core::ffi::c_double = 0.;
    f2min = f1 + ks * ts;
    if c1 == 0.0f64 {
        return f2min;
    }
    if ts < 10.0f64 && f1 > 0.01f64 * c1 {
        f2 = f1 + ks * (1.0f64 + c1 / f1) * ts;
        return if f2 >= f2min { f2 } else { f2min };
    }
    c2 = c1 * log(f1 + c1) - ks * ts;
    i = 1 as ::core::ffi::c_int;
    while i <= 20 as ::core::ffi::c_int {
        df2 = (f2 - f1 - c1 * log(f2 + c1) + c2) / (1.0f64 - c1 / (f2 + c1));
        if fabs(df2) < 0.00001f64 {
            return if f2 >= f2min { f2 } else { f2min };
        }
        f2 -= df2;
        i += 1;
    }
    return f2min;
}
unsafe extern "C" fn curvenum_setParams(
    mut infil: *mut TCurveNum,
    mut p: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if *p.offset(0 as ::core::ffi::c_int as isize) < 10.0f64 {
        *p.offset(0 as ::core::ffi::c_int as isize) = 10.0f64;
    }
    if *p.offset(0 as ::core::ffi::c_int as isize) > 99.0f64 {
        *p.offset(0 as ::core::ffi::c_int as isize) = 99.0f64;
    }
    (*infil).Smax = (1000.0f64 / *p.offset(0 as ::core::ffi::c_int as isize) - 10.0f64) / 12.0f64;
    if (*infil).Smax < 0.0f64 {
        return FALSE;
    }
    if *p.offset(2 as ::core::ffi::c_int as isize) > 0.0f64 {
        (*infil).regen = 1.0f64
            / (*p.offset(2 as ::core::ffi::c_int as isize) * SECperDAY as ::core::ffi::c_double);
    } else {
        return FALSE;
    }
    (*infil).Tmax = 0.06f64 / (*infil).regen;
    return TRUE;
}
unsafe extern "C" fn curvenum_initState(mut infil: *mut TCurveNum) {
    (*infil).S = (*infil).Smax;
    (*infil).P = 0.0f64;
    (*infil).F = 0.0f64;
    (*infil).T = 0.0f64;
    (*infil).Se = (*infil).Smax;
    (*infil).f = 0.0f64;
}
unsafe extern "C" fn curvenum_getState(
    mut infil: *mut TCurveNum,
    mut x: *mut ::core::ffi::c_double,
) {
    *x.offset(0 as ::core::ffi::c_int as isize) = (*infil).S;
    *x.offset(1 as ::core::ffi::c_int as isize) = (*infil).P;
    *x.offset(2 as ::core::ffi::c_int as isize) = (*infil).F;
    *x.offset(3 as ::core::ffi::c_int as isize) = (*infil).T;
    *x.offset(4 as ::core::ffi::c_int as isize) = (*infil).Se;
    *x.offset(5 as ::core::ffi::c_int as isize) = (*infil).f;
}
unsafe extern "C" fn curvenum_setState(
    mut infil: *mut TCurveNum,
    mut x: *mut ::core::ffi::c_double,
) {
    (*infil).S = *x.offset(0 as ::core::ffi::c_int as isize);
    (*infil).P = *x.offset(1 as ::core::ffi::c_int as isize);
    (*infil).F = *x.offset(2 as ::core::ffi::c_int as isize);
    (*infil).T = *x.offset(3 as ::core::ffi::c_int as isize);
    (*infil).Se = *x.offset(4 as ::core::ffi::c_int as isize);
    (*infil).f = *x.offset(5 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn curvenum_getInfil(
    mut infil: *mut TCurveNum,
    mut tstep: ::core::ffi::c_double,
    mut irate: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut F1: ::core::ffi::c_double = 0.;
    let mut f1: ::core::ffi::c_double = 0.0f64;
    let mut fa: ::core::ffi::c_double = irate + depth / tstep;
    if irate > ZERO {
        if (*infil).T >= (*infil).Tmax {
            (*infil).P = 0.0f64;
            (*infil).F = 0.0f64;
            (*infil).f = 0.0f64;
            (*infil).Se = (*infil).S;
        }
        (*infil).T = 0.0f64;
        (*infil).P += irate * tstep;
        F1 = (*infil).P * (1.0f64 - (*infil).P / ((*infil).P + (*infil).Se));
        f1 = (F1 - (*infil).F) / tstep;
        if f1 < 0.0f64 || (*infil).S <= 0.0f64 {
            f1 = 0.0f64;
        }
    } else if depth > MIN_TOTAL_DEPTH && (*infil).S > 0.0f64 {
        f1 = (*infil).f;
        if f1 * tstep > (*infil).S {
            f1 = (*infil).S / tstep;
        }
    } else {
        (*infil).T += tstep;
    }
    if f1 > 0.0f64 {
        f1 = if f1 <= fa { f1 } else { fa };
        f1 = if f1 >= 0.0f64 { f1 } else { 0.0f64 };
        (*infil).F += f1 * tstep;
        if (*infil).regen > 0.0f64 {
            (*infil).S -= f1 * tstep;
            if (*infil).S < 0.0f64 {
                (*infil).S = 0.0f64;
            }
        }
    } else {
        (*infil).S += (*infil).regen * (*infil).Smax * tstep * Evap.recoveryFactor;
        if (*infil).S > (*infil).Smax {
            (*infil).S = (*infil).Smax;
        }
    }
    (*infil).f = f1;
    return f1;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
