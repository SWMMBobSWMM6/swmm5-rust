#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut StartDryDays: ::core::ffi::c_double;
    static mut StartDateTime: DateTime;
    static mut NewRunoffTime: ::core::ffi::c_double;
    static mut Subcatch: *mut TSubcatch;
    static mut Pollut: *mut TPollut;
    static mut Landuse: *mut TLanduse;
    static mut Tseries: *mut TTable;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn massbal_updateLoadingTotals(
        type_0: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
        w: ::core::ffi::c_double,
    );
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
    fn strcomp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut BuildupTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut NormalizerWords: [*mut ::core::ffi::c_char; 0];
    static mut NoYesWords: [*mut ::core::ffi::c_char; 0];
    static mut QualUnitsWords: [*mut ::core::ffi::c_char; 0];
    static mut WashoffTypeWords: [*mut ::core::ffi::c_char; 0];
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
pub type LoadingType = ::core::ffi::c_uint;
pub const FINAL_LOAD: LoadingType = 6;
pub const RUNOFF_LOAD: LoadingType = 5;
pub const INFIL_LOAD: LoadingType = 4;
pub const BMP_REMOVAL_LOAD: LoadingType = 3;
pub const SWEEPING_LOAD: LoadingType = 2;
pub const DEPOSITION_LOAD: LoadingType = 1;
pub const BUILDUP_LOAD: LoadingType = 0;
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
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_KEYWORD: ErrorType = 205;
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NONE: ErrorType = 0;
#[no_mangle]
pub unsafe extern "C" fn landuse_readParams(
    mut j: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 1 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        LANDUSE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    let ref mut fresh0 = (*Landuse.offset(j as isize)).ID;
    *fresh0 = id;
    if ntoks > 1 as ::core::ffi::c_int {
        if ntoks < 4 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if getDouble(
            *tok.offset(1 as ::core::ffi::c_int as isize),
            &raw mut (*Landuse.offset(j as isize)).sweepInterval,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        if getDouble(
            *tok.offset(2 as ::core::ffi::c_int as isize),
            &raw mut (*Landuse.offset(j as isize)).sweepRemoval,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        if getDouble(
            *tok.offset(3 as ::core::ffi::c_int as isize),
            &raw mut (*Landuse.offset(j as isize)).sweepDays0,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(3 as ::core::ffi::c_int as isize),
            );
        }
    } else {
        (*Landuse.offset(j as isize)).sweepInterval = 0.0f64;
        (*Landuse.offset(j as isize)).sweepRemoval = 0.0f64;
        (*Landuse.offset(j as isize)).sweepDays0 = 0.0f64;
    }
    if (*Landuse.offset(j as isize)).sweepRemoval < 0.0f64
        || (*Landuse.offset(j as isize)).sweepRemoval > 1.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn landuse_readPollutParams(
    mut j: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut coPollut: ::core::ffi::c_int = 0;
    let mut snowFlag: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 4] = [0.; 4];
    let mut coFrac: ::core::ffi::c_double = 0.;
    let mut cDWF: ::core::ffi::c_double = 0.;
    let mut cInit: ::core::ffi::c_double = 0.;
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        POLLUT as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    k = findmatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut QualUnitsWords as *mut *mut ::core::ffi::c_char,
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    i = 2 as ::core::ffi::c_int;
    while i <= 4 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(i as isize),
            (&raw mut x as *mut ::core::ffi::c_double)
                .offset((i - 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
            || x[(i - 2 as ::core::ffi::c_int) as usize] < 0.0f64
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(i as isize));
        }
        i += 1;
    }
    if getDouble(
        *tok.offset(5 as ::core::ffi::c_int as isize),
        (&raw mut x as *mut ::core::ffi::c_double).offset(3 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(5 as ::core::ffi::c_int as isize),
        );
    }
    snowFlag = 0 as ::core::ffi::c_int;
    coPollut = -(1 as ::core::ffi::c_int);
    coFrac = 0.0f64;
    cDWF = 0.0f64;
    cInit = 0.0f64;
    if ntoks >= 7 as ::core::ffi::c_int {
        snowFlag = findmatch(
            *tok.offset(6 as ::core::ffi::c_int as isize),
            &raw mut NoYesWords as *mut *mut ::core::ffi::c_char,
        );
        if snowFlag < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks >= 9 as ::core::ffi::c_int {
        if strcomp(
            *tok.offset(7 as ::core::ffi::c_int as isize),
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            coPollut = project_findObject(
                POLLUT as ::core::ffi::c_int,
                *tok.offset(7 as ::core::ffi::c_int as isize),
            );
            if coPollut < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(7 as ::core::ffi::c_int as isize),
                );
            }
            if getDouble(
                *tok.offset(8 as ::core::ffi::c_int as isize),
                &raw mut coFrac,
            ) == 0
                || coFrac < 0.0f64
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(8 as ::core::ffi::c_int as isize),
                );
            }
        }
    }
    if ntoks >= 10 as ::core::ffi::c_int {
        if getDouble(*tok.offset(9 as ::core::ffi::c_int as isize), &raw mut cDWF) == 0
            || cDWF < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(9 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks >= 11 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(10 as ::core::ffi::c_int as isize),
            &raw mut cInit,
        ) == 0
            || cInit < 0.0f64
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(9 as ::core::ffi::c_int as isize),
            );
        }
    }
    let ref mut fresh1 = (*Pollut.offset(j as isize)).ID;
    *fresh1 = id;
    (*Pollut.offset(j as isize)).units = k;
    if (*Pollut.offset(j as isize)).units == MG as ::core::ffi::c_int {
        (*Pollut.offset(j as isize)).mcf = UCF(MASS as ::core::ffi::c_int);
    } else if (*Pollut.offset(j as isize)).units == UG as ::core::ffi::c_int {
        (*Pollut.offset(j as isize)).mcf = UCF(MASS as ::core::ffi::c_int) / 1000.0f64;
    } else {
        (*Pollut.offset(j as isize)).mcf = 1.0f64;
    }
    (*Pollut.offset(j as isize)).pptConcen = x[0 as ::core::ffi::c_int as usize];
    (*Pollut.offset(j as isize)).gwConcen = x[1 as ::core::ffi::c_int as usize];
    (*Pollut.offset(j as isize)).rdiiConcen = x[2 as ::core::ffi::c_int as usize];
    (*Pollut.offset(j as isize)).kDecay =
        x[3 as ::core::ffi::c_int as usize] / SECperDAY as ::core::ffi::c_double;
    (*Pollut.offset(j as isize)).snowOnly = snowFlag;
    (*Pollut.offset(j as isize)).coPollut = coPollut;
    (*Pollut.offset(j as isize)).coFraction = coFrac;
    (*Pollut.offset(j as isize)).dwfConcen = cDWF;
    (*Pollut.offset(j as isize)).initConcen = cInit;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn landuse_readBuildupParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut c: [::core::ffi::c_double; 3] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ];
    let mut tmax: ::core::ffi::c_double = 0.;
    if ntoks < 3 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    j = project_findObject(
        LANDUSE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    p = project_findObject(
        POLLUT as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if p < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    k = findmatch(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        &raw mut BuildupTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    (*(*Landuse.offset(j as isize)).buildupFunc.offset(p as isize)).funcType = k;
    if k > NO_BUILDUP as ::core::ffi::c_int {
        if ntoks < 7 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if k != EXTERNAL_BUILDUP as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < 3 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset((i + 3 as ::core::ffi::c_int) as isize),
                    (&raw mut c as *mut ::core::ffi::c_double).offset(i as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                    || c[i as usize] < 0.0f64
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset((i + 3 as ::core::ffi::c_int) as isize),
                    );
                }
                i += 1;
            }
        }
        n = findmatch(
            *tok.offset(6 as ::core::ffi::c_int as isize),
            &raw mut NormalizerWords as *mut *mut ::core::ffi::c_char,
        );
        if n < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
        }
        (*(*Landuse.offset(j as isize)).buildupFunc.offset(p as isize)).normalizer = n;
    }
    match (*(*Landuse.offset(j as isize)).buildupFunc.offset(p as isize)).funcType {
        1 => {
            if c[2 as ::core::ffi::c_int as usize] > 0.0f64
                && (c[2 as ::core::ffi::c_int as usize] < 0.01f64
                    || c[2 as ::core::ffi::c_int as usize] > 10.0f64)
            {
                return error_setInpError(
                    ERR_KEYWORD as ::core::ffi::c_int,
                    *tok.offset(5 as ::core::ffi::c_int as isize),
                );
            }
            if c[1 as ::core::ffi::c_int as usize] * c[2 as ::core::ffi::c_int as usize] == 0.0f64 {
                tmax = 0.0f64;
            } else if log10(c[0 as ::core::ffi::c_int as usize])
                / c[2 as ::core::ffi::c_int as usize]
                > 3.5f64
            {
                tmax = 3650.0f64;
            } else {
                tmax = pow(
                    c[0 as ::core::ffi::c_int as usize] / c[1 as ::core::ffi::c_int as usize],
                    1.0f64 / c[2 as ::core::ffi::c_int as usize],
                );
            }
        }
        2 => {
            if c[1 as ::core::ffi::c_int as usize] == 0.0f64 {
                tmax = 0.0f64;
            } else {
                tmax = -log(0.001f64) / c[1 as ::core::ffi::c_int as usize];
            }
        }
        3 => {
            tmax = 1000.0f64 * c[2 as ::core::ffi::c_int as usize];
        }
        4 => {
            if getDouble(
                *tok.offset(3 as ::core::ffi::c_int as isize),
                (&raw mut c as *mut ::core::ffi::c_double).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
                || c[0 as ::core::ffi::c_int as usize] < 0.0f64
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(3 as ::core::ffi::c_int as isize),
                );
            }
            if getDouble(
                *tok.offset(4 as ::core::ffi::c_int as isize),
                (&raw mut c as *mut ::core::ffi::c_double).offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
                || c[1 as ::core::ffi::c_int as usize] < 0.0f64
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(3 as ::core::ffi::c_int as isize),
                );
            }
            n = project_findObject(
                TSERIES as ::core::ffi::c_int,
                *tok.offset(5 as ::core::ffi::c_int as isize),
            );
            if n < 0 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(4 as ::core::ffi::c_int as isize),
                );
            }
            (*Tseries.offset(n as isize)).refersTo = EXTERNAL_BUILDUP as ::core::ffi::c_int;
            c[2 as ::core::ffi::c_int as usize] = n as ::core::ffi::c_double;
            tmax = 0.0f64;
        }
        _ => {
            tmax = 0.0f64;
        }
    }
    (*(*Landuse.offset(j as isize)).buildupFunc.offset(p as isize)).coeff
        [0 as ::core::ffi::c_int as usize] = c[0 as ::core::ffi::c_int as usize];
    (*(*Landuse.offset(j as isize)).buildupFunc.offset(p as isize)).coeff
        [1 as ::core::ffi::c_int as usize] = c[1 as ::core::ffi::c_int as usize];
    (*(*Landuse.offset(j as isize)).buildupFunc.offset(p as isize)).coeff
        [2 as ::core::ffi::c_int as usize] = c[2 as ::core::ffi::c_int as usize];
    (*(*Landuse.offset(j as isize)).buildupFunc.offset(p as isize)).maxDays = tmax;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn landuse_readWashoffParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut func: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 4] = [0.; 4];
    if ntoks < 3 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        x[i as usize] = 0.0f64;
        i += 1;
    }
    func = NO_WASHOFF as ::core::ffi::c_int;
    j = project_findObject(
        LANDUSE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    p = project_findObject(
        POLLUT as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if p < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    if ntoks > 2 as ::core::ffi::c_int {
        func = findmatch(
            *tok.offset(2 as ::core::ffi::c_int as isize),
            &raw mut WashoffTypeWords as *mut *mut ::core::ffi::c_char,
        );
        if func < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        if func != NO_WASHOFF as ::core::ffi::c_int {
            if ntoks < 5 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
            if ntoks >= 6 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset(5 as ::core::ffi::c_int as isize),
                    (&raw mut x as *mut ::core::ffi::c_double)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            if ntoks >= 7 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset(6 as ::core::ffi::c_int as isize),
                    (&raw mut x as *mut ::core::ffi::c_double)
                        .offset(3 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
        }
    }
    if x[0 as ::core::ffi::c_int as usize] < 0.0f64 {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    if x[1 as ::core::ffi::c_int as usize] < -10.0f64
        || x[1 as ::core::ffi::c_int as usize] > 10.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(4 as ::core::ffi::c_int as isize),
        );
    }
    if x[2 as ::core::ffi::c_int as usize] < 0.0f64
        || x[2 as ::core::ffi::c_int as usize] > 100.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(5 as ::core::ffi::c_int as isize),
        );
    }
    if x[3 as ::core::ffi::c_int as usize] < 0.0f64
        || x[3 as ::core::ffi::c_int as usize] > 100.0f64
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(6 as ::core::ffi::c_int as isize),
        );
    }
    if func == EXPON_WASHOFF as ::core::ffi::c_int {
        x[0 as ::core::ffi::c_int as usize] /= 3600.0f64;
    }
    if func == RATING_WASHOFF as ::core::ffi::c_int {
        x[0 as ::core::ffi::c_int as usize] *= pow(
            UCF(FLOW as ::core::ffi::c_int),
            x[1 as ::core::ffi::c_int as usize],
        );
    }
    if func == EMC_WASHOFF as ::core::ffi::c_int {
        x[0 as ::core::ffi::c_int as usize] *= LperFT3;
    }
    (*(*Landuse.offset(j as isize)).washoffFunc.offset(p as isize)).funcType = func;
    (*(*Landuse.offset(j as isize)).washoffFunc.offset(p as isize)).coeff =
        x[0 as ::core::ffi::c_int as usize];
    (*(*Landuse.offset(j as isize)).washoffFunc.offset(p as isize)).expon =
        x[1 as ::core::ffi::c_int as usize];
    (*(*Landuse.offset(j as isize)).washoffFunc.offset(p as isize)).sweepEffic =
        x[2 as ::core::ffi::c_int as usize] / 100.0f64;
    (*(*Landuse.offset(j as isize)).washoffFunc.offset(p as isize)).bmpEffic =
        x[3 as ::core::ffi::c_int as usize] / 100.0f64;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn landuse_getInitBuildup(
    mut landFactor: *mut TLandFactor,
    mut initBuildup: *mut ::core::ffi::c_double,
    mut area: ::core::ffi::c_double,
    mut curb: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut startDrySeconds: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut fArea: ::core::ffi::c_double = 0.;
    let mut fCurb: ::core::ffi::c_double = 0.;
    let mut buildup: ::core::ffi::c_double = 0.;
    startDrySeconds = StartDryDays * SECperDAY as ::core::ffi::c_double;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
        (*landFactor.offset(i as isize)).lastSwept = (StartDateTime as ::core::ffi::c_double
            - (*Landuse.offset(i as isize)).sweepDays0)
            as DateTime;
        f = (*landFactor.offset(i as isize)).fraction;
        fArea = f * area * UCF(LANDAREA as ::core::ffi::c_int);
        fCurb = f * curb;
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            buildup = 0.0f64;
            if *initBuildup.offset(p as isize) > 0.0f64 {
                buildup = *initBuildup.offset(p as isize) * fArea;
            } else {
                buildup = landuse_getBuildup(i, p, fArea, fCurb, buildup, startDrySeconds);
            }
            *(*landFactor.offset(i as isize)).buildup.offset(p as isize) = buildup;
            p += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn landuse_getBuildup(
    mut i: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut area: ::core::ffi::c_double,
    mut curb: ::core::ffi::c_double,
    mut buildup: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut n: ::core::ffi::c_int = 0;
    let mut days: ::core::ffi::c_double = 0.;
    let mut perUnit: ::core::ffi::c_double = 0.;
    if (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).funcType
        == NO_BUILDUP as ::core::ffi::c_int
        || tStep == 0.0f64
    {
        return buildup;
    }
    n = (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).normalizer;
    perUnit = 1.0f64;
    if n == PER_AREA as ::core::ffi::c_int {
        perUnit = area;
    }
    if n == PER_CURB as ::core::ffi::c_int {
        perUnit = curb;
    }
    if perUnit == 0.0f64 {
        return 0.0f64;
    }
    if (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).funcType
        == EXTERNAL_BUILDUP as ::core::ffi::c_int
    {
        return landuse_getExternalBuildup(i, p, buildup / perUnit, tStep) * perUnit;
    }
    days = landuse_getBuildupDays(i, p, buildup / perUnit);
    days += tStep / SECperDAY as ::core::ffi::c_double;
    return landuse_getBuildupMass(i, p, days) * perUnit;
}
unsafe extern "C" fn landuse_getBuildupDays(
    mut i: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut buildup: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut c0: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [0 as ::core::ffi::c_int as usize];
    let mut c1: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [1 as ::core::ffi::c_int as usize];
    let mut c2: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [2 as ::core::ffi::c_int as usize];
    if buildup == 0.0f64 {
        return 0.0f64;
    }
    if buildup >= c0 {
        return (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).maxDays;
    }
    match (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).funcType {
        1 => {
            if c1 * c2 == 0.0f64 {
                return 0.0f64;
            } else {
                return pow(buildup / c1, 1.0f64 / c2);
            }
        }
        2 => {
            if c0 * c1 == 0.0f64 {
                return 0.0f64;
            } else {
                return -log(1.0f64 - buildup / c0) / c1;
            }
        }
        3 => {
            if c0 == 0.0f64 {
                return 0.0f64;
            } else {
                return buildup * c2 / (c0 - buildup);
            }
        }
        _ => return 0.0f64,
    };
}
unsafe extern "C" fn landuse_getBuildupMass(
    mut i: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut days: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut b: ::core::ffi::c_double = 0.;
    let mut c0: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [0 as ::core::ffi::c_int as usize];
    let mut c1: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [1 as ::core::ffi::c_int as usize];
    let mut c2: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [2 as ::core::ffi::c_int as usize];
    if days == 0.0f64 {
        return 0.0f64;
    }
    if days >= (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).maxDays {
        return c0;
    }
    match (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).funcType {
        1 => {
            b = c1 * pow(days, c2);
            if b > c0 {
                b = c0;
            }
        }
        2 => {
            b = c0 * (1.0f64 - exp(-days * c1));
        }
        3 => {
            b = days * c0 / (c2 + days);
        }
        _ => {
            b = 0.0f64;
        }
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn landuse_getAvgBmpEffic(
    mut j: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_double = 0.0f64;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
        r += (*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize)).fraction
            * (*(*Landuse.offset(i as isize)).washoffFunc.offset(p as isize)).bmpEffic;
        i += 1;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn landuse_getWashoffLoad(
    mut i: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut area: ::core::ffi::c_double,
    mut landFactor: *mut TLandFactor,
    mut runoff: ::core::ffi::c_double,
    mut vOutflow: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut landuseArea: ::core::ffi::c_double = 0.;
    let mut buildup: ::core::ffi::c_double = 0.;
    let mut washoffQual: ::core::ffi::c_double = 0.;
    let mut washoffLoad: ::core::ffi::c_double = 0.;
    let mut bmpRemoval: ::core::ffi::c_double = 0.;
    buildup = *(*landFactor.offset(i as isize)).buildup.offset(p as isize);
    landuseArea = (*landFactor.offset(i as isize)).fraction * area;
    washoffQual = landuse_getWashoffQual(i, p, buildup, runoff, landuseArea);
    washoffLoad = washoffQual * vOutflow * landuseArea / area * (*Pollut.offset(p as isize)).mcf;
    if (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).funcType
        != NO_BUILDUP as ::core::ffi::c_int
        || buildup > washoffLoad
    {
        washoffLoad = if washoffLoad <= buildup {
            washoffLoad
        } else {
            buildup
        };
        buildup -= washoffLoad;
        *(*landFactor.offset(i as isize)).buildup.offset(p as isize) = buildup;
    } else {
        massbal_updateLoadingTotals(BUILDUP_LOAD as ::core::ffi::c_int, p, washoffLoad);
        *(*landFactor.offset(i as isize)).buildup.offset(p as isize) = 0.0f64;
    }
    bmpRemoval =
        (*(*Landuse.offset(i as isize)).washoffFunc.offset(p as isize)).bmpEffic * washoffLoad;
    if bmpRemoval > 0.0f64 {
        massbal_updateLoadingTotals(BMP_REMOVAL_LOAD as ::core::ffi::c_int, p, bmpRemoval);
        washoffLoad -= bmpRemoval;
    }
    return washoffLoad / (*Pollut.offset(p as isize)).mcf;
}
unsafe extern "C" fn landuse_getWashoffQual(
    mut i: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut buildup: ::core::ffi::c_double,
    mut runoff: ::core::ffi::c_double,
    mut area: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut cWashoff: ::core::ffi::c_double = 0.0f64;
    let mut coeff: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).washoffFunc.offset(p as isize)).coeff;
    let mut expon: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).washoffFunc.offset(p as isize)).expon;
    let mut func: ::core::ffi::c_int =
        (*(*Landuse.offset(i as isize)).washoffFunc.offset(p as isize)).funcType;
    if func == NO_WASHOFF as ::core::ffi::c_int || runoff == 0.0f64 {
        return 0.0f64;
    }
    if (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).funcType
        != NO_BUILDUP as ::core::ffi::c_int
        && buildup == 0.0f64
    {
        return 0.0f64;
    }
    if func == EXPON_WASHOFF as ::core::ffi::c_int {
        cWashoff = coeff * pow(runoff * UCF(RAINFALL as ::core::ffi::c_int), expon) * buildup
            / (*Pollut.offset(p as isize)).mcf;
        cWashoff /= runoff * area;
    } else if func == RATING_WASHOFF as ::core::ffi::c_int {
        cWashoff = coeff * pow(runoff * area, expon - 1.0f64);
    } else if func == EMC_WASHOFF as ::core::ffi::c_int {
        cWashoff = coeff;
    }
    return cWashoff;
}
#[no_mangle]
pub unsafe extern "C" fn landuse_getCoPollutLoad(
    mut p: ::core::ffi::c_int,
    mut washoff: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_double = 0.;
    k = (*Pollut.offset(p as isize)).coPollut;
    if k >= 0 as ::core::ffi::c_int {
        w = (*Pollut.offset(p as isize)).coFraction * *washoff.offset(k as isize);
        massbal_updateLoadingTotals(
            BUILDUP_LOAD as ::core::ffi::c_int,
            p,
            w * (*Pollut.offset(p as isize)).mcf,
        );
        return w;
    }
    return 0.0f64;
}
unsafe extern "C" fn landuse_getExternalBuildup(
    mut i: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut buildup: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut maxBuildup: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [0 as ::core::ffi::c_int as usize];
    let mut sf: ::core::ffi::c_double =
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [1 as ::core::ffi::c_int as usize];
    let mut ts: ::core::ffi::c_int = crate::ccast::f64_to_i32(floor(
        (*(*Landuse.offset(i as isize)).buildupFunc.offset(p as isize)).coeff
            [2 as ::core::ffi::c_int as usize],
    ));
    let mut rate: ::core::ffi::c_double = 0.0f64;
    if NewRunoffTime == 0.0f64 {
        return 0.0f64;
    }
    if ts >= 0 as ::core::ffi::c_int {
        rate = sf
            * table_tseriesLookup(
                Tseries.offset(ts as isize) as *mut TTable,
                getDateTime(NewRunoffTime) as ::core::ffi::c_double,
                FALSE as ::core::ffi::c_char,
            );
    }
    buildup = buildup + rate * tStep / SECperDAY as ::core::ffi::c_double;
    buildup = if buildup <= maxBuildup {
        buildup
    } else {
        maxBuildup
    };
    return buildup;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LperFT3: ::core::ffi::c_double = 28.317f64;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;

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
