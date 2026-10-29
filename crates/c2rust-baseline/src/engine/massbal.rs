#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut RptFlags: TRptFlags;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut RouteModel: ::core::ffi::c_int;
    static mut IgnoreGwater: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut IgnoreRouting: ::core::ffi::c_int;
    static mut GwaterError: ::core::ffi::c_double;
    static mut RunoffError: ::core::ffi::c_double;
    static mut FlowError: ::core::ffi::c_double;
    static mut QualError: ::core::ffi::c_double;
    static mut Snow: TSnow;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Pollut: *mut TPollut;
    fn report_writeRunoffError(totals: *mut TRunoffTotals, area: ::core::ffi::c_double);
    fn report_writeLoadingError(totals: *mut TLoadingTotals);
    fn report_writeGwaterError(totals: *mut TGwaterTotals, area: ::core::ffi::c_double);
    fn report_writeFlowError(totals: *mut TRoutingTotals);
    fn report_writeQualError(totals: *mut TRoutingTotals);
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn snow_getSnowCover(subcatch: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn gwater_getVolume(subcatch: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn subcatch_getStorage(subcatch: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
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
pub type NodeType = ::core::ffi::c_uint;
pub const DIVIDER: NodeType = 3;
pub const STORAGE: NodeType = 2;
pub const OUTFALL: NodeType = 1;
pub const JUNCTION: NodeType = 0;
pub type ConcUnitsType = ::core::ffi::c_uint;
pub const COUNT: ConcUnitsType = 2;
pub const UG: ConcUnitsType = 1;
pub const MG: ConcUnitsType = 0;
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
pub type SysFlowType = ::core::ffi::c_uint;
pub const SYS_PET: SysFlowType = 14;
pub const SYS_EVAP: SysFlowType = 13;
pub const SYS_STORAGE: SysFlowType = 12;
pub const SYS_OUTFLOW: SysFlowType = 11;
pub const SYS_FLOODING: SysFlowType = 10;
pub const SYS_INFLOW: SysFlowType = 9;
pub const SYS_EXFLOW: SysFlowType = 8;
pub const SYS_IIFLOW: SysFlowType = 7;
pub const SYS_GWFLOW: SysFlowType = 6;
pub const SYS_DWFLOW: SysFlowType = 5;
pub const SYS_RUNOFF: SysFlowType = 4;
pub const SYS_INFIL: SysFlowType = 3;
pub const SYS_SNOWDEPTH: SysFlowType = 2;
pub const SYS_RAINFALL: SysFlowType = 1;
pub const SYS_TEMPERATURE: SysFlowType = 0;
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
pub type RouteModelType = ::core::ffi::c_uint;
pub const DW: RouteModelType = 4;
pub const EKW: RouteModelType = 3;
pub const KW: RouteModelType = 2;
pub const SF: RouteModelType = 1;
pub const NO_ROUTING: RouteModelType = 0;
pub type InflowType = ::core::ffi::c_uint;
pub const MASS_INFLOW: InflowType = 7;
pub const CONCEN_INFLOW: InflowType = 6;
pub const FLOW_INFLOW: InflowType = 5;
pub const RDII_INFLOW: InflowType = 4;
pub const GROUNDWATER_INFLOW: InflowType = 3;
pub const WET_WEATHER_INFLOW: InflowType = 2;
pub const DRY_WEATHER_INFLOW: InflowType = 1;
pub const EXTERNAL_INFLOW: InflowType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TRunoffTotals {
    pub rainfall: ::core::ffi::c_double,
    pub evap: ::core::ffi::c_double,
    pub infil: ::core::ffi::c_double,
    pub runoff: ::core::ffi::c_double,
    pub drains: ::core::ffi::c_double,
    pub runon: ::core::ffi::c_double,
    pub initStorage: ::core::ffi::c_double,
    pub finalStorage: ::core::ffi::c_double,
    pub initSnowCover: ::core::ffi::c_double,
    pub finalSnowCover: ::core::ffi::c_double,
    pub snowRemoved: ::core::ffi::c_double,
    pub pctError: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLoadingTotals {
    pub initLoad: ::core::ffi::c_double,
    pub buildup: ::core::ffi::c_double,
    pub deposition: ::core::ffi::c_double,
    pub sweeping: ::core::ffi::c_double,
    pub bmpRemoval: ::core::ffi::c_double,
    pub infil: ::core::ffi::c_double,
    pub runoff: ::core::ffi::c_double,
    pub finalLoad: ::core::ffi::c_double,
    pub pctError: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TGwaterTotals {
    pub infil: ::core::ffi::c_double,
    pub upperEvap: ::core::ffi::c_double,
    pub lowerEvap: ::core::ffi::c_double,
    pub lowerPerc: ::core::ffi::c_double,
    pub gwater: ::core::ffi::c_double,
    pub initStorage: ::core::ffi::c_double,
    pub finalStorage: ::core::ffi::c_double,
    pub pctError: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TRoutingTotals {
    pub dwInflow: ::core::ffi::c_double,
    pub wwInflow: ::core::ffi::c_double,
    pub gwInflow: ::core::ffi::c_double,
    pub iiInflow: ::core::ffi::c_double,
    pub exInflow: ::core::ffi::c_double,
    pub flooding: ::core::ffi::c_double,
    pub outflow: ::core::ffi::c_double,
    pub evapLoss: ::core::ffi::c_double,
    pub seepLoss: ::core::ffi::c_double,
    pub reacted: ::core::ffi::c_double,
    pub initStorage: ::core::ffi::c_double,
    pub finalStorage: ::core::ffi::c_double,
    pub pctError: ::core::ffi::c_double,
}
pub const ERR_MEMORY: ErrorType = 101;
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
pub const ERR_NONE: ErrorType = 0;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TINY: ::core::ffi::c_double = 1.0E-6f64;
pub const LperFT3: ::core::ffi::c_double = 28.317f64;
static mut MAX_RUNOFF_BALANCE_ERR: ::core::ffi::c_double = 10.0f64;
static mut MAX_FLOW_BALANCE_ERR: ::core::ffi::c_double = 10.0f64;
#[no_mangle]
pub static mut RunoffTotals: TRunoffTotals = TRunoffTotals {
    rainfall: 0.,
    evap: 0.,
    infil: 0.,
    runoff: 0.,
    drains: 0.,
    runon: 0.,
    initStorage: 0.,
    finalStorage: 0.,
    initSnowCover: 0.,
    finalSnowCover: 0.,
    snowRemoved: 0.,
    pctError: 0.,
};
#[no_mangle]
pub static mut LoadingTotals: *mut TLoadingTotals =
    ::core::ptr::null::<TLoadingTotals>() as *mut TLoadingTotals;
#[no_mangle]
pub static mut GwaterTotals: TGwaterTotals = TGwaterTotals {
    infil: 0.,
    upperEvap: 0.,
    lowerEvap: 0.,
    lowerPerc: 0.,
    gwater: 0.,
    initStorage: 0.,
    finalStorage: 0.,
    pctError: 0.,
};
#[no_mangle]
pub static mut FlowTotals: TRoutingTotals = TRoutingTotals {
    dwInflow: 0.,
    wwInflow: 0.,
    gwInflow: 0.,
    iiInflow: 0.,
    exInflow: 0.,
    flooding: 0.,
    outflow: 0.,
    evapLoss: 0.,
    seepLoss: 0.,
    reacted: 0.,
    initStorage: 0.,
    finalStorage: 0.,
    pctError: 0.,
};
#[no_mangle]
pub static mut QualTotals: *mut TRoutingTotals =
    ::core::ptr::null::<TRoutingTotals>() as *mut TRoutingTotals;
#[no_mangle]
pub static mut StepFlowTotals: TRoutingTotals = TRoutingTotals {
    dwInflow: 0.,
    wwInflow: 0.,
    gwInflow: 0.,
    iiInflow: 0.,
    exInflow: 0.,
    flooding: 0.,
    outflow: 0.,
    evapLoss: 0.,
    seepLoss: 0.,
    reacted: 0.,
    initStorage: 0.,
    finalStorage: 0.,
    pctError: 0.,
};
#[no_mangle]
pub static mut OldStepFlowTotals: TRoutingTotals = TRoutingTotals {
    dwInflow: 0.,
    wwInflow: 0.,
    gwInflow: 0.,
    iiInflow: 0.,
    exInflow: 0.,
    flooding: 0.,
    outflow: 0.,
    evapLoss: 0.,
    seepLoss: 0.,
    reacted: 0.,
    initStorage: 0.,
    finalStorage: 0.,
    pctError: 0.,
};
#[no_mangle]
pub static mut StepQualTotals: *mut TRoutingTotals =
    ::core::ptr::null::<TRoutingTotals>() as *mut TRoutingTotals;
#[no_mangle]
pub static mut NodeInflow: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub static mut NodeOutflow: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub static mut TotalArea: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn massbal_open() -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    RunoffError = 0.0f64;
    GwaterError = 0.0f64;
    FlowError = 0.0f64;
    QualError = 0.0f64;
    RunoffTotals.rainfall = 0.0f64;
    RunoffTotals.evap = 0.0f64;
    RunoffTotals.infil = 0.0f64;
    RunoffTotals.runoff = 0.0f64;
    RunoffTotals.runon = 0.0f64;
    RunoffTotals.drains = 0.0f64;
    RunoffTotals.snowRemoved = 0.0f64;
    RunoffTotals.initStorage = 0.0f64;
    RunoffTotals.initSnowCover = 0.0f64;
    TotalArea = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        RunoffTotals.initStorage += subcatch_getStorage(j);
        RunoffTotals.initSnowCover += snow_getSnowCover(j);
        TotalArea += (*Subcatch.offset(j as isize)).area;
        j += 1;
    }
    GwaterTotals.infil = 0.0f64;
    GwaterTotals.upperEvap = 0.0f64;
    GwaterTotals.lowerEvap = 0.0f64;
    GwaterTotals.lowerPerc = 0.0f64;
    GwaterTotals.gwater = 0.0f64;
    GwaterTotals.initStorage = 0.0f64;
    GwaterTotals.finalStorage = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        GwaterTotals.initStorage += gwater_getVolume(j) * (*Subcatch.offset(j as isize)).area;
        j += 1;
    }
    FlowTotals.dwInflow = 0.0f64;
    FlowTotals.wwInflow = 0.0f64;
    FlowTotals.gwInflow = 0.0f64;
    FlowTotals.iiInflow = 0.0f64;
    FlowTotals.exInflow = 0.0f64;
    FlowTotals.flooding = 0.0f64;
    FlowTotals.outflow = 0.0f64;
    FlowTotals.evapLoss = 0.0f64;
    FlowTotals.seepLoss = 0.0f64;
    FlowTotals.reacted = 0.0f64;
    FlowTotals.initStorage = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        FlowTotals.initStorage += (*Node.offset(j as isize)).newVolume;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        FlowTotals.initStorage += (*Link.offset(j as isize)).newVolume;
        j += 1;
    }
    StepFlowTotals = FlowTotals;
    LoadingTotals = ::core::ptr::null_mut::<TLoadingTotals>();
    QualTotals = ::core::ptr::null_mut::<TRoutingTotals>();
    StepQualTotals = ::core::ptr::null_mut::<TRoutingTotals>();
    NodeInflow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    NodeOutflow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    n = Nobjects[POLLUT as ::core::ffi::c_int as usize];
    if n > 0 as ::core::ffi::c_int {
        LoadingTotals = calloc(
            n as size_t,
            ::core::mem::size_of::<TLoadingTotals>() as size_t,
        ) as *mut TLoadingTotals;
        if LoadingTotals.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        }
        j = 0 as ::core::ffi::c_int;
        while j < n {
            (*LoadingTotals.offset(j as isize)).initLoad = massbal_getBuildup(j);
            (*LoadingTotals.offset(j as isize)).buildup = 0.0f64;
            (*LoadingTotals.offset(j as isize)).deposition = 0.0f64;
            (*LoadingTotals.offset(j as isize)).sweeping = 0.0f64;
            (*LoadingTotals.offset(j as isize)).infil = 0.0f64;
            (*LoadingTotals.offset(j as isize)).bmpRemoval = 0.0f64;
            (*LoadingTotals.offset(j as isize)).runoff = 0.0f64;
            (*LoadingTotals.offset(j as isize)).finalLoad = 0.0f64;
            j += 1;
        }
    }
    if n > 0 as ::core::ffi::c_int {
        QualTotals = calloc(
            n as size_t,
            ::core::mem::size_of::<TRoutingTotals>() as size_t,
        ) as *mut TRoutingTotals;
        StepQualTotals = calloc(
            n as size_t,
            ::core::mem::size_of::<TRoutingTotals>() as size_t,
        ) as *mut TRoutingTotals;
        if QualTotals.is_null() || StepQualTotals.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        }
    }
    j = 0 as ::core::ffi::c_int;
    while j < n {
        (*QualTotals.offset(j as isize)).dwInflow = 0.0f64;
        (*QualTotals.offset(j as isize)).wwInflow = 0.0f64;
        (*QualTotals.offset(j as isize)).gwInflow = 0.0f64;
        (*QualTotals.offset(j as isize)).exInflow = 0.0f64;
        (*QualTotals.offset(j as isize)).flooding = 0.0f64;
        (*QualTotals.offset(j as isize)).outflow = 0.0f64;
        (*QualTotals.offset(j as isize)).evapLoss = 0.0f64;
        (*QualTotals.offset(j as isize)).seepLoss = 0.0f64;
        (*QualTotals.offset(j as isize)).reacted = 0.0f64;
        (*QualTotals.offset(j as isize)).initStorage = massbal_getStoredMass(j);
        j += 1;
    }
    massbal_initTimeStepTotals();
    if Nobjects[NODE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        NodeInflow = calloc(
            Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        if NodeInflow.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        }
        NodeOutflow = calloc(
            Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        if NodeOutflow.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        }
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            *NodeInflow.offset(j as isize) = (*Node.offset(j as isize)).newVolume;
            j += 1;
        }
    }
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_close() {
    if !LoadingTotals.is_null() {
        free(LoadingTotals as *mut ::core::ffi::c_void);
        LoadingTotals = ::core::ptr::null_mut::<TLoadingTotals>();
    }
    if !QualTotals.is_null() {
        free(QualTotals as *mut ::core::ffi::c_void);
        QualTotals = ::core::ptr::null_mut::<TRoutingTotals>();
    }
    if !StepQualTotals.is_null() {
        free(StepQualTotals as *mut ::core::ffi::c_void);
        StepQualTotals = ::core::ptr::null_mut::<TRoutingTotals>();
    }
    if !NodeInflow.is_null() {
        free(NodeInflow as *mut ::core::ffi::c_void);
        NodeInflow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !NodeOutflow.is_null() {
        free(NodeOutflow as *mut ::core::ffi::c_void);
        NodeOutflow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn massbal_report() {
    let mut j: ::core::ffi::c_int = 0;
    let mut gwArea: ::core::ffi::c_double = 0.0f64;
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        if massbal_getRunoffError() > MAX_RUNOFF_BALANCE_ERR
            || RptFlags.continuity as ::core::ffi::c_int == TRUE
        {
            report_writeRunoffError(&raw mut RunoffTotals, TotalArea);
        }
        if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
            && IgnoreQuality == 0
        {
            if massbal_getLoadingError() > MAX_RUNOFF_BALANCE_ERR
                || RptFlags.continuity as ::core::ffi::c_int == TRUE
            {
                report_writeLoadingError(LoadingTotals);
            }
        }
    }
    if Nobjects[AQUIFER as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
        && IgnoreGwater == 0
    {
        if massbal_getGwaterError() > MAX_RUNOFF_BALANCE_ERR
            || RptFlags.continuity as ::core::ffi::c_int == TRUE
        {
            j = 0 as ::core::ffi::c_int;
            while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
                if !(*Subcatch.offset(j as isize)).groundwater.is_null() {
                    gwArea += (*Subcatch.offset(j as isize)).area;
                }
                j += 1;
            }
            if gwArea > 0.0f64 {
                report_writeGwaterError(&raw mut GwaterTotals, gwArea);
            }
        }
    }
    if Nobjects[NODE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int && IgnoreRouting == 0
    {
        if massbal_getFlowError() > MAX_FLOW_BALANCE_ERR
            || RptFlags.continuity as ::core::ffi::c_int == TRUE
        {
            report_writeFlowError(&raw mut FlowTotals);
        }
        if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
            && IgnoreQuality == 0
        {
            if massbal_getQualError() > MAX_FLOW_BALANCE_ERR
                || RptFlags.continuity as ::core::ffi::c_int == TRUE
            {
                report_writeQualError(QualTotals);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getBuildup(mut p: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut load: ::core::ffi::c_double = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
            load += *(*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize))
                .buildup
                .offset(p as isize);
            i += 1;
        }
        load += *(*Subcatch.offset(j as isize)).pondedQual.offset(p as isize)
            * (*Pollut.offset(p as isize)).mcf;
        j += 1;
    }
    return load;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_updateRunoffTotals(
    mut flowType: ::core::ffi::c_int,
    mut v: ::core::ffi::c_double,
) {
    match flowType {
        0 => {
            RunoffTotals.rainfall += v;
        }
        1 => {
            RunoffTotals.evap += v;
        }
        2 => {
            RunoffTotals.infil += v;
        }
        3 => {
            RunoffTotals.runoff += v;
        }
        4 => {
            RunoffTotals.drains += v;
        }
        5 => {
            RunoffTotals.runon += v;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn massbal_updateGwaterTotals(
    mut vInfil: ::core::ffi::c_double,
    mut vUpperEvap: ::core::ffi::c_double,
    mut vLowerEvap: ::core::ffi::c_double,
    mut vLowerPerc: ::core::ffi::c_double,
    mut vGwater: ::core::ffi::c_double,
) {
    GwaterTotals.infil += vInfil;
    GwaterTotals.upperEvap += vUpperEvap;
    GwaterTotals.lowerEvap += vLowerEvap;
    GwaterTotals.lowerPerc += vLowerPerc;
    GwaterTotals.gwater += vGwater;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_initTimeStepTotals() {
    let mut j: ::core::ffi::c_int = 0;
    OldStepFlowTotals = StepFlowTotals;
    StepFlowTotals.dwInflow = 0.0f64;
    StepFlowTotals.wwInflow = 0.0f64;
    StepFlowTotals.gwInflow = 0.0f64;
    StepFlowTotals.iiInflow = 0.0f64;
    StepFlowTotals.exInflow = 0.0f64;
    StepFlowTotals.flooding = 0.0f64;
    StepFlowTotals.outflow = 0.0f64;
    StepFlowTotals.evapLoss = 0.0f64;
    StepFlowTotals.seepLoss = 0.0f64;
    StepFlowTotals.reacted = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        (*StepQualTotals.offset(j as isize)).dwInflow = 0.0f64;
        (*StepQualTotals.offset(j as isize)).wwInflow = 0.0f64;
        (*StepQualTotals.offset(j as isize)).gwInflow = 0.0f64;
        (*StepQualTotals.offset(j as isize)).iiInflow = 0.0f64;
        (*StepQualTotals.offset(j as isize)).exInflow = 0.0f64;
        (*StepQualTotals.offset(j as isize)).flooding = 0.0f64;
        (*StepQualTotals.offset(j as isize)).outflow = 0.0f64;
        (*StepQualTotals.offset(j as isize)).reacted = 0.0f64;
        (*StepQualTotals.offset(j as isize)).seepLoss = 0.0f64;
        (*StepQualTotals.offset(j as isize)).initStorage = 0.0f64;
        (*StepQualTotals.offset(j as isize)).finalStorage = 0.0f64;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addInflowFlow(
    mut type_0: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) {
    match type_0 {
        1 => {
            StepFlowTotals.dwInflow += q;
        }
        2 => {
            StepFlowTotals.wwInflow += q;
        }
        3 => {
            StepFlowTotals.gwInflow += q;
        }
        4 => {
            StepFlowTotals.iiInflow += q;
        }
        0 => {
            StepFlowTotals.exInflow += q;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn massbal_updateLoadingTotals(
    mut type_0: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut w: ::core::ffi::c_double,
) {
    match type_0 {
        0 => {
            (*LoadingTotals.offset(p as isize)).buildup += w;
        }
        1 => {
            (*LoadingTotals.offset(p as isize)).deposition += w;
        }
        2 => {
            (*LoadingTotals.offset(p as isize)).sweeping += w;
        }
        4 => {
            (*LoadingTotals.offset(p as isize)).infil += w;
        }
        3 => {
            (*LoadingTotals.offset(p as isize)).bmpRemoval += w;
        }
        5 => {
            (*LoadingTotals.offset(p as isize)).runoff += w;
        }
        6 => {
            (*LoadingTotals.offset(p as isize)).finalLoad += w;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addInflowQual(
    mut type_0: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut w: ::core::ffi::c_double,
) {
    if p < 0 as ::core::ffi::c_int || p >= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        return;
    }
    match type_0 {
        1 => {
            (*StepQualTotals.offset(p as isize)).dwInflow += w;
        }
        2 => {
            (*StepQualTotals.offset(p as isize)).wwInflow += w;
        }
        3 => {
            (*StepQualTotals.offset(p as isize)).gwInflow += w;
        }
        0 => {
            (*StepQualTotals.offset(p as isize)).exInflow += w;
        }
        4 => {
            (*StepQualTotals.offset(p as isize)).iiInflow += w;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addOutflowFlow(
    mut q: ::core::ffi::c_double,
    mut isFlooded: ::core::ffi::c_int,
) {
    if isFlooded != 0 {
        StepFlowTotals.flooding += q;
    } else {
        StepFlowTotals.outflow += q;
    };
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addOutflowQual(
    mut p: ::core::ffi::c_int,
    mut w: ::core::ffi::c_double,
    mut isFlooded: ::core::ffi::c_int,
) {
    if p < 0 as ::core::ffi::c_int || p >= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        return;
    }
    if w >= 0.0f64 {
        if isFlooded != 0 {
            (*StepQualTotals.offset(p as isize)).flooding += w;
        } else {
            (*StepQualTotals.offset(p as isize)).outflow += w;
        }
    } else {
        (*StepQualTotals.offset(p as isize)).exInflow -= w;
    };
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addReactedMass(
    mut p: ::core::ffi::c_int,
    mut w: ::core::ffi::c_double,
) {
    if p < 0 as ::core::ffi::c_int || p >= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        return;
    }
    (*StepQualTotals.offset(p as isize)).reacted += w;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addSeepageLoss(
    mut p: ::core::ffi::c_int,
    mut w: ::core::ffi::c_double,
) {
    if p < 0 as ::core::ffi::c_int || p >= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        return;
    }
    (*StepQualTotals.offset(p as isize)).seepLoss += w;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addToFinalStorage(
    mut p: ::core::ffi::c_int,
    mut w: ::core::ffi::c_double,
) {
    if p < 0 as ::core::ffi::c_int || p >= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        return;
    }
    (*StepQualTotals.offset(p as isize)).finalStorage += w;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addNodeLosses(
    mut evapLoss: ::core::ffi::c_double,
    mut seepLoss: ::core::ffi::c_double,
) {
    StepFlowTotals.evapLoss += evapLoss;
    StepFlowTotals.seepLoss += seepLoss;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_addLinkLosses(
    mut evapLoss: ::core::ffi::c_double,
    mut seepLoss: ::core::ffi::c_double,
) {
    StepFlowTotals.evapLoss += evapLoss;
    StepFlowTotals.seepLoss += seepLoss;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_updateRoutingTotals(mut tStep: ::core::ffi::c_double) {
    let mut j: ::core::ffi::c_int = 0;
    FlowTotals.dwInflow += StepFlowTotals.dwInflow * tStep;
    FlowTotals.wwInflow += StepFlowTotals.wwInflow * tStep;
    FlowTotals.gwInflow += StepFlowTotals.gwInflow * tStep;
    FlowTotals.iiInflow += StepFlowTotals.iiInflow * tStep;
    FlowTotals.exInflow += StepFlowTotals.exInflow * tStep;
    FlowTotals.flooding += StepFlowTotals.flooding * tStep;
    FlowTotals.outflow += StepFlowTotals.outflow * tStep;
    FlowTotals.evapLoss += StepFlowTotals.evapLoss * tStep;
    FlowTotals.seepLoss += StepFlowTotals.seepLoss * tStep;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        (*QualTotals.offset(j as isize)).dwInflow +=
            (*StepQualTotals.offset(j as isize)).dwInflow * tStep;
        (*QualTotals.offset(j as isize)).wwInflow +=
            (*StepQualTotals.offset(j as isize)).wwInflow * tStep;
        (*QualTotals.offset(j as isize)).gwInflow +=
            (*StepQualTotals.offset(j as isize)).gwInflow * tStep;
        (*QualTotals.offset(j as isize)).iiInflow +=
            (*StepQualTotals.offset(j as isize)).iiInflow * tStep;
        (*QualTotals.offset(j as isize)).exInflow +=
            (*StepQualTotals.offset(j as isize)).exInflow * tStep;
        (*QualTotals.offset(j as isize)).flooding +=
            (*StepQualTotals.offset(j as isize)).flooding * tStep;
        (*QualTotals.offset(j as isize)).outflow +=
            (*StepQualTotals.offset(j as isize)).outflow * tStep;
        (*QualTotals.offset(j as isize)).reacted +=
            (*StepQualTotals.offset(j as isize)).reacted * tStep;
        (*QualTotals.offset(j as isize)).seepLoss +=
            (*StepQualTotals.offset(j as isize)).seepLoss * tStep;
        (*QualTotals.offset(j as isize)).finalStorage +=
            (*StepQualTotals.offset(j as isize)).finalStorage;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        *NodeInflow.offset(j as isize) += (*Node.offset(j as isize)).inflow * tStep;
        if (*Node.offset(j as isize)).type_0 == OUTFALL as ::core::ffi::c_int
            || (*Node.offset(j as isize)).degree == 0 as ::core::ffi::c_int
                && (*Node.offset(j as isize)).type_0 != STORAGE as ::core::ffi::c_int
        {
            *NodeOutflow.offset(j as isize) += (*Node.offset(j as isize)).inflow * tStep;
        } else {
            *NodeOutflow.offset(j as isize) += (*Node.offset(j as isize)).outflow * tStep;
            if (*Node.offset(j as isize)).newVolume <= (*Node.offset(j as isize)).fullVolume {
                *NodeOutflow.offset(j as isize) += (*Node.offset(j as isize)).overflow * tStep;
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getStorage(
    mut isFinalStorage: ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = 0;
    let mut totalStorage: ::core::ffi::c_double = 0.0f64;
    let mut nodeStorage: ::core::ffi::c_double = 0.;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        nodeStorage = (*Node.offset(j as isize)).newVolume;
        if isFinalStorage != 0 {
            *NodeOutflow.offset(j as isize) += nodeStorage;
        }
        totalStorage += nodeStorage;
        j += 1;
    }
    if isFinalStorage as ::core::ffi::c_int != 0 && RouteModel == SF as ::core::ffi::c_int {
        return totalStorage;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        totalStorage += (*Link.offset(j as isize)).newVolume;
        j += 1;
    }
    return totalStorage;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getSysFlows(
    mut f: ::core::ffi::c_double,
    mut sysFlows: *mut ::core::ffi::c_double,
) {
    let mut f1: ::core::ffi::c_double = 1.0f64 - f;
    *sysFlows.offset(SYS_DWFLOW as ::core::ffi::c_int as isize) = (f1 * OldStepFlowTotals.dwInflow
        + f * StepFlowTotals.dwInflow)
        * UCF(FLOW as ::core::ffi::c_int);
    *sysFlows.offset(SYS_GWFLOW as ::core::ffi::c_int as isize) = (f1 * OldStepFlowTotals.gwInflow
        + f * StepFlowTotals.gwInflow)
        * UCF(FLOW as ::core::ffi::c_int);
    *sysFlows.offset(SYS_IIFLOW as ::core::ffi::c_int as isize) = (f1 * OldStepFlowTotals.iiInflow
        + f * StepFlowTotals.iiInflow)
        * UCF(FLOW as ::core::ffi::c_int);
    *sysFlows.offset(SYS_EXFLOW as ::core::ffi::c_int as isize) = (f1 * OldStepFlowTotals.exInflow
        + f * StepFlowTotals.exInflow)
        * UCF(FLOW as ::core::ffi::c_int);
    *sysFlows.offset(SYS_FLOODING as ::core::ffi::c_int as isize) =
        (f1 * OldStepFlowTotals.flooding + f * StepFlowTotals.flooding)
            * UCF(FLOW as ::core::ffi::c_int);
    *sysFlows.offset(SYS_OUTFLOW as ::core::ffi::c_int as isize) = (f1 * OldStepFlowTotals.outflow
        + f * StepFlowTotals.outflow)
        * UCF(FLOW as ::core::ffi::c_int);
    *sysFlows.offset(SYS_STORAGE as ::core::ffi::c_int as isize) =
        (f1 * OldStepFlowTotals.finalStorage + f * StepFlowTotals.finalStorage)
            * UCF(VOLUME as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getRunoffError() -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = 0;
    let mut totalInflow: ::core::ffi::c_double = 0.;
    let mut totalOutflow: ::core::ffi::c_double = 0.;
    RunoffTotals.finalStorage = 0.0f64;
    RunoffTotals.finalSnowCover = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        RunoffTotals.finalStorage += subcatch_getStorage(j);
        RunoffTotals.finalSnowCover += snow_getSnowCover(j);
        j += 1;
    }
    RunoffTotals.snowRemoved = Snow.removed;
    totalInflow = RunoffTotals.rainfall
        + RunoffTotals.runon
        + RunoffTotals.initStorage
        + RunoffTotals.initSnowCover;
    totalOutflow = RunoffTotals.evap
        + RunoffTotals.infil
        + RunoffTotals.runoff
        + RunoffTotals.drains
        + RunoffTotals.snowRemoved
        + RunoffTotals.finalStorage
        + RunoffTotals.finalSnowCover;
    RunoffTotals.pctError = 0.0f64;
    if fabs(totalInflow - totalOutflow) < 1.0f64 {
        RunoffTotals.pctError = TINY;
    } else if totalInflow > 0.0f64 {
        RunoffTotals.pctError = 100.0f64 * (1.0f64 - totalOutflow / totalInflow);
    } else if totalOutflow > 0.0f64 {
        RunoffTotals.pctError = 100.0f64 * (totalInflow / totalOutflow - 1.0f64);
    }
    RunoffError = RunoffTotals.pctError;
    return RunoffTotals.pctError;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getLoadingError() -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = 0;
    let mut loadIn: ::core::ffi::c_double = 0.;
    let mut loadOut: ::core::ffi::c_double = 0.;
    let mut maxError: ::core::ffi::c_double = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        (*LoadingTotals.offset(j as isize)).finalLoad += massbal_getBuildup(j);
        loadIn = (*LoadingTotals.offset(j as isize)).initLoad
            + (*LoadingTotals.offset(j as isize)).buildup
            + (*LoadingTotals.offset(j as isize)).deposition;
        loadOut = (*LoadingTotals.offset(j as isize)).sweeping
            + (*LoadingTotals.offset(j as isize)).infil
            + (*LoadingTotals.offset(j as isize)).bmpRemoval
            + (*LoadingTotals.offset(j as isize)).runoff
            + (*LoadingTotals.offset(j as isize)).finalLoad;
        (*LoadingTotals.offset(j as isize)).pctError = 0.0f64;
        if fabs(loadIn - loadOut) < 0.001f64 {
            (*LoadingTotals.offset(j as isize)).pctError = TINY;
        } else if loadIn > 0.0f64 {
            (*LoadingTotals.offset(j as isize)).pctError = 100.0f64 * (1.0f64 - loadOut / loadIn);
        } else if loadOut > 0.0f64 {
            (*LoadingTotals.offset(j as isize)).pctError = 100.0f64 * (loadIn / loadOut - 1.0f64);
        }
        maxError = if maxError >= (*LoadingTotals.offset(j as isize)).pctError {
            maxError
        } else {
            (*LoadingTotals.offset(j as isize)).pctError
        };
        if (*Pollut.offset(j as isize)).units == COUNT as ::core::ffi::c_int {
            (*LoadingTotals.offset(j as isize)).initLoad =
                if (*LoadingTotals.offset(j as isize)).initLoad > 0.0f64 {
                    log10((*LoadingTotals.offset(j as isize)).initLoad)
                } else {
                    (*LoadingTotals.offset(j as isize)).initLoad
                };
            (*LoadingTotals.offset(j as isize)).buildup =
                if (*LoadingTotals.offset(j as isize)).buildup > 0.0f64 {
                    log10((*LoadingTotals.offset(j as isize)).buildup)
                } else {
                    (*LoadingTotals.offset(j as isize)).buildup
                };
            (*LoadingTotals.offset(j as isize)).deposition =
                if (*LoadingTotals.offset(j as isize)).deposition > 0.0f64 {
                    log10((*LoadingTotals.offset(j as isize)).deposition)
                } else {
                    (*LoadingTotals.offset(j as isize)).deposition
                };
            (*LoadingTotals.offset(j as isize)).sweeping =
                if (*LoadingTotals.offset(j as isize)).sweeping > 0.0f64 {
                    log10((*LoadingTotals.offset(j as isize)).sweeping)
                } else {
                    (*LoadingTotals.offset(j as isize)).sweeping
                };
            (*LoadingTotals.offset(j as isize)).infil =
                if (*LoadingTotals.offset(j as isize)).infil > 0.0f64 {
                    log10((*LoadingTotals.offset(j as isize)).infil)
                } else {
                    (*LoadingTotals.offset(j as isize)).infil
                };
            (*LoadingTotals.offset(j as isize)).bmpRemoval =
                if (*LoadingTotals.offset(j as isize)).bmpRemoval > 0.0f64 {
                    log10((*LoadingTotals.offset(j as isize)).bmpRemoval)
                } else {
                    (*LoadingTotals.offset(j as isize)).bmpRemoval
                };
            (*LoadingTotals.offset(j as isize)).runoff =
                if (*LoadingTotals.offset(j as isize)).runoff > 0.0f64 {
                    log10((*LoadingTotals.offset(j as isize)).runoff)
                } else {
                    (*LoadingTotals.offset(j as isize)).runoff
                };
            (*LoadingTotals.offset(j as isize)).finalLoad =
                if (*LoadingTotals.offset(j as isize)).finalLoad > 0.0f64 {
                    log10((*LoadingTotals.offset(j as isize)).finalLoad)
                } else {
                    (*LoadingTotals.offset(j as isize)).finalLoad
                };
        }
        j += 1;
    }
    return maxError;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getGwaterError() -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = 0;
    let mut totalInflow: ::core::ffi::c_double = 0.;
    let mut totalOutflow: ::core::ffi::c_double = 0.;
    GwaterTotals.finalStorage = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        GwaterTotals.finalStorage += gwater_getVolume(j) * (*Subcatch.offset(j as isize)).area;
        j += 1;
    }
    totalInflow = GwaterTotals.infil + GwaterTotals.initStorage;
    totalOutflow = GwaterTotals.upperEvap
        + GwaterTotals.lowerEvap
        + GwaterTotals.lowerPerc
        + GwaterTotals.gwater
        + GwaterTotals.finalStorage;
    GwaterTotals.pctError = 0.0f64;
    if fabs(totalInflow - totalOutflow) < 1.0f64 {
        GwaterTotals.pctError = TINY;
    } else if totalInflow > 0.0f64 {
        GwaterTotals.pctError = 100.0f64 * (1.0f64 - totalOutflow / totalInflow);
    } else if totalOutflow > 0.0f64 {
        GwaterTotals.pctError = 100.0f64 * (totalInflow / totalOutflow - 1.0f64);
    }
    GwaterError = GwaterTotals.pctError;
    return GwaterTotals.pctError;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getFlowError() -> ::core::ffi::c_double {
    let mut totalInflow: ::core::ffi::c_double = 0.;
    let mut totalOutflow: ::core::ffi::c_double = 0.;
    FlowTotals.finalStorage = massbal_getStorage(TRUE as ::core::ffi::c_char);
    totalInflow = FlowTotals.initStorage + FlowTotals.wwInflow + FlowTotals.iiInflow;
    totalOutflow = FlowTotals.finalStorage
        + FlowTotals.flooding
        + FlowTotals.evapLoss
        + FlowTotals.seepLoss
        + FlowTotals.reacted;
    if FlowTotals.dwInflow >= 0.0f64 {
        totalInflow += FlowTotals.dwInflow;
    } else {
        totalOutflow -= FlowTotals.dwInflow;
    }
    if FlowTotals.gwInflow >= 0.0f64 {
        totalInflow += FlowTotals.gwInflow;
    } else {
        totalOutflow -= FlowTotals.gwInflow;
    }
    if FlowTotals.exInflow >= 0.0f64 {
        totalInflow += FlowTotals.exInflow;
    } else {
        totalOutflow -= FlowTotals.exInflow;
    }
    if FlowTotals.outflow >= 0.0f64 {
        totalOutflow += FlowTotals.outflow;
    } else {
        totalInflow -= FlowTotals.outflow;
    }
    FlowTotals.pctError = 0.0f64;
    if fabs(totalInflow - totalOutflow) < 1.0f64 {
        FlowTotals.pctError = TINY;
    } else if fabs(totalInflow) > 0.0f64 {
        FlowTotals.pctError = 100.0f64 * (1.0f64 - totalOutflow / totalInflow);
    } else if fabs(totalOutflow) > 0.0f64 {
        FlowTotals.pctError = 100.0f64 * (totalInflow / totalOutflow - 1.0f64);
    }
    FlowError = FlowTotals.pctError;
    return FlowTotals.pctError;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getQualError() -> ::core::ffi::c_double {
    let mut p: ::core::ffi::c_int = 0;
    let mut maxQualError: ::core::ffi::c_double = 0.0f64;
    let mut totalInflow: ::core::ffi::c_double = 0.;
    let mut totalOutflow: ::core::ffi::c_double = 0.;
    let mut cf: ::core::ffi::c_double = 0.;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        (*QualTotals.offset(p as isize)).finalStorage += massbal_getStoredMass(p);
        totalInflow = (*QualTotals.offset(p as isize)).dwInflow
            + (*QualTotals.offset(p as isize)).wwInflow
            + (*QualTotals.offset(p as isize)).gwInflow
            + (*QualTotals.offset(p as isize)).iiInflow
            + (*QualTotals.offset(p as isize)).exInflow
            + (*QualTotals.offset(p as isize)).initStorage;
        totalOutflow = (*QualTotals.offset(p as isize)).flooding
            + (*QualTotals.offset(p as isize)).outflow
            + (*QualTotals.offset(p as isize)).reacted
            + (*QualTotals.offset(p as isize)).seepLoss
            + (*QualTotals.offset(p as isize)).finalStorage;
        (*QualTotals.offset(p as isize)).pctError = 0.0f64;
        if fabs(totalInflow - totalOutflow) < 0.001f64 {
            (*QualTotals.offset(p as isize)).pctError = TINY;
        } else if totalInflow > 0.0f64 {
            (*QualTotals.offset(p as isize)).pctError =
                100.0f64 * (1.0f64 - totalOutflow / totalInflow);
        } else if totalOutflow > 0.0f64 {
            (*QualTotals.offset(p as isize)).pctError =
                100.0f64 * (totalInflow / totalOutflow - 1.0f64);
        }
        if fabs((*QualTotals.offset(p as isize)).pctError) > fabs(maxQualError) {
            maxQualError = (*QualTotals.offset(p as isize)).pctError;
        }
        cf = LperFT3;
        if (*Pollut.offset(p as isize)).units == COUNT as ::core::ffi::c_int {
            (*QualTotals.offset(p as isize)).dwInflow =
                if cf * (*QualTotals.offset(p as isize)).dwInflow > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).dwInflow)
                } else {
                    cf * (*QualTotals.offset(p as isize)).dwInflow
                };
            (*QualTotals.offset(p as isize)).wwInflow =
                if cf * (*QualTotals.offset(p as isize)).wwInflow > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).wwInflow)
                } else {
                    cf * (*QualTotals.offset(p as isize)).wwInflow
                };
            (*QualTotals.offset(p as isize)).gwInflow =
                if cf * (*QualTotals.offset(p as isize)).gwInflow > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).gwInflow)
                } else {
                    cf * (*QualTotals.offset(p as isize)).gwInflow
                };
            (*QualTotals.offset(p as isize)).iiInflow =
                if cf * (*QualTotals.offset(p as isize)).iiInflow > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).iiInflow)
                } else {
                    cf * (*QualTotals.offset(p as isize)).iiInflow
                };
            (*QualTotals.offset(p as isize)).exInflow =
                if cf * (*QualTotals.offset(p as isize)).exInflow > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).exInflow)
                } else {
                    cf * (*QualTotals.offset(p as isize)).exInflow
                };
            (*QualTotals.offset(p as isize)).flooding =
                if cf * (*QualTotals.offset(p as isize)).flooding > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).flooding)
                } else {
                    cf * (*QualTotals.offset(p as isize)).flooding
                };
            (*QualTotals.offset(p as isize)).outflow =
                if cf * (*QualTotals.offset(p as isize)).outflow > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).outflow)
                } else {
                    cf * (*QualTotals.offset(p as isize)).outflow
                };
            (*QualTotals.offset(p as isize)).reacted =
                if cf * (*QualTotals.offset(p as isize)).reacted > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).reacted)
                } else {
                    cf * (*QualTotals.offset(p as isize)).reacted
                };
            (*QualTotals.offset(p as isize)).seepLoss =
                if cf * (*QualTotals.offset(p as isize)).seepLoss > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).seepLoss)
                } else {
                    cf * (*QualTotals.offset(p as isize)).seepLoss
                };
            (*QualTotals.offset(p as isize)).initStorage =
                if cf * (*QualTotals.offset(p as isize)).initStorage > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).initStorage)
                } else {
                    cf * (*QualTotals.offset(p as isize)).initStorage
                };
            (*QualTotals.offset(p as isize)).finalStorage =
                if cf * (*QualTotals.offset(p as isize)).finalStorage > 0.0f64 {
                    log10(cf * (*QualTotals.offset(p as isize)).finalStorage)
                } else {
                    cf * (*QualTotals.offset(p as isize)).finalStorage
                };
        } else {
            cf = cf * UCF(MASS as ::core::ffi::c_int);
            if (*Pollut.offset(p as isize)).units == UG as ::core::ffi::c_int {
                cf /= 1000.0f64;
            }
            (*QualTotals.offset(p as isize)).dwInflow *= cf;
            (*QualTotals.offset(p as isize)).wwInflow *= cf;
            (*QualTotals.offset(p as isize)).gwInflow *= cf;
            (*QualTotals.offset(p as isize)).iiInflow *= cf;
            (*QualTotals.offset(p as isize)).exInflow *= cf;
            (*QualTotals.offset(p as isize)).flooding *= cf;
            (*QualTotals.offset(p as isize)).outflow *= cf;
            (*QualTotals.offset(p as isize)).reacted *= cf;
            (*QualTotals.offset(p as isize)).seepLoss *= cf;
            (*QualTotals.offset(p as isize)).initStorage *= cf;
            (*QualTotals.offset(p as isize)).finalStorage *= cf;
        }
        p += 1;
    }
    QualError = maxQualError;
    return maxQualError;
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getStepFlowError() -> ::core::ffi::c_double {
    let mut totalInflow: ::core::ffi::c_double = 0.;
    let mut totalOutflow: ::core::ffi::c_double = 0.;
    totalInflow = StepFlowTotals.dwInflow
        + StepFlowTotals.wwInflow
        + StepFlowTotals.gwInflow
        + StepFlowTotals.iiInflow
        + StepFlowTotals.exInflow;
    totalOutflow = StepFlowTotals.flooding
        + StepFlowTotals.outflow
        + StepFlowTotals.evapLoss
        + StepFlowTotals.seepLoss
        + StepFlowTotals.reacted;
    if fabs(totalInflow) > 0.0f64 {
        return 1.0f64 - totalOutflow / totalInflow;
    } else if fabs(totalOutflow) > 0.0f64 {
        return totalInflow / totalOutflow - 1.0f64;
    } else {
        return 0.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn massbal_getStoredMass(mut p: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = 0;
    let mut storedMass: ::core::ffi::c_double = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        storedMass += (*Node.offset(j as isize)).newVolume
            * *(*Node.offset(j as isize)).newQual.offset(p as isize);
        j += 1;
    }
    if RouteModel != SF as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
            storedMass += (*Link.offset(j as isize)).newVolume
                * *(*Link.offset(j as isize)).newQual.offset(p as isize);
            j += 1;
        }
    }
    return storedMass;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
