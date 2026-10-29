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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn mathexpr_create(
        s: *mut ::core::ffi::c_char,
        getVar: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>,
    ) -> *mut MathExpr;
    fn mathexpr_eval(
        expr: *mut MathExpr,
        getVal: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_double>,
    ) -> ::core::ffi::c_double;
    fn mathexpr_delete(expr: *mut MathExpr);
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut Node: *mut TNode;
    static mut Storage: *mut TStorage;
    fn project_findObject(
        type_0: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn massbal_addReactedMass(pollut: ::core::ffi::c_int, mass: ::core::ffi::c_double);
    fn node_getSurfArea(
        node: ::core::ffi::c_int,
        depth: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn findmatch(
        s: *mut ::core::ffi::c_char,
        keyword: *mut *mut ::core::ffi::c_char,
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
    fn error_setInpError(
        errcode: ::core::ffi::c_int,
        s: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut ProcessVarWords: [*mut ::core::ffi::c_char; 0];
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
pub type NodeType = ::core::ffi::c_uint;
pub const DIVIDER: NodeType = 3;
pub const STORAGE: NodeType = 2;
pub const OUTFALL: NodeType = 1;
pub const JUNCTION: NodeType = 0;
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
pub type TreatmentType = ::core::ffi::c_uint;
pub const CONCEN: TreatmentType = 1;
pub const REMOVAL: TreatmentType = 0;
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
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_MATH_EXPR: ErrorType = 233;
pub const ERR_KEYWORD: ErrorType = 205;
pub const ERR_NAME: ErrorType = 209;
pub const ERR_ITEMS: ErrorType = 203;
pub const ERR_CYCLIC_TREATMENT: ErrorType = 161;
pub const pvAREA: ProcessVarType = 4;
pub const pvDEPTH: ProcessVarType = 3;
pub const pvFLOW: ProcessVarType = 2;
pub const pvDT: ProcessVarType = 1;
pub const pvHRT: ProcessVarType = 0;
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
pub type ProcessVarType = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ZERO: ::core::ffi::c_double = 1.0E-10f64;
static mut PVMAX: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut ErrCode: ::core::ffi::c_int = 0;
static mut J: ::core::ffi::c_int = 0;
static mut Dt: ::core::ffi::c_double = 0.;
static mut Q: ::core::ffi::c_double = 0.;
static mut V: ::core::ffi::c_double = 0.;
static mut R: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
static mut Cin: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn treatmnt_open() -> ::core::ffi::c_int {
    R = ::core::ptr::null_mut::<::core::ffi::c_double>();
    Cin = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if Nobjects[POLLUT as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        R = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        Cin = calloc(
            Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        ) as *mut ::core::ffi::c_double;
        if R.is_null() || Cin.is_null() {
            report_writeErrorMsg(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return FALSE;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn treatmnt_close() {
    if !R.is_null() {
        free(R as *mut ::core::ffi::c_void);
        R = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    if !Cin.is_null() {
        free(Cin as *mut ::core::ffi::c_void);
        Cin = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn treatmnt_readExpression(
    mut tok: *mut *mut ::core::ffi::c_char,
    mut ntoks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut expr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut equation: *mut MathExpr = ::core::ptr::null_mut::<MathExpr>();
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
    sstrncpy(
        &raw mut s as *mut ::core::ffi::c_char,
        *tok.offset(2 as ::core::ffi::c_int as isize),
        MAXLINE as size_t,
    );
    i = 3 as ::core::ffi::c_int;
    while i < ntoks {
        sstrcat(
            &raw mut s as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            MAXLINE as size_t,
        );
        sstrcat(
            &raw mut s as *mut ::core::ffi::c_char,
            *tok.offset(i as isize),
            MAXLINE as size_t,
        );
        i += 1;
    }
    if (if s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int >= 'a' as i32
        && s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int <= 'z' as i32
    {
        s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
    } else {
        s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    }) == 'R' as i32
    {
        k = 0 as ::core::ffi::c_int;
    } else if (if s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int >= 'a' as i32
        && s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int <= 'z' as i32
    {
        s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
    } else {
        s[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    }) == 'C' as i32
    {
        k = 1 as ::core::ffi::c_int;
    } else {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            *tok.offset(2 as ::core::ffi::c_int as isize),
        );
    }
    expr = strchr(&raw mut s as *mut ::core::ffi::c_char, '=' as i32);
    if expr.is_null() {
        return error_setInpError(
            ERR_KEYWORD as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        expr = expr.offset(1);
    }
    if (*Node.offset(j as isize)).treatment.is_null() {
        if createTreatment(j) == 0 {
            return error_setInpError(
                ERR_MEMORY as ::core::ffi::c_int,
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    equation = mathexpr_create(
        expr,
        Some(
            getVariableIndex
                as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
    );
    if equation.is_null() {
        return error_setInpError(
            ERR_MATH_EXPR as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    if !(*Node.offset(j as isize)).treatment.is_null() {
        (*(*Node.offset(j as isize)).treatment.offset(p as isize)).treatType = k;
        let ref mut fresh0 = (*(*Node.offset(j as isize)).treatment.offset(p as isize)).equation;
        *fresh0 = equation;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn treatmnt_delete(mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    if !(*Node.offset(j as isize)).treatment.is_null() {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            mathexpr_delete((*(*Node.offset(j as isize)).treatment.offset(p as isize)).equation);
            p += 1;
        }
        free((*Node.offset(j as isize)).treatment as *mut ::core::ffi::c_void);
    }
    let ref mut fresh3 = (*Node.offset(j as isize)).treatment;
    *fresh3 = ::core::ptr::null_mut::<TTreatment>();
}
#[no_mangle]
pub unsafe extern "C" fn treatmnt_setInflow(
    mut qIn: ::core::ffi::c_double,
    mut wIn: *mut ::core::ffi::c_double,
) {
    let mut p: ::core::ffi::c_int = 0;
    if qIn > 0.0f64 {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *Cin.offset(p as isize) = *wIn.offset(p as isize) / qIn;
            p += 1;
        }
    } else {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            *Cin.offset(p as isize) = 0.0f64;
            p += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn treatmnt_treat(
    mut j: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
    mut v: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut p: ::core::ffi::c_int = 0;
    let mut cOut: ::core::ffi::c_double = 0.;
    let mut massLost: ::core::ffi::c_double = 0.;
    let mut treatment: *mut TTreatment = ::core::ptr::null_mut::<TTreatment>();
    if (*Node.offset(j as isize)).treatment.is_null() {
        return;
    }
    ErrCode = 0 as ::core::ffi::c_int;
    J = j;
    Dt = tStep;
    Q = q;
    V = v;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        *R.offset(p as isize) = -1.0f64;
        p += 1;
    }
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        treatment = (*Node.offset(j as isize)).treatment.offset(p as isize) as *mut TTreatment;
        if (*treatment).equation.is_null() {
            *R.offset(p as isize) = 0.0f64;
        } else if (*treatment).treatType == REMOVAL as ::core::ffi::c_int && q <= ZERO {
            *R.offset(p as isize) = 0.0f64;
        } else {
            getRemoval(p);
        }
        p += 1;
    }
    if ErrCode == ERR_CYCLIC_TREATMENT as ::core::ffi::c_int {
        report_writeErrorMsg(
            ERR_CYCLIC_TREATMENT as ::core::ffi::c_int,
            (*Node.offset(J as isize)).ID,
        );
    } else {
        p = 0 as ::core::ffi::c_int;
        while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            if !(*R.offset(p as isize) == 0.0f64) {
                treatment =
                    (*Node.offset(j as isize)).treatment.offset(p as isize) as *mut TTreatment;
                if (*treatment).treatType == REMOVAL as ::core::ffi::c_int {
                    if *Cin.offset(p as isize) == 0.0f64 {
                        cOut = *(*Node.offset(j as isize)).newQual.offset(p as isize);
                    } else {
                        cOut = (1.0f64 - *R.offset(p as isize)) * *Cin.offset(p as isize);
                    }
                    cOut = if cOut <= *(*Node.offset(j as isize)).newQual.offset(p as isize) {
                        cOut
                    } else {
                        *(*Node.offset(j as isize)).newQual.offset(p as isize)
                    };
                } else {
                    cOut = (1.0f64 - *R.offset(p as isize))
                        * *(*Node.offset(j as isize)).newQual.offset(p as isize);
                }
                massLost = (*Cin.offset(p as isize) * q * tStep
                    + *(*Node.offset(j as isize)).oldQual.offset(p as isize)
                        * (*Node.offset(j as isize)).oldVolume
                    - cOut * (q * tStep + (*Node.offset(j as isize)).oldVolume))
                    / tStep;
                massLost = if 0.0f64 >= massLost { 0.0f64 } else { massLost };
                massbal_addReactedMass(p, massLost);
                *(*Node.offset(j as isize)).newQual.offset(p as isize) = cOut;
            }
            p += 1;
        }
    };
}
unsafe extern "C" fn createTreatment(mut j: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p: ::core::ffi::c_int = 0;
    let ref mut fresh1 = (*Node.offset(j as isize)).treatment;
    *fresh1 = calloc(
        Nobjects[POLLUT as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TTreatment>() as size_t,
    ) as *mut TTreatment;
    if (*Node.offset(j as isize)).treatment.is_null() {
        return FALSE;
    }
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        let ref mut fresh2 = (*(*Node.offset(j as isize)).treatment.offset(p as isize)).equation;
        *fresh2 = ::core::ptr::null_mut::<MathExpr>();
        p += 1;
    }
    return TRUE;
}
unsafe extern "C" fn getVariableIndex(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = PVMAX;
    k = findmatch(s, &raw mut ProcessVarWords as *mut *mut ::core::ffi::c_char);
    if k >= 0 as ::core::ffi::c_int {
        return k;
    }
    k = project_findObject(POLLUT as ::core::ffi::c_int, s);
    if k >= 0 as ::core::ffi::c_int {
        return k + m;
    }
    if (if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'a' as i32
        && *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'z' as i32
    {
        *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & !(32 as ::core::ffi::c_int)
    } else {
        *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    }) == 'R' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '_' as i32
    {
        k = project_findObject(
            POLLUT as ::core::ffi::c_int,
            s.offset(2 as ::core::ffi::c_int as isize),
        );
        if k >= 0 as ::core::ffi::c_int {
            return Nobjects[POLLUT as ::core::ffi::c_int as usize] + k + m;
        }
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn getVariableValue(mut varCode: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut p: ::core::ffi::c_int = 0;
    let mut a1: ::core::ffi::c_double = 0.;
    let mut a2: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut treatment: *mut TTreatment = ::core::ptr::null_mut::<TTreatment>();
    if varCode < PVMAX {
        match varCode {
            0 => {
                if (*Node.offset(J as isize)).type_0 == STORAGE as ::core::ffi::c_int {
                    return (*Storage.offset((*Node.offset(J as isize)).subIndex as isize)).hrt
                        / 3600.0f64;
                } else {
                    return 0.0f64;
                }
            }
            1 => return Dt,
            2 => return Q * UCF(FLOW as ::core::ffi::c_int),
            3 => {
                y = ((*Node.offset(J as isize)).oldDepth + (*Node.offset(J as isize)).newDepth)
                    / 2.0f64;
                return y * UCF(LENGTH as ::core::ffi::c_int);
            }
            4 => {
                a1 = node_getSurfArea(J, (*Node.offset(J as isize)).oldDepth);
                a2 = node_getSurfArea(J, (*Node.offset(J as isize)).newDepth);
                return (a1 + a2) / 2.0f64
                    * UCF(LENGTH as ::core::ffi::c_int)
                    * UCF(LENGTH as ::core::ffi::c_int);
            }
            _ => return 0.0f64,
        }
    } else if varCode < PVMAX + Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        p = varCode - PVMAX;
        treatment = (*Node.offset(J as isize)).treatment.offset(p as isize) as *mut TTreatment;
        if (*treatment).treatType == REMOVAL as ::core::ffi::c_int {
            return *Cin.offset(p as isize);
        }
        return *(*Node.offset(J as isize)).newQual.offset(p as isize);
    } else {
        p = varCode - PVMAX - Nobjects[POLLUT as ::core::ffi::c_int as usize];
        if p >= Nobjects[POLLUT as ::core::ffi::c_int as usize] {
            return 0.0f64;
        }
        return getRemoval(p);
    };
}
unsafe extern "C" fn getRemoval(mut p: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut c0: ::core::ffi::c_double = *(*Node.offset(J as isize)).newQual.offset(p as isize);
    let mut r: ::core::ffi::c_double = 0.;
    let mut treatment: *mut TTreatment = ::core::ptr::null_mut::<TTreatment>();
    if *R.offset(p as isize) > 1.0f64 || ErrCode != 0 {
        ErrCode = 1 as ::core::ffi::c_int;
        return 0.0f64;
    }
    if *R.offset(p as isize) >= 0.0f64 && *R.offset(p as isize) <= 1.0f64 {
        return *R.offset(p as isize);
    }
    *R.offset(p as isize) = 10.0f64;
    if c0 == 0.0f64 {
        *R.offset(p as isize) = 0.0f64;
        return 0.0f64;
    }
    treatment = (*Node.offset(J as isize)).treatment.offset(p as isize) as *mut TTreatment;
    r = mathexpr_eval(
        (*treatment).equation,
        Some(getVariableValue as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_double),
    );
    r = if 0.0f64 >= r { 0.0f64 } else { r };
    if (*treatment).treatType == REMOVAL as ::core::ffi::c_int {
        r = if 1.0f64 <= r { 1.0f64 } else { r };
        *R.offset(p as isize) = r;
    } else {
        r = if c0 <= r { c0 } else { r };
        *R.offset(p as isize) = 1.0f64 - r / c0;
    }
    return *R.offset(p as isize);
}
