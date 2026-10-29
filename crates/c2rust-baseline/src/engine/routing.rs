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
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn datetime_monthOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_dayOfWeek(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_hourOfDay(date: DateTime) -> ::core::ffi::c_int;
    fn inlet_findCapturedFlows(tStep: ::core::ffi::c_double);
    fn inlet_adjustQualInflows();
    fn inlet_adjustQualOutflows();
    fn infil_setInfilFactor(j: ::core::ffi::c_int);
    static mut Fhotstart1: TFile;
    static mut Finflows: TFile;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut RouteModel: ::core::ffi::c_int;
    static mut SkipSteadyState: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut RuleStep: ::core::ffi::c_int;
    static mut NumEvents: ::core::ffi::c_int;
    static mut SysFlowTol: ::core::ffi::c_double;
    static mut LatFlowTol: ::core::ffi::c_double;
    static mut StartDateTime: DateTime;
    static mut ReportTime: ::core::ffi::c_double;
    static mut OldRunoffTime: ::core::ffi::c_double;
    static mut NewRunoffTime: ::core::ffi::c_double;
    static mut OldRoutingTime: ::core::ffi::c_double;
    static mut NewRoutingTime: ::core::ffi::c_double;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Outfall: *mut TOutfall;
    static mut Storage: *mut TStorage;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Pollut: *mut TPollut;
    static mut Event: *mut TEvent;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn rdii_getNumRdiiFlows(aDate: DateTime) -> ::core::ffi::c_int;
    fn rdii_getRdiiFlow(
        index: ::core::ffi::c_int,
        node: *mut ::core::ffi::c_int,
        q: *mut ::core::ffi::c_double,
    );
    fn flowrout_init(routingModel: ::core::ffi::c_int);
    fn flowrout_close(routingModel: ::core::ffi::c_int);
    fn flowrout_getRoutingStep(
        routingModel: ::core::ffi::c_int,
        fixedStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn flowrout_execute(
        links: *mut ::core::ffi::c_int,
        routingModel: ::core::ffi::c_int,
        tStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn toposort_sortLinks(links: *mut ::core::ffi::c_int);
    fn qualrout_init();
    fn qualrout_execute(tStep: ::core::ffi::c_double);
    fn treatmnt_open() -> ::core::ffi::c_int;
    fn treatmnt_close();
    fn massbal_updateRoutingTotals(tStep: ::core::ffi::c_double);
    fn massbal_initTimeStepTotals();
    fn massbal_addInflowFlow(type_0: ::core::ffi::c_int, q: ::core::ffi::c_double);
    fn massbal_addInflowQual(
        type_0: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
        w: ::core::ffi::c_double,
    );
    fn massbal_addOutflowFlow(q: ::core::ffi::c_double, isFlooded: ::core::ffi::c_int);
    fn massbal_addOutflowQual(
        pollut: ::core::ffi::c_int,
        mass: ::core::ffi::c_double,
        isFlooded: ::core::ffi::c_int,
    );
    fn massbal_addNodeLosses(evapLoss: ::core::ffi::c_double, infilLoss: ::core::ffi::c_double);
    fn massbal_addLinkLosses(evapLoss: ::core::ffi::c_double, infilLoss: ::core::ffi::c_double);
    fn massbal_getStepFlowError() -> ::core::ffi::c_double;
    fn stats_updateFlowStats(tStep: ::core::ffi::c_double, aDate: DateTime);
    fn stats_updateTimeStepStats(
        tStep: ::core::ffi::c_double,
        trialsCount: ::core::ffi::c_int,
        steadyState: ::core::ffi::c_int,
    );
    fn subcatch_getWtdOutflow(
        subcatch: ::core::ffi::c_int,
        wt: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn surfqual_getWtdWashoff(
        subcatch: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
        wt: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn node_initFlows(node: ::core::ffi::c_int, tStep: ::core::ffi::c_double);
    fn node_setOldHydState(node: ::core::ffi::c_int);
    fn node_setOldQualState(node: ::core::ffi::c_int);
    fn node_getLosses(
        node: ::core::ffi::c_int,
        tStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn node_getSystemOutflow(
        node: ::core::ffi::c_int,
        isFlooded: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn inflow_getExtInflow(inflow: *mut TExtInflow, aDate: DateTime) -> ::core::ffi::c_double;
    fn inflow_getDwfInflow(
        inflow: *mut TDwfInflow,
        m: ::core::ffi::c_int,
        d: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn iface_openRoutingFiles();
    fn iface_closeRoutingFiles();
    fn iface_getNumIfaceNodes(aDate: DateTime) -> ::core::ffi::c_int;
    fn iface_getIfaceNode(index: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn iface_getIfaceFlow(index: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn iface_getIfaceQual(
        index: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn link_setOldHydState(link: ::core::ffi::c_int);
    fn link_setOldQualState(link: ::core::ffi::c_int);
    fn link_setTargetSetting(j: ::core::ffi::c_int);
    fn link_setSetting(j: ::core::ffi::c_int, tstep: ::core::ffi::c_double);
    fn controls_evaluate(
        currentTime: DateTime,
        elapsedTime: DateTime,
        tStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
    fn lid_addDrainInflow(subcatch: ::core::ffi::c_int, f: ::core::ffi::c_double);
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
pub type FileUsageType = ::core::ffi::c_uint;
pub const SAVE_FILE: FileUsageType = 3;
pub const USE_FILE: FileUsageType = 2;
pub const SCRATCH_FILE: FileUsageType = 1;
pub const NO_FILE: FileUsageType = 0;
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
pub struct TEvent {
    pub start: DateTime,
    pub end: DateTime,
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
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TINY: ::core::ffi::c_double = 1.0E-6f64;
pub const FLOW_TOL: ::core::ffi::c_double = 0.00001f64;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;
static mut SortedLinks: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut NextEvent: ::core::ffi::c_int = 0;
static mut BetweenEvents: ::core::ffi::c_int = 0;
static mut NewRuleTime: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn routing_open() -> ::core::ffi::c_int {
    if treatmnt_open() == 0 {
        return ErrorCode;
    }
    SortedLinks = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        SortedLinks = calloc(
            Nobjects[LINK as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        ) as *mut ::core::ffi::c_int;
        if SortedLinks.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        }
        toposort_sortLinks(SortedLinks as *mut ::core::ffi::c_int);
        if ErrorCode != 0 {
            return ErrorCode;
        }
    }
    iface_openRoutingFiles();
    flowrout_init(RouteModel);
    if Fhotstart1.mode as ::core::ffi::c_int == NO_FILE as ::core::ffi::c_int {
        qualrout_init();
    }
    if NumEvents > 0 as ::core::ffi::c_int {
        sortEvents();
    }
    NextEvent = 0 as ::core::ffi::c_int;
    BetweenEvents = (NumEvents > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    NewRuleTime = 0.0f64;
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn routing_close(mut routingModel: ::core::ffi::c_int) {
    iface_closeRoutingFiles();
    flowrout_close(routingModel);
    treatmnt_close();
    if !SortedLinks.is_null() {
        free(SortedLinks as *mut ::core::ffi::c_void);
        SortedLinks = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn routing_getRoutingStep(
    mut routingModel: ::core::ffi::c_int,
    mut fixedStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut date1: ::core::ffi::c_double = 0.;
    let mut date2: ::core::ffi::c_double = 0.;
    let mut nextTime: ::core::ffi::c_double = 0.;
    let mut routingStep: ::core::ffi::c_double = 0.0f64;
    let mut nextRuleTime: ::core::ffi::c_double = 0.;
    let mut nextRoutingTime: ::core::ffi::c_double = 0.;
    if Nobjects[LINK as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return fixedStep;
    }
    if NumEvents > 0 as ::core::ffi::c_int && BetweenEvents != 0 {
        nextTime = if NewRunoffTime <= ReportTime {
            NewRunoffTime
        } else {
            ReportTime
        };
        date1 = getDateTime(NewRoutingTime) as ::core::ffi::c_double;
        date2 = getDateTime(nextTime) as ::core::ffi::c_double;
        if date2 > date1 && date2 < (*Event.offset(NextEvent as isize)).start {
            routingStep = (nextTime - NewRoutingTime) / 1000.0f64;
        } else {
            date1 = getDateTime(NewRoutingTime + 1000.0f64 * fixedStep) as ::core::ffi::c_double;
            if date1 < (*Event.offset(NextEvent as isize)).start {
                return fixedStep;
            }
        }
    }
    if routingStep == 0.0f64 {
        routingStep = flowrout_getRoutingStep(routingModel, fixedStep);
    }
    if RuleStep > 0 as ::core::ffi::c_int {
        nextRuleTime = NewRuleTime + 1000.0f64 * RuleStep as ::core::ffi::c_double;
        nextRoutingTime = NewRoutingTime + 1000.0f64 * routingStep;
        if nextRoutingTime >= nextRuleTime {
            routingStep = (nextRuleTime - NewRoutingTime) / 1000.0f64;
        }
    }
    return routingStep;
}
#[no_mangle]
pub unsafe extern "C" fn routing_execute(
    mut routingModel: ::core::ffi::c_int,
    mut routingStep: ::core::ffi::c_double,
) {
    let mut trialsCount: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut actionCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut inSteadyState: ::core::ffi::c_int = TRUE;
    let mut currentDate: DateTime = 0.;
    let mut stepFlowError: ::core::ffi::c_double = 0.;
    if ErrorCode != 0 {
        return;
    }
    massbal_updateRoutingTotals(routingStep / 2.0f64);
    currentDate = getDateTime(NewRoutingTime);
    actionCount = evaluateControlRules(currentDate, routingStep);
    stepFlowError = massbal_getStepFlowError();
    massbal_initTimeStepTotals();
    initSystemInflows();
    BetweenEvents = isBetweenEvents(currentDate);
    if BetweenEvents == FALSE {
        addSystemInflows(currentDate, routingStep);
        inlet_findCapturedFlows(routingStep);
        inSteadyState = isInSteadyState(actionCount, stepFlowError);
        if inSteadyState == FALSE {
            trialsCount = routeFlow(routingModel, routingStep);
        }
        if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
            && IgnoreQuality == 0
        {
            inlet_adjustQualInflows();
            qualrout_execute(routingStep);
        }
        removeSystemOutflows(routingStep);
        inlet_adjustQualOutflows();
        if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
            stats_updateFlowStats(routingStep, getDateTime(NewRoutingTime));
            stats_updateTimeStepStats(routingStep, trialsCount, inSteadyState);
        }
    }
    massbal_updateRoutingTotals(routingStep / 2.0f64);
}
unsafe extern "C" fn evaluateControlRules(
    mut currentDate: DateTime,
    mut routingStep: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut actionCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        link_setTargetSetting(j);
        j += 1;
    }
    if RuleStep == 0 as ::core::ffi::c_int || fabs(NewRoutingTime - NewRuleTime) < 1.0f64 {
        controls_evaluate(
            currentDate,
            currentDate - StartDateTime,
            routingStep / SECperDAY as ::core::ffi::c_double,
        );
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(j as isize)).targetSetting != (*Link.offset(j as isize)).setting {
            if (*Link.offset(j as isize)).targetSetting * (*Link.offset(j as isize)).setting
                == 0.0f64
            {
                (*Link.offset(j as isize)).timeLastSet = currentDate as ::core::ffi::c_double;
            }
            link_setSetting(j, routingStep);
            actionCount += 1;
        }
        j += 1;
    }
    OldRoutingTime = NewRoutingTime;
    NewRoutingTime = NewRoutingTime + 1000.0f64 * routingStep;
    if fabs(NewRoutingTime - (NewRuleTime + 1000.0f64 * RuleStep as ::core::ffi::c_double))
        < 1 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        NewRuleTime += 1000.0f64 * RuleStep as ::core::ffi::c_double;
    }
    return actionCount;
}
unsafe extern "C" fn initSystemInflows() {
    let mut j: ::core::ffi::c_int = 0;
    if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            node_setOldQualState(j);
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
            link_setOldQualState(j);
            j += 1;
        }
    }
    infil_setInfilFactor(-(1 as ::core::ffi::c_int));
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(j as isize)).oldLatFlow = (*Node.offset(j as isize)).newLatFlow;
        (*Node.offset(j as isize)).newLatFlow = 0.0f64;
        j += 1;
    }
}
unsafe extern "C" fn isBetweenEvents(mut currentDate: DateTime) -> ::core::ffi::c_int {
    if NumEvents == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if currentDate > (*Event.offset(NextEvent as isize)).end {
        NextEvent += 1;
        return TRUE;
    } else if currentDate >= (*Event.offset(NextEvent as isize)).start {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn addSystemInflows(
    mut currentDate: DateTime,
    mut routingStep: ::core::ffi::c_double,
) {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(j as isize)).losses = node_getLosses(j, routingStep);
        j += 1;
    }
    addExternalInflows(currentDate);
    addDryWeatherInflows(currentDate);
    addWetWeatherInflows(OldRoutingTime);
    addGroundwaterInflows(OldRoutingTime);
    addLidDrainInflows(OldRoutingTime);
    addRdiiInflows(currentDate);
    addIfaceInflows(currentDate);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(j as isize)).qualInflow = if 0.0f64 >= (*Node.offset(j as isize)).newLatFlow {
            0.0f64
        } else {
            (*Node.offset(j as isize)).newLatFlow
        };
        j += 1;
    }
}
unsafe extern "C" fn isInSteadyState(
    mut actionCount: ::core::ffi::c_int,
    mut stepFlowError: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if SkipSteadyState != 0 {
        if OldRoutingTime == 0.0f64
            || actionCount > 0 as ::core::ffi::c_int
            || fabs(stepFlowError) > SysFlowTol
            || inflowHasChanged() != 0
        {
            return FALSE;
        } else {
            return TRUE;
        }
    }
    return FALSE;
}
unsafe extern "C" fn routeFlow(
    mut routingModel: ::core::ffi::c_int,
    mut routingStep: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut stepCount: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        link_setOldHydState(j);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        node_setOldHydState(j);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        node_initFlows(j, routingStep);
        j += 1;
    }
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        stepCount = flowrout_execute(
            SortedLinks as *mut ::core::ffi::c_int,
            routingModel,
            routingStep,
        );
    }
    return stepCount;
}
unsafe extern "C" fn removeSystemOutflows(mut routingStep: ::core::ffi::c_double) {
    removeStorageLosses(routingStep);
    removeConduitLosses();
    removeOutflows(routingStep);
}
unsafe extern "C" fn addExternalInflows(mut currentDate: DateTime) {
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut inflow: *mut TExtInflow = ::core::ptr::null_mut::<TExtInflow>();
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        q = (*Node.offset(j as isize)).apiExtInflow;
        inflow = (*Node.offset(j as isize)).extInflow;
        while !inflow.is_null() {
            if (*inflow).type_0 == FLOW_INFLOW as ::core::ffi::c_int {
                q += inflow_getExtInflow(inflow, currentDate);
                break;
            } else {
                inflow = (*inflow).next as *mut TExtInflow;
            }
        }
        if fabs(q) < FLOW_TOL {
            q = 0.0f64;
        }
        (*Node.offset(j as isize)).newLatFlow += q;
        if q >= 0.0f64 {
            massbal_addInflowFlow(EXTERNAL_INFLOW as ::core::ffi::c_int, q);
            if (*Node.offset(j as isize)).type_0 == OUTFALL as ::core::ffi::c_int
                && (*Node.offset(j as isize)).oldNetInflow < 0.0f64
            {
                q = q - (*Node.offset(j as isize)).oldNetInflow;
            }
            inflow = (*Node.offset(j as isize)).extInflow;
            while !inflow.is_null() {
                if (*inflow).type_0 != FLOW_INFLOW as ::core::ffi::c_int {
                    p = (*inflow).param;
                    w = inflow_getExtInflow(inflow, currentDate);
                    if (*inflow).type_0 == CONCEN_INFLOW as ::core::ffi::c_int {
                        w *= q;
                    }
                    *(*Node.offset(j as isize)).newQual.offset(p as isize) += w;
                    massbal_addInflowQual(EXTERNAL_INFLOW as ::core::ffi::c_int, p, w);
                }
                inflow = (*inflow).next as *mut TExtInflow;
            }
        } else {
            massbal_addOutflowFlow(-q, FALSE);
        }
        j += 1;
    }
}
unsafe extern "C" fn addDryWeatherInflows(mut currentDate: DateTime) {
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut hour: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut inflow: *mut TDwfInflow = ::core::ptr::null_mut::<TDwfInflow>();
    month = datetime_monthOfYear(currentDate) - 1 as ::core::ffi::c_int;
    day = datetime_dayOfWeek(currentDate) - 1 as ::core::ffi::c_int;
    hour = datetime_hourOfDay(currentDate);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        inflow = (*Node.offset(j as isize)).dwfInflow;
        if !inflow.is_null() {
            q = 0.0f64;
            while !inflow.is_null() {
                if (*inflow).param < 0 as ::core::ffi::c_int {
                    q = inflow_getDwfInflow(inflow, month, day, hour);
                    break;
                } else {
                    inflow = (*inflow).next as *mut TDwfInflow;
                }
            }
            if fabs(q) < FLOW_TOL {
                q = 0.0f64;
            }
            (*Node.offset(j as isize)).newLatFlow += q;
            massbal_addInflowFlow(DRY_WEATHER_INFLOW as ::core::ffi::c_int, q);
            if !(q <= 0.0f64) {
                p = 0 as ::core::ffi::c_int;
                while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    if (*Pollut.offset(p as isize)).dwfConcen > 0.0f64 {
                        w = q * (*Pollut.offset(p as isize)).dwfConcen;
                        *(*Node.offset(j as isize)).newQual.offset(p as isize) += w;
                        massbal_addInflowQual(DRY_WEATHER_INFLOW as ::core::ffi::c_int, p, w);
                    }
                    p += 1;
                }
                inflow = (*Node.offset(j as isize)).dwfInflow;
                while !inflow.is_null() {
                    if (*inflow).param >= 0 as ::core::ffi::c_int {
                        p = (*inflow).param;
                        w = q * inflow_getDwfInflow(inflow, month, day, hour);
                        *(*Node.offset(j as isize)).newQual.offset(p as isize) += w;
                        massbal_addInflowQual(DRY_WEATHER_INFLOW as ::core::ffi::c_int, p, w);
                        if (*Pollut.offset(p as isize)).dwfConcen > 0.0f64 {
                            w = q * (*Pollut.offset(p as isize)).dwfConcen;
                            *(*Node.offset(j as isize)).newQual.offset(p as isize) -= w;
                            massbal_addInflowQual(DRY_WEATHER_INFLOW as ::core::ffi::c_int, p, -w);
                        }
                    }
                    inflow = (*inflow).next as *mut TDwfInflow;
                }
            }
        }
        j += 1;
    }
}
unsafe extern "C" fn addWetWeatherInflows(mut routingTime: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    f = (routingTime - OldRunoffTime) / (NewRunoffTime - OldRunoffTime);
    if f < 0.0f64 {
        f = 0.0f64;
    }
    if f > 1.0f64 {
        f = 1.0f64;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        j = (*Subcatch.offset(i as isize)).outNode;
        if j >= 0 as ::core::ffi::c_int {
            q = subcatch_getWtdOutflow(i, f);
            (*Node.offset(j as isize)).newLatFlow += q;
            massbal_addInflowFlow(WET_WEATHER_INFLOW as ::core::ffi::c_int, q);
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                w = surfqual_getWtdWashoff(i, p, f);
                *(*Node.offset(j as isize)).newQual.offset(p as isize) += w;
                massbal_addInflowQual(WET_WEATHER_INFLOW as ::core::ffi::c_int, p, w);
                p += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn addGroundwaterInflows(mut routingTime: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut gw: *mut TGroundwater = ::core::ptr::null_mut::<TGroundwater>();
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    f = (routingTime - OldRunoffTime) / (NewRunoffTime - OldRunoffTime);
    if f < 0.0f64 {
        f = 0.0f64;
    }
    if f > 1.0f64 {
        f = 1.0f64;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        gw = (*Subcatch.offset(i as isize)).groundwater;
        if !gw.is_null() {
            j = (*gw).node;
            if j >= 0 as ::core::ffi::c_int {
                q = ((1.0f64 - f) * (*gw).oldFlow + f * (*gw).newFlow)
                    * (*Subcatch.offset(i as isize)).area;
                if !(fabs(q) < FLOW_TOL) {
                    (*Node.offset(j as isize)).newLatFlow += q;
                    massbal_addInflowFlow(GROUNDWATER_INFLOW as ::core::ffi::c_int, q);
                    if q > 0.0f64 {
                        p = 0 as ::core::ffi::c_int;
                        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                            w = q * (*Pollut.offset(p as isize)).gwConcen;
                            *(*Node.offset(j as isize)).newQual.offset(p as isize) += w;
                            massbal_addInflowQual(GROUNDWATER_INFLOW as ::core::ffi::c_int, p, w);
                            p += 1;
                        }
                    }
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn addLidDrainInflows(mut routingTime: ::core::ffi::c_double) {
    let mut j: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    f = (routingTime - OldRunoffTime) / (NewRunoffTime - OldRunoffTime);
    if f < 0.0f64 {
        f = 0.0f64;
    }
    if f > 1.0f64 {
        f = 1.0f64;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        if (*Subcatch.offset(j as isize)).area > 0.0f64
            && (*Subcatch.offset(j as isize)).lidArea > 0.0f64
        {
            lid_addDrainInflow(j, f);
        }
        j += 1;
    }
}
unsafe extern "C" fn addRdiiInflows(mut currentDate: DateTime) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut numRdiiNodes: ::core::ffi::c_int = 0;
    numRdiiNodes = rdii_getNumRdiiFlows(currentDate);
    i = 0 as ::core::ffi::c_int;
    while i < numRdiiNodes {
        rdii_getRdiiFlow(i, &raw mut j, &raw mut q);
        if !(j < 0 as ::core::ffi::c_int) {
            if !(fabs(q) < FLOW_TOL) {
                (*Node.offset(j as isize)).newLatFlow += q;
                massbal_addInflowFlow(RDII_INFLOW as ::core::ffi::c_int, q);
                if q > 0.0f64 {
                    p = 0 as ::core::ffi::c_int;
                    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                        w = q * (*Pollut.offset(p as isize)).rdiiConcen;
                        *(*Node.offset(j as isize)).newQual.offset(p as isize) += w;
                        massbal_addInflowQual(RDII_INFLOW as ::core::ffi::c_int, p, w);
                        p += 1;
                    }
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn addIfaceInflows(mut currentDate: DateTime) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut numIfaceNodes: ::core::ffi::c_int = 0;
    if Finflows.mode as ::core::ffi::c_int != USE_FILE as ::core::ffi::c_int {
        return;
    }
    numIfaceNodes = iface_getNumIfaceNodes(currentDate);
    i = 0 as ::core::ffi::c_int;
    while i < numIfaceNodes {
        j = iface_getIfaceNode(i);
        if !(j < 0 as ::core::ffi::c_int) {
            q = iface_getIfaceFlow(i);
            if !(fabs(q) < FLOW_TOL) {
                (*Node.offset(j as isize)).newLatFlow += q;
                massbal_addInflowFlow(EXTERNAL_INFLOW as ::core::ffi::c_int, q);
                if q > 0.0f64 {
                    p = 0 as ::core::ffi::c_int;
                    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                        w = q * iface_getIfaceQual(i, p);
                        *(*Node.offset(j as isize)).newQual.offset(p as isize) += w;
                        massbal_addInflowQual(EXTERNAL_INFLOW as ::core::ffi::c_int, p, w);
                        p += 1;
                    }
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn inflowHasChanged() -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut diff: ::core::ffi::c_double = 0.;
    let mut qOld: ::core::ffi::c_double = 0.;
    let mut qNew: ::core::ffi::c_double = 0.;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        qOld = (*Node.offset(j as isize)).oldLatFlow;
        qNew = (*Node.offset(j as isize)).newLatFlow;
        if fabs(qOld) > TINY {
            diff = qNew / qOld - 1.0f64;
        } else if fabs(qNew) > TINY {
            diff = 1.0f64;
        } else {
            diff = 0.0f64;
        }
        if fabs(diff) > LatFlowTol {
            return TRUE;
        }
        if (*Node.offset(j as isize)).type_0 == OUTFALL as ::core::ffi::c_int
            || (*Node.offset(j as isize)).degree == 0 as ::core::ffi::c_int
        {
            qOld = (*Node.offset(j as isize)).oldFlowInflow;
            qNew = (*Node.offset(j as isize)).inflow;
            if fabs(qOld) > TINY {
                diff = qNew / qOld - 1.0f64;
            } else if fabs(qNew) > TINY {
                diff = 1.0f64;
            } else {
                diff = 0.0f64;
            }
            if fabs(diff) > LatFlowTol {
                return TRUE;
            }
        }
        j += 1;
    }
    return FALSE;
}
unsafe extern "C" fn removeStorageLosses(mut tStep: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut evapLoss: ::core::ffi::c_double = 0.0f64;
    let mut exfilLoss: ::core::ffi::c_double = 0.0f64;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if (*Node.offset(i as isize)).type_0 == STORAGE as ::core::ffi::c_int {
            evapLoss += (*Storage.offset((*Node.offset(i as isize)).subIndex as isize)).evapLoss;
            exfilLoss += (*Storage.offset((*Node.offset(i as isize)).subIndex as isize)).exfilLoss;
        }
        i += 1;
    }
    massbal_addNodeLosses(evapLoss / tStep, exfilLoss / tStep);
}
unsafe extern "C" fn removeConduitLosses() {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut barrels: ::core::ffi::c_double = 0.;
    let mut evapLoss: ::core::ffi::c_double = 0.0f64;
    let mut seepLoss: ::core::ffi::c_double = 0.0f64;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
            k = (*Link.offset(i as isize)).subIndex;
            barrels = (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_double;
            evapLoss += (*Conduit.offset(k as isize)).evapLossRate * barrels;
            seepLoss += (*Conduit.offset(k as isize)).seepLossRate * barrels;
        }
        i += 1;
    }
    massbal_addLinkLosses(evapLoss, seepLoss);
}
unsafe extern "C" fn removeOutflows(mut tStep: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut isFlooded: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if (*Node.offset(i as isize)).type_0 == OUTFALL as ::core::ffi::c_int
            && (*Node.offset(i as isize)).inflow > 0.0f64
        {
            k = (*Node.offset(i as isize)).subIndex;
            if (*Outfall.offset(k as isize)).routeTo >= 0 as ::core::ffi::c_int {
                v = (*Node.offset(i as isize)).inflow * tStep;
                (*Outfall.offset(k as isize)).vRouted += v;
                p = 0 as ::core::ffi::c_int;
                while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    *(*Outfall.offset(k as isize)).wRouted.offset(p as isize) +=
                        *(*Node.offset(i as isize)).newQual.offset(p as isize) * v;
                    p += 1;
                }
            }
        }
        q = node_getSystemOutflow(i, &raw mut isFlooded);
        if q > 0.0f64 {
            massbal_addOutflowFlow(q, isFlooded);
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                w = q * *(*Node.offset(i as isize)).newQual.offset(p as isize);
                massbal_addOutflowQual(p, w, isFlooded);
                p += 1;
            }
        } else {
            massbal_addInflowFlow(EXTERNAL_INFLOW as ::core::ffi::c_int, -q);
        }
        q = (*Node.offset(i as isize)).newLatFlow;
        if q < 0.0f64 {
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                w = -q * *(*Node.offset(i as isize)).newQual.offset(p as isize);
                massbal_addOutflowQual(p, w, FALSE);
                p += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn sortEvents() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut gap: ::core::ffi::c_int = 0;
    let mut temp: TEvent = TEvent { start: 0., end: 0. };
    gap = NumEvents / 2 as ::core::ffi::c_int;
    while gap >= 1 as ::core::ffi::c_int {
        i = gap;
        while i < NumEvents {
            temp = *Event.offset(i as isize);
            j = i - gap;
            while j >= 0 as ::core::ffi::c_int && (*Event.offset(j as isize)).start > temp.start {
                *Event.offset((j + gap) as isize) = *Event.offset(j as isize);
                j -= gap;
            }
            if j != i - gap {
                *Event.offset((j + gap) as isize) = temp;
            }
            i += gap;
        }
        gap /= 2 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumEvents - 1 as ::core::ffi::c_int {
        if (*Event.offset(i as isize)).end
            > (*Event.offset((i + 1 as ::core::ffi::c_int) as isize)).start
        {
            (*Event.offset(i as isize)).end =
                (*Event.offset((i + 1 as ::core::ffi::c_int) as isize)).start;
        }
        i += 1;
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
#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
