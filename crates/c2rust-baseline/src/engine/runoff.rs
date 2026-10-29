#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn datetime_dayOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_timeDiff(date1: DateTime, date2: DateTime) -> ::core::ffi::c_long;
    static mut Frunoff: TFile;
    static mut Fclimate: TFile;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut SweepStart: ::core::ffi::c_int;
    static mut Nnodes: [::core::ffi::c_int; 5];
    static mut FlowUnits: ::core::ffi::c_int;
    static mut IgnoreSnowmelt: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut ErrorCode: ::core::ffi::c_int;
    static mut WetStep: ::core::ffi::c_int;
    static mut DryStep: ::core::ffi::c_int;
    static mut SweepEnd: ::core::ffi::c_int;
    static mut OldRunoffTime: ::core::ffi::c_double;
    static mut NewRunoffTime: ::core::ffi::c_double;
    static mut TotalDuration: ::core::ffi::c_double;
    static mut Gage: *mut TGage;
    static mut Subcatch: *mut TSubcatch;
    static mut Aquifer: *mut TAquifer;
    static mut Outfall: *mut TOutfall;
    static mut Pollut: *mut TPollut;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn climate_setState(aDate: DateTime);
    fn climate_getNextEvapDate() -> DateTime;
    fn snow_plowSnow(subcatch: ::core::ffi::c_int, tStep: ::core::ffi::c_double);
    fn massbal_updateRunoffTotals(type_0: ::core::ffi::c_int, v: ::core::ffi::c_double);
    fn massbal_updateLoadingTotals(
        type_0: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
        w: ::core::ffi::c_double,
    );
    fn stats_updateMaxRunoff();
    fn gage_setState(gage: ::core::ffi::c_int, aDate: DateTime);
    fn gage_getNextRainDate(gage: ::core::ffi::c_int, aDate: DateTime) -> DateTime;
    fn gage_updatePastRain(j: ::core::ffi::c_int, tStep: ::core::ffi::c_int);
    fn subcatch_setOldState(subcatch: ::core::ffi::c_int);
    fn subcatch_getRunon(subcatch: ::core::ffi::c_int);
    fn subcatch_addRunonFlow(subcatch: ::core::ffi::c_int, flow: ::core::ffi::c_double);
    fn subcatch_getRunoff(
        subcatch: ::core::ffi::c_int,
        tStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn subcatch_getResults(
        subcatch: ::core::ffi::c_int,
        wt: ::core::ffi::c_double,
        x: *mut ::core::ffi::c_float,
    );
    fn surfqual_getWashoff(
        subcatch: ::core::ffi::c_int,
        runoff: ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
    );
    fn surfqual_getBuildup(subcatch: ::core::ffi::c_int, tStep: ::core::ffi::c_double);
    fn surfqual_sweepBuildup(subcatch: ::core::ffi::c_int, aDate: DateTime);
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
    fn odesolve_open(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn odesolve_close();
    static mut SubcatchResults: *mut ::core::ffi::c_float;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type FileUsageType = ::core::ffi::c_uint;
pub const SAVE_FILE: FileUsageType = 3;
pub const USE_FILE: FileUsageType = 2;
pub const SCRATCH_FILE: FileUsageType = 1;
pub const NO_FILE: FileUsageType = 0;
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
pub type LoadingType = ::core::ffi::c_uint;
pub const FINAL_LOAD: LoadingType = 6;
pub const RUNOFF_LOAD: LoadingType = 5;
pub const INFIL_LOAD: LoadingType = 4;
pub const BMP_REMOVAL_LOAD: LoadingType = 3;
pub const SWEEPING_LOAD: LoadingType = 2;
pub const DEPOSITION_LOAD: LoadingType = 1;
pub const BUILDUP_LOAD: LoadingType = 0;
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
pub struct TFile {
    pub name: [::core::ffi::c_char; 260],
    pub mode: ::core::ffi::c_char,
    pub state: ::core::ffi::c_char,
    pub file: *mut FILE,
}
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
pub struct TOutfall {
    pub type_0: ::core::ffi::c_int,
    pub hasFlapGate: ::core::ffi::c_char,
    pub fixedStage: ::core::ffi::c_double,
    pub tideCurve: ::core::ffi::c_int,
    pub stageSeries: ::core::ffi::c_int,
    pub routeTo: ::core::ffi::c_int,
    pub vRouted: ::core::ffi::c_double,
    pub wRouted: *mut ::core::ffi::c_double,
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
pub const ERR_RUNOFF_FILE_FORMAT: ErrorType = 325;
pub const ERR_RUNOFF_FILE_OPEN: ErrorType = 323;
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_ODE_SOLVER: ErrorType = 105;
pub const ERR_TIMESTEP: ErrorType = 107;
pub const ERR_RUNOFF_FILE_READ: ErrorType = 329;
pub const ERR_RUNOFF_FILE_END: ErrorType = 327;
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
pub const ERR_KINWAVE: ErrorType = 103;
pub const ERR_NONE: ErrorType = 0;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAXODES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MIN_RUNOFF: ::core::ffi::c_double = 2.31481e-8f64;
pub const LperFT3: ::core::ffi::c_double = 28.317f64;
pub const MAX_SUBCATCH_RESULTS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
static mut IsRaining: ::core::ffi::c_char = 0;
static mut HasRunoff: ::core::ffi::c_char = 0;
static mut HasSnow: ::core::ffi::c_char = 0;
static mut Nsteps: ::core::ffi::c_int = 0;
static mut MaxSteps: ::core::ffi::c_int = 0;
static mut MaxStepsPos: ::core::ffi::c_long = 0;
#[no_mangle]
pub static mut HasWetLids: ::core::ffi::c_char = 0;
#[no_mangle]
pub static mut OutflowLoad: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn runoff_open() -> ::core::ffi::c_int {
    IsRaining = FALSE as ::core::ffi::c_char;
    HasRunoff = FALSE as ::core::ffi::c_char;
    HasSnow = FALSE as ::core::ffi::c_char;
    Nsteps = 0 as ::core::ffi::c_int;
    if odesolve_open(MAXODES) == 0 {
        report_writeErrorMsg(
            ERR_ODE_SOLVER as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    OutflowLoad = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        OutflowLoad = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        if OutflowLoad.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    match Frunoff.mode as ::core::ffi::c_int {
        2 => {
            Frunoff.file = fopen(
                &raw mut Frunoff.name as *mut ::core::ffi::c_char,
                b"r+b\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if Frunoff.file.is_null() {
                report_writeErrorMsg(
                    ERR_RUNOFF_FILE_OPEN as ::core::ffi::c_int,
                    &raw mut Frunoff.name as *mut ::core::ffi::c_char,
                );
            } else {
                runoff_initFile();
            }
        }
        3 => {
            Frunoff.file = fopen(
                &raw mut Frunoff.name as *mut ::core::ffi::c_char,
                b"w+b\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if Frunoff.file.is_null() {
                report_writeErrorMsg(
                    ERR_RUNOFF_FILE_OPEN as ::core::ffi::c_int,
                    &raw mut Frunoff.name as *mut ::core::ffi::c_char,
                );
            } else {
                runoff_initFile();
            }
        }
        _ => {}
    }
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn runoff_close() {
    odesolve_close();
    if !OutflowLoad.is_null() {
        free(OutflowLoad as *mut ::core::ffi::c_void);
        OutflowLoad = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !Frunoff.file.is_null() {
        if Frunoff.mode as ::core::ffi::c_int == SAVE_FILE as ::core::ffi::c_int {
            fseek(Frunoff.file, MaxStepsPos, SEEK_SET);
            fwrite(
                &raw mut Nsteps as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                1 as size_t,
                Frunoff.file,
            );
        }
        fclose(Frunoff.file);
    }
    if !Fclimate.file.is_null() {
        fclose(Fclimate.file);
    }
}
#[no_mangle]
pub unsafe extern "C" fn runoff_execute() {
    let mut j: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut runoffStep: ::core::ffi::c_double = 0.;
    let mut oldRunoffStep: ::core::ffi::c_double = 0.;
    let mut runoff: ::core::ffi::c_double = 0.;
    let mut currentDate: DateTime = 0.;
    let mut canSweep: ::core::ffi::c_char = 0;
    if ErrorCode != 0 {
        return;
    }
    oldRunoffStep = (NewRunoffTime - OldRunoffTime) / 1000.0f64;
    currentDate = getDateTime(NewRunoffTime);
    climate_setState(currentDate);
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        OldRunoffTime = NewRunoffTime;
        NewRunoffTime += DryStep as ::core::ffi::c_double * 1000.0f64;
        NewRunoffTime = if NewRunoffTime <= TotalDuration {
            NewRunoffTime
        } else {
            TotalDuration
        };
        return;
    }
    IsRaining = FALSE as ::core::ffi::c_char;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        gage_setState(j, currentDate);
        if (*Gage.offset(j as isize)).rainfall > 0.0f64 {
            IsRaining = TRUE as ::core::ffi::c_char;
        }
        j += 1;
    }
    if Frunoff.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        runoff_readFromFile();
        return;
    }
    day = datetime_dayOfYear(currentDate);
    canSweep = FALSE as ::core::ffi::c_char;
    if SweepStart <= SweepEnd {
        if day >= SweepStart && day <= SweepEnd {
            canSweep = TRUE as ::core::ffi::c_char;
        }
    } else if day <= SweepEnd || day >= SweepStart {
        canSweep = TRUE as ::core::ffi::c_char;
    }
    runoffStep = runoff_getTimeStep(currentDate);
    if runoffStep <= 0.0f64 {
        ErrorCode = ERR_TIMESTEP as ::core::ffi::c_int;
        return;
    }
    OldRunoffTime = NewRunoffTime;
    NewRunoffTime += 1000 as ::core::ffi::c_int as ::core::ffi::c_double * runoffStep;
    if NewRunoffTime > TotalDuration {
        runoffStep = (TotalDuration - OldRunoffTime) / 1000.0f64;
        NewRunoffTime = TotalDuration;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        gage_updatePastRain(j, crate::ccast::f64_to_i32(runoffStep));
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        subcatch_setOldState(j);
        j += 1;
    }
    if oldRunoffStep > 0.0f64 {
        runoff_getOutfallRunon(oldRunoffStep);
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        if !((*Subcatch.offset(j as isize)).area == 0.0f64) {
            subcatch_getRunon(j);
            if IgnoreSnowmelt == 0 {
                snow_plowSnow(j, runoffStep);
            }
        }
        j += 1;
    }
    HasSnow = FALSE as ::core::ffi::c_char;
    HasRunoff = FALSE as ::core::ffi::c_char;
    HasWetLids = FALSE as ::core::ffi::c_char;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        if !((*Subcatch.offset(j as isize)).area == 0.0f64) {
            runoff = subcatch_getRunoff(j, runoffStep);
            if runoff > 0.0f64 {
                HasRunoff = TRUE as ::core::ffi::c_char;
            }
            if (*Subcatch.offset(j as isize)).newSnowDepth > 0.0f64 {
                HasSnow = TRUE as ::core::ffi::c_char;
            }
            if !(IgnoreQuality != 0) {
                if runoff < MIN_RUNOFF {
                    surfqual_getBuildup(j, runoffStep);
                }
                if canSweep as ::core::ffi::c_int != 0
                    && (*Subcatch.offset(j as isize)).rainfall <= MIN_RUNOFF
                {
                    surfqual_sweepBuildup(j, currentDate);
                }
                surfqual_getWashoff(j, runoff, runoffStep);
            }
        }
        j += 1;
    }
    stats_updateMaxRunoff();
    Nsteps += 1;
    if Frunoff.mode as ::core::ffi::c_int == SAVE_FILE as ::core::ffi::c_int {
        runoff_saveToFile(runoffStep as ::core::ffi::c_float);
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        (*Subcatch.offset(j as isize)).runon = 0.0f64;
        j += 1;
    }
}
unsafe extern "C" fn runoff_getTimeStep(mut currentDate: DateTime) -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = 0;
    let mut timeStep: ::core::ffi::c_long = 0;
    let mut maxStep: ::core::ffi::c_long = DryStep as ::core::ffi::c_long;
    timeStep = datetime_timeDiff(climate_getNextEvapDate(), currentDate);
    if timeStep as ::core::ffi::c_double > 0.0f64 && timeStep < maxStep {
        maxStep = timeStep;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        timeStep = datetime_timeDiff(gage_getNextRainDate(j, currentDate), currentDate);
        if timeStep > 0 as ::core::ffi::c_long && timeStep < maxStep {
            maxStep = timeStep;
        }
        j += 1;
    }
    if IsRaining as ::core::ffi::c_int != 0
        || HasSnow as ::core::ffi::c_int != 0
        || HasRunoff as ::core::ffi::c_int != 0
        || HasWetLids as ::core::ffi::c_int != 0
    {
        timeStep = WetStep as ::core::ffi::c_long;
    } else {
        timeStep = DryStep as ::core::ffi::c_long;
    }
    if timeStep > maxStep {
        timeStep = maxStep;
    }
    return timeStep as ::core::ffi::c_double;
}
unsafe extern "C" fn runoff_initFile() {
    let mut nSubcatch: ::core::ffi::c_int = 0;
    let mut nPollut: ::core::ffi::c_int = 0;
    let mut flowUnits: ::core::ffi::c_int = 0;
    let mut fileStamp: [::core::ffi::c_char; 13] =
        ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"SWMM5-RUNOFF\0");
    let mut fStamp: [::core::ffi::c_char; 13] =
        ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"SWMM5-RUNOFF\0");
    MaxSteps = 0 as ::core::ffi::c_int;
    if Frunoff.mode as ::core::ffi::c_int == SAVE_FILE as ::core::ffi::c_int {
        nSubcatch = Nobjects[SUBCATCH as ::core::ffi::c_int as usize];
        nPollut = Nobjects[POLLUT as ::core::ffi::c_int as usize];
        flowUnits = FlowUnits;
        fwrite(
            &raw mut fileStamp as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            strlen(&raw mut fileStamp as *mut ::core::ffi::c_char),
            Frunoff.file,
        );
        fwrite(
            &raw mut nSubcatch as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frunoff.file,
        );
        fwrite(
            &raw mut nPollut as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frunoff.file,
        );
        fwrite(
            &raw mut flowUnits as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frunoff.file,
        );
        MaxStepsPos = ftell(Frunoff.file);
        fwrite(
            &raw mut MaxSteps as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frunoff.file,
        );
    }
    if Frunoff.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        fread(
            &raw mut fStamp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            strlen(&raw mut fileStamp as *mut ::core::ffi::c_char),
            Frunoff.file,
        );
        if strcmp(
            &raw mut fStamp as *mut ::core::ffi::c_char,
            &raw mut fileStamp as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            report_writeErrorMsg(
                ERR_RUNOFF_FILE_FORMAT as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return;
        }
        nSubcatch = -(1 as ::core::ffi::c_int);
        nPollut = -(1 as ::core::ffi::c_int);
        flowUnits = -(1 as ::core::ffi::c_int);
        fread(
            &raw mut nSubcatch as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frunoff.file,
        );
        fread(
            &raw mut nPollut as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frunoff.file,
        );
        fread(
            &raw mut flowUnits as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frunoff.file,
        );
        fread(
            &raw mut MaxSteps as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frunoff.file,
        );
        if nSubcatch != Nobjects[SUBCATCH as ::core::ffi::c_int as usize]
            || nPollut != Nobjects[POLLUT as ::core::ffi::c_int as usize]
            || flowUnits != FlowUnits
            || MaxSteps <= 0 as ::core::ffi::c_int
        {
            report_writeErrorMsg(
                ERR_RUNOFF_FILE_FORMAT as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn runoff_saveToFile(mut tStep: ::core::ffi::c_float) {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = MAX_SUBCATCH_RESULTS
        + Nobjects[POLLUT as ::core::ffi::c_int as usize]
        - 1 as ::core::ffi::c_int;
    fwrite(
        &raw mut tStep as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        1 as size_t,
        Frunoff.file,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        subcatch_getResults(j, 1.0f64, SubcatchResults as *mut ::core::ffi::c_float);
        fwrite(
            SubcatchResults as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            n as size_t,
            Frunoff.file,
        );
        j += 1;
    }
}
unsafe extern "C" fn runoff_readFromFile() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nResults: ::core::ffi::c_int = 0;
    let mut kount: ::core::ffi::c_int = 0;
    let mut tStep: ::core::ffi::c_float = 0.;
    let mut gw: *mut TGroundwater = ::core::ptr::null_mut::<TGroundwater>();
    if Nsteps > MaxSteps {
        report_writeErrorMsg(
            ERR_RUNOFF_FILE_END as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        subcatch_setOldState(j);
        j += 1;
    }
    kount = 0 as ::core::ffi::c_int;
    kount += fread(
        &raw mut tStep as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        1 as size_t,
        Frunoff.file,
    ) as ::core::ffi::c_int;
    nResults = MAX_SUBCATCH_RESULTS + Nobjects[POLLUT as ::core::ffi::c_int as usize]
        - 1 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        kount += fread(
            SubcatchResults as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            nResults as size_t,
            Frunoff.file,
        ) as ::core::ffi::c_int;
        (*Subcatch.offset(j as isize)).newSnowDepth = *SubcatchResults
            .offset(SUBCATCH_SNOWDEPTH as ::core::ffi::c_int as isize)
            as ::core::ffi::c_double
            / UCF(RAINDEPTH as ::core::ffi::c_int);
        (*Subcatch.offset(j as isize)).evapLoss = *SubcatchResults
            .offset(SUBCATCH_EVAP as ::core::ffi::c_int as isize)
            as ::core::ffi::c_double
            / UCF(RAINFALL as ::core::ffi::c_int);
        (*Subcatch.offset(j as isize)).infilLoss = *SubcatchResults
            .offset(SUBCATCH_INFIL as ::core::ffi::c_int as isize)
            as ::core::ffi::c_double
            / UCF(RAINFALL as ::core::ffi::c_int);
        (*Subcatch.offset(j as isize)).newRunoff = *SubcatchResults
            .offset(SUBCATCH_RUNOFF as ::core::ffi::c_int as isize)
            as ::core::ffi::c_double
            / UCF(FLOW as ::core::ffi::c_int);
        gw = (*Subcatch.offset(j as isize)).groundwater;
        if !gw.is_null() {
            (*gw).newFlow = *SubcatchResults.offset(SUBCATCH_GW_FLOW as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / UCF(FLOW as ::core::ffi::c_int);
            (*gw).lowerDepth = (*Aquifer.offset((*gw).aquifer as isize)).bottomElev
                - *SubcatchResults.offset(SUBCATCH_GW_ELEV as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / UCF(LENGTH as ::core::ffi::c_int);
            (*gw).theta = *SubcatchResults
                .offset(SUBCATCH_SOIL_MOIST as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *(*Subcatch.offset(j as isize)).newQual.offset(i as isize) = *SubcatchResults
                .offset((SUBCATCH_WASHOFF as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_double;
            i += 1;
        }
        j += 1;
    }
    if kount
        < 1 as ::core::ffi::c_int + Nobjects[SUBCATCH as ::core::ffi::c_int as usize] * nResults
    {
        report_writeErrorMsg(
            ERR_RUNOFF_FILE_READ as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    OldRunoffTime = NewRunoffTime;
    NewRunoffTime = OldRunoffTime + tStep as ::core::ffi::c_double * 1000.0f64;
    NewRunoffTime = if NewRunoffTime <= TotalDuration {
        NewRunoffTime
    } else {
        TotalDuration
    };
    Nsteps += 1;
}
unsafe extern "C" fn runoff_getOutfallRunon(mut tStep: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < Nnodes[OUTFALL as ::core::ffi::c_int as usize] {
        k = (*Outfall.offset(i as isize)).routeTo;
        if !(k < 0 as ::core::ffi::c_int) {
            if !((*Subcatch.offset(k as isize)).area == 0.0f64) {
                subcatch_addRunonFlow(k, (*Outfall.offset(i as isize)).vRouted / tStep);
                massbal_updateRunoffTotals(
                    RUNOFF_RUNON as ::core::ffi::c_int,
                    (*Outfall.offset(i as isize)).vRouted,
                );
                (*Outfall.offset(i as isize)).vRouted = 0.0f64;
                p = 0 as ::core::ffi::c_int;
                while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    w = *(*Outfall.offset(i as isize)).wRouted.offset(p as isize) * LperFT3;
                    massbal_updateLoadingTotals(
                        DEPOSITION_LOAD as ::core::ffi::c_int,
                        p,
                        w * (*Pollut.offset(p as isize)).mcf,
                    );
                    *(*Subcatch.offset(k as isize)).newQual.offset(p as isize) += w / tStep;
                    *(*Outfall.offset(i as isize)).wRouted.offset(p as isize) = 0.0f64;
                    p += 1;
                }
            }
        }
        i += 1;
    }
}

#[repr(C)]
pub struct _IO_wide_data {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _opaque: [u8; 0],
}
