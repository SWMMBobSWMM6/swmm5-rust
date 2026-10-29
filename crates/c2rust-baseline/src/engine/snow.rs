#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut UnitSystem: ::core::ffi::c_int;
    static mut Temp: TTemp;
    static mut Wind: TWind;
    static mut Snow: TSnow;
    static mut Snowmelt: *mut TSnowmelt;
    static mut Subcatch: *mut TSubcatch;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn gage_getPrecip(
        gage: ::core::ffi::c_int,
        rainfall: *mut ::core::ffi::c_double,
        snowfall: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut SnowmeltWords: [*mut ::core::ffi::c_char; 0];
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
pub type UnitsType = ::core::ffi::c_uint;
pub const SI: UnitsType = 1;
pub const US: UnitsType = 0;
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
pub type SubAreaType = ::core::ffi::c_uint;
pub const PERV: SubAreaType = 2;
pub const IMPERV1: SubAreaType = 1;
pub const IMPERV0: SubAreaType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTemp {
    pub dataSource: ::core::ffi::c_int,
    pub tSeries: ::core::ffi::c_int,
    pub fileStartDate: DateTime,
    pub elev: ::core::ffi::c_double,
    pub anglat: ::core::ffi::c_double,
    pub dtlong: ::core::ffi::c_double,
    pub ta: ::core::ffi::c_double,
    pub tmax: ::core::ffi::c_double,
    pub ea: ::core::ffi::c_double,
    pub gamma: ::core::ffi::c_double,
    pub tanAnglat: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TWind {
    pub type_0: ::core::ffi::c_int,
    pub aws: [::core::ffi::c_double; 12],
    pub ws: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSnow {
    pub snotmp: ::core::ffi::c_double,
    pub tipm: ::core::ffi::c_double,
    pub rnm: ::core::ffi::c_double,
    pub adc: [[::core::ffi::c_double; 10]; 2],
    pub season: ::core::ffi::c_double,
    pub removed: ::core::ffi::c_double,
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
pub struct TSnowmelt {
    pub ID: *mut ::core::ffi::c_char,
    pub snn: ::core::ffi::c_double,
    pub si: [::core::ffi::c_double; 3],
    pub dhmin: [::core::ffi::c_double; 3],
    pub dhmax: [::core::ffi::c_double; 3],
    pub tbase: [::core::ffi::c_double; 3],
    pub fwfrac: [::core::ffi::c_double; 3],
    pub wsnow: [::core::ffi::c_double; 3],
    pub fwnow: [::core::ffi::c_double; 3],
    pub weplow: ::core::ffi::c_double,
    pub sfrac: [::core::ffi::c_double; 5],
    pub toSubcatch: ::core::ffi::c_int,
    pub dhm: [::core::ffi::c_double; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLandFactor {
    pub fraction: ::core::ffi::c_double,
    pub buildup: *mut ::core::ffi::c_double,
    pub lastSwept: DateTime,
}
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
pub const SNOW_REMOVAL: SnowKeywords = 3;
pub const SNOW_PLOWABLE: SnowKeywords = 0;
pub const SNOW_PERV: SnowKeywords = 2;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_SNOWPACK_PARAMS: ErrorType = 182;
pub const SNOW_IMPERV: SnowKeywords = 1;
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
pub const ERR_DUP_NAME: ErrorType = 207;
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NONE: ErrorType = 0;
pub type SnowKeywords = ::core::ffi::c_uint;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn snow_readMeltParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 7] = [0.; 7];
    if ntoks < 8 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        SNOWMELT as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if (*Snowmelt.offset(j as isize)).ID.is_null() {
        let ref mut fresh0 = (*Snowmelt.offset(j as isize)).ID;
        *fresh0 = project_findID(
            SNOWMELT as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    k = findmatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut SnowmeltWords as *mut *mut ::core::ffi::c_char,
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n = 7 as ::core::ffi::c_int;
    if k == SNOW_REMOVAL as ::core::ffi::c_int {
        n = 6 as ::core::ffi::c_int;
    }
    if ntoks < n + 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        x[i as usize] = 0.0f64;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        if getDouble(
            *tok.offset((i + 2 as ::core::ffi::c_int) as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset((i + 2 as ::core::ffi::c_int) as isize),
            );
        }
        i += 1;
    }
    if k == SNOW_REMOVAL as ::core::ffi::c_int {
        x[6 as ::core::ffi::c_int as usize] = -1.0f64;
        if ntoks >= 9 as ::core::ffi::c_int {
            m = project_findObject(
                SUBCATCH as ::core::ffi::c_int,
                *tok.offset(8 as ::core::ffi::c_int as isize),
            );
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(8 as ::core::ffi::c_int as isize),
                );
            }
            x[6 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
        }
    }
    setMeltParams(j, k, &raw mut x as *mut ::core::ffi::c_double);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn snow_createSnowpack(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut snowpack: *mut TSnowpack = ::core::ptr::null_mut::<TSnowpack>();
    snowpack = malloc(::core::mem::size_of::<TSnowpack>() as size_t) as *mut TSnowpack;
    if snowpack.is_null() {
        return FALSE;
    }
    let ref mut fresh1 = (*Subcatch.offset(j as isize)).snowpack;
    *fresh1 = snowpack;
    (*snowpack).snowmeltIndex = k;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn snow_initSnowpack(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut snowDepth: ::core::ffi::c_double = 0.0f64;
    let mut snowpack: *mut TSnowpack = ::core::ptr::null_mut::<TSnowpack>();
    snowpack = (*Subcatch.offset(j as isize)).snowpack;
    if snowpack.is_null() {
        return;
    }
    k = (*(*Subcatch.offset(j as isize)).snowpack).snowmeltIndex;
    f = (*Snowmelt.offset(k as isize)).snn;
    (*snowpack).fArea[SNOW_PLOWABLE as ::core::ffi::c_int as usize] =
        f * (*Subcatch.offset(j as isize)).fracImperv;
    (*snowpack).fArea[SNOW_IMPERV as ::core::ffi::c_int as usize] =
        (1.0f64 - f) * (*Subcatch.offset(j as isize)).fracImperv;
    (*snowpack).fArea[SNOW_PERV as ::core::ffi::c_int as usize] =
        1.0f64 - (*Subcatch.offset(j as isize)).fracImperv;
    i = SNOW_PLOWABLE as ::core::ffi::c_int;
    while i <= SNOW_PERV as ::core::ffi::c_int {
        if (*snowpack).fArea[i as usize] > 0.0f64 {
            (*snowpack).wsnow[i as usize] = (*Snowmelt.offset(k as isize)).wsnow[i as usize];
            (*snowpack).fw[i as usize] = (*Snowmelt.offset(k as isize)).fwnow[i as usize];
        } else {
            (*snowpack).wsnow[i as usize] = 0.0f64;
            (*snowpack).fw[i as usize] = 0.0f64;
        }
        (*snowpack).coldc[i as usize] = 0.0f64;
        (*snowpack).ati[i as usize] = (*Snowmelt.offset(k as isize)).tbase[i as usize];
        (*snowpack).awe[i as usize] = 1.0f64;
        snowDepth += (*snowpack).wsnow[i as usize] * (*snowpack).fArea[i as usize];
        i += 1;
    }
    (*Subcatch.offset(j as isize)).newSnowDepth = snowDepth;
}
#[no_mangle]
pub unsafe extern "C" fn snow_initSnowmelt(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        (*Snowmelt.offset(j as isize)).snn = 0.0f64;
        (*Snowmelt.offset(j as isize)).si[i as usize] = 0.0f64;
        (*Snowmelt.offset(j as isize)).dhmin[i as usize] = 0.0f64;
        (*Snowmelt.offset(j as isize)).dhmax[i as usize] = 0.0f64;
        (*Snowmelt.offset(j as isize)).tbase[i as usize] = 0.0f64;
        (*Snowmelt.offset(j as isize)).fwfrac[i as usize] = 0.0f64;
        (*Snowmelt.offset(j as isize)).wsnow[i as usize] = 0.0f64;
        (*Snowmelt.offset(j as isize)).fwnow[i as usize] = 0.0f64;
        (*Snowmelt.offset(j as isize)).weplow = 1.0e6f64;
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int {
            (*Snowmelt.offset(j as isize)).sfrac[k as usize] = 0.0f64;
            k += 1;
        }
        (*Snowmelt.offset(j as isize)).toSubcatch = -(1 as ::core::ffi::c_int);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn snow_validateSnowmelt(mut j: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_char = FALSE as ::core::ffi::c_char;
    let mut sum: ::core::ffi::c_double = 0.0f64;
    k = SNOW_PLOWABLE as ::core::ffi::c_int;
    while k <= SNOW_PERV as ::core::ffi::c_int {
        if (*Snowmelt.offset(j as isize)).dhmin[k as usize]
            > (*Snowmelt.offset(j as isize)).dhmax[k as usize]
        {
            err = TRUE as ::core::ffi::c_char;
        }
        if (*Snowmelt.offset(j as isize)).fwfrac[k as usize] < 0.0f64
            || (*Snowmelt.offset(j as isize)).fwfrac[k as usize] > 1.0f64
        {
            err = TRUE as ::core::ffi::c_char;
        }
        k += 1;
    }
    if (*Snowmelt.offset(j as isize)).snn < 0.0f64 || (*Snowmelt.offset(j as isize)).snn > 1.0f64 {
        err = TRUE as ::core::ffi::c_char;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 5 as ::core::ffi::c_int {
        sum += (*Snowmelt.offset(j as isize)).sfrac[k as usize];
        k += 1;
    }
    if sum > 1.01f64 {
        err = TRUE as ::core::ffi::c_char;
    }
    if err != 0 {
        report_writeErrorMsg(
            ERR_SNOWPACK_PARAMS as ::core::ffi::c_int,
            (*Snowmelt.offset(j as isize)).ID,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn snow_getState(
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    let mut snowpack: *mut TSnowpack = (*Subcatch.offset(i as isize)).snowpack;
    if snowpack.is_null() {
        return;
    }
    *x.offset(0 as ::core::ffi::c_int as isize) = (*snowpack).wsnow[j as usize];
    *x.offset(1 as ::core::ffi::c_int as isize) = (*snowpack).fw[j as usize];
    *x.offset(2 as ::core::ffi::c_int as isize) = (*snowpack).coldc[j as usize];
    *x.offset(3 as ::core::ffi::c_int as isize) = (*snowpack).ati[j as usize];
    *x.offset(4 as ::core::ffi::c_int as isize) = (*snowpack).awe[j as usize];
}
#[no_mangle]
pub unsafe extern "C" fn snow_setState(
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    let mut snowpack: *mut TSnowpack = (*Subcatch.offset(i as isize)).snowpack;
    if snowpack.is_null() {
        return;
    }
    (*snowpack).wsnow[j as usize] = *x.offset(0 as ::core::ffi::c_int as isize);
    (*snowpack).fw[j as usize] = *x.offset(1 as ::core::ffi::c_int as isize);
    (*snowpack).coldc[j as usize] = *x.offset(2 as ::core::ffi::c_int as isize);
    (*snowpack).ati[j as usize] = *x.offset(3 as ::core::ffi::c_int as isize);
    (*snowpack).awe[j as usize] = *x.offset(4 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn setMeltParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    if k >= SNOW_PLOWABLE as ::core::ffi::c_int && k <= SNOW_PERV as ::core::ffi::c_int {
        (*Snowmelt.offset(j as isize)).dhmin[k as usize] =
            *x.offset(0 as ::core::ffi::c_int as isize) * UCF(TEMPERATURE as ::core::ffi::c_int)
                / UCF(RAINFALL as ::core::ffi::c_int);
        (*Snowmelt.offset(j as isize)).dhmax[k as usize] =
            *x.offset(1 as ::core::ffi::c_int as isize) * UCF(TEMPERATURE as ::core::ffi::c_int)
                / UCF(RAINFALL as ::core::ffi::c_int);
        (*Snowmelt.offset(j as isize)).tbase[k as usize] =
            *x.offset(2 as ::core::ffi::c_int as isize);
        if UnitSystem == SI as ::core::ffi::c_int {
            (*Snowmelt.offset(j as isize)).tbase[k as usize] =
                9.0f64 / 5.0f64 * (*Snowmelt.offset(j as isize)).tbase[k as usize] + 32.0f64;
        }
        (*Snowmelt.offset(j as isize)).fwfrac[k as usize] =
            *x.offset(3 as ::core::ffi::c_int as isize);
        (*Snowmelt.offset(j as isize)).wsnow[k as usize] =
            *x.offset(4 as ::core::ffi::c_int as isize) / UCF(RAINDEPTH as ::core::ffi::c_int);
        *x.offset(5 as ::core::ffi::c_int as isize) = if *x.offset(5 as ::core::ffi::c_int as isize)
            <= *x.offset(3 as ::core::ffi::c_int as isize)
                * *x.offset(4 as ::core::ffi::c_int as isize)
        {
            *x.offset(5 as ::core::ffi::c_int as isize)
        } else {
            *x.offset(3 as ::core::ffi::c_int as isize)
                * *x.offset(4 as ::core::ffi::c_int as isize)
        };
        (*Snowmelt.offset(j as isize)).fwnow[k as usize] =
            *x.offset(5 as ::core::ffi::c_int as isize) / UCF(RAINDEPTH as ::core::ffi::c_int);
        if k == SNOW_PLOWABLE as ::core::ffi::c_int {
            (*Snowmelt.offset(j as isize)).snn = *x.offset(6 as ::core::ffi::c_int as isize);
        } else {
            (*Snowmelt.offset(j as isize)).si[k as usize] =
                *x.offset(6 as ::core::ffi::c_int as isize) / UCF(RAINDEPTH as ::core::ffi::c_int);
        }
    } else if k == SNOW_REMOVAL as ::core::ffi::c_int {
        (*Snowmelt.offset(j as isize)).weplow =
            *x.offset(0 as ::core::ffi::c_int as isize) / UCF(RAINDEPTH as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i <= 4 as ::core::ffi::c_int {
            (*Snowmelt.offset(j as isize)).sfrac[i as usize] =
                *x.offset((i + 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
        if *x.offset(6 as ::core::ffi::c_int as isize) >= 0.0f64 {
            (*Snowmelt.offset(j as isize)).toSubcatch =
                crate::ccast::f64_to_i32((*x.offset(6 as ::core::ffi::c_int as isize) + 0.01f64));
        } else {
            (*Snowmelt.offset(j as isize)).toSubcatch = -(1 as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn snow_setMeltCoeffs(
    mut j: ::core::ffi::c_int,
    mut s: ::core::ffi::c_double,
) {
    let mut k: ::core::ffi::c_int = 0;
    k = SNOW_PLOWABLE as ::core::ffi::c_int;
    while k <= SNOW_PERV as ::core::ffi::c_int {
        (*Snowmelt.offset(j as isize)).dhm[k as usize] = 0.5f64
            * ((*Snowmelt.offset(j as isize)).dhmax[k as usize] * (1.0f64 + s)
                + (*Snowmelt.offset(j as isize)).dhmin[k as usize] * (1.0f64 - s));
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn snow_plowSnow(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut rainfall: ::core::ffi::c_double = 0.;
    let mut snowfall: ::core::ffi::c_double = 0.;
    let mut exc: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut sfracTotal: ::core::ffi::c_double = 0.;
    let mut snowpack: *mut TSnowpack = ::core::ptr::null_mut::<TSnowpack>();
    snowpack = (*Subcatch.offset(j as isize)).snowpack;
    if snowpack.is_null() {
        return;
    }
    gage_getPrecip(
        (*Subcatch.offset(j as isize)).gage,
        &raw mut rainfall,
        &raw mut snowfall,
    );
    i = SNOW_PLOWABLE as ::core::ffi::c_int;
    while i <= SNOW_PERV as ::core::ffi::c_int {
        if (*snowpack).fArea[i as usize] > 0.0f64 {
            (*snowpack).wsnow[i as usize] += snowfall * tStep;
            (*snowpack).imelt[i as usize] = 0.0f64;
        }
        i += 1;
    }
    if (*snowpack).fArea[SNOW_PLOWABLE as ::core::ffi::c_int as usize] > 0.0f64 {
        k = (*snowpack).snowmeltIndex;
        if (*snowpack).wsnow[SNOW_PLOWABLE as ::core::ffi::c_int as usize]
            >= (*Snowmelt.offset(k as isize)).weplow
        {
            exc = (*snowpack).wsnow[SNOW_PLOWABLE as ::core::ffi::c_int as usize];
            f = (*snowpack).fArea[SNOW_PLOWABLE as ::core::ffi::c_int as usize]
                * ((*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea);
            Snow.removed +=
                (*Snowmelt.offset(k as isize)).sfrac[0 as ::core::ffi::c_int as usize] * exc * f;
            sfracTotal = (*Snowmelt.offset(k as isize)).sfrac[0 as ::core::ffi::c_int as usize];
            if (*snowpack).fArea[SNOW_IMPERV as ::core::ffi::c_int as usize] > 0.0f64 {
                f = (*snowpack).fArea[SNOW_PLOWABLE as ::core::ffi::c_int as usize]
                    / (*snowpack).fArea[SNOW_IMPERV as ::core::ffi::c_int as usize];
                (*snowpack).wsnow[SNOW_IMPERV as ::core::ffi::c_int as usize] +=
                    (*Snowmelt.offset(k as isize)).sfrac[1 as ::core::ffi::c_int as usize]
                        * exc
                        * f;
                sfracTotal +=
                    (*Snowmelt.offset(k as isize)).sfrac[1 as ::core::ffi::c_int as usize];
            }
            if (*snowpack).fArea[SNOW_PERV as ::core::ffi::c_int as usize] > 0.0f64 {
                f = (*snowpack).fArea[SNOW_PLOWABLE as ::core::ffi::c_int as usize]
                    / (*snowpack).fArea[SNOW_PERV as ::core::ffi::c_int as usize];
                (*snowpack).wsnow[SNOW_PERV as ::core::ffi::c_int as usize] +=
                    (*Snowmelt.offset(k as isize)).sfrac[2 as ::core::ffi::c_int as usize]
                        * exc
                        * f;
                sfracTotal +=
                    (*Snowmelt.offset(k as isize)).sfrac[2 as ::core::ffi::c_int as usize];
            }
            (*snowpack).imelt[SNOW_PLOWABLE as ::core::ffi::c_int as usize] =
                (*Snowmelt.offset(k as isize)).sfrac[3 as ::core::ffi::c_int as usize] * exc
                    / tStep;
            sfracTotal += (*Snowmelt.offset(k as isize)).sfrac[3 as ::core::ffi::c_int as usize];
            if (*Snowmelt.offset(k as isize)).sfrac[4 as ::core::ffi::c_int as usize] > 0.0f64 {
                m = (*Snowmelt.offset(k as isize)).toSubcatch;
                if !(*Subcatch.offset(m as isize)).snowpack.is_null() {
                    f = (*(*Subcatch.offset(m as isize)).snowpack).fArea
                        [SNOW_PERV as ::core::ffi::c_int as usize];
                } else {
                    f = 0.0f64;
                }
                if f > 0.0f64 {
                    f = (*snowpack).fArea[SNOW_PLOWABLE as ::core::ffi::c_int as usize] / f;
                    (*(*Subcatch.offset(m as isize)).snowpack).wsnow
                        [SNOW_PERV as ::core::ffi::c_int as usize] +=
                        (*Snowmelt.offset(k as isize)).sfrac[4 as ::core::ffi::c_int as usize]
                            * exc
                            * f;
                    sfracTotal +=
                        (*Snowmelt.offset(k as isize)).sfrac[4 as ::core::ffi::c_int as usize];
                }
            }
            sfracTotal = if sfracTotal <= 1.0f64 {
                sfracTotal
            } else {
                1.0f64
            };
            (*snowpack).wsnow[SNOW_PLOWABLE as ::core::ffi::c_int as usize] =
                exc * (1.0f64 - sfracTotal);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn snow_getSnowMelt(
    mut j: ::core::ffi::c_int,
    mut rainfall: ::core::ffi::c_double,
    mut snowfall: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
    mut netPrecip: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut rmelt: ::core::ffi::c_double = 0.;
    let mut smelt: ::core::ffi::c_double = 0.;
    let mut asc: ::core::ffi::c_double = 0.;
    let mut snowDepth: ::core::ffi::c_double = 0.0f64;
    let mut impervPrecip: ::core::ffi::c_double = 0.;
    let mut snowpack: *mut TSnowpack = ::core::ptr::null_mut::<TSnowpack>();
    snowpack = (*Subcatch.offset(j as isize)).snowpack;
    rmelt = getRainmelt(rainfall);
    i = SNOW_PLOWABLE as ::core::ffi::c_int;
    while i <= SNOW_PERV as ::core::ffi::c_int {
        if (*snowpack).wsnow[i as usize] <= 0.001f64 / 12.0f64 {
            asc = 0.0f64;
            smelt = 0.0f64;
            (*snowpack).imelt[i as usize] +=
                ((*snowpack).wsnow[i as usize] + (*snowpack).fw[i as usize]) / tStep;
            (*snowpack).wsnow[i as usize] = 0.0f64;
            (*snowpack).fw[i as usize] = 0.0f64;
            (*snowpack).coldc[i as usize] = 0.0f64;
        } else {
            asc = getArealDepletion(snowpack, i, snowfall, tStep);
            smelt = meltSnowpack(snowpack, i, rmelt, asc, snowfall, tStep);
            smelt = routeSnowmelt(snowpack, i, smelt, asc, rainfall, tStep);
        }
        *netPrecip.offset(i as isize) =
            smelt + (*snowpack).imelt[i as usize] + rainfall * (1.0f64 - asc);
        snowDepth += (*snowpack).wsnow[i as usize] * (*snowpack).fArea[i as usize];
        i += 1;
    }
    if (*Subcatch.offset(j as isize)).fracImperv > 0.0f64 {
        impervPrecip = (*netPrecip.offset(SNOW_PLOWABLE as ::core::ffi::c_int as isize)
            * (*snowpack).fArea[SNOW_PLOWABLE as ::core::ffi::c_int as usize]
            + *netPrecip.offset(SNOW_IMPERV as ::core::ffi::c_int as isize)
                * (*snowpack).fArea[SNOW_IMPERV as ::core::ffi::c_int as usize])
            / (*Subcatch.offset(j as isize)).fracImperv;
        *netPrecip.offset(IMPERV0 as ::core::ffi::c_int as isize) = impervPrecip;
        *netPrecip.offset(IMPERV1 as ::core::ffi::c_int as isize) = impervPrecip;
    }
    return snowDepth;
}
#[no_mangle]
pub unsafe extern "C" fn snow_getSnowCover(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut snowCover: ::core::ffi::c_double = 0.0f64;
    let mut snowpack: *mut TSnowpack = ::core::ptr::null_mut::<TSnowpack>();
    snowpack = (*Subcatch.offset(j as isize)).snowpack;
    if snowpack.is_null() {
        return 0.0f64;
    }
    i = SNOW_PLOWABLE as ::core::ffi::c_int;
    while i <= SNOW_PERV as ::core::ffi::c_int {
        snowCover += ((*snowpack).wsnow[i as usize] + (*snowpack).fw[i as usize])
            * (*snowpack).fArea[i as usize];
        i += 1;
    }
    return snowCover
        * ((*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea);
}
unsafe extern "C" fn getArealDepletion(
    mut snowpack: *mut TSnowpack,
    mut i: ::core::ffi::c_int,
    mut snowfall: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut asc: ::core::ffi::c_double = 0.;
    let mut si: ::core::ffi::c_double = 0.;
    let mut awesi: ::core::ffi::c_double = 0.;
    let mut awe: ::core::ffi::c_double = 0.;
    let mut sba: ::core::ffi::c_double = 0.;
    let mut sbws: ::core::ffi::c_double = 0.;
    if i == SNOW_PLOWABLE as ::core::ffi::c_int {
        return 1.0f64;
    }
    k = (*snowpack).snowmeltIndex;
    si = (*Snowmelt.offset(k as isize)).si[i as usize];
    if si == 0.0f64 || (*snowpack).wsnow[i as usize] >= si {
        (*snowpack).awe[i as usize] = 1.0f64;
        return 1.0f64;
    }
    if (*snowpack).wsnow[i as usize] == 0.0f64 {
        (*snowpack).awe[i as usize] = 1.0f64;
        return 0.0f64;
    }
    if snowfall > 0.0f64 {
        awe = ((*snowpack).wsnow[i as usize] - snowfall * tStep) / si;
        awe = if awe >= 0.0f64 { awe } else { 0.0f64 };
        sba = getArealSnowCover(i, awe);
        sbws = awe + 0.75f64 * snowfall * tStep / si;
        sbws = if sbws <= 1.0f64 { sbws } else { 1.0f64 };
        (*snowpack).awe[i as usize] = awe;
        (*snowpack).sba[i as usize] = sba;
        (*snowpack).sbws[i as usize] = sbws;
        return 1.0f64;
    } else {
        awe = (*snowpack).awe[i as usize];
        sba = (*snowpack).sba[i as usize];
        sbws = (*snowpack).sbws[i as usize];
        awesi = (*snowpack).wsnow[i as usize] / si;
        if awesi < (*snowpack).awe[i as usize] {
            (*snowpack).awe[i as usize] = 1.0f64;
            asc = getArealSnowCover(i, awesi);
        } else if awesi >= (*snowpack).sbws[i as usize] {
            asc = 1.0f64;
        } else {
            asc = sba + (1.0f64 - sba) / (sbws - awe) * (awesi - awe);
        }
        return asc;
    };
}
unsafe extern "C" fn getArealSnowCover(
    mut i: ::core::ffi::c_int,
    mut awesi: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut asc1: ::core::ffi::c_double = 0.;
    let mut asc2: ::core::ffi::c_double = 0.;
    if i == SNOW_IMPERV as ::core::ffi::c_int {
        k = 0 as ::core::ffi::c_int;
    } else if i == SNOW_PERV as ::core::ffi::c_int {
        k = 1 as ::core::ffi::c_int;
    } else {
        return 1.0f64;
    }
    if awesi <= 0.0f64 {
        return 0.0f64;
    }
    if awesi >= 0.9999f64 {
        return 1.0f64;
    }
    m = crate::ccast::f64_to_i32((awesi * 10.0f64 + 0.00001f64));
    asc1 = Snow.adc[k as usize][m as usize];
    if m >= 9 as ::core::ffi::c_int {
        asc2 = 1.0f64;
    } else {
        asc2 = Snow.adc[k as usize][(m + 1 as ::core::ffi::c_int) as usize];
    }
    return asc1
        + (asc2 - asc1) / 0.1f64
            * (awesi - 0.1f64 * m as ::core::ffi::c_float as ::core::ffi::c_double);
}
unsafe extern "C" fn meltSnowpack(
    mut snowpack: *mut TSnowpack,
    mut i: ::core::ffi::c_int,
    mut rmelt: ::core::ffi::c_double,
    mut asc: ::core::ffi::c_double,
    mut snowfall: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut smelt: ::core::ffi::c_double = 0.;
    let mut ccFactor: ::core::ffi::c_double = 0.;
    k = (*snowpack).snowmeltIndex;
    if rmelt > 0.0f64 {
        smelt = rmelt;
    } else if Temp.ta >= (*Snowmelt.offset(k as isize)).tbase[i as usize] {
        smelt = (*Snowmelt.offset(k as isize)).dhm[i as usize]
            * (Temp.ta - (*Snowmelt.offset(k as isize)).tbase[i as usize]);
    } else {
        updateColdContent(snowpack, i, asc, snowfall, tStep);
        return 0.0f64;
    }
    smelt *= asc;
    ccFactor = tStep * Snow.rnm * asc;
    smelt = reduceColdContent(snowpack, i, smelt, ccFactor);
    (*snowpack).ati[i as usize] = (*Snowmelt.offset(k as isize)).tbase[i as usize];
    return smelt;
}
unsafe extern "C" fn getRainmelt(mut rainfall: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut uadj: ::core::ffi::c_double = 0.;
    let mut t1: ::core::ffi::c_double = 0.;
    let mut t2: ::core::ffi::c_double = 0.;
    let mut t3: ::core::ffi::c_double = 0.;
    let mut smelt: ::core::ffi::c_double = 0.;
    rainfall = rainfall * 43200.0f64;
    if rainfall > 0.02f64 {
        uadj = 0.006f64 * Wind.ws;
        t1 = Temp.ta - 32.0f64;
        t2 = 7.5f64 * Temp.gamma * uadj;
        t3 = 8.5f64 * uadj * (Temp.ea - 0.18f64);
        smelt = t1 * (0.001167f64 + t2 + 0.007f64 * rainfall) + t3;
        return smelt / 43200.0f64;
    } else {
        return 0.0f64;
    };
}
unsafe extern "C" fn updateColdContent(
    mut snowpack: *mut TSnowpack,
    mut i: ::core::ffi::c_int,
    mut asc: ::core::ffi::c_double,
    mut snowfall: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut k: ::core::ffi::c_int = 0;
    let mut ati: ::core::ffi::c_double = 0.;
    let mut cc: ::core::ffi::c_double = 0.;
    let mut ccMax: ::core::ffi::c_double = 0.;
    let mut tipm: ::core::ffi::c_double = 0.;
    ati = (*snowpack).ati[i as usize];
    cc = (*snowpack).coldc[i as usize];
    if snowfall * 43200.0f64 > 0.02f64 {
        ati = Temp.ta;
    } else {
        tipm = 1.0f64 - pow(1.0f64 - Snow.tipm, tStep / (6.0f64 * 3600.0f64));
        ati += tipm * (Temp.ta - ati);
    }
    k = (*snowpack).snowmeltIndex;
    ati = if ati <= (*Snowmelt.offset(k as isize)).tbase[i as usize] {
        ati
    } else {
        (*Snowmelt.offset(k as isize)).tbase[i as usize]
    };
    cc += Snow.rnm * (*Snowmelt.offset(k as isize)).dhm[i as usize] * (ati - Temp.ta) * tStep * asc;
    cc = if cc >= 0.0f64 { cc } else { 0.0f64 };
    ccMax = (*snowpack).wsnow[i as usize] * 0.007f64 / 12.0f64
        * ((*Snowmelt.offset(k as isize)).tbase[i as usize] - ati);
    cc = if cc <= ccMax { cc } else { ccMax };
    (*snowpack).coldc[i as usize] = cc;
    (*snowpack).ati[i as usize] = ati;
}
unsafe extern "C" fn reduceColdContent(
    mut snowpack: *mut TSnowpack,
    mut i: ::core::ffi::c_int,
    mut smelt: ::core::ffi::c_double,
    mut ccFactor: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut cc: ::core::ffi::c_double = 0.;
    cc = (*snowpack).coldc[i as usize];
    if smelt * ccFactor > cc {
        smelt -= cc / ccFactor;
        cc = 0.0f64;
    } else {
        cc -= smelt * ccFactor;
        smelt = 0.0f64;
    }
    (*snowpack).coldc[i as usize] = cc;
    return smelt;
}
unsafe extern "C" fn routeSnowmelt(
    mut snowpack: *mut TSnowpack,
    mut i: ::core::ffi::c_int,
    mut smelt: ::core::ffi::c_double,
    mut asc: ::core::ffi::c_double,
    mut rainfall: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut vmelt: ::core::ffi::c_double = 0.;
    k = (*snowpack).snowmeltIndex;
    vmelt = smelt * tStep;
    vmelt = if vmelt <= (*snowpack).wsnow[i as usize] {
        vmelt
    } else {
        (*snowpack).wsnow[i as usize]
    };
    (*snowpack).wsnow[i as usize] -= vmelt;
    (*snowpack).fw[i as usize] += vmelt + rainfall * tStep * asc;
    vmelt = (*snowpack).fw[i as usize]
        - (*Snowmelt.offset(k as isize)).fwfrac[i as usize] * (*snowpack).wsnow[i as usize];
    vmelt = if vmelt >= 0.0f64 { vmelt } else { 0.0f64 };
    (*snowpack).fw[i as usize] -= vmelt;
    return vmelt / tStep;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
