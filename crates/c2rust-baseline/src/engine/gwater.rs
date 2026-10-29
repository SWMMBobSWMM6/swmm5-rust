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
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn datetime_monthOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn mathexpr_create(
        s: *mut ::core::ffi::c_char,
        getVar: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>,
    ) -> *mut MathExpr;
    fn mathexpr_eval(
        expr: *mut MathExpr,
        getVal: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_double>,
    ) -> ::core::ffi::c_double;
    fn mathexpr_delete(expr: *mut MathExpr);
    static mut NewRunoffTime: ::core::ffi::c_double;
    static mut Evap: TEvap;
    static mut Subcatch: *mut TSubcatch;
    static mut Aquifer: *mut TAquifer;
    static mut Node: *mut TNode;
    static mut Pattern: *mut TPattern;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn massbal_updateGwaterTotals(
        vInfil: ::core::ffi::c_double,
        vUpperEvap: ::core::ffi::c_double,
        vLowerEvap: ::core::ffi::c_double,
        vLowerPerc: ::core::ffi::c_double,
        vGwater: ::core::ffi::c_double,
    );
    fn stats_updateGwaterStats(
        j: ::core::ffi::c_int,
        infil: ::core::ffi::c_double,
        evap: ::core::ffi::c_double,
        latFlow: ::core::ffi::c_double,
        deepFlow: ::core::ffi::c_double,
        theta: ::core::ffi::c_double,
        waterTable: ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
    );
    fn subcatch_getFracPerv(subcatch: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "match"]
    fn match_0(
        str: *mut ::core::ffi::c_char,
        substr: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sstrncpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        n: size_t,
    ) -> size_t;
    fn sstrcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        destsize: size_t,
    ) -> size_t;
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn odesolve_integrate(
        ystart: *mut ::core::ffi::c_double,
        n: ::core::ffi::c_int,
        x1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        eps: ::core::ffi::c_double,
        h1: ::core::ffi::c_double,
        derivs: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
            ) -> (),
        >,
    ) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAquifer {
    pub ID: *mut ::core::ffi::c_char,
    pub porosity: ::core::ffi::c_double,
    pub wiltingPoint: ::core::ffi::c_double,
    pub fieldCapacity: ::core::ffi::c_double,
    pub conductivity: ::core::ffi::c_double,
    pub conductSlope: ::core::ffi::c_double,
    pub tensionSlope: ::core::ffi::c_double,
    pub upperEvapFrac: ::core::ffi::c_double,
    pub lowerEvapDepth: ::core::ffi::c_double,
    pub lowerLossCoeff: ::core::ffi::c_double,
    pub bottomElev: ::core::ffi::c_double,
    pub waterTableElev: ::core::ffi::c_double,
    pub upperMoisture: ::core::ffi::c_double,
    pub upperEvapPat: ::core::ffi::c_int,
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
pub const ERR_NAME: ErrorType = 209;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_MATH_EXPR: ErrorType = 233;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_AQUIFER_PARAMS: ErrorType = 109;
pub const ERR_GROUND_ELEV: ErrorType = 110;
pub const THETA: GWstates = 0;
pub const LOWERDEPTH: GWstates = 1;
pub const gwvA: GWvariables = 10;
pub const gwvFU: GWvariables = 9;
pub const gwvFI: GWvariables = 8;
pub const gwvPHI: GWvariables = 7;
pub const gwvTHETA: GWvariables = 6;
pub const gwvK: GWvariables = 5;
pub const gwvKS: GWvariables = 4;
pub const gwvHGS: GWvariables = 3;
pub const gwvHCB: GWvariables = 2;
pub const gwvHSW: GWvariables = 1;
pub const gwvHGW: GWvariables = 0;
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
pub const ERR_SUBCATCH_OUTLET: ErrorType = 108;
pub const ERR_TIMESTEP: ErrorType = 107;
pub const ERR_ODE_SOLVER: ErrorType = 105;
pub const ERR_KINWAVE: ErrorType = 103;
pub const ERR_NONE: ErrorType = 0;
pub type GWstates = ::core::ffi::c_uint;
pub type GWvariables = ::core::ffi::c_uint;
pub const gwvMAX: GWvariables = 11;
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MISSING: ::core::ffi::c_double = -1.0E10f64;
static mut GWTOL: ::core::ffi::c_double = 0.0001f64;
static mut XTOL: ::core::ffi::c_double = 0.001f64;
static mut GWVarWords: [*mut ::core::ffi::c_char; 12] = [
    b"HGW\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"HSW\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"HCB\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"HGS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"KS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"THETA\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"PHI\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"FI\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"FU\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut Area: ::core::ffi::c_double = 0.;
static mut Infil: ::core::ffi::c_double = 0.;
static mut MaxEvap: ::core::ffi::c_double = 0.;
static mut AvailEvap: ::core::ffi::c_double = 0.;
static mut UpperEvap: ::core::ffi::c_double = 0.;
static mut LowerEvap: ::core::ffi::c_double = 0.;
static mut UpperPerc: ::core::ffi::c_double = 0.;
static mut LowerLoss: ::core::ffi::c_double = 0.;
static mut GWFlow: ::core::ffi::c_double = 0.;
static mut MaxUpperPerc: ::core::ffi::c_double = 0.;
static mut MaxGWFlowPos: ::core::ffi::c_double = 0.;
static mut MaxGWFlowNeg: ::core::ffi::c_double = 0.;
static mut FracPerv: ::core::ffi::c_double = 0.;
static mut TotalDepth: ::core::ffi::c_double = 0.;
static mut Theta: ::core::ffi::c_double = 0.;
static mut HydCon: ::core::ffi::c_double = 0.;
static mut Hgw: ::core::ffi::c_double = 0.;
static mut Hstar: ::core::ffi::c_double = 0.;
static mut Hsw: ::core::ffi::c_double = 0.;
static mut Tstep: ::core::ffi::c_double = 0.;
static mut A: TAquifer = TAquifer {
    ID: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    porosity: 0.,
    wiltingPoint: 0.,
    fieldCapacity: 0.,
    conductivity: 0.,
    conductSlope: 0.,
    tensionSlope: 0.,
    upperEvapFrac: 0.,
    lowerEvapDepth: 0.,
    lowerLossCoeff: 0.,
    bottomElev: 0.,
    waterTableElev: 0.,
    upperMoisture: 0.,
    upperEvapPat: 0,
};
static mut GW: *mut TGroundwater = ::core::ptr::null::<TGroundwater>() as *mut TGroundwater;
static mut LatFlowExpr: *mut MathExpr = ::core::ptr::null::<MathExpr>() as *mut MathExpr;
static mut DeepFlowExpr: *mut MathExpr = ::core::ptr::null::<MathExpr>() as *mut MathExpr;
#[no_mangle]
pub unsafe extern "C" fn gwater_readAquiferParams(
    mut j: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 12] = [0.; 12];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 13 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        AQUIFER as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < 11 as ::core::ffi::c_int {
        x[i as usize] = 0.0f64;
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < 13 as ::core::ffi::c_int {
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
    p = -(1 as ::core::ffi::c_int);
    if ntoks > 13 as ::core::ffi::c_int {
        p = project_findObject(
            TIMEPATTERN as ::core::ffi::c_int,
            *tok.offset(13 as ::core::ffi::c_int as isize),
        );
        if p < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(13 as ::core::ffi::c_int as isize),
            );
        }
    }
    let ref mut fresh0 = (*Aquifer.offset(j as isize)).ID;
    *fresh0 = id;
    (*Aquifer.offset(j as isize)).porosity = x[0 as ::core::ffi::c_int as usize];
    (*Aquifer.offset(j as isize)).wiltingPoint = x[1 as ::core::ffi::c_int as usize];
    (*Aquifer.offset(j as isize)).fieldCapacity = x[2 as ::core::ffi::c_int as usize];
    (*Aquifer.offset(j as isize)).conductivity =
        x[3 as ::core::ffi::c_int as usize] / UCF(RAINFALL as ::core::ffi::c_int);
    (*Aquifer.offset(j as isize)).conductSlope = x[4 as ::core::ffi::c_int as usize];
    (*Aquifer.offset(j as isize)).tensionSlope =
        x[5 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
    (*Aquifer.offset(j as isize)).upperEvapFrac = x[6 as ::core::ffi::c_int as usize];
    (*Aquifer.offset(j as isize)).lowerEvapDepth =
        x[7 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
    (*Aquifer.offset(j as isize)).lowerLossCoeff =
        x[8 as ::core::ffi::c_int as usize] / UCF(RAINFALL as ::core::ffi::c_int);
    (*Aquifer.offset(j as isize)).bottomElev =
        x[9 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
    (*Aquifer.offset(j as isize)).waterTableElev =
        x[10 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
    (*Aquifer.offset(j as isize)).upperMoisture = x[11 as ::core::ffi::c_int as usize];
    (*Aquifer.offset(j as isize)).upperEvapPat = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gwater_readGroundwaterParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 11] = [0.; 11];
    let mut gw: *mut TGroundwater = ::core::ptr::null_mut::<TGroundwater>();
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
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
    if ntoks < 11 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    k = project_findObject(
        AQUIFER as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    if n < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset((i + 3 as ::core::ffi::c_int) as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset((i + 3 as ::core::ffi::c_int) as isize),
            );
        }
        i += 1;
    }
    i = 7 as ::core::ffi::c_int;
    while i < 11 as ::core::ffi::c_int {
        x[i as usize] = MISSING;
        m = i + 3 as ::core::ffi::c_int;
        if ntoks > m && **tok.offset(m as isize) as ::core::ffi::c_int != '*' as i32 {
            if getDouble(
                *tok.offset(m as isize),
                (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(m as isize),
                );
            }
            if i < 10 as ::core::ffi::c_int {
                x[i as usize] /= UCF(LENGTH as ::core::ffi::c_int);
            }
        }
        i += 1;
    }
    if (*Subcatch.offset(j as isize)).groundwater.is_null() {
        gw = malloc(::core::mem::size_of::<TGroundwater>() as size_t) as *mut TGroundwater;
        if gw.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        let ref mut fresh1 = (*Subcatch.offset(j as isize)).groundwater;
        *fresh1 = gw;
    } else {
        gw = (*Subcatch.offset(j as isize)).groundwater;
    }
    (*gw).aquifer = k;
    (*gw).node = n;
    (*gw).surfElev = x[0 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
    (*gw).a1 = x[1 as ::core::ffi::c_int as usize];
    (*gw).b1 = x[2 as ::core::ffi::c_int as usize];
    (*gw).a2 = x[3 as ::core::ffi::c_int as usize];
    (*gw).b2 = x[4 as ::core::ffi::c_int as usize];
    (*gw).a3 = x[5 as ::core::ffi::c_int as usize];
    (*gw).fixedDepth = x[6 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
    (*gw).nodeElev = x[7 as ::core::ffi::c_int as usize];
    (*gw).bottomElev = x[8 as ::core::ffi::c_int as usize];
    (*gw).waterTableElev = x[9 as ::core::ffi::c_int as usize];
    (*gw).upperMoisture = x[10 as ::core::ffi::c_int as usize];
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gwater_readFlowExpression(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut exprStr: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut expr: *mut MathExpr = ::core::ptr::null_mut::<MathExpr>();
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
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
    k = 1 as ::core::ffi::c_int;
    if match_0(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        b"LAT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        k = 1 as ::core::ffi::c_int;
    } else if match_0(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        b"DEEP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        k = 2 as ::core::ffi::c_int;
    } else {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    sstrncpy(
        &raw mut exprStr as *mut ::core::ffi::c_char,
        *tok.offset(2 as ::core::ffi::c_int as isize),
        MAXLINE as size_t,
    );
    i = 3 as ::core::ffi::c_int;
    while i < ntoks {
        sstrcat(
            &raw mut exprStr as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            (MAXLINE + 1 as ::core::ffi::c_int) as size_t,
        );
        sstrcat(
            &raw mut exprStr as *mut ::core::ffi::c_char,
            *tok.offset(i as isize),
            (MAXLINE + 1 as ::core::ffi::c_int) as size_t,
        );
        i += 1;
    }
    if k == 1 as ::core::ffi::c_int {
        mathexpr_delete((*Subcatch.offset(j as isize)).gwLatFlowExpr);
    } else {
        mathexpr_delete((*Subcatch.offset(j as isize)).gwDeepFlowExpr);
    }
    expr = mathexpr_create(
        &raw mut exprStr as *mut ::core::ffi::c_char,
        Some(
            getVariableIndex
                as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
    );
    if expr.is_null() {
        return error_setInpError(
            ERR_MATH_EXPR as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if k == 1 as ::core::ffi::c_int {
        let ref mut fresh2 = (*Subcatch.offset(j as isize)).gwLatFlowExpr;
        *fresh2 = expr;
    } else {
        let ref mut fresh3 = (*Subcatch.offset(j as isize)).gwDeepFlowExpr;
        *fresh3 = expr;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gwater_deleteFlowExpression(mut j: ::core::ffi::c_int) {
    mathexpr_delete((*Subcatch.offset(j as isize)).gwLatFlowExpr);
    mathexpr_delete((*Subcatch.offset(j as isize)).gwDeepFlowExpr);
}
#[no_mangle]
pub unsafe extern "C" fn gwater_validateAquifer(mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    if (*Aquifer.offset(j as isize)).porosity <= 0.0f64
        || (*Aquifer.offset(j as isize)).fieldCapacity >= (*Aquifer.offset(j as isize)).porosity
        || (*Aquifer.offset(j as isize)).wiltingPoint >= (*Aquifer.offset(j as isize)).fieldCapacity
        || (*Aquifer.offset(j as isize)).conductivity <= 0.0f64
        || (*Aquifer.offset(j as isize)).conductSlope < 0.0f64
        || (*Aquifer.offset(j as isize)).tensionSlope < 0.0f64
        || (*Aquifer.offset(j as isize)).upperEvapFrac < 0.0f64
        || (*Aquifer.offset(j as isize)).lowerEvapDepth < 0.0f64
        || (*Aquifer.offset(j as isize)).waterTableElev < (*Aquifer.offset(j as isize)).bottomElev
        || (*Aquifer.offset(j as isize)).upperMoisture > (*Aquifer.offset(j as isize)).porosity
        || (*Aquifer.offset(j as isize)).upperMoisture < (*Aquifer.offset(j as isize)).wiltingPoint
    {
        report_writeErrorMsg(
            ERR_AQUIFER_PARAMS as ::core::ffi::c_int,
            (*Aquifer.offset(j as isize)).ID,
        );
    }
    p = (*Aquifer.offset(j as isize)).upperEvapPat;
    if p >= 0 as ::core::ffi::c_int
        && (*Pattern.offset(p as isize)).type_0 != MONTHLY_PATTERN as ::core::ffi::c_int
    {
        report_writeErrorMsg(
            ERR_AQUIFER_PARAMS as ::core::ffi::c_int,
            (*Aquifer.offset(j as isize)).ID,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn gwater_validate(mut j: ::core::ffi::c_int) {
    let mut a: TAquifer = TAquifer {
        ID: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        porosity: 0.,
        wiltingPoint: 0.,
        fieldCapacity: 0.,
        conductivity: 0.,
        conductSlope: 0.,
        tensionSlope: 0.,
        upperEvapFrac: 0.,
        lowerEvapDepth: 0.,
        lowerLossCoeff: 0.,
        bottomElev: 0.,
        waterTableElev: 0.,
        upperMoisture: 0.,
        upperEvapPat: 0,
    };
    let mut gw: *mut TGroundwater = ::core::ptr::null_mut::<TGroundwater>();
    gw = (*Subcatch.offset(j as isize)).groundwater;
    if !gw.is_null() {
        a = *Aquifer.offset((*gw).aquifer as isize);
        if (*gw).bottomElev == MISSING {
            (*gw).bottomElev = a.bottomElev;
        }
        if (*gw).waterTableElev == MISSING {
            (*gw).waterTableElev = a.waterTableElev;
        }
        if (*gw).upperMoisture == MISSING {
            (*gw).upperMoisture = a.upperMoisture;
        }
        if (*gw).surfElev < (*gw).waterTableElev {
            report_writeErrorMsg(
                ERR_GROUND_ELEV as ::core::ffi::c_int,
                (*Subcatch.offset(j as isize)).ID,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gwater_initState(mut j: ::core::ffi::c_int) {
    let mut a: TAquifer = TAquifer {
        ID: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        porosity: 0.,
        wiltingPoint: 0.,
        fieldCapacity: 0.,
        conductivity: 0.,
        conductSlope: 0.,
        tensionSlope: 0.,
        upperEvapFrac: 0.,
        lowerEvapDepth: 0.,
        lowerLossCoeff: 0.,
        bottomElev: 0.,
        waterTableElev: 0.,
        upperMoisture: 0.,
        upperEvapPat: 0,
    };
    let mut gw: *mut TGroundwater = ::core::ptr::null_mut::<TGroundwater>();
    gw = (*Subcatch.offset(j as isize)).groundwater;
    if !gw.is_null() {
        a = *Aquifer.offset((*gw).aquifer as isize);
        (*gw).theta = (*gw).upperMoisture;
        if (*gw).theta >= a.porosity {
            (*gw).theta = a.porosity - XTOL;
        }
        (*gw).lowerDepth = (*gw).waterTableElev - (*gw).bottomElev;
        if (*gw).lowerDepth >= (*gw).surfElev - (*gw).bottomElev {
            (*gw).lowerDepth = (*gw).surfElev - (*gw).bottomElev - XTOL;
        }
        (*gw).oldFlow = 0.0f64;
        (*gw).newFlow = 0.0f64;
        (*gw).evapLoss = 0.0f64;
        (*gw).maxInfilVol = ((*gw).surfElev - (*gw).waterTableElev) * (a.porosity - (*gw).theta)
            / subcatch_getFracPerv(j);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gwater_getState(
    mut j: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    let mut gw: *mut TGroundwater = (*Subcatch.offset(j as isize)).groundwater;
    *x.offset(0 as ::core::ffi::c_int as isize) = (*gw).theta;
    *x.offset(1 as ::core::ffi::c_int as isize) = (*gw).bottomElev + (*gw).lowerDepth;
    *x.offset(2 as ::core::ffi::c_int as isize) = (*gw).newFlow;
    *x.offset(3 as ::core::ffi::c_int as isize) = (*gw).maxInfilVol;
}
#[no_mangle]
pub unsafe extern "C" fn gwater_setState(
    mut j: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    let mut gw: *mut TGroundwater = (*Subcatch.offset(j as isize)).groundwater;
    if gw.is_null() {
        return;
    }
    (*gw).theta = *x.offset(0 as ::core::ffi::c_int as isize);
    (*gw).lowerDepth = *x.offset(1 as ::core::ffi::c_int as isize) - (*gw).bottomElev;
    (*gw).oldFlow = *x.offset(2 as ::core::ffi::c_int as isize);
    if *x.offset(3 as ::core::ffi::c_int as isize) != MISSING {
        (*gw).maxInfilVol = *x.offset(3 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gwater_getVolume(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut a: TAquifer = TAquifer {
        ID: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        porosity: 0.,
        wiltingPoint: 0.,
        fieldCapacity: 0.,
        conductivity: 0.,
        conductSlope: 0.,
        tensionSlope: 0.,
        upperEvapFrac: 0.,
        lowerEvapDepth: 0.,
        lowerLossCoeff: 0.,
        bottomElev: 0.,
        waterTableElev: 0.,
        upperMoisture: 0.,
        upperEvapPat: 0,
    };
    let mut gw: *mut TGroundwater = ::core::ptr::null_mut::<TGroundwater>();
    let mut upperDepth: ::core::ffi::c_double = 0.;
    gw = (*Subcatch.offset(j as isize)).groundwater;
    if gw.is_null() {
        return 0.0f64;
    }
    a = *Aquifer.offset((*gw).aquifer as isize);
    upperDepth = (*gw).surfElev - (*gw).bottomElev - (*gw).lowerDepth;
    return upperDepth * (*gw).theta + (*gw).lowerDepth * a.porosity;
}
#[no_mangle]
pub unsafe extern "C" fn gwater_getGroundwater(
    mut j: ::core::ffi::c_int,
    mut evap: ::core::ffi::c_double,
    mut infil: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut n: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 2] = [0.; 2];
    let mut vUpper: ::core::ffi::c_double = 0.;
    let mut nodeFlow: ::core::ffi::c_double = 0.;
    GW = (*Subcatch.offset(j as isize)).groundwater;
    if GW.is_null() {
        return;
    }
    LatFlowExpr = (*Subcatch.offset(j as isize)).gwLatFlowExpr;
    DeepFlowExpr = (*Subcatch.offset(j as isize)).gwDeepFlowExpr;
    A = *Aquifer.offset((*GW).aquifer as isize);
    FracPerv = subcatch_getFracPerv(j);
    if FracPerv <= 0.0f64 {
        return;
    }
    Area = (*Subcatch.offset(j as isize)).area;
    infil = infil / Area / tStep;
    Infil = infil;
    Tstep = tStep;
    evap = evap / Area / tStep;
    MaxEvap = Evap.rate * FracPerv;
    AvailEvap = if MaxEvap - evap >= 0.0f64 {
        MaxEvap - evap
    } else {
        0.0f64
    };
    TotalDepth = (*GW).surfElev - (*GW).bottomElev;
    if TotalDepth <= 0.0f64 {
        return;
    }
    n = (*GW).node;
    if (*GW).nodeElev != MISSING {
        Hstar = (*GW).nodeElev - (*GW).bottomElev;
    } else {
        Hstar = (*Node.offset(n as isize)).invertElev - (*GW).bottomElev;
    }
    if (*GW).fixedDepth > 0.0f64 {
        Hsw = (*GW).fixedDepth + (*Node.offset(n as isize)).invertElev - (*GW).bottomElev;
    } else {
        Hsw = (*Node.offset(n as isize)).newDepth + (*Node.offset(n as isize)).invertElev
            - (*GW).bottomElev;
    }
    x[THETA as ::core::ffi::c_int as usize] = (*GW).theta;
    x[LOWERDEPTH as ::core::ffi::c_int as usize] = (*GW).lowerDepth;
    vUpper = (TotalDepth - x[LOWERDEPTH as ::core::ffi::c_int as usize])
        * (x[THETA as ::core::ffi::c_int as usize] - A.fieldCapacity);
    vUpper = if 0.0f64 >= vUpper { 0.0f64 } else { vUpper };
    MaxUpperPerc = vUpper / tStep;
    MaxGWFlowPos = x[LOWERDEPTH as ::core::ffi::c_int as usize] * A.porosity / tStep;
    MaxGWFlowNeg = (TotalDepth - x[LOWERDEPTH as ::core::ffi::c_int as usize])
        * (A.porosity - x[THETA as ::core::ffi::c_int as usize])
        / tStep;
    nodeFlow =
        ((*Node.offset(n as isize)).inflow + (*Node.offset(n as isize)).newVolume / tStep) / Area;
    MaxGWFlowNeg = -if MaxGWFlowNeg <= nodeFlow {
        MaxGWFlowNeg
    } else {
        nodeFlow
    };
    odesolve_integrate(
        &raw mut x as *mut ::core::ffi::c_double,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        tStep,
        GWTOL,
        tStep,
        Some(
            getDxDt
                as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                ) -> (),
        ),
    );
    x[THETA as ::core::ffi::c_int as usize] =
        if x[THETA as ::core::ffi::c_int as usize] >= A.wiltingPoint {
            x[THETA as ::core::ffi::c_int as usize]
        } else {
            A.wiltingPoint
        };
    if x[THETA as ::core::ffi::c_int as usize] >= A.porosity {
        x[THETA as ::core::ffi::c_int as usize] = A.porosity - XTOL;
        x[LOWERDEPTH as ::core::ffi::c_int as usize] = TotalDepth - XTOL;
    }
    x[LOWERDEPTH as ::core::ffi::c_int as usize] =
        if x[LOWERDEPTH as ::core::ffi::c_int as usize] >= 0.0f64 {
            x[LOWERDEPTH as ::core::ffi::c_int as usize]
        } else {
            0.0f64
        };
    if x[LOWERDEPTH as ::core::ffi::c_int as usize] >= TotalDepth {
        x[LOWERDEPTH as ::core::ffi::c_int as usize] = TotalDepth - XTOL;
    }
    (*GW).theta = x[THETA as ::core::ffi::c_int as usize];
    (*GW).lowerDepth = x[LOWERDEPTH as ::core::ffi::c_int as usize];
    getFluxes((*GW).theta, (*GW).lowerDepth);
    (*GW).oldFlow = (*GW).newFlow;
    (*GW).newFlow = GWFlow;
    (*GW).evapLoss = UpperEvap + LowerEvap;
    (*GW).maxInfilVol = (TotalDepth - x[LOWERDEPTH as ::core::ffi::c_int as usize])
        * (A.porosity - x[THETA as ::core::ffi::c_int as usize])
        / FracPerv;
    updateMassBal(Area, tStep);
    stats_updateGwaterStats(
        j,
        infil,
        (*GW).evapLoss,
        GWFlow,
        LowerLoss,
        (*GW).theta,
        (*GW).lowerDepth + (*GW).bottomElev,
        tStep,
    );
}
unsafe extern "C" fn updateMassBal(
    mut area: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut vInfil: ::core::ffi::c_double = 0.;
    let mut vUpperEvap: ::core::ffi::c_double = 0.;
    let mut vLowerEvap: ::core::ffi::c_double = 0.;
    let mut vLowerPerc: ::core::ffi::c_double = 0.;
    let mut vGwater: ::core::ffi::c_double = 0.;
    let mut ft2sec: ::core::ffi::c_double = area * tStep;
    vInfil = Infil * ft2sec;
    vUpperEvap = UpperEvap * ft2sec;
    vLowerEvap = LowerEvap * ft2sec;
    vLowerPerc = LowerLoss * ft2sec;
    vGwater = 0.5f64 * ((*GW).oldFlow + (*GW).newFlow) * ft2sec;
    massbal_updateGwaterTotals(vInfil, vUpperEvap, vLowerEvap, vLowerPerc, vGwater);
}
unsafe extern "C" fn getFluxes(
    mut theta: ::core::ffi::c_double,
    mut lowerDepth: ::core::ffi::c_double,
) {
    let mut upperDepth: ::core::ffi::c_double = 0.;
    lowerDepth = if lowerDepth >= 0.0f64 {
        lowerDepth
    } else {
        0.0f64
    };
    lowerDepth = if lowerDepth <= TotalDepth {
        lowerDepth
    } else {
        TotalDepth
    };
    upperDepth = TotalDepth - lowerDepth;
    Hgw = lowerDepth;
    Theta = theta;
    getEvapRates(theta, upperDepth);
    UpperPerc = getUpperPerc(theta, upperDepth);
    UpperPerc = if UpperPerc <= MaxUpperPerc {
        UpperPerc
    } else {
        MaxUpperPerc
    };
    if !DeepFlowExpr.is_null() {
        LowerLoss = mathexpr_eval(
            DeepFlowExpr,
            Some(
                getVariableValue
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_double,
            ),
        ) / UCF(RAINFALL as ::core::ffi::c_int);
    } else {
        LowerLoss = A.lowerLossCoeff * lowerDepth / TotalDepth;
    }
    LowerLoss = if LowerLoss <= lowerDepth / Tstep {
        LowerLoss
    } else {
        lowerDepth / Tstep
    };
    GWFlow = getGWFlow(lowerDepth);
    if !LatFlowExpr.is_null() {
        GWFlow += mathexpr_eval(
            LatFlowExpr,
            Some(
                getVariableValue
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_double,
            ),
        ) / UCF(GWFLOW as ::core::ffi::c_int);
    }
    if GWFlow >= 0.0f64 {
        GWFlow = if GWFlow <= MaxGWFlowPos {
            GWFlow
        } else {
            MaxGWFlowPos
        };
    } else {
        GWFlow = if GWFlow >= MaxGWFlowNeg {
            GWFlow
        } else {
            MaxGWFlowNeg
        };
    };
}
unsafe extern "C" fn getDxDt(
    mut t: ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_double,
    mut dxdt: *mut ::core::ffi::c_double,
) {
    let mut qUpper: ::core::ffi::c_double = 0.;
    let mut qLower: ::core::ffi::c_double = 0.;
    let mut denom: ::core::ffi::c_double = 0.;
    getFluxes(
        *x.offset(THETA as ::core::ffi::c_int as isize),
        *x.offset(LOWERDEPTH as ::core::ffi::c_int as isize),
    );
    qUpper = Infil - UpperEvap - UpperPerc;
    qLower = UpperPerc - LowerLoss - LowerEvap - GWFlow;
    denom = TotalDepth - *x.offset(LOWERDEPTH as ::core::ffi::c_int as isize);
    if denom > 0.0f64 {
        *dxdt.offset(THETA as ::core::ffi::c_int as isize) = qUpper / denom;
    } else {
        *dxdt.offset(THETA as ::core::ffi::c_int as isize) = 0.0f64;
    }
    denom = A.porosity - *x.offset(THETA as ::core::ffi::c_int as isize);
    if denom > 0.0f64 {
        *dxdt.offset(LOWERDEPTH as ::core::ffi::c_int as isize) = qLower / denom;
    } else {
        *dxdt.offset(LOWERDEPTH as ::core::ffi::c_int as isize) = 0.0f64;
    };
}
unsafe extern "C" fn getEvapRates(
    mut theta: ::core::ffi::c_double,
    mut upperDepth: ::core::ffi::c_double,
) {
    let mut p: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut lowerFrac: ::core::ffi::c_double = 0.;
    let mut upperFrac: ::core::ffi::c_double = 0.;
    UpperEvap = 0.0f64;
    LowerEvap = 0.0f64;
    if Infil > 0.0f64 {
        return;
    }
    upperFrac = A.upperEvapFrac;
    f = 1.0f64;
    p = A.upperEvapPat;
    if p >= 0 as ::core::ffi::c_int {
        month = datetime_monthOfYear(getDateTime(NewRunoffTime));
        f = (*Pattern.offset(p as isize)).factor[(month - 1 as ::core::ffi::c_int) as usize];
    }
    upperFrac *= f;
    if theta > A.wiltingPoint {
        UpperEvap = upperFrac * MaxEvap;
        UpperEvap = if UpperEvap <= AvailEvap {
            UpperEvap
        } else {
            AvailEvap
        };
    }
    if A.lowerEvapDepth > 0.0f64 {
        lowerFrac = (A.lowerEvapDepth - upperDepth) / A.lowerEvapDepth;
        lowerFrac = if 0.0f64 >= lowerFrac {
            0.0f64
        } else {
            lowerFrac
        };
        lowerFrac = if lowerFrac <= 1.0f64 {
            lowerFrac
        } else {
            1.0f64
        };
        LowerEvap = lowerFrac * (1.0f64 - upperFrac) * MaxEvap;
        LowerEvap = if LowerEvap <= AvailEvap - UpperEvap {
            LowerEvap
        } else {
            AvailEvap - UpperEvap
        };
    }
}
unsafe extern "C" fn getUpperPerc(
    mut theta: ::core::ffi::c_double,
    mut upperDepth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut delta: ::core::ffi::c_double = 0.;
    let mut dhdz: ::core::ffi::c_double = 0.;
    let mut hydcon: ::core::ffi::c_double = 0.;
    if upperDepth <= 0.0f64 || theta <= A.fieldCapacity {
        return 0.0f64;
    }
    delta = theta - A.porosity;
    hydcon = A.conductivity * exp(delta * A.conductSlope);
    delta = theta - A.fieldCapacity;
    dhdz = 1.0f64 + A.tensionSlope * 2.0f64 * delta / upperDepth;
    HydCon = hydcon;
    return hydcon * dhdz;
}
unsafe extern "C" fn getGWFlow(mut lowerDepth: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut q: ::core::ffi::c_double = 0.;
    let mut t1: ::core::ffi::c_double = 0.;
    let mut t2: ::core::ffi::c_double = 0.;
    let mut t3: ::core::ffi::c_double = 0.;
    if lowerDepth <= Hstar {
        return 0.0f64;
    }
    if (*GW).b1 == 0.0f64 {
        t1 = (*GW).a1;
    } else {
        t1 = (*GW).a1
            * pow(
                (lowerDepth - Hstar) * UCF(LENGTH as ::core::ffi::c_int),
                (*GW).b1,
            );
    }
    if (*GW).b2 == 0.0f64 {
        t2 = (*GW).a2;
    } else if Hsw > Hstar {
        t2 = (*GW).a2 * pow((Hsw - Hstar) * UCF(LENGTH as ::core::ffi::c_int), (*GW).b2);
    } else {
        t2 = 0.0f64;
    }
    t3 = (*GW).a3
        * lowerDepth
        * Hsw
        * UCF(LENGTH as ::core::ffi::c_int)
        * UCF(LENGTH as ::core::ffi::c_int);
    q = (t1 - t2 + t3) / UCF(GWFLOW as ::core::ffi::c_int);
    if q < 0.0f64 && (*GW).a3 != 0.0f64 {
        q = 0.0f64;
    }
    return q;
}
unsafe extern "C" fn getVariableIndex(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    k = findmatch(s, &raw mut GWVarWords as *mut *mut ::core::ffi::c_char);
    if k >= 0 as ::core::ffi::c_int {
        return k;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn getVariableValue(mut varIndex: ::core::ffi::c_int) -> ::core::ffi::c_double {
    match varIndex {
        0 => return Hgw * UCF(LENGTH as ::core::ffi::c_int),
        1 => return Hsw * UCF(LENGTH as ::core::ffi::c_int),
        2 => return Hstar * UCF(LENGTH as ::core::ffi::c_int),
        3 => return TotalDepth * UCF(LENGTH as ::core::ffi::c_int),
        4 => return A.conductivity * UCF(RAINFALL as ::core::ffi::c_int),
        5 => return HydCon * UCF(RAINFALL as ::core::ffi::c_int),
        6 => return Theta,
        7 => return A.porosity,
        8 => return Infil * UCF(RAINFALL as ::core::ffi::c_int),
        9 => return UpperPerc * UCF(RAINFALL as ::core::ffi::c_int),
        10 => return Area * UCF(LANDAREA as ::core::ffi::c_int),
        _ => return 0.0f64,
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
