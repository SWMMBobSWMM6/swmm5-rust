#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn datetime_encodeDate(
        year: ::core::ffi::c_int,
        month: ::core::ffi::c_int,
        day: ::core::ffi::c_int,
    ) -> DateTime;
    fn datetime_encodeTime(
        hour: ::core::ffi::c_int,
        minute: ::core::ffi::c_int,
        second: ::core::ffi::c_int,
    ) -> DateTime;
    fn datetime_decodeTime(
        time: DateTime,
        h: *mut ::core::ffi::c_int,
        m: *mut ::core::ffi::c_int,
        s: *mut ::core::ffi::c_int,
    );
    fn datetime_dayOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_strToDate(s: *mut ::core::ffi::c_char, d: *mut DateTime) -> ::core::ffi::c_int;
    fn datetime_strToTime(s: *mut ::core::ffi::c_char, t: *mut DateTime) -> ::core::ffi::c_int;
    fn inlet_create(nInlets: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inlet_delete();
    fn inlet_validate();
    fn infil_create(n: ::core::ffi::c_int);
    fn infil_delete();
    static mut Finp: TFile;
    static mut Fout: TFile;
    static mut Frpt: TFile;
    static mut Fclimate: TFile;
    static mut Frain: TFile;
    static mut Frunoff: TFile;
    static mut Frdii: TFile;
    static mut Fhotstart1: TFile;
    static mut Fhotstart2: TFile;
    static mut Finflows: TFile;
    static mut Foutflows: TFile;
    static mut Title: [[::core::ffi::c_char; 1025]; 3];
    static mut TempDir: [::core::ffi::c_char; 260];
    static mut RptFlags: TRptFlags;
    static mut Nnodes: [::core::ffi::c_int; 5];
    static mut InertDamping: ::core::ffi::c_int;
    static mut IgnoreSnowmelt: ::core::ffi::c_int;
    static mut IgnoreGwater: ::core::ffi::c_int;
    static mut IgnoreRouting: ::core::ffi::c_int;
    static mut IgnoreQuality: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut WetStep: ::core::ffi::c_int;
    static mut DryStep: ::core::ffi::c_int;
    static mut ReportStep: ::core::ffi::c_int;
    static mut RuleStep: ::core::ffi::c_int;
    static mut SweepStart: ::core::ffi::c_int;
    static mut SweepEnd: ::core::ffi::c_int;
    static mut MaxTrials: ::core::ffi::c_int;
    static mut NumThreads: ::core::ffi::c_int;
    static mut NumEvents: ::core::ffi::c_int;
    static mut NormalFlowLtd: ::core::ffi::c_int;
    static mut SlopeWeighting: ::core::ffi::c_int;
    static mut Compatibility: ::core::ffi::c_int;
    static mut SkipSteadyState: ::core::ffi::c_int;
    static mut IgnoreRainfall: ::core::ffi::c_int;
    static mut IgnoreRDII: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut AllowPonding: ::core::ffi::c_int;
    static mut Nlinks: [::core::ffi::c_int; 5];
    static mut UnitSystem: ::core::ffi::c_int;
    static mut FlowUnits: ::core::ffi::c_int;
    static mut InfilModel: ::core::ffi::c_int;
    static mut RouteModel: ::core::ffi::c_int;
    static mut ForceMainEqn: ::core::ffi::c_int;
    static mut LinkOffsets: ::core::ffi::c_int;
    static mut SurchargeMethod: ::core::ffi::c_int;
    static mut MinRouteStep: ::core::ffi::c_double;
    static mut StartDryDays: ::core::ffi::c_double;
    static mut RouteStep: ::core::ffi::c_double;
    static mut LengtheningStep: ::core::ffi::c_double;
    static mut SysFlowTol: ::core::ffi::c_double;
    static mut CourantFactor: ::core::ffi::c_double;
    static mut MinSurfArea: ::core::ffi::c_double;
    static mut MinSlope: ::core::ffi::c_double;
    static mut HeadTol: ::core::ffi::c_double;
    static mut LatFlowTol: ::core::ffi::c_double;
    static mut CrownCutoff: ::core::ffi::c_double;
    static mut StartDate: DateTime;
    static mut StartTime: DateTime;
    static mut StartDateTime: DateTime;
    static mut EndDate: DateTime;
    static mut EndTime: DateTime;
    static mut EndDateTime: DateTime;
    static mut ReportStartDate: DateTime;
    static mut ReportStartTime: DateTime;
    static mut ReportStart: DateTime;
    static mut TotalDuration: ::core::ffi::c_double;
    static mut Temp: TTemp;
    static mut Evap: TEvap;
    static mut Wind: TWind;
    static mut Snow: TSnow;
    static mut Adjust: TAdjust;
    static mut Snowmelt: *mut TSnowmelt;
    static mut Gage: *mut TGage;
    static mut Subcatch: *mut TSubcatch;
    static mut Aquifer: *mut TAquifer;
    static mut UnitHyd: *mut TUnitHyd;
    static mut Node: *mut TNode;
    static mut Outfall: *mut TOutfall;
    static mut Divider: *mut TDivider;
    static mut Storage: *mut TStorage;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Pump: *mut TPump;
    static mut Orifice: *mut TOrifice;
    static mut Weir: *mut TWeir;
    static mut Outlet: *mut TOutlet;
    static mut Pollut: *mut TPollut;
    static mut Landuse: *mut TLanduse;
    static mut Pattern: *mut TPattern;
    static mut Curve: *mut TTable;
    static mut Tseries: *mut TTable;
    static mut Transect: *mut TTransect;
    static mut Shape: *mut TShape;
    static mut Event: *mut TEvent;
    fn input_countObjects() -> ::core::ffi::c_int;
    fn input_readData() -> ::core::ffi::c_int;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn report_writeWarningMsg(msg: *mut ::core::ffi::c_char, id: *mut ::core::ffi::c_char);
    fn report_writeTseriesErrorMsg(code: ::core::ffi::c_int, tseries: *mut TTable);
    fn climate_validate();
    fn climate_initState();
    fn snow_validateSnowmelt(snowIndex: ::core::ffi::c_int);
    fn snow_initSnowmelt(snowIndex: ::core::ffi::c_int);
    fn gwater_deleteFlowExpression(subcatch: ::core::ffi::c_int);
    fn gwater_validateAquifer(aquifer: ::core::ffi::c_int);
    fn rdii_deleteRdiiInflow(node: ::core::ffi::c_int);
    fn rdii_initUnitHyd(unitHyd: ::core::ffi::c_int);
    fn dynwave_validate();
    fn treatmnt_delete(node: ::core::ffi::c_int);
    fn gage_validate(gage: ::core::ffi::c_int);
    fn gage_initState(gage: ::core::ffi::c_int);
    fn subcatch_validate(subcatch: ::core::ffi::c_int);
    fn subcatch_initState(subcatch: ::core::ffi::c_int);
    fn node_validate(node: ::core::ffi::c_int);
    fn node_initState(node: ::core::ffi::c_int);
    fn inflow_initDwfPattern(pattern: ::core::ffi::c_int);
    fn inflow_deleteExtInflows(node: ::core::ffi::c_int);
    fn inflow_deleteDwfInflows(node: ::core::ffi::c_int);
    fn link_validate(link: ::core::ffi::c_int);
    fn link_initState(link: ::core::ffi::c_int);
    fn transect_create(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn transect_delete();
    fn street_create(nStreets: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn street_delete();
    fn shape_validate(shape: *mut TShape, curve: *mut TTable) -> ::core::ffi::c_int;
    fn controls_create(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn controls_delete();
    fn table_deleteEntries(table: *mut TTable);
    fn table_init(table: *mut TTable);
    fn table_validate(table: *mut TTable) -> ::core::ffi::c_int;
    fn table_tseriesInit(table: *mut TTable);
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
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
    fn sstrcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        destsize: size_t,
    ) -> size_t;
    fn writecon(s: *const ::core::ffi::c_char);
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut FlowUnitWords: [*mut ::core::ffi::c_char; 0];
    static mut ForceMainEqnWords: [*mut ::core::ffi::c_char; 0];
    static mut InertDampingWords: [*mut ::core::ffi::c_char; 0];
    static mut InfilModelWords: [*mut ::core::ffi::c_char; 0];
    static mut LinkOffsetWords: [*mut ::core::ffi::c_char; 0];
    static mut NormalFlowWords: [*mut ::core::ffi::c_char; 0];
    static mut NoYesWords: [*mut ::core::ffi::c_char; 0];
    static mut OldRouteModelWords: [*mut ::core::ffi::c_char; 0];
    static mut OptionWords: [*mut ::core::ffi::c_char; 0];
    static mut RouteModelWords: [*mut ::core::ffi::c_char; 0];
    static mut SurchargeWords: [*mut ::core::ffi::c_char; 0];
    fn lid_create(lidCount: ::core::ffi::c_int, subcatchCount: ::core::ffi::c_int);
    fn lid_delete();
    fn lid_validate();
    fn lid_initState();
    fn HTcreate() -> *mut HTtable;
    fn HTinsert(
        _: *mut HTtable,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn HTfind(_: *mut HTtable, _: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn HTfindKey(_: *mut HTtable, _: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn HTfree(_: *mut HTtable);
    fn AllocInit() -> *mut alloc_handle_t;
    fn Alloc(_: ::core::ffi::c_long) -> *mut ::core::ffi::c_char;
    fn AllocFreePool();
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
pub type TempType = ::core::ffi::c_uint;
pub const FILE_TEMP: TempType = 2;
pub const TSERIES_TEMP: TempType = 1;
pub const NO_TEMP: TempType = 0;
pub type WindType = ::core::ffi::c_uint;
pub const FILE_WIND: WindType = 1;
pub const MONTHLY_WIND: WindType = 0;
pub type EvapType = ::core::ffi::c_uint;
pub const DRYONLY: EvapType = 6;
pub const RECOVERY: EvapType = 5;
pub const FILE_EVAP: EvapType = 4;
pub const TEMPERATURE_EVAP: EvapType = 3;
pub const TIMESERIES_EVAP: EvapType = 2;
pub const MONTHLY_EVAP: EvapType = 1;
pub const CONSTANT_EVAP: EvapType = 0;
pub type NormalizerType = ::core::ffi::c_uint;
pub const PER_CURB: NormalizerType = 1;
pub const PER_AREA: NormalizerType = 0;
pub type BuildupType = ::core::ffi::c_uint;
pub const EXTERNAL_BUILDUP: BuildupType = 4;
pub const SATUR_BUILDUP: BuildupType = 3;
pub const EXPON_BUILDUP: BuildupType = 2;
pub const POWER_BUILDUP: BuildupType = 1;
pub const NO_BUILDUP: BuildupType = 0;
pub type WashoffType = ::core::ffi::c_uint;
pub const EMC_WASHOFF: WashoffType = 3;
pub const RATING_WASHOFF: WashoffType = 2;
pub const EXPON_WASHOFF: WashoffType = 1;
pub const NO_WASHOFF: WashoffType = 0;
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
pub type CompatibilityType = ::core::ffi::c_uint;
pub const SWMM4: CompatibilityType = 2;
pub const SWMM3: CompatibilityType = 1;
pub const SWMM5: CompatibilityType = 0;
pub type NormalFlowType = ::core::ffi::c_uint;
pub const NEITHER: NormalFlowType = 3;
pub const BOTH: NormalFlowType = 2;
pub const FROUDE: NormalFlowType = 1;
pub const SLOPE: NormalFlowType = 0;
pub type InertialDampingType = ::core::ffi::c_uint;
pub const FULL_DAMPING: InertialDampingType = 2;
pub const PARTIAL_DAMPING: InertialDampingType = 1;
pub const NO_DAMPING: InertialDampingType = 0;
pub type SurchargeMethodType = ::core::ffi::c_uint;
pub const SLOT: SurchargeMethodType = 1;
pub const EXTRAN: SurchargeMethodType = 0;
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
pub type InputOptionType = ::core::ffi::c_uint;
pub const SURCHARGE_METHOD: InputOptionType = 42;
pub const NUM_THREADS: InputOptionType = 41;
pub const MIN_ROUTE_STEP: InputOptionType = 40;
pub const IGNORE_RDII: InputOptionType = 39;
pub const LAT_FLOW_TOL: InputOptionType = 38;
pub const SYS_FLOW_TOL: InputOptionType = 37;
pub const HEAD_TOL: InputOptionType = 36;
pub const MAX_TRIALS: InputOptionType = 35;
pub const IGNORE_QUALITY: InputOptionType = 34;
pub const IGNORE_ROUTING: InputOptionType = 33;
pub const IGNORE_GWATER: InputOptionType = 32;
pub const IGNORE_SNOWMELT: InputOptionType = 31;
pub const MIN_SLOPE: InputOptionType = 30;
pub const LINK_OFFSETS: InputOptionType = 29;
pub const FORCE_MAIN_EQN: InputOptionType = 28;
pub const IGNORE_RAINFALL: InputOptionType = 27;
pub const TEMPDIR: InputOptionType = 26;
pub const SKIP_STEADY_STATE: InputOptionType = 25;
pub const COMPATIBILITY: InputOptionType = 24;
pub const MIN_SURFAREA: InputOptionType = 23;
pub const LENGTHENING_STEP: InputOptionType = 22;
pub const NORMAL_FLOW_LTD: InputOptionType = 21;
pub const VARIABLE_STEP: InputOptionType = 20;
pub const SLOPE_WEIGHTING: InputOptionType = 19;
pub const INERT_DAMPING: InputOptionType = 18;
pub const ALLOW_PONDING: InputOptionType = 17;
pub const REPORT_STEP: InputOptionType = 16;
pub const RULE_STEP: InputOptionType = 15;
pub const ROUTE_STEP: InputOptionType = 14;
pub const DRY_STEP: InputOptionType = 13;
pub const WET_STEP: InputOptionType = 12;
pub const START_DRY_DAYS: InputOptionType = 11;
pub const SWEEP_END: InputOptionType = 10;
pub const SWEEP_START: InputOptionType = 9;
pub const REPORT_START_TIME: InputOptionType = 8;
pub const REPORT_START_DATE: InputOptionType = 7;
pub const END_TIME: InputOptionType = 6;
pub const END_DATE: InputOptionType = 5;
pub const START_TIME: InputOptionType = 4;
pub const START_DATE: InputOptionType = 3;
pub const ROUTE_MODEL: InputOptionType = 2;
pub const INFIL_MODEL: InputOptionType = 1;
pub const FLOW_UNITS: InputOptionType = 0;
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
pub type InfilType = ::core::ffi::c_uint;
pub const CURVE_NUMBER: InfilType = 4;
pub const MOD_GREEN_AMPT: InfilType = 3;
pub const GREEN_AMPT: InfilType = 2;
pub const MOD_HORTON: InfilType = 1;
pub const HORTON: InfilType = 0;
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
pub const ERR_RPT_FILE: ErrorType = 305;
pub const ERR_INP_FILE: ErrorType = 303;
pub const ERR_FILE_NAME: ErrorType = 301;
pub type HTtable = *mut HTentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HTentry {
    pub key: *mut ::core::ffi::c_char,
    pub data: ::core::ffi::c_int,
    pub next: *mut HTentry,
}
pub const ERR_REPORT_STEP: ErrorType = 195;
pub const ERR_REPORT_DATE: ErrorType = 193;
pub const ERR_START_DATE: ErrorType = 191;
pub const ERR_MEMORY: ErrorType = 101;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alloc_handle_t {
    pub dummy: ::core::ffi::c_long,
}
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_DATETIME: ErrorType = 213;
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
pub const ERR_NAME: ErrorType = 209;
pub const ERR_DUP_NAME: ErrorType = 207;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_LINE_LENGTH: ErrorType = 201;
pub const ERR_INPUT: ErrorType = 200;
pub const ERR_LID_CAPTURE_AREA: ErrorType = 188;
pub const ERR_LID_AREAS: ErrorType = 187;
pub const ERR_LID_PARAMS: ErrorType = 185;
pub const ERR_LID_LAYER: ErrorType = 184;
pub const ERR_LID_TYPE: ErrorType = 183;
pub const ERR_SNOWPACK_PARAMS: ErrorType = 182;
pub const ERR_SNOWMELT_PARAMS: ErrorType = 181;
pub const ERR_TIMESERIES_SEQUENCE: ErrorType = 173;
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
pub const MAXTITLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAXFNAME: ::core::ffi::c_int = 259 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BIG: ::core::ffi::c_double = 1.0E10f64;
pub const MISSING: ::core::ffi::c_double = -1.0E10f64;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const FMT11: [::core::ffi::c_char; 38] = unsafe {
    ::core::mem::transmute::<[u8; 38], [::core::ffi::c_char; 38]>(
        *b"\n    Cannot use duplicate file names.\0",
    )
};
pub const FMT12: [::core::ffi::c_char; 29] = unsafe {
    ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
        *b"\n    Cannot open input file \0",
    )
};
pub const FMT13: [::core::ffi::c_char; 30] = unsafe {
    ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
        *b"\n    Cannot open report file \0",
    )
};
pub const WARN06: [::core::ffi::c_char; 73] = unsafe {
    ::core::mem::transmute::<[u8; 73], [::core::ffi::c_char; 73]>(
        *b"WARNING 06: dry weather time step increased to the wet weather time step\0",
    )
};
pub const WARN07: [::core::ffi::c_char; 67] = unsafe {
    ::core::mem::transmute::<[u8; 67], [::core::ffi::c_char; 67]>(
        *b"WARNING 07: routing time step reduced to the wet weather time step\0",
    )
};
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn omp_get_max_threads() -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
static mut Htable: [*mut HTtable; 18] = [::core::ptr::null::<HTtable>() as *mut HTtable; 18];
static mut MemPoolAllocated: ::core::ffi::c_char = 0;
#[no_mangle]
pub unsafe extern "C" fn project_open(
    mut f1: *const ::core::ffi::c_char,
    mut f2: *const ::core::ffi::c_char,
    mut f3: *const ::core::ffi::c_char,
) {
    initPointers();
    setDefaults();
    openFiles(f1, f2, f3);
}
#[no_mangle]
pub unsafe extern "C" fn project_readInput() {
    createHashTables();
    input_countObjects();
    createObjects();
    input_readData();
    if ErrorCode != 0 {
        return;
    }
    StartDateTime = StartDate + StartTime;
    EndDateTime = EndDate + EndTime;
    ReportStart = ReportStartDate + ReportStartTime;
    ReportStart = if ReportStart >= StartDateTime {
        ReportStart
    } else {
        StartDateTime
    };
    if EndDateTime <= StartDateTime {
        report_writeErrorMsg(
            ERR_START_DATE as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else if EndDateTime <= ReportStart {
        report_writeErrorMsg(
            ERR_REPORT_DATE as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        TotalDuration = floor(
            (EndDateTime as ::core::ffi::c_double - StartDateTime as ::core::ffi::c_double)
                * SECperDAY as ::core::ffi::c_double,
        );
        if ReportStep as ::core::ffi::c_double > TotalDuration {
            ReportStep = crate::ccast::f64_to_i32(TotalDuration);
        }
        if (ReportStep as ::core::ffi::c_double) < RouteStep {
            report_writeErrorMsg(
                ERR_REPORT_STEP as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        TotalDuration *= 1000.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn project_validate() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[CURVE as ::core::ffi::c_int as usize] {
        err = table_validate(Curve.offset(i as isize) as *mut TTable);
        if err != 0 {
            report_writeErrorMsg(
                ERR_CURVE_SEQUENCE as ::core::ffi::c_int,
                (*Curve.offset(i as isize)).ID,
            );
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[TSERIES as ::core::ffi::c_int as usize] {
        err = table_validate(Tseries.offset(i as isize) as *mut TTable);
        if err != 0 {
            report_writeTseriesErrorMsg(err, Tseries.offset(i as isize) as *mut TTable);
        }
        i += 1;
    }
    climate_validate();
    lid_validate();
    if Nobjects[SNOWMELT as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        IgnoreSnowmelt = TRUE;
    }
    if Nobjects[AQUIFER as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        IgnoreGwater = TRUE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[AQUIFER as ::core::ffi::c_int as usize] {
        gwater_validateAquifer(i);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        subcatch_validate(i);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        gage_validate(i);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[SNOWMELT as ::core::ffi::c_int as usize] {
        snow_validateSnowmelt(i);
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[CURVE as ::core::ffi::c_int as usize] {
        if (*Curve.offset(i as isize)).curveType == SHAPE_CURVE as ::core::ffi::c_int {
            (*Curve.offset(i as isize)).refersTo = j;
            (*Shape.offset(j as isize)).curve = i;
            if shape_validate(
                Shape.offset(j as isize) as *mut TShape,
                Curve.offset(i as isize) as *mut TTable,
            ) == 0
            {
                report_writeErrorMsg(
                    ERR_CURVE_SEQUENCE as ::core::ffi::c_int,
                    (*Curve.offset(i as isize)).ID,
                );
            }
            j += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(i as isize)).oldDepth = (*Node.offset(i as isize)).fullDepth;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        link_validate(i);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        node_validate(i);
        i += 1;
    }
    if DryStep < WetStep {
        report_writeWarningMsg(
            WARN06.as_ptr() as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        DryStep = WetStep;
    }
    if RouteStep > WetStep as ::core::ffi::c_double {
        report_writeWarningMsg(
            WARN07.as_ptr() as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        RouteStep = WetStep as ::core::ffi::c_double;
    }
    if RptFlags.subcatchments as ::core::ffi::c_int == ALL as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
            (*Subcatch.offset(i as isize)).rptFlag = 1 as ::core::ffi::c_int;
            i += 1;
        }
    }
    if RptFlags.nodes as ::core::ffi::c_int == ALL as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
            (*Node.offset(i as isize)).rptFlag = 1 as ::core::ffi::c_int;
            i += 1;
        }
    }
    if RptFlags.links as ::core::ffi::c_int == ALL as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
            (*Link.offset(i as isize)).rptFlag = 1 as ::core::ffi::c_int;
            i += 1;
        }
    }
    if RouteModel == DW as ::core::ffi::c_int {
        dynwave_validate();
    }
    inlet_validate();
    if NumThreads == 0 as ::core::ffi::c_int {
        NumThreads = omp_get_max_threads();
    } else {
        NumThreads = if NumThreads <= omp_get_max_threads() {
            NumThreads
        } else {
            omp_get_max_threads()
        };
    }
    if Nobjects[LINK as ::core::ffi::c_int as usize] < 4 as ::core::ffi::c_int * NumThreads {
        NumThreads = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn project_close() {
    deleteObjects();
    deleteHashTables();
}
#[no_mangle]
pub unsafe extern "C" fn project_init() -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    climate_initState();
    lid_initState();
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[TSERIES as ::core::ffi::c_int as usize] {
        table_tseriesInit(Tseries.offset(j as isize) as *mut TTable);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        gage_initState(j);
        j += 1;
    }
    k = 1 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        subcatch_initState(j);
        if (*Subcatch.offset(j as isize)).rptFlag > 0 as ::core::ffi::c_int {
            (*Subcatch.offset(j as isize)).rptFlag = k;
            k += 1;
        }
        j += 1;
    }
    k = 1 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        node_initState(j);
        if (*Node.offset(j as isize)).rptFlag > 0 as ::core::ffi::c_int {
            (*Node.offset(j as isize)).rptFlag = k;
            k += 1;
        }
        j += 1;
    }
    k = 1 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        link_initState(j);
        if (*Link.offset(j as isize)).rptFlag > 0 as ::core::ffi::c_int {
            (*Link.offset(j as isize)).rptFlag = k;
            k += 1;
        }
        j += 1;
    }
    return ErrorCode;
}
#[no_mangle]
pub unsafe extern "C" fn project_addObject(
    mut type_0: ::core::ffi::c_int,
    mut id: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_long = 0;
    let mut newID: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if project_findObject(type_0, id) >= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    len = strlen(id) as ::core::ffi::c_long;
    newID = Alloc(
        ((len + 1 as ::core::ffi::c_long) as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as ::core::ffi::c_long,
    );
    sstrncpy(newID, id, len as size_t);
    result = HTinsert(Htable[type_0 as usize], newID, n);
    if result == 0 as ::core::ffi::c_int {
        result = -(1 as ::core::ffi::c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn project_findObject(
    mut type_0: ::core::ffi::c_int,
    mut id: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return HTfind(Htable[type_0 as usize], id);
}
#[no_mangle]
pub unsafe extern "C" fn project_findID(
    mut type_0: ::core::ffi::c_int,
    mut id: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return HTfindKey(Htable[type_0 as usize], id);
}
#[no_mangle]
pub unsafe extern "C" fn project_createMatrix(
    mut nrows: ::core::ffi::c_int,
    mut ncols: ::core::ffi::c_int,
) -> *mut *mut ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut a: *mut *mut ::core::ffi::c_double =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_double>();
    let mut size: size_t = (nrows as size_t).wrapping_mul(ncols as size_t);
    if nrows < 1 as ::core::ffi::c_int || ncols < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_double>();
    }
    a = malloc(
        (nrows as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_double>() as size_t),
    ) as *mut *mut ::core::ffi::c_double;
    if a.is_null() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_double>();
    }
    let ref mut fresh44 = *a.offset(0 as ::core::ffi::c_int as isize);
    *fresh44 = malloc(size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t))
        as *mut ::core::ffi::c_double;
    if (*a.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        free(a as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_double>();
    }
    i = 1 as ::core::ffi::c_int;
    while i < nrows {
        let ref mut fresh45 = *a.offset(i as isize);
        *fresh45 = (*a.offset((i - 1 as ::core::ffi::c_int) as isize)).offset(ncols as isize);
        i += 1;
    }
    memset(
        *a.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        size,
    );
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn project_freeMatrix(mut a: *mut *mut ::core::ffi::c_double) {
    if !a.is_null() {
        if !(*a.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            free(*a.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void);
        }
        free(a as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn project_readOption(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut tStep: ::core::ffi::c_double = 0.;
    let mut strDate: [::core::ffi::c_char; 25] = [0; 25];
    let mut aTime: DateTime = 0.;
    let mut aDate: DateTime = 0.;
    k = findmatch(s1, &raw mut OptionWords as *mut *mut ::core::ffi::c_char);
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s1);
    }
    match k {
        0 => {
            m = findmatch(s2, &raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char);
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
            FlowUnits = m;
            if FlowUnits <= MGD as ::core::ffi::c_int {
                UnitSystem = US as ::core::ffi::c_int;
            } else {
                UnitSystem = SI as ::core::ffi::c_int;
            }
        }
        1 => {
            m = findmatch(
                s2,
                &raw mut InfilModelWords as *mut *mut ::core::ffi::c_char,
            );
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
            InfilModel = m;
        }
        2 => {
            m = findmatch(
                s2,
                &raw mut RouteModelWords as *mut *mut ::core::ffi::c_char,
            );
            if m < 0 as ::core::ffi::c_int {
                m = findmatch(
                    s2,
                    &raw mut OldRouteModelWords as *mut *mut ::core::ffi::c_char,
                );
            }
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
            if m == NO_ROUTING as ::core::ffi::c_int {
                IgnoreRouting = TRUE;
            } else {
                RouteModel = m;
            }
            if RouteModel == EKW as ::core::ffi::c_int {
                RouteModel = KW as ::core::ffi::c_int;
            }
        }
        3 => {
            if datetime_strToDate(s2, &raw mut StartDate) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, s2);
            }
        }
        4 => {
            if datetime_strToTime(s2, &raw mut StartTime) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, s2);
            }
        }
        5 => {
            if datetime_strToDate(s2, &raw mut EndDate) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, s2);
            }
        }
        6 => {
            if datetime_strToTime(s2, &raw mut EndTime) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, s2);
            }
        }
        7 => {
            if datetime_strToDate(s2, &raw mut ReportStartDate) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, s2);
            }
        }
        8 => {
            if datetime_strToTime(s2, &raw mut ReportStartTime) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, s2);
            }
        }
        9 | 10 => {
            sstrncpy(
                &raw mut strDate as *mut ::core::ffi::c_char,
                s2,
                24 as size_t,
            );
            sstrcat(
                &raw mut strDate as *mut ::core::ffi::c_char,
                b"/1947\0" as *const u8 as *const ::core::ffi::c_char,
                25 as size_t,
            );
            if datetime_strToDate(&raw mut strDate as *mut ::core::ffi::c_char, &raw mut aDate) == 0
            {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, s2);
            }
            m = datetime_dayOfYear(aDate);
            if k == SWEEP_START as ::core::ffi::c_int {
                SweepStart = m;
            } else {
                SweepEnd = m;
            }
        }
        11 => {
            StartDryDays = atof(s2);
            if StartDryDays < 0.0f64 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
        }
        12 | 13 | 16 | 15 => {
            if datetime_strToTime(s2, &raw mut aTime) == 0 {
                return error_setInpError(ERR_DATETIME as ::core::ffi::c_int, s2);
            }
            datetime_decodeTime(aTime, &raw mut h, &raw mut m, &raw mut s);
            h += 24 as ::core::ffi::c_int * crate::ccast::f64_to_i32(aTime);
            s = s + 60 as ::core::ffi::c_int * m + 3600 as ::core::ffi::c_int * h;
            if k == RULE_STEP as ::core::ffi::c_int {
                if s < 0 as ::core::ffi::c_int {
                    return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
                }
            } else if s <= 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            match k {
                12 => {
                    WetStep = s;
                }
                13 => {
                    DryStep = s;
                }
                16 => {
                    ReportStep = s;
                }
                15 => {
                    RuleStep = s;
                }
                _ => {}
            }
        }
        18 => {
            m = findmatch(
                s2,
                &raw mut InertDampingWords as *mut *mut ::core::ffi::c_char,
            );
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            } else {
                InertDamping = m;
            }
        }
        17 | 19 | 25 | 27 | 31 | 32 | 33 | 34 | 39 => {
            m = findmatch(s2, &raw mut NoYesWords as *mut *mut ::core::ffi::c_char);
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
            match k {
                17 => {
                    AllowPonding = m;
                }
                19 => {
                    SlopeWeighting = m;
                }
                25 => {
                    SkipSteadyState = m;
                }
                27 => {
                    IgnoreRainfall = m;
                }
                31 => {
                    IgnoreSnowmelt = m;
                }
                32 => {
                    IgnoreGwater = m;
                }
                33 => {
                    IgnoreRouting = m;
                }
                34 => {
                    IgnoreQuality = m;
                }
                39 => {
                    IgnoreRDII = m;
                }
                _ => {}
            }
        }
        21 => {
            m = findmatch(
                s2,
                &raw mut NormalFlowWords as *mut *mut ::core::ffi::c_char,
            );
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
            NormalFlowLtd = m;
        }
        28 => {
            m = findmatch(
                s2,
                &raw mut ForceMainEqnWords as *mut *mut ::core::ffi::c_char,
            );
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
            ForceMainEqn = m;
        }
        29 => {
            m = findmatch(
                s2,
                &raw mut LinkOffsetWords as *mut *mut ::core::ffi::c_char,
            );
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
            LinkOffsets = m;
        }
        24 => {
            if strcomp(s2, b"3\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
                Compatibility = SWMM3 as ::core::ffi::c_int;
            } else if strcomp(s2, b"4\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
                Compatibility = SWMM4 as ::core::ffi::c_int;
            } else if strcomp(s2, b"5\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
                Compatibility = SWMM5 as ::core::ffi::c_int;
            } else {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
        }
        14 | 22 => {
            if getDouble(s2, &raw mut tStep) == 0 {
                if datetime_strToTime(s2, &raw mut aTime) == 0 {
                    return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
                } else {
                    datetime_decodeTime(aTime, &raw mut h, &raw mut m, &raw mut s);
                    h += 24 as ::core::ffi::c_int * crate::ccast::f64_to_i32(aTime);
                    s = s + 60 as ::core::ffi::c_int * m + 3600 as ::core::ffi::c_int * h;
                    tStep = s as ::core::ffi::c_double;
                }
            }
            if k == ROUTE_STEP as ::core::ffi::c_int {
                if tStep <= 0.0f64 {
                    return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
                }
                RouteStep = tStep;
            } else {
                LengtheningStep = if 0.0f64 >= tStep { 0.0f64 } else { tStep };
            }
        }
        40 => {
            if getDouble(s2, &raw mut MinRouteStep) == 0 || MinRouteStep < 0.0f64 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
        }
        41 => {
            m = atoi(s2);
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            NumThreads = m;
        }
        20 => {
            if getDouble(s2, &raw mut CourantFactor) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            if CourantFactor < 0.0f64 || CourantFactor > 2.0f64 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
        }
        23 => {
            if getDouble(s2, &raw mut MinSurfArea) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            if MinSurfArea < 0.0f64 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
        }
        30 => {
            if getDouble(s2, &raw mut MinSlope) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            if MinSlope < 0.0f64 || MinSlope >= 100 as ::core::ffi::c_int as ::core::ffi::c_double {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            MinSlope /= 100.0f64;
        }
        35 => {
            m = atoi(s2);
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            MaxTrials = m;
        }
        36 => {
            if getDouble(s2, &raw mut HeadTol) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
        }
        37 => {
            if getDouble(s2, &raw mut SysFlowTol) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            SysFlowTol /= 100.0f64;
        }
        38 => {
            if getDouble(s2, &raw mut LatFlowTol) == 0 {
                return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, s2);
            }
            LatFlowTol /= 100.0f64;
        }
        42 => {
            m = findmatch(s2, &raw mut SurchargeWords as *mut *mut ::core::ffi::c_char);
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_KEYWORD as ::core::ffi::c_int, s2);
            }
            SurchargeMethod = m;
        }
        26 => {
            sstrncpy(
                &raw mut TempDir as *mut ::core::ffi::c_char,
                s2,
                MAXFNAME as size_t,
            );
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn initPointers() {
    Gage = ::core::ptr::null_mut::<TGage>();
    Subcatch = ::core::ptr::null_mut::<TSubcatch>();
    Node = ::core::ptr::null_mut::<TNode>();
    Outfall = ::core::ptr::null_mut::<TOutfall>();
    Divider = ::core::ptr::null_mut::<TDivider>();
    Storage = ::core::ptr::null_mut::<TStorage>();
    Link = ::core::ptr::null_mut::<TLink>();
    Conduit = ::core::ptr::null_mut::<TConduit>();
    Pump = ::core::ptr::null_mut::<TPump>();
    Orifice = ::core::ptr::null_mut::<TOrifice>();
    Weir = ::core::ptr::null_mut::<TWeir>();
    Outlet = ::core::ptr::null_mut::<TOutlet>();
    Pollut = ::core::ptr::null_mut::<TPollut>();
    Landuse = ::core::ptr::null_mut::<TLanduse>();
    Pattern = ::core::ptr::null_mut::<TPattern>();
    Curve = ::core::ptr::null_mut::<TTable>();
    Tseries = ::core::ptr::null_mut::<TTable>();
    Transect = ::core::ptr::null_mut::<TTransect>();
    Shape = ::core::ptr::null_mut::<TShape>();
    Aquifer = ::core::ptr::null_mut::<TAquifer>();
    UnitHyd = ::core::ptr::null_mut::<TUnitHyd>();
    Snowmelt = ::core::ptr::null_mut::<TSnowmelt>();
    Event = ::core::ptr::null_mut::<TEvent>();
    MemPoolAllocated = FALSE as ::core::ffi::c_char;
}
unsafe extern "C" fn setDefaults() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MAXTITLE {
        sstrncpy(
            &raw mut *(&raw mut Title as *mut [::core::ffi::c_char; 1025]).offset(i as isize)
                as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as size_t,
        );
        i += 1;
    }
    sstrncpy(
        &raw mut TempDir as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        0 as size_t,
    );
    Frain.mode = SCRATCH_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    Fclimate.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    Frunoff.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    Frdii.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    Fhotstart1.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    Fhotstart2.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    Finflows.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    Foutflows.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    Frain.file = ::core::ptr::null_mut::<FILE>();
    Fclimate.file = ::core::ptr::null_mut::<FILE>();
    Frunoff.file = ::core::ptr::null_mut::<FILE>();
    Frdii.file = ::core::ptr::null_mut::<FILE>();
    Fhotstart1.file = ::core::ptr::null_mut::<FILE>();
    Fhotstart2.file = ::core::ptr::null_mut::<FILE>();
    Finflows.file = ::core::ptr::null_mut::<FILE>();
    Foutflows.file = ::core::ptr::null_mut::<FILE>();
    Fout.file = ::core::ptr::null_mut::<FILE>();
    Fout.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    UnitSystem = US as ::core::ffi::c_int;
    FlowUnits = CFS as ::core::ffi::c_int;
    InfilModel = HORTON as ::core::ffi::c_int;
    RouteModel = DW as ::core::ffi::c_int;
    SurchargeMethod = EXTRAN as ::core::ffi::c_int;
    CrownCutoff = 0.96f64;
    AllowPonding = FALSE;
    InertDamping = PARTIAL_DAMPING as ::core::ffi::c_int;
    NormalFlowLtd = BOTH as ::core::ffi::c_int;
    ForceMainEqn = H_W as ::core::ffi::c_int;
    LinkOffsets = DEPTH_OFFSET as ::core::ffi::c_int;
    LengtheningStep = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    CourantFactor = 0.75f64;
    MinSurfArea = 0.0f64;
    MinSlope = 0.0f64;
    SkipSteadyState = FALSE;
    IgnoreRainfall = FALSE;
    IgnoreRDII = FALSE;
    IgnoreSnowmelt = FALSE;
    IgnoreGwater = FALSE;
    IgnoreRouting = FALSE;
    IgnoreQuality = FALSE;
    WetStep = 300 as ::core::ffi::c_int;
    DryStep = 3600 as ::core::ffi::c_int;
    RuleStep = 0 as ::core::ffi::c_int;
    RouteStep = 20 as ::core::ffi::c_int as ::core::ffi::c_double;
    MinRouteStep = 0.5f64;
    ReportStep = 900 as ::core::ffi::c_int;
    StartDryDays = 0.0f64;
    MaxTrials = 0 as ::core::ffi::c_int;
    HeadTol = 0.0f64;
    SysFlowTol = 0.05f64;
    LatFlowTol = 0.05f64;
    NumThreads = 1 as ::core::ffi::c_int;
    NumEvents = 0 as ::core::ffi::c_int;
    SlopeWeighting = TRUE;
    Compatibility = SWMM4 as ::core::ffi::c_int;
    StartDate = datetime_encodeDate(
        2004 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    StartTime = datetime_encodeTime(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    StartDateTime = StartDate + StartTime;
    EndDate = StartDate;
    EndTime = 0.0f64 as DateTime;
    ReportStartDate = NO_DATE as DateTime;
    ReportStartTime = NO_DATE as DateTime;
    SweepStart = 1 as ::core::ffi::c_int;
    SweepEnd = 365 as ::core::ffi::c_int;
    RptFlags.disabled = FALSE as ::core::ffi::c_char;
    RptFlags.input = FALSE as ::core::ffi::c_char;
    RptFlags.continuity = TRUE as ::core::ffi::c_char;
    RptFlags.flowStats = TRUE as ::core::ffi::c_char;
    RptFlags.controls = FALSE as ::core::ffi::c_char;
    RptFlags.subcatchments = FALSE as ::core::ffi::c_char;
    RptFlags.nodes = FALSE as ::core::ffi::c_char;
    RptFlags.links = FALSE as ::core::ffi::c_char;
    RptFlags.averages = FALSE as ::core::ffi::c_char;
    Temp.dataSource = NO_TEMP as ::core::ffi::c_int;
    Temp.tSeries = -(1 as ::core::ffi::c_int);
    Temp.ta = 70.0f64;
    Temp.elev = 0.0f64;
    Temp.anglat = 40.0f64;
    Temp.dtlong = 0.0f64;
    Temp.tmax = MISSING;
    Wind.type_0 = MONTHLY_WIND as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        Wind.aws[i as usize] = 0.0f64;
        i += 1;
    }
    Snow.snotmp = 34.0f64;
    Snow.tipm = 0.5f64;
    Snow.rnm = 0.6f64;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < 10 as ::core::ffi::c_int {
            Snow.adc[i as usize][j as usize] = 1.0f64;
            j += 1;
        }
        i += 1;
    }
    Evap.type_0 = CONSTANT_EVAP as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        Evap.monthlyEvap[i as usize] = 0.0f64;
        Evap.panCoeff[i as usize] = 1.0f64;
        i += 1;
    }
    Evap.recoveryPattern = -(1 as ::core::ffi::c_int);
    Evap.recoveryFactor = 1.0f64;
    Evap.tSeries = -(1 as ::core::ffi::c_int);
    Evap.dryOnly = FALSE;
    i = 0 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        Adjust.temp[i as usize] = 0.0f64;
        Adjust.evap[i as usize] = 0.0f64;
        Adjust.rain[i as usize] = 1.0f64;
        Adjust.hydcon[i as usize] = 1.0f64;
        i += 1;
    }
    Adjust.rainFactor = 1.0f64;
    Adjust.hydconFactor = 1.0f64;
}
unsafe extern "C" fn openFiles(
    mut f1: *const ::core::ffi::c_char,
    mut f2: *const ::core::ffi::c_char,
    mut f3: *const ::core::ffi::c_char,
) {
    Finp.file = ::core::ptr::null_mut::<FILE>();
    Frpt.file = ::core::ptr::null_mut::<FILE>();
    Fout.file = ::core::ptr::null_mut::<FILE>();
    sstrncpy(
        &raw mut Finp.name as *mut ::core::ffi::c_char,
        f1,
        MAXFNAME as size_t,
    );
    sstrncpy(
        &raw mut Frpt.name as *mut ::core::ffi::c_char,
        f2,
        MAXFNAME as size_t,
    );
    sstrncpy(
        &raw mut Fout.name as *mut ::core::ffi::c_char,
        f3,
        MAXFNAME as size_t,
    );
    if strcomp(f1, f2) != 0 || strcomp(f1, f3) != 0 || strcomp(f2, f3) != 0 {
        writecon(FMT11.as_ptr());
        ErrorCode = ERR_FILE_NAME as ::core::ffi::c_int;
        return;
    }
    Finp.file = fopen(f1, b"rt\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if Finp.file.is_null() {
        writecon(FMT12.as_ptr());
        writecon(f1);
        ErrorCode = ERR_INP_FILE as ::core::ffi::c_int;
        return;
    }
    Frpt.file = fopen(f2, b"wt\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if Frpt.file.is_null() {
        writecon(FMT13.as_ptr());
        ErrorCode = ERR_RPT_FILE as ::core::ffi::c_int;
        return;
    }
}
unsafe extern "C" fn createObjects() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    if ErrorCode != 0 {
        return;
    }
    Gage = calloc(
        Nobjects[GAGE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TGage>() as size_t,
    ) as *mut TGage;
    Subcatch = calloc(
        Nobjects[SUBCATCH as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TSubcatch>() as size_t,
    ) as *mut TSubcatch;
    Node = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TNode>() as size_t,
    ) as *mut TNode;
    Outfall = calloc(
        Nnodes[OUTFALL as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TOutfall>() as size_t,
    ) as *mut TOutfall;
    Divider = calloc(
        Nnodes[DIVIDER as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TDivider>() as size_t,
    ) as *mut TDivider;
    Storage = calloc(
        Nnodes[STORAGE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TStorage>() as size_t,
    ) as *mut TStorage;
    Link = calloc(
        Nobjects[LINK as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TLink>() as size_t,
    ) as *mut TLink;
    Conduit = calloc(
        Nlinks[CONDUIT as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TConduit>() as size_t,
    ) as *mut TConduit;
    Pump = calloc(
        Nlinks[PUMP as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TPump>() as size_t,
    ) as *mut TPump;
    Orifice = calloc(
        Nlinks[ORIFICE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TOrifice>() as size_t,
    ) as *mut TOrifice;
    Weir = calloc(
        Nlinks[WEIR as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TWeir>() as size_t,
    ) as *mut TWeir;
    Outlet = calloc(
        Nlinks[OUTLET as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TOutlet>() as size_t,
    ) as *mut TOutlet;
    Pollut = calloc(
        Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TPollut>() as size_t,
    ) as *mut TPollut;
    Landuse = calloc(
        Nobjects[LANDUSE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TLanduse>() as size_t,
    ) as *mut TLanduse;
    Pattern = calloc(
        Nobjects[TIMEPATTERN as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TPattern>() as size_t,
    ) as *mut TPattern;
    Curve = calloc(
        Nobjects[CURVE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TTable>() as size_t,
    ) as *mut TTable;
    Tseries = calloc(
        Nobjects[TSERIES as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TTable>() as size_t,
    ) as *mut TTable;
    Aquifer = calloc(
        Nobjects[AQUIFER as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TAquifer>() as size_t,
    ) as *mut TAquifer;
    UnitHyd = calloc(
        Nobjects[UNITHYD as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TUnitHyd>() as size_t,
    ) as *mut TUnitHyd;
    Snowmelt = calloc(
        Nobjects[SNOWMELT as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TSnowmelt>() as size_t,
    ) as *mut TSnowmelt;
    Shape = calloc(
        Nobjects[SHAPE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TShape>() as size_t,
    ) as *mut TShape;
    Event = calloc(
        (NumEvents as size_t).wrapping_add(1 as size_t),
        ::core::mem::size_of::<TEvent>() as size_t,
    ) as *mut TEvent;
    (*Event.offset(NumEvents as isize)).start = BIG as DateTime;
    (*Event.offset(NumEvents as isize)).end = (BIG + 1.0f64) as DateTime;
    lid_create(
        Nobjects[LID as ::core::ffi::c_int as usize],
        Nobjects[SUBCATCH as ::core::ffi::c_int as usize],
    );
    ErrorCode = controls_create(Nobjects[CONTROL as ::core::ffi::c_int as usize]);
    if ErrorCode != 0 {
        return;
    }
    ErrorCode = transect_create(Nobjects[TRANSECT as ::core::ffi::c_int as usize]);
    if ErrorCode != 0 {
        return;
    }
    ErrorCode = street_create(Nobjects[STREET as ::core::ffi::c_int as usize]);
    if ErrorCode != 0 {
        return;
    }
    ErrorCode = inlet_create(Nobjects[INLET as ::core::ffi::c_int as usize]);
    if ErrorCode != 0 {
        return;
    }
    infil_create(Nobjects[SUBCATCH as ::core::ffi::c_int as usize]);
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        let ref mut fresh20 = (*Subcatch.offset(j as isize)).initBuildup;
        *fresh20 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        let ref mut fresh21 = (*Subcatch.offset(j as isize)).oldQual;
        *fresh21 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        let ref mut fresh22 = (*Subcatch.offset(j as isize)).newQual;
        *fresh22 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        let ref mut fresh23 = (*Subcatch.offset(j as isize)).pondedQual;
        *fresh23 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        let ref mut fresh24 = (*Subcatch.offset(j as isize)).totalLoad;
        *fresh24 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        let ref mut fresh25 = (*Node.offset(j as isize)).oldQual;
        *fresh25 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        let ref mut fresh26 = (*Node.offset(j as isize)).newQual;
        *fresh26 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        let ref mut fresh27 = (*Node.offset(j as isize)).extInflow;
        *fresh27 = ::core::ptr::null_mut::<TExtInflow>();
        let ref mut fresh28 = (*Node.offset(j as isize)).dwfInflow;
        *fresh28 = ::core::ptr::null_mut::<TDwfInflow>();
        let ref mut fresh29 = (*Node.offset(j as isize)).rdiiInflow;
        *fresh29 = ::core::ptr::null_mut::<TRdiiInflow>();
        let ref mut fresh30 = (*Node.offset(j as isize)).treatment;
        *fresh30 = ::core::ptr::null_mut::<TTreatment>();
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        let ref mut fresh31 = (*Link.offset(j as isize)).inlet;
        *fresh31 = ::core::ptr::null_mut::<TInlet>();
        let ref mut fresh32 = (*Link.offset(j as isize)).oldQual;
        *fresh32 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        let ref mut fresh33 = (*Link.offset(j as isize)).newQual;
        *fresh33 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        let ref mut fresh34 = (*Link.offset(j as isize)).totalLoad;
        *fresh34 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
        let ref mut fresh35 = (*Landuse.offset(j as isize)).buildupFunc;
        *fresh35 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TBuildup>() as size_t,
        ) as *mut TBuildup;
        let ref mut fresh36 = (*Landuse.offset(j as isize)).washoffFunc;
        *fresh36 = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TWashoff>() as size_t,
        ) as *mut TWashoff;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        let ref mut fresh37 = (*Subcatch.offset(j as isize)).landFactor;
        *fresh37 = calloc(
            Nobjects[LANDUSE as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<TLandFactor>() as size_t,
        ) as *mut TLandFactor;
        k = 0 as ::core::ffi::c_int;
        while k < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
            let ref mut fresh38 =
                (*(*Subcatch.offset(j as isize)).landFactor.offset(k as isize)).buildup;
            *fresh38 = calloc(
                Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
            ) as *mut ::core::ffi::c_double;
            k += 1;
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
        k = 0 as ::core::ffi::c_int;
        while k < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            (*(*Landuse.offset(j as isize)).buildupFunc.offset(k as isize)).funcType =
                NO_BUILDUP as ::core::ffi::c_int;
            (*(*Landuse.offset(j as isize)).buildupFunc.offset(k as isize)).normalizer =
                PER_AREA as ::core::ffi::c_int;
            (*(*Landuse.offset(j as isize)).washoffFunc.offset(k as isize)).funcType =
                NO_WASHOFF as ::core::ffi::c_int;
            k += 1;
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        (*Gage.offset(j as isize)).tSeries = -(1 as ::core::ffi::c_int);
        sstrncpy(
            &raw mut (*Gage.offset(j as isize)).fname as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as size_t,
        );
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        (*Subcatch.offset(j as isize)).outSubcatch = -(1 as ::core::ffi::c_int);
        (*Subcatch.offset(j as isize)).outNode = -(1 as ::core::ffi::c_int);
        (*Subcatch.offset(j as isize)).infil = -(1 as ::core::ffi::c_int);
        let ref mut fresh39 = (*Subcatch.offset(j as isize)).groundwater;
        *fresh39 = ::core::ptr::null_mut::<TGroundwater>();
        let ref mut fresh40 = (*Subcatch.offset(j as isize)).gwLatFlowExpr;
        *fresh40 = ::core::ptr::null_mut::<MathExpr>();
        let ref mut fresh41 = (*Subcatch.offset(j as isize)).gwDeepFlowExpr;
        *fresh41 = ::core::ptr::null_mut::<MathExpr>();
        let ref mut fresh42 = (*Subcatch.offset(j as isize)).snowpack;
        *fresh42 = ::core::ptr::null_mut::<TSnowpack>();
        (*Subcatch.offset(j as isize)).lidArea = 0.0f64;
        k = 0 as ::core::ffi::c_int;
        while k < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *(*Subcatch.offset(j as isize))
                .initBuildup
                .offset(k as isize) = 0.0f64;
            k += 1;
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[UNITHYD as ::core::ffi::c_int as usize] {
        rdii_initUnitHyd(j);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SNOWMELT as ::core::ffi::c_int as usize] {
        snow_initSnowmelt(j);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nnodes[STORAGE as ::core::ffi::c_int as usize] {
        let ref mut fresh43 = (*Storage.offset(j as isize)).exfil;
        *fresh43 = ::core::ptr::null_mut::<TExfil>();
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        (*Link.offset(j as isize)).xsect.type_0 = -(1 as ::core::ffi::c_int);
        (*Link.offset(j as isize)).cLossInlet = 0.0f64;
        (*Link.offset(j as isize)).cLossOutlet = 0.0f64;
        (*Link.offset(j as isize)).cLossAvg = 0.0f64;
        (*Link.offset(j as isize)).hasFlapGate = FALSE;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nlinks[PUMP as ::core::ffi::c_int as usize] {
        (*Pump.offset(j as isize)).pumpCurve = -(1 as ::core::ffi::c_int);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
        (*Subcatch.offset(j as isize)).rptFlag = FALSE;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(j as isize)).rptFlag = FALSE;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        (*Link.offset(j as isize)).rptFlag = FALSE;
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[CURVE as ::core::ffi::c_int as usize] {
        table_init(Curve.offset(j as isize) as *mut TTable);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[TSERIES as ::core::ffi::c_int as usize] {
        table_init(Tseries.offset(j as isize) as *mut TTable);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[TIMEPATTERN as ::core::ffi::c_int as usize] {
        inflow_initDwfPattern(j);
        j += 1;
    }
}
unsafe extern "C" fn deleteObjects() {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    if !Subcatch.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
            k = 0 as ::core::ffi::c_int;
            while k < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
                if !(*(*Subcatch.offset(j as isize)).landFactor.offset(k as isize))
                    .buildup
                    .is_null()
                {
                    free(
                        (*(*Subcatch.offset(j as isize)).landFactor.offset(k as isize)).buildup
                            as *mut ::core::ffi::c_void,
                    );
                    let ref mut fresh0 =
                        (*(*Subcatch.offset(j as isize)).landFactor.offset(k as isize)).buildup;
                    *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_double>();
                }
                k += 1;
            }
            if !(*Subcatch.offset(j as isize)).landFactor.is_null() {
                free((*Subcatch.offset(j as isize)).landFactor as *mut ::core::ffi::c_void);
                let ref mut fresh1 = (*Subcatch.offset(j as isize)).landFactor;
                *fresh1 = ::core::ptr::null_mut::<TLandFactor>();
            }
            if !(*Subcatch.offset(j as isize)).groundwater.is_null() {
                free((*Subcatch.offset(j as isize)).groundwater as *mut ::core::ffi::c_void);
                let ref mut fresh2 = (*Subcatch.offset(j as isize)).groundwater;
                *fresh2 = ::core::ptr::null_mut::<TGroundwater>();
            }
            gwater_deleteFlowExpression(j);
            if !(*Subcatch.offset(j as isize)).snowpack.is_null() {
                free((*Subcatch.offset(j as isize)).snowpack as *mut ::core::ffi::c_void);
                let ref mut fresh3 = (*Subcatch.offset(j as isize)).snowpack;
                *fresh3 = ::core::ptr::null_mut::<TSnowpack>();
            }
            j += 1;
        }
    }
    if !Landuse.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
            if !(*Landuse.offset(j as isize)).buildupFunc.is_null() {
                free((*Landuse.offset(j as isize)).buildupFunc as *mut ::core::ffi::c_void);
                let ref mut fresh4 = (*Landuse.offset(j as isize)).buildupFunc;
                *fresh4 = ::core::ptr::null_mut::<TBuildup>();
            }
            if !(*Landuse.offset(j as isize)).washoffFunc.is_null() {
                free((*Landuse.offset(j as isize)).washoffFunc as *mut ::core::ffi::c_void);
                let ref mut fresh5 = (*Landuse.offset(j as isize)).washoffFunc;
                *fresh5 = ::core::ptr::null_mut::<TWashoff>();
            }
            j += 1;
        }
    }
    if !Subcatch.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[SUBCATCH as ::core::ffi::c_int as usize] {
            if !(*Subcatch.offset(j as isize)).initBuildup.is_null() {
                free((*Subcatch.offset(j as isize)).initBuildup as *mut ::core::ffi::c_void);
                let ref mut fresh6 = (*Subcatch.offset(j as isize)).initBuildup;
                *fresh6 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(*Subcatch.offset(j as isize)).oldQual.is_null() {
                free((*Subcatch.offset(j as isize)).oldQual as *mut ::core::ffi::c_void);
                let ref mut fresh7 = (*Subcatch.offset(j as isize)).oldQual;
                *fresh7 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(*Subcatch.offset(j as isize)).newQual.is_null() {
                free((*Subcatch.offset(j as isize)).newQual as *mut ::core::ffi::c_void);
                let ref mut fresh8 = (*Subcatch.offset(j as isize)).newQual;
                *fresh8 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(*Subcatch.offset(j as isize)).pondedQual.is_null() {
                free((*Subcatch.offset(j as isize)).pondedQual as *mut ::core::ffi::c_void);
                let ref mut fresh9 = (*Subcatch.offset(j as isize)).pondedQual;
                *fresh9 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(*Subcatch.offset(j as isize)).totalLoad.is_null() {
                free((*Subcatch.offset(j as isize)).totalLoad as *mut ::core::ffi::c_void);
                let ref mut fresh10 = (*Subcatch.offset(j as isize)).totalLoad;
                *fresh10 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            j += 1;
        }
    }
    if !Node.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            if !(*Node.offset(j as isize)).oldQual.is_null() {
                free((*Node.offset(j as isize)).oldQual as *mut ::core::ffi::c_void);
                let ref mut fresh11 = (*Node.offset(j as isize)).oldQual;
                *fresh11 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(*Node.offset(j as isize)).newQual.is_null() {
                free((*Node.offset(j as isize)).newQual as *mut ::core::ffi::c_void);
                let ref mut fresh12 = (*Node.offset(j as isize)).newQual;
                *fresh12 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            j += 1;
        }
    }
    if !Link.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
            if !(*Link.offset(j as isize)).oldQual.is_null() {
                free((*Link.offset(j as isize)).oldQual as *mut ::core::ffi::c_void);
                let ref mut fresh13 = (*Link.offset(j as isize)).oldQual;
                *fresh13 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(*Link.offset(j as isize)).newQual.is_null() {
                free((*Link.offset(j as isize)).newQual as *mut ::core::ffi::c_void);
                let ref mut fresh14 = (*Link.offset(j as isize)).newQual;
                *fresh14 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            if !(*Link.offset(j as isize)).totalLoad.is_null() {
                free((*Link.offset(j as isize)).totalLoad as *mut ::core::ffi::c_void);
                let ref mut fresh15 = (*Link.offset(j as isize)).totalLoad;
                *fresh15 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            j += 1;
        }
    }
    infil_delete();
    if !Node.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nnodes[STORAGE as ::core::ffi::c_int as usize] {
            if !(*Storage.offset(j as isize)).exfil.is_null() {
                if !(*(*Storage.offset(j as isize)).exfil).btmExfil.is_null() {
                    free(
                        (*(*Storage.offset(j as isize)).exfil).btmExfil as *mut ::core::ffi::c_void,
                    );
                    let ref mut fresh16 = (*(*Storage.offset(j as isize)).exfil).btmExfil;
                    *fresh16 = ::core::ptr::null_mut::<TGrnAmpt>();
                }
                if !(*(*Storage.offset(j as isize)).exfil).bankExfil.is_null() {
                    free(
                        (*(*Storage.offset(j as isize)).exfil).bankExfil
                            as *mut ::core::ffi::c_void,
                    );
                    let ref mut fresh17 = (*(*Storage.offset(j as isize)).exfil).bankExfil;
                    *fresh17 = ::core::ptr::null_mut::<TGrnAmpt>();
                }
                if !(*Storage.offset(j as isize)).exfil.is_null() {
                    free((*Storage.offset(j as isize)).exfil as *mut ::core::ffi::c_void);
                    let ref mut fresh18 = (*Storage.offset(j as isize)).exfil;
                    *fresh18 = ::core::ptr::null_mut::<TExfil>();
                }
            }
            j += 1;
        }
    }
    if !Node.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nnodes[OUTFALL as ::core::ffi::c_int as usize] {
            if !(*Outfall.offset(j as isize)).wRouted.is_null() {
                free((*Outfall.offset(j as isize)).wRouted as *mut ::core::ffi::c_void);
                let ref mut fresh19 = (*Outfall.offset(j as isize)).wRouted;
                *fresh19 = ::core::ptr::null_mut::<::core::ffi::c_double>();
            }
            j += 1;
        }
    }
    if !Node.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
            inflow_deleteExtInflows(j);
            inflow_deleteDwfInflows(j);
            rdii_deleteRdiiInflow(j);
            treatmnt_delete(j);
            j += 1;
        }
    }
    if !Tseries.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[TSERIES as ::core::ffi::c_int as usize] {
            table_deleteEntries(Tseries.offset(j as isize) as *mut TTable);
            j += 1;
        }
    }
    if !Curve.is_null() {
        j = 0 as ::core::ffi::c_int;
        while j < Nobjects[CURVE as ::core::ffi::c_int as usize] {
            table_deleteEntries(Curve.offset(j as isize) as *mut TTable);
            j += 1;
        }
    }
    transect_delete();
    street_delete();
    inlet_delete();
    controls_delete();
    lid_delete();
    if !Gage.is_null() {
        free(Gage as *mut ::core::ffi::c_void);
        Gage = ::core::ptr::null_mut::<TGage>();
    }
    if !Subcatch.is_null() {
        free(Subcatch as *mut ::core::ffi::c_void);
        Subcatch = ::core::ptr::null_mut::<TSubcatch>();
    }
    if !Node.is_null() {
        free(Node as *mut ::core::ffi::c_void);
        Node = ::core::ptr::null_mut::<TNode>();
    }
    if !Outfall.is_null() {
        free(Outfall as *mut ::core::ffi::c_void);
        Outfall = ::core::ptr::null_mut::<TOutfall>();
    }
    if !Divider.is_null() {
        free(Divider as *mut ::core::ffi::c_void);
        Divider = ::core::ptr::null_mut::<TDivider>();
    }
    if !Storage.is_null() {
        free(Storage as *mut ::core::ffi::c_void);
        Storage = ::core::ptr::null_mut::<TStorage>();
    }
    if !Link.is_null() {
        free(Link as *mut ::core::ffi::c_void);
        Link = ::core::ptr::null_mut::<TLink>();
    }
    if !Conduit.is_null() {
        free(Conduit as *mut ::core::ffi::c_void);
        Conduit = ::core::ptr::null_mut::<TConduit>();
    }
    if !Pump.is_null() {
        free(Pump as *mut ::core::ffi::c_void);
        Pump = ::core::ptr::null_mut::<TPump>();
    }
    if !Orifice.is_null() {
        free(Orifice as *mut ::core::ffi::c_void);
        Orifice = ::core::ptr::null_mut::<TOrifice>();
    }
    if !Weir.is_null() {
        free(Weir as *mut ::core::ffi::c_void);
        Weir = ::core::ptr::null_mut::<TWeir>();
    }
    if !Outlet.is_null() {
        free(Outlet as *mut ::core::ffi::c_void);
        Outlet = ::core::ptr::null_mut::<TOutlet>();
    }
    if !Pollut.is_null() {
        free(Pollut as *mut ::core::ffi::c_void);
        Pollut = ::core::ptr::null_mut::<TPollut>();
    }
    if !Landuse.is_null() {
        free(Landuse as *mut ::core::ffi::c_void);
        Landuse = ::core::ptr::null_mut::<TLanduse>();
    }
    if !Pattern.is_null() {
        free(Pattern as *mut ::core::ffi::c_void);
        Pattern = ::core::ptr::null_mut::<TPattern>();
    }
    if !Curve.is_null() {
        free(Curve as *mut ::core::ffi::c_void);
        Curve = ::core::ptr::null_mut::<TTable>();
    }
    if !Tseries.is_null() {
        free(Tseries as *mut ::core::ffi::c_void);
        Tseries = ::core::ptr::null_mut::<TTable>();
    }
    if !Aquifer.is_null() {
        free(Aquifer as *mut ::core::ffi::c_void);
        Aquifer = ::core::ptr::null_mut::<TAquifer>();
    }
    if !UnitHyd.is_null() {
        free(UnitHyd as *mut ::core::ffi::c_void);
        UnitHyd = ::core::ptr::null_mut::<TUnitHyd>();
    }
    if !Snowmelt.is_null() {
        free(Snowmelt as *mut ::core::ffi::c_void);
        Snowmelt = ::core::ptr::null_mut::<TSnowmelt>();
    }
    if !Shape.is_null() {
        free(Shape as *mut ::core::ffi::c_void);
        Shape = ::core::ptr::null_mut::<TShape>();
    }
    if !Event.is_null() {
        free(Event as *mut ::core::ffi::c_void);
        Event = ::core::ptr::null_mut::<TEvent>();
    }
}
unsafe extern "C" fn createHashTables() {
    let mut j: ::core::ffi::c_int = 0;
    MemPoolAllocated = FALSE as ::core::ffi::c_char;
    j = 0 as ::core::ffi::c_int;
    while j < MAX_OBJ_TYPES as ::core::ffi::c_int {
        Htable[j as usize] = HTcreate();
        if Htable[j as usize].is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        j += 1;
    }
    if AllocInit().is_null() {
        report_writeErrorMsg(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        MemPoolAllocated = TRUE as ::core::ffi::c_char;
    };
}
unsafe extern "C" fn deleteHashTables() {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < MAX_OBJ_TYPES as ::core::ffi::c_int {
        if !Htable[j as usize].is_null() {
            HTfree(Htable[j as usize]);
        }
        j += 1;
    }
    if MemPoolAllocated != 0 {
        AllocFreePool();
    }
}
pub const NO_DATE: ::core::ffi::c_int = -(693594 as ::core::ffi::c_int);
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
