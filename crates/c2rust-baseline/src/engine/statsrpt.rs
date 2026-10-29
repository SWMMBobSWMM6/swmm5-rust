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
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn inlet_writeStatsReport();
    static mut Frpt: TFile;
    static mut ReportStepCount: ::core::ffi::c_long;
    static mut Nnodes: [::core::ffi::c_int; 5];
    static mut Nlinks: [::core::ffi::c_int; 5];
    static mut UnitSystem: ::core::ffi::c_int;
    static mut FlowUnits: ::core::ffi::c_int;
    static mut RouteModel: ::core::ffi::c_int;
    static mut IgnoreRainfall: ::core::ffi::c_int;
    static mut IgnoreSnowmelt: ::core::ffi::c_int;
    static mut IgnoreGwater: ::core::ffi::c_int;
    static mut IgnoreRouting: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut NewRunoffTime: ::core::ffi::c_double;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Orifice: *mut TOrifice;
    static mut Pollut: *mut TPollut;
    fn report_writeLine(line: *const ::core::ffi::c_char);
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn sstrncpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        n: size_t,
    ) -> size_t;
    fn getElapsedTime(
        aDate: DateTime,
        days: *mut ::core::ffi::c_int,
        hrs: *mut ::core::ffi::c_int,
        mins: *mut ::core::ffi::c_int,
    );
    static mut SubcatchStats: *mut TSubcatchStats;
    static mut NodeStats: *mut TNodeStats;
    static mut LinkStats: *mut TLinkStats;
    static mut StorageStats: *mut TStorageStats;
    static mut OutfallStats: *mut TOutfallStats;
    static mut PumpStats: *mut TPumpStats;
    static mut MaxOutfallFlow: ::core::ffi::c_double;
    static mut RoutingTimeSpan: ::core::ffi::c_double;
    static mut NodeInflow: *mut ::core::ffi::c_double;
    static mut NodeOutflow: *mut ::core::ffi::c_double;
    static mut FlowUnitWords: [*mut ::core::ffi::c_char; 0];
    static mut LinkTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut LoadUnitsWords: [*mut ::core::ffi::c_char; 0];
    static mut NodeTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut PondingUnitsWords: [*mut ::core::ffi::c_char; 0];
    static mut VolUnitsWords: [*mut ::core::ffi::c_char; 0];
    static mut VolUnitsWords2: [*mut ::core::ffi::c_char; 0];
    fn lid_writeWaterBalance();
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
pub type UnitsType = ::core::ffi::c_uint;
pub const SI: UnitsType = 1;
pub const US: UnitsType = 0;
pub type FlowUnitsType = ::core::ffi::c_uint;
pub const MLD: FlowUnitsType = 5;
pub const LPS: FlowUnitsType = 4;
pub const CMS: FlowUnitsType = 3;
pub const MGD: FlowUnitsType = 2;
pub const GPM: FlowUnitsType = 1;
pub const CFS: FlowUnitsType = 0;
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
pub type OrificeType = ::core::ffi::c_uint;
pub const BOTTOM_ORIFICE: OrificeType = 1;
pub const SIDE_ORIFICE: OrificeType = 0;
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
pub struct TOrifice {
    pub type_0: ::core::ffi::c_int,
    pub shape: ::core::ffi::c_int,
    pub cDisch: ::core::ffi::c_double,
    pub orate: ::core::ffi::c_double,
    pub cOrif: ::core::ffi::c_double,
    pub hCrit: ::core::ffi::c_double,
    pub cWeir: ::core::ffi::c_double,
    pub length: ::core::ffi::c_double,
    pub surfArea: ::core::ffi::c_double,
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
pub const LperFT3: ::core::ffi::c_double = 28.317f64;
static mut FlowFmt: [::core::ffi::c_char; 6] = [0; 6];
static mut Vcf: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn statsrpt_writeReport() {
    if FlowUnits == MGD as ::core::ffi::c_int || FlowUnits == CMS as ::core::ffi::c_int {
        sstrncpy(
            &raw mut FlowFmt as *mut ::core::ffi::c_char,
            b"%9.3f\0" as *const u8 as *const ::core::ffi::c_char,
            5 as size_t,
        );
    } else {
        sstrncpy(
            &raw mut FlowFmt as *mut ::core::ffi::c_char,
            b"%9.2f\0" as *const u8 as *const ::core::ffi::c_char,
            5 as size_t,
        );
    }
    if UnitSystem == US as ::core::ffi::c_int {
        Vcf = 7.48f64 / 1.0e6f64;
    } else {
        Vcf = 28.317f64 / 1.0e6f64;
    }
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        if IgnoreRainfall == 0
            || Nobjects[SNOWMELT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
                && IgnoreSnowmelt == 0
            || Nobjects[AQUIFER as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
                && IgnoreGwater == 0
        {
            writeSubcatchRunoff();
            lid_writeWaterBalance();
            if IgnoreGwater == 0 {
                writeGroundwater();
            }
            if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
                && IgnoreQuality == 0
            {
                writeSubcatchLoads();
            }
        }
    }
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int && IgnoreRouting == 0
    {
        writeNodeDepths();
        writeNodeFlows();
        if RouteModel == DW as ::core::ffi::c_int {
            writeNodeSurcharge();
        }
        writeNodeFlooding();
        writeStorageVolumes();
        writeOutfallLoads();
        inlet_writeStatsReport();
        writeLinkFlows();
        writeFlowClass();
        writeLinkSurcharge();
        writePumpFlows();
        if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
            && IgnoreQuality == 0
        {
            writeLinkLoads();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn writeSubcatchRunoff() {
    let mut j: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_double = 0.;
    let mut x: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Subcatchment Runoff Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  ------------------------------------------------------------------------------------------------------------------------------\n                            Total      Total      Total      Total     Imperv       Perv      Total       Total     Peak  Runoff\n                           Precip      Runon       Evap      Infil     Runoff     Runoff     Runoff      Runoff   Runoff   Coeff\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Subcatchment                 in         in         in         in         in         in         in    %8s      %3s\0"
                as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut VolUnitsWords as *mut *mut ::core::ffi::c_char)
                .offset(UnitSystem as isize),
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Subcatchment                 mm         mm         mm         mm         mm         mm         mm    %8s      %3s\0"
                as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut VolUnitsWords as *mut *mut ::core::ffi::c_char)
                .offset(UnitSystem as isize),
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
        );
    }
    fprintf(
        Frpt.file,
        b"\n  ------------------------------------------------------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        a = (*Subcatch.offset(j as isize)).area;
        if !(a == 0.0f64) {
            fprintf(
                Frpt.file,
                b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Subcatch.offset(j as isize)).ID,
            );
            x = (*SubcatchStats.offset(j as isize)).precip * UCF(RAINDEPTH as ::core::ffi::c_int);
            fprintf(
                Frpt.file,
                b" %10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x / a,
            );
            x = (*SubcatchStats.offset(j as isize)).runon * UCF(RAINDEPTH as ::core::ffi::c_int);
            fprintf(
                Frpt.file,
                b" %10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x / a,
            );
            x = (*SubcatchStats.offset(j as isize)).evap * UCF(RAINDEPTH as ::core::ffi::c_int);
            fprintf(
                Frpt.file,
                b" %10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x / a,
            );
            x = (*SubcatchStats.offset(j as isize)).infil * UCF(RAINDEPTH as ::core::ffi::c_int);
            fprintf(
                Frpt.file,
                b" %10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x / a,
            );
            x = (*SubcatchStats.offset(j as isize)).impervRunoff
                * UCF(RAINDEPTH as ::core::ffi::c_int);
            fprintf(
                Frpt.file,
                b" %10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x / a,
            );
            x = (*SubcatchStats.offset(j as isize)).pervRunoff
                * UCF(RAINDEPTH as ::core::ffi::c_int);
            fprintf(
                Frpt.file,
                b" %10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x / a,
            );
            x = (*SubcatchStats.offset(j as isize)).runoff * UCF(RAINDEPTH as ::core::ffi::c_int);
            fprintf(
                Frpt.file,
                b" %10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x / a,
            );
            x = (*SubcatchStats.offset(j as isize)).runoff * Vcf;
            fprintf(
                Frpt.file,
                b"%12.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x,
            );
            x = (*SubcatchStats.offset(j as isize)).maxFlow * UCF(FLOW as ::core::ffi::c_int);
            fprintf(
                Frpt.file,
                b" %8.2f\0" as *const u8 as *const ::core::ffi::c_char,
                x,
            );
            r = (*SubcatchStats.offset(j as isize)).precip
                + (*SubcatchStats.offset(j as isize)).runon;
            if r > 0.0f64 {
                r = (*SubcatchStats.offset(j as isize)).runoff / r;
            }
            fprintf(
                Frpt.file,
                b"%8.3f\0" as *const u8 as *const ::core::ffi::c_char,
                r,
            );
        }
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeGroundwater() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut totalSeconds: ::core::ffi::c_double = NewRunoffTime / 1000.0f64;
    let mut x: [::core::ffi::c_double; 9] = [0.; 9];
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        if !(*Subcatch.offset(j as isize)).groundwater.is_null() {
            count += 1;
        }
        j += 1;
    }
    if count == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*******************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Groundwater Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*******************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  -----------------------------------------------------------------------------------------------------\n                                            Total    Total  Maximum  Average  Average    Final    Final\n                          Total    Total    Lower  Lateral  Lateral    Upper    Water    Upper    Water\n                          Infil     Evap  Seepage  Outflow  Outflow   Moist.    Table   Moist.    Table\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Subcatchment               in       in       in       in      %3s                ft                ft\0"
                as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Subcatchment               mm       mm       mm       mm      %3s                 m                 m\0"
                as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
        );
    }
    fprintf(
        Frpt.file,
        b"\n  -----------------------------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        if !((*Subcatch.offset(j as isize)).area == 0.0f64
            || (*Subcatch.offset(j as isize)).groundwater.is_null())
        {
            fprintf(
                Frpt.file,
                b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Subcatch.offset(j as isize)).ID,
            );
            x[0 as ::core::ffi::c_int as usize] =
                (*(*Subcatch.offset(j as isize)).groundwater).stats.infil
                    * UCF(RAINDEPTH as ::core::ffi::c_int);
            x[1 as ::core::ffi::c_int as usize] =
                (*(*Subcatch.offset(j as isize)).groundwater).stats.evap
                    * UCF(RAINDEPTH as ::core::ffi::c_int);
            x[2 as ::core::ffi::c_int as usize] =
                (*(*Subcatch.offset(j as isize)).groundwater).stats.deepFlow
                    * UCF(RAINDEPTH as ::core::ffi::c_int);
            x[3 as ::core::ffi::c_int as usize] =
                (*(*Subcatch.offset(j as isize)).groundwater).stats.latFlow
                    * UCF(RAINDEPTH as ::core::ffi::c_int);
            x[4 as ::core::ffi::c_int as usize] =
                (*(*Subcatch.offset(j as isize)).groundwater).stats.maxFlow
                    * UCF(FLOW as ::core::ffi::c_int)
                    * (*Subcatch.offset(j as isize)).area;
            x[5 as ::core::ffi::c_int as usize] = (*(*Subcatch.offset(j as isize)).groundwater)
                .stats
                .avgUpperMoist
                / totalSeconds;
            x[6 as ::core::ffi::c_int as usize] = (*(*Subcatch.offset(j as isize)).groundwater)
                .stats
                .avgWaterTable
                * UCF(LENGTH as ::core::ffi::c_int)
                / totalSeconds;
            x[7 as ::core::ffi::c_int as usize] = (*(*Subcatch.offset(j as isize)).groundwater)
                .stats
                .finalUpperMoist;
            x[8 as ::core::ffi::c_int as usize] = (*(*Subcatch.offset(j as isize)).groundwater)
                .stats
                .finalWaterTable
                * UCF(LENGTH as ::core::ffi::c_int);
            i = 0 as ::core::ffi::c_int;
            while i < 9 as ::core::ffi::c_int {
                fprintf(
                    Frpt.file,
                    b" %8.2f\0" as *const u8 as *const ::core::ffi::c_char,
                    x[i as usize],
                );
                i += 1;
            }
        }
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeSubcatchLoads() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    let mut totals: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut units: [::core::ffi::c_char; 15] = [0; 15];
    let mut subcatchLine: [::core::ffi::c_char; 21] =
        ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"--------------------\0");
    let mut pollutLine: [::core::ffi::c_char; 15] =
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"--------------\0");
    totals = calloc(
        Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    ) as *mut ::core::ffi::c_double;
    if !totals.is_null() {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(
            b"****************************\0" as *const u8 as *const ::core::ffi::c_char,
        );
        report_writeLine(
            b"Subcatchment Washoff Summary\0" as *const u8 as *const ::core::ffi::c_char,
        );
        report_writeLine(
            b"****************************\0" as *const u8 as *const ::core::ffi::c_char,
        );
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n  %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut subcatchLine as *mut ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut pollutLine as *mut ::core::ffi::c_char,
            );
            p += 1;
        }
        fprintf(
            Frpt.file,
            b"\n                      \0" as *const u8 as *const ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%14s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Pollut.offset(p as isize)).ID,
            );
            p += 1;
        }
        fprintf(
            Frpt.file,
            b"\n  Subcatchment        \0" as *const u8 as *const ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
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
            *totals.offset(p as isize) = 0.0f64;
            p += 1;
        }
        fprintf(
            Frpt.file,
            b"\n  %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut subcatchLine as *mut ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut pollutLine as *mut ::core::ffi::c_char,
            );
            p += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Subcatch.offset(j as isize)).ID,
            );
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                x = *(*Subcatch.offset(j as isize)).totalLoad.offset(p as isize);
                *totals.offset(p as isize) += x;
                if (*Pollut.offset(p as isize)).units == COUNT as ::core::ffi::c_int {
                    x = if x > 0.0f64 { log10(x) } else { x };
                }
                fprintf(
                    Frpt.file,
                    b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
                    x,
                );
                p += 1;
            }
            j += 1;
        }
        fprintf(
            Frpt.file,
            b"\n  %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut subcatchLine as *mut ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut pollutLine as *mut ::core::ffi::c_char,
            );
            p += 1;
        }
        fprintf(
            Frpt.file,
            b"\n  System              \0" as *const u8 as *const ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            x = *totals.offset(p as isize);
            if (*Pollut.offset(p as isize)).units == COUNT as ::core::ffi::c_int {
                x = if x > 0.0f64 { log10(x) } else { x };
            }
            fprintf(
                Frpt.file,
                b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
                x,
            );
            p += 1;
        }
        free(totals as *mut ::core::ffi::c_void);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn writeNodeDepths() {
    let mut j: ::core::ffi::c_int = 0;
    let mut days: ::core::ffi::c_int = 0;
    let mut hrs: ::core::ffi::c_int = 0;
    let mut mins: ::core::ffi::c_int = 0;
    if Nobjects[LINK as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"******************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Node Depth Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"******************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  ---------------------------------------------------------------------------------\n                                 Average  Maximum  Maximum  Time of Max    Reported\n                                   Depth    Depth      HGL   Occurrence   Max Depth\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Node                 Type         Feet     Feet     Feet  days hr:min        Feet\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Node                 Type       Meters   Meters   Meters  days hr:min      Meters\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        Frpt.file,
        b"\n  ---------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        fprintf(
            Frpt.file,
            b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Node.offset(j as isize)).ID,
        );
        fprintf(
            Frpt.file,
            b" %-9s \0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut NodeTypeWords as *mut *mut ::core::ffi::c_char)
                .offset((*Node.offset(j as isize)).type_0 as isize),
        );
        getElapsedTime(
            (*NodeStats.offset(j as isize)).maxDepthDate,
            &raw mut days,
            &raw mut hrs,
            &raw mut mins,
        );
        fprintf(
            Frpt.file,
            b"%7.2f  %7.2f  %7.2f  %4d  %02d:%02d  %10.2f\0" as *const u8
                as *const ::core::ffi::c_char,
            (*NodeStats.offset(j as isize)).avgDepth / ReportStepCount as ::core::ffi::c_double
                * UCF(LENGTH as ::core::ffi::c_int),
            (*NodeStats.offset(j as isize)).maxDepth * UCF(LENGTH as ::core::ffi::c_int),
            ((*NodeStats.offset(j as isize)).maxDepth + (*Node.offset(j as isize)).invertElev)
                * UCF(LENGTH as ::core::ffi::c_int),
            days,
            hrs,
            mins,
            (*NodeStats.offset(j as isize)).maxRptDepth,
        );
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeNodeFlows() {
    let mut j: ::core::ffi::c_int = 0;
    let mut days1: ::core::ffi::c_int = 0;
    let mut hrs1: ::core::ffi::c_int = 0;
    let mut mins1: ::core::ffi::c_int = 0;
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*******************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Node Inflow Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*******************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  -------------------------------------------------------------------------------------------------\n                                  Maximum  Maximum                  Lateral       Total        Flow\n                                  Lateral    Total  Time of Max      Inflow      Inflow     Balance\n                                   Inflow   Inflow   Occurrence      Volume      Volume       Error\n  Node                 Type           %3s      %3s  days hr:min    %8s    %8s     Percent\0"
            as *const u8 as *const ::core::ffi::c_char,
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
            .offset(FlowUnits as isize),
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
            .offset(FlowUnits as isize),
        *(&raw mut VolUnitsWords as *mut *mut ::core::ffi::c_char)
            .offset(UnitSystem as isize),
        *(&raw mut VolUnitsWords as *mut *mut ::core::ffi::c_char)
            .offset(UnitSystem as isize),
    );
    fprintf(
        Frpt.file,
        b"\n  -------------------------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        fprintf(
            Frpt.file,
            b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Node.offset(j as isize)).ID,
        );
        fprintf(
            Frpt.file,
            b" %-9s\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut NodeTypeWords as *mut *mut ::core::ffi::c_char)
                .offset((*Node.offset(j as isize)).type_0 as isize),
        );
        getElapsedTime(
            (*NodeStats.offset(j as isize)).maxInflowDate,
            &raw mut days1,
            &raw mut hrs1,
            &raw mut mins1,
        );
        fprintf(
            Frpt.file,
            &raw mut FlowFmt as *mut ::core::ffi::c_char,
            (*NodeStats.offset(j as isize)).maxLatFlow * UCF(FLOW as ::core::ffi::c_int),
        );
        fprintf(
            Frpt.file,
            &raw mut FlowFmt as *mut ::core::ffi::c_char,
            (*NodeStats.offset(j as isize)).maxInflow * UCF(FLOW as ::core::ffi::c_int),
        );
        fprintf(
            Frpt.file,
            b"  %4d  %02d:%02d\0" as *const u8 as *const ::core::ffi::c_char,
            days1,
            hrs1,
            mins1,
        );
        fprintf(
            Frpt.file,
            b"%12.3g\0" as *const u8 as *const ::core::ffi::c_char,
            (*NodeStats.offset(j as isize)).totLatFlow * Vcf,
        );
        fprintf(
            Frpt.file,
            b"%12.3g\0" as *const u8 as *const ::core::ffi::c_char,
            *NodeInflow.offset(j as isize) * Vcf,
        );
        if fabs(*NodeOutflow.offset(j as isize)) < 1.0f64 {
            fprintf(
                Frpt.file,
                b"%12.3f %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*NodeInflow.offset(j as isize) - *NodeOutflow.offset(j as isize)) * Vcf * 1.0e6f64,
                *(&raw mut VolUnitsWords2 as *mut *mut ::core::ffi::c_char)
                    .offset(UnitSystem as isize),
            );
        } else {
            fprintf(
                Frpt.file,
                b"%12.3f\0" as *const u8 as *const ::core::ffi::c_char,
                (*NodeInflow.offset(j as isize) - *NodeOutflow.offset(j as isize))
                    / *NodeOutflow.offset(j as isize)
                    * 100.0f64,
            );
        }
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeNodeSurcharge() {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut t: ::core::ffi::c_double = 0.;
    let mut d1: ::core::ffi::c_double = 0.;
    let mut d2: ::core::ffi::c_double = 0.;
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"**********************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Node Surcharge Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"**********************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(j as isize)).type_0 == OUTFALL as ::core::ffi::c_int) {
            if !((*NodeStats.offset(j as isize)).timeSurcharged == 0.0f64) {
                t = if 0.01f64 >= (*NodeStats.offset(j as isize)).timeSurcharged / 3600.0f64 {
                    0.01f64
                } else {
                    (*NodeStats.offset(j as isize)).timeSurcharged / 3600.0f64
                };
                if n == 0 as ::core::ffi::c_int {
                    report_writeLine(
                        b"Surcharging occurs when water rises above the top of the highest conduit.\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    fprintf(
                        Frpt.file,
                        b"\n  ---------------------------------------------------------------------\n                                               Max. Height   Min. Depth\n                                   Hours       Above Crown    Below Rim\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    if UnitSystem == US as ::core::ffi::c_int {
                        fprintf(
                            Frpt.file,
                            b"\n  Node                 Type      Surcharged           Feet         Feet\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        fprintf(
                            Frpt.file,
                            b"\n  Node                 Type      Surcharged         Meters       Meters\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    fprintf(
                        Frpt.file,
                        b"\n  ---------------------------------------------------------------------\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    n = 1 as ::core::ffi::c_int;
                }
                fprintf(
                    Frpt.file,
                    b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Node.offset(j as isize)).ID,
                );
                fprintf(
                    Frpt.file,
                    b" %-9s\0" as *const u8 as *const ::core::ffi::c_char,
                    *(&raw mut NodeTypeWords as *mut *mut ::core::ffi::c_char)
                        .offset((*Node.offset(j as isize)).type_0 as isize),
                );
                d1 = (*NodeStats.offset(j as isize)).maxDepth
                    + (*Node.offset(j as isize)).invertElev
                    - (*Node.offset(j as isize)).crownElev;
                if d1 < 0.0f64 {
                    d1 = 0.0f64;
                }
                d2 =
                    (*Node.offset(j as isize)).fullDepth - (*NodeStats.offset(j as isize)).maxDepth;
                if d2 < 0.0f64 {
                    d2 = 0.0f64;
                }
                fprintf(
                    Frpt.file,
                    b"  %9.2f      %9.3f    %9.3f\0" as *const u8 as *const ::core::ffi::c_char,
                    t,
                    d1 * UCF(LENGTH as ::core::ffi::c_int),
                    d2 * UCF(LENGTH as ::core::ffi::c_int),
                );
            }
        }
        j += 1;
    }
    if n == 0 as ::core::ffi::c_int {
        report_writeLine(b"No nodes were surcharged.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeNodeFlooding() {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut days: ::core::ffi::c_int = 0;
    let mut hrs: ::core::ffi::c_int = 0;
    let mut mins: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_double = 0.;
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*********************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Node Flooding Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*********************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(j as isize)).type_0 == OUTFALL as ::core::ffi::c_int) {
            if !((*NodeStats.offset(j as isize)).timeFlooded == 0.0f64) {
                t = if 0.01f64 >= (*NodeStats.offset(j as isize)).timeFlooded / 3600.0f64 {
                    0.01f64
                } else {
                    (*NodeStats.offset(j as isize)).timeFlooded / 3600.0f64
                };
                if n == 0 as ::core::ffi::c_int {
                    report_writeLine(
                        b"Flooding refers to all water that overflows a node, whether it ponds or not.\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    fprintf(
                        Frpt.file,
                        b"\n  --------------------------------------------------------------------------\n                                                             Total   Maximum\n                                 Maximum   Time of Max       Flood    Ponded\n                        Hours       Rate    Occurrence      Volume\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    if RouteModel == DW as ::core::ffi::c_int {
                        fprintf(
                            Frpt.file,
                            b"     Depth\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        fprintf(
                            Frpt.file,
                            b"    Volume\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    fprintf(
                        Frpt.file,
                        b"\n  Node                 Flooded       %3s   days hr:min    %8s\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                            .offset(FlowUnits as isize),
                        *(&raw mut VolUnitsWords as *mut *mut ::core::ffi::c_char)
                            .offset(UnitSystem as isize),
                    );
                    if RouteModel == DW as ::core::ffi::c_int {
                        fprintf(
                            Frpt.file,
                            b"    %6s\0" as *const u8 as *const ::core::ffi::c_char,
                            *(&raw mut PondingUnitsWords as *mut *mut ::core::ffi::c_char)
                                .offset(UnitSystem as isize),
                        );
                    } else if UnitSystem == US as ::core::ffi::c_int {
                        fprintf(
                            Frpt.file,
                            b"  1000 ft\xB3\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        fprintf(
                            Frpt.file,
                            b"   1000 m\xB3\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    fprintf(
                        Frpt.file,
                        b"\n  --------------------------------------------------------------------------\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    n = 1 as ::core::ffi::c_int;
                }
                fprintf(
                    Frpt.file,
                    b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Node.offset(j as isize)).ID,
                );
                fprintf(
                    Frpt.file,
                    b" %7.2f \0" as *const u8 as *const ::core::ffi::c_char,
                    t,
                );
                fprintf(
                    Frpt.file,
                    &raw mut FlowFmt as *mut ::core::ffi::c_char,
                    (*NodeStats.offset(j as isize)).maxOverflow * UCF(FLOW as ::core::ffi::c_int),
                );
                getElapsedTime(
                    (*NodeStats.offset(j as isize)).maxOverflowDate,
                    &raw mut days,
                    &raw mut hrs,
                    &raw mut mins,
                );
                fprintf(
                    Frpt.file,
                    b"   %4d  %02d:%02d\0" as *const u8 as *const ::core::ffi::c_char,
                    days,
                    hrs,
                    mins,
                );
                fprintf(
                    Frpt.file,
                    b"%12.3f\0" as *const u8 as *const ::core::ffi::c_char,
                    (*NodeStats.offset(j as isize)).volFlooded * Vcf,
                );
                if RouteModel == DW as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b" %9.3f\0" as *const u8 as *const ::core::ffi::c_char,
                        ((*NodeStats.offset(j as isize)).maxDepth
                            - (*Node.offset(j as isize)).fullDepth)
                            * UCF(LENGTH as ::core::ffi::c_int),
                    );
                } else {
                    fprintf(
                        Frpt.file,
                        b" %9.3f\0" as *const u8 as *const ::core::ffi::c_char,
                        (*NodeStats.offset(j as isize)).maxPondedVol / 1000.0f64
                            * UCF(VOLUME as ::core::ffi::c_int),
                    );
                }
            }
        }
        j += 1;
    }
    if n == 0 as ::core::ffi::c_int {
        report_writeLine(b"No nodes were flooded.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeStorageVolumes() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut days: ::core::ffi::c_int = 0;
    let mut hrs: ::core::ffi::c_int = 0;
    let mut mins: ::core::ffi::c_int = 0;
    let mut avgVol: ::core::ffi::c_double = 0.;
    let mut maxVol: ::core::ffi::c_double = 0.;
    let mut pctAvgVol: ::core::ffi::c_double = 0.;
    let mut pctMaxVol: ::core::ffi::c_double = 0.;
    let mut pctEvapLoss: ::core::ffi::c_double = 0.;
    let mut pctSeepLoss: ::core::ffi::c_double = 0.;
    if Nnodes[STORAGE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"**********************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Storage Volume Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"**********************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n  ------------------------------------------------------------------------------------------------\n                         Average    Avg   Evap  Exfil     Maximum    Max    Time of Max    Maximum\n                          Volume   Pcnt   Pcnt   Pcnt      Volume   Pcnt     Occurrence    Outflow\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        if UnitSystem == US as ::core::ffi::c_int {
            fprintf(
                Frpt.file,
                b"\n  Storage Unit          1000 ft\xB3   Full   Loss   Loss    1000 ft\xB3   Full    days hr:min        \0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            fprintf(
                Frpt.file,
                b"\n  Storage Unit           1000 m\xB3   Full   Loss   Loss     1000 m\xB3   Full    days hr:min        \0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        fprintf(
            Frpt.file,
            b"%3s\0" as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        );
        fprintf(
            Frpt.file,
            b"\n  ------------------------------------------------------------------------------------------------\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            if !((*Node.offset(j as isize)).type_0 != STORAGE as ::core::ffi::c_int) {
                k = (*Node.offset(j as isize)).subIndex;
                fprintf(
                    Frpt.file,
                    b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Node.offset(j as isize)).ID,
                );
                avgVol = (*StorageStats.offset(k as isize)).avgVol
                    / ReportStepCount as ::core::ffi::c_double;
                maxVol = (*StorageStats.offset(k as isize)).maxVol;
                pctMaxVol = 0.0f64;
                pctAvgVol = 0.0f64;
                if (*Node.offset(j as isize)).fullVolume > 0.0f64 {
                    pctAvgVol = avgVol / (*Node.offset(j as isize)).fullVolume * 100.0f64;
                    pctMaxVol = maxVol / (*Node.offset(j as isize)).fullVolume * 100.0f64;
                }
                pctEvapLoss = 0.0f64;
                pctSeepLoss = 0.0f64;
                if *NodeInflow.offset(j as isize) > 0.0f64 {
                    pctEvapLoss = (*StorageStats.offset(k as isize)).evapLosses
                        / *NodeInflow.offset(j as isize)
                        * 100.0f64;
                    pctSeepLoss = (*StorageStats.offset(k as isize)).exfilLosses
                        / *NodeInflow.offset(j as isize)
                        * 100.0f64;
                }
                fprintf(
                    Frpt.file,
                    b"%10.3f  %5.1f  %5.1f  %5.1f  %10.3f  %5.1f\0" as *const u8
                        as *const ::core::ffi::c_char,
                    avgVol * UCF(VOLUME as ::core::ffi::c_int) / 1000.0f64,
                    pctAvgVol,
                    pctEvapLoss,
                    pctSeepLoss,
                    maxVol * UCF(VOLUME as ::core::ffi::c_int) / 1000.0f64,
                    pctMaxVol,
                );
                getElapsedTime(
                    (*StorageStats.offset(k as isize)).maxVolDate,
                    &raw mut days,
                    &raw mut hrs,
                    &raw mut mins,
                );
                fprintf(
                    Frpt.file,
                    b"    %4d  %02d:%02d  \0" as *const u8 as *const ::core::ffi::c_char,
                    days,
                    hrs,
                    mins,
                );
                fprintf(
                    Frpt.file,
                    &raw mut FlowFmt as *mut ::core::ffi::c_char,
                    (*StorageStats.offset(k as isize)).maxFlow * UCF(FLOW as ::core::ffi::c_int),
                );
            }
            j += 1;
        }
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn writeOutfallLoads() {
    let mut units: [::core::ffi::c_char; 15] = [0; 15];
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    let mut outfallCount: ::core::ffi::c_double = 0.;
    let mut flowCount: ::core::ffi::c_double = 0.;
    let mut flowSum: ::core::ffi::c_double = 0.;
    let mut freqSum: ::core::ffi::c_double = 0.;
    let mut volSum: ::core::ffi::c_double = 0.;
    let mut totals: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if Nnodes[OUTFALL as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        totals = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *totals.offset(p as isize) = 0.0f64;
            p += 1;
        }
        flowSum = 0.0f64;
        freqSum = 0.0f64;
        volSum = 0.0f64;
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"***********************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Outfall Loading Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"***********************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n  -----------------------------------------------------------\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"--------------\0" as *const u8 as *const ::core::ffi::c_char,
            );
            p += 1;
        }
        fprintf(
            Frpt.file,
            b"\n                         Flow       Avg       Max       Total\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"         Total\0" as *const u8 as *const ::core::ffi::c_char,
            );
            p += 1;
        }
        fprintf(
            Frpt.file,
            b"\n                         Freq      Flow      Flow      Volume\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"%14s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Pollut.offset(p as isize)).ID,
            );
            p += 1;
        }
        fprintf(
            Frpt.file,
            b"\n  Outfall Node           Pcnt       %3s       %3s    %8s\0" as *const u8
                as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
            *(&raw mut VolUnitsWords as *mut *mut ::core::ffi::c_char).offset(UnitSystem as isize),
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
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
            b"\n  -----------------------------------------------------------\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"--------------\0" as *const u8 as *const ::core::ffi::c_char,
            );
            p += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            if !((*Node.offset(j as isize)).type_0 != OUTFALL as ::core::ffi::c_int) {
                k = (*Node.offset(j as isize)).subIndex;
                flowCount =
                    (*OutfallStats.offset(k as isize)).totalPeriods as ::core::ffi::c_double;
                fprintf(
                    Frpt.file,
                    b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Node.offset(j as isize)).ID,
                );
                x = 100.0f64 * flowCount / ReportStepCount as ::core::ffi::c_double;
                fprintf(
                    Frpt.file,
                    b"%7.2f\0" as *const u8 as *const ::core::ffi::c_char,
                    x,
                );
                freqSum += x;
                if flowCount > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    x = (*OutfallStats.offset(k as isize)).avgFlow
                        * UCF(FLOW as ::core::ffi::c_int)
                        / flowCount;
                } else {
                    x = 0.0f64;
                }
                flowSum += x;
                fprintf(Frpt.file, b" \0" as *const u8 as *const ::core::ffi::c_char);
                fprintf(Frpt.file, &raw mut FlowFmt as *mut ::core::ffi::c_char, x);
                fprintf(Frpt.file, b" \0" as *const u8 as *const ::core::ffi::c_char);
                fprintf(
                    Frpt.file,
                    &raw mut FlowFmt as *mut ::core::ffi::c_char,
                    (*OutfallStats.offset(k as isize)).maxFlow * UCF(FLOW as ::core::ffi::c_int),
                );
                fprintf(
                    Frpt.file,
                    b"%12.3f\0" as *const u8 as *const ::core::ffi::c_char,
                    *NodeInflow.offset(j as isize) * Vcf,
                );
                volSum += *NodeInflow.offset(j as isize);
                p = 0 as ::core::ffi::c_int;
                while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    x = *(*OutfallStats.offset(k as isize))
                        .totalLoad
                        .offset(p as isize)
                        * LperFT3
                        * (*Pollut.offset(p as isize)).mcf;
                    *totals.offset(p as isize) += x;
                    if (*Pollut.offset(p as isize)).units == COUNT as ::core::ffi::c_int {
                        x = if x > 0.0f64 { log10(x) } else { x };
                    }
                    fprintf(
                        Frpt.file,
                        b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
                        x,
                    );
                    p += 1;
                }
            }
            j += 1;
        }
        outfallCount = Nnodes[OUTFALL as ::core::ffi::c_int as usize] as ::core::ffi::c_double;
        fprintf(
            Frpt.file,
            b"\n  -----------------------------------------------------------\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"--------------\0" as *const u8 as *const ::core::ffi::c_char,
            );
            p += 1;
        }
        fprintf(
            Frpt.file,
            b"\n  System              %7.2f \0" as *const u8 as *const ::core::ffi::c_char,
            freqSum / outfallCount,
        );
        fprintf(
            Frpt.file,
            &raw mut FlowFmt as *mut ::core::ffi::c_char,
            flowSum,
        );
        fprintf(Frpt.file, b" \0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            &raw mut FlowFmt as *mut ::core::ffi::c_char,
            MaxOutfallFlow * UCF(FLOW as ::core::ffi::c_int),
        );
        fprintf(
            Frpt.file,
            b"%12.3f\0" as *const u8 as *const ::core::ffi::c_char,
            volSum * Vcf,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            x = *totals.offset(p as isize);
            if (*Pollut.offset(p as isize)).units == COUNT as ::core::ffi::c_int {
                x = if x > 0.0f64 { log10(x) } else { x };
            }
            fprintf(
                Frpt.file,
                b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
                x,
            );
            p += 1;
        }
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        free(totals as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn writeLinkFlows() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut days: ::core::ffi::c_int = 0;
    let mut hrs: ::core::ffi::c_int = 0;
    let mut mins: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_double = 0.;
    let mut fullDepth: ::core::ffi::c_double = 0.;
    if Nobjects[LINK as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"********************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Link Flow Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"********************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  -----------------------------------------------------------------------------\n                                 Maximum  Time of Max   Maximum    Max/    Max/\n                                  |Flow|   Occurrence   |Veloc|    Full    Full\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Link                 Type          %3s  days hr:min    ft/sec    Flow   Depth\0"
                as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Link                 Type          %3s  days hr:min     m/sec    Flow   Depth\0"
                as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
        );
    }
    fprintf(
        Frpt.file,
        b"\n  -----------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        k = (*Link.offset(j as isize)).subIndex;
        fprintf(
            Frpt.file,
            b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Link.offset(j as isize)).ID,
        );
        if (*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int {
            fprintf(
                Frpt.file,
                b" DUMMY   \0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if (*Link.offset(j as isize)).xsect.type_0 == IRREGULAR as ::core::ffi::c_int {
            fprintf(
                Frpt.file,
                b" CHANNEL \0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            fprintf(
                Frpt.file,
                b" %-7s \0" as *const u8 as *const ::core::ffi::c_char,
                *(&raw mut LinkTypeWords as *mut *mut ::core::ffi::c_char)
                    .offset((*Link.offset(j as isize)).type_0 as isize),
            );
        }
        getElapsedTime(
            (*LinkStats.offset(j as isize)).maxFlowDate,
            &raw mut days,
            &raw mut hrs,
            &raw mut mins,
        );
        fprintf(
            Frpt.file,
            &raw mut FlowFmt as *mut ::core::ffi::c_char,
            (*LinkStats.offset(j as isize)).maxFlow * UCF(FLOW as ::core::ffi::c_int),
        );
        fprintf(
            Frpt.file,
            b"  %4d  %02d:%02d\0" as *const u8 as *const ::core::ffi::c_char,
            days,
            hrs,
            mins,
        );
        if (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int
            && (*Link.offset(j as isize)).qFull > 0.0f64
        {
            fprintf(
                Frpt.file,
                b"          \0" as *const u8 as *const ::core::ffi::c_char,
            );
            fprintf(
                Frpt.file,
                b"  %6.2f\0" as *const u8 as *const ::core::ffi::c_char,
                (*LinkStats.offset(j as isize)).maxFlow / (*Link.offset(j as isize)).qFull,
            );
        } else if !((*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int) {
            if !((*Link.offset(j as isize)).type_0 == OUTLET as ::core::ffi::c_int) {
                if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
                    v = (*LinkStats.offset(j as isize)).maxVeloc
                        * UCF(LENGTH as ::core::ffi::c_int);
                    if v > 50.0f64 {
                        fprintf(
                            Frpt.file,
                            b"    >50.00\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        fprintf(
                            Frpt.file,
                            b"   %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
                            v,
                        );
                    }
                    fprintf(
                        Frpt.file,
                        b"  %6.2f\0" as *const u8 as *const ::core::ffi::c_char,
                        (*LinkStats.offset(j as isize)).maxFlow
                            / (*Link.offset(j as isize)).qFull
                            / (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_double,
                    );
                } else {
                    fprintf(
                        Frpt.file,
                        b"                  \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fullDepth = (*Link.offset(j as isize)).xsect.yFull;
                if (*Link.offset(j as isize)).type_0 == ORIFICE as ::core::ffi::c_int
                    && (*Orifice.offset(k as isize)).type_0 == BOTTOM_ORIFICE as ::core::ffi::c_int
                {
                    fullDepth = 0.0f64;
                }
                if fullDepth > 0.0f64 {
                    fprintf(
                        Frpt.file,
                        b"  %6.2f\0" as *const u8 as *const ::core::ffi::c_char,
                        (*LinkStats.offset(j as isize)).maxDepth / fullDepth,
                    );
                } else {
                    fprintf(
                        Frpt.file,
                        b"        \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeFlowClass() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut totalSeconds: ::core::ffi::c_double = RoutingTimeSpan;
    if RouteModel != DW as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Flow Classification Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  -------------------------------------------------------------------------------------\n                      Adjusted    ---------- Fraction of Time in Flow Class ---------- \n                       /Actual         Up    Down  Sub   Sup   Up    Down  Norm  Inlet \n  Conduit               Length    Dry  Dry   Dry   Crit  Crit  Crit  Crit  Ltd   Ctrl  \n  -------------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if !((*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int) {
            if !((*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int) {
                k = (*Link.offset(j as isize)).subIndex;
                fprintf(
                    Frpt.file,
                    b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Link.offset(j as isize)).ID,
                );
                fprintf(
                    Frpt.file,
                    b"  %6.2f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Conduit.offset(k as isize)).modLength / (*Conduit.offset(k as isize)).length,
                );
                i = 0 as ::core::ffi::c_int;
                while i < MAX_FLOW_CLASSES as ::core::ffi::c_int {
                    let ref mut fresh0 =
                        (*LinkStats.offset(j as isize)).timeInFlowClass[i as usize];
                    *fresh0 /= totalSeconds;
                    fprintf(
                        Frpt.file,
                        b"  %4.2f\0" as *const u8 as *const ::core::ffi::c_char,
                        *fresh0,
                    );
                    i += 1;
                }
                fprintf(
                    Frpt.file,
                    b"  %4.2f\0" as *const u8 as *const ::core::ffi::c_char,
                    (*LinkStats.offset(j as isize)).timeNormalFlow / totalSeconds,
                );
                fprintf(
                    Frpt.file,
                    b"  %4.2f\0" as *const u8 as *const ::core::ffi::c_char,
                    (*LinkStats.offset(j as isize)).timeInletControl / totalSeconds,
                );
            }
        }
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeLinkSurcharge() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut t: [::core::ffi::c_double; 5] = [0.; 5];
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Conduit Surcharge Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if !((*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int
            || (*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int)
        {
            t[0 as ::core::ffi::c_int as usize] =
                (*LinkStats.offset(j as isize)).timeSurcharged / 3600.0f64;
            t[1 as ::core::ffi::c_int as usize] =
                (*LinkStats.offset(j as isize)).timeFullUpstream / 3600.0f64;
            t[2 as ::core::ffi::c_int as usize] =
                (*LinkStats.offset(j as isize)).timeFullDnstream / 3600.0f64;
            t[3 as ::core::ffi::c_int as usize] =
                (*LinkStats.offset(j as isize)).timeFullFlow / 3600.0f64;
            if !(t[0 as ::core::ffi::c_int as usize]
                + t[1 as ::core::ffi::c_int as usize]
                + t[2 as ::core::ffi::c_int as usize]
                + t[3 as ::core::ffi::c_int as usize]
                == 0.0f64)
            {
                t[4 as ::core::ffi::c_int as usize] =
                    (*LinkStats.offset(j as isize)).timeCapacityLimited / 3600.0f64;
                i = 0 as ::core::ffi::c_int;
                while i < 5 as ::core::ffi::c_int {
                    t[i as usize] = if 0.01f64 >= t[i as usize] {
                        0.01f64
                    } else {
                        t[i as usize]
                    };
                    i += 1;
                }
                if n == 0 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n  ----------------------------------------------------------------------------\n                                                           Hours        Hours \n                         --------- Hours Full --------   Above Full   Capacity\n  Conduit                Both Ends  Upstream  Dnstream   Normal Flow   Limited\n  ----------------------------------------------------------------------------\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    n = 1 as ::core::ffi::c_int;
                }
                fprintf(
                    Frpt.file,
                    b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Link.offset(j as isize)).ID,
                );
                fprintf(
                    Frpt.file,
                    b"    %8.2f  %8.2f  %8.2f  %8.2f     %8.2f\0" as *const u8
                        as *const ::core::ffi::c_char,
                    t[0 as ::core::ffi::c_int as usize],
                    t[1 as ::core::ffi::c_int as usize],
                    t[2 as ::core::ffi::c_int as usize],
                    t[3 as ::core::ffi::c_int as usize],
                    t[4 as ::core::ffi::c_int as usize],
                );
            }
        }
        j += 1;
    }
    if n == 0 as ::core::ffi::c_int {
        report_writeLine(
            b"No conduits were surcharged.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writePumpFlows() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut avgFlow: ::core::ffi::c_double = 0.;
    let mut pctUtilized: ::core::ffi::c_double = 0.;
    let mut pctOffCurve1: ::core::ffi::c_double = 0.;
    let mut pctOffCurve2: ::core::ffi::c_double = 0.;
    let mut totalSeconds: ::core::ffi::c_double = RoutingTimeSpan;
    if Nlinks[PUMP as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Pumping Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  ---------------------------------------------------------------------------------------------------------\n                                                  Min       Avg       Max     Total     Power    %% Time Off\n                        Percent   Number of      Flow      Flow      Flow    Volume     Usage    Pump Curve\n  Pump                 Utilized   Start-Ups       %3s       %3s       %3s  %8s     Kw-hr    Low   High\n  ---------------------------------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
            .offset(FlowUnits as isize),
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
            .offset(FlowUnits as isize),
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
            .offset(FlowUnits as isize),
        *(&raw mut VolUnitsWords as *mut *mut ::core::ffi::c_char)
            .offset(UnitSystem as isize),
    );
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if !((*Link.offset(j as isize)).type_0 != PUMP as ::core::ffi::c_int) {
            k = (*Link.offset(j as isize)).subIndex;
            fprintf(
                Frpt.file,
                b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Link.offset(j as isize)).ID,
            );
            pctUtilized = (*PumpStats.offset(k as isize)).utilized / totalSeconds * 100.0f64;
            avgFlow = (*PumpStats.offset(k as isize)).avgFlow;
            if (*PumpStats.offset(k as isize)).totalPeriods > 0 as ::core::ffi::c_int {
                avgFlow /= (*PumpStats.offset(k as isize)).totalPeriods as ::core::ffi::c_double;
            }
            fprintf(
                Frpt.file,
                b" %8.2f  %10d %9.2f %9.2f %9.2f %9.3f %9.2f\0" as *const u8
                    as *const ::core::ffi::c_char,
                pctUtilized,
                (*PumpStats.offset(k as isize)).startUps,
                (*PumpStats.offset(k as isize)).minFlow * UCF(FLOW as ::core::ffi::c_int),
                avgFlow * UCF(FLOW as ::core::ffi::c_int),
                (*PumpStats.offset(k as isize)).maxFlow * UCF(FLOW as ::core::ffi::c_int),
                (*PumpStats.offset(k as isize)).volume * Vcf,
                (*PumpStats.offset(k as isize)).energy,
            );
            pctOffCurve1 = (*PumpStats.offset(k as isize)).offCurveLow;
            pctOffCurve2 = (*PumpStats.offset(k as isize)).offCurveHigh;
            if (*PumpStats.offset(k as isize)).utilized > 0.0f64 {
                pctOffCurve1 = pctOffCurve1 / (*PumpStats.offset(k as isize)).utilized * 100.0f64;
                pctOffCurve2 = pctOffCurve2 / (*PumpStats.offset(k as isize)).utilized * 100.0f64;
            }
            fprintf(
                Frpt.file,
                b" %6.1f %6.1f\0" as *const u8 as *const ::core::ffi::c_char,
                pctOffCurve1,
                pctOffCurve2,
            );
        }
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeLinkLoads() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    let mut units: [::core::ffi::c_char; 15] = [0; 15];
    let mut linkLine: [::core::ffi::c_char; 21] =
        ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"--------------------\0");
    let mut pollutLine: [::core::ffi::c_char; 15] =
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"--------------\0");
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Link Pollutant Load Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"***************************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut linkLine as *mut ::core::ffi::c_char,
    );
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        fprintf(
            Frpt.file,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut pollutLine as *mut ::core::ffi::c_char,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n                      \0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        fprintf(
            Frpt.file,
            b"%14s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Pollut.offset(p as isize)).ID,
        );
        p += 1;
    }
    fprintf(
        Frpt.file,
        b"\n  Link                \0" as *const u8 as *const ::core::ffi::c_char,
    );
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
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
        b"\n  %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut linkLine as *mut ::core::ffi::c_char,
    );
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        fprintf(
            Frpt.file,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut pollutLine as *mut ::core::ffi::c_char,
        );
        p += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        fprintf(
            Frpt.file,
            b"\n  %-20s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Link.offset(j as isize)).ID,
        );
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            x = *(*Link.offset(j as isize)).totalLoad.offset(p as isize)
                * LperFT3
                * (*Pollut.offset(p as isize)).mcf;
            if (*Pollut.offset(p as isize)).units == COUNT as ::core::ffi::c_int {
                x = if x > 0.0f64 { log10(x) } else { x };
            }
            if x < 10000.0f64 {
                fprintf(
                    Frpt.file,
                    b"%14.3f\0" as *const u8 as *const ::core::ffi::c_char,
                    x,
                );
            } else {
                fprintf(
                    Frpt.file,
                    b"%14.3e\0" as *const u8 as *const ::core::ffi::c_char,
                    x,
                );
            }
            p += 1;
        }
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
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
