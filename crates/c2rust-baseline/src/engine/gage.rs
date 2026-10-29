#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn datetime_strToDate(s: *mut ::core::ffi::c_char, d: *mut DateTime) -> ::core::ffi::c_int;
    fn datetime_strToTime(s: *mut ::core::ffi::c_char, t: *mut DateTime) -> ::core::ffi::c_int;
    fn datetime_addSeconds(date1: DateTime, seconds: ::core::ffi::c_double) -> DateTime;
    static mut Frain: TFile;
    static mut UnitSystem: ::core::ffi::c_int;
    static mut IgnoreRainfall: ::core::ffi::c_int;
    static mut IgnoreSnowmelt: ::core::ffi::c_int;
    static mut WetStep: ::core::ffi::c_int;
    static mut StartDateTime: DateTime;
    static mut Temp: TTemp;
    static mut Snow: TSnow;
    static mut Adjust: TAdjust;
    static mut Gage: *mut TGage;
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
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sstrncpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        n: size_t,
    ) -> size_t;
    fn addAbsolutePath(fname: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut GageDataWords: [*mut ::core::ffi::c_char; 0];
    static mut RainTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut RainUnitsWords: [*mut ::core::ffi::c_char; 0];
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
pub type GageDataType = ::core::ffi::c_uint;
pub const RAIN_FILE: GageDataType = 1;
pub const RAIN_TSERIES: GageDataType = 0;
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
pub type RainfallType = ::core::ffi::c_uint;
pub const CUMULATIVE_RAINFALL: RainfallType = 2;
pub const RAINFALL_VOLUME: RainfallType = 1;
pub const RAINFALL_INTENSITY: RainfallType = 0;
pub type DateTime = ::core::ffi::c_double;
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
pub struct TAdjust {
    pub temp: [::core::ffi::c_double; 12],
    pub evap: [::core::ffi::c_double; 12],
    pub rain: [::core::ffi::c_double; 12],
    pub hydcon: [::core::ffi::c_double; 12],
    pub rainFactor: ::core::ffi::c_double,
    pub hydconFactor: ::core::ffi::c_double,
}
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_DATETIME: ErrorType = 213;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_RAIN_GAGE_INTERVAL: ErrorType = 159;
pub const ERR_RAIN_GAGE_TSERIES: ErrorType = 158;
pub const ERR_RAIN_GAGE_FORMAT: ErrorType = 157;
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
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NO_DATE: ::core::ffi::c_int = -(693594 as ::core::ffi::c_int);
pub const MAXPASTRAIN: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
#[no_mangle]
pub static mut OneSecond: ::core::ffi::c_double = 1.1574074e-5f64;
#[no_mangle]
pub unsafe extern "C" fn gage_readParams(
    mut j: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fname: [::core::ffi::c_char; 260] = [0; 260];
    let mut staID: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut x: [::core::ffi::c_double; 7] = [0.; 7];
    if ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    id = project_findID(
        GAGE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if id.is_null() {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    x[0 as ::core::ffi::c_int as usize] = -1.0f64;
    x[1 as ::core::ffi::c_int as usize] = 1.0f64;
    x[2 as ::core::ffi::c_int as usize] = 3600.0f64;
    x[3 as ::core::ffi::c_int as usize] = 1.0f64;
    x[4 as ::core::ffi::c_int as usize] = NO_DATE as ::core::ffi::c_double;
    x[5 as ::core::ffi::c_int as usize] = NO_DATE as ::core::ffi::c_double;
    x[6 as ::core::ffi::c_int as usize] = 0.0f64;
    fname[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    staID[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    if ntoks < 5 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    k = findmatch(
        *tok.offset(4 as ::core::ffi::c_int as isize),
        &raw mut GageDataWords as *mut *mut ::core::ffi::c_char,
    );
    if k == RAIN_TSERIES as ::core::ffi::c_int {
        err = readGageSeriesFormat(tok, ntoks, &raw mut x as *mut ::core::ffi::c_double);
    } else if k == RAIN_FILE as ::core::ffi::c_int {
        if ntoks < 8 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        sstrncpy(
            &raw mut fname as *mut ::core::ffi::c_char,
            *tok.offset(5 as ::core::ffi::c_int as isize),
            MAXFNAME as size_t,
        );
        sstrncpy(
            &raw mut staID as *mut ::core::ffi::c_char,
            *tok.offset(6 as ::core::ffi::c_int as isize),
            MAXMSG as size_t,
        );
        err = readGageFileFormat(tok, ntoks, &raw mut x as *mut ::core::ffi::c_double);
    } else {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(4 as ::core::ffi::c_int as isize),
        );
    }
    if err > 0 as ::core::ffi::c_int {
        return err;
    }
    let ref mut fresh0 = (*Gage.offset(j as isize)).ID;
    *fresh0 = id;
    (*Gage.offset(j as isize)).tSeries = crate::ccast::f64_to_i32(x[0 as ::core::ffi::c_int as usize]);
    (*Gage.offset(j as isize)).rainType = crate::ccast::f64_to_i32(x[1 as ::core::ffi::c_int as usize]);
    (*Gage.offset(j as isize)).rainInterval =
        crate::ccast::f64_to_i32(x[2 as ::core::ffi::c_int as usize]);
    (*Gage.offset(j as isize)).snowFactor = x[3 as ::core::ffi::c_int as usize];
    (*Gage.offset(j as isize)).rainUnits =
        crate::ccast::f64_to_i32(x[6 as ::core::ffi::c_int as usize]);
    if (*Gage.offset(j as isize)).tSeries >= 0 as ::core::ffi::c_int {
        (*Gage.offset(j as isize)).dataSource = RAIN_TSERIES as ::core::ffi::c_int;
    } else {
        (*Gage.offset(j as isize)).dataSource = RAIN_FILE as ::core::ffi::c_int;
    }
    if (*Gage.offset(j as isize)).dataSource == RAIN_FILE as ::core::ffi::c_int {
        sstrncpy(
            &raw mut (*Gage.offset(j as isize)).fname as *mut ::core::ffi::c_char,
            addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
            MAXFNAME as size_t,
        );
        sstrncpy(
            &raw mut (*Gage.offset(j as isize)).staID as *mut ::core::ffi::c_char,
            &raw mut staID as *mut ::core::ffi::c_char,
            MAXMSG as size_t,
        );
        (*Gage.offset(j as isize)).startFileDate = x[4 as ::core::ffi::c_int as usize] as DateTime;
        (*Gage.offset(j as isize)).endFileDate = x[5 as ::core::ffi::c_int as usize] as DateTime;
    }
    (*Gage.offset(j as isize)).unitsFactor = 1.0f64;
    (*Gage.offset(j as isize)).coGage = -(1 as ::core::ffi::c_int);
    (*Gage.offset(j as isize)).isUsed = FALSE;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readGageSeriesFormat(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut m: ::core::ffi::c_int = 0;
    let mut ts: ::core::ffi::c_int = 0;
    let mut aTime: DateTime = 0.;
    if ntoks < 6 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    m = findmatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut RainTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if m < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    *x.offset(1 as ::core::ffi::c_int as isize) = m as ::core::ffi::c_double;
    if getDouble(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        x.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
    ) != 0
    {
        *x.offset(2 as ::core::ffi::c_int as isize) = floor(
            *x.offset(2 as ::core::ffi::c_int as isize)
                * 3600 as ::core::ffi::c_int as ::core::ffi::c_double
                + 0.5f64,
        );
    } else if datetime_strToTime(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        &raw mut aTime,
    ) != 0
    {
        *x.offset(2 as ::core::ffi::c_int as isize) =
            floor(aTime as ::core::ffi::c_double * SECperDAY as ::core::ffi::c_double + 0.5f64);
    } else {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if *x.offset(2 as ::core::ffi::c_int as isize) <= 0.0f64 {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        x.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    ts = project_findObject(
        TSERIES as ::core::ffi::c_int,
        *tok.offset(5 as ::core::ffi::c_int as isize),
    );
    if ts < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(5 as ::core::ffi::c_int as isize),
        );
    }
    *x.offset(0 as ::core::ffi::c_int as isize) = ts as ::core::ffi::c_double;
    sstrncpy(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readGageFileFormat(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut m: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0;
    let mut aDate: DateTime = 0.;
    let mut aTime: DateTime = 0.;
    m = findmatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut RainTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if m < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    *x.offset(1 as ::core::ffi::c_int as isize) = m as ::core::ffi::c_double;
    if getDouble(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        x.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
    ) != 0
    {
        *x.offset(2 as ::core::ffi::c_int as isize) *=
            3600 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if datetime_strToTime(
        *tok.offset(2 as ::core::ffi::c_int as isize),
        &raw mut aTime,
    ) != 0
    {
        *x.offset(2 as ::core::ffi::c_int as isize) =
            floor(aTime as ::core::ffi::c_double * SECperDAY as ::core::ffi::c_double + 0.5f64);
    } else {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if *x.offset(2 as ::core::ffi::c_int as isize) <= 0.0f64 {
        return error_setInpError(
            ERR_DATETIME as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    if getDouble(
        *tok.offset(3 as ::core::ffi::c_int as isize),
        x.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
    ) == 0
    {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            *tok.offset(3 as ::core::ffi::c_int as isize),
        );
    }
    u = findmatch(
        *tok.offset(7 as ::core::ffi::c_int as isize),
        &raw mut RainUnitsWords as *mut *mut ::core::ffi::c_char,
    );
    if u < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(7 as ::core::ffi::c_int as isize),
        );
    }
    *x.offset(6 as ::core::ffi::c_int as isize) = u as ::core::ffi::c_double;
    if ntoks > 8 as ::core::ffi::c_int
        && **tok.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '*' as i32
    {
        if datetime_strToDate(
            *tok.offset(8 as ::core::ffi::c_int as isize),
            &raw mut aDate,
        ) == 0
        {
            return error_setInpError(
                ERR_DATETIME as ::core::ffi::c_int,
                *tok.offset(8 as ::core::ffi::c_int as isize),
            );
        }
        *x.offset(4 as ::core::ffi::c_int as isize) =
            aDate as ::core::ffi::c_float as ::core::ffi::c_double;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gage_validate(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut gageInterval: ::core::ffi::c_int = 0;
    if (*Gage.offset(j as isize)).dataSource == RAIN_TSERIES as ::core::ffi::c_int {
        if (*Gage.offset(j as isize)).isUsed == 0 {
            return;
        }
        k = (*Gage.offset(j as isize)).tSeries;
        i = 0 as ::core::ffi::c_int;
        while i < j {
            if (*Gage.offset(i as isize)).dataSource == RAIN_TSERIES as ::core::ffi::c_int
                && (*Gage.offset(i as isize)).tSeries == k
                && (*Gage.offset(i as isize)).isUsed != 0
            {
                (*Gage.offset(j as isize)).coGage = i;
                if (*Gage.offset(j as isize)).rainType != (*Gage.offset(i as isize)).rainType {
                    report_writeErrorMsg(
                        ERR_RAIN_GAGE_FORMAT as ::core::ffi::c_int,
                        (*Gage.offset(j as isize)).ID,
                    );
                }
                return;
            }
            i += 1;
        }
        if (*Tseries.offset(k as isize)).refersTo >= 0 as ::core::ffi::c_int {
            report_writeErrorMsg(
                ERR_RAIN_GAGE_TSERIES as ::core::ffi::c_int,
                (*Gage.offset(j as isize)).ID,
            );
        }
        gageInterval = crate::ccast::f64_to_i32(floor(
            (*Tseries.offset(k as isize)).dxMin * SECperDAY as ::core::ffi::c_double + 0.5f64,
        ));
        if gageInterval > 0 as ::core::ffi::c_int
            && (*Gage.offset(j as isize)).rainInterval > gageInterval
        {
            report_writeErrorMsg(
                ERR_RAIN_GAGE_INTERVAL as ::core::ffi::c_int,
                (*Gage.offset(j as isize)).ID,
            );
        }
        if (*Gage.offset(j as isize)).rainInterval < gageInterval {
            report_writeWarningMsg(
                WARN09.as_ptr() as *mut ::core::ffi::c_char,
                (*Gage.offset(j as isize)).ID,
            );
        }
        if (*Gage.offset(j as isize)).rainInterval < WetStep {
            report_writeWarningMsg(
                WARN01.as_ptr() as *mut ::core::ffi::c_char,
                (*Gage.offset(j as isize)).ID,
            );
            WetStep = (*Gage.offset(j as isize)).rainInterval;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gage_initState(mut j: ::core::ffi::c_int) {
    (*Gage.offset(j as isize)).rainfall = 0.0f64;
    (*Gage.offset(j as isize)).apiRainfall = MISSING;
    (*Gage.offset(j as isize)).reportRainfall = 0.0f64;
    if IgnoreRainfall != 0 {
        return;
    }
    if (*Gage.offset(j as isize)).dataSource == RAIN_FILE as ::core::ffi::c_int {
        (*Gage.offset(j as isize)).currentFilePos = (*Gage.offset(j as isize)).startFilePos;
        if UnitSystem == SI as ::core::ffi::c_int {
            (*Gage.offset(j as isize)).unitsFactor = MMperINCH;
        }
    }
    if getFirstRainfall(j) != 0 {
        (*Gage.offset(j as isize)).endDate = datetime_addSeconds(
            (*Gage.offset(j as isize)).startDate,
            (*Gage.offset(j as isize)).rainInterval as ::core::ffi::c_double,
        );
        if (*Gage.offset(j as isize)).startDate > StartDateTime {
            (*Gage.offset(j as isize)).nextDate = (*Gage.offset(j as isize)).startDate;
            (*Gage.offset(j as isize)).nextRainfall = (*Gage.offset(j as isize)).rainfall;
            (*Gage.offset(j as isize)).startDate = StartDateTime;
            (*Gage.offset(j as isize)).endDate = (*Gage.offset(j as isize)).nextDate;
            (*Gage.offset(j as isize)).rainfall = 0.0f64;
        } else if getNextRainfall(j) == 0 {
            (*Gage.offset(j as isize)).nextDate = NO_DATE as DateTime;
        }
    } else {
        (*Gage.offset(j as isize)).startDate = NO_DATE as DateTime;
    }
    initPastRain(j);
}
#[no_mangle]
pub unsafe extern "C" fn gage_setState(mut j: ::core::ffi::c_int, mut t: DateTime) {
    if (*Gage.offset(j as isize)).isUsed == FALSE {
        return;
    }
    if IgnoreRainfall != 0 {
        (*Gage.offset(j as isize)).rainfall = 0.0f64;
        return;
    }
    if (*Gage.offset(j as isize)).coGage >= 0 as ::core::ffi::c_int {
        (*Gage.offset(j as isize)).rainfall =
            (*Gage.offset((*Gage.offset(j as isize)).coGage as isize)).rainfall;
        return;
    }
    if (*Gage.offset(j as isize)).apiRainfall != MISSING {
        (*Gage.offset(j as isize)).rainfall = (*Gage.offset(j as isize)).apiRainfall;
        return;
    }
    t += OneSecond;
    loop {
        if (*Gage.offset(j as isize)).startDate == NO_DATE as DateTime {
            (*Gage.offset(j as isize)).rainfall = 0.0f64;
            return;
        }
        if t < (*Gage.offset(j as isize)).startDate {
            (*Gage.offset(j as isize)).rainfall = 0.0f64;
            return;
        }
        if t < (*Gage.offset(j as isize)).endDate {
            return;
        }
        if (*Gage.offset(j as isize)).nextDate == NO_DATE as DateTime {
            (*Gage.offset(j as isize)).rainfall = 0.0f64;
            return;
        }
        if t < (*Gage.offset(j as isize)).nextDate {
            (*Gage.offset(j as isize)).rainfall = 0.0f64;
            return;
        }
        (*Gage.offset(j as isize)).startDate = (*Gage.offset(j as isize)).nextDate;
        (*Gage.offset(j as isize)).endDate = datetime_addSeconds(
            (*Gage.offset(j as isize)).startDate,
            (*Gage.offset(j as isize)).rainInterval as ::core::ffi::c_double,
        );
        (*Gage.offset(j as isize)).rainfall = (*Gage.offset(j as isize)).nextRainfall;
        if getNextRainfall(j) == 0 {
            (*Gage.offset(j as isize)).nextDate = NO_DATE as DateTime;
        }
    }
}
unsafe extern "C" fn initPastRain(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i <= MAXPASTRAIN {
        (*Gage.offset(j as isize)).pastRain[i as usize] = 0.0f64;
        i += 1;
    }
    (*Gage.offset(j as isize)).pastInterval = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gage_updatePastRain(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_double = 0.;
    r = (*Gage.offset(j as isize)).rainfall / 3600.0f64;
    while tStep > 0 as ::core::ffi::c_int {
        t = 3600 as ::core::ffi::c_int - (*Gage.offset(j as isize)).pastInterval;
        if tStep > t {
            (*Gage.offset(j as isize)).pastRain[0 as ::core::ffi::c_int as usize] +=
                t as ::core::ffi::c_double * r;
            i = MAXPASTRAIN;
            while i > 0 as ::core::ffi::c_int {
                (*Gage.offset(j as isize)).pastRain[i as usize] =
                    (*Gage.offset(j as isize)).pastRain[(i - 1 as ::core::ffi::c_int) as usize];
                i -= 1;
            }
            (*Gage.offset(j as isize)).pastInterval = 0 as ::core::ffi::c_int;
            (*Gage.offset(j as isize)).pastRain[0 as ::core::ffi::c_int as usize] = 0.0f64;
            tStep -= t;
        } else {
            (*Gage.offset(j as isize)).pastRain[0 as ::core::ffi::c_int as usize] +=
                tStep as ::core::ffi::c_double * r;
            (*Gage.offset(j as isize)).pastInterval += tStep;
            tStep = 0 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gage_getPastRain(
    mut j: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_double = 0.0f64;
    if n < 1 as ::core::ffi::c_int || n > MAXPASTRAIN {
        return 0.0f64;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        result += (*Gage.offset(j as isize)).pastRain[i as usize];
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gage_getNextRainDate(
    mut j: ::core::ffi::c_int,
    mut aDate: DateTime,
) -> DateTime {
    if (*Gage.offset(j as isize)).isUsed == FALSE {
        return aDate;
    }
    aDate += OneSecond;
    if aDate < (*Gage.offset(j as isize)).startDate {
        return (*Gage.offset(j as isize)).startDate;
    }
    if aDate < (*Gage.offset(j as isize)).endDate {
        return (*Gage.offset(j as isize)).endDate;
    }
    return (*Gage.offset(j as isize)).nextDate;
}
#[no_mangle]
pub unsafe extern "C" fn gage_getPrecip(
    mut j: ::core::ffi::c_int,
    mut rainfall: *mut ::core::ffi::c_double,
    mut snowfall: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    *rainfall = 0.0f64;
    *snowfall = 0.0f64;
    if IgnoreSnowmelt == 0 && Temp.ta <= Snow.snotmp {
        *snowfall = (*Gage.offset(j as isize)).rainfall * (*Gage.offset(j as isize)).snowFactor
            / UCF(RAINFALL as ::core::ffi::c_int);
    } else {
        *rainfall = (*Gage.offset(j as isize)).rainfall / UCF(RAINFALL as ::core::ffi::c_int);
    }
    return *rainfall + *snowfall;
}
#[no_mangle]
pub unsafe extern "C" fn gage_setReportRainfall(
    mut j: ::core::ffi::c_int,
    mut reportDate: DateTime,
) {
    let mut result: ::core::ffi::c_double = 0.;
    if (*Gage.offset(j as isize)).coGage >= 0 as ::core::ffi::c_int {
        (*Gage.offset(j as isize)).reportRainfall =
            (*Gage.offset((*Gage.offset(j as isize)).coGage as isize)).reportRainfall;
        return;
    }
    if (*Gage.offset(j as isize)).apiRainfall != MISSING {
        (*Gage.offset(j as isize)).reportRainfall = (*Gage.offset(j as isize)).apiRainfall;
        return;
    }
    reportDate += OneSecond;
    if reportDate < (*Gage.offset(j as isize)).endDate {
        result = (*Gage.offset(j as isize)).rainfall;
    } else if reportDate < (*Gage.offset(j as isize)).nextDate {
        result = 0.0f64;
    } else {
        result = (*Gage.offset(j as isize)).nextRainfall;
    }
    (*Gage.offset(j as isize)).reportRainfall = result;
}
unsafe extern "C" fn getFirstRainfall(mut j: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut vFirst: ::core::ffi::c_float = 0.;
    let mut rFirst: ::core::ffi::c_double = 0.;
    (*Gage.offset(j as isize)).startDate = NO_DATE as DateTime;
    (*Gage.offset(j as isize)).rainfall = 0.0f64;
    (*Gage.offset(j as isize)).rainAccum = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if (*Gage.offset(j as isize)).dataSource == RAIN_FILE as ::core::ffi::c_int {
        if !Frain.file.is_null()
            && (*Gage.offset(j as isize)).endFilePos > (*Gage.offset(j as isize)).startFilePos
        {
            fseek(
                Frain.file,
                (*Gage.offset(j as isize)).startFilePos,
                SEEK_SET,
            );
            fread(
                &raw mut (*Gage.offset(j as isize)).startDate as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<DateTime>() as size_t,
                1 as size_t,
                Frain.file,
            );
            fread(
                &raw mut vFirst as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                1 as size_t,
                Frain.file,
            );
            (*Gage.offset(j as isize)).currentFilePos = ftell(Frain.file);
            (*Gage.offset(j as isize)).rainfall =
                convertRainfall(j, vFirst as ::core::ffi::c_double);
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    } else {
        k = (*Gage.offset(j as isize)).tSeries;
        if k >= 0 as ::core::ffi::c_int {
            if table_getFirstEntry(
                Tseries.offset(k as isize) as *mut TTable,
                &raw mut (*Gage.offset(j as isize)).startDate,
                &raw mut rFirst,
            ) != 0
            {
                (*Gage.offset(j as isize)).rainfall = convertRainfall(j, rFirst);
                return 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn getNextRainfall(mut j: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut vNext: ::core::ffi::c_float = 0.;
    let mut rNext: ::core::ffi::c_double = 0.;
    (*Gage.offset(j as isize)).nextRainfall = 0.0f64;
    loop {
        if (*Gage.offset(j as isize)).dataSource == RAIN_FILE as ::core::ffi::c_int {
            if !Frain.file.is_null()
                && (*Gage.offset(j as isize)).currentFilePos < (*Gage.offset(j as isize)).endFilePos
            {
                fseek(
                    Frain.file,
                    (*Gage.offset(j as isize)).currentFilePos,
                    SEEK_SET,
                );
                fread(
                    &raw mut (*Gage.offset(j as isize)).nextDate as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<DateTime>() as size_t,
                    1 as size_t,
                    Frain.file,
                );
                fread(
                    &raw mut vNext as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                    1 as size_t,
                    Frain.file,
                );
                (*Gage.offset(j as isize)).currentFilePos = ftell(Frain.file);
                rNext = convertRainfall(j, vNext as ::core::ffi::c_double);
            } else {
                return 0 as ::core::ffi::c_int;
            }
        } else {
            k = (*Gage.offset(j as isize)).tSeries;
            if k >= 0 as ::core::ffi::c_int {
                if table_getNextEntry(
                    Tseries.offset(k as isize) as *mut TTable,
                    &raw mut (*Gage.offset(j as isize)).nextDate,
                    &raw mut rNext,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                rNext = convertRainfall(j, rNext);
            } else {
                return 0 as ::core::ffi::c_int;
            }
        }
        if !(rNext == 0.0f64) {
            break;
        }
    }
    (*Gage.offset(j as isize)).nextRainfall = rNext;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn convertRainfall(
    mut j: ::core::ffi::c_int,
    mut r: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut r1: ::core::ffi::c_double = 0.;
    match (*Gage.offset(j as isize)).rainType {
        0 => {
            r1 = r;
        }
        1 => {
            r1 = r / (*Gage.offset(j as isize)).rainInterval as ::core::ffi::c_double * 3600.0f64;
        }
        2 => {
            if r < (*Gage.offset(j as isize)).rainAccum {
                r1 = r / (*Gage.offset(j as isize)).rainInterval as ::core::ffi::c_double
                    * 3600.0f64;
            } else {
                r1 = (r - (*Gage.offset(j as isize)).rainAccum)
                    / (*Gage.offset(j as isize)).rainInterval as ::core::ffi::c_double
                    * 3600.0f64;
            }
            (*Gage.offset(j as isize)).rainAccum = r;
        }
        _ => {
            r1 = r;
        }
    }
    return r1 * (*Gage.offset(j as isize)).unitsFactor * Adjust.rainFactor;
}
pub const MAXMSG: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXFNAME: ::core::ffi::c_int = 259 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MISSING: ::core::ffi::c_double = -1.0E10f64;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;
pub const MMperINCH: ::core::ffi::c_double = 25.40f64;
pub const WARN01: [::core::ffi::c_char; 78] = unsafe {
    ::core::mem::transmute::<[u8; 78], [::core::ffi::c_char; 78]>(
        *b"WARNING 01: wet weather time step reduced to recording interval for Rain Gage\0",
    )
};
pub const WARN09: [::core::ffi::c_char; 79] = unsafe {
    ::core::mem::transmute::<[u8; 79], [::core::ffi::c_char; 79]>(
        *b"WARNING 09: time series interval greater than recording interval for Rain Gage\0",
    )
};
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
