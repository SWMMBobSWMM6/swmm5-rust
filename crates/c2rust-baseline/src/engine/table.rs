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
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn rewind(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn datetime_strToDate(s: *mut ::core::ffi::c_char, d: *mut DateTime) -> ::core::ffi::c_int;
    fn datetime_strToTime(s: *mut ::core::ffi::c_char, t: *mut DateTime) -> ::core::ffi::c_int;
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
    fn addAbsolutePath(fname: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut CurveTypeWords: [*mut ::core::ffi::c_char; 0];
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
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_TABLE_FILE_READ: ErrorType = 363;
pub const ERR_CURVE_SEQUENCE: ErrorType = 171;
pub const ERR_TABLE_FILE_OPEN: ErrorType = 361;
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
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXFNAME: ::core::ffi::c_int = 259 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BIG: ::core::ffi::c_double = 1.0E10f64;
pub const SEPSTR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b" \t\n\r\0") };
pub const w_FILE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"FILE\0") };
#[no_mangle]
pub unsafe extern "C" fn table_interpolate(
    mut x: ::core::ffi::c_double,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut dx: ::core::ffi::c_double = x2 - x1;
    if fabs(dx) < 1.0e-20f64 {
        return (y1 + y2) / 2.0f64;
    }
    return y1 + (x - x1) * (y2 - y1) / dx;
}
#[no_mangle]
pub unsafe extern "C" fn table_readCurve(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut k1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    if ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        CURVE as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if (*Curve.offset(j as isize)).ID.is_null() {
        let ref mut fresh0 = (*Curve.offset(j as isize)).ID;
        *fresh0 = project_findID(
            CURVE as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
        m = findmatch(
            *tok.offset(1 as ::core::ffi::c_int as isize),
            &raw mut CurveTypeWords as *mut *mut ::core::ffi::c_char,
        );
        if m < 0 as ::core::ffi::c_int {
            return error_setInpError(
                ERR_KEYWORD as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        (*Curve.offset(j as isize)).curveType = m;
        if ntoks == 2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        k1 = 2 as ::core::ffi::c_int;
    }
    k = k1;
    while k < ntoks {
        if k + 1 as ::core::ffi::c_int >= ntoks {
            return error_setInpError(
                ERR_ITEMS as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if getDouble(*tok.offset(k as isize), &raw mut x) == 0 {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(k as isize));
        }
        if getDouble(
            *tok.offset((k + 1 as ::core::ffi::c_int) as isize),
            &raw mut y,
        ) == 0
        {
            return error_setInpError(
                ERR_NUMBER as ::core::ffi::c_int,
                *tok.offset((k + 1 as ::core::ffi::c_int) as isize),
            );
        }
        table_addEntry(Curve.offset(j as isize) as *mut TTable, x, y);
        k = k + 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn table_readTimeseries(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut state: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut d: DateTime = 0.;
    let mut t: DateTime = 0.;
    let mut fname: [::core::ffi::c_char; 260] = [0; 260];
    if ntoks < 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    j = project_findObject(
        TSERIES as ::core::ffi::c_int,
        *tok.offset(0 as ::core::ffi::c_int as isize),
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_NAME as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if (*Tseries.offset(j as isize)).ID.is_null() {
        let ref mut fresh1 = (*Tseries.offset(j as isize)).ID;
        *fresh1 = project_findID(
            TSERIES as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if strcomp(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        w_FILE.as_ptr(),
    ) != 0
    {
        sstrncpy(
            &raw mut fname as *mut ::core::ffi::c_char,
            *tok.offset(2 as ::core::ffi::c_int as isize),
            MAXFNAME as size_t,
        );
        sstrncpy(
            &raw mut (*Tseries.offset(j as isize)).file.name as *mut ::core::ffi::c_char,
            addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
            MAXFNAME as size_t,
        );
        (*Tseries.offset(j as isize)).file.mode =
            USE_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    }
    x = 0.0f64;
    k = 1 as ::core::ffi::c_int;
    state = 1 as ::core::ffi::c_int;
    while k < ntoks {
        match state {
            1 => {
                if datetime_strToDate(*tok.offset(k as isize), &raw mut d) != 0 {
                    (*Tseries.offset(j as isize)).lastDate = d as ::core::ffi::c_double;
                    k += 1;
                }
                state = 2 as ::core::ffi::c_int;
            }
            2 => {
                if k >= ntoks {
                    return error_setInpError(
                        ERR_ITEMS as ::core::ffi::c_int,
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                if getDouble(*tok.offset(k as isize), &raw mut t) != 0 {
                    t /= 24.0f64;
                } else if datetime_strToTime(*tok.offset(k as isize), &raw mut t) == 0 {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset(k as isize),
                    );
                }
                x = ((*Tseries.offset(j as isize)).lastDate as DateTime + t)
                    as ::core::ffi::c_double;
                k += 1;
                state = 3 as ::core::ffi::c_int;
            }
            3 => {
                if k >= ntoks {
                    return error_setInpError(
                        ERR_ITEMS as ::core::ffi::c_int,
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                if getDouble(*tok.offset(k as isize), &raw mut y) == 0 {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset(k as isize),
                    );
                }
                table_addEntry(Tseries.offset(j as isize) as *mut TTable, x, y);
                k += 1;
                state = 1 as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn table_addEntry(
    mut table: *mut TTable,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    entry = malloc(::core::mem::size_of::<TTableEntry>() as size_t) as *mut TTableEntry;
    if entry.is_null() {
        return FALSE;
    }
    (*entry).x = x;
    (*entry).y = y;
    (*entry).next = ::core::ptr::null_mut::<TableEntry>();
    if (*table).firstEntry.is_null() {
        (*table).firstEntry = entry;
    } else {
        (*(*table).lastEntry).next = entry as *mut TableEntry;
    }
    (*table).lastEntry = entry;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn table_deleteEntries(mut table: *mut TTable) {
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    let mut nextEntry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    entry = (*table).firstEntry;
    while !entry.is_null() {
        nextEntry = (*entry).next as *mut TTableEntry;
        free(entry as *mut ::core::ffi::c_void);
        entry = nextEntry;
    }
    (*table).firstEntry = ::core::ptr::null_mut::<TTableEntry>();
    (*table).lastEntry = ::core::ptr::null_mut::<TTableEntry>();
    (*table).thisEntry = ::core::ptr::null_mut::<TTableEntry>();
    if !(*table).file.file.is_null() {
        fclose((*table).file.file);
        (*table).file.file = ::core::ptr::null_mut::<FILE>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn table_init(mut table: *mut TTable) {
    (*table).ID = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*table).refersTo = -(1 as ::core::ffi::c_int);
    (*table).firstEntry = ::core::ptr::null_mut::<TTableEntry>();
    (*table).lastEntry = ::core::ptr::null_mut::<TTableEntry>();
    (*table).thisEntry = (*table).firstEntry;
    (*table).lastDate = 0.0f64;
    (*table).x1 = 0.0f64;
    (*table).x2 = 0.0f64;
    (*table).y1 = 0.0f64;
    (*table).y2 = 0.0f64;
    (*table).dxMin = 0.0f64;
    (*table).file.mode = NO_FILE as ::core::ffi::c_int as ::core::ffi::c_char;
    (*table).file.file = ::core::ptr::null_mut::<FILE>();
    (*table).curveType = -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn table_validate(mut table: *mut TTable) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_double = 0.;
    let mut x2: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut dx: ::core::ffi::c_double = 0.;
    let mut dxMin: ::core::ffi::c_double = BIG;
    if (*table).file.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        (*table).file.file = fopen(
            &raw mut (*table).file.name as *mut ::core::ffi::c_char,
            b"rt\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if (*table).file.file.is_null() {
            return ERR_TABLE_FILE_OPEN as ::core::ffi::c_int;
        }
    }
    result = table_getFirstEntry(table, &raw mut x1, &raw mut y1);
    if result == 0 && (*table).file.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        return ERR_TABLE_FILE_READ as ::core::ffi::c_int;
    }
    while table_getNextEntry(table, &raw mut x2, &raw mut y2) != 0 {
        dx = x2 - x1;
        if dx <= 0.0f64 {
            (*table).x2 = x2;
            return ERR_CURVE_SEQUENCE as ::core::ffi::c_int;
        }
        dxMin = if dxMin <= dx { dxMin } else { dx };
        x1 = x2;
    }
    (*table).dxMin = dxMin;
    if (*table).file.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int
        && feof((*table).file.file) == 0
    {
        return ERR_TABLE_FILE_READ as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn table_getFirstEntry(
    mut table: *mut TTable,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    *x = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    *y = 0.0f64;
    if (*table).file.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        if (*table).file.file.is_null() {
            return FALSE;
        }
        rewind((*table).file.file);
        return table_getNextFileEntry(table, x, y);
    }
    entry = (*table).firstEntry;
    if !entry.is_null() {
        *x = (*entry).x;
        *y = (*entry).y;
        (*table).thisEntry = entry;
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn table_getNextEntry(
    mut table: *mut TTable,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    if (*table).file.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        return table_getNextFileEntry(table, x, y);
    }
    entry = (*(*table).thisEntry).next as *mut TTableEntry;
    if !entry.is_null() {
        *x = (*entry).x;
        *y = (*entry).y;
        (*table).thisEntry = entry;
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn table_lookup(
    mut table: *mut TTable,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut x1: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut x2: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    entry = (*table).firstEntry;
    if entry.is_null() {
        return 0.0f64;
    }
    x1 = (*entry).x;
    y1 = (*entry).y;
    if x <= x1 {
        return y1;
    }
    while !(*entry).next.is_null() {
        entry = (*entry).next as *mut TTableEntry;
        x2 = (*entry).x;
        y2 = (*entry).y;
        if x <= x2 {
            return table_interpolate(x, x1, y1, x2, y2);
        }
        x1 = x2;
        y1 = y2;
    }
    return y1;
}
#[no_mangle]
pub unsafe extern "C" fn table_getSlope(
    mut table: *mut TTable,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut x1: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut x2: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut dx: ::core::ffi::c_double = 0.;
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    entry = (*table).firstEntry;
    if entry.is_null() {
        return 0.0f64;
    }
    x1 = (*entry).x;
    y1 = (*entry).y;
    x2 = x1;
    y2 = y1;
    while !(*entry).next.is_null() {
        entry = (*entry).next as *mut TTableEntry;
        x2 = (*entry).x;
        y2 = (*entry).y;
        if x <= x2 {
            break;
        }
        x1 = x2;
        y1 = y2;
    }
    dx = x2 - x1;
    if dx == 0.0f64 {
        return 0.0f64;
    }
    return (y2 - y1) / dx;
}
#[no_mangle]
pub unsafe extern "C" fn table_lookupEx(
    mut table: *mut TTable,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut x1: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut x2: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut s: ::core::ffi::c_double = 0.0f64;
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    entry = (*table).firstEntry;
    if entry.is_null() {
        return 0.0f64;
    }
    x1 = (*entry).x;
    y1 = (*entry).y;
    if x <= x1 {
        if x1 > 0.0f64 {
            return x / x1 * y1;
        } else {
            return y1;
        }
    }
    while !(*entry).next.is_null() {
        entry = (*entry).next as *mut TTableEntry;
        x2 = (*entry).x;
        y2 = (*entry).y;
        if x2 != x1 {
            s = (y2 - y1) / (x2 - x1);
        }
        if x <= x2 {
            return table_interpolate(x, x1, y1, x2, y2);
        }
        x1 = x2;
        y1 = y2;
    }
    if s < 0.0f64 {
        s = 0.0f64;
    }
    return y1 + s * (x - x1);
}
#[no_mangle]
pub unsafe extern "C" fn table_intervalLookup(
    mut table: *mut TTable,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    entry = (*table).firstEntry;
    if entry.is_null() {
        return 0.0f64;
    }
    if x < (*entry).x {
        return (*entry).y;
    }
    while !(*entry).next.is_null() {
        entry = (*entry).next as *mut TTableEntry;
        if x < (*entry).x {
            return (*entry).y;
        }
    }
    return (*entry).y;
}
#[no_mangle]
pub unsafe extern "C" fn table_inverseLookup(
    mut table: *mut TTable,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut x1: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut x2: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    entry = (*table).firstEntry;
    if entry.is_null() {
        return 0.0f64;
    }
    x1 = (*entry).x;
    y1 = (*entry).y;
    if y <= y1 {
        return x1;
    }
    while !(*entry).next.is_null() {
        entry = (*entry).next as *mut TTableEntry;
        x2 = (*entry).x;
        y2 = (*entry).y;
        if y <= y2 {
            return table_interpolate(y, y1, x1, y2, x2);
        }
        x1 = x2;
        y1 = y2;
    }
    return x1;
}
#[no_mangle]
pub unsafe extern "C" fn table_getMaxY(
    mut table: *mut TTable,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut ymax: ::core::ffi::c_double = 0.;
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    entry = (*table).firstEntry;
    ymax = (*entry).y;
    while x > (*entry).x && !(*entry).next.is_null() {
        entry = (*entry).next as *mut TTableEntry;
        if (*entry).y < ymax {
            return ymax;
        }
        ymax = (*entry).y;
    }
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn table_getStorageVolume(
    mut table: *mut TTable,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a: ::core::ffi::c_double = 0.;
    let mut a1: ::core::ffi::c_double = 0.;
    let mut x1: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    let mut dx: ::core::ffi::c_double = 0.0f64;
    let mut dy: ::core::ffi::c_double = 0.0f64;
    let mut s: ::core::ffi::c_double = 0.;
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    v = 0.0f64;
    entry = (*table).firstEntry;
    if entry.is_null() {
        return 0.0f64;
    }
    x1 = (*entry).x;
    a1 = (*entry).y;
    if x <= x1 {
        if x1 < 1.0e-6f64 {
            return 0.0f64;
        }
        return a1 / x1 * x * x / 2.0f64;
    }
    while !(*entry).next.is_null() {
        entry = (*entry).next as *mut TTableEntry;
        if (*entry).x >= x {
            a = table_interpolate(x, x1, a1, (*entry).x, (*entry).y);
            return v + (a1 + a) / 2.0f64 * (x - x1);
        } else {
            dx = (*entry).x - x1;
            dy = (*entry).y - a1;
            v = v + (a1 + (*entry).y) / 2.0f64 * dx;
            x1 = (*entry).x;
            a1 = (*entry).y;
        }
    }
    if dx > 1.0e-6f64 {
        s = dy / dx;
        a = a1 + s * (x - x1);
        if a < 0.0f64 {
            v = v - a1 * a1 / s / 2.0f64;
        } else {
            v = v + (a1 + a) / 2.0f64 * (x - x1);
        }
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn table_getStorageDepth(
    mut table: *mut TTable,
    mut v: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a1: ::core::ffi::c_double = 0.;
    let mut a2: ::core::ffi::c_double = 0.;
    let mut d1: ::core::ffi::c_double = 0.;
    let mut d2: ::core::ffi::c_double = 0.;
    let mut dd: ::core::ffi::c_double = 0.0f64;
    let mut da: ::core::ffi::c_double = 0.0f64;
    let mut v1: ::core::ffi::c_double = 0.;
    let mut v2: ::core::ffi::c_double = 0.;
    let mut s: ::core::ffi::c_double = 0.;
    let mut entry: *mut TTableEntry = ::core::ptr::null_mut::<TTableEntry>();
    if v == 0.0f64 {
        return 0.0f64;
    }
    entry = (*table).firstEntry;
    if entry.is_null() {
        return 0.0f64;
    }
    d1 = (*entry).x;
    a1 = (*entry).y;
    v1 = a1 * d1 / 2.0f64;
    if v <= v1 {
        if a1 > 0.0f64 {
            return sqrt(2.0f64 * v * d1 / a1);
        } else {
            return 0.0f64;
        }
    }
    while !(*entry).next.is_null() {
        entry = (*entry).next as *mut TTableEntry;
        d2 = (*entry).x;
        a2 = (*entry).y;
        dd = d2 - d1;
        da = a2 - a1;
        v2 = v1 + (a1 + a2) / 2.0f64 * dd;
        if v <= v2 {
            if dd <= 0.0f64 {
                return d1;
            }
            if da == 0.0f64 {
                if fabs(v2 - v1) < 1.0e-6f64 {
                    return d1;
                } else {
                    return d1 + dd * (v - v1) / (v2 - v1);
                }
            }
            if da < 0.0f64 {
                d1 = d2;
                a1 = a2;
                v1 = v2;
            }
            s = da / dd;
            return d1 + (sqrt(a1 * a1 + 2.0f64 * s * (v - v1)) - a1) / s;
        }
        d1 = d2;
        a1 = a2;
        v1 = v2;
    }
    if dd == 0.0f64 || da == 0.0f64 {
        if a1 > 0.0f64 {
            dd = (v - v1) / a1;
        } else {
            dd = 0.0f64;
        }
    } else {
        s = da / dd;
        dd = (sqrt(a1 * a1 + 2.0f64 * s * (v - v1)) - a1) / s;
        if dd < 0.0f64 {
            dd = 0.0f64;
        }
    }
    return d1 + dd;
}
#[no_mangle]
pub unsafe extern "C" fn table_tseriesInit(mut table: *mut TTable) {
    table_getFirstEntry(table, &raw mut (*table).x1, &raw mut (*table).y1);
    (*table).x2 = (*table).x1;
    (*table).y2 = (*table).y1;
    table_getNextEntry(table, &raw mut (*table).x2, &raw mut (*table).y2);
}
#[no_mangle]
pub unsafe extern "C" fn table_tseriesLookup(
    mut table: *mut TTable,
    mut x: ::core::ffi::c_double,
    mut extend: ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    if (*table).x1 <= x && (*table).x2 >= x && (*table).x1 != (*table).x2 {
        return table_interpolate(x, (*table).x1, (*table).y1, (*table).x2, (*table).y2);
    }
    if (*table).file.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int
        && feof((*table).file.file) != 0
    {
        if extend as ::core::ffi::c_int == TRUE {
            return (*table).y1;
        } else {
            return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    if (*table).x1 == (*table).x2 || x < (*table).x1 {
        table_getFirstEntry(table, &raw mut (*table).x1, &raw mut (*table).y1);
        if x < (*table).x1 {
            if extend as ::core::ffi::c_int == TRUE {
                return (*table).y1;
            } else {
                return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
        }
    }
    (*table).x1 = (*table).x2;
    (*table).y1 = (*table).y2;
    while table_getNextEntry(table, &raw mut (*table).x2, &raw mut (*table).y2) != 0 {
        if x <= (*table).x2 {
            return table_interpolate(x, (*table).x1, (*table).y1, (*table).x2, (*table).y2);
        }
        (*table).x1 = (*table).x2;
        (*table).y1 = (*table).y2;
    }
    if extend as ::core::ffi::c_int == TRUE {
        return (*table).y1;
    } else {
        return 0.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn table_getNextFileEntry(
    mut table: *mut TTable,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut code: ::core::ffi::c_int = 0;
    if (*table).file.file.is_null() {
        return FALSE;
    }
    while feof((*table).file.file) == 0
        && !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            MAXLINE,
            (*table).file.file,
        )
        .is_null()
    {
        code = table_parseFileLine(&raw mut line as *mut ::core::ffi::c_char, table, x, y);
        if code < 0 as ::core::ffi::c_int {
            continue;
        }
        return code;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn table_parseFileLine(
    mut line: *mut ::core::ffi::c_char,
    mut table: *mut TTable,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut s1: [::core::ffi::c_char; 50] = [0; 50];
    let mut s2: [::core::ffi::c_char; 50] = [0; 50];
    let mut s3: [::core::ffi::c_char; 50] = [0; 50];
    let mut tStr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut yStr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut yy: ::core::ffi::c_double = 0.;
    let mut d: DateTime = 0.;
    let mut t: DateTime = 0.;
    n = sscanf(
        line,
        b"%s %s %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut s1 as *mut ::core::ffi::c_char,
        &raw mut s2 as *mut ::core::ffi::c_char,
        &raw mut s3 as *mut ::core::ffi::c_char,
    );
    tStr = strtok(line, SEPSTR.as_ptr());
    if tStr.is_null() || *tStr as ::core::ffi::c_int == ';' as i32 {
        return -(1 as ::core::ffi::c_int);
    }
    if n == 2 as ::core::ffi::c_int {
        d = (*table).lastDate as DateTime;
        tStr = &raw mut s1 as *mut ::core::ffi::c_char;
        yStr = &raw mut s2 as *mut ::core::ffi::c_char;
    } else if n == 3 as ::core::ffi::c_int {
        if datetime_strToDate(&raw mut s1 as *mut ::core::ffi::c_char, &raw mut d) == 0 {
            return FALSE;
        }
        (*table).lastDate = d as ::core::ffi::c_double;
        tStr = &raw mut s2 as *mut ::core::ffi::c_char;
        yStr = &raw mut s3 as *mut ::core::ffi::c_char;
    } else {
        return FALSE;
    }
    if getDouble(tStr, &raw mut t) != 0 {
        t /= 24.0f64;
    } else if datetime_strToTime(tStr, &raw mut t) == 0 {
        return FALSE;
    }
    if getDouble(yStr, &raw mut yy) == 0 {
        return FALSE;
    }
    *x = (d + t) as ::core::ffi::c_double;
    *y = yy;
    return TRUE;
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
