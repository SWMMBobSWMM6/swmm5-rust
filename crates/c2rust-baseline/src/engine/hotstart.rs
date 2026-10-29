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
    fn rewind(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn infil_getState(j: ::core::ffi::c_int, x: *mut ::core::ffi::c_double);
    fn infil_setState(j: ::core::ffi::c_int, x: *mut ::core::ffi::c_double);
    static mut Fhotstart1: TFile;
    static mut Fhotstart2: TFile;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut ErrorCode: ::core::ffi::c_int;
    static mut FlowUnits: ::core::ffi::c_int;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Storage: *mut TStorage;
    static mut Link: *mut TLink;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn snow_getState(
        subcatch: ::core::ffi::c_int,
        subArea: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_double,
    );
    fn snow_setState(
        subcatch: ::core::ffi::c_int,
        subArea: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_double,
    );
    fn gwater_getState(subcatch: ::core::ffi::c_int, x: *mut ::core::ffi::c_double);
    fn gwater_setState(subcatch: ::core::ffi::c_int, x: *mut ::core::ffi::c_double);
    fn link_setTargetSetting(j: ::core::ffi::c_int);
    fn link_setSetting(j: ::core::ffi::c_int, tstep: ::core::ffi::c_double);
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
pub struct TExfil {
    pub btmExfil: *mut TGrnAmpt,
    pub bankExfil: *mut TGrnAmpt,
    pub btmArea: ::core::ffi::c_double,
    pub bankMinDepth: ::core::ffi::c_double,
    pub bankMaxDepth: ::core::ffi::c_double,
    pub bankMaxArea: ::core::ffi::c_double,
}
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
pub struct TStorage {
    pub fEvap: ::core::ffi::c_double,
    pub a0: ::core::ffi::c_double,
    pub a1: ::core::ffi::c_double,
    pub a2: ::core::ffi::c_double,
    pub aCurve: ::core::ffi::c_int,
    pub shape: ::core::ffi::c_int,
    pub exfil: *mut TExfil,
    pub hrt: ::core::ffi::c_double,
    pub evapLoss: ::core::ffi::c_double,
    pub exfilLoss: ::core::ffi::c_double,
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
pub const ERR_HOTSTART_FILE_OPEN: ErrorType = 331;
pub const ERR_HOTSTART_FILE_READ: ErrorType = 335;
pub const ERR_HOTSTART_FILE_FORMAT: ErrorType = 333;
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
pub const ERR_KINWAVE: ErrorType = 103;
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NONE: ErrorType = 0;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MISSING: ::core::ffi::c_double = -1.0E10f64;
static mut fileVersion: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn hotstart_open() -> ::core::ffi::c_int {
    if openHotstartFile1() == 0 {
        return FALSE;
    }
    if openHotstartFile2() == 0 {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn hotstart_close() {
    if !Fhotstart2.file.is_null() {
        saveRunoff();
        saveRouting();
        fclose(Fhotstart2.file);
    }
}
unsafe extern "C" fn openHotstartFile1() -> ::core::ffi::c_int {
    let mut nSubcatch: ::core::ffi::c_int = 0;
    let mut nNodes: ::core::ffi::c_int = 0;
    let mut nLinks: ::core::ffi::c_int = 0;
    let mut nPollut: ::core::ffi::c_int = 0;
    let mut nLandUses: ::core::ffi::c_int = 0;
    let mut flowUnits: ::core::ffi::c_int = 0;
    let mut fStamp: [::core::ffi::c_char; 15] =
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"SWMM5-HOTSTART\0");
    let mut fileStamp: [::core::ffi::c_char; 15] =
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"SWMM5-HOTSTART\0");
    let mut fStampx: [::core::ffi::c_char; 16] =
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SWMM5-HOTSTARTx\0");
    let mut fileStamp2: [::core::ffi::c_char; 16] =
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SWMM5-HOTSTART2\0");
    let mut fileStamp3: [::core::ffi::c_char; 16] =
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SWMM5-HOTSTART3\0");
    let mut fileStamp4: [::core::ffi::c_char; 16] =
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SWMM5-HOTSTART4\0");
    if Fhotstart1.mode as ::core::ffi::c_int != USE_FILE as ::core::ffi::c_int {
        return TRUE;
    }
    Fhotstart1.file = fopen(
        &raw mut Fhotstart1.name as *mut ::core::ffi::c_char,
        b"r+b\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Fhotstart1.file.is_null() {
        report_writeErrorMsg(
            ERR_HOTSTART_FILE_OPEN as ::core::ffi::c_int,
            &raw mut Fhotstart1.name as *mut ::core::ffi::c_char,
        );
        return FALSE;
    }
    fread(
        &raw mut fStampx as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        strlen(&raw mut fileStamp2 as *mut ::core::ffi::c_char),
        Fhotstart1.file,
    );
    if strcmp(
        &raw mut fStampx as *mut ::core::ffi::c_char,
        &raw mut fileStamp4 as *mut ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        fileVersion = 4 as ::core::ffi::c_int;
    } else if strcmp(
        &raw mut fStampx as *mut ::core::ffi::c_char,
        &raw mut fileStamp3 as *mut ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        fileVersion = 3 as ::core::ffi::c_int;
    } else if strcmp(
        &raw mut fStampx as *mut ::core::ffi::c_char,
        &raw mut fileStamp2 as *mut ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        fileVersion = 2 as ::core::ffi::c_int;
    } else {
        rewind(Fhotstart1.file);
        fread(
            &raw mut fStamp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            strlen(&raw mut fileStamp as *mut ::core::ffi::c_char),
            Fhotstart1.file,
        );
        if strcmp(
            &raw mut fStamp as *mut ::core::ffi::c_char,
            &raw mut fileStamp as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            report_writeErrorMsg(
                ERR_HOTSTART_FILE_FORMAT as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return FALSE;
        }
        fileVersion = 1 as ::core::ffi::c_int;
    }
    nSubcatch = -(1 as ::core::ffi::c_int);
    nNodes = -(1 as ::core::ffi::c_int);
    nLinks = -(1 as ::core::ffi::c_int);
    nPollut = -(1 as ::core::ffi::c_int);
    nLandUses = -(1 as ::core::ffi::c_int);
    flowUnits = -(1 as ::core::ffi::c_int);
    if fileVersion >= 2 as ::core::ffi::c_int {
        fread(
            &raw mut nSubcatch as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Fhotstart1.file,
        );
    } else {
        nSubcatch = Nobjects[SUBCATCH as ::core::ffi::c_int as usize];
    }
    if fileVersion >= 3 as ::core::ffi::c_int {
        fread(
            &raw mut nLandUses as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Fhotstart1.file,
        );
    } else {
        nLandUses = Nobjects[LANDUSE as ::core::ffi::c_int as usize];
    }
    fread(
        &raw mut nNodes as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart1.file,
    );
    fread(
        &raw mut nLinks as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart1.file,
    );
    fread(
        &raw mut nPollut as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart1.file,
    );
    fread(
        &raw mut flowUnits as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart1.file,
    );
    if nSubcatch != Nobjects[SUBCATCH as ::core::ffi::c_int as usize]
        || nLandUses != Nobjects[LANDUSE as ::core::ffi::c_int as usize]
        || nNodes != Nobjects[NODE as ::core::ffi::c_int as usize]
        || nLinks != Nobjects[LINK as ::core::ffi::c_int as usize]
        || nPollut != Nobjects[POLLUT as ::core::ffi::c_int as usize]
        || flowUnits != FlowUnits
    {
        report_writeErrorMsg(
            ERR_HOTSTART_FILE_FORMAT as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return FALSE;
    }
    if fileVersion >= 3 as ::core::ffi::c_int {
        readRunoff();
    }
    readRouting();
    fclose(Fhotstart1.file);
    if ErrorCode != 0 {
        return FALSE;
    } else {
        return TRUE;
    };
}
unsafe extern "C" fn openHotstartFile2() -> ::core::ffi::c_int {
    let mut nSubcatch: ::core::ffi::c_int = 0;
    let mut nLandUses: ::core::ffi::c_int = 0;
    let mut nNodes: ::core::ffi::c_int = 0;
    let mut nLinks: ::core::ffi::c_int = 0;
    let mut nPollut: ::core::ffi::c_int = 0;
    let mut flowUnits: ::core::ffi::c_int = 0;
    let mut fileStamp: [::core::ffi::c_char; 16] =
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SWMM5-HOTSTART4\0");
    if Fhotstart2.mode as ::core::ffi::c_int != SAVE_FILE as ::core::ffi::c_int {
        return TRUE;
    }
    Fhotstart2.file = fopen(
        &raw mut Fhotstart2.name as *mut ::core::ffi::c_char,
        b"w+b\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Fhotstart2.file.is_null() {
        report_writeErrorMsg(
            ERR_HOTSTART_FILE_OPEN as ::core::ffi::c_int,
            &raw mut Fhotstart2.name as *mut ::core::ffi::c_char,
        );
        return FALSE;
    }
    nSubcatch = Nobjects[SUBCATCH as ::core::ffi::c_int as usize];
    nLandUses = Nobjects[LANDUSE as ::core::ffi::c_int as usize];
    nNodes = Nobjects[NODE as ::core::ffi::c_int as usize];
    nLinks = Nobjects[LINK as ::core::ffi::c_int as usize];
    nPollut = Nobjects[POLLUT as ::core::ffi::c_int as usize];
    flowUnits = FlowUnits;
    fwrite(
        &raw mut fileStamp as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        strlen(&raw mut fileStamp as *mut ::core::ffi::c_char),
        Fhotstart2.file,
    );
    fwrite(
        &raw mut nSubcatch as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart2.file,
    );
    fwrite(
        &raw mut nLandUses as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart2.file,
    );
    fwrite(
        &raw mut nNodes as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart2.file,
    );
    fwrite(
        &raw mut nLinks as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart2.file,
    );
    fwrite(
        &raw mut nPollut as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart2.file,
    );
    fwrite(
        &raw mut flowUnits as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fhotstart2.file,
    );
    return TRUE;
}
unsafe extern "C" fn saveRouting() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_float; 3] = [0.; 3];
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        x[0 as ::core::ffi::c_int as usize] =
            (*Node.offset(i as isize)).newDepth as ::core::ffi::c_float;
        x[1 as ::core::ffi::c_int as usize] =
            (*Node.offset(i as isize)).newLatFlow as ::core::ffi::c_float;
        fwrite(
            &raw mut x as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            2 as size_t,
            Fhotstart2.file,
        );
        if (*Node.offset(i as isize)).type_0 == STORAGE as ::core::ffi::c_int {
            j = (*Node.offset(i as isize)).subIndex;
            x[0 as ::core::ffi::c_int as usize] =
                (*Storage.offset(j as isize)).hrt as ::core::ffi::c_float;
            fwrite(
                (&raw mut x as *mut ::core::ffi::c_float).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                1 as size_t,
                Fhotstart2.file,
            );
        }
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            x[0 as ::core::ffi::c_int as usize] =
                *(*Node.offset(i as isize)).newQual.offset(j as isize) as ::core::ffi::c_float;
            fwrite(
                (&raw mut x as *mut ::core::ffi::c_float).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                1 as size_t,
                Fhotstart2.file,
            );
            j += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        x[0 as ::core::ffi::c_int as usize] =
            (*Link.offset(i as isize)).newFlow as ::core::ffi::c_float;
        x[1 as ::core::ffi::c_int as usize] =
            (*Link.offset(i as isize)).newDepth as ::core::ffi::c_float;
        x[2 as ::core::ffi::c_int as usize] =
            (*Link.offset(i as isize)).setting as ::core::ffi::c_float;
        fwrite(
            &raw mut x as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            3 as size_t,
            Fhotstart2.file,
        );
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            x[0 as ::core::ffi::c_int as usize] =
                *(*Link.offset(i as isize)).newQual.offset(j as isize) as ::core::ffi::c_float;
            fwrite(
                (&raw mut x as *mut ::core::ffi::c_float).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                1 as size_t,
                Fhotstart2.file,
            );
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn readRouting() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_float = 0.;
    let mut xgw: [::core::ffi::c_double; 4] = [0.; 4];
    let mut f: *mut FILE = Fhotstart1.file;
    if fileVersion == 2 as ::core::ffi::c_int {
        xgw[2 as ::core::ffi::c_int as usize] = 0.0f64;
        xgw[3 as ::core::ffi::c_int as usize] = MISSING;
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
            if readFloat(&raw mut x, f) == 0 {
                return;
            }
            xgw[0 as ::core::ffi::c_int as usize] = x as ::core::ffi::c_double;
            if readFloat(&raw mut x, f) == 0 {
                return;
            }
            xgw[1 as ::core::ffi::c_int as usize] = x as ::core::ffi::c_double;
            if !(*Subcatch.offset(i as isize)).groundwater.is_null() {
                gwater_setState(i, &raw mut xgw as *mut ::core::ffi::c_double);
            }
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if readFloat(&raw mut x, f) == 0 {
            return;
        }
        (*Node.offset(i as isize)).newDepth = x as ::core::ffi::c_double;
        if readFloat(&raw mut x, f) == 0 {
            return;
        }
        (*Node.offset(i as isize)).newLatFlow = x as ::core::ffi::c_double;
        if fileVersion >= 4 as ::core::ffi::c_int
            && (*Node.offset(i as isize)).type_0 == STORAGE as ::core::ffi::c_int
        {
            if readFloat(&raw mut x, f) == 0 {
                return;
            }
            j = (*Node.offset(i as isize)).subIndex;
            (*Storage.offset(j as isize)).hrt = x as ::core::ffi::c_double;
        }
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            if readFloat(&raw mut x, f) == 0 {
                return;
            }
            *(*Node.offset(i as isize)).newQual.offset(j as isize) = x as ::core::ffi::c_double;
            j += 1;
        }
        if fileVersion <= 2 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
            while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                if readFloat(&raw mut x, f) == 0 {
                    return;
                }
                j += 1;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if readFloat(&raw mut x, f) == 0 {
            return;
        }
        (*Link.offset(i as isize)).newFlow = x as ::core::ffi::c_double;
        if readFloat(&raw mut x, f) == 0 {
            return;
        }
        (*Link.offset(i as isize)).newDepth = x as ::core::ffi::c_double;
        if readFloat(&raw mut x, f) == 0 {
            return;
        }
        (*Link.offset(i as isize)).setting = x as ::core::ffi::c_double;
        (*Link.offset(i as isize)).targetSetting = x as ::core::ffi::c_double;
        link_setTargetSetting(i);
        link_setSetting(i, 0.0f64);
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            if readFloat(&raw mut x, f) == 0 {
                return;
            }
            *(*Link.offset(i as isize)).newQual.offset(j as isize) = x as ::core::ffi::c_double;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn saveRunoff() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    let mut f: *mut FILE = Fhotstart2.file;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        j = 0 as ::core::ffi::c_int;
        while j < 3 as ::core::ffi::c_int {
            x[j as usize] = (*Subcatch.offset(i as isize)).subArea[j as usize].depth;
            j += 1;
        }
        x[3 as ::core::ffi::c_int as usize] = (*Subcatch.offset(i as isize)).newRunoff;
        fwrite(
            &raw mut x as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
            4 as size_t,
            f,
        );
        j = 0 as ::core::ffi::c_int;
        while j < 6 as ::core::ffi::c_int {
            x[j as usize] = 0.0f64;
            j += 1;
        }
        infil_getState(i, &raw mut x as *mut ::core::ffi::c_double);
        fwrite(
            &raw mut x as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
            6 as size_t,
            f,
        );
        if !(*Subcatch.offset(i as isize)).groundwater.is_null() {
            gwater_getState(i, &raw mut x as *mut ::core::ffi::c_double);
            fwrite(
                &raw mut x as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                4 as size_t,
                f,
            );
        }
        if !(*Subcatch.offset(i as isize)).snowpack.is_null() {
            j = 0 as ::core::ffi::c_int;
            while j < 3 as ::core::ffi::c_int {
                snow_getState(i, j, &raw mut x as *mut ::core::ffi::c_double);
                fwrite(
                    &raw mut x as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                    5 as size_t,
                    f,
                );
                j += 1;
            }
        }
        if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
            while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                x[0 as ::core::ffi::c_int as usize] =
                    *(*Subcatch.offset(i as isize)).newQual.offset(j as isize);
                fwrite(
                    &raw mut x as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                    1 as size_t,
                    f,
                );
                j += 1;
            }
            j = 0 as ::core::ffi::c_int;
            while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                x[0 as ::core::ffi::c_int as usize] =
                    *(*Subcatch.offset(i as isize)).pondedQual.offset(j as isize);
                fwrite(
                    &raw mut x as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                    1 as size_t,
                    f,
                );
                j += 1;
            }
            k = 0 as ::core::ffi::c_int;
            while k < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
                j = 0 as ::core::ffi::c_int;
                while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    x[0 as ::core::ffi::c_int as usize] =
                        *(*(*Subcatch.offset(i as isize)).landFactor.offset(k as isize))
                            .buildup
                            .offset(j as isize);
                    fwrite(
                        &raw mut x as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                        Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
                        f,
                    );
                    j += 1;
                }
                x[0 as ::core::ffi::c_int as usize] =
                    (*(*Subcatch.offset(i as isize)).landFactor.offset(k as isize)).lastSwept
                        as ::core::ffi::c_double;
                fwrite(
                    &raw mut x as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                    1 as size_t,
                    f,
                );
                k += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn readRunoff() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    let mut f: *mut FILE = Fhotstart1.file;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        j = 0 as ::core::ffi::c_int;
        while j < 3 as ::core::ffi::c_int {
            if readDouble(
                &raw mut (*(&raw mut (*Subcatch.offset(i as isize)).subArea as *mut TSubarea)
                    .offset(j as isize))
                .depth,
                f,
            ) == 0
            {
                return;
            }
            j += 1;
        }
        if readDouble(&raw mut (*Subcatch.offset(i as isize)).newRunoff, f) == 0 {
            return;
        }
        j = 0 as ::core::ffi::c_int;
        while j < 6 as ::core::ffi::c_int {
            if readDouble(
                (&raw mut x as *mut ::core::ffi::c_double).offset(j as isize)
                    as *mut ::core::ffi::c_double,
                f,
            ) == 0
            {
                return;
            }
            j += 1;
        }
        infil_setState(i, &raw mut x as *mut ::core::ffi::c_double);
        if !(*Subcatch.offset(i as isize)).groundwater.is_null() {
            j = 0 as ::core::ffi::c_int;
            while j < 4 as ::core::ffi::c_int {
                if readDouble(
                    (&raw mut x as *mut ::core::ffi::c_double).offset(j as isize)
                        as *mut ::core::ffi::c_double,
                    f,
                ) == 0
                {
                    return;
                }
                j += 1;
            }
            gwater_setState(i, &raw mut x as *mut ::core::ffi::c_double);
        }
        if !(*Subcatch.offset(i as isize)).snowpack.is_null() {
            j = 0 as ::core::ffi::c_int;
            while j < 3 as ::core::ffi::c_int {
                k = 0 as ::core::ffi::c_int;
                while k < 5 as ::core::ffi::c_int {
                    if readDouble(
                        (&raw mut x as *mut ::core::ffi::c_double).offset(k as isize)
                            as *mut ::core::ffi::c_double,
                        f,
                    ) == 0
                    {
                        return;
                    }
                    k += 1;
                }
                snow_setState(i, j, &raw mut x as *mut ::core::ffi::c_double);
                j += 1;
            }
        }
        if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
            while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                if readDouble(
                    (*Subcatch.offset(i as isize)).newQual.offset(j as isize)
                        as *mut ::core::ffi::c_double,
                    f,
                ) == 0
                {
                    return;
                }
                j += 1;
            }
            j = 0 as ::core::ffi::c_int;
            while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                if readDouble(
                    (*Subcatch.offset(i as isize)).pondedQual.offset(j as isize)
                        as *mut ::core::ffi::c_double,
                    f,
                ) == 0
                {
                    return;
                }
                j += 1;
            }
            k = 0 as ::core::ffi::c_int;
            while k < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
                j = 0 as ::core::ffi::c_int;
                while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    if readDouble(
                        (*(*Subcatch.offset(i as isize)).landFactor.offset(k as isize))
                            .buildup
                            .offset(j as isize)
                            as *mut ::core::ffi::c_double,
                        f,
                    ) == 0
                    {
                        return;
                    }
                    j += 1;
                }
                if readDouble(
                    &raw mut (*(*Subcatch.offset(i as isize)).landFactor.offset(k as isize))
                        .lastSwept,
                    f,
                ) == 0
                {
                    return;
                }
                k += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn readFloat(
    mut x: *mut ::core::ffi::c_float,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    fread(
        x as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        1 as size_t,
        f,
    );
    if *x != *x {
        report_writeErrorMsg(
            ERR_HOTSTART_FILE_READ as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        *x = 0.0f32;
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn readDouble(
    mut x: *mut ::core::ffi::c_double,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    if feof(f) != 0 {
        *x = 0.0f64;
        report_writeErrorMsg(
            ERR_HOTSTART_FILE_READ as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return FALSE;
    }
    fread(
        x as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        1 as size_t,
        f,
    );
    if *x != *x {
        *x = 0.0f64;
        return FALSE;
    }
    return TRUE;
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
#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
