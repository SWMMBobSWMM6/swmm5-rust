#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut Transect: *mut TTransect;
    fn project_findID(
        type_0: ::core::ffi::c_int,
        id: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDouble(s: *mut ::core::ffi::c_char, y: *mut ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut TransectKeyWords: [*mut ::core::ffi::c_char; 0];
}
pub type size_t = usize;
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_TRANSECT_SEQUENCE: ErrorType = 221;
pub const ERR_TRANSECT_UNKNOWN: ErrorType = 219;
pub const ERR_NUMBER: ErrorType = 211;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_TRANSECT_NO_DEPTH: ErrorType = 231;
pub const ERR_TRANSECT_OVERBANK: ErrorType = 229;
pub const ERR_TRANSECT_MANNING: ErrorType = 227;
pub const ERR_TRANSECT_TOO_MANY: ErrorType = 225;
pub const ERR_TRANSECT_TOO_FEW: ErrorType = 223;
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
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PHI: ::core::ffi::c_double = 1.486f64;
pub const N_TRANSECT_TBL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const MAXSTATION: ::core::ffi::c_int = 1500 as ::core::ffi::c_int;
static mut Ntransects: ::core::ffi::c_int = 0;
static mut Nstations: ::core::ffi::c_int = 0;
static mut Station: [::core::ffi::c_double; 1501] = [0.; 1501];
static mut Elev: [::core::ffi::c_double; 1501] = [0.; 1501];
static mut Nleft: ::core::ffi::c_double = 0.;
static mut Nright: ::core::ffi::c_double = 0.;
static mut Nchannel: ::core::ffi::c_double = 0.;
static mut Xleftbank: ::core::ffi::c_double = 0.;
static mut Xrightbank: ::core::ffi::c_double = 0.;
static mut Xfactor: ::core::ffi::c_double = 0.;
static mut Yfactor: ::core::ffi::c_double = 0.;
static mut Lfactor: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn transect_create(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    Ntransects = n;
    if n == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    Transect = calloc(
        Ntransects as size_t,
        ::core::mem::size_of::<TTransect>() as size_t,
    ) as *mut TTransect;
    if Transect.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    Nchannel = 0.0f64;
    Nleft = 0.0f64;
    Nright = 0.0f64;
    Nstations = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn transect_delete() {
    if Ntransects == 0 as ::core::ffi::c_int {
        return;
    }
    if !Transect.is_null() {
        free(Transect as *mut ::core::ffi::c_void);
        Transect = ::core::ptr::null_mut::<TTransect>();
    }
    Ntransects = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn transect_readParams(
    mut count: *mut ::core::ffi::c_int,
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut index: ::core::ffi::c_int = *count;
    let mut errcode: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 10] = [0.; 10];
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    k = findmatch(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        &raw mut TransectKeyWords as *mut *mut ::core::ffi::c_char,
    );
    if k < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    match k {
        0 => {
            transect_validate(index - 1 as ::core::ffi::c_int);
            if ntoks < 4 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            i = 1 as ::core::ffi::c_int;
            while i <= 3 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset(i as isize),
                    (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
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
            return setManning(&raw mut x as *mut ::core::ffi::c_double);
        }
        1 => {
            if ntoks < 10 as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            id = project_findID(
                TRANSECT as ::core::ffi::c_int,
                *tok.offset(1 as ::core::ffi::c_int as isize),
            );
            if id.is_null() {
                return error_setInpError(
                    ERR_NAME as ::core::ffi::c_int,
                    *tok.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            i = 2 as ::core::ffi::c_int;
            while i < 10 as ::core::ffi::c_int {
                if getDouble(
                    *tok.offset(i as isize),
                    (&raw mut x as *mut ::core::ffi::c_double).offset(i as isize)
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
            *count = index + 1 as ::core::ffi::c_int;
            return setParams(index, id, &raw mut x as *mut ::core::ffi::c_double);
        }
        2 => {
            if (ntoks - 1 as ::core::ffi::c_int) % 2 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            i = 1 as ::core::ffi::c_int;
            while i < ntoks {
                if getDouble(
                    *tok.offset(i as isize),
                    (&raw mut x as *mut ::core::ffi::c_double)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset(i as isize),
                    );
                }
                if getDouble(
                    *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
                    (&raw mut x as *mut ::core::ffi::c_double)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_double,
                ) == 0
                {
                    return error_setInpError(
                        ERR_NUMBER as ::core::ffi::c_int,
                        *tok.offset((i + 1 as ::core::ffi::c_int) as isize),
                    );
                }
                errcode = addStation(
                    x[1 as ::core::ffi::c_int as usize],
                    x[2 as ::core::ffi::c_int as usize],
                );
                if errcode != 0 {
                    return errcode;
                }
                i += 2 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn transect_validate(mut j: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ymin: ::core::ffi::c_double = 0.;
    let mut ymax: ::core::ffi::c_double = 0.;
    let mut oldNchannel: ::core::ffi::c_double = Nchannel;
    if j < 0 as ::core::ffi::c_int || j >= Ntransects {
        return;
    }
    if Nstations < 2 as ::core::ffi::c_int {
        report_writeErrorMsg(
            ERR_TRANSECT_TOO_FEW as ::core::ffi::c_int,
            (*Transect.offset(j as isize)).ID,
        );
        return;
    }
    if Nstations >= MAXSTATION {
        report_writeErrorMsg(
            ERR_TRANSECT_TOO_MANY as ::core::ffi::c_int,
            (*Transect.offset(j as isize)).ID,
        );
        return;
    }
    if Nchannel <= 0.0f64 {
        report_writeErrorMsg(
            ERR_TRANSECT_MANNING as ::core::ffi::c_int,
            (*Transect.offset(j as isize)).ID,
        );
        return;
    }
    if Xleftbank > Xrightbank {
        report_writeErrorMsg(
            ERR_TRANSECT_OVERBANK as ::core::ffi::c_int,
            (*Transect.offset(j as isize)).ID,
        );
        return;
    }
    Nchannel = Nchannel * sqrt(Lfactor);
    (*Transect.offset(j as isize)).lengthFactor = Lfactor;
    ymax = Elev[1 as ::core::ffi::c_int as usize];
    ymin = Elev[1 as ::core::ffi::c_int as usize];
    i = 2 as ::core::ffi::c_int;
    while i <= Nstations {
        ymax = if Elev[i as usize] >= ymax {
            Elev[i as usize]
        } else {
            ymax
        };
        ymin = if Elev[i as usize] <= ymin {
            Elev[i as usize]
        } else {
            ymin
        };
        i += 1;
    }
    if ymin >= ymax {
        report_writeErrorMsg(
            ERR_TRANSECT_NO_DEPTH as ::core::ffi::c_int,
            (*Transect.offset(j as isize)).ID,
        );
        return;
    }
    Station[0 as ::core::ffi::c_int as usize] = Station[1 as ::core::ffi::c_int as usize];
    Elev[0 as ::core::ffi::c_int as usize] = ymax;
    Nstations += 1;
    Station[Nstations as usize] = Station[(Nstations - 1 as ::core::ffi::c_int) as usize];
    Elev[Nstations as usize] = Elev[0 as ::core::ffi::c_int as usize];
    (*Transect.offset(j as isize)).nTbl = N_TRANSECT_TBL;
    createTables(Transect.offset(j as isize) as *mut TTransect, ymin, ymax);
    (*Transect.offset(j as isize)).roughness = oldNchannel;
}
unsafe extern "C" fn createTables(
    mut transect: *mut TTransect,
    mut ymin: ::core::ffi::c_double,
    mut ymax: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut nLast: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    (*transect).yFull = ymax - ymin;
    (*transect).wMax = 0.0f64;
    (*transect).areaTbl[0 as ::core::ffi::c_int as usize] = 0.0f64;
    (*transect).hradTbl[0 as ::core::ffi::c_int as usize] = 0.0f64;
    (*transect).widthTbl[0 as ::core::ffi::c_int as usize] = 0.0f64;
    dy = (ymax - ymin)
        / ((*transect).nTbl as ::core::ffi::c_double
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double);
    y = ymin;
    i = 1 as ::core::ffi::c_int;
    while i < (*transect).nTbl {
        y += dy;
        (*transect).areaTbl[i as usize] = 0.0f64;
        (*transect).hradTbl[i as usize] = 0.0f64;
        (*transect).widthTbl[i as usize] = 0.0f64;
        getGeometry(transect, i, y);
        i += 1;
    }
    setMaxSectionFactor(transect);
    nLast = (*transect).nTbl - 1 as ::core::ffi::c_int;
    (*transect).aFull = (*transect).areaTbl[nLast as usize];
    (*transect).rFull = (*transect).hradTbl[nLast as usize];
    (*transect).wMax = (*transect).widthTbl[nLast as usize];
    i = 1 as ::core::ffi::c_int;
    while i <= nLast {
        (*transect).areaTbl[i as usize] /= (*transect).aFull;
        (*transect).hradTbl[i as usize] /= (*transect).rFull;
        (*transect).widthTbl[i as usize] /= (*transect).wMax;
        i += 1;
    }
    (*transect).widthTbl[0 as ::core::ffi::c_int as usize] =
        (*transect).widthTbl[1 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn setManning(mut n: *mut ::core::ffi::c_double) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 1 as ::core::ffi::c_int;
    while i <= 3 as ::core::ffi::c_int {
        if *n.offset(i as isize) < 0.0f64 {
            return ERR_NUMBER as ::core::ffi::c_int;
        }
        i += 1;
    }
    if *n.offset(1 as ::core::ffi::c_int as isize) > 0.0f64 {
        Nleft = *n.offset(1 as ::core::ffi::c_int as isize);
    }
    if *n.offset(2 as ::core::ffi::c_int as isize) > 0.0f64 {
        Nright = *n.offset(2 as ::core::ffi::c_int as isize);
    }
    if *n.offset(3 as ::core::ffi::c_int as isize) > 0.0f64 {
        Nchannel = *n.offset(3 as ::core::ffi::c_int as isize);
    }
    if Nleft == 0.0f64 {
        Nleft = Nchannel;
    }
    if Nright == 0.0f64 {
        Nright = Nchannel;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn setParams(
    mut j: ::core::ffi::c_int,
    mut id: *mut ::core::ffi::c_char,
    mut x: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if j < 0 as ::core::ffi::c_int || j >= Ntransects {
        return ERR_NUMBER as ::core::ffi::c_int;
    }
    let ref mut fresh0 = (*Transect.offset(j as isize)).ID;
    *fresh0 = id;
    Xleftbank = *x.offset(3 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
    Xrightbank = *x.offset(4 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
    Lfactor = *x.offset(7 as ::core::ffi::c_int as isize);
    if Lfactor == 0.0f64 {
        Lfactor = 1.0f64;
    }
    Xfactor = *x.offset(8 as ::core::ffi::c_int as isize);
    if Xfactor == 0.0f64 {
        Xfactor = 1.0f64;
    }
    Xleftbank *= Xfactor;
    Xrightbank *= Xfactor;
    Yfactor = *x.offset(9 as ::core::ffi::c_int as isize) / UCF(LENGTH as ::core::ffi::c_int);
    Nstations = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn addStation(
    mut y: ::core::ffi::c_double,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if Nstations < 0 as ::core::ffi::c_int {
        return ERR_TRANSECT_UNKNOWN as ::core::ffi::c_int;
    }
    Nstations += 1;
    if Nstations >= MAXSTATION {
        return 0 as ::core::ffi::c_int;
    }
    Station[Nstations as usize] = x * Xfactor / UCF(LENGTH as ::core::ffi::c_int);
    Elev[Nstations as usize] = (y + Yfactor) / UCF(LENGTH as ::core::ffi::c_int);
    if Nstations > 1 as ::core::ffi::c_int
        && Station[Nstations as usize] < Station[(Nstations - 1 as ::core::ffi::c_int) as usize]
    {
        return ERR_TRANSECT_SEQUENCE as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn getGeometry(
    mut transect: *mut TTransect,
    mut i: ::core::ffi::c_int,
    mut y: ::core::ffi::c_double,
) {
    let mut k: ::core::ffi::c_int = 0;
    let mut ylo: ::core::ffi::c_double = 0.;
    let mut yhi: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut wp: ::core::ffi::c_double = 0.;
    let mut wpSum: ::core::ffi::c_double = 0.;
    let mut a: ::core::ffi::c_double = 0.;
    let mut aSum: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut qSum: ::core::ffi::c_double = 0.;
    let mut findFlow: ::core::ffi::c_int = 0;
    wpSum = 0.0f64;
    aSum = 0.0f64;
    qSum = 0.0f64;
    k = 1 as ::core::ffi::c_int;
    while k <= Nstations {
        if Elev[(k - 1 as ::core::ffi::c_int) as usize] >= Elev[k as usize] {
            yhi = Elev[(k - 1 as ::core::ffi::c_int) as usize];
            ylo = Elev[k as usize];
        } else {
            yhi = Elev[k as usize];
            ylo = Elev[(k - 1 as ::core::ffi::c_int) as usize];
        }
        if !(ylo >= y) {
            getSliceGeom(k, y, ylo, yhi, &raw mut w, &raw mut a, &raw mut wp);
            wpSum += wp;
            aSum += a;
            (*transect).areaTbl[i as usize] += a;
            (*transect).widthTbl[i as usize] += w;
            if Elev[k as usize] >= y {
                findFlow = TRUE;
            } else {
                findFlow = FALSE;
            }
            q = getFlow(k, aSum, wpSum, findFlow);
            if q > 0.0f64 {
                qSum += q;
                aSum = 0.0f64;
                wpSum = 0.0f64;
            }
        }
        k += 1;
    }
    aSum = (*transect).areaTbl[i as usize];
    if aSum == 0.0f64 {
        (*transect).hradTbl[i as usize] =
            (*transect).hradTbl[(i - 1 as ::core::ffi::c_int) as usize];
    } else {
        (*transect).hradTbl[i as usize] = pow(qSum * Nchannel / 1.49f64 / aSum, 1.5f64);
    };
}
unsafe extern "C" fn getSliceGeom(
    mut k: ::core::ffi::c_int,
    mut y: ::core::ffi::c_double,
    mut ylo: ::core::ffi::c_double,
    mut yhi: ::core::ffi::c_double,
    mut w: *mut ::core::ffi::c_double,
    mut a: *mut ::core::ffi::c_double,
    mut wp: *mut ::core::ffi::c_double,
) {
    let mut width: ::core::ffi::c_double = 0.;
    let mut ratio: ::core::ffi::c_double = 0.;
    width = fabs(Station[k as usize] - Station[(k - 1 as ::core::ffi::c_int) as usize]);
    *w = width;
    *wp = sqrt(width * width + (yhi - ylo) * (yhi - ylo));
    *a = 0.0f64;
    if y > yhi {
        *a = width * (y - yhi + (y - ylo)) / 2.0f64;
    } else if yhi > ylo {
        ratio = (y - ylo) / (yhi - ylo);
        *a = width * (yhi - ylo) / 2.0f64 * ratio * ratio;
        *w *= ratio;
        *wp *= ratio;
    }
}
unsafe extern "C" fn getFlow(
    mut k: ::core::ffi::c_int,
    mut a: ::core::ffi::c_double,
    mut wp: ::core::ffi::c_double,
    mut findFlow: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut n: ::core::ffi::c_double = 0.;
    if findFlow == FALSE {
        if k == Nstations - 1 as ::core::ffi::c_int {
            findFlow = TRUE;
        } else if Station[k as usize] == Xleftbank {
            if Nleft != Nchannel
                && Station[k as usize] != Station[(k - 1 as ::core::ffi::c_int) as usize]
            {
                findFlow = TRUE;
            }
        } else if Station[k as usize] == Xrightbank {
            if Nright != Nchannel
                && Station[k as usize] != Station[(k + 1 as ::core::ffi::c_int) as usize]
            {
                findFlow = TRUE;
            }
        }
    }
    if findFlow != 0 {
        n = Nchannel;
        if Station[(k - 1 as ::core::ffi::c_int) as usize] < Xleftbank {
            n = Nleft;
        }
        if Station[k as usize] > Xrightbank {
            n = Nright;
        }
        return PHI / n * a * pow(a / wp, 2.0f64 / 3.0f64);
    }
    return 0.0f64;
}
unsafe extern "C" fn setMaxSectionFactor(mut transect: *mut TTransect) {
    let mut i: ::core::ffi::c_int = 0;
    let mut sf: ::core::ffi::c_double = 0.;
    (*transect).aMax = 0.0f64;
    (*transect).sMax = 0.0f64;
    i = 1 as ::core::ffi::c_int;
    while i < (*transect).nTbl {
        sf =
            (*transect).areaTbl[i as usize] * pow((*transect).hradTbl[i as usize], 2.0f64 / 3.0f64);
        if sf > (*transect).sMax {
            (*transect).sMax = sf;
            (*transect).aMax = (*transect).areaTbl[i as usize];
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn transect_createStreetTransect(mut street: *mut TStreet) {
    let mut ymin: ::core::ffi::c_double = 0.;
    let mut ymax: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut y3: ::core::ffi::c_double = 0.;
    let mut y4: ::core::ffi::c_double = 0.;
    let mut w1: ::core::ffi::c_double = 0.;
    let mut w2: ::core::ffi::c_double = 0.;
    let mut w3: ::core::ffi::c_double = 0.;
    let mut w4: ::core::ffi::c_double = 0.;
    ymin = 0.0f64;
    w1 = (*street).backWidth;
    w2 = (*street).gutterWidth;
    w3 = (*street).width;
    w4 = w3 - w2;
    y3 = (*street).gutterDepression + (*street).slope * w2;
    y1 = (*street).curbHeight + (*street).gutterDepression;
    ymax = (*street).backSlope * (*street).backWidth + y1;
    y4 = y3 + (*street).slope * w4;
    ymax = if ymax >= y4 { ymax } else { y4 };
    Station[0 as ::core::ffi::c_int as usize] = 0.0f64;
    Elev[0 as ::core::ffi::c_int as usize] = ymax;
    Station[1 as ::core::ffi::c_int as usize] = w1;
    Elev[1 as ::core::ffi::c_int as usize] = y1;
    Station[2 as ::core::ffi::c_int as usize] = w1;
    Elev[2 as ::core::ffi::c_int as usize] = 0.0f64;
    Station[3 as ::core::ffi::c_int as usize] = w1 + w2;
    Elev[3 as ::core::ffi::c_int as usize] = y3;
    Station[4 as ::core::ffi::c_int as usize] = w1 + w3;
    Elev[4 as ::core::ffi::c_int as usize] = y4;
    if (*street).sides == 1 as ::core::ffi::c_int {
        Station[5 as ::core::ffi::c_int as usize] = Station[4 as ::core::ffi::c_int as usize];
        Elev[5 as ::core::ffi::c_int as usize] = ymax;
        Nstations = 5 as ::core::ffi::c_int;
        (*street).transect.nTbl = N_TRANSECT_TBL;
    } else {
        Station[5 as ::core::ffi::c_int as usize] = Station[4 as ::core::ffi::c_int as usize] + w4;
        Elev[5 as ::core::ffi::c_int as usize] = y3;
        Station[6 as ::core::ffi::c_int as usize] = Station[5 as ::core::ffi::c_int as usize] + w2;
        Elev[6 as ::core::ffi::c_int as usize] = 0.0f64;
        Station[7 as ::core::ffi::c_int as usize] = Station[6 as ::core::ffi::c_int as usize];
        Elev[7 as ::core::ffi::c_int as usize] = y1;
        Station[8 as ::core::ffi::c_int as usize] = Station[7 as ::core::ffi::c_int as usize] + w1;
        Elev[8 as ::core::ffi::c_int as usize] = ymax;
        Nstations = 8 as ::core::ffi::c_int;
        (*street).transect.nTbl = N_TRANSECT_TBL;
    }
    Nchannel = (*street).roughness;
    if (*street).backWidth == 0.0f64 {
        Nleft = Nchannel;
        Nright = Nchannel;
        Xleftbank = Station[0 as ::core::ffi::c_int as usize];
        Xrightbank = Station[Nstations as usize];
    } else {
        Nleft = (*street).backRoughness;
        Nright = Nleft;
        Xleftbank = Station[1 as ::core::ffi::c_int as usize];
        if (*street).sides == 2 as ::core::ffi::c_int {
            Xrightbank = Station[(Nstations - 1 as ::core::ffi::c_int) as usize];
        } else {
            Xrightbank = Station[Nstations as usize];
        }
    }
    createTables(&raw mut (*street).transect, ymin, ymax);
    (*street).transect.roughness = (*street).roughness;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
