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
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut Frpt: TFile;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut RouteModel: ::core::ffi::c_int;
    static mut Node: *mut TNode;
    static mut Divider: *mut TDivider;
    static mut Link: *mut TLink;
    static mut Pump: *mut TPump;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
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
pub type XsectType = ::core::ffi::c_uint;
pub const STREET_XSECT: XsectType = 25;
pub const FORCE_MAIN: XsectType = 24;
pub const CUSTOM: XsectType = 23;
pub const IRREGULAR: XsectType = 22;
pub const SEMICIRCULAR: XsectType = 21;
pub const BASKETHANDLE: XsectType = 20;
pub const SEMIELLIPTICAL: XsectType = 19;
pub const CATENARY: XsectType = 18;
pub const GOTHIC: XsectType = 17;
pub const HORSESHOE: XsectType = 16;
pub const EGGSHAPED: XsectType = 15;
pub const ARCH: XsectType = 14;
pub const VERT_ELLIPSE: XsectType = 13;
pub const HORIZ_ELLIPSE: XsectType = 12;
pub const MOD_BASKET: XsectType = 11;
pub const RECT_ROUND: XsectType = 10;
pub const RECT_TRIANG: XsectType = 9;
pub const POWERFUNC: XsectType = 8;
pub const PARABOLIC: XsectType = 7;
pub const TRIANGULAR: XsectType = 6;
pub const TRAPEZOIDAL: XsectType = 5;
pub const RECT_OPEN: XsectType = 4;
pub const RECT_CLOSED: XsectType = 3;
pub const FILLED_CIRCULAR: XsectType = 2;
pub const CIRCULAR: XsectType = 1;
pub const DUMMY: XsectType = 0;
pub type RouteModelType = ::core::ffi::c_uint;
pub const DW: RouteModelType = 4;
pub const EKW: RouteModelType = 3;
pub const KW: RouteModelType = 2;
pub const SF: RouteModelType = 1;
pub const NO_ROUTING: RouteModelType = 0;
pub type PumpCurveType = ::core::ffi::c_uint;
pub const IDEAL_PUMP: PumpCurveType = 5;
pub const TYPE5_PUMP: PumpCurveType = 4;
pub const TYPE4_PUMP: PumpCurveType = 3;
pub const TYPE3_PUMP: PumpCurveType = 2;
pub const TYPE2_PUMP: PumpCurveType = 1;
pub const TYPE1_PUMP: PumpCurveType = 0;
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
pub struct TPump {
    pub type_0: ::core::ffi::c_int,
    pub pumpCurve: ::core::ffi::c_int,
    pub initSetting: ::core::ffi::c_double,
    pub yOn: ::core::ffi::c_double,
    pub yOff: ::core::ffi::c_double,
    pub xMin: ::core::ffi::c_double,
    pub xMax: ::core::ffi::c_double,
}
pub const UNDIRECTED: AdjListType = 0;
pub const ERR_LOOP: ErrorType = 131;
pub const DIRECTED: AdjListType = 1;
pub const ERR_MEMORY: ErrorType = 101;
pub const ERR_DUMMY_LINK: ErrorType = 134;
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
pub const ERR_MULTI_OUTLET: ErrorType = 133;
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
pub type AdjListType = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut InDegree: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut StartPos: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut AdjList: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut Stack: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut First: ::core::ffi::c_int = 0;
static mut Last: ::core::ffi::c_int = 0;
static mut Examined: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut InTree: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut LoopLinks: *mut ::core::ffi::c_int =
    ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
