#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut UnitSystem: ::core::ffi::c_int;
    static mut RouteModel: ::core::ffi::c_int;
    static mut ForceMainEqn: ::core::ffi::c_int;
    static mut LinkOffsets: ::core::ffi::c_int;
    static mut LengtheningStep: ::core::ffi::c_double;
    static mut RouteStep: ::core::ffi::c_double;
    static mut MinSlope: ::core::ffi::c_double;
    static mut StartDate: DateTime;
    static mut Evap: TEvap;
    static mut Adjust: TAdjust;
    static mut Node: *mut TNode;
    static mut Outfall: *mut TOutfall;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Pump: *mut TPump;
    static mut Orifice: *mut TOrifice;
    static mut Weir: *mut TWeir;
    static mut Outlet: *mut TOutlet;
    static mut Curve: *mut TTable;
    static mut Transect: *mut TTransect;
    static mut Street: *mut TStreet;
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
    fn node_setOutletDepth(
        node: ::core::ffi::c_int,
        yNorm: ::core::ffi::c_double,
        yCrit: ::core::ffi::c_double,
        z: ::core::ffi::c_double,
    );
    fn node_getOutflow(node: ::core::ffi::c_int, link: ::core::ffi::c_int)
        -> ::core::ffi::c_double;
    fn xsect_isOpen(type_0: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xsect_setParams(
        xsect: *mut TXsect,
        type_0: ::core::ffi::c_int,
        p: *mut ::core::ffi::c_double,
        ucf: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn xsect_setIrregXsectParams(xsect: *mut TXsect);
    fn xsect_setCustomXsectParams(xsect: *mut TXsect);
    fn xsect_setStreetXsectParams(xsect: *mut TXsect);
    fn xsect_getYofA(xsect: *mut TXsect, area: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getAofS(xsect: *mut TXsect, sFactor: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getAofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getWofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getYcrit(xsect: *mut TXsect, q: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn roadway_getInflow(
        link: ::core::ffi::c_int,
        dir: ::core::ffi::c_double,
        hcrest: ::core::ffi::c_double,
        h1: ::core::ffi::c_double,
        h2: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn forcemain_getEquivN(j: ::core::ffi::c_int, k: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn forcemain_getRoughFactor(
        j: ::core::ffi::c_int,
        lengthFactor: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn table_getFirstEntry(
        table: *mut TTable,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn table_getNextEntry(
        table: *mut TTable,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn table_lookup(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn table_intervalLookup(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn table_getSlope(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcomp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut NoYesWords: [*mut ::core::ffi::c_char; 0];
    static mut OffOnWords: [*mut ::core::ffi::c_char; 0];
    static mut OrificeTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut RelationWords: [*mut ::core::ffi::c_char; 0];
    static mut WeirTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut XsectTypeWords: [*mut ::core::ffi::c_char; 0];
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
pub type LinkResultType = ::core::ffi::c_uint;
pub const LINK_QUAL: LinkResultType = 5;
pub const LINK_CAPACITY: LinkResultType = 4;
pub const LINK_VOLUME: LinkResultType = 3;
pub const LINK_VELOCITY: LinkResultType = 2;
pub const LINK_DEPTH: LinkResultType = 1;
pub const LINK_FLOW: LinkResultType = 0;
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
pub type ForceMainType = ::core::ffi::c_uint;
pub const D_W: ForceMainType = 1;
pub const H_W: ForceMainType = 0;
pub type OffsetType = ::core::ffi::c_uint;
pub const ELEV_OFFSET: OffsetType = 1;
pub const DEPTH_OFFSET: OffsetType = 0;
pub type StorageType = ::core::ffi::c_uint;
pub const PYRAMIDAL: StorageType = 5;
pub const PARABOLOID: StorageType = 4;
pub const CONICAL: StorageType = 3;
pub const CYLINDRICAL: StorageType = 2;
pub const FUNCTIONAL: StorageType = 1;
pub const TABULAR: StorageType = 0;
pub type PumpCurveType = ::core::ffi::c_uint;
pub const IDEAL_PUMP: PumpCurveType = 5;
pub const TYPE5_PUMP: PumpCurveType = 4;
pub const TYPE4_PUMP: PumpCurveType = 3;
pub const TYPE3_PUMP: PumpCurveType = 2;
pub const TYPE2_PUMP: PumpCurveType = 1;
pub const TYPE1_PUMP: PumpCurveType = 0;
pub type OrificeType = ::core::ffi::c_uint;
pub const BOTTOM_ORIFICE: OrificeType = 1;
pub const SIDE_ORIFICE: OrificeType = 0;
pub type WeirType = ::core::ffi::c_uint;
pub const ROADWAY_WEIR: WeirType = 4;
pub const TRAPEZOIDAL_WEIR: WeirType = 3;
pub const VNOTCH_WEIR: WeirType = 2;
pub const SIDEFLOW_WEIR: WeirType = 1;
pub const TRANSVERSE_WEIR: WeirType = 0;
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
pub type NoYesType = ::core::ffi::c_uint;
pub const YES: NoYesType = 1;
pub const NO: NoYesType = 0;
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
pub struct TAdjust {
    pub temp: [::core::ffi::c_double; 12],
    pub evap: [::core::ffi::c_double; 12],
    pub rain: [::core::ffi::c_double; 12],
    pub hydcon: [::core::ffi::c_double; 12],
    pub rainFactor: ::core::ffi::c_double,
    pub hydconFactor: ::core::ffi::c_double,
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
pub struct TPump {
    pub type_0: ::core::ffi::c_int,
    pub pumpCurve: ::core::ffi::c_int,
    pub initSetting: ::core::ffi::c_double,
    pub yOn: ::core::ffi::c_double,
    pub yOff: ::core::ffi::c_double,
    pub xMin: ::core::ffi::c_double,
    pub xMax: ::core::ffi::c_double,
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
pub struct TWeir {
    pub type_0: ::core::ffi::c_int,
    pub cDisch1: ::core::ffi::c_double,
    pub cDisch2: ::core::ffi::c_double,
    pub endCon: ::core::ffi::c_double,
    pub canSurcharge: ::core::ffi::c_int,
    pub roadWidth: ::core::ffi::c_double,
    pub roadSurface: ::core::ffi::c_int,
    pub cdCurve: ::core::ffi::c_int,
    pub cSurcharge: ::core::ffi::c_double,
    pub length: ::core::ffi::c_double,
    pub slope: ::core::ffi::c_double,
    pub surfArea: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TOutlet {
    pub qCoeff: ::core::ffi::c_double,
    pub qExpon: ::core::ffi::c_double,
    pub qCurve: ::core::ffi::c_int,
    pub curveType: ::core::ffi::c_int,
}
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_REGULATOR_SHAPE: ErrorType = 143;
pub const ERR_PUMP_LIMITS: ErrorType = 122;
pub const ERR_NO_CURVE: ErrorType = 121;
pub const ERR_XSECT: ErrorType = 119;
pub const ERR_NO_XSECT: ErrorType = 117;
pub const ERR_BARRELS: ErrorType = 114;
pub const ERR_ROUGHNESS: ErrorType = 113;
pub const ERR_LENGTH: ErrorType = 111;
pub const ERR_DUMMY_LINK: ErrorType = 134;
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
pub const ERR_OUTFALL: ErrorType = 141;
pub const ERR_STORAGE_VOLUME: ErrorType = 140;
pub const ERR_REGULATOR: ErrorType = 139;
pub const ERR_NODE_DEPTH: ErrorType = 138;
pub const ERR_WEIR_DIVIDER: ErrorType = 137;
pub const ERR_DIVIDER_LINK: ErrorType = 136;
pub const ERR_DIVIDER: ErrorType = 135;
pub const ERR_MULTI_OUTLET: ErrorType = 133;
pub const ERR_LOOP: ErrorType = 131;
pub const ERR_SLOPE: ErrorType = 115;
pub const ERR_ELEV_DROP: ErrorType = 112;
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
pub const TINY: ::core::ffi::c_double = 1.0E-6f64;
pub const MISSING: ::core::ffi::c_double = -1.0E10f64;
pub const GRAVITY: ::core::ffi::c_double = 32.2f64;
pub const PHI: ::core::ffi::c_double = 1.486f64;
pub const FUDGE: ::core::ffi::c_double = 0.0001f64;
pub const M3perFT3: ::core::ffi::c_double = 0.028317f64;
pub const KWperHP: ::core::ffi::c_double = 0.7457f64;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const WARN03: [::core::ffi::c_char; 45] = unsafe {
    ::core::mem::transmute::<[u8; 45], [::core::ffi::c_char; 45]>(
        *b"WARNING 03: negative offset ignored for Link\0",
    )
};
pub const WARN04: [::core::ffi::c_char; 52] = unsafe {
    ::core::mem::transmute::<[u8; 52], [::core::ffi::c_char; 52]>(
        *b"WARNING 04: minimum elevation drop used for Conduit\0",
    )
};
pub const WARN05: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"WARNING 05: minimum slope used for Conduit\0",
    )
};
pub const WARN08: [::core::ffi::c_char; 54] = unsafe {
    ::core::mem::transmute::<[u8; 54], [::core::ffi::c_char; 54]>(
        *b"WARNING 08: elevation drop exceeds length for Conduit\0",
    )
};
pub const WARN10a: [::core::ffi::c_char; 74] = unsafe {
    ::core::mem::transmute::<[u8; 74], [::core::ffi::c_char; 74]>(
        *b"WARNING 10: crest elevation is below downstream invert for regulator Link\0",
    )
};
pub const WARN10b: [::core::ffi::c_char; 75] = unsafe {
    ::core::mem::transmute::<[u8; 75], [::core::ffi::c_char; 75]>(
        *b"WARNING 10: crest elevation raised to downstream invert for regulator Link\0",
    )
};
pub const w_HEAD: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HEAD\0") };
static mut MIN_DELTA_Z: ::core::ffi::c_double = 0.001f64;
#[no_mangle]
pub unsafe extern "C" fn link_readParams(
    mut j: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match type_0 {
        0 => return conduit_readParams(j, k, tok, ntoks),
        1 => return pump_readParams(j, k, tok, ntoks),
        2 => return orifice_readParams(j, k, tok, ntoks),
        3 => return weir_readParams(j, k, tok, ntoks),
        4 => return outlet_readParams(j, k, tok, ntoks),
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn link_readXsectParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 4] = [0.; 4];
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        LINK as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    k = findmatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut XsectTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        (*Conduit.offset((*Link.offset(j as isize)).subIndex as isize)).barrels =
            1 as ::core::ffi::c_char;
    }
    (*Link.offset(j as isize)).xsect.culvertCode = 0 as ::core::ffi::c_int;
    if k == IRREGULAR as ::core::ffi::c_int {
        i = project_findObject(
            TRANSECT as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        (*Link.offset(j as isize)).xsect.type_0 = k;
        (*Link.offset(j as isize)).xsect.transect = i;
        return 0 as ::core::ffi::c_int;
    } else if k == STREET_XSECT as ::core::ffi::c_int {
        i = project_findObject(
            STREET as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        (*Link.offset(j as isize)).xsect.type_0 = k;
        (*Link.offset(j as isize)).xsect.transect = i;
        return 0 as ::core::ffi::c_int;
    } else {
        if ntoks < 6 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if k == CUSTOM as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(2 as ::core::ffi::c_int as isize),
                (&raw mut x as *mut ::core::ffi::c_double).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
                || x[0 as ::core::ffi::c_int as usize] <= 0.0f64
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(2 as ::core::ffi::c_int as isize),
                );
            }
            i = project_findObject(
                CURVE as ::core::ffi::c_int,
                *tok.offset(3 as ::core::ffi::c_int as isize),
            );
            if i < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(3 as ::core::ffi::c_int as isize),
                );
            }
            (*Link.offset(j as isize)).xsect.type_0 = k;
            (*Link.offset(j as isize)).xsect.transect = i;
            (*Link.offset(j as isize)).xsect.yFull =
                x[0 as ::core::ffi::c_int as usize] / UCF(LENGTH as ::core::ffi::c_int);
        } else {
            i = 2 as ::core::ffi::c_int;
            while i <= 5 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset(i as isize),
                    (&raw mut x as *mut ::core::ffi::c_double)
                        .offset((i - 2 as ::core::ffi::c_int) as isize)
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
        }
        if (*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int
            && k == RECT_OPEN as ::core::ffi::c_int
        {
            x[2 as ::core::ffi::c_int as usize] = 0.0f64;
            x[3 as ::core::ffi::c_int as usize] = 0.0f64;
        }
        if xsect_setParams(
            &raw mut (*Link.offset(j as isize)).xsect,
            k,
            &raw mut x as *mut ::core::ffi::c_double,
            UCF(LENGTH as ::core::ffi::c_int),
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int
            && ntoks >= 7 as ::core::ffi::c_int
        {
            i = atoi(*tok.offset(6 as ::core::ffi::c_int as isize));
            if i <= 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(6 as ::core::ffi::c_int as isize),
                );
            } else {
                (*Conduit.offset((*Link.offset(j as isize)).subIndex as isize)).barrels =
                    i as ::core::ffi::c_char;
            }
        }
        if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int
            && ntoks >= 8 as ::core::ffi::c_int
        {
            i = atoi(*tok.offset(7 as ::core::ffi::c_int as isize));
            if i < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(7 as ::core::ffi::c_int as isize),
                );
            } else {
                (*Link.offset(j as isize)).xsect.culvertCode = i;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn link_readLossParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 3] = [0.; 3];
    let mut seepRate: ::core::ffi::c_double = 0.0f64;
    if ntoks < 4 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        LINK as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
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
            || x[(i - 1 as ::core::ffi::c_int) as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(i as isize));
        }
        i += 1;
    }
    k = 0 as ::core::ffi::c_int;
    if ntoks >= 5 as ::core::ffi::c_int {
        k = findmatch(
            *tok.offset(4 as ::core::ffi::c_int as isize),
            &raw mut NoYesWords as *mut *mut ::core::ffi::c_char,
        );
        if k < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(4 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks >= 6 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(5 as ::core::ffi::c_int as isize),
            &raw mut seepRate,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(5 as ::core::ffi::c_int as isize),
            );
        }
    }
    (*Link.offset(j as isize)).cLossInlet = x[0 as ::core::ffi::c_int as usize];
    (*Link.offset(j as isize)).cLossOutlet = x[1 as ::core::ffi::c_int as usize];
    (*Link.offset(j as isize)).cLossAvg = x[2 as ::core::ffi::c_int as usize];
    (*Link.offset(j as isize)).hasFlapGate = k;
    (*Link.offset(j as isize)).seepRate = seepRate / UCF(RAINFALL as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn link_setParams(
    mut j: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut n1: ::core::ffi::c_int,
    mut n2: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) {
    (*Link.offset(j as isize)).node1 = n1;
    (*Link.offset(j as isize)).node2 = n2;
    (*Link.offset(j as isize)).type_0 = type_0;
    (*Link.offset(j as isize)).subIndex = k;
    (*Link.offset(j as isize)).offset1 = 0.0f64;
    (*Link.offset(j as isize)).offset2 = 0.0f64;
    (*Link.offset(j as isize)).q0 = 0.0f64;
    (*Link.offset(j as isize)).qFull = 0.0f64;
    (*Link.offset(j as isize)).setting = 1.0f64;
    (*Link.offset(j as isize)).targetSetting = 1.0f64;
    (*Link.offset(j as isize)).hasFlapGate = 0 as ::core::ffi::c_int;
    (*Link.offset(j as isize)).qLimit = 0.0f64;
    (*Link.offset(j as isize)).direction = 1 as ::core::ffi::c_schar;
    match type_0 {
        0 => {
            (*Conduit.offset(k as isize)).length =
                *x.offset(0 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Conduit.offset(k as isize)).modLength = (*Conduit.offset(k as isize)).length;
            (*Conduit.offset(k as isize)).roughness = *x.offset(1 as ::core::ffi::c_int as isize);
            (*Link.offset(j as isize)).offset1 =
                *x.offset(2 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Link.offset(j as isize)).offset2 =
                *x.offset(3 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Link.offset(j as isize)).q0 =
                *x.offset(4 as ::core::ffi::c_int as isize) / UCF(FLOW as ::core::ffi::c_int);
            (*Link.offset(j as isize)).qLimit =
                *x.offset(5 as ::core::ffi::c_int as isize) / UCF(FLOW as ::core::ffi::c_int);
        }
        1 => {
            (*Pump.offset(k as isize)).pumpCurve =
                crate::ccast::f64_to_i32(*x.offset(0 as ::core::ffi::c_int as isize));
            (*Link.offset(j as isize)).hasFlapGate = FALSE;
            (*Pump.offset(k as isize)).initSetting = *x.offset(1 as ::core::ffi::c_int as isize);
            (*Pump.offset(k as isize)).yOn =
                *x.offset(2 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Pump.offset(k as isize)).yOff =
                *x.offset(3 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Pump.offset(k as isize)).xMin = 0.0f64;
            (*Pump.offset(k as isize)).xMax = 0.0f64;
        }
        2 => {
            (*Orifice.offset(k as isize)).type_0 =
                crate::ccast::f64_to_i32(*x.offset(0 as ::core::ffi::c_int as isize));
            (*Link.offset(j as isize)).offset1 =
                *x.offset(1 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Link.offset(j as isize)).offset2 = (*Link.offset(j as isize)).offset1;
            (*Orifice.offset(k as isize)).cDisch = *x.offset(2 as ::core::ffi::c_int as isize);
            (*Link.offset(j as isize)).hasFlapGate =
                if *x.offset(3 as ::core::ffi::c_int as isize) > 0.0f64 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
            (*Orifice.offset(k as isize)).orate =
                *x.offset(4 as ::core::ffi::c_int as isize) * 3600.0f64;
        }
        3 => {
            (*Weir.offset(k as isize)).type_0 =
                crate::ccast::f64_to_i32(*x.offset(0 as ::core::ffi::c_int as isize));
            (*Link.offset(j as isize)).offset1 =
                *x.offset(1 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Link.offset(j as isize)).offset2 = (*Link.offset(j as isize)).offset1;
            (*Weir.offset(k as isize)).cDisch1 = *x.offset(2 as ::core::ffi::c_int as isize);
            (*Link.offset(j as isize)).hasFlapGate =
                if *x.offset(3 as ::core::ffi::c_int as isize) > 0.0f64 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
            (*Weir.offset(k as isize)).endCon = *x.offset(4 as ::core::ffi::c_int as isize);
            (*Weir.offset(k as isize)).cDisch2 = *x.offset(5 as ::core::ffi::c_int as isize);
            (*Weir.offset(k as isize)).canSurcharge =
                crate::ccast::f64_to_i32(*x.offset(6 as ::core::ffi::c_int as isize));
            (*Weir.offset(k as isize)).roadWidth =
                *x.offset(7 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Weir.offset(k as isize)).roadSurface =
                crate::ccast::f64_to_i32(*x.offset(8 as ::core::ffi::c_int as isize));
            (*Weir.offset(k as isize)).cdCurve =
                crate::ccast::f64_to_i32(*x.offset(9 as ::core::ffi::c_int as isize));
        }
        4 => {
            (*Link.offset(j as isize)).offset1 =
                *x.offset(0 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
            (*Link.offset(j as isize)).offset2 = (*Link.offset(j as isize)).offset1;
            (*Outlet.offset(k as isize)).qCoeff = *x.offset(1 as ::core::ffi::c_int as isize);
            (*Outlet.offset(k as isize)).qExpon = *x.offset(2 as ::core::ffi::c_int as isize);
            (*Outlet.offset(k as isize)).qCurve =
                crate::ccast::f64_to_i32(*x.offset(3 as ::core::ffi::c_int as isize));
            (*Link.offset(j as isize)).hasFlapGate =
                if *x.offset(4 as ::core::ffi::c_int as isize) > 0.0f64 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
            (*Outlet.offset(k as isize)).curveType =
                crate::ccast::f64_to_i32(*x.offset(5 as ::core::ffi::c_int as isize));
            xsect_setParams(
                &raw mut (*Link.offset(j as isize)).xsect,
                DUMMY as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_double>(),
                0.0f64,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn link_validate(mut j: ::core::ffi::c_int) {
    let mut n: ::core::ffi::c_int = 0;
    if LinkOffsets == ELEV_OFFSET as ::core::ffi::c_int {
        link_convertOffsets(j);
    }
    match (*Link.offset(j as isize)).type_0 {
        0 => {
            conduit_validate(j, (*Link.offset(j as isize)).subIndex);
        }
        1 => {
            pump_validate(j, (*Link.offset(j as isize)).subIndex);
        }
        2 => {
            orifice_validate(j, (*Link.offset(j as isize)).subIndex);
        }
        3 => {
            weir_validate(j, (*Link.offset(j as isize)).subIndex);
        }
        _ => {}
    }
    match (*Link.offset(j as isize)).type_0 {
        2 | 3 | 4 => {
            if (*Node.offset((*Link.offset(j as isize)).node1 as isize)).invertElev
                + (*Link.offset(j as isize)).offset1
                < (*Node.offset((*Link.offset(j as isize)).node2 as isize)).invertElev
            {
                if RouteModel == DW as ::core::ffi::c_int {
                    (*Link.offset(j as isize)).offset1 =
                        (*Node.offset((*Link.offset(j as isize)).node2 as isize)).invertElev
                            - (*Node.offset((*Link.offset(j as isize)).node1 as isize)).invertElev;
                    report_writeWarningMsg(
                        WARN10b.as_ptr() as *mut ::core::ffi::c_char,
                        (*Link.offset(j as isize)).ID,
                    );
                } else {
                    report_writeWarningMsg(
                        WARN10a.as_ptr() as *mut ::core::ffi::c_char,
                        (*Link.offset(j as isize)).ID,
                    );
                }
            }
        }
        _ => {}
    }
    if (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int
        || (*Link.offset(j as isize)).type_0 == ORIFICE as ::core::ffi::c_int
            && (*Orifice.offset((*Link.offset(j as isize)).subIndex as isize)).type_0
                == BOTTOM_ORIFICE as ::core::ffi::c_int
    {
        return;
    }
    n = (*Link.offset(j as isize)).node1;
    if (*Node.offset(n as isize)).type_0 != STORAGE as ::core::ffi::c_int
        || (*Node.offset(n as isize)).surDepth > 0.0f64
    {
        (*Node.offset(n as isize)).fullDepth = if (*Node.offset(n as isize)).fullDepth
            >= (*Link.offset(j as isize)).offset1 + (*Link.offset(j as isize)).xsect.yFull
        {
            (*Node.offset(n as isize)).fullDepth
        } else {
            (*Link.offset(j as isize)).offset1 + (*Link.offset(j as isize)).xsect.yFull
        };
    }
    n = (*Link.offset(j as isize)).node2;
    if ((*Node.offset(n as isize)).type_0 != STORAGE as ::core::ffi::c_int
        || (*Node.offset(n as isize)).surDepth > 0.0f64)
        && (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int
    {
        (*Node.offset(n as isize)).fullDepth = if (*Node.offset(n as isize)).fullDepth
            >= (*Link.offset(j as isize)).offset2 + (*Link.offset(j as isize)).xsect.yFull
        {
            (*Node.offset(n as isize)).fullDepth
        } else {
            (*Link.offset(j as isize)).offset2 + (*Link.offset(j as isize)).xsect.yFull
        };
    }
}
unsafe extern "C" fn link_convertOffsets(mut j: ::core::ffi::c_int) {
    let mut elev: ::core::ffi::c_double = 0.;
    elev = (*Node.offset((*Link.offset(j as isize)).node1 as isize)).invertElev;
    (*Link.offset(j as isize)).offset1 =
        link_getOffsetHeight(j, (*Link.offset(j as isize)).offset1, elev);
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        elev = (*Node.offset((*Link.offset(j as isize)).node2 as isize)).invertElev;
        (*Link.offset(j as isize)).offset2 =
            link_getOffsetHeight(j, (*Link.offset(j as isize)).offset2, elev);
    } else {
        (*Link.offset(j as isize)).offset2 = (*Link.offset(j as isize)).offset1;
    };
}
unsafe extern "C" fn link_getOffsetHeight(
    mut j: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_double,
    mut elev: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if offset <= MISSING || (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int {
        return 0.0f64;
    }
    offset -= elev;
    if offset >= 0.0f64 {
        return offset;
    }
    if offset >= -MIN_DELTA_Z {
        return 0.0f64;
    }
    report_writeWarningMsg(
        WARN03.as_ptr() as *mut ::core::ffi::c_char,
        (*Link.offset(j as isize)).ID,
    );
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn link_initState(mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    (*Link.offset(j as isize)).oldFlow = (*Link.offset(j as isize)).q0;
    (*Link.offset(j as isize)).newFlow = (*Link.offset(j as isize)).q0;
    (*Link.offset(j as isize)).oldDepth = 0.0f64;
    (*Link.offset(j as isize)).newDepth = 0.0f64;
    (*Link.offset(j as isize)).oldVolume = 0.0f64;
    (*Link.offset(j as isize)).newVolume = 0.0f64;
    (*Link.offset(j as isize)).setting = 1.0f64;
    (*Link.offset(j as isize)).targetSetting = 1.0f64;
    (*Link.offset(j as isize)).timeLastSet = StartDate as ::core::ffi::c_double;
    (*Link.offset(j as isize)).inletControl = FALSE as ::core::ffi::c_char;
    (*Link.offset(j as isize)).normalFlow = FALSE as ::core::ffi::c_char;
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        conduit_initState(j, (*Link.offset(j as isize)).subIndex);
    }
    if (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int {
        pump_initState(j, (*Link.offset(j as isize)).subIndex);
    }
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        *(*Link.offset(j as isize)).oldQual.offset(p as isize) = 0.0f64;
        *(*Link.offset(j as isize)).newQual.offset(p as isize) = 0.0f64;
        *(*Link.offset(j as isize)).totalLoad.offset(p as isize) = 0.0f64;
        p += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn link_getInflow(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    if (*Link.offset(j as isize)).setting == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return 0.0f64;
    }
    match (*Link.offset(j as isize)).type_0 {
        0 => return conduit_getInflow(j),
        1 => return pump_getInflow(j),
        2 => return orifice_getInflow(j),
        3 => return weir_getInflow(j),
        4 => return outlet_getInflow(j),
        _ => return node_getOutflow((*Link.offset(j as isize)).node1, j),
    };
}
#[no_mangle]
pub unsafe extern "C" fn link_setOldHydState(mut j: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    (*Link.offset(j as isize)).oldDepth = (*Link.offset(j as isize)).newDepth;
    (*Link.offset(j as isize)).oldFlow = (*Link.offset(j as isize)).newFlow;
    (*Link.offset(j as isize)).oldVolume = (*Link.offset(j as isize)).newVolume;
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        k = (*Link.offset(j as isize)).subIndex;
        (*Conduit.offset(k as isize)).q1Old = (*Conduit.offset(k as isize)).q1;
        (*Conduit.offset(k as isize)).q2Old = (*Conduit.offset(k as isize)).q2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn link_setOldQualState(mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        *(*Link.offset(j as isize)).oldQual.offset(p as isize) =
            *(*Link.offset(j as isize)).newQual.offset(p as isize);
        *(*Link.offset(j as isize)).newQual.offset(p as isize) = 0.0f64;
        p += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn link_setTargetSetting(mut j: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    if (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int {
        k = (*Link.offset(j as isize)).subIndex;
        n1 = (*Link.offset(j as isize)).node1;
        (*Link.offset(j as isize)).targetSetting = (*Link.offset(j as isize)).setting;
        if (*Pump.offset(k as isize)).yOff > 0.0f64
            && (*Link.offset(j as isize)).setting > 0.0f64
            && (*Node.offset(n1 as isize)).newDepth < (*Pump.offset(k as isize)).yOff
        {
            (*Link.offset(j as isize)).targetSetting = 0.0f64;
        }
        if (*Pump.offset(k as isize)).yOn > 0.0f64
            && (*Link.offset(j as isize)).setting == 0.0f64
            && (*Node.offset(n1 as isize)).newDepth > (*Pump.offset(k as isize)).yOn
        {
            (*Link.offset(j as isize)).targetSetting = 1.0f64;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn link_setSetting(
    mut j: ::core::ffi::c_int,
    mut tstep: ::core::ffi::c_double,
) {
    if (*Link.offset(j as isize)).type_0 == ORIFICE as ::core::ffi::c_int {
        orifice_setSetting(j, tstep);
    } else if (*Link.offset(j as isize)).type_0 == WEIR as ::core::ffi::c_int {
        weir_setSetting(j);
    } else {
        (*Link.offset(j as isize)).setting = (*Link.offset(j as isize)).targetSetting;
    };
}
#[no_mangle]
pub unsafe extern "C" fn link_setFlapGate(
    mut j: ::core::ffi::c_int,
    mut n1: ::core::ffi::c_int,
    mut n2: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*Link.offset(j as isize)).hasFlapGate != 0 {
        if (q * (*Link.offset(j as isize)).direction as ::core::ffi::c_double) < 0.0f64 {
            return TRUE;
        }
    }
    if q < 0.0f64 {
        n = n2;
    }
    if q > 0.0f64 {
        n = n1;
    }
    if n >= 0 as ::core::ffi::c_int
        && (*Node.offset(n as isize)).type_0 == OUTFALL as ::core::ffi::c_int
        && (*Outfall.offset((*Node.offset(n as isize)).subIndex as isize)).hasFlapGate
            as ::core::ffi::c_int
            != 0
    {
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn link_getResults(
    mut j: ::core::ffi::c_int,
    mut f: ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_float,
) {
    let mut p: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut u: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    let mut c: ::core::ffi::c_double = 0.;
    let mut f1: ::core::ffi::c_double = 1.0f64 - f;
    y = f1 * (*Link.offset(j as isize)).oldDepth + f * (*Link.offset(j as isize)).newDepth;
    q = f1 * (*Link.offset(j as isize)).oldFlow + f * (*Link.offset(j as isize)).newFlow;
    v = f1 * (*Link.offset(j as isize)).oldVolume + f * (*Link.offset(j as isize)).newVolume;
    u = link_getVelocity(j, q, y);
    c = 0.0f64;
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        if (*Link.offset(j as isize)).xsect.type_0 != DUMMY as ::core::ffi::c_int {
            c = xsect_getAofY(&raw mut (*Link.offset(j as isize)).xsect, y)
                / (*Link.offset(j as isize)).xsect.aFull;
        }
    } else {
        c = (*Link.offset(j as isize)).setting;
    }
    if (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int
        && (*Link.offset(j as isize)).oldFlow * (*Link.offset(j as isize)).newFlow == 0.0f64
    {
        if f >= f1 {
            q = (*Link.offset(j as isize)).newFlow;
        } else {
            q = (*Link.offset(j as isize)).oldFlow;
        }
    }
    y *= UCF(LENGTH as ::core::ffi::c_int);
    v *= UCF(VOLUME as ::core::ffi::c_int);
    q *= UCF(FLOW as ::core::ffi::c_int)
        * (*Link.offset(j as isize)).direction as ::core::ffi::c_double;
    u *= UCF(LENGTH as ::core::ffi::c_int)
        * (*Link.offset(j as isize)).direction as ::core::ffi::c_double;
    *x.offset(LINK_DEPTH as ::core::ffi::c_int as isize) = y as ::core::ffi::c_float;
    *x.offset(LINK_FLOW as ::core::ffi::c_int as isize) = q as ::core::ffi::c_float;
    *x.offset(LINK_VELOCITY as ::core::ffi::c_int as isize) = u as ::core::ffi::c_float;
    *x.offset(LINK_VOLUME as ::core::ffi::c_int as isize) = v as ::core::ffi::c_float;
    *x.offset(LINK_CAPACITY as ::core::ffi::c_int as isize) = c as ::core::ffi::c_float;
    if IgnoreQuality == 0 {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            c = f1 * *(*Link.offset(j as isize)).oldQual.offset(p as isize)
                + f * *(*Link.offset(j as isize)).newQual.offset(p as isize);
            *x.offset((LINK_QUAL as ::core::ffi::c_int + p) as isize) = c as ::core::ffi::c_float;
            p += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn link_setOutfallDepth(mut j: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut yCrit: ::core::ffi::c_double = 0.0f64;
    let mut yNorm: ::core::ffi::c_double = 0.0f64;
    if (*Node.offset((*Link.offset(j as isize)).node2 as isize)).type_0
        == OUTFALL as ::core::ffi::c_int
    {
        n = (*Link.offset(j as isize)).node2;
        z = (*Link.offset(j as isize)).offset2;
    } else if (*Node.offset((*Link.offset(j as isize)).node1 as isize)).type_0
        == OUTFALL as ::core::ffi::c_int
    {
        n = (*Link.offset(j as isize)).node1;
        z = (*Link.offset(j as isize)).offset1;
    } else {
        return;
    }
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        k = (*Link.offset(j as isize)).subIndex;
        q = fabs(
            (*Link.offset(j as isize)).newFlow
                / (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int
                    as ::core::ffi::c_double,
        );
        yNorm = link_getYnorm(j, q);
        yCrit = link_getYcrit(j, q);
    }
    node_setOutletDepth(n, yNorm, yCrit, z);
}
#[no_mangle]
pub unsafe extern "C" fn link_getYcrit(
    mut j: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return xsect_getYcrit(&raw mut (*Link.offset(j as isize)).xsect, q);
}
#[no_mangle]
pub unsafe extern "C" fn link_getYnorm(
    mut j: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_double = 0.;
    let mut a: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    if (*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int {
        return 0.0f64;
    }
    if (*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int {
        return 0.0f64;
    }
    q = fabs(q);
    k = (*Link.offset(j as isize)).subIndex;
    if q > (*Conduit.offset(k as isize)).qMax {
        q = (*Conduit.offset(k as isize)).qMax;
    }
    if q <= 0.0f64 {
        return 0.0f64;
    }
    s = q / (*Conduit.offset(k as isize)).beta;
    a = xsect_getAofS(&raw mut (*Link.offset(j as isize)).xsect, s);
    y = xsect_getYofA(&raw mut (*Link.offset(j as isize)).xsect, a);
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn link_getLength(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        return conduit_getLength(j);
    }
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn link_getVelocity(
    mut j: ::core::ffi::c_int,
    mut flow: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut area: ::core::ffi::c_double = 0.;
    let mut veloc: ::core::ffi::c_double = 0.0f64;
    let mut k: ::core::ffi::c_int = 0;
    if depth <= 0.01f64 {
        return 0.0f64;
    }
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        k = (*Link.offset(j as isize)).subIndex;
        flow /=
            (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int as ::core::ffi::c_double;
        area = xsect_getAofY(&raw mut (*Link.offset(j as isize)).xsect, depth);
        if area > FUDGE {
            veloc = flow / area;
        }
    }
    return veloc;
}
#[no_mangle]
pub unsafe extern "C" fn link_getFroude(
    mut j: ::core::ffi::c_int,
    mut v: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut xsect: *mut TXsect = &raw mut (*Link.offset(j as isize)).xsect;
    if (*Link.offset(j as isize)).type_0 != CONDUIT as ::core::ffi::c_int {
        return 0.0f64;
    }
    if y <= FUDGE {
        return 0.0f64;
    }
    if xsect_isOpen((*xsect).type_0) == 0 && (*xsect).yFull - y <= FUDGE {
        return 0.0f64;
    }
    y = xsect_getAofY(xsect, y) / xsect_getWofY(xsect, y);
    return fabs(v) / sqrt(GRAVITY * y);
}
#[no_mangle]
pub unsafe extern "C" fn link_getPower(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut n1: ::core::ffi::c_int = (*Link.offset(j as isize)).node1;
    let mut n2: ::core::ffi::c_int = (*Link.offset(j as isize)).node2;
    let mut dh: ::core::ffi::c_double = (*Node.offset(n1 as isize)).invertElev
        + (*Node.offset(n1 as isize)).newDepth
        - ((*Node.offset(n2 as isize)).invertElev + (*Node.offset(n2 as isize)).newDepth);
    let mut q: ::core::ffi::c_double = fabs((*Link.offset(j as isize)).newFlow);
    return fabs(dh) * q / 8.814f64 * KWperHP;
}
#[no_mangle]
pub unsafe extern "C" fn link_getLossRate(
    mut j: ::core::ffi::c_int,
    mut routeModel: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
    mut tstep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        return conduit_getLossRate(j, routeModel, q, tstep);
    } else {
        return 0.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn link_getFullState(
    mut a1: ::core::ffi::c_double,
    mut a2: ::core::ffi::c_double,
    mut aFull: ::core::ffi::c_double,
) -> ::core::ffi::c_char {
    if a1 >= aFull {
        if a2 >= aFull {
            return ALL_FULL as ::core::ffi::c_int as ::core::ffi::c_char;
        } else {
            return UP_FULL as ::core::ffi::c_int as ::core::ffi::c_char;
        }
    }
    if a2 >= aFull {
        return DN_FULL as ::core::ffi::c_int as ::core::ffi::c_char;
    }
    return 0 as ::core::ffi::c_char;
}
unsafe extern "C" fn conduit_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 7 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        LINK as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    n1 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if n1 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n2 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    if n2 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(4 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(1 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(4 as ::core::ffi::c_int as isize),
        );
    }
    if LinkOffsets == ELEV_OFFSET as ::core::ffi::c_int
        && **tok.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
    {
        x[2 as ::core::ffi::c_int as usize] = MISSING;
    } else if getDouble(
        *tok.offset(5 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(2 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(5 as ::core::ffi::c_int as isize),
        );
    }
    if LinkOffsets == ELEV_OFFSET as ::core::ffi::c_int
        && **tok.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
    {
        x[3 as ::core::ffi::c_int as usize] = MISSING;
    } else if getDouble(
        *tok.offset(6 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(3 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(6 as ::core::ffi::c_int as isize),
        );
    }
    x[4 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks >= 8 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(7 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(4 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(7 as ::core::ffi::c_int as isize),
            );
        }
    }
    x[5 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks >= 9 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(8 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(5 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(8 as ::core::ffi::c_int as isize),
            );
        }
    }
    let ref mut fresh4 = (*Link.offset(j as isize)).ID;
    *fresh4 = id;
    link_setParams(
        j,
        CONDUIT as ::core::ffi::c_int,
        n1,
        n2,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conduit_validate(mut j: ::core::ffi::c_int, mut k: ::core::ffi::c_int) {
    let mut aa: ::core::ffi::c_double = 0.;
    let mut lengthFactor: ::core::ffi::c_double = 0.;
    let mut roughness: ::core::ffi::c_double = 0.;
    let mut slope: ::core::ffi::c_double = 0.;
    if (*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int
        && RouteModel == DW as ::core::ffi::c_int
    {
        if (*Node.offset((*Link.offset(j as isize)).node1 as isize)).type_0
            == STORAGE as ::core::ffi::c_int
        {
            report_writeErrorMsg(
                ERR_DUMMY_LINK as ::core::ffi::c_int,
                (*Node.offset((*Link.offset(j as isize)).node1 as isize)).ID,
            );
            return;
        }
    }
    if (*Link.offset(j as isize)).xsect.type_0 == CUSTOM as ::core::ffi::c_int {
        xsect_setCustomXsectParams(&raw mut (*Link.offset(j as isize)).xsect);
    }
    if (*Link.offset(j as isize)).xsect.type_0 == IRREGULAR as ::core::ffi::c_int {
        xsect_setIrregXsectParams(&raw mut (*Link.offset(j as isize)).xsect);
        (*Conduit.offset(k as isize)).roughness =
            (*Transect.offset((*Link.offset(j as isize)).xsect.transect as isize)).roughness;
    }
    if (*Link.offset(j as isize)).xsect.type_0 == STREET_XSECT as ::core::ffi::c_int {
        xsect_setStreetXsectParams(&raw mut (*Link.offset(j as isize)).xsect);
        (*Conduit.offset(k as isize)).roughness =
            (*Street.offset((*Link.offset(j as isize)).xsect.transect as isize)).roughness;
    }
    if (*Link.offset(j as isize)).xsect.type_0 == FORCE_MAIN as ::core::ffi::c_int {
        if ForceMainEqn == D_W as ::core::ffi::c_int {
            (*Link.offset(j as isize)).xsect.rBot /= UCF(RAINDEPTH as ::core::ffi::c_int);
        }
        if (*Link.offset(j as isize)).xsect.rBot <= 0.0f64 {
            report_writeErrorMsg(
                ERR_XSECT as ::core::ffi::c_int,
                (*Link.offset(j as isize)).ID,
            );
        }
    }
    if (*Conduit.offset(k as isize)).length <= 0.0f64 {
        report_writeErrorMsg(
            ERR_LENGTH as ::core::ffi::c_int,
            (*Link.offset(j as isize)).ID,
        );
    }
    if (*Conduit.offset(k as isize)).roughness <= 0.0f64 {
        report_writeErrorMsg(
            ERR_ROUGHNESS as ::core::ffi::c_int,
            (*Link.offset(j as isize)).ID,
        );
    }
    if (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        report_writeErrorMsg(
            ERR_BARRELS as ::core::ffi::c_int,
            (*Link.offset(j as isize)).ID,
        );
    }
    if (*Link.offset(j as isize)).xsect.type_0 != DUMMY as ::core::ffi::c_int {
        if (*Link.offset(j as isize)).xsect.type_0 < 0 as ::core::ffi::c_int {
            report_writeErrorMsg(
                ERR_NO_XSECT as ::core::ffi::c_int,
                (*Link.offset(j as isize)).ID,
            );
        } else if (*Link.offset(j as isize)).xsect.aFull <= 0.0f64 {
            report_writeErrorMsg(
                ERR_XSECT as ::core::ffi::c_int,
                (*Link.offset(j as isize)).ID,
            );
        }
    }
    if ErrorCode != 0 {
        return;
    }
    if (*Link.offset(j as isize)).offset1 < 0.0f64 {
        report_writeWarningMsg(
            WARN03.as_ptr() as *mut ::core::ffi::c_char,
            (*Link.offset(j as isize)).ID,
        );
        (*Link.offset(j as isize)).offset1 = 0.0f64;
    }
    if (*Link.offset(j as isize)).offset2 < 0.0f64 {
        report_writeWarningMsg(
            WARN03.as_ptr() as *mut ::core::ffi::c_char,
            (*Link.offset(j as isize)).ID,
        );
        (*Link.offset(j as isize)).offset2 = 0.0f64;
    }
    if (*Link.offset(j as isize)).xsect.type_0 == FILLED_CIRCULAR as ::core::ffi::c_int {
        (*Link.offset(j as isize)).offset1 += (*Link.offset(j as isize)).xsect.yBot;
        (*Link.offset(j as isize)).offset2 += (*Link.offset(j as isize)).xsect.yBot;
    }
    slope = conduit_getSlope(j);
    (*Conduit.offset(k as isize)).slope = slope;
    if RouteModel == DW as ::core::ffi::c_int
        && slope < 0.0f64
        && (*Link.offset(j as isize)).xsect.type_0 != DUMMY as ::core::ffi::c_int
    {
        conduit_reverse(j, k);
    }
    roughness = (*Conduit.offset(k as isize)).roughness;
    if RouteModel == DW as ::core::ffi::c_int
        && (*Link.offset(j as isize)).xsect.type_0 == FORCE_MAIN as ::core::ffi::c_int
    {
        roughness = forcemain_getEquivN(j, k);
    }
    if (*Link.offset(j as isize)).xsect.type_0 == IRREGULAR as ::core::ffi::c_int {
        lengthFactor =
            (*Transect.offset((*Link.offset(j as isize)).xsect.transect as isize)).lengthFactor;
        roughness *= sqrt(lengthFactor);
    }
    lengthFactor = 1.0f64;
    if RouteModel == DW as ::core::ffi::c_int
        && LengtheningStep > 0.0f64
        && (*Link.offset(j as isize)).xsect.type_0 != DUMMY as ::core::ffi::c_int
    {
        lengthFactor = conduit_getLengthFactor(j, k, roughness);
    }
    if lengthFactor != 1.0f64 {
        (*Conduit.offset(k as isize)).modLength = lengthFactor * conduit_getLength(j);
        slope /= lengthFactor;
        roughness = roughness / sqrt(lengthFactor);
    }
    if RouteModel == DW as ::core::ffi::c_int
        && (*Link.offset(j as isize)).xsect.type_0 == FORCE_MAIN as ::core::ffi::c_int
    {
        (*Link.offset(j as isize)).xsect.sBot = forcemain_getRoughFactor(j, lengthFactor);
    }
    (*Conduit.offset(k as isize)).roughFactor =
        GRAVITY * (roughness / 1.486f64 * (roughness / 1.486f64));
    if (*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int {
        (*Conduit.offset(k as isize)).beta = 0.0f64;
    } else {
        (*Conduit.offset(k as isize)).beta = PHI * sqrt(fabs(slope)) / roughness;
    }
    (*Link.offset(j as isize)).qFull =
        (*Link.offset(j as isize)).xsect.sFull * (*Conduit.offset(k as isize)).beta;
    (*Conduit.offset(k as isize)).qMax =
        (*Link.offset(j as isize)).xsect.sMax * (*Conduit.offset(k as isize)).beta;
    aa = (*Conduit.offset(k as isize)).beta / sqrt(32.2f64)
        * pow((*Link.offset(j as isize)).xsect.yFull, 0.1666667f64)
        * 0.3f64;
    if aa >= 1.0f64 {
        (*Conduit.offset(k as isize)).superCritical = TRUE as ::core::ffi::c_char;
    } else {
        (*Conduit.offset(k as isize)).superCritical = FALSE as ::core::ffi::c_char;
    }
    if (*Link.offset(j as isize)).cLossInlet == 0.0f64
        && (*Link.offset(j as isize)).cLossOutlet == 0.0f64
        && (*Link.offset(j as isize)).cLossAvg == 0.0f64
    {
        (*Conduit.offset(k as isize)).hasLosses = FALSE as ::core::ffi::c_char;
    } else {
        (*Conduit.offset(k as isize)).hasLosses = TRUE as ::core::ffi::c_char;
    };
}
unsafe extern "C" fn conduit_reverse(mut j: ::core::ffi::c_int, mut k: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_double = 0.;
    let mut cLoss: ::core::ffi::c_double = 0.;
    i = (*Link.offset(j as isize)).node1;
    (*Link.offset(j as isize)).node1 = (*Link.offset(j as isize)).node2;
    (*Link.offset(j as isize)).node2 = i;
    z = (*Link.offset(j as isize)).offset1;
    (*Link.offset(j as isize)).offset1 = (*Link.offset(j as isize)).offset2;
    (*Link.offset(j as isize)).offset2 = z;
    cLoss = (*Link.offset(j as isize)).cLossInlet;
    (*Link.offset(j as isize)).cLossInlet = (*Link.offset(j as isize)).cLossOutlet;
    (*Link.offset(j as isize)).cLossOutlet = cLoss;
    (*Conduit.offset(k as isize)).slope = -(*Conduit.offset(k as isize)).slope;
    let ref mut fresh5 = (*Link.offset(j as isize)).direction;
    *fresh5 = (*fresh5 as ::core::ffi::c_int
        * -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar as ::core::ffi::c_int)
        as ::core::ffi::c_schar;
    (*Link.offset(j as isize)).q0 = -(*Link.offset(j as isize)).q0;
}
unsafe extern "C" fn conduit_getLength(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = (*Link.offset(j as isize)).subIndex;
    let mut t: ::core::ffi::c_int = 0;
    if (*Link.offset(j as isize)).xsect.type_0 != IRREGULAR as ::core::ffi::c_int {
        return (*Conduit.offset(k as isize)).length;
    }
    t = (*Link.offset(j as isize)).xsect.transect;
    if t < 0 as ::core::ffi::c_int || t >= Nobjects[TRANSECT as ::core::ffi::c_int as usize] {
        return (*Conduit.offset(k as isize)).length;
    }
    return (*Conduit.offset(k as isize)).length / (*Transect.offset(t as isize)).lengthFactor;
}
unsafe extern "C" fn conduit_getLengthFactor(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut roughness: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut ratio: ::core::ffi::c_double = 0.;
    let mut yFull: ::core::ffi::c_double = 0.;
    let mut vFull: ::core::ffi::c_double = 0.;
    let mut tStep: ::core::ffi::c_double = 0.;
    yFull = (*Link.offset(j as isize)).xsect.yFull;
    if xsect_isOpen((*Link.offset(j as isize)).xsect.type_0) != 0 {
        yFull = (*Link.offset(j as isize)).xsect.aFull
            / xsect_getWofY(&raw mut (*Link.offset(j as isize)).xsect, yFull);
    }
    vFull = PHI / roughness
        * (*Link.offset(j as isize)).xsect.sFull
        * sqrt(fabs((*Conduit.offset(k as isize)).slope))
        / (*Link.offset(j as isize)).xsect.aFull;
    if LengtheningStep == 0.0f64 {
        tStep = RouteStep;
    } else {
        tStep = if RouteStep <= LengtheningStep {
            RouteStep
        } else {
            LengtheningStep
        };
    }
    ratio = (sqrt(GRAVITY * yFull) + vFull) * tStep / conduit_getLength(j);
    if ratio > 1.0f64 {
        return ratio;
    } else {
        return 1.0f64;
    };
}
unsafe extern "C" fn conduit_getSlope(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut elev1: ::core::ffi::c_double = 0.;
    let mut elev2: ::core::ffi::c_double = 0.;
    let mut delta: ::core::ffi::c_double = 0.;
    let mut slope: ::core::ffi::c_double = 0.;
    let mut length: ::core::ffi::c_double = conduit_getLength(j);
    elev1 = (*Link.offset(j as isize)).offset1
        + (*Node.offset((*Link.offset(j as isize)).node1 as isize)).invertElev;
    elev2 = (*Link.offset(j as isize)).offset2
        + (*Node.offset((*Link.offset(j as isize)).node2 as isize)).invertElev;
    delta = fabs(elev1 - elev2);
    if delta < MIN_DELTA_Z {
        report_writeWarningMsg(
            WARN04.as_ptr() as *mut ::core::ffi::c_char,
            (*Link.offset(j as isize)).ID,
        );
        delta = MIN_DELTA_Z;
    }
    if delta >= length {
        report_writeWarningMsg(
            WARN08.as_ptr() as *mut ::core::ffi::c_char,
            (*Link.offset(j as isize)).ID,
        );
        slope = delta / length;
    } else {
        slope = delta / sqrt(length * length - delta * delta);
    }
    if MinSlope > 0.0f64 && slope < MinSlope {
        report_writeWarningMsg(
            WARN05.as_ptr() as *mut ::core::ffi::c_char,
            (*Link.offset(j as isize)).ID,
        );
        slope = MinSlope;
        if RouteModel == SF as ::core::ffi::c_int || RouteModel == KW as ::core::ffi::c_int {
            return slope;
        }
    }
    if elev1 < elev2 {
        slope = -slope;
    }
    return slope;
}
unsafe extern "C" fn conduit_initState(mut j: ::core::ffi::c_int, mut k: ::core::ffi::c_int) {
    (*Link.offset(j as isize)).newDepth = link_getYnorm(
        j,
        (*Link.offset(j as isize)).q0
            / (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    (*Link.offset(j as isize)).oldDepth = (*Link.offset(j as isize)).newDepth;
    (*Conduit.offset(k as isize)).evapLossRate = 0.0f64;
    (*Conduit.offset(k as isize)).seepLossRate = 0.0f64;
}
unsafe extern "C" fn conduit_getInflow(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut qIn: ::core::ffi::c_double = node_getOutflow((*Link.offset(j as isize)).node1, j);
    if (*Link.offset(j as isize)).qLimit > 0.0f64 {
        qIn = if qIn <= (*Link.offset(j as isize)).qLimit {
            qIn
        } else {
            (*Link.offset(j as isize)).qLimit
        };
    }
    return qIn;
}
unsafe extern "C" fn conduit_getLossRate(
    mut j: ::core::ffi::c_int,
    mut routeModel: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
    mut tstep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut xsect: *mut TXsect = ::core::ptr::null_mut::<TXsect>();
    let mut depth: ::core::ffi::c_double =
        0.5f64 * ((*Link.offset(j as isize)).oldDepth + (*Link.offset(j as isize)).newDepth);
    let mut length: ::core::ffi::c_double = 0.;
    let mut width: ::core::ffi::c_double = 0.;
    let mut topWidth: ::core::ffi::c_double = 0.;
    let mut evapLossRate: ::core::ffi::c_double = 0.0f64;
    let mut seepLossRate: ::core::ffi::c_double = 0.0f64;
    let mut totalLossRate: ::core::ffi::c_double = 0.0f64;
    if depth > FUDGE {
        xsect = &raw mut (*Link.offset(j as isize)).xsect;
        length = conduit_getLength(j);
        if xsect_isOpen((*xsect).type_0) != 0 && Evap.rate > 0.0f64 {
            topWidth = xsect_getWofY(xsect, depth);
            evapLossRate = topWidth * length * Evap.rate;
        }
        if (*Link.offset(j as isize)).seepRate > 0.0f64 {
            if (*xsect).type_0 == RECT_CLOSED as ::core::ffi::c_int {
                width = (*xsect).wMax;
            } else {
                if depth >= (*xsect).ywMax {
                    depth = (*xsect).ywMax;
                }
                width = xsect_getWofY(xsect, depth);
            }
            seepLossRate = (*Link.offset(j as isize)).seepRate * width * length;
            seepLossRate *= Adjust.hydconFactor;
        }
        totalLossRate = evapLossRate + seepLossRate;
        if routeModel == DW as ::core::ffi::c_int {
            q = (*Link.offset(j as isize)).newVolume / tstep;
        } else {
            q = if q < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -q
            } else {
                q
            };
        }
        if totalLossRate > q {
            evapLossRate = evapLossRate * q / totalLossRate;
            seepLossRate = seepLossRate * q / totalLossRate;
            totalLossRate = q;
        }
    }
    (*Conduit.offset((*Link.offset(j as isize)).subIndex as isize)).evapLossRate = evapLossRate;
    (*Conduit.offset((*Link.offset(j as isize)).subIndex as isize)).seepLossRate = seepLossRate;
    return totalLossRate;
}
unsafe extern "C" fn pump_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut m: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 4] = [0.; 4];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        LINK as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    n1 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if n1 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n2 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    if n2 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    x[0 as ::core::ffi::c_int as usize] = -1.0f64;
    if ntoks >= 4 as ::core::ffi::c_int {
        if strcomp(
            *tok.offset(3 as ::core::ffi::c_int as isize),
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
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
            x[0 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
        }
    }
    x[1 as ::core::ffi::c_int as usize] = 1.0f64;
    if ntoks >= 5 as ::core::ffi::c_int {
        m = findmatch(
            *tok.offset(4 as ::core::ffi::c_int as isize),
            &raw mut OffOnWords as *mut *mut ::core::ffi::c_char,
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(4 as ::core::ffi::c_int as isize),
            );
        }
        x[1 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    }
    x[2 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks >= 6 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(5 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(2 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[2 as ::core::ffi::c_int as usize] < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(5 as ::core::ffi::c_int as isize),
            );
        }
    }
    x[3 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks >= 7 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(6 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(3 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[3 as ::core::ffi::c_int as usize] < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
        }
    }
    let ref mut fresh3 = (*Link.offset(j as isize)).ID;
    *fresh3 = id;
    link_setParams(
        j,
        PUMP as ::core::ffi::c_int,
        n1,
        n2,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn pump_validate(mut j: ::core::ffi::c_int, mut k: ::core::ffi::c_int) {
    let mut m: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    (*Link.offset(j as isize)).xsect.yFull = 0.0f64;
    m = (*Pump.offset(k as isize)).pumpCurve;
    if m < 0 as ::core::ffi::c_int {
        (*Pump.offset(k as isize)).type_0 = IDEAL_PUMP as ::core::ffi::c_int;
    } else if (*Curve.offset(m as isize)).curveType < PUMP1_CURVE as ::core::ffi::c_int
        || (*Curve.offset(m as isize)).curveType > PUMP5_CURVE as ::core::ffi::c_int
    {
        report_writeErrorMsg(
            ERR_NO_CURVE as ::core::ffi::c_int,
            (*Link.offset(j as isize)).ID,
        );
    } else {
        (*Pump.offset(k as isize)).type_0 =
            (*Curve.offset(m as isize)).curveType - PUMP1_CURVE as ::core::ffi::c_int;
        if table_getFirstEntry(
            Curve.offset(m as isize) as *mut TTable,
            &raw mut x,
            &raw mut y,
        ) != 0
        {
            (*Link.offset(j as isize)).qFull = y;
            (*Pump.offset(k as isize)).xMin = x;
            (*Pump.offset(k as isize)).xMax = x;
            while table_getNextEntry(
                Curve.offset(m as isize) as *mut TTable,
                &raw mut x,
                &raw mut y,
            ) != 0
            {
                (*Link.offset(j as isize)).qFull = if y >= (*Link.offset(j as isize)).qFull {
                    y
                } else {
                    (*Link.offset(j as isize)).qFull
                };
                (*Pump.offset(k as isize)).xMax = x;
            }
        }
        (*Link.offset(j as isize)).qFull /= UCF(FLOW as ::core::ffi::c_int);
    }
    if (*Pump.offset(k as isize)).yOn > 0.0f64
        && (*Pump.offset(k as isize)).yOn <= (*Pump.offset(k as isize)).yOff
    {
        report_writeErrorMsg(
            ERR_PUMP_LIMITS as ::core::ffi::c_int,
            (*Link.offset(j as isize)).ID,
        );
    }
    if (*Pump.offset(k as isize)).type_0 == TYPE1_PUMP as ::core::ffi::c_int {
        n1 = (*Link.offset(j as isize)).node1;
        if (*Node.offset(n1 as isize)).type_0 != STORAGE as ::core::ffi::c_int {
            (*Node.offset(n1 as isize)).fullVolume = if (*Node.offset(n1 as isize)).fullVolume
                >= (*Pump.offset(k as isize)).xMax / UCF(VOLUME as ::core::ffi::c_int)
            {
                (*Node.offset(n1 as isize)).fullVolume
            } else {
                (*Pump.offset(k as isize)).xMax / UCF(VOLUME as ::core::ffi::c_int)
            };
        }
    }
}
unsafe extern "C" fn pump_initState(mut j: ::core::ffi::c_int, mut k: ::core::ffi::c_int) {
    (*Link.offset(j as isize)).setting = (*Pump.offset(k as isize)).initSetting;
    (*Link.offset(j as isize)).targetSetting = (*Pump.offset(k as isize)).initSetting;
}
unsafe extern "C" fn pump_getInflow(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut vol: ::core::ffi::c_double = 0.;
    let mut depth: ::core::ffi::c_double = 0.;
    let mut head: ::core::ffi::c_double = 0.;
    let mut qIn: ::core::ffi::c_double = 0.;
    let mut qIn1: ::core::ffi::c_double = 0.;
    let mut dh: ::core::ffi::c_double = 0.001f64;
    let mut s: ::core::ffi::c_double = 1.0f64;
    k = (*Link.offset(j as isize)).subIndex;
    m = (*Pump.offset(k as isize)).pumpCurve;
    n1 = (*Link.offset(j as isize)).node1;
    n2 = (*Link.offset(j as isize)).node2;
    (*Link.offset(j as isize)).flowClass = NO as ::core::ffi::c_int;
    (*Link.offset(j as isize)).setting = (*Link.offset(j as isize)).targetSetting;
    if (*Link.offset(j as isize)).setting == 0.0f64 {
        return 0.0f64;
    }
    if (*Pump.offset(k as isize)).type_0 == IDEAL_PUMP as ::core::ffi::c_int {
        qIn = (*Node.offset(n1 as isize)).inflow + (*Node.offset(n1 as isize)).overflow;
    } else {
        match (*Curve.offset(m as isize)).curveType {
            7 => {
                vol = (*Node.offset(n1 as isize)).newVolume * UCF(VOLUME as ::core::ffi::c_int);
                qIn = table_intervalLookup(Curve.offset(m as isize) as *mut TTable, vol)
                    / UCF(FLOW as ::core::ffi::c_int);
                if vol < (*Pump.offset(k as isize)).xMin || vol > (*Pump.offset(k as isize)).xMax {
                    (*Link.offset(j as isize)).flowClass = YES as ::core::ffi::c_int;
                }
            }
            8 => {
                depth = (*Node.offset(n1 as isize)).newDepth * UCF(LENGTH as ::core::ffi::c_int);
                qIn = table_intervalLookup(Curve.offset(m as isize) as *mut TTable, depth)
                    / UCF(FLOW as ::core::ffi::c_int);
                if depth < (*Pump.offset(k as isize)).xMin
                    || depth > (*Pump.offset(k as isize)).xMax
                {
                    (*Link.offset(j as isize)).flowClass = YES as ::core::ffi::c_int;
                }
            }
            9 | 11 => {
                if (*Curve.offset(m as isize)).curveType == PUMP5_CURVE as ::core::ffi::c_int {
                    s = (*Link.offset(j as isize)).setting;
                }
                head = ((*Node.offset(n2 as isize)).newDepth
                    + (*Node.offset(n2 as isize)).invertElev
                    - ((*Node.offset(n1 as isize)).newDepth
                        + (*Node.offset(n1 as isize)).invertElev))
                    / s
                    / s;
                head = (if head >= 0.0f64 { head } else { 0.0f64 })
                    * UCF(LENGTH as ::core::ffi::c_int);
                qIn = table_lookup(Curve.offset(m as isize) as *mut TTable, head)
                    / UCF(FLOW as ::core::ffi::c_int);
                (*Link.offset(j as isize)).dqdh =
                    -table_getSlope(Curve.offset(m as isize) as *mut TTable, head)
                        * UCF(LENGTH as ::core::ffi::c_int)
                        / UCF(FLOW as ::core::ffi::c_int)
                        / s;
                if head < (*Pump.offset(k as isize)).xMin || head > (*Pump.offset(k as isize)).xMax
                {
                    (*Link.offset(j as isize)).flowClass = YES as ::core::ffi::c_int;
                }
            }
            10 => {
                depth = (*Node.offset(n1 as isize)).newDepth;
                qIn = table_lookup(
                    Curve.offset(m as isize) as *mut TTable,
                    depth * UCF(LENGTH as ::core::ffi::c_int),
                ) / UCF(FLOW as ::core::ffi::c_int);
                qIn1 = table_lookup(
                    Curve.offset(m as isize) as *mut TTable,
                    (depth + dh) * UCF(LENGTH as ::core::ffi::c_int),
                ) / UCF(FLOW as ::core::ffi::c_int);
                (*Link.offset(j as isize)).dqdh = (qIn1 - qIn) / dh;
                depth *= UCF(LENGTH as ::core::ffi::c_int);
                if depth < (*Pump.offset(k as isize)).xMin {
                    (*Link.offset(j as isize)).flowClass = DN_DRY as ::core::ffi::c_int;
                }
                if depth > (*Pump.offset(k as isize)).xMax {
                    (*Link.offset(j as isize)).flowClass = UP_DRY as ::core::ffi::c_int;
                }
            }
            _ => {
                qIn = 0.0f64;
            }
        }
    }
    if qIn < 0.0f64 {
        qIn = 0.0f64;
    }
    return qIn * (*Link.offset(j as isize)).setting;
}
unsafe extern "C" fn orifice_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut m: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 5] = [0.; 5];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        LINK as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    n1 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if n1 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n2 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    if n2 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    m = findmatch(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        &raw mut OrificeTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if m < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    x[0 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    if LinkOffsets == ELEV_OFFSET as ::core::ffi::c_int
        && **tok.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
    {
        x[1 as ::core::ffi::c_int as usize] = MISSING;
    } else if getDouble(
        *tok.offset(4 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(1 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(4 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(5 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(2 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
        || x[2 as ::core::ffi::c_int as usize] < 0.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(5 as ::core::ffi::c_int as isize),
        );
    }
    x[3 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks >= 7 as ::core::ffi::c_int {
        m = findmatch(
            *tok.offset(6 as ::core::ffi::c_int as isize),
            &raw mut NoYesWords as *mut *mut ::core::ffi::c_char,
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
        }
        x[3 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    }
    x[4 as ::core::ffi::c_int as usize] = 0.0f64;
    if ntoks >= 8 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(7 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(4 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[4 as ::core::ffi::c_int as usize] < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(7 as ::core::ffi::c_int as isize),
            );
        }
    }
    let ref mut fresh2 = (*Link.offset(j as isize)).ID;
    *fresh2 = id;
    link_setParams(
        j,
        ORIFICE as ::core::ffi::c_int,
        n1,
        n2,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn orifice_validate(mut j: ::core::ffi::c_int, mut k: ::core::ffi::c_int) {
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*Link.offset(j as isize)).xsect.type_0 != RECT_CLOSED as ::core::ffi::c_int
        && (*Link.offset(j as isize)).xsect.type_0 != CIRCULAR as ::core::ffi::c_int
    {
        err = ERR_REGULATOR_SHAPE as ::core::ffi::c_int;
    }
    if err > 0 as ::core::ffi::c_int {
        report_writeErrorMsg(err, (*Link.offset(j as isize)).ID);
        return;
    }
    if (*Link.offset(j as isize)).offset1 < 0.0f64 {
        (*Link.offset(j as isize)).offset1 = 0.0f64;
    }
    orifice_setSetting(j, 0.0f64);
    (*Orifice.offset(k as isize)).length =
        2.0f64 * RouteStep * sqrt(GRAVITY * (*Link.offset(j as isize)).xsect.yFull);
    (*Orifice.offset(k as isize)).length = if 200.0f64 >= (*Orifice.offset(k as isize)).length {
        200.0f64
    } else {
        (*Orifice.offset(k as isize)).length
    };
    (*Orifice.offset(k as isize)).surfArea = 0.0f64;
}
unsafe extern "C" fn orifice_setSetting(
    mut j: ::core::ffi::c_int,
    mut tstep: ::core::ffi::c_double,
) {
    let mut k: ::core::ffi::c_int = (*Link.offset(j as isize)).subIndex;
    let mut delta: ::core::ffi::c_double = 0.;
    let mut step: ::core::ffi::c_double = 0.;
    let mut h: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    if (*Orifice.offset(k as isize)).orate == 0.0f64 || tstep == 0.0f64 {
        (*Link.offset(j as isize)).setting = (*Link.offset(j as isize)).targetSetting;
    } else {
        delta = (*Link.offset(j as isize)).targetSetting - (*Link.offset(j as isize)).setting;
        step = tstep / (*Orifice.offset(k as isize)).orate;
        if step + 0.001f64 >= fabs(delta) {
            (*Link.offset(j as isize)).setting = (*Link.offset(j as isize)).targetSetting;
        } else {
            (*Link.offset(j as isize)).setting +=
                (if delta < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_double
                    * step;
        }
    }
    h = (*Link.offset(j as isize)).setting * (*Link.offset(j as isize)).xsect.yFull;
    f = xsect_getAofY(&raw mut (*Link.offset(j as isize)).xsect, h) * sqrt(2.0f64 * GRAVITY);
    (*Orifice.offset(k as isize)).cOrif = (*Orifice.offset(k as isize)).cDisch * f;
    (*Orifice.offset(k as isize)).cWeir = orifice_getWeirCoeff(j, k, h) * f;
}
unsafe extern "C" fn orifice_getWeirCoeff(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut h: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut w: ::core::ffi::c_double = 0.;
    let mut aOverL: ::core::ffi::c_double = 0.;
    if (*Orifice.offset(k as isize)).type_0 == BOTTOM_ORIFICE as ::core::ffi::c_int {
        if (*Link.offset(j as isize)).xsect.type_0 == CIRCULAR as ::core::ffi::c_int {
            aOverL = h / 4.0f64;
        } else {
            w = (*Link.offset(j as isize)).xsect.wMax;
            aOverL = h * w / (2.0f64 * (h + w));
        }
        h = (*Orifice.offset(k as isize)).cDisch / 0.414f64 * aOverL;
        (*Orifice.offset(k as isize)).hCrit = h;
    } else {
        (*Orifice.offset(k as isize)).hCrit = h;
        h = h / 2.0f64;
    }
    return (*Orifice.offset(k as isize)).cDisch * sqrt(h);
}
unsafe extern "C" fn orifice_getInflow(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut head: ::core::ffi::c_double = 0.;
    let mut h1: ::core::ffi::c_double = 0.;
    let mut h2: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut dir: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut hcrest: ::core::ffi::c_double = 0.0f64;
    let mut hcrown: ::core::ffi::c_double = 0.0f64;
    let mut hmidpt: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut ratio: ::core::ffi::c_double = 0.;
    n1 = (*Link.offset(j as isize)).node1;
    n2 = (*Link.offset(j as isize)).node2;
    k = (*Link.offset(j as isize)).subIndex;
    if RouteModel == DW as ::core::ffi::c_int {
        h1 = (*Node.offset(n1 as isize)).newDepth + (*Node.offset(n1 as isize)).invertElev;
        h2 = (*Node.offset(n2 as isize)).newDepth + (*Node.offset(n2 as isize)).invertElev;
    } else {
        h1 = (*Node.offset(n1 as isize)).newDepth + (*Node.offset(n1 as isize)).invertElev;
        h2 = (*Node.offset(n1 as isize)).invertElev;
    }
    dir = if h1 >= h2 { 1.0f64 } else { -1.0f64 };
    y1 = (*Node.offset(n1 as isize)).newDepth;
    if dir < 0.0f64 {
        head = h1;
        h1 = h2;
        h2 = head;
        y1 = (*Node.offset(n2 as isize)).newDepth;
    }
    if (*Orifice.offset(k as isize)).type_0 == BOTTOM_ORIFICE as ::core::ffi::c_int {
        hcrest = (*Node.offset(n1 as isize)).invertElev + (*Link.offset(j as isize)).offset1;
        if h1 < hcrest {
            head = 0.0f64;
        } else if h2 > hcrest {
            head = h1 - h2;
        } else {
            head = h1 - hcrest;
        }
        f = head / (*Orifice.offset(k as isize)).hCrit;
        f = if f <= 1.0f64 { f } else { 1.0f64 };
    } else {
        hcrest = (*Node.offset(n1 as isize)).invertElev + (*Link.offset(j as isize)).offset1;
        hcrown =
            hcrest + (*Link.offset(j as isize)).xsect.yFull * (*Link.offset(j as isize)).setting;
        hmidpt = (hcrest + hcrown) / 2.0f64;
        if h1 < hcrown && hcrown > hcrest {
            f = (h1 - hcrest) / (hcrown - hcrest);
        } else {
            f = 1.0f64;
        }
        if f < 1.0f64 {
            head = h1 - hcrest;
        } else if h2 < hmidpt {
            head = h1 - hmidpt;
        } else {
            head = h1 - h2;
        }
    }
    if head <= FUDGE || y1 <= FUDGE || link_setFlapGate(j, n1, n2, dir) != 0 {
        (*Link.offset(j as isize)).newDepth = 0.0f64;
        (*Link.offset(j as isize)).flowClass = DRY as ::core::ffi::c_int;
        (*Orifice.offset(k as isize)).surfArea = FUDGE * (*Orifice.offset(k as isize)).length;
        (*Link.offset(j as isize)).dqdh = 0.0f64;
        return 0.0f64;
    }
    (*Link.offset(j as isize)).flowClass = SUBCRITICAL as ::core::ffi::c_int;
    if hcrest > h2 {
        if dir == 1.0f64 {
            (*Link.offset(j as isize)).flowClass = DN_CRITICAL as ::core::ffi::c_int;
        } else {
            (*Link.offset(j as isize)).flowClass = UP_CRITICAL as ::core::ffi::c_int;
        }
    }
    y1 = (*Link.offset(j as isize)).xsect.yFull * (*Link.offset(j as isize)).setting;
    if (*Orifice.offset(k as isize)).type_0 == SIDE_ORIFICE as ::core::ffi::c_int {
        (*Link.offset(j as isize)).newDepth = y1 * f;
        (*Orifice.offset(k as isize)).surfArea = xsect_getWofY(
            &raw mut (*Link.offset(j as isize)).xsect,
            (*Link.offset(j as isize)).newDepth,
        ) * (*Orifice.offset(k as isize)).length;
    } else {
        (*Link.offset(j as isize)).newDepth = y1;
        (*Orifice.offset(k as isize)).surfArea =
            xsect_getAofY(&raw mut (*Link.offset(j as isize)).xsect, y1);
    }
    q = dir * orifice_getFlow(j, k, head, f, (*Link.offset(j as isize)).hasFlapGate);
    if f < 1.0f64 && h2 > hcrest {
        ratio = (h2 - hcrest) / (h1 - hcrest);
        q *= pow(1.0f64 - pow(ratio, 1.5f64), 0.385f64);
    }
    return q;
}
unsafe extern "C" fn orifice_getFlow(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut head: ::core::ffi::c_double,
    mut f: ::core::ffi::c_double,
    mut hasFlapGate: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut area: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut veloc: ::core::ffi::c_double = 0.;
    let mut hLoss: ::core::ffi::c_double = 0.;
    if head == 0.0f64 || f <= 0.0f64 {
        (*Link.offset(j as isize)).dqdh = 0.0f64;
        return 0.0f64;
    } else if f < 1.0f64 {
        q = (*Orifice.offset(k as isize)).cWeir * pow(f, 1.5f64);
        (*Link.offset(j as isize)).dqdh = 1.5f64 * q / (f * (*Orifice.offset(k as isize)).hCrit);
    } else {
        q = (*Orifice.offset(k as isize)).cOrif * sqrt(head);
        (*Link.offset(j as isize)).dqdh = q / (2.0f64 * head);
    }
    if hasFlapGate != 0 {
        area = xsect_getAofY(
            &raw mut (*Link.offset(j as isize)).xsect,
            (*Link.offset(j as isize)).setting * (*Link.offset(j as isize)).xsect.yFull,
        );
        veloc = q / area;
        hLoss = 4.0f64 / GRAVITY * veloc * veloc * exp(-1.15f64 * veloc / sqrt(head));
        if f < 1.0f64 {
            f = f - hLoss / (*Orifice.offset(k as isize)).hCrit;
            if f < 0.0f64 {
                f = 0.0f64;
            }
        } else {
            head = head - hLoss;
            if head < 0.0f64 {
                head = 0.0f64;
            }
        }
        q = orifice_getFlow(j, k, head, f, FALSE);
    }
    return q;
}
unsafe extern "C" fn weir_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut m: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 10] = [0.; 10];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        LINK as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    n1 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if n1 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n2 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    if n2 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    m = findmatch(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        &raw mut WeirTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if m < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    x[0 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    if LinkOffsets == ELEV_OFFSET as ::core::ffi::c_int
        && **tok.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
    {
        x[1 as ::core::ffi::c_int as usize] = MISSING;
    } else if getDouble(
        *tok.offset(4 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(1 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(4 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(5 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(2 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
        || x[2 as ::core::ffi::c_int as usize] < 0.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(5 as ::core::ffi::c_int as isize),
        );
    }
    x[3 as ::core::ffi::c_int as usize] = 0.0f64;
    x[4 as ::core::ffi::c_int as usize] = 0.0f64;
    x[5 as ::core::ffi::c_int as usize] = 0.0f64;
    x[6 as ::core::ffi::c_int as usize] = 1.0f64;
    x[7 as ::core::ffi::c_int as usize] = 0.0f64;
    x[8 as ::core::ffi::c_int as usize] = 0.0f64;
    x[9 as ::core::ffi::c_int as usize] = -1.0f64;
    if ntoks >= 7 as ::core::ffi::c_int
        && **tok.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '*' as i32
    {
        m = findmatch(
            *tok.offset(6 as ::core::ffi::c_int as isize),
            &raw mut NoYesWords as *mut *mut ::core::ffi::c_char,
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
        }
        x[3 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    }
    if ntoks >= 8 as ::core::ffi::c_int
        && **tok.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '*' as i32
    {
        if getDouble(
            *tok.offset(7 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(4 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[4 as ::core::ffi::c_int as usize] < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(7 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks >= 9 as ::core::ffi::c_int
        && **tok.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '*' as i32
    {
        if getDouble(
            *tok.offset(8 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(5 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[5 as ::core::ffi::c_int as usize] < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(8 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks >= 10 as ::core::ffi::c_int
        && **tok.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '*' as i32
    {
        m = findmatch(
            *tok.offset(9 as ::core::ffi::c_int as isize),
            &raw mut NoYesWords as *mut *mut ::core::ffi::c_char,
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(9 as ::core::ffi::c_int as isize),
            );
        }
        x[6 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    }
    m = crate::ccast::f64_to_i32(x[0 as ::core::ffi::c_int as usize]);
    if m == ROADWAY_WEIR as ::core::ffi::c_int {
        if ntoks >= 11 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset(10 as ::core::ffi::c_int as isize),
                (&raw mut x as *mut ::core::ffi::c_double).offset(7 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
                || x[7 as ::core::ffi::c_int as usize] < 0.0f64
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(10 as ::core::ffi::c_int as isize),
                );
            }
        }
        if ntoks >= 12 as ::core::ffi::c_int {
            if strcomp(
                *tok.offset(11 as ::core::ffi::c_int as isize),
                b"PAVED\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
            {
                x[8 as ::core::ffi::c_int as usize] = 1.0f64;
            } else if strcomp(
                *tok.offset(11 as ::core::ffi::c_int as isize),
                b"GRAVEL\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
            {
                x[8 as ::core::ffi::c_int as usize] = 2.0f64;
            }
        }
    }
    if ntoks >= 13 as ::core::ffi::c_int
        && **tok.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '*' as i32
    {
        m = project_findObject(
            CURVE as ::core::ffi::c_int,
            *tok.offset(12 as ::core::ffi::c_int as isize),
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(12 as ::core::ffi::c_int as isize),
            );
        }
        x[9 as ::core::ffi::c_int as usize] = m as ::core::ffi::c_double;
    }
    let ref mut fresh1 = (*Link.offset(j as isize)).ID;
    *fresh1 = id;
    link_setParams(
        j,
        WEIR as ::core::ffi::c_int,
        n1,
        n2,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn weir_validate(mut j: ::core::ffi::c_int, mut k: ::core::ffi::c_int) {
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut q: ::core::ffi::c_double = 0.;
    let mut q1: ::core::ffi::c_double = 0.;
    let mut q2: ::core::ffi::c_double = 0.;
    let mut head: ::core::ffi::c_double = 0.;
    match (*Weir.offset(k as isize)).type_0 {
        0 | 1 | 4 => {
            if (*Link.offset(j as isize)).xsect.type_0 != RECT_OPEN as ::core::ffi::c_int {
                err = ERR_REGULATOR_SHAPE as ::core::ffi::c_int;
            }
            (*Weir.offset(k as isize)).slope = 0.0f64;
        }
        2 => {
            if (*Link.offset(j as isize)).xsect.type_0 != TRIANGULAR as ::core::ffi::c_int {
                err = ERR_REGULATOR_SHAPE as ::core::ffi::c_int;
            } else {
                (*Weir.offset(k as isize)).slope = (*Link.offset(j as isize)).xsect.sBot;
            }
        }
        3 => {
            if (*Link.offset(j as isize)).xsect.type_0 != TRAPEZOIDAL as ::core::ffi::c_int {
                err = ERR_REGULATOR_SHAPE as ::core::ffi::c_int;
            } else {
                (*Weir.offset(k as isize)).slope = (*Link.offset(j as isize)).xsect.sBot;
            }
        }
        _ => {}
    }
    if err > 0 as ::core::ffi::c_int {
        report_writeErrorMsg(err, (*Link.offset(j as isize)).ID);
        return;
    }
    if (*Link.offset(j as isize)).offset1 < 0.0f64 {
        (*Link.offset(j as isize)).offset1 = 0.0f64;
    }
    (*Weir.offset(k as isize)).length =
        2.0f64 * RouteStep * sqrt(GRAVITY * (*Link.offset(j as isize)).xsect.yFull);
    (*Weir.offset(k as isize)).length = if 200.0f64 >= (*Weir.offset(k as isize)).length {
        200.0f64
    } else {
        (*Weir.offset(k as isize)).length
    };
    (*Weir.offset(k as isize)).surfArea = 0.0f64;
    head = (*Link.offset(j as isize)).xsect.yFull;
    weir_getFlow(j, k, head, 1.0f64, FALSE, &raw mut q1, &raw mut q2);
    q = q1 + q2;
    head = head / 2.0f64;
    (*Weir.offset(k as isize)).cSurcharge = q / sqrt(head);
}
unsafe extern "C" fn weir_setSetting(mut j: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = (*Link.offset(j as isize)).subIndex;
    let mut h: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut q1: ::core::ffi::c_double = 0.;
    let mut q2: ::core::ffi::c_double = 0.;
    (*Link.offset(j as isize)).setting = (*Link.offset(j as isize)).targetSetting;
    if (*Weir.offset(k as isize)).canSurcharge == 0 {
        return;
    }
    if (*Weir.offset(k as isize)).type_0 == ROADWAY_WEIR as ::core::ffi::c_int {
        return;
    }
    if (*Link.offset(j as isize)).setting == 0.0f64 {
        (*Weir.offset(k as isize)).cSurcharge = 0.0f64;
    } else {
        h = (*Link.offset(j as isize)).setting * (*Link.offset(j as isize)).xsect.yFull;
        weir_getFlow(j, k, h, 1.0f64, FALSE, &raw mut q1, &raw mut q2);
        q = q1 + q2;
        h = h / 2.0f64;
        (*Weir.offset(k as isize)).cSurcharge = q / sqrt(h);
    };
}
unsafe extern "C" fn weir_getInflow(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut q1: ::core::ffi::c_double = 0.;
    let mut q2: ::core::ffi::c_double = 0.;
    let mut head: ::core::ffi::c_double = 0.;
    let mut h1: ::core::ffi::c_double = 0.;
    let mut h2: ::core::ffi::c_double = 0.;
    let mut hcrest: ::core::ffi::c_double = 0.;
    let mut hcrown: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut dir: ::core::ffi::c_double = 0.;
    let mut ratio: ::core::ffi::c_double = 0.;
    let mut weirPower: [::core::ffi::c_double; 4] = [1.5f64, 5.0f64 / 3.0f64, 2.5f64, 1.5f64];
    n1 = (*Link.offset(j as isize)).node1;
    n2 = (*Link.offset(j as isize)).node2;
    k = (*Link.offset(j as isize)).subIndex;
    if RouteModel == DW as ::core::ffi::c_int {
        h1 = (*Node.offset(n1 as isize)).newDepth + (*Node.offset(n1 as isize)).invertElev;
        h2 = (*Node.offset(n2 as isize)).newDepth + (*Node.offset(n2 as isize)).invertElev;
    } else {
        h1 = (*Node.offset(n1 as isize)).newDepth + (*Node.offset(n1 as isize)).invertElev;
        h2 = (*Node.offset(n1 as isize)).invertElev;
    }
    dir = if h1 > h2 { 1.0f64 } else { -1.0f64 };
    if dir < 0.0f64 {
        head = h1;
        h1 = h2;
        h2 = head;
    }
    hcrest = (*Node.offset(n1 as isize)).invertElev + (*Link.offset(j as isize)).offset1;
    hcrown = hcrest + (*Link.offset(j as isize)).xsect.yFull;
    if (*Weir.offset(k as isize)).type_0 == ROADWAY_WEIR as ::core::ffi::c_int {
        return roadway_getInflow(j, dir, hcrest, h1, h2);
    }
    hcrest +=
        (1.0f64 - (*Link.offset(j as isize)).setting) * (*Link.offset(j as isize)).xsect.yFull;
    head = h1 - hcrest;
    (*Link.offset(j as isize)).dqdh = 0.0f64;
    if head <= FUDGE || hcrest >= hcrown || link_setFlapGate(j, n1, n2, dir) != 0 {
        (*Link.offset(j as isize)).newDepth = 0.0f64;
        (*Link.offset(j as isize)).flowClass = DRY as ::core::ffi::c_int;
        return 0.0f64;
    }
    (*Link.offset(j as isize)).flowClass = SUBCRITICAL as ::core::ffi::c_int;
    if hcrest > h2 {
        if dir == 1.0f64 {
            (*Link.offset(j as isize)).flowClass = DN_CRITICAL as ::core::ffi::c_int;
        } else {
            (*Link.offset(j as isize)).flowClass = UP_CRITICAL as ::core::ffi::c_int;
        }
    }
    y = (*Link.offset(j as isize)).xsect.yFull
        - (hcrown - (if h1 <= hcrown { h1 } else { hcrown }));
    (*Weir.offset(k as isize)).surfArea =
        xsect_getWofY(&raw mut (*Link.offset(j as isize)).xsect, y)
            * (*Weir.offset(k as isize)).length;
    if h1 >= hcrown {
        if (*Weir.offset(k as isize)).canSurcharge != 0 {
            y = (hcrest + hcrown) / 2.0f64;
            if h2 < y {
                head = h1 - y;
            } else {
                head = h1 - h2;
            }
            y = hcrown - hcrest;
            q1 = weir_getOrificeFlow(j, head, y, (*Weir.offset(k as isize)).cSurcharge);
            (*Link.offset(j as isize)).newDepth = y;
            return dir * q1;
        } else {
            head = hcrown - hcrest;
        }
    }
    weir_getFlow(
        j,
        k,
        head,
        dir,
        (*Link.offset(j as isize)).hasFlapGate,
        &raw mut q1,
        &raw mut q2,
    );
    if h2 > hcrest {
        ratio = (h2 - hcrest) / (h1 - hcrest);
        q1 *= pow(
            1.0f64 - pow(ratio, weirPower[(*Weir.offset(k as isize)).type_0 as usize]),
            0.385f64,
        );
        if q2 > 0.0f64 {
            q2 *= pow(
                1.0f64 - pow(ratio, weirPower[VNOTCH_WEIR as ::core::ffi::c_int as usize]),
                0.385f64,
            );
        }
    }
    (*Link.offset(j as isize)).newDepth = if h1 - hcrest <= (*Link.offset(j as isize)).xsect.yFull {
        h1 - hcrest
    } else {
        (*Link.offset(j as isize)).xsect.yFull
    };
    return dir * (q1 + q2);
}
unsafe extern "C" fn weir_getFlow(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut head: ::core::ffi::c_double,
    mut dir: ::core::ffi::c_double,
    mut hasFlapGate: ::core::ffi::c_int,
    mut q1: *mut ::core::ffi::c_double,
    mut q2: *mut ::core::ffi::c_double,
) {
    let mut length: ::core::ffi::c_double = 0.;
    let mut h: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut hLoss: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = 0.;
    let mut veloc: ::core::ffi::c_double = 0.;
    let mut wType: ::core::ffi::c_int = 0;
    let mut cdCurve: ::core::ffi::c_int = (*Weir.offset(k as isize)).cdCurve;
    let mut cDisch1: ::core::ffi::c_double = (*Weir.offset(k as isize)).cDisch1;
    *q1 = 0.0f64;
    *q2 = 0.0f64;
    (*Link.offset(j as isize)).dqdh = 0.0f64;
    if head <= 0.0f64 {
        return;
    }
    length = (*Link.offset(j as isize)).xsect.wMax * UCF(LENGTH as ::core::ffi::c_int);
    h = head * UCF(LENGTH as ::core::ffi::c_int);
    if cdCurve >= 0 as ::core::ffi::c_int {
        cDisch1 = table_lookup(Curve.offset(cdCurve as isize) as *mut TTable, h);
    }
    wType = (*Weir.offset(k as isize)).type_0;
    if wType == VNOTCH_WEIR as ::core::ffi::c_int && (*Link.offset(j as isize)).setting < 1.0f64 {
        wType = TRAPEZOIDAL_WEIR as ::core::ffi::c_int;
    }
    match wType {
        0 => {
            length -= 0.1f64 * (*Weir.offset(k as isize)).endCon * h;
            length = if length >= 0.0f64 { length } else { 0.0f64 };
            *q1 = cDisch1 * length * pow(h, 1.5f64);
        }
        1 => {
            length -= 0.1f64 * (*Weir.offset(k as isize)).endCon * h;
            length = if length >= 0.0f64 { length } else { 0.0f64 };
            if dir < 0.0f64 {
                *q1 = cDisch1 * length * pow(h, 1.5f64);
            } else {
                *q1 = cDisch1 * pow(length, 0.83f64) * pow(h, 1.67f64);
            }
        }
        2 => {
            *q1 = cDisch1 * (*Weir.offset(k as isize)).slope * pow(h, 2.5f64);
        }
        3 => {
            y = (1.0f64 - (*Link.offset(j as isize)).setting)
                * (*Link.offset(j as isize)).xsect.yFull;
            length = xsect_getWofY(&raw mut (*Link.offset(j as isize)).xsect, y)
                * UCF(LENGTH as ::core::ffi::c_int);
            *q1 = cDisch1 * length * pow(h, 1.5f64);
            *q2 = (*Weir.offset(k as isize)).cDisch2
                * (*Weir.offset(k as isize)).slope
                * pow(h, 2.5f64);
        }
        _ => {}
    }
    if UnitSystem == SI as ::core::ffi::c_int {
        *q1 /= M3perFT3;
        *q2 /= M3perFT3;
    }
    if hasFlapGate != 0 {
        area = weir_getOpenArea(j, head);
        if area > TINY {
            veloc = (*q1 + *q2) / area;
            hLoss = 4.0f64 / GRAVITY * veloc * veloc * exp(-1.15f64 * veloc / sqrt(head));
            head = head - hLoss;
            if head < 0.0f64 {
                head = 0.0f64;
            }
            weir_getFlow(j, k, head, dir, FALSE, q1, q2);
        }
    }
    (*Link.offset(j as isize)).dqdh = weir_getdqdh(k, dir, head, *q1, *q2);
}
unsafe extern "C" fn weir_getOrificeFlow(
    mut j: ::core::ffi::c_int,
    mut head: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut cOrif: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    let mut hloss: ::core::ffi::c_double = 0.;
    q = cOrif * sqrt(head);
    if (*Link.offset(j as isize)).hasFlapGate != 0 {
        a = weir_getOpenArea(j, y);
        if a > 0.0f64 {
            v = q / a;
            hloss = 4.0f64 / GRAVITY * v * v * exp(-1.15f64 * v / sqrt(y));
            head -= hloss;
            head = if head >= 0.0f64 { head } else { 0.0f64 };
            q = cOrif * sqrt(head);
        }
    }
    if head > 0.0f64 {
        (*Link.offset(j as isize)).dqdh = q / (2.0f64 * head);
    } else {
        (*Link.offset(j as isize)).dqdh = 0.0f64;
    }
    return q;
}
unsafe extern "C" fn weir_getOpenArea(
    mut j: ::core::ffi::c_int,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut z: ::core::ffi::c_double = 0.;
    let mut zy: ::core::ffi::c_double = 0.;
    z = (1.0f64 - (*Link.offset(j as isize)).setting) * (*Link.offset(j as isize)).xsect.yFull;
    zy = z + y;
    zy = if zy <= (*Link.offset(j as isize)).xsect.yFull {
        zy
    } else {
        (*Link.offset(j as isize)).xsect.yFull
    };
    return xsect_getAofY(&raw mut (*Link.offset(j as isize)).xsect, zy)
        - xsect_getAofY(&raw mut (*Link.offset(j as isize)).xsect, z);
}
unsafe extern "C" fn weir_getdqdh(
    mut k: ::core::ffi::c_int,
    mut dir: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
    mut q1: ::core::ffi::c_double,
    mut q2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut q1h: ::core::ffi::c_double = 0.;
    let mut q2h: ::core::ffi::c_double = 0.;
    if fabs(h) < FUDGE {
        return 0.0f64;
    }
    q1h = fabs(q1 / h);
    q2h = fabs(q2 / h);
    match (*Weir.offset(k as isize)).type_0 {
        0 => return 1.5f64 * q1h,
        1 => {
            if dir < 0.0f64 {
                return 1.5f64 * q1h;
            } else {
                return 1.67f64 * q1h;
            }
        }
        2 => {
            if q2h == 0.0f64 {
                return 2.5f64 * q1h;
            } else {
                return 1.5f64 * q1h + 2.5f64 * q2h;
            }
        }
        3 => return 1.5f64 * q1h + 2.5f64 * q2h,
        _ => {}
    }
    return 0.0f64;
}
unsafe extern "C" fn outlet_readParams(
    mut j: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 6] = [0.; 6];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        LINK as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    n1 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if n1 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    n2 = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(2 as ::core::ffi::c_int as isize),
    );
    if n2 < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if LinkOffsets == ELEV_OFFSET as ::core::ffi::c_int
        && **tok.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
    {
        x[0 as ::core::ffi::c_int as usize] = MISSING;
    } else {
        if getDouble(
            *tok.offset(3 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(3 as ::core::ffi::c_int as isize),
            );
        }
        if LinkOffsets == DEPTH_OFFSET as ::core::ffi::c_int
            && x[0 as ::core::ffi::c_int as usize] < 0.0f64
        {
            x[0 as ::core::ffi::c_int as usize] = 0.0f64;
        }
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
    x[1 as ::core::ffi::c_int as usize] = 0.0f64;
    x[2 as ::core::ffi::c_int as usize] = 0.0f64;
    x[3 as ::core::ffi::c_int as usize] = -1.0f64;
    x[4 as ::core::ffi::c_int as usize] = 0.0f64;
    x[5 as ::core::ffi::c_int as usize] = NODE_DEPTH as ::core::ffi::c_int as ::core::ffi::c_double;
    s = strtok(
        *tok.offset(4 as ::core::ffi::c_int as isize),
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
    );
    s = strtok(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if strcomp(s, w_HEAD.as_ptr()) != 0 {
        x[5 as ::core::ffi::c_int as usize] =
            NODE_HEAD as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if m == FUNCTIONAL as ::core::ffi::c_int {
        if ntoks < 7 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if getDouble(
            *tok.offset(5 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(1 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(5 as ::core::ffi::c_int as isize),
            );
        }
        if getDouble(
            *tok.offset(6 as ::core::ffi::c_int as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(2 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
        }
        n = 7 as ::core::ffi::c_int;
    } else {
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
        x[3 as ::core::ffi::c_int as usize] = i as ::core::ffi::c_double;
        n = 6 as ::core::ffi::c_int;
    }
    if ntoks > n {
        i = findmatch(
            *tok.offset(n as isize),
            &raw mut NoYesWords as *mut *mut ::core::ffi::c_char,
        );
        if i < 0 as ::core::ffi::c_int {
            return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, *tok.offset(n as isize));
        }
        x[4 as ::core::ffi::c_int as usize] = i as ::core::ffi::c_double;
    }
    let ref mut fresh0 = (*Link.offset(j as isize)).ID;
    *fresh0 = id;
    link_setParams(
        j,
        OUTLET as ::core::ffi::c_int,
        n1,
        n2,
        k,
        &raw mut x as *mut ::core::ffi::c_double,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn outlet_getInflow(mut j: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut head: ::core::ffi::c_double = 0.;
    let mut hcrest: ::core::ffi::c_double = 0.;
    let mut h1: ::core::ffi::c_double = 0.;
    let mut h2: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut dir: ::core::ffi::c_double = 0.;
    n1 = (*Link.offset(j as isize)).node1;
    n2 = (*Link.offset(j as isize)).node2;
    k = (*Link.offset(j as isize)).subIndex;
    if RouteModel == DW as ::core::ffi::c_int {
        h1 = (*Node.offset(n1 as isize)).newDepth + (*Node.offset(n1 as isize)).invertElev;
        h2 = (*Node.offset(n2 as isize)).newDepth + (*Node.offset(n2 as isize)).invertElev;
    } else {
        h1 = (*Node.offset(n1 as isize)).newDepth + (*Node.offset(n1 as isize)).invertElev;
        h2 = (*Node.offset(n1 as isize)).invertElev;
    }
    dir = if h1 >= h2 { 1.0f64 } else { -1.0f64 };
    y1 = (*Node.offset(n1 as isize)).newDepth;
    if dir < 0.0f64 {
        y1 = h1;
        h1 = h2;
        h2 = y1;
        y1 = (*Node.offset(n2 as isize)).newDepth;
    }
    hcrest = (*Node.offset(n1 as isize)).invertElev + (*Link.offset(j as isize)).offset1;
    if (*Outlet.offset(k as isize)).curveType == NODE_HEAD as ::core::ffi::c_int
        && RouteModel == DW as ::core::ffi::c_int
    {
        head = h1 - (if h2 >= hcrest { h2 } else { hcrest });
    } else {
        head = h1 - hcrest;
    }
    if head <= FUDGE || y1 <= FUDGE || link_setFlapGate(j, n1, n2, dir) != 0 {
        (*Link.offset(j as isize)).newDepth = 0.0f64;
        (*Link.offset(j as isize)).flowClass = DRY as ::core::ffi::c_int;
        return 0.0f64;
    }
    (*Link.offset(j as isize)).newDepth = head;
    (*Link.offset(j as isize)).flowClass = SUBCRITICAL as ::core::ffi::c_int;
    return dir * (*Link.offset(j as isize)).setting * outlet_getFlow(k, head);
}
unsafe extern "C" fn outlet_getFlow(
    mut k: ::core::ffi::c_int,
    mut head: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut m: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_double = 0.;
    h = head * UCF(LENGTH as ::core::ffi::c_int);
    m = (*Outlet.offset(k as isize)).qCurve;
    if m >= 0 as ::core::ffi::c_int {
        return table_lookup(Curve.offset(m as isize) as *mut TTable, h)
            / UCF(FLOW as ::core::ffi::c_int);
    } else {
        return (*Outlet.offset(k as isize)).qCoeff * pow(h, (*Outlet.offset(k as isize)).qExpon)
            / UCF(FLOW as ::core::ffi::c_int);
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

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
