#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn rewind(__stream: *mut FILE);
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
    fn datetime_addSeconds(date1: DateTime, seconds: ::core::ffi::c_double) -> DateTime;
    fn datetime_timeDiff(date1: DateTime, date2: DateTime) -> ::core::ffi::c_long;
    static mut Frain: TFile;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut Gage: *mut TGage;
    fn report_writeLine(line: *const ::core::ffi::c_char);
    fn report_writeRainStats(gage: ::core::ffi::c_int, rainStats: *mut TRainStats);
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn rdii_openRdii();
    fn rdii_closeRdii();
    fn getTempFileName(s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcomp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sstrncpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        n: size_t,
    ) -> size_t;
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
pub type FileUsageType = ::core::ffi::c_uint;
pub const SAVE_FILE: FileUsageType = 3;
pub const USE_FILE: FileUsageType = 2;
pub const SCRATCH_FILE: FileUsageType = 1;
pub const NO_FILE: FileUsageType = 0;
pub type GageDataType = ::core::ffi::c_uint;
pub const RAIN_FILE: GageDataType = 1;
pub const RAIN_TSERIES: GageDataType = 0;
pub type UnitsType = ::core::ffi::c_uint;
pub const SI: UnitsType = 1;
pub const US: UnitsType = 0;
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
pub struct TRainStats {
    pub startDate: DateTime,
    pub endDate: DateTime,
    pub periodsRain: ::core::ffi::c_long,
    pub periodsMissing: ::core::ffi::c_long,
    pub periodsMalfunc: ::core::ffi::c_long,
}
pub const ERR_RAIN_FILE_GAGE: ErrorType = 321;
pub const ERR_RAIN_IFACE_FORMAT: ErrorType = 320;
pub const CMC_FIF: RainFileFormat = 8;
pub const CMC_HLY: RainFileFormat = 7;
pub const AES_HLY: RainFileFormat = 6;
pub const MISSING_PERIOD: ConditionCodes = 3;
pub const DELETED_PERIOD: ConditionCodes = 2;
pub const NO_CONDITION: ConditionCodes = 0;
pub const ACCUMULATED_PERIOD: ConditionCodes = 1;
pub const NWS_ONLINE_15: RainFileFormat = 5;
pub const NWS_ONLINE_60: RainFileFormat = 4;
pub const NWS_COMMA_DELIMITED: RainFileFormat = 3;
pub const NWS_SPACE_DELIMITED: RainFileFormat = 2;
pub const NWS_TAPE: RainFileFormat = 1;
pub const ERR_RAIN_FILE_SEQUENCE: ErrorType = 318;
pub const STD_SPACE_DELIMITED: RainFileFormat = 9;
pub const ERR_RAIN_FILE_FORMAT: ErrorType = 319;
pub const UNKNOWN_FORMAT: RainFileFormat = 0;
pub const ERR_RAIN_FILE_DATA: ErrorType = 317;
pub const ERR_RAIN_FILE_CONFLICT: ErrorType = 156;
pub const ERR_RAIN_FILE_OPEN: ErrorType = 315;
pub const ERR_RAIN_FILE_SCRATCH: ErrorType = 313;
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
pub type RainFileFormat = ::core::ffi::c_uint;
pub type ConditionCodes = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAXMSG: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MMperINCH: ::core::ffi::c_double = 25.40f64;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NO_DATE: ::core::ffi::c_int = -(693594 as ::core::ffi::c_int);
#[no_mangle]
pub static mut RainStats: TRainStats = TRainStats {
    startDate: 0.,
    endDate: 0.,
    periodsRain: 0,
    periodsMissing: 0,
    periodsMalfunc: 0,
};
#[no_mangle]
pub static mut Condition: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut TimeOffset: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut DataOffset: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ValueOffset: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut RainType: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut Interval: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut UnitsFactor: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut RainAccum: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut StationID: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut AccumStartDate: DateTime = 0.;
#[no_mangle]
pub static mut PreviousDate: DateTime = 0.;
#[no_mangle]
pub static mut GageIndex: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut hasStationName: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn rain_open() {
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    count = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        if (*Gage.offset(i as isize)).dataSource == RAIN_FILE as ::core::ffi::c_int {
            count += 1;
        }
        i += 1;
    }
    Frain.file = ::core::ptr::null_mut::<FILE>();
    if count == 0 as ::core::ffi::c_int {
        Frain.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    } else {
        match Frain.mode as ::core::ffi::c_int {
            1 => {
                getTempFileName(&raw mut Frain.name as *mut ::core::ffi::c_char);
                Frain.file = fopen(
                    &raw mut Frain.name as *mut ::core::ffi::c_char,
                    b"w+b\0" as *const u8 as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if Frain.file.is_null() {
                    report_writeErrorMsg(
                        ERR_RAIN_FILE_SCRATCH as ::core::ffi::c_int,
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    return;
                }
            }
            2 => {
                Frain.file = fopen(
                    &raw mut Frain.name as *mut ::core::ffi::c_char,
                    b"r+b\0" as *const u8 as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if Frain.file.is_null() {
                    report_writeErrorMsg(
                        ERR_RAIN_FILE_OPEN as ::core::ffi::c_int,
                        &raw mut Frain.name as *mut ::core::ffi::c_char,
                    );
                    return;
                }
            }
            3 => {
                Frain.file = fopen(
                    &raw mut Frain.name as *mut ::core::ffi::c_char,
                    b"w+b\0" as *const u8 as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if Frain.file.is_null() {
                    report_writeErrorMsg(
                        ERR_RAIN_FILE_OPEN as ::core::ffi::c_int,
                        &raw mut Frain.name as *mut ::core::ffi::c_char,
                    );
                    return;
                }
            }
            _ => {}
        }
    }
    if Frain.mode as ::core::ffi::c_int == SCRATCH_FILE as ::core::ffi::c_int
        || Frain.mode as ::core::ffi::c_int == SAVE_FILE as ::core::ffi::c_int
    {
        createRainFile(count);
    }
    if Frain.mode as ::core::ffi::c_int != NO_FILE as ::core::ffi::c_int {
        initRainFile();
    }
    rdii_openRdii();
}
#[no_mangle]
pub unsafe extern "C" fn rain_close() {
    if !Frain.file.is_null() {
        fclose(Frain.file);
        if Frain.mode as ::core::ffi::c_int == SCRATCH_FILE as ::core::ffi::c_int {
            remove(&raw mut Frain.name as *mut ::core::ffi::c_char);
        }
    }
    Frain.file = ::core::ptr::null_mut::<FILE>();
    rdii_closeRdii();
}
unsafe extern "C" fn createRainFile(mut count: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut kount: ::core::ffi::c_int = count;
    let mut filePos1: ::core::ffi::c_int = 0;
    let mut filePos2: ::core::ffi::c_int = 0;
    let mut filePos3: ::core::ffi::c_int = 0;
    let mut interval: ::core::ffi::c_int = 0;
    let mut dummy: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut staID: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut fileStamp: [::core::ffi::c_char; 11] =
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"SWMM5-RAIN\0");
    if ErrorCode != 0 || Frain.file.is_null() {
        return;
    }
    fwrite(
        &raw mut fileStamp as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        strlen(&raw mut fileStamp as *mut ::core::ffi::c_char),
        Frain.file,
    );
    fwrite(
        &raw mut kount as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Frain.file,
    );
    filePos1 = ftell(Frain.file) as ::core::ffi::c_int;
    if count > 0 as ::core::ffi::c_int {
        report_writeRainStats(-(1 as ::core::ffi::c_int), &raw mut RainStats);
    }
    strcpy(
        &raw mut staID as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < count {
        fwrite(
            &raw mut staID as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            (MAXMSG + 1 as ::core::ffi::c_int) as size_t,
            Frain.file,
        );
        k = 1 as ::core::ffi::c_int;
        while k <= 3 as ::core::ffi::c_int {
            fwrite(
                &raw mut dummy as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                1 as size_t,
                Frain.file,
            );
            k += 1;
        }
        i += 1;
    }
    filePos2 = ftell(Frain.file) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        if !(ErrorCode != 0
            || (*Gage.offset(i as isize)).dataSource != RAIN_FILE as ::core::ffi::c_int)
        {
            if rainFileConflict(i) != 0 {
                break;
            }
            fseek(Frain.file, filePos2 as ::core::ffi::c_long, SEEK_SET);
            if addGageToRainFile(i) != 0 {
                filePos3 = ftell(Frain.file) as ::core::ffi::c_int;
                fseek(Frain.file, filePos1 as ::core::ffi::c_long, SEEK_SET);
                sstrncpy(
                    &raw mut staID as *mut ::core::ffi::c_char,
                    &raw mut (*Gage.offset(i as isize)).staID as *mut ::core::ffi::c_char,
                    MAXMSG as size_t,
                );
                interval = Interval;
                fwrite(
                    &raw mut staID as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                    (MAXMSG + 1 as ::core::ffi::c_int) as size_t,
                    Frain.file,
                );
                fwrite(
                    &raw mut interval as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                    1 as size_t,
                    Frain.file,
                );
                fwrite(
                    &raw mut filePos2 as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                    1 as size_t,
                    Frain.file,
                );
                fwrite(
                    &raw mut filePos3 as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                    1 as size_t,
                    Frain.file,
                );
                filePos1 = ftell(Frain.file) as ::core::ffi::c_int;
                filePos2 = filePos3;
                report_writeRainStats(i, &raw mut RainStats);
            }
        }
        i += 1;
    }
    if ErrorCode != 0 {
        fclose(Frain.file);
        Frain.file = ::core::ptr::null_mut::<FILE>();
        remove(&raw mut Frain.name as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn rainFileConflict(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut staID: *mut ::core::ffi::c_char =
        &raw mut (*Gage.offset(i as isize)).staID as *mut ::core::ffi::c_char;
    let mut fname: *mut ::core::ffi::c_char =
        &raw mut (*Gage.offset(i as isize)).fname as *mut ::core::ffi::c_char;
    j = 0 as ::core::ffi::c_int;
    while j < i {
        if strcomp(
            &raw mut (*Gage.offset(j as isize)).staID as *mut ::core::ffi::c_char,
            staID,
        ) != 0
            && strcomp(
                &raw mut (*Gage.offset(j as isize)).fname as *mut ::core::ffi::c_char,
                fname,
            ) == 0
        {
            report_writeErrorMsg(
                ERR_RAIN_FILE_CONFLICT as ::core::ffi::c_int,
                (*Gage.offset(i as isize)).ID,
            );
            return 1 as ::core::ffi::c_int;
        }
        j += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn addGageToRainFile(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut fileFormat: ::core::ffi::c_int = 0;
    let mut hdrLines: ::core::ffi::c_int = 0;
    StationID = ::core::ptr::null_mut::<::core::ffi::c_char>();
    f = fopen(
        &raw mut (*Gage.offset(i as isize)).fname as *mut ::core::ffi::c_char,
        b"rt\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        report_writeErrorMsg(
            ERR_RAIN_FILE_DATA as ::core::ffi::c_int,
            &raw mut (*Gage.offset(i as isize)).fname as *mut ::core::ffi::c_char,
        );
    } else {
        fileFormat = findFileFormat(f, i, &raw mut hdrLines);
        if fileFormat == UNKNOWN_FORMAT as ::core::ffi::c_int {
            report_writeErrorMsg(
                ERR_RAIN_FILE_FORMAT as ::core::ffi::c_int,
                &raw mut (*Gage.offset(i as isize)).fname as *mut ::core::ffi::c_char,
            );
        } else {
            GageIndex = i;
            readFile(
                f,
                fileFormat,
                hdrLines,
                (*Gage.offset(i as isize)).startFileDate,
                (*Gage.offset(i as isize)).endFileDate,
            );
        }
        fclose(f);
    }
    if ErrorCode != 0 {
        return 0 as ::core::ffi::c_int;
    } else {
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn initRainFile() {
    let mut fileStamp: [::core::ffi::c_char; 11] =
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"SWMM5-RAIN\0");
    let mut fStamp: [::core::ffi::c_char; 11] =
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"SWMM5-RAIN\0");
    let mut i: ::core::ffi::c_int = 0;
    let mut kount: ::core::ffi::c_int = 0;
    let mut filePos: ::core::ffi::c_long = 0;
    if ErrorCode != 0 || Frain.file.is_null() {
        return;
    }
    rewind(Frain.file);
    fread(
        &raw mut fStamp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        strlen(&raw mut fileStamp as *mut ::core::ffi::c_char),
        Frain.file,
    );
    if strcmp(
        &raw mut fStamp as *mut ::core::ffi::c_char,
        &raw mut fileStamp as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        report_writeErrorMsg(
            ERR_RAIN_IFACE_FORMAT as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    fread(
        &raw mut kount as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as size_t,
        Frain.file,
    );
    filePos = ftell(Frain.file);
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[GAGE as ::core::ffi::c_int as usize] {
        if !(ErrorCode != 0
            || (*Gage.offset(i as isize)).dataSource != RAIN_FILE as ::core::ffi::c_int)
        {
            fseek(Frain.file, filePos, SEEK_SET);
            if findGageInFile(i, kount) == 0
                || (*Gage.offset(i as isize)).startFilePos == (*Gage.offset(i as isize)).endFilePos
            {
                report_writeErrorMsg(
                    ERR_RAIN_FILE_GAGE as ::core::ffi::c_int,
                    (*Gage.offset(i as isize)).ID,
                );
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn findGageInFile(
    mut i: ::core::ffi::c_int,
    mut kount: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut interval: ::core::ffi::c_int = 0;
    let mut filePos1: ::core::ffi::c_int = 0;
    let mut filePos2: ::core::ffi::c_int = 0;
    let mut staID: [::core::ffi::c_char; 1025] = ::core::mem::transmute::<
        [u8; 1025],
        [::core::ffi::c_char; 1025],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    k = 1 as ::core::ffi::c_int;
    while k <= kount {
        fread(
            &raw mut staID as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            (MAXMSG + 1 as ::core::ffi::c_int) as size_t,
            Frain.file,
        );
        fread(
            &raw mut interval as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frain.file,
        );
        fread(
            &raw mut filePos1 as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frain.file,
        );
        fread(
            &raw mut filePos2 as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            1 as size_t,
            Frain.file,
        );
        if strcmp(
            &raw mut staID as *mut ::core::ffi::c_char,
            &raw mut (*Gage.offset(i as isize)).staID as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*Gage.offset(i as isize)).rainType = RAINFALL_VOLUME as ::core::ffi::c_int;
            (*Gage.offset(i as isize)).rainInterval = interval;
            (*Gage.offset(i as isize)).startFilePos = filePos1 as ::core::ffi::c_long;
            (*Gage.offset(i as isize)).endFilePos = filePos2 as ::core::ffi::c_long;
            (*Gage.offset(i as isize)).currentFilePos = (*Gage.offset(i as isize)).startFilePos;
            return TRUE;
        }
        k += 1;
    }
    return FALSE;
}
unsafe extern "C" fn findFileFormat(
    mut f: *mut FILE,
    mut i: ::core::ffi::c_int,
    mut hdrLines: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fileFormat: ::core::ffi::c_int = 0;
    let mut lineCount: ::core::ffi::c_int = 0;
    let mut maxCount: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    let mut div: ::core::ffi::c_int = 0;
    let mut sn2: ::core::ffi::c_long = 0;
    let mut recdType: [::core::ffi::c_char; 4] = [0; 4];
    let mut elemType: [::core::ffi::c_char; 5] = [0; 5];
    let mut coopID: [::core::ffi::c_char; 6] =
        ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"\0\0\0\0\0\0");
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut year: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut hour: ::core::ffi::c_int = 0;
    let mut minute: ::core::ffi::c_int = 0;
    let mut elem: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_float = 0.;
    fileFormat = UNKNOWN_FORMAT as ::core::ffi::c_int;
    hasStationName = FALSE;
    UnitsFactor = 1.0f64;
    Interval = 0 as ::core::ffi::c_int;
    *hdrLines = 0 as ::core::ffi::c_int;
    lineCount = 1 as ::core::ffi::c_int;
    while lineCount <= maxCount {
        if fgets(&raw mut line as *mut ::core::ffi::c_char, MAXLINE, f).is_null() {
            return fileFormat;
        }
        n = sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%6ld %2d %4s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut sn2,
            &raw mut div,
            &raw mut elemType as *mut ::core::ffi::c_char,
        );
        if n == 3 as ::core::ffi::c_int {
            Interval = getNWSInterval(&raw mut elemType as *mut ::core::ffi::c_char);
            TimeOffset = Interval;
            if Interval > 0 as ::core::ffi::c_int {
                fileFormat = NWS_SPACE_DELIMITED as ::core::ffi::c_int;
                break;
            }
        }
        n = sscanf(
            (&raw mut line as *mut ::core::ffi::c_char).offset(37 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            b"%2d %4s %2s %4d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut div,
            &raw mut elemType as *mut ::core::ffi::c_char,
            &raw mut recdType as *mut ::core::ffi::c_char,
            &raw mut year,
        );
        if n == 4 as ::core::ffi::c_int {
            Interval = getNWSInterval(&raw mut elemType as *mut ::core::ffi::c_char);
            TimeOffset = Interval;
            if Interval > 0 as ::core::ffi::c_int {
                fileFormat = NWS_SPACE_DELIMITED as ::core::ffi::c_int;
                hasStationName = TRUE;
                break;
            }
        }
        n = sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%6ld,%2d,%4s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut sn2,
            &raw mut div,
            &raw mut elemType as *mut ::core::ffi::c_char,
        );
        if n == 3 as ::core::ffi::c_int {
            Interval = getNWSInterval(&raw mut elemType as *mut ::core::ffi::c_char);
            TimeOffset = Interval;
            if Interval > 0 as ::core::ffi::c_int {
                fileFormat = NWS_COMMA_DELIMITED as ::core::ffi::c_int;
                break;
            }
        }
        n = sscanf(
            (&raw mut line as *mut ::core::ffi::c_char).offset(37 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char,
            b"%2d,%4s,%2s,%4d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut div,
            &raw mut elemType as *mut ::core::ffi::c_char,
            &raw mut recdType as *mut ::core::ffi::c_char,
            &raw mut year,
        );
        if n == 4 as ::core::ffi::c_int {
            Interval = getNWSInterval(&raw mut elemType as *mut ::core::ffi::c_char);
            TimeOffset = Interval;
            if Interval > 0 as ::core::ffi::c_int {
                fileFormat = NWS_COMMA_DELIMITED as ::core::ffi::c_int;
                hasStationName = TRUE;
                break;
            }
        }
        n = sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%3s%6ld%2d%4s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut recdType as *mut ::core::ffi::c_char,
            &raw mut sn2,
            &raw mut div,
            &raw mut elemType as *mut ::core::ffi::c_char,
        );
        if n == 4 as ::core::ffi::c_int {
            Interval = getNWSInterval(&raw mut elemType as *mut ::core::ffi::c_char);
            TimeOffset = Interval;
            if Interval > 0 as ::core::ffi::c_int {
                fileFormat = NWS_TAPE as ::core::ffi::c_int;
                break;
            }
        }
        n = sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%5s%6ld\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut coopID as *mut ::core::ffi::c_char,
            &raw mut sn2,
        );
        if n == 2 as ::core::ffi::c_int
            && strcmp(
                &raw mut coopID as *mut ::core::ffi::c_char,
                b"COOP:\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            fileFormat = findNWSOnlineFormat(f, &raw mut line as *mut ::core::ffi::c_char);
            break;
        } else {
            n = sscanf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"%7ld%3d%2d%2d%3d\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut sn2,
                &raw mut year,
                &raw mut month,
                &raw mut day,
                &raw mut elem,
            );
            if n == 5 as ::core::ffi::c_int {
                if elem == 123 as ::core::ffi::c_int
                    && strlen(&raw mut line as *mut ::core::ffi::c_char) >= 185 as size_t
                {
                    fileFormat = AES_HLY as ::core::ffi::c_int;
                    Interval = 3600 as ::core::ffi::c_int;
                    TimeOffset = Interval;
                    UnitsFactor = 1.0f64 / MMperINCH;
                    break;
                }
            }
            n = sscanf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"%7ld%4d%2d%2d%3d\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut sn2,
                &raw mut year,
                &raw mut month,
                &raw mut day,
                &raw mut elem,
            );
            if n == 5 as ::core::ffi::c_int {
                if elem == 159 as ::core::ffi::c_int
                    && strlen(&raw mut line as *mut ::core::ffi::c_char) >= 691 as size_t
                {
                    fileFormat = CMC_FIF as ::core::ffi::c_int;
                    Interval = 900 as ::core::ffi::c_int;
                } else if elem == 123 as ::core::ffi::c_int
                    && strlen(&raw mut line as *mut ::core::ffi::c_char) >= 186 as size_t
                {
                    fileFormat = CMC_HLY as ::core::ffi::c_int;
                    Interval = 3600 as ::core::ffi::c_int;
                }
                if fileFormat == CMC_FIF as ::core::ffi::c_int
                    || fileFormat == CMC_HLY as ::core::ffi::c_int
                {
                    TimeOffset = Interval;
                    UnitsFactor = 1.0f64 / MMperINCH;
                    break;
                }
            }
            if parseStdLine(
                &raw mut line as *mut ::core::ffi::c_char,
                &raw mut year,
                &raw mut month,
                &raw mut day,
                &raw mut hour,
                &raw mut minute,
                &raw mut x,
            ) != 0
            {
                fileFormat = STD_SPACE_DELIMITED as ::core::ffi::c_int;
                RainType = (*Gage.offset(i as isize)).rainType;
                Interval = (*Gage.offset(i as isize)).rainInterval;
                if (*Gage.offset(i as isize)).rainUnits == SI as ::core::ffi::c_int {
                    UnitsFactor = 1.0f64 / MMperINCH;
                }
                TimeOffset = 0 as ::core::ffi::c_int;
                StationID = &raw mut (*Gage.offset(i as isize)).staID as *mut ::core::ffi::c_char;
                break;
            } else {
                *hdrLines += 1;
                lineCount += 1;
            }
        }
    }
    if fileFormat != UNKNOWN_FORMAT as ::core::ffi::c_int {
        (*Gage.offset(i as isize)).rainInterval = Interval;
    }
    return fileFormat;
}
unsafe extern "C" fn findNWSOnlineFormat(
    mut f: *mut FILE,
    mut line: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut fileFormat: ::core::ffi::c_int = UNKNOWN_FORMAT as ::core::ffi::c_int;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    rewind(f);
    fgets(line, MAXLINE, f);
    str = strstr(line, b"HPCP\0" as *const u8 as *const ::core::ffi::c_char);
    if !str.is_null() {
        Interval = 3600 as ::core::ffi::c_int;
        TimeOffset = Interval;
        ValueOffset = str.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
        fileFormat = NWS_ONLINE_60 as ::core::ffi::c_int;
    } else {
        str = strstr(line, b"QPCP\0" as *const u8 as *const ::core::ffi::c_char);
        if !str.is_null() {
            Interval = 900 as ::core::ffi::c_int;
            TimeOffset = Interval;
            ValueOffset = str.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
            fileFormat = NWS_ONLINE_15 as ::core::ffi::c_int;
        } else {
            return UNKNOWN_FORMAT as ::core::ffi::c_int;
        }
    }
    n = 1 as ::core::ffi::c_int;
    while n <= 5 as ::core::ffi::c_int {
        if fgets(line, MAXLINE, f).is_null() {
            return UNKNOWN_FORMAT as ::core::ffi::c_int;
        }
        if strstr(line, b"COOP:\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            n += 1;
        } else {
            str = strrchr(line, ':' as i32);
            if str.is_null() {
                return UNKNOWN_FORMAT as ::core::ffi::c_int;
            }
            n = str.offset_from(line) as ::core::ffi::c_long as ::core::ffi::c_int;
            DataOffset = n - 11 as ::core::ffi::c_int;
            return fileFormat;
        }
    }
    return UNKNOWN_FORMAT as ::core::ffi::c_int;
}
unsafe extern "C" fn getNWSInterval(mut elemType: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    if strcmp(
        elemType,
        b"HPCP\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return 3600 as ::core::ffi::c_int;
    } else if strcmp(
        elemType,
        b"QPCP\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return 900 as ::core::ffi::c_int;
    } else if strcmp(
        elemType,
        b"QGAG\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return 900 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn readFile(
    mut f: *mut FILE,
    mut fileFormat: ::core::ffi::c_int,
    mut hdrLines: ::core::ffi::c_int,
    mut day1: DateTime,
    mut day2: DateTime,
) {
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    rewind(f);
    RainStats.startDate = NO_DATE as DateTime;
    RainStats.endDate = NO_DATE as DateTime;
    RainStats.periodsRain = 0 as ::core::ffi::c_long;
    RainStats.periodsMissing = 0 as ::core::ffi::c_long;
    RainStats.periodsMalfunc = 0 as ::core::ffi::c_long;
    RainAccum = 0.0f32;
    AccumStartDate = NO_DATE as DateTime;
    PreviousDate = NO_DATE as DateTime;
    i = 1 as ::core::ffi::c_int;
    while i <= hdrLines {
        if fgets(&raw mut line as *mut ::core::ffi::c_char, MAXLINE, f).is_null() {
            return;
        }
        i += 1;
    }
    while !fgets(&raw mut line as *mut ::core::ffi::c_char, MAXLINE, f).is_null() {
        match fileFormat {
            9 => {
                n = readStdLine(&raw mut line as *mut ::core::ffi::c_char, day1, day2);
            }
            1 | 2 | 3 | 4 | 5 => {
                n = readNWSLine(
                    &raw mut line as *mut ::core::ffi::c_char,
                    fileFormat,
                    day1,
                    day2,
                );
            }
            6 | 8 | 7 => {
                n = readCMCLine(
                    &raw mut line as *mut ::core::ffi::c_char,
                    fileFormat,
                    day1,
                    day2,
                );
            }
            _ => {
                n = -(1 as ::core::ffi::c_int);
            }
        }
        if n < 0 as ::core::ffi::c_int {
            break;
        }
    }
}
unsafe extern "C" fn readNWSLine(
    mut line: *mut ::core::ffi::c_char,
    mut fileFormat: ::core::ffi::c_int,
    mut day1: DateTime,
    mut day2: DateTime,
) -> ::core::ffi::c_int {
    let mut flag1: ::core::ffi::c_char = 0;
    let mut flag2: ::core::ffi::c_char = 0;
    let mut isMissing: ::core::ffi::c_char = 0;
    let mut date1: DateTime = 0.;
    let mut result: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let mut k: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut hour: ::core::ffi::c_int = 0;
    let mut minute: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_long = 0;
    let mut x: ::core::ffi::c_float = 0.;
    let mut lineLength: ::core::ffi::c_int =
        strlen(line) as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    let mut nameLength: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match fileFormat {
        1 => {
            if lineLength <= 30 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if sscanf(
                line.offset(17 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
                b"%4d%2d%4d%3d\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut y,
                &raw mut m,
                &raw mut d,
                &raw mut n,
            ) < 4 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            k = 30 as ::core::ffi::c_int;
        }
        2 => {
            if hasStationName != 0 {
                nameLength = 31 as ::core::ffi::c_int;
            }
            if lineLength <= 28 as ::core::ffi::c_int + nameLength {
                return 0 as ::core::ffi::c_int;
            }
            k = 18 as ::core::ffi::c_int + nameLength;
            if sscanf(
                line.offset(k as isize) as *mut ::core::ffi::c_char,
                b"%4d %2d %2d\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut y,
                &raw mut m,
                &raw mut d,
            ) < 3 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            k = k + 10 as ::core::ffi::c_int;
        }
        3 => {
            if lineLength <= 28 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if sscanf(
                line.offset(18 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
                b"%4d,%2d,%2d\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut y,
                &raw mut m,
                &raw mut d,
            ) < 3 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            k = 28 as ::core::ffi::c_int;
        }
        4 | 5 => {
            if lineLength <= DataOffset + 23 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if sscanf(
                line.offset(DataOffset as isize) as *mut ::core::ffi::c_char,
                b"%4d%2d%2d\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut y,
                &raw mut m,
                &raw mut d,
            ) < 3 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            k = DataOffset + 8 as ::core::ffi::c_int;
        }
        _ => return 0 as ::core::ffi::c_int,
    }
    date1 = datetime_encodeDate(y, m, d);
    if day1 != NO_DATE as DateTime && date1 < day1 {
        return 0 as ::core::ffi::c_int;
    }
    if day2 != NO_DATE as DateTime && date1 > day2 {
        return -(1 as ::core::ffi::c_int);
    }
    while k < lineLength {
        flag1 = 0 as ::core::ffi::c_char;
        flag2 = 0 as ::core::ffi::c_char;
        v = 99999 as ::core::ffi::c_long;
        hour = 25 as ::core::ffi::c_int;
        minute = 0 as ::core::ffi::c_int;
        match fileFormat {
            1 => {
                n = sscanf(
                    line.offset(k as isize) as *mut ::core::ffi::c_char,
                    b"%2d%2d%6ld%c%c\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut hour,
                    &raw mut minute,
                    &raw mut v,
                    &raw mut flag1,
                    &raw mut flag2,
                );
                k += 12 as ::core::ffi::c_int;
            }
            2 => {
                n = sscanf(
                    line.offset(k as isize) as *mut ::core::ffi::c_char,
                    b" %2d%2d %6ld %c %c\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut hour,
                    &raw mut minute,
                    &raw mut v,
                    &raw mut flag1,
                    &raw mut flag2,
                );
                k += 16 as ::core::ffi::c_int;
            }
            3 => {
                n = sscanf(
                    line.offset(k as isize) as *mut ::core::ffi::c_char,
                    b",%2d%2d,%6ld,%c,%c\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut hour,
                    &raw mut minute,
                    &raw mut v,
                    &raw mut flag1,
                    &raw mut flag2,
                );
                k += 16 as ::core::ffi::c_int;
            }
            4 | 5 => {
                n = sscanf(
                    line.offset(k as isize) as *mut ::core::ffi::c_char,
                    b" %2d:%2d\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut hour,
                    &raw mut minute,
                );
                n += readNwsOnlineValue(
                    line.offset(ValueOffset as isize) as *mut ::core::ffi::c_char,
                    &raw mut v,
                    &raw mut flag1,
                );
                if hour == 0 as ::core::ffi::c_int {
                    hour = 24 as ::core::ffi::c_int;
                    date1 -= 1.0f64;
                }
                k += lineLength;
            }
            _ => {
                n = 0 as ::core::ffi::c_int;
            }
        }
        if n < 3 as ::core::ffi::c_int || hour >= 25 as ::core::ffi::c_int {
            break;
        }
        setCondition(flag1);
        if Condition == DELETED_PERIOD as ::core::ffi::c_int
            || Condition == MISSING_PERIOD as ::core::ffi::c_int
            || flag1 as ::core::ffi::c_int == 'M' as i32
        {
            isMissing = TRUE as ::core::ffi::c_char;
        } else if v >= 9999 as ::core::ffi::c_long {
            isMissing = TRUE as ::core::ffi::c_char;
        } else {
            isMissing = FALSE as ::core::ffi::c_char;
        }
        if flag1 as ::core::ffi::c_int == 'a' as i32 {
            AccumStartDate = date1 + datetime_encodeTime(hour, minute, 0 as ::core::ffi::c_int);
        } else if flag1 as ::core::ffi::c_int == 'A' as i32 {
            saveAccumRainfall(date1, hour, minute, v);
        } else {
            x = v as ::core::ffi::c_float / 100.0f32;
            if x > 0 as ::core::ffi::c_int as ::core::ffi::c_float
                || isMissing as ::core::ffi::c_int != 0
            {
                saveRainfall(date1, hour, minute, x, isMissing);
            }
        }
        if flag1 as ::core::ffi::c_int == 'A' as i32
            || flag1 as ::core::ffi::c_int == '}' as i32
            || flag1 as ::core::ffi::c_int == ']' as i32
        {
            Condition = 0 as ::core::ffi::c_int;
        }
    }
    return result as ::core::ffi::c_int;
}
unsafe extern "C" fn readNwsOnlineValue(
    mut s: *mut ::core::ffi::c_char,
    mut v: *mut ::core::ffi::c_long,
    mut flag: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_float = 99.99f32;
    if !strchr(s, '.' as i32).is_null() {
        n = sscanf(
            s,
            b"%f %c\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut x,
            flag,
        );
        *v = crate::ccast::f32_to_i64((100.0f32 * x + 0.5f32));
    } else {
        n = sscanf(
            s,
            b"%ld %c\0" as *const u8 as *const ::core::ffi::c_char,
            v,
            flag,
        );
    }
    return n;
}
unsafe extern "C" fn setCondition(mut flag: ::core::ffi::c_char) {
    match flag as ::core::ffi::c_int {
        97 | 65 => {
            Condition = ACCUMULATED_PERIOD as ::core::ffi::c_int;
        }
        123 | 125 => {
            Condition = DELETED_PERIOD as ::core::ffi::c_int;
        }
        91 | 93 => {
            Condition = MISSING_PERIOD as ::core::ffi::c_int;
        }
        _ => {
            Condition = NO_CONDITION as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn readCMCLine(
    mut line: *mut ::core::ffi::c_char,
    mut fileFormat: ::core::ffi::c_int,
    mut day1: DateTime,
    mut day2: DateTime,
) -> ::core::ffi::c_int {
    let mut flag: ::core::ffi::c_char = 0;
    let mut isMissing: ::core::ffi::c_char = 0;
    let mut date1: DateTime = 0.;
    let mut sn: ::core::ffi::c_long = 0;
    let mut v: ::core::ffi::c_long = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut jMax: ::core::ffi::c_int = 0;
    let mut elem: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut hour: ::core::ffi::c_int = 0;
    let mut minute: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_float = 0.;
    if fileFormat == AES_HLY as ::core::ffi::c_int {
        if sscanf(
            line,
            b"%7ld%3d%2d%2d%3d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut sn,
            &raw mut y,
            &raw mut m,
            &raw mut d,
            &raw mut elem,
        ) < 5 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if y < 100 as ::core::ffi::c_int {
            y = y + 2000 as ::core::ffi::c_int;
        } else {
            y = y + 1000 as ::core::ffi::c_int;
        }
        col = 17 as ::core::ffi::c_int;
    } else {
        if sscanf(
            line,
            b"%7ld%4d%2d%2d%3d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut sn,
            &raw mut y,
            &raw mut m,
            &raw mut d,
            &raw mut elem,
        ) < 5 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        col = 18 as ::core::ffi::c_int;
    }
    date1 = datetime_encodeDate(y, m, d);
    if day1 != NO_DATE as DateTime && date1 < day1 {
        return 0 as ::core::ffi::c_int;
    }
    if day2 != NO_DATE as DateTime && date1 > day2 {
        return -(1 as ::core::ffi::c_int);
    }
    if fileFormat == AES_HLY as ::core::ffi::c_int && elem != 123 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    } else if fileFormat == CMC_FIF as ::core::ffi::c_int && elem != 159 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    } else if fileFormat == CMC_HLY as ::core::ffi::c_int && elem != 123 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    hour = 0 as ::core::ffi::c_int;
    minute = 0 as ::core::ffi::c_int;
    jMax = 24 as ::core::ffi::c_int;
    if fileFormat == CMC_FIF as ::core::ffi::c_int {
        jMax = 96 as ::core::ffi::c_int;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= jMax {
        if sscanf(
            line.offset(col as isize) as *mut ::core::ffi::c_char,
            b"%6ld%c\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut v,
            &raw mut flag,
        ) < 2 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        col += 7 as ::core::ffi::c_int;
        if v == -(99999 as ::core::ffi::c_int) as ::core::ffi::c_long {
            isMissing = TRUE as ::core::ffi::c_char;
        } else {
            isMissing = FALSE as ::core::ffi::c_char;
        }
        x = (v as ::core::ffi::c_double / 10.0f64 / MMperINCH) as ::core::ffi::c_float;
        if x > 0 as ::core::ffi::c_int as ::core::ffi::c_float
            || isMissing as ::core::ffi::c_int != 0
        {
            saveRainfall(date1, hour, minute, x, isMissing);
        }
        if fileFormat == CMC_FIF as ::core::ffi::c_int {
            minute += 15 as ::core::ffi::c_int;
            if minute == 60 as ::core::ffi::c_int {
                minute = 0 as ::core::ffi::c_int;
                hour += 1;
            }
        } else {
            hour += 1;
        }
        j += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn readStdLine(
    mut line: *mut ::core::ffi::c_char,
    mut day1: DateTime,
    mut day2: DateTime,
) -> ::core::ffi::c_int {
    let mut date1: DateTime = 0.;
    let mut date2: DateTime = 0.;
    let mut year: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut hour: ::core::ffi::c_int = 0;
    let mut minute: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_float = 0.;
    if parseStdLine(
        line,
        &raw mut year,
        &raw mut month,
        &raw mut day,
        &raw mut hour,
        &raw mut minute,
        &raw mut x,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    date1 = datetime_encodeDate(year, month, day);
    if day1 != NO_DATE as DateTime && date1 < day1 {
        return 0 as ::core::ffi::c_int;
    }
    if day2 != NO_DATE as DateTime && date1 > day2 {
        return -(1 as ::core::ffi::c_int);
    }
    date2 = date1 + datetime_encodeTime(hour, minute, 0 as ::core::ffi::c_int);
    if date2 <= PreviousDate {
        report_writeErrorMsg(
            ERR_RAIN_FILE_SEQUENCE as ::core::ffi::c_int,
            &raw mut (*Gage.offset(GageIndex as isize)).fname as *mut ::core::ffi::c_char,
        );
        report_writeLine(line);
        return -(1 as ::core::ffi::c_int);
    }
    PreviousDate = date2;
    match RainType {
        0 => {
            x = x * Interval as ::core::ffi::c_float / 3600.0f32;
        }
        2 => {
            if x >= RainAccum {
                x = x - RainAccum;
                RainAccum += x;
            } else {
                RainAccum = x;
            }
        }
        _ => {}
    }
    x *= UnitsFactor as ::core::ffi::c_float;
    saveRainfall(date1, hour, minute, x, FALSE as ::core::ffi::c_char);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn parseStdLine(
    mut line: *mut ::core::ffi::c_char,
    mut year: *mut ::core::ffi::c_int,
    mut month: *mut ::core::ffi::c_int,
    mut day: *mut ::core::ffi::c_int,
    mut hour: *mut ::core::ffi::c_int,
    mut minute: *mut ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut token: [::core::ffi::c_char; 1024] = [0; 1024];
    n = sscanf(
        line,
        b"%s %d %d %d %d %d %f\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut token as *mut ::core::ffi::c_char,
        year,
        month,
        day,
        hour,
        minute,
        value,
    );
    if n < 7 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !StationID.is_null() && strcomp(&raw mut token as *mut ::core::ffi::c_char, StationID) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn saveAccumRainfall(
    mut date1: DateTime,
    mut hour: ::core::ffi::c_int,
    mut minute: ::core::ffi::c_int,
    mut v: ::core::ffi::c_long,
) {
    let mut date2: DateTime = 0.;
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_float = 0.;
    if AccumStartDate == NO_DATE as DateTime {
        return;
    }
    date2 = date1 + datetime_encodeTime(hour, minute, 0 as ::core::ffi::c_int);
    n = (datetime_timeDiff(date2, AccumStartDate) / Interval as ::core::ffi::c_long
        + 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
    if v == 99999 as ::core::ffi::c_long {
        RainStats.periodsMissing += n as ::core::ffi::c_long;
        return;
    }
    RainStats.periodsRain += n as ::core::ffi::c_long;
    x = v as ::core::ffi::c_float / n as ::core::ffi::c_float / 100.0f32;
    if x > 0.0f32 {
        date2 = datetime_addSeconds(AccumStartDate, -TimeOffset as ::core::ffi::c_double);
        if RainStats.startDate == NO_DATE as DateTime {
            RainStats.startDate = date2;
        }
        j = 0 as ::core::ffi::c_int;
        while j < n {
            fwrite(
                &raw mut date2 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<DateTime>() as size_t,
                1 as size_t,
                Frain.file,
            );
            fwrite(
                &raw mut x as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                1 as size_t,
                Frain.file,
            );
            date2 = datetime_addSeconds(date2, Interval as ::core::ffi::c_double);
            RainStats.endDate = date2;
            j += 1;
        }
    }
    AccumStartDate = NO_DATE as DateTime;
}
unsafe extern "C" fn saveRainfall(
    mut date1: DateTime,
    mut hour: ::core::ffi::c_int,
    mut minute: ::core::ffi::c_int,
    mut x: ::core::ffi::c_float,
    mut isMissing: ::core::ffi::c_char,
) {
    let mut date2: DateTime = 0.;
    let mut seconds: ::core::ffi::c_int = 0;
    if isMissing != 0 {
        RainStats.periodsMissing += 1;
    } else {
        RainStats.periodsRain += 1;
    }
    if isMissing == 0 {
        seconds =
            3600 as ::core::ffi::c_int * hour + 60 as ::core::ffi::c_int * minute - TimeOffset;
        date2 = datetime_addSeconds(date1, seconds as ::core::ffi::c_double);
        fwrite(
            &raw mut date2 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<DateTime>() as size_t,
            1 as size_t,
            Frain.file,
        );
        fwrite(
            &raw mut x as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
            1 as size_t,
            Frain.file,
        );
        if RainStats.startDate == NO_DATE as DateTime {
            RainStats.startDate = date2;
        }
        RainStats.endDate = date2;
    }
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
