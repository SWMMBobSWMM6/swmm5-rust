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
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn datetime_monthOfYear(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_dayOfWeek(date: DateTime) -> ::core::ffi::c_int;
    fn datetime_hourOfDay(date: DateTime) -> ::core::ffi::c_int;
    static mut Node: *mut TNode;
    static mut Pattern: *mut TPattern;
    static mut Tseries: *mut TTable;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    #[link_name = "match"]
    fn match_0(
        str: *mut ::core::ffi::c_char,
        substr: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut PatternTypeWords: [*mut ::core::ffi::c_char; 0];
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
pub type InflowType = ::core::ffi::c_uint;
pub const MASS_INFLOW: InflowType = 7;
pub const CONCEN_INFLOW: InflowType = 6;
pub const FLOW_INFLOW: InflowType = 5;
pub const RDII_INFLOW: InflowType = 4;
pub const GROUNDWATER_INFLOW: InflowType = 3;
pub const WET_WEATHER_INFLOW: InflowType = 2;
pub const DRY_WEATHER_INFLOW: InflowType = 1;
pub const EXTERNAL_INFLOW: InflowType = 0;
pub type PatternType = ::core::ffi::c_uint;
pub const WEEKEND_PATTERN: PatternType = 3;
pub const HOURLY_PATTERN: PatternType = 2;
pub const DAILY_PATTERN: PatternType = 1;
pub const MONTHLY_PATTERN: PatternType = 0;
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_ITEMS: ErrorType = 203;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LperFT3: ::core::ffi::c_double = 28.317f64;
pub const w_FLOW: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"FLOW\0") };
pub const w_CONCEN: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"CONCEN\0") };
pub const w_MASS: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"MASS\0") };
#[no_mangle]
pub unsafe extern "C" fn inflow_readExtInflow(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut param: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = CONCEN_INFLOW as ::core::ffi::c_int;
    let mut tseries: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut basePat: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cf: ::core::ffi::c_double = 1.0f64;
    let mut sf: ::core::ffi::c_double = 1.0f64;
    let mut baseline: ::core::ffi::c_double = 0.0f64;
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    param = project_findObject(
        POLLUT as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if param < 0 as ::core::ffi::c_int {
        if match_0(
            *tok.offset(1 as ::core::ffi::c_int as isize),
            w_FLOW.as_ptr() as *mut ::core::ffi::c_char,
        ) != 0
        {
            param = -(1 as ::core::ffi::c_int);
        } else {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
    }
    if strlen(*tok.offset(2 as ::core::ffi::c_int as isize)) > 0 as size_t {
        tseries = project_findObject(
            TSERIES as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
        if tseries < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        (*Tseries.offset(tseries as isize)).refersTo = EXTERNAL_INFLOW as ::core::ffi::c_int;
    }
    if param == -(1 as ::core::ffi::c_int) {
        type_0 = FLOW_INFLOW as ::core::ffi::c_int;
        cf = 1.0f64 / UCF(FLOW as ::core::ffi::c_int);
    }
    if ntoks >= 4 as ::core::ffi::c_int && param > -(1 as ::core::ffi::c_int) {
        if match_0(
            *tok.offset(3 as ::core::ffi::c_int as isize),
            w_CONCEN.as_ptr() as *mut ::core::ffi::c_char,
        ) != 0
        {
            type_0 = CONCEN_INFLOW as ::core::ffi::c_int;
        } else if match_0(
            *tok.offset(3 as ::core::ffi::c_int as isize),
            w_MASS.as_ptr() as *mut ::core::ffi::c_char,
        ) != 0
        {
            type_0 = MASS_INFLOW as ::core::ffi::c_int;
        } else {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(3 as ::core::ffi::c_int as isize),
            );
        }
        if ntoks >= 5 as ::core::ffi::c_int && type_0 == MASS_INFLOW as ::core::ffi::c_int {
            if getDouble(*tok.offset(4 as ::core::ffi::c_int as isize), &raw mut cf) == 0 {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(4 as ::core::ffi::c_int as isize),
                );
            }
            if cf <= 0.0f64 {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset(4 as ::core::ffi::c_int as isize),
                );
            }
        }
    }
    if ntoks >= 6 as ::core::ffi::c_int {
        if getDouble(*tok.offset(5 as ::core::ffi::c_int as isize), &raw mut sf) == 0 {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(5 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks >= 7 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(6 as ::core::ffi::c_int as isize),
            &raw mut baseline,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset(6 as ::core::ffi::c_int as isize),
            );
        }
    }
    if ntoks >= 8 as ::core::ffi::c_int {
        basePat = project_findObject(
            TIMEPATTERN as ::core::ffi::c_int,
            *tok.offset(7 as ::core::ffi::c_int as isize),
        );
        if basePat < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(7 as ::core::ffi::c_int as isize),
            );
        }
    }
    if type_0 == MASS_INFLOW as ::core::ffi::c_int {
        cf /= LperFT3;
    }
    return inflow_setExtInflow(j, param, type_0, tseries, basePat, cf, baseline, sf);
}
#[no_mangle]
pub unsafe extern "C" fn inflow_setExtInflow(
    mut j: ::core::ffi::c_int,
    mut param: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut tseries: ::core::ffi::c_int,
    mut basePat: ::core::ffi::c_int,
    mut cf: ::core::ffi::c_double,
    mut baseline: ::core::ffi::c_double,
    mut sf: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut inflow: *mut TExtInflow = ::core::ptr::null_mut::<TExtInflow>();
    inflow = (*Node.offset(j as isize)).extInflow;
    while !inflow.is_null() {
        if (*inflow).param == param {
            break;
        }
        inflow = (*inflow).next as *mut TExtInflow;
    }
    if inflow.is_null() {
        inflow = malloc(::core::mem::size_of::<TExtInflow>() as size_t) as *mut TExtInflow;
        if inflow.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        (*inflow).next = (*Node.offset(j as isize)).extInflow as *mut ExtInflow;
        let ref mut fresh0 = (*Node.offset(j as isize)).extInflow;
        *fresh0 = inflow;
    }
    (*inflow).param = param;
    (*inflow).type_0 = type_0;
    (*inflow).tSeries = tseries;
    (*inflow).cFactor = cf;
    (*inflow).sFactor = sf;
    (*inflow).baseline = baseline;
    (*inflow).basePat = basePat;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflow_deleteExtInflows(mut j: ::core::ffi::c_int) {
    let mut inflow1: *mut TExtInflow = ::core::ptr::null_mut::<TExtInflow>();
    let mut inflow2: *mut TExtInflow = ::core::ptr::null_mut::<TExtInflow>();
    inflow1 = (*Node.offset(j as isize)).extInflow;
    while !inflow1.is_null() {
        inflow2 = (*inflow1).next as *mut TExtInflow;
        free(inflow1 as *mut ::core::ffi::c_void);
        inflow1 = inflow2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn inflow_getExtInflow(
    mut inflow: *mut TExtInflow,
    mut aDate: DateTime,
) -> ::core::ffi::c_double {
    let mut month: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut hour: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = (*inflow).basePat;
    let mut k: ::core::ffi::c_int = (*inflow).tSeries;
    let mut cf: ::core::ffi::c_double = (*inflow).cFactor;
    let mut sf: ::core::ffi::c_double = (*inflow).sFactor;
    let mut blv: ::core::ffi::c_double = (*inflow).baseline;
    let mut tsv: ::core::ffi::c_double = 0.0f64;
    if p >= 0 as ::core::ffi::c_int {
        month = datetime_monthOfYear(aDate) - 1 as ::core::ffi::c_int;
        day = datetime_dayOfWeek(aDate) - 1 as ::core::ffi::c_int;
        hour = datetime_hourOfDay(aDate);
        blv *= getPatternFactor(p, month, day, hour);
    }
    if k >= 0 as ::core::ffi::c_int {
        tsv = table_tseriesLookup(
            Tseries.offset(k as isize) as *mut TTable,
            aDate as ::core::ffi::c_double,
            FALSE as ::core::ffi::c_char,
        ) * sf;
    }
    return cf * (tsv + blv);
}
#[no_mangle]
pub unsafe extern "C" fn inflow_readDwfInflow(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut pats: [::core::ffi::c_int; 4] = [0; 4];
    let mut x: ::core::ffi::c_double = 0.;
    let mut inflow: *mut TDwfInflow = ::core::ptr::null_mut::<TDwfInflow>();
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        NODE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    k = project_findObject(
        POLLUT as ::core::ffi::c_int,
        *tok.offset(1 as ::core::ffi::c_int as isize),
    );
    if k < 0 as ::core::ffi::c_int {
        if match_0(
            *tok.offset(1 as ::core::ffi::c_int as isize),
            w_FLOW.as_ptr() as *mut ::core::ffi::c_char,
        ) != 0
        {
            k = -(1 as ::core::ffi::c_int);
        } else {
            return error_setInpError(
                ERR_NAME as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
    }
    if getDouble(*tok.offset(2 as ::core::ffi::c_int as isize), &raw mut x) == 0 {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if k == -(1 as ::core::ffi::c_int) {
        x /= UCF(FLOW as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        pats[i as usize] = -(1 as ::core::ffi::c_int);
        i += 1;
    }
    i = 3 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        if i >= ntoks {
            break;
        }
        if !(strlen(*tok.offset(i as isize)) == 0 as size_t) {
            m = project_findObject(TIMEPATTERN as ::core::ffi::c_int, *tok.offset(i as isize));
            if m < 0 as ::core::ffi::c_int {
                return error_setInpError(ERR_NAME as ::core::ffi::c_int, *tok.offset(i as isize));
            }
            pats[(i - 3 as ::core::ffi::c_int) as usize] = m;
        }
        i += 1;
    }
    inflow = (*Node.offset(j as isize)).dwfInflow;
    while !inflow.is_null() {
        if (*inflow).param == k {
            break;
        }
        inflow = (*inflow).next as *mut TDwfInflow;
    }
    if inflow.is_null() {
        inflow = malloc(::core::mem::size_of::<TDwfInflow>() as size_t) as *mut TDwfInflow;
        if inflow.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        (*inflow).next = (*Node.offset(j as isize)).dwfInflow as *mut DwfInflow;
        let ref mut fresh1 = (*Node.offset(j as isize)).dwfInflow;
        *fresh1 = inflow;
    }
    (*inflow).param = k;
    (*inflow).avgValue = x;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*inflow).patterns[i as usize] = pats[i as usize];
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflow_deleteDwfInflows(mut j: ::core::ffi::c_int) {
    let mut inflow1: *mut TDwfInflow = ::core::ptr::null_mut::<TDwfInflow>();
    let mut inflow2: *mut TDwfInflow = ::core::ptr::null_mut::<TDwfInflow>();
    inflow1 = (*Node.offset(j as isize)).dwfInflow;
    while !inflow1.is_null() {
        inflow2 = (*inflow1).next as *mut TDwfInflow;
        free(inflow1 as *mut ::core::ffi::c_void);
        inflow1 = inflow2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn inflow_initDwfInflow(mut inflow: *mut TDwfInflow) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut tmpPattern: [::core::ffi::c_int; 4] = [0; 4];
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        tmpPattern[i as usize] = -(1 as ::core::ffi::c_int);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        p = (*inflow).patterns[i as usize];
        if p >= 0 as ::core::ffi::c_int {
            tmpPattern[(*Pattern.offset(p as isize)).type_0 as usize] = p;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*inflow).patterns[i as usize] = tmpPattern[i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn inflow_getDwfInflow(
    mut inflow: *mut TDwfInflow,
    mut month: ::core::ffi::c_int,
    mut day: ::core::ffi::c_int,
    mut hour: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 1.0f64;
    p1 = (*inflow).patterns[MONTHLY_PATTERN as ::core::ffi::c_int as usize];
    if p1 >= 0 as ::core::ffi::c_int {
        f *= getPatternFactor(p1, month, day, hour);
    }
    p1 = (*inflow).patterns[DAILY_PATTERN as ::core::ffi::c_int as usize];
    if p1 >= 0 as ::core::ffi::c_int {
        f *= getPatternFactor(p1, month, day, hour);
    }
    p1 = (*inflow).patterns[HOURLY_PATTERN as ::core::ffi::c_int as usize];
    p2 = (*inflow).patterns[WEEKEND_PATTERN as ::core::ffi::c_int as usize];
    if p2 >= 0 as ::core::ffi::c_int {
        if day == 0 as ::core::ffi::c_int || day == 6 as ::core::ffi::c_int {
            f *= getPatternFactor(p2, month, day, hour);
        } else if p1 >= 0 as ::core::ffi::c_int {
            f *= getPatternFactor(p1, month, day, hour);
        }
    } else if p1 >= 0 as ::core::ffi::c_int {
        f *= getPatternFactor(p1, month, day, hour);
    }
    return f * (*inflow).avgValue;
}
#[no_mangle]
pub unsafe extern "C" fn inflow_initDwfPattern(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 24 as ::core::ffi::c_int {
        (*Pattern.offset(j as isize)).factor[i as usize] = 1.0f64;
        i += 1;
    }
    (*Pattern.offset(j as isize)).count = 0 as ::core::ffi::c_int;
    (*Pattern.offset(j as isize)).type_0 = -(1 as ::core::ffi::c_int);
    let ref mut fresh4 = (*Pattern.offset(j as isize)).ID;
    *fresh4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn inflow_readDwfPattern(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        TIMEPATTERN as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if (*Pattern.offset(j as isize)).ID.is_null() {
        let ref mut fresh2 = (*Pattern.offset(j as isize)).ID;
        *fresh2 = project_findID(
            TIMEPATTERN as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
        k = findmatch(
            *tok.offset(1 as ::core::ffi::c_int as isize),
            &raw mut PatternTypeWords as *mut *mut ::core::ffi::c_char,
        );
        if k < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        (*Pattern.offset(j as isize)).type_0 = k;
        n = 2 as ::core::ffi::c_int;
    }
    while ntoks > n && (*Pattern.offset(j as isize)).count < 24 as ::core::ffi::c_int {
        i = (*Pattern.offset(j as isize)).count;
        if getDouble(
            *tok.offset(n as isize),
            (&raw mut (*Pattern.offset(j as isize)).factor as *mut ::core::ffi::c_double)
                .offset(i as isize) as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(n as isize));
        }
        let ref mut fresh3 = (*Pattern.offset(j as isize)).count;
        *fresh3 += 1;
        n += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getPatternFactor(
    mut p: ::core::ffi::c_int,
    mut month: ::core::ffi::c_int,
    mut day: ::core::ffi::c_int,
    mut hour: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    match (*Pattern.offset(p as isize)).type_0 {
        0 => {
            if month >= 0 as ::core::ffi::c_int && month < 12 as ::core::ffi::c_int {
                return (*Pattern.offset(p as isize)).factor[month as usize];
            }
        }
        1 => {
            if day >= 0 as ::core::ffi::c_int && day < 7 as ::core::ffi::c_int {
                return (*Pattern.offset(p as isize)).factor[day as usize];
            }
        }
        2 => {
            if hour >= 0 as ::core::ffi::c_int && hour < 24 as ::core::ffi::c_int {
                return (*Pattern.offset(p as isize)).factor[hour as usize];
            }
        }
        3 => {
            if day == 0 as ::core::ffi::c_int || day == 6 as ::core::ffi::c_int {
                if hour >= 0 as ::core::ffi::c_int && hour < 24 as ::core::ffi::c_int {
                    return (*Pattern.offset(p as isize)).factor[hour as usize];
                }
            }
        }
        _ => {}
    }
    return 1.0f64;
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
