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
    fn grnampt_setParams(infil: *mut TGrnAmpt, p: *mut ::core::ffi::c_double)
        -> ::core::ffi::c_int;
    fn grnampt_initState(infil: *mut TGrnAmpt);
    fn grnampt_getInfil(
        infil: *mut TGrnAmpt,
        tstep: ::core::ffi::c_double,
        irate: ::core::ffi::c_double,
        depth: ::core::ffi::c_double,
        modelType: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    static mut Adjust: TAdjust;
    static mut Storage: *mut TStorage;
    static mut Curve: *mut TTable;
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
    fn table_lookupEx(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub type StorageType = ::core::ffi::c_uint;
pub const PYRAMIDAL: StorageType = 5;
pub const PARABOLOID: StorageType = 4;
pub const CONICAL: StorageType = 3;
pub const CYLINDRICAL: StorageType = 2;
pub const FUNCTIONAL: StorageType = 1;
pub const TABULAR: StorageType = 0;
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
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_MEMORY: ErrorType = 101;
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
pub const ERR_ITEMS: ErrorType = 203;
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
pub const ERR_NAME: ErrorType = 209;
pub const ERR_DUP_NAME: ErrorType = 207;
pub const ERR_KEYWORD: ErrorType = 205;
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
pub const BIG: ::core::ffi::c_double = 1.0E10f64;
#[no_mangle]
pub unsafe extern "C" fn exfil_readStorageParams(
    mut k: ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 3] = [0.; 3];
    if ntoks == n + 1 as ::core::ffi::c_int {
        if getDouble(
            *tok.offset(n as isize),
            (&raw mut x as *mut ::core::ffi::c_double).offset(1 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        ) == 0
        {
            return error_setInpError(ERR_NUMBER as ::core::ffi::c_int, *tok.offset(n as isize));
        }
        x[0 as ::core::ffi::c_int as usize] = 0.0f64;
        x[2 as ::core::ffi::c_int as usize] = 0.0f64;
    } else if ntoks < n + 3 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            if getDouble(
                *tok.offset((n + i) as isize),
                (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
                    as *mut ::core::ffi::c_double,
            ) == 0
            {
                return error_setInpError(
                    ERR_NUMBER as ::core::ffi::c_int,
                    *tok.offset((n + i) as isize),
                );
            }
            i += 1;
        }
    }
    if x[1 as ::core::ffi::c_int as usize] == 0.0f64 {
        return 0 as ::core::ffi::c_int;
    }
    return createStorageExfil(k, &raw mut x as *mut ::core::ffi::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn exfil_initState(mut k: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_double = 0.;
    let mut alast: ::core::ffi::c_double = 0.;
    let mut d: ::core::ffi::c_double = 0.;
    let mut aCurve: *mut TTable = ::core::ptr::null_mut::<TTable>();
    let mut exfil: *mut TExfil = (*Storage.offset(k as isize)).exfil;
    if !exfil.is_null() {
        grnampt_initState((*exfil).btmExfil);
        grnampt_initState((*exfil).bankExfil);
        match (*Storage.offset(k as isize)).shape {
            0 => {
                i = (*Storage.offset(k as isize)).aCurve;
                (*exfil).btmArea = 0.0f64;
                (*exfil).bankMinDepth = 0.0f64;
                (*exfil).bankMaxDepth = 0.0f64;
                (*exfil).bankMaxArea = 0.0f64;
                if i >= 0 as ::core::ffi::c_int {
                    aCurve = Curve.offset(i as isize) as *mut TTable;
                    (*(*Storage.offset(k as isize)).exfil).btmArea = table_lookupEx(aCurve, 0.0f64);
                    table_getFirstEntry(aCurve, &raw mut d, &raw mut a);
                    alast = a;
                    while table_getNextEntry(aCurve, &raw mut d, &raw mut a) != 0 {
                        if a < alast {
                            break;
                        }
                        if a > alast {
                            (*exfil).bankMaxArea = a;
                            (*exfil).bankMaxDepth = d;
                        } else {
                            if !((*exfil).bankMaxArea == 0.0f64) {
                                break;
                            }
                            (*exfil).bankMinDepth = d;
                        }
                        alast = a;
                    }
                    (*exfil).btmArea /=
                        UCF(LENGTH as ::core::ffi::c_int) * UCF(LENGTH as ::core::ffi::c_int);
                    (*exfil).bankMaxArea /=
                        UCF(LENGTH as ::core::ffi::c_int) * UCF(LENGTH as ::core::ffi::c_int);
                    (*exfil).bankMinDepth /= UCF(LENGTH as ::core::ffi::c_int);
                    (*exfil).bankMaxDepth /= UCF(LENGTH as ::core::ffi::c_int);
                }
            }
            1 => {
                (*exfil).btmArea = (*Storage.offset(k as isize)).a0;
                if (*Storage.offset(k as isize)).a2 == 0.0f64 {
                    (*exfil).btmArea += (*Storage.offset(k as isize)).a1;
                }
                (*exfil).bankMinDepth = 0.0f64;
                (*exfil).bankMaxDepth = BIG;
                (*exfil).bankMaxArea = BIG;
            }
            2 | 3 | 5 => {
                (*exfil).btmArea = (*Storage.offset(k as isize)).a0;
                (*exfil).bankMinDepth = 0.0f64;
                (*exfil).bankMaxDepth = BIG;
                (*exfil).bankMaxArea = BIG;
            }
            _ => {}
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn exfil_getLoss(
    mut exfil: *mut TExfil,
    mut tStep: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
    mut area: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut exfilRate: ::core::ffi::c_double = 0.0f64;
    if (*(*exfil).btmExfil).IMDmax == 0.0f64 {
        exfilRate = (*(*exfil).btmExfil).Ks * Adjust.hydconFactor;
    } else {
        exfilRate = grnampt_getInfil(
            (*exfil).btmExfil,
            tStep,
            0.0f64,
            depth,
            MOD_GREEN_AMPT as ::core::ffi::c_int,
        );
    }
    exfilRate *= (*exfil).btmArea;
    if depth > (*exfil).bankMinDepth {
        area = (if area <= (*exfil).bankMaxArea {
            area
        } else {
            (*exfil).bankMaxArea
        }) - (*exfil).btmArea;
        if area > 0.0f64 {
            if (*(*exfil).btmExfil).IMDmax == 0.0f64 {
                exfilRate += area * (*(*exfil).btmExfil).Ks * Adjust.hydconFactor;
            } else {
                if depth > (*exfil).bankMaxDepth {
                    depth = depth - (*exfil).bankMaxDepth
                        + ((*exfil).bankMaxDepth - (*exfil).bankMinDepth) / 2.0f64;
                } else {
                    depth = (depth - (*exfil).bankMinDepth) / 2.0f64;
                }
                exfilRate += area
                    * grnampt_getInfil(
                        (*exfil).bankExfil,
                        tStep,
                        0.0f64,
                        depth,
                        MOD_GREEN_AMPT as ::core::ffi::c_int,
                    );
            }
        }
    }
    return exfilRate;
}
unsafe extern "C" fn createStorageExfil(
    mut k: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut exfil: *mut TExfil = ::core::ptr::null_mut::<TExfil>();
    exfil = (*Storage.offset(k as isize)).exfil;
    if exfil.is_null() {
        exfil = malloc(::core::mem::size_of::<TExfil>() as size_t) as *mut TExfil;
        if exfil.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        let ref mut fresh0 = (*Storage.offset(k as isize)).exfil;
        *fresh0 = exfil;
        (*exfil).btmExfil = ::core::ptr::null_mut::<TGrnAmpt>();
        (*exfil).bankExfil = ::core::ptr::null_mut::<TGrnAmpt>();
        (*exfil).btmExfil = malloc(::core::mem::size_of::<TGrnAmpt>() as size_t) as *mut TGrnAmpt;
        if (*exfil).btmExfil.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        (*exfil).bankExfil = malloc(::core::mem::size_of::<TGrnAmpt>() as size_t) as *mut TGrnAmpt;
        if (*exfil).bankExfil.is_null() {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    if grnampt_setParams((*exfil).btmExfil, x) == 0 {
        return error_setInpError(
            ERR_NUMBER as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    grnampt_setParams((*exfil).bankExfil, x);
    return 0 as ::core::ffi::c_int;
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
