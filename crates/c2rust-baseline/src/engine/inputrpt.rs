#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut Frpt: TFile;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut Gage: *mut TGage;
    static mut Subcatch: *mut TSubcatch;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Pump: *mut TPump;
    static mut Pollut: *mut TPollut;
    static mut Landuse: *mut TLanduse;
    static mut Curve: *mut TTable;
    static mut Tseries: *mut TTable;
    static mut Transect: *mut TTransect;
    static mut Street: *mut TStreet;
    static mut Shape: *mut TShape;
    fn report_writeLine(line: *const ::core::ffi::c_char);
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    static mut LinkTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut NodeTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut PumpTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut QualUnitsWords: [*mut ::core::ffi::c_char; 0];
    static mut RainTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut XsectTypeWords: [*mut ::core::ffi::c_char; 0];
    fn lid_writeSummary();
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
pub struct TShape {
    pub curve: ::core::ffi::c_int,
    pub nTbl: ::core::ffi::c_int,
    pub aFull: ::core::ffi::c_double,
    pub rFull: ::core::ffi::c_double,
    pub wMax: ::core::ffi::c_double,
    pub sMax: ::core::ffi::c_double,
    pub aMax: ::core::ffi::c_double,
    pub areaTbl: [::core::ffi::c_double; 51],
    pub hradTbl: [::core::ffi::c_double; 51],
    pub widthTbl: [::core::ffi::c_double; 51],
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
pub struct TBuildup {
    pub normalizer: ::core::ffi::c_int,
    pub funcType: ::core::ffi::c_int,
    pub coeff: [::core::ffi::c_double; 3],
    pub maxDays: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TWashoff {
    pub funcType: ::core::ffi::c_int,
    pub coeff: ::core::ffi::c_double,
    pub expon: ::core::ffi::c_double,
    pub sweepEffic: ::core::ffi::c_double,
    pub bmpEffic: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLanduse {
    pub ID: *mut ::core::ffi::c_char,
    pub sweepInterval: ::core::ffi::c_double,
    pub sweepRemoval: ::core::ffi::c_double,
    pub sweepDays0: ::core::ffi::c_double,
    pub buildupFunc: *mut TBuildup,
    pub washoffFunc: *mut TWashoff,
}
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;
pub const N_TRANSECT_TBL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const N_SHAPE_TBL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn inputrpt_writeInput() {
    let mut m: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut lidCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ErrorCode != 0 {
        return;
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*************\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"Element Count\0" as *const u8 as *const ::core::ffi::c_char);
    report_writeLine(b"*************\0" as *const u8 as *const ::core::ffi::c_char);
    fprintf(
        Frpt.file,
        b"\n  Number of rain gages ...... %d\0" as *const u8 as *const ::core::ffi::c_char,
        Nobjects[GAGE as ::core::ffi::c_int as usize],
    );
    fprintf(
        Frpt.file,
        b"\n  Number of subcatchments ... %d\0" as *const u8 as *const ::core::ffi::c_char,
        Nobjects[SUBCATCH as ::core::ffi::c_int as usize],
    );
    fprintf(
        Frpt.file,
        b"\n  Number of nodes ........... %d\0" as *const u8 as *const ::core::ffi::c_char,
        Nobjects[NODE as ::core::ffi::c_int as usize],
    );
    fprintf(
        Frpt.file,
        b"\n  Number of links ........... %d\0" as *const u8 as *const ::core::ffi::c_char,
        Nobjects[LINK as ::core::ffi::c_int as usize],
    );
    fprintf(
        Frpt.file,
        b"\n  Number of pollutants ...... %d\0" as *const u8 as *const ::core::ffi::c_char,
        Nobjects[POLLUT as ::core::ffi::c_int as usize],
    );
    fprintf(
        Frpt.file,
        b"\n  Number of land uses ....... %d\0" as *const u8 as *const ::core::ffi::c_char,
        Nobjects[LANDUSE as ::core::ffi::c_int as usize],
    );
    if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*****************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Pollutant Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*****************\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n                               Ppt.      GW         Kdecay\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  Name                 Units   Concen.   Concen.    1/days    CoPollutant\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  -----------------------------------------------------------------------\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"\n  %-20s %5s%10.2f%10.2f%10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                (*Pollut.offset(i as isize)).ID,
                *(&raw mut QualUnitsWords as *mut *mut ::core::ffi::c_char)
                    .offset((*Pollut.offset(i as isize)).units as isize),
                (*Pollut.offset(i as isize)).pptConcen,
                (*Pollut.offset(i as isize)).gwConcen,
                (*Pollut.offset(i as isize)).kDecay * SECperDAY as ::core::ffi::c_double,
            );
            if (*Pollut.offset(i as isize)).coPollut >= 0 as ::core::ffi::c_int {
                fprintf(
                    Frpt.file,
                    b"    %-s  (%.2f)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Pollut.offset((*Pollut.offset(i as isize)).coPollut as isize)).ID,
                    (*Pollut.offset(i as isize)).coFraction,
                );
            }
            i += 1;
        }
    }
    if Nobjects[LANDUSE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"***************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Landuse Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"***************\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n                         Sweeping   Maximum      Last\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  Name                   Interval   Removal     Swept\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  ---------------------------------------------------\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"\n  %-20s %10.2f%10.2f%10.2f\0" as *const u8 as *const ::core::ffi::c_char,
                (*Landuse.offset(i as isize)).ID,
                (*Landuse.offset(i as isize)).sweepInterval,
                (*Landuse.offset(i as isize)).sweepRemoval,
                (*Landuse.offset(i as isize)).sweepDays0,
            );
            i += 1;
        }
    }
    if Nobjects[GAGE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"****************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Raingage Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"****************\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n                                                      Data       Recording\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  Name                 Data Source                    Type       Interval \0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  ------------------------------------------------------------------------\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[GAGE as ::core::ffi::c_int as usize] {
            if (*Gage.offset(i as isize)).tSeries >= 0 as ::core::ffi::c_int {
                fprintf(
                    Frpt.file,
                    b"\n  %-20s %-30s \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Gage.offset(i as isize)).ID,
                    (*Tseries.offset((*Gage.offset(i as isize)).tSeries as isize)).ID,
                );
                fprintf(
                    Frpt.file,
                    b"%-10s %3d min.\0" as *const u8 as *const ::core::ffi::c_char,
                    *(&raw mut RainTypeWords as *mut *mut ::core::ffi::c_char)
                        .offset((*Gage.offset(i as isize)).rainType as isize),
                    (*Gage.offset(i as isize)).rainInterval / 60 as ::core::ffi::c_int,
                );
            } else {
                fprintf(
                    Frpt.file,
                    b"\n  %-20s %-30s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Gage.offset(i as isize)).ID,
                    &raw mut (*Gage.offset(i as isize)).fname as *mut ::core::ffi::c_char,
                );
            }
            i += 1;
        }
    }
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"********************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Subcatchment Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"********************\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n  Name                       Area     Width   %%Imperv    %%Slope Rain Gage            Outlet              \0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  -----------------------------------------------------------------------------------------------------------\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"\n  %-20s %10.2f%10.2f%10.2f%10.4f %-20s \0" as *const u8
                    as *const ::core::ffi::c_char,
                (*Subcatch.offset(i as isize)).ID,
                (*Subcatch.offset(i as isize)).area * UCF(LANDAREA as ::core::ffi::c_int),
                (*Subcatch.offset(i as isize)).width * UCF(LENGTH as ::core::ffi::c_int),
                (*Subcatch.offset(i as isize)).fracImperv * 100.0f64,
                (*Subcatch.offset(i as isize)).slope * 100.0f64,
                (*Gage.offset((*Subcatch.offset(i as isize)).gage as isize)).ID,
            );
            if (*Subcatch.offset(i as isize)).outNode >= 0 as ::core::ffi::c_int {
                fprintf(
                    Frpt.file,
                    b"%-20s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Node.offset((*Subcatch.offset(i as isize)).outNode as isize)).ID,
                );
            } else if (*Subcatch.offset(i as isize)).outSubcatch >= 0 as ::core::ffi::c_int {
                fprintf(
                    Frpt.file,
                    b"%-20s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Subcatch.offset((*Subcatch.offset(i as isize)).outSubcatch as isize)).ID,
                );
            }
            if (*Subcatch.offset(i as isize)).lidArea != 0. {
                lidCount += 1;
            }
            i += 1;
        }
    }
    if lidCount > 0 as ::core::ffi::c_int {
        lid_writeSummary();
    }
    if Nobjects[NODE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Node Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"************\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n                                           Invert      Max.    Ponded    External\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  Name                 Type                 Elev.     Depth      Area    Inflow  \0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  -------------------------------------------------------------------------------\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"\n  %-20s %-16s%10.2f%10.2f%10.1f\0" as *const u8 as *const ::core::ffi::c_char,
                (*Node.offset(i as isize)).ID,
                *(&raw mut NodeTypeWords as *mut *mut ::core::ffi::c_char).offset(
                    ((*Node.offset(i as isize)).type_0 - JUNCTION as ::core::ffi::c_int) as isize,
                ),
                (*Node.offset(i as isize)).invertElev * UCF(LENGTH as ::core::ffi::c_int),
                (*Node.offset(i as isize)).fullDepth * UCF(LENGTH as ::core::ffi::c_int),
                (*Node.offset(i as isize)).pondedArea
                    * UCF(LENGTH as ::core::ffi::c_int)
                    * UCF(LENGTH as ::core::ffi::c_int),
            );
            if !(*Node.offset(i as isize)).extInflow.is_null()
                || !(*Node.offset(i as isize)).dwfInflow.is_null()
                || !(*Node.offset(i as isize)).rdiiInflow.is_null()
            {
                fprintf(
                    Frpt.file,
                    b"    Yes\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            i += 1;
        }
    }
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Link Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"************\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n  Name             From Node        To Node          Type            Length    %%Slope Roughness\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  ---------------------------------------------------------------------------------------------\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
            if (*Link.offset(i as isize)).direction as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            {
                fprintf(
                    Frpt.file,
                    b"\n  %-16s %-16s %-16s \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Link.offset(i as isize)).ID,
                    (*Node.offset((*Link.offset(i as isize)).node1 as isize)).ID,
                    (*Node.offset((*Link.offset(i as isize)).node2 as isize)).ID,
                );
            } else {
                fprintf(
                    Frpt.file,
                    b"\n  %-16s %-16s %-16s \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Link.offset(i as isize)).ID,
                    (*Node.offset((*Link.offset(i as isize)).node2 as isize)).ID,
                    (*Node.offset((*Link.offset(i as isize)).node1 as isize)).ID,
                );
            }
            if (*Link.offset(i as isize)).type_0 == PUMP as ::core::ffi::c_int {
                k = (*Link.offset(i as isize)).subIndex;
                fprintf(
                    Frpt.file,
                    b"%-5s PUMP  \0" as *const u8 as *const ::core::ffi::c_char,
                    *(&raw mut PumpTypeWords as *mut *mut ::core::ffi::c_char)
                        .offset((*Pump.offset(k as isize)).type_0 as isize),
                );
            } else {
                fprintf(
                    Frpt.file,
                    b"%-12s\0" as *const u8 as *const ::core::ffi::c_char,
                    *(&raw mut LinkTypeWords as *mut *mut ::core::ffi::c_char).offset(
                        ((*Link.offset(i as isize)).type_0 - CONDUIT as ::core::ffi::c_int)
                            as isize,
                    ),
                );
            }
            if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
                k = (*Link.offset(i as isize)).subIndex;
                fprintf(
                    Frpt.file,
                    b"%10.1f%10.4f%10.4f\0" as *const u8 as *const ::core::ffi::c_char,
                    (*Conduit.offset(k as isize)).length * UCF(LENGTH as ::core::ffi::c_int),
                    (*Conduit.offset(k as isize)).slope
                        * 100.0f64
                        * (*Link.offset(i as isize)).direction as ::core::ffi::c_int
                            as ::core::ffi::c_double,
                    (*Conduit.offset(k as isize)).roughness,
                );
            }
            i += 1;
        }
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*********************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Cross Section Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*********************\0" as *const u8 as *const ::core::ffi::c_char);
        fprintf(
            Frpt.file,
            b"\n                                        Full     Full     Hyd.     Max.   No. of     Full\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  Conduit          Shape               Depth     Area     Rad.    Width  Barrels     Flow\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            Frpt.file,
            b"\n  ---------------------------------------------------------------------------------------\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
            if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
                k = (*Link.offset(i as isize)).subIndex;
                fprintf(
                    Frpt.file,
                    b"\n  %-16s \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Link.offset(i as isize)).ID,
                );
                if (*Link.offset(i as isize)).xsect.type_0 == CUSTOM as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"%-16s \0" as *const u8 as *const ::core::ffi::c_char,
                        (*Curve.offset((*Link.offset(i as isize)).xsect.transect as isize)).ID,
                    );
                } else if (*Link.offset(i as isize)).xsect.type_0 == IRREGULAR as ::core::ffi::c_int
                {
                    fprintf(
                        Frpt.file,
                        b"%-16s \0" as *const u8 as *const ::core::ffi::c_char,
                        (*Transect.offset((*Link.offset(i as isize)).xsect.transect as isize)).ID,
                    );
                } else if (*Link.offset(i as isize)).xsect.type_0
                    == STREET_XSECT as ::core::ffi::c_int
                {
                    fprintf(
                        Frpt.file,
                        b"%-16s \0" as *const u8 as *const ::core::ffi::c_char,
                        (*Street.offset((*Link.offset(i as isize)).xsect.transect as isize)).ID,
                    );
                } else {
                    fprintf(
                        Frpt.file,
                        b"%-16s \0" as *const u8 as *const ::core::ffi::c_char,
                        *(&raw mut XsectTypeWords as *mut *mut ::core::ffi::c_char)
                            .offset((*Link.offset(i as isize)).xsect.type_0 as isize),
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%8.2f %8.2f %8.2f %8.2f      %3d %8.2f\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*Link.offset(i as isize)).xsect.yFull * UCF(LENGTH as ::core::ffi::c_int),
                    (*Link.offset(i as isize)).xsect.aFull
                        * UCF(LENGTH as ::core::ffi::c_int)
                        * UCF(LENGTH as ::core::ffi::c_int),
                    (*Link.offset(i as isize)).xsect.rFull * UCF(LENGTH as ::core::ffi::c_int),
                    (*Link.offset(i as isize)).xsect.wMax * UCF(LENGTH as ::core::ffi::c_int),
                    (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int,
                    (*Link.offset(i as isize)).qFull * UCF(FLOW as ::core::ffi::c_int),
                );
            }
            i += 1;
        }
    }
    if Nobjects[SHAPE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Shape Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"*************\0" as *const u8 as *const ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[SHAPE as ::core::ffi::c_int as usize] {
            k = (*Shape.offset(i as isize)).curve;
            fprintf(
                Frpt.file,
                b"\n\n  Shape %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Curve.offset(k as isize)).ID,
            );
            fprintf(
                Frpt.file,
                b"\n  Area:  \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < N_SHAPE_TBL {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Shape.offset(i as isize)).areaTbl[m as usize],
                );
                m += 1;
            }
            fprintf(
                Frpt.file,
                b"\n  Hrad:  \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < N_SHAPE_TBL {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Shape.offset(i as isize)).hradTbl[m as usize],
                );
                m += 1;
            }
            fprintf(
                Frpt.file,
                b"\n  Width: \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < N_SHAPE_TBL {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Shape.offset(i as isize)).widthTbl[m as usize],
                );
                m += 1;
            }
            i += 1;
        }
    }
    if Nobjects[TRANSECT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"****************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Transect Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"****************\0" as *const u8 as *const ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[TRANSECT as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"\n\n  Transect %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Transect.offset(i as isize)).ID,
            );
            fprintf(
                Frpt.file,
                b"\n  Area:  \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < N_TRANSECT_TBL {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Transect.offset(i as isize)).areaTbl[m as usize],
                );
                m += 1;
            }
            fprintf(
                Frpt.file,
                b"\n  Hrad:  \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < N_TRANSECT_TBL {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Transect.offset(i as isize)).hradTbl[m as usize],
                );
                m += 1;
            }
            fprintf(
                Frpt.file,
                b"\n  Width: \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < N_TRANSECT_TBL {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Transect.offset(i as isize)).widthTbl[m as usize],
                );
                m += 1;
            }
            i += 1;
        }
    }
    if Nobjects[STREET as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"**************\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"Street Summary\0" as *const u8 as *const ::core::ffi::c_char);
        report_writeLine(b"**************\0" as *const u8 as *const ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[STREET as ::core::ffi::c_int as usize] {
            fprintf(
                Frpt.file,
                b"\n\n  Street %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Street.offset(i as isize)).ID,
            );
            fprintf(
                Frpt.file,
                b"\n  Area:  \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < (*Street.offset(i as isize)).transect.nTbl {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Street.offset(i as isize)).transect.areaTbl[m as usize],
                );
                m += 1;
            }
            fprintf(
                Frpt.file,
                b"\n  Hrad:  \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < (*Street.offset(i as isize)).transect.nTbl {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Street.offset(i as isize)).transect.hradTbl[m as usize],
                );
                m += 1;
            }
            fprintf(
                Frpt.file,
                b"\n  Width: \0" as *const u8 as *const ::core::ffi::c_char,
            );
            m = 1 as ::core::ffi::c_int;
            while m < (*Street.offset(i as isize)).transect.nTbl {
                if m % 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        Frpt.file,
                        b"\n          \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                fprintf(
                    Frpt.file,
                    b"%10.4f \0" as *const u8 as *const ::core::ffi::c_char,
                    (*Street.offset(i as isize)).transect.widthTbl[m as usize],
                );
                m += 1;
            }
            i += 1;
        }
    }
    report_writeLine(b"\0" as *const u8 as *const ::core::ffi::c_char);
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
