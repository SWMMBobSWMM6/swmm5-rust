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
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut ReportStepCount: ::core::ffi::c_long;
    static mut RptFlags: TRptFlags;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut Nnodes: [::core::ffi::c_int; 5];
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut RouteModel: ::core::ffi::c_int;
    static mut AllowPonding: ::core::ffi::c_int;
    static mut Nlinks: [::core::ffi::c_int; 5];
    static mut CourantFactor: ::core::ffi::c_double;
    static mut RouteStep: ::core::ffi::c_double;
    static mut MinRouteStep: ::core::ffi::c_double;
    static mut StartDateTime: DateTime;
    static mut ReportStart: DateTime;
    static mut OldRoutingTime: ::core::ffi::c_double;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Storage: *mut TStorage;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    fn report_writeMaxStats(
        massBalErrs: *mut TMaxStats,
        CourantCrit: *mut TMaxStats,
        nMaxStats: ::core::ffi::c_int,
    );
    fn report_writeMaxFlowTurns(flowTurns: *mut TMaxStats, nMaxStats: ::core::ffi::c_int);
    fn report_writeNonconvergedStats(
        maxNonconverged: *mut TMaxStats,
        nMaxStats: ::core::ffi::c_int,
    );
    fn report_writeTimeStepStats(timeStepStats: *mut TTimeStepStats);
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn statsrpt_writeReport();
    fn link_getVelocity(
        link: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn link_getPower(link: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn street_getExtentFilled(link: ::core::ffi::c_int) -> ::core::ffi::c_double;
    static mut NodeInflow: *mut ::core::ffi::c_double;
    static mut NodeOutflow: *mut ::core::ffi::c_double;
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
pub type LinkType = ::core::ffi::c_uint;
pub const OUTLET: LinkType = 4;
pub const WEIR: LinkType = 3;
pub const ORIFICE: LinkType = 2;
pub const PUMP: LinkType = 1;
pub const CONDUIT: LinkType = 0;
pub type XsectType = ::core::ffi::c_uint;
pub const STREET_XSECT: XsectType = 25;
pub const FORCE_MAIN: XsectType = 24;
pub const CUSTOM: XsectType = 23;
pub const IRREGULAR: XsectType = 22;
pub const SEMICIRCULAR: XsectType = 21;
pub const BASKETHANDLE: XsectType = 20;
pub const SEMIELLIPTICAL: XsectType = 19;
pub const CATENARY: XsectType = 18;
pub const GOTHIC: XsectType = 17;
pub const HORSESHOE: XsectType = 16;
pub const EGGSHAPED: XsectType = 15;
pub const ARCH: XsectType = 14;
pub const VERT_ELLIPSE: XsectType = 13;
pub const HORIZ_ELLIPSE: XsectType = 12;
pub const MOD_BASKET: XsectType = 11;
pub const RECT_ROUND: XsectType = 10;
pub const RECT_TRIANG: XsectType = 9;
pub const POWERFUNC: XsectType = 8;
pub const PARABOLIC: XsectType = 7;
pub const TRIANGULAR: XsectType = 6;
pub const TRAPEZOIDAL: XsectType = 5;
pub const RECT_OPEN: XsectType = 4;
pub const RECT_CLOSED: XsectType = 3;
pub const FILLED_CIRCULAR: XsectType = 2;
pub const CIRCULAR: XsectType = 1;
pub const DUMMY: XsectType = 0;
pub type FlowClassType = ::core::ffi::c_uint;
pub const ALL_FULL: FlowClassType = 10;
pub const DN_FULL: FlowClassType = 9;
pub const UP_FULL: FlowClassType = 8;
pub const MAX_FLOW_CLASSES: FlowClassType = 7;
pub const DN_CRITICAL: FlowClassType = 6;
pub const UP_CRITICAL: FlowClassType = 5;
pub const SUPCRITICAL: FlowClassType = 4;
pub const SUBCRITICAL: FlowClassType = 3;
pub const DN_DRY: FlowClassType = 2;
pub const UP_DRY: FlowClassType = 1;
pub const DRY: FlowClassType = 0;
pub type RouteModelType = ::core::ffi::c_uint;
pub const DW: RouteModelType = 4;
pub const EKW: RouteModelType = 3;
pub const KW: RouteModelType = 2;
pub const SF: RouteModelType = 1;
pub const NO_ROUTING: RouteModelType = 0;
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
pub struct TSubcatchStats {
    pub precip: ::core::ffi::c_double,
    pub runon: ::core::ffi::c_double,
    pub evap: ::core::ffi::c_double,
    pub infil: ::core::ffi::c_double,
    pub runoff: ::core::ffi::c_double,
    pub maxFlow: ::core::ffi::c_double,
    pub impervRunoff: ::core::ffi::c_double,
    pub pervRunoff: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TNodeStats {
    pub avgDepth: ::core::ffi::c_double,
    pub maxDepth: ::core::ffi::c_double,
    pub maxDepthDate: DateTime,
    pub maxRptDepth: ::core::ffi::c_double,
    pub volFlooded: ::core::ffi::c_double,
    pub timeFlooded: ::core::ffi::c_double,
    pub timeSurcharged: ::core::ffi::c_double,
    pub timeCourantCritical: ::core::ffi::c_double,
    pub totLatFlow: ::core::ffi::c_double,
    pub maxLatFlow: ::core::ffi::c_double,
    pub maxInflow: ::core::ffi::c_double,
    pub maxOverflow: ::core::ffi::c_double,
    pub maxPondedVol: ::core::ffi::c_double,
    pub nonConvergedCount: ::core::ffi::c_int,
    pub maxInflowDate: DateTime,
    pub maxOverflowDate: DateTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TStorageStats {
    pub initVol: ::core::ffi::c_double,
    pub avgVol: ::core::ffi::c_double,
    pub maxVol: ::core::ffi::c_double,
    pub maxFlow: ::core::ffi::c_double,
    pub evapLosses: ::core::ffi::c_double,
    pub exfilLosses: ::core::ffi::c_double,
    pub maxVolDate: DateTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TOutfallStats {
    pub avgFlow: ::core::ffi::c_double,
    pub maxFlow: ::core::ffi::c_double,
    pub totalLoad: *mut ::core::ffi::c_double,
    pub totalPeriods: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPumpStats {
    pub utilized: ::core::ffi::c_double,
    pub minFlow: ::core::ffi::c_double,
    pub avgFlow: ::core::ffi::c_double,
    pub maxFlow: ::core::ffi::c_double,
    pub volume: ::core::ffi::c_double,
    pub energy: ::core::ffi::c_double,
    pub offCurveLow: ::core::ffi::c_double,
    pub offCurveHigh: ::core::ffi::c_double,
    pub startUps: ::core::ffi::c_int,
    pub totalPeriods: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLinkStats {
    pub maxFlow: ::core::ffi::c_double,
    pub maxFlowDate: DateTime,
    pub maxVeloc: ::core::ffi::c_double,
    pub maxDepth: ::core::ffi::c_double,
    pub maxStreetFilled: ::core::ffi::c_double,
    pub timeNormalFlow: ::core::ffi::c_double,
    pub timeInletControl: ::core::ffi::c_double,
    pub timeSurcharged: ::core::ffi::c_double,
    pub timeFullUpstream: ::core::ffi::c_double,
    pub timeFullDnstream: ::core::ffi::c_double,
    pub timeFullFlow: ::core::ffi::c_double,
    pub timeCapacityLimited: ::core::ffi::c_double,
    pub timeInFlowClass: [::core::ffi::c_double; 7],
    pub timeCourantCritical: ::core::ffi::c_double,
    pub flowTurns: ::core::ffi::c_long,
    pub flowTurnSign: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TMaxStats {
    pub objType: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub value: ::core::ffi::c_double,
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
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MIN_RUNOFF_FLOW: ::core::ffi::c_double = 0.001f64;
pub const FUDGE: ::core::ffi::c_double = 0.0001f64;
pub const TIMELEVELS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MAX_STATS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut TimeStepStats: TTimeStepStats = TTimeStepStats {
    minTimeStep: 0.,
    maxTimeStep: 0.,
    routingTime: 0.,
    timeStepCount: 0,
    trialsCount: 0.,
    steadyStateTime: 0.,
    timeStepIntervals: [0.; 6],
    timeStepCounts: [0; 6],
};
static mut MaxMassBalErrs: [TMaxStats; 5] = [TMaxStats {
    objType: 0,
    index: 0,
    value: 0.,
}; 5];
static mut MaxCourantCrit: [TMaxStats; 5] = [TMaxStats {
    objType: 0,
    index: 0,
    value: 0.,
}; 5];
static mut MaxFlowTurns: [TMaxStats; 5] = [TMaxStats {
    objType: 0,
    index: 0,
    value: 0.,
}; 5];
static mut MaxNonConverged: [TMaxStats; 5] = [TMaxStats {
    objType: 0,
    index: 0,
    value: 0.,
}; 5];
static mut SysOutfallFlow: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut SubcatchStats: *mut TSubcatchStats =
    ::core::ptr::null::<TSubcatchStats>() as *mut TSubcatchStats;
#[no_mangle]
pub static mut NodeStats: *mut TNodeStats = ::core::ptr::null::<TNodeStats>() as *mut TNodeStats;
#[no_mangle]
pub static mut LinkStats: *mut TLinkStats = ::core::ptr::null::<TLinkStats>() as *mut TLinkStats;
#[no_mangle]
pub static mut StorageStats: *mut TStorageStats =
    ::core::ptr::null::<TStorageStats>() as *mut TStorageStats;
#[no_mangle]
pub static mut OutfallStats: *mut TOutfallStats =
    ::core::ptr::null::<TOutfallStats>() as *mut TOutfallStats;
#[no_mangle]
pub static mut PumpStats: *mut TPumpStats = ::core::ptr::null::<TPumpStats>() as *mut TPumpStats;
#[no_mangle]
pub static mut MaxOutfallFlow: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut MaxRunoffFlow: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut RoutingTimeSpan: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn stats_open() -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut timeStepDelta: ::core::ffi::c_double = 0.;
    let mut logMaxTimeStep: ::core::ffi::c_double = 0.;
    let mut logMinTimeStep: ::core::ffi::c_double = 0.;
    NodeStats = ::core::ptr::null_mut::<TNodeStats>();
    LinkStats = ::core::ptr::null_mut::<TLinkStats>();
    StorageStats = ::core::ptr::null_mut::<TStorageStats>();
    OutfallStats = ::core::ptr::null_mut::<TOutfallStats>();
    PumpStats = ::core::ptr::null_mut::<TPumpStats>();
    SubcatchStats = ::core::ptr::null_mut::<TSubcatchStats>();
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        SubcatchStats = calloc(
            Nobjects[SUBCATCH as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TSubcatchStats>() as size_t,
        ) as *mut TSubcatchStats;
        if SubcatchStats.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        }
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
            (*SubcatchStats.offset(j as isize)).precip = 0.0f64;
            (*SubcatchStats.offset(j as isize)).runon = 0.0f64;
            (*SubcatchStats.offset(j as isize)).evap = 0.0f64;
            (*SubcatchStats.offset(j as isize)).infil = 0.0f64;
            (*SubcatchStats.offset(j as isize)).runoff = 0.0f64;
            (*SubcatchStats.offset(j as isize)).maxFlow = 0.0f64;
            (*SubcatchStats.offset(j as isize)).impervRunoff = 0.0f64;
            (*SubcatchStats.offset(j as isize)).pervRunoff = 0.0f64;
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
            if !(*Subcatch.offset(j as isize)).groundwater.is_null() {
                (*(*Subcatch.offset(j as isize)).groundwater)
                    .stats
                    .avgUpperMoist = 0.0f64;
                (*(*Subcatch.offset(j as isize)).groundwater)
                    .stats
                    .avgWaterTable = 0.0f64;
                (*(*Subcatch.offset(j as isize)).groundwater).stats.infil = 0.0f64;
                (*(*Subcatch.offset(j as isize)).groundwater).stats.latFlow = 0.0f64;
                (*(*Subcatch.offset(j as isize)).groundwater).stats.deepFlow = 0.0f64;
                (*(*Subcatch.offset(j as isize)).groundwater).stats.evap = 0.0f64;
                (*(*Subcatch.offset(j as isize)).groundwater).stats.maxFlow = 0.0f64;
                (*(*Subcatch.offset(j as isize)).groundwater)
                    .stats
                    .finalUpperMoist = 0.0f64;
                (*(*Subcatch.offset(j as isize)).groundwater)
                    .stats
                    .finalWaterTable = 0.0f64;
            }
            j += 1;
        }
    }
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        NodeStats = calloc(
            Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TNodeStats>() as size_t,
        ) as *mut TNodeStats;
        LinkStats = calloc(
            Nobjects[LINK as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TLinkStats>() as size_t,
        ) as *mut TLinkStats;
        if NodeStats.is_null() || LinkStats.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        }
    }
    if !NodeStats.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            (*NodeStats.offset(j as isize)).avgDepth = 0.0f64;
            (*NodeStats.offset(j as isize)).maxDepth = 0.0f64;
            (*NodeStats.offset(j as isize)).maxDepthDate = StartDateTime;
            (*NodeStats.offset(j as isize)).maxRptDepth = 0.0f64;
            (*NodeStats.offset(j as isize)).volFlooded = 0.0f64;
            (*NodeStats.offset(j as isize)).timeFlooded = 0.0f64;
            (*NodeStats.offset(j as isize)).timeSurcharged = 0.0f64;
            (*NodeStats.offset(j as isize)).timeCourantCritical = 0.0f64;
            (*NodeStats.offset(j as isize)).totLatFlow = 0.0f64;
            (*NodeStats.offset(j as isize)).maxLatFlow = 0.0f64;
            (*NodeStats.offset(j as isize)).maxInflow = 0.0f64;
            (*NodeStats.offset(j as isize)).maxOverflow = 0.0f64;
            (*NodeStats.offset(j as isize)).maxPondedVol = 0.0f64;
            (*NodeStats.offset(j as isize)).nonConvergedCount = 0 as ::core::ffi::c_int;
            (*NodeStats.offset(j as isize)).maxInflowDate = StartDateTime;
            (*NodeStats.offset(j as isize)).maxOverflowDate = StartDateTime;
            j += 1;
        }
    }
    if !LinkStats.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
            (*LinkStats.offset(j as isize)).maxFlow = 0.0f64;
            (*LinkStats.offset(j as isize)).maxVeloc = 0.0f64;
            (*LinkStats.offset(j as isize)).maxDepth = 0.0f64;
            (*LinkStats.offset(j as isize)).maxStreetFilled = 0.0f64;
            (*LinkStats.offset(j as isize)).timeSurcharged = 0.0f64;
            (*LinkStats.offset(j as isize)).timeFullUpstream = 0.0f64;
            (*LinkStats.offset(j as isize)).timeFullDnstream = 0.0f64;
            (*LinkStats.offset(j as isize)).timeFullFlow = 0.0f64;
            (*LinkStats.offset(j as isize)).timeCapacityLimited = 0.0f64;
            (*LinkStats.offset(j as isize)).timeCourantCritical = 0.0f64;
            k = 0 as ::core::ffi::c_int;
            while k < MAX_FLOW_CLASSES as ::core::ffi::c_int {
                (*LinkStats.offset(j as isize)).timeInFlowClass[k as usize] = 0.0f64;
                k += 1;
            }
            (*LinkStats.offset(j as isize)).flowTurns = 0 as ::core::ffi::c_long;
            (*LinkStats.offset(j as isize)).flowTurnSign = 0 as ::core::ffi::c_int;
            j += 1;
        }
    }
    if Nnodes[STORAGE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        StorageStats = calloc(
            Nnodes[STORAGE as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TStorageStats>() as size_t,
        ) as *mut TStorageStats;
        if StorageStats.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        } else {
            k = 0 as ::core::ffi::c_int;
            while k < Nobjects[NODE as ::core::ffi::c_int as usize] {
                if !((*Node.offset(k as isize)).type_0 != STORAGE as ::core::ffi::c_int) {
                    j = (*Node.offset(k as isize)).subIndex;
                    (*StorageStats.offset(j as isize)).initVol =
                        (*Node.offset(k as isize)).newVolume;
                    (*StorageStats.offset(j as isize)).avgVol = 0.0f64;
                    (*StorageStats.offset(j as isize)).maxVol = 0.0f64;
                    (*StorageStats.offset(j as isize)).maxFlow = 0.0f64;
                    (*StorageStats.offset(j as isize)).evapLosses = 0.0f64;
                    (*StorageStats.offset(j as isize)).exfilLosses = 0.0f64;
                    (*StorageStats.offset(j as isize)).maxVolDate = StartDateTime;
                }
                k += 1;
            }
        }
    }
    if Nnodes[OUTFALL as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        OutfallStats = calloc(
            Nnodes[OUTFALL as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TOutfallStats>() as size_t,
        ) as *mut TOutfallStats;
        if OutfallStats.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        } else {
            j = 0 as ::core::ffi::c_int;
            while j < Nnodes[OUTFALL as ::core::ffi::c_int as usize] {
                (*OutfallStats.offset(j as isize)).avgFlow = 0.0f64;
                (*OutfallStats.offset(j as isize)).maxFlow = 0.0f64;
                (*OutfallStats.offset(j as isize)).totalPeriods = 0 as ::core::ffi::c_int;
                if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
                    let ref mut fresh0 = (*OutfallStats.offset(j as isize)).totalLoad;
                    *fresh0 = calloc(
                        Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
                        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                    ) as *mut ::core::ffi::c_double;
                    if (*OutfallStats.offset(j as isize)).totalLoad.is_null() {
                        report_writeErrorMsg(
                            ERR_MEMORY as ::core::ffi::c_int,
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        return ErrorCode;
                    }
                    k = 0 as ::core::ffi::c_int;
                    while k < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                        *(*OutfallStats.offset(j as isize))
                            .totalLoad
                            .offset(k as isize) = 0.0f64;
                        k += 1;
                    }
                } else {
                    let ref mut fresh1 = (*OutfallStats.offset(j as isize)).totalLoad;
                    *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_double>();
                }
                j += 1;
            }
        }
    }
    if Nlinks[PUMP as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        PumpStats = calloc(
            Nlinks[PUMP as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TPumpStats>() as size_t,
        ) as *mut TPumpStats;
        if PumpStats.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return ErrorCode;
        } else {
            j = 0 as ::core::ffi::c_int;
            while j < Nlinks[PUMP as ::core::ffi::c_int as usize] {
                (*PumpStats.offset(j as isize)).utilized = 0.0f64;
                (*PumpStats.offset(j as isize)).minFlow = 0.0f64;
                (*PumpStats.offset(j as isize)).avgFlow = 0.0f64;
                (*PumpStats.offset(j as isize)).maxFlow = 0.0f64;
                (*PumpStats.offset(j as isize)).volume = 0.0f64;
                (*PumpStats.offset(j as isize)).energy = 0.0f64;
                (*PumpStats.offset(j as isize)).startUps = 0 as ::core::ffi::c_int;
                (*PumpStats.offset(j as isize)).offCurveLow = 0.0f64;
                (*PumpStats.offset(j as isize)).offCurveHigh = 0.0f64;
                j += 1;
            }
        }
    }
    MaxRunoffFlow = 0.0f64;
    MaxOutfallFlow = 0.0f64;
    TimeStepStats.maxTimeStep = 0.0f64;
    TimeStepStats.minTimeStep = RouteStep;
    TimeStepStats.routingTime = 0.0f64;
    TimeStepStats.trialsCount = 0.0f64;
    TimeStepStats.steadyStateTime = 0.0f64;
    TimeStepStats.timeStepCount = 0 as ::core::ffi::c_int;
    logMaxTimeStep = log10(RouteStep);
    logMinTimeStep = log10(MinRouteStep);
    timeStepDelta = (logMaxTimeStep - logMinTimeStep)
        / (TIMELEVELS - 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    TimeStepStats.timeStepIntervals[0 as ::core::ffi::c_int as usize] = RouteStep;
    j = 1 as ::core::ffi::c_int;
    while j < TIMELEVELS {
        TimeStepStats.timeStepIntervals[j as usize] = pow(
            10.0f64,
            logMaxTimeStep - j as ::core::ffi::c_double * timeStepDelta,
        );
        TimeStepStats.timeStepCounts[j as usize] = 0 as ::core::ffi::c_int;
        j += 1;
    }
    TimeStepStats.timeStepIntervals[(TIMELEVELS - 1 as ::core::ffi::c_int) as usize] = MinRouteStep;
    RoutingTimeSpan = 0.0f64;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stats_close() {
    let mut j: ::core::ffi::c_int = 0;
    if !SubcatchStats.is_null() {
        free(SubcatchStats as *mut ::core::ffi::c_void);
        SubcatchStats = ::core::ptr::null_mut::<TSubcatchStats>();
    }
    if !NodeStats.is_null() {
        free(NodeStats as *mut ::core::ffi::c_void);
        NodeStats = ::core::ptr::null_mut::<TNodeStats>();
    }
    if !LinkStats.is_null() {
        free(LinkStats as *mut ::core::ffi::c_void);
        LinkStats = ::core::ptr::null_mut::<TLinkStats>();
    }
    if !StorageStats.is_null() {
        free(StorageStats as *mut ::core::ffi::c_void);
        StorageStats = ::core::ptr::null_mut::<TStorageStats>();
    }
    if !OutfallStats.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nnodes[OUTFALL as ::core::ffi::c_int as usize] {
            if !(*OutfallStats.offset(j as isize)).totalLoad.is_null() {
                free((*OutfallStats.offset(j as isize)).totalLoad as *mut ::core::ffi::c_void);
                let ref mut fresh2 = (*OutfallStats.offset(j as isize)).totalLoad;
                *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            j += 1;
        }
        if !OutfallStats.is_null() {
            free(OutfallStats as *mut ::core::ffi::c_void);
            OutfallStats = ::core::ptr::null_mut::<TOutfallStats>();
        }
    }
    if !PumpStats.is_null() {
        free(PumpStats as *mut ::core::ffi::c_void);
        PumpStats = ::core::ptr::null_mut::<TPumpStats>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_report() {
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
        && RouteModel != NO_ROUTING as ::core::ffi::c_int
    {
        stats_findMaxStats();
        if RptFlags.disabled == 0 && RptFlags.flowStats as ::core::ffi::c_int != 0 {
            report_writeMaxStats(
                &raw mut MaxMassBalErrs as *mut TMaxStats,
                &raw mut MaxCourantCrit as *mut TMaxStats,
                MAX_STATS,
            );
            report_writeMaxFlowTurns(&raw mut MaxFlowTurns as *mut TMaxStats, MAX_STATS);
            report_writeNonconvergedStats(&raw mut MaxNonConverged as *mut TMaxStats, MAX_STATS);
            report_writeTimeStepStats(&raw mut TimeStepStats);
        }
    }
    if RptFlags.disabled == 0 {
        statsrpt_writeReport();
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_updateSubcatchStats(
    mut j: ::core::ffi::c_int,
    mut rainVol: ::core::ffi::c_double,
    mut runonVol: ::core::ffi::c_double,
    mut evapVol: ::core::ffi::c_double,
    mut infilVol: ::core::ffi::c_double,
    mut impervVol: ::core::ffi::c_double,
    mut pervVol: ::core::ffi::c_double,
    mut runoffVol: ::core::ffi::c_double,
    mut runoff: ::core::ffi::c_double,
) {
    (*SubcatchStats.offset(j as isize)).precip += rainVol;
    (*SubcatchStats.offset(j as isize)).runon += runonVol;
    (*SubcatchStats.offset(j as isize)).evap += evapVol;
    (*SubcatchStats.offset(j as isize)).infil += infilVol;
    (*SubcatchStats.offset(j as isize)).runoff += runoffVol;
    (*SubcatchStats.offset(j as isize)).maxFlow =
        if (*SubcatchStats.offset(j as isize)).maxFlow >= runoff {
            (*SubcatchStats.offset(j as isize)).maxFlow
        } else {
            runoff
        };
    (*SubcatchStats.offset(j as isize)).impervRunoff += impervVol;
    (*SubcatchStats.offset(j as isize)).pervRunoff += pervVol;
}
#[no_mangle]
pub unsafe extern "C" fn stats_updateGwaterStats(
    mut j: ::core::ffi::c_int,
    mut infil: ::core::ffi::c_double,
    mut evap: ::core::ffi::c_double,
    mut latFlow: ::core::ffi::c_double,
    mut deepFlow: ::core::ffi::c_double,
    mut theta: ::core::ffi::c_double,
    mut waterTable: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    (*(*Subcatch.offset(j as isize)).groundwater).stats.infil += infil * tStep;
    (*(*Subcatch.offset(j as isize)).groundwater).stats.evap += evap * tStep;
    (*(*Subcatch.offset(j as isize)).groundwater).stats.latFlow += latFlow * tStep;
    (*(*Subcatch.offset(j as isize)).groundwater).stats.deepFlow += deepFlow * tStep;
    (*(*Subcatch.offset(j as isize)).groundwater)
        .stats
        .avgUpperMoist += theta * tStep;
    (*(*Subcatch.offset(j as isize)).groundwater)
        .stats
        .avgWaterTable += waterTable * tStep;
    (*(*Subcatch.offset(j as isize)).groundwater)
        .stats
        .finalUpperMoist = theta;
    (*(*Subcatch.offset(j as isize)).groundwater)
        .stats
        .finalWaterTable = waterTable;
    if fabs(latFlow) > fabs((*(*Subcatch.offset(j as isize)).groundwater).stats.maxFlow) {
        (*(*Subcatch.offset(j as isize)).groundwater).stats.maxFlow = latFlow;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_updateMaxRunoff() {
    let mut j: ::core::ffi::c_int = 0;
    let mut sysRunoff: ::core::ffi::c_double = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        sysRunoff += (*Subcatch.offset(j as isize)).newRunoff;
        j += 1;
    }
    MaxRunoffFlow = if MaxRunoffFlow >= sysRunoff {
        MaxRunoffFlow
    } else {
        sysRunoff
    };
}
#[no_mangle]
pub unsafe extern "C" fn stats_updateMaxNodeDepth(
    mut j: ::core::ffi::c_int,
    mut depth: ::core::ffi::c_double,
) {
    if !NodeStats.is_null() {
        (*NodeStats.offset(j as isize)).maxRptDepth =
            if (*NodeStats.offset(j as isize)).maxRptDepth >= depth {
                (*NodeStats.offset(j as isize)).maxRptDepth
            } else {
                depth
            };
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_updateFlowStats(
    mut tStep: ::core::ffi::c_double,
    mut aDate: DateTime,
) {
    let mut j: ::core::ffi::c_int = 0;
    if aDate < ReportStart {
        return;
    }
    SysOutfallFlow = 0.0f64;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        stats_updateNodeStats(j, tStep, aDate);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        stats_updateLinkStats(j, tStep, aDate);
        j += 1;
    }
    ReportStepCount += 1;
    RoutingTimeSpan += tStep;
    MaxOutfallFlow = if MaxOutfallFlow >= SysOutfallFlow {
        MaxOutfallFlow
    } else {
        SysOutfallFlow
    };
}
#[no_mangle]
pub unsafe extern "C" fn stats_updateTimeStepStats(
    mut tStep: ::core::ffi::c_double,
    mut trialsCount: ::core::ffi::c_int,
    mut steadyState: ::core::ffi::c_int,
) {
    let mut j: ::core::ffi::c_int = 0;
    TimeStepStats.steadyStateTime += steadyState as ::core::ffi::c_double * tStep;
    if steadyState == FALSE {
        if OldRoutingTime > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            TimeStepStats.minTimeStep = if TimeStepStats.minTimeStep <= tStep {
                TimeStepStats.minTimeStep
            } else {
                tStep
            };
            j = 1 as ::core::ffi::c_int;
            while j < TIMELEVELS {
                if tStep >= TimeStepStats.timeStepIntervals[j as usize] {
                    TimeStepStats.timeStepCounts[j as usize] += 1;
                    break;
                } else {
                    j += 1;
                }
            }
        }
        TimeStepStats.maxTimeStep = if TimeStepStats.maxTimeStep >= tStep {
            TimeStepStats.maxTimeStep
        } else {
            tStep
        };
        TimeStepStats.routingTime += tStep;
        TimeStepStats.timeStepCount += 1;
        TimeStepStats.trialsCount += trialsCount as ::core::ffi::c_double;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_updateCriticalTimeCount(
    mut node: ::core::ffi::c_int,
    mut link: ::core::ffi::c_int,
) {
    if node >= 0 as ::core::ffi::c_int {
        (*NodeStats.offset(node as isize)).timeCourantCritical += 1.0f64;
    } else if link >= 0 as ::core::ffi::c_int {
        (*LinkStats.offset(link as isize)).timeCourantCritical += 1.0f64;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_updateConvergenceStats(
    mut node: ::core::ffi::c_int,
    mut converged: ::core::ffi::c_int,
) {
    if converged == FALSE {
        let ref mut fresh7 = (*NodeStats.offset(node as isize)).nonConvergedCount;
        *fresh7 += 1;
    }
}
unsafe extern "C" fn stats_updateNodeStats(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
    mut aDate: DateTime,
) {
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut newVolume: ::core::ffi::c_double = (*Node.offset(j as isize)).newVolume;
    let mut newDepth: ::core::ffi::c_double = (*Node.offset(j as isize)).newDepth;
    let mut canPond: ::core::ffi::c_int =
        (AllowPonding != 0 && (*Node.offset(j as isize)).pondedArea > 0.0f64) as ::core::ffi::c_int;
    (*NodeStats.offset(j as isize)).avgDepth += newDepth;
    if newDepth > (*NodeStats.offset(j as isize)).maxDepth {
        (*NodeStats.offset(j as isize)).maxDepth = newDepth;
        (*NodeStats.offset(j as isize)).maxDepthDate = aDate;
    }
    if (*Node.offset(j as isize)).type_0 != OUTFALL as ::core::ffi::c_int {
        if newVolume > (*Node.offset(j as isize)).fullVolume
            || (*Node.offset(j as isize)).overflow > 0.0f64
        {
            (*NodeStats.offset(j as isize)).timeFlooded += tStep;
            (*NodeStats.offset(j as isize)).volFlooded +=
                (*Node.offset(j as isize)).overflow * tStep;
            if canPond != 0 {
                (*NodeStats.offset(j as isize)).maxPondedVol = if (*NodeStats.offset(j as isize))
                    .maxPondedVol
                    >= newVolume - (*Node.offset(j as isize)).fullVolume
                {
                    (*NodeStats.offset(j as isize)).maxPondedVol
                } else {
                    newVolume - (*Node.offset(j as isize)).fullVolume
                };
            }
        }
        if RouteModel == DW as ::core::ffi::c_int {
            if ((*Node.offset(j as isize)).type_0 != STORAGE as ::core::ffi::c_int
                || (*Node.offset(j as isize)).surDepth > 0.0f64)
                && newDepth + (*Node.offset(j as isize)).invertElev + FUDGE
                    >= (*Node.offset(j as isize)).crownElev
            {
                (*NodeStats.offset(j as isize)).timeSurcharged += tStep;
            }
        }
    }
    if (*Node.offset(j as isize)).type_0 == STORAGE as ::core::ffi::c_int {
        k = (*Node.offset(j as isize)).subIndex;
        (*StorageStats.offset(k as isize)).avgVol += newVolume;
        (*StorageStats.offset(k as isize)).evapLosses +=
            (*Storage.offset((*Node.offset(j as isize)).subIndex as isize)).evapLoss;
        (*StorageStats.offset(k as isize)).exfilLosses +=
            (*Storage.offset((*Node.offset(j as isize)).subIndex as isize)).exfilLoss;
        newVolume = if newVolume <= (*Node.offset(j as isize)).fullVolume {
            newVolume
        } else {
            (*Node.offset(j as isize)).fullVolume
        };
        if newVolume > (*StorageStats.offset(k as isize)).maxVol {
            (*StorageStats.offset(k as isize)).maxVol = newVolume;
            (*StorageStats.offset(k as isize)).maxVolDate = aDate;
        }
        (*StorageStats.offset(k as isize)).maxFlow =
            if (*StorageStats.offset(k as isize)).maxFlow >= (*Node.offset(j as isize)).outflow {
                (*StorageStats.offset(k as isize)).maxFlow
            } else {
                (*Node.offset(j as isize)).outflow
            };
    }
    if (*Node.offset(j as isize)).type_0 == OUTFALL as ::core::ffi::c_int {
        k = (*Node.offset(j as isize)).subIndex;
        if (*Node.offset(j as isize)).inflow >= MIN_RUNOFF_FLOW {
            (*OutfallStats.offset(k as isize)).avgFlow += (*Node.offset(j as isize)).inflow;
            (*OutfallStats.offset(k as isize)).maxFlow = if (*OutfallStats.offset(k as isize))
                .maxFlow
                >= (*Node.offset(j as isize)).inflow
            {
                (*OutfallStats.offset(k as isize)).maxFlow
            } else {
                (*Node.offset(j as isize)).inflow
            };
            let ref mut fresh6 = (*OutfallStats.offset(k as isize)).totalPeriods;
            *fresh6 += 1;
        }
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *(*OutfallStats.offset(k as isize))
                .totalLoad
                .offset(p as isize) += (*Node.offset(j as isize)).inflow
                * *(*Node.offset(j as isize)).newQual.offset(p as isize)
                * tStep;
            p += 1;
        }
        SysOutfallFlow += (*Node.offset(j as isize)).inflow;
    }
    (*NodeStats.offset(j as isize)).totLatFlow += ((*Node.offset(j as isize)).oldLatFlow
        + (*Node.offset(j as isize)).newLatFlow)
        * 0.5f64
        * tStep;
    if fabs((*Node.offset(j as isize)).newLatFlow)
        > fabs((*NodeStats.offset(j as isize)).maxLatFlow)
    {
        (*NodeStats.offset(j as isize)).maxLatFlow = (*Node.offset(j as isize)).newLatFlow;
    }
    if (*Node.offset(j as isize)).inflow > (*NodeStats.offset(j as isize)).maxInflow {
        (*NodeStats.offset(j as isize)).maxInflow = (*Node.offset(j as isize)).inflow;
        (*NodeStats.offset(j as isize)).maxInflowDate = aDate;
    }
    if (*Node.offset(j as isize)).overflow > (*NodeStats.offset(j as isize)).maxOverflow {
        (*NodeStats.offset(j as isize)).maxOverflow = (*Node.offset(j as isize)).overflow;
        (*NodeStats.offset(j as isize)).maxOverflowDate = aDate;
    }
}
unsafe extern "C" fn stats_updateLinkStats(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
    mut aDate: DateTime,
) {
    let mut k: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    let mut dq: ::core::ffi::c_double = 0.;
    dq = (*Link.offset(j as isize)).newFlow - (*Link.offset(j as isize)).oldFlow;
    q = fabs((*Link.offset(j as isize)).newFlow);
    if q > (*LinkStats.offset(j as isize)).maxFlow {
        (*LinkStats.offset(j as isize)).maxFlow = q;
        (*LinkStats.offset(j as isize)).maxFlowDate = aDate;
    }
    v = link_getVelocity(j, q, (*Link.offset(j as isize)).newDepth);
    if v > (*LinkStats.offset(j as isize)).maxVeloc {
        (*LinkStats.offset(j as isize)).maxVeloc = v;
    }
    if (*Link.offset(j as isize)).newDepth > (*LinkStats.offset(j as isize)).maxDepth {
        (*LinkStats.offset(j as isize)).maxDepth = (*Link.offset(j as isize)).newDepth;
    }
    if (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int {
        if q >= (*Link.offset(j as isize)).qFull {
            (*LinkStats.offset(j as isize)).timeFullFlow += tStep;
        }
        if q > MIN_RUNOFF_FLOW {
            k = (*Link.offset(j as isize)).subIndex;
            (*PumpStats.offset(k as isize)).minFlow =
                if (*PumpStats.offset(k as isize)).minFlow <= q {
                    (*PumpStats.offset(k as isize)).minFlow
                } else {
                    q
                };
            (*PumpStats.offset(k as isize)).maxFlow = (*LinkStats.offset(j as isize)).maxFlow;
            (*PumpStats.offset(k as isize)).avgFlow += q;
            (*PumpStats.offset(k as isize)).volume += q * tStep;
            (*PumpStats.offset(k as isize)).utilized += tStep;
            (*PumpStats.offset(k as isize)).energy += link_getPower(j) * tStep / 3600.0f64;
            if (*Link.offset(j as isize)).flowClass == DN_DRY as ::core::ffi::c_int {
                (*PumpStats.offset(k as isize)).offCurveLow += tStep;
            }
            if (*Link.offset(j as isize)).flowClass == UP_DRY as ::core::ffi::c_int {
                (*PumpStats.offset(k as isize)).offCurveHigh += tStep;
            }
            if (*Link.offset(j as isize)).oldFlow < MIN_RUNOFF_FLOW {
                let ref mut fresh3 = (*PumpStats.offset(k as isize)).startUps;
                *fresh3 += 1;
            }
            let ref mut fresh4 = (*PumpStats.offset(k as isize)).totalPeriods;
            *fresh4 += 1;
            (*LinkStats.offset(j as isize)).timeSurcharged += tStep;
            (*LinkStats.offset(j as isize)).timeFullUpstream += tStep;
            (*LinkStats.offset(j as isize)).timeFullDnstream += tStep;
        }
    } else if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        if (*Link.offset(j as isize)).normalFlow != 0 {
            (*LinkStats.offset(j as isize)).timeNormalFlow += tStep;
        }
        if (*Link.offset(j as isize)).inletControl != 0 {
            (*LinkStats.offset(j as isize)).timeInletControl += tStep;
        }
        k = (*Link.offset(j as isize)).flowClass;
        if k >= 0 as ::core::ffi::c_int && k < MAX_FLOW_CLASSES as ::core::ffi::c_int {
            (*LinkStats.offset(j as isize)).timeInFlowClass[k as usize] += tStep;
        }
        k = (*Link.offset(j as isize)).subIndex;
        if q >= (*Link.offset(j as isize)).qFull
            * (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_double
        {
            (*LinkStats.offset(j as isize)).timeFullFlow += tStep;
        }
        if (*Conduit.offset(k as isize)).capacityLimited != 0 {
            (*LinkStats.offset(j as isize)).timeCapacityLimited += tStep;
        }
        match (*Conduit.offset(k as isize)).fullState as ::core::ffi::c_int {
            10 => {
                (*LinkStats.offset(j as isize)).timeSurcharged += tStep;
                (*LinkStats.offset(j as isize)).timeFullUpstream += tStep;
                (*LinkStats.offset(j as isize)).timeFullDnstream += tStep;
            }
            8 => {
                (*LinkStats.offset(j as isize)).timeFullUpstream += tStep;
            }
            9 => {
                (*LinkStats.offset(j as isize)).timeFullDnstream += tStep;
            }
            _ => {}
        }
        if (*Link.offset(j as isize)).xsect.type_0 == STREET_XSECT as ::core::ffi::c_int {
            (*LinkStats.offset(j as isize)).maxStreetFilled =
                if (*LinkStats.offset(j as isize)).maxStreetFilled >= street_getExtentFilled(j) {
                    (*LinkStats.offset(j as isize)).maxStreetFilled
                } else {
                    street_getExtentFilled(j)
                };
        }
    }
    k = (*LinkStats.offset(j as isize)).flowTurnSign;
    (*LinkStats.offset(j as isize)).flowTurnSign =
        if dq < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        };
    if fabs(dq) > 0.001f64
        && k * (*LinkStats.offset(j as isize)).flowTurnSign < 0 as ::core::ffi::c_int
    {
        let ref mut fresh5 = (*LinkStats.offset(j as isize)).flowTurns;
        *fresh5 += 1;
    }
}
unsafe extern "C" fn stats_findMaxStats() {
    let mut j: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    let mut z: ::core::ffi::c_double = 0.;
    let mut stepCount: ::core::ffi::c_double = 0.;
    j = 0 as ::core::ffi::c_int;
    while j < MAX_STATS {
        MaxMassBalErrs[j as usize].objType = NODE as ::core::ffi::c_int;
        MaxMassBalErrs[j as usize].index = -(1 as ::core::ffi::c_int);
        MaxMassBalErrs[j as usize].value = -1.0f64;
        MaxCourantCrit[j as usize].index = -(1 as ::core::ffi::c_int);
        MaxCourantCrit[j as usize].value = -1.0f64;
        MaxFlowTurns[j as usize].index = -(1 as ::core::ffi::c_int);
        MaxFlowTurns[j as usize].value = -1.0f64;
        MaxNonConverged[j as usize].index = -(1 as ::core::ffi::c_int);
        MaxNonConverged[j as usize].value = 0.0f64;
        j += 1;
    }
    if ReportStepCount > 2 as ::core::ffi::c_long {
        stepCount = ReportStepCount as ::core::ffi::c_double;
        z = 100.0f64 / (2.0f64 / 3.0f64 * (stepCount - 2.0f64));
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
            x = (*LinkStats.offset(j as isize)).flowTurns as ::core::ffi::c_double * z;
            stats_updateMaxStats(
                &raw mut MaxFlowTurns as *mut TMaxStats,
                LINK as ::core::ffi::c_int,
                j,
                x,
            );
            j += 1;
        }
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(j as isize)).degree <= 0 as ::core::ffi::c_int) {
            if !(*NodeInflow.offset(j as isize) <= 0.1f64) {
                if *NodeInflow.offset(j as isize) > 0.0f64 {
                    x = 1.0f64 - *NodeOutflow.offset(j as isize) / *NodeInflow.offset(j as isize);
                } else if *NodeOutflow.offset(j as isize) > 0.0f64 {
                    x = -1.0f64;
                } else {
                    x = 0.0f64;
                }
                stats_updateMaxStats(
                    &raw mut MaxMassBalErrs as *mut TMaxStats,
                    NODE as ::core::ffi::c_int,
                    j,
                    100.0f64 * x,
                );
            }
        }
        j += 1;
    }
    stepCount = TimeStepStats.timeStepCount as ::core::ffi::c_double;
    if RouteModel == DW as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            stats_updateMaxStats(
                &raw mut MaxNonConverged as *mut TMaxStats,
                NODE as ::core::ffi::c_int,
                j,
                (*NodeStats.offset(j as isize)).nonConvergedCount as ::core::ffi::c_double
                    / stepCount,
            );
            j += 1;
        }
    }
    if RouteModel != DW as ::core::ffi::c_int || CourantFactor == 0.0f64 {
        return;
    }
    if stepCount == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        x = (*NodeStats.offset(j as isize)).timeCourantCritical / stepCount;
        stats_updateMaxStats(
            &raw mut MaxCourantCrit as *mut TMaxStats,
            NODE as ::core::ffi::c_int,
            j,
            100.0f64 * x,
        );
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        x = (*LinkStats.offset(j as isize)).timeCourantCritical / stepCount;
        stats_updateMaxStats(
            &raw mut MaxCourantCrit as *mut TMaxStats,
            LINK as ::core::ffi::c_int,
            j,
            100.0f64 * x,
        );
        j += 1;
    }
}
unsafe extern "C" fn stats_updateMaxStats(
    mut maxStats: *mut TMaxStats,
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut x: ::core::ffi::c_double,
) {
    let mut k: ::core::ffi::c_int = 0;
    let mut maxStats1: TMaxStats = TMaxStats {
        objType: 0,
        index: 0,
        value: 0.,
    };
    let mut maxStats2: TMaxStats = TMaxStats {
        objType: 0,
        index: 0,
        value: 0.,
    };
    maxStats1.objType = i;
    maxStats1.index = j;
    maxStats1.value = x;
    k = 0 as ::core::ffi::c_int;
    while k < MAX_STATS {
        if fabs(maxStats1.value) > fabs((*maxStats.offset(k as isize)).value) {
            maxStats2 = *maxStats.offset(k as isize);
            *maxStats.offset(k as isize) = maxStats1;
            maxStats1 = maxStats2;
        }
        k += 1;
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
