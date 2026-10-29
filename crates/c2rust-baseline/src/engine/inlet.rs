#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut LinkStats: *mut TLinkStats;
    static mut Frpt: TFile;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut UnitSystem: ::core::ffi::c_int;
    static mut RouteModel: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut FlowUnits: ::core::ffi::c_int;
    static mut ReportStart: DateTime;
    static mut NewRoutingTime: ::core::ffi::c_double;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Curve: *mut TTable;
    static mut Street: *mut TStreet;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn report_writeLine(line: *const ::core::ffi::c_char);
    fn report_writeWarningMsg(msg: *mut ::core::ffi::c_char, id: *mut ::core::ffi::c_char);
    fn xsect_getYofA(xsect_0: *mut TXsect, area: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getAofS(xsect_0: *mut TXsect, sFactor: ::core::ffi::c_double)
        -> ::core::ffi::c_double;
    fn xsect_getWofY(xsect_0: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn table_lookupEx(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getInt(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut FlowUnitWords: [*mut ::core::ffi::c_char; 0];
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
pub type RouteModelType = ::core::ffi::c_uint;
pub const DW: RouteModelType = 4;
pub const EKW: RouteModelType = 3;
pub const KW: RouteModelType = 2;
pub const SF: RouteModelType = 1;
pub const NO_ROUTING: RouteModelType = 0;
pub type CurveType = ::core::ffi::c_uint;
pub const PUMP5_CURVE: CurveType = 11;
pub const PUMP4_CURVE: CurveType = 10;
pub const PUMP3_CURVE: CurveType = 9;
pub const PUMP2_CURVE: CurveType = 8;
pub const PUMP1_CURVE: CurveType = 7;
pub const WEIR_CURVE: CurveType = 6;
pub const SHAPE_CURVE: CurveType = 5;
pub const CONTROL_CURVE: CurveType = 4;
pub const RATING_CURVE: CurveType = 3;
pub const TIDAL_CURVE: CurveType = 2;
pub const DIVERSION_CURVE: CurveType = 1;
pub const STORAGE_CURVE: CurveType = 0;
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
pub struct TInlet {
    pub linkIndex: ::core::ffi::c_int,
    pub designIndex: ::core::ffi::c_int,
    pub nodeIndex: ::core::ffi::c_int,
    pub numInlets: ::core::ffi::c_int,
    pub placement: ::core::ffi::c_int,
    pub clogFactor: ::core::ffi::c_double,
    pub flowLimit: ::core::ffi::c_double,
    pub localDepress: ::core::ffi::c_double,
    pub localWidth: ::core::ffi::c_double,
    pub flowFactor: ::core::ffi::c_double,
    pub flowCapture: ::core::ffi::c_double,
    pub backflow: ::core::ffi::c_double,
    pub backflowRatio: ::core::ffi::c_double,
    pub stats: TInletStats,
    pub nextInlet: *mut TInlet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TInletStats {
    pub flowPeriods: ::core::ffi::c_int,
    pub capturePeriods: ::core::ffi::c_int,
    pub backflowPeriods: ::core::ffi::c_int,
    pub peakFlow: ::core::ffi::c_double,
    pub peakFlowCapture: ::core::ffi::c_double,
    pub avgFlowCapture: ::core::ffi::c_double,
    pub bypassFreq: ::core::ffi::c_double,
}
pub const CUSTOM_INLET: InletType = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TInletDesign {
    pub ID: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub grateInlet: TGrateInlet,
    pub slottedInlet: TSlottedInlet,
    pub curbInlet: TCurbInlet,
    pub customCurve: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TCurbInlet {
    pub length: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
    pub throatAngle: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSlottedInlet {
    pub length: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TGrateInlet {
    pub type_0: ::core::ffi::c_int,
    pub length: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub fracOpenArea: ::core::ffi::c_double,
    pub splashVeloc: ::core::ffi::c_double,
}
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_NUMBER: ErrorType = 211;
pub const SLOTTED_INLET: InletType = 3;
pub const COMBO_INLET: InletType = 2;
pub const CURB_INLET: InletType = 1;
pub const VERTICAL_THROAT: ThroatAngleType = 2;
pub const DROP_CURB_INLET: InletType = 5;
pub const GRATE_INLET: InletType = 0;
pub const GENERIC: GrateType = 7;
pub const DROP_GRATE_INLET: InletType = 4;
pub const AUTOMATIC: InletPlacementType = 0;
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
pub struct TInletNode {
    pub numInletLinks: ::core::ffi::c_int,
    pub numStdInletLinks: ::core::ffi::c_int,
    pub numCustomInlets: ::core::ffi::c_int,
    pub totalInletArea: ::core::ffi::c_double,
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
pub struct TStreet {
    pub ID: *mut ::core::ffi::c_char,
    pub sides: ::core::ffi::c_int,
    pub slope: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub curbHeight: ::core::ffi::c_double,
    pub gutterDepression: ::core::ffi::c_double,
    pub gutterWidth: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub backSlope: ::core::ffi::c_double,
    pub backWidth: ::core::ffi::c_double,
    pub backRoughness: ::core::ffi::c_double,
    pub transect: TTransect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTransect {
    pub ID: *mut ::core::ffi::c_char,
    pub yFull: ::core::ffi::c_double,
    pub aFull: ::core::ffi::c_double,
    pub rFull: ::core::ffi::c_double,
    pub wMax: ::core::ffi::c_double,
    pub ywMax: ::core::ffi::c_double,
    pub sMax: ::core::ffi::c_double,
    pub aMax: ::core::ffi::c_double,
    pub lengthFactor: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub areaTbl: [::core::ffi::c_double; 51],
    pub hradTbl: [::core::ffi::c_double; 51],
    pub widthTbl: [::core::ffi::c_double; 51],
    pub nTbl: ::core::ffi::c_int,
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
pub struct TTreatment {
    pub treatType: ::core::ffi::c_int,
    pub equation: *mut MathExpr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TRdiiInflow {
    pub unitHyd: ::core::ffi::c_int,
    pub area: ::core::ffi::c_double,
}
pub type TDwfInflow = DwfInflow;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DwfInflow {
    pub param: ::core::ffi::c_int,
    pub avgValue: ::core::ffi::c_double,
    pub patterns: [::core::ffi::c_int; 4],
    pub next: *mut DwfInflow,
}
pub type TExtInflow = ExtInflow;
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
pub struct TFile {
    pub name: [::core::ffi::c_char; 260],
    pub mode: ::core::ffi::c_char,
    pub state: ::core::ffi::c_char,
    pub file: *mut FILE,
}
pub type TTableEntry = TableEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableEntry {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub next: *mut TableEntry,
}
pub const ON_SAG: InletPlacementType = 2;
pub const ON_GRADE: InletPlacementType = 1;
pub const INCLINED_THROAT: ThroatAngleType = 1;
pub const HORIZONTAL_THROAT: ThroatAngleType = 0;
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
pub type InletType = ::core::ffi::c_uint;
pub type GrateType = ::core::ffi::c_uint;
pub const RETICULINE: GrateType = 6;
pub const TILT_BAR_30: GrateType = 5;
pub const TILT_BAR_45: GrateType = 4;
pub const CURVED_VANE: GrateType = 3;
pub const P30: GrateType = 2;
pub const P50x100: GrateType = 1;
pub const P50: GrateType = 0;
pub type InletPlacementType = ::core::ffi::c_uint;
pub type ThroatAngleType = ::core::ffi::c_uint;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BIG: ::core::ffi::c_double = 1.0E10f64;
pub const MIN_RUNOFF_FLOW: ::core::ffi::c_double = 0.001f64;
pub const FUDGE: ::core::ffi::c_double = 0.0001f64;
pub const WARN12: [::core::ffi::c_char; 63] = unsafe {
    ::core::mem::transmute::<[u8; 63], [::core::ffi::c_char; 63]>(
        *b"WARNING 12: inlet removed due to unsupported shape for Conduit\0",
    )
};
#[no_mangle]
pub static mut InletDesigns: *mut TInletDesign =
    ::core::ptr::null::<TInletDesign>() as *mut TInletDesign;
#[no_mangle]
pub static mut InletDesignCount: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut UsesInlets: ::core::ffi::c_int = 0;
static mut InletTypeWords: [*mut ::core::ffi::c_char; 8] = [
    b"GRATE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"CURB\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SLOTTED\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"DROP_GRATE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"DROP_CURB\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"CUSTOM\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut GrateTypeWords: [*mut ::core::ffi::c_char; 9] = [
    b"P_BAR-50\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"P_BAR-50x100\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"P_BAR-30\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"CURVED_VANE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TILT_BAR-45\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TILT_BAR-30\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"RETICULINE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"GENERIC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut ThroatAngleWords: [*mut ::core::ffi::c_char; 4] = [
    b"HORIZONTAL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"INCLINED\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"VERTICAL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut PlacementTypeWords: [*mut ::core::ffi::c_char; 3] = [
    b"AUTOMATIC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ON_GRADE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ON_SAG\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
static mut SplashCoeffs: [[::core::ffi::c_double; 4]; 7] = [
    [2.22f64, 4.03f64, 0.65f64, 0.06f64],
    [0.74f64, 2.44f64, 0.27f64, 0.02f64],
    [1.76f64, 3.12f64, 0.45f64, 0.03f64],
    [0.30f64, 4.85f64, 1.31f64, 0.15f64],
    [0.99f64, 2.64f64, 0.36f64, 0.03f64],
    [0.51f64, 2.34f64, 0.2f64, 0.01f64],
    [0.28f64, 2.28f64, 0.18f64, 0.01f64],
];
static mut GrateOpeningRatios: [::core::ffi::c_double; 8] = [
    0.90f64, 0.80f64, 0.60f64, 0.35f64, 0.17f64, 0.34f64, 0.80f64, 1.00f64,
];
static mut Sx: ::core::ffi::c_double = 0.;
static mut SL: ::core::ffi::c_double = 0.;
static mut Sw: ::core::ffi::c_double = 0.;
static mut a: ::core::ffi::c_double = 0.;
static mut W: ::core::ffi::c_double = 0.;
static mut T: ::core::ffi::c_double = 0.;
static mut n: ::core::ffi::c_double = 0.;
static mut Nsides: ::core::ffi::c_int = 0;
static mut Tcrown: ::core::ffi::c_double = 0.;
static mut Beta: ::core::ffi::c_double = 0.;
static mut Qfactor: ::core::ffi::c_double = 0.;
static mut xsect: *mut TXsect = ::core::ptr::null::<TXsect>() as *mut TXsect;
static mut InletFlow: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
static mut FirstInlet: *mut TInlet = ::core::ptr::null::<TInlet>() as *mut TInlet;
#[no_mangle]
pub unsafe extern "C" fn inlet_create(mut numInlets: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    InletDesigns = ::core::ptr::null_mut::<TInletDesign>();
    InletFlow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    InletDesignCount = 0 as ::core::ffi::c_int;
    UsesInlets = FALSE;
    FirstInlet = ::core::ptr::null_mut::<TInlet>();
    InletDesigns = calloc(
        numInlets as size_t,
        ::core::mem::size_of::<TInletDesign>() as size_t,
    ) as *mut TInletDesign;
    if InletDesigns.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    InletDesignCount = numInlets;
    InletFlow = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    ) as *mut ::core::ffi::c_double;
    if InletFlow.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < InletDesignCount {
        (*InletDesigns.offset(i as isize)).customCurve = -(1 as ::core::ffi::c_int);
        (*InletDesigns.offset(i as isize)).curbInlet.length = 0.0f64;
        (*InletDesigns.offset(i as isize)).grateInlet.length = 0.0f64;
        (*InletDesigns.offset(i as isize)).slottedInlet.length = 0.0f64;
        (*InletDesigns.offset(i as isize)).type_0 = CUSTOM_INLET as ::core::ffi::c_int;
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inlet_delete() {
    let mut inlet: *mut TInlet = FirstInlet;
    let mut nextInlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    while !inlet.is_null() {
        nextInlet = (*inlet).nextInlet;
        free(inlet as *mut ::core::ffi::c_void);
        inlet = nextInlet;
    }
    FirstInlet = ::core::ptr::null_mut::<TInlet>();
    if !InletFlow.is_null() {
        free(InletFlow as *mut ::core::ffi::c_void);
        InletFlow = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !InletDesigns.is_null() {
        free(InletDesigns as *mut ::core::ffi::c_void);
        InletDesigns = ::core::ptr::null_mut::<TInletDesign>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn inlet_readDesignParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = project_findObject(
        INLET as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if i < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    let ref mut fresh0 = (*InletDesigns.offset(i as isize)).ID;
    *fresh0 = project_findID(
        INLET as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    (*InletDesigns.offset(i as isize)).type_0 = findmatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut InletTypeWords as *mut *mut ::core::ffi::c_char,
    );
    match (*InletDesigns.offset(i as isize)).type_0 {
        0 | 4 => return readGrateInletParams(i, tok, ntoks),
        1 | 5 => return readCurbInletParams(i, tok, ntoks),
        3 => return readSlottedInletParams(i, tok, ntoks),
        6 => return readCustomInletParams(i, tok, ntoks),
        _ => {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn inlet_readUsageParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut linkIndex: ::core::ffi::c_int = 0;
    let mut designIndex: ::core::ffi::c_int = 0;
    let mut nodeIndex: ::core::ffi::c_int = 0;
    let mut numInlets: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut placement: ::core::ffi::c_int = AUTOMATIC as ::core::ffi::c_int;
    let mut flowLimit: ::core::ffi::c_double = 0.0f64;
    let mut pctClogged: ::core::ffi::c_double = 0.0f64;
    let mut aLocal: ::core::ffi::c_double = 0.0f64;
    let mut wLocal: ::core::ffi::c_double = 0.0f64;
    let mut inlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    linkIndex = project_findObject(
        LINK as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if linkIndex < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    designIndex = project_findObject(
        INLET as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if designIndex < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    nodeIndex = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    if nodeIndex < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if ntoks > 3 as ::core::ffi::c_int {
        if getInt(
            *tok.offset(3 as ::core::ffi::c_int as isize),
            &raw mut numInlets,
        ) == 0
            || numInlets < 1 as ::core::ffi::c_int
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(3 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks > 4 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(4 as ::core::ffi::c_int as isize),
            &raw mut pctClogged,
        ) == 0
            || pctClogged < 0.0f64
            || pctClogged > 99.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(4 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks > 5 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(5 as ::core::ffi::c_int as isize),
            &raw mut flowLimit,
        ) == 0
            || flowLimit < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(5 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks > 6 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(6 as ::core::ffi::c_int as isize),
            &raw mut aLocal,
        ) == 0
            || aLocal < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks > 7 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(7 as ::core::ffi::c_int as isize),
            &raw mut wLocal,
        ) == 0
            || wLocal < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(7 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks > 8 as ::core::ffi::c_int {
        placement = findmatch(
            *tok.offset(8 as ::core::ffi::c_int as isize),
            &raw mut PlacementTypeWords as *mut *mut ::core::ffi::c_char,
        );
        if placement < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(8 as ::core::ffi::c_int as isize),
            );
        }
    }
    inlet = (*Link.offset(linkIndex as isize)).inlet;
    if inlet.is_null() {
        inlet = malloc(::core::mem::size_of::<TInlet>() as size_t) as *mut TInlet;
        if inlet.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        let ref mut fresh1 = (*Link.offset(linkIndex as isize)).inlet;
        *fresh1 = inlet;
        (*inlet).nextInlet = FirstInlet;
        FirstInlet = inlet;
    }
    (*inlet).linkIndex = linkIndex;
    (*inlet).designIndex = designIndex;
    (*inlet).nodeIndex = nodeIndex;
    (*inlet).numInlets = numInlets;
    (*inlet).placement = placement;
    (*inlet).clogFactor = 1.0f64 - pctClogged / 100.0f64;
    (*inlet).flowLimit = flowLimit / UCF(FLOW as ::core::ffi::c_int);
    (*inlet).localDepress = aLocal / UCF(LENGTH as ::core::ffi::c_int);
    (*inlet).localWidth = wLocal / UCF(LENGTH as ::core::ffi::c_int);
    (*inlet).flowFactor = 0.0f64;
    (*inlet).backflowRatio = 0.0f64;
    initInletStats(inlet);
    UsesInlets = TRUE;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inlet_validate() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut inletType: ::core::ffi::c_int = 0;
    let mut inletValid: ::core::ffi::c_int = 0;
    let mut inlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    let mut prevInlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    if UsesInlets == 0 {
        return;
    }
    prevInlet = FirstInlet;
    inlet = FirstInlet;
    while !inlet.is_null() {
        inletValid = FALSE;
        i = (*inlet).linkIndex;
        xsect = &raw mut (*Link.offset(i as isize)).xsect;
        inletType = (*InletDesigns.offset((*inlet).designIndex as isize)).type_0;
        if inletType == CUSTOM_INLET as ::core::ffi::c_int {
            j = (*InletDesigns.offset((*inlet).designIndex as isize)).customCurve;
            if j >= 0 as ::core::ffi::c_int {
                if (*Curve.offset(j as isize)).curveType == DIVERSION_CURVE as ::core::ffi::c_int
                    || (*Curve.offset(j as isize)).curveType == RATING_CURVE as ::core::ffi::c_int
                {
                    inletValid = TRUE;
                }
            }
        } else if ((*xsect).type_0 == TRAPEZOIDAL as ::core::ffi::c_int
            || (*xsect).type_0 == RECT_OPEN as ::core::ffi::c_int)
            && (inletType == DROP_GRATE_INLET as ::core::ffi::c_int
                || inletType == DROP_CURB_INLET as ::core::ffi::c_int)
        {
            inletValid = TRUE;
        } else if (*xsect).type_0 == STREET_XSECT as ::core::ffi::c_int
            && inletType != DROP_GRATE_INLET as ::core::ffi::c_int
            && inletType != DROP_CURB_INLET as ::core::ffi::c_int
        {
            inletValid = TRUE;
        }
        if inletValid != 0 {
            (*Node.offset((*Link.offset(i as isize)).node2 as isize)).inlet =
                BYPASS as ::core::ffi::c_int;
            (*Node.offset((*inlet).nodeIndex as isize)).inlet = CAPTURE as ::core::ffi::c_int;
            (*inlet).backflow = 0.0f64;
            getConduitGeometry(inlet);
            (*inlet).flowFactor = 0.56f64 / n * pow(SL, 0.5f64) * pow(Sx, 1.67f64);
            prevInlet = inlet;
            inlet = (*inlet).nextInlet;
        } else {
            report_writeWarningMsg(
                WARN12.as_ptr() as *mut ::core::ffi::c_char,
                (*Link.offset(i as isize)).ID,
            );
            if inlet == FirstInlet {
                FirstInlet = (*inlet).nextInlet;
                prevInlet = FirstInlet;
                free(inlet as *mut ::core::ffi::c_void);
                inlet = FirstInlet;
            } else {
                (*prevInlet).nextInlet = (*inlet).nextInlet;
                free(inlet as *mut ::core::ffi::c_void);
                inlet = (*prevInlet).nextInlet;
            }
            let ref mut fresh2 = (*Link.offset(i as isize)).inlet;
            *fresh2 = ::core::ptr::null_mut::<TInlet>();
        }
    }
    getBackflowRatios();
}
#[no_mangle]
pub unsafe extern "C" fn inlet_findCapturedFlows(mut tStep: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut placement: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut inlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    if UsesInlets == 0 {
        return;
    }
    memset(
        InletFlow as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (Nobjects[NODE as ::core::ffi::c_int as usize] as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
    );
    if RouteModel != DW as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            (*Node.offset(j as isize)).inflow = if 0.0f64 >= (*Node.offset(j as isize)).newLatFlow {
                0.0f64
            } else {
                (*Node.offset(j as isize)).newLatFlow
            };
            j += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
            (*Node.offset((*Link.offset(i as isize)).node2 as isize)).inflow +=
                if 0.0f64 >= (*Link.offset(i as isize)).newFlow {
                    0.0f64
                } else {
                    (*Link.offset(i as isize)).newFlow
                };
            i += 1;
        }
    }
    inlet = FirstInlet;
    while !inlet.is_null() {
        i = (*inlet).linkIndex;
        j = (*Link.offset(i as isize)).node2;
        m = (*inlet).nodeIndex;
        placement = getInletPlacement(inlet, j);
        if (*InletDesigns.offset((*inlet).designIndex as isize)).type_0
            == CUSTOM_INLET as ::core::ffi::c_int
        {
            q = fabs((*Link.offset(i as isize)).newFlow);
            (*inlet).flowCapture =
                getCustomCapturedFlow(inlet, q, (*Node.offset(j as isize)).newDepth);
        } else if placement == ON_GRADE as ::core::ffi::c_int {
            q = fabs((*Link.offset(i as isize)).newFlow);
            (*inlet).flowCapture =
                getOnGradeCapturedFlow(inlet, q, (*Node.offset(j as isize)).newDepth);
        } else {
            q = (*Node.offset(j as isize)).inflow;
            (*inlet).flowCapture =
                getOnSagCapturedFlow(inlet, q, (*Node.offset(j as isize)).newDepth);
        }
        if fabs((*inlet).flowCapture) < FUDGE {
            (*inlet).flowCapture = 0.0f64;
        }
        *InletFlow.offset(j as isize) += (*inlet).flowCapture;
        (*inlet).backflow = (*Node.offset(m as isize)).overflow * (*inlet).backflowRatio;
        if fabs((*inlet).backflow) < FUDGE {
            (*inlet).backflow = 0.0f64;
        }
        inlet = (*inlet).nextInlet;
    }
    inlet = FirstInlet;
    while !inlet.is_null() {
        i = (*inlet).linkIndex;
        j = (*Link.offset(i as isize)).node2;
        m = (*inlet).nodeIndex;
        if RouteModel != DW as ::core::ffi::c_int
            && getInletPlacement(inlet, j) == ON_SAG as ::core::ffi::c_int
        {
            q = (*Node.offset(j as isize)).newVolume / tStep;
            q += if (*Node.offset(j as isize)).inflow >= 0.0f64 {
                (*Node.offset(j as isize)).inflow
            } else {
                0.0f64
            };
            if *InletFlow.offset(j as isize) > q {
                (*inlet).flowCapture *= q / *InletFlow.offset(j as isize);
            }
        }
        (*Node.offset(j as isize)).newLatFlow -= (*inlet).flowCapture - (*inlet).backflow;
        (*Node.offset(m as isize)).newLatFlow += (*inlet).flowCapture;
        if getDateTime(NewRoutingTime) > ReportStart {
            updateInletStats(inlet, fabs((*Link.offset(i as isize)).newFlow));
        }
        inlet = (*inlet).nextInlet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn inlet_adjustQualInflows() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut qNet: ::core::ffi::c_double = 0.;
    let mut inlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    if UsesInlets == 0 {
        return;
    }
    if IgnoreQuality != 0
        || Nobjects[POLLUT as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
    {
        return;
    }
    inlet = FirstInlet;
    while !inlet.is_null() {
        i = (*inlet).linkIndex;
        j = (*Link.offset(i as isize)).node2;
        m = (*inlet).nodeIndex;
        qNet = (*inlet).flowCapture - (*inlet).backflow;
        if qNet > 0.0f64 {
            (*Node.offset(m as isize)).qualInflow += qNet;
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                *(*Node.offset(m as isize)).newQual.offset(p as isize) +=
                    qNet * *(*Node.offset(j as isize)).oldQual.offset(p as isize);
                p += 1;
            }
        } else {
            qNet = -qNet;
            (*Node.offset(j as isize)).qualInflow += qNet;
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                *(*Node.offset(j as isize)).newQual.offset(p as isize) +=
                    qNet * *(*Node.offset(m as isize)).oldQual.offset(p as isize);
                p += 1;
            }
        }
        inlet = (*inlet).nextInlet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn inlet_adjustQualOutflows() {
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut inlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    extern "C" {
        static mut StepFlowTotals: TRoutingTotals;
    }
    extern "C" {
        static mut StepQualTotals: *mut TRoutingTotals;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if (*Node.offset(j as isize)).inlet == CAPTURE as ::core::ffi::c_int {
            q = (*Node.offset(j as isize)).overflow;
            if q > 0.0f64 {
                StepFlowTotals.flooding -= q;
                if IgnoreQuality == 0 {
                    p = 0 as ::core::ffi::c_int;
                    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                        w = q * *(*Node.offset(j as isize)).newQual.offset(p as isize);
                        (*StepQualTotals.offset(p as isize)).flooding -= w;
                        p += 1;
                    }
                }
            }
        }
        j += 1;
    }
    if IgnoreQuality == 0
        && Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
    {
        inlet = FirstInlet;
        while !inlet.is_null() {
            j = (*Link.offset((*inlet).linkIndex as isize)).node2;
            q = (*inlet).flowCapture - (*inlet).backflow;
            if q > 0.0f64 && (*Node.offset(j as isize)).newLatFlow < 0.0f64 {
                p = 0 as ::core::ffi::c_int;
                while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                    w = q * *(*Node.offset(j as isize)).newQual.offset(p as isize);
                    (*StepQualTotals.offset(p as isize)).outflow -= w;
                    p += 1;
                }
            }
            inlet = (*inlet).nextInlet;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn inlet_writeStatsReport() {
    let mut j: ::core::ffi::c_int = 0;
    let mut header: ::core::ffi::c_int = FALSE;
    if Nobjects[STREET as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        return;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(j as isize)).xsect.type_0 == STREET_XSECT as ::core::ffi::c_int {
            if header == 0 {
                writeStreetStatsHeader();
                header = TRUE;
            }
            writeStreetStats(j);
        }
        j += 1;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn inlet_capturedFlow(mut i: ::core::ffi::c_int) -> ::core::ffi::c_double {
    if !(*Link.offset(i as isize)).inlet.is_null() {
        return (*(*Link.offset(i as isize)).inlet).flowCapture;
    }
    return 0.0f64;
}
unsafe extern "C" fn readGrateInletParams(
    mut i: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut grateType: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_double = 0.;
    let mut length: ::core::ffi::c_double = 0.;
    let mut areaRatio: ::core::ffi::c_double = 0.0f64;
    let mut vSplash: ::core::ffi::c_double = 0.0f64;
    if ntoks < 5 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if getDouble(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        &raw mut length,
    ) == 0
        || length <= 0.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        &raw mut width,
    ) == 0
        || width <= 0.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    grateType = findmatch(
        *tok.offset(4 as ::core::ffi::c_int as isize),
        &raw mut GrateTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if grateType < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(4 as ::core::ffi::c_int as isize),
        );
    }
    if grateType == GENERIC as ::core::ffi::c_int {
        if ntoks < 6 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if getDouble(
            *tok.offset(5 as ::core::ffi::c_int as isize),
            &raw mut areaRatio,
        ) == 0
            || areaRatio <= 0.0f64
            || areaRatio > 1.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(5 as ::core::ffi::c_int as isize),
            );
        }
        if ntoks > 6 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(6 as ::core::ffi::c_int as isize),
                &raw mut vSplash,
            ) == 0
                || vSplash < 0.0f64
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(6 as ::core::ffi::c_int as isize),
                );
            }
        }
    }
    (*InletDesigns.offset(i as isize)).grateInlet.length =
        length / UCF(LENGTH as ::core::ffi::c_int);
    (*InletDesigns.offset(i as isize)).grateInlet.width = width / UCF(LENGTH as ::core::ffi::c_int);
    (*InletDesigns.offset(i as isize)).grateInlet.type_0 = grateType;
    (*InletDesigns.offset(i as isize)).grateInlet.fracOpenArea = areaRatio;
    (*InletDesigns.offset(i as isize)).grateInlet.splashVeloc =
        vSplash / UCF(LENGTH as ::core::ffi::c_int);
    if (*InletDesigns.offset(i as isize)).type_0 == GRATE_INLET as ::core::ffi::c_int
        && (*InletDesigns.offset(i as isize)).curbInlet.length > 0.0f64
    {
        (*InletDesigns.offset(i as isize)).type_0 = COMBO_INLET as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readCurbInletParams(
    mut i: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut throatAngle: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_double = 0.;
    let mut length: ::core::ffi::c_double = 0.;
    if ntoks < 4 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if getDouble(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        &raw mut length,
    ) == 0
        || length <= 0.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        &raw mut height,
    ) == 0
        || height <= 0.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    throatAngle = VERTICAL_THROAT as ::core::ffi::c_int;
    if (*InletDesigns.offset(i as isize)).type_0 == CURB_INLET as ::core::ffi::c_int
        && ntoks > 4 as ::core::ffi::c_int
    {
        throatAngle = findmatch(
            *tok.offset(4 as ::core::ffi::c_int as isize),
            &raw mut ThroatAngleWords as *mut *mut ::core::ffi::c_char,
        );
        if throatAngle < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(4 as ::core::ffi::c_int as isize),
            );
        }
    }
    (*InletDesigns.offset(i as isize)).curbInlet.length =
        length / UCF(LENGTH as ::core::ffi::c_int);
    (*InletDesigns.offset(i as isize)).curbInlet.height =
        height / UCF(LENGTH as ::core::ffi::c_int);
    (*InletDesigns.offset(i as isize)).curbInlet.throatAngle = throatAngle;
    if (*InletDesigns.offset(i as isize)).type_0 == CURB_INLET as ::core::ffi::c_int
        && (*InletDesigns.offset(i as isize)).grateInlet.length > 0.0f64
    {
        (*InletDesigns.offset(i as isize)).type_0 = COMBO_INLET as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readSlottedInletParams(
    mut i: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut width: ::core::ffi::c_double = 0.;
    let mut length: ::core::ffi::c_double = 0.;
    if ntoks < 4 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if getDouble(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        &raw mut length,
    ) == 0
        || length <= 0.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        &raw mut width,
    ) == 0
        || width <= 0.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    (*InletDesigns.offset(i as isize)).slottedInlet.length =
        length / UCF(LENGTH as ::core::ffi::c_int);
    (*InletDesigns.offset(i as isize)).slottedInlet.width =
        width / UCF(LENGTH as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readCustomInletParams(
    mut i: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        c = project_findObject(
            CURVE as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
        if c < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
    }
    (*InletDesigns.offset(i as isize)).customCurve = c;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn initInletStats(mut inlet: *mut TInlet) {
    if !inlet.is_null() {
        (*inlet).flowCapture = 0.0f64;
        (*inlet).backflow = 0.0f64;
        (*inlet).stats.flowPeriods = 0 as ::core::ffi::c_int;
        (*inlet).stats.capturePeriods = 0 as ::core::ffi::c_int;
        (*inlet).stats.backflowPeriods = 0 as ::core::ffi::c_int;
        (*inlet).stats.peakFlow = 0.0f64;
        (*inlet).stats.peakFlowCapture = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*inlet).stats.avgFlowCapture = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*inlet).stats.bypassFreq = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
}
unsafe extern "C" fn updateInletStats(mut inlet: *mut TInlet, mut q: ::core::ffi::c_double) {
    let mut qCapture: ::core::ffi::c_double = (*inlet).flowCapture;
    let mut qBackflow: ::core::ffi::c_double = (*inlet).backflow;
    let mut qNet: ::core::ffi::c_double = qCapture - qBackflow;
    let mut qBypass: ::core::ffi::c_double = q - qNet;
    let mut fCapture: ::core::ffi::c_double = 0.0f64;
    if q < MIN_RUNOFF_FLOW && qBackflow <= 0.0f64 {
        return;
    }
    (*inlet).stats.flowPeriods += 1;
    if qNet > 0.0f64 {
        (*inlet).stats.capturePeriods += 1;
        fCapture = qNet / q;
        fCapture = if fCapture <= 1.0f64 { fCapture } else { 1.0f64 };
        (*inlet).stats.avgFlowCapture += fCapture;
        if qBypass > MIN_RUNOFF_FLOW {
            (*inlet).stats.bypassFreq += 1.;
        }
    } else {
        (*inlet).stats.backflowPeriods += 1;
    }
    if q > (*inlet).stats.peakFlow {
        (*inlet).stats.peakFlow = q;
        (*inlet).stats.peakFlowCapture = fCapture * 100.0f64;
    }
}
unsafe extern "C" fn writeStreetStatsHeader() {
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*******************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Street Flow Summary\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*******************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  ---------------------------------------------------------------------------------------------------------------------------------------\n                                                                                        Peak     Avg.   Bypass     Back     Peak     Peak\n                        Peak   Maximum   Maximum                                        Flow     Flow     Flow     Flow  Capture   Bypass\n                        Flow    Spread     Depth  Inlet             Inlet     Inlet  Capture  Capture     Freq     Freq  / Inlet     Flow\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    if UnitSystem == US as ::core::ffi::c_int {
        fprintf(
            Frpt.file,
            b"\n  Street Conduit         %3s        ft        ft  Design            Location  Count     Pcnt     Pcnt     Pcnt     Pcnt      %3s      %3s\0"
                as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
        );
    } else {
        fprintf(
            Frpt.file,
            b"\n  Street Conduit         %3s         m         m  Design            Location            Pcnt     Pcnt     Pcnt     Pcnt      %3s      %3s\0"
                as *const u8 as *const ::core::ffi::c_char,
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
            *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char)
                .offset(FlowUnits as isize),
        );
    }
    fprintf(
        Frpt.file,
        b"\n  ---------------------------------------------------------------------------------------------------------------------------------------\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn writeStreetStats(mut link: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_int = 0;
    let mut placement: ::core::ffi::c_int = 0;
    let mut maxSpread: ::core::ffi::c_double = 0.;
    let mut maxDepth: ::core::ffi::c_double = 0.;
    let mut maxFlow: ::core::ffi::c_double = 0.;
    let mut fp: ::core::ffi::c_double = 0.;
    let mut cp: ::core::ffi::c_double = 0.;
    let mut afc: ::core::ffi::c_double = 0.0f64;
    let mut bpf: ::core::ffi::c_double = 0.0f64;
    let mut inlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    k = (*Link.offset(link as isize)).subIndex;
    t = (*Link.offset(link as isize)).xsect.transect;
    inlet = (*Link.offset(link as isize)).inlet;
    maxFlow = (*LinkStats.offset(link as isize)).maxFlow;
    maxDepth = (*LinkStats.offset(link as isize)).maxDepth;
    maxSpread = xsect_getWofY(&raw mut (*Link.offset(link as isize)).xsect, maxDepth)
        / (*Street.offset(t as isize)).sides as ::core::ffi::c_double;
    maxSpread = if maxSpread <= (*Street.offset(t as isize)).width {
        maxSpread
    } else {
        (*Street.offset(t as isize)).width
    };
    fprintf(
        Frpt.file,
        b"\n  %-16s\0" as *const u8 as *const ::core::ffi::c_char,
        (*Link.offset(link as isize)).ID,
    );
    fprintf(
        Frpt.file,
        b" %9.3f\0" as *const u8 as *const ::core::ffi::c_char,
        maxFlow * UCF(FLOW as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b" %9.3f\0" as *const u8 as *const ::core::ffi::c_char,
        maxSpread * UCF(LENGTH as ::core::ffi::c_int),
    );
    fprintf(
        Frpt.file,
        b" %9.3f\0" as *const u8 as *const ::core::ffi::c_char,
        maxDepth * UCF(LENGTH as ::core::ffi::c_int),
    );
    if !inlet.is_null() {
        fprintf(
            Frpt.file,
            b"  %-16s\0" as *const u8 as *const ::core::ffi::c_char,
            (*InletDesigns.offset((*inlet).designIndex as isize)).ID,
        );
        placement = getInletPlacement(inlet, (*Link.offset((*inlet).linkIndex as isize)).node2);
        if placement == ON_GRADE as ::core::ffi::c_int {
            fprintf(
                Frpt.file,
                b"  ON-GRADE\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            fprintf(
                Frpt.file,
                b"  ON-SAG  \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        fprintf(
            Frpt.file,
            b"  %5d\0" as *const u8 as *const ::core::ffi::c_char,
            (*inlet).numInlets,
        );
        fp = (*inlet).stats.flowPeriods as ::core::ffi::c_double / 100.0f64;
        if fp > 0.0f64 {
            cp = (*inlet).stats.capturePeriods as ::core::ffi::c_double / 100.0f64;
            fprintf(
                Frpt.file,
                b"  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
                (*inlet).stats.peakFlowCapture,
            );
            if cp > 0.0f64 {
                afc = (*inlet).stats.avgFlowCapture / cp;
                bpf = (*inlet).stats.bypassFreq / cp;
            }
            fprintf(
                Frpt.file,
                b"  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
                afc,
            );
            fprintf(
                Frpt.file,
                b"  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
                bpf,
            );
            fprintf(
                Frpt.file,
                b"  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
                (*inlet).stats.backflowPeriods as ::core::ffi::c_double / fp,
            );
            fprintf(
                Frpt.file,
                b"  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
                maxFlow / (*Street.offset(t as isize)).sides as ::core::ffi::c_double
                    * UCF(FLOW as ::core::ffi::c_int)
                    * 0.01f64
                    * (*inlet).stats.peakFlowCapture
                    / (*inlet).numInlets as ::core::ffi::c_double,
            );
            fprintf(
                Frpt.file,
                b"  %7.2f\0" as *const u8 as *const ::core::ffi::c_char,
                maxFlow
                    * UCF(FLOW as ::core::ffi::c_int)
                    * 0.01f64
                    * (100.0f64 - (*inlet).stats.peakFlowCapture),
            );
        }
    }
}
unsafe extern "C" fn getInletPlacement(
    mut inlet: *mut TInlet,
    mut j: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*inlet).placement == AUTOMATIC as ::core::ffi::c_int {
        if (*Node.offset(j as isize)).degree > 0 as ::core::ffi::c_int {
            return ON_GRADE as ::core::ffi::c_int;
        } else {
            return ON_SAG as ::core::ffi::c_int;
        }
    } else {
        return (*inlet).placement;
    };
}
unsafe extern "C" fn getConduitGeometry(mut inlet: *mut TInlet) {
    let mut linkIndex: ::core::ffi::c_int = (*inlet).linkIndex;
    let mut t: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = (*Link.offset(linkIndex as isize)).subIndex;
    SL = (*Conduit.offset(k as isize)).slope;
    Beta = (*Conduit.offset(k as isize)).beta;
    xsect = &raw mut (*Link.offset(linkIndex as isize)).xsect;
    if (*xsect).type_0 == STREET_XSECT as ::core::ffi::c_int {
        t = (*xsect).transect;
        Sx = (*Street.offset(t as isize)).slope;
        a = (*Street.offset(t as isize)).gutterDepression;
        W = (*Street.offset(t as isize)).gutterWidth;
        n = (*Street.offset(t as isize)).roughness;
        Nsides = (*Street.offset(t as isize)).sides;
        Tcrown = (*Street.offset(t as isize)).width;
        Qfactor = (*inlet).flowFactor;
        if !inlet.is_null()
            && (*inlet).localDepress * (*inlet).localWidth
                > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            a += (*inlet).localDepress;
            W = (*inlet).localWidth;
        }
        if W * a > 0.0f64 {
            Sw = Sx + a / W;
        } else {
            Sw = Sx;
        }
    } else {
        a = 0.0f64;
        W = 0.0f64;
        n = (*Conduit.offset(k as isize)).roughness;
        Nsides = 1 as ::core::ffi::c_int;
        Sx = 0.01f64;
        Sw = Sx;
    };
}
unsafe extern "C" fn getFlowSpread(mut Q: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut iter: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut f1: ::core::ffi::c_double = 0.;
    let mut Sr: ::core::ffi::c_double = 0.;
    let mut Ts1: ::core::ffi::c_double = 0.;
    let mut Ts2: ::core::ffi::c_double = 0.;
    let mut Tw: ::core::ffi::c_double = 0.;
    let mut Qs: ::core::ffi::c_double = 0.;
    let mut Eo: ::core::ffi::c_double = 0.;
    f = Qfactor;
    if a == 0.0f64 {
        Ts1 = pow(Q / f, 0.375f64);
    } else {
        f1 = f * pow(a / W / Sx, 1.67f64);
        Tw = pow(Q / f1, 0.375f64);
        if Tw <= W {
            Ts1 = Tw;
        } else {
            Sr = (Sx + a / W) / Sx;
            iter = 1 as ::core::ffi::c_int;
            Ts1 = pow(Q / f, 0.375f64) - W;
            if Ts1 <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                Ts1 = Tw - W;
            }
            while iter < 11 as ::core::ffi::c_int {
                Eo = getEo(Sr, Ts1, W);
                Qs = (1.0f64 - Eo) * Q;
                Ts2 = pow(Qs / f, 0.375f64);
                if fabs(Ts2 - Ts1) < 0.01f64 {
                    break;
                }
                Ts1 = Ts2;
                iter += 1;
            }
            Ts1 = Ts2 + W;
        }
    }
    return if Ts1 <= Tcrown { Ts1 } else { Tcrown };
}
unsafe extern "C" fn getEo(
    mut Sr: ::core::ffi::c_double,
    mut Ts: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut x: ::core::ffi::c_double = 0.;
    x = Sr / (Ts / w);
    x = pow(1.0f64 + x, 2.67f64) - 1.0f64;
    x = 1.0f64 + Sr / x;
    return 1.0f64 / x;
}
unsafe extern "C" fn getOnGradeCapturedFlow(
    mut inlet: *mut TInlet,
    mut q: ::core::ffi::c_double,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut linkIndex: ::core::ffi::c_int = 0;
    let mut qApproach: ::core::ffi::c_double = 0.;
    let mut qc: ::core::ffi::c_double = 0.;
    let mut qCaptured: ::core::ffi::c_double = 0.;
    let mut qBypassed: ::core::ffi::c_double = 0.;
    let mut qMax: ::core::ffi::c_double = 0.;
    if (*inlet).numInlets == 0 as ::core::ffi::c_int {
        return 0.0f64;
    }
    linkIndex = (*inlet).linkIndex;
    qApproach = q;
    if qApproach < MIN_RUNOFF_FLOW {
        return 0.0f64;
    }
    getConduitGeometry(inlet);
    qApproach /= Nsides as ::core::ffi::c_double;
    qBypassed = qApproach;
    qCaptured = 0.0f64;
    qMax = BIG;
    if (*inlet).flowLimit > 0.0f64 {
        qMax = (*inlet).flowLimit;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= (*inlet).numInlets {
        qc = getOnGradeInletCapture((*inlet).designIndex, qBypassed, d) * (*inlet).clogFactor;
        qc = if qc <= qMax { qc } else { qMax };
        qc = if qc <= qBypassed { qc } else { qBypassed };
        qCaptured += qc;
        qBypassed -= qc;
        if qBypassed < MIN_RUNOFF_FLOW {
            break;
        }
        i += 1;
    }
    qCaptured *= Nsides as ::core::ffi::c_double;
    return qCaptured;
}
unsafe extern "C" fn getOnGradeInletCapture(
    mut i: ::core::ffi::c_int,
    mut Q: ::core::ffi::c_double,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut Q1: ::core::ffi::c_double = Q;
    let mut Qc: ::core::ffi::c_double = 0.0f64;
    let mut Lsweep: ::core::ffi::c_double = 0.0f64;
    let mut Lcurb: ::core::ffi::c_double = 0.0f64;
    let mut Lgrate: ::core::ffi::c_double = 0.0f64;
    if (*InletDesigns.offset(i as isize)).type_0 == DROP_CURB_INLET as ::core::ffi::c_int {
        Qc = getOnSagInletCapture(i, d);
        return if Qc <= Q { Qc } else { Q };
    }
    if (*InletDesigns.offset(i as isize)).type_0 == DROP_GRATE_INLET as ::core::ffi::c_int {
        Qc = getGrateInletCapture(i, Q);
        return if Qc <= Q { Qc } else { Q };
    }
    T = getFlowSpread(Q);
    if (*InletDesigns.offset(i as isize)).type_0 == SLOTTED_INLET as ::core::ffi::c_int {
        Qc = getCurbInletCapture(Q, (*InletDesigns.offset(i as isize)).slottedInlet.length);
        return if Qc <= Q { Qc } else { Q };
    }
    Lcurb = (*InletDesigns.offset(i as isize)).curbInlet.length;
    Lgrate = (*InletDesigns.offset(i as isize)).grateInlet.length;
    if Lcurb > 0.0f64 {
        Lsweep = Lcurb - Lgrate;
        if Lsweep > 0.0f64 {
            Qc = getCurbInletCapture(Q1, Lsweep);
            Q1 -= Qc;
        }
    }
    if Lgrate > 0.0f64 && Q1 > 0.0f64 {
        if Q1 != Q {
            T = getFlowSpread(Q1);
        }
        Qc += getGrateInletCapture(i, Q1);
    }
    return Qc;
}
unsafe extern "C" fn getGrateInletCapture(
    mut i: ::core::ffi::c_int,
    mut Q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut grateType: ::core::ffi::c_int = 0;
    let mut Lg: ::core::ffi::c_double = 0.;
    let mut Wg: ::core::ffi::c_double = 0.;
    let mut A: ::core::ffi::c_double = 0.;
    let mut Y: ::core::ffi::c_double = 0.;
    let mut Eo: ::core::ffi::c_double = 0.;
    let mut V: ::core::ffi::c_double = 0.;
    let mut Vo: ::core::ffi::c_double = 0.;
    let mut Qo: ::core::ffi::c_double = Q;
    let mut Rf: ::core::ffi::c_double = 1.0f64;
    let mut Rs: ::core::ffi::c_double = 0.0f64;
    Lg = (*InletDesigns.offset(i as isize)).grateInlet.length;
    Wg = (*InletDesigns.offset(i as isize)).grateInlet.width;
    if (*xsect).type_0 == TRAPEZOIDAL as ::core::ffi::c_int
        || (*xsect).type_0 == RECT_OPEN as ::core::ffi::c_int
    {
        A = xsect_getAofS(xsect, Q / Beta);
        Y = xsect_getYofA(xsect, A);
        T = xsect_getWofY(xsect, Y);
        Eo = Beta * pow(Y * Wg, 1.67f64)
            / pow(
                Wg + 2 as ::core::ffi::c_int as ::core::ffi::c_double * Y,
                0.67f64,
            )
            / Q;
        if Wg > 0.99f64 * (*xsect).yBot
            && (*xsect).type_0 == TRAPEZOIDAL as ::core::ffi::c_int
            && (*xsect).sBot > 0.0f64
        {
            Wg = (*xsect).yBot;
            Sx = 1.0f64 / (*xsect).sBot;
        }
    } else if a == 0.0f64 {
        A = T * T * Sx / 2.0f64;
        Eo = getGutterFlowRatio(Wg);
        if T >= Tcrown {
            Qo = Qfactor * pow(Tcrown, 2.67f64);
        }
    } else {
        if T <= W {
            A = T * T * Sw / 2.0f64;
        } else {
            A = (T * T * Sx + a * W) / 2.0f64;
        }
        Eo = getGutterFlowRatio(W);
        if Eo < 1.0f64 {
            if T >= Tcrown {
                Qo = Qfactor * pow(Tcrown, 2.67f64) / (1.0f64 - Eo);
            }
            Eo = Eo * getGutterAreaRatio(Wg, A);
        }
    }
    V = Qo / A;
    grateType = (*InletDesigns.offset(i as isize)).grateInlet.type_0;
    if grateType < 0 as ::core::ffi::c_int || grateType == GENERIC as ::core::ffi::c_int {
        Vo = (*InletDesigns.offset(i as isize)).grateInlet.splashVeloc;
    } else {
        Vo = getSplashOverVelocity(grateType, Lg);
    }
    if V > Vo {
        Rf = 1.0f64 - 0.09f64 * (V - Vo);
    }
    if Eo < 1.0f64 {
        Rs = 1.0f64 / (1.0f64 + 0.15f64 * pow(V, 1.8f64) / Sx / pow(Lg, 2.3f64));
    }
    return Q * (Rf * Eo + Rs * (1.0f64 - Eo));
}
unsafe extern "C" fn getCurbInletCapture(
    mut Q: ::core::ffi::c_double,
    mut L: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut Se: ::core::ffi::c_double = Sx;
    let mut Lt: ::core::ffi::c_double = 0.;
    let mut Sr: ::core::ffi::c_double = 0.;
    let mut Eo: ::core::ffi::c_double = 0.0f64;
    let mut E: ::core::ffi::c_double = 1.0f64;
    if a > 0.0f64 {
        Sr = Sw / Sx;
        Eo = getEo(Sr, T - W, W);
        Se = Sx + a / W * Eo;
    }
    Lt = 0.6f64 * pow(Q, 0.42f64) * pow(SL, 0.3f64) * pow(1.0f64 / (n * Se), 0.6f64);
    if L < Lt {
        E = 1.0f64 - L / Lt;
        E = 1 as ::core::ffi::c_int as ::core::ffi::c_double - pow(E, 1.8f64);
    }
    E = if E <= 1.0f64 { E } else { 1.0f64 };
    E = if E >= 0.0f64 { E } else { 0.0f64 };
    return E * Q;
}
unsafe extern "C" fn getGutterFlowRatio(mut w: ::core::ffi::c_double) -> ::core::ffi::c_double {
    if T <= w {
        return 1.0f64;
    } else if a > 0.0f64 {
        return getEo(Sw / Sx, T - w, w);
    } else {
        return 1.0f64 - pow(1.0f64 - w / T, 2.67f64);
    };
}
unsafe extern "C" fn getGutterAreaRatio(
    mut Wg: ::core::ffi::c_double,
    mut A: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut As: ::core::ffi::c_double = 0.;
    let mut Ag: ::core::ffi::c_double = 0.;
    if Wg >= W {
        return 1.0f64;
    }
    if T <= Wg {
        return 1.0f64;
    }
    if T <= W {
        return Wg / T;
    }
    As = 0.5f64 * ((T - W) * (T - W)) * Sx;
    Ag = Wg * (T * Sx + a - Wg * Sw / 2.0f64);
    return Ag / (A - As);
}
unsafe extern "C" fn getSplashOverVelocity(
    mut grateType: ::core::ffi::c_int,
    mut L: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return SplashCoeffs[grateType as usize][0 as ::core::ffi::c_int as usize]
        + SplashCoeffs[grateType as usize][1 as ::core::ffi::c_int as usize] * L
        - SplashCoeffs[grateType as usize][2 as ::core::ffi::c_int as usize] * L * L
        + SplashCoeffs[grateType as usize][3 as ::core::ffi::c_int as usize] * L * L * L;
}
unsafe extern "C" fn getOnSagCapturedFlow(
    mut inlet: *mut TInlet,
    mut q: ::core::ffi::c_double,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut linkIndex: ::core::ffi::c_int = 0;
    let mut designIndex: ::core::ffi::c_int = 0;
    let mut totalInlets: ::core::ffi::c_int = 0;
    let mut qCaptured: ::core::ffi::c_double = 0.0f64;
    let mut qMax: ::core::ffi::c_double = BIG;
    if (*inlet).numInlets == 0 as ::core::ffi::c_int {
        return 0.0f64;
    }
    totalInlets = Nsides * (*inlet).numInlets;
    linkIndex = (*inlet).linkIndex;
    designIndex = (*inlet).designIndex;
    getConduitGeometry(inlet);
    if (*inlet).flowLimit > 0.0f64 {
        qMax = (*inlet).flowLimit;
    }
    qCaptured = getOnSagInletCapture(designIndex, fabs(d));
    qCaptured *= (*inlet).clogFactor;
    qCaptured = if qCaptured <= qMax { qCaptured } else { qMax };
    qCaptured *= totalInlets as ::core::ffi::c_double;
    return qCaptured;
}
unsafe extern "C" fn getOnSagInletCapture(
    mut i: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut Lsweep: ::core::ffi::c_double = 0.0f64;
    let mut Lcurb: ::core::ffi::c_double = 0.0f64;
    let mut Lgrate: ::core::ffi::c_double = 0.0f64;
    let mut Qsw: ::core::ffi::c_double = 0.0f64;
    let mut Qso: ::core::ffi::c_double = 0.0f64;
    let mut Qgw: ::core::ffi::c_double = 0.0f64;
    let mut Qgo: ::core::ffi::c_double = 0.0f64;
    let mut Qcw: ::core::ffi::c_double = 0.0f64;
    let mut Qco: ::core::ffi::c_double = 0.0f64;
    if (*InletDesigns.offset(i as isize)).slottedInlet.length > 0.0f64 {
        return getOnSagSlottedFlow(i, d);
    }
    Lgrate = (*InletDesigns.offset(i as isize)).grateInlet.length;
    if Lgrate > 0.0f64 {
        findOnSagGrateFlows(i, d, &raw mut Qgw, &raw mut Qgo);
    }
    Lcurb = (*InletDesigns.offset(i as isize)).curbInlet.length;
    if Lcurb > 0.0f64 {
        Lsweep = Lcurb - Lgrate;
        if Lsweep > 0.0f64 {
            findOnSagCurbFlows(i, d, Lsweep, &raw mut Qsw, &raw mut Qso);
        }
        if Qgo > 0.0f64 {
            findOnSagCurbFlows(i, d, Lgrate, &raw mut Qcw, &raw mut Qco);
        }
    }
    return Qgw + Qgo + Qsw + Qso + Qco;
}
unsafe extern "C" fn findOnSagGrateFlows(
    mut i: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
    mut Qw: *mut ::core::ffi::c_double,
    mut Qo: *mut ::core::ffi::c_double,
) {
    let mut grateType: ::core::ffi::c_int = (*InletDesigns.offset(i as isize)).grateInlet.type_0;
    let mut Lg: ::core::ffi::c_double = (*InletDesigns.offset(i as isize)).grateInlet.length;
    let mut Wg: ::core::ffi::c_double = (*InletDesigns.offset(i as isize)).grateInlet.width;
    let mut P: ::core::ffi::c_double = 0.;
    let mut Ao: ::core::ffi::c_double = 0.;
    let mut di: ::core::ffi::c_double = 0.;
    if (*InletDesigns.offset(i as isize)).type_0 == DROP_GRATE_INLET as ::core::ffi::c_int {
        di = d;
        P = 2.0f64 * (Lg + Wg);
    } else {
        if d <= Wg * Sw {
            Wg = d / Sw;
        }
        di = d - Wg / 2.0f64 * Sw;
        P = Lg + 2.0f64 * Wg;
    }
    if grateType == GENERIC as ::core::ffi::c_int {
        Ao = Lg * Wg * (*InletDesigns.offset(i as isize)).grateInlet.fracOpenArea;
    } else {
        Ao = Lg * Wg * GrateOpeningRatios[grateType as usize];
    }
    if d <= 1.79f64 * Ao / P {
        *Qw = 3.0f64 * P * pow(di, 1.5f64);
    } else {
        *Qo = 0.67f64 * Ao * sqrt(2.0f64 * 32.16f64 * di);
    };
}
unsafe extern "C" fn findOnSagCurbFlows(
    mut i: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
    mut L: ::core::ffi::c_double,
    mut Qw: *mut ::core::ffi::c_double,
    mut Qo: *mut ::core::ffi::c_double,
) {
    let mut throatAngle: ::core::ffi::c_int =
        (*InletDesigns.offset(i as isize)).curbInlet.throatAngle;
    let mut h: ::core::ffi::c_double = (*InletDesigns.offset(i as isize)).curbInlet.height;
    let mut Qweir: ::core::ffi::c_double = 0.;
    let mut Qorif: ::core::ffi::c_double = 0.;
    let mut P: ::core::ffi::c_double = 0.;
    let mut dweir: ::core::ffi::c_double = 0.;
    let mut dorif: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    if L <= 0.0f64 {
        return;
    }
    if (*InletDesigns.offset(i as isize)).type_0 == DROP_CURB_INLET as ::core::ffi::c_int {
        L = L * 4.0f64;
    }
    dorif = 1.4f64 * h;
    if d > dorif {
        *Qo = getCurbOrificeFlow(d, h, L, throatAngle);
        return;
    }
    if a == 0.0f64 || L > 12.0f64 {
        dweir = h;
        if d < dweir {
            *Qw = 3.0f64 * L * pow(d, 1.5f64);
            return;
        } else {
            Qweir = 3.0f64 * L * pow(dweir, 1.5f64);
        }
    } else {
        P = L + 1.8f64 * W;
        dweir = h + a;
        if d < dweir {
            *Qw = 2.3f64 * P * pow(d, 1.5f64);
            return;
        } else {
            Qweir = 2.3f64 * P * pow(dweir, 1.5f64);
        }
    }
    Qorif = getCurbOrificeFlow(dorif, h, L, throatAngle);
    r = (d - dweir) / (dorif - dweir);
    *Qw = (1.0f64 - r) * Qweir;
    *Qo = r * Qorif;
}
unsafe extern "C" fn getCurbOrificeFlow(
    mut di: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
    mut L: ::core::ffi::c_double,
    mut throatAngle: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut d: ::core::ffi::c_double = di;
    if throatAngle == HORIZONTAL_THROAT as ::core::ffi::c_int {
        d = di - h / 2.0f64;
    } else if throatAngle == INCLINED_THROAT as ::core::ffi::c_int {
        d = di - h / 2.0f64 * 0.7071f64;
    }
    return 0.67f64 * h * L * sqrt(2.0f64 * 32.16f64 * d);
}
unsafe extern "C" fn getOnSagSlottedFlow(
    mut i: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut L: ::core::ffi::c_double = (*InletDesigns.offset(i as isize)).slottedInlet.length;
    let mut w: ::core::ffi::c_double = (*InletDesigns.offset(i as isize)).slottedInlet.width;
    if d <= 2.587f64 * w {
        return 2.48f64 * L * pow(d, 1.5f64);
    } else {
        return 0.8f64 * L * w * sqrt(64.32f64 * d);
    };
}
unsafe extern "C" fn getBackflowRatios() {
    let mut inlet: *mut TInlet = ::core::ptr::null_mut::<TInlet>();
    let mut area: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut n_0: ::core::ffi::c_int = 0;
    let mut inletNodes: *mut TInletNode = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TInletNode>() as size_t,
    ) as *mut TInletNode;
    if inletNodes.is_null() {
        return;
    }
    inlet = FirstInlet;
    while !inlet.is_null() {
        n_0 = (*inlet).nodeIndex;
        let ref mut fresh3 = (*inletNodes.offset(n_0 as isize)).numInletLinks;
        *fresh3 += 1;
        area = getInletArea(inlet);
        if area > 0.0f64 {
            let ref mut fresh4 = (*inletNodes.offset(n_0 as isize)).numStdInletLinks;
            *fresh4 += 1;
            (*inletNodes.offset(n_0 as isize)).totalInletArea += area;
        } else {
            (*inletNodes.offset(n_0 as isize)).numCustomInlets += (*inlet).numInlets;
        }
        inlet = (*inlet).nextInlet;
    }
    inlet = FirstInlet;
    while !inlet.is_null() {
        n_0 = (*inlet).nodeIndex;
        f = (*inletNodes.offset(n_0 as isize)).numStdInletLinks as ::core::ffi::c_double
            / (*inletNodes.offset(n_0 as isize)).numInletLinks as ::core::ffi::c_double;
        area = getInletArea(inlet);
        if area == 0.0f64 {
            (*inlet).backflowRatio = (*inlet).numInlets as ::core::ffi::c_double
                / (*inletNodes.offset(n_0 as isize)).numCustomInlets as ::core::ffi::c_double
                * (1.0f64 - f);
        } else {
            (*inlet).backflowRatio = area / (*inletNodes.offset(n_0 as isize)).totalInletArea * f;
        }
        inlet = (*inlet).nextInlet;
    }
    free(inletNodes as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn getInletArea(mut inlet: *mut TInlet) -> ::core::ffi::c_double {
    let mut area: ::core::ffi::c_double = 0.0f64;
    let mut curbLength: ::core::ffi::c_double = 0.;
    let mut i: ::core::ffi::c_int = (*inlet).designIndex;
    let mut grateType: ::core::ffi::c_int = (*InletDesigns.offset(i as isize)).grateInlet.type_0;
    if (*InletDesigns.offset(i as isize)).grateInlet.length > 0.0f64 {
        area = (*InletDesigns.offset(i as isize)).grateInlet.length
            * (*InletDesigns.offset(i as isize)).grateInlet.width;
        if grateType == GENERIC as ::core::ffi::c_int {
            area *= (*InletDesigns.offset(i as isize)).grateInlet.fracOpenArea;
        } else {
            area *= GrateOpeningRatios[grateType as usize];
        }
    }
    curbLength = (*InletDesigns.offset(i as isize)).curbInlet.length
        - (*InletDesigns.offset(i as isize)).grateInlet.length;
    if curbLength > 0.0f64 {
        area += curbLength * (*InletDesigns.offset(i as isize)).curbInlet.height;
    }
    if (*InletDesigns.offset(i as isize)).slottedInlet.length > 0.0f64 {
        area = (*InletDesigns.offset(i as isize)).slottedInlet.length
            * (*InletDesigns.offset(i as isize)).slottedInlet.width;
    }
    return area * (*inlet).numInlets as ::core::ffi::c_double * (*inlet).clogFactor;
}
unsafe extern "C" fn getCustomCapturedFlow(
    mut inlet: *mut TInlet,
    mut q: ::core::ffi::c_double,
    mut d: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = (*inlet).designIndex;
    let mut j: ::core::ffi::c_int = 0;
    let mut sides: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    let mut qApproach: ::core::ffi::c_double = 0.;
    let mut qBypassed: ::core::ffi::c_double = 0.;
    let mut qCaptured: ::core::ffi::c_double = 0.;
    let mut qIncrement: ::core::ffi::c_double = 0.;
    let mut qMax: ::core::ffi::c_double = BIG;
    if (*inlet).numInlets == 0 as ::core::ffi::c_int {
        return 0.0f64;
    }
    qMax = BIG;
    if (*inlet).flowLimit > 0.0f64 {
        qMax = (*inlet).flowLimit;
    }
    xsect = &raw mut (*Link.offset((*inlet).linkIndex as isize)).xsect;
    if (*xsect).type_0 == STREET_XSECT as ::core::ffi::c_int {
        sides = (*Street.offset((*xsect).transect as isize)).sides;
    }
    qApproach = q / sides as ::core::ffi::c_double;
    qBypassed = qApproach;
    qCaptured = 0.0f64;
    c = (*InletDesigns.offset(i as isize)).customCurve;
    if c >= 0 as ::core::ffi::c_int {
        if (*Curve.offset(c as isize)).curveType == DIVERSION_CURVE as ::core::ffi::c_int {
            j = 1 as ::core::ffi::c_int;
            while j <= (*inlet).numInlets {
                qIncrement = (*inlet).clogFactor
                    * table_lookupEx(
                        Curve.offset(c as isize) as *mut TTable,
                        qBypassed * UCF(FLOW as ::core::ffi::c_int),
                    )
                    / UCF(FLOW as ::core::ffi::c_int);
                qIncrement = if qIncrement <= qMax { qIncrement } else { qMax };
                qIncrement = if qIncrement <= qBypassed {
                    qIncrement
                } else {
                    qBypassed
                };
                qCaptured += qIncrement;
                qBypassed -= qIncrement;
                if qBypassed < MIN_RUNOFF_FLOW {
                    break;
                }
                j += 1;
            }
        } else if (*Curve.offset(c as isize)).curveType == RATING_CURVE as ::core::ffi::c_int {
            qCaptured = (*inlet).numInlets as ::core::ffi::c_double
                * (*inlet).clogFactor
                * table_lookupEx(
                    Curve.offset(c as isize) as *mut TTable,
                    d * UCF(LENGTH as ::core::ffi::c_int),
                )
                / UCF(FLOW as ::core::ffi::c_int);
        }
        qCaptured *= sides as ::core::ffi::c_double;
    }
    return qCaptured;
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
