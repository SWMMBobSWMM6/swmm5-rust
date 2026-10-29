#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn datetime_monthOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_dayOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_dayOfWeek(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_strToDate(s: *mut ::core::ffi::c_char, d: *mut DateTime) -> ::core::ffi::c_int;
    fn datetime_strToTime(s: *mut ::core::ffi::c_char, t: *mut DateTime) -> ::core::ffi::c_int;
    fn mathexpr_create(
        s: *mut ::core::ffi::c_char,
        getVar: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>,
    ) -> *mut MathExpr;
    fn mathexpr_eval(
        expr: *mut MathExpr,
        getVal: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_double>,
    ) -> ::core::ffi::c_double;
    fn mathexpr_delete(expr: *mut MathExpr);
    static mut RptFlags: TRptFlags;
    static mut ElapsedTime: ::core::ffi::c_double;
    static mut Gage: *mut TGage;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Curve: *mut TTable;
    static mut Tseries: *mut TTable;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn report_writeControlAction(
        aDate: DateTime,
        linkID: *mut ::core::ffi::c_char,
        value: ::core::ffi::c_double,
        ruleID: *mut ::core::ffi::c_char,
    );
    fn report_writeWarningMsg(msg: *mut ::core::ffi::c_char, id: *mut ::core::ffi::c_char);
    fn gage_getPastRain(gage: ::core::ffi::c_int, hrs: ::core::ffi::c_int)
        -> ::core::ffi::c_double;
    fn link_getVelocity(
        link: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn table_lookup(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
    fn sstrcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        destsize: size_t,
    ) -> size_t;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut RuleKeyWords: [*mut ::core::ffi::c_char; 0];
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
pub type LinkType = ::core::ffi::c_uint;
pub const OUTLET: LinkType = 4;
pub const WEIR: LinkType = 3;
pub const ORIFICE: LinkType = 2;
pub const PUMP: LinkType = 1;
pub const CONDUIT: LinkType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TRptFlags {
    pub disabled: ::core::ffi::c_char,
    pub input: ::core::ffi::c_char,
    pub subcatchments: ::core::ffi::c_char,
    pub nodes: ::core::ffi::c_char,
    pub links: ::core::ffi::c_char,
    pub continuity: ::core::ffi::c_char,
    pub flowStats: ::core::ffi::c_char,
    pub controls: ::core::ffi::c_char,
    pub averages: ::core::ffi::c_char,
    pub linesPerPage: ::core::ffi::c_int,
}
pub const ERR_MEMORY: ErrorType = 101;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TExpression {
    pub expression: *mut MathExpr,
    pub name: [::core::ffi::c_char; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TNamedVariable {
    pub variable: TVariable,
    pub name: [::core::ffi::c_char; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TVariable {
    pub object: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub attribute: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TRule {
    pub ID: *mut ::core::ffi::c_char,
    pub priority: ::core::ffi::c_double,
    pub firstPremise: *mut TPremise,
    pub lastPremise: *mut TPremise,
    pub thenActions: *mut TAction,
    pub elseActions: *mut TAction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAction {
    pub rule: ::core::ffi::c_int,
    pub link: ::core::ffi::c_int,
    pub attribute: ::core::ffi::c_int,
    pub curve: ::core::ffi::c_int,
    pub tseries: ::core::ffi::c_int,
    pub value: ::core::ffi::c_double,
    pub kp: ::core::ffi::c_double,
    pub ki: ::core::ffi::c_double,
    pub kd: ::core::ffi::c_double,
    pub e1: ::core::ffi::c_double,
    pub e2: ::core::ffi::c_double,
    pub next: *mut TAction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPremise {
    pub type_0: ::core::ffi::c_int,
    pub exprIndex: ::core::ffi::c_int,
    pub lhsVar: TVariable,
    pub rhsVar: TVariable,
    pub relation: ::core::ffi::c_int,
    pub value: ::core::ffi::c_double,
    pub next: *mut TPremise,
}
pub const r_PRIORITY: RuleState = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TActionList {
    pub action: *mut TAction,
    pub next: *mut TActionList,
}
pub const ERR_KEYWORD: ErrorType = 205;
pub const r_DAYOFYEAR: RuleAttrib = 18;
pub const r_MONTH: RuleAttrib = 20;
pub const r_DAY: RuleAttrib = 19;
pub const r_CLOCKTIME: RuleAttrib = 17;
pub const r_DATE: RuleAttrib = 16;
pub const r_TIME: RuleAttrib = 15;
pub const r_SETTING: RuleAttrib = 9;
pub const r_FLOW: RuleAttrib = 5;
pub const r_OUTLET: RuleObject = 7;
pub const r_WEIR: RuleObject = 6;
pub const r_ORIFICE: RuleObject = 5;
pub const r_STATUS: RuleAttrib = 8;
pub const r_PUMP: RuleObject = 4;
pub const r_VELOCITY: RuleAttrib = 12;
pub const r_SLOPE: RuleAttrib = 11;
pub const r_LENGTH: RuleAttrib = 10;
pub const r_FULLDEPTH: RuleAttrib = 7;
pub const r_FULLFLOW: RuleAttrib = 6;
pub const r_DEPTH: RuleAttrib = 0;
pub const r_CONDUIT: RuleObject = 3;
pub const r_LINK: RuleObject = 2;
pub const r_TIMECLOSED: RuleAttrib = 14;
pub const r_TIMEOPEN: RuleAttrib = 13;
pub const r_INFLOW: RuleAttrib = 4;
pub const r_VOLUME: RuleAttrib = 3;
pub const r_HEAD: RuleAttrib = 2;
pub const r_MAXDEPTH: RuleAttrib = 1;
pub const r_NODE: RuleObject = 1;
pub const r_GAGE: RuleObject = 0;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_MATH_EXPR: ErrorType = 233;
pub const ERR_RULE: ErrorType = 217;
pub const ERR_NUMBER: ErrorType = 211;
pub const r_ELSE: RuleState = 5;
pub const r_THEN: RuleState = 4;
pub const r_PID: RuleSetting = 2;
pub const r_TIMESERIES: RuleSetting = 1;
pub const r_CURVE: RuleSetting = 0;
pub const r_IF: RuleState = 1;
pub const r_OR: RuleState = 3;
pub const ERR_DATETIME: ErrorType = 213;
pub const r_AND: RuleState = 2;
pub const r_RULE: RuleState = 0;
pub const GE: RuleRelation = 5;
pub const GT: RuleRelation = 4;
pub const LE: RuleRelation = 3;
pub const LT: RuleRelation = 2;
pub const NE: RuleRelation = 1;
pub const EQ: RuleRelation = 0;
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
pub const ERR_SUBCATCH_OUTLET: ErrorType = 108;
pub const ERR_TIMESTEP: ErrorType = 107;
pub const ERR_ODE_SOLVER: ErrorType = 105;
pub const ERR_KINWAVE: ErrorType = 103;
pub const ERR_NONE: ErrorType = 0;
pub type RuleState = ::core::ffi::c_uint;
pub const r_ERROR: RuleState = 9;
pub const r_EXPRESSION: RuleState = 8;
pub const r_VARIABLE: RuleState = 7;
pub type RuleObject = ::core::ffi::c_uint;
pub const r_SIMULATION: RuleObject = 8;
pub type RuleAttrib = ::core::ffi::c_uint;
pub type RuleRelation = ::core::ffi::c_uint;
pub type RuleSetting = ::core::ffi::c_uint;
pub const r_NUMERIC: RuleSetting = 3;
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TINY: ::core::ffi::c_double = 1.0E-6f64;
pub const MISSING: ::core::ffi::c_double = -1.0E10f64;
pub const WARN11: [::core::ffi::c_char; 52] = unsafe {
    ::core::mem::transmute::<[u8; 52], [::core::ffi::c_char; 52]>(
        *b"WARNING 11: non-matching attributes in Control Rule\0",
    )
};
pub const w_VARIABLE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"VARIABLE\0") };
pub const w_EXPRESSION: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"EXPRESSION\0") };
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const MAXPASTRAIN: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const MAXVARNAME: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut ObjectWords: [*mut ::core::ffi::c_char; 10] = [
    b"GAGE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"NODE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"LINK\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"CONDUIT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"PUMP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ORIFICE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"WEIR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"OUTLET\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SIMULATION\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut AttribWords: [*mut ::core::ffi::c_char; 22] = [
    b"DEPTH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"MAXDEPTH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"HEAD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"VOLUME\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"INFLOW\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"FLOW\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"FULLFLOW\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"FULLDEPTH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"STATUS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SETTING\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"LENGTH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SLOPE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"VELOCITY\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TIMEOPEN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TIMECLOSED\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TIME\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"DATE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"CLOCKTIME\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"DAYOFYEAR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"DAY\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"MONTH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut RelOpWords: [*mut ::core::ffi::c_char; 7] = [
    b"=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"<>\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"<\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"<=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b">\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b">=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut StatusWords: [*mut ::core::ffi::c_char; 3] = [
    b"OFF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ON\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut ConduitWords: [*mut ::core::ffi::c_char; 3] = [
    b"CLOSED\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"OPEN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut SettingTypeWords: [*mut ::core::ffi::c_char; 4] = [
    b"CURVE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TIMESERIES\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"PID\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut IntensityWord: *mut ::core::ffi::c_char =
    b"INTENSITY\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut Rules: *mut TRule = ::core::ptr::null::<TRule>() as *mut TRule;
#[no_mangle]
pub static mut ActionList: *mut TActionList =
    ::core::ptr::null::<TActionList>() as *mut TActionList;
#[no_mangle]
pub static mut InputState: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut RuleCount: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ControlValue: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut SetPoint: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut CurrentDate: DateTime = 0.;
#[no_mangle]
pub static mut CurrentTime: DateTime = 0.;
#[no_mangle]
pub static mut VariableCount: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ExpressionCount: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut CurrentVariable: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut CurrentExpression: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut NamedVariable: *mut TNamedVariable =
    ::core::ptr::null::<TNamedVariable>() as *mut TNamedVariable;
#[no_mangle]
pub static mut Expression: *mut TExpression =
    ::core::ptr::null::<TExpression>() as *mut TExpression;
#[no_mangle]
pub unsafe extern "C" fn controls_init() {
    Rules = ::core::ptr::null_mut::<TRule>();
    NamedVariable = ::core::ptr::null_mut::<TNamedVariable>();
    Expression = ::core::ptr::null_mut::<TExpression>();
    RuleCount = 0 as ::core::ffi::c_int;
    VariableCount = 0 as ::core::ffi::c_int;
    ExpressionCount = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn controls_addToCount(mut s: *mut ::core::ffi::c_char) {
    if match_0(s, w_VARIABLE.as_ptr() as *mut ::core::ffi::c_char) != 0 {
        VariableCount += 1;
    } else if match_0(s, w_EXPRESSION.as_ptr() as *mut ::core::ffi::c_char) != 0 {
        ExpressionCount += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn controls_create(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    ActionList = ::core::ptr::null_mut::<TActionList>();
    InputState = r_PRIORITY as ::core::ffi::c_int;
    RuleCount = n;
    if RuleCount > 0 as ::core::ffi::c_int {
        Rules = calloc(
            RuleCount as size_t,
            ::core::mem::size_of::<TRule>() as size_t,
        ) as *mut TRule;
        if Rules.is_null() {
            return ERR_MEMORY as ::core::ffi::c_int;
        }
        r = 0 as ::core::ffi::c_int;
        while r < RuleCount {
            let ref mut fresh0 = (*Rules.offset(r as isize)).ID;
            *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let ref mut fresh1 = (*Rules.offset(r as isize)).firstPremise;
            *fresh1 = ::core::ptr::null_mut::<TPremise>();
            let ref mut fresh2 = (*Rules.offset(r as isize)).lastPremise;
            *fresh2 = ::core::ptr::null_mut::<TPremise>();
            let ref mut fresh3 = (*Rules.offset(r as isize)).thenActions;
            *fresh3 = ::core::ptr::null_mut::<TAction>();
            let ref mut fresh4 = (*Rules.offset(r as isize)).elseActions;
            *fresh4 = ::core::ptr::null_mut::<TAction>();
            (*Rules.offset(r as isize)).priority = 0.0f64;
            r += 1;
        }
    }
    CurrentVariable = -(1 as ::core::ffi::c_int);
    CurrentExpression = -(1 as ::core::ffi::c_int);
    if VariableCount > 0 as ::core::ffi::c_int {
        NamedVariable = calloc(
            VariableCount as size_t,
            ::core::mem::size_of::<TNamedVariable>() as size_t,
        ) as *mut TNamedVariable;
        if NamedVariable.is_null() {
            return ERR_MEMORY as ::core::ffi::c_int;
        }
    }
    if ExpressionCount > 0 as ::core::ffi::c_int {
        Expression = calloc(
            ExpressionCount as size_t,
            ::core::mem::size_of::<TExpression>() as size_t,
        ) as *mut TExpression;
        if Expression.is_null() {
            return ERR_MEMORY as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn controls_delete() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < ExpressionCount {
        mathexpr_delete((*Expression.offset(i as isize)).expression);
        let ref mut fresh5 = (*Expression.offset(i as isize)).expression;
        *fresh5 = ::core::ptr::null_mut::<MathExpr>();
        i += 1;
    }
    if !Expression.is_null() {
        free(Expression as *mut ::core::ffi::c_void);
        Expression = ::core::ptr::null_mut::<TExpression>();
    }
    if !NamedVariable.is_null() {
        free(NamedVariable as *mut ::core::ffi::c_void);
        NamedVariable = ::core::ptr::null_mut::<TNamedVariable>();
    }
    if RuleCount == 0 as ::core::ffi::c_int {
        return;
    }
    deleteActionList();
    deleteRules();
}
#[no_mangle]
pub unsafe extern "C" fn controls_addVariable(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut nToks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut v1: TVariable = TVariable {
        object: 0,
        index: 0,
        attribute: 0,
    };
    let mut k: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    CurrentVariable += 1;
    if nToks < 5 as ::core::ffi::c_int {
        return ERR_ITEMS as ::core::ffi::c_int;
    }
    if findExactMatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut AttribWords as *mut *mut ::core::ffi::c_char,
    ) >= 0 as ::core::ffi::c_int
    {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    if match_0(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        b"=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
    {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if match_0(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        b"SIMULATION\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && nToks < 6 as ::core::ffi::c_int
    {
        return ERR_ITEMS as ::core::ffi::c_int;
    }
    k = 3 as ::core::ffi::c_int;
    err = getPremiseVariable(tok, nToks, &raw mut k, &raw mut v1);
    if err > 0 as ::core::ffi::c_int {
        return err;
    }
    k = CurrentVariable;
    (*NamedVariable.offset(k as isize)).variable = v1;
    sstrncpy(
        &raw mut (*NamedVariable.offset(k as isize)).name as *mut ::core::ffi::c_char,
        *tok.offset(1 as ::core::ffi::c_int as isize),
        MAXVARNAME as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn controls_addExpression(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut nToks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut s: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut expr: *mut MathExpr = ::core::ptr::null_mut::<MathExpr>();
    CurrentExpression += 1;
    if nToks < 4 as ::core::ffi::c_int {
        return ERR_ITEMS as ::core::ffi::c_int;
    }
    k = CurrentExpression;
    let ref mut fresh6 = (*Expression.offset(k as isize)).expression;
    *fresh6 = ::core::ptr::null_mut::<MathExpr>();
    sstrncpy(
        &raw mut (*Expression.offset(k as isize)).name as *mut ::core::ffi::c_char,
        *tok.offset(1 as ::core::ffi::c_int as isize),
        MAXVARNAME as size_t,
    );
    sstrncpy(
        &raw mut s as *mut ::core::ffi::c_char,
        *tok.offset(3 as ::core::ffi::c_int as isize),
        MAXLINE as size_t,
    );
    i = 4 as ::core::ffi::c_int;
    while i < nToks {
        sstrcat(
            &raw mut s as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            MAXLINE as size_t,
        );
        sstrcat(
            &raw mut s as *mut ::core::ffi::c_char,
            *tok.offset(i as isize),
            MAXLINE as size_t,
        );
        i += 1;
    }
    expr = mathexpr_create(
        &raw mut s as *mut ::core::ffi::c_char,
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
    let ref mut fresh7 = (*Expression.offset(k as isize)).expression;
    *fresh7 = expr;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getVariableIndex(
    mut varName: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < VariableCount {
        if match_0(
            varName,
            &raw mut (*NamedVariable.offset(i as isize)).name as *mut ::core::ffi::c_char,
        ) != 0
        {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn getNamedVariableValue(
    mut varIndex: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    return getVariableValue((*NamedVariable.offset(varIndex as isize)).variable);
}
#[no_mangle]
pub unsafe extern "C" fn getExpressionIndex(
    mut exprName: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < ExpressionCount {
        if match_0(
            exprName,
            &raw mut (*Expression.offset(i as isize)).name as *mut ::core::ffi::c_char,
        ) != 0
        {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn controls_addRuleClause(
    mut r: ::core::ffi::c_int,
    mut keyword: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut nToks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match keyword {
        0 => {
            if (*Rules.offset(r as isize)).ID.is_null() {
                let ref mut fresh8 = (*Rules.offset(r as isize)).ID;
                *fresh8 = project_findID(
                    CONTROL as ::core::ffi::c_int,
                    *tok.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            InputState = r_RULE as ::core::ffi::c_int;
            if nToks > 2 as ::core::ffi::c_int {
                return ERR_RULE as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        1 => {
            if InputState != r_RULE as ::core::ffi::c_int {
                return ERR_RULE as ::core::ffi::c_int;
            }
            InputState = r_IF as ::core::ffi::c_int;
            return addPremise(r, r_AND as ::core::ffi::c_int, tok, nToks);
        }
        2 => {
            if InputState == r_IF as ::core::ffi::c_int {
                return addPremise(r, r_AND as ::core::ffi::c_int, tok, nToks);
            } else if InputState == r_THEN as ::core::ffi::c_int
                || InputState == r_ELSE as ::core::ffi::c_int
            {
                return addAction(r, tok, nToks);
            } else {
                return ERR_RULE as ::core::ffi::c_int;
            }
        }
        3 => {
            if InputState != r_IF as ::core::ffi::c_int {
                return ERR_RULE as ::core::ffi::c_int;
            }
            return addPremise(r, r_OR as ::core::ffi::c_int, tok, nToks);
        }
        4 => {
            if InputState != r_IF as ::core::ffi::c_int {
                return ERR_RULE as ::core::ffi::c_int;
            }
            InputState = r_THEN as ::core::ffi::c_int;
            return addAction(r, tok, nToks);
        }
        5 => {
            if InputState != r_THEN as ::core::ffi::c_int {
                return ERR_RULE as ::core::ffi::c_int;
            }
            InputState = r_ELSE as ::core::ffi::c_int;
            return addAction(r, tok, nToks);
        }
        6 => {
            if InputState != r_THEN as ::core::ffi::c_int
                && InputState != r_ELSE as ::core::ffi::c_int
            {
                return ERR_RULE as ::core::ffi::c_int;
            }
            InputState = r_PRIORITY as ::core::ffi::c_int;
            if getDouble(
                *tok.offset(1 as ::core::ffi::c_int as isize),
                &raw mut (*Rules.offset(r as isize)).priority,
            ) == 0
            {
                return ERR_NUMBER as ::core::ffi::c_int;
            }
            if nToks > 2 as ::core::ffi::c_int {
                return ERR_RULE as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn controls_evaluate(
    mut currentTime: DateTime,
    mut elapsedTime: DateTime,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_int = 0;
    let mut p: *mut TPremise = ::core::ptr::null_mut::<TPremise>();
    let mut a: *mut TAction = ::core::ptr::null_mut::<TAction>();
    CurrentDate = floor(currentTime as ::core::ffi::c_double) as DateTime;
    CurrentTime = (currentTime as ::core::ffi::c_double
        - floor(currentTime as ::core::ffi::c_double)) as DateTime;
    ElapsedTime = elapsedTime as ::core::ffi::c_double;
    if RuleCount == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    clearActionList();
    r = 0 as ::core::ffi::c_int;
    while r < RuleCount {
        result = TRUE;
        p = (*Rules.offset(r as isize)).firstPremise;
        while !p.is_null() {
            if (*p).type_0 == r_OR as ::core::ffi::c_int {
                if result == FALSE {
                    result = evaluatePremise(p, tStep);
                }
            } else {
                if result == FALSE {
                    break;
                }
                result = evaluatePremise(p, tStep);
            }
            p = (*p).next;
        }
        if result == TRUE {
            a = (*Rules.offset(r as isize)).thenActions;
        } else {
            a = (*Rules.offset(r as isize)).elseActions;
        }
        while !a.is_null() {
            updateActionValue(a, currentTime, tStep);
            updateActionList(a);
            a = (*a).next;
        }
        r += 1;
    }
    if !ActionList.is_null() {
        return executeActionList(currentTime);
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn addPremise(
    mut r: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut nToks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut relation: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut value: ::core::ffi::c_double = MISSING;
    let mut p: *mut TPremise = ::core::ptr::null_mut::<TPremise>();
    let mut v1: TVariable = TVariable {
        object: 0,
        index: 0,
        attribute: 0,
    };
    let mut v2: TVariable = TVariable {
        object: 0,
        index: 0,
        attribute: 0,
    };
    let mut obj: ::core::ffi::c_int = 0;
    let mut exprIndex: ::core::ffi::c_int = 0;
    let mut varIndex: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if nToks < 4 as ::core::ffi::c_int {
        return ERR_ITEMS as ::core::ffi::c_int;
    }
    v1.attribute = -(1 as ::core::ffi::c_int);
    v1.object = -(1 as ::core::ffi::c_int);
    v1.index = -(1 as ::core::ffi::c_int);
    n = 1 as ::core::ffi::c_int;
    exprIndex = getExpressionIndex(*tok.offset(1 as ::core::ffi::c_int as isize));
    if exprIndex < 0 as ::core::ffi::c_int {
        varIndex = getVariableIndex(*tok.offset(n as isize));
        if varIndex >= 0 as ::core::ffi::c_int {
            v1 = (*NamedVariable.offset(varIndex as isize)).variable;
        } else {
            err = getPremiseVariable(tok, nToks, &raw mut n, &raw mut v1);
            if err > 0 as ::core::ffi::c_int {
                return err;
            }
        }
    }
    n += 1;
    if n >= nToks {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    relation = findExactMatch(
        *tok.offset(n as isize),
        &raw mut RelOpWords as *mut *mut ::core::ffi::c_char,
    );
    if relation < 0 as ::core::ffi::c_int {
        return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, *tok.offset(n as isize));
    }
    v2.attribute = -(1 as ::core::ffi::c_int);
    v2.object = -(1 as ::core::ffi::c_int);
    v2.index = -(1 as ::core::ffi::c_int);
    n += 1;
    if n >= nToks {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    varIndex = getVariableIndex(*tok.offset(n as isize));
    if varIndex >= 0 as ::core::ffi::c_int {
        v2 = (*NamedVariable.offset(varIndex as isize)).variable;
    } else {
        obj = findmatch(
            *tok.offset(n as isize),
            &raw mut ObjectWords as *mut *mut ::core::ffi::c_char,
        );
        if obj >= 0 as ::core::ffi::c_int {
            err = getPremiseVariable(tok, nToks, &raw mut n, &raw mut v2);
            if err > 0 as ::core::ffi::c_int {
                return ERR_RULE as ::core::ffi::c_int;
            }
            if exprIndex < 0 as ::core::ffi::c_int && v1.attribute != v2.attribute {
                report_writeWarningMsg(
                    WARN11.as_ptr() as *mut ::core::ffi::c_char,
                    (*Rules.offset(r as isize)).ID,
                );
            }
        } else {
            err = getPremiseValue(*tok.offset(n as isize), v1.attribute, &raw mut value);
            if err > 0 as ::core::ffi::c_int {
                return err;
            }
        }
    }
    n += 1;
    if n < nToks
        && findmatch(
            *tok.offset(n as isize),
            &raw mut RuleKeyWords as *mut *mut ::core::ffi::c_char,
        ) >= 0 as ::core::ffi::c_int
    {
        return ERR_RULE as ::core::ffi::c_int;
    }
    p = malloc(::core::mem::size_of::<TPremise>() as size_t) as *mut TPremise;
    if p.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    (*p).type_0 = type_0;
    (*p).exprIndex = exprIndex;
    (*p).lhsVar = v1;
    (*p).rhsVar = v2;
    (*p).relation = relation;
    (*p).value = value;
    (*p).next = ::core::ptr::null_mut::<TPremise>();
    if (*Rules.offset(r as isize)).firstPremise.is_null() {
        let ref mut fresh11 = (*Rules.offset(r as isize)).firstPremise;
        *fresh11 = p;
    } else {
        let ref mut fresh12 = (*(*Rules.offset(r as isize)).lastPremise).next;
        *fresh12 = p;
    }
    let ref mut fresh13 = (*Rules.offset(r as isize)).lastPremise;
    *fresh13 = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getPremiseVariable(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut nToks: ::core::ffi::c_int,
    mut k: *mut ::core::ffi::c_int,
    mut v: *mut TVariable,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = *k;
    let mut object: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut index: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut obj: ::core::ffi::c_int = 0;
    let mut attrib: ::core::ffi::c_int = 0;
    obj = findmatch(
        *tok.offset(n as isize),
        &raw mut ObjectWords as *mut *mut ::core::ffi::c_char,
    );
    if obj < 0 as ::core::ffi::c_int {
        return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, *tok.offset(n as isize));
    }
    n += 1;
    if n >= nToks {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    match obj {
        0 => {
            index = project_findObject(GAGE as ::core::ffi::c_int, *tok.offset(n as isize));
            if index < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_NAME as ::core::ffi::c_int, *tok.offset(n as isize));
            }
            object = r_GAGE as ::core::ffi::c_int;
        }
        1 => {
            index = project_findObject(NODE as ::core::ffi::c_int, *tok.offset(n as isize));
            if index < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_NAME as ::core::ffi::c_int, *tok.offset(n as isize));
            }
            object = r_NODE as ::core::ffi::c_int;
        }
        2 | 3 | 4 | 5 | 6 | 7 => {
            index = project_findObject(LINK as ::core::ffi::c_int, *tok.offset(n as isize));
            if index < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_NAME as ::core::ffi::c_int, *tok.offset(n as isize));
            }
            object = r_LINK as ::core::ffi::c_int;
        }
        _ => {
            n -= 1;
        }
    }
    n += 1;
    if n >= nToks {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if object == r_GAGE as ::core::ffi::c_int {
        attrib = getGageAttrib(*tok.offset(n as isize));
    } else {
        attrib = findmatch(
            *tok.offset(n as isize),
            &raw mut AttribWords as *mut *mut ::core::ffi::c_char,
        );
    }
    if attrib < 0 as ::core::ffi::c_int {
        return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, *tok.offset(n as isize));
    }
    if !(obj == r_GAGE as ::core::ffi::c_int) {
        if obj == r_NODE as ::core::ffi::c_int {
            match attrib {
                0 | 1 | 2 | 3 | 4 => {}
                _ => {
                    return error_setInpError(
                        ERR_KEYWORD as ::core::ffi::c_int,
                        *tok.offset(n as isize),
                    );
                }
            }
        } else if !(object == r_LINK as ::core::ffi::c_int
            && index >= 0 as ::core::ffi::c_int
            && (attrib == r_TIMEOPEN as ::core::ffi::c_int
                || attrib == r_TIMECLOSED as ::core::ffi::c_int))
        {
            if obj == r_LINK as ::core::ffi::c_int || obj == r_CONDUIT as ::core::ffi::c_int {
                match attrib {
                    8 | 0 | 6 | 7 | 5 | 10 | 11 | 12 => {}
                    _ => {
                        return error_setInpError(
                            ERR_KEYWORD as ::core::ffi::c_int,
                            *tok.offset(n as isize),
                        );
                    }
                }
            } else if obj == r_PUMP as ::core::ffi::c_int {
                match attrib {
                    5 | 9 | 8 => {}
                    _ => {
                        return error_setInpError(
                            ERR_KEYWORD as ::core::ffi::c_int,
                            *tok.offset(n as isize),
                        );
                    }
                }
            } else if obj == r_ORIFICE as ::core::ffi::c_int
                || obj == r_WEIR as ::core::ffi::c_int
                || obj == r_OUTLET as ::core::ffi::c_int
            {
                match attrib {
                    5 | 9 => {}
                    _ => {
                        return error_setInpError(
                            ERR_KEYWORD as ::core::ffi::c_int,
                            *tok.offset(n as isize),
                        );
                    }
                }
            } else {
                match attrib {
                    15 | 16 | 17 | 19 | 20 | 18 => {}
                    _ => {
                        return error_setInpError(
                            ERR_KEYWORD as ::core::ffi::c_int,
                            *tok.offset(n as isize),
                        );
                    }
                }
            }
        }
    }
    (*v).object = object;
    (*v).index = index;
    (*v).attribute = attrib;
    *k = n;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getGageAttrib(mut token: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut attrib: ::core::ffi::c_int = 0;
    if match_0(token, IntensityWord) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    attrib = atoi(token);
    if attrib < 1 as ::core::ffi::c_int || attrib > MAXPASTRAIN {
        return -(1 as ::core::ffi::c_int);
    }
    return attrib;
}
#[no_mangle]
pub unsafe extern "C" fn getPremiseValue(
    mut token: *mut ::core::ffi::c_char,
    mut attrib: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut strDate: [::core::ffi::c_char; 25] = [0; 25];
    match attrib {
        8 => {
            *value = findmatch(token, &raw mut StatusWords as *mut *mut ::core::ffi::c_char)
                as ::core::ffi::c_double;
            if *value < 0.0f64 {
                *value = findmatch(
                    token,
                    &raw mut ConduitWords as *mut *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_double;
            }
            if *value < 0.0f64 {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, token);
            }
        }
        15 | 17 | 13 | 14 => {
            if datetime_strToTime(token, value as *mut DateTime) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, token);
            }
        }
        16 => {
            if datetime_strToDate(token, value as *mut DateTime) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, token);
            }
        }
        19 => {
            if getDouble(token, value) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, token);
            }
            if *value < 1.0f64 || *value > 7.0f64 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, token);
            }
        }
        20 => {
            if getDouble(token, value) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, token);
            }
            if *value < 1.0f64 || *value > 12.0f64 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, token);
            }
        }
        18 => {
            sstrncpy(
                &raw mut strDate as *mut ::core::ffi::c_char,
                token,
                6 as size_t,
            );
            sstrcat(
                &raw mut strDate as *mut ::core::ffi::c_char,
                b"/1947\0" as *const u8 as *const ::core::ffi::c_char,
                25 as size_t,
            );
            if datetime_strToDate(
                &raw mut strDate as *mut ::core::ffi::c_char,
                value as *mut DateTime,
            ) != 0
            {
                *value = datetime_dayOfYear(*value) as ::core::ffi::c_double;
            } else if getDouble(token, value) == 0
                || *value < 1 as ::core::ffi::c_int as ::core::ffi::c_double
                || *value > 365 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, token);
            }
        }
        _ => {
            if getDouble(token, value) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, token);
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addAction(
    mut r: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut nToks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut obj: ::core::ffi::c_int = 0;
    let mut link: ::core::ffi::c_int = 0;
    let mut attrib: ::core::ffi::c_int = 0;
    let mut curve: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut tseries: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut n: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut values: [::core::ffi::c_double; 3] = [1.0f64, 0.0f64, 0.0f64];
    let mut a: *mut TAction = ::core::ptr::null_mut::<TAction>();
    if nToks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    obj = findmatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut ObjectWords as *mut *mut ::core::ffi::c_char,
    );
    if obj != r_LINK as ::core::ffi::c_int
        && obj != r_CONDUIT as ::core::ffi::c_int
        && obj != r_PUMP as ::core::ffi::c_int
        && obj != r_ORIFICE as ::core::ffi::c_int
        && obj != r_WEIR as ::core::ffi::c_int
        && obj != r_OUTLET as ::core::ffi::c_int
    {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    link = project_findObject(
        LINK as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    if link < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    match obj {
        3 => {
            if (*Link.offset(link as isize)).type_0 != CONDUIT as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(2 as ::core::ffi::c_int as isize),
                );
            }
        }
        4 => {
            if (*Link.offset(link as isize)).type_0 != PUMP as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(2 as ::core::ffi::c_int as isize),
                );
            }
        }
        5 => {
            if (*Link.offset(link as isize)).type_0 != ORIFICE as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(2 as ::core::ffi::c_int as isize),
                );
            }
        }
        6 => {
            if (*Link.offset(link as isize)).type_0 != WEIR as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(2 as ::core::ffi::c_int as isize),
                );
            }
        }
        7 => {
            if (*Link.offset(link as isize)).type_0 != OUTLET as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(2 as ::core::ffi::c_int as isize),
                );
            }
        }
        _ => {}
    }
    attrib = findmatch(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        &raw mut AttribWords as *mut *mut ::core::ffi::c_char,
    );
    if attrib < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    if obj == r_CONDUIT as ::core::ffi::c_int {
        if attrib == r_STATUS as ::core::ffi::c_int {
            values[0 as ::core::ffi::c_int as usize] = findmatch(
                *tok.offset(5 as ::core::ffi::c_int as isize),
                &raw mut ConduitWords as *mut *mut ::core::ffi::c_char,
            ) as ::core::ffi::c_double;
            if values[0 as ::core::ffi::c_int as usize] < 0.0f64 {
                return error_setInpError(
                    ERR_KEYWORD as ::core::ffi::c_int,
                    *tok.offset(5 as ::core::ffi::c_int as isize),
                );
            }
        } else {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(3 as ::core::ffi::c_int as isize),
            );
        }
    } else if obj == r_PUMP as ::core::ffi::c_int {
        if attrib == r_STATUS as ::core::ffi::c_int {
            values[0 as ::core::ffi::c_int as usize] = findmatch(
                *tok.offset(5 as ::core::ffi::c_int as isize),
                &raw mut StatusWords as *mut *mut ::core::ffi::c_char,
            ) as ::core::ffi::c_double;
            if values[0 as ::core::ffi::c_int as usize] < 0.0f64 {
                return error_setInpError(
                    ERR_KEYWORD as ::core::ffi::c_int,
                    *tok.offset(5 as ::core::ffi::c_int as isize),
                );
            }
        } else if attrib == r_SETTING as ::core::ffi::c_int {
            err = setActionSetting(
                tok,
                nToks,
                &raw mut curve,
                &raw mut tseries,
                &raw mut attrib,
                &raw mut values as *mut ::core::ffi::c_double,
            );
            if err > 0 as ::core::ffi::c_int {
                return err;
            }
        } else {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(3 as ::core::ffi::c_int as isize),
            );
        }
    } else if obj == r_ORIFICE as ::core::ffi::c_int
        || obj == r_WEIR as ::core::ffi::c_int
        || obj == r_OUTLET as ::core::ffi::c_int
    {
        if attrib == r_SETTING as ::core::ffi::c_int {
            err = setActionSetting(
                tok,
                nToks,
                &raw mut curve,
                &raw mut tseries,
                &raw mut attrib,
                &raw mut values as *mut ::core::ffi::c_double,
            );
            if err > 0 as ::core::ffi::c_int {
                return err;
            }
            if attrib == r_SETTING as ::core::ffi::c_int
                && (values[0 as ::core::ffi::c_int as usize] < 0.0f64
                    || values[0 as ::core::ffi::c_int as usize] > 1.0f64)
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(5 as ::core::ffi::c_int as isize),
                );
            }
        } else {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(3 as ::core::ffi::c_int as isize),
            );
        }
    } else {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n = 6 as ::core::ffi::c_int;
    if curve >= 0 as ::core::ffi::c_int || tseries >= 0 as ::core::ffi::c_int {
        n = 7 as ::core::ffi::c_int;
    }
    if attrib == r_PID as ::core::ffi::c_int {
        n = 9 as ::core::ffi::c_int;
    }
    if n < nToks
        && findmatch(
            *tok.offset(n as isize),
            &raw mut RuleKeyWords as *mut *mut ::core::ffi::c_char,
        ) >= 0 as ::core::ffi::c_int
    {
        return ERR_RULE as ::core::ffi::c_int;
    }
    a = malloc(::core::mem::size_of::<TAction>() as size_t) as *mut TAction;
    if a.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    (*a).rule = r;
    (*a).link = link;
    (*a).attribute = attrib;
    (*a).curve = curve;
    (*a).tseries = tseries;
    (*a).value = values[0 as ::core::ffi::c_int as usize];
    if attrib == r_PID as ::core::ffi::c_int {
        (*a).kp = values[0 as ::core::ffi::c_int as usize];
        (*a).ki = values[1 as ::core::ffi::c_int as usize];
        (*a).kd = values[2 as ::core::ffi::c_int as usize];
        (*a).e1 = 0.0f64;
        (*a).e2 = 0.0f64;
    }
    if InputState == r_THEN as ::core::ffi::c_int {
        (*a).next = (*Rules.offset(r as isize)).thenActions;
        let ref mut fresh9 = (*Rules.offset(r as isize)).thenActions;
        *fresh9 = a;
    } else {
        (*a).next = (*Rules.offset(r as isize)).elseActions;
        let ref mut fresh10 = (*Rules.offset(r as isize)).elseActions;
        *fresh10 = a;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setActionSetting(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut nToks: ::core::ffi::c_int,
    mut curve: *mut ::core::ffi::c_int,
    mut tseries: *mut ::core::ffi::c_int,
    mut attrib: *mut ::core::ffi::c_int,
    mut values: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    if nToks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    k = findmatch(
        *tok.offset(5 as ::core::ffi::c_int as isize),
        &raw mut SettingTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if k >= 0 as ::core::ffi::c_int && nToks < 7 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    match k {
        0 => {
            m = project_findObject(
                CURVE as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(6 as ::core::ffi::c_int as isize),
                );
            }
            *curve = m;
        }
        1 => {
            m = project_findObject(
                TSERIES as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(6 as ::core::ffi::c_int as isize),
                );
            }
            *tseries = m;
            (*Tseries.offset(m as isize)).refersTo = CONTROL as ::core::ffi::c_int;
        }
        2 => {
            if nToks < 9 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            m = 6 as ::core::ffi::c_int;
            while m <= 8 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset(m as isize),
                    values.offset((m - 6 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset(m as isize),
                    );
                }
                m += 1;
            }
            *attrib = r_PID as ::core::ffi::c_int;
        }
        _ => {
            if getDouble(
                *tok.offset(5 as ::core::ffi::c_int as isize),
                values.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(5 as ::core::ffi::c_int as isize),
                );
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn updateActionValue(
    mut a: *mut TAction,
    mut currentTime: DateTime,
    mut dt: ::core::ffi::c_double,
) {
    if (*a).curve >= 0 as ::core::ffi::c_int {
        (*a).value = table_lookup(
            Curve.offset((*a).curve as isize) as *mut TTable,
            ControlValue,
        );
    } else if (*a).tseries >= 0 as ::core::ffi::c_int {
        (*a).value = table_tseriesLookup(
            Tseries.offset((*a).tseries as isize) as *mut TTable,
            currentTime as ::core::ffi::c_double,
            TRUE as ::core::ffi::c_char,
        );
    } else if (*a).attribute == r_PID as ::core::ffi::c_int {
        (*a).value = getPIDSetting(a, dt);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getPIDSetting(
    mut a: *mut TAction,
    mut dt: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut e0: ::core::ffi::c_double = 0.;
    let mut setting: ::core::ffi::c_double = 0.;
    let mut p: ::core::ffi::c_double = 0.;
    let mut i: ::core::ffi::c_double = 0.;
    let mut d: ::core::ffi::c_double = 0.;
    let mut update: ::core::ffi::c_double = 0.;
    let mut tolerance: ::core::ffi::c_double = 0.0001f64;
    dt *= 1440.0f64;
    e0 = SetPoint - ControlValue;
    if fabs(e0) > TINY {
        if SetPoint != 0.0f64 {
            e0 = e0 / SetPoint;
        } else {
            e0 = e0 / ControlValue;
        }
    }
    if fabs(e0 - (*a).e1) < tolerance {
        (*a).e2 = 0.0f64;
        (*a).e1 = 0.0f64;
    }
    p = e0 - (*a).e1;
    if (*a).ki == 0.0f64 {
        i = 0.0f64;
    } else {
        i = e0 * dt / (*a).ki;
    }
    d = (*a).kd * (e0 - 2.0f64 * (*a).e1 + (*a).e2) / dt;
    update = (*a).kp * (p + i + d);
    if fabs(update) < tolerance {
        update = 0.0f64;
    }
    setting = (*Link.offset((*a).link as isize)).targetSetting + update;
    (*a).e2 = (*a).e1;
    (*a).e1 = e0;
    if setting < 0.0f64 {
        setting = 0.0f64;
    }
    if (*Link.offset((*a).link as isize)).type_0 != PUMP as ::core::ffi::c_int && setting > 1.0f64 {
        setting = 1.0f64;
    }
    return setting;
}
#[no_mangle]
pub unsafe extern "C" fn updateActionList(mut a: *mut TAction) {
    let mut listItem: *mut TActionList = ::core::ptr::null_mut::<TActionList>();
    let mut a1: *mut TAction = ::core::ptr::null_mut::<TAction>();
    let mut priority: ::core::ffi::c_double = (*Rules.offset((*a).rule as isize)).priority;
    listItem = ActionList;
    while !listItem.is_null() {
        a1 = (*listItem).action;
        if a1.is_null() {
            break;
        }
        if (*a1).link == (*a).link {
            if priority > (*Rules.offset((*a1).rule as isize)).priority {
                (*listItem).action = a;
            }
            return;
        }
        listItem = (*listItem).next;
    }
    if listItem.is_null() {
        listItem = malloc(::core::mem::size_of::<TActionList>() as size_t) as *mut TActionList;
        (*listItem).next = ActionList;
        ActionList = listItem;
    }
    (*listItem).action = a;
}
#[no_mangle]
pub unsafe extern "C" fn executeActionList(mut currentTime: DateTime) -> ::core::ffi::c_int {
    let mut listItem: *mut TActionList = ::core::ptr::null_mut::<TActionList>();
    let mut nextItem: *mut TActionList = ::core::ptr::null_mut::<TActionList>();
    let mut a1: *mut TAction = ::core::ptr::null_mut::<TAction>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    listItem = ActionList;
    while !listItem.is_null() {
        a1 = (*listItem).action;
        if a1.is_null() {
            break;
        }
        if (*a1).link >= 0 as ::core::ffi::c_int {
            if (*Link.offset((*a1).link as isize)).targetSetting != (*a1).value {
                (*Link.offset((*a1).link as isize)).targetSetting = (*a1).value;
                if RptFlags.controls as ::core::ffi::c_int != 0
                    && (*a1).curve < 0 as ::core::ffi::c_int
                    && (*a1).tseries < 0 as ::core::ffi::c_int
                    && (*a1).attribute != r_PID as ::core::ffi::c_int
                {
                    report_writeControlAction(
                        currentTime,
                        (*Link.offset((*a1).link as isize)).ID,
                        (*a1).value,
                        (*Rules.offset((*a1).rule as isize)).ID,
                    );
                }
                count += 1;
            }
        }
        nextItem = (*listItem).next;
        listItem = nextItem;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn evaluatePremise(
    mut p: *mut TPremise,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut lhsValue: ::core::ffi::c_double = 0.;
    let mut rhsValue: ::core::ffi::c_double = 0.;
    let mut result: ::core::ffi::c_int = FALSE;
    if (*p).exprIndex >= 0 as ::core::ffi::c_int {
        lhsValue = mathexpr_eval(
            (*Expression.offset((*p).exprIndex as isize)).expression,
            Some(
                getNamedVariableValue
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_double,
            ),
        );
    } else {
        lhsValue = getVariableValue((*p).lhsVar);
    }
    if (*p).value == MISSING {
        rhsValue = getVariableValue((*p).rhsVar);
    } else {
        rhsValue = (*p).value;
    }
    if lhsValue == MISSING || rhsValue == MISSING {
        return FALSE;
    }
    match (*p).lhsVar.attribute {
        15 | 17 => return compareTimes(lhsValue, (*p).relation, rhsValue, tStep / 2.0f64),
        13 | 14 => {
            result = compareTimes(lhsValue, (*p).relation, rhsValue, tStep / 2.0f64);
            ControlValue = lhsValue * 24.0f64;
            return result;
        }
        _ => return compareValues(lhsValue, (*p).relation, rhsValue),
    };
}
#[no_mangle]
pub unsafe extern "C" fn getVariableValue(mut v: TVariable) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut j: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if v.object == r_GAGE as ::core::ffi::c_int {
        return getRainValue(v);
    }
    if v.object == r_NODE as ::core::ffi::c_int {
        i = v.index;
    }
    if v.object == r_LINK as ::core::ffi::c_int {
        j = v.index;
    }
    match v.attribute {
        15 => return ElapsedTime,
        16 => return CurrentDate as ::core::ffi::c_double,
        17 => return CurrentTime as ::core::ffi::c_double,
        19 => return datetime_dayOfWeek(CurrentDate) as ::core::ffi::c_double,
        20 => return datetime_monthOfYear(CurrentDate) as ::core::ffi::c_double,
        18 => return datetime_dayOfYear(CurrentDate) as ::core::ffi::c_double,
        8 => {
            if j < 0 as ::core::ffi::c_int
                || (*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int
                    && (*Link.offset(j as isize)).type_0 != PUMP as ::core::ffi::c_int
            {
                return MISSING;
            } else {
                return (*Link.offset(j as isize)).setting;
            }
        }
        9 => {
            if j < 0 as ::core::ffi::c_int
                || (*Link.offset(j as isize)).type_0 != PUMP as ::core::ffi::c_int
                    && (*Link.offset(j as isize)).type_0 != ORIFICE as ::core::ffi::c_int
                    && (*Link.offset(j as isize)).type_0 != WEIR as ::core::ffi::c_int
            {
                return MISSING;
            } else {
                return (*Link.offset(j as isize)).setting;
            }
        }
        5 => {
            if j < 0 as ::core::ffi::c_int {
                return MISSING;
            } else {
                return (*Link.offset(j as isize)).direction as ::core::ffi::c_int
                    as ::core::ffi::c_double
                    * (*Link.offset(j as isize)).newFlow
                    * UCF(FLOW as ::core::ffi::c_int);
            }
        }
        6 | 7 | 12 | 10 | 11 => {
            if j < 0 as ::core::ffi::c_int {
                return MISSING;
            } else if (*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int {
                return MISSING;
            }
            match v.attribute {
                6 => {
                    return (*Link.offset(j as isize)).qFull * UCF(FLOW as ::core::ffi::c_int);
                }
                7 => {
                    return (*Link.offset(j as isize)).xsect.yFull
                        * UCF(LENGTH as ::core::ffi::c_int);
                }
                12 => {
                    return link_getVelocity(
                        j,
                        (*Link.offset(j as isize)).newFlow,
                        (*Link.offset(j as isize)).newDepth,
                    ) * UCF(LENGTH as ::core::ffi::c_int);
                }
                10 => {
                    return (*Conduit.offset((*Link.offset(j as isize)).subIndex as isize)).length
                        * UCF(LENGTH as ::core::ffi::c_int);
                }
                11 => {
                    return (*Conduit.offset((*Link.offset(j as isize)).subIndex as isize)).slope;
                }
                _ => return MISSING,
            }
        }
        0 => {
            if j >= 0 as ::core::ffi::c_int {
                return (*Link.offset(j as isize)).newDepth * UCF(LENGTH as ::core::ffi::c_int);
            } else if i >= 0 as ::core::ffi::c_int {
                return (*Node.offset(i as isize)).newDepth * UCF(LENGTH as ::core::ffi::c_int);
            } else {
                return MISSING;
            }
        }
        1 => {
            if i >= 0 as ::core::ffi::c_int {
                return (*Node.offset(i as isize)).fullDepth * UCF(LENGTH as ::core::ffi::c_int);
            } else {
                return MISSING;
            }
        }
        2 => {
            if i < 0 as ::core::ffi::c_int {
                return MISSING;
            }
            return ((*Node.offset(i as isize)).newDepth + (*Node.offset(i as isize)).invertElev)
                * UCF(LENGTH as ::core::ffi::c_int);
        }
        3 => {
            if i < 0 as ::core::ffi::c_int {
                return MISSING;
            }
            return (*Node.offset(i as isize)).newVolume * UCF(VOLUME as ::core::ffi::c_int);
        }
        4 => {
            if i < 0 as ::core::ffi::c_int {
                return MISSING;
            } else {
                return (*Node.offset(i as isize)).newLatFlow * UCF(FLOW as ::core::ffi::c_int);
            }
        }
        13 => {
            if j < 0 as ::core::ffi::c_int {
                return MISSING;
            }
            if (*Link.offset(j as isize)).setting <= 0.0f64 {
                return MISSING;
            }
            return CurrentDate as ::core::ffi::c_double + CurrentTime as ::core::ffi::c_double
                - (*Link.offset(j as isize)).timeLastSet;
        }
        14 => {
            if j < 0 as ::core::ffi::c_int {
                return MISSING;
            }
            if (*Link.offset(j as isize)).setting > 0.0f64 {
                return MISSING;
            }
            return CurrentDate as ::core::ffi::c_double + CurrentTime as ::core::ffi::c_double
                - (*Link.offset(j as isize)).timeLastSet;
        }
        _ => return MISSING,
    };
}
#[no_mangle]
pub unsafe extern "C" fn getRainValue(mut v: TVariable) -> ::core::ffi::c_double {
    if v.index < 0 as ::core::ffi::c_int {
        return MISSING;
    } else if (*Gage.offset(v.index as isize)).isUsed == FALSE {
        return 0.0f64;
    } else if v.attribute == 0 as ::core::ffi::c_int {
        return (*Gage.offset(v.index as isize)).rainfall;
    } else {
        return gage_getPastRain(v.index, v.attribute);
    };
}
#[no_mangle]
pub unsafe extern "C" fn compareTimes(
    mut lhsValue: ::core::ffi::c_double,
    mut relation: ::core::ffi::c_int,
    mut rhsValue: ::core::ffi::c_double,
    mut halfStep: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if relation == EQ as ::core::ffi::c_int {
        if lhsValue >= rhsValue - halfStep && lhsValue < rhsValue + halfStep {
            return TRUE;
        }
        return FALSE;
    } else if relation == NE as ::core::ffi::c_int {
        if lhsValue < rhsValue - halfStep || lhsValue >= rhsValue + halfStep {
            return TRUE;
        }
        return FALSE;
    } else {
        return compareValues(lhsValue, relation, rhsValue);
    };
}
#[no_mangle]
pub unsafe extern "C" fn compareValues(
    mut lhsValue: ::core::ffi::c_double,
    mut relation: ::core::ffi::c_int,
    mut rhsValue: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    SetPoint = rhsValue;
    ControlValue = lhsValue;
    match relation {
        0 => {
            if lhsValue == rhsValue {
                return TRUE;
            }
        }
        1 => {
            if lhsValue != rhsValue {
                return TRUE;
            }
        }
        2 => {
            if lhsValue < rhsValue {
                return TRUE;
            }
        }
        3 => {
            if lhsValue <= rhsValue {
                return TRUE;
            }
        }
        4 => {
            if lhsValue > rhsValue {
                return TRUE;
            }
        }
        5 => {
            if lhsValue >= rhsValue {
                return TRUE;
            }
        }
        _ => {}
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn clearActionList() {
    let mut listItem: *mut TActionList = ::core::ptr::null_mut::<TActionList>();
    listItem = ActionList;
    while !listItem.is_null() {
        (*listItem).action = ::core::ptr::null_mut::<TAction>();
        listItem = (*listItem).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn deleteActionList() {
    let mut listItem: *mut TActionList = ::core::ptr::null_mut::<TActionList>();
    let mut nextItem: *mut TActionList = ::core::ptr::null_mut::<TActionList>();
    listItem = ActionList;
    while !listItem.is_null() {
        nextItem = (*listItem).next;
        free(listItem as *mut ::core::ffi::c_void);
        listItem = nextItem;
    }
    ActionList = ::core::ptr::null_mut::<TActionList>();
}
#[no_mangle]
pub unsafe extern "C" fn deleteRules() {
    let mut p: *mut TPremise = ::core::ptr::null_mut::<TPremise>();
    let mut pnext: *mut TPremise = ::core::ptr::null_mut::<TPremise>();
    let mut a: *mut TAction = ::core::ptr::null_mut::<TAction>();
    let mut anext: *mut TAction = ::core::ptr::null_mut::<TAction>();
    let mut r: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while r < RuleCount {
        p = (*Rules.offset(r as isize)).firstPremise;
        while !p.is_null() {
            pnext = (*p).next;
            free(p as *mut ::core::ffi::c_void);
            p = pnext;
        }
        a = (*Rules.offset(r as isize)).thenActions;
        while !a.is_null() {
            anext = (*a).next;
            free(a as *mut ::core::ffi::c_void);
            a = anext;
        }
        a = (*Rules.offset(r as isize)).elseActions;
        while !a.is_null() {
            anext = (*a).next;
            free(a as *mut ::core::ffi::c_void);
            a = anext;
        }
        r += 1;
    }
    if !Rules.is_null() {
        free(Rules as *mut ::core::ffi::c_void);
        Rules = ::core::ptr::null_mut::<TRule>();
    }
    RuleCount = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findExactMatch(
    mut s: *mut ::core::ffi::c_char,
    mut keyword: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*keyword.offset(i as isize)).is_null() {
        if strcomp(s, *keyword.offset(i as isize)) != 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
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
#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
