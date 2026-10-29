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
    fn datetime_monthOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn infil_initState(j: ::core::ffi::c_int);
    fn infil_setInfilFactor(j: ::core::ffi::c_int);
    fn infil_getInfil(
        area: ::core::ffi::c_int,
        tstep: ::core::ffi::c_double,
        rainfall: ::core::ffi::c_double,
        runon: ::core::ffi::c_double,
        depth: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut IgnoreSnowmelt: ::core::ffi::c_int;
    static mut IgnoreGwater: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut OldRunoffTime: ::core::ffi::c_double;
    static mut Evap: TEvap;
    static mut Gage: *mut TGage;
    static mut Subcatch: *mut TSubcatch;
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
    fn snow_createSnowpack(
        subcacth: ::core::ffi::c_int,
        snowIndex: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn snow_initSnowpack(subcatch: ::core::ffi::c_int);
    fn snow_getSnowMelt(
        subcatch: ::core::ffi::c_int,
        rainfall: ::core::ffi::c_double,
        snowfall: ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
        netPrecip: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn gwater_validate(subcatch: ::core::ffi::c_int);
    fn gwater_initState(subcatch: ::core::ffi::c_int);
    fn gwater_getGroundwater(
        subcatch: ::core::ffi::c_int,
        evap: ::core::ffi::c_double,
        infil: ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
    );
    fn massbal_updateRunoffTotals(type_0: ::core::ffi::c_int, v: ::core::ffi::c_double);
    fn stats_updateSubcatchStats(
        subcatch: ::core::ffi::c_int,
        rainVol: ::core::ffi::c_double,
        runonVol: ::core::ffi::c_double,
        evapVol: ::core::ffi::c_double,
        infilVol: ::core::ffi::c_double,
        impervVol: ::core::ffi::c_double,
        pervVol: ::core::ffi::c_double,
        runoffVol: ::core::ffi::c_double,
        runoff: ::core::ffi::c_double,
    );
    fn gage_getPrecip(
        gage: ::core::ffi::c_int,
        rainfall: *mut ::core::ffi::c_double,
        snowfall: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn surfqual_initState(subcatch: ::core::ffi::c_int);
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn lid_setOldGroupState(subcatch: ::core::ffi::c_int);
    fn lid_getPervArea(subcatch: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn lid_getFlowToPerv(subcatch: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn lid_getDrainFlow(
        subcatch: ::core::ffi::c_int,
        timePeriod: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn lid_getStoredVolume(subcatch: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn lid_addDrainRunon(subcatch: ::core::ffi::c_int);
    fn lid_getRunoff(subcatch: ::core::ffi::c_int, tStep: ::core::ffi::c_double);
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
pub type SubcatchResultType = ::core::ffi::c_uint;
pub const SUBCATCH_WASHOFF: SubcatchResultType = 8;
pub const SUBCATCH_SOIL_MOIST: SubcatchResultType = 7;
pub const SUBCATCH_GW_ELEV: SubcatchResultType = 6;
pub const SUBCATCH_GW_FLOW: SubcatchResultType = 5;
pub const SUBCATCH_RUNOFF: SubcatchResultType = 4;
pub const SUBCATCH_INFIL: SubcatchResultType = 3;
pub const SUBCATCH_EVAP: SubcatchResultType = 2;
pub const SUBCATCH_SNOWDEPTH: SubcatchResultType = 1;
pub const SUBCATCH_RAINFALL: SubcatchResultType = 0;
pub type RunoffFlowType = ::core::ffi::c_uint;
pub const RUNOFF_RUNON: RunoffFlowType = 5;
pub const RUNOFF_DRAINS: RunoffFlowType = 4;
pub const RUNOFF_RUNOFF: RunoffFlowType = 3;
pub const RUNOFF_INFIL: RunoffFlowType = 2;
pub const RUNOFF_EVAP: RunoffFlowType = 1;
pub const RUNOFF_RAINFALL: RunoffFlowType = 0;
pub type SubAreaType = ::core::ffi::c_uint;
pub const PERV: SubAreaType = 2;
pub const IMPERV1: SubAreaType = 1;
pub const IMPERV0: SubAreaType = 0;
pub type RunoffRoutingType = ::core::ffi::c_uint;
pub const TO_PERV: RunoffRoutingType = 2;
pub const TO_IMPERV: RunoffRoutingType = 1;
pub const TO_OUTLET: RunoffRoutingType = 0;
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
pub struct TGage {
    pub ID: *mut ::core::ffi::c_char,
    pub dataSource: ::core::ffi::c_int,
    pub tSeries: ::core::ffi::c_int,
    pub fname: [::core::ffi::c_char; 260],
    pub staID: [::core::ffi::c_char; 1025],
    pub startFileDate: DateTime,
    pub endFileDate: DateTime,
    pub rainType: ::core::ffi::c_int,
    pub rainInterval: ::core::ffi::c_int,
    pub rainUnits: ::core::ffi::c_int,
    pub snowFactor: ::core::ffi::c_double,
    pub startFilePos: ::core::ffi::c_long,
    pub endFilePos: ::core::ffi::c_long,
    pub currentFilePos: ::core::ffi::c_long,
    pub rainAccum: ::core::ffi::c_double,
    pub unitsFactor: ::core::ffi::c_double,
    pub startDate: DateTime,
    pub endDate: DateTime,
    pub nextDate: DateTime,
    pub rainfall: ::core::ffi::c_double,
    pub nextRainfall: ::core::ffi::c_double,
    pub apiRainfall: ::core::ffi::c_double,
    pub reportRainfall: ::core::ffi::c_double,
    pub pastRain: [::core::ffi::c_double; 49],
    pub pastInterval: ::core::ffi::c_int,
    pub coGage: ::core::ffi::c_int,
    pub isUsed: ::core::ffi::c_int,
    pub isCurrent: ::core::ffi::c_int,
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_SUBCATCH_OUTLET: ErrorType = 108;
pub const CURRENT: TimePeriod = 1;
pub const PREVIOUS: TimePeriod = 0;
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
pub const ERR_TIMESTEP: ErrorType = 107;
pub const ERR_ODE_SOLVER: ErrorType = 105;
pub const ERR_KINWAVE: ErrorType = 103;
pub const ERR_NONE: ErrorType = 0;
pub type TimePeriod = ::core::ffi::c_uint;
pub const w_OUTLET: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"OUTLET\0") };
pub const w_IMPERV: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"IMPERV\0") };
pub const w_PERV: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"PERV\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut MCOEFF: ::core::ffi::c_double = 1.49f64;
#[no_mangle]
pub static mut MEXP: ::core::ffi::c_double = 1.6666667f64;
#[no_mangle]
pub static mut ODETOL: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub static mut Vevap: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut Vpevap: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut Vinfil: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut Vinflow: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut Voutflow: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut VlidIn: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut VlidInfil: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut VlidOut: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut VlidDrain: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut VlidReturn: ::core::ffi::c_double = 0.;
static mut theSubarea: *mut TSubarea = ::core::ptr::null::<TSubarea>() as *mut TSubarea;
static mut Dstore: ::core::ffi::c_double = 0.;
static mut Alpha: ::core::ffi::c_double = 0.;
static mut RunoffRoutingWords: [*mut ::core::ffi::c_char; 4] = [
    w_OUTLET.as_ptr() as *mut ::core::ffi::c_char,
    w_IMPERV.as_ptr() as *mut ::core::ffi::c_char,
    w_PERV.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn subcatch_readParams(
    mut j: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut x: [::core::ffi::c_double; 9] = [0.; 9];
    if ntoks < 8 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        SUBCATCH as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    k = project_findObject(
        GAGE as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    x[0 as ::core::ffi::c_int as usize] = k as ::core::ffi::c_double;
    m = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    x[1 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    m = project_findObject(
        SUBCATCH as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    x[2 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    if x[1 as ::core::ffi::c_int as usize] < 0.0f64 && x[2 as ::core::ffi::c_int as usize] < 0.0f64
    {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    i = 3 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[i as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(i as isize));
        }
        i += 1;
    }
    x[8 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    if ntoks > 8 as ::core::ffi::c_int {
        k = project_findObject(
            SNOWMELT as ::core::ffi::c_int,
            *tok.offset(8 as ::core::ffi::c_int as isize),
        );
        if k < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(8 as ::core::ffi::c_int as isize),
            );
        }
        x[8 as ::core::ffi::c_int as usize] = k as ::core::ffi::c_double;
    }
    let ref mut fresh0 = (*Subcatch.offset(j as isize)).ID;
    *fresh0 = id;
    (*Subcatch.offset(j as isize)).gage = crate::ccast::f64_to_i32(x[0 as ::core::ffi::c_int as usize]);
    (*Subcatch.offset(j as isize)).outNode =
        crate::ccast::f64_to_i32(x[1 as ::core::ffi::c_int as usize]);
    (*Subcatch.offset(j as isize)).outSubcatch =
        crate::ccast::f64_to_i32(x[2 as ::core::ffi::c_int as usize]);
    (*Subcatch.offset(j as isize)).area =
        x[3 as ::core::ffi::c_int as usize] / UCF(LANDAREA as ::core::ffi::c_int);
    (*Subcatch.offset(j as isize)).fracImperv = (if x[4 as ::core::ffi::c_int as usize] <= 100.0f64
    {
        x[4 as ::core::ffi::c_int as usize]
    } else {
        100.0f64
    }) / 100.0f64;
    (*Subcatch.offset(j as isize)).width =
        x[5 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
    (*Subcatch.offset(j as isize)).slope = x[6 as ::core::ffi::c_int as usize] / 100.0f64;
    (*Subcatch.offset(j as isize)).curbLength = x[7 as ::core::ffi::c_int as usize];
    (*Subcatch.offset(j as isize)).nPervPattern = -(1 as ::core::ffi::c_int);
    (*Subcatch.offset(j as isize)).dStorePattern = -(1 as ::core::ffi::c_int);
    (*Subcatch.offset(j as isize)).infilPattern = -(1 as ::core::ffi::c_int);
    if x[8 as ::core::ffi::c_int as usize] >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if snow_createSnowpack(j, crate::ccast::f64_to_i32(x[8 as ::core::ffi::c_int as usize])) == 0 {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_readSubareaParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 7] = [0.; 7];
    if ntoks < 7 as ::core::ffi::c_int {
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
    i = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[i as usize] < 0.0f64
        {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
            );
        }
        i += 1;
    }
    m = findmatch(
        *tok.offset(6 as ::core::ffi::c_int as isize),
        &raw mut RunoffRoutingWords as *mut *mut ::core::ffi::c_char,
    );
    if m < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(6 as ::core::ffi::c_int as isize),
        );
    }
    x[5 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    x[6 as ::core::ffi::c_int as usize] = 1.0f64;
    if ntoks >= 8 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(7 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(6 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[6 as ::core::ffi::c_int as usize] < 0.0f64
            || x[6 as ::core::ffi::c_int as usize] > 100.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(7 as ::core::ffi::c_int as isize),
            );
        }
        x[6 as ::core::ffi::c_int as usize] /= 100.0f64;
    }
    (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].N =
        x[0 as ::core::ffi::c_int as usize];
    (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].N =
        x[0 as ::core::ffi::c_int as usize];
    (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].N =
        x[1 as ::core::ffi::c_int as usize];
    (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].dStore = 0.0f64;
    (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].dStore =
        x[2 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].dStore =
        x[3 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].fArea =
        (*Subcatch.offset(j as isize)).fracImperv * x[4 as ::core::ffi::c_int as usize] / 100.0f64;
    (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].fArea =
        (*Subcatch.offset(j as isize)).fracImperv
            * (1.0f64 - x[4 as ::core::ffi::c_int as usize] / 100.0f64);
    (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].fArea =
        1.0f64 - (*Subcatch.offset(j as isize)).fracImperv;
    i = IMPERV0 as ::core::ffi::c_int;
    while i <= PERV as ::core::ffi::c_int {
        (*Subcatch.offset(j as isize)).subArea[i as usize].routeTo =
            TO_OUTLET as ::core::ffi::c_int;
        (*Subcatch.offset(j as isize)).subArea[i as usize].fOutlet = 1.0f64;
        i += 1;
    }
    k = crate::ccast::f64_to_i32(x[5 as ::core::ffi::c_int as usize]);
    if (*Subcatch.offset(j as isize)).fracImperv == 0.0f64
        || (*Subcatch.offset(j as isize)).fracImperv == 1.0f64
    {
        k = TO_OUTLET as ::core::ffi::c_int;
    }
    if k == TO_IMPERV as ::core::ffi::c_int && (*Subcatch.offset(j as isize)).fracImperv != 0. {
        (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].routeTo = k;
        (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].fOutlet =
            1.0f64 - x[6 as ::core::ffi::c_int as usize];
    }
    if k == TO_PERV as ::core::ffi::c_int {
        (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].routeTo = k;
        (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].routeTo = k;
        (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].fOutlet =
            1.0f64 - x[6 as ::core::ffi::c_int as usize];
        (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].fOutlet =
            1.0f64 - x[6 as ::core::ffi::c_int as usize];
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_readLanduseParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
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
    k = 2 as ::core::ffi::c_int;
    while k <= ntoks {
        m = project_findObject(
            LANDUSE as ::core::ffi::c_int,
            *tok.offset((k - 1 as ::core::ffi::c_int) as isize),
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset((k - 1 as ::core::ffi::c_int) as isize),
            );
        }
        if k + 1 as ::core::ffi::c_int > ntoks {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if getDouble(*tok.offset(k as isize), &raw mut f) == 0 {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(k as isize));
        }
        (*(*Subcatch.offset(j as isize)).landFactor.offset(m as isize)).fraction = f / 100.0f64;
        k = k + 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_readInitBuildup(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
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
    k = 2 as ::core::ffi::c_int;
    while k <= ntoks {
        m = project_findObject(
            POLLUT as ::core::ffi::c_int,
            *tok.offset((k - 1 as ::core::ffi::c_int) as isize),
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset((k - 1 as ::core::ffi::c_int) as isize),
            );
        }
        if k + 1 as ::core::ffi::c_int > ntoks {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if getDouble(*tok.offset(k as isize), &raw mut x) == 0 {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(k as isize));
        }
        *(*Subcatch.offset(j as isize))
            .initBuildup
            .offset(m as isize) = x;
        k = k + 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_validate(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut area: ::core::ffi::c_double = 0.;
    let mut nonLidArea: ::core::ffi::c_double = (*Subcatch.offset(j as isize)).area;
    if (*Subcatch.offset(j as isize)).outNode >= 0 as ::core::ffi::c_int
        && (*Subcatch.offset(j as isize)).outSubcatch >= 0 as ::core::ffi::c_int
    {
        report_writeErrorMsg(
            ERR_SUBCATCH_OUTLET as ::core::ffi::c_int,
            (*Subcatch.offset(j as isize)).ID,
        );
    }
    gwater_validate(j);
    nonLidArea -= (*Subcatch.offset(j as isize)).lidArea;
    i = IMPERV0 as ::core::ffi::c_int;
    while i <= PERV as ::core::ffi::c_int {
        if i == PERV as ::core::ffi::c_int {
            area = (1.0f64 - (*Subcatch.offset(j as isize)).fracImperv) * nonLidArea;
        } else {
            area = (*Subcatch.offset(j as isize)).fracImperv * nonLidArea;
        }
        (*Subcatch.offset(j as isize)).subArea[i as usize].alpha = 0.0f64;
        if area > 0.0f64 && (*Subcatch.offset(j as isize)).subArea[i as usize].N > 0.0f64 {
            (*Subcatch.offset(j as isize)).subArea[i as usize].alpha =
                MCOEFF * (*Subcatch.offset(j as isize)).width / area
                    * sqrt((*Subcatch.offset(j as isize)).slope)
                    / (*Subcatch.offset(j as isize)).subArea[i as usize].N;
        }
        i += 1;
    }
    i = (*Subcatch.offset(j as isize)).gage;
    if i >= 0 as ::core::ffi::c_int {
        (*Gage.offset(i as isize)).isUsed = TRUE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_initState(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    (*Subcatch.offset(j as isize)).rainfall = 0.0f64;
    (*Subcatch.offset(j as isize)).oldRunoff = 0.0f64;
    (*Subcatch.offset(j as isize)).newRunoff = 0.0f64;
    (*Subcatch.offset(j as isize)).oldSnowDepth = 0.0f64;
    (*Subcatch.offset(j as isize)).newSnowDepth = 0.0f64;
    (*Subcatch.offset(j as isize)).runon = 0.0f64;
    (*Subcatch.offset(j as isize)).evapLoss = 0.0f64;
    (*Subcatch.offset(j as isize)).infilLoss = 0.0f64;
    if (*Subcatch.offset(j as isize)).infil == j {
        infil_initState(j);
    }
    if !(*Subcatch.offset(j as isize)).groundwater.is_null() {
        gwater_initState(j);
    }
    if !(*Subcatch.offset(j as isize)).snowpack.is_null() {
        snow_initSnowpack(j);
    }
    i = IMPERV0 as ::core::ffi::c_int;
    while i <= PERV as ::core::ffi::c_int {
        (*Subcatch.offset(j as isize)).subArea[i as usize].depth = 0.0f64;
        (*Subcatch.offset(j as isize)).subArea[i as usize].inflow = 0.0f64;
        (*Subcatch.offset(j as isize)).subArea[i as usize].runoff = 0.0f64;
        i += 1;
    }
    surfqual_initState(j);
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_setOldState(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    (*Subcatch.offset(j as isize)).oldRunoff = (*Subcatch.offset(j as isize)).newRunoff;
    (*Subcatch.offset(j as isize)).oldSnowDepth = (*Subcatch.offset(j as isize)).newSnowDepth;
    i = IMPERV0 as ::core::ffi::c_int;
    while i <= PERV as ::core::ffi::c_int {
        (*Subcatch.offset(j as isize)).subArea[i as usize].inflow = 0.0f64;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        *(*Subcatch.offset(j as isize)).oldQual.offset(i as isize) =
            *(*Subcatch.offset(j as isize)).newQual.offset(i as isize);
        *(*Subcatch.offset(j as isize)).newQual.offset(i as isize) = 0.0f64;
        i += 1;
    }
    lid_setOldGroupState(j);
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_getFracPerv(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut fracPerv: ::core::ffi::c_double = 1.0f64 - (*Subcatch.offset(j as isize)).fracImperv;
    if (*Subcatch.offset(j as isize)).lidArea > 0.0f64 {
        fracPerv = (fracPerv
            * ((*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea)
            + lid_getPervArea(j))
            / (*Subcatch.offset(j as isize)).area;
        fracPerv = if fracPerv <= 1.0f64 { fracPerv } else { 1.0f64 };
    }
    return fracPerv;
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_getStorage(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_double = 0.0f64;
    i = IMPERV0 as ::core::ffi::c_int;
    while i <= PERV as ::core::ffi::c_int {
        v += (*Subcatch.offset(j as isize)).subArea[i as usize].depth
            * (*Subcatch.offset(j as isize)).subArea[i as usize].fArea;
        i += 1;
    }
    return v * ((*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea)
        + lid_getStoredVolume(j);
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_getRunon(mut j: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut q1: ::core::ffi::c_double = 0.;
    let mut q2: ::core::ffi::c_double = 0.;
    let mut pervArea: ::core::ffi::c_double = 0.;
    k = (*Subcatch.offset(j as isize)).outSubcatch;
    q = (*Subcatch.offset(j as isize)).oldRunoff;
    if k >= 0 as ::core::ffi::c_int && k != j {
        subcatch_addRunonFlow(k, q);
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *(*Subcatch.offset(k as isize)).newQual.offset(p as isize) +=
                q * *(*Subcatch.offset(j as isize)).oldQual.offset(p as isize) * LperFT3;
            p += 1;
        }
    }
    if (*Subcatch.offset(j as isize)).lidArea > 0.0f64 {
        lid_addDrainRunon(j);
    }
    if (*Subcatch.offset(j as isize)).fracImperv < 1.0f64
        && (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].routeTo
            == TO_PERV as ::core::ffi::c_int
    {
        q1 = (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].runoff
            * (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].fArea;
        q2 = (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].runoff
            * (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].fArea;
        q = q1 + q2;
        (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].inflow += q
            * (1.0f64
                - (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize]
                    .fOutlet)
            / (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].fArea;
    }
    if (*Subcatch.offset(j as isize)).fracImperv > 0.0f64
        && (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].routeTo
            == TO_IMPERV as ::core::ffi::c_int
        && (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].fArea
            > 0.0f64
    {
        q = (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].runoff;
        (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].inflow += q
            * (1.0f64
                - (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize]
                    .fOutlet)
            * (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].fArea
            / (*Subcatch.offset(j as isize)).subArea[IMPERV1 as ::core::ffi::c_int as usize].fArea;
    }
    if (*Subcatch.offset(j as isize)).lidArea > 0.0f64
        && (*Subcatch.offset(j as isize)).fracImperv < 1.0f64
    {
        pervArea = (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize]
            .fArea
            * ((*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea);
        q = lid_getFlowToPerv(j);
        if pervArea > 0.0f64 {
            (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].inflow +=
                q / pervArea;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_addRunonFlow(
    mut k: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut nonLidArea: ::core::ffi::c_double = 0.;
    if (*Subcatch.offset(k as isize)).area <= 0.0f64 {
        return;
    }
    nonLidArea = (*Subcatch.offset(k as isize)).area - (*Subcatch.offset(k as isize)).lidArea;
    if nonLidArea > 0.0f64 {
        q = q / nonLidArea;
    } else {
        q = q / (*Subcatch.offset(k as isize)).area;
    }
    (*Subcatch.offset(k as isize)).runon += q;
    i = IMPERV0 as ::core::ffi::c_int;
    while i <= PERV as ::core::ffi::c_int {
        (*Subcatch.offset(k as isize)).subArea[i as usize].inflow += q;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_getRunoff(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut nonLidArea: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = 0.;
    let mut netPrecip: [::core::ffi::c_double; 3] = [0.; 3];
    let mut vRain: ::core::ffi::c_double = 0.;
    let mut vRunon: ::core::ffi::c_double = 0.0f64;
    let mut vOutflow: ::core::ffi::c_double = 0.0f64;
    let mut runoff: ::core::ffi::c_double = 0.0f64;
    let mut evapRate: ::core::ffi::c_double = 0.0f64;
    let mut subAreaRunoff: ::core::ffi::c_double = 0.;
    let mut vImpervRunoff: ::core::ffi::c_double = 0.0f64;
    let mut vPervRunoff: ::core::ffi::c_double = 0.0f64;
    Vevap = 0.0f64;
    Vpevap = 0.0f64;
    Vinfil = 0.0f64;
    Voutflow = 0.0f64;
    VlidIn = 0.0f64;
    VlidInfil = 0.0f64;
    VlidOut = 0.0f64;
    VlidDrain = 0.0f64;
    VlidReturn = 0.0f64;
    nonLidArea = (*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea;
    vRunon = (*Subcatch.offset(j as isize)).runon * tStep * nonLidArea;
    Vinflow = vRunon + subcatch_getDepth(j) * nonLidArea;
    if nonLidArea == 0.0f64 {
        vRunon = (*Subcatch.offset(j as isize)).runon * tStep * (*Subcatch.offset(j as isize)).area;
    }
    getNetPrecip(j, &raw mut netPrecip as *mut ::core::ffi::c_double, tStep);
    if Evap.dryOnly != 0 && (*Subcatch.offset(j as isize)).rainfall > 0.0f64 {
        evapRate = 0.0f64;
    } else {
        evapRate = Evap.rate;
    }
    infil_setInfilFactor(j);
    if nonLidArea > 0.0f64 {
        i = IMPERV0 as ::core::ffi::c_int;
        while i <= PERV as ::core::ffi::c_int {
            area = nonLidArea * (*Subcatch.offset(j as isize)).subArea[i as usize].fArea;
            (*Subcatch.offset(j as isize)).subArea[i as usize].runoff =
                getSubareaRunoff(j, i, area, netPrecip[i as usize], evapRate, tStep);
            subAreaRunoff = (*Subcatch.offset(j as isize)).subArea[i as usize].runoff * area;
            if i == PERV as ::core::ffi::c_int {
                vPervRunoff = subAreaRunoff * tStep;
            } else {
                vImpervRunoff += subAreaRunoff * tStep;
            }
            runoff += subAreaRunoff;
            i += 1;
        }
    }
    if (*Subcatch.offset(j as isize)).lidArea > 0.0f64 {
        lid_getRunoff(j, tStep);
    }
    if IgnoreGwater == 0 && !(*Subcatch.offset(j as isize)).groundwater.is_null() {
        gwater_getGroundwater(j, Vpevap, Vinfil + VlidInfil, tStep);
    }
    area = (*Subcatch.offset(j as isize)).area;
    (*Subcatch.offset(j as isize)).evapLoss = Vevap / tStep / area;
    (*Subcatch.offset(j as isize)).infilLoss = (Vinfil + VlidInfil) / tStep / area;
    vOutflow = Voutflow - VlidIn + VlidOut;
    (*Subcatch.offset(j as isize)).newRunoff = vOutflow / tStep;
    vRain = (*Subcatch.offset(j as isize)).rainfall * tStep * area;
    stats_updateSubcatchStats(
        j,
        vRain,
        vRunon,
        Vevap,
        Vinfil + VlidInfil,
        vImpervRunoff,
        vPervRunoff,
        vOutflow + VlidDrain,
        (*Subcatch.offset(j as isize)).newRunoff + VlidDrain / tStep,
    );
    if (*Subcatch.offset(j as isize)).outNode == -(1 as ::core::ffi::c_int)
        && (*Subcatch.offset(j as isize)).outSubcatch != j
    {
        vOutflow = 0.0f64;
    }
    massbal_updateRunoffTotals(RUNOFF_RAINFALL as ::core::ffi::c_int, vRain);
    massbal_updateRunoffTotals(RUNOFF_EVAP as ::core::ffi::c_int, Vevap);
    massbal_updateRunoffTotals(RUNOFF_INFIL as ::core::ffi::c_int, Vinfil + VlidInfil);
    massbal_updateRunoffTotals(RUNOFF_RUNOFF as ::core::ffi::c_int, vOutflow);
    return runoff / area;
}
unsafe extern "C" fn getNetPrecip(
    mut j: ::core::ffi::c_int,
    mut netPrecip: *mut ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut rainfall: ::core::ffi::c_double = 0.0f64;
    let mut snowfall: ::core::ffi::c_double = 0.0f64;
    k = (*Subcatch.offset(j as isize)).gage;
    if k >= 0 as ::core::ffi::c_int {
        gage_getPrecip(k, &raw mut rainfall, &raw mut snowfall);
    }
    (*Subcatch.offset(j as isize)).rainfall = rainfall + snowfall;
    if !(*Subcatch.offset(j as isize)).snowpack.is_null() && IgnoreSnowmelt == 0 {
        (*Subcatch.offset(j as isize)).newSnowDepth = snow_getSnowMelt(
            j,
            rainfall,
            snowfall,
            tStep,
            netPrecip as *mut ::core::ffi::c_double,
        );
    } else {
        i = IMPERV0 as ::core::ffi::c_int;
        while i <= PERV as ::core::ffi::c_int {
            *netPrecip.offset(i as isize) = rainfall + snowfall;
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_getDepth(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut fArea: ::core::ffi::c_double = 0.;
    let mut depth: ::core::ffi::c_double = 0.0f64;
    i = IMPERV0 as ::core::ffi::c_int;
    while i <= PERV as ::core::ffi::c_int {
        fArea = (*Subcatch.offset(j as isize)).subArea[i as usize].fArea;
        if fArea > 0.0f64 {
            depth += (*Subcatch.offset(j as isize)).subArea[i as usize].depth * fArea;
        }
        i += 1;
    }
    return depth;
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_getWtdOutflow(
    mut j: ::core::ffi::c_int,
    mut f: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if (*Subcatch.offset(j as isize)).area == 0.0f64 {
        return 0.0f64;
    }
    return (1.0f64 - f) * (*Subcatch.offset(j as isize)).oldRunoff
        + f * (*Subcatch.offset(j as isize)).newRunoff;
}
#[no_mangle]
pub unsafe extern "C" fn subcatch_getResults(
    mut j: ::core::ffi::c_int,
    mut f: ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_float,
) {
    let mut p: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut f1: ::core::ffi::c_double = 1.0f64 - f;
    let mut z: ::core::ffi::c_double = 0.;
    let mut runoff: ::core::ffi::c_double = 0.;
    let mut gw: *mut TGroundwater = ::core::ptr::null_mut::<TGroundwater>();
    k = (*Subcatch.offset(j as isize)).gage;
    if k >= 0 as ::core::ffi::c_int {
        *x.offset(SUBCATCH_RAINFALL as ::core::ffi::c_int as isize) =
            (*Gage.offset(k as isize)).reportRainfall as ::core::ffi::c_float;
    } else {
        *x.offset(SUBCATCH_RAINFALL as ::core::ffi::c_int as isize) = 0.0f32;
    }
    z = (f1 * (*Subcatch.offset(j as isize)).oldSnowDepth
        + f * (*Subcatch.offset(j as isize)).newSnowDepth)
        * UCF(RAINDEPTH as ::core::ffi::c_int);
    *x.offset(SUBCATCH_SNOWDEPTH as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
    *x.offset(SUBCATCH_EVAP as ::core::ffi::c_int as isize) =
        ((*Subcatch.offset(j as isize)).evapLoss * UCF(EVAPRATE as ::core::ffi::c_int))
            as ::core::ffi::c_float;
    *x.offset(SUBCATCH_INFIL as ::core::ffi::c_int as isize) =
        ((*Subcatch.offset(j as isize)).infilLoss * UCF(RAINFALL as ::core::ffi::c_int))
            as ::core::ffi::c_float;
    runoff = f1 * (*Subcatch.offset(j as isize)).oldRunoff
        + f * (*Subcatch.offset(j as isize)).newRunoff;
    if (*Subcatch.offset(j as isize)).lidArea > 0.0f64 {
        runoff += f1 * lid_getDrainFlow(j, PREVIOUS as ::core::ffi::c_int)
            + f * lid_getDrainFlow(j, CURRENT as ::core::ffi::c_int);
    }
    if runoff < MIN_RUNOFF * (*Subcatch.offset(j as isize)).area {
        runoff = 0.0f64;
    }
    *x.offset(SUBCATCH_RUNOFF as ::core::ffi::c_int as isize) =
        (runoff * UCF(FLOW as ::core::ffi::c_int)) as ::core::ffi::c_float;
    gw = (*Subcatch.offset(j as isize)).groundwater;
    if !gw.is_null() {
        z = (f1 * (*gw).oldFlow + f * (*gw).newFlow)
            * (*Subcatch.offset(j as isize)).area
            * UCF(FLOW as ::core::ffi::c_int);
        *x.offset(SUBCATCH_GW_FLOW as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
        z = ((*gw).bottomElev + (*gw).lowerDepth) * UCF(LENGTH as ::core::ffi::c_int);
        *x.offset(SUBCATCH_GW_ELEV as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
        z = (*gw).theta;
        *x.offset(SUBCATCH_SOIL_MOIST as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
    } else {
        *x.offset(SUBCATCH_GW_FLOW as ::core::ffi::c_int as isize) = 0.0f32;
        *x.offset(SUBCATCH_GW_ELEV as ::core::ffi::c_int as isize) = 0.0f32;
        *x.offset(SUBCATCH_SOIL_MOIST as ::core::ffi::c_int as isize) = 0.0f32;
    }
    if IgnoreQuality == 0 {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            if runoff == 0.0f64 {
                z = 0.0f64;
            } else {
                z = f1 * *(*Subcatch.offset(j as isize)).oldQual.offset(p as isize)
                    + f * *(*Subcatch.offset(j as isize)).newQual.offset(p as isize);
            }
            *x.offset((SUBCATCH_WASHOFF as ::core::ffi::c_int + p) as isize) =
                z as ::core::ffi::c_float;
            p += 1;
        }
    }
}
unsafe extern "C" fn getSubareaRunoff(
    mut j: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
    mut area: ::core::ffi::c_double,
    mut precip: ::core::ffi::c_double,
    mut evap: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut tRunoff: ::core::ffi::c_double = 0.;
    let mut surfMoisture: ::core::ffi::c_double = 0.;
    let mut surfEvap: ::core::ffi::c_double = 0.;
    let mut infil: ::core::ffi::c_double = 0.0f64;
    let mut runoff: ::core::ffi::c_double = 0.0f64;
    let mut subarea: *mut TSubarea = ::core::ptr::null_mut::<TSubarea>();
    if area == 0.0f64 {
        return 0.0f64;
    }
    subarea = (&raw mut (*Subcatch.offset(j as isize)).subArea as *mut TSubarea).offset(i as isize)
        as *mut TSubarea;
    tRunoff = tStep;
    surfMoisture = (*subarea).depth / tStep;
    surfEvap = if surfMoisture <= evap {
        surfMoisture
    } else {
        evap
    };
    if i == PERV as ::core::ffi::c_int {
        infil = getSubareaInfil(j, subarea, precip, tStep);
    }
    (*subarea).inflow += precip;
    surfMoisture += (*subarea).inflow;
    Vinflow += precip * area * tStep;
    Vevap += surfEvap * area * tStep;
    if i == PERV as ::core::ffi::c_int {
        Vpevap += Vevap;
    }
    Vinfil += infil * area * tStep;
    Alpha = (*subarea).alpha;
    Dstore = (*subarea).dStore;
    adjustSubareaParams(i, j);
    if surfEvap + infil >= surfMoisture {
        (*subarea).depth = 0.0f64;
    } else {
        (*subarea).inflow -= surfEvap + infil;
        updatePondedDepth(subarea, &raw mut tRunoff);
    }
    runoff = findSubareaRunoff(subarea, tRunoff);
    Voutflow += (*subarea).fOutlet * runoff * area * tStep;
    return runoff;
}
unsafe extern "C" fn getSubareaInfil(
    mut j: ::core::ffi::c_int,
    mut subarea: *mut TSubarea,
    mut precip: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut infil: ::core::ffi::c_double = 0.0f64;
    infil = infil_getInfil(j, tStep, precip, (*subarea).inflow, (*subarea).depth);
    if IgnoreGwater == 0 && !(*Subcatch.offset(j as isize)).groundwater.is_null() {
        infil = if infil <= (*(*Subcatch.offset(j as isize)).groundwater).maxInfilVol / tStep {
            infil
        } else {
            (*(*Subcatch.offset(j as isize)).groundwater).maxInfilVol / tStep
        };
    }
    return infil;
}
unsafe extern "C" fn findSubareaRunoff(
    mut subarea: *mut TSubarea,
    mut tRunoff: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut xDepth: ::core::ffi::c_double = (*subarea).depth - Dstore;
    let mut runoff: ::core::ffi::c_double = 0.0f64;
    if xDepth > ZERO {
        if (*subarea).N > 0.0f64 {
            runoff = Alpha * pow(xDepth, MEXP);
        } else {
            runoff = xDepth / tRunoff;
            (*subarea).depth = Dstore;
        }
    } else {
        runoff = 0.0f64;
    }
    return runoff;
}
unsafe extern "C" fn updatePondedDepth(
    mut subarea: *mut TSubarea,
    mut dt: *mut ::core::ffi::c_double,
) {
    let mut ix: ::core::ffi::c_double = (*subarea).inflow;
    let mut dx: ::core::ffi::c_double = 0.;
    let mut tx: ::core::ffi::c_double = *dt;
    if (*subarea).depth + ix * tx <= Dstore {
        (*subarea).depth += ix * tx;
    } else {
        dx = Dstore - (*subarea).depth;
        if dx > 0.0f64 && ix > 0.0f64 {
            tx -= dx / ix;
            (*subarea).depth = Dstore;
        }
        if Alpha > 0.0f64 && tx > 0.0f64 {
            theSubarea = subarea;
            odesolve_integrate(
                &raw mut (*subarea).depth,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                tx,
                ODETOL,
                tx,
                Some(
                    getDdDt
                        as unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            *mut ::core::ffi::c_double,
                            *mut ::core::ffi::c_double,
                        ) -> (),
                ),
            );
        } else {
            if tx < 0.0f64 {
                tx = 0.0f64;
            }
            (*subarea).depth += ix * tx;
        }
    }
    if (*subarea).depth < 0.0f64 {
        (*subarea).depth = 0.0f64;
    }
    *dt = tx;
}
unsafe extern "C" fn getDdDt(
    mut t: ::core::ffi::c_double,
    mut d: *mut ::core::ffi::c_double,
    mut dddt: *mut ::core::ffi::c_double,
) {
    let mut ix: ::core::ffi::c_double = (*theSubarea).inflow;
    let mut rx: ::core::ffi::c_double = *d - Dstore;
    if rx < 0.0f64 {
        rx = 0.0f64;
    } else {
        rx = Alpha * pow(rx, MEXP);
    }
    *dddt = ix - rx;
}
unsafe extern "C" fn adjustSubareaParams(mut i: ::core::ffi::c_int, mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    if i == PERV as ::core::ffi::c_int {
        p = (*Subcatch.offset(j as isize)).dStorePattern;
        if p >= 0 as ::core::ffi::c_int
            && (*Pattern.offset(p as isize)).type_0 == MONTHLY_PATTERN as ::core::ffi::c_int
        {
            m = datetime_monthOfYear(getDateTime(OldRunoffTime)) - 1 as ::core::ffi::c_int;
            f = (*Pattern.offset(p as isize)).factor[m as usize];
            if f >= 0.0f64 {
                Dstore *= f;
            }
        }
        p = (*Subcatch.offset(j as isize)).nPervPattern;
        if p >= 0 as ::core::ffi::c_int
            && (*Pattern.offset(p as isize)).type_0 == MONTHLY_PATTERN as ::core::ffi::c_int
        {
            m = datetime_monthOfYear(getDateTime(OldRunoffTime)) - 1 as ::core::ffi::c_int;
            f = (*Pattern.offset(p as isize)).factor[m as usize];
            if f <= 0.0f64 {
                Alpha = 0.0f64;
            } else {
                Alpha /= f;
            }
        }
    }
}
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ZERO: ::core::ffi::c_double = 1.0E-10f64;
pub const MIN_RUNOFF: ::core::ffi::c_double = 2.31481e-8f64;
pub const LperFT3: ::core::ffi::c_double = 28.317f64;
