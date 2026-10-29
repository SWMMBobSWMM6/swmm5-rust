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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut Fout: TFile;
    static mut Foutflows: TFile;
    static mut Nperiods: ::core::ffi::c_long;
    static mut RptFlags: TRptFlags;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut UnitSystem: ::core::ffi::c_int;
    static mut FlowUnits: ::core::ffi::c_int;
    static mut IgnoreRouting: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut ReportStep: ::core::ffi::c_int;
    static mut ReportStart: DateTime;
    static mut StartDateTime: DateTime;
    static mut NewRunoffTime: ::core::ffi::c_double;
    static mut OldRunoffTime: ::core::ffi::c_double;
    static mut OldRoutingTime: ::core::ffi::c_double;
    static mut NewRoutingTime: ::core::ffi::c_double;
    static mut Temp: TTemp;
    static mut Evap: TEvap;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Pollut: *mut TPollut;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn stats_updateMaxNodeDepth(node: ::core::ffi::c_int, depth: ::core::ffi::c_double);
    fn gage_setReportRainfall(gage: ::core::ffi::c_int, aDate: DateTime);
    fn subcatch_getResults(
        subcatch: ::core::ffi::c_int,
        wt: ::core::ffi::c_double,
        x: *mut ::core::ffi::c_float,
    );
    fn node_getResults(
        node: ::core::ffi::c_int,
        wt: ::core::ffi::c_double,
        x: *mut ::core::ffi::c_float,
    );
    fn iface_saveOutletResults(reportDate: DateTime, file: *mut FILE);
    fn link_getResults(
        link: ::core::ffi::c_int,
        wt: ::core::ffi::c_double,
        x: *mut ::core::ffi::c_float,
    );
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getTempFileName(s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn writecon(s: *const ::core::ffi::c_char);
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type off_t = __off_t;
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
pub const ERR_OUT_WRITE: ErrorType = 309;
pub const INPUT_LENGTH: InputDataType = 5;
pub const INPUT_MAX_DEPTH: InputDataType = 3;
pub const INPUT_OFFSET: InputDataType = 4;
pub const INPUT_TYPE_CODE: InputDataType = 0;
pub const INPUT_INVERT: InputDataType = 2;
pub const INPUT_AREA: InputDataType = 1;
pub const ERR_MEMORY: ErrorType = 101;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAvgResults {
    pub xAvg: *mut ::core::ffi::c_float,
}
pub const ERR_OUT_FILE: ErrorType = 307;
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
pub const ERR_OUT_SIZE: ErrorType = 308;
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
pub type InputDataType = ::core::ffi::c_uint;
pub const VERSION: ::core::ffi::c_int = 52004 as ::core::ffi::c_int;
pub const MAGICNUMBER: ::core::ffi::c_int = 516114522 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FMT14: [::core::ffi::c_char; 30] = unsafe {
    ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
        *b"\n    Cannot open output file \0",
    )
};
pub const MAX_SUBCATCH_RESULTS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MAX_NODE_RESULTS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const MAX_LINK_RESULTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MAX_SYS_RESULTS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NO_DATE: ::core::ffi::c_int = -(693594 as ::core::ffi::c_int);
static mut IDStartPos: off_t = 0;
static mut InputStartPos: off_t = 0;
static mut OutputStartPos: off_t = 0;
static mut BytesPerPeriod: off_t = 0;
static mut NumSubcatchVars: ::core::ffi::c_int = 0;
static mut NumNodeVars: ::core::ffi::c_int = 0;
static mut NumLinkVars: ::core::ffi::c_int = 0;
static mut NumSubcatch: ::core::ffi::c_int = 0;
static mut NumNodes: ::core::ffi::c_int = 0;
static mut NumLinks: ::core::ffi::c_int = 0;
static mut NumPolluts: ::core::ffi::c_int = 0;
static mut SysResults: [::core::ffi::c_float; 15] = [0.; 15];
static mut AvgLinkResults: *mut TAvgResults =
    ::core::ptr::null::<TAvgResults>() as *mut TAvgResults;
static mut AvgNodeResults: *mut TAvgResults =
    ::core::ptr::null::<TAvgResults>() as *mut TAvgResults;
static mut Nsteps: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SubcatchResults: *mut ::core::ffi::c_float =
    ::core::ptr::null::<::core::ffi::c_float>() as *mut ::core::ffi::c_float;
