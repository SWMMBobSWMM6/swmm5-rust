#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    fn ctime(__timer: *const time_t) -> *mut ::core::ffi::c_char;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn datetime_encodeTime(
        hour: ::core::ffi::c_int,
        minute: ::core::ffi::c_int,
        second: ::core::ffi::c_int,
    ) -> DateTime;
    fn datetime_dateToStr(date: DateTime, s: *mut ::core::ffi::c_char);
    fn datetime_timeToStr(time_0: DateTime, s: *mut ::core::ffi::c_char);
    static mut Frpt: TFile;
    static mut Frunoff: TFile;
    static mut Nperiods: ::core::ffi::c_long;
    static mut NonConvergeCount: ::core::ffi::c_long;
    static mut Msg: [::core::ffi::c_char; 1025];
    static mut ErrorMsg: [::core::ffi::c_char; 1025];
    static mut Title: [[::core::ffi::c_char; 1025]; 3];
    static mut RptFlags: TRptFlags;
    static mut IgnoreRainfall: ::core::ffi::c_int;
    static mut UnitSystem: ::core::ffi::c_int;
    static mut SurchargeMethod: ::core::ffi::c_int;
    static mut InfilModel: ::core::ffi::c_int;
    static mut AllowPonding: ::core::ffi::c_int;
    static mut FlowUnits: ::core::ffi::c_int;
    static mut RouteModel: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut IgnoreRDII: ::core::ffi::c_int;
    static mut IgnoreSnowmelt: ::core::ffi::c_int;
    static mut IgnoreGwater: ::core::ffi::c_int;
    static mut IgnoreRouting: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut Warnings: ::core::ffi::c_int;
    static mut WetStep: ::core::ffi::c_int;
    static mut DryStep: ::core::ffi::c_int;
    static mut ReportStep: ::core::ffi::c_int;
    static mut MaxTrials: ::core::ffi::c_int;
    static mut NumThreads: ::core::ffi::c_int;
    static mut StartDryDays: ::core::ffi::c_double;
    static mut HeadTol: ::core::ffi::c_double;
    static mut CourantFactor: ::core::ffi::c_double;
    static mut RouteStep: ::core::ffi::c_double;
    static mut StartTime: DateTime;
    static mut EndDate: DateTime;
    static mut StartDate: DateTime;
    static mut EndTime: DateTime;
    static mut Gage: *mut TGage;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Pollut: *mut TPollut;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn output_readDateTime(period: ::core::ffi::c_long, aDate: *mut DateTime);
    fn output_readSubcatchResults(period: ::core::ffi::c_long, index: ::core::ffi::c_int);
    fn output_readNodeResults(_: ::core::ffi::c_long, index: ::core::ffi::c_int);
    fn output_readLinkResults(_: ::core::ffi::c_long, index: ::core::ffi::c_int);
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
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
    static mut SubcatchResults: *mut ::core::ffi::c_float;
    static mut NodeResults: *mut ::core::ffi::c_float;
    static mut LinkResults: *mut ::core::ffi::c_float;
    static mut ErrString: [::core::ffi::c_char; 81];
    fn error_getMsg(
        i: ::core::ffi::c_int,
        msg: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut FlowUnitWords: [*mut ::core::ffi::c_char; 0];
    static mut InfilModelWords: [*mut ::core::ffi::c_char; 0];
    static mut LoadUnitsWords: [*mut ::core::ffi::c_char; 0];
    static mut NoYesWords: [*mut ::core::ffi::c_char; 0];
    static mut QualUnitsWords: [*mut ::core::ffi::c_char; 0];
    static mut ReportWords: [*mut ::core::ffi::c_char; 0];
    static mut RouteModelWords: [*mut ::core::ffi::c_char; 0];
    static mut SectWords: [*mut ::core::ffi::c_char; 0];
    static mut SurchargeWords: [*mut ::core::ffi::c_char; 0];
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub type NodeResultType = ::core::ffi::c_uint;
pub const NODE_QUAL: NodeResultType = 6;
pub const NODE_OVERFLOW: NodeResultType = 5;
pub const NODE_INFLOW: NodeResultType = 4;
pub const NODE_LATFLOW: NodeResultType = 3;
pub const NODE_VOLUME: NodeResultType = 2;
pub const NODE_HEAD: NodeResultType = 1;
pub const NODE_DEPTH: NodeResultType = 0;
pub type LinkResultType = ::core::ffi::c_uint;
pub const LINK_QUAL: LinkResultType = 5;
pub const LINK_CAPACITY: LinkResultType = 4;
pub const LINK_VOLUME: LinkResultType = 3;
pub const LINK_VELOCITY: LinkResultType = 2;
pub const LINK_DEPTH: LinkResultType = 1;
pub const LINK_FLOW: LinkResultType = 0;
pub type RouteModelType = ::core::ffi::c_uint;
pub const DW: RouteModelType = 4;
pub const EKW: RouteModelType = 3;
pub const KW: RouteModelType = 2;
pub const SF: RouteModelType = 1;
pub const NO_ROUTING: RouteModelType = 0;
pub type NoYesType = ::core::ffi::c_uint;
pub const YES: NoYesType = 1;
pub const NO: NoYesType = 0;
pub type NoneAllType = ::core::ffi::c_uint;
pub const SOME: NoneAllType = 2;
pub const ALL: NoneAllType = 1;
pub const NONE: NoneAllType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTimeStepStats {
    pub minTimeStep: ::core::ffi::c_double,
    pub maxTimeStep: ::core::ffi::c_double,
    pub routingTime: ::core::ffi::c_double,
    pub timeStepCount: ::core::ffi::c_int,
    pub trialsCount: ::core::ffi::c_double,
    pub steadyStateTime: ::core::ffi::c_double,
    pub timeStepIntervals: [::core::ffi::c_double; 6],
    pub timeStepCounts: [::core::ffi::c_int; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TRainStats {
    pub startDate: DateTime,
    pub endDate: DateTime,
    pub periodsRain: ::core::ffi::c_long,
    pub periodsMissing: ::core::ffi::c_long,
    pub periodsMalfunc: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TMaxStats {
    pub objType: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub value: ::core::ffi::c_double,
}
pub const ERR_NAME: ErrorType = 209;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_FILE_NAME: ErrorType = 301;
pub const ERR_INPUT: ErrorType = 200;
pub const ERR_SYSTEM: ErrorType = 500;
pub const ERR_OUT_FILE: ErrorType = 307;
pub const ERR_TIMESTEP: ErrorType = 107;
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_TIMESERIES_SEQUENCE: ErrorType = 173;
pub const ERR_CURVE_SEQUENCE: ErrorType = 171;
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
pub const ERR_RPT_FILE: ErrorType = 305;
pub const ERR_INP_FILE: ErrorType = 303;
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
pub const ERR_DUP_NAME: ErrorType = 207;
pub const ERR_LINE_LENGTH: ErrorType = 201;
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
pub const ERR_ODE_SOLVER: ErrorType = 105;
pub const ERR_KINWAVE: ErrorType = 103;
pub const ERR_NONE: ErrorType = 0;
pub const MAXTITLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAXMSG: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MGDperCFS: ::core::ffi::c_double = 0.64632f64;
pub const MLDperCFS: ::core::ffi::c_double = 2.4466f64;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;
pub const FMT08: [::core::ffi::c_char; 64] = unsafe {
    ::core::mem::transmute::<[u8; 64], [::core::ffi::c_char; 64]>(
        *b"\n  EPA STORM WATER MANAGEMENT MODEL - VERSION 5.2 (Build 5.2.4)\0",
    )
};
pub const FMT09: [::core::ffi::c_char; 64] = unsafe {
    ::core::mem::transmute::<[u8; 64], [::core::ffi::c_char; 64]>(
        *b"\n  ------------------------------------------------------------\0",
    )
};
pub const FMT10: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\n\0") };
pub const FMT17: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"at line %ld of input file:\0")
};
pub const FMT18: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"at line %ld of %s] section:\0")
};
pub const FMT20: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(
        *b"\n\n  Analysis begun on:  %s\0",
    )
};
pub const FMT20a: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"  Analysis ended on:  %s\0")
};
pub const FMT21: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"  Total elapsed time: \0")
};
pub const w_NONE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"NONE\0") };
pub const w_ALL: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"ALL\0") };
pub const NO_DATE: ::core::ffi::c_int = -(693594 as ::core::ffi::c_int);
pub const TIMELEVELS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LINE_10: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"----------\0") };
pub const LINE_12: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"------------\0") };
static mut SysTime: time_t = 0;
#[no_mangle]
pub unsafe extern "C" fn report_readOptions(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_char = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_int = 0;
    if ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    k = findmatch(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        &raw mut ReportWords as *mut *mut ::core::ffi::c_char,
    ) as ::core::ffi::c_char;
    if (k as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if (k as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
        || k as ::core::ffi::c_int > 4 as ::core::ffi::c_int
    {
        m = findmatch(
            *tok.offset(1 as ::core::ffi::c_int as isize),
            &raw mut NoYesWords as *mut *mut ::core::ffi::c_char,
        );
        if m < NO as ::core::ffi::c_int || m > YES as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        match k as ::core::ffi::c_int {
            0 => {
                RptFlags.disabled = m as ::core::ffi::c_char;
                return 0 as ::core::ffi::c_int;
            }
            1 => {
                RptFlags.input = m as ::core::ffi::c_char;
                return 0 as ::core::ffi::c_int;
            }
            5 => {
                RptFlags.continuity = m as ::core::ffi::c_char;
                return 0 as ::core::ffi::c_int;
            }
            6 => {
                RptFlags.flowStats = m as ::core::ffi::c_char;
                return 0 as ::core::ffi::c_int;
            }
            7 => {
                RptFlags.controls = m as ::core::ffi::c_char;
                return 0 as ::core::ffi::c_int;
            }
            8 => {
                RptFlags.averages = m as ::core::ffi::c_char;
                return 0 as ::core::ffi::c_int;
            }
            9 => return 0 as ::core::ffi::c_int,
            _ => {
                return error_setInpError(
                    ERR_KEYWORD as ::core::ffi::c_int,
                    *tok.offset(1 as ::core::ffi::c_int as isize),
                );
            }
        }
    } else {
        match k as ::core::ffi::c_int {
            2 => {
                m = SUBCATCH as ::core::ffi::c_int;
            }
            3 => {
                m = NODE as ::core::ffi::c_int;
            }
            4 => {
                m = LINK as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    if strcomp(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        w_NONE.as_ptr(),
    ) != 0
    {
        k = NONE as ::core::ffi::c_int as ::core::ffi::c_char;
    } else if strcomp(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        w_ALL.as_ptr(),
    ) != 0
    {
        k = ALL as ::core::ffi::c_int as ::core::ffi::c_char;
    } else {
        k = SOME as ::core::ffi::c_int as ::core::ffi::c_char;
        t = 1 as ::core::ffi::c_int;
        while t < ntoks {
            j = project_findObject(m, *tok.offset(t as isize));
            if j < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_NAME as ::core::ffi::c_int, *tok.offset(t as isize));
            }
            match m {
                1 => {
                    (*Subcatch.offset(j as isize)).rptFlag = TRUE;
                }
                2 => {
                    (*Node.offset(j as isize)).rptFlag = TRUE;
                }
                3 => {
                    (*Link.offset(j as isize)).rptFlag = TRUE;
                }
                _ => {}
            }
            t += 1;
        }
    }
    match m {
        1 => {
            RptFlags.subcatchments = k;
        }
        2 => {
            RptFlags.nodes = k;
        }
        3 => {
            RptFlags.links = k;
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn report_writeLine(mut line: *const ::core::ffi::c_char) {
    if !Frpt.file.is_null() {
        fprintf(
            Frpt.file,
            b"\n  %s\0" as *const u8 as *const ::core::ffi::c_char,
            line,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_writeSysTime() {
    let mut theTime: [::core::ffi::c_char; 9] = [0; 9];
    let mut elapsedTime: ::core::ffi::c_double = 0.;
    let mut endTime: time_t = 0;
    if !Frpt.file.is_null() {
        fprintf(Frpt.file, FMT20.as_ptr(), ctime(&raw mut SysTime));
        time(&raw mut endTime);
        fprintf(Frpt.file, FMT20a.as_ptr(), ctime(&raw mut endTime));
        elapsedTime = difftime(endTime, SysTime);
        fprintf(Frpt.file, FMT21.as_ptr());
        if elapsedTime < 1.0f64 {
            fprintf(
                Frpt.file,
                b"< 1 sec\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            elapsedTime /= SECperDAY as ::core::ffi::c_double;
            if elapsedTime >= 1.0f64 {
                fprintf(
                    Frpt.file,
                    b"%d.\0" as *const u8 as *const ::core::ffi::c_char,
                    crate::ccast::f64_to_i32(floor(elapsedTime)),
                );
                elapsedTime -= floor(elapsedTime);
            }
            datetime_timeToStr(
                elapsedTime as DateTime,
                &raw mut theTime as *mut ::core::ffi::c_char,
            );
            fprintf(
                Frpt.file,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut theTime as *mut ::core::ffi::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_writeLogo() {
    fprintf(Frpt.file, FMT08.as_ptr());
    fprintf(Frpt.file, FMT09.as_ptr());
    fprintf(Frpt.file, FMT10.as_ptr());
    time(&raw mut SysTime);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeTitle() {
    let mut i: ::core::ffi::c_int = 0;
    let mut lineCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ErrorCode != 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAXTITLE {
        if strlen(
            &raw mut *(&raw mut Title as *mut [::core::ffi::c_char; 1025]).offset(i as isize)
                as *mut ::core::ffi::c_char,
        ) > 0 as size_t
        {
            report_writeLine(
                &raw mut *(&raw mut Title as *mut [::core::ffi::c_char; 1025]).offset(i as isize)
                    as *mut ::core::ffi::c_char,
            );
            lineCount += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_writeOptions() {
    let mut str: [::core::ffi::c_char; 80] = [0; 80];
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"****************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Analysis Options\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"****************\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  Flow Units ............... %s\0" as *const u8 as *const ::core::ffi::c_char,
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
    );
    fprintf(
        Frpt.file,
        b"\n  Process Models:\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n    Rainfall/Runoff ........ \0" as *const u8 as *const ::core::ffi::c_char,
    );
    if IgnoreRainfall != 0
        || Nobjects[GAGE as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
    {
        fprintf(
            Frpt.file,
            b"NO\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"YES\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n    RDII ................... \0" as *const u8 as *const ::core::ffi::c_char,
    );
    if IgnoreRDII != 0
        || Nobjects[UNITHYD as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
    {
        fprintf(
            Frpt.file,
            b"NO\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"YES\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n    Snowmelt ............... \0" as *const u8 as *const ::core::ffi::c_char,
    );
    if IgnoreSnowmelt != 0
        || Nobjects[SNOWMELT as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
    {
        fprintf(
            Frpt.file,
            b"NO\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"YES\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n    Groundwater ............ \0" as *const u8 as *const ::core::ffi::c_char,
    );
    if IgnoreGwater != 0
        || Nobjects[AQUIFER as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
    {
        fprintf(
            Frpt.file,
            b"NO\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"YES\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n    Flow Routing ........... \0" as *const u8 as *const ::core::ffi::c_char,
    );
    if IgnoreRouting != 0
        || Nobjects[LINK as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
    {
        fprintf(
            Frpt.file,
            b"NO\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"YES\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n    Ponding Allowed ........ \0" as *const u8 as *const ::core::ffi::c_char,
        );
        if AllowPonding != 0 {
            fprintf(
                Frpt.file,
                b"YES\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            fprintf(
                Frpt.file,
                b"NO\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    fprintf(
        Frpt.file,
        b"\n    Water Quality .......... \0" as *const u8 as *const ::core::ffi::c_char,
    );
    if IgnoreQuality != 0
        || Nobjects[POLLUT as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
    {
        fprintf(
            Frpt.file,
            b"NO\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"YES\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Infiltration Method ...... %s\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut InfilModelWords as *mut *mut ::core::ffi::c_char)
                .offset(InfilModel as isize),
        );
    }
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Flow Routing Method ...... %s\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut RouteModelWords as *mut *mut ::core::ffi::c_char)
                .offset(RouteModel as isize),
        );
    }
    if RouteModel == DW as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Surcharge Method ......... %s\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut SurchargeWords as *mut *mut ::core::ffi::c_char)
                .offset(SurchargeMethod as isize),
        );
    }
    datetime_dateToStr(StartDate, &raw mut str as *mut ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  Starting Date ............ %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut str as *mut ::core::ffi::c_char,
    );
    datetime_timeToStr(StartTime, &raw mut str as *mut ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b" %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut str as *mut ::core::ffi::c_char,
    );
    datetime_dateToStr(EndDate, &raw mut str as *mut ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  Ending Date .............. %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut str as *mut ::core::ffi::c_char,
    );
    datetime_timeToStr(EndTime, &raw mut str as *mut ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b" %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut str as *mut ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  Antecedent Dry Days ...... %.1f\0" as *const u8 as *const ::core::ffi::c_char,
        StartDryDays,
    );
    datetime_timeToStr(
        datetime_encodeTime(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int, ReportStep),
        &raw mut str as *mut ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  Report Time Step ......... %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut str as *mut ::core::ffi::c_char,
    );
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        datetime_timeToStr(
            datetime_encodeTime(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int, WetStep),
            &raw mut str as *mut ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  Wet Time Step ............ %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut str as *mut ::core::ffi::c_char,
        );
        datetime_timeToStr(
            datetime_encodeTime(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int, DryStep),
            &raw mut str as *mut ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  Dry Time Step ............ %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut str as *mut ::core::ffi::c_char,
        );
    }
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Routing Time Step ........ %.2f sec\0" as *const u8 as *const ::core::ffi::c_char,
            RouteStep,
        );
        if RouteModel == DW as ::core::ffi::c_int {
            fprintf(
                Frpt.file,
                b"\n  Variable Time Step ....... \0" as *const u8 as *const ::core::ffi::c_char,
            );
            if CourantFactor > 0.0f64 {
                fprintf(
                    Frpt.file,
                    b"YES\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                fprintf(
                    Frpt.file,
                    b"NO\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            fprintf(
                Frpt.file,
                b"\n  Maximum Trials ........... %d\0" as *const u8 as *const ::core::ffi::c_char,
                MaxTrials,
            );
            fprintf(
                Frpt.file,
                b"\n  Number of Threads ........ %d\0" as *const u8 as *const ::core::ffi::c_char,
                NumThreads,
            );
            fprintf(
                Frpt.file,
                b"\n  Head Tolerance ........... %.6f \0" as *const u8
                    as *const ::core::ffi::c_char,
                HeadTol * UCF(LENGTH as ::core::ffi::c_int),
            );
            if UnitSystem == US as ::core::ffi::c_int {
                fprintf(
                    Frpt.file,
                    b"ft\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                fprintf(Frpt.file, b"m\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeRainStats(mut i: ::core::ffi::c_int, mut r: *mut TRainStats) {
    let mut date1: [::core::ffi::c_char; 12] =
        ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"***********\0");
    let mut date2: [::core::ffi::c_char; 12] =
        ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"***********\0");
    if RptFlags.disabled != 0 {
        return;
    }
    if i < 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*********************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Rainfall File Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*********************\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n  Station    First        Last         Recording   Periods    Periods    Periods\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  ID         Date         Date         Frequency  w/Precip    Missing    Malfunc.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  -------------------------------------------------------------------------------\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        if (*r).startDate != NO_DATE as DateTime {
            datetime_dateToStr((*r).startDate, &raw mut date1 as *mut ::core::ffi::c_char);
        }
        if (*r).endDate != NO_DATE as DateTime {
            datetime_dateToStr((*r).endDate, &raw mut date2 as *mut ::core::ffi::c_char);
        }
        fprintf(
            Frpt.file,
            b"%-10s %10s   %-10s   %5d min    %6ld     %6ld     %6ld\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut (*Gage.offset(i as isize)).staID as *mut ::core::ffi::c_char,
            &raw mut date1 as *mut ::core::ffi::c_char,
            &raw mut date2 as *mut ::core::ffi::c_char,
            (*Gage.offset(i as isize)).rainInterval / 60 as ::core::ffi::c_int,
            (*r).periodsRain,
            (*r).periodsMissing,
            (*r).periodsMalfunc,
        );
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeRdiiStats(
    mut rainVol: ::core::ffi::c_double,
    mut rdiiVol: ::core::ffi::c_double,
) {
    let mut ratio: ::core::ffi::c_double = 0.;
    let mut ucf1: ::core::ffi::c_double = 0.;
    let mut ucf2: ::core::ffi::c_double = 0.;
    if RptFlags.disabled != 0 {
        return;
    }
    ucf1 = UCF(LENGTH as ::core::ffi::c_int) * UCF(LANDAREA as ::core::ffi::c_int);
    if UnitSystem == US as ::core::ffi::c_int {
        ucf2 = MGDperCFS / SECperDAY as ::core::ffi::c_double;
    } else {
        ucf2 = MLDperCFS / SECperDAY as ::core::ffi::c_double;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  **********************           Volume        Volume\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Rainfall Dependent I/I        acre-feet      10^6 gal\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Rainfall Dependent I/I        hectare-m      10^6 ltr\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n  **********************        ---------     ---------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  Sewershed Rainfall ......%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        rainVol * ucf1,
        rainVol * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  RDII Produced ...........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        rdiiVol * ucf1,
        rdiiVol * ucf2,
    );
    if rainVol == 0.0f64 {
        ratio = 0.0f64;
    } else {
        ratio = rdiiVol / rainVol;
    }
    fprintf(
        Frpt.file,
        b"\n  RDII Ratio ..............%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        ratio,
    );
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeControlActionsHeading() {
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*********************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Control Actions Taken\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*********************\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn report_writeControlAction(
    mut aDate: DateTime,
    mut linkID: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_double,
    mut ruleID: *mut ::core::ffi::c_char,
) {
    let mut theDate: [::core::ffi::c_char; 12] = [0; 12];
    let mut theTime: [::core::ffi::c_char; 9] = [0; 9];
    if RptFlags.disabled != 0 {
        return;
    }
    datetime_dateToStr(aDate, &raw mut theDate as *mut ::core::ffi::c_char);
    datetime_timeToStr(aDate, &raw mut theTime as *mut ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"  %11s: %8s Link %s setting changed to %6.2f by Control %s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut theDate as *mut ::core::ffi::c_char,
        &raw mut theTime as *mut ::core::ffi::c_char,
        linkID,
        value,
        ruleID,
    );
}
#[no_mangle]
pub unsafe extern "C" fn report_writeRunoffError(
    mut totals: *mut TRunoffTotals,
    mut totalArea: ::core::ffi::c_double,
) {
    if Frunoff.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n  **************************\n  Runoff Quantity Continuity\n  **************************\n  Runoff supplied by interface file %s\0"
                as *const u8 as *const ::core::ffi::c_char,
            &raw mut Frunoff.name as *mut ::core::ffi::c_char,
        );
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    if totalArea == 0.0f64 {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  **************************        Volume         Depth\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Runoff Quantity Continuity     acre-feet        inches\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Runoff Quantity Continuity     hectare-m            mm\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n  **************************     ---------       -------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if (*totals).initStorage > 0.0f64 {
        fprintf(
            Frpt.file,
            b"\n  Initial LID Storage ......%14.3f%14.3f\0" as *const u8
                as *const ::core::ffi::c_char,
            (*totals).initStorage
                * UCF(LENGTH as ::core::ffi::c_int)
                * UCF(LANDAREA as ::core::ffi::c_int),
            (*totals).initStorage / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
        );
    }
    if Nobjects[SNOWMELT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Initial Snow Cover .......%14.3f%14.3f\0" as *const u8
                as *const ::core::ffi::c_char,
            (*totals).initSnowCover
                * UCF(LENGTH as ::core::ffi::c_int)
                * UCF(LANDAREA as ::core::ffi::c_int),
            (*totals).initSnowCover / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
        );
    }
    fprintf(
        Frpt.file,
        b"\n  Total Precipitation ......%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).rainfall
            * UCF(LENGTH as ::core::ffi::c_int)
            * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).rainfall / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    if (*totals).runon > 0.0f64 {
        fprintf(
            Frpt.file,
            b"\n  Outfall Runon ............%14.3f%14.3f\0" as *const u8
                as *const ::core::ffi::c_char,
            (*totals).runon
                * UCF(LENGTH as ::core::ffi::c_int)
                * UCF(LANDAREA as ::core::ffi::c_int),
            (*totals).runon / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
        );
    }
    fprintf(
        Frpt.file,
        b"\n  Evaporation Loss .........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).evap * UCF(LENGTH as ::core::ffi::c_int) * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).evap / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Infiltration Loss ........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).infil * UCF(LENGTH as ::core::ffi::c_int) * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).infil / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Surface Runoff ...........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).runoff * UCF(LENGTH as ::core::ffi::c_int) * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).runoff / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    if (*totals).drains > 0.0f64 {
        fprintf(
            Frpt.file,
            b"\n  LID Drainage .............%14.3f%14.3f\0" as *const u8
                as *const ::core::ffi::c_char,
            (*totals).drains
                * UCF(LENGTH as ::core::ffi::c_int)
                * UCF(LANDAREA as ::core::ffi::c_int),
            (*totals).drains / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
        );
    }
    if Nobjects[SNOWMELT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Snow Removed .............%14.3f%14.3f\0" as *const u8
                as *const ::core::ffi::c_char,
            (*totals).snowRemoved
                * UCF(LENGTH as ::core::ffi::c_int)
                * UCF(LANDAREA as ::core::ffi::c_int),
            (*totals).snowRemoved / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
        );
        fprintf(
            Frpt.file,
            b"\n  Final Snow Cover .........%14.3f%14.3f\0" as *const u8
                as *const ::core::ffi::c_char,
            (*totals).finalSnowCover
                * UCF(LENGTH as ::core::ffi::c_int)
                * UCF(LANDAREA as ::core::ffi::c_int),
            (*totals).finalSnowCover / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
        );
    }
    fprintf(
        Frpt.file,
        b"\n  Final Storage ............%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).finalStorage
            * UCF(LENGTH as ::core::ffi::c_int)
            * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).finalStorage / totalArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Continuity Error (%%) .....%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).pctError,
    );
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeLoadingError(mut totals: *mut TLoadingTotals) {
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    p1 = 1 as ::core::ffi::c_int;
    p2 = if 5 as ::core::ffi::c_int <= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        5 as ::core::ffi::c_int
    } else {
        Nobjects[POLLUT as ::core::ffi::c_int as usize]
    };
    while p1 <= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        report_LoadingErrors(
            p1 - 1 as ::core::ffi::c_int,
            p2 - 1 as ::core::ffi::c_int,
            totals,
        );
        p1 = p2 + 1 as ::core::ffi::c_int;
        p2 = p1 + 4 as ::core::ffi::c_int;
        p2 = if p2 <= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            p2
        } else {
            Nobjects[POLLUT as ::core::ffi::c_int as usize]
        };
    }
}
unsafe extern "C" fn report_LoadingErrors(
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut totals: *mut TLoadingTotals,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut cf: ::core::ffi::c_double = 1.0f64;
    let mut units: [::core::ffi::c_char; 15] = [0; 15];
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  **************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Pollut.offset(p as isize)).ID,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Runoff Quality Continuity \0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        i = UnitSystem;
        if (*Pollut.offset(p as isize)).units == COUNT as ::core::ffi::c_int {
            i = 2 as ::core::ffi::c_int;
        }
        sstrncpy(
            &raw mut units as *mut ::core::ffi::c_char,
            *(&raw mut LoadUnitsWords as *mut *mut ::core::ffi::c_char).offset(i as isize),
            14 as size_t,
        );
        fprintf(
            Frpt.file,
            b"%14s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut units as *mut ::core::ffi::c_char,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  **************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"    ----------\0" as *const u8 as *const ::core::ffi::c_char,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Initial Buildup ..........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).initLoad * cf,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Surface Buildup ..........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).buildup * cf,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Wet Deposition ...........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).deposition * cf,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Sweeping Removal .........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).sweeping * cf,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Infiltration Loss ........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).infil * cf,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  BMP Removal ..............\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).bmpRemoval * cf,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Surface Runoff ...........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).runoff * cf,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Remaining Buildup ........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).finalLoad * cf,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Continuity Error (%%) .....\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*totals.offset(p as isize)).pctError,
        );
        p += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeGwaterError(
    mut totals: *mut TGwaterTotals,
    mut gwArea: ::core::ffi::c_double,
) {
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  **************************        Volume         Depth\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Groundwater Continuity         acre-feet        inches\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Groundwater Continuity         hectare-m            mm\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n  **************************     ---------       -------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  Initial Storage ..........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).initStorage
            * UCF(LENGTH as ::core::ffi::c_int)
            * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).initStorage / gwArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Infiltration .............%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).infil * UCF(LENGTH as ::core::ffi::c_int) * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).infil / gwArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Upper Zone ET ............%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).upperEvap
            * UCF(LENGTH as ::core::ffi::c_int)
            * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).upperEvap / gwArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Lower Zone ET ............%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).lowerEvap
            * UCF(LENGTH as ::core::ffi::c_int)
            * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).lowerEvap / gwArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Deep Percolation .........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).lowerPerc
            * UCF(LENGTH as ::core::ffi::c_int)
            * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).lowerPerc / gwArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Groundwater Flow .........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).gwater * UCF(LENGTH as ::core::ffi::c_int) * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).gwater / gwArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Final Storage ............%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).finalStorage
            * UCF(LENGTH as ::core::ffi::c_int)
            * UCF(LANDAREA as ::core::ffi::c_int),
        (*totals).finalStorage / gwArea * UCF(RAINDEPTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b"\n  Continuity Error (%%) .....%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).pctError,
    );
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeFlowError(mut totals: *mut TRoutingTotals) {
    let mut ucf1: ::core::ffi::c_double = 0.;
    let mut ucf2: ::core::ffi::c_double = 0.;
    ucf1 = UCF(LENGTH as ::core::ffi::c_int) * UCF(LANDAREA as ::core::ffi::c_int);
    if UnitSystem == US as ::core::ffi::c_int {
        ucf2 = MGDperCFS / SECperDAY as ::core::ffi::c_double;
    } else {
        ucf2 = MLDperCFS / SECperDAY as ::core::ffi::c_double;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  **************************        Volume        Volume\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Flow Routing Continuity        acre-feet      10^6 gal\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Flow Routing Continuity        hectare-m      10^6 ltr\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n  **************************     ---------     ---------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        Frpt.file,
        b"\n  Dry Weather Inflow .......%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).dwInflow * ucf1,
        (*totals).dwInflow * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  Wet Weather Inflow .......%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).wwInflow * ucf1,
        (*totals).wwInflow * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  Groundwater Inflow .......%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).gwInflow * ucf1,
        (*totals).gwInflow * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  RDII Inflow ..............%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).iiInflow * ucf1,
        (*totals).iiInflow * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  External Inflow ..........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).exInflow * ucf1,
        (*totals).exInflow * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  External Outflow .........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).outflow * ucf1,
        (*totals).outflow * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  Flooding Loss ............%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).flooding * ucf1,
        (*totals).flooding * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  Evaporation Loss .........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).evapLoss * ucf1,
        (*totals).evapLoss * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  Exfiltration Loss ........%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).seepLoss * ucf1,
        (*totals).seepLoss * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  Initial Stored Volume ....%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).initStorage * ucf1,
        (*totals).initStorage * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  Final Stored Volume ......%14.3f%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).finalStorage * ucf1,
        (*totals).finalStorage * ucf2,
    );
    fprintf(
        Frpt.file,
        b"\n  Continuity Error (%%) .....%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
        (*totals).pctError,
    );
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeQualError(mut QualTotals: *mut TRoutingTotals) {
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    p1 = 1 as ::core::ffi::c_int;
    p2 = if 5 as ::core::ffi::c_int <= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        5 as ::core::ffi::c_int
    } else {
        Nobjects[POLLUT as ::core::ffi::c_int as usize]
    };
    while p1 <= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        report_QualErrors(
            p1 - 1 as ::core::ffi::c_int,
            p2 - 1 as ::core::ffi::c_int,
            QualTotals,
        );
        p1 = p2 + 1 as ::core::ffi::c_int;
        p2 = p1 + 4 as ::core::ffi::c_int;
        p2 = if p2 <= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            p2
        } else {
            Nobjects[POLLUT as ::core::ffi::c_int as usize]
        };
    }
}
unsafe extern "C" fn report_QualErrors(
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut QualTotals: *mut TRoutingTotals,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut units: [::core::ffi::c_char; 15] = [0; 15];
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  **************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Pollut.offset(p as isize)).ID,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Quality Routing Continuity\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        i = UnitSystem;
        if (*Pollut.offset(p as isize)).units == COUNT as ::core::ffi::c_int {
            i = 2 as ::core::ffi::c_int;
        }
        sstrncpy(
            &raw mut units as *mut ::core::ffi::c_char,
            *(&raw mut LoadUnitsWords as *mut *mut ::core::ffi::c_char).offset(i as isize),
            14 as size_t,
        );
        fprintf(
            Frpt.file,
            b"%14s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut units as *mut ::core::ffi::c_char,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  **************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"    ----------\0" as *const u8 as *const ::core::ffi::c_char,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Dry Weather Inflow .......\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).dwInflow,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Wet Weather Inflow .......\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).wwInflow,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Groundwater Inflow .......\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).gwInflow,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  RDII Inflow ..............\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).iiInflow,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  External Inflow ..........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).exInflow,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  External Outflow .........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).outflow,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Flooding Loss ............\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).flooding,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Exfiltration Loss ........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).seepLoss,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Mass Reacted .............\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).reacted,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Initial Stored Mass ......\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).initStorage,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Final Stored Mass ........\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).finalStorage,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Continuity Error (%%) .....\0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = p1;
    while p <= p2 {
        fprintf(
            Frpt.file,
            b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
            (*QualTotals.offset(p as isize)).pctError,
        );
        p += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeMaxStats(
    mut maxMassBalErrs: *mut TMaxStats,
    mut maxCourantCrit: *mut TMaxStats,
    mut nMaxStats: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    if RouteModel != DW as ::core::ffi::c_int
        || Nobjects[LINK as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
    {
        return;
    }
    if nMaxStats <= 0 as ::core::ffi::c_int {
        return;
    }
    if (*maxMassBalErrs.offset(0 as ::core::ffi::c_int as isize)).index >= 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*************************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Highest Continuity Errors\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*************************\0" as *const u8 as *const ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < nMaxStats {
            j = (*maxMassBalErrs.offset(i as isize)).index;
            if !(j < 0 as ::core::ffi::c_int) {
                fprintf(
                    Frpt.file,
                    b"\n  Node %s (%.2f%%)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Node.offset(j as isize)).ID,
                    (*maxMassBalErrs.offset(i as isize)).value,
                );
            }
            i += 1;
        }
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if CourantFactor == 0.0f64 {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Time-Step Critical Elements\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************************\0" as *const u8 as *const ::core::ffi::c_char);
    k = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nMaxStats {
        j = (*maxCourantCrit.offset(i as isize)).index;
        if !(j < 0 as ::core::ffi::c_int) {
            k += 1;
            if (*maxCourantCrit.offset(i as isize)).objType == NODE as ::core::ffi::c_int {
                fprintf(
                    Frpt.file,
                    b"\n  Node %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Node.offset(j as isize)).ID,
                );
            } else {
                fprintf(
                    Frpt.file,
                    b"\n  Link %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Link.offset(j as isize)).ID,
                );
            }
            fprintf(
                Frpt.file,
                b" (%.2f%%)\0" as *const u8 as *const ::core::ffi::c_char,
                (*maxCourantCrit.offset(i as isize)).value,
            );
        }
        i += 1;
    }
    if k == 0 as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  None\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeMaxFlowTurns(
    mut flowTurns: *mut TMaxStats,
    mut nMaxStats: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if Nobjects[LINK as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(
        b"********************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    report_writeLine(
        b"Highest Flow Instability Indexes\0" as *const u8 as *const ::core::ffi::c_char,
    );
    report_writeLine(
        b"********************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if nMaxStats <= 0 as ::core::ffi::c_int
        || (*flowTurns.offset(0 as ::core::ffi::c_int as isize)).index <= 0 as ::core::ffi::c_int
    {
        fprintf(
            Frpt.file,
            b"\n  All links are stable.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < nMaxStats {
            j = (*flowTurns.offset(i as isize)).index;
            if !(j < 0 as ::core::ffi::c_int) {
                fprintf(
                    Frpt.file,
                    b"\n  Link %s (%.0f)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Link.offset(j as isize)).ID,
                    (*flowTurns.offset(i as isize)).value,
                );
            }
            i += 1;
        }
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeNonconvergedStats(
    mut maxNonconverged: *mut TMaxStats,
    mut nMaxStats: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if Nobjects[NODE as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
        || RouteModel != DW as ::core::ffi::c_int
    {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(
        b"*********************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    report_writeLine(
        b"Most Frequent Nonconverging Nodes\0" as *const u8 as *const ::core::ffi::c_char,
    );
    report_writeLine(
        b"*********************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if nMaxStats <= 0 as ::core::ffi::c_int
        || (*maxNonconverged.offset(0 as ::core::ffi::c_int as isize)).index
            <= 0 as ::core::ffi::c_int
        || (*maxNonconverged.offset(0 as ::core::ffi::c_int as isize)).value < 0.00005f64
    {
        fprintf(
            Frpt.file,
            b"\n  Convergence obtained at all time steps.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < nMaxStats {
            j = (*maxNonconverged.offset(i as isize)).index;
            if !(j < 0 as ::core::ffi::c_int
                || (*maxNonconverged.offset(i as isize)).value <= 0.0f64)
            {
                fprintf(
                    Frpt.file,
                    b"\n  Node %s (%.2f%%)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Node.offset(j as isize)).ID,
                    100.0f64 * (*maxNonconverged.offset(i as isize)).value,
                );
            }
            i += 1;
        }
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn report_writeTimeStepStats(mut timeStepStats: *mut TTimeStepStats) {
    let mut timeStepCount: ::core::ffi::c_double =
        (*timeStepStats).timeStepCount as ::core::ffi::c_double;
    let mut totalRoutingTime: ::core::ffi::c_double = 0.;
    let mut fSteadyState: ::core::ffi::c_double = 0.0f64;
    if Nobjects[LINK as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
        || (*timeStepStats).timeStepCount as ::core::ffi::c_double == 0.0f64
    {
        return;
    }
    totalRoutingTime = (*timeStepStats).steadyStateTime + (*timeStepStats).routingTime;
    if totalRoutingTime > 0.0f64 {
        fSteadyState = 100.0f64 * (*timeStepStats).steadyStateTime / totalRoutingTime;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Routing Time Step Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*************************\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  Minimum Time Step           :  %7.2f sec\0" as *const u8
            as *const ::core::ffi::c_char,
        (*timeStepStats).minTimeStep,
    );
    fprintf(
        Frpt.file,
        b"\n  Average Time Step           :  %7.2f sec\0" as *const u8
            as *const ::core::ffi::c_char,
        (*timeStepStats).routingTime / timeStepCount,
    );
    fprintf(
        Frpt.file,
        b"\n  Maximum Time Step           :  %7.2f sec\0" as *const u8
            as *const ::core::ffi::c_char,
        (*timeStepStats).maxTimeStep,
    );
    fprintf(
        Frpt.file,
        b"\n  %% of Time in Steady State   :  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
        if fSteadyState <= 100.0f64 {
            fSteadyState
        } else {
            100.0f64
        },
    );
    fprintf(
        Frpt.file,
        b"\n  Average Iterations per Step :  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
        (*timeStepStats).trialsCount / timeStepCount,
    );
    fprintf(
        Frpt.file,
        b"\n  %% of Steps Not Converging   :  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
        100.0f64 * NonConvergeCount as ::core::ffi::c_double / timeStepCount,
    );
    if RouteModel == DW as ::core::ffi::c_int && CourantFactor > 0.0f64 {
        report_RouteStepFreq(timeStepStats);
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn report_RouteStepFreq(mut timeStepStats: *mut TTimeStepStats) {
    let mut totalSteps: ::core::ffi::c_double = 0.0f64;
    let mut i: ::core::ffi::c_int = 0;
    i = 1 as ::core::ffi::c_int;
    while i < TIMELEVELS {
        totalSteps += (*timeStepStats).timeStepCounts[i as usize] as ::core::ffi::c_double;
        i += 1;
    }
    if totalSteps == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return;
    }
    fprintf(
        Frpt.file,
        b"\n  Time Step Frequencies       :\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 1 as ::core::ffi::c_int;
    while i < TIMELEVELS {
        fprintf(
            Frpt.file,
            b"\n     %6.3f - %6.3f sec      :  %7.2f %%\0" as *const u8
                as *const ::core::ffi::c_char,
            (*timeStepStats).timeStepIntervals[(i - 1 as ::core::ffi::c_int) as usize],
            (*timeStepStats).timeStepIntervals[i as usize],
            100.0f64 * (*timeStepStats).timeStepCounts[i as usize] as ::core::ffi::c_double
                / totalSteps,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_writeReport() {
    if ErrorCode != 0 {
        return;
    }
    if Nperiods == 0 as ::core::ffi::c_long {
        return;
    }
    if RptFlags.subcatchments as ::core::ffi::c_int != NONE as ::core::ffi::c_int
        && (IgnoreRainfall == FALSE || IgnoreSnowmelt == FALSE || IgnoreGwater == FALSE)
    {
        report_Subcatchments();
    }
    if IgnoreRouting == TRUE && IgnoreQuality == TRUE {
        return;
    }
    if RptFlags.nodes as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
        report_Nodes();
    }
    if RptFlags.links as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
        report_Links();
    }
}
unsafe extern "C" fn report_Subcatchments() {
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut period: ::core::ffi::c_int = 0;
    let mut days: DateTime = 0.;
    let mut theDate: [::core::ffi::c_char; 12] = [0; 12];
    let mut theTime: [::core::ffi::c_char; 9] = [0; 9];
    let mut hasSnowmelt: ::core::ffi::c_int = (Nobjects[SNOWMELT as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
        && IgnoreSnowmelt == 0) as ::core::ffi::c_int;
    let mut hasGwater: ::core::ffi::c_int = (Nobjects[AQUIFER as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
        && IgnoreGwater == 0) as ::core::ffi::c_int;
    let mut hasQuality: ::core::ffi::c_int = (Nobjects[POLLUT as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
        && IgnoreQuality == 0) as ::core::ffi::c_int;
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(
        b"********************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    report_writeLine(
        b"Subcatchment Time Series Results\0" as *const u8 as *const ::core::ffi::c_char,
    );
    report_writeLine(
        b"********************************\0" as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        k = (*Subcatch.offset(j as isize)).rptFlag - 1 as ::core::ffi::c_int;
        if k >= 0 as ::core::ffi::c_int {
            report_SubcatchHeader((*Subcatch.offset(j as isize)).ID);
            period = 1 as ::core::ffi::c_int;
            while period as ::core::ffi::c_long <= Nperiods {
                output_readDateTime(period as ::core::ffi::c_long, &raw mut days);
                datetime_dateToStr(days, &raw mut theDate as *mut ::core::ffi::c_char);
                datetime_timeToStr(days, &raw mut theTime as *mut ::core::ffi::c_char);
                output_readSubcatchResults(period as ::core::ffi::c_long, k);
                fprintf(
                    Frpt.file,
                    b"\n  %11s %8s %10.3f%10.3f%10.4f\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut theDate as *mut ::core::ffi::c_char,
                    &raw mut theTime as *mut ::core::ffi::c_char,
                    *SubcatchResults.offset(SUBCATCH_RAINFALL as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                    *SubcatchResults.offset(SUBCATCH_EVAP as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double
                        / 24.0f64
                        + *SubcatchResults.offset(SUBCATCH_INFIL as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_double,
                    *SubcatchResults.offset(SUBCATCH_RUNOFF as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                );
                if hasSnowmelt != 0 {
                    fprintf(
                        Frpt.file,
                        b"  %10.3f\0" as *const u8 as *const ::core::ffi::c_char,
                        *SubcatchResults.offset(SUBCATCH_SNOWDEPTH as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_double,
                    );
                }
                if hasGwater != 0 {
                    fprintf(
                        Frpt.file,
                        b"%10.3f%10.4f\0" as *const u8 as *const ::core::ffi::c_char,
                        *SubcatchResults.offset(SUBCATCH_GW_ELEV as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_double,
                        *SubcatchResults.offset(SUBCATCH_GW_FLOW as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_double,
                    );
                }
                if hasQuality != 0 {
                    p = 0 as ::core::ffi::c_int;
                    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                        fprintf(
                            Frpt.file,
                            b"%10.3f\0" as *const u8 as *const ::core::ffi::c_char,
                            *SubcatchResults
                                .offset((SUBCATCH_WASHOFF as ::core::ffi::c_int + p) as isize)
                                as ::core::ffi::c_double,
                        );
                        p += 1;
                    }
                }
                period += 1;
            }
            report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        }
        j += 1;
    }
}
unsafe extern "C" fn report_SubcatchHeader(mut id: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut hasSnowmelt: ::core::ffi::c_int = (Nobjects[SNOWMELT as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
        && IgnoreSnowmelt == 0) as ::core::ffi::c_int;
    let mut hasGwater: ::core::ffi::c_int = (Nobjects[AQUIFER as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
        && IgnoreGwater == 0) as ::core::ffi::c_int;
    let mut hasQuality: ::core::ffi::c_int = (Nobjects[POLLUT as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
        && IgnoreQuality == 0) as ::core::ffi::c_int;
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  <<< Subcatchment %s >>>\0" as *const u8 as *const ::core::ffi::c_char,
        id,
    );
    report_writeLine(
        b"---------------------------------------------------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if hasSnowmelt > 0 as ::core::ffi::c_int {
        fprintf(Frpt.file, LINE_12.as_ptr());
    }
    if hasGwater != 0 {
        fprintf(Frpt.file, LINE_10.as_ptr());
        fprintf(Frpt.file, LINE_10.as_ptr());
    }
    if hasQuality != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(Frpt.file, LINE_10.as_ptr());
            i += 1;
        }
    }
    fprintf(
        Frpt.file,
        b"\n  Date        Time        Precip.    Losses    Runoff\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if hasSnowmelt != 0 {
        fprintf(
            Frpt.file,
            b"  Snow Depth\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if hasGwater != 0 {
        fprintf(
            Frpt.file,
            b"  GW Elev.   GW Flow\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if hasQuality != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%10s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Pollut.offset(i as isize)).ID,
            );
            i += 1;
        }
    }
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n                            in/hr     in/hr %9s\0" as *const u8
                as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n                            mm/hr     mm/hr %9s\0" as *const u8
                as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        );
    }
    if hasSnowmelt != 0 {
        if UnitSystem == US as ::core::ffi::c_int {
            fprintf(
                Frpt.file,
                b"      inches\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            fprintf(
                Frpt.file,
                b"     mmeters\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if hasGwater != 0 {
        if UnitSystem == US as ::core::ffi::c_int {
            fprintf(
                Frpt.file,
                b"      feet %9s\0" as *const u8 as *const ::core::ffi::c_char,
                *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                    .offset(FlowUnits as isize),
            );
        } else {
            fprintf(
                Frpt.file,
                b"    meters %9s\0" as *const u8 as *const ::core::ffi::c_char,
                *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                    .offset(FlowUnits as isize),
            );
        }
    }
    if hasQuality != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%10s\0" as *const u8 as *const ::core::ffi::c_char,
                *(&raw mut QualUnitsWords as *mut *mut ::core::ffi::c_char)
                    .offset((*Pollut.offset(i as isize)).units as isize),
            );
            i += 1;
        }
    }
    report_writeLine(
        b"---------------------------------------------------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if hasSnowmelt != 0 {
        fprintf(Frpt.file, LINE_12.as_ptr());
    }
    if hasGwater != 0 {
        fprintf(Frpt.file, LINE_10.as_ptr());
        fprintf(Frpt.file, LINE_10.as_ptr());
    }
    if hasQuality != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(Frpt.file, LINE_10.as_ptr());
            i += 1;
        }
    }
}
unsafe extern "C" fn report_Nodes() {
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut period: ::core::ffi::c_int = 0;
    let mut days: DateTime = 0.;
    let mut theDate: [::core::ffi::c_char; 12] = [0; 12];
    let mut theTime: [::core::ffi::c_char; 9] = [0; 9];
    if Nobjects[NODE as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Node Time Series Results\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"************************\0" as *const u8 as *const ::core::ffi::c_char);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        k = (*Node.offset(j as isize)).rptFlag - 1 as ::core::ffi::c_int;
        if k >= 0 as ::core::ffi::c_int {
            report_NodeHeader((*Node.offset(j as isize)).ID);
            period = 1 as ::core::ffi::c_int;
            while period as ::core::ffi::c_long <= Nperiods {
                output_readDateTime(period as ::core::ffi::c_long, &raw mut days);
                datetime_dateToStr(days, &raw mut theDate as *mut ::core::ffi::c_char);
                datetime_timeToStr(days, &raw mut theTime as *mut ::core::ffi::c_char);
                output_readNodeResults(period as ::core::ffi::c_long, k);
                fprintf(
                    Frpt.file,
                    b"\n  %11s %8s  %9.3f %9.3f %9.3f %9.3f\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &raw mut theDate as *mut ::core::ffi::c_char,
                    &raw mut theTime as *mut ::core::ffi::c_char,
                    *NodeResults.offset(NODE_INFLOW as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                    *NodeResults.offset(NODE_OVERFLOW as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                    *NodeResults.offset(NODE_DEPTH as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                    *NodeResults.offset(NODE_HEAD as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                );
                if IgnoreQuality == 0 {
                    p = 0 as ::core::ffi::c_int;
                    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                        fprintf(
                            Frpt.file,
                            b" %9.3f\0" as *const u8 as *const ::core::ffi::c_char,
                            *NodeResults.offset((NODE_QUAL as ::core::ffi::c_int + p) as isize)
                                as ::core::ffi::c_double,
                        );
                        p += 1;
                    }
                }
                period += 1;
            }
            report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        }
        j += 1;
    }
}
unsafe extern "C" fn report_NodeHeader(mut id: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut lengthUnits: [::core::ffi::c_char; 9] = [0; 9];
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  <<< Node %s >>>\0" as *const u8 as *const ::core::ffi::c_char,
        id,
    );
    report_writeLine(
        b"----------------------------------------------------------------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        fprintf(Frpt.file, LINE_10.as_ptr());
        i += 1;
    }
    fprintf(
        Frpt.file,
        b"\n                           Inflow  Flooding     Depth      Head\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if IgnoreQuality == 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%10s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Pollut.offset(i as isize)).ID,
            );
            i += 1;
        }
    }
    if UnitSystem == US as ::core::ffi::c_int {
        sstrncpy(
            &raw mut lengthUnits as *mut ::core::ffi::c_char,
            b"feet\0" as *const u8 as *const ::core::ffi::c_char,
            8 as size_t,
        );
    } else {
        sstrncpy(
            &raw mut lengthUnits as *mut ::core::ffi::c_char,
            b"meters\0" as *const u8 as *const ::core::ffi::c_char,
            8 as size_t,
        );
    }
    fprintf(
        Frpt.file,
        b"\n  Date        Time      %9s %9s %9s %9s\0" as *const u8 as *const ::core::ffi::c_char,
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        &raw mut lengthUnits as *mut ::core::ffi::c_char,
        &raw mut lengthUnits as *mut ::core::ffi::c_char,
    );
    if IgnoreQuality == 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%10s\0" as *const u8 as *const ::core::ffi::c_char,
                *(&raw mut QualUnitsWords as *mut *mut ::core::ffi::c_char)
                    .offset((*Pollut.offset(i as isize)).units as isize),
            );
            i += 1;
        }
    }
    report_writeLine(
        b"----------------------------------------------------------------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if IgnoreQuality == 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(Frpt.file, LINE_10.as_ptr());
            i += 1;
        }
    }
}
unsafe extern "C" fn report_Links() {
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut period: ::core::ffi::c_int = 0;
    let mut days: DateTime = 0.;
    let mut theDate: [::core::ffi::c_char; 12] = [0; 12];
    let mut theTime: [::core::ffi::c_char; 9] = [0; 9];
    if Nobjects[LINK as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Link Time Series Results\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"************************\0" as *const u8 as *const ::core::ffi::c_char);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        k = (*Link.offset(j as isize)).rptFlag - 1 as ::core::ffi::c_int;
        if k >= 0 as ::core::ffi::c_int {
            report_LinkHeader((*Link.offset(j as isize)).ID);
            period = 1 as ::core::ffi::c_int;
            while period as ::core::ffi::c_long <= Nperiods {
                output_readDateTime(period as ::core::ffi::c_long, &raw mut days);
                datetime_dateToStr(days, &raw mut theDate as *mut ::core::ffi::c_char);
                datetime_timeToStr(days, &raw mut theTime as *mut ::core::ffi::c_char);
                output_readLinkResults(period as ::core::ffi::c_long, k);
                fprintf(
                    Frpt.file,
                    b"\n  %11s %8s  %9.3f %9.3f %9.3f %9.3f\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &raw mut theDate as *mut ::core::ffi::c_char,
                    &raw mut theTime as *mut ::core::ffi::c_char,
                    *LinkResults.offset(LINK_FLOW as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                    *LinkResults.offset(LINK_VELOCITY as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                    *LinkResults.offset(LINK_DEPTH as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                    *LinkResults.offset(LINK_CAPACITY as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_double,
                );
                if IgnoreQuality == 0 {
                    p = 0 as ::core::ffi::c_int;
                    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                        fprintf(
                            Frpt.file,
                            b" %9.3f\0" as *const u8 as *const ::core::ffi::c_char,
                            *LinkResults.offset((LINK_QUAL as ::core::ffi::c_int + p) as isize)
                                as ::core::ffi::c_double,
                        );
                        p += 1;
                    }
                }
                period += 1;
            }
            report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        }
        j += 1;
    }
}
unsafe extern "C" fn report_LinkHeader(mut id: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  <<< Link %s >>>\0" as *const u8 as *const ::core::ffi::c_char,
        id,
    );
    report_writeLine(
        b"----------------------------------------------------------------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        fprintf(Frpt.file, LINE_10.as_ptr());
        i += 1;
    }
    fprintf(
        Frpt.file,
        b"\n                             Flow  Velocity     Depth  Capacity/\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if IgnoreQuality == 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%10s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Pollut.offset(i as isize)).ID,
            );
            i += 1;
        }
    }
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Date        Time     %10s    ft/sec      feet   Setting \0" as *const u8
                as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Date        Time     %10s     m/sec    meters   Setting \0" as *const u8
                as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        );
    }
    if IgnoreQuality == 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b" %9s\0" as *const u8 as *const ::core::ffi::c_char,
                *(&raw mut QualUnitsWords as *mut *mut ::core::ffi::c_char)
                    .offset((*Pollut.offset(i as isize)).units as isize),
            );
            i += 1;
        }
    }
    report_writeLine(
        b"----------------------------------------------------------------\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if IgnoreQuality == 0 {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(Frpt.file, LINE_10.as_ptr());
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_writeErrorMsg(
    mut code: ::core::ffi::c_int,
    mut s: *mut ::core::ffi::c_char,
) {
    if !Frpt.file.is_null() {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            error_getMsg(code, &raw mut Msg as *mut ::core::ffi::c_char),
            s,
        );
    }
    ErrorCode = code;
    if ErrorCode <= ERR_INPUT as ::core::ffi::c_int
        || ErrorCode >= ERR_FILE_NAME as ::core::ffi::c_int
    {
        snprintf(
            &raw mut ErrorMsg as *mut ::core::ffi::c_char,
            MAXMSG as size_t,
            error_getMsg(ErrorCode, &raw mut Msg as *mut ::core::ffi::c_char),
            s,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_writeErrorCode() {
    if !Frpt.file.is_null() {
        if ErrorCode >= ERR_MEMORY as ::core::ffi::c_int
            && ErrorCode <= ERR_TIMESTEP as ::core::ffi::c_int
            || ErrorCode >= ERR_FILE_NAME as ::core::ffi::c_int
                && ErrorCode <= ERR_OUT_FILE as ::core::ffi::c_int
            || ErrorCode == ERR_SYSTEM as ::core::ffi::c_int
        {
            fprintf(
                Frpt.file,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                error_getMsg(ErrorCode, &raw mut Msg as *mut ::core::ffi::c_char),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_writeInputErrorMsg(
    mut k: ::core::ffi::c_int,
    mut sect: ::core::ffi::c_int,
    mut line: *mut ::core::ffi::c_char,
    mut lineCount: ::core::ffi::c_long,
) {
    if !Frpt.file.is_null() {
        report_writeErrorMsg(k, &raw mut ErrString as *mut ::core::ffi::c_char);
        if sect < 0 as ::core::ffi::c_int {
            fprintf(Frpt.file, FMT17.as_ptr(), lineCount);
        } else {
            fprintf(
                Frpt.file,
                FMT18.as_ptr(),
                lineCount,
                *(&raw mut SectWords as *mut *mut ::core::ffi::c_char).offset(sect as isize),
            );
        }
        fprintf(
            Frpt.file,
            b"\n  %s\0" as *const u8 as *const ::core::ffi::c_char,
            line,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_writeWarningMsg(
    mut msg: *mut ::core::ffi::c_char,
    mut id: *mut ::core::ffi::c_char,
) {
    fprintf(
        Frpt.file,
        b"\n  %s %s\0" as *const u8 as *const ::core::ffi::c_char,
        msg,
        id,
    );
    Warnings += 1;
}
#[no_mangle]
pub unsafe extern "C" fn report_writeTseriesErrorMsg(
    mut code: ::core::ffi::c_int,
    mut tseries: *mut TTable,
) {
    let mut theDate: [::core::ffi::c_char; 20] = [0; 20];
    let mut theTime: [::core::ffi::c_char; 20] = [0; 20];
    let mut x: DateTime = 0.;
    if code == ERR_CURVE_SEQUENCE as ::core::ffi::c_int {
        x = (*tseries).x2 as DateTime;
        datetime_dateToStr(x, &raw mut theDate as *mut ::core::ffi::c_char);
        datetime_timeToStr(x, &raw mut theTime as *mut ::core::ffi::c_char);
        report_writeErrorMsg(ERR_TIMESERIES_SEQUENCE as ::core::ffi::c_int, (*tseries).ID);
        fprintf(
            Frpt.file,
            b" at %s %s.\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut theDate as *mut ::core::ffi::c_char,
            &raw mut theTime as *mut ::core::ffi::c_char,
        );
    } else {
        report_writeErrorMsg(code, (*tseries).ID);
    };
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
#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
