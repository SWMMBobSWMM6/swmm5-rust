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
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
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
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    static mut Frain: TFile;
    static mut Frunoff: TFile;
    static mut Foutflows: TFile;
    static mut Finflows: TFile;
    static mut Fhotstart2: TFile;
    static mut Fhotstart1: TFile;
    static mut Frdii: TFile;
    static mut Title: [[::core::ffi::c_char; 1025]; 3];
    static mut FlowUnits: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut RouteModel: ::core::ffi::c_int;
    static mut ReportStep: ::core::ffi::c_int;
    static mut StartDateTime: DateTime;
    static mut ReportStart: DateTime;
    static mut Node: *mut TNode;
    static mut Pollut: *mut TPollut;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn project_createMatrix(
        nrows: ::core::ffi::c_int,
        ncols: ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_double;
    fn project_freeMatrix(m: *mut *mut ::core::ffi::c_double);
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
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
    static mut FileModeWords: [*mut ::core::ffi::c_char; 0];
    static mut FileTypeWords: [*mut ::core::ffi::c_char; 0];
    static mut FlowUnitWords: [*mut ::core::ffi::c_char; 0];
    static mut QualUnitsWords: [*mut ::core::ffi::c_char; 0];
    static mut Qcf: [::core::ffi::c_double; 0];
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
pub type FileType = ::core::ffi::c_uint;
pub const OUTFLOWS_FILE: FileType = 5;
pub const INFLOWS_FILE: FileType = 4;
pub const RDII_FILE: FileType = 3;
pub const HOTSTART_FILE: FileType = 2;
pub const RUNOFF_FILE: FileType = 1;
pub const RAINFALL_FILE: FileType = 0;
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
pub type RouteModelType = ::core::ffi::c_uint;
pub const DW: RouteModelType = 4;
pub const EKW: RouteModelType = 3;
pub const KW: RouteModelType = 2;
pub const SF: RouteModelType = 1;
pub const NO_ROUTING: RouteModelType = 0;
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
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_ROUTING_FILE_FORMAT: ErrorType = 353;
pub const ERR_ROUTING_FILE_NOMATCH: ErrorType = 355;
pub const ERR_ROUTING_FILE_OPEN: ErrorType = 351;
pub const ERR_ROUTING_FILE_NAMES: ErrorType = 357;
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
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut ::core::ffi::c_char);
}
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXFNAME: ::core::ffi::c_int = 259 as ::core::ffi::c_int;
pub const SEPSTR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b" \t\n\r\0") };
pub const NO_DATE: ::core::ffi::c_int = -(693594 as ::core::ffi::c_int);
static mut IfaceFlowUnits: ::core::ffi::c_int = 0;
static mut IfaceStep: ::core::ffi::c_int = 0;
static mut NumIfacePolluts: ::core::ffi::c_int = 0;
static mut IfacePolluts: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut NumIfaceNodes: ::core::ffi::c_int = 0;
static mut IfaceNodes: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut OldIfaceValues: *mut *mut ::core::ffi::c_double =
    ::core::ptr::null::<*mut ::core::ffi::c_double>() as *mut *mut ::core::ffi::c_double;
static mut NewIfaceValues: *mut *mut ::core::ffi::c_double =
    ::core::ptr::null::<*mut ::core::ffi::c_double>() as *mut *mut ::core::ffi::c_double;
