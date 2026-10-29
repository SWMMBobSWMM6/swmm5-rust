#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut Fhotstart1: TFile;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut ErrorCode: ::core::ffi::c_int;
    static mut AllowPonding: ::core::ffi::c_int;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Pump: *mut TPump;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn kinwave_execute(
        link: ::core::ffi::c_int,
        qin: *mut ::core::ffi::c_double,
        qout: *mut ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn dynwave_init();
    fn dynwave_close();
    fn dynwave_getRoutingStep(fixedStep: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn dynwave_execute(tStep: ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn node_getDepth(
        node: ::core::ffi::c_int,
        volume: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn node_getVolume(
        node: ::core::ffi::c_int,
        depth: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn node_getMaxOutflow(
        node: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn link_getInflow(link: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn link_setOutfallDepth(link: ::core::ffi::c_int);
    fn link_getLength(link: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn link_getLossRate(
        link: ::core::ffi::c_int,
        routeModel: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        tstep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn xsect_getYofA(xsect: *mut TXsect, area: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getAofS(xsect: *mut TXsect, sFactor: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getAofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
pub type FileUsageType = ::core::ffi::c_uint;
pub const SAVE_FILE: FileUsageType = 3;
pub const USE_FILE: FileUsageType = 2;
pub const SCRATCH_FILE: FileUsageType = 1;
pub const NO_FILE: FileUsageType = 0;
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
pub type FlowClassType = ::core::ffi::c_uint;
pub const ALL_FULL: FlowClassType = 10;
pub const DN_FULL: FlowClassType = 9;
pub const UP_FULL: FlowClassType = 8;
pub const MAX_FLOW_CLASSES: FlowClassType = 7;
pub const DN_CRITICAL: FlowClassType = 6;
pub const UP_CRITICAL: FlowClassType = 5;
pub const SUPCRITICAL: FlowClassType = 4;
pub const SUBCRITICAL: FlowClassType = 3;
pub const DN_DRY: FlowClassType = 2;
pub const UP_DRY: FlowClassType = 1;
pub const DRY: FlowClassType = 0;
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
pub struct TConduit {
    pub length: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub barrels: ::core::ffi::c_char,
    pub modLength: ::core::ffi::c_double,
    pub roughFactor: ::core::ffi::c_double,
    pub slope: ::core::ffi::c_double,
    pub beta: ::core::ffi::c_double,
    pub qMax: ::core::ffi::c_double,
    pub a1: ::core::ffi::c_double,
    pub a2: ::core::ffi::c_double,
    pub q1: ::core::ffi::c_double,
    pub q2: ::core::ffi::c_double,
    pub q1Old: ::core::ffi::c_double,
    pub q2Old: ::core::ffi::c_double,
    pub evapLossRate: ::core::ffi::c_double,
    pub seepLossRate: ::core::ffi::c_double,
    pub capacityLimited: ::core::ffi::c_char,
    pub superCritical: ::core::ffi::c_char,
    pub hasLosses: ::core::ffi::c_char,
    pub fullState: ::core::ffi::c_char,
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
pub const ERR_REGULATOR: ErrorType = 139;
pub const ERR_SLOPE: ErrorType = 115;
pub const ERR_MULTI_OUTLET: ErrorType = 133;
pub const ERR_OUTFALL: ErrorType = 141;
pub const ERR_DIVIDER: ErrorType = 135;
pub const ERR_NO_OUTLETS: ErrorType = 145;
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
pub const ERR_REGULATOR_SHAPE: ErrorType = 143;
pub const ERR_STORAGE_VOLUME: ErrorType = 140;
pub const ERR_NODE_DEPTH: ErrorType = 138;
pub const ERR_WEIR_DIVIDER: ErrorType = 137;
pub const ERR_DIVIDER_LINK: ErrorType = 136;
pub const ERR_LOOP: ErrorType = 131;
pub const ERR_PUMP_LIMITS: ErrorType = 122;
pub const ERR_NO_CURVE: ErrorType = 121;
pub const ERR_XSECT: ErrorType = 119;
pub const ERR_NO_XSECT: ErrorType = 117;
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
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FUDGE: ::core::ffi::c_double = 0.0001f64;
static mut OMEGA: ::core::ffi::c_double = 0.55f64;
static mut MAXITER: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut STOPTOL: ::core::ffi::c_double = 0.005f64;
#[no_mangle]
pub unsafe extern "C" fn flowrout_init(mut routingModel: ::core::ffi::c_int) {
    if routingModel == DW as ::core::ffi::c_int {
        validateGeneralLayout();
        dynwave_init();
        if Fhotstart1.mode as ::core::ffi::c_int == NO_FILE as ::core::ffi::c_int {
            initNodeDepths();
            initLinkDepths();
        }
    } else {
        validateTreeLayout();
    }
    initNodes();
    initLinks(routingModel);
}
#[no_mangle]
pub unsafe extern "C" fn flowrout_close(mut routingModel: ::core::ffi::c_int) {
    if routingModel == DW as ::core::ffi::c_int {
        dynwave_close();
    }
}
#[no_mangle]
pub unsafe extern "C" fn flowrout_getRoutingStep(
    mut routingModel: ::core::ffi::c_int,
    mut fixedStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if routingModel == DW as ::core::ffi::c_int {
        return dynwave_getRoutingStep(fixedStep);
    }
    return fixedStep;
}
#[no_mangle]
pub unsafe extern "C" fn flowrout_execute(
    mut links: *mut ::core::ffi::c_int,
    mut routingModel: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut qin: ::core::ffi::c_double = 0.;
    let mut qout: ::core::ffi::c_double = 0.;
    let mut steps: ::core::ffi::c_double = 0.;
    if ErrorCode != 0 {
        return 0 as ::core::ffi::c_int;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(j as isize)).updated = FALSE as ::core::ffi::c_char;
        (*Node.offset(j as isize)).overflow = 0.0f64;
        if (*Node.offset(j as isize)).type_0 != STORAGE as ::core::ffi::c_int
            && (*Node.offset(j as isize)).newVolume > (*Node.offset(j as isize)).fullVolume
        {
            (*Node.offset(j as isize)).overflow = ((*Node.offset(j as isize)).newVolume
                - (*Node.offset(j as isize)).fullVolume)
                / tStep;
        }
        j += 1;
    }
    if routingModel == DW as ::core::ffi::c_int {
        return dynwave_execute(tStep);
    }
    steps = 0.0f64;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        j = *links.offset(i as isize);
        n1 = (*Link.offset(j as isize)).node1;
        if (*Node.offset(n1 as isize)).type_0 == STORAGE as ::core::ffi::c_int {
            updateStorageState(n1, i, links, tStep);
        }
        qin = getLinkInflow(j, tStep);
        if routingModel == SF as ::core::ffi::c_int {
            steps +=
                steadyflow_execute(j, &raw mut qin, &raw mut qout, tStep) as ::core::ffi::c_double;
        } else {
            steps +=
                kinwave_execute(j, &raw mut qin, &raw mut qout, tStep) as ::core::ffi::c_double;
        }
        (*Link.offset(j as isize)).newFlow = qout;
        (*Node.offset((*Link.offset(j as isize)).node1 as isize)).outflow += qin;
        (*Node.offset((*Link.offset(j as isize)).node2 as isize)).inflow += qout;
        i += 1;
    }
    if Nobjects[LINK as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        steps /= Nobjects[LINK as ::core::ffi::c_int as usize] as ::core::ffi::c_double;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        setNewNodeState(j, tStep);
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        setNewLinkState(j);
        j += 1;
    }
    return crate::ccast::f64_to_i32((steps + 0.5f64));
}
unsafe extern "C" fn validateTreeLayout() {
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[NODE as ::core::ffi::c_int as usize] {
        match (*Node.offset(j as isize)).type_0 {
            3 => {
                if (*Node.offset(j as isize)).degree > 2 as ::core::ffi::c_int {
                    report_writeErrorMsg(
                        ERR_DIVIDER as ::core::ffi::c_int,
                        (*Node.offset(j as isize)).ID,
                    );
                }
            }
            1 => {
                if (*Node.offset(j as isize)).degree > 0 as ::core::ffi::c_int {
                    report_writeErrorMsg(
                        ERR_OUTFALL as ::core::ffi::c_int,
                        (*Node.offset(j as isize)).ID,
                    );
                }
            }
            2 => {}
            _ => {
                if (*Node.offset(j as isize)).degree > 1 as ::core::ffi::c_int {
                    report_writeErrorMsg(
                        ERR_MULTI_OUTLET as ::core::ffi::c_int,
                        (*Node.offset(j as isize)).ID,
                    );
                }
            }
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        match (*Link.offset(j as isize)).type_0 {
            0 => {
                if (*Conduit.offset((*Link.offset(j as isize)).subIndex as isize)).slope < 0.0f64
                    && (*Link.offset(j as isize)).xsect.type_0 != DUMMY as ::core::ffi::c_int
                {
                    report_writeErrorMsg(
                        ERR_SLOPE as ::core::ffi::c_int,
                        (*Link.offset(j as isize)).ID,
                    );
                }
            }
            2 | 3 | 4 => {
                if (*Node.offset((*Link.offset(j as isize)).node1 as isize)).type_0
                    != STORAGE as ::core::ffi::c_int
                {
                    report_writeErrorMsg(
                        ERR_REGULATOR as ::core::ffi::c_int,
                        (*Link.offset(j as isize)).ID,
                    );
                }
            }
            _ => {}
        }
        j += 1;
    }
}
unsafe extern "C" fn validateGeneralLayout() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut outletCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(i as isize)).inflow = 0.0f64;
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        i = (*Link.offset(j as isize)).node1;
        if (*Node.offset(i as isize)).type_0 != OUTFALL as ::core::ffi::c_int {
            i = (*Link.offset(j as isize)).node2;
        }
        (*Node.offset(i as isize)).inflow += 1.0f64;
        if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int
            && (*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int
            || (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int
                && (*Pump.offset((*Link.offset(j as isize)).subIndex as isize)).type_0
                    == IDEAL_PUMP as ::core::ffi::c_int
        {
            i = (*Link.offset(j as isize)).node1;
            if ((*Link.offset(j as isize)).direction as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                i = (*Link.offset(j as isize)).node2;
            }
            if (*Node.offset(i as isize)).degree > 1 as ::core::ffi::c_int {
                report_writeErrorMsg(
                    ERR_DUMMY_LINK as ::core::ffi::c_int,
                    (*Node.offset(i as isize)).ID,
                );
            }
        }
        j += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if (*Node.offset(i as isize)).type_0 == OUTFALL as ::core::ffi::c_int {
            if (*Node.offset(i as isize)).degree
                + crate::ccast::f64_to_i32((*Node.offset(i as isize)).inflow)
                > 1 as ::core::ffi::c_int
            {
                report_writeErrorMsg(
                    ERR_OUTFALL as ::core::ffi::c_int,
                    (*Node.offset(i as isize)).ID,
                );
            } else {
                outletCount += 1;
            }
        }
        i += 1;
    }
    if outletCount == 0 as ::core::ffi::c_int {
        report_writeErrorMsg(
            ERR_NO_OUTLETS as ::core::ffi::c_int,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if (*Node.offset(i as isize)).inflow == 0.0f64 {
            (*Node.offset(i as isize)).degree = -(*Node.offset(i as isize)).degree;
        }
        (*Node.offset(i as isize)).inflow = 0.0f64;
        i += 1;
    }
}
unsafe extern "C" fn initNodeDepths() {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(i as isize)).inflow = 0.0f64;
        (*Node.offset(i as isize)).outflow = 0.0f64;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(i as isize)).newDepth > FUDGE {
            y = (*Link.offset(i as isize)).newDepth + (*Link.offset(i as isize)).offset1;
        } else {
            y = 0.0f64;
        }
        n = (*Link.offset(i as isize)).node1;
        (*Node.offset(n as isize)).inflow += y;
        (*Node.offset(n as isize)).outflow += 1.0f64;
        n = (*Link.offset(i as isize)).node2;
        (*Node.offset(n as isize)).inflow += y;
        (*Node.offset(n as isize)).outflow += 1.0f64;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(i as isize)).type_0 == OUTFALL as ::core::ffi::c_int) {
            if !((*Node.offset(i as isize)).type_0 == STORAGE as ::core::ffi::c_int) {
                if !((*Node.offset(i as isize)).initDepth > 0.0f64) {
                    if (*Node.offset(i as isize)).outflow > 0.0f64 {
                        (*Node.offset(i as isize)).newDepth =
                            (*Node.offset(i as isize)).inflow / (*Node.offset(i as isize)).outflow;
                    }
                }
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        link_setOutfallDepth(i);
        i += 1;
    }
}
unsafe extern "C" fn initLinkDepths() {
    let mut i: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
            if !((*Link.offset(i as isize)).q0 != 0.0f64) {
                y1 = (*Node.offset((*Link.offset(i as isize)).node1 as isize)).newDepth
                    - (*Link.offset(i as isize)).offset1;
                y1 = if y1 >= 0.0f64 { y1 } else { 0.0f64 };
                y1 = if y1 <= (*Link.offset(i as isize)).xsect.yFull {
                    y1
                } else {
                    (*Link.offset(i as isize)).xsect.yFull
                };
                y2 = (*Node.offset((*Link.offset(i as isize)).node2 as isize)).newDepth
                    - (*Link.offset(i as isize)).offset2;
                y2 = if y2 >= 0.0f64 { y2 } else { 0.0f64 };
                y2 = if y2 <= (*Link.offset(i as isize)).xsect.yFull {
                    y2
                } else {
                    (*Link.offset(i as isize)).xsect.yFull
                };
                y = 0.5f64 * (y1 + y2);
                y = if y >= 0.0001f64 { y } else { 0.0001f64 };
                (*Link.offset(i as isize)).newDepth = y;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn initNodes() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Node.offset(i as isize)).inflow = (*Node.offset(i as isize)).newLatFlow;
        (*Node.offset(i as isize)).outflow = 0.0f64;
        (*Node.offset(i as isize)).newVolume = 0.0f64;
        if AllowPonding != 0
            && (*Node.offset(i as isize)).pondedArea > 0.0f64
            && (*Node.offset(i as isize)).newDepth > (*Node.offset(i as isize)).fullDepth
        {
            (*Node.offset(i as isize)).newVolume = (*Node.offset(i as isize)).fullVolume
                + ((*Node.offset(i as isize)).newDepth - (*Node.offset(i as isize)).fullDepth)
                    * (*Node.offset(i as isize)).pondedArea;
        } else {
            (*Node.offset(i as isize)).newVolume =
                node_getVolume(i, (*Node.offset(i as isize)).newDepth);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(i as isize)).newFlow >= 0.0f64 {
            (*Node.offset((*Link.offset(i as isize)).node1 as isize)).outflow +=
                (*Link.offset(i as isize)).newFlow;
            (*Node.offset((*Link.offset(i as isize)).node2 as isize)).inflow +=
                (*Link.offset(i as isize)).newFlow;
        } else {
            (*Node.offset((*Link.offset(i as isize)).node1 as isize)).inflow -=
                (*Link.offset(i as isize)).newFlow;
            (*Node.offset((*Link.offset(i as isize)).node2 as isize)).outflow -=
                (*Link.offset(i as isize)).newFlow;
        }
        i += 1;
    }
}
unsafe extern "C" fn initLinks(mut routingModel: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if routingModel == SF as ::core::ffi::c_int {
            (*Link.offset(i as isize)).newFlow = 0.0f64;
        } else if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
            k = (*Link.offset(i as isize)).subIndex;
            (*Conduit.offset(k as isize)).q1 = (*Link.offset(i as isize)).newFlow
                / (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int
                    as ::core::ffi::c_double;
            (*Conduit.offset(k as isize)).q2 = (*Conduit.offset(k as isize)).q1;
            (*Conduit.offset(k as isize)).a1 = xsect_getAofY(
                &raw mut (*Link.offset(i as isize)).xsect,
                (*Link.offset(i as isize)).newDepth,
            );
            (*Conduit.offset(k as isize)).a2 = (*Conduit.offset(k as isize)).a1;
            (*Link.offset(i as isize)).newVolume = (*Conduit.offset(k as isize)).a1
                * link_getLength(i)
                * (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int
                    as ::core::ffi::c_double;
            (*Link.offset(i as isize)).oldVolume = (*Link.offset(i as isize)).newVolume;
        }
        i += 1;
    }
}
unsafe extern "C" fn getLinkInflow(
    mut j: ::core::ffi::c_int,
    mut dt: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut n1: ::core::ffi::c_int = (*Link.offset(j as isize)).node1;
    let mut q: ::core::ffi::c_double = 0.;
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int
        || (*Link.offset(j as isize)).type_0 == PUMP as ::core::ffi::c_int
        || (*Node.offset(n1 as isize)).type_0 == STORAGE as ::core::ffi::c_int
    {
        q = link_getInflow(j);
    } else {
        q = 0.0f64;
    }
    return node_getMaxOutflow(n1, q, dt);
}
unsafe extern "C" fn updateStorageState(
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut links: *mut ::core::ffi::c_int,
    mut dt: ::core::ffi::c_double,
) {
    let mut iter: ::core::ffi::c_int = 0;
    let mut stopped: ::core::ffi::c_int = 0;
    let mut vFixed: ::core::ffi::c_double = 0.;
    let mut v2: ::core::ffi::c_double = 0.;
    let mut d1: ::core::ffi::c_double = 0.;
    let mut d2: ::core::ffi::c_double = 0.;
    if (*Node.offset(i as isize)).type_0 != STORAGE as ::core::ffi::c_int {
        return;
    }
    if (*Node.offset(i as isize)).updated != 0 {
        return;
    }
    vFixed = (*Node.offset(i as isize)).oldVolume
        + 0.5f64
            * ((*Node.offset(i as isize)).oldNetInflow + (*Node.offset(i as isize)).inflow
                - (*Node.offset(i as isize)).outflow)
            * dt;
    d1 = (*Node.offset(i as isize)).newDepth;
    iter = 1 as ::core::ffi::c_int;
    stopped = FALSE;
    while iter < MAXITER && stopped == 0 {
        v2 = vFixed - 0.5f64 * getStorageOutflow(i, j, links, dt) * dt;
        v2 = if 0.0f64 >= v2 { 0.0f64 } else { v2 };
        (*Node.offset(i as isize)).overflow = 0.0f64;
        if v2 > (*Node.offset(i as isize)).fullVolume {
            (*Node.offset(i as isize)).overflow = (v2
                - (if (*Node.offset(i as isize)).oldVolume >= (*Node.offset(i as isize)).fullVolume
                {
                    (*Node.offset(i as isize)).oldVolume
                } else {
                    (*Node.offset(i as isize)).fullVolume
                }))
                / dt;
            if (*Node.offset(i as isize)).overflow < FUDGE {
                (*Node.offset(i as isize)).overflow = 0.0f64;
            }
            if AllowPonding == 0 || (*Node.offset(i as isize)).pondedArea == 0.0f64 {
                v2 = (*Node.offset(i as isize)).fullVolume;
            }
        }
        (*Node.offset(i as isize)).newVolume = v2;
        d2 = node_getDepth(i, v2);
        (*Node.offset(i as isize)).newDepth = d2;
        d2 = (1.0f64 - OMEGA) * d1 + OMEGA * d2;
        if fabs(d2 - d1) <= STOPTOL {
            stopped = TRUE;
        }
        d1 = d2;
        iter += 1;
    }
    (*Node.offset(i as isize)).updated = TRUE as ::core::ffi::c_char;
}
unsafe extern "C" fn getStorageOutflow(
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut links: *mut ::core::ffi::c_int,
    mut dt: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut outflow: ::core::ffi::c_double = 0.0f64;
    k = j;
    while k < Nobjects[LINK as ::core::ffi::c_int as usize] {
        m = *links.offset(k as isize);
        if (*Link.offset(m as isize)).node1 != i {
            break;
        }
        outflow += getLinkInflow(m, dt);
        k += 1;
    }
    return outflow;
}
unsafe extern "C" fn setNewNodeState(mut j: ::core::ffi::c_int, mut dt: ::core::ffi::c_double) {
    let mut canPond: ::core::ffi::c_int = 0;
    let mut newNetInflow: ::core::ffi::c_double = 0.;
    if (*Node.offset(j as isize)).type_0 == STORAGE as ::core::ffi::c_int {
        if (*Node.offset(j as isize)).updated as ::core::ffi::c_int == FALSE {
            updateStorageState(
                j,
                Nobjects[LINK as ::core::ffi::c_int as usize],
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                dt,
            );
        }
        return;
    }
    newNetInflow = (*Node.offset(j as isize)).inflow
        - (*Node.offset(j as isize)).outflow
        - (*Node.offset(j as isize)).losses;
    (*Node.offset(j as isize)).newVolume = (*Node.offset(j as isize)).oldVolume + newNetInflow * dt;
    if (*Node.offset(j as isize)).newVolume < FUDGE {
        (*Node.offset(j as isize)).newVolume = 0.0f64;
    }
    (*Node.offset(j as isize)).overflow = 0.0f64;
    canPond =
        (AllowPonding != 0 && (*Node.offset(j as isize)).pondedArea > 0.0f64) as ::core::ffi::c_int;
    if (*Node.offset(j as isize)).newVolume > (*Node.offset(j as isize)).fullVolume {
        (*Node.offset(j as isize)).overflow = ((*Node.offset(j as isize)).newVolume
            - (if (*Node.offset(j as isize)).oldVolume >= (*Node.offset(j as isize)).fullVolume {
                (*Node.offset(j as isize)).oldVolume
            } else {
                (*Node.offset(j as isize)).fullVolume
            }))
            / dt;
        if (*Node.offset(j as isize)).overflow < FUDGE {
            (*Node.offset(j as isize)).overflow = 0.0f64;
        }
        if canPond == 0 {
            (*Node.offset(j as isize)).newVolume = (*Node.offset(j as isize)).fullVolume;
        }
    }
    (*Node.offset(j as isize)).newDepth = node_getDepth(j, (*Node.offset(j as isize)).newVolume);
}
unsafe extern "C" fn setNewLinkState(mut j: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    (*Link.offset(j as isize)).newDepth = 0.0f64;
    (*Link.offset(j as isize)).newVolume = 0.0f64;
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        k = (*Link.offset(j as isize)).subIndex;
        a = 0.5f64 * ((*Conduit.offset(k as isize)).a1 + (*Conduit.offset(k as isize)).a2);
        (*Link.offset(j as isize)).newVolume = a
            * link_getLength(j)
            * (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int as ::core::ffi::c_double;
        y1 = xsect_getYofA(
            &raw mut (*Link.offset(j as isize)).xsect,
            (*Conduit.offset(k as isize)).a1,
        );
        y2 = xsect_getYofA(
            &raw mut (*Link.offset(j as isize)).xsect,
            (*Conduit.offset(k as isize)).a2,
        );
        (*Link.offset(j as isize)).newDepth = 0.5f64 * (y1 + y2);
        updateNodeDepth(
            (*Link.offset(j as isize)).node1,
            y1 + (*Link.offset(j as isize)).offset1,
        );
        updateNodeDepth(
            (*Link.offset(j as isize)).node2,
            y2 + (*Link.offset(j as isize)).offset2,
        );
        if (*Conduit.offset(k as isize)).a1 >= (*Link.offset(j as isize)).xsect.aFull {
            (*Conduit.offset(k as isize)).capacityLimited = TRUE as ::core::ffi::c_char;
            (*Conduit.offset(k as isize)).fullState =
                ALL_FULL as ::core::ffi::c_int as ::core::ffi::c_char;
        } else {
            (*Conduit.offset(k as isize)).capacityLimited = FALSE as ::core::ffi::c_char;
            (*Conduit.offset(k as isize)).fullState = 0 as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn updateNodeDepth(mut i: ::core::ffi::c_int, mut y: ::core::ffi::c_double) {
    if (*Node.offset(i as isize)).type_0 == STORAGE as ::core::ffi::c_int {
        return;
    }
    if (*Node.offset(i as isize)).type_0 != OUTFALL as ::core::ffi::c_int
        && (*Node.offset(i as isize)).degree > 0 as ::core::ffi::c_int
        && (*Node.offset(i as isize)).overflow > 0.0f64
    {
        y = (*Node.offset(i as isize)).fullDepth;
    }
    if (*Node.offset(i as isize)).newDepth < y {
        (*Node.offset(i as isize)).newDepth = y;
        if (*Node.offset(i as isize)).fullDepth > 0.0f64 && y > (*Node.offset(i as isize)).fullDepth
        {
            (*Node.offset(i as isize)).newDepth = (*Node.offset(i as isize)).fullDepth;
        }
    }
}
unsafe extern "C" fn steadyflow_execute(
    mut j: ::core::ffi::c_int,
    mut qin: *mut ::core::ffi::c_double,
    mut qout: *mut ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    if (*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        k = (*Link.offset(j as isize)).subIndex;
        q = *qin
            / (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int as ::core::ffi::c_double;
        if (*Link.offset(j as isize)).xsect.type_0 == DUMMY as ::core::ffi::c_int {
            (*Conduit.offset(k as isize)).a1 = 0.0f64;
        } else {
            q -= link_getLossRate(j, SF as ::core::ffi::c_int, q, tStep);
            if q > (*Link.offset(j as isize)).qFull {
                q = (*Link.offset(j as isize)).qFull;
                (*Conduit.offset(k as isize)).a1 = (*Link.offset(j as isize)).xsect.aFull;
                *qin = q
                    * (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int
                        as ::core::ffi::c_double;
            } else {
                s = q / (*Conduit.offset(k as isize)).beta;
                (*Conduit.offset(k as isize)).a1 =
                    xsect_getAofS(&raw mut (*Link.offset(j as isize)).xsect, s);
            }
        }
        (*Conduit.offset(k as isize)).a2 = (*Conduit.offset(k as isize)).a1;
        (*Conduit.offset(k as isize)).q1Old = (*Conduit.offset(k as isize)).q1;
        (*Conduit.offset(k as isize)).q2Old = (*Conduit.offset(k as isize)).q2;
        (*Conduit.offset(k as isize)).q1 = q;
        (*Conduit.offset(k as isize)).q2 = q;
        *qout = q
            * (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        *qout = *qin;
    }
    return 1 as ::core::ffi::c_int;
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
#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
