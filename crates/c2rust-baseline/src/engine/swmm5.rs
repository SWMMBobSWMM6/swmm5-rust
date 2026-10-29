#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static mut stdout: *mut FILE;
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn datetime_decodeDate(
        date: DateTime,
        y: *mut ::core::ffi::c_int,
        m: *mut ::core::ffi::c_int,
        d: *mut ::core::ffi::c_int,
    );
    fn datetime_decodeTime(
        time: DateTime,
        h: *mut ::core::ffi::c_int,
        m: *mut ::core::ffi::c_int,
        s: *mut ::core::ffi::c_int,
    );
    fn datetime_dayOfWeek(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_setDateFormat(fmt: ::core::ffi::c_int);
    fn datetime_addSeconds(date1: DateTime, seconds: ::core::ffi::c_double) -> DateTime;
    fn project_open(
        f1: *const ::core::ffi::c_char,
        f2: *const ::core::ffi::c_char,
        f3: *const ::core::ffi::c_char,
    );
    fn project_close();
    fn project_readInput();
    fn project_validate();
    fn project_init() -> ::core::ffi::c_int;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn report_writeLine(line: *const ::core::ffi::c_char);
    fn report_writeSysTime();
    fn report_writeLogo();
    fn report_writeTitle();
    fn report_writeOptions();
    fn report_writeReport();
    fn report_writeControlActionsHeading();
    fn inputrpt_writeInput();
    fn climate_setState(aDate: DateTime);
    fn rain_open();
    fn rain_close();
    fn runoff_open() -> ::core::ffi::c_int;
    fn runoff_execute();
    fn runoff_close();
    fn routing_open() -> ::core::ffi::c_int;
    fn routing_getRoutingStep(
        routingModel: ::core::ffi::c_int,
        fixedStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn routing_execute(routingModel: ::core::ffi::c_int, routingStep: ::core::ffi::c_double);
    fn routing_close(routingModel: ::core::ffi::c_int);
    fn output_open() -> ::core::ffi::c_int;
    fn output_end();
    fn output_close();
    fn output_saveResults(reportTime: ::core::ffi::c_double);
    fn output_updateAvgResults();
    fn output_readDateTime(period: ::core::ffi::c_long, aDate: *mut DateTime);
    fn output_readSubcatchResults(period: ::core::ffi::c_long, index: ::core::ffi::c_int);
    fn output_readNodeResults(_: ::core::ffi::c_long, index: ::core::ffi::c_int);
    fn output_readLinkResults(_: ::core::ffi::c_long, index: ::core::ffi::c_int);
    fn massbal_open() -> ::core::ffi::c_int;
    fn massbal_close();
    fn massbal_report();
    fn stats_open() -> ::core::ffi::c_int;
    fn stats_close();
    fn stats_report();
    fn hotstart_open() -> ::core::ffi::c_int;
    fn hotstart_close();
    fn link_setSetting(j: ::core::ffi::c_int, tstep: ::core::ffi::c_double);
    fn link_getVelocity(
        link: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn xsect_getWofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn error_getMsg(
        i: ::core::ffi::c_int,
        msg: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
pub type OutfallType = ::core::ffi::c_uint;
pub const TIMESERIES_OUTFALL: OutfallType = 4;
pub const TIDAL_OUTFALL: OutfallType = 3;
pub const FIXED_OUTFALL: OutfallType = 2;
pub const NORMAL_OUTFALL: OutfallType = 1;
pub const FREE_OUTFALL: OutfallType = 0;
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
pub struct TWind {
    pub type_0: ::core::ffi::c_int,
    pub aws: [::core::ffi::c_double; 12],
    pub ws: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSnow {
    pub snotmp: ::core::ffi::c_double,
    pub tipm: ::core::ffi::c_double,
    pub rnm: ::core::ffi::c_double,
    pub adc: [[::core::ffi::c_double; 10]; 2],
    pub season: ::core::ffi::c_double,
    pub removed: ::core::ffi::c_double,
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
pub struct TEvent {
    pub start: DateTime,
    pub end: DateTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAquifer {
    pub ID: *mut ::core::ffi::c_char,
    pub porosity: ::core::ffi::c_double,
    pub wiltingPoint: ::core::ffi::c_double,
    pub fieldCapacity: ::core::ffi::c_double,
    pub conductivity: ::core::ffi::c_double,
    pub conductSlope: ::core::ffi::c_double,
    pub tensionSlope: ::core::ffi::c_double,
    pub upperEvapFrac: ::core::ffi::c_double,
    pub lowerEvapDepth: ::core::ffi::c_double,
    pub lowerLossCoeff: ::core::ffi::c_double,
    pub bottomElev: ::core::ffi::c_double,
    pub waterTableElev: ::core::ffi::c_double,
    pub upperMoisture: ::core::ffi::c_double,
    pub upperEvapPat: ::core::ffi::c_int,
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
pub struct TSnowmelt {
    pub ID: *mut ::core::ffi::c_char,
    pub snn: ::core::ffi::c_double,
    pub si: [::core::ffi::c_double; 3],
    pub dhmin: [::core::ffi::c_double; 3],
    pub dhmax: [::core::ffi::c_double; 3],
    pub tbase: [::core::ffi::c_double; 3],
    pub fwfrac: [::core::ffi::c_double; 3],
    pub wsnow: [::core::ffi::c_double; 3],
    pub fwnow: [::core::ffi::c_double; 3],
    pub weplow: ::core::ffi::c_double,
    pub sfrac: [::core::ffi::c_double; 5],
    pub toSubcatch: ::core::ffi::c_int,
    pub dhm: [::core::ffi::c_double; 3],
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
pub struct TPattern {
    pub ID: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub factor: [::core::ffi::c_double; 24],
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
pub struct TUnitHyd {
    pub ID: *mut ::core::ffi::c_char,
    pub rainGage: ::core::ffi::c_int,
    pub iaMax: [[::core::ffi::c_double; 3]; 12],
    pub iaRecov: [[::core::ffi::c_double; 3]; 12],
    pub iaInit: [[::core::ffi::c_double; 3]; 12],
    pub r: [[::core::ffi::c_double; 3]; 12],
    pub tBase: [[::core::ffi::c_long; 3]; 12],
    pub tPeak: [[::core::ffi::c_long; 3]; 12],
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const swmm_SYSTEM: C2RustUnnamed = 100;
pub const swmm_LINK: C2RustUnnamed = 3;
pub const swmm_NODE: C2RustUnnamed = 2;
pub const swmm_SUBCATCH: C2RustUnnamed = 1;
pub const swmm_GAGE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const swmm_GAGE_RAINFALL: C2RustUnnamed_0 = 100;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const swmm_SUBCATCH_RPTFLAG: C2RustUnnamed_1 = 206;
pub const swmm_SUBCATCH_RUNOFF: C2RustUnnamed_1 = 205;
pub const swmm_SUBCATCH_INFIL: C2RustUnnamed_1 = 204;
pub const swmm_SUBCATCH_EVAP: C2RustUnnamed_1 = 203;
pub const swmm_SUBCATCH_RAINFALL: C2RustUnnamed_1 = 202;
pub const swmm_SUBCATCH_RAINGAGE: C2RustUnnamed_1 = 201;
pub const swmm_SUBCATCH_AREA: C2RustUnnamed_1 = 200;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const swmm_NODE_RPTFLAG: C2RustUnnamed_2 = 309;
pub const swmm_NODE_OVERFLOW: C2RustUnnamed_2 = 308;
pub const swmm_NODE_INFLOW: C2RustUnnamed_2 = 307;
pub const swmm_NODE_LATFLOW: C2RustUnnamed_2 = 306;
pub const swmm_NODE_VOLUME: C2RustUnnamed_2 = 305;
pub const swmm_NODE_HEAD: C2RustUnnamed_2 = 304;
pub const swmm_NODE_DEPTH: C2RustUnnamed_2 = 303;
pub const swmm_NODE_MAXDEPTH: C2RustUnnamed_2 = 302;
pub const swmm_NODE_ELEV: C2RustUnnamed_2 = 301;
pub const swmm_NODE_TYPE: C2RustUnnamed_2 = 300;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const swmm_LINK_RPTFLAG: C2RustUnnamed_3 = 414;
pub const swmm_LINK_TOPWIDTH: C2RustUnnamed_3 = 413;
pub const swmm_LINK_VELOCITY: C2RustUnnamed_3 = 412;
pub const swmm_LINK_DEPTH: C2RustUnnamed_3 = 411;
pub const swmm_LINK_FLOW: C2RustUnnamed_3 = 410;
pub const swmm_LINK_TIMECLOSED: C2RustUnnamed_3 = 409;
pub const swmm_LINK_TIMEOPEN: C2RustUnnamed_3 = 408;
pub const swmm_LINK_SETTING: C2RustUnnamed_3 = 407;
pub const swmm_LINK_FULLFLOW: C2RustUnnamed_3 = 406;
pub const swmm_LINK_FULLDEPTH: C2RustUnnamed_3 = 405;
pub const swmm_LINK_SLOPE: C2RustUnnamed_3 = 404;
pub const swmm_LINK_LENGTH: C2RustUnnamed_3 = 403;
pub const swmm_LINK_NODE2: C2RustUnnamed_3 = 402;
pub const swmm_LINK_NODE1: C2RustUnnamed_3 = 401;
pub const swmm_LINK_TYPE: C2RustUnnamed_3 = 400;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const swmm_FLOWUNITS: C2RustUnnamed_4 = 8;
pub const swmm_NOREPORT: C2RustUnnamed_4 = 7;
pub const swmm_TOTALSTEPS: C2RustUnnamed_4 = 6;
pub const swmm_REPORTSTEP: C2RustUnnamed_4 = 5;
pub const swmm_MAXROUTESTEP: C2RustUnnamed_4 = 4;
pub const swmm_ROUTESTEP: C2RustUnnamed_4 = 3;
pub const swmm_ELAPSEDTIME: C2RustUnnamed_4 = 2;
pub const swmm_CURRENTDATE: C2RustUnnamed_4 = 1;
pub const swmm_STARTDATE: C2RustUnnamed_4 = 0;
pub const VERSION: ::core::ffi::c_int = 52004 as ::core::ffi::c_int;
pub const MAXMSG: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXFNAME: ::core::ffi::c_int = 259 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MSECperDAY: ::core::ffi::c_double = 8.64e7f64;
pub const M_D_Y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Finp: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Finflows: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Foutflows: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Fclimate: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Frain: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Frdii: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Fhotstart1: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Fhotstart2: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Frpt: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Fout: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Frunoff: TFile = TFile {
    name: [0; 260],
    mode: 0,
    state: 0,
    file: ::core::ptr::null::<FILE>() as *mut FILE,
};
#[no_mangle]
pub static mut Nperiods: ::core::ffi::c_long = 0;
#[no_mangle]
pub static mut TotalStepCount: ::core::ffi::c_long = 0;
#[no_mangle]
pub static mut ReportStepCount: ::core::ffi::c_long = 0;
#[no_mangle]
pub static mut NonConvergeCount: ::core::ffi::c_long = 0;
#[no_mangle]
pub static mut Msg: [::core::ffi::c_char; 1025] = [0; 1025];
#[no_mangle]
pub static mut ErrorMsg: [::core::ffi::c_char; 1025] = [0; 1025];
#[no_mangle]
pub static mut Title: [[::core::ffi::c_char; 1025]; 3] = [[0; 1025]; 3];
#[no_mangle]
pub static mut TempDir: [::core::ffi::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut InpDir: [::core::ffi::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut RptFlags: TRptFlags = TRptFlags {
    disabled: 0,
    input: 0,
    subcatchments: 0,
    nodes: 0,
    links: 0,
    continuity: 0,
    flowStats: 0,
    controls: 0,
    averages: 0,
    linesPerPage: 0,
};
#[no_mangle]
pub static mut NumEvents: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut NormalFlowLtd: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SlopeWeighting: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut Compatibility: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SkipSteadyState: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut IgnoreRainfall: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut IgnoreRDII: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut IgnoreSnowmelt: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut IgnoreGwater: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut IgnoreRouting: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut IgnoreQuality: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ErrorCode: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut Warnings: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut WetStep: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut DryStep: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ReportStep: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut RuleStep: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SweepStart: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SweepEnd: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut MaxTrials: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut NumThreads: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut Nobjects: [::core::ffi::c_int; 18] = [0; 18];
#[no_mangle]
pub static mut Nnodes: [::core::ffi::c_int; 5] = [0; 5];
#[no_mangle]
pub static mut Nlinks: [::core::ffi::c_int; 5] = [0; 5];
#[no_mangle]
pub static mut UnitSystem: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut FlowUnits: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut InfilModel: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut RouteModel: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ForceMainEqn: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut LinkOffsets: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SurchargeMethod: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut AllowPonding: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut InertDamping: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SysFlowTol: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut QualError: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut LatFlowTol: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut RouteStep: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut HeadTol: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut MinRouteStep: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut LengtheningStep: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut StartDryDays: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut CourantFactor: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut MinSurfArea: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut MinSlope: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut RunoffError: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut GwaterError: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut FlowError: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut CrownCutoff: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut StartDate: DateTime = 0.;
#[no_mangle]
pub static mut StartTime: DateTime = 0.;
#[no_mangle]
pub static mut StartDateTime: DateTime = 0.;
#[no_mangle]
pub static mut EndDate: DateTime = 0.;
#[no_mangle]
pub static mut EndTime: DateTime = 0.;
#[no_mangle]
pub static mut EndDateTime: DateTime = 0.;
#[no_mangle]
pub static mut ReportStartDate: DateTime = 0.;
#[no_mangle]
pub static mut ReportStartTime: DateTime = 0.;
#[no_mangle]
pub static mut ReportStart: DateTime = 0.;
#[no_mangle]
pub static mut TotalDuration: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut ElapsedTime: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut ReportTime: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut OldRunoffTime: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut NewRunoffTime: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut OldRoutingTime: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut NewRoutingTime: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut Temp: TTemp = TTemp {
    dataSource: 0,
    tSeries: 0,
    fileStartDate: 0.,
    elev: 0.,
    anglat: 0.,
    dtlong: 0.,
    ta: 0.,
    tmax: 0.,
    ea: 0.,
    gamma: 0.,
    tanAnglat: 0.,
};
#[no_mangle]
pub static mut Evap: TEvap = TEvap {
    type_0: 0,
    tSeries: 0,
    monthlyEvap: [0.; 12],
    panCoeff: [0.; 12],
    recoveryPattern: 0,
    dryOnly: 0,
    rate: 0.,
    recoveryFactor: 0.,
};
#[no_mangle]
pub static mut Wind: TWind = TWind {
    type_0: 0,
    aws: [0.; 12],
    ws: 0.,
};
#[no_mangle]
pub static mut Snow: TSnow = TSnow {
    snotmp: 0.,
    tipm: 0.,
    rnm: 0.,
    adc: [[0.; 10]; 2],
    season: 0.,
    removed: 0.,
};
#[no_mangle]
pub static mut Adjust: TAdjust = TAdjust {
    temp: [0.; 12],
    evap: [0.; 12],
    rain: [0.; 12],
    hydcon: [0.; 12],
    rainFactor: 0.,
    hydconFactor: 0.,
};
#[no_mangle]
pub static mut Snowmelt: *mut TSnowmelt = ::core::ptr::null::<TSnowmelt>() as *mut TSnowmelt;
#[no_mangle]
pub static mut Gage: *mut TGage = ::core::ptr::null::<TGage>() as *mut TGage;
#[no_mangle]
pub static mut Subcatch: *mut TSubcatch = ::core::ptr::null::<TSubcatch>() as *mut TSubcatch;
#[no_mangle]
pub static mut Aquifer: *mut TAquifer = ::core::ptr::null::<TAquifer>() as *mut TAquifer;
#[no_mangle]
pub static mut UnitHyd: *mut TUnitHyd = ::core::ptr::null::<TUnitHyd>() as *mut TUnitHyd;
#[no_mangle]
pub static mut Node: *mut TNode = ::core::ptr::null::<TNode>() as *mut TNode;
#[no_mangle]
pub static mut Outfall: *mut TOutfall = ::core::ptr::null::<TOutfall>() as *mut TOutfall;
#[no_mangle]
pub static mut Divider: *mut TDivider = ::core::ptr::null::<TDivider>() as *mut TDivider;
#[no_mangle]
pub static mut Storage: *mut TStorage = ::core::ptr::null::<TStorage>() as *mut TStorage;
#[no_mangle]
pub static mut Link: *mut TLink = ::core::ptr::null::<TLink>() as *mut TLink;
#[no_mangle]
pub static mut Conduit: *mut TConduit = ::core::ptr::null::<TConduit>() as *mut TConduit;
#[no_mangle]
pub static mut Pump: *mut TPump = ::core::ptr::null::<TPump>() as *mut TPump;
#[no_mangle]
pub static mut Orifice: *mut TOrifice = ::core::ptr::null::<TOrifice>() as *mut TOrifice;
#[no_mangle]
pub static mut Weir: *mut TWeir = ::core::ptr::null::<TWeir>() as *mut TWeir;
#[no_mangle]
pub static mut Outlet: *mut TOutlet = ::core::ptr::null::<TOutlet>() as *mut TOutlet;
#[no_mangle]
pub static mut Pollut: *mut TPollut = ::core::ptr::null::<TPollut>() as *mut TPollut;
#[no_mangle]
pub static mut Landuse: *mut TLanduse = ::core::ptr::null::<TLanduse>() as *mut TLanduse;
#[no_mangle]
pub static mut Pattern: *mut TPattern = ::core::ptr::null::<TPattern>() as *mut TPattern;
#[no_mangle]
pub static mut Curve: *mut TTable = ::core::ptr::null::<TTable>() as *mut TTable;
#[no_mangle]
pub static mut Tseries: *mut TTable = ::core::ptr::null::<TTable>() as *mut TTable;
#[no_mangle]
pub static mut Transect: *mut TTransect = ::core::ptr::null::<TTransect>() as *mut TTransect;
#[no_mangle]
pub static mut Street: *mut TStreet = ::core::ptr::null::<TStreet>() as *mut TStreet;
#[no_mangle]
pub static mut Shape: *mut TShape = ::core::ptr::null::<TShape>() as *mut TShape;
#[no_mangle]
pub static mut Event: *mut TEvent = ::core::ptr::null::<TEvent>() as *mut TEvent;
#[no_mangle]
pub static mut Ucf: [[::core::ffi::c_double; 2]; 10] = [
    [43200.0f64, 1097280.0f64],
    [12.0f64, 304.8f64],
    [1036800.0f64, 26334720.0f64],
    [1.0f64, 0.3048f64],
    [2.2956e-5f64, 0.92903e-5f64],
    [1.0f64, 0.02832f64],
    [1.0f64, 1.608f64],
    [1.0f64, 1.8f64],
    [2.203e-6f64, 1.0e-6f64],
    [43560.0f64, 3048.0f64],
];
#[no_mangle]
pub static mut Qcf: [::core::ffi::c_double; 6] = [
    1.0f64, 448.831f64, 0.64632f64, 0.02832f64, 28.317f64, 2.4466f64,
];
static mut IsOpenFlag: ::core::ffi::c_int = 0;
static mut IsStartedFlag: ::core::ffi::c_int = 0;
static mut SaveResultsFlag: ::core::ffi::c_int = 0;
static mut ExceptionCount: ::core::ffi::c_int = 0;
static mut DoRunoff: ::core::ffi::c_int = 0;
static mut DoRouting: ::core::ffi::c_int = 0;
static mut RoutingDuration: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn swmm_run(
    mut f1: *const ::core::ffi::c_char,
    mut f2: *const ::core::ffi::c_char,
    mut f3: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut newHour: ::core::ffi::c_long = 0;
    let mut oldHour: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut theDay: ::core::ffi::c_long = 0;
    let mut theHour: ::core::ffi::c_long = 0;
    let mut elapsedTime: ::core::ffi::c_double = 0.0f64;
    IsOpenFlag = FALSE;
    IsStartedFlag = FALSE;
    SaveResultsFlag = TRUE;
    ErrorCode = 0 as ::core::ffi::c_int;
    writecon(b"\n o  Retrieving project data\0" as *const u8 as *const ::core::ffi::c_char);
    swmm_open(f1, f2, f3);
    if ErrorCode == 0 {
        swmm_start(TRUE);
        if ErrorCode == 0 {
            writecon(
                b"\n o  Simulating day: 0     hour:  0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            loop {
                swmm_step(&raw mut elapsedTime);
                newHour = crate::ccast::f64_to_i64((elapsedTime * 24.0f64));
                if newHour > oldHour {
                    theDay = crate::ccast::f64_to_i64(elapsedTime);
                    theHour = crate::ccast::f64_to_i64(((elapsedTime - floor(elapsedTime)) * 24.0f64));
                    writecon(
                        b"\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    snprintf(
                        &raw mut Msg as *mut ::core::ffi::c_char,
                        MAXMSG as size_t,
                        b"%-5ld hour: %-2ld\0" as *const u8 as *const ::core::ffi::c_char,
                        theDay,
                        theHour,
                    );
                    writecon(&raw mut Msg as *mut ::core::ffi::c_char);
                    oldHour = newHour;
                }
                if !(elapsedTime > 0.0f64 && ErrorCode == 0) {
                    break;
                }
            }
            writecon(
                b"\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            writecon(
                b"Simulation complete           \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        swmm_end();
    }
    if ErrorCode == 0 && Fout.mode as ::core::ffi::c_int == SCRATCH_FILE as ::core::ffi::c_int {
        writecon(b"\n o  Writing output report\0" as *const u8 as *const ::core::ffi::c_char);
        swmm_report();
    }
    swmm_close();
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_open(
    mut f1: *const ::core::ffi::c_char,
    mut f2: *const ::core::ffi::c_char,
    mut f3: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    datetime_setDateFormat(M_D_Y);
    ErrorCode = 0 as ::core::ffi::c_int;
    ErrorMsg[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    Warnings = 0 as ::core::ffi::c_int;
    IsOpenFlag = FALSE;
    IsStartedFlag = FALSE;
    ExceptionCount = 0 as ::core::ffi::c_int;
    strcpy(
        &raw mut InpDir as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    project_open(f1, f2, f3);
    getAbsolutePath(
        f1,
        &raw mut InpDir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 260]>() as size_t,
    );
    if ErrorCode != 0 {
        return ErrorCode;
    }
    IsOpenFlag = TRUE;
    report_writeLogo();
    project_readInput();
    if ErrorCode != 0 {
        return ErrorCode;
    }
    report_writeTitle();
    project_validate();
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_start(mut saveResults_0: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if ErrorCode != 0 {
        return ErrorCode;
    }
    if IsOpenFlag == 0 {
        ErrorCode = ERR_API_NOT_OPEN as ::core::ffi::c_int;
        return ErrorCode;
    }
    if IsStartedFlag != 0 {
        ErrorCode = ERR_API_NOT_ENDED as ::core::ffi::c_int;
        return ErrorCode;
    }
    if RptFlags.disabled == 0 {
        if RptFlags.input != 0 {
            inputrpt_writeInput();
        }
        report_writeOptions();
    }
    SaveResultsFlag = saveResults_0;
    ExceptionCount = 0 as ::core::ffi::c_int;
    ElapsedTime = 0.0f64;
    RoutingDuration = TotalDuration;
    NewRunoffTime = 0.0f64;
    NewRoutingTime = 0.0f64;
    ReportTime =
        1000 as ::core::ffi::c_int as ::core::ffi::c_double * ReportStep as ::core::ffi::c_double;
    TotalStepCount = 0 as ::core::ffi::c_long;
    ReportStepCount = 0 as ::core::ffi::c_long;
    NonConvergeCount = 0 as ::core::ffi::c_long;
    IsStartedFlag = TRUE;
    RunoffError = 0.0f64;
    GwaterError = 0.0f64;
    FlowError = 0.0f64;
    QualError = 0.0f64;
    if IgnoreRainfall == 0 {
        rain_open();
    }
    if ErrorCode != 0 {
        return ErrorCode;
    }
    project_init();
    if Nobjects[SUBCATCH as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        DoRunoff = TRUE;
    } else {
        DoRunoff = FALSE;
    }
    if Nobjects[NODE as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int && IgnoreRouting == 0
    {
        DoRouting = TRUE;
    } else {
        DoRouting = FALSE;
    }
    output_open();
    if DoRunoff != 0 {
        runoff_open();
    }
    if hotstart_open() == 0 {
        return ErrorCode;
    }
    if DoRouting != 0 {
        routing_open();
    }
    massbal_open();
    stats_open();
    if RptFlags.disabled == 0 && RptFlags.controls as ::core::ffi::c_int != 0 {
        report_writeControlActionsHeading();
    }
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_step(
    mut elapsedTime: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    *elapsedTime = 0.0f64;
    if ErrorCode != 0 {
        return ErrorCode;
    }
    if IsOpenFlag == 0 {
        ErrorCode = ERR_API_NOT_OPEN as ::core::ffi::c_int;
        return ErrorCode;
    }
    if IsStartedFlag == 0 {
        ErrorCode = ERR_API_NOT_STARTED as ::core::ffi::c_int;
        return ErrorCode;
    }
    if NewRoutingTime < RoutingDuration {
        execRouting();
    }
    if SaveResultsFlag != 0 {
        saveResults();
    }
    if NewRoutingTime < RoutingDuration {
        ElapsedTime = NewRoutingTime / MSECperDAY;
    } else {
        ElapsedTime = 0.0f64;
    }
    *elapsedTime = ElapsedTime;
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_stride(
    mut strideStep: ::core::ffi::c_int,
    mut elapsedTime: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut realRouteStep: ::core::ffi::c_double = RouteStep;
    *elapsedTime = 0.0f64;
    if ErrorCode != 0 {
        return ErrorCode;
    }
    if IsOpenFlag == 0 {
        ErrorCode = ERR_API_NOT_OPEN as ::core::ffi::c_int;
        return ErrorCode;
    }
    if IsStartedFlag == 0 {
        ErrorCode = ERR_API_NOT_STARTED as ::core::ffi::c_int;
        return ErrorCode;
    }
    RoutingDuration = NewRoutingTime + 1000.0f64 * strideStep as ::core::ffi::c_double;
    RoutingDuration = if TotalDuration <= RoutingDuration {
        TotalDuration
    } else {
        RoutingDuration
    };
    if (strideStep as ::core::ffi::c_double) < RouteStep {
        RouteStep = strideStep as ::core::ffi::c_double;
    }
    loop {
        swmm_step(elapsedTime);
        if !(*elapsedTime > 0.0f64 && ErrorCode == 0) {
            break;
        }
    }
    RouteStep = realRouteStep;
    RoutingDuration = TotalDuration;
    if NewRoutingTime < TotalDuration {
        ElapsedTime = NewRoutingTime / MSECperDAY;
    } else {
        ElapsedTime = 0.0f64;
    }
    *elapsedTime = ElapsedTime;
    return ErrorCode;
}
unsafe extern "C" fn execRouting() {
    let mut nextRoutingTime: ::core::ffi::c_double = 0.;
    let mut routingStep: ::core::ffi::c_double = 0.;
    TotalStepCount += 1;
    if DoRouting == 0 {
        routingStep = (if WetStep <= ReportStep {
            WetStep
        } else {
            ReportStep
        }) as ::core::ffi::c_double;
    } else {
        routingStep = routing_getRoutingStep(RouteModel, RouteStep);
    }
    if routingStep <= 0.0f64 {
        ErrorCode = ERR_TIMESTEP as ::core::ffi::c_int;
        return;
    }
    nextRoutingTime = NewRoutingTime + 1000.0f64 * routingStep;
    if nextRoutingTime > RoutingDuration {
        routingStep = (RoutingDuration - NewRoutingTime) / 1000.0f64;
        routingStep = if routingStep >= 1.0f64 / 1000.0f64 {
            routingStep
        } else {
            1.0f64 / 1000.0f64
        };
        nextRoutingTime = RoutingDuration;
    }
    if DoRunoff != 0 {
        while NewRunoffTime < nextRoutingTime {
            runoff_execute();
            if ErrorCode != 0 {
                return;
            }
        }
    } else {
        climate_setState(getDateTime(NewRoutingTime));
    }
    if DoRouting != 0 {
        routing_execute(RouteModel, routingStep);
    } else {
        NewRoutingTime = nextRoutingTime;
    };
}
unsafe extern "C" fn saveResults() {
    if NewRoutingTime >= ReportTime {
        if RptFlags.averages != 0 {
            if NewRoutingTime == ReportTime {
                output_updateAvgResults();
            }
            output_saveResults(ReportTime);
            if NewRoutingTime > ReportTime {
                output_updateAvgResults();
            }
        } else {
            output_saveResults(ReportTime);
        }
        ReportTime = ReportTime
            + 1000 as ::core::ffi::c_int as ::core::ffi::c_double
                * ReportStep as ::core::ffi::c_double;
    } else if RptFlags.averages != 0 {
        output_updateAvgResults();
    }
}
#[no_mangle]
pub unsafe extern "C" fn swmm_end() -> ::core::ffi::c_int {
    if IsOpenFlag == 0 {
        ErrorCode = ERR_API_NOT_OPEN as ::core::ffi::c_int;
        return ErrorCode;
    }
    if IsStartedFlag != 0 {
        if !Fout.file.is_null() {
            output_end();
        }
        if ErrorCode == 0 && RptFlags.disabled as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            massbal_report();
            stats_report();
        }
        stats_close();
        massbal_close();
        if IgnoreRainfall == 0 {
            rain_close();
        }
        if DoRunoff != 0 {
            runoff_close();
        }
        if DoRouting != 0 {
            routing_close(RouteModel);
        }
        hotstart_close();
        IsStartedFlag = FALSE;
    }
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_report() -> ::core::ffi::c_int {
    if ErrorCode == 0 {
        report_writeReport();
    }
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_writeLine(mut line: *const ::core::ffi::c_char) {
    if IsOpenFlag != 0 {
        report_writeLine(line);
    }
}
#[no_mangle]
pub unsafe extern "C" fn swmm_close() -> ::core::ffi::c_int {
    if !Fout.file.is_null() {
        output_close();
    }
    if IsOpenFlag != 0 {
        project_close();
    }
    report_writeSysTime();
    if !Finp.file.is_null() {
        fclose(Finp.file);
    }
    if !Frpt.file.is_null() {
        fclose(Frpt.file);
    }
    if !Fout.file.is_null() {
        fclose(Fout.file);
        if Fout.mode as ::core::ffi::c_int == SCRATCH_FILE as ::core::ffi::c_int {
            remove(&raw mut Fout.name as *mut ::core::ffi::c_char);
        }
    }
    IsOpenFlag = FALSE;
    IsStartedFlag = FALSE;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getMassBalErr(
    mut runoffErr: *mut ::core::ffi::c_float,
    mut flowErr: *mut ::core::ffi::c_float,
    mut qualErr: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    *runoffErr = 0.0f32;
    *flowErr = 0.0f32;
    *qualErr = 0.0f32;
    if IsOpenFlag != 0 && IsStartedFlag == 0 {
        *runoffErr = RunoffError as ::core::ffi::c_float;
        *flowErr = FlowError as ::core::ffi::c_float;
        *qualErr = QualError as ::core::ffi::c_float;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getVersion() -> ::core::ffi::c_int {
    return VERSION;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getWarnings() -> ::core::ffi::c_int {
    return Warnings;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getError(
    mut errMsg: *mut ::core::ffi::c_char,
    mut msgLen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ErrorCode > 0 as ::core::ffi::c_int
        && strlen(&raw mut ErrorMsg as *mut ::core::ffi::c_char) == 0 as size_t
    {
        error_getMsg(ErrorCode, &raw mut ErrorMsg as *mut ::core::ffi::c_char);
    }
    sstrncpy(
        errMsg,
        &raw mut ErrorMsg as *mut ::core::ffi::c_char,
        msgLen as size_t,
    );
    if msgLen > 0 as ::core::ffi::c_int
        && *errMsg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
    {
        *errMsg.offset(0 as ::core::ffi::c_int as isize) = ' ' as i32 as ::core::ffi::c_char;
    }
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getCount(mut objType: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if IsOpenFlag == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if objType < swmm_GAGE as ::core::ffi::c_int || objType > swmm_LINK as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return Nobjects[objType as usize];
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getName(
    mut objType: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    mut name: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) {
    let mut idName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *name.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    if IsOpenFlag == 0 {
        return;
    }
    if objType < swmm_GAGE as ::core::ffi::c_int || objType > swmm_LINK as ::core::ffi::c_int {
        return;
    }
    if index < 0 as ::core::ffi::c_int || index >= Nobjects[objType as usize] {
        return;
    }
    match objType {
        0 => {
            idName = (*Gage.offset(index as isize)).ID;
        }
        1 => {
            idName = (*Subcatch.offset(index as isize)).ID;
        }
        2 => {
            idName = (*Node.offset(index as isize)).ID;
        }
        3 => {
            idName = (*Link.offset(index as isize)).ID;
        }
        _ => {}
    }
    if !idName.is_null() {
        sstrncpy(name, idName, size as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getIndex(
    mut objType: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if IsOpenFlag == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if objType < swmm_GAGE as ::core::ffi::c_int || objType > swmm_LINK as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return project_findObject(objType, name);
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if IsOpenFlag == 0 {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if property < 100 as ::core::ffi::c_int {
        return getSystemValue(property);
    }
    if property < 200 as ::core::ffi::c_int {
        return getGageValue(property, index);
    }
    if property < 300 as ::core::ffi::c_int {
        return getSubcatchValue(property, index);
    }
    if property < 400 as ::core::ffi::c_int {
        return getNodeValue(property, index);
    }
    if property < 500 as ::core::ffi::c_int {
        return getLinkValue(property, index);
    }
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_setValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) {
    if IsOpenFlag == 0 {
        return;
    }
    match property {
        100 => {
            if index < 0 as ::core::ffi::c_int
                || index >= Nobjects[GAGE as ::core::ffi::c_int as usize]
            {
                return;
            }
            if value >= 0.0f64 {
                (*Gage.offset(index as isize)).apiRainfall = value;
            }
            return;
        }
        206 => {
            if IsStartedFlag == 0
                && index >= 0 as ::core::ffi::c_int
                && index < Nobjects[SUBCATCH as ::core::ffi::c_int as usize]
            {
                (*Subcatch.offset(index as isize)).rptFlag = (value > 0.0f64) as ::core::ffi::c_int;
            }
            return;
        }
        306 => {
            setNodeLatFlow(index, value);
            return;
        }
        304 => {
            setOutfallStage(index, value);
            return;
        }
        309 => {
            if IsStartedFlag == 0
                && index >= 0 as ::core::ffi::c_int
                && index < Nobjects[NODE as ::core::ffi::c_int as usize]
            {
                (*Node.offset(index as isize)).rptFlag = (value > 0.0f64) as ::core::ffi::c_int;
            }
            return;
        }
        407 => {
            setLinkSetting(index, value);
            return;
        }
        414 => {
            if IsStartedFlag == 0
                && index >= 0 as ::core::ffi::c_int
                && index < Nobjects[LINK as ::core::ffi::c_int as usize]
            {
                (*Link.offset(index as isize)).rptFlag = (value > 0.0f64) as ::core::ffi::c_int;
            }
            return;
        }
        3 => {
            setRoutingStep(value);
            return;
        }
        5 => {
            if IsStartedFlag == 0 && value > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                ReportStep = crate::ccast::f64_to_i32(value);
            }
            return;
        }
        7 => {
            if IsStartedFlag == 0 {
                RptFlags.disabled = (value > 0.0f64) as ::core::ffi::c_int as ::core::ffi::c_char;
            }
            return;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn swmm_getSavedValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    mut period: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if IsOpenFlag == 0 {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if IsStartedFlag != 0 {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if period < 1 as ::core::ffi::c_int || period as ::core::ffi::c_long > Nperiods {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if property == swmm_CURRENTDATE as ::core::ffi::c_int {
        return getSavedDate(period);
    }
    if property >= 200 as ::core::ffi::c_int && property < 300 as ::core::ffi::c_int {
        return getSavedSubcatchValue(property, index, period);
    }
    if property < 400 as ::core::ffi::c_int {
        return getSavedNodeValue(property, index, period);
    }
    if property < 500 as ::core::ffi::c_int {
        return getSavedLinkValue(property, index, period);
    }
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn swmm_decodeDate(
    mut date: ::core::ffi::c_double,
    mut year: *mut ::core::ffi::c_int,
    mut month: *mut ::core::ffi::c_int,
    mut day: *mut ::core::ffi::c_int,
    mut hour: *mut ::core::ffi::c_int,
    mut minute: *mut ::core::ffi::c_int,
    mut second: *mut ::core::ffi::c_int,
    mut dayOfWeek: *mut ::core::ffi::c_int,
) {
    datetime_decodeDate(date as DateTime, year, month, day);
    datetime_decodeTime(date as DateTime, hour, minute, second);
    *dayOfWeek = datetime_dayOfWeek(date as DateTime);
}
unsafe extern "C" fn getGageValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if index < 0 as ::core::ffi::c_int || index >= Nobjects[GAGE as ::core::ffi::c_int as usize] {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if property == swmm_GAGE_RAINFALL as ::core::ffi::c_int {
        return (*Gage.offset(index as isize)).reportRainfall;
    }
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
unsafe extern "C" fn getSubcatchValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut subcatch: *mut TSubcatch = ::core::ptr::null_mut::<TSubcatch>();
    if index < 0 as ::core::ffi::c_int || index >= Nobjects[SUBCATCH as ::core::ffi::c_int as usize]
    {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    subcatch = Subcatch.offset(index as isize) as *mut TSubcatch;
    match property {
        200 => return (*subcatch).area * UCF(LANDAREA as ::core::ffi::c_int),
        201 => return (*subcatch).gage as ::core::ffi::c_double,
        202 => {
            if (*subcatch).gage >= 0 as ::core::ffi::c_int {
                return (*Gage.offset((*subcatch).gage as isize)).reportRainfall;
            } else {
                return 0.0f64;
            }
        }
        203 => return (*subcatch).evapLoss * UCF(EVAPRATE as ::core::ffi::c_int),
        204 => return (*subcatch).infilLoss * UCF(RAINFALL as ::core::ffi::c_int),
        205 => return (*subcatch).newRunoff * UCF(FLOW as ::core::ffi::c_int),
        206 => {
            return ((*subcatch).rptFlag > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_double;
        }
        _ => return 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
}
unsafe extern "C" fn getNodeValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut node: *mut TNode = ::core::ptr::null_mut::<TNode>();
    if index < 0 as ::core::ffi::c_int || index >= Nobjects[NODE as ::core::ffi::c_int as usize] {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    node = Node.offset(index as isize) as *mut TNode;
    match property {
        300 => return (*node).type_0 as ::core::ffi::c_double,
        301 => return (*node).invertElev * UCF(LENGTH as ::core::ffi::c_int),
        302 => return (*node).fullDepth * UCF(LENGTH as ::core::ffi::c_int),
        303 => return (*node).newDepth * UCF(LENGTH as ::core::ffi::c_int),
        304 => {
            return ((*node).newDepth + (*node).invertElev) * UCF(LENGTH as ::core::ffi::c_int);
        }
        305 => return (*node).newVolume * UCF(VOLUME as ::core::ffi::c_int),
        306 => return (*node).newLatFlow * UCF(FLOW as ::core::ffi::c_int),
        307 => return (*node).inflow * UCF(FLOW as ::core::ffi::c_int),
        308 => return (*node).overflow * UCF(FLOW as ::core::ffi::c_int),
        309 => {
            return ((*node).rptFlag > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_double;
        }
        _ => return 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
}
unsafe extern "C" fn getLinkValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut link: *mut TLink = ::core::ptr::null_mut::<TLink>();
    if index < 0 as ::core::ffi::c_int || index >= Nobjects[LINK as ::core::ffi::c_int as usize] {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    link = Link.offset(index as isize) as *mut TLink;
    match property {
        400 => return (*link).type_0 as ::core::ffi::c_double,
        401 => return (*link).node1 as ::core::ffi::c_double,
        402 => return (*link).node2 as ::core::ffi::c_double,
        403 => {
            if (*link).type_0 == CONDUIT as ::core::ffi::c_int {
                return (*Conduit.offset((*link).subIndex as isize)).length
                    * UCF(LENGTH as ::core::ffi::c_int);
            } else {
                return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        404 => {
            if (*link).type_0 == CONDUIT as ::core::ffi::c_int {
                return (*Conduit.offset((*link).subIndex as isize)).slope;
            } else {
                return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        405 => return (*link).xsect.yFull * UCF(LENGTH as ::core::ffi::c_int),
        406 => return (*link).qFull * UCF(FLOW as ::core::ffi::c_int),
        410 => {
            return (*link).newFlow
                * UCF(FLOW as ::core::ffi::c_int)
                * (*link).direction as ::core::ffi::c_double;
        }
        412 => {
            return link_getVelocity(index, fabs((*link).newFlow), (*link).newDepth)
                * UCF(LENGTH as ::core::ffi::c_int);
        }
        411 => return (*link).newDepth * UCF(LENGTH as ::core::ffi::c_int),
        413 => {
            if (*link).type_0 == CONDUIT as ::core::ffi::c_int {
                return xsect_getWofY(&raw mut (*link).xsect, (*link).newDepth)
                    * UCF(LENGTH as ::core::ffi::c_int);
            } else {
                return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        407 => return (*link).setting,
        408 => {
            if (*link).setting > 0.0f64 {
                return (getDateTime(NewRoutingTime) as ::core::ffi::c_double - (*link).timeLastSet)
                    * 24.0f64;
            } else {
                return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        409 => {
            if (*link).setting == 0.0f64 {
                return (getDateTime(NewRoutingTime) as ::core::ffi::c_double - (*link).timeLastSet)
                    * 24.0f64;
            } else {
                return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
        414 => {
            return ((*link).rptFlag > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_double;
        }
        _ => return 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
}
unsafe extern "C" fn getSystemValue(mut property: ::core::ffi::c_int) -> ::core::ffi::c_double {
    match property {
        0 => return StartDateTime as ::core::ffi::c_double,
        1 => return StartDateTime as ::core::ffi::c_double + ElapsedTime,
        2 => return ElapsedTime,
        3 => return RouteStep,
        4 => return getMaxRouteStep(),
        5 => return ReportStep as ::core::ffi::c_double,
        6 => return Nperiods as ::core::ffi::c_double,
        7 => return RptFlags.disabled as ::core::ffi::c_double,
        8 => return FlowUnits as ::core::ffi::c_double,
        _ => return 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
}
unsafe extern "C" fn setNodeLatFlow(
    mut index: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) {
    if index < 0 as ::core::ffi::c_int || index >= Nobjects[NODE as ::core::ffi::c_int as usize] {
        return;
    }
    (*Node.offset(index as isize)).apiExtInflow = value / UCF(FLOW as ::core::ffi::c_int);
}
unsafe extern "C" fn setOutfallStage(
    mut index: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) {
    let mut node: *mut TNode = ::core::ptr::null_mut::<TNode>();
    if index < 0 as ::core::ffi::c_int || index >= Nobjects[NODE as ::core::ffi::c_int as usize] {
        return;
    }
    node = Node.offset(index as isize) as *mut TNode;
    if (*node).type_0 != OUTFALL as ::core::ffi::c_int {
        return;
    }
    (*Outfall.offset((*node).subIndex as isize)).fixedStage =
        value / UCF(LENGTH as ::core::ffi::c_int);
    (*Outfall.offset((*node).subIndex as isize)).type_0 = FIXED_OUTFALL as ::core::ffi::c_int;
}
unsafe extern "C" fn setLinkSetting(
    mut index: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) {
    let mut link: *mut TLink = ::core::ptr::null_mut::<TLink>();
    if index < 0 as ::core::ffi::c_int || index >= Nobjects[LINK as ::core::ffi::c_int as usize] {
        return;
    }
    link = Link.offset(index as isize) as *mut TLink;
    if value < 0.0f64 || (*link).type_0 == CONDUIT as ::core::ffi::c_int {
        return;
    }
    if (*link).type_0 != PUMP as ::core::ffi::c_int && value > 1.0f64 {
        value = 1.0f64;
    }
    if (*link).targetSetting == value {
        return;
    }
    (*link).targetSetting = value;
    if (*link).targetSetting * (*link).setting == 0.0f64 {
        (*link).timeLastSet = StartDateTime as ::core::ffi::c_double + ElapsedTime;
    }
    link_setSetting(index, 0.0f64);
}
unsafe extern "C" fn getSavedDate(mut period: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut days: ::core::ffi::c_double = 0.;
    output_readDateTime(period as ::core::ffi::c_long, &raw mut days);
    return days;
}
unsafe extern "C" fn getSavedSubcatchValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    mut period: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    extern "C" {
        static mut SubcatchResults: *mut ::core::ffi::c_float;
    }
    let mut outIndex: ::core::ffi::c_int =
        (*Subcatch.offset(index as isize)).rptFlag - 1 as ::core::ffi::c_int;
    if outIndex < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    output_readSubcatchResults(period as ::core::ffi::c_long, outIndex);
    match property {
        202 => {
            return *SubcatchResults.offset(SUBCATCH_RAINFALL as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        203 => {
            return *SubcatchResults.offset(SUBCATCH_EVAP as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        204 => {
            return *SubcatchResults.offset(SUBCATCH_INFIL as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        205 => {
            return *SubcatchResults.offset(SUBCATCH_RUNOFF as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        _ => return 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
}
unsafe extern "C" fn getSavedNodeValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    mut period: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    extern "C" {
        static mut NodeResults: *mut ::core::ffi::c_float;
    }
    let mut outIndex: ::core::ffi::c_int =
        (*Node.offset(index as isize)).rptFlag - 1 as ::core::ffi::c_int;
    if outIndex < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    output_readNodeResults(period as ::core::ffi::c_long, outIndex);
    match property {
        303 => {
            return *NodeResults.offset(NODE_DEPTH as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        304 => {
            return *NodeResults.offset(NODE_HEAD as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        305 => {
            return *NodeResults.offset(NODE_VOLUME as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        306 => {
            return *NodeResults.offset(NODE_LATFLOW as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        307 => {
            return *NodeResults.offset(NODE_INFLOW as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        308 => {
            return *NodeResults.offset(NODE_OVERFLOW as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        _ => return 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
}
unsafe extern "C" fn getSavedLinkValue(
    mut property: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    mut period: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut y: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    extern "C" {
        static mut LinkResults: *mut ::core::ffi::c_float;
    }
    let mut outIndex: ::core::ffi::c_int =
        (*Link.offset(index as isize)).rptFlag - 1 as ::core::ffi::c_int;
    if outIndex < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    output_readLinkResults(period as ::core::ffi::c_long, outIndex);
    match property {
        410 => {
            return *LinkResults.offset(LINK_FLOW as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        411 => {
            return *LinkResults.offset(LINK_DEPTH as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        412 => {
            return *LinkResults.offset(LINK_VELOCITY as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        413 => {
            y = *LinkResults.offset(LINK_DEPTH as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / UCF(LENGTH as ::core::ffi::c_int);
            w = xsect_getWofY(&raw mut (*Link.offset(index as isize)).xsect, y);
            return w * UCF(LENGTH as ::core::ffi::c_int);
        }
        407 => {
            return *LinkResults.offset(LINK_CAPACITY as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double;
        }
        _ => return 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
}
unsafe extern "C" fn getMaxRouteStep() -> ::core::ffi::c_double {
    let mut tmpCourantFactor: ::core::ffi::c_double = CourantFactor;
    let mut result: ::core::ffi::c_double = RouteStep;
    if IsStartedFlag == 0 || RouteModel != DW as ::core::ffi::c_int {
        return result;
    }
    CourantFactor = 1.0f64;
    result = routing_getRoutingStep(RouteModel, MinRouteStep);
    CourantFactor = tmpCourantFactor;
    return result;
}
unsafe extern "C" fn setRoutingStep(mut value: ::core::ffi::c_double) {
    if value <= 0.0f64 {
        return;
    }
    if value <= MinRouteStep {
        value = MinRouteStep;
    }
    CourantFactor = 0.0f64;
    RouteStep = value;
}
#[no_mangle]
pub unsafe extern "C" fn UCF(mut u: ::core::ffi::c_int) -> ::core::ffi::c_double {
    if u < FLOW as ::core::ffi::c_int {
        return Ucf[u as usize][UnitSystem as usize];
    } else {
        return Qcf[FlowUnits as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn sstrncpy(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut n: size_t,
) -> size_t {
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if n > 0 as size_t {
        while *src.offset(offset as isize) as ::core::ffi::c_int != '\0' as i32 {
            if offset as size_t == n {
                break;
            }
            *dest.offset(offset as isize) = *src.offset(offset as isize);
            offset += 1;
        }
    }
    *dest.offset(offset as isize) = '\0' as i32 as ::core::ffi::c_char;
    return strlen(dest);
}
#[no_mangle]
pub unsafe extern "C" fn sstrcat(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut size: size_t,
) -> size_t {
    let mut dest_len: size_t = 0;
    let mut src_len: size_t = 0;
    let mut offset: size_t = 0;
    let mut src_index: size_t = 0;
    dest_len = strlen(dest);
    src_len = strlen(src);
    offset = dest_len;
    src_index = 0 as size_t;
    while *src.offset(src_index as isize) as ::core::ffi::c_int != '\0' as i32 {
        *dest.offset(offset as isize) = *src.offset(src_index as isize);
        offset = offset.wrapping_add(1);
        src_index = src_index.wrapping_add(1);
        if offset == size.wrapping_sub(1 as size_t) {
            break;
        }
    }
    *dest.offset(offset as isize) = '\0' as i32 as ::core::ffi::c_char;
    return strlen(dest);
}
#[no_mangle]
pub unsafe extern "C" fn strcomp(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (if *s1.offset(i as isize) as ::core::ffi::c_int >= 'a' as i32
        && *s1.offset(i as isize) as ::core::ffi::c_int <= 'z' as i32
    {
        *s1.offset(i as isize) as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
    } else {
        *s1.offset(i as isize) as ::core::ffi::c_int
    }) == (if *s2.offset(i as isize) as ::core::ffi::c_int >= 'a' as i32
        && *s2.offset(i as isize) as ::core::ffi::c_int <= 'z' as i32
    {
        *s2.offset(i as isize) as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
    } else {
        *s2.offset(i as isize) as ::core::ffi::c_int
    }) {
        if *s1.offset((i + 1 as ::core::ffi::c_int) as isize) == 0
            && *s2.offset((i + 1 as ::core::ffi::c_int) as isize) == 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getTempFileName(
    mut fname: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    sstrncpy(
        fname,
        b"swmmXXXXXX\0" as *const u8 as *const ::core::ffi::c_char,
        MAXFNAME as size_t,
    );
    mkstemp(fname);
    return fname;
}
#[no_mangle]
pub unsafe extern "C" fn getElapsedTime(
    mut aDate: DateTime,
    mut days: *mut ::core::ffi::c_int,
    mut hrs: *mut ::core::ffi::c_int,
    mut mins: *mut ::core::ffi::c_int,
) {
    let mut x: DateTime = 0.;
    let mut secs: ::core::ffi::c_int = 0;
    x = aDate - ReportStart;
    if x <= 0.0f64 {
        *days = 0 as ::core::ffi::c_int;
        *hrs = 0 as ::core::ffi::c_int;
        *mins = 0 as ::core::ffi::c_int;
    } else {
        *days = crate::ccast::f64_to_i32(x);
        datetime_decodeTime(x, hrs, mins, &raw mut secs);
    };
}
#[no_mangle]
pub unsafe extern "C" fn getDateTime(mut elapsedMsec: ::core::ffi::c_double) -> DateTime {
    return datetime_addSeconds(
        StartDateTime,
        (elapsedMsec + 1 as ::core::ffi::c_int as ::core::ffi::c_double) / 1000.0f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn isRelativePath(
    mut fname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !strchr(fname, ':' as i32).is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if *fname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\\' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    if *fname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn getAbsolutePath(
    mut fname: *const ::core::ffi::c_char,
    mut absPath: *mut ::core::ffi::c_char,
    mut size: size_t,
) {
    let mut endOfDir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if fname.is_null() || strlen(fname) == 0 as size_t {
        return;
    }
    if isRelativePath(fname) != 0 {
        realpath(fname, absPath);
    } else {
        sstrncpy(absPath, fname, strlen(fname));
    }
    endOfDir = strrchr(absPath, '/' as i32);
    if !endOfDir.is_null() {
        *endOfDir.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addAbsolutePath(
    mut fname: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut n: size_t = 0;
    let mut buffer: [::core::ffi::c_char; 259] = [0; 259];
    if isRelativePath(fname) != 0 {
        n = snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            MAXFNAME as size_t,
            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut InpDir as *mut ::core::ffi::c_char,
            fname,
        ) as size_t;
        if n > 0 as size_t {
            sstrncpy(
                fname,
                &raw mut buffer as *mut ::core::ffi::c_char,
                MAXFNAME as size_t,
            );
        }
    }
    return fname;
}
#[no_mangle]
pub unsafe extern "C" fn writecon(mut s: *const ::core::ffi::c_char) {
    fprintf(
        stdout,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        s,
    );
    fflush(stdout);
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