#[no_mangle]
pub static mut NodeResults: *mut ::core::ffi::c_float =
    ::core::ptr::null::<::core::ffi::c_float>() as *mut ::core::ffi::c_float;
#[no_mangle]
pub static mut LinkResults: *mut ::core::ffi::c_float =
    ::core::ptr::null::<::core::ffi::c_float>() as *mut ::core::ffi::c_float;
#[no_mangle]
pub unsafe extern "C" fn output_open() -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_float = 0.;
    let mut z: ::core::ffi::c_double = 0.;
    let mut numResults: off_t = 0;
    output_openOutFile();
    if ErrorCode != 0 {
        return ErrorCode;
    }
    if IgnoreQuality != 0 {
        NumPolluts = 0 as ::core::ffi::c_int;
    } else {
        NumPolluts = Nobjects[POLLUT as ::core::ffi::c_int as usize];
    }
    NumSubcatchVars = MAX_SUBCATCH_RESULTS - 1 as ::core::ffi::c_int + NumPolluts;
    NumNodeVars = MAX_NODE_RESULTS - 1 as ::core::ffi::c_int + NumPolluts;
    NumLinkVars = MAX_LINK_RESULTS - 1 as ::core::ffi::c_int + NumPolluts;
    NumSubcatch = 0 as ::core::ffi::c_int;
    NumNodes = 0 as ::core::ffi::c_int;
    NumLinks = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        if (*Subcatch.offset(j as isize)).rptFlag != 0 {
            NumSubcatch += 1;
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if (*Node.offset(j as isize)).rptFlag != 0 {
            NumNodes += 1;
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(j as isize)).rptFlag != 0 {
            NumLinks += 1;
        }
        j += 1;
    }
    numResults = NumSubcatch as off_t * NumSubcatchVars as off_t
        + NumNodes as off_t * NumNodeVars as off_t
        + NumLinks as off_t * NumLinkVars as off_t
        + MAX_SYS_RESULTS as off_t;
    BytesPerPeriod = (::core::mem::size_of::<::core::ffi::c_double>() as usize).wrapping_add(
        (numResults as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize),
    ) as off_t;
    Nperiods = 0 as ::core::ffi::c_long;
    SubcatchResults = ::core::ptr::null_mut::<::core::ffi::c_float>();
    NodeResults = ::core::ptr::null_mut::<::core::ffi::c_float>();
    LinkResults = ::core::ptr::null_mut::<::core::ffi::c_float>();
    SubcatchResults = calloc(
        NumSubcatchVars as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    NodeResults = calloc(
        NumNodeVars as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    LinkResults = calloc(
        NumLinkVars as size_t,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    ) as *mut ::core::ffi::c_float;
    if SubcatchResults.is_null() || NodeResults.is_null() || LinkResults.is_null() {
        report_writeErrorMsg(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return ErrorCode;
    }
    AvgNodeResults = ::core::ptr::null_mut::<TAvgResults>();
    AvgLinkResults = ::core::ptr::null_mut::<TAvgResults>();
    if RptFlags.averages as ::core::ffi::c_int != 0 && output_openAvgResults() == 0 {
        report_writeErrorMsg(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return ErrorCode;
    }
    fseeko(Fout.file, 0 as __off_t, SEEK_SET);
    k = MAGICNUMBER;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = VERSION;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = FlowUnits;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NumSubcatch;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NumNodes;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NumLinks;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NumPolluts;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    IDStartPos = ftell(Fout.file) as off_t;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        if (*Subcatch.offset(j as isize)).rptFlag != 0 {
            output_saveID((*Subcatch.offset(j as isize)).ID, Fout.file);
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if (*Node.offset(j as isize)).rptFlag != 0 {
            output_saveID((*Node.offset(j as isize)).ID, Fout.file);
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(j as isize)).rptFlag != 0 {
            output_saveID((*Link.offset(j as isize)).ID, Fout.file);
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < NumPolluts {
        output_saveID((*Pollut.offset(j as isize)).ID, Fout.file);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < NumPolluts {
        k = (*Pollut.offset(j as isize)).units;
        fwrite(
            &raw mut k as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Fout.file,
        );
        j += 1;
    }
    InputStartPos = ftell(Fout.file) as off_t;
    k = 1 as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_AREA as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        if !((*Subcatch.offset(j as isize)).rptFlag == 0) {
            *SubcatchResults.offset(0 as ::core::ffi::c_int as isize) =
                ((*Subcatch.offset(j as isize)).area * UCF(LANDAREA as ::core::ffi::c_int))
                    as ::core::ffi::c_float;
            fwrite(
                SubcatchResults.offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                1 as size_t,
                Fout.file,
            );
        }
        j += 1;
    }
    k = 3 as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_TYPE_CODE as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_INVERT as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_MAX_DEPTH as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(j as isize)).rptFlag == 0) {
            k = (*Node.offset(j as isize)).type_0;
            *NodeResults.offset(0 as ::core::ffi::c_int as isize) =
                ((*Node.offset(j as isize)).invertElev * UCF(LENGTH as ::core::ffi::c_int))
                    as ::core::ffi::c_float;
            *NodeResults.offset(1 as ::core::ffi::c_int as isize) =
                ((*Node.offset(j as isize)).fullDepth * UCF(LENGTH as ::core::ffi::c_int))
                    as ::core::ffi::c_float;
            fwrite(
                &raw mut k as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                1 as size_t,
                Fout.file,
            );
            fwrite(
                NodeResults as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                2 as size_t,
                Fout.file,
            );
        }
        j += 1;
    }
    k = 5 as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_TYPE_CODE as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_OFFSET as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_OFFSET as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_MAX_DEPTH as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = INPUT_LENGTH as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if !((*Link.offset(j as isize)).rptFlag == 0) {
            k = (*Link.offset(j as isize)).type_0;
            if k == PUMP as ::core::ffi::c_int {
                m = 0 as ::core::ffi::c_int;
                while m < 4 as ::core::ffi::c_int {
                    *LinkResults.offset(m as isize) = 0.0f32;
                    m += 1;
                }
            } else {
                *LinkResults.offset(0 as ::core::ffi::c_int as isize) =
                    ((*Link.offset(j as isize)).offset1 * UCF(LENGTH as ::core::ffi::c_int))
                        as ::core::ffi::c_float;
                *LinkResults.offset(1 as ::core::ffi::c_int as isize) =
                    ((*Link.offset(j as isize)).offset2 * UCF(LENGTH as ::core::ffi::c_int))
                        as ::core::ffi::c_float;
                if ((*Link.offset(j as isize)).direction as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    x = *LinkResults.offset(0 as ::core::ffi::c_int as isize);
                    *LinkResults.offset(0 as ::core::ffi::c_int as isize) =
                        *LinkResults.offset(1 as ::core::ffi::c_int as isize);
                    *LinkResults.offset(1 as ::core::ffi::c_int as isize) = x;
                }
                if k == OUTLET as ::core::ffi::c_int {
                    *LinkResults.offset(2 as ::core::ffi::c_int as isize) = 0.0f32;
                } else {
                    *LinkResults.offset(2 as ::core::ffi::c_int as isize) = ((*Link
                        .offset(j as isize))
                    .xsect
                    .yFull
                        * UCF(LENGTH as ::core::ffi::c_int))
                        as ::core::ffi::c_float;
                }
                if k == CONDUIT as ::core::ffi::c_int {
                    m = (*Link.offset(j as isize)).subIndex;
                    *LinkResults.offset(3 as ::core::ffi::c_int as isize) =
                        ((*Conduit.offset(m as isize)).length * UCF(LENGTH as ::core::ffi::c_int))
                            as ::core::ffi::c_float;
                } else {
                    *LinkResults.offset(3 as ::core::ffi::c_int as isize) = 0.0f32;
                }
            }
            fwrite(
                &raw mut k as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                1 as size_t,
                Fout.file,
            );
            fwrite(
                LinkResults as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                4 as size_t,
                Fout.file,
            );
        }
        j += 1;
    }
    k = NumSubcatchVars;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = SUBCATCH_RAINFALL as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = SUBCATCH_SNOWDEPTH as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = SUBCATCH_EVAP as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = SUBCATCH_INFIL as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = SUBCATCH_RUNOFF as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = SUBCATCH_GW_FLOW as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = SUBCATCH_GW_ELEV as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = SUBCATCH_SOIL_MOIST as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    j = 0 as ::core::ffi::c_int;
    while j < NumPolluts {
        k = SUBCATCH_WASHOFF as ::core::ffi::c_int + j;
        fwrite(
            &raw mut k as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Fout.file,
        );
        j += 1;
    }
    k = NumNodeVars;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NODE_DEPTH as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NODE_HEAD as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NODE_VOLUME as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NODE_LATFLOW as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NODE_INFLOW as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = NODE_OVERFLOW as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    j = 0 as ::core::ffi::c_int;
    while j < NumPolluts {
        k = NODE_QUAL as ::core::ffi::c_int + j;
        fwrite(
            &raw mut k as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Fout.file,
        );
        j += 1;
    }
    k = NumLinkVars;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = LINK_FLOW as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = LINK_DEPTH as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = LINK_VELOCITY as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = LINK_VOLUME as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = LINK_CAPACITY as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    j = 0 as ::core::ffi::c_int;
    while j < NumPolluts {
        k = LINK_QUAL as ::core::ffi::c_int + j;
        fwrite(
            &raw mut k as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Fout.file,
        );
        j += 1;
    }
    k = MAX_SYS_RESULTS;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = 0 as ::core::ffi::c_int;
    while k < MAX_SYS_RESULTS {
        fwrite(
            &raw mut k as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Fout.file,
        );
        k += 1;
    }
    z = ReportStep as ::core::ffi::c_double / 86400.0f64;
    if StartDateTime as ::core::ffi::c_double + z > ReportStart {
        z = StartDateTime as ::core::ffi::c_double;
    } else {
        z = floor(
            (ReportStart as ::core::ffi::c_double - StartDateTime as ::core::ffi::c_double) / z,
        ) - 1.0f64;
        z = StartDateTime as ::core::ffi::c_double
            + z * ReportStep as ::core::ffi::c_double / 86400.0f64;
    }
    fwrite(
        &raw mut z as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = ReportStep;
    if fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    ) < 1 as ::core::ffi::c_ulong
    {
        report_writeErrorMsg(
            ERR_OUT_WRITE as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return ErrorCode;
    }
    OutputStartPos = ftell(Fout.file) as off_t;
    return ErrorCode;
}
unsafe extern "C" fn output_openOutFile() {
    if !Fout.file.is_null() {
        fclose(Fout.file);
    } else if strlen(&raw mut Fout.name as *mut ::core::ffi::c_char) != 0 as size_t {
        Fout.mode = SAVE_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    } else {
        Fout.mode = SCRATCH_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
        getTempFileName(&raw mut Fout.name as *mut ::core::ffi::c_char);
    }
    Fout.file = fopen(
        &raw mut Fout.name as *mut ::core::ffi::c_char,
        b"w+b\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Fout.file.is_null() {
        writecon(FMT14.as_ptr());
        ErrorCode = ERR_OUT_FILE as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn output_saveResults(mut reportTime: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    extern "C" {
        static mut StepFlowTotals: TRoutingTotals;
    }
    let mut reportDate: DateTime = getDateTime(reportTime);
    let mut date: ::core::ffi::c_double = 0.;
    if reportDate < ReportStart {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAX_SYS_RESULTS {
        SysResults[i as usize] = 0.0f32;
        i += 1;
    }
    date = reportDate as ::core::ffi::c_double;
    fwrite(
        &raw mut date as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        1 as size_t,
        Fout.file,
    );
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        output_saveSubcatchResults(reportTime, Fout.file);
    }
    if RptFlags.averages != 0 {
        output_saveAvgResults(Fout.file);
    } else {
        if Nobjects[NODE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
            output_saveNodeResults(reportTime, Fout.file);
        }
        if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
            output_saveLinkResults(reportTime, Fout.file);
        }
    }
    SysResults[SYS_FLOODING as ::core::ffi::c_int as usize] =
        (StepFlowTotals.flooding * UCF(FLOW as ::core::ffi::c_int)) as ::core::ffi::c_float;
    SysResults[SYS_OUTFLOW as ::core::ffi::c_int as usize] =
        (StepFlowTotals.outflow * UCF(FLOW as ::core::ffi::c_int)) as ::core::ffi::c_float;
    SysResults[SYS_DWFLOW as ::core::ffi::c_int as usize] =
        (StepFlowTotals.dwInflow * UCF(FLOW as ::core::ffi::c_int)) as ::core::ffi::c_float;
    SysResults[SYS_GWFLOW as ::core::ffi::c_int as usize] =
        (StepFlowTotals.gwInflow * UCF(FLOW as ::core::ffi::c_int)) as ::core::ffi::c_float;
    SysResults[SYS_IIFLOW as ::core::ffi::c_int as usize] =
        (StepFlowTotals.iiInflow * UCF(FLOW as ::core::ffi::c_int)) as ::core::ffi::c_float;
    SysResults[SYS_EXFLOW as ::core::ffi::c_int as usize] =
        (StepFlowTotals.exInflow * UCF(FLOW as ::core::ffi::c_int)) as ::core::ffi::c_float;
    SysResults[SYS_INFLOW as ::core::ffi::c_int as usize] = SysResults
        [SYS_RUNOFF as ::core::ffi::c_int as usize]
        + SysResults[SYS_DWFLOW as ::core::ffi::c_int as usize]
        + SysResults[SYS_GWFLOW as ::core::ffi::c_int as usize]
        + SysResults[SYS_IIFLOW as ::core::ffi::c_int as usize]
        + SysResults[SYS_EXFLOW as ::core::ffi::c_int as usize];
    fwrite(
        &raw mut SysResults as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        MAX_SYS_RESULTS as size_t,
        Fout.file,
    );
    if Foutflows.mode as ::core::ffi::c_int == SAVE_FILE as ::core::ffi::c_int && IgnoreRouting == 0
    {
        iface_saveOutletResults(reportDate, Foutflows.file);
    }
    Nperiods += 1;
}
#[no_mangle]
pub unsafe extern "C" fn output_end() {
    let mut k: ::core::ffi::c_int = 0;
    fwrite(
        &raw mut IDStartPos as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    fwrite(
        &raw mut InputStartPos as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    fwrite(
        &raw mut OutputStartPos as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = Nperiods as ::core::ffi::c_int;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = ErrorCode;
    fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    );
    k = MAGICNUMBER;
    if fwrite(
        &raw mut k as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Fout.file,
    ) < 1 as ::core::ffi::c_ulong
    {
        report_writeErrorMsg(
            ERR_OUT_WRITE as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn output_close() {
    if !SubcatchResults.is_null() {
        free(SubcatchResults as *mut ::core::ffi::c_void);
        SubcatchResults = ::core::ptr::null_mut::<::core::ffi::c_float>();
    }
    if !NodeResults.is_null() {
        free(NodeResults as *mut ::core::ffi::c_void);
        NodeResults = ::core::ptr::null_mut::<::core::ffi::c_float>();
    }
    if !LinkResults.is_null() {
        free(LinkResults as *mut ::core::ffi::c_void);
        LinkResults = ::core::ptr::null_mut::<::core::ffi::c_float>();
    }
    output_closeAvgResults();
}
unsafe extern "C" fn output_saveID(mut id: *mut ::core::ffi::c_char, mut file: *mut FILE) {
    let mut n: ::core::ffi::c_int = strlen(id) as ::core::ffi::c_int;
    fwrite(
        &raw mut n as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        file,
    );
    fwrite(
        id as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        n as size_t,
        file,
    );
}
unsafe extern "C" fn output_saveSubcatchResults(
    mut reportTime: ::core::ffi::c_double,
    mut file: *mut FILE,
) {
    let mut j: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = 0.;
    let mut totalArea: ::core::ffi::c_float = 0.0f32;
    let mut reportDate: DateTime = getDateTime(reportTime);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        gage_setReportRainfall(j, reportDate);
        j += 1;
    }
    f = (reportTime - OldRunoffTime) / (NewRunoffTime - OldRunoffTime);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        subcatch_getResults(j, f, SubcatchResults as *mut ::core::ffi::c_float);
        if (*Subcatch.offset(j as isize)).rptFlag != 0 {
            fwrite(
                SubcatchResults as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                NumSubcatchVars as size_t,
                file,
            );
        }
        area = (*Subcatch.offset(j as isize)).area * UCF(LANDAREA as ::core::ffi::c_int);
        totalArea += area as ::core::ffi::c_float;
        SysResults[SYS_RAINFALL as ::core::ffi::c_int as usize] +=
            (*SubcatchResults.offset(SUBCATCH_RAINFALL as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                * area) as ::core::ffi::c_float;
        SysResults[SYS_SNOWDEPTH as ::core::ffi::c_int as usize] +=
            (*SubcatchResults.offset(SUBCATCH_SNOWDEPTH as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                * area) as ::core::ffi::c_float;
        SysResults[SYS_EVAP as ::core::ffi::c_int as usize] +=
            (*SubcatchResults.offset(SUBCATCH_EVAP as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                * area) as ::core::ffi::c_float;
        if !(*Subcatch.offset(j as isize)).groundwater.is_null() {
            SysResults[SYS_EVAP as ::core::ffi::c_int as usize] +=
                ((*(*Subcatch.offset(j as isize)).groundwater).evapLoss
                    * UCF(EVAPRATE as ::core::ffi::c_int)
                    * area) as ::core::ffi::c_float;
        }
        SysResults[SYS_INFIL as ::core::ffi::c_int as usize] +=
            (*SubcatchResults.offset(SUBCATCH_INFIL as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                * area) as ::core::ffi::c_float;
        SysResults[SYS_RUNOFF as ::core::ffi::c_int as usize] +=
            *SubcatchResults.offset(SUBCATCH_RUNOFF as ::core::ffi::c_int as isize);
        j += 1;
    }
    if totalArea as ::core::ffi::c_double > 0.0f64 {
        SysResults[SYS_EVAP as ::core::ffi::c_int as usize] /= totalArea;
        SysResults[SYS_RAINFALL as ::core::ffi::c_int as usize] /= totalArea;
        SysResults[SYS_SNOWDEPTH as ::core::ffi::c_int as usize] /= totalArea;
        SysResults[SYS_INFIL as ::core::ffi::c_int as usize] /= totalArea;
    }
    if UnitSystem == SI as ::core::ffi::c_int {
        f = 5.0f64 / 9.0f64 * (Temp.ta - 32.0f64);
    } else {
        f = Temp.ta;
    }
    SysResults[SYS_TEMPERATURE as ::core::ffi::c_int as usize] = f as ::core::ffi::c_float;
    f = Evap.rate * UCF(EVAPRATE as ::core::ffi::c_int);
    SysResults[SYS_PET as ::core::ffi::c_int as usize] = f as ::core::ffi::c_float;
}
unsafe extern "C" fn output_saveNodeResults(
    mut reportTime: ::core::ffi::c_double,
    mut file: *mut FILE,
) {
    let mut j: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double =
        (reportTime - OldRoutingTime) / (NewRoutingTime - OldRoutingTime);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        node_getResults(j, f, NodeResults as *mut ::core::ffi::c_float);
        if (*Node.offset(j as isize)).rptFlag != 0 {
            fwrite(
                NodeResults as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                NumNodeVars as size_t,
                file,
            );
        }
        stats_updateMaxNodeDepth(
            j,
            *NodeResults.offset(NODE_DEPTH as ::core::ffi::c_int as isize) as ::core::ffi::c_double,
        );
        SysResults[SYS_STORAGE as ::core::ffi::c_int as usize] +=
            *NodeResults.offset(NODE_VOLUME as ::core::ffi::c_int as isize);
        j += 1;
    }
}
unsafe extern "C" fn output_saveLinkResults(
    mut reportTime: ::core::ffi::c_double,
    mut file: *mut FILE,
) {
    let mut j: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut z: ::core::ffi::c_double = 0.;
    f = (reportTime - OldRoutingTime) / (NewRoutingTime - OldRoutingTime);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(j as isize)).rptFlag != 0 {
            link_getResults(j, f, LinkResults as *mut ::core::ffi::c_float);
            fwrite(
                LinkResults as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                NumLinkVars as size_t,
                file,
            );
        }
        z = ((1.0f64 - f) * (*Link.offset(j as isize)).oldVolume
            + f * (*Link.offset(j as isize)).newVolume)
            * UCF(VOLUME as ::core::ffi::c_int);
        SysResults[SYS_STORAGE as ::core::ffi::c_int as usize] += z as ::core::ffi::c_float;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn output_readDateTime(
    mut period: ::core::ffi::c_long,
    mut days: *mut DateTime,
) {
    let mut p: off_t = period as off_t;
    let mut bytePos: off_t = OutputStartPos + (p - 1 as off_t) * BytesPerPeriod;
    fseeko(Fout.file, bytePos as __off_t, SEEK_SET);
    *days = NO_DATE as DateTime;
    fread(
        days as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        1 as size_t,
        Fout.file,
    );
}
#[no_mangle]
pub unsafe extern "C" fn output_readSubcatchResults(
    mut period: ::core::ffi::c_long,
    mut index: ::core::ffi::c_int,
) {
    let mut offset: ::core::ffi::c_long = (index * NumSubcatchVars) as ::core::ffi::c_long;
    let mut p: off_t = period as off_t;
    let mut bytePos: off_t = ((OutputStartPos + (p - 1 as off_t) * BytesPerPeriod) as usize)
        .wrapping_add(::core::mem::size_of::<::core::ffi::c_double>() as usize)
        .wrapping_add(
            (offset as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize),
        ) as off_t;
    fseeko(Fout.file, bytePos as __off_t, SEEK_SET);
    fread(
        SubcatchResults as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        NumSubcatchVars as size_t,
        Fout.file,
    );
}
#[no_mangle]
pub unsafe extern "C" fn output_readNodeResults(
    mut period: ::core::ffi::c_long,
    mut index: ::core::ffi::c_int,
) {
    let mut offset: ::core::ffi::c_long =
        (NumSubcatch * NumSubcatchVars + index * NumNodeVars) as ::core::ffi::c_long;
    let mut p: off_t = period as off_t;
    let mut bytePos: off_t = ((OutputStartPos + (p - 1 as off_t) * BytesPerPeriod) as usize)
        .wrapping_add(::core::mem::size_of::<::core::ffi::c_double>() as usize)
        .wrapping_add(
            (offset as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize),
        ) as off_t;
    fseeko(Fout.file, bytePos as __off_t, SEEK_SET);
    fread(
        NodeResults as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        NumNodeVars as size_t,
        Fout.file,
    );
}
#[no_mangle]
pub unsafe extern "C" fn output_readLinkResults(
    mut period: ::core::ffi::c_long,
    mut index: ::core::ffi::c_int,
) {
    let mut offset: ::core::ffi::c_long = (NumSubcatch * NumSubcatchVars
        + NumNodes * NumNodeVars
        + index * NumLinkVars) as ::core::ffi::c_long;
    let mut p: off_t = period as off_t;
    let mut bytePos: off_t = ((OutputStartPos + (p - 1 as off_t) * BytesPerPeriod) as usize)
        .wrapping_add(::core::mem::size_of::<::core::ffi::c_double>() as usize)
        .wrapping_add(
            (offset as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize),
        ) as off_t;
    fseeko(Fout.file, bytePos as __off_t, SEEK_SET);
    fread(
        LinkResults as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        NumLinkVars as size_t,
        Fout.file,
    );
    fread(
        &raw mut SysResults as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        MAX_SYS_RESULTS as size_t,
        Fout.file,
    );
}
unsafe extern "C" fn output_openAvgResults() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    AvgNodeResults = calloc(
        NumNodes as size_t,
        ::core::mem::size_of::<TAvgResults>() as size_t,
    ) as *mut TAvgResults;
    if AvgNodeResults.is_null() {
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumNodes {
        let ref mut fresh0 = (*AvgNodeResults.offset(i as isize)).xAvg;
        *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_float>();
        i += 1;
    }
    AvgLinkResults = calloc(
        NumLinks as size_t,
        ::core::mem::size_of::<TAvgResults>() as size_t,
    ) as *mut TAvgResults;
    if AvgLinkResults.is_null() {
        output_closeAvgResults();
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumLinks {
        let ref mut fresh1 = (*AvgLinkResults.offset(i as isize)).xAvg;
        *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_float>();
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumNodes {
        let ref mut fresh2 = (*AvgNodeResults.offset(i as isize)).xAvg;
        *fresh2 = calloc(
            NumNodeVars as size_t,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        ) as *mut ::core::ffi::c_float;
        if (*AvgNodeResults.offset(i as isize)).xAvg.is_null() {
            output_closeAvgResults();
            return FALSE;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumLinks {
        let ref mut fresh3 = (*AvgLinkResults.offset(i as isize)).xAvg;
        *fresh3 = calloc(
            NumLinkVars as size_t,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        ) as *mut ::core::ffi::c_float;
        if (*AvgLinkResults.offset(i as isize)).xAvg.is_null() {
            output_closeAvgResults();
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
unsafe extern "C" fn output_closeAvgResults() {
    let mut i: ::core::ffi::c_int = 0;
    if !AvgNodeResults.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < NumNodes {
            if !(*AvgNodeResults.offset(i as isize)).xAvg.is_null() {
                free((*AvgNodeResults.offset(i as isize)).xAvg as *mut ::core::ffi::c_void);
                let ref mut fresh4 = (*AvgNodeResults.offset(i as isize)).xAvg;
                *fresh4 = ::core::ptr::null_mut::<::core::ffi::c_float>();
            }
            i += 1;
        }
        if !AvgNodeResults.is_null() {
            free(AvgNodeResults as *mut ::core::ffi::c_void);
            AvgNodeResults = ::core::ptr::null_mut::<TAvgResults>();
        }
    }
    if !AvgLinkResults.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < NumLinks {
            if !(*AvgLinkResults.offset(i as isize)).xAvg.is_null() {
                free((*AvgLinkResults.offset(i as isize)).xAvg as *mut ::core::ffi::c_void);
                let ref mut fresh5 = (*AvgLinkResults.offset(i as isize)).xAvg;
                *fresh5 = ::core::ptr::null_mut::<::core::ffi::c_float>();
            }
            i += 1;
        }
        if !AvgLinkResults.is_null() {
            free(AvgLinkResults as *mut ::core::ffi::c_void);
            AvgLinkResults = ::core::ptr::null_mut::<TAvgResults>();
        }
    }
}
unsafe extern "C" fn output_initAvgResults() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    Nsteps = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < NumNodes {
        j = 0 as ::core::ffi::c_int;
        while j < NumNodeVars {
            *(*AvgNodeResults.offset(i as isize)).xAvg.offset(j as isize) = 0.0f32;
            j += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumLinks {
        j = 0 as ::core::ffi::c_int;
        while j < NumLinkVars {
            *(*AvgLinkResults.offset(i as isize)).xAvg.offset(j as isize) = 0.0f32;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn output_updateAvgResults() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut sign: ::core::ffi::c_int = 0;
    k = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(i as isize)).rptFlag == 0) {
            node_getResults(i, 1.0f64, NodeResults as *mut ::core::ffi::c_float);
            j = 0 as ::core::ffi::c_int;
            while j < NumNodeVars {
                *(*AvgNodeResults.offset(k as isize)).xAvg.offset(j as isize) +=
                    *NodeResults.offset(j as isize);
                j += 1;
            }
            k += 1;
        }
        i += 1;
    }
    k = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if !((*Link.offset(i as isize)).rptFlag == 0) {
            link_getResults(i, 1.0f64, LinkResults as *mut ::core::ffi::c_float);
            sign = if *LinkResults.offset(LINK_FLOW as ::core::ffi::c_int as isize)
                < 0 as ::core::ffi::c_int as ::core::ffi::c_float
            {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
            j = 0 as ::core::ffi::c_int;
            while j < NumLinkVars {
                if j == LINK_CAPACITY as ::core::ffi::c_int {
                    if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
                        *(*AvgLinkResults.offset(k as isize)).xAvg.offset(j as isize) +=
                            *LinkResults.offset(j as isize);
                    } else {
                        *(*AvgLinkResults.offset(k as isize)).xAvg.offset(j as isize) = *LinkResults
                            .offset(j as isize)
                            * (Nsteps + 1 as ::core::ffi::c_int) as ::core::ffi::c_float;
                    }
                } else {
                    *(*AvgLinkResults.offset(k as isize)).xAvg.offset(j as isize) +=
                        *LinkResults.offset(j as isize);
                }
                j += 1;
            }
            k += 1;
        }
        i += 1;
    }
    Nsteps += 1;
}
unsafe extern "C" fn output_saveAvgResults(mut file: *mut FILE) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < NumNodes {
        j = 0 as ::core::ffi::c_int;
        while j < NumNodeVars {
            *NodeResults.offset(j as isize) =
                *(*AvgNodeResults.offset(i as isize)).xAvg.offset(j as isize)
                    / Nsteps as ::core::ffi::c_float;
            j += 1;
        }
        fwrite(
            NodeResults as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            NumNodeVars as size_t,
            file,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        stats_updateMaxNodeDepth(
            i,
            (*Node.offset(i as isize)).newDepth * UCF(LENGTH as ::core::ffi::c_int),
        );
        SysResults[SYS_STORAGE as ::core::ffi::c_int as usize] +=
            ((*Node.offset(i as isize)).newVolume * UCF(VOLUME as ::core::ffi::c_int))
                as ::core::ffi::c_float;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumLinks {
        j = 0 as ::core::ffi::c_int;
        while j < NumLinkVars {
            *LinkResults.offset(j as isize) =
                *(*AvgLinkResults.offset(i as isize)).xAvg.offset(j as isize)
                    / Nsteps as ::core::ffi::c_float;
            j += 1;
        }
        fwrite(
            LinkResults as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            NumLinkVars as size_t,
            file,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        SysResults[SYS_STORAGE as ::core::ffi::c_int as usize] +=
            ((*Link.offset(i as isize)).newVolume * UCF(VOLUME as ::core::ffi::c_int))
                as ::core::ffi::c_float;
        i += 1;
    }
    output_initAvgResults();
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
