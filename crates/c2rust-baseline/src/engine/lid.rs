#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn infil_getInfil(
        area: ::core::ffi::c_int,
        tstep: ::core::ffi::c_double,
        rainfall: ::core::ffi::c_double,
        runon: ::core::ffi::c_double,
        depth: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn grnampt_getParams(j: ::core::ffi::c_int, p: *mut ::core::ffi::c_double);
    fn grnampt_setParams(infil: *mut TGrnAmpt, p: *mut ::core::ffi::c_double)
        -> ::core::ffi::c_int;
    fn grnampt_initState(infil: *mut TGrnAmpt);
    static mut Frpt: TFile;
    static mut Msg: [::core::ffi::c_char; 1025];
    static mut Title: [[::core::ffi::c_char; 1025]; 3];
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut UnitSystem: ::core::ffi::c_int;
    static mut IgnoreGwater: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut StartDryDays: ::core::ffi::c_double;
    static mut Evap: TEvap;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Pollut: *mut TPollut;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn massbal_updateRunoffTotals(type_0: ::core::ffi::c_int, v: ::core::ffi::c_double);
    fn massbal_updateLoadingTotals(
        type_0: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
        w: ::core::ffi::c_double,
    );
    fn massbal_addInflowFlow(type_0: ::core::ffi::c_int, q: ::core::ffi::c_double);
    fn massbal_addInflowQual(
        type_0: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
        w: ::core::ffi::c_double,
    );
    fn subcatch_addRunonFlow(subcatch: ::core::ffi::c_int, flow: ::core::ffi::c_double);
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
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut Vevap: ::core::ffi::c_double;
    static mut Vpevap: ::core::ffi::c_double;
    static mut Vinfil: ::core::ffi::c_double;
    static mut VlidInfil: ::core::ffi::c_double;
    static mut VlidIn: ::core::ffi::c_double;
    static mut VlidOut: ::core::ffi::c_double;
    static mut VlidDrain: ::core::ffi::c_double;
    static mut VlidReturn: ::core::ffi::c_double;
    static mut HasWetLids: ::core::ffi::c_char;
    fn lidproc_initWaterBalance(lidUnit: *mut TLidUnit, initVol: ::core::ffi::c_double);
    fn lidproc_getOutflow(
        lidUnit: *mut TLidUnit,
        lidProc: *mut TLidProc,
        inflow: ::core::ffi::c_double,
        evap: ::core::ffi::c_double,
        infil: ::core::ffi::c_double,
        maxInfil: ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
        lidEvap: *mut ::core::ffi::c_double,
        lidInfil: *mut ::core::ffi::c_double,
        lidDrain: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn lidproc_saveResults(
        lidUnit: *mut TLidUnit,
        ucfRainfall: ::core::ffi::c_double,
        ucfRainDepth: ::core::ffi::c_double,
    );
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
pub type SubAreaType = ::core::ffi::c_uint;
pub const PERV: SubAreaType = 2;
pub const IMPERV1: SubAreaType = 1;
pub const IMPERV0: SubAreaType = 0;
pub type RunoffRoutingType = ::core::ffi::c_uint;
pub const TO_PERV: RunoffRoutingType = 2;
pub const TO_IMPERV: RunoffRoutingType = 1;
pub const TO_OUTLET: RunoffRoutingType = 0;
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
pub type InfilType = ::core::ffi::c_uint;
pub const CURVE_NUMBER: InfilType = 4;
pub const MOD_GREEN_AMPT: InfilType = 3;
pub const GREEN_AMPT: InfilType = 2;
pub const MOD_HORTON: InfilType = 1;
pub const HORTON: InfilType = 0;
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
pub struct TFile {
    pub name: [::core::ffi::c_char; 260],
    pub mode: ::core::ffi::c_char,
    pub state: ::core::ffi::c_char,
    pub file: *mut FILE,
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NONE: ErrorType = 0;
pub type LidTypes = ::core::ffi::c_uint;
pub const ROOF_DISCON: LidTypes = 7;
pub const VEG_SWALE: LidTypes = 6;
pub const RAIN_BARREL: LidTypes = 5;
pub const POROUS_PAVEMENT: LidTypes = 4;
pub const INFIL_TRENCH: LidTypes = 3;
pub const GREEN_ROOF: LidTypes = 2;
pub const RAIN_GARDEN: LidTypes = 1;
pub const BIO_CELL: LidTypes = 0;
pub type TimePeriod = ::core::ffi::c_uint;
pub const CURRENT: TimePeriod = 1;
pub const PREVIOUS: TimePeriod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSurfaceLayer {
    pub thickness: ::core::ffi::c_double,
    pub voidFrac: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub surfSlope: ::core::ffi::c_double,
    pub sideSlope: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub canOverflow: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPavementLayer {
    pub thickness: ::core::ffi::c_double,
    pub voidFrac: ::core::ffi::c_double,
    pub impervFrac: ::core::ffi::c_double,
    pub kSat: ::core::ffi::c_double,
    pub clogFactor: ::core::ffi::c_double,
    pub regenDays: ::core::ffi::c_double,
    pub regenDegree: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSoilLayer {
    pub thickness: ::core::ffi::c_double,
    pub porosity: ::core::ffi::c_double,
    pub fieldCap: ::core::ffi::c_double,
    pub wiltPoint: ::core::ffi::c_double,
    pub suction: ::core::ffi::c_double,
    pub kSat: ::core::ffi::c_double,
    pub kSlope: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TStorageLayer {
    pub thickness: ::core::ffi::c_double,
    pub voidFrac: ::core::ffi::c_double,
    pub kSat: ::core::ffi::c_double,
    pub clogFactor: ::core::ffi::c_double,
    pub covered: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TDrainLayer {
    pub coeff: ::core::ffi::c_double,
    pub expon: ::core::ffi::c_double,
    pub offset: ::core::ffi::c_double,
    pub delay: ::core::ffi::c_double,
    pub hOpen: ::core::ffi::c_double,
    pub hClose: ::core::ffi::c_double,
    pub qCurve: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TDrainMatLayer {
    pub thickness: ::core::ffi::c_double,
    pub voidFrac: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLidProc {
    pub ID: *mut ::core::ffi::c_char,
    pub lidType: ::core::ffi::c_int,
    pub surface: TSurfaceLayer,
    pub pavement: TPavementLayer,
    pub soil: TSoilLayer,
    pub storage: TStorageLayer,
    pub drain: TDrainLayer,
    pub drainMat: TDrainMatLayer,
    pub drainRmvl: *mut ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TWaterBalance {
    pub inflow: ::core::ffi::c_double,
    pub evap: ::core::ffi::c_double,
    pub infil: ::core::ffi::c_double,
    pub surfFlow: ::core::ffi::c_double,
    pub drainFlow: ::core::ffi::c_double,
    pub initVol: ::core::ffi::c_double,
    pub finalVol: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLidRptFile {
    pub file: *mut FILE,
    pub wasDry: ::core::ffi::c_int,
    pub results: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLidUnit {
    pub lidIndex: ::core::ffi::c_int,
    pub number: ::core::ffi::c_int,
    pub area: ::core::ffi::c_double,
    pub fullWidth: ::core::ffi::c_double,
    pub botWidth: ::core::ffi::c_double,
    pub initSat: ::core::ffi::c_double,
    pub fromImperv: ::core::ffi::c_double,
    pub fromPerv: ::core::ffi::c_double,
    pub toPerv: ::core::ffi::c_int,
    pub drainSubcatch: ::core::ffi::c_int,
    pub drainNode: ::core::ffi::c_int,
    pub rptFile: *mut TLidRptFile,
    pub soilInfil: TGrnAmpt,
    pub surfaceDepth: ::core::ffi::c_double,
    pub paveDepth: ::core::ffi::c_double,
    pub soilMoisture: ::core::ffi::c_double,
    pub storageDepth: ::core::ffi::c_double,
    pub oldFluxRates: [::core::ffi::c_double; 4],
    pub dryTime: ::core::ffi::c_double,
    pub oldDrainFlow: ::core::ffi::c_double,
    pub newDrainFlow: ::core::ffi::c_double,
    pub volTreated: ::core::ffi::c_double,
    pub nextRegenDay: ::core::ffi::c_double,
    pub waterBalance: TWaterBalance,
}
pub type TLidGroup = *mut LidGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LidGroup {
    pub pervArea: ::core::ffi::c_double,
    pub flowToPerv: ::core::ffi::c_double,
    pub oldDrainFlow: ::core::ffi::c_double,
    pub newDrainFlow: ::core::ffi::c_double,
    pub lidList: *mut TLidList,
}
pub type TLidList = LidList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LidList {
    pub lidUnit: *mut TLidUnit,
    pub nextLidUnit: *mut LidList,
}
pub const REMOVALS: LidLayerTypes = 6;
pub const DRAINMAT: LidLayerTypes = 4;
pub const DRAIN: LidLayerTypes = 5;
pub const PAVE: LidLayerTypes = 3;
pub const STOR: LidLayerTypes = 2;
pub const SOIL: LidLayerTypes = 1;
pub const SURF: LidLayerTypes = 0;
pub type LidLayerTypes = ::core::ffi::c_uint;
pub const w_YES: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"YES\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const ERR_PAVE_LAYER: [::core::ffi::c_char; 35] = unsafe {
    ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
        *b" - check pavement layer parameters\0",
    )
};
pub const ERR_SOIL_LAYER: [::core::ffi::c_char; 31] = unsafe {
    ::core::mem::transmute::<[u8; 31], [::core::ffi::c_char; 31]>(
        *b" - check soil layer parameters\0",
    )
};
pub const ERR_STOR_LAYER: [::core::ffi::c_char; 34] = unsafe {
    ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
        *b" - check storage layer parameters\0",
    )
};
pub const ERR_SWALE_SURF: [::core::ffi::c_char; 34] = unsafe {
    ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
        *b" - check swale surface parameters\0",
    )
};
pub const ERR_GREEN_AMPT: [::core::ffi::c_char; 44] = unsafe {
    ::core::mem::transmute::<[u8; 44], [::core::ffi::c_char; 44]>(
        *b" - check subcatchment Green-Ampt parameters\0",
    )
};
pub const ERR_DRAIN_HEADS: [::core::ffi::c_char; 35] = unsafe {
    ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
        *b" - invalid drain open/closed heads\0",
    )
};
pub const ERR_SWALE_WIDTH: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b" - invalid swale width\0")
};
#[no_mangle]
pub static mut LidLayerWords: [*mut ::core::ffi::c_char; 8] = [
    b"SURFACE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SOIL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"STORAGE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"PAVEMENT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"DRAINMAT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"DRAIN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"REMOVALS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut LidTypeWords: [*mut ::core::ffi::c_char; 9] = [
    b"BC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"RG\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"GR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"IT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"PP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"RB\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"VS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"RD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut LidProcs: *mut TLidProc = ::core::ptr::null::<TLidProc>() as *mut TLidProc;
static mut LidCount: ::core::ffi::c_int = 0;
static mut LidGroups: *mut TLidGroup = ::core::ptr::null::<TLidGroup>() as *mut TLidGroup;
static mut GroupCount: ::core::ffi::c_int = 0;
static mut EvapRate: ::core::ffi::c_double = 0.;
static mut NativeInfil: ::core::ffi::c_double = 0.;
static mut MaxNativeInfil: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn lid_create(
    mut lidCount: ::core::ffi::c_int,
    mut subcatchCount: ::core::ffi::c_int,
) {
    let mut j: ::core::ffi::c_int = 0;
    LidProcs = ::core::ptr::null_mut::<TLidProc>();
    LidGroups = ::core::ptr::null_mut::<TLidGroup>();
    LidCount = lidCount;
    GroupCount = subcatchCount;
    if GroupCount > 0 as ::core::ffi::c_int {
        LidGroups = calloc(
            GroupCount as size_t,
            ::core::mem::size_of::<TLidGroup>() as size_t,
        ) as *mut TLidGroup;
        if LidGroups.is_null() {
            ErrorCode = ERR_MEMORY as ::core::ffi::c_int;
            return;
        }
    }
    j = 0 as ::core::ffi::c_int;
    while j < GroupCount {
        let ref mut fresh0 = *LidGroups.offset(j as isize);
        *fresh0 = ::core::ptr::null_mut::<LidGroup>();
        j += 1;
    }
    if LidCount == 0 as ::core::ffi::c_int {
        return;
    }
    LidProcs = calloc(
        LidCount as size_t,
        ::core::mem::size_of::<TLidProc>() as size_t,
    ) as *mut TLidProc;
    if LidProcs.is_null() {
        ErrorCode = ERR_MEMORY as ::core::ffi::c_int;
        return;
    }
    j = 0 as ::core::ffi::c_int;
    while j < LidCount {
        (*LidProcs.offset(j as isize)).lidType = -(1 as ::core::ffi::c_int);
        (*LidProcs.offset(j as isize)).surface.thickness = 0.0f64;
        (*LidProcs.offset(j as isize)).surface.voidFrac = 1.0f64;
        (*LidProcs.offset(j as isize)).surface.roughness = 0.0f64;
        (*LidProcs.offset(j as isize)).surface.surfSlope = 0.0f64;
        (*LidProcs.offset(j as isize)).pavement.thickness = 0.0f64;
        (*LidProcs.offset(j as isize)).soil.thickness = 0.0f64;
        (*LidProcs.offset(j as isize)).storage.thickness = 0.0f64;
        (*LidProcs.offset(j as isize)).storage.kSat = 0.0f64;
        (*LidProcs.offset(j as isize)).drain.coeff = 0.0f64;
        (*LidProcs.offset(j as isize)).drain.offset = 0.0f64;
        (*LidProcs.offset(j as isize)).drainMat.thickness = 0.0f64;
        (*LidProcs.offset(j as isize)).drainMat.roughness = 0.0f64;
        let ref mut fresh1 = (*LidProcs.offset(j as isize)).drainRmvl;
        *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_double>();
        let ref mut fresh2 = (*LidProcs.offset(j as isize)).drainRmvl;
        *fresh2 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        if (*LidProcs.offset(j as isize)).drainRmvl.is_null() {
            ErrorCode = ERR_MEMORY as ::core::ffi::c_int;
            return;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lid_delete() {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < GroupCount {
        freeLidGroup(j);
        j += 1;
    }
    if !LidGroups.is_null() {
        free(LidGroups as *mut ::core::ffi::c_void);
        LidGroups = ::core::ptr::null_mut::<TLidGroup>();
    }
    j = 0 as ::core::ffi::c_int;
    while j < LidCount {
        if !(*LidProcs.offset(j as isize)).drainRmvl.is_null() {
            free((*LidProcs.offset(j as isize)).drainRmvl as *mut ::core::ffi::c_void);
            let ref mut fresh3 = (*LidProcs.offset(j as isize)).drainRmvl;
            *fresh3 = ::core::ptr::null_mut::<::core::ffi::c_double>();
        }
        j += 1;
    }
    if !LidProcs.is_null() {
        free(LidProcs as *mut ::core::ffi::c_void);
        LidProcs = ::core::ptr::null_mut::<TLidProc>();
    }
    GroupCount = 0 as ::core::ffi::c_int;
    LidCount = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn freeLidGroup(mut j: ::core::ffi::c_int) {
    let mut lidGroup: TLidGroup = *LidGroups.offset(j as isize);
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut nextLidUnit: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    if lidGroup.is_null() {
        return;
    }
    lidList = (*lidGroup).lidList;
    while !lidList.is_null() {
        lidUnit = (*lidList).lidUnit;
        if !(*lidUnit).rptFile.is_null() {
            if !(*(*lidUnit).rptFile).file.is_null() {
                fclose((*(*lidUnit).rptFile).file);
            }
            free((*lidUnit).rptFile as *mut ::core::ffi::c_void);
        }
        nextLidUnit = (*lidList).nextLidUnit as *mut TLidList;
        free(lidUnit as *mut ::core::ffi::c_void);
        free(lidList as *mut ::core::ffi::c_void);
        lidList = nextLidUnit;
    }
    free(lidGroup as *mut ::core::ffi::c_void);
    let ref mut fresh4 = *LidGroups.offset(j as isize);
    *fresh4 = ::core::ptr::null_mut::<LidGroup>();
}
#[no_mangle]
pub unsafe extern "C" fn lid_readProcParams(
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    if ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        LID as ::core::ffi::c_int,
        *toks.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *toks.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if (*LidProcs.offset(j as isize)).ID.is_null() {
        let ref mut fresh5 = (*LidProcs.offset(j as isize)).ID;
        *fresh5 = project_findID(
            LID as ::core::ffi::c_int,
            *toks.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    m = findmatch(
        *toks.offset(1 as ::core::ffi::c_int as isize),
        &raw mut LidTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if m >= 0 as ::core::ffi::c_int {
        (*LidProcs.offset(j as isize)).lidType = m;
        return 0 as ::core::ffi::c_int;
    } else {
        m = findmatch(
            *toks.offset(1 as ::core::ffi::c_int as isize),
            &raw mut LidLayerWords as *mut *mut ::core::ffi::c_char,
        );
    }
    match m {
        0 => return readSurfaceData(j, toks, ntoks),
        1 => return readSoilData(j, toks, ntoks),
        2 => return readStorageData(j, toks, ntoks),
        3 => return readPavementData(j, toks, ntoks),
        5 => return readDrainData(j, toks, ntoks),
        4 => return readDrainMatData(j, toks, ntoks),
        6 => return readRemovalsData(j, toks, ntoks),
        _ => {}
    }
    return error_setInpError(
        ERR_KEYWORD as ::core::ffi::c_int,
        *toks.offset(1 as ::core::ffi::c_int as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lid_readGroupParams(
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    let mut fname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut drainSubcatch: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut drainNode: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if ntoks < 8 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        SUBCATCH as ::core::ffi::c_int,
        *toks.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *toks.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    k = project_findObject(
        LID as ::core::ffi::c_int,
        *toks.offset(1 as ::core::ffi::c_int as isize),
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *toks.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n = atoi(*toks.offset(2 as ::core::ffi::c_int as isize));
    if n < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *toks.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if n == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    i = 3 as ::core::ffi::c_int;
    while i <= 7 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 3 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[(i - 3 as ::core::ffi::c_int) as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        i += 1;
    }
    i = 2 as ::core::ffi::c_int;
    while i <= 3 as ::core::ffi::c_int {
        if x[i as usize] > 100.0f64 {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *toks.offset((i + 3 as ::core::ffi::c_int) as isize),
            );
        }
        i += 1;
    }
    if ntoks >= 9 as ::core::ffi::c_int
        && strcmp(
            *toks.offset(8 as ::core::ffi::c_int as isize),
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        fname = *toks.offset(8 as ::core::ffi::c_int as isize);
    }
    if ntoks >= 10 as ::core::ffi::c_int
        && strcmp(
            *toks.offset(9 as ::core::ffi::c_int as isize),
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        drainSubcatch = project_findObject(
            SUBCATCH as ::core::ffi::c_int,
            *toks.offset(9 as ::core::ffi::c_int as isize),
        );
        if drainSubcatch < 0 as ::core::ffi::c_int {
            drainNode = project_findObject(
                NODE as ::core::ffi::c_int,
                *toks.offset(9 as ::core::ffi::c_int as isize),
            );
            if drainNode < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *toks.offset(9 as ::core::ffi::c_int as isize),
                );
            }
        }
    }
    x[5 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks >= 11 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(10 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(5 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[5 as ::core::ffi::c_int as usize] < 0.0f64
            || x[5 as ::core::ffi::c_int as usize] > 100.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *toks.offset(10 as ::core::ffi::c_int as isize),
            );
        }
    }
    return addLidUnit(
        j,
        k,
        n,
        &raw mut x as *mut ::core::ffi::c_double,
        fname,
        drainSubcatch,
        drainNode,
    );
}
unsafe extern "C" fn addLidUnit(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
    mut fname: *mut ::core::ffi::c_char,
    mut drainSubcatch: ::core::ffi::c_int,
    mut drainNode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    lidGroup = *LidGroups.offset(j as isize);
    if lidGroup.is_null() {
        lidGroup =
            malloc(::core::mem::size_of::<LidGroup>() as size_t) as *mut LidGroup as TLidGroup;
        if lidGroup.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        (*lidGroup).lidList = ::core::ptr::null_mut::<TLidList>();
        let ref mut fresh6 = *LidGroups.offset(j as isize);
        *fresh6 = lidGroup;
    }
    lidUnit = malloc(::core::mem::size_of::<TLidUnit>() as size_t) as *mut TLidUnit;
    if lidUnit.is_null() {
        return error_setInpError(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    (*lidUnit).rptFile = ::core::ptr::null_mut::<TLidRptFile>();
    lidList = malloc(::core::mem::size_of::<TLidList>() as size_t) as *mut TLidList;
    if lidList.is_null() {
        free(lidUnit as *mut ::core::ffi::c_void);
        return error_setInpError(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    (*lidList).lidUnit = lidUnit;
    (*lidList).nextLidUnit = (*lidGroup).lidList as *mut LidList;
    (*lidGroup).lidList = lidList;
    (*lidUnit).lidIndex = k;
    (*lidUnit).number = n;
    (*lidUnit).area = *x.offset(0 as ::core::ffi::c_int as isize)
        / (UCF(LENGTH as ::core::ffi::c_int) * UCF(LENGTH as ::core::ffi::c_int));
    (*lidUnit).fullWidth =
        *x.offset(1 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
    (*lidUnit).initSat = *x.offset(2 as ::core::ffi::c_int as isize) / 100.0f64;
    (*lidUnit).fromImperv = *x.offset(3 as ::core::ffi::c_int as isize) / 100.0f64;
    (*lidUnit).toPerv =
        (*x.offset(4 as ::core::ffi::c_int as isize) > 0.0f64) as ::core::ffi::c_int;
    (*lidUnit).fromPerv = *x.offset(5 as ::core::ffi::c_int as isize) / 100.0f64;
    (*lidUnit).drainSubcatch = drainSubcatch;
    (*lidUnit).drainNode = drainNode;
    if !fname.is_null() {
        if createLidRptFile(lidUnit, fname) == 0 {
            return error_setInpError(ERR_RPT_FILE as ::core::ffi::c_int, fname);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn createLidRptFile(
    mut lidUnit: *mut TLidUnit,
    mut fname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rptFile: *mut TLidRptFile = ::core::ptr::null_mut::<TLidRptFile>();
    rptFile = malloc(::core::mem::size_of::<TLidRptFile>() as size_t) as *mut TLidRptFile;
    if rptFile.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*lidUnit).rptFile = rptFile;
    (*rptFile).file = fopen(fname, b"wt\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if (*rptFile).file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn readSurfaceData(
    mut j: ::core::ffi::c_int,
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 5] = [0.; 5];
    if ntoks < 7 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 2 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[(i - 2 as ::core::ffi::c_int) as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        i += 1;
    }
    if x[1 as ::core::ffi::c_int as usize] >= 1.0f64 {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *toks.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    if x[0 as ::core::ffi::c_int as usize] == 0.0f64 {
        x[1 as ::core::ffi::c_int as usize] = 0.0f64;
    }
    (*LidProcs.offset(j as isize)).surface.thickness =
        x[0 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).surface.voidFrac = 1.0f64 - x[1 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).surface.roughness = x[2 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).surface.surfSlope =
        x[3 as ::core::ffi::c_int as usize] / 100.0f64;
    (*LidProcs.offset(j as isize)).surface.sideSlope = x[4 as ::core::ffi::c_int as usize];
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readPavementData(
    mut j: ::core::ffi::c_int,
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 7] = [0.; 7];
    if ntoks < 7 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 2 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[(i - 2 as ::core::ffi::c_int) as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        i += 1;
    }
    x[5 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks > 7 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(7 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(5 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[5 as ::core::ffi::c_int as usize] < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *toks.offset(7 as ::core::ffi::c_int as isize),
            );
        }
    }
    x[6 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks > 8 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(8 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(6 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[6 as ::core::ffi::c_int as usize] < 0.0f64
            || x[6 as ::core::ffi::c_int as usize] > 1.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *toks.offset(8 as ::core::ffi::c_int as isize),
            );
        }
    }
    x[1 as ::core::ffi::c_int as usize] =
        x[1 as ::core::ffi::c_int as usize] / (x[1 as ::core::ffi::c_int as usize] + 1.0f64);
    (*LidProcs.offset(j as isize)).pavement.thickness =
        x[0 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).pavement.voidFrac = x[1 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).pavement.impervFrac = x[2 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).pavement.kSat =
        x[3 as ::core::ffi::c_int as usize] / UCF(RAINFALL as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).pavement.clogFactor = x[4 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).pavement.regenDays = x[5 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).pavement.regenDegree = x[6 as ::core::ffi::c_int as usize];
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readSoilData(
    mut j: ::core::ffi::c_int,
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 7] = [0.; 7];
    if ntoks < 9 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 2 as ::core::ffi::c_int;
    while i < 9 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[(i - 2 as ::core::ffi::c_int) as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        i += 1;
    }
    (*LidProcs.offset(j as isize)).soil.thickness =
        x[0 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).soil.porosity = x[1 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).soil.fieldCap = x[2 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).soil.wiltPoint = x[3 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).soil.kSat =
        x[4 as ::core::ffi::c_int as usize] / UCF(RAINFALL as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).soil.kSlope = x[5 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).soil.suction =
        x[6 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readStorageData(
    mut j: ::core::ffi::c_int,
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut covered: ::core::ffi::c_int = FALSE;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 2 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[(i - 2 as ::core::ffi::c_int) as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        i += 1;
    }
    if ntoks > 6 as ::core::ffi::c_int {
        if match_0(
            *toks.offset(6 as ::core::ffi::c_int as isize),
            w_YES.as_ptr() as *mut ::core::ffi::c_char,
        ) != 0
        {
            covered = TRUE;
        }
    }
    x[1 as ::core::ffi::c_int as usize] =
        x[1 as ::core::ffi::c_int as usize] / (x[1 as ::core::ffi::c_int as usize] + 1.0f64);
    (*LidProcs.offset(j as isize)).storage.thickness =
        x[0 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).storage.voidFrac = x[1 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).storage.kSat =
        x[2 as ::core::ffi::c_int as usize] / UCF(RAINFALL as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).storage.clogFactor = x[3 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).storage.covered = covered;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readDrainData(
    mut j: ::core::ffi::c_int,
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        x[i as usize] = 0.0f64;
        i += 1;
    }
    i = 2 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        if ntoks > i
            && (getDouble(
                *toks.offset(i as isize),
                (&raw mut x as *mut ::core::ffi::c_double)
                    .offset((i - 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
                || x[(i - 2 as ::core::ffi::c_int) as usize] < 0.0f64)
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        i += 1;
    }
    i = -(1 as ::core::ffi::c_int);
    if ntoks >= 9 as ::core::ffi::c_int {
        i = project_findObject(
            CURVE as ::core::ffi::c_int,
            *toks.offset(8 as ::core::ffi::c_int as isize),
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *toks.offset(8 as ::core::ffi::c_int as isize),
            );
        }
    }
    (*LidProcs.offset(j as isize)).drain.coeff = x[0 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).drain.expon = x[1 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).drain.offset =
        x[2 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).drain.delay = x[3 as ::core::ffi::c_int as usize] * 3600.0f64;
    (*LidProcs.offset(j as isize)).drain.hOpen =
        x[4 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).drain.hClose =
        x[5 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).drain.qCurve = i;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readDrainMatData(
    mut j: ::core::ffi::c_int,
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 3] = [0.; 3];
    if ntoks < 5 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if (*LidProcs.offset(j as isize)).lidType != GREEN_ROOF as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    i = 2 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        if getDouble(
            *toks.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[(i - 2 as ::core::ffi::c_int) as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        i += 1;
    }
    (*LidProcs.offset(j as isize)).drainMat.thickness =
        x[0 as ::core::ffi::c_int as usize] / UCF(RAINDEPTH as ::core::ffi::c_int);
    (*LidProcs.offset(j as isize)).drainMat.voidFrac = x[1 as ::core::ffi::c_int as usize];
    (*LidProcs.offset(j as isize)).drainMat.roughness = x[2 as ::core::ffi::c_int as usize];
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readRemovalsData(
    mut j: ::core::ffi::c_int,
    mut toks: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0;
    let mut rmvl: ::core::ffi::c_double = 0.;
    if ntoks < 4 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    while ntoks > i {
        p = project_findObject(POLLUT as ::core::ffi::c_int, *toks.offset(i as isize));
        if p < 0 as ::core::ffi::c_int {
            return error_setInpError(ERR_NAME as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        i += 1;
        if ntoks == i {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if getDouble(*toks.offset(i as isize), &raw mut rmvl) == 0
            || rmvl < 0.0f64
            || rmvl > 100.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *toks.offset(i as isize));
        }
        *(*LidProcs.offset(j as isize)).drainRmvl.offset(p as isize) = rmvl / 100.0f64;
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lid_writeSummary() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut pctArea: ::core::ffi::c_double = 0.;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    fprintf(
        Frpt.file,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  *******************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  LID Control Summary\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  *******************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n                                   No. of        Unit        Unit      %% Area    %% Imperv      %% Perv\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  Subcatchment     LID Control      Units        Area       Width     Covered     Treated     Treated\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  ---------------------------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < GroupCount {
        lidGroup = *LidGroups.offset(j as isize);
        if !lidGroup.is_null() {
            lidList = (*lidGroup).lidList;
            while !lidList.is_null() {
                lidUnit = (*lidList).lidUnit;
                k = (*lidUnit).lidIndex;
                pctArea = (*lidUnit).area * (*lidUnit).number as ::core::ffi::c_double
                    / (*Subcatch.offset(j as isize)).area
                    * 100.0f64;
                fprintf(
                    Frpt.file,
                    b"\n  %-16s %-16s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Subcatch.offset(j as isize)).ID,
                    (*LidProcs.offset(k as isize)).ID,
                );
                fprintf(
                    Frpt.file,
                    b"%6d  %10.2f  %10.2f  %10.2f  %10.2f  %10.2f\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*lidUnit).number,
                    (*lidUnit).area
                        * (UCF(LENGTH as ::core::ffi::c_int) * UCF(LENGTH as ::core::ffi::c_int)),
                    (*lidUnit).fullWidth * UCF(LENGTH as ::core::ffi::c_int),
                    pctArea,
                    (*lidUnit).fromImperv * 100.0f64,
                    (*lidUnit).fromPerv * 100.0f64,
                );
                lidList = (*lidList).nextLidUnit as *mut TLidList;
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lid_validate() {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < LidCount {
        validateLidProc(j);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < GroupCount {
        validateLidGroup(j);
        j += 1;
    }
}
unsafe extern "C" fn validateLidProc(mut j: ::core::ffi::c_int) {
    let mut layerMissing: ::core::ffi::c_int = FALSE;
    if (*LidProcs.offset(j as isize)).lidType < 0 as ::core::ffi::c_int {
        report_writeErrorMsg(
            ERR_LID_TYPE as ::core::ffi::c_int,
            (*LidProcs.offset(j as isize)).ID,
        );
        return;
    }
    match (*LidProcs.offset(j as isize)).lidType {
        0 | 1 => {
            if (*LidProcs.offset(j as isize)).soil.thickness <= 0.0f64 {
                layerMissing = TRUE;
            }
        }
        2 => {
            if (*LidProcs.offset(j as isize)).soil.thickness <= 0.0f64 {
                layerMissing = TRUE;
            }
            if (*LidProcs.offset(j as isize)).drainMat.thickness <= 0.0f64 {
                layerMissing = TRUE;
            }
        }
        4 => {
            if (*LidProcs.offset(j as isize)).pavement.thickness <= 0.0f64 {
                layerMissing = TRUE;
            }
        }
        3 => {
            if (*LidProcs.offset(j as isize)).storage.thickness <= 0.0f64 {
                layerMissing = TRUE;
            }
        }
        _ => {}
    }
    if layerMissing != 0 {
        report_writeErrorMsg(
            ERR_LID_LAYER as ::core::ffi::c_int,
            (*LidProcs.offset(j as isize)).ID,
        );
        return;
    }
    if (*LidProcs.offset(j as isize)).lidType == POROUS_PAVEMENT as ::core::ffi::c_int {
        if (*LidProcs.offset(j as isize)).pavement.thickness <= 0.0f64
            || (*LidProcs.offset(j as isize)).pavement.kSat <= 0.0f64
            || (*LidProcs.offset(j as isize)).pavement.voidFrac <= 0.0f64
            || (*LidProcs.offset(j as isize)).pavement.voidFrac > 1.0f64
            || (*LidProcs.offset(j as isize)).pavement.impervFrac > 1.0f64
        {
            sstrncpy(
                &raw mut Msg as *mut ::core::ffi::c_char,
                (*LidProcs.offset(j as isize)).ID,
                MAXMSG as size_t,
            );
            sstrcat(
                &raw mut Msg as *mut ::core::ffi::c_char,
                ERR_PAVE_LAYER.as_ptr(),
                MAXMSG as size_t,
            );
            report_writeErrorMsg(
                ERR_LID_PARAMS as ::core::ffi::c_int,
                &raw mut Msg as *mut ::core::ffi::c_char,
            );
        }
    }
    if (*LidProcs.offset(j as isize)).soil.thickness > 0.0f64 {
        if (*LidProcs.offset(j as isize)).soil.porosity <= 0.0f64
            || (*LidProcs.offset(j as isize)).soil.fieldCap
                >= (*LidProcs.offset(j as isize)).soil.porosity
            || (*LidProcs.offset(j as isize)).soil.wiltPoint
                >= (*LidProcs.offset(j as isize)).soil.fieldCap
            || (*LidProcs.offset(j as isize)).soil.kSat <= 0.0f64
            || (*LidProcs.offset(j as isize)).soil.kSlope < 0.0f64
        {
            sstrncpy(
                &raw mut Msg as *mut ::core::ffi::c_char,
                (*LidProcs.offset(j as isize)).ID,
                MAXMSG as size_t,
            );
            sstrcat(
                &raw mut Msg as *mut ::core::ffi::c_char,
                ERR_SOIL_LAYER.as_ptr(),
                MAXMSG as size_t,
            );
            report_writeErrorMsg(
                ERR_LID_PARAMS as ::core::ffi::c_int,
                &raw mut Msg as *mut ::core::ffi::c_char,
            );
        }
    }
    if (*LidProcs.offset(j as isize)).storage.thickness > 0.0f64 {
        if (*LidProcs.offset(j as isize)).storage.voidFrac <= 0.0f64
            || (*LidProcs.offset(j as isize)).storage.voidFrac > 1.0f64
        {
            sstrncpy(
                &raw mut Msg as *mut ::core::ffi::c_char,
                (*LidProcs.offset(j as isize)).ID,
                MAXMSG as size_t,
            );
            sstrcat(
                &raw mut Msg as *mut ::core::ffi::c_char,
                ERR_STOR_LAYER.as_ptr(),
                MAXMSG as size_t,
            );
            report_writeErrorMsg(
                ERR_LID_PARAMS as ::core::ffi::c_int,
                &raw mut Msg as *mut ::core::ffi::c_char,
            );
        }
    } else {
        (*LidProcs.offset(j as isize)).storage.voidFrac = 1.0f64;
        (*LidProcs.offset(j as isize)).drain.offset = 0.0f64;
    }
    if (*LidProcs.offset(j as isize)).drain.hOpen > 0.0f64
        && (*LidProcs.offset(j as isize)).drain.hOpen <= (*LidProcs.offset(j as isize)).drain.hClose
    {
        sstrncpy(
            &raw mut Msg as *mut ::core::ffi::c_char,
            (*LidProcs.offset(j as isize)).ID,
            MAXMSG as size_t,
        );
        sstrcat(
            &raw mut Msg as *mut ::core::ffi::c_char,
            ERR_DRAIN_HEADS.as_ptr(),
            MAXMSG as size_t,
        );
        report_writeErrorMsg(
            ERR_LID_PARAMS as ::core::ffi::c_int,
            &raw mut Msg as *mut ::core::ffi::c_char,
        );
    }
    if (*LidProcs.offset(j as isize)).lidType == VEG_SWALE as ::core::ffi::c_int {
        if (*LidProcs.offset(j as isize)).surface.roughness
            * (*LidProcs.offset(j as isize)).surface.surfSlope
            <= 0.0f64
            || (*LidProcs.offset(j as isize)).surface.thickness == 0.0f64
        {
            sstrncpy(
                &raw mut Msg as *mut ::core::ffi::c_char,
                (*LidProcs.offset(j as isize)).ID,
                MAXMSG as size_t,
            );
            sstrcat(
                &raw mut Msg as *mut ::core::ffi::c_char,
                ERR_SWALE_SURF.as_ptr(),
                MAXMSG as size_t,
            );
            report_writeErrorMsg(
                ERR_LID_PARAMS as ::core::ffi::c_int,
                &raw mut Msg as *mut ::core::ffi::c_char,
            );
        } else {
            (*LidProcs.offset(j as isize)).surface.alpha = 1.49f64
                * sqrt((*LidProcs.offset(j as isize)).surface.surfSlope)
                / (*LidProcs.offset(j as isize)).surface.roughness;
        }
    } else if (*LidProcs.offset(j as isize)).surface.roughness > 0.0f64 {
        (*LidProcs.offset(j as isize)).surface.alpha = 1.49f64
            / (*LidProcs.offset(j as isize)).surface.roughness
            * sqrt((*LidProcs.offset(j as isize)).surface.surfSlope);
    } else {
        (*LidProcs.offset(j as isize)).surface.alpha = 0.0f64;
    }
    if (*LidProcs.offset(j as isize)).drainMat.roughness > 0.0f64 {
        (*LidProcs.offset(j as isize)).drainMat.alpha = 1.49f64
            / (*LidProcs.offset(j as isize)).drainMat.roughness
            * sqrt((*LidProcs.offset(j as isize)).surface.surfSlope);
    } else {
        (*LidProcs.offset(j as isize)).drainMat.alpha = 0.0f64;
    }
    if (*LidProcs.offset(j as isize)).pavement.thickness > 0.0f64 {
        (*LidProcs.offset(j as isize)).pavement.clogFactor *=
            (*LidProcs.offset(j as isize)).pavement.thickness
                * (*LidProcs.offset(j as isize)).pavement.voidFrac
                * (1.0f64 - (*LidProcs.offset(j as isize)).pavement.impervFrac);
    }
    if (*LidProcs.offset(j as isize)).storage.thickness > 0.0f64 {
        (*LidProcs.offset(j as isize)).storage.clogFactor *=
            (*LidProcs.offset(j as isize)).storage.thickness
                * (*LidProcs.offset(j as isize)).storage.voidFrac;
    } else {
        (*LidProcs.offset(j as isize)).storage.clogFactor = 0.0f64;
    }
    (*LidProcs.offset(j as isize)).surface.canOverflow = TRUE as ::core::ffi::c_char;
    match (*LidProcs.offset(j as isize)).lidType {
        7 => {
            (*LidProcs.offset(j as isize)).surface.canOverflow = FALSE as ::core::ffi::c_char;
        }
        3 | 4 | 0 | 1 | 2 => {
            if (*LidProcs.offset(j as isize)).surface.alpha > 0.0f64 {
                (*LidProcs.offset(j as isize)).surface.canOverflow = FALSE as ::core::ffi::c_char;
            }
        }
        _ => {}
    }
    if (*LidProcs.offset(j as isize)).lidType == RAIN_BARREL as ::core::ffi::c_int {
        (*LidProcs.offset(j as isize)).storage.voidFrac = 1.0f64;
        (*LidProcs.offset(j as isize)).storage.kSat = 0.0f64;
    }
    if (*LidProcs.offset(j as isize)).lidType == GREEN_ROOF as ::core::ffi::c_int {
        (*LidProcs.offset(j as isize)).storage.thickness =
            (*LidProcs.offset(j as isize)).drainMat.thickness;
        (*LidProcs.offset(j as isize)).storage.voidFrac =
            (*LidProcs.offset(j as isize)).drainMat.voidFrac;
        (*LidProcs.offset(j as isize)).storage.clogFactor = 0.0f64;
        (*LidProcs.offset(j as isize)).storage.kSat = 0.0f64;
    }
}
unsafe extern "C" fn validateLidGroup(mut j: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    let mut p: [::core::ffi::c_double; 3] = [0.; 3];
    let mut totalArea: ::core::ffi::c_double = (*Subcatch.offset(j as isize)).area;
    let mut totalLidArea: ::core::ffi::c_double = 0.0f64;
    let mut fromImperv: ::core::ffi::c_double = 0.0f64;
    let mut fromPerv: ::core::ffi::c_double = 0.0f64;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    lidGroup = *LidGroups.offset(j as isize);
    if lidGroup.is_null() {
        return;
    }
    lidList = (*lidGroup).lidList;
    while !lidList.is_null() {
        lidUnit = (*lidList).lidUnit;
        k = (*lidUnit).lidIndex;
        totalLidArea += (*lidUnit).area * (*lidUnit).number as ::core::ffi::c_double;
        fromImperv += (*lidUnit).fromImperv;
        fromPerv += (*lidUnit).fromPerv;
        (*lidUnit).soilInfil.Ks = 0.0f64;
        if (*LidProcs.offset(k as isize)).soil.thickness > 0.0f64 {
            p[0 as ::core::ffi::c_int as usize] =
                (*LidProcs.offset(k as isize)).soil.suction * UCF(RAINDEPTH as ::core::ffi::c_int);
            p[1 as ::core::ffi::c_int as usize] =
                (*LidProcs.offset(k as isize)).soil.kSat * UCF(RAINFALL as ::core::ffi::c_int);
            p[2 as ::core::ffi::c_int as usize] = ((*LidProcs.offset(k as isize)).soil.porosity
                - (*LidProcs.offset(k as isize)).soil.wiltPoint)
                * (1.0f64 - (*lidUnit).initSat);
            if grnampt_setParams(
                &raw mut (*lidUnit).soilInfil,
                &raw mut p as *mut ::core::ffi::c_double,
            ) == FALSE
            {
                sstrncpy(
                    &raw mut Msg as *mut ::core::ffi::c_char,
                    (*LidProcs.offset(k as isize)).ID,
                    MAXMSG as size_t,
                );
                sstrcat(
                    &raw mut Msg as *mut ::core::ffi::c_char,
                    ERR_SOIL_LAYER.as_ptr(),
                    MAXMSG as size_t,
                );
                report_writeErrorMsg(
                    ERR_LID_PARAMS as ::core::ffi::c_int,
                    &raw mut Msg as *mut ::core::ffi::c_char,
                );
            }
        }
        if (*LidProcs.offset(k as isize)).lidType == VEG_SWALE as ::core::ffi::c_int {
            if (*Subcatch.offset(j as isize)).infilModel == GREEN_AMPT as ::core::ffi::c_int
                || (*Subcatch.offset(j as isize)).infilModel == MOD_GREEN_AMPT as ::core::ffi::c_int
            {
                grnampt_getParams(j, &raw mut p as *mut ::core::ffi::c_double);
                if grnampt_setParams(
                    &raw mut (*lidUnit).soilInfil,
                    &raw mut p as *mut ::core::ffi::c_double,
                ) == FALSE
                {
                    sstrncpy(
                        &raw mut Msg as *mut ::core::ffi::c_char,
                        (*LidProcs.offset(k as isize)).ID,
                        MAXMSG as size_t,
                    );
                    sstrcat(
                        &raw mut Msg as *mut ::core::ffi::c_char,
                        ERR_GREEN_AMPT.as_ptr(),
                        MAXMSG as size_t,
                    );
                    report_writeErrorMsg(
                        ERR_LID_PARAMS as ::core::ffi::c_int,
                        &raw mut Msg as *mut ::core::ffi::c_char,
                    );
                }
            }
            if (*lidUnit).fullWidth <= 0.0f64 {
                sstrncpy(
                    &raw mut Msg as *mut ::core::ffi::c_char,
                    (*LidProcs.offset(k as isize)).ID,
                    MAXMSG as size_t,
                );
                sstrcat(
                    &raw mut Msg as *mut ::core::ffi::c_char,
                    ERR_SWALE_WIDTH.as_ptr(),
                    MAXMSG as size_t,
                );
                report_writeErrorMsg(
                    ERR_LID_PARAMS as ::core::ffi::c_int,
                    &raw mut Msg as *mut ::core::ffi::c_char,
                );
            }
        }
        if (*Subcatch.offset(j as isize)).fracImperv >= 0.999f64 {
            (*lidUnit).toPerv = 0 as ::core::ffi::c_int;
        }
        if (*lidUnit).drainNode == -(1 as ::core::ffi::c_int)
            && (*lidUnit).drainSubcatch == -(1 as ::core::ffi::c_int)
        {
            (*lidUnit).drainNode = (*Subcatch.offset(j as isize)).outNode;
            (*lidUnit).drainSubcatch = (*Subcatch.offset(j as isize)).outSubcatch;
        }
        lidList = (*lidList).nextLidUnit as *mut TLidList;
    }
    if totalLidArea > 1.001f64 * totalArea {
        report_writeErrorMsg(
            ERR_LID_AREAS as ::core::ffi::c_int,
            (*Subcatch.offset(j as isize)).ID,
        );
    }
    if fromImperv > 1.001f64 || fromPerv > 1.001f64 {
        report_writeErrorMsg(
            ERR_LID_CAPTURE_AREA as ::core::ffi::c_int,
            (*Subcatch.offset(j as isize)).ID,
        );
    }
    if totalLidArea > 0.999f64 * totalArea {
        totalLidArea = totalArea;
    }
    (*Subcatch.offset(j as isize)).lidArea = totalLidArea;
}
#[no_mangle]
pub unsafe extern "C" fn lid_initState() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    let mut initVol: ::core::ffi::c_double = 0.;
    let mut initDryTime: ::core::ffi::c_double = StartDryDays * SECperDAY as ::core::ffi::c_double;
    HasWetLids = FALSE as ::core::ffi::c_char;
    j = 0 as ::core::ffi::c_int;
    while j < GroupCount {
        lidGroup = *LidGroups.offset(j as isize);
        if !lidGroup.is_null() {
            (*lidGroup).pervArea = 0.0f64;
            (*lidGroup).flowToPerv = 0.0f64;
            (*lidGroup).oldDrainFlow = 0.0f64;
            (*lidGroup).newDrainFlow = 0.0f64;
            lidList = (*lidGroup).lidList;
            while !lidList.is_null() {
                lidUnit = (*lidList).lidUnit;
                k = (*lidUnit).lidIndex;
                (*lidUnit).surfaceDepth = 0.0f64;
                (*lidUnit).storageDepth = 0.0f64;
                (*lidUnit).soilMoisture = 0.0f64;
                (*lidUnit).paveDepth = 0.0f64;
                (*lidUnit).dryTime = initDryTime;
                (*lidUnit).volTreated = 0.0f64;
                (*lidUnit).nextRegenDay = (*LidProcs.offset(k as isize)).pavement.regenDays;
                initVol = 0.0f64;
                if (*LidProcs.offset(k as isize)).soil.thickness > 0.0f64 {
                    (*lidUnit).soilMoisture = (*LidProcs.offset(k as isize)).soil.wiltPoint
                        + (*lidUnit).initSat
                            * ((*LidProcs.offset(k as isize)).soil.porosity
                                - (*LidProcs.offset(k as isize)).soil.wiltPoint);
                    initVol +=
                        (*lidUnit).soilMoisture * (*LidProcs.offset(k as isize)).soil.thickness;
                }
                if (*LidProcs.offset(k as isize)).storage.thickness > 0.0f64 {
                    (*lidUnit).storageDepth =
                        (*lidUnit).initSat * (*LidProcs.offset(k as isize)).storage.thickness;
                    initVol +=
                        (*lidUnit).storageDepth * (*LidProcs.offset(k as isize)).storage.voidFrac;
                }
                if (*lidUnit).initSat > 0.0f64 {
                    HasWetLids = TRUE as ::core::ffi::c_char;
                }
                lidproc_initWaterBalance(lidUnit, initVol);
                (*lidUnit).volTreated = 0.0f64;
                if !(*lidUnit).rptFile.is_null() {
                    initLidRptFile(
                        &raw mut *(&raw mut Title as *mut [::core::ffi::c_char; 1025])
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                        (*LidProcs.offset(k as isize)).ID,
                        (*Subcatch.offset(j as isize)).ID,
                        lidUnit,
                    );
                }
                (*lidUnit).oldDrainFlow = 0.0f64;
                (*lidUnit).newDrainFlow = 0.0f64;
                i = 0 as ::core::ffi::c_int;
                while i < MAX_LAYERS {
                    (*lidUnit).oldFluxRates[i as usize] = 0.0f64;
                    i += 1;
                }
                if (*lidUnit).soilInfil.Ks > 0.0f64 {
                    grnampt_initState(&raw mut (*lidUnit).soilInfil);
                }
                if isLidPervious((*lidUnit).lidIndex) != 0 {
                    (*lidGroup).pervArea +=
                        (*lidUnit).area * (*lidUnit).number as ::core::ffi::c_double;
                }
                lidList = (*lidList).nextLidUnit as *mut TLidList;
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lid_setOldGroupState(mut j: ::core::ffi::c_int) {
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    if !(*LidGroups.offset(j as isize)).is_null() {
        (**LidGroups.offset(j as isize)).oldDrainFlow =
            (**LidGroups.offset(j as isize)).newDrainFlow;
        (**LidGroups.offset(j as isize)).newDrainFlow = 0.0f64;
        lidList = (**LidGroups.offset(j as isize)).lidList;
        while !lidList.is_null() {
            (*(*lidList).lidUnit).oldDrainFlow = (*(*lidList).lidUnit).newDrainFlow;
            (*(*lidList).lidUnit).newDrainFlow = 0.0f64;
            lidList = (*lidList).nextLidUnit as *mut TLidList;
        }
    }
}
unsafe extern "C" fn isLidPervious(mut k: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return ((*LidProcs.offset(k as isize)).storage.thickness == 0.0f64
        || (*LidProcs.offset(k as isize)).storage.kSat > 0.0f64) as ::core::ffi::c_int;
}
unsafe extern "C" fn getSurfaceDepth(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut depth: ::core::ffi::c_double = 0.0f64;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    lidGroup = *LidGroups.offset(j as isize);
    if lidGroup.is_null() {
        return 0.0f64;
    }
    if (*Subcatch.offset(j as isize)).lidArea == 0.0f64 {
        return 0.0f64;
    }
    lidList = (*lidGroup).lidList;
    while !lidList.is_null() {
        lidUnit = (*lidList).lidUnit;
        k = (*lidUnit).lidIndex;
        depth += (*lidUnit).surfaceDepth
            * (*LidProcs.offset(k as isize)).surface.voidFrac
            * (*lidUnit).area
            * (*lidUnit).number as ::core::ffi::c_double;
        lidList = (*lidList).nextLidUnit as *mut TLidList;
    }
    return depth / (*Subcatch.offset(j as isize)).lidArea;
}
#[no_mangle]
pub unsafe extern "C" fn lid_getPervArea(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    if !(*LidGroups.offset(j as isize)).is_null() {
        return (**LidGroups.offset(j as isize)).pervArea;
    } else {
        return 0.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lid_getFlowToPerv(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    if !(*LidGroups.offset(j as isize)).is_null() {
        return (**LidGroups.offset(j as isize)).flowToPerv;
    }
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn lid_getStoredVolume(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut total: ::core::ffi::c_double = 0.0f64;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    lidGroup = *LidGroups.offset(j as isize);
    if lidGroup.is_null() || (*Subcatch.offset(j as isize)).lidArea == 0.0f64 {
        return 0.0f64;
    }
    lidList = (*lidGroup).lidList;
    while !lidList.is_null() {
        lidUnit = (*lidList).lidUnit;
        total += (*lidUnit).waterBalance.finalVol
            * (*lidUnit).area
            * (*lidUnit).number as ::core::ffi::c_double;
        lidList = (*lidList).nextLidUnit as *mut TLidList;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn lid_getDrainFlow(
    mut j: ::core::ffi::c_int,
    mut timePeriod: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if !(*LidGroups.offset(j as isize)).is_null() {
        if timePeriod == PREVIOUS as ::core::ffi::c_int {
            return (**LidGroups.offset(j as isize)).oldDrainFlow;
        } else {
            return (**LidGroups.offset(j as isize)).newDrainFlow;
        }
    }
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn lid_addDrainLoads(
    mut j: ::core::ffi::c_int,
    mut c: *mut ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut isRunoffLoad: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    lidGroup = *LidGroups.offset(j as isize);
    if !lidGroup.is_null() {
        lidList = (*lidGroup).lidList;
        while !lidList.is_null() {
            lidUnit = (*lidList).lidUnit;
            isRunoffLoad = ((*lidUnit).drainNode >= 0 as ::core::ffi::c_int
                || (*lidUnit).drainSubcatch == j) as ::core::ffi::c_int;
            if (*lidUnit).toPerv == 0 {
                p = 0 as ::core::ffi::c_int;
                while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    w = (*lidUnit).newDrainFlow
                        * *c.offset(p as isize)
                        * tStep
                        * LperFT3
                        * (*Pollut.offset(p as isize)).mcf;
                    r = *(*LidProcs.offset((*lidUnit).lidIndex as isize))
                        .drainRmvl
                        .offset(p as isize);
                    massbal_updateLoadingTotals(BMP_REMOVAL_LOAD as ::core::ffi::c_int, p, r * w);
                    if isRunoffLoad != 0 {
                        massbal_updateLoadingTotals(
                            RUNOFF_LOAD as ::core::ffi::c_int,
                            p,
                            w * (1.0f64 - r),
                        );
                    }
                    p += 1;
                }
            }
            lidList = (*lidList).nextLidUnit as *mut TLidList;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lid_addDrainRunon(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    lidGroup = *LidGroups.offset(j as isize);
    if !lidGroup.is_null() {
        lidList = (*lidGroup).lidList;
        while !lidList.is_null() {
            lidUnit = (*lidList).lidUnit;
            i = (*lidUnit).lidIndex;
            k = (*lidUnit).drainSubcatch;
            if k >= 0 as ::core::ffi::c_int && k != j {
                q = (*lidUnit).oldDrainFlow;
                subcatch_addRunonFlow(k, q);
                p = 0 as ::core::ffi::c_int;
                while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    w = q * *(*Subcatch.offset(j as isize)).oldQual.offset(p as isize) * LperFT3;
                    w = w * (1.0f64 - *(*LidProcs.offset(i as isize)).drainRmvl.offset(p as isize));
                    *(*Subcatch.offset(k as isize)).newQual.offset(p as isize) += w;
                    p += 1;
                }
            }
            lidList = (*lidList).nextLidUnit as *mut TLidList;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lid_addDrainInflow(
    mut j: ::core::ffi::c_int,
    mut f: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut w1: ::core::ffi::c_double = 0.;
    let mut w2: ::core::ffi::c_double = 0.;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    lidGroup = *LidGroups.offset(j as isize);
    if !lidGroup.is_null() {
        lidList = (*lidGroup).lidList;
        while !lidList.is_null() {
            lidUnit = (*lidList).lidUnit;
            i = (*lidUnit).lidIndex;
            k = (*lidUnit).drainNode;
            if k >= 0 as ::core::ffi::c_int {
                q = (1.0f64 - f) * (*lidUnit).oldDrainFlow + f * (*lidUnit).newDrainFlow;
                (*Node.offset(k as isize)).newLatFlow += q;
                massbal_addInflowFlow(WET_WEATHER_INFLOW as ::core::ffi::c_int, q);
                p = 0 as ::core::ffi::c_int;
                while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    w1 = (*lidUnit).oldDrainFlow
                        * *(*Subcatch.offset(j as isize)).oldQual.offset(p as isize);
                    w2 = (*lidUnit).newDrainFlow
                        * *(*Subcatch.offset(j as isize)).newQual.offset(p as isize);
                    w = (1.0f64 - f) * w1 + f * w2;
                    w = w * (1.0f64 - *(*LidProcs.offset(i as isize)).drainRmvl.offset(p as isize));
                    *(*Node.offset(k as isize)).newQual.offset(p as isize) += w;
                    massbal_addInflowQual(WET_WEATHER_INFLOW as ::core::ffi::c_int, p, w);
                    p += 1;
                }
            }
            lidList = (*lidList).nextLidUnit as *mut TLidList;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lid_getRunoff(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
) {
    let mut theLidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidArea: ::core::ffi::c_double = 0.;
    let mut qImperv: ::core::ffi::c_double = 0.0f64;
    let mut qPerv: ::core::ffi::c_double = 0.0f64;
    let mut lidInflow: ::core::ffi::c_double = 0.0f64;
    let mut qRunoff: ::core::ffi::c_double = 0.0f64;
    let mut qDrain: ::core::ffi::c_double = 0.0f64;
    let mut qReturn: ::core::ffi::c_double = 0.0f64;
    theLidGroup = *LidGroups.offset(j as isize);
    if theLidGroup.is_null() {
        return;
    }
    lidList = (*theLidGroup).lidList;
    if lidList.is_null() {
        return;
    }
    EvapRate = Evap.rate;
    if Evap.dryOnly != 0 && (*Subcatch.offset(j as isize)).rainfall > 0.0f64 {
        EvapRate = 0.0f64;
    }
    findNativeInfil(j, tStep);
    if (*Subcatch.offset(j as isize)).area > (*Subcatch.offset(j as isize)).lidArea {
        qImperv = getImpervAreaRunoff(j);
        qPerv = getPervAreaRunoff(j);
    }
    while !lidList.is_null() {
        lidUnit = (*lidList).lidUnit;
        lidArea = (*lidUnit).area * (*lidUnit).number as ::core::ffi::c_double;
        if lidArea > 0.0f64 {
            lidInflow = (qImperv * (*lidUnit).fromImperv + qPerv * (*lidUnit).fromPerv) / lidArea;
            VlidIn += lidInflow * lidArea * tStep;
            lidInflow = lidInflow + getRainInflow(j, lidUnit);
            if (*Subcatch.offset(j as isize)).area == (*Subcatch.offset(j as isize)).lidArea {
                lidInflow += (*Subcatch.offset(j as isize)).runon;
            }
            evalLidUnit(
                j,
                lidUnit,
                lidArea,
                lidInflow,
                tStep,
                &raw mut qRunoff,
                &raw mut qDrain,
                &raw mut qReturn,
            );
        }
        lidList = (*lidList).nextLidUnit as *mut TLidList;
    }
    (*theLidGroup).newDrainFlow = qDrain;
    (*theLidGroup).flowToPerv = qReturn;
    VlidOut = qRunoff * tStep;
    VlidDrain = qDrain * tStep;
    VlidReturn = qReturn * tStep;
}
unsafe extern "C" fn findNativeInfil(mut j: ::core::ffi::c_int, mut tStep: ::core::ffi::c_double) {
    let mut nonLidArea: ::core::ffi::c_double = 0.;
    nonLidArea = (*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea;
    if nonLidArea > 0.0f64 && (*Subcatch.offset(j as isize)).fracImperv < 1.0f64 {
        NativeInfil = Vinfil / nonLidArea / tStep;
    } else {
        NativeInfil = infil_getInfil(
            j,
            tStep,
            (*Subcatch.offset(j as isize)).rainfall,
            (*Subcatch.offset(j as isize)).runon,
            getSurfaceDepth(j),
        );
    }
    if IgnoreGwater == 0 && !(*Subcatch.offset(j as isize)).groundwater.is_null() {
        MaxNativeInfil = (*(*Subcatch.offset(j as isize)).groundwater).maxInfilVol / tStep;
    } else {
        MaxNativeInfil = BIG;
    };
}
unsafe extern "C" fn getRainInflow(
    mut j: ::core::ffi::c_int,
    mut lidUnit: *mut TLidUnit,
) -> ::core::ffi::c_double {
    let mut lidProc: *mut TLidProc = LidProcs.offset((*lidUnit).lidIndex as isize) as *mut TLidProc;
    if (*lidProc).lidType == RAIN_BARREL as ::core::ffi::c_int && (*lidProc).storage.covered == TRUE
    {
        return 0.0f64;
    }
    return (*Subcatch.offset(j as isize)).rainfall;
}
unsafe extern "C" fn getImpervAreaRunoff(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.0f64;
    let mut nonLidArea: ::core::ffi::c_double = 0.;
    i = IMPERV0 as ::core::ffi::c_int;
    while i <= IMPERV1 as ::core::ffi::c_int {
        q += (*Subcatch.offset(j as isize)).subArea[i as usize].runoff
            * (*Subcatch.offset(j as isize)).subArea[i as usize].fArea;
        i += 1;
    }
    if (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].routeTo
        == TO_PERV as ::core::ffi::c_int
        && (*Subcatch.offset(j as isize)).fracImperv < 1.0f64
    {
        q *= (*Subcatch.offset(j as isize)).subArea[IMPERV0 as ::core::ffi::c_int as usize].fOutlet;
    }
    nonLidArea = (*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea;
    return q * nonLidArea;
}
unsafe extern "C" fn getPervAreaRunoff(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut q: ::core::ffi::c_double = 0.0f64;
    let mut nonLidArea: ::core::ffi::c_double = 0.;
    q = (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].runoff
        * (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].fArea;
    if (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].routeTo
        == TO_IMPERV as ::core::ffi::c_int
        && (*Subcatch.offset(j as isize)).fracImperv > 0.0f64
    {
        q *= (*Subcatch.offset(j as isize)).subArea[PERV as ::core::ffi::c_int as usize].fOutlet;
    }
    nonLidArea = (*Subcatch.offset(j as isize)).area - (*Subcatch.offset(j as isize)).lidArea;
    return q * nonLidArea;
}
unsafe extern "C" fn evalLidUnit(
    mut j: ::core::ffi::c_int,
    mut lidUnit: *mut TLidUnit,
    mut lidArea: ::core::ffi::c_double,
    mut lidInflow: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
    mut qRunoff: *mut ::core::ffi::c_double,
    mut qDrain: *mut ::core::ffi::c_double,
    mut qReturn: *mut ::core::ffi::c_double,
) {
    let mut lidProc: *mut TLidProc = ::core::ptr::null_mut::<TLidProc>();
    let mut lidRunoff: ::core::ffi::c_double = 0.;
    let mut lidEvap: ::core::ffi::c_double = 0.;
    let mut lidInfil: ::core::ffi::c_double = 0.;
    let mut lidDrain: ::core::ffi::c_double = 0.;
    lidProc = LidProcs.offset((*lidUnit).lidIndex as isize) as *mut TLidProc;
    lidEvap = 0.0f64;
    lidInfil = 0.0f64;
    lidRunoff = lidproc_getOutflow(
        lidUnit,
        lidProc,
        lidInflow,
        EvapRate,
        NativeInfil,
        MaxNativeInfil,
        tStep,
        &raw mut lidEvap,
        &raw mut lidInfil,
        &raw mut lidDrain,
    ) * lidArea;
    lidDrain *= lidArea;
    if (*lidUnit).toPerv != 0
        && (*Subcatch.offset(j as isize)).area > (*Subcatch.offset(j as isize)).lidArea
    {
        *qReturn += lidRunoff;
        lidRunoff = 0.0f64;
        if (*lidUnit).drainNode == (*Subcatch.offset(j as isize)).outNode
            && (*lidUnit).drainSubcatch == (*Subcatch.offset(j as isize)).outSubcatch
        {
            *qReturn += lidDrain;
            lidDrain = 0.0f64;
        }
    }
    if (*lidUnit).drainNode >= 0 as ::core::ffi::c_int {
        massbal_updateRunoffTotals(RUNOFF_DRAINS as ::core::ffi::c_int, lidDrain * tStep);
    }
    (*lidUnit).newDrainFlow = lidDrain;
    Vevap += lidEvap * tStep * lidArea;
    VlidInfil += lidInfil * tStep * lidArea;
    if isLidPervious((*lidUnit).lidIndex) != 0 {
        Vpevap += lidEvap * tStep * lidArea;
    }
    if (*Subcatch.offset(j as isize)).rainfall > MIN_RUNOFF {
        (*lidUnit).dryTime = 0.0f64;
    } else {
        (*lidUnit).dryTime += tStep;
    }
    lidproc_saveResults(
        lidUnit,
        UCF(RAINFALL as ::core::ffi::c_int),
        UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    *qRunoff += lidRunoff;
    *qDrain += lidDrain;
}
#[no_mangle]
pub unsafe extern "C" fn lid_writeWaterBalance() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ucf: ::core::ffi::c_double = UCF(RAINDEPTH as ::core::ffi::c_int);
    let mut inflow: ::core::ffi::c_double = 0.;
    let mut outflow: ::core::ffi::c_double = 0.;
    let mut err: ::core::ffi::c_double = 0.;
    let mut lidUnit: *mut TLidUnit = ::core::ptr::null_mut::<TLidUnit>();
    let mut lidList: *mut TLidList = ::core::ptr::null_mut::<TLidList>();
    let mut lidGroup: TLidGroup = ::core::ptr::null_mut::<LidGroup>();
    j = 0 as ::core::ffi::c_int;
    while j < GroupCount {
        if !(*LidGroups.offset(j as isize)).is_null() {
            k += 1;
        }
        j += 1;
    }
    if k == 0 as ::core::ffi::c_int {
        return;
    }
    fprintf(
        Frpt.file,
        b"\n\n  ***********************\n  LID Performance Summary\n  ***********************\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  --------------------------------------------------------------------------------------------------------------------\n                                         Total      Evap     Infil   Surface    Drain    Initial     Final  Continuity\n                                        Inflow      Loss      Loss   Outflow   Outflow   Storage   Storage       Error\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Subcatchment      LID Control             in        in        in        in        in        in        in           %%\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Subcatchment      LID Control             mm        mm        mm        mm        mm        mm        mm           %%\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n  --------------------------------------------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < GroupCount {
        lidGroup = *LidGroups.offset(j as isize);
        if !(lidGroup.is_null() || (*Subcatch.offset(j as isize)).lidArea == 0.0f64) {
            lidList = (*lidGroup).lidList;
            while !lidList.is_null() {
                lidUnit = (*lidList).lidUnit;
                k = (*lidUnit).lidIndex;
                fprintf(
                    Frpt.file,
                    b"\n  %-16s  %-16s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Subcatch.offset(j as isize)).ID,
                    (*LidProcs.offset(k as isize)).ID,
                );
                fprintf(
                    Frpt.file,
                    b"%10.2f%10.2f%10.2f%10.2f%10.2f%10.2f%10.2f\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*lidUnit).waterBalance.inflow * ucf,
                    (*lidUnit).waterBalance.evap * ucf,
                    (*lidUnit).waterBalance.infil * ucf,
                    (*lidUnit).waterBalance.surfFlow * ucf,
                    (*lidUnit).waterBalance.drainFlow * ucf,
                    (*lidUnit).waterBalance.initVol * ucf,
                    (*lidUnit).waterBalance.finalVol * ucf,
                );
                inflow = (*lidUnit).waterBalance.initVol + (*lidUnit).waterBalance.inflow;
                outflow = (*lidUnit).waterBalance.finalVol
                    + (*lidUnit).waterBalance.evap
                    + (*lidUnit).waterBalance.infil
                    + (*lidUnit).waterBalance.surfFlow
                    + (*lidUnit).waterBalance.drainFlow;
                if inflow > 0.0f64 {
                    err = (inflow - outflow) / inflow;
                } else {
                    err = 1.0f64;
                }
                fprintf(
                    Frpt.file,
                    b"  %10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                    err * 100.0f64,
                );
                lidList = (*lidList).nextLidUnit as *mut TLidList;
            }
        }
        j += 1;
    }
}
unsafe extern "C" fn initLidRptFile(
    mut title: *mut ::core::ffi::c_char,
    mut lidID: *mut ::core::ffi::c_char,
    mut subcatchID: *mut ::core::ffi::c_char,
    mut lidUnit: *mut TLidUnit,
) {
    static mut colCount: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    static mut head1: [*mut ::core::ffi::c_char; 14] = [
        b"\n                    \t\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"  Elapsed\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    Total\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    Total\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"  Surface\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" Pavement\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"     Soil\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"  Storage\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"  Surface\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    Drain\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"  Surface\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" Pavement\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"     Soil\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"  Storage\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    static mut head2: [*mut ::core::ffi::c_char; 14] = [
        b"\n                    \t\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"     Time\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"   Inflow\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"     Evap\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    Infil\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"     Perc\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"     Perc\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    Exfil\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"   Runoff\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"  OutFlow\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    Level\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    Level\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" Moisture\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    Level\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    static mut units1: [*mut ::core::ffi::c_char; 14] = [
        b"\nDate        Time    \t\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"    Hours\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    in/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    in/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    in/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    in/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    in/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    in/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    in/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    in/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"   inches\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"   inches\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"  Content\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"   inches\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    static mut units2: [*mut ::core::ffi::c_char; 14] = [
        b"\nDate        Time    \t\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"    Hours\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    mm/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    mm/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    mm/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    mm/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    mm/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    mm/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    mm/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"    mm/hr\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"       mm\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"       mm\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"  Content\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"       mm\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    static mut line9: [::core::ffi::c_char; 11] =
        unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b" ---------\0") };
    let mut i: ::core::ffi::c_int = 0;
    let mut f: *mut FILE = (*(*lidUnit).rptFile).file;
    if f.is_null() {
        return;
    }
    fprintf(
        f,
        b"SWMM5 LID Report File\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"\nProject:  %s\0" as *const u8 as *const ::core::ffi::c_char,
        title,
    );
    fprintf(
        f,
        b"\nLID Unit: %s in Subcatchment %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        lidID,
        subcatchID,
    );
    i = 0 as ::core::ffi::c_int;
    while i < colCount {
        fprintf(
            f,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            head1[i as usize],
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < colCount {
        fprintf(
            f,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            head2[i as usize],
        );
        i += 1;
    }
    if UnitSystem == US as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < colCount {
            fprintf(
                f,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                units1[i as usize],
            );
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < colCount {
            fprintf(
                f,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                units2[i as usize],
            );
            i += 1;
        }
    }
    fprintf(
        f,
        b"\n----------- --------\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 1 as ::core::ffi::c_int;
    while i < colCount {
        fprintf(
            f,
            b"\t%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut line9 as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    (*(*lidUnit).rptFile).wasDry = 1 as ::core::ffi::c_int;
    sstrncpy(
        &raw mut (*(*lidUnit).rptFile).results as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        0 as size_t,
    );
}
pub const MAX_LAYERS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAXMSG: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BIG: ::core::ffi::c_double = 1.0E10f64;
pub const MIN_RUNOFF: ::core::ffi::c_double = 2.31481e-8f64;
pub const LperFT3: ::core::ffi::c_double = 28.317f64;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;

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