static mut LoopLinksLast: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn toposort_sortLinks(mut sortedLinks: *mut ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        *sortedLinks.offset(i as isize) = i;
        i += 1;
    }
    if RouteModel == DW as ::core::ffi::c_int {
        checkDummyLinks();
        if ErrorCode != 0 {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
            (*Node.offset(i as isize)).degree = 0 as ::core::ffi::c_int;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
            n = (*Link.offset(i as isize)).node1;
            if ((*Link.offset(i as isize)).direction as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                n = (*Link.offset(i as isize)).node2;
            }
            if (*Node.offset(n as isize)).type_0 == OUTFALL as ::core::ffi::c_int {
                if ((*Link.offset(i as isize)).direction as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    n = (*Link.offset(i as isize)).node1;
                } else {
                    n = (*Link.offset(i as isize)).node2;
                }
                let ref mut fresh0 = (*Node.offset(n as isize)).degree;
                *fresh0 += 1;
            } else {
                let ref mut fresh1 = (*Node.offset(n as isize)).degree;
                *fresh1 += 1;
            }
            i += 1;
        }
        return;
    }
    if ErrorCode != 0 {
        return;
    }
    InDegree = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    StartPos = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    AdjList = calloc(
        Nobjects[LINK as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    Stack = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if InDegree.is_null() || StartPos.is_null() || AdjList.is_null() || Stack.is_null() {
        report_writeErrorMsg(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        createAdjList(DIRECTED as ::core::ffi::c_int);
        adjustAdjList();
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
            *InDegree.offset(i as isize) = 0 as ::core::ffi::c_int;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
            let ref mut fresh2 = *InDegree.offset((*Link.offset(i as isize)).node2 as isize);
            *fresh2 += 1;
            i += 1;
        }
        n = topoSort(sortedLinks);
    }
    if !InDegree.is_null() {
        free(InDegree as *mut ::core::ffi::c_void);
        InDegree = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !StartPos.is_null() {
        free(StartPos as *mut ::core::ffi::c_void);
        StartPos = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !AdjList.is_null() {
        free(AdjList as *mut ::core::ffi::c_void);
        AdjList = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !Stack.is_null() {
        free(Stack as *mut ::core::ffi::c_void);
        Stack = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if ErrorCode == 0 && n != Nobjects[LINK as ::core::ffi::c_int as usize] {
        report_writeErrorMsg(
            ERR_LOOP as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        findCycles();
    }
}
unsafe extern "C" fn createAdjList(mut listType: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(i as isize)).degree = 0 as ::core::ffi::c_int;
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        let ref mut fresh3 = (*Node.offset((*Link.offset(j as isize)).node1 as isize)).degree;
        *fresh3 += 1;
        if listType == UNDIRECTED as ::core::ffi::c_int {
            let ref mut fresh4 = (*Node.offset((*Link.offset(j as isize)).node2 as isize)).degree;
            *fresh4 += 1;
        }
        j += 1;
    }
    *StartPos.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int {
        *StartPos.offset((i + 1 as ::core::ffi::c_int) as isize) =
            *StartPos.offset(i as isize) + (*Node.offset(i as isize)).degree;
        (*Node.offset(i as isize)).degree = 0 as ::core::ffi::c_int;
        i += 1;
    }
    (*Node.offset(
        (Nobjects[NODE as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int) as isize,
    ))
    .degree = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        i = (*Link.offset(j as isize)).node1;
        k = *StartPos.offset(i as isize) + (*Node.offset(i as isize)).degree;
        *AdjList.offset(k as isize) = j;
        let ref mut fresh5 = (*Node.offset(i as isize)).degree;
        *fresh5 += 1;
        if listType == UNDIRECTED as ::core::ffi::c_int {
            i = (*Link.offset(j as isize)).node2;
            k = *StartPos.offset(i as isize) + (*Node.offset(i as isize)).degree;
            *AdjList.offset(k as isize) = j;
            let ref mut fresh6 = (*Node.offset(i as isize)).degree;
            *fresh6 += 1;
        }
        j += 1;
    }
}
unsafe extern "C" fn adjustAdjList() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(i as isize)).type_0 != DIVIDER as ::core::ffi::c_int) {
            if !((*Node.offset(i as isize)).degree != 2 as ::core::ffi::c_int) {
                k = (*Node.offset(i as isize)).subIndex;
                m = *StartPos.offset(i as isize);
                j = *AdjList.offset(m as isize);
                if j == (*Divider.offset(k as isize)).link {
                    *AdjList.offset(m as isize) =
                        *AdjList.offset((m + 1 as ::core::ffi::c_int) as isize);
                    *AdjList.offset((m + 1 as ::core::ffi::c_int) as isize) = j;
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn topoSort(mut sortedLinks: *mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i1: ::core::ffi::c_int = 0;
    let mut i2: ::core::ffi::c_int = 0;
    let mut k1: ::core::ffi::c_int = 0;
    let mut k2: ::core::ffi::c_int = 0;
    First = 0 as ::core::ffi::c_int;
    Last = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if *InDegree.offset(i as isize) == 0 as ::core::ffi::c_int {
            Last += 1;
            *Stack.offset(Last as isize) = i;
        }
        i += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while First <= Last {
        i1 = *Stack.offset(First as isize);
        k1 = *StartPos.offset(i1 as isize);
        k2 = k1 + (*Node.offset(i1 as isize)).degree;
        k = k1;
        while k < k2 {
            j = *AdjList.offset(k as isize);
            *sortedLinks.offset(n as isize) = j;
            n += 1;
            i2 = (*Link.offset(j as isize)).node2;
            let ref mut fresh7 = *InDegree.offset(i2 as isize);
            *fresh7 -= 1;
            if *InDegree.offset(i2 as isize) == 0 as ::core::ffi::c_int {
                Last += 1;
                *Stack.offset(Last as isize) = i2;
            }
            k += 1;
        }
        First += 1;
    }
    return n;
}
unsafe extern "C" fn findCycles() {
    let mut i: ::core::ffi::c_int = 0;
    AdjList = calloc(
        (2 as size_t).wrapping_mul(Nobjects[LINK as ::core::ffi::c_int as usize] as size_t),
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    StartPos = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    Stack = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    Examined = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
    ) as *mut ::core::ffi::c_char;
    InTree = calloc(
        Nobjects[LINK as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
    ) as *mut ::core::ffi::c_char;
    LoopLinks = calloc(
        Nobjects[LINK as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if !StartPos.is_null()
        && !AdjList.is_null()
        && !Stack.is_null()
        && !Examined.is_null()
        && !InTree.is_null()
        && !LoopLinks.is_null()
    {
        createAdjList(UNDIRECTED as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
            *Examined.offset(i as isize) = 0 as ::core::ffi::c_char;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
            *InTree.offset(i as isize) = 0 as ::core::ffi::c_char;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
            if !(*Examined.offset(i as isize) != 0) {
                Last = -(1 as ::core::ffi::c_int);
                findSpanningTree(i);
            }
            i += 1;
        }
    }
    if !StartPos.is_null() {
        free(StartPos as *mut ::core::ffi::c_void);
        StartPos = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !AdjList.is_null() {
        free(AdjList as *mut ::core::ffi::c_void);
        AdjList = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !Stack.is_null() {
        free(Stack as *mut ::core::ffi::c_void);
        Stack = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !Examined.is_null() {
        free(Examined as *mut ::core::ffi::c_void);
        Examined = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !InTree.is_null() {
        free(InTree as *mut ::core::ffi::c_void);
        InTree = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !LoopLinks.is_null() {
        free(LoopLinks as *mut ::core::ffi::c_void);
        LoopLinks = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
}
unsafe extern "C" fn findSpanningTree(mut startNode: ::core::ffi::c_int) {
    let mut nextNode: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    m = *StartPos.offset(startNode as isize);
    while m < *StartPos.offset(startNode as isize) + (*Node.offset(startNode as isize)).degree {
        k = *AdjList.offset(m as isize);
        if (*Link.offset(k as isize)).node1 == startNode {
            j = (*Link.offset(k as isize)).node2;
        } else {
            j = (*Link.offset(k as isize)).node1;
        }
        if *InTree.offset(k as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if *Examined.offset(j as isize) != 0 {
                *InTree.offset(k as isize) = 2 as ::core::ffi::c_char;
                evalLoop(k);
            } else {
                *Examined.offset(j as isize) = 1 as ::core::ffi::c_char;
                Last += 1;
                *Stack.offset(Last as isize) = j;
                *InTree.offset(k as isize) = 1 as ::core::ffi::c_char;
            }
        }
        m += 1;
    }
    if Last >= 0 as ::core::ffi::c_int {
        nextNode = *Stack.offset(Last as isize);
        Last -= 1;
        findSpanningTree(nextNode);
    }
}
unsafe extern "C" fn evalLoop(mut startLink: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut i1: ::core::ffi::c_int = 0;
    let mut i2: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut kount: ::core::ffi::c_int = 0;
    let mut isCycle: ::core::ffi::c_int = 0;
    LoopLinksLast = 0 as ::core::ffi::c_int;
    *LoopLinks.offset(0 as ::core::ffi::c_int as isize) = startLink;
    i1 = (*Link.offset(startLink as isize)).node1;
    i2 = (*Link.offset(startLink as isize)).node2;
    if traceLoop(i1, i2, startLink) == 0 {
        return;
    }
    isCycle = TRUE;
    j = *LoopLinks.offset(0 as ::core::ffi::c_int as isize);
    i2 = (*Link.offset(j as isize)).node2;
    i = 1 as ::core::ffi::c_int;
    while i <= LoopLinksLast {
        j = *LoopLinks.offset(i as isize);
        i1 = (*Link.offset(j as isize)).node1;
        if i1 == i2 {
            i2 = (*Link.offset(j as isize)).node2;
            i += 1;
        } else {
            isCycle = FALSE;
            break;
        }
    }
    if isCycle != 0 {
        kount = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i <= LoopLinksLast {
            if kount % 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                fprintf(
                    Frpt.file,
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            kount += 1;
            fprintf(
                Frpt.file,
                b"  %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*Link.offset(*LoopLinks.offset(i as isize) as isize)).ID,
            );
            if i < LoopLinksLast {
                fprintf(
                    Frpt.file,
                    b"  -->\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn traceLoop(
    mut i1: ::core::ffi::c_int,
    mut i2: ::core::ffi::c_int,
    mut k1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    if i1 == i2 {
        return TRUE;
    }
    m = *StartPos.offset(i1 as isize);
    while m < *StartPos.offset(i1 as isize) + (*Node.offset(i1 as isize)).degree {
        k = *AdjList.offset(m as isize);
        if !(k == k1 || *InTree.offset(k as isize) as ::core::ffi::c_int != 1 as ::core::ffi::c_int)
        {
            if (*Link.offset(k as isize)).node1 == i1 {
                j = (*Link.offset(k as isize)).node2;
            } else {
                j = (*Link.offset(k as isize)).node1;
            }
            if traceLoop(j, i2, k) != 0 {
                LoopLinksLast += 1;
                *LoopLinks.offset(LoopLinksLast as isize) = k;
                return TRUE;
            }
        }
        m += 1;
    }
    return FALSE;
}
unsafe extern "C" fn checkDummyLinks() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut marked: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    marked = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if marked.is_null() {
        report_writeErrorMsg(
            ERR_MEMORY as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        j = (*Link.offset(i as isize)).node2;
        if ((*Link.offset(i as isize)).direction as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            j = (*Link.offset(i as isize)).node1;
        }
        if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int
            && (*Link.offset(i as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int
            || (*Link.offset(i as isize)).type_0 == PUMP as ::core::ffi::c_int
                && (*Pump.offset((*Link.offset(i as isize)).subIndex as isize)).type_0
                    == IDEAL_PUMP as ::core::ffi::c_int
        {
            if *marked.offset(j as isize) == 0 as ::core::ffi::c_int {
                *marked.offset(j as isize) = 1 as ::core::ffi::c_int;
            }
        } else {
            *marked.offset(j as isize) = -(1 as ::core::ffi::c_int);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int
            && (*Link.offset(i as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int
            || (*Link.offset(i as isize)).type_0 == PUMP as ::core::ffi::c_int
                && (*Pump.offset((*Link.offset(i as isize)).subIndex as isize)).type_0
                    == IDEAL_PUMP as ::core::ffi::c_int
        {
            j = (*Link.offset(i as isize)).node1;
            if *marked.offset(j as isize) > 0 as ::core::ffi::c_int {
                report_writeErrorMsg(
                    ERR_DUMMY_LINK as ::core::ffi::c_int,
                    (*Node.offset(j as isize)).ID,
                );
            }
        }
        i += 1;
    }
    if !marked.is_null() {
        free(marked as *mut ::core::ffi::c_void);
        marked = ::core::ptr::null_mut::<::core::ffi::c_int>();
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
#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
