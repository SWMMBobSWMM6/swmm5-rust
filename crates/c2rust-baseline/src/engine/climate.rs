#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn rewind(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn datetime_encodeDate(
        year: ::core::ffi::c_int,
        month: ::core::ffi::c_int,
        day: ::core::ffi::c_int,
    ) -> DateTime;
    fn datetime_decodeDate(
        date: DateTime,
        y: *mut ::core::ffi::c_int,
        m: *mut ::core::ffi::c_int,
        d: *mut ::core::ffi::c_int,
    );
    fn datetime_monthOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_dayOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_daysPerMonth(
        year: ::core::ffi::c_int,
        month: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn datetime_strToDate(s: *mut ::core::ffi::c_char, d: *mut DateTime) -> ::core::ffi::c_int;
    static mut Fclimate: TFile;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut UnitSystem: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut StartDate: DateTime;
    static mut StartDateTime: DateTime;
    static mut EndDateTime: DateTime;
    static mut Temp: TTemp;
    static mut Evap: TEvap;
    static mut Wind: TWind;
    static mut Snow: TSnow;
    static mut Adjust: TAdjust;
    static mut Subcatch: *mut TSubcatch;
    static mut Pattern: *mut TPattern;
    static mut Tseries: *mut TTable;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn snow_setMeltCoeffs(snowIndex: ::core::ffi::c_int, season: ::core::ffi::c_double);
    fn table_getFirstEntry(
        table: *mut TTable,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn table_getNextEntry(
        table: *mut TTable,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn table_tseriesLookup(
        table: *mut TTable,
        t: ::core::ffi::c_double,
        extend: ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
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
    fn strcomp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sstrncpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        n: size_t,
    ) -> size_t;
    fn addAbsolutePath(fname: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut EvapTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut TempKeyWords: [*mut ::core::ffi::c_char; 0];
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
pub type FileUsageType = ::core::ffi::c_uint;
pub const SAVE_FILE: FileUsageType = 3;
pub const USE_FILE: FileUsageType = 2;
pub const SCRATCH_FILE: FileUsageType = 1;
pub const NO_FILE: FileUsageType = 0;
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
pub type TempType = ::core::ffi::c_uint;
pub const FILE_TEMP: TempType = 2;
pub const TSERIES_TEMP: TempType = 1;
pub const NO_TEMP: TempType = 0;
pub type WindType = ::core::ffi::c_uint;
pub const FILE_WIND: WindType = 1;
pub const MONTHLY_WIND: WindType = 0;
pub type EvapType = ::core::ffi::c_uint;
pub const DRYONLY: EvapType = 6;
pub const RECOVERY: EvapType = 5;
pub const FILE_EVAP: EvapType = 4;
pub const TEMPERATURE_EVAP: EvapType = 3;
pub const TIMESERIES_EVAP: EvapType = 2;
pub const MONTHLY_EVAP: EvapType = 1;
pub const CONSTANT_EVAP: EvapType = 0;
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
pub struct TFile {
    pub name: [::core::ffi::c_char; 260],
    pub mode: ::core::ffi::c_char,
    pub state: ::core::ffi::c_char,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableEntry {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub next: *mut TableEntry,
}
pub type TTableEntry = TableEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTable {
    pub ID: *mut ::core::ffi::c_char,
    pub curveType: ::core::ffi::c_int,
    pub refersTo: ::core::ffi::c_int,
    pub dxMin: ::core::ffi::c_double,
    pub lastDate: ::core::ffi::c_double,
    pub x1: ::core::ffi::c_double,
    pub x2: ::core::ffi::c_double,
    pub y1: ::core::ffi::c_double,
    pub y2: ::core::ffi::c_double,
    pub firstEntry: *mut TTableEntry,
    pub lastEntry: *mut TTableEntry,
    pub thisEntry: *mut TTableEntry,
    pub file: TFile,
}
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
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_ITEMS: ErrorType = 203;
pub const DEG_C: TempUnitsType = 1;
pub const DEG_F: TempUnitsType = 2;
pub const ERR_DATETIME: ErrorType = 213;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_SNOWMELT_PARAMS: ErrorType = 181;
pub const WIND: ClimateVarType = 3;
pub const TMIN: ClimateVarType = 0;
pub const WDMV: WindSpeedType = 0;
pub const DEG_C10: TempUnitsType = 0;
pub const EVAP: ClimateVarType = 2;
pub const TMAX: ClimateVarType = 1;
pub const GHCND: ClimateFileFormats = 2;
pub const DLY0204: ClimateFileFormats = 4;
pub const TD3200: ClimateFileFormats = 3;
pub const USER_PREPARED: ClimateFileFormats = 1;
pub const ERR_CLIMATE_FILE_READ: ErrorType = 338;
pub const ERR_CLIMATE_END_OF_FILE: ErrorType = 339;
pub const UNKNOWN_FORMAT: ClimateFileFormats = 0;
pub const AWND: WindSpeedType = 1;
pub const ERR_CLIMATE_FILE_OPEN: ErrorType = 337;
pub const ERR_NO_CLIMATE_FILE: ErrorType = 336;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TMovAve {
    pub tAve: ::core::ffi::c_double,
    pub tRng: ::core::ffi::c_double,
    pub ta: [::core::ffi::c_double; 7],
    pub tr: [::core::ffi::c_double; 7],
    pub count: ::core::ffi::c_int,
    pub maxCount: ::core::ffi::c_int,
    pub front: ::core::ffi::c_int,
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
pub type ClimateFileFormats = ::core::ffi::c_uint;
pub type ClimateVarType = ::core::ffi::c_uint;
pub type WindSpeedType = ::core::ffi::c_uint;
pub type TempUnitsType = ::core::ffi::c_uint;
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXFNAME: ::core::ffi::c_int = 259 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MISSING: ::core::ffi::c_double = -1.0E10f64;
pub const PI: ::core::ffi::c_double = 3.141592654f64;
pub const MMperINCH: ::core::ffi::c_double = 25.40f64;
pub const w_FILE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"FILE\0") };
pub const w_IMPERV: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"IMPERV\0") };
pub const w_PERV: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"PERV\0") };
pub const w_NO: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"NO\0") };
pub const w_YES: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"YES\0") };
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut ::core::ffi::c_char);
}
pub const NO_DATE: ::core::ffi::c_int = -(693594 as ::core::ffi::c_int);
static mut MAXCLIMATEVARS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut MAXDAYSPERMONTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut ClimateVarWords: [*mut ::core::ffi::c_char; 6] = [
    b"TMIN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TMAX\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"EVAP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"WDMV\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"AWND\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut TempUnitsWords: [*mut ::core::ffi::c_char; 4] = [
    b"C10\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut Tmin: ::core::ffi::c_double = 0.;
static mut Tmax: ::core::ffi::c_double = 0.;
static mut Trng: ::core::ffi::c_double = 0.;
static mut Trng1: ::core::ffi::c_double = 0.;
static mut Tave: ::core::ffi::c_double = 0.;
static mut Hrsr: ::core::ffi::c_double = 0.;
static mut Hrss: ::core::ffi::c_double = 0.;
static mut Hrday: ::core::ffi::c_double = 0.;
static mut Dhrdy: ::core::ffi::c_double = 0.;
static mut Dydif: ::core::ffi::c_double = 0.;
static mut LastDay: DateTime = 0.;
static mut Tma: TMovAve = TMovAve {
    tAve: 0.,
    tRng: 0.,
    ta: [0.; 7],
    tr: [0.; 7],
    count: 0,
    maxCount: 0,
    front: 0,
};
static mut NextEvapDate: DateTime = 0.;
static mut NextEvapRate: ::core::ffi::c_double = 0.;
static mut FileFormat: ::core::ffi::c_int = 0;
static mut FileYear: ::core::ffi::c_int = 0;
static mut FileMonth: ::core::ffi::c_int = 0;
static mut FileDay: ::core::ffi::c_int = 0;
static mut FileLastDay: ::core::ffi::c_int = 0;
static mut FileElapsedDays: ::core::ffi::c_int = 0;
static mut FileValue: [::core::ffi::c_double; 4] = [0.; 4];
static mut FileData: [[::core::ffi::c_double; 32]; 4] = [[0.; 32]; 4];
static mut FileLine: [::core::ffi::c_char; 1025] = [0; 1025];
static mut FileFieldPos: [::core::ffi::c_int; 4] = [0; 4];
static mut FileDateFieldPos: ::core::ffi::c_int = 0;
static mut FileWindType: ::core::ffi::c_int = 0;
static mut FileTempUnits: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn climate_readParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    let mut y: ::core::ffi::c_double = 0.;
    let mut aDate: DateTime = 0.;
    let mut fname: [::core::ffi::c_char; 260] = [0; 260];
    k = findmatch(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        &raw mut TempKeyWords as *mut *mut ::core::ffi::c_char,
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    match k {
        0 => {
            if ntoks < 2 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            i = project_findObject(
                TSERIES as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
            if i < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            Temp.dataSource = TSERIES_TEMP as ::core::ffi::c_int;
            Temp.tSeries = i;
            (*Tseries.offset(i as isize)).refersTo = TSERIES_TEMP as ::core::ffi::c_int;
        }
        1 => {
            if ntoks < 2 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            Temp.dataSource = FILE_TEMP as ::core::ffi::c_int;
            Fclimate.mode = USE_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
            sstrncpy(
                &raw mut fname as *mut ::core::ffi::c_char,
                *tok.offset(1 as ::core::ffi::c_int as isize),
                MAXFNAME as size_t,
            );
            sstrncpy(
                &raw mut Fclimate.name as *mut ::core::ffi::c_char,
                addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
                MAXFNAME as size_t,
            );
            Temp.fileStartDate = NO_DATE as DateTime;
            if ntoks > 2 as ::core::ffi::c_int {
                if **tok.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '*' as i32
                {
                    if datetime_strToDate(
                        *tok.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut aDate,
                    ) == 0
                    {
                        return error_setInpError(
                            ERR_DATETIME as ::core::ffi::c_int,
                            *tok.offset(2 as ::core::ffi::c_int as isize),
                        );
                    }
                    Temp.fileStartDate = aDate;
                }
            }
            FileTempUnits = DEG_F as ::core::ffi::c_int;
            if UnitSystem == SI as ::core::ffi::c_int {
                FileTempUnits = DEG_C as ::core::ffi::c_int;
            }
            if ntoks > 3 as ::core::ffi::c_int {
                i = findmatch(
                    *tok.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut TempUnitsWords as *mut *mut ::core::ffi::c_char,
                );
                if i < 0 as ::core::ffi::c_int {
                    return error_setInpError(
                        ERR_KEYWORD as ::core::ffi::c_int,
                        *tok.offset(3 as ::core::ffi::c_int as isize),
                    );
                }
                FileTempUnits = i;
            }
        }
        2 => {
            if strcomp(
                *tok.offset(1 as ::core::ffi::c_int as isize),
                w_FILE.as_ptr(),
            ) != 0
            {
                Wind.type_0 = FILE_WIND as ::core::ffi::c_int;
            } else {
                if ntoks < 14 as ::core::ffi::c_int {
                    return error_setInpError(
                        ERR_ITEMS as ::core::ffi::c_int,
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                Wind.type_0 = MONTHLY_WIND as ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < 12 as ::core::ffi::c_int {
                    if getDouble(
                        *tok.offset((i + 2 as ::core::ffi::c_int) as isize),
                        &raw mut y,
                    ) == 0
                    {
                        return error_setInpError(
                            ERR_NUMBER as ::core::ffi::c_int,
                            *tok.offset((i + 2 as ::core::ffi::c_int) as isize),
                        );
                    }
                    Wind.aws[i as usize] = y;
                    i += 1;
                }
            }
        }
        3 => {
            if ntoks < 7 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            i = 1 as ::core::ffi::c_int;
            while i < 7 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset(i as isize),
                    (&raw mut x as *mut ::core::ffi::c_double)
                        .offset((i - 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset(i as isize),
                    );
                }
                i += 1;
            }
            if UnitSystem == SI as ::core::ffi::c_int {
                x[0 as ::core::ffi::c_int as usize] =
                    9.0f64 / 5.0f64 * x[0 as ::core::ffi::c_int as usize] + 32.0f64;
            }
            Snow.snotmp = x[0 as ::core::ffi::c_int as usize];
            Snow.tipm = x[1 as ::core::ffi::c_int as usize];
            Snow.rnm = x[2 as ::core::ffi::c_int as usize];
            Temp.elev = x[3 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
            Temp.anglat = x[4 as ::core::ffi::c_int as usize];
            Temp.dtlong = x[5 as ::core::ffi::c_int as usize] / 60.0f64;
        }
        4 => {
            if ntoks < 12 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            if match_0(
                *tok.offset(1 as ::core::ffi::c_int as isize),
                w_IMPERV.as_ptr() as *mut ::core::ffi::c_char,
            ) != 0
            {
                i = 0 as ::core::ffi::c_int;
            } else if match_0(
                *tok.offset(1 as ::core::ffi::c_int as isize),
                w_PERV.as_ptr() as *mut ::core::ffi::c_char,
            ) != 0
            {
                i = 1 as ::core::ffi::c_int;
            } else {
                return error_setInpError(
                    ERR_KEYWORD as ::core::ffi::c_int,
                    *tok.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            j = 0 as ::core::ffi::c_int;
            while j < 10 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset((j + 2 as ::core::ffi::c_int) as isize),
                    &raw mut y,
                ) == 0
                    || y < 0.0f64
                    || y > 1.0f64
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset((j + 2 as ::core::ffi::c_int) as isize),
                    );
                }
                Snow.adc[i as usize][j as usize] = y;
                j += 1;
            }
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn climate_readEvapParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    k = findmatch(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        &raw mut EvapTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if k == RECOVERY as ::core::ffi::c_int {
        if ntoks < 2 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = project_findObject(
            TIMEPATTERN as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        Evap.recoveryPattern = i;
        return 0 as ::core::ffi::c_int;
    }
    if k == DRYONLY as ::core::ffi::c_int {
        if ntoks < 2 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if strcomp(*tok.offset(1 as ::core::ffi::c_int as isize), w_NO.as_ptr()) != 0 {
            Evap.dryOnly = FALSE;
        } else if strcomp(
            *tok.offset(1 as ::core::ffi::c_int as isize),
            w_YES.as_ptr(),
        ) != 0
        {
            Evap.dryOnly = TRUE;
        } else {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    Evap.type_0 = k;
    if k != TEMPERATURE_EVAP as ::core::ffi::c_int && ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    match k {
        0 => {
            if getDouble(*tok.offset(1 as ::core::ffi::c_int as isize), &raw mut x) == 0 {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            i = 0 as ::core::ffi::c_int;
            while i < 12 as ::core::ffi::c_int {
                Evap.monthlyEvap[i as usize] = x;
                i += 1;
            }
        }
        1 => {
            if ntoks < 13 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            i = 0 as ::core::ffi::c_int;
            while i < 12 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
                    (&raw mut Evap.monthlyEvap as *mut ::core::ffi::c_double).offset(i as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
                    );
                }
                i += 1;
            }
        }
        2 => {
            i = project_findObject(
                TSERIES as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
            if i < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            Evap.tSeries = i;
            (*Tseries.offset(i as isize)).refersTo = TIMESERIES_EVAP as ::core::ffi::c_int;
        }
        4 => {
            if ntoks > 1 as ::core::ffi::c_int {
                if ntoks < 13 as ::core::ffi::c_int {
                    return error_setInpError(
                        ERR_ITEMS as ::core::ffi::c_int,
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                i = 0 as ::core::ffi::c_int;
                while i < 12 as ::core::ffi::c_int {
                    if getDouble(
                        *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
                        (&raw mut Evap.panCoeff as *mut ::core::ffi::c_double).offset(i as isize)
                            as *mut ::core::ffi::c_double,
                    ) == 0
                    {
                        return error_setInpError(
                            ERR_NUMBER as ::core::ffi::c_int,
                            *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
                        );
                    }
                    i += 1;
                }
            }
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn climate_readAdjustments(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if ntoks == 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        b"TEMP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if ntoks < 13 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = 1 as ::core::ffi::c_int;
        while i < 13 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(i as isize),
                (&raw mut Adjust.temp as *mut ::core::ffi::c_double)
                    .offset((i - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(i as isize),
                );
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        b"EVAP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if ntoks < 13 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = 1 as ::core::ffi::c_int;
        while i < 13 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(i as isize),
                (&raw mut Adjust.evap as *mut ::core::ffi::c_double)
                    .offset((i - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(i as isize),
                );
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        b"RAIN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if ntoks < 13 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = 1 as ::core::ffi::c_int;
        while i < 13 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(i as isize),
                (&raw mut Adjust.rain as *mut ::core::ffi::c_double)
                    .offset((i - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(i as isize),
                );
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        b"CONDUCT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if ntoks < 13 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = 1 as ::core::ffi::c_int;
        while i < 13 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(i as isize),
                (&raw mut Adjust.hydcon as *mut ::core::ffi::c_double)
                    .offset((i - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(i as isize),
                );
            }
            if Adjust.hydcon[(i - 1 as ::core::ffi::c_int) as usize] <= 0.0f64 {
                Adjust.hydcon[(i - 1 as ::core::ffi::c_int) as usize] = 1.0f64;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        b"N-PERV\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if ntoks < 3 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = project_findObject(
            SUBCATCH as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        j = project_findObject(
            TIMEPATTERN as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
        if j < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        (*Subcatch.offset(i as isize)).nPervPattern = j;
        return 0 as ::core::ffi::c_int;
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        b"DSTORE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if ntoks < 3 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = project_findObject(
            SUBCATCH as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        j = project_findObject(
            TIMEPATTERN as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
        if j < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        (*Subcatch.offset(i as isize)).dStorePattern = j;
        return 0 as ::core::ffi::c_int;
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        b"INFIL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if ntoks < 3 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = project_findObject(
            SUBCATCH as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        j = project_findObject(
            TIMEPATTERN as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
        if j < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        (*Subcatch.offset(i as isize)).infilPattern = j;
        return 0 as ::core::ffi::c_int;
    }
    return error_setInpError(
        ERR_KEYWORD as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn climate_validate() {
    let mut i: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_double = 0.;
    let mut z: ::core::ffi::c_double = 0.;
    let mut pa: ::core::ffi::c_double = 0.;
    if Wind.type_0 == FILE_WIND as ::core::ffi::c_int
        || Evap.type_0 == FILE_EVAP as ::core::ffi::c_int
        || Evap.type_0 == TEMPERATURE_EVAP as ::core::ffi::c_int
    {
        if Fclimate.mode as ::core::ffi::c_int == NO_FILE as ::core::ffi::c_int {
            report_writeErrorMsg(
                ERR_NO_CLIMATE_FILE as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    if Fclimate.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        climate_openFile();
    }
    if Snow.tipm < 0.0f64 || Snow.tipm > 1.0f64 || Snow.rnm < 0.0f64 || Snow.rnm > 1.0f64 {
        report_writeErrorMsg(
            ERR_SNOWMELT_PARAMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    a = Temp.anglat;
    if a <= -89.99f64 || a >= 89.99f64 {
        report_writeErrorMsg(
            ERR_SNOWMELT_PARAMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        Temp.tanAnglat = tan(a * PI / 180.0f64);
    }
    z = Temp.elev / 1000.0f64;
    if z <= 0.0f64 {
        pa = 29.9f64;
    } else {
        pa = 29.9f64 - 1.02f64 * z + 0.0032f64 * pow(z, 2.4f64);
    }
    Temp.gamma = 0.000359f64 * pa;
    i = 0 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        if UnitSystem == SI as ::core::ffi::c_int {
            Adjust.temp[i as usize] *= 9.0f64 / 5.0f64;
        }
        Adjust.evap[i as usize] /= UCF(EVAPRATE as ::core::ffi::c_int);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn climate_openFile() {
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    Fclimate.file = fopen(
        &raw mut Fclimate.name as *mut ::core::ffi::c_char,
        b"rt\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Fclimate.file.is_null() {
        report_writeErrorMsg(
            ERR_CLIMATE_FILE_OPEN as ::core::ffi::c_int,
            &raw mut Fclimate.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    FileValue[TMIN as ::core::ffi::c_int as usize] = Temp.ta;
    FileValue[TMAX as ::core::ffi::c_int as usize] = Temp.ta;
    FileValue[EVAP as ::core::ffi::c_int as usize] = 0.0f64;
    FileValue[WIND as ::core::ffi::c_int as usize] = 0.0f64;
    FileFormat = getFileFormat();
    if FileFormat == UNKNOWN_FORMAT as ::core::ffi::c_int {
        report_writeErrorMsg(
            ERR_CLIMATE_FILE_READ as ::core::ffi::c_int,
            &raw mut Fclimate.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    rewind(Fclimate.file);
    sstrncpy(
        &raw mut FileLine as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        0 as size_t,
    );
    if Temp.fileStartDate == NO_DATE as DateTime {
        datetime_decodeDate(
            StartDate,
            &raw mut FileYear,
            &raw mut FileMonth,
            &raw mut FileDay,
        );
    } else {
        datetime_decodeDate(
            Temp.fileStartDate,
            &raw mut FileYear,
            &raw mut FileMonth,
            &raw mut FileDay,
        );
    }
    while feof(Fclimate.file) == 0 {
        sstrncpy(
            &raw mut FileLine as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as size_t,
        );
        readFileLine(&raw mut y, &raw mut m);
        if y == FileYear && m == FileMonth {
            break;
        }
    }
    if feof(Fclimate.file) != 0 {
        report_writeErrorMsg(
            ERR_CLIMATE_END_OF_FILE as ::core::ffi::c_int,
            &raw mut Fclimate.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    if ErrorCode == 0 {
        FileElapsedDays = 0 as ::core::ffi::c_int;
        FileLastDay = datetime_daysPerMonth(FileYear, FileMonth);
        readFileValues();
        i = TMIN as ::core::ffi::c_int;
        while i <= WIND as ::core::ffi::c_int {
            if !(FileData[i as usize][FileDay as usize] == MISSING) {
                FileValue[i as usize] = FileData[i as usize][FileDay as usize];
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn climate_initState() {
    LastDay = NO_DATE as DateTime;
    Temp.tmax = MISSING;
    Snow.removed = 0.0f64;
    NextEvapDate = StartDate;
    NextEvapRate = 0.0f64;
    if Evap.type_0 == TIMESERIES_EVAP as ::core::ffi::c_int
        && Evap.tSeries >= 0 as ::core::ffi::c_int
    {
        table_getFirstEntry(
            Tseries.offset(Evap.tSeries as isize) as *mut TTable,
            &raw mut NextEvapDate,
            &raw mut NextEvapRate,
        );
        if NextEvapDate < StartDate {
            setNextEvapDate(StartDate);
        }
        Evap.rate = NextEvapRate / UCF(EVAPRATE as ::core::ffi::c_int);
        setNextEvapDate(NextEvapDate);
    }
    if Evap.type_0 == TEMPERATURE_EVAP as ::core::ffi::c_int {
        Tma.maxCount = (::core::mem::size_of::<[::core::ffi::c_double; 7]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_double>() as usize)
            as ::core::ffi::c_int;
        Tma.count = 0 as ::core::ffi::c_int;
        Tma.front = 0 as ::core::ffi::c_int;
        Tma.tAve = 0.0f64;
        Tma.tRng = 0.0f64;
    }
}
#[no_mangle]
pub unsafe extern "C" fn climate_setState(mut theDate: DateTime) {
    if Fclimate.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        updateFileValues(theDate);
    }
    if Temp.dataSource != NO_TEMP as ::core::ffi::c_int {
        setTemp(theDate);
    }
    setEvap(theDate);
    setWind(theDate);
    Adjust.rainFactor =
        Adjust.rain[(datetime_monthOfYear(theDate) - 1 as ::core::ffi::c_int) as usize];
    Adjust.hydconFactor =
        Adjust.hydcon[(datetime_monthOfYear(theDate) - 1 as ::core::ffi::c_int) as usize];
    setNextEvapDate(theDate);
}
#[no_mangle]
pub unsafe extern "C" fn climate_getNextEvapDate() -> DateTime {
    return NextEvapDate;
}
unsafe extern "C" fn setNextEvapDate(mut theDate: DateTime) {
    let mut yr: ::core::ffi::c_int = 0;
    let mut mon: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_double = 0.;
    let mut e: ::core::ffi::c_double = 0.;
    if NextEvapDate > theDate {
        return;
    }
    match Evap.type_0 {
        0 => {
            NextEvapDate = (theDate as ::core::ffi::c_double + 365.0f64) as DateTime;
        }
        1 => {
            datetime_decodeDate(theDate, &raw mut yr, &raw mut mon, &raw mut day);
            if mon == 12 as ::core::ffi::c_int {
                mon = 1 as ::core::ffi::c_int;
                yr += 1;
            } else {
                mon += 1;
            }
            NextEvapDate = datetime_encodeDate(yr, mon, 1 as ::core::ffi::c_int);
        }
        2 => {
            k = Evap.tSeries;
            if k >= 0 as ::core::ffi::c_int {
                NextEvapDate = (theDate as ::core::ffi::c_double + 365.0f64) as DateTime;
                while table_getNextEntry(
                    Tseries.offset(k as isize) as *mut TTable,
                    &raw mut d,
                    &raw mut e,
                ) != 0
                    && d <= EndDateTime
                {
                    if !(d >= theDate) {
                        continue;
                    }
                    NextEvapDate = d as DateTime;
                    NextEvapRate = e;
                    break;
                }
            }
        }
        4 => {
            NextEvapDate = (floor(theDate as ::core::ffi::c_double) + 1.0f64) as DateTime;
        }
        _ => {
            NextEvapDate = (theDate as ::core::ffi::c_double + 365.0f64) as DateTime;
        }
    };
}
unsafe extern "C" fn updateFileValues(mut theDate: DateTime) {
    let mut i: ::core::ffi::c_int = 0;
    let mut deltaDays: ::core::ffi::c_int = 0;
    deltaDays = crate::ccast::f64_to_i32((floor(theDate as ::core::ffi::c_double)
        - floor(StartDateTime as ::core::ffi::c_double)));
    if deltaDays > FileElapsedDays {
        FileElapsedDays += 1;
        FileDay += 1;
        if FileDay > FileLastDay {
            FileMonth += 1;
            if FileMonth > 12 as ::core::ffi::c_int {
                FileMonth = 1 as ::core::ffi::c_int;
                FileYear += 1;
            }
            readFileValues();
            FileDay = 1 as ::core::ffi::c_int;
            FileLastDay = datetime_daysPerMonth(FileYear, FileMonth);
        }
        i = TMIN as ::core::ffi::c_int;
        while i <= WIND as ::core::ffi::c_int {
            if !(FileData[i as usize][FileDay as usize] == MISSING) {
                FileValue[i as usize] = FileData[i as usize][FileDay as usize];
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn setTemp(mut theDate: DateTime) {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut mon: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut theDay: DateTime = 0.;
    let mut hour: ::core::ffi::c_double = 0.;
    let mut tmp: ::core::ffi::c_double = 0.;
    mon = datetime_monthOfYear(theDate);
    theDay = floor(theDate as ::core::ffi::c_double) as DateTime;
    if theDay > LastDay {
        day = datetime_dayOfYear(theDate);
        if Temp.dataSource == FILE_TEMP as ::core::ffi::c_int {
            Tmin = FileValue[TMIN as ::core::ffi::c_int as usize]
                + Adjust.temp[(mon - 1 as ::core::ffi::c_int) as usize];
            Tmax = FileValue[TMAX as ::core::ffi::c_int as usize]
                + Adjust.temp[(mon - 1 as ::core::ffi::c_int) as usize];
            if Tmin > Tmax {
                tmp = Tmin;
                Tmin = Tmax;
                Tmax = tmp;
            }
            updateTempTimes(day);
            if Evap.type_0 == TEMPERATURE_EVAP as ::core::ffi::c_int {
                updateTempMoveAve(Tmin, Tmax);
                FileValue[EVAP as ::core::ffi::c_int as usize] =
                    getTempEvap(day, Tma.tAve, Tma.tRng);
            }
        }
        Snow.season = sin(0.0172615f64 * (day as ::core::ffi::c_double - 81.0f64));
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[SNOWMELT as ::core::ffi::c_int as usize] {
            snow_setMeltCoeffs(j, Snow.season);
            j += 1;
        }
        LastDay = theDate;
    }
    if Temp.dataSource == FILE_TEMP as ::core::ffi::c_int {
        hour = (theDate as ::core::ffi::c_double - theDay as ::core::ffi::c_double) * 24.0f64;
        if hour < Hrsr {
            Temp.ta = Tmin + Trng1 / 2.0f64 * sin(PI / Dydif * (Hrsr - hour));
        } else if hour >= Hrsr && hour <= Hrss {
            Temp.ta = Tave + Trng * sin(PI / Dhrdy * (Hrday - hour));
        } else {
            Temp.ta = Tmax - Trng * sin(PI / Dydif * (hour - Hrss));
        }
    }
    if Temp.dataSource == TSERIES_TEMP as ::core::ffi::c_int {
        k = Temp.tSeries;
        if k >= 0 as ::core::ffi::c_int {
            Temp.ta = table_tseriesLookup(
                Tseries.offset(k as isize) as *mut TTable,
                theDate as ::core::ffi::c_double,
                TRUE as ::core::ffi::c_char,
            );
            if UnitSystem == SI as ::core::ffi::c_int {
                Temp.ta = 9.0f64 / 5.0f64 * Temp.ta + 32.0f64;
            }
            Temp.ta += Adjust.temp[(mon - 1 as ::core::ffi::c_int) as usize];
        }
    }
    Temp.ea = 8.1175e6f64 * exp(-7701.544f64 / (Temp.ta + 405.0265f64));
}
unsafe extern "C" fn setEvap(mut theDate: DateTime) {
    let mut k: ::core::ffi::c_int = 0;
    let mut mon: ::core::ffi::c_int = datetime_monthOfYear(theDate);
    match Evap.type_0 {
        0 => {
            Evap.rate = Evap.monthlyEvap[0 as ::core::ffi::c_int as usize]
                / UCF(EVAPRATE as ::core::ffi::c_int);
        }
        1 => {
            Evap.rate = Evap.monthlyEvap[(mon - 1 as ::core::ffi::c_int) as usize]
                / UCF(EVAPRATE as ::core::ffi::c_int);
        }
        2 => {
            if theDate >= NextEvapDate {
                Evap.rate = NextEvapRate / UCF(EVAPRATE as ::core::ffi::c_int);
            }
        }
        4 => {
            Evap.rate = FileValue[EVAP as ::core::ffi::c_int as usize]
                / UCF(EVAPRATE as ::core::ffi::c_int);
            Evap.rate *= Evap.panCoeff[(mon - 1 as ::core::ffi::c_int) as usize];
        }
        3 => {
            Evap.rate = FileValue[EVAP as ::core::ffi::c_int as usize]
                / UCF(EVAPRATE as ::core::ffi::c_int);
        }
        _ => {
            Evap.rate = 0.0f64;
        }
    }
    Evap.rate += Adjust.evap[(mon - 1 as ::core::ffi::c_int) as usize];
    Evap.recoveryFactor = 1.0f64;
    k = Evap.recoveryPattern;
    if k >= 0 as ::core::ffi::c_int
        && (*Pattern.offset(k as isize)).type_0 == MONTHLY_PATTERN as ::core::ffi::c_int
    {
        Evap.recoveryFactor =
            (*Pattern.offset(k as isize)).factor[(mon - 1 as ::core::ffi::c_int) as usize];
    }
}
unsafe extern "C" fn setWind(mut theDate: DateTime) {
    let mut yr: ::core::ffi::c_int = 0;
    let mut mon: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    match Wind.type_0 {
        0 => {
            datetime_decodeDate(theDate, &raw mut yr, &raw mut mon, &raw mut day);
            Wind.ws = Wind.aws[(mon - 1 as ::core::ffi::c_int) as usize]
                / UCF(WINDSPEED as ::core::ffi::c_int);
        }
        1 => {
            Wind.ws = FileValue[WIND as ::core::ffi::c_int as usize];
        }
        _ => {
            Wind.ws = 0.0f64;
        }
    };
}
unsafe extern "C" fn updateTempTimes(mut day: ::core::ffi::c_int) {
    let mut decl: ::core::ffi::c_double = 0.;
    let mut hrang: ::core::ffi::c_double = 0.;
    let mut arg: ::core::ffi::c_double = 0.;
    decl = 0.40928f64 * cos(0.017202f64 * (172.0f64 - day as ::core::ffi::c_double));
    arg = -tan(decl) * Temp.tanAnglat;
    if arg <= -1.0f64 {
        arg = PI;
    } else if arg >= 1.0f64 {
        arg = 0.0f64;
    } else {
        arg = acos(arg);
    }
    hrang = 3.8197f64 * arg;
    Hrsr = 12.0f64 - hrang + Temp.dtlong;
    Hrss = 12.0f64 + hrang + Temp.dtlong - 3.0f64;
    Dhrdy = Hrsr - Hrss;
    Dydif = 24.0f64 + Hrsr - Hrss;
    Hrday = (Hrsr + Hrss) / 2.0f64;
    Tave = (Tmin + Tmax) / 2.0f64;
    Trng = (Tmax - Tmin) / 2.0f64;
    if Temp.tmax == MISSING {
        Trng1 = Tmax - Tmin;
    } else {
        Trng1 = Temp.tmax - Tmin;
    }
    Temp.tmax = Tmax;
}
unsafe extern "C" fn getTempEvap(
    mut day: ::core::ffi::c_int,
    mut tave: ::core::ffi::c_double,
    mut trng: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a: ::core::ffi::c_double = 2.0f64 * PI / 365.0f64;
    let mut ta: ::core::ffi::c_double = (tave - 32.0f64) * 5.0f64 / 9.0f64;
    let mut tr: ::core::ffi::c_double = trng * 5.0f64 / 9.0f64;
    let mut lamda: ::core::ffi::c_double = 2.50f64 - 0.002361f64 * ta;
    let mut dr: ::core::ffi::c_double = 1.0f64 + 0.033f64 * cos(a * day as ::core::ffi::c_double);
    let mut phi: ::core::ffi::c_double = Temp.anglat * 2.0f64 * PI / 360.0f64;
    let mut del: ::core::ffi::c_double =
        0.4093f64 * sin(a * (284.0f64 + day as ::core::ffi::c_double));
    let mut omega: ::core::ffi::c_double = acos(-tan(phi) * tan(del));
    let mut ra: ::core::ffi::c_double =
        37.6f64 * dr * (omega * sin(phi) * sin(del) + cos(phi) * cos(del) * sin(omega));
    let mut e: ::core::ffi::c_double = 0.0023f64 * ra / lamda * sqrt(tr) * (ta + 17.8f64);
    if e < 0.0f64 {
        e = 0.0f64;
    }
    if UnitSystem == US as ::core::ffi::c_int {
        e /= MMperINCH;
    }
    return e;
}
unsafe extern "C" fn getFileFormat() -> ::core::ffi::c_int {
    let mut recdType: [::core::ffi::c_char; 4] =
        ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\0\0\0\0");
    let mut elemType: [::core::ffi::c_char; 4] =
        ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\0\0\0\0");
    let mut filler: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\0\0\0\0\0");
    let mut staID: [::core::ffi::c_char; 80] = [0; 80];
    let mut s: [::core::ffi::c_char; 80] = [0; 80];
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut y: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Fclimate.file,
    )
    .is_null()
    {
        return UNKNOWN_FORMAT as ::core::ffi::c_int;
    }
    sstrncpy(
        &raw mut recdType as *mut ::core::ffi::c_char,
        &raw mut line as *mut ::core::ffi::c_char,
        3 as size_t,
    );
    sstrncpy(
        &raw mut filler as *mut ::core::ffi::c_char,
        (&raw mut line as *mut ::core::ffi::c_char).offset(23 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        4 as size_t,
    );
    if strcmp(
        &raw mut recdType as *mut ::core::ffi::c_char,
        b"DLY\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut filler as *mut ::core::ffi::c_char,
            b"9999\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return TD3200 as ::core::ffi::c_int;
    }
    if strlen(&raw mut line as *mut ::core::ffi::c_char) >= 233 as size_t {
        sstrncpy(
            &raw mut elemType as *mut ::core::ffi::c_char,
            (&raw mut line as *mut ::core::ffi::c_char).offset(13 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            3 as size_t,
        );
        n = atoi(&raw mut elemType as *mut ::core::ffi::c_char);
        if n == 1 as ::core::ffi::c_int
            || n == 2 as ::core::ffi::c_int
            || n == 151 as ::core::ffi::c_int
        {
            return DLY0204 as ::core::ffi::c_int;
        }
    }
    n = sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%s %d %d %d %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut staID as *mut ::core::ffi::c_char,
        &raw mut y,
        &raw mut m,
        &raw mut d,
        &raw mut s as *mut ::core::ffi::c_char,
    );
    if n == 5 as ::core::ffi::c_int {
        return USER_PREPARED as ::core::ffi::c_int;
    }
    if isGhcndFormat(&raw mut line as *mut ::core::ffi::c_char) != 0 {
        return GHCND as ::core::ffi::c_int;
    }
    return UNKNOWN_FORMAT as ::core::ffi::c_int;
}
unsafe extern "C" fn readFileLine(mut y: *mut ::core::ffi::c_int, mut m: *mut ::core::ffi::c_int) {
    while strlen(&raw mut FileLine as *mut ::core::ffi::c_char) == 0 as size_t {
        if fgets(
            &raw mut FileLine as *mut ::core::ffi::c_char,
            MAXLINE,
            Fclimate.file,
        )
        .is_null()
        {
            return;
        }
        if FileLine[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\n' as i32 {
            FileLine[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        }
    }
    match FileFormat {
        1 => {
            readUserFileLine(y, m);
        }
        3 => {
            readTD3200FileLine(y, m);
        }
        4 => {
            readDLY0204FileLine(y, m);
        }
        2 => {
            readGhcndFileLine(y, m);
        }
        _ => {}
    };
}
unsafe extern "C" fn readUserFileLine(
    mut y: *mut ::core::ffi::c_int,
    mut m: *mut ::core::ffi::c_int,
) {
    let mut n: ::core::ffi::c_int = 0;
    let mut staID: [::core::ffi::c_char; 80] = [0; 80];
    n = sscanf(
        &raw mut FileLine as *mut ::core::ffi::c_char,
        b"%s %d %d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut staID as *mut ::core::ffi::c_char,
        y,
        m,
    );
    if n < 3 as ::core::ffi::c_int {
        report_writeErrorMsg(
            ERR_CLIMATE_FILE_READ as ::core::ffi::c_int,
            &raw mut Fclimate.name as *mut ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn readTD3200FileLine(
    mut y: *mut ::core::ffi::c_int,
    mut m: *mut ::core::ffi::c_int,
) {
    let mut recdType: [::core::ffi::c_char; 4] =
        ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\0\0\0\0");
    let mut year: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\0\0\0\0\0");
    let mut month: [::core::ffi::c_char; 3] =
        ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\0\0\0");
    if strlen(&raw mut FileLine as *mut ::core::ffi::c_char) < 30 as size_t {
        report_writeErrorMsg(
            ERR_CLIMATE_FILE_READ as ::core::ffi::c_int,
            &raw mut Fclimate.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    sstrncpy(
        &raw mut recdType as *mut ::core::ffi::c_char,
        &raw mut FileLine as *mut ::core::ffi::c_char,
        3 as size_t,
    );
    if strcmp(
        &raw mut recdType as *mut ::core::ffi::c_char,
        b"DLY\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        report_writeErrorMsg(
            ERR_CLIMATE_FILE_READ as ::core::ffi::c_int,
            &raw mut Fclimate.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    sstrncpy(
        &raw mut year as *mut ::core::ffi::c_char,
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(17 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        4 as size_t,
    );
    sstrncpy(
        &raw mut month as *mut ::core::ffi::c_char,
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(21 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        2 as size_t,
    );
    *y = atoi(&raw mut year as *mut ::core::ffi::c_char);
    *m = atoi(&raw mut month as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn readDLY0204FileLine(
    mut y: *mut ::core::ffi::c_int,
    mut m: *mut ::core::ffi::c_int,
) {
    let mut year: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\0\0\0\0\0");
    let mut month: [::core::ffi::c_char; 3] =
        ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\0\0\0");
    if strlen(&raw mut FileLine as *mut ::core::ffi::c_char) < 16 as size_t {
        report_writeErrorMsg(
            ERR_CLIMATE_FILE_READ as ::core::ffi::c_int,
            &raw mut Fclimate.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    sstrncpy(
        &raw mut year as *mut ::core::ffi::c_char,
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(7 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        4 as size_t,
    );
    sstrncpy(
        &raw mut month as *mut ::core::ffi::c_char,
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(11 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        2 as size_t,
    );
    *y = atoi(&raw mut year as *mut ::core::ffi::c_char);
    *m = atoi(&raw mut month as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn readFileValues() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MAXCLIMATEVARS {
        j = 0 as ::core::ffi::c_int;
        while j < MAXDAYSPERMONTH {
            FileData[i as usize][j as usize] = MISSING;
            j += 1;
        }
        i += 1;
    }
    while ErrorCode == 0 {
        if feof(Fclimate.file) != 0 {
            return;
        }
        readFileLine(&raw mut y, &raw mut m);
        if y > FileYear || m > FileMonth {
            return;
        }
        match FileFormat {
            1 => {
                parseUserFileLine();
            }
            3 => {
                parseTD3200FileLine();
            }
            4 => {
                parseDLY0204FileLine();
            }
            2 => {
                parseGhcndFileLine();
            }
            _ => {}
        }
        sstrncpy(
            &raw mut FileLine as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as size_t,
        );
    }
}
unsafe extern "C" fn parseUserFileLine() {
    let mut n: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut staID: [::core::ffi::c_char; 80] = [0; 80];
    let mut s0: [::core::ffi::c_char; 80] = ::core::mem::transmute::<
        [u8; 80],
        [::core::ffi::c_char; 80],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut s1: [::core::ffi::c_char; 80] = ::core::mem::transmute::<
        [u8; 80],
        [::core::ffi::c_char; 80],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut s2: [::core::ffi::c_char; 80] = ::core::mem::transmute::<
        [u8; 80],
        [::core::ffi::c_char; 80],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut s3: [::core::ffi::c_char; 80] = ::core::mem::transmute::<
        [u8; 80],
        [::core::ffi::c_char; 80],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut x: ::core::ffi::c_double = 0.;
    n = sscanf(
        &raw mut FileLine as *mut ::core::ffi::c_char,
        b"%s %d %d %d %s %s %s %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut staID as *mut ::core::ffi::c_char,
        &raw mut y,
        &raw mut m,
        &raw mut d,
        &raw mut s0 as *mut ::core::ffi::c_char,
        &raw mut s1 as *mut ::core::ffi::c_char,
        &raw mut s2 as *mut ::core::ffi::c_char,
        &raw mut s3 as *mut ::core::ffi::c_char,
    );
    if n < 4 as ::core::ffi::c_int {
        return;
    }
    if d < 1 as ::core::ffi::c_int || d > 31 as ::core::ffi::c_int {
        return;
    }
    if strlen(&raw mut s0 as *mut ::core::ffi::c_char) > 0 as size_t
        && *(&raw mut s0 as *mut ::core::ffi::c_char) as ::core::ffi::c_int != '*' as i32
    {
        x = atof(&raw mut s0 as *mut ::core::ffi::c_char);
        if UnitSystem == SI as ::core::ffi::c_int {
            x = 9.0f64 / 5.0f64 * x + 32.0f64;
        }
        FileData[TMAX as ::core::ffi::c_int as usize][d as usize] = x;
    }
    if strlen(&raw mut s1 as *mut ::core::ffi::c_char) > 0 as size_t
        && *(&raw mut s1 as *mut ::core::ffi::c_char) as ::core::ffi::c_int != '*' as i32
    {
        x = atof(&raw mut s1 as *mut ::core::ffi::c_char);
        if UnitSystem == SI as ::core::ffi::c_int {
            x = 9.0f64 / 5.0f64 * x + 32.0f64;
        }
        FileData[TMIN as ::core::ffi::c_int as usize][d as usize] = x;
    }
    if strlen(&raw mut s2 as *mut ::core::ffi::c_char) > 0 as size_t
        && *(&raw mut s2 as *mut ::core::ffi::c_char) as ::core::ffi::c_int != '*' as i32
    {
        FileData[EVAP as ::core::ffi::c_int as usize][d as usize] =
            atof(&raw mut s2 as *mut ::core::ffi::c_char);
    }
    if strlen(&raw mut s3 as *mut ::core::ffi::c_char) > 0 as size_t
        && *(&raw mut s3 as *mut ::core::ffi::c_char) as ::core::ffi::c_int != '*' as i32
    {
        FileData[WIND as ::core::ffi::c_int as usize][d as usize] =
            atof(&raw mut s3 as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn parseTD3200FileLine() {
    let mut i: ::core::ffi::c_int = 0;
    let mut param: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\0\0\0\0\0");
    sstrncpy(
        &raw mut param as *mut ::core::ffi::c_char,
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(11 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        4 as size_t,
    );
    i = 0 as ::core::ffi::c_int;
    while i < MAXCLIMATEVARS {
        if strcmp(
            &raw mut param as *mut ::core::ffi::c_char,
            ClimateVarWords[i as usize],
        ) == 0 as ::core::ffi::c_int
        {
            setTD3200FileValues(i);
        }
        i += 1;
    }
}
unsafe extern "C" fn setTD3200FileValues(mut i: ::core::ffi::c_int) {
    let mut valCount: [::core::ffi::c_char; 4] =
        ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\0\0\0\0");
    let mut day: [::core::ffi::c_char; 3] =
        ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\0\0\0");
    let mut sign: [::core::ffi::c_char; 2] =
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\0\0");
    let mut value: [::core::ffi::c_char; 6] =
        ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"\0\0\0\0\0\0");
    let mut flag2: [::core::ffi::c_char; 2] =
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\0\0");
    let mut x: ::core::ffi::c_double = 0.;
    let mut nValues: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    sstrncpy(
        &raw mut valCount as *mut ::core::ffi::c_char,
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(27 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        3 as size_t,
    );
    nValues = atoi(&raw mut valCount as *mut ::core::ffi::c_char);
    if strlen(&raw mut FileLine as *mut ::core::ffi::c_char) as ::core::ffi::c_int
        >= 12 as ::core::ffi::c_int * nValues + 30 as ::core::ffi::c_int
    {
        j = 0 as ::core::ffi::c_int;
        while j < nValues {
            k = 30 as ::core::ffi::c_int + j * 12 as ::core::ffi::c_int;
            sstrncpy(
                &raw mut day as *mut ::core::ffi::c_char,
                (&raw mut FileLine as *mut ::core::ffi::c_char).offset(k as isize)
                    as *mut ::core::ffi::c_char,
                2 as size_t,
            );
            sstrncpy(
                &raw mut sign as *mut ::core::ffi::c_char,
                (&raw mut FileLine as *mut ::core::ffi::c_char)
                    .offset((k + 4 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char,
                1 as size_t,
            );
            sstrncpy(
                &raw mut value as *mut ::core::ffi::c_char,
                (&raw mut FileLine as *mut ::core::ffi::c_char)
                    .offset((k + 5 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char,
                5 as size_t,
            );
            sstrncpy(
                &raw mut flag2 as *mut ::core::ffi::c_char,
                (&raw mut FileLine as *mut ::core::ffi::c_char)
                    .offset((k + 11 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char,
                1 as size_t,
            );
            d = atoi(&raw mut day as *mut ::core::ffi::c_char);
            if strcmp(
                &raw mut value as *mut ::core::ffi::c_char,
                b"99999\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && (flag2[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '0' as i32
                    || flag2[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '1' as i32)
                && d > 0 as ::core::ffi::c_int
                && d <= 31 as ::core::ffi::c_int
            {
                x = atof(&raw mut value as *mut ::core::ffi::c_char);
                if sign[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '-' as i32 {
                    x = -x;
                }
                if i == EVAP as ::core::ffi::c_int {
                    x /= 100.0f64;
                    if UnitSystem == SI as ::core::ffi::c_int {
                        x *= MMperINCH;
                    }
                }
                if i == WIND as ::core::ffi::c_int {
                    x /= 24.0f64;
                }
                FileData[i as usize][d as usize] = x;
            }
            j += 1;
        }
    }
}
unsafe extern "C" fn parseDLY0204FileLine() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut param: [::core::ffi::c_char; 4] =
        ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\0\0\0\0");
    let mut sign: [::core::ffi::c_char; 2] =
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\0\0");
    let mut value: [::core::ffi::c_char; 6] =
        ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"\0\0\0\0\0\0");
    let mut code: [::core::ffi::c_char; 2] =
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\0\0");
    let mut x: ::core::ffi::c_double = 0.;
    sstrncpy(
        &raw mut param as *mut ::core::ffi::c_char,
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(13 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        3 as size_t,
    );
    p = atoi(&raw mut param as *mut ::core::ffi::c_char);
    if p == 1 as ::core::ffi::c_int {
        p = TMAX as ::core::ffi::c_int;
    } else if p == 2 as ::core::ffi::c_int {
        p = TMIN as ::core::ffi::c_int;
    } else if p == 151 as ::core::ffi::c_int {
        p = EVAP as ::core::ffi::c_int;
    } else {
        return;
    }
    if strlen(&raw mut FileLine as *mut ::core::ffi::c_char) < 233 as size_t {
        return;
    }
    k = 16 as ::core::ffi::c_int;
    j = 1 as ::core::ffi::c_int;
    while j <= 31 as ::core::ffi::c_int {
        sstrncpy(
            &raw mut sign as *mut ::core::ffi::c_char,
            (&raw mut FileLine as *mut ::core::ffi::c_char).offset(k as isize)
                as *mut ::core::ffi::c_char,
            1 as size_t,
        );
        sstrncpy(
            &raw mut value as *mut ::core::ffi::c_char,
            (&raw mut FileLine as *mut ::core::ffi::c_char)
                .offset((k + 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_char,
            5 as size_t,
        );
        sstrncpy(
            &raw mut code as *mut ::core::ffi::c_char,
            (&raw mut FileLine as *mut ::core::ffi::c_char)
                .offset((k + 6 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_char,
            1 as size_t,
        );
        k += 7 as ::core::ffi::c_int;
        if strcmp(
            &raw mut value as *mut ::core::ffi::c_char,
            b"99999\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut value as *mut ::core::ffi::c_char,
                b"     \0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            match p {
                1 | 0 => {
                    x = atof(&raw mut value as *mut ::core::ffi::c_char) / 10.0f64;
                    if sign[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '-' as i32 {
                        x = -x;
                    }
                    x = 9.0f64 / 5.0f64 * x + 32.0f64;
                }
                2 => {
                    x = atof(&raw mut value as *mut ::core::ffi::c_char) / 10.0f64;
                    if UnitSystem == US as ::core::ffi::c_int {
                        x /= MMperINCH;
                    }
                }
                _ => return,
            }
            FileData[p as usize][j as usize] = x;
        }
        j += 1;
    }
}
unsafe extern "C" fn isGhcndFormat(mut line: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ptr = strstr(line, b"DATE\0" as *const u8 as *const ::core::ffi::c_char);
    if ptr.is_null() {
        return FALSE;
    }
    FileDateFieldPos = ptr.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
    i = TMIN as ::core::ffi::c_int;
    while i <= WIND as ::core::ffi::c_int {
        FileFieldPos[i as usize] = -(1 as ::core::ffi::c_int);
        i += 1;
    }
    ptr = strstr(line, b"TMIN\0" as *const u8 as *const ::core::ffi::c_char);
    if !ptr.is_null() {
        FileFieldPos[TMIN as ::core::ffi::c_int as usize] =
            ptr.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
    ptr = strstr(line, b"TMAX\0" as *const u8 as *const ::core::ffi::c_char);
    if !ptr.is_null() {
        FileFieldPos[TMAX as ::core::ffi::c_int as usize] =
            ptr.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
    ptr = strstr(line, b"EVAP\0" as *const u8 as *const ::core::ffi::c_char);
    if !ptr.is_null() {
        FileFieldPos[EVAP as ::core::ffi::c_int as usize] =
            ptr.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
    FileWindType = WDMV as ::core::ffi::c_int;
    ptr = strstr(line, b"WDMV\0" as *const u8 as *const ::core::ffi::c_char);
    if ptr.is_null() {
        FileWindType = AWND as ::core::ffi::c_int;
        ptr = strstr(line, b"AWND\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !ptr.is_null() {
        FileFieldPos[WIND as ::core::ffi::c_int as usize] =
            ptr.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
    i = TMIN as ::core::ffi::c_int;
    while i <= WIND as ::core::ffi::c_int {
        if FileFieldPos[i as usize] >= 0 as ::core::ffi::c_int {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
unsafe extern "C" fn readGhcndFileLine(
    mut y: *mut ::core::ffi::c_int,
    mut m: *mut ::core::ffi::c_int,
) {
    let mut n: ::core::ffi::c_int = sscanf(
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(FileDateFieldPos as isize)
            as *mut ::core::ffi::c_char,
        b"%4d%2d\0" as *const u8 as *const ::core::ffi::c_char,
        y,
        m,
    );
    if n != 2 as ::core::ffi::c_int {
        *y = -(99999 as ::core::ffi::c_int);
        *m = -(99999 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn parseGhcndFileLine() {
    let mut y: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_double = 0.;
    n = sscanf(
        (&raw mut FileLine as *mut ::core::ffi::c_char).offset(FileDateFieldPos as isize)
            as *mut ::core::ffi::c_char,
        b"%4d%2d%2d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut y,
        &raw mut m,
        &raw mut d,
    );
    if n < 3 as ::core::ffi::c_int {
        return;
    }
    if d < 1 as ::core::ffi::c_int || d > 31 as ::core::ffi::c_int {
        return;
    }
    i = TMIN as ::core::ffi::c_int;
    while i <= WIND as ::core::ffi::c_int {
        if FileFieldPos[i as usize] >= 0 as ::core::ffi::c_int {
            if sscanf(
                (&raw mut FileLine as *mut ::core::ffi::c_char).offset(
                    *(&raw mut FileFieldPos as *mut ::core::ffi::c_int).offset(i as isize) as isize,
                ) as *mut ::core::ffi::c_char,
                b"%8lf\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut v,
            ) > 0 as ::core::ffi::c_int
            {
                if fabs(v) < 9999.0f64 {
                    FileData[i as usize][d as usize] = convertGhcndValue(i, v);
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn convertGhcndValue(
    mut var: ::core::ffi::c_int,
    mut v: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    match var {
        0 | 1 => match FileTempUnits {
            0 => return v / 10.0f64 * 9.0f64 / 5.0f64 + 32.0f64,
            1 => return v * 9.0f64 / 5.0f64 + 32.0f64,
            _ => return v,
        },
        2 => match FileTempUnits {
            0 => {
                v /= 10.0f64;
                if UnitSystem == US as ::core::ffi::c_int {
                    v /= MMperINCH;
                }
                return v;
            }
            1 => {
                if UnitSystem == US as ::core::ffi::c_int {
                    v /= MMperINCH;
                }
                return v;
            }
            _ => {
                if UnitSystem == SI as ::core::ffi::c_int {
                    v *= MMperINCH;
                }
                return v;
            }
        },
        3 => match FileTempUnits {
            0 => {
                if FileWindType == WDMV as ::core::ffi::c_int {
                    return v * 0.62137f64 / 24.0f64;
                } else {
                    return v / 10.0f64 / 1000.0f64 * 0.62137f64 * 3600.0f64;
                }
            }
            1 => {
                if FileWindType == WDMV as ::core::ffi::c_int {
                    return v * 0.62137f64 / 24.0f64;
                } else {
                    return v / 1000.0f64 * 0.62137f64 * 3600.0f64;
                }
            }
            _ => {
                if FileWindType == WDMV as ::core::ffi::c_int {
                    return v / 24.0f64;
                } else {
                    return v;
                }
            }
        },
        _ => return v,
    };
}
unsafe extern "C" fn updateTempMoveAve(
    mut tmin: ::core::ffi::c_double,
    mut tmax: ::core::ffi::c_double,
) {
    let mut ta: ::core::ffi::c_double = 0.;
    let mut tr: ::core::ffi::c_double = 0.;
    let mut kount: ::core::ffi::c_int = Tma.count;
    let mut count: ::core::ffi::c_double = kount as ::core::ffi::c_double;
    ta = (tmin + tmax) / 2.0f64;
    tr = fabs(tmax - tmin);
    if kount == Tma.maxCount {
        Tma.tAve = (Tma.tAve * count + ta - Tma.ta[Tma.front as usize]) / count;
        Tma.tRng = (Tma.tRng * count + tr - Tma.tr[Tma.front as usize]) / count;
        Tma.ta[Tma.front as usize] = ta;
        Tma.tr[Tma.front as usize] = tr;
        Tma.front += 1;
        if Tma.front as ::core::ffi::c_double == count {
            Tma.front = 0 as ::core::ffi::c_int;
        }
    } else {
        Tma.tAve =
            (Tma.tAve * count + ta) / (count + 1 as ::core::ffi::c_int as ::core::ffi::c_double);
        Tma.tRng =
            (Tma.tRng * count + tr) / (count + 1 as ::core::ffi::c_int as ::core::ffi::c_double);
        Tma.ta[Tma.front as usize] = ta;
        Tma.tr[Tma.front as usize] = tr;
        Tma.count += 1;
        Tma.front += 1;
        if Tma.count == Tma.maxCount {
            Tma.front = 0 as ::core::ffi::c_int;
        }
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

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
