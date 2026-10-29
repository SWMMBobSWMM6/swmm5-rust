#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn datetime_encodeDate(
        year: ::core::ffi::c_int,
        month: ::core::ffi::c_int,
        day: ::core::ffi::c_int,
    ) -> DateTime;
    fn datetime_encodeTime(
        hour: ::core::ffi::c_int,
        minute: ::core::ffi::c_int,
        second: ::core::ffi::c_int,
    ) -> DateTime;
    fn datetime_monthOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_findMonth(s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn datetime_addSeconds(date1: DateTime, seconds: ::core::ffi::c_double) -> DateTime;
    static mut Frdii: TFile;
    static mut IgnoreRDII: ::core::ffi::c_int;
    static mut WetStep: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut ErrorCode: ::core::ffi::c_int;
    static mut StartDateTime: DateTime;
    static mut TotalDuration: ::core::ffi::c_double;
    static mut Adjust: TAdjust;
    static mut Gage: *mut TGage;
    static mut UnitHyd: *mut TUnitHyd;
    static mut Node: *mut TNode;
    static mut Tseries: *mut TTable;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn report_writeRdiiStats(totalRain: ::core::ffi::c_double, totalRdii: ::core::ffi::c_double);
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn gage_initState(gage: ::core::ffi::c_int);
    fn gage_setState(gage: ::core::ffi::c_int, aDate: DateTime);
    fn table_tseriesInit(table: *mut TTable);
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn getTempFileName(s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "match"]
    fn match_0(
        str: *mut ::core::ffi::c_char,
        substr: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut Qcf: [::core::ffi::c_double; 0];
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut FlowUnitWords: [*mut ::core::ffi::c_char; 0];
    static mut UHTypeWords: [*mut ::core::ffi::c_char; 0];
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
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
pub struct TUnitHyd {
    pub ID: *mut ::core::ffi::c_char,
    pub rainGage: ::core::ffi::c_int,
    pub iaMax: [[::core::ffi::c_double; 3]; 12],
    pub iaRecov: [[::core::ffi::c_double; 3]; 12],
    pub iaInit: [[::core::ffi::c_double; 3]; 12],
    pub r: [[::core::ffi::c_double; 3]; 12],
    pub tBase: [[::core::ffi::c_long; 3]; 12],
    pub tPeak: [[::core::ffi::c_long; 3]; 12],
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_KEYWORD: ErrorType = 205;
pub const TEXT: FileTypes = 1;
pub const ERR_RDII_FILE_FORMAT: ErrorType = 345;
pub const ERR_RDII_FILE_OPEN: ErrorType = 343;
pub const ERR_RDII_FILE_SCRATCH: ErrorType = 341;
pub const BINARY: FileTypes = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUHGroup {
    pub isUsed: ::core::ffi::c_int,
    pub rainInterval: ::core::ffi::c_int,
    pub area: ::core::ffi::c_double,
    pub rdii: ::core::ffi::c_double,
    pub gageDate: DateTime,
    pub lastDate: DateTime,
    pub uh: [TUHData; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUHData {
    pub pastRain: *mut ::core::ffi::c_double,
    pub pastMonth: *mut ::core::ffi::c_char,
    pub period: ::core::ffi::c_int,
    pub hasPastRain: ::core::ffi::c_int,
    pub maxPeriods: ::core::ffi::c_int,
    pub drySeconds: ::core::ffi::c_long,
    pub iaUsed: ::core::ffi::c_double,
}
pub const ERR_RDII_AREA: ErrorType = 155;
pub const ERR_UNITHYD_RATIOS: ErrorType = 153;
pub const ERR_UNITHYD_TIMES: ErrorType = 151;
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
pub type FileTypes = ::core::ffi::c_uint;
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;
pub const w_ALL: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"ALL\0") };
pub const NO_DATE: ::core::ffi::c_int = -(693594 as ::core::ffi::c_int);
pub const FILE_STAMP: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"SWMM5-RDII\0") };
#[no_mangle]
pub static mut ZERO_RDII: ::core::ffi::c_double = 0.0001f64;
#[no_mangle]
pub static mut FileStamp: [::core::ffi::c_char; 11] = FILE_STAMP;
static mut UHGroup: *mut TUHGroup = ::core::ptr::null::<TUHGroup>() as *mut TUHGroup;
static mut RdiiStep: ::core::ffi::c_int = 0;
static mut NumRdiiNodes: ::core::ffi::c_int = 0;
static mut RdiiNodeIndex: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut RdiiNodeFlow: *mut ::core::ffi::c_float =
    ::core::ptr::null::<::core::ffi::c_float>() as *mut ::core::ffi::c_float;
static mut RdiiFlowUnits: ::core::ffi::c_int = 0;
static mut RdiiStartDate: DateTime = 0.;
static mut RdiiEndDate: DateTime = 0.;
static mut TotalRainVol: ::core::ffi::c_double = 0.;
static mut TotalRdiiVol: ::core::ffi::c_double = 0.;
static mut RdiiFileType: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn rdii_readRdiiInflow(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_double = 0.;
    let mut inflow: *mut TRdiiInflow = ::core::ptr::null_mut::<TRdiiInflow>();
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    k = project_findObject(
        UNITHYD as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(*tok.offset(2 as ::core::ffi::c_int as isize), &raw mut a) == 0 || a < 0.0f64 {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    inflow = (*Node.offset(j as isize)).rdiiInflow;
    if inflow.is_null() {
        inflow = malloc(::core::mem::size_of::<TRdiiInflow>() as size_t) as *mut TRdiiInflow;
        if inflow.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    (*inflow).unitHyd = k;
    (*inflow).area = a / UCF(LANDAREA as ::core::ffi::c_int);
    let ref mut fresh0 = (*Node.offset(j as isize)).rdiiInflow;
    *fresh0 = inflow;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdii_initUnitHyd(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    m = 0 as ::core::ffi::c_int;
    while m < 12 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            (*UnitHyd.offset(j as isize)).iaMax[m as usize][i as usize] = 0.0f64;
            (*UnitHyd.offset(j as isize)).iaRecov[m as usize][i as usize] = 0.0f64;
            (*UnitHyd.offset(j as isize)).iaInit[m as usize][i as usize] = 0.0f64;
            (*UnitHyd.offset(j as isize)).r[m as usize][i as usize] = 0.0f64;
            (*UnitHyd.offset(j as isize)).tPeak[m as usize][i as usize] = 0 as ::core::ffi::c_long;
            (*UnitHyd.offset(j as isize)).tBase[m as usize][i as usize] = 0 as ::core::ffi::c_long;
            i += 1;
        }
        m += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rdii_readUnitHydParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    j = project_findObject(
        UNITHYD as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if (*UnitHyd.offset(j as isize)).ID.is_null() {
        let ref mut fresh2 = (*UnitHyd.offset(j as isize)).ID;
        *fresh2 = project_findID(
            UNITHYD as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if ntoks == 2 as ::core::ffi::c_int {
        g = project_findObject(
            GAGE as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
        if g < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        (*UnitHyd.offset(j as isize)).rainGage = g;
        (*Gage.offset(g as isize)).isUsed = TRUE;
        return 0 as ::core::ffi::c_int;
    } else if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    m = datetime_findMonth(*tok.offset(1 as ::core::ffi::c_int as isize));
    if m == 0 as ::core::ffi::c_int {
        if match_0(
            *tok.offset(1 as ::core::ffi::c_int as isize),
            w_ALL.as_ptr() as *mut ::core::ffi::c_char,
        ) == 0
        {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
    }
    k = findmatch(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        &raw mut UHTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if k < 0 as ::core::ffi::c_int {
        return readOldUHFormat(j, m, tok, ntoks);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
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
    i = 3 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        x[i as usize] = 0.0f64;
        if ntoks > i + 3 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset((i + 3 as ::core::ffi::c_int) as isize),
                (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset((i + 2 as ::core::ffi::c_int) as isize),
                );
            }
        }
        i += 1;
    }
    setUnitHydParams(j, k, m, &raw mut x as *mut ::core::ffi::c_double);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readOldUHFormat(
    mut j: ::core::ffi::c_int,
    mut m: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: [::core::ffi::c_double; 9] = [0.; 9];
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    if ntoks < 11 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < 9 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset((i + 2 as ::core::ffi::c_int) as isize),
            (&raw mut p as *mut ::core::ffi::c_double).offset(i as isize)
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
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        x[(i + 3 as ::core::ffi::c_int) as usize] = 0.0f64;
        if ntoks > i + 11 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset((i + 11 as ::core::ffi::c_int) as isize),
                (&raw mut x as *mut ::core::ffi::c_double)
                    .offset((i + 3 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset((i + 11 as ::core::ffi::c_int) as isize),
                );
            }
        }
        i += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < 3 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            x[i as usize] = p[(3 as ::core::ffi::c_int * k + i) as usize];
            setUnitHydParams(j, k, m, &raw mut x as *mut ::core::ffi::c_double);
            i += 1;
        }
        k += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn setUnitHydParams(
    mut j: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
    mut m: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    let mut m1: ::core::ffi::c_int = 0;
    let mut m2: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_double = 0.;
    let mut k: ::core::ffi::c_double = 0.;
    let mut tBase: ::core::ffi::c_double = 0.;
    if m == 0 as ::core::ffi::c_int {
        m1 = 0 as ::core::ffi::c_int;
        m2 = 11 as ::core::ffi::c_int;
    } else {
        m1 = m - 1 as ::core::ffi::c_int;
        m2 = m1;
    }
    m = m1;
    while m <= m2 {
        (*UnitHyd.offset(j as isize)).r[m as usize][i as usize] =
            *x.offset(0 as ::core::ffi::c_int as isize);
        t = *x.offset(1 as ::core::ffi::c_int as isize);
        k = *x.offset(2 as ::core::ffi::c_int as isize);
        tBase = t * (1.0f64 + k);
        (*UnitHyd.offset(j as isize)).tPeak[m as usize][i as usize] =
            crate::ccast::f64_to_i64((t * 3600.0f64));
        (*UnitHyd.offset(j as isize)).tBase[m as usize][i as usize] =
            crate::ccast::f64_to_i64((tBase * 3600.0f64));
        (*UnitHyd.offset(j as isize)).iaMax[m as usize][i as usize] =
            *x.offset(3 as ::core::ffi::c_int as isize);
        (*UnitHyd.offset(j as isize)).iaRecov[m as usize][i as usize] =
            *x.offset(4 as ::core::ffi::c_int as isize);
        (*UnitHyd.offset(j as isize)).iaInit[m as usize][i as usize] =
            *x.offset(5 as ::core::ffi::c_int as isize);
        m += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rdii_deleteRdiiInflow(mut j: ::core::ffi::c_int) {
    if !(*Node.offset(j as isize)).rdiiInflow.is_null() {
        free((*Node.offset(j as isize)).rdiiInflow as *mut ::core::ffi::c_void);
        let ref mut fresh1 = (*Node.offset(j as isize)).rdiiInflow;
        *fresh1 = ::core::ptr::null_mut::<TRdiiInflow>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn rdii_openRdii() {
    let mut fStamp: [::core::ffi::c_char; 11] = FILE_STAMP;
    RdiiNodeIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    RdiiNodeFlow = ::core::ptr::null_mut::<::core::ffi::c_float>();
    NumRdiiNodes = 0 as ::core::ffi::c_int;
    RdiiStartDate = NO_DATE as DateTime;
    if IgnoreRDII != 0 {
        return;
    }
    if Frdii.mode as ::core::ffi::c_int != USE_FILE as ::core::ffi::c_int {
        createRdiiFile();
    }
    if Frdii.mode as ::core::ffi::c_int == NO_FILE as ::core::ffi::c_int || ErrorCode != 0 {
        return;
    }
    Frdii.file = fopen(
        &raw mut Frdii.name as *mut ::core::ffi::c_char,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Frdii.file.is_null() {
        if Frdii.mode as ::core::ffi::c_int == SCRATCH_FILE as ::core::ffi::c_int {
            report_writeErrorMsg(
                ERR_RDII_FILE_SCRATCH as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            report_writeErrorMsg(
                ERR_RDII_FILE_OPEN as ::core::ffi::c_int,
                &raw mut Frdii.name as *mut ::core::ffi::c_char,
            );
        }
        return;
    }
    fread(
        &raw mut fStamp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        strlen(&raw const FileStamp as *const ::core::ffi::c_char),
        Frdii.file,
    );
    if strcmp(
        &raw mut fStamp as *mut ::core::ffi::c_char,
        &raw const FileStamp as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        RdiiFileType = BINARY as ::core::ffi::c_int;
        ErrorCode = readRdiiFileHeader();
    } else {
        fclose(Frdii.file);
        RdiiFileType = TEXT as ::core::ffi::c_int;
        openRdiiTextFile();
    }
    if ErrorCode != 0 {
        report_writeErrorMsg(ErrorCode, &raw mut Frdii.name as *mut ::core::ffi::c_char);
    } else {
        readRdiiFlows();
    };
}
unsafe extern "C" fn openRdiiTextFile() {
    Frdii.file = fopen(
        &raw mut Frdii.name as *mut ::core::ffi::c_char,
        b"rt\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Frdii.file.is_null() {
        if Frdii.mode as ::core::ffi::c_int == SCRATCH_FILE as ::core::ffi::c_int {
            report_writeErrorMsg(
                ERR_RDII_FILE_SCRATCH as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            report_writeErrorMsg(
                ERR_RDII_FILE_OPEN as ::core::ffi::c_int,
                &raw mut Frdii.name as *mut ::core::ffi::c_char,
            );
        }
        return;
    }
    ErrorCode = readRdiiTextFileHeader();
    if ErrorCode != 0 {
        report_writeErrorMsg(ErrorCode, &raw mut Frdii.name as *mut ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rdii_closeRdii() {
    if !Frdii.file.is_null() {
        fclose(Frdii.file);
    }
    if Frdii.mode as ::core::ffi::c_int == SCRATCH_FILE as ::core::ffi::c_int {
        remove(&raw mut Frdii.name as *mut ::core::ffi::c_char);
    }
    if !RdiiNodeIndex.is_null() {
        free(RdiiNodeIndex as *mut ::core::ffi::c_void);
        RdiiNodeIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !RdiiNodeFlow.is_null() {
        free(RdiiNodeFlow as *mut ::core::ffi::c_void);
        RdiiNodeFlow = ::core::ptr::null_mut::<::core::ffi::c_float>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn rdii_getNumRdiiFlows(mut aDate: DateTime) -> ::core::ffi::c_int {
    if NumRdiiNodes == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if Frdii.file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    while feof(Frdii.file) == 0 {
        if RdiiStartDate == NO_DATE as DateTime {
            return 0 as ::core::ffi::c_int;
        }
        if aDate < RdiiStartDate {
            return 0 as ::core::ffi::c_int;
        }
        if aDate < RdiiEndDate {
            return NumRdiiNodes;
        } else {
            readRdiiFlows();
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdii_getRdiiFlow(
    mut i: ::core::ffi::c_int,
    mut j: *mut ::core::ffi::c_int,
    mut q: *mut ::core::ffi::c_double,
) {
    if i >= 0 as ::core::ffi::c_int && i < NumRdiiNodes {
        *j = *RdiiNodeIndex.offset(i as isize);
        *q = *RdiiNodeFlow.offset(i as isize) as ::core::ffi::c_double;
    }
}
unsafe extern "C" fn readRdiiFileHeader() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    fread(
        &raw mut RdiiStep as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Frdii.file,
    );
    if RdiiStep <= 0 as ::core::ffi::c_int {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    fread(
        &raw mut NumRdiiNodes as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Frdii.file,
    );
    if NumRdiiNodes <= 0 as ::core::ffi::c_int {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    RdiiNodeIndex = calloc(
        NumRdiiNodes as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if RdiiNodeIndex.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    RdiiNodeFlow = calloc(
        NumRdiiNodes as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    if RdiiNodeFlow.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    if feof(Frdii.file) != 0 {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    fread(
        RdiiNodeIndex as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        NumRdiiNodes as size_t,
        Frdii.file,
    );
    i = 0 as ::core::ffi::c_int;
    while i < NumRdiiNodes {
        j = *RdiiNodeIndex.offset(i as isize);
        if (*Node.offset(j as isize)).rdiiInflow.is_null() {
            return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
        }
        i += 1;
    }
    if feof(Frdii.file) != 0 {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readRdiiTextFileHeader() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut s1: [::core::ffi::c_char; 1025] = ::core::mem::transmute::<
        [u8; 1025],
        [::core::ffi::c_char; 1025],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut s2: [::core::ffi::c_char; 1025] = [0; 1025];
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Frdii.file,
    );
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut s1 as *mut ::core::ffi::c_char,
    ) == 0
        || strcmp(
            &raw mut s1 as *mut ::core::ffi::c_char,
            b"SWMM5\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Frdii.file,
    );
    RdiiStep = 0 as ::core::ffi::c_int;
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Frdii.file,
    );
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut RdiiStep,
    ) == 0
        || RdiiStep <= 0 as ::core::ffi::c_int
    {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Frdii.file,
    );
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Frdii.file,
    );
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%s %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut s1 as *mut ::core::ffi::c_char,
        &raw mut s2 as *mut ::core::ffi::c_char,
    ) < 2 as ::core::ffi::c_int
    {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    RdiiFlowUnits = findmatch(
        &raw mut s2 as *mut ::core::ffi::c_char,
        &raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char,
    );
    if RdiiFlowUnits < 0 as ::core::ffi::c_int {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Frdii.file,
    );
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut NumRdiiNodes,
    ) < 1 as ::core::ffi::c_int
        || NumRdiiNodes <= 0 as ::core::ffi::c_int
    {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    RdiiNodeIndex = calloc(
        NumRdiiNodes as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if RdiiNodeIndex.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    RdiiNodeFlow = calloc(
        NumRdiiNodes as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    if RdiiNodeFlow.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumRdiiNodes {
        if feof(Frdii.file) != 0 {
            return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
        }
        fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            MAXLINE,
            Frdii.file,
        );
        if sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s1 as *mut ::core::ffi::c_char,
        ) == 0
        {
            return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
        }
        j = project_findObject(
            NODE as ::core::ffi::c_int,
            &raw mut s1 as *mut ::core::ffi::c_char,
        );
        if j < 0 as ::core::ffi::c_int {
            return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
        }
        *RdiiNodeIndex.offset(i as isize) = j;
        i += 1;
    }
    if feof(Frdii.file) != 0 {
        return ERR_RDII_FILE_FORMAT as ::core::ffi::c_int;
    }
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Frdii.file,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readRdiiFlows() {
    if RdiiFileType == TEXT as ::core::ffi::c_int {
        readRdiiTextFlows();
    } else {
        RdiiStartDate = NO_DATE as DateTime;
        RdiiEndDate = NO_DATE as DateTime;
        if feof(Frdii.file) != 0 {
            return;
        }
        fread(
            &raw mut RdiiStartDate as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<DateTime>() as size_t,
            1 as size_t,
            Frdii.file,
        );
        if RdiiStartDate == NO_DATE as DateTime {
            return;
        }
        if (fread(
            RdiiNodeFlow as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            NumRdiiNodes as size_t,
            Frdii.file,
        ) as size_t)
            < NumRdiiNodes as size_t
        {
            RdiiStartDate = NO_DATE as DateTime;
        } else {
            RdiiEndDate = datetime_addSeconds(RdiiStartDate, RdiiStep as ::core::ffi::c_double);
        }
    };
}
unsafe extern "C" fn readRdiiTextFlows() {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut yr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut day: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut min: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sec: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_double = 0.;
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut s: [::core::ffi::c_char; 1025] = [0; 1025];
    RdiiStartDate = NO_DATE as DateTime;
    i = 0 as ::core::ffi::c_int;
    while i < NumRdiiNodes {
        if feof(Frdii.file) != 0 {
            return;
        }
        fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            MAXLINE,
            Frdii.file,
        );
        n = sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%s %d %d %d %d %d %d %lf\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s as *mut ::core::ffi::c_char,
            &raw mut yr,
            &raw mut mon,
            &raw mut day,
            &raw mut hr,
            &raw mut min,
            &raw mut sec,
            &raw mut x,
        );
        if n < 8 as ::core::ffi::c_int {
            return;
        }
        *RdiiNodeFlow.offset(i as isize) = (x / *(&raw mut Qcf as *mut ::core::ffi::c_double)
            .offset(RdiiFlowUnits as isize))
            as ::core::ffi::c_float;
        i += 1;
    }
    RdiiStartDate = datetime_encodeDate(yr, mon, day) + datetime_encodeTime(hr, min, sec);
    RdiiEndDate = datetime_addSeconds(RdiiStartDate, RdiiStep as ::core::ffi::c_double);
}
unsafe extern "C" fn createRdiiFile() {
    let mut hasRdii: ::core::ffi::c_int = 0;
    let mut elapsedTime: ::core::ffi::c_double = 0.;
    let mut duration: ::core::ffi::c_double = 0.;
    let mut currentDate: DateTime = 0.;
    RdiiStep = WetStep;
    NumRdiiNodes = getNumRdiiNodes();
    if NumRdiiNodes == 0 as ::core::ffi::c_int {
        Frdii.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
        return;
    } else if Frdii.mode as ::core::ffi::c_int == NO_FILE as ::core::ffi::c_int {
        Frdii.mode = SCRATCH_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    }
    validateRdii();
    initGageData();
    if ErrorCode != 0 {
        return;
    }
    openRdiiProcessor();
    if ErrorCode == 0 {
        initUnitHydData();
        duration = TotalDuration / 1000.0f64;
        elapsedTime = 0.0f64;
        while elapsedTime <= duration && ErrorCode == 0 {
            currentDate = (StartDateTime as ::core::ffi::c_double
                + elapsedTime / SECperDAY as ::core::ffi::c_double)
                as DateTime;
            getRainfall(currentDate);
            getUnitHydRdii(currentDate);
            hasRdii = getNodeRdii();
            if hasRdii != 0 {
                saveRdiiFlows(currentDate);
            }
            elapsedTime += RdiiStep as ::core::ffi::c_double;
        }
    }
    closeRdiiProcessor();
}
unsafe extern "C" fn getNumRdiiNodes() -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !(*Node.offset(j as isize)).rdiiInflow.is_null() {
            n += 1;
        }
        j += 1;
    }
    return n;
}
unsafe extern "C" fn validateRdii() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut rsum: ::core::ffi::c_double = 0.;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
        m = 0 as ::core::ffi::c_int;
        while m < 12 as ::core::ffi::c_int {
            rsum = 0.0f64;
            k = 0 as ::core::ffi::c_int;
            while k < 3 as ::core::ffi::c_int {
                if !((*UnitHyd.offset(j as isize)).tBase[m as usize][k as usize]
                    == 0 as ::core::ffi::c_long)
                {
                    if ((*UnitHyd.offset(j as isize)).tPeak[m as usize][k as usize]
                        as ::core::ffi::c_double)
                        < 0.0f64
                    {
                        report_writeErrorMsg(
                            ERR_UNITHYD_TIMES as ::core::ffi::c_int,
                            (*UnitHyd.offset(j as isize)).ID,
                        );
                    }
                    if (*UnitHyd.offset(j as isize)).r[m as usize][k as usize] < 0.0f64 {
                        report_writeErrorMsg(
                            ERR_UNITHYD_RATIOS as ::core::ffi::c_int,
                            (*UnitHyd.offset(j as isize)).ID,
                        );
                    } else {
                        rsum += (*UnitHyd.offset(j as isize)).r[m as usize][k as usize];
                    }
                }
                k += 1;
            }
            if rsum > 1.01f64 {
                report_writeErrorMsg(
                    ERR_UNITHYD_RATIOS as ::core::ffi::c_int,
                    (*UnitHyd.offset(j as isize)).ID,
                );
            }
            m += 1;
        }
        j += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !(*Node.offset(i as isize)).rdiiInflow.is_null() {
            if (*(*Node.offset(i as isize)).rdiiInflow).area < 0.0f64 {
                report_writeErrorMsg(
                    ERR_RDII_AREA as ::core::ffi::c_int,
                    (*Node.offset(i as isize)).ID,
                );
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn openRdiiProcessor() {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    UHGroup = ::core::ptr::null_mut::<TUHGroup>();
    RdiiNodeIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    RdiiNodeFlow = ::core::ptr::null_mut::<::core::ffi::c_float>();
    TotalRainVol = 0.0f64;
    TotalRdiiVol = 0.0f64;
    if allocRdiiMemory() == 0 {
        report_writeErrorMsg(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    if openNewRdiiFile() == 0 {
        report_writeErrorMsg(
            ERR_RDII_FILE_SCRATCH as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    n = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !(*Node.offset(j as isize)).rdiiInflow.is_null() && !RdiiNodeIndex.is_null() {
            *RdiiNodeIndex.offset(n as isize) = j;
            n += 1;
        }
        j += 1;
    }
}
unsafe extern "C" fn allocRdiiMemory() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    UHGroup = calloc(
        Nobjects[UNITHYD as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TUHGroup>() as size_t,
    ) as *mut TUHGroup;
    if UHGroup.is_null() {
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
        (*UHGroup.offset(i as isize)).rainInterval = getRainInterval(i);
        k = 0 as ::core::ffi::c_int;
        while k < 3 as ::core::ffi::c_int {
            let ref mut fresh5 = (*UHGroup.offset(i as isize)).uh[k as usize].pastRain;
            *fresh5 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            let ref mut fresh6 = (*UHGroup.offset(i as isize)).uh[k as usize].pastMonth;
            *fresh6 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*UHGroup.offset(i as isize)).uh[k as usize].maxPeriods = getMaxPeriods(i, k);
            n = (*UHGroup.offset(i as isize)).uh[k as usize].maxPeriods;
            if n > 0 as ::core::ffi::c_int {
                let ref mut fresh7 = (*UHGroup.offset(i as isize)).uh[k as usize].pastRain;
                *fresh7 = calloc(
                    n as size_t,
                    ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                ) as *mut ::core::ffi::c_double;
                if (*UHGroup.offset(i as isize)).uh[k as usize]
                    .pastRain
                    .is_null()
                {
                    return FALSE;
                }
                let ref mut fresh8 = (*UHGroup.offset(i as isize)).uh[k as usize].pastMonth;
                *fresh8 = calloc(
                    n as size_t,
                    ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                ) as *mut ::core::ffi::c_char;
                if (*UHGroup.offset(i as isize)).uh[k as usize]
                    .pastMonth
                    .is_null()
                {
                    return FALSE;
                }
            }
            k += 1;
        }
        i += 1;
    }
    RdiiNodeIndex = calloc(
        NumRdiiNodes as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if RdiiNodeIndex.is_null() {
        return FALSE;
    }
    RdiiNodeFlow = calloc(
        NumRdiiNodes as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    if RdiiNodeFlow.is_null() {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn getRainInterval(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut ri: ::core::ffi::c_int = 0;
    let mut tLimb: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    ri = WetStep;
    m = 0 as ::core::ffi::c_int;
    while m < 12 as ::core::ffi::c_int {
        k = 0 as ::core::ffi::c_int;
        while k < 3 as ::core::ffi::c_int {
            if (*UnitHyd.offset(i as isize)).tPeak[m as usize][k as usize]
                > 0 as ::core::ffi::c_long
            {
                tLimb = (*UnitHyd.offset(i as isize)).tPeak[m as usize][k as usize]
                    as ::core::ffi::c_int;
                ri = if ri <= tLimb { ri } else { tLimb };
                tLimb = ((*UnitHyd.offset(i as isize)).tBase[m as usize][k as usize]
                    - tLimb as ::core::ffi::c_long) as ::core::ffi::c_int;
                if tLimb > 0 as ::core::ffi::c_int {
                    ri = if ri <= tLimb { ri } else { tLimb };
                }
            }
            k += 1;
        }
        m += 1;
    }
    return ri;
}
unsafe extern "C" fn getMaxPeriods(
    mut i: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nMax: ::core::ffi::c_int = 0;
    let mut rainInterval: ::core::ffi::c_int = 0;
    rainInterval = (*UHGroup.offset(i as isize)).rainInterval;
    nMax = 0 as ::core::ffi::c_int;
    m = 0 as ::core::ffi::c_int;
    while m < 12 as ::core::ffi::c_int {
        n = ((*UnitHyd.offset(i as isize)).tBase[m as usize][k as usize]
            / rainInterval as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
        nMax = if n >= nMax { n } else { nMax };
        m += 1;
    }
    return nMax;
}
unsafe extern "C" fn initGageData() {
    let mut i: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    g = 0 as ::core::ffi::c_int;
    while g < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        if (*Gage.offset(g as isize)).tSeries >= 0 as ::core::ffi::c_int {
            table_tseriesInit(
                Tseries.offset((*Gage.offset(g as isize)).tSeries as isize) as *mut TTable
            );
        }
        gage_initState(g);
        g += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
        g = (*UnitHyd.offset(i as isize)).rainGage;
        if g >= 0 as ::core::ffi::c_int {
            if (*Gage.offset(g as isize)).coGage >= 0 as ::core::ffi::c_int {
                (*UnitHyd.offset(i as isize)).rainGage = (*Gage.offset(g as isize)).coGage;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn initUnitHydData() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    month = datetime_monthOfYear(StartDateTime) - 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
        k = 0 as ::core::ffi::c_int;
        while k < 3 as ::core::ffi::c_int {
            (*UHGroup.offset(i as isize)).uh[k as usize].drySeconds =
                ((*UHGroup.offset(i as isize)).uh[k as usize].maxPeriods
                    * (*UHGroup.offset(i as isize)).rainInterval
                    + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
            (*UHGroup.offset(i as isize)).uh[k as usize].period =
                (*UHGroup.offset(i as isize)).uh[k as usize].maxPeriods + 1 as ::core::ffi::c_int;
            (*UHGroup.offset(i as isize)).uh[k as usize].hasPastRain = FALSE;
            (*UHGroup.offset(i as isize)).uh[k as usize].iaUsed =
                (*UnitHyd.offset(i as isize)).iaInit[month as usize][k as usize];
            k += 1;
        }
        (*UHGroup.offset(i as isize)).gageDate = StartDateTime;
        (*UHGroup.offset(i as isize)).area = 0.0f64;
        (*UHGroup.offset(i as isize)).rdii = 0.0f64;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
        (*UHGroup.offset(i as isize)).isUsed = FALSE;
        i += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n < NumRdiiNodes {
        j = *RdiiNodeIndex.offset(n as isize);
        i = (*(*Node.offset(j as isize)).rdiiInflow).unitHyd;
        (*UHGroup.offset(i as isize)).isUsed = TRUE;
        (*UHGroup.offset(i as isize)).lastDate = StartDateTime;
        (*UHGroup.offset(i as isize)).area += (*(*Node.offset(j as isize)).rdiiInflow).area;
        n += 1;
    }
}
unsafe extern "C" fn openNewRdiiFile() -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    if Frdii.mode as ::core::ffi::c_int == SCRATCH_FILE as ::core::ffi::c_int {
        getTempFileName(&raw mut Frdii.name as *mut ::core::ffi::c_char);
    }
    Frdii.file = fopen(
        &raw mut Frdii.name as *mut ::core::ffi::c_char,
        b"w+b\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Frdii.file.is_null() {
        return FALSE;
    }
    fwrite(
        &raw const FileStamp as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        strlen(&raw const FileStamp as *const ::core::ffi::c_char),
        Frdii.file,
    );
    fwrite(
        &raw mut RdiiStep as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Frdii.file,
    );
    fwrite(
        &raw mut NumRdiiNodes as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Frdii.file,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !(*Node.offset(j as isize)).rdiiInflow.is_null() {
            fwrite(
                &raw mut j as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                1 as size_t,
                Frdii.file,
            );
        }
        j += 1;
    }
    return TRUE;
}
unsafe extern "C" fn getRainfall(mut currentDate: DateTime) {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut rainInterval: ::core::ffi::c_int = 0;
    let mut rainDepth: ::core::ffi::c_double = 0.;
    let mut excessDepth: ::core::ffi::c_double = 0.;
    let mut gageDate: DateTime = 0.;
    month = datetime_monthOfYear(currentDate) - 1 as ::core::ffi::c_int;
    g = 0 as ::core::ffi::c_int;
    while g < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        (*Gage.offset(g as isize)).isCurrent = FALSE;
        g += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
        g = (*UnitHyd.offset(j as isize)).rainGage;
        rainInterval = (*UHGroup.offset(j as isize)).rainInterval;
        while (*UHGroup.offset(j as isize)).gageDate < currentDate {
            gageDate = (*UHGroup.offset(j as isize)).gageDate;
            Adjust.rainFactor =
                Adjust.rain[(datetime_monthOfYear(gageDate) - 1 as ::core::ffi::c_int) as usize];
            if (*Gage.offset(g as isize)).isCurrent == 0 {
                gage_setState(g, gageDate);
                (*Gage.offset(g as isize)).isCurrent = TRUE;
            }
            rainDepth = (*Gage.offset(g as isize)).rainfall * rainInterval as ::core::ffi::c_double
                / 3600.0f64;
            TotalRainVol += rainDepth / UCF(RAINDEPTH as ::core::ffi::c_int)
                * (*UHGroup.offset(j as isize)).area;
            k = 0 as ::core::ffi::c_int;
            while k < 3 as ::core::ffi::c_int {
                excessDepth = applyIA(
                    j,
                    k,
                    gageDate,
                    rainInterval as ::core::ffi::c_double,
                    rainDepth,
                );
                updateDryPeriod(j, k, excessDepth, rainInterval);
                i = (*UHGroup.offset(j as isize)).uh[k as usize].period;
                if i >= (*UHGroup.offset(j as isize)).uh[k as usize].maxPeriods {
                    i = 0 as ::core::ffi::c_int;
                }
                *(*UHGroup.offset(j as isize)).uh[k as usize]
                    .pastRain
                    .offset(i as isize) = excessDepth;
                *(*UHGroup.offset(j as isize)).uh[k as usize]
                    .pastMonth
                    .offset(i as isize) = month as ::core::ffi::c_char;
                (*UHGroup.offset(j as isize)).uh[k as usize].period = i + 1 as ::core::ffi::c_int;
                k += 1;
            }
            (*UHGroup.offset(j as isize)).gageDate =
                datetime_addSeconds(gageDate, rainInterval as ::core::ffi::c_double);
        }
        j += 1;
    }
}
unsafe extern "C" fn applyIA(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut aDate: DateTime,
    mut dt: ::core::ffi::c_double,
    mut rainDepth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut m: ::core::ffi::c_int = 0;
    let mut ia: ::core::ffi::c_double = 0.;
    let mut netRainDepth: ::core::ffi::c_double = 0.;
    m = datetime_monthOfYear(aDate) - 1 as ::core::ffi::c_int;
    ia = (*UnitHyd.offset(j as isize)).iaMax[m as usize][k as usize]
        - (*UHGroup.offset(j as isize)).uh[k as usize].iaUsed;
    ia = if ia >= 0.0f64 { ia } else { 0.0f64 };
    if rainDepth > 0.0f64 {
        netRainDepth = rainDepth - ia;
        netRainDepth = if netRainDepth >= 0.0f64 {
            netRainDepth
        } else {
            0.0f64
        };
        ia = rainDepth - netRainDepth;
        (*UHGroup.offset(j as isize)).uh[k as usize].iaUsed += ia;
    } else {
        (*UHGroup.offset(j as isize)).uh[k as usize].iaUsed -=
            dt / 86400.0f64 * (*UnitHyd.offset(j as isize)).iaRecov[m as usize][k as usize];
        (*UHGroup.offset(j as isize)).uh[k as usize].iaUsed =
            if (*UHGroup.offset(j as isize)).uh[k as usize].iaUsed >= 0.0f64 {
                (*UHGroup.offset(j as isize)).uh[k as usize].iaUsed
            } else {
                0.0f64
            };
        netRainDepth = 0.0f64;
    }
    return netRainDepth;
}
unsafe extern "C" fn updateDryPeriod(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut rainDepth: ::core::ffi::c_double,
    mut rainInterval: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    if rainDepth > 0.0f64 {
        if (*UHGroup.offset(j as isize)).uh[k as usize].drySeconds
            >= (rainInterval * (*UHGroup.offset(j as isize)).uh[k as usize].maxPeriods)
                as ::core::ffi::c_long
        {
            i = 0 as ::core::ffi::c_int;
            while i < (*UHGroup.offset(j as isize)).uh[k as usize].maxPeriods {
                *(*UHGroup.offset(j as isize)).uh[k as usize]
                    .pastRain
                    .offset(i as isize) = 0.0f64;
                i += 1;
            }
            (*UHGroup.offset(j as isize)).uh[k as usize].period = 0 as ::core::ffi::c_int;
        }
        (*UHGroup.offset(j as isize)).uh[k as usize].drySeconds = 0 as ::core::ffi::c_long;
        (*UHGroup.offset(j as isize)).uh[k as usize].hasPastRain = TRUE;
    } else {
        (*UHGroup.offset(j as isize)).uh[k as usize].drySeconds +=
            rainInterval as ::core::ffi::c_long;
        if (*UHGroup.offset(j as isize)).uh[k as usize].drySeconds
            >= (rainInterval * (*UHGroup.offset(j as isize)).uh[k as usize].maxPeriods)
                as ::core::ffi::c_long
        {
            (*UHGroup.offset(j as isize)).uh[k as usize].hasPastRain = FALSE;
        } else {
            (*UHGroup.offset(j as isize)).uh[k as usize].hasPastRain = TRUE;
        }
    };
}
unsafe extern "C" fn getUnitHydRdii(mut currentDate: DateTime) {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut rainInterval: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
        if !((*UHGroup.offset(j as isize)).isUsed == 0) {
            if !(currentDate < (*UHGroup.offset(j as isize)).lastDate) {
                (*UHGroup.offset(j as isize)).lastDate = (*UHGroup.offset(j as isize)).gageDate;
                rainInterval = (*UHGroup.offset(j as isize)).rainInterval;
                (*UHGroup.offset(j as isize)).rdii = 0.0f64;
                k = 0 as ::core::ffi::c_int;
                while k < 3 as ::core::ffi::c_int {
                    if (*UHGroup.offset(j as isize)).uh[k as usize].hasPastRain != 0 {
                        (*UHGroup.offset(j as isize)).rdii += getUnitHydConvol(j, k, rainInterval);
                    }
                    k += 1;
                }
            }
        }
        j += 1;
    }
}
unsafe extern "C" fn getUnitHydConvol(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut rainInterval: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut pMax: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_double = 0.;
    let mut u: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    let mut rdii: ::core::ffi::c_double = 0.;
    let mut uh: *mut TUHData = ::core::ptr::null_mut::<TUHData>();
    rdii = 0.0f64;
    uh = (&raw mut (*UHGroup.offset(j as isize)).uh as *mut TUHData).offset(k as isize)
        as *mut TUHData;
    i = (*uh).period - 1 as ::core::ffi::c_int;
    if i < 0 as ::core::ffi::c_int {
        i = (*uh).maxPeriods - 1 as ::core::ffi::c_int;
    }
    pMax = (*uh).maxPeriods;
    p = 1 as ::core::ffi::c_int;
    while p < pMax {
        v = *(*uh).pastRain.offset(i as isize);
        m = *(*uh).pastMonth.offset(i as isize) as ::core::ffi::c_int;
        if v > 0.0f64 {
            t = (p as ::core::ffi::c_double - 0.5f64) * rainInterval as ::core::ffi::c_double;
            u = getUnitHydOrd(j, m, k, t) * (*UnitHyd.offset(j as isize)).r[m as usize][k as usize];
            rdii += u * v;
        }
        p = p + 1 as ::core::ffi::c_int;
        i = i - 1 as ::core::ffi::c_int;
        if i < 0 as ::core::ffi::c_int {
            i = (*uh).maxPeriods - 1 as ::core::ffi::c_int;
        }
    }
    return rdii;
}
unsafe extern "C" fn getUnitHydOrd(
    mut h: ::core::ffi::c_int,
    mut m: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut t: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut qPeak: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut t1: ::core::ffi::c_double = 0.;
    let mut t2: ::core::ffi::c_double = 0.;
    let mut tBase: ::core::ffi::c_double = 0.;
    tBase = (*UnitHyd.offset(h as isize)).tBase[m as usize][k as usize] as ::core::ffi::c_double;
    if t >= tBase {
        return 0.0f64;
    }
    qPeak = 2.0f64 / tBase * 3600.0f64;
    t1 = (*UnitHyd.offset(h as isize)).tPeak[m as usize][k as usize] as ::core::ffi::c_double;
    t2 = tBase - t1;
    if t <= t1 {
        f = t / t1;
    } else {
        f = 1.0f64 - (t - t1) / t2;
    }
    return (if f >= 0.0f64 { f } else { 0.0f64 }) * qPeak;
}
unsafe extern "C" fn getNodeRdii() -> ::core::ffi::c_int {
    let mut hasRdii: ::core::ffi::c_int = FALSE;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut rdii: ::core::ffi::c_double = 0.;
    n = 0 as ::core::ffi::c_int;
    while n < NumRdiiNodes {
        j = *RdiiNodeIndex.offset(n as isize);
        i = (*(*Node.offset(j as isize)).rdiiInflow).unitHyd;
        rdii = (*UHGroup.offset(i as isize)).rdii * (*(*Node.offset(j as isize)).rdiiInflow).area
            / UCF(RAINFALL as ::core::ffi::c_int);
        if rdii < ZERO_RDII {
            rdii = 0.0f64;
        } else {
            hasRdii = TRUE;
        }
        *RdiiNodeFlow.offset(n as isize) = rdii as ::core::ffi::c_float;
        if rdii > 0.0f64 {
            TotalRdiiVol += rdii * RdiiStep as ::core::ffi::c_double;
        }
        n += 1;
    }
    return hasRdii;
}
unsafe extern "C" fn saveRdiiFlows(mut currentDate: DateTime) {
    fwrite(
        &raw mut currentDate as *const ::core::ffi::c_void,
        ::core::mem::size_of::<DateTime>() as size_t,
        1 as size_t,
        Frdii.file,
    );
    fwrite(
        RdiiNodeFlow as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        NumRdiiNodes as size_t,
        Frdii.file,
    );
}
unsafe extern "C" fn closeRdiiProcessor() {
    if ErrorCode == 0 {
        report_writeRdiiStats(TotalRainVol, TotalRdiiVol);
    }
    freeRdiiMemory();
    if !Frdii.file.is_null() {
        fclose(Frdii.file);
    }
}
unsafe extern "C" fn freeRdiiMemory() {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    if !UHGroup.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
            k = 0 as ::core::ffi::c_int;
            while k < 3 as ::core::ffi::c_int {
                if !(*UHGroup.offset(i as isize)).uh[k as usize]
                    .pastRain
                    .is_null()
                {
                    free(
                        (*UHGroup.offset(i as isize)).uh[k as usize].pastRain
                            as *mut ::core::ffi::c_void,
                    );
                    let ref mut fresh3 = (*UHGroup.offset(i as isize)).uh[k as usize].pastRain;
                    *fresh3 = ::core::ptr::null_mut::<::core::ffi::c_double>();
                }
                if !(*UHGroup.offset(i as isize)).uh[k as usize]
                    .pastMonth
                    .is_null()
                {
                    free(
                        (*UHGroup.offset(i as isize)).uh[k as usize].pastMonth
                            as *mut ::core::ffi::c_void,
                    );
                    let ref mut fresh4 = (*UHGroup.offset(i as isize)).uh[k as usize].pastMonth;
                    *fresh4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                k += 1;
            }
            i += 1;
        }
        if !UHGroup.is_null() {
            free(UHGroup as *mut ::core::ffi::c_void);
            UHGroup = ::core::ptr::null_mut::<TUHGroup>();
        }
    }
    if !RdiiNodeIndex.is_null() {
        free(RdiiNodeIndex as *mut ::core::ffi::c_void);
        RdiiNodeIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !RdiiNodeFlow.is_null() {
        free(RdiiNodeFlow as *mut ::core::ffi::c_void);
        RdiiNodeFlow = ::core::ptr::null_mut::<::core::ffi::c_float>();
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

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
