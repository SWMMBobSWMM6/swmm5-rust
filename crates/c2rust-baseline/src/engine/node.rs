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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn exfil_readStorageParams(
        k: ::core::ffi::c_int,
        tok: *mut *mut ::core::ffi::c_char,
        ntoks: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn exfil_initState(k: ::core::ffi::c_int);
    fn exfil_getLoss(
        exfil: *mut TExfil,
        tStep: ::core::ffi::c_double,
        depth: ::core::ffi::c_double,
        area: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut RouteModel: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut StartDateTime: DateTime;
    static mut NewRoutingTime: ::core::ffi::c_double;
    static mut Evap: TEvap;
    static mut Node: *mut TNode;
    static mut Outfall: *mut TOutfall;
    static mut Divider: *mut TDivider;
    static mut Storage: *mut TStorage;
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
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn report_writeWarningMsg(msg: *mut ::core::ffi::c_char, id: *mut ::core::ffi::c_char);
    fn inflow_initDwfInflow(inflow: *mut TDwfInflow);
    fn xsect_getSofA(xsect: *mut TXsect, area: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getAofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn table_getFirstEntry(
        table: *mut TTable,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn table_lookup(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn table_lookupEx(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn table_getStorageVolume(
        table: *mut TTable,
        x: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn table_getStorageDepth(table: *mut TTable, v: ::core::ffi::c_double)
        -> ::core::ffi::c_double;
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
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut DividerTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut NoYesWords: [*mut ::core::ffi::c_char; 0];
    static mut OutfallTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut RelationWords: [*mut ::core::ffi::c_char; 0];
    fn findroot_Newton(
        x1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        rts: *mut ::core::ffi::c_double,
        xacc: ::core::ffi::c_double,
        func: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        p: *mut ::core::ffi::c_void,
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
pub type NodeResultType = ::core::ffi::c_uint;
pub const NODE_QUAL: NodeResultType = 6;
pub const NODE_OVERFLOW: NodeResultType = 5;
pub const NODE_INFLOW: NodeResultType = 4;
pub const NODE_LATFLOW: NodeResultType = 3;
pub const NODE_VOLUME: NodeResultType = 2;
pub const NODE_HEAD: NodeResultType = 1;
pub const NODE_DEPTH: NodeResultType = 0;
pub type RouteModelType = ::core::ffi::c_uint;
pub const DW: RouteModelType = 4;
pub const EKW: RouteModelType = 3;
pub const KW: RouteModelType = 2;
pub const SF: RouteModelType = 1;
pub const NO_ROUTING: RouteModelType = 0;
pub type OutfallType = ::core::ffi::c_uint;
pub const TIMESERIES_OUTFALL: OutfallType = 4;
pub const TIDAL_OUTFALL: OutfallType = 3;
pub const FIXED_OUTFALL: OutfallType = 2;
pub const NORMAL_OUTFALL: OutfallType = 1;
pub const FREE_OUTFALL: OutfallType = 0;
pub type StorageType = ::core::ffi::c_uint;
pub const PYRAMIDAL: StorageType = 5;
pub const PARABOLOID: StorageType = 4;
pub const CONICAL: StorageType = 3;
pub const CYLINDRICAL: StorageType = 2;
pub const FUNCTIONAL: StorageType = 1;
pub const TABULAR: StorageType = 0;
pub type DividerType = ::core::ffi::c_uint;
pub const OVERFLOW_DIVIDER: DividerType = 3;
pub const WEIR_DIVIDER: DividerType = 2;
pub const TABULAR_DIVIDER: DividerType = 1;
pub const CUTOFF_DIVIDER: DividerType = 0;
pub type NodeInletType = ::core::ffi::c_uint;
pub const CAPTURE: NodeInletType = 2;
pub const BYPASS: NodeInletType = 1;
pub const NO_INLET: NodeInletType = 0;
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
pub struct TDivider {
    pub link: ::core::ffi::c_int,
    pub type_0: ::core::ffi::c_int,
    pub qMin: ::core::ffi::c_double,
    pub qMax: ::core::ffi::c_double,
    pub dhMax: ::core::ffi::c_double,
    pub cWeir: ::core::ffi::c_double,
    pub flowCurve: ::core::ffi::c_int,
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
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_WEIR_DIVIDER: ErrorType = 137;
pub const ERR_DIVIDER_LINK: ErrorType = 136;
pub const ERR_STORAGE_VOLUME: ErrorType = 140;
pub const ERR_NODE_DEPTH: ErrorType = 138;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TStorageVol {
    pub k: ::core::ffi::c_int,
    pub v: ::core::ffi::c_double,
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
pub const ERR_REGULATOR: ErrorType = 139;
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
pub const PI: ::core::ffi::c_double = 3.141592654f64;
pub const FLOW_TOL: ::core::ffi::c_double = 0.00001f64;
pub const FUDGE: ::core::ffi::c_double = 0.0001f64;
pub const MSECperDAY: ::core::ffi::c_double = 8.64e7f64;
pub const WARN02: [::core::ffi::c_char; 45] = unsafe {
    ::core::mem::transmute::<[u8; 45], [::core::ffi::c_char; 45]>(
        *b"WARNING 02: maximum depth increased for Node\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn node_readParams(
    mut j: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match type_0 {
        0 => return junc_readParams(j, k, tok, ntoks),
        1 => return outfall_readParams(j, k, tok, ntoks),
        2 => return storage_readParams(j, k, tok, ntoks),
        3 => return divider_readParams(j, k, tok, ntoks),
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn node_setParams(
    mut j: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    (*Node.offset(j as isize)).type_0 = type_0;
    (*Node.offset(j as isize)).subIndex = k;
    (*Node.offset(j as isize)).invertElev =
        *x.offset(0 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
    (*Node.offset(j as isize)).crownElev = (*Node.offset(j as isize)).invertElev;
    (*Node.offset(j as isize)).initDepth = 0.0f64;
    (*Node.offset(j as isize)).newVolume = 0.0f64;
    (*Node.offset(j as isize)).fullVolume = 0.0f64;
    (*Node.offset(j as isize)).fullDepth = 0.0f64;
    (*Node.offset(j as isize)).surDepth = 0.0f64;
    (*Node.offset(j as isize)).pondedArea = 0.0f64;
    (*Node.offset(j as isize)).degree = 0 as ::core::ffi::c_int;
    (*Node.offset(j as isize)).inlet = NO_INLET as ::core::ffi::c_int;
    match type_0 {
        0 => {
            (*Node.offset(j as isize)).fullDepth =
                *x.offset(1 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Node.offset(j as isize)).initDepth =
                *x.offset(2 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Node.offset(j as isize)).surDepth =
                *x.offset(3 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Node.offset(j as isize)).pondedArea = *x.offset(4 as ::core::ffi::c_int as isize)
                / (UCF(LENGTH as ::core::ffi::c_int) * UCF(LENGTH as ::core::ffi::c_int));
        }
        1 => {
            (*Outfall.offset(k as isize)).type_0 =
                crate::ccast::f64_to_i32(*x.offset(1 as ::core::ffi::c_int as isize));
            (*Outfall.offset(k as isize)).fixedStage =
                *x.offset(2 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Outfall.offset(k as isize)).tideCurve =
                crate::ccast::f64_to_i32(*x.offset(3 as ::core::ffi::c_int as isize));
            (*Outfall.offset(k as isize)).stageSeries =
                crate::ccast::f64_to_i32(*x.offset(4 as ::core::ffi::c_int as isize));
            (*Outfall.offset(k as isize)).hasFlapGate =
                crate::ccast::f64_to_i8(*x.offset(5 as ::core::ffi::c_int as isize));
            (*Outfall.offset(k as isize)).routeTo =
                crate::ccast::f64_to_i32(*x.offset(6 as ::core::ffi::c_int as isize));
            let ref mut fresh1 = (*Outfall.offset(k as isize)).wRouted;
            *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            if (*Outfall.offset(k as isize)).routeTo >= 0 as ::core::ffi::c_int {
                let ref mut fresh2 = (*Outfall.offset(k as isize)).wRouted;
                *fresh2 = calloc(
                    Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
                    ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                ) as *mut ::core::ffi::c_double;
            }
        }
        2 => {
            (*Node.offset(j as isize)).fullDepth =
                *x.offset(1 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Node.offset(j as isize)).initDepth =
                *x.offset(2 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Storage.offset(k as isize)).shape =
                crate::ccast::f64_to_i32(*x.offset(3 as ::core::ffi::c_int as isize));
            (*Storage.offset(k as isize)).a1 = *x.offset(4 as ::core::ffi::c_int as isize);
            (*Storage.offset(k as isize)).a2 = *x.offset(5 as ::core::ffi::c_int as isize);
            (*Storage.offset(k as isize)).a0 = *x.offset(6 as ::core::ffi::c_int as isize);
            (*Storage.offset(k as isize)).aCurve =
                crate::ccast::f64_to_i32(*x.offset(7 as ::core::ffi::c_int as isize));
            (*Node.offset(j as isize)).surDepth =
                *x.offset(8 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Storage.offset(k as isize)).fEvap = *x.offset(9 as ::core::ffi::c_int as isize);
        }
        3 => {
            (*Divider.offset(k as isize)).link =
                crate::ccast::f64_to_i32(*x.offset(1 as ::core::ffi::c_int as isize));
            (*Divider.offset(k as isize)).type_0 =
                crate::ccast::f64_to_i32(*x.offset(2 as ::core::ffi::c_int as isize));
            (*Divider.offset(k as isize)).flowCurve =
                crate::ccast::f64_to_i32(*x.offset(3 as ::core::ffi::c_int as isize));
            (*Divider.offset(k as isize)).qMin =
                *x.offset(4 as ::core::ffi::c_int as isize) / UCF(FLOW as ::core::ffi::c_int);
            (*Divider.offset(k as isize)).dhMax = *x.offset(5 as ::core::ffi::c_int as isize);
            (*Divider.offset(k as isize)).cWeir = *x.offset(6 as ::core::ffi::c_int as isize);
            (*Node.offset(j as isize)).fullDepth =
                *x.offset(7 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Node.offset(j as isize)).initDepth =
                *x.offset(8 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Node.offset(j as isize)).surDepth =
                *x.offset(9 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Node.offset(j as isize)).pondedArea = *x.offset(10 as ::core::ffi::c_int as isize)
                / (UCF(LENGTH as ::core::ffi::c_int) * UCF(LENGTH as ::core::ffi::c_int));
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn node_validate(mut j: ::core::ffi::c_int) {
    let mut inflow: *mut TDwfInflow = ::core::ptr::null_mut::<TDwfInflow>();
    if (*Node.offset(j as isize)).fullDepth > (*Node.offset(j as isize)).oldDepth
        && (*Node.offset(j as isize)).oldDepth > 0.0f64
    {
        report_writeWarningMsg(
            WARN02.as_ptr() as *mut ::core::ffi::c_char,
            (*Node.offset(j as isize)).ID,
        );
    }
    if (*Node.offset(j as isize)).initDepth
        > (*Node.offset(j as isize)).fullDepth + (*Node.offset(j as isize)).surDepth
    {
        report_writeErrorMsg(
            ERR_NODE_DEPTH as ::core::ffi::c_int,
            (*Node.offset(j as isize)).ID,
        );
    }
    if (*Node.offset(j as isize)).type_0 == STORAGE as ::core::ffi::c_int {
        if node_getVolume(j, (*Node.offset(j as isize)).fullDepth) < 0.0f64 {
            report_writeErrorMsg(
                ERR_STORAGE_VOLUME as ::core::ffi::c_int,
                (*Node.offset(j as isize)).ID,
            );
        }
    }
    if (*Node.offset(j as isize)).type_0 == DIVIDER as ::core::ffi::c_int {
        divider_validate(j);
    }
    inflow = (*Node.offset(j as isize)).dwfInflow;
    while !inflow.is_null() {
        inflow_initDwfInflow(inflow);
        inflow = (*inflow).next as *mut TDwfInflow;
    }
}
#[no_mangle]
pub unsafe extern "C" fn node_initState(mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    (*Node.offset(j as isize)).oldDepth = (*Node.offset(j as isize)).initDepth;
    (*Node.offset(j as isize)).newDepth = (*Node.offset(j as isize)).oldDepth;
    (*Node.offset(j as isize)).crownElev = (*Node.offset(j as isize)).invertElev;
    (*Node.offset(j as isize)).fullVolume = node_getVolume(j, (*Node.offset(j as isize)).fullDepth);
    (*Node.offset(j as isize)).oldVolume = node_getVolume(j, (*Node.offset(j as isize)).oldDepth);
    (*Node.offset(j as isize)).newVolume = (*Node.offset(j as isize)).oldVolume;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        *(*Node.offset(j as isize)).oldQual.offset(p as isize) = 0.0f64;
        *(*Node.offset(j as isize)).newQual.offset(p as isize) = 0.0f64;
        p += 1;
    }
    (*Node.offset(j as isize)).oldLatFlow = 0.0f64;
    (*Node.offset(j as isize)).newLatFlow = 0.0f64;
    (*Node.offset(j as isize)).apiExtInflow = 0.0f64;
    (*Node.offset(j as isize)).losses = 0.0f64;
    if (*Node.offset(j as isize)).type_0 == STORAGE as ::core::ffi::c_int {
        k = (*Node.offset(j as isize)).subIndex;
        (*Storage.offset(k as isize)).hrt = 0.0f64;
        if !(*Storage.offset(k as isize)).exfil.is_null() {
            exfil_initState(k);
        }
    }
    if (*Node.offset(j as isize)).type_0 == OUTFALL as ::core::ffi::c_int {
        k = (*Node.offset(j as isize)).subIndex;
        if (*Outfall.offset(k as isize)).routeTo >= 0 as ::core::ffi::c_int {
            (*Outfall.offset(k as isize)).vRouted = 0.0f64;
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                *(*Outfall.offset(k as isize)).wRouted.offset(p as isize) = 0.0f64;
                p += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn node_setOldHydState(mut j: ::core::ffi::c_int) {
    (*Node.offset(j as isize)).oldDepth = (*Node.offset(j as isize)).newDepth;
    (*Node.offset(j as isize)).oldVolume = (*Node.offset(j as isize)).newVolume;
    (*Node.offset(j as isize)).oldFlowInflow = (*Node.offset(j as isize)).inflow;
    (*Node.offset(j as isize)).oldNetInflow =
        (*Node.offset(j as isize)).inflow - (*Node.offset(j as isize)).outflow;
}
#[no_mangle]
pub unsafe extern "C" fn node_setOldQualState(mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        *(*Node.offset(j as isize)).oldQual.offset(p as isize) =
            *(*Node.offset(j as isize)).newQual.offset(p as isize);
        *(*Node.offset(j as isize)).newQual.offset(p as isize) = 0.0f64;
        p += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn node_initFlows(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
) {
    (*Node.offset(j as isize)).inflow = (*Node.offset(j as isize)).newLatFlow;
    (*Node.offset(j as isize)).outflow = (*Node.offset(j as isize)).losses;
    if (*Node.offset(j as isize)).newVolume > (*Node.offset(j as isize)).fullVolume {
        (*Node.offset(j as isize)).overflow =
            ((*Node.offset(j as isize)).newVolume - (*Node.offset(j as isize)).fullVolume) / tStep;
    } else {
        (*Node.offset(j as isize)).overflow = 0.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn node_getDepth(
    mut j: ::core::ffi::c_int,
    mut v: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    match (*Node.offset(j as isize)).type_0 {
        2 => return storage_getDepth(j, v),
        _ => return 0.0f64,
    };
}
#[no_mangle]
pub unsafe extern "C" fn node_getVolume(
    mut j: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    match (*Node.offset(j as isize)).type_0 {
        2 => return storage_getVolume(j, d),
        _ => {
            if (*Node.offset(j as isize)).fullDepth > 0.0f64 {
                return (*Node.offset(j as isize)).fullVolume
                    * (d / (*Node.offset(j as isize)).fullDepth);
            } else {
                return 0.0f64;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn node_getSurfArea(
    mut j: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    match (*Node.offset(j as isize)).type_0 {
        2 => return storage_getSurfArea(j, d),
        _ => return 0.0f64,
    };
}
#[no_mangle]
pub unsafe extern "C" fn node_getOutflow(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    match (*Node.offset(j as isize)).type_0 {
        3 => return divider_getOutflow(j, k),
        2 => return storage_getOutflow(j, k),
        _ => {
            return (*Node.offset(j as isize)).inflow + (*Node.offset(j as isize)).overflow;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn node_getMaxOutflow(
    mut j: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut qMax: ::core::ffi::c_double = 0.;
    if (*Node.offset(j as isize)).fullVolume > 0.0f64 {
        qMax = (*Node.offset(j as isize)).inflow + (*Node.offset(j as isize)).oldVolume / tStep;
        if q > qMax {
            q = qMax;
        }
    }
    return if 0.0f64 >= q { 0.0f64 } else { q };
}
#[no_mangle]
pub unsafe extern "C" fn node_getSystemOutflow(
    mut j: ::core::ffi::c_int,
    mut isFlooded: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut outflow: ::core::ffi::c_double = 0.0f64;
    *isFlooded = FALSE;
    if (*Node.offset(j as isize)).type_0 == OUTFALL as ::core::ffi::c_int {
        if (*Node.offset(j as isize)).outflow == 0.0f64 {
            outflow = (*Node.offset(j as isize)).inflow;
        } else if (*Node.offset(j as isize)).inflow == 0.0f64 {
            outflow = -(*Node.offset(j as isize)).outflow;
            (*Node.offset(j as isize)).inflow = fabs(outflow);
        }
        (*Node.offset(j as isize)).overflow = 0.0f64;
        (*Node.offset(j as isize)).newVolume = 0.0f64;
    } else if RouteModel != DW as ::core::ffi::c_int
        && (*Node.offset(j as isize)).degree == 0 as ::core::ffi::c_int
        && (*Node.offset(j as isize)).type_0 != STORAGE as ::core::ffi::c_int
    {
        if (*Node.offset(j as isize)).outflow == 0.0f64 {
            outflow = (*Node.offset(j as isize)).inflow;
        }
        (*Node.offset(j as isize)).overflow = 0.0f64;
        (*Node.offset(j as isize)).newVolume = 0.0f64;
    } else {
        if (*Node.offset(j as isize)).newVolume <= (*Node.offset(j as isize)).fullVolume {
            outflow = (*Node.offset(j as isize)).overflow;
        }
        if outflow > 0.0f64 {
            *isFlooded = TRUE;
        }
    }
    return outflow;
}
#[no_mangle]
pub unsafe extern "C" fn node_getResults(
    mut j: ::core::ffi::c_int,
    mut f: ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_float,
) {
    let mut p: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_double = 0.;
    let mut f1: ::core::ffi::c_double = 1.0f64 - f;
    z = (f1 * (*Node.offset(j as isize)).oldDepth + f * (*Node.offset(j as isize)).newDepth)
        * UCF(LENGTH as ::core::ffi::c_int);
    *x.offset(NODE_DEPTH as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
    z = (*Node.offset(j as isize)).invertElev * UCF(LENGTH as ::core::ffi::c_int);
    *x.offset(NODE_HEAD as ::core::ffi::c_int as isize) =
        *x.offset(NODE_DEPTH as ::core::ffi::c_int as isize) + z as ::core::ffi::c_float;
    z = (f1 * (*Node.offset(j as isize)).oldVolume + f * (*Node.offset(j as isize)).newVolume)
        * UCF(VOLUME as ::core::ffi::c_int);
    *x.offset(NODE_VOLUME as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
    z = (f1 * (*Node.offset(j as isize)).oldLatFlow + f * (*Node.offset(j as isize)).newLatFlow)
        * UCF(FLOW as ::core::ffi::c_int);
    *x.offset(NODE_LATFLOW as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
    z = (f1 * (*Node.offset(j as isize)).oldFlowInflow + f * (*Node.offset(j as isize)).inflow)
        * UCF(FLOW as ::core::ffi::c_int);
    *x.offset(NODE_INFLOW as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
    z = (*Node.offset(j as isize)).overflow * UCF(FLOW as ::core::ffi::c_int);
    *x.offset(NODE_OVERFLOW as ::core::ffi::c_int as isize) = z as ::core::ffi::c_float;
    if IgnoreQuality == 0 {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            z = f1 * *(*Node.offset(j as isize)).oldQual.offset(p as isize)
                + f * *(*Node.offset(j as isize)).newQual.offset(p as isize);
            *x.offset((NODE_QUAL as ::core::ffi::c_int + p) as isize) = z as ::core::ffi::c_float;
            p += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn node_setOutletDepth(
    mut j: ::core::ffi::c_int,
    mut yNorm: ::core::ffi::c_double,
    mut yCrit: ::core::ffi::c_double,
    mut z: ::core::ffi::c_double,
) {
    match (*Node.offset(j as isize)).type_0 {
        2 => return,
        1 => {
            outfall_setOutletDepth(j, yNorm, yCrit, z);
        }
        _ => {
            if z > 0.0f64 {
                (*Node.offset(j as isize)).newDepth = 0.0f64;
            } else {
                (*Node.offset(j as isize)).newDepth = if yNorm <= yCrit { yNorm } else { yCrit };
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn node_getPondedArea(
    mut j: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a: ::core::ffi::c_double = 0.;
    if d <= (*Node.offset(j as isize)).fullDepth || (*Node.offset(j as isize)).pondedArea == 0.0f64
    {
        return node_getSurfArea(j, d);
    }
    d = d - (*Node.offset(j as isize)).fullDepth;
    a = (*Node.offset(j as isize)).pondedArea;
    if a <= 0.0f64 {
        a = node_getSurfArea(j, (*Node.offset(j as isize)).fullDepth);
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn node_getLosses(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if (*Node.offset(j as isize)).type_0 == STORAGE as ::core::ffi::c_int {
        return storage_getLosses(j, tStep);
    } else {
        return 0.0f64;
    };
}
unsafe extern "C" fn junc_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        NODE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    i = 1 as ::core::ffi::c_int;
    while i <= 5 as ::core::ffi::c_int {
        x[(i - 1 as ::core::ffi::c_int) as usize] = 0.0f64;
        if i < ntoks {
            if getDouble(
                *tok.offset(i as isize),
                (&raw mut x as *mut ::core::ffi::c_double)
                    .offset((i - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(i as isize),
                );
            }
        }
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= 4 as ::core::ffi::c_int {
        if x[i as usize] < 0.0f64 {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
            );
        }
        i += 1;
    }
    let ref mut fresh5 = (*Node.offset(j as isize)).ID;
    *fresh5 = id;
    node_setParams(
        j,
        JUNCTION as ::core::ffi::c_int,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn storage_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 10] = [0.; 10];
    let mut y: [::core::ffi::c_double; 3] = [0.; 3];
    let mut A: ::core::ffi::c_double = 0.;
    let mut B: ::core::ffi::c_double = 0.;
    let mut L: ::core::ffi::c_double = 0.;
    let mut W: ::core::ffi::c_double = 0.;
    let mut Z: ::core::ffi::c_double = 0.;
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        NODE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    i = 1 as ::core::ffi::c_int;
    while i <= 3 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(i as isize));
        }
        i += 1;
    }
    m = findmatch(
        *tok.offset(4 as ::core::ffi::c_int as isize),
        &raw mut RelationWords as *mut *mut ::core::ffi::c_char,
    );
    if m < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(4 as ::core::ffi::c_int as isize),
        );
    }
    x[3 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    x[4 as ::core::ffi::c_int as usize] = 0.0f64;
    x[5 as ::core::ffi::c_int as usize] = 0.0f64;
    x[6 as ::core::ffi::c_int as usize] = 0.0f64;
    x[7 as ::core::ffi::c_int as usize] = -1.0f64;
    x[8 as ::core::ffi::c_int as usize] = 0.0f64;
    x[9 as ::core::ffi::c_int as usize] = 0.0f64;
    if m == TABULAR as ::core::ffi::c_int {
        i = project_findObject(
            CURVE as ::core::ffi::c_int,
            *tok.offset(5 as ::core::ffi::c_int as isize),
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(5 as ::core::ffi::c_int as isize),
            );
        }
        x[7 as ::core::ffi::c_int as usize] = i as ::core::ffi::c_double;
        n = 6 as ::core::ffi::c_int;
    } else {
        if ntoks < 8 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = 5 as ::core::ffi::c_int;
        while i <= 7 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(i as isize),
                (&raw mut y as *mut ::core::ffi::c_double)
                    .offset((i - 5 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(i as isize),
                );
            }
            i += 1;
        }
        n = 8 as ::core::ffi::c_int;
    }
    match m {
        1 => {
            if y[2 as ::core::ffi::c_int as usize] < 0.0f64 {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(7 as ::core::ffi::c_int as isize),
                );
            }
        }
        2 | 3 | 4 | 5 => {
            if y[0 as ::core::ffi::c_int as usize] <= 0.0f64 {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(5 as ::core::ffi::c_int as isize),
                );
            }
            if y[1 as ::core::ffi::c_int as usize] <= 0.0f64 {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(6 as ::core::ffi::c_int as isize),
                );
            }
            if y[2 as ::core::ffi::c_int as usize] < 0.0f64 {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(7 as ::core::ffi::c_int as isize),
                );
            }
        }
        _ => {}
    }
    if m == PARABOLOID as ::core::ffi::c_int && y[2 as ::core::ffi::c_int as usize] == 0.0f64 {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(7 as ::core::ffi::c_int as isize),
        );
    }
    match m {
        1 => {
            x[4 as ::core::ffi::c_int as usize] = y[0 as ::core::ffi::c_int as usize];
            x[5 as ::core::ffi::c_int as usize] = y[1 as ::core::ffi::c_int as usize];
            x[6 as ::core::ffi::c_int as usize] = y[2 as ::core::ffi::c_int as usize];
        }
        2 => {
            A = y[0 as ::core::ffi::c_int as usize] / 2.0f64;
            B = y[1 as ::core::ffi::c_int as usize] / 2.0f64;
            x[4 as ::core::ffi::c_int as usize] = 0.0f64;
            x[5 as ::core::ffi::c_int as usize] = 0.0f64;
            x[6 as ::core::ffi::c_int as usize] = PI * A * B;
        }
        3 => {
            A = y[0 as ::core::ffi::c_int as usize] / 2.0f64;
            B = y[1 as ::core::ffi::c_int as usize] / 2.0f64;
            Z = y[2 as ::core::ffi::c_int as usize];
            x[4 as ::core::ffi::c_int as usize] = 2.0f64 * PI * B * Z;
            x[5 as ::core::ffi::c_int as usize] = PI * B / A * Z * Z;
            x[6 as ::core::ffi::c_int as usize] = PI * A * B;
        }
        4 => {
            A = y[0 as ::core::ffi::c_int as usize] / 2.0f64;
            B = y[1 as ::core::ffi::c_int as usize] / 2.0f64;
            Z = y[2 as ::core::ffi::c_int as usize];
            x[4 as ::core::ffi::c_int as usize] = PI * A * B / Z;
            x[5 as ::core::ffi::c_int as usize] = 0.0f64;
            x[6 as ::core::ffi::c_int as usize] = 0.0f64;
        }
        5 => {
            L = y[0 as ::core::ffi::c_int as usize];
            W = y[1 as ::core::ffi::c_int as usize];
            Z = y[2 as ::core::ffi::c_int as usize];
            x[4 as ::core::ffi::c_int as usize] = 2.0f64 * (L + W) * Z;
            x[5 as ::core::ffi::c_int as usize] = 4.0f64 * Z * Z;
            x[6 as ::core::ffi::c_int as usize] = L * W;
        }
        _ => {}
    }
    if ntoks > n {
        if getDouble(
            *tok.offset(n as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(8 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(n as isize));
        }
        n += 1;
    }
    if ntoks > n {
        if getDouble(
            *tok.offset(n as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(9 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(n as isize));
        }
        n += 1;
    }
    let ref mut fresh3 = (*Node.offset(j as isize)).ID;
    *fresh3 = id;
    node_setParams(
        j,
        STORAGE as ::core::ffi::c_int,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    if ntoks > n {
        return exfil_readStorageParams(k, tok, ntoks, n);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn storage_getDepth(
    mut j: ::core::ffi::c_int,
    mut v: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = (*Node.offset(j as isize)).subIndex;
    let mut i: ::core::ffi::c_int = (*Storage.offset(k as isize)).aCurve;
    let mut shape: ::core::ffi::c_int = (*Storage.offset(k as isize)).shape;
    let mut a0: ::core::ffi::c_double = (*Storage.offset(k as isize)).a0;
    let mut a1: ::core::ffi::c_double = (*Storage.offset(k as isize)).a1;
    let mut a2: ::core::ffi::c_double = (*Storage.offset(k as isize)).a2;
    let mut d: ::core::ffi::c_double = 0.;
    let mut e: ::core::ffi::c_double = 0.;
    let mut storageVol: TStorageVol = TStorageVol { k: 0, v: 0. };
    if (*Node.offset(j as isize)).fullVolume > 0.0f64 && v >= (*Node.offset(j as isize)).fullVolume
    {
        return (*Node.offset(j as isize)).fullDepth;
    }
    if v == 0.0f64 {
        return 0.0f64;
    }
    v *= UCF(VOLUME as ::core::ffi::c_int);
    storageVol.k = k;
    storageVol.v = v;
    match shape {
        0 => {
            i = (*Storage.offset(k as isize)).aCurve;
            if i >= 0 as ::core::ffi::c_int {
                d = table_getStorageDepth(Curve.offset(i as isize) as *mut TTable, v);
            } else {
                d = 0.0f64;
            }
        }
        2 => {
            d = v / a0;
        }
        4 => {
            d = sqrt(2.0f64 * v / a1);
        }
        1 => {
            if a2 == 0.0f64 {
                d = v / (a0 + a1);
            } else if a0 == 0.0f64 {
                e = 1.0f64 / (a2 + 1.0f64);
                d = pow(v / (a1 * e), e);
            } else if a2 == 1.0f64 && a1 > 0.0f64 {
                d = (sqrt(a0 * a0 + 2.0f64 * a1 * v) - a0) / a1;
            } else {
                d = v / (a0 + a1);
                findroot_Newton(
                    0.0f64,
                    (*Node.offset(j as isize)).fullDepth * UCF(LENGTH as ::core::ffi::c_int),
                    &raw mut d,
                    0.001f64,
                    Some(
                        storage_getVolDiff
                            as unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                *mut ::core::ffi::c_double,
                                *mut ::core::ffi::c_double,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    &raw mut storageVol as *mut ::core::ffi::c_void,
                );
            }
        }
        3 | 5 => {
            d = v / a0;
            findroot_Newton(
                0.0f64,
                (*Node.offset(j as isize)).fullDepth * UCF(LENGTH as ::core::ffi::c_int),
                &raw mut d,
                0.001f64,
                Some(
                    storage_getVolDiff
                        as unsafe extern "C" fn(
                            ::core::ffi::c_double,
                            *mut ::core::ffi::c_double,
                            *mut ::core::ffi::c_double,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                &raw mut storageVol as *mut ::core::ffi::c_void,
            );
        }
        _ => {
            d = 0.0f64;
        }
    }
    d /= UCF(LENGTH as ::core::ffi::c_int);
    if d > (*Node.offset(j as isize)).fullDepth {
        d = (*Node.offset(j as isize)).fullDepth;
    }
    return d;
}
unsafe extern "C" fn storage_getVolDiff(
    mut y: ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
    mut df: *mut ::core::ffi::c_double,
    mut p: *mut ::core::ffi::c_void,
) {
    let mut k: ::core::ffi::c_int = 0;
    let mut storageVol: *mut TStorageVol = ::core::ptr::null_mut::<TStorageVol>();
    storageVol = p as *mut TStorageVol;
    k = (*storageVol).k;
    *f = storage_getVolume(k, y) - (*storageVol).v;
    *df = storage_getSurfArea(k, y);
}
unsafe extern "C" fn storage_getVolume(
    mut j: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = (*Node.offset(j as isize)).subIndex;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    if d == 0.0f64 {
        return 0.0f64;
    }
    if d >= (*Node.offset(j as isize)).fullDepth && (*Node.offset(j as isize)).fullVolume > 0.0f64 {
        return (*Node.offset(j as isize)).fullVolume;
    }
    match (*Storage.offset(k as isize)).shape {
        0 => {
            i = (*Storage.offset(k as isize)).aCurve;
            if i >= 0 as ::core::ffi::c_int {
                return table_getStorageVolume(
                    Curve.offset(i as isize) as *mut TTable,
                    d * UCF(LENGTH as ::core::ffi::c_int),
                ) / UCF(VOLUME as ::core::ffi::c_int);
            } else {
                return 0.0f64;
            }
        }
        1 => {
            d *= UCF(LENGTH as ::core::ffi::c_int);
            n = (*Storage.offset(k as isize)).a2 + 1.0f64;
            v = (*Storage.offset(k as isize)).a0 * d
                + (*Storage.offset(k as isize)).a1 / n * pow(d, n);
            return v / UCF(VOLUME as ::core::ffi::c_int);
        }
        2 | 3 | 4 | 5 => {
            d *= UCF(LENGTH as ::core::ffi::c_int);
            v = d
                * ((*Storage.offset(k as isize)).a0
                    + d * ((*Storage.offset(k as isize)).a1 / 2.0f64
                        + d * (*Storage.offset(k as isize)).a2 / 3.0f64));
            return v / UCF(VOLUME as ::core::ffi::c_int);
        }
        _ => return 0.0f64,
    };
}
unsafe extern "C" fn storage_getSurfArea(
    mut j: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut area: ::core::ffi::c_double = 0.0f64;
    let mut k: ::core::ffi::c_int = (*Node.offset(j as isize)).subIndex;
    let mut i: ::core::ffi::c_int = 0;
    match (*Storage.offset(k as isize)).shape {
        0 => {
            i = (*Storage.offset(k as isize)).aCurve;
            if i >= 0 as ::core::ffi::c_int {
                area = table_lookupEx(
                    Curve.offset(i as isize) as *mut TTable,
                    d * UCF(LENGTH as ::core::ffi::c_int),
                );
            }
        }
        1 => {
            area = (*Storage.offset(k as isize)).a0
                + (*Storage.offset(k as isize)).a1
                    * pow(
                        d * UCF(LENGTH as ::core::ffi::c_int),
                        (*Storage.offset(k as isize)).a2,
                    );
        }
        2 | 3 | 4 | 5 => {
            d *= UCF(LENGTH as ::core::ffi::c_int);
            area = (*Storage.offset(k as isize)).a0
                + d * ((*Storage.offset(k as isize)).a1 + d * (*Storage.offset(k as isize)).a2);
        }
        _ => return 0.0f64,
    }
    return area / UCF(LENGTH as ::core::ffi::c_int) / UCF(LENGTH as ::core::ffi::c_int);
}
unsafe extern "C" fn storage_getOutflow(
    mut j: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    if (*Link.offset(i as isize)).type_0 != CONDUIT as ::core::ffi::c_int {
        return 0.0f64;
    }
    y = (*Node.offset(j as isize)).newDepth - (*Link.offset(i as isize)).offset1;
    if y <= 0.0f64 {
        return 0.0f64;
    }
    if y >= (*Link.offset(i as isize)).xsect.yFull {
        return (*Link.offset(i as isize)).qFull;
    }
    k = (*Link.offset(i as isize)).subIndex;
    a = xsect_getAofY(&raw mut (*Link.offset(i as isize)).xsect, y);
    return (*Conduit.offset(k as isize)).beta
        * xsect_getSofA(&raw mut (*Link.offset(i as isize)).xsect, a);
}
unsafe extern "C" fn storage_getLosses(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut depth: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = 0.;
    let mut evapRate: ::core::ffi::c_double = 0.0f64;
    let mut exfilRate: ::core::ffi::c_double = 0.0f64;
    let mut totalLoss: ::core::ffi::c_double = 0.0f64;
    let mut lossRatio: ::core::ffi::c_double = 0.;
    let mut exfil: *mut TExfil = ::core::ptr::null_mut::<TExfil>();
    k = (*Node.offset(j as isize)).subIndex;
    evapRate = Evap.rate * (*Storage.offset(k as isize)).fEvap;
    exfil = (*Storage.offset(k as isize)).exfil;
    if evapRate > 0.0f64 || !exfil.is_null() {
        depth = (*Node.offset(j as isize)).newDepth;
        area = storage_getSurfArea(j, depth);
        if (*Node.offset(j as isize)).newVolume > FUDGE {
            evapRate = area * evapRate;
        }
        if !exfil.is_null() {
            exfilRate = exfil_getLoss(exfil, tStep, depth, area);
        }
        totalLoss = (evapRate + exfilRate) * tStep;
        if totalLoss > (*Node.offset(j as isize)).newVolume {
            lossRatio = (*Node.offset(j as isize)).newVolume / totalLoss;
            evapRate *= lossRatio;
            exfilRate *= lossRatio;
        }
    }
    (*Storage.offset((*Node.offset(j as isize)).subIndex as isize)).evapLoss = evapRate * tStep;
    (*Storage.offset((*Node.offset(j as isize)).subIndex as isize)).exfilLoss = exfilRate * tStep;
    return evapRate + exfilRate;
}
unsafe extern "C" fn divider_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut m1: ::core::ffi::c_int = 0;
    let mut m2: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 11] = [0.; 11];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 4 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        NODE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    i = 1 as ::core::ffi::c_int;
    while i < 11 as ::core::ffi::c_int {
        x[i as usize] = 0.0f64;
        i += 1;
    }
    if strlen(*tok.offset(2 as ::core::ffi::c_int as isize)) == 0 as size_t
        || strcmp(
            *tok.offset(2 as ::core::ffi::c_int as isize),
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        x[1 as ::core::ffi::c_int as usize] = -1.0f64;
    } else {
        m1 = project_findObject(
            LINK as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
        if m1 < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        x[1 as ::core::ffi::c_int as usize] = m1 as ::core::ffi::c_double;
    }
    n = 4 as ::core::ffi::c_int;
    m1 = findmatch(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        &raw mut DividerTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if m1 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    x[2 as ::core::ffi::c_int as usize] = m1 as ::core::ffi::c_double;
    x[3 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    if m1 == TABULAR_DIVIDER as ::core::ffi::c_int {
        if ntoks < 5 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        m2 = project_findObject(
            CURVE as ::core::ffi::c_int,
            *tok.offset(4 as ::core::ffi::c_int as isize),
        );
        if m2 < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(4 as ::core::ffi::c_int as isize),
            );
        }
        x[3 as ::core::ffi::c_int as usize] = m2 as ::core::ffi::c_double;
        n = 5 as ::core::ffi::c_int;
    }
    if m1 == CUTOFF_DIVIDER as ::core::ffi::c_int {
        if ntoks < 5 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if getDouble(
            *tok.offset(4 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(4 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(4 as ::core::ffi::c_int as isize),
            );
        }
        n = 5 as ::core::ffi::c_int;
    }
    if m1 == WEIR_DIVIDER as ::core::ffi::c_int {
        if ntoks < 7 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        i = 4 as ::core::ffi::c_int;
        while i < 7 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(i as isize),
                (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(i as isize),
                );
            }
            i += 1;
        }
        n = 7 as ::core::ffi::c_int;
    }
    if m1 == OVERFLOW_DIVIDER as ::core::ffi::c_int {
        n = 4 as ::core::ffi::c_int;
    }
    m = 7 as ::core::ffi::c_int;
    i = n;
    while i < ntoks && m < 11 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(m as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(i as isize));
        }
        m += 1;
        i += 1;
    }
    let ref mut fresh0 = (*Node.offset(j as isize)).ID;
    *fresh0 = id;
    node_setParams(
        j,
        DIVIDER as ::core::ffi::c_int,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn divider_validate(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    k = (*Node.offset(j as isize)).subIndex;
    i = (*Divider.offset(k as isize)).link;
    if i < 0 as ::core::ffi::c_int
        || (*Link.offset(i as isize)).node1 != j && (*Link.offset(i as isize)).node2 != j
    {
        report_writeErrorMsg(
            ERR_DIVIDER_LINK as ::core::ffi::c_int,
            (*Node.offset(j as isize)).ID,
        );
    }
    if (*Divider.offset(k as isize)).type_0 == WEIR_DIVIDER as ::core::ffi::c_int {
        if (*Divider.offset(k as isize)).dhMax <= 0.0f64
            || (*Divider.offset(k as isize)).cWeir <= 0.0f64
        {
            report_writeErrorMsg(
                ERR_WEIR_DIVIDER as ::core::ffi::c_int,
                (*Node.offset(j as isize)).ID,
            );
        } else {
            (*Divider.offset(k as isize)).qMax = (*Divider.offset(k as isize)).cWeir
                * pow((*Divider.offset(k as isize)).dhMax, 1.5f64)
                / UCF(FLOW as ::core::ffi::c_int);
            if (*Divider.offset(k as isize)).qMin > (*Divider.offset(k as isize)).qMax {
                report_writeErrorMsg(
                    ERR_WEIR_DIVIDER as ::core::ffi::c_int,
                    (*Node.offset(j as isize)).ID,
                );
            }
        }
    }
}
unsafe extern "C" fn divider_getOutflow(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut qIn: ::core::ffi::c_double = 0.;
    let mut qOut: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    qIn = (*Node.offset(j as isize)).inflow + (*Node.offset(j as isize)).overflow;
    i = (*Node.offset(j as isize)).subIndex;
    match (*Divider.offset(i as isize)).type_0 {
        0 => {
            if qIn <= (*Divider.offset(i as isize)).qMin {
                qOut = 0.0f64;
            } else {
                qOut = qIn - (*Divider.offset(i as isize)).qMin;
            }
        }
        3 => {
            if k != (*Divider.offset(i as isize)).link {
                qOut = qIn;
            } else {
                qOut = qIn - (*Node.offset(j as isize)).outflow;
            }
            if qOut < FLOW_TOL {
                qOut = 0.0f64;
            }
            return qOut;
        }
        2 => {
            if qIn <= (*Divider.offset(i as isize)).qMin {
                qOut = 0.0f64;
            } else {
                f = (qIn - (*Divider.offset(i as isize)).qMin)
                    / ((*Divider.offset(i as isize)).qMax - (*Divider.offset(i as isize)).qMin);
                if f > 1.0f64 {
                    qOut = (*Divider.offset(i as isize)).qMax * sqrt(f);
                } else {
                    qOut = (*Divider.offset(i as isize)).cWeir
                        * pow(f * (*Divider.offset(i as isize)).dhMax, 1.5f64)
                        / UCF(FLOW as ::core::ffi::c_int);
                }
            }
        }
        1 => {
            m = (*Divider.offset(i as isize)).flowCurve;
            if m >= 0 as ::core::ffi::c_int {
                qOut = table_lookup(
                    Curve.offset(m as isize) as *mut TTable,
                    qIn * UCF(FLOW as ::core::ffi::c_int),
                ) / UCF(FLOW as ::core::ffi::c_int);
            } else {
                qOut = 0.0f64;
            }
        }
        _ => {
            qOut = 0.0f64;
        }
    }
    if qOut > qIn {
        qOut = qIn;
    }
    if k != (*Divider.offset(i as isize)).link {
        qOut = qIn - qOut;
    }
    return qOut;
}
unsafe extern "C" fn outfall_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 7] = [0.; 7];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        NODE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    i = findmatch(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        &raw mut OutfallTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if i < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    x[1 as ::core::ffi::c_int as usize] = i as ::core::ffi::c_double;
    x[2 as ::core::ffi::c_int as usize] = 0.0f64;
    x[3 as ::core::ffi::c_int as usize] = -1.0f64;
    x[4 as ::core::ffi::c_int as usize] = -1.0f64;
    x[5 as ::core::ffi::c_int as usize] = 0.0f64;
    x[6 as ::core::ffi::c_int as usize] = -1.0f64;
    n = 4 as ::core::ffi::c_int;
    if i >= FIXED_OUTFALL as ::core::ffi::c_int {
        if ntoks < 4 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        n = 5 as ::core::ffi::c_int;
        match i {
            2 => {
                if getDouble(
                    *tok.offset(3 as ::core::ffi::c_int as isize),
                    (&raw mut x as *mut ::core::ffi::c_double)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset(3 as ::core::ffi::c_int as isize),
                    );
                }
            }
            3 => {
                m = project_findObject(
                    CURVE as ::core::ffi::c_int,
                    *tok.offset(3 as ::core::ffi::c_int as isize),
                );
                if m < 0 as ::core::ffi::c_int {
                    return error_setInpError(
                        ERR_NAME as ::core::ffi::c_int,
                        *tok.offset(3 as ::core::ffi::c_int as isize),
                    );
                }
                x[3 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
            }
            4 => {
                m = project_findObject(
                    TSERIES as ::core::ffi::c_int,
                    *tok.offset(3 as ::core::ffi::c_int as isize),
                );
                if m < 0 as ::core::ffi::c_int {
                    return error_setInpError(
                        ERR_NAME as ::core::ffi::c_int,
                        *tok.offset(3 as ::core::ffi::c_int as isize),
                    );
                }
                x[4 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
                (*Tseries.offset(m as isize)).refersTo = TIMESERIES_OUTFALL as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    if ntoks == n {
        m = findmatch(
            *tok.offset((n - 1 as ::core::ffi::c_int) as isize),
            &raw mut NoYesWords as *mut *mut ::core::ffi::c_char,
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset((n - 1 as ::core::ffi::c_int) as isize),
            );
        }
        x[5 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    }
    if ntoks == n + 1 as ::core::ffi::c_int {
        m = project_findObject(SUBCATCH as ::core::ffi::c_int, *tok.offset(n as isize));
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(ERR_NAME as ::core::ffi::c_int, *tok.offset(n as isize));
        }
        x[6 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    }
    let ref mut fresh4 = (*Node.offset(j as isize)).ID;
    *fresh4 = id;
    node_setParams(
        j,
        OUTFALL as ::core::ffi::c_int,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn outfall_setOutletDepth(
    mut j: ::core::ffi::c_int,
    mut yNorm: ::core::ffi::c_double,
    mut yCrit: ::core::ffi::c_double,
    mut z: ::core::ffi::c_double,
) {
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut yNew: ::core::ffi::c_double = 0.;
    let mut stage: ::core::ffi::c_double = 0.;
    let mut k: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = (*Node.offset(j as isize)).subIndex;
    let mut currentDate: DateTime = 0.;
    match (*Outfall.offset(i as isize)).type_0 {
        0 => {
            if z > 0.0f64 {
                (*Node.offset(j as isize)).newDepth = 0.0f64;
            } else {
                (*Node.offset(j as isize)).newDepth = if yNorm <= yCrit { yNorm } else { yCrit };
            }
            return;
        }
        1 => {
            if z > 0.0f64 {
                (*Node.offset(j as isize)).newDepth = 0.0f64;
            } else {
                (*Node.offset(j as isize)).newDepth = yNorm;
            }
            return;
        }
        2 => {
            stage = (*Outfall.offset(i as isize)).fixedStage;
        }
        3 => {
            k = (*Outfall.offset(i as isize)).tideCurve;
            table_getFirstEntry(
                Curve.offset(k as isize) as *mut TTable,
                &raw mut x,
                &raw mut y,
            );
            currentDate = (NewRoutingTime / MSECperDAY) as DateTime;
            x += (currentDate as ::core::ffi::c_double
                - floor(currentDate as ::core::ffi::c_double))
                * 24.0f64;
            stage = table_lookup(Curve.offset(k as isize) as *mut TTable, x)
                / UCF(LENGTH as ::core::ffi::c_int);
        }
        4 => {
            k = (*Outfall.offset(i as isize)).stageSeries;
            currentDate =
                (StartDateTime as ::core::ffi::c_double + NewRoutingTime / MSECperDAY) as DateTime;
            stage = table_tseriesLookup(
                Tseries.offset(k as isize) as *mut TTable,
                currentDate as ::core::ffi::c_double,
                TRUE as ::core::ffi::c_char,
            ) / UCF(LENGTH as ::core::ffi::c_int);
        }
        _ => {
            stage = (*Node.offset(j as isize)).invertElev;
        }
    }
    yCrit = if yCrit <= yNorm { yCrit } else { yNorm };
    if yCrit + z + (*Node.offset(j as isize)).invertElev < stage {
        yNew = stage - (*Node.offset(j as isize)).invertElev;
    } else if z > 0.0f64 {
        if stage < (*Node.offset(j as isize)).invertElev + z {
            yNew = if 0.0f64 >= stage - (*Node.offset(j as isize)).invertElev {
                0.0f64
            } else {
                stage - (*Node.offset(j as isize)).invertElev
            };
        } else {
            yNew = z + yCrit;
        }
    } else {
        yNew = yCrit;
    }
    (*Node.offset(j as isize)).newDepth = yNew;
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
