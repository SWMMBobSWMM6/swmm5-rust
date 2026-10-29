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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn rewind(__stream: *mut FILE);
    fn datetime_strToDate(s: *mut ::core::ffi::c_char, d: *mut DateTime) -> ::core::ffi::c_int;
    fn datetime_strToTime(s: *mut ::core::ffi::c_char, t: *mut DateTime) -> ::core::ffi::c_int;
    fn inlet_readDesignParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn inlet_readUsageParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn infil_readParams(
        m: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut Finp: TFile;
    static mut Title: [[::core::ffi::c_char; 1025]; 3];
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut Nnodes: [::core::ffi::c_int; 5];
    static mut Nlinks: [::core::ffi::c_int; 5];
    static mut InfilModel: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut NumEvents: ::core::ffi::c_int;
    static mut StartDate: DateTime;
    static mut StartTime: DateTime;
    static mut Tseries: *mut TTable;
    static mut Event: *mut TEvent;
    fn project_readOption(
        s1: *mut ::core::ffi::c_char,
        s2: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_addObject(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn report_readOptions(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn report_writeLine(line: *const ::core::ffi::c_char);
    fn report_writeInputErrorMsg(
        k: ::core::ffi::c_int,
        sect: ::core::ffi::c_int,
        line: *mut ::core::ffi::c_char,
        lineCount: ::core::ffi::c_long,
    );
    fn climate_readParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn climate_readEvapParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn climate_readAdjustments(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn snow_readMeltParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn gwater_readAquiferParams(
        aquifer: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn gwater_readGroundwaterParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn gwater_readFlowExpression(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn rdii_readRdiiInflow(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn rdii_readUnitHydParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn landuse_readParams(
        landuse: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn landuse_readPollutParams(
        pollut: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn landuse_readBuildupParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn landuse_readWashoffParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn treatmnt_readExpression(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn gage_readParams(
        gage: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn subcatch_readParams(
        subcatch: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn subcatch_readSubareaParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn subcatch_readLanduseParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn subcatch_readInitBuildup(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn node_readParams(
        node: ::core::ffi::c_int,
        type_0: ::core::ffi::c_int,
        subIndex: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn inflow_readExtInflow(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn inflow_readDwfInflow(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn inflow_readDwfPattern(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn iface_readFileParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn link_readParams(
        link: ::core::ffi::c_int,
        type_0: ::core::ffi::c_int,
        subIndex: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn link_readXsectParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn link_readLossParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn transect_readParams(
        count: *mut ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn transect_validate(j: ::core::ffi::c_int);
    fn street_readParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn controls_init();
    fn controls_addToCount(s: *mut ::core::ffi::c_char);
    fn controls_addVariable(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn controls_addExpression(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn controls_addRuleClause(
        rule: ::core::ffi::c_int,
        keyword: ::core::ffi::c_int,
        Tok_0: *mut *mut ::core::ffi::c_char,
        nTokens: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn table_readCurve(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn table_readTimeseries(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sstrncpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        n: size_t,
    ) -> size_t;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut CurveTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut RuleKeyWords: [*mut ::core::ffi::c_char; 0];
    static mut SectWords: [*mut ::core::ffi::c_char; 0];
    fn lid_readProcParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn lid_readGroupParams(
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub type LinkType = ::core::ffi::c_uint;
pub const OUTLET: LinkType = 4;
pub const WEIR: LinkType = 3;
pub const ORIFICE: LinkType = 2;
pub const PUMP: LinkType = 1;
pub const CONDUIT: LinkType = 0;
pub type CurveType = ::core::ffi::c_uint;
pub const PUMP5_CURVE: CurveType = 11;
pub const PUMP4_CURVE: CurveType = 10;
pub const PUMP3_CURVE: CurveType = 9;
pub const PUMP2_CURVE: CurveType = 8;
pub const PUMP1_CURVE: CurveType = 7;
pub const WEIR_CURVE: CurveType = 6;
pub const SHAPE_CURVE: CurveType = 5;
pub const CONTROL_CURVE: CurveType = 4;
pub const RATING_CURVE: CurveType = 3;
pub const TIDAL_CURVE: CurveType = 2;
pub const DIVERSION_CURVE: CurveType = 1;
pub const STORAGE_CURVE: CurveType = 0;
pub type InputSectionType = ::core::ffi::c_uint;
pub const s_INLET: InputSectionType = 56;
pub const s_INLET_USAGE: InputSectionType = 55;
pub const s_STREET: InputSectionType = 54;
pub const s_EVENT: InputSectionType = 53;
pub const s_ADJUST: InputSectionType = 52;
pub const s_GWF: InputSectionType = 51;
pub const s_LID_USAGE: InputSectionType = 50;
pub const s_LID_CONTROL: InputSectionType = 49;
pub const s_MAP: InputSectionType = 48;
pub const s_PROFILE: InputSectionType = 47;
pub const s_TAG: InputSectionType = 46;
pub const s_BACKDROP: InputSectionType = 45;
pub const s_SYMBOL: InputSectionType = 44;
pub const s_LABEL: InputSectionType = 43;
pub const s_POLYGON: InputSectionType = 42;
pub const s_VERTICES: InputSectionType = 41;
pub const s_COORDINATE: InputSectionType = 40;
pub const s_REPORT: InputSectionType = 39;
pub const s_TIMESERIES: InputSectionType = 38;
pub const s_CURVE: InputSectionType = 37;
pub const s_TREATMENT: InputSectionType = 36;
pub const s_LOADING: InputSectionType = 35;
pub const s_UNITHYD: InputSectionType = 34;
pub const s_RDII: InputSectionType = 33;
pub const s_PATTERN: InputSectionType = 32;
pub const s_DWF: InputSectionType = 31;
pub const s_INFLOW: InputSectionType = 30;
pub const s_COVERAGE: InputSectionType = 29;
pub const s_WASHOFF: InputSectionType = 28;
pub const s_BUILDUP: InputSectionType = 27;
pub const s_LANDUSE: InputSectionType = 26;
pub const s_POLLUTANT: InputSectionType = 25;
pub const s_CONTROL: InputSectionType = 24;
pub const s_LOSSES: InputSectionType = 23;
pub const s_TRANSECT: InputSectionType = 22;
pub const s_XSECTION: InputSectionType = 21;
pub const s_OUTLET: InputSectionType = 20;
pub const s_WEIR: InputSectionType = 19;
pub const s_ORIFICE: InputSectionType = 18;
pub const s_PUMP: InputSectionType = 17;
pub const s_CONDUIT: InputSectionType = 16;
pub const s_DIVIDER: InputSectionType = 15;
pub const s_STORAGE: InputSectionType = 14;
pub const s_OUTFALL: InputSectionType = 13;
pub const s_JUNCTION: InputSectionType = 12;
pub const s_SNOWMELT: InputSectionType = 11;
pub const s_GROUNDWATER: InputSectionType = 10;
pub const s_AQUIFER: InputSectionType = 9;
pub const s_INFIL: InputSectionType = 8;
pub const s_SUBAREA: InputSectionType = 7;
pub const s_SUBCATCH: InputSectionType = 6;
pub const s_EVAP: InputSectionType = 5;
pub const s_TEMP: InputSectionType = 4;
pub const s_RAINGAGE: InputSectionType = 3;
pub const s_FILE: InputSectionType = 2;
pub const s_OPTION: InputSectionType = 1;
pub const s_TITLE: InputSectionType = 0;
pub type DateTime = ::core::ffi::c_double;
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
pub struct TEvent {
    pub start: DateTime,
    pub end: DateTime,
}
pub const ERR_INPUT: ErrorType = 200;
pub const ERR_DUP_NAME: ErrorType = 207;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_DATETIME: ErrorType = 213;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_RULE: ErrorType = 217;
pub const ERR_LINE_LENGTH: ErrorType = 201;
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
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_NAME: ErrorType = 209;
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NONE: ErrorType = 0;
pub const MAXTITLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAXMSG: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXTOKS: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const SEPSTR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b" \t\n\r\0") };
pub const FMT19: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
        *b"\n  Maximum error count exceeded.\0",
    )
};
pub const w_RULE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"RULE\0") };
pub const w_VARIABLE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"VARIABLE\0") };
pub const w_EXPRESSION: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"EXPRESSION\0") };
pub const MAX_NODE_TYPES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MAX_LINK_TYPES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut MAXERRS: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
static mut Tok: [*mut ::core::ffi::c_char; 40] =
    [::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char; 40];
static mut Ntokens: ::core::ffi::c_int = 0;
static mut Mobjects: [::core::ffi::c_int; 18] = [0; 18];
static mut Mnodes: [::core::ffi::c_int; 5] = [0; 5];
static mut Mlinks: [::core::ffi::c_int; 5] = [0; 5];
static mut Mevents: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn input_countObjects() -> ::core::ffi::c_int {
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut wLine: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut tok: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sect: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut newsect: ::core::ffi::c_int = 0;
    let mut errcode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut errsum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut lineCount: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    if ErrorCode != 0 {
        return ErrorCode;
    }
    error_setInpError(
        0 as ::core::ffi::c_int,
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < MAX_OBJ_TYPES as ::core::ffi::c_int {
        Nobjects[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAX_NODE_TYPES {
        Nnodes[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAX_LINK_TYPES {
        Nlinks[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    controls_init();
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finp.file,
    )
    .is_null()
    {
        lineCount += 1;
        sstrncpy(
            &raw mut wLine as *mut ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
            MAXLINE as size_t,
        );
        tok = strtok(&raw mut wLine as *mut ::core::ffi::c_char, SEPSTR.as_ptr());
        if tok.is_null() {
            continue;
        }
        if *tok as ::core::ffi::c_int == ';' as i32 {
            continue;
        }
        if *tok as ::core::ffi::c_int == '[' as i32 {
            newsect = findmatch(tok, &raw mut SectWords as *mut *mut ::core::ffi::c_char);
            if newsect >= 0 as ::core::ffi::c_int {
                sect = newsect;
                continue;
            } else {
                sect = -(1 as ::core::ffi::c_int);
                errcode = ERR_KEYWORD as ::core::ffi::c_int;
            }
        }
        if sect == s_OPTION as ::core::ffi::c_int {
            errcode = readOption(&raw mut line as *mut ::core::ffi::c_char);
        } else if sect >= 0 as ::core::ffi::c_int {
            errcode = addObject(sect, tok);
        }
        if !(errcode != 0) {
            continue;
        }
        report_writeInputErrorMsg(
            errcode,
            sect,
            &raw mut line as *mut ::core::ffi::c_char,
            lineCount,
        );
        errsum += 1;
        if errsum >= MAXERRS {
            break;
        }
    }
    if errsum > 0 as ::core::ffi::c_int {
        ErrorCode = ERR_INPUT as ::core::ffi::c_int;
    }
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn input_readData() -> ::core::ffi::c_int {
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut wLine: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut comment: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sect: ::core::ffi::c_int = 0;
    let mut newsect: ::core::ffi::c_int = 0;
    let mut inperr: ::core::ffi::c_int = 0;
    let mut errsum: ::core::ffi::c_int = 0;
    let mut lineLength: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut lineCount: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    if ErrorCode != 0 {
        return ErrorCode;
    }
    error_setInpError(
        0 as ::core::ffi::c_int,
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < MAX_OBJ_TYPES as ::core::ffi::c_int {
        Mobjects[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAX_NODE_TYPES {
        Mnodes[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAX_LINK_TYPES {
        Mlinks[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    Mevents = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[TSERIES as ::core::ffi::c_int as usize] {
        (*Tseries.offset(i as isize)).lastDate = (StartDate + StartTime) as ::core::ffi::c_double;
        i += 1;
    }
    sect = 0 as ::core::ffi::c_int;
    errsum = 0 as ::core::ffi::c_int;
    rewind(Finp.file);
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finp.file,
    )
    .is_null()
    {
        lineCount += 1;
        sstrncpy(
            &raw mut wLine as *mut ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
            MAXLINE as size_t,
        );
        Ntokens = getTokens(&raw mut wLine as *mut ::core::ffi::c_char);
        if Ntokens == 0 as ::core::ffi::c_int {
            continue;
        }
        if *Tok[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == ';' as i32 {
            continue;
        }
        lineLength = strlen(&raw mut line as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
        if lineLength >= MAXLINE {
            comment = strchr(&raw mut line as *mut ::core::ffi::c_char, ';' as i32);
            if !comment.is_null() {
                lineLength = comment.offset_from(&raw mut line as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as ::core::ffi::c_int;
            }
            if lineLength >= MAXLINE {
                inperr = ERR_LINE_LENGTH as ::core::ffi::c_int;
                report_writeInputErrorMsg(
                    inperr,
                    sect,
                    &raw mut line as *mut ::core::ffi::c_char,
                    lineCount,
                );
                errsum += 1;
            }
        }
        if *Tok[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '[' as i32 {
            newsect = findmatch(
                Tok[0 as ::core::ffi::c_int as usize],
                &raw mut SectWords as *mut *mut ::core::ffi::c_char,
            );
            if newsect >= 0 as ::core::ffi::c_int {
                if sect == s_TRANSECT as ::core::ffi::c_int {
                    transect_validate(
                        Nobjects[TRANSECT as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int,
                    );
                }
                sect = newsect;
            } else {
                inperr = error_setInpError(
                    ERR_KEYWORD as ::core::ffi::c_int,
                    Tok[0 as ::core::ffi::c_int as usize],
                );
                report_writeInputErrorMsg(
                    inperr,
                    sect,
                    &raw mut line as *mut ::core::ffi::c_char,
                    lineCount,
                );
                errsum += 1;
                break;
            }
        } else {
            inperr = parseLine(sect, &raw mut line as *mut ::core::ffi::c_char);
            if inperr > 0 as ::core::ffi::c_int {
                errsum += 1;
                if errsum > MAXERRS {
                    report_writeLine(FMT19.as_ptr());
                } else {
                    report_writeInputErrorMsg(
                        inperr,
                        sect,
                        &raw mut line as *mut ::core::ffi::c_char,
                        lineCount,
                    );
                }
            }
            if errsum > MAXERRS {
                break;
            }
        }
    }
    if errsum > 0 as ::core::ffi::c_int {
        ErrorCode = ERR_INPUT as ::core::ffi::c_int;
    }
    return ErrorCode;
}
unsafe extern "C" fn addObject(
    mut objType: ::core::ffi::c_int,
    mut id: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut errcode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match objType {
        3 => {
            if project_addObject(
                GAGE as ::core::ffi::c_int,
                id,
                Nobjects[GAGE as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[GAGE as ::core::ffi::c_int as usize] += 1;
        }
        6 => {
            if project_addObject(
                SUBCATCH as ::core::ffi::c_int,
                id,
                Nobjects[SUBCATCH as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[SUBCATCH as ::core::ffi::c_int as usize] += 1;
        }
        9 => {
            if project_addObject(
                AQUIFER as ::core::ffi::c_int,
                id,
                Nobjects[AQUIFER as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[AQUIFER as ::core::ffi::c_int as usize] += 1;
        }
        34 => {
            if project_findObject(UNITHYD as ::core::ffi::c_int, id) < 0 as ::core::ffi::c_int {
                if project_addObject(
                    UNITHYD as ::core::ffi::c_int,
                    id,
                    Nobjects[UNITHYD as ::core::ffi::c_int as usize],
                ) == 0
                {
                    errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
                }
                Nobjects[UNITHYD as ::core::ffi::c_int as usize] += 1;
            }
        }
        11 => {
            if project_findObject(SNOWMELT as ::core::ffi::c_int, id) < 0 as ::core::ffi::c_int {
                if project_addObject(
                    SNOWMELT as ::core::ffi::c_int,
                    id,
                    Nobjects[SNOWMELT as ::core::ffi::c_int as usize],
                ) == 0
                {
                    errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
                }
                Nobjects[SNOWMELT as ::core::ffi::c_int as usize] += 1;
            }
        }
        12 => {
            if project_addObject(
                NODE as ::core::ffi::c_int,
                id,
                Nobjects[NODE as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[NODE as ::core::ffi::c_int as usize] += 1;
            Nnodes[JUNCTION as ::core::ffi::c_int as usize] += 1;
        }
        13 => {
            if project_addObject(
                NODE as ::core::ffi::c_int,
                id,
                Nobjects[NODE as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[NODE as ::core::ffi::c_int as usize] += 1;
            Nnodes[OUTFALL as ::core::ffi::c_int as usize] += 1;
        }
        14 => {
            if project_addObject(
                NODE as ::core::ffi::c_int,
                id,
                Nobjects[NODE as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[NODE as ::core::ffi::c_int as usize] += 1;
            Nnodes[STORAGE as ::core::ffi::c_int as usize] += 1;
        }
        15 => {
            if project_addObject(
                NODE as ::core::ffi::c_int,
                id,
                Nobjects[NODE as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[NODE as ::core::ffi::c_int as usize] += 1;
            Nnodes[DIVIDER as ::core::ffi::c_int as usize] += 1;
        }
        16 => {
            if project_addObject(
                LINK as ::core::ffi::c_int,
                id,
                Nobjects[LINK as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[LINK as ::core::ffi::c_int as usize] += 1;
            Nlinks[CONDUIT as ::core::ffi::c_int as usize] += 1;
        }
        17 => {
            if project_addObject(
                LINK as ::core::ffi::c_int,
                id,
                Nobjects[LINK as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[LINK as ::core::ffi::c_int as usize] += 1;
            Nlinks[PUMP as ::core::ffi::c_int as usize] += 1;
        }
        18 => {
            if project_addObject(
                LINK as ::core::ffi::c_int,
                id,
                Nobjects[LINK as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[LINK as ::core::ffi::c_int as usize] += 1;
            Nlinks[ORIFICE as ::core::ffi::c_int as usize] += 1;
        }
        19 => {
            if project_addObject(
                LINK as ::core::ffi::c_int,
                id,
                Nobjects[LINK as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[LINK as ::core::ffi::c_int as usize] += 1;
            Nlinks[WEIR as ::core::ffi::c_int as usize] += 1;
        }
        20 => {
            if project_addObject(
                LINK as ::core::ffi::c_int,
                id,
                Nobjects[LINK as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[LINK as ::core::ffi::c_int as usize] += 1;
            Nlinks[OUTLET as ::core::ffi::c_int as usize] += 1;
        }
        25 => {
            if project_addObject(
                POLLUT as ::core::ffi::c_int,
                id,
                Nobjects[POLLUT as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[POLLUT as ::core::ffi::c_int as usize] += 1;
        }
        26 => {
            if project_addObject(
                LANDUSE as ::core::ffi::c_int,
                id,
                Nobjects[LANDUSE as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[LANDUSE as ::core::ffi::c_int as usize] += 1;
        }
        32 => {
            if project_findObject(TIMEPATTERN as ::core::ffi::c_int, id) < 0 as ::core::ffi::c_int {
                if project_addObject(
                    TIMEPATTERN as ::core::ffi::c_int,
                    id,
                    Nobjects[TIMEPATTERN as ::core::ffi::c_int as usize],
                ) == 0
                {
                    errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
                }
                Nobjects[TIMEPATTERN as ::core::ffi::c_int as usize] += 1;
            }
        }
        37 => {
            if project_findObject(CURVE as ::core::ffi::c_int, id) < 0 as ::core::ffi::c_int {
                if project_addObject(
                    CURVE as ::core::ffi::c_int,
                    id,
                    Nobjects[CURVE as ::core::ffi::c_int as usize],
                ) == 0
                {
                    errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
                }
                Nobjects[CURVE as ::core::ffi::c_int as usize] += 1;
                id = strtok(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    SEPSTR.as_ptr(),
                );
                if findmatch(id, &raw mut CurveTypeWords as *mut *mut ::core::ffi::c_char)
                    == SHAPE_CURVE as ::core::ffi::c_int
                {
                    Nobjects[SHAPE as ::core::ffi::c_int as usize] += 1;
                }
            }
        }
        38 => {
            if project_findObject(TSERIES as ::core::ffi::c_int, id) < 0 as ::core::ffi::c_int {
                if project_addObject(
                    TSERIES as ::core::ffi::c_int,
                    id,
                    Nobjects[TSERIES as ::core::ffi::c_int as usize],
                ) == 0
                {
                    errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
                }
                Nobjects[TSERIES as ::core::ffi::c_int as usize] += 1;
            }
        }
        24 => {
            if match_0(id, w_RULE.as_ptr() as *mut ::core::ffi::c_char) != 0 {
                Nobjects[CONTROL as ::core::ffi::c_int as usize] += 1;
            } else {
                controls_addToCount(id);
            }
        }
        22 => {
            if match_0(
                id,
                b"X1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
            {
                id = strtok(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    SEPSTR.as_ptr(),
                );
                if !id.is_null() {
                    if project_addObject(
                        TRANSECT as ::core::ffi::c_int,
                        id,
                        Nobjects[TRANSECT as ::core::ffi::c_int as usize],
                    ) == 0
                    {
                        errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
                    }
                    Nobjects[TRANSECT as ::core::ffi::c_int as usize] += 1;
                }
            }
        }
        49 => {
            if project_findObject(LID as ::core::ffi::c_int, id) < 0 as ::core::ffi::c_int {
                if project_addObject(
                    LID as ::core::ffi::c_int,
                    id,
                    Nobjects[LID as ::core::ffi::c_int as usize],
                ) == 0
                {
                    errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
                }
                Nobjects[LID as ::core::ffi::c_int as usize] += 1;
            }
        }
        53 => {
            NumEvents += 1;
        }
        54 => {
            if project_addObject(
                STREET as ::core::ffi::c_int,
                id,
                Nobjects[STREET as ::core::ffi::c_int as usize],
            ) == 0
            {
                errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
            }
            Nobjects[STREET as ::core::ffi::c_int as usize] += 1;
        }
        56 => {
            if project_findObject(INLET as ::core::ffi::c_int, id) < 0 as ::core::ffi::c_int {
                if project_addObject(
                    INLET as ::core::ffi::c_int,
                    id,
                    Nobjects[INLET as ::core::ffi::c_int as usize],
                ) == 0
                {
                    errcode = error_setInpError(ERR_DUP_NAME as ::core::ffi::c_int, id);
                }
                Nobjects[INLET as ::core::ffi::c_int as usize] += 1;
            }
        }
        _ => {}
    }
    return errcode;
}
unsafe extern "C" fn parseLine(
    mut sect: ::core::ffi::c_int,
    mut line: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    match sect {
        0 => return readTitle(line),
        3 => {
            j = Mobjects[GAGE as ::core::ffi::c_int as usize];
            err = gage_readParams(j, &raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
            Mobjects[GAGE as ::core::ffi::c_int as usize] += 1;
            return err;
        }
        4 => {
            return climate_readParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        5 => {
            return climate_readEvapParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        52 => {
            return climate_readAdjustments(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        6 => {
            j = Mobjects[SUBCATCH as ::core::ffi::c_int as usize];
            err = subcatch_readParams(j, &raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
            Mobjects[SUBCATCH as ::core::ffi::c_int as usize] += 1;
            return err;
        }
        7 => {
            return subcatch_readSubareaParams(
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        8 => {
            return infil_readParams(
                InfilModel,
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        9 => {
            j = Mobjects[AQUIFER as ::core::ffi::c_int as usize];
            err =
                gwater_readAquiferParams(j, &raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
            Mobjects[AQUIFER as ::core::ffi::c_int as usize] += 1;
            return err;
        }
        10 => {
            return gwater_readGroundwaterParams(
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        51 => {
            return gwater_readFlowExpression(
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        11 => {
            return snow_readMeltParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        12 => return readNode(JUNCTION as ::core::ffi::c_int),
        13 => return readNode(OUTFALL as ::core::ffi::c_int),
        14 => return readNode(STORAGE as ::core::ffi::c_int),
        15 => return readNode(DIVIDER as ::core::ffi::c_int),
        16 => return readLink(CONDUIT as ::core::ffi::c_int),
        17 => return readLink(PUMP as ::core::ffi::c_int),
        18 => return readLink(ORIFICE as ::core::ffi::c_int),
        19 => return readLink(WEIR as ::core::ffi::c_int),
        20 => return readLink(OUTLET as ::core::ffi::c_int),
        21 => {
            return link_readXsectParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        22 => {
            return transect_readParams(
                (&raw mut Mobjects as *mut ::core::ffi::c_int)
                    .offset(TRANSECT as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_int,
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        23 => {
            return link_readLossParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        25 => {
            j = Mobjects[POLLUT as ::core::ffi::c_int as usize];
            err =
                landuse_readPollutParams(j, &raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
            Mobjects[POLLUT as ::core::ffi::c_int as usize] += 1;
            return err;
        }
        26 => {
            j = Mobjects[LANDUSE as ::core::ffi::c_int as usize];
            err = landuse_readParams(j, &raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
            Mobjects[LANDUSE as ::core::ffi::c_int as usize] += 1;
            return err;
        }
        27 => {
            return landuse_readBuildupParams(
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        28 => {
            return landuse_readWashoffParams(
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        29 => {
            return subcatch_readLanduseParams(
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        30 => {
            return inflow_readExtInflow(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        31 => {
            return inflow_readDwfInflow(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        32 => {
            return inflow_readDwfPattern(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        33 => {
            return rdii_readRdiiInflow(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        34 => {
            return rdii_readUnitHydParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        35 => {
            return subcatch_readInitBuildup(
                &raw mut Tok as *mut *mut ::core::ffi::c_char,
                Ntokens,
            );
        }
        36 => {
            return treatmnt_readExpression(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        37 => {
            return table_readCurve(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        38 => {
            return table_readTimeseries(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        24 => return readControl(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens),
        39 => {
            return report_readOptions(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        2 => {
            return iface_readFileParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        49 => {
            return lid_readProcParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        50 => {
            return lid_readGroupParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        53 => return readEvent(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens),
        54 => {
            return street_readParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        56 => {
            return inlet_readDesignParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        55 => {
            return inlet_readUsageParams(&raw mut Tok as *mut *mut ::core::ffi::c_char, Ntokens);
        }
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn readControl(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut keyword: ::core::ffi::c_int = 0;
    if ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        w_VARIABLE.as_ptr() as *mut ::core::ffi::c_char,
    ) != 0
    {
        return controls_addVariable(tok, ntoks);
    }
    if match_0(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        w_EXPRESSION.as_ptr() as *mut ::core::ffi::c_char,
    ) != 0
    {
        return controls_addExpression(tok, ntoks);
    }
    keyword = findmatch(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        &raw mut RuleKeyWords as *mut *mut ::core::ffi::c_char,
    );
    if keyword < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if keyword == 0 as ::core::ffi::c_int {
        if project_addObject(
            CONTROL as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
            Mobjects[CONTROL as ::core::ffi::c_int as usize],
        ) == 0
        {
            return error_setInpError(
                ERR_DUP_NAME as ::core::ffi::c_int,
                Tok[1 as ::core::ffi::c_int as usize],
            );
        }
        Mobjects[CONTROL as ::core::ffi::c_int as usize] += 1;
    }
    index = Mobjects[CONTROL as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int;
    if index < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_RULE as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return controls_addRuleClause(
        index,
        keyword,
        &raw mut Tok as *mut *mut ::core::ffi::c_char,
        Ntokens,
    );
}
unsafe extern "C" fn readOption(mut line: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    Ntokens = getTokens(line);
    if Ntokens < 2 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return project_readOption(
        Tok[0 as ::core::ffi::c_int as usize],
        Tok[1 as ::core::ffi::c_int as usize],
    );
}
unsafe extern "C" fn readTitle(mut line: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MAXTITLE {
        if strlen(
            &raw mut *(&raw mut Title as *mut [::core::ffi::c_char; 1025]).offset(i as isize)
                as *mut ::core::ffi::c_char,
        ) == 0 as size_t
        {
            n = strlen(line) as ::core::ffi::c_int;
            if *line.offset((n - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 10 as ::core::ffi::c_int
            {
                *line.offset((n - 1 as ::core::ffi::c_int) as isize) =
                    ' ' as i32 as ::core::ffi::c_char;
            }
            sstrncpy(
                &raw mut *(&raw mut Title as *mut [::core::ffi::c_char; 1025]).offset(i as isize)
                    as *mut ::core::ffi::c_char,
                line,
                MAXMSG as size_t,
            );
            break;
        } else {
            i += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readNode(mut type_0: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = Mobjects[NODE as ::core::ffi::c_int as usize];
    let mut k: ::core::ffi::c_int = Mnodes[type_0 as usize];
    let mut err: ::core::ffi::c_int = node_readParams(
        j,
        type_0,
        k,
        &raw mut Tok as *mut *mut ::core::ffi::c_char,
        Ntokens,
    );
    Mobjects[NODE as ::core::ffi::c_int as usize] += 1;
    Mnodes[type_0 as usize] += 1;
    return err;
}
unsafe extern "C" fn readLink(mut type_0: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = Mobjects[LINK as ::core::ffi::c_int as usize];
    let mut k: ::core::ffi::c_int = Mlinks[type_0 as usize];
    let mut err: ::core::ffi::c_int = link_readParams(
        j,
        type_0,
        k,
        &raw mut Tok as *mut *mut ::core::ffi::c_char,
        Ntokens,
    );
    Mobjects[LINK as ::core::ffi::c_int as usize] += 1;
    Mlinks[type_0 as usize] += 1;
    return err;
}
unsafe extern "C" fn readEvent(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut x: [DateTime; 4] = [0.; 4];
    if ntoks < 4 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if datetime_strToDate(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut DateTime).offset(0 as ::core::ffi::c_int as isize) as *mut DateTime,
    ) == 0
    {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if datetime_strToTime(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut DateTime).offset(1 as ::core::ffi::c_int as isize) as *mut DateTime,
    ) == 0
    {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    if datetime_strToDate(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut DateTime).offset(2 as ::core::ffi::c_int as isize) as *mut DateTime,
    ) == 0
    {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if datetime_strToTime(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut DateTime).offset(3 as ::core::ffi::c_int as isize) as *mut DateTime,
    ) == 0
    {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    (*Event.offset(Mevents as isize)).start =
        x[0 as ::core::ffi::c_int as usize] + x[1 as ::core::ffi::c_int as usize];
    (*Event.offset(Mevents as isize)).end =
        x[2 as ::core::ffi::c_int as usize] + x[3 as ::core::ffi::c_int as usize];
    if (*Event.offset(Mevents as isize)).start >= (*Event.offset(Mevents as isize)).end {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            b" - start date exceeds end date\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    Mevents += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findmatch(
    mut s: *mut ::core::ffi::c_char,
    mut keyword: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*keyword.offset(i as isize)).is_null() {
        if match_0(s, *keyword.offset(i as isize)) != 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[export_name = "match"]
pub unsafe extern "C" fn match_0(
    mut str: *mut ::core::ffi::c_char,
    mut substr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    if *substr.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    k = 0 as ::core::ffi::c_int;
    while *str.offset(k as isize) != 0 {
        if *str.offset(k as isize) as ::core::ffi::c_int != ' ' as i32 {
            break;
        }
        k += 1;
    }
    i = k;
    j = 0 as ::core::ffi::c_int;
    while *substr.offset(j as isize) != 0 {
        if *str.offset(i as isize) == 0
            || (if *str.offset(i as isize) as ::core::ffi::c_int >= 'a' as i32
                && *str.offset(i as isize) as ::core::ffi::c_int <= 'z' as i32
            {
                *str.offset(i as isize) as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
            } else {
                *str.offset(i as isize) as ::core::ffi::c_int
            }) != (if *substr.offset(j as isize) as ::core::ffi::c_int >= 'a' as i32
                && *substr.offset(j as isize) as ::core::ffi::c_int <= 'z' as i32
            {
                *substr.offset(j as isize) as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
            } else {
                *substr.offset(j as isize) as ::core::ffi::c_int
            })
        {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
        j += 1;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getInt(
    mut s: *mut ::core::ffi::c_char,
    mut y: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_double = 0.;
    if getDouble(s, &raw mut x) != 0 {
        if x < 0.0f64 {
            x -= 0.01f64;
        } else {
            x += 0.01f64;
        }
        *y = crate::ccast::f64_to_i32(x);
        return 1 as ::core::ffi::c_int;
    }
    *y = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getFloat(
    mut s: *mut ::core::ffi::c_char,
    mut y: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *y = strtod(s, &raw mut endptr) as ::core::ffi::c_float;
    if *endptr as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getDouble(
    mut s: *mut ::core::ffi::c_char,
    mut y: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *y = strtod(s, &raw mut endptr);
    if *endptr as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn getTokens(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut c: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    n = 0 as ::core::ffi::c_int;
    while n < MAXTOKS {
        Tok[n as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
        n += 1;
    }
    n = 0 as ::core::ffi::c_int;
    c = strchr(s, ';' as i32);
    if !c.is_null() {
        *c = '\0' as i32 as ::core::ffi::c_char;
    }
    len = strlen(s) as ::core::ffi::c_int;
    while len > 0 as ::core::ffi::c_int && n < MAXTOKS {
        m = strcspn(s, SEPSTR.as_ptr()) as ::core::ffi::c_int;
        if m == 0 as ::core::ffi::c_int {
            s = s.offset(1);
        } else {
            if *s as ::core::ffi::c_int == '"' as i32 {
                s = s.offset(1);
                len -= 1;
                m = strcspn(s, b"\"\n\0" as *const u8 as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int;
            }
            *s.offset(m as isize) = '\0' as i32 as ::core::ffi::c_char;
            Tok[n as usize] = s;
            n += 1;
            s = s.offset((m + 1 as ::core::ffi::c_int) as isize);
        }
        len -= m + 1 as ::core::ffi::c_int;
    }
    return n;
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