static mut IfaceFrac: ::core::ffi::c_double = 0.;
static mut OldIfaceDate: DateTime = 0.;
static mut NewIfaceDate: DateTime = 0.;
#[no_mangle]
pub unsafe extern "C" fn iface_readFileParams(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_char = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut fname: [::core::ffi::c_char; 260] = [0; 260];
    if ntoks < 2 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_ITEMS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    k = findmatch(
        *tok.offset(0 as ::core::ffi::c_int as isize),
        &raw mut FileModeWords as *mut *mut ::core::ffi::c_char,
    ) as ::core::ffi::c_char;
    if (k as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    j = findmatch(
        *tok.offset(1 as ::core::ffi::c_int as isize),
        &raw mut FileTypeWords as *mut *mut ::core::ffi::c_char,
    );
    if j < 0 as ::core::ffi::c_int {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    if ntoks < 3 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    sstrncpy(
        &raw mut fname as *mut ::core::ffi::c_char,
        *tok.offset(2 as ::core::ffi::c_int as isize),
        MAXFNAME as size_t,
    );
    match j {
        0 => {
            Frain.mode = k;
            sstrncpy(
                &raw mut Frain.name as *mut ::core::ffi::c_char,
                addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
                MAXFNAME as size_t,
            );
        }
        1 => {
            Frunoff.mode = k;
            sstrncpy(
                &raw mut Frunoff.name as *mut ::core::ffi::c_char,
                addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
                MAXFNAME as size_t,
            );
        }
        2 => {
            if k as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
                Fhotstart1.mode = k;
                sstrncpy(
                    &raw mut Fhotstart1.name as *mut ::core::ffi::c_char,
                    addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
                    MAXFNAME as size_t,
                );
            } else if k as ::core::ffi::c_int == SAVE_FILE as ::core::ffi::c_int {
                Fhotstart2.mode = k;
                sstrncpy(
                    &raw mut Fhotstart2.name as *mut ::core::ffi::c_char,
                    addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
                    MAXFNAME as size_t,
                );
            }
        }
        3 => {
            Frdii.mode = k;
            sstrncpy(
                &raw mut Frdii.name as *mut ::core::ffi::c_char,
                &raw mut fname as *mut ::core::ffi::c_char,
                MAXFNAME as size_t,
            );
        }
        4 => {
            if k as ::core::ffi::c_int != USE_FILE as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            Finflows.mode = k;
            sstrncpy(
                &raw mut Finflows.name as *mut ::core::ffi::c_char,
                addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
                MAXFNAME as size_t,
            );
        }
        5 => {
            if k as ::core::ffi::c_int != SAVE_FILE as ::core::ffi::c_int {
                return error_setInpError(
                    ERR_ITEMS as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
            }
            Foutflows.mode = k;
            sstrncpy(
                &raw mut Foutflows.name as *mut ::core::ffi::c_char,
                addAbsolutePath(&raw mut fname as *mut ::core::ffi::c_char),
                MAXFNAME as size_t,
            );
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn iface_openRoutingFiles() {
    NumIfacePolluts = 0 as ::core::ffi::c_int;
    IfacePolluts = ::core::ptr::null_mut::<::core::ffi::c_int>();
    NumIfaceNodes = 0 as ::core::ffi::c_int;
    IfaceNodes = ::core::ptr::null_mut::<::core::ffi::c_int>();
    OldIfaceValues = ::core::ptr::null_mut::<*mut ::core::ffi::c_double>();
    NewIfaceValues = ::core::ptr::null_mut::<*mut ::core::ffi::c_double>();
    if Foutflows.mode as ::core::ffi::c_int != NO_FILE as ::core::ffi::c_int
        && Finflows.mode as ::core::ffi::c_int != NO_FILE as ::core::ffi::c_int
    {
        if strcomp(
            &raw mut Foutflows.name as *mut ::core::ffi::c_char,
            &raw mut Finflows.name as *mut ::core::ffi::c_char,
        ) != 0
        {
            report_writeErrorMsg(
                ERR_ROUTING_FILE_NAMES as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return;
        }
    }
    if Foutflows.mode as ::core::ffi::c_int == SAVE_FILE as ::core::ffi::c_int {
        openFileForOutput();
    }
    if Finflows.mode as ::core::ffi::c_int == USE_FILE as ::core::ffi::c_int {
        openFileForInput();
    }
}
#[no_mangle]
pub unsafe extern "C" fn iface_closeRoutingFiles() {
    if !IfacePolluts.is_null() {
        free(IfacePolluts as *mut ::core::ffi::c_void);
        IfacePolluts = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !IfaceNodes.is_null() {
        free(IfaceNodes as *mut ::core::ffi::c_void);
        IfaceNodes = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !OldIfaceValues.is_null() {
        project_freeMatrix(OldIfaceValues);
    }
    if !NewIfaceValues.is_null() {
        project_freeMatrix(NewIfaceValues);
    }
    if !Finflows.file.is_null() {
        fclose(Finflows.file);
    }
    if !Foutflows.file.is_null() {
        fclose(Foutflows.file);
    }
}
#[no_mangle]
pub unsafe extern "C" fn iface_getNumIfaceNodes(mut currentDate: DateTime) -> ::core::ffi::c_int {
    if OldIfaceDate > currentDate {
        return 0 as ::core::ffi::c_int;
    }
    while NewIfaceDate < currentDate && NewIfaceDate != NO_DATE as DateTime {
        setOldIfaceValues();
        readNewIfaceValues();
    }
    if NewIfaceDate == NO_DATE as DateTime {
        return 0 as ::core::ffi::c_int;
    }
    IfaceFrac =
        ((currentDate - OldIfaceDate) / (NewIfaceDate - OldIfaceDate)) as ::core::ffi::c_double;
    IfaceFrac = if 0.0f64 >= IfaceFrac {
        0.0f64
    } else {
        IfaceFrac
    };
    IfaceFrac = if IfaceFrac <= 1.0f64 {
        IfaceFrac
    } else {
        1.0f64
    };
    return NumIfaceNodes;
}
#[no_mangle]
pub unsafe extern "C" fn iface_getIfaceNode(mut index: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if index >= 0 as ::core::ffi::c_int && index < NumIfaceNodes {
        return *IfaceNodes.offset(index as isize);
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn iface_getIfaceFlow(
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut q1: ::core::ffi::c_double = 0.;
    let mut q2: ::core::ffi::c_double = 0.;
    if index >= 0 as ::core::ffi::c_int && index < NumIfaceNodes {
        q1 = *(*OldIfaceValues.offset(index as isize)).offset(0 as ::core::ffi::c_int as isize);
        q2 = *(*NewIfaceValues.offset(index as isize)).offset(0 as ::core::ffi::c_int as isize);
        return (1.0f64 - IfaceFrac) * q1 + IfaceFrac * q2;
    } else {
        return 0.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn iface_getIfaceQual(
    mut index: ::core::ffi::c_int,
    mut pollut: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_double = 0.;
    let mut c2: ::core::ffi::c_double = 0.;
    if index >= 0 as ::core::ffi::c_int && index < NumIfaceNodes {
        j = *IfacePolluts.offset(pollut as isize);
        if j < 0 as ::core::ffi::c_int {
            return 0.0f64;
        }
        c1 = *(*OldIfaceValues.offset(index as isize))
            .offset((j + 1 as ::core::ffi::c_int) as isize);
        c2 = *(*NewIfaceValues.offset(index as isize))
            .offset((j + 1 as ::core::ffi::c_int) as isize);
        return (1.0f64 - IfaceFrac) * c1 + IfaceFrac * c2;
    } else {
        return 0.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn iface_saveOutletResults(mut reportDate: DateTime, mut file: *mut FILE) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut yr: ::core::ffi::c_int = 0;
    let mut mon: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut hr: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut sec: ::core::ffi::c_int = 0;
    let mut theDate: [::core::ffi::c_char; 26] = [0; 26];
    datetime_decodeDate(reportDate, &raw mut yr, &raw mut mon, &raw mut day);
    datetime_decodeTime(reportDate, &raw mut hr, &raw mut min, &raw mut sec);
    snprintf(
        &raw mut theDate as *mut ::core::ffi::c_char,
        26 as size_t,
        b" %04d %02d  %02d  %02d  %02d  %02d \0" as *const u8 as *const ::core::ffi::c_char,
        yr,
        mon,
        day,
        hr,
        min,
        sec,
    );
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !(isOutletNode(i) == 0) {
            fprintf(
                file,
                b"\n%-16s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Node.offset(i as isize)).ID,
            );
            fprintf(
                file,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut theDate as *mut ::core::ffi::c_char,
            );
            fprintf(
                file,
                b" %-10f\0" as *const u8 as *const ::core::ffi::c_char,
                (*Node.offset(i as isize)).inflow * UCF(FLOW as ::core::ffi::c_int),
            );
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                fprintf(
                    file,
                    b" %-10f\0" as *const u8 as *const ::core::ffi::c_char,
                    *(*Node.offset(i as isize)).newQual.offset(p as isize),
                );
                p += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn openFileForOutput() {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    Foutflows.file = fopen(
        &raw mut Foutflows.name as *mut ::core::ffi::c_char,
        b"wt\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Foutflows.file.is_null() {
        report_writeErrorMsg(
            ERR_ROUTING_FILE_OPEN as ::core::ffi::c_int,
            &raw mut Foutflows.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    fprintf(
        Foutflows.file,
        b"SWMM5 Interface File\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        Foutflows.file,
        b"\n%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut *(&raw mut Title as *mut [::core::ffi::c_char; 1025])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
    );
    fprintf(
        Foutflows.file,
        b"\n%-4d - reporting time step in sec\0" as *const u8 as *const ::core::ffi::c_char,
        ReportStep,
    );
    fprintf(
        Foutflows.file,
        b"\n%-4d - number of constituents as listed below:\0" as *const u8
            as *const ::core::ffi::c_char,
        Nobjects[POLLUT as ::core::ffi::c_int as usize] + 1 as ::core::ffi::c_int,
    );
    fprintf(
        Foutflows.file,
        b"\nFLOW %s\0" as *const u8 as *const ::core::ffi::c_char,
        *(&raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char).offset(FlowUnits as isize),
    );
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        fprintf(
            Foutflows.file,
            b"\n%s %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Pollut.offset(i as isize)).ID,
            *(&raw mut QualUnitsWords as *mut *mut ::core::ffi::c_char)
                .offset((*Pollut.offset(i as isize)).units as isize),
        );
        i += 1;
    }
    n = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if isOutletNode(i) != 0 {
            n += 1;
        }
        i += 1;
    }
    fprintf(
        Foutflows.file,
        b"\n%-4d - number of nodes as listed below:\0" as *const u8 as *const ::core::ffi::c_char,
        n,
    );
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if isOutletNode(i) != 0 {
            fprintf(
                Foutflows.file,
                b"\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Node.offset(i as isize)).ID,
            );
        }
        i += 1;
    }
    fprintf(
        Foutflows.file,
        b"\nNode             Year Mon Day Hr  Min Sec FLOW      \0" as *const u8
            as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        fprintf(
            Foutflows.file,
            b" %-10s\0" as *const u8 as *const ::core::ffi::c_char,
            (*Pollut.offset(i as isize)).ID,
        );
        i += 1;
    }
    if ReportStart == StartDateTime {
        iface_saveOutletResults(ReportStart, Foutflows.file);
    }
}
unsafe extern "C" fn openFileForInput() {
    let mut err: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut s: [::core::ffi::c_char; 1025] = [0; 1025];
    Finflows.file = fopen(
        &raw mut Finflows.name as *mut ::core::ffi::c_char,
        b"rt\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if Finflows.file.is_null() {
        report_writeErrorMsg(
            ERR_ROUTING_FILE_OPEN as ::core::ffi::c_int,
            &raw mut Finflows.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finflows.file,
    );
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut s as *mut ::core::ffi::c_char,
    ) == 0
        || strcomp(
            &raw mut s as *mut ::core::ffi::c_char,
            b"SWMM5\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        report_writeErrorMsg(
            ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int,
            &raw mut Finflows.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finflows.file,
    );
    IfaceStep = 0 as ::core::ffi::c_int;
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finflows.file,
    );
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut IfaceStep,
    ) == 0
        || IfaceStep <= 0 as ::core::ffi::c_int
    {
        report_writeErrorMsg(
            ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int,
            &raw mut Finflows.name as *mut ::core::ffi::c_char,
        );
        return;
    }
    err = getIfaceFilePolluts();
    if err > 0 as ::core::ffi::c_int {
        report_writeErrorMsg(err, &raw mut Finflows.name as *mut ::core::ffi::c_char);
        return;
    }
    err = getIfaceFileNodes();
    if err > 0 as ::core::ffi::c_int {
        report_writeErrorMsg(err, &raw mut Finflows.name as *mut ::core::ffi::c_char);
        return;
    }
    OldIfaceValues = project_createMatrix(NumIfaceNodes, 1 as ::core::ffi::c_int + NumIfacePolluts);
    NewIfaceValues = project_createMatrix(NumIfaceNodes, 1 as ::core::ffi::c_int + NumIfacePolluts);
    if OldIfaceValues.is_null() || NewIfaceValues.is_null() {
        report_writeErrorMsg(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    readNewIfaceValues();
    OldIfaceDate = NewIfaceDate;
}
unsafe extern "C" fn getIfaceFilePolluts() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut s1: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut s2: [::core::ffi::c_char; 1025] = [0; 1025];
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finflows.file,
    );
    NumIfacePolluts = -(1 as ::core::ffi::c_int);
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut NumIfacePolluts,
    ) != 0
    {
        NumIfacePolluts -= 1;
    }
    if NumIfacePolluts < 0 as ::core::ffi::c_int {
        return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
    }
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finflows.file,
    );
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%s %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut s1 as *mut ::core::ffi::c_char,
        &raw mut s2 as *mut ::core::ffi::c_char,
    ) < 2 as ::core::ffi::c_int
    {
        return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
    }
    if strcomp(
        &raw mut s1 as *mut ::core::ffi::c_char,
        b"FLOW\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
    }
    IfaceFlowUnits = findmatch(
        &raw mut s2 as *mut ::core::ffi::c_char,
        &raw mut FlowUnitWords as *mut *mut ::core::ffi::c_char,
    );
    if IfaceFlowUnits < 0 as ::core::ffi::c_int {
        return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
    }
    if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        IfacePolluts = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        ) as *mut ::core::ffi::c_int;
        if IfacePolluts.is_null() {
            return ERR_MEMORY as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *IfacePolluts.offset(i as isize) = -(1 as ::core::ffi::c_int);
            i += 1;
        }
    }
    if NumIfacePolluts > 0 as ::core::ffi::c_int
        && Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < NumIfacePolluts {
            if feof(Finflows.file) != 0 {
                return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
            }
            fgets(
                &raw mut line as *mut ::core::ffi::c_char,
                MAXLINE,
                Finflows.file,
            );
            if sscanf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"%s %s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut s1 as *mut ::core::ffi::c_char,
                &raw mut s2 as *mut ::core::ffi::c_char,
            ) < 2 as ::core::ffi::c_int
            {
                return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
            }
            j = project_findObject(
                POLLUT as ::core::ffi::c_int,
                &raw mut s1 as *mut ::core::ffi::c_char,
            );
            if !(j < 0 as ::core::ffi::c_int) {
                if strcomp(
                    &raw mut s2 as *mut ::core::ffi::c_char,
                    *(&raw mut QualUnitsWords as *mut *mut ::core::ffi::c_char)
                        .offset((*Pollut.offset(j as isize)).units as isize),
                ) == 0
                {
                    return ERR_ROUTING_FILE_NOMATCH as ::core::ffi::c_int;
                }
                *IfacePolluts.offset(j as isize) = i;
            }
            i += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn getIfaceFileNodes() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut s: [::core::ffi::c_char; 1025] = [0; 1025];
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finflows.file,
    );
    if sscanf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut NumIfaceNodes,
    ) == 0
        || NumIfaceNodes <= 0 as ::core::ffi::c_int
    {
        return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
    }
    IfaceNodes = calloc(
        NumIfaceNodes as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if IfaceNodes.is_null() {
        return ERR_MEMORY as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NumIfaceNodes {
        if feof(Finflows.file) != 0 {
            return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
        }
        *IfaceNodes.offset(i as isize) = 0 as ::core::ffi::c_int;
        fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            MAXLINE,
            Finflows.file,
        );
        if sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s as *mut ::core::ffi::c_char,
        ) == 0
        {
            return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
        }
        *IfaceNodes.offset(i as isize) = project_findObject(
            NODE as ::core::ffi::c_int,
            &raw mut s as *mut ::core::ffi::c_char,
        );
        i += 1;
    }
    if feof(Finflows.file) != 0 {
        return ERR_ROUTING_FILE_FORMAT as ::core::ffi::c_int;
    }
    fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        MAXLINE,
        Finflows.file,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn readNewIfaceValues() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut yr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut day: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut min: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sec: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    NewIfaceDate = NO_DATE as DateTime;
    i = 0 as ::core::ffi::c_int;
    while i < NumIfaceNodes {
        if feof(Finflows.file) != 0 {
            return;
        }
        fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            MAXLINE,
            Finflows.file,
        );
        if strtok(&raw mut line as *mut ::core::ffi::c_char, SEPSTR.as_ptr()).is_null() {
            return;
        }
        s = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            SEPSTR.as_ptr(),
        );
        if s.is_null() {
            return;
        }
        yr = atoi(s);
        s = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            SEPSTR.as_ptr(),
        );
        if s.is_null() {
            return;
        }
        mon = atoi(s);
        s = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            SEPSTR.as_ptr(),
        );
        if s.is_null() {
            return;
        }
        day = atoi(s);
        s = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            SEPSTR.as_ptr(),
        );
        if s.is_null() {
            return;
        }
        hr = atoi(s);
        s = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            SEPSTR.as_ptr(),
        );
        if s.is_null() {
            return;
        }
        min = atoi(s);
        s = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            SEPSTR.as_ptr(),
        );
        if s.is_null() {
            return;
        }
        sec = atoi(s);
        s = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            SEPSTR.as_ptr(),
        );
        if s.is_null() {
            return;
        }
        *(*NewIfaceValues.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize) =
            atof(s) / *(&raw mut Qcf as *mut ::core::ffi::c_double).offset(IfaceFlowUnits as isize);
        j = 1 as ::core::ffi::c_int;
        while j <= NumIfacePolluts {
            s = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                SEPSTR.as_ptr(),
            );
            if s.is_null() {
                return;
            }
            *(*NewIfaceValues.offset(i as isize)).offset(j as isize) = atof(s);
            j += 1;
        }
        i += 1;
    }
    NewIfaceDate = datetime_encodeDate(yr, mon, day) + datetime_encodeTime(hr, min, sec);
}
unsafe extern "C" fn setOldIfaceValues() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    OldIfaceDate = NewIfaceDate;
    i = 0 as ::core::ffi::c_int;
    while i < NumIfaceNodes {
        j = 0 as ::core::ffi::c_int;
        while j < NumIfacePolluts + 1 as ::core::ffi::c_int {
            *(*OldIfaceValues.offset(i as isize)).offset(j as isize) =
                *(*NewIfaceValues.offset(i as isize)).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn isOutletNode(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if RouteModel == DW as ::core::ffi::c_int {
        return ((*Node.offset(i as isize)).type_0 == OUTFALL as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    } else {
        return ((*Node.offset(i as isize)).degree == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
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
