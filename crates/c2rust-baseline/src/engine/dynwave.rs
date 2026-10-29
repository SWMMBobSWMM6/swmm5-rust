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
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut NonConvergeCount: ::core::ffi::c_long;
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut Nlinks: [::core::ffi::c_int; 5];
    static mut SurchargeMethod: ::core::ffi::c_int;
    static mut AllowPonding: ::core::ffi::c_int;
    static mut ErrorCode: ::core::ffi::c_int;
    static mut MaxTrials: ::core::ffi::c_int;
    static mut RouteStep: ::core::ffi::c_double;
    static mut MinRouteStep: ::core::ffi::c_double;
    static mut CourantFactor: ::core::ffi::c_double;
    static mut CrownCutoff: ::core::ffi::c_double;
    static mut MinSurfArea: ::core::ffi::c_double;
    static mut HeadTol: ::core::ffi::c_double;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    static mut Pump: *mut TPump;
    static mut Orifice: *mut TOrifice;
    fn report_writeErrorMsg(code: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    fn dwflow_findConduitFlow(
        j: ::core::ffi::c_int,
        steps: ::core::ffi::c_int,
        omega: ::core::ffi::c_double,
        dt: ::core::ffi::c_double,
    );
    fn stats_updateCriticalTimeCount(node: ::core::ffi::c_int, link: ::core::ffi::c_int);
    fn stats_updateConvergenceStats(node: ::core::ffi::c_int, converged: ::core::ffi::c_int);
    fn node_getSurfArea(
        node: ::core::ffi::c_int,
        depth: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn node_getVolume(
        node: ::core::ffi::c_int,
        depth: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn node_getPondedArea(
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
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
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
pub type SurchargeMethodType = ::core::ffi::c_uint;
pub const SLOT: SurchargeMethodType = 1;
pub const EXTRAN: SurchargeMethodType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TOrifice {
    pub type_0: ::core::ffi::c_int,
    pub shape: ::core::ffi::c_int,
    pub cDisch: ::core::ffi::c_double,
    pub orate: ::core::ffi::c_double,
    pub cOrif: ::core::ffi::c_double,
    pub hCrit: ::core::ffi::c_double,
    pub cWeir: ::core::ffi::c_double,
    pub length: ::core::ffi::c_double,
    pub surfArea: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TXnode {
    pub converged: ::core::ffi::c_char,
    pub newSurfArea: ::core::ffi::c_double,
    pub oldSurfArea: ::core::ffi::c_double,
    pub sumdqdh: ::core::ffi::c_double,
    pub dYdT: ::core::ffi::c_double,
}
pub const ERR_MEMORY: ErrorType = 101;
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
pub const FUDGE: ::core::ffi::c_double = 0.0001f64;
static mut MINTIMESTEP: ::core::ffi::c_double = 0.001f64;
static mut OMEGA: ::core::ffi::c_double = 0.5f64;
static mut DEFAULT_SURFAREA: ::core::ffi::c_double = 12.566f64;
static mut DEFAULT_HEADTOL: ::core::ffi::c_double = 0.005f64;
static mut EXTRAN_CROWN_CUTOFF: ::core::ffi::c_double = 0.96f64;
static mut SLOT_CROWN_CUTOFF: ::core::ffi::c_double = 0.985257f64;
static mut DEFAULT_MAXTRIALS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut VariableStep: ::core::ffi::c_double = 0.;
static mut Xnode: *mut TXnode = ::core::ptr::null::<TXnode>() as *mut TXnode;
static mut Omega: ::core::ffi::c_double = 0.;
static mut Steps: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn dynwave_init() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_double = 0.;
    VariableStep = 0.0f64;
    Xnode = calloc(
        Nobjects[NODE as ::core::ffi::c_int as usize] as size_t,
        ::core::mem::size_of::<TXnode>() as size_t,
    ) as *mut TXnode;
    if Xnode.is_null() {
        report_writeErrorMsg(
            ERR_MEMORY as ::core::ffi::c_int,
            b" Not enough memory for dynamic wave routing.\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Xnode.offset(i as isize)).newSurfArea = 0.0f64;
        (*Xnode.offset(i as isize)).oldSurfArea = 0.0f64;
        (*Node.offset(i as isize)).crownElev = (*Node.offset(i as isize)).invertElev;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        j = (*Link.offset(i as isize)).node1;
        z = (*Node.offset(j as isize)).invertElev
            + (*Link.offset(i as isize)).offset1
            + (*Link.offset(i as isize)).xsect.yFull;
        (*Node.offset(j as isize)).crownElev = if (*Node.offset(j as isize)).crownElev >= z {
            (*Node.offset(j as isize)).crownElev
        } else {
            z
        };
        j = (*Link.offset(i as isize)).node2;
        z = (*Node.offset(j as isize)).invertElev
            + (*Link.offset(i as isize)).offset2
            + (*Link.offset(i as isize)).xsect.yFull;
        (*Node.offset(j as isize)).crownElev = if (*Node.offset(j as isize)).crownElev >= z {
            (*Node.offset(j as isize)).crownElev
        } else {
            z
        };
        (*Link.offset(i as isize)).flowClass = DRY as ::core::ffi::c_int;
        (*Link.offset(i as isize)).dqdh = 0.0f64;
        i += 1;
    }
    if SurchargeMethod == SLOT as ::core::ffi::c_int {
        CrownCutoff = SLOT_CROWN_CUTOFF;
    } else {
        CrownCutoff = EXTRAN_CROWN_CUTOFF;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dynwave_close() {
    if !Xnode.is_null() {
        free(Xnode as *mut ::core::ffi::c_void);
        Xnode = ::core::ptr::null_mut::<TXnode>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn dynwave_validate() {
    if MinRouteStep > RouteStep {
        MinRouteStep = RouteStep;
    }
    if MinRouteStep < MINTIMESTEP {
        MinRouteStep = MINTIMESTEP;
    }
    if MinSurfArea == 0.0f64 {
        MinSurfArea = DEFAULT_SURFAREA;
    } else {
        MinSurfArea /= UCF(LENGTH as ::core::ffi::c_int) * UCF(LENGTH as ::core::ffi::c_int);
    }
    if HeadTol == 0.0f64 {
        HeadTol = DEFAULT_HEADTOL;
    } else {
        HeadTol /= UCF(LENGTH as ::core::ffi::c_int);
    }
    if MaxTrials == 0 as ::core::ffi::c_int {
        MaxTrials = DEFAULT_MAXTRIALS;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dynwave_getRoutingStep(
    mut fixedStep: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if CourantFactor == 0.0f64 {
        return fixedStep;
    }
    if fixedStep < MINTIMESTEP {
        return fixedStep;
    }
    if VariableStep == 0.0f64 {
        VariableStep = MinRouteStep;
    } else {
        VariableStep = getVariableStep(fixedStep);
    }
    VariableStep = floor(1000.0f64 * VariableStep) / 1000.0f64;
    return VariableStep;
}
#[no_mangle]
pub unsafe extern "C" fn dynwave_execute(mut tStep: ::core::ffi::c_double) -> ::core::ffi::c_int {
    let mut converged: ::core::ffi::c_int = 0;
    if ErrorCode != 0 {
        return 0 as ::core::ffi::c_int;
    }
    Steps = 0 as ::core::ffi::c_int;
    converged = FALSE;
    Omega = OMEGA;
    initRoutingStep();
    while Steps < MaxTrials {
        initNodeStates();
        findLinkFlows(tStep);
        converged = findNodeDepths(tStep);
        Steps += 1;
        if !(Steps > 1 as ::core::ffi::c_int) {
            continue;
        }
        if converged != 0 {
            break;
        }
        findBypassedLinks();
    }
    if converged == 0 {
        updateConvergenceStats();
    }
    findLimitedLinks();
    return Steps;
}
unsafe extern "C" fn updateConvergenceStats() {
    let mut i: ::core::ffi::c_int = 0;
    NonConvergeCount += 1;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        stats_updateConvergenceStats(
            i,
            (*Xnode.offset(i as isize)).converged as ::core::ffi::c_int,
        );
        i += 1;
    }
}
unsafe extern "C" fn initRoutingStep() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        (*Xnode.offset(i as isize)).converged = FALSE as ::core::ffi::c_char;
        (*Xnode.offset(i as isize)).dYdT = 0.0f64;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        (*Link.offset(i as isize)).bypassed = FALSE as ::core::ffi::c_char;
        (*Link.offset(i as isize)).surfArea1 = 0.0f64;
        (*Link.offset(i as isize)).surfArea2 = 0.0f64;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nlinks[CONDUIT as ::core::ffi::c_int as usize] {
        (*Conduit.offset(i as isize)).a2 = (*Conduit.offset(i as isize)).a1;
        i += 1;
    }
}
unsafe extern "C" fn initNodeStates() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if AllowPonding != 0 {
            (*Xnode.offset(i as isize)).newSurfArea =
                node_getPondedArea(i, (*Node.offset(i as isize)).newDepth);
        } else {
            (*Xnode.offset(i as isize)).newSurfArea =
                node_getSurfArea(i, (*Node.offset(i as isize)).newDepth);
        }
        (*Node.offset(i as isize)).inflow = 0.0f64;
        (*Node.offset(i as isize)).outflow = (*Node.offset(i as isize)).losses;
        if (*Node.offset(i as isize)).newLatFlow >= 0.0f64 {
            (*Node.offset(i as isize)).inflow += (*Node.offset(i as isize)).newLatFlow;
        } else {
            (*Node.offset(i as isize)).outflow -= (*Node.offset(i as isize)).newLatFlow;
        }
        (*Xnode.offset(i as isize)).sumdqdh = 0.0f64;
        i += 1;
    }
}
unsafe extern "C" fn findBypassedLinks() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Xnode.offset((*Link.offset(i as isize)).node1 as isize)).converged
            as ::core::ffi::c_int
            != 0
            && (*Xnode.offset((*Link.offset(i as isize)).node2 as isize)).converged
                as ::core::ffi::c_int
                != 0
        {
            (*Link.offset(i as isize)).bypassed = TRUE as ::core::ffi::c_char;
        } else {
            (*Link.offset(i as isize)).bypassed = FALSE as ::core::ffi::c_char;
        }
        i += 1;
    }
}
unsafe extern "C" fn findLimitedLinks() {
    let mut j: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut h1: ::core::ffi::c_double = 0.;
    let mut h2: ::core::ffi::c_double = 0.;
    j = 0 as ::core::ffi::c_int;
    while j < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if !(isTrueConduit(j) == 0) {
            k = (*Link.offset(j as isize)).subIndex;
            (*Conduit.offset(k as isize)).capacityLimited = FALSE as ::core::ffi::c_char;
            if (*Conduit.offset(k as isize)).a1 >= (*Link.offset(j as isize)).xsect.aFull {
                n1 = (*Link.offset(j as isize)).node1;
                n2 = (*Link.offset(j as isize)).node2;
                h1 = (*Node.offset(n1 as isize)).newDepth + (*Node.offset(n1 as isize)).invertElev;
                h2 = (*Node.offset(n2 as isize)).newDepth + (*Node.offset(n2 as isize)).invertElev;
                if h1 - h2
                    > fabs((*Conduit.offset(k as isize)).slope)
                        * (*Conduit.offset(k as isize)).length
                {
                    (*Conduit.offset(k as isize)).capacityLimited = TRUE as ::core::ffi::c_char;
                }
            }
        }
        j += 1;
    }
}
unsafe extern "C" fn findLinkFlows(mut dt: ::core::ffi::c_double) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if isTrueConduit(i) != 0 && (*Link.offset(i as isize)).bypassed == 0 {
            dwflow_findConduitFlow(i, Steps, Omega, dt);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if isTrueConduit(i) != 0 {
            updateNodeFlows(i);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if isTrueConduit(i) == 0 {
            if (*Link.offset(i as isize)).bypassed == 0 {
                findNonConduitFlow(i, dt);
            }
            updateNodeFlows(i);
        }
        i += 1;
    }
}
unsafe extern "C" fn isTrueConduit(mut j: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return ((*Link.offset(j as isize)).type_0 == CONDUIT as ::core::ffi::c_int
        && (*Link.offset(j as isize)).xsect.type_0 != DUMMY as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn findNonConduitFlow(mut i: ::core::ffi::c_int, mut dt: ::core::ffi::c_double) {
    let mut qLast: ::core::ffi::c_double = 0.;
    let mut qNew: ::core::ffi::c_double = 0.;
    qLast = (*Link.offset(i as isize)).newFlow;
    (*Link.offset(i as isize)).dqdh = 0.0f64;
    qNew = link_getInflow(i);
    if (*Link.offset(i as isize)).type_0 == PUMP as ::core::ffi::c_int {
        qNew = getModPumpFlow(i, qNew, dt);
    }
    findNonConduitSurfArea(i);
    if Steps > 0 as ::core::ffi::c_int
        && (*Link.offset(i as isize)).type_0 != PUMP as ::core::ffi::c_int
    {
        qNew = (1.0f64 - Omega) * qLast + Omega * qNew;
        if qNew * qLast < 0.0f64 {
            qNew = 0.001f64
                * (if qNew < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_double;
        }
    }
    (*Link.offset(i as isize)).newFlow = qNew;
}
unsafe extern "C" fn getModPumpFlow(
    mut i: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
    mut dt: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = (*Link.offset(i as isize)).node1;
    let mut k: ::core::ffi::c_int = (*Link.offset(i as isize)).subIndex;
    let mut newNetInflow: ::core::ffi::c_double = 0.;
    let mut netFlowVolume: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    if q == 0.0f64 {
        return q;
    }
    if (*Node.offset(j as isize)).type_0 == STORAGE as ::core::ffi::c_int {
        return node_getMaxOutflow(j, q, dt);
    }
    match (*Pump.offset(k as isize)).type_0 {
        0 => return node_getMaxOutflow(j, q, dt),
        1 | 3 | 2 => {
            newNetInflow =
                (*Node.offset(j as isize)).inflow - (*Node.offset(j as isize)).outflow - q;
            netFlowVolume = 0.5f64 * ((*Node.offset(j as isize)).oldNetInflow + newNetInflow) * dt;
            y = (*Node.offset(j as isize)).oldDepth
                + netFlowVolume / (*Xnode.offset(j as isize)).newSurfArea;
            if y <= 0.0f64 {
                return (*Node.offset(j as isize)).inflow;
            }
        }
        _ => {}
    }
    return q;
}
unsafe extern "C" fn findNonConduitSurfArea(mut i: ::core::ffi::c_int) {
    if (*Link.offset(i as isize)).type_0 == ORIFICE as ::core::ffi::c_int {
        (*Link.offset(i as isize)).surfArea1 =
            (*Orifice.offset((*Link.offset(i as isize)).subIndex as isize)).surfArea / 2.0f64;
    } else {
        (*Link.offset(i as isize)).surfArea1 = 0.0f64;
    }
    (*Link.offset(i as isize)).surfArea2 = (*Link.offset(i as isize)).surfArea1;
    if (*Link.offset(i as isize)).flowClass == UP_CRITICAL as ::core::ffi::c_int
        || (*Node.offset((*Link.offset(i as isize)).node1 as isize)).type_0
            == STORAGE as ::core::ffi::c_int
    {
        (*Link.offset(i as isize)).surfArea1 = 0.0f64;
    }
    if (*Link.offset(i as isize)).flowClass == DN_CRITICAL as ::core::ffi::c_int
        || (*Node.offset((*Link.offset(i as isize)).node2 as isize)).type_0
            == STORAGE as ::core::ffi::c_int
    {
        (*Link.offset(i as isize)).surfArea2 = 0.0f64;
    }
}
unsafe extern "C" fn updateNodeFlows(mut i: ::core::ffi::c_int) {
    let mut k: ::core::ffi::c_int = 0;
    let mut barrels: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut n1: ::core::ffi::c_int = (*Link.offset(i as isize)).node1;
    let mut n2: ::core::ffi::c_int = (*Link.offset(i as isize)).node2;
    let mut q: ::core::ffi::c_double = (*Link.offset(i as isize)).newFlow;
    let mut conduitLossRate: ::core::ffi::c_double = 0.0f64;
    if q >= 0.0f64 {
        (*Node.offset(n1 as isize)).outflow += q;
        (*Node.offset(n2 as isize)).inflow += q;
    } else {
        (*Node.offset(n1 as isize)).inflow -= q;
        (*Node.offset(n2 as isize)).outflow -= q;
    }
    if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
        k = (*Link.offset(i as isize)).subIndex;
        barrels = (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int;
        conduitLossRate = ((*Conduit.offset(k as isize)).evapLossRate
            + (*Conduit.offset(k as isize)).seepLossRate)
            * barrels as ::core::ffi::c_double;
        if conduitLossRate > 0.0f64 {
            if (*Node.offset(n1 as isize)).type_0 != OUTFALL as ::core::ffi::c_int
                && (*Node.offset(n2 as isize)).type_0 != OUTFALL as ::core::ffi::c_int
            {
                conduitLossRate /= 2.0f64;
            }
            if (*Node.offset(n1 as isize)).type_0 != OUTFALL as ::core::ffi::c_int {
                (*Node.offset(n1 as isize)).outflow += conduitLossRate;
            }
            if (*Node.offset(n2 as isize)).type_0 != OUTFALL as ::core::ffi::c_int {
                (*Node.offset(n2 as isize)).outflow += conduitLossRate;
            }
        }
    }
    (*Xnode.offset((*Link.offset(i as isize)).node1 as isize)).newSurfArea +=
        (*Link.offset(i as isize)).surfArea1 * barrels as ::core::ffi::c_double;
    (*Xnode.offset((*Link.offset(i as isize)).node2 as isize)).newSurfArea +=
        (*Link.offset(i as isize)).surfArea2 * barrels as ::core::ffi::c_double;
    (*Xnode.offset((*Link.offset(i as isize)).node1 as isize)).sumdqdh +=
        (*Link.offset(i as isize)).dqdh;
    if (*Link.offset(i as isize)).type_0 == PUMP as ::core::ffi::c_int {
        k = (*Link.offset(i as isize)).subIndex;
        if (*Pump.offset(k as isize)).type_0 != TYPE4_PUMP as ::core::ffi::c_int {
            (*Xnode.offset(n2 as isize)).sumdqdh += (*Link.offset(i as isize)).dqdh;
        }
    } else {
        (*Xnode.offset(n2 as isize)).sumdqdh += (*Link.offset(i as isize)).dqdh;
    };
}
unsafe extern "C" fn findNodeDepths(mut dt: ::core::ffi::c_double) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut yOld: ::core::ffi::c_double = 0.0f64;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        link_setOutfallDepth(i);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(i as isize)).type_0 == OUTFALL as ::core::ffi::c_int) {
            yOld = (*Node.offset(i as isize)).newDepth;
            setNodeDepth(i, dt);
            (*Xnode.offset(i as isize)).converged = TRUE as ::core::ffi::c_char;
            if fabs(yOld - (*Node.offset(i as isize)).newDepth) > HeadTol {
                (*Xnode.offset(i as isize)).converged = FALSE as ::core::ffi::c_char;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(i as isize)).type_0 == OUTFALL as ::core::ffi::c_int) {
            if (*Xnode.offset(i as isize)).converged as ::core::ffi::c_int == FALSE {
                return FALSE;
            }
        }
        i += 1;
    }
    return TRUE;
}
unsafe extern "C" fn setNodeDepth(mut i: ::core::ffi::c_int, mut dt: ::core::ffi::c_double) {
    let mut canPond: ::core::ffi::c_int = 0;
    let mut isPonded: ::core::ffi::c_int = 0;
    let mut isSurcharged: ::core::ffi::c_int = FALSE;
    let mut dQ: ::core::ffi::c_double = 0.;
    let mut dV: ::core::ffi::c_double = 0.;
    let mut dy: ::core::ffi::c_double = 0.;
    let mut yMax: ::core::ffi::c_double = 0.;
    let mut yOld: ::core::ffi::c_double = 0.;
    let mut yLast: ::core::ffi::c_double = 0.;
    let mut yNew: ::core::ffi::c_double = 0.;
    let mut yCrown: ::core::ffi::c_double = 0.;
    let mut surfArea: ::core::ffi::c_double = 0.;
    let mut denom: ::core::ffi::c_double = 0.;
    let mut corr: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    canPond =
        (AllowPonding != 0 && (*Node.offset(i as isize)).pondedArea > 0.0f64) as ::core::ffi::c_int;
    isPonded = (canPond != 0
        && (*Node.offset(i as isize)).newDepth > (*Node.offset(i as isize)).fullDepth)
        as ::core::ffi::c_int;
    yCrown = (*Node.offset(i as isize)).crownElev - (*Node.offset(i as isize)).invertElev;
    yOld = (*Node.offset(i as isize)).oldDepth;
    yLast = (*Node.offset(i as isize)).newDepth;
    (*Node.offset(i as isize)).overflow = 0.0f64;
    surfArea = (*Xnode.offset(i as isize)).newSurfArea;
    surfArea = if surfArea >= MinSurfArea {
        surfArea
    } else {
        MinSurfArea
    };
    dQ = (*Node.offset(i as isize)).inflow - (*Node.offset(i as isize)).outflow;
    dV = 0.5f64 * ((*Node.offset(i as isize)).oldNetInflow + dQ) * dt;
    if SurchargeMethod == EXTRAN as ::core::ffi::c_int {
        if isPonded != 0 {
            isSurcharged = FALSE;
        } else if (*Node.offset(i as isize)).type_0 == STORAGE as ::core::ffi::c_int {
            isSurcharged = ((*Node.offset(i as isize)).surDepth > 0.0f64
                && yLast > (*Node.offset(i as isize)).fullDepth)
                as ::core::ffi::c_int;
        } else {
            isSurcharged = (yCrown > 0.0f64 && yLast > yCrown) as ::core::ffi::c_int;
        }
    }
    if isSurcharged == 0 {
        dy = dV / surfArea;
        yNew = yOld + dy;
        if isPonded == 0 {
            (*Xnode.offset(i as isize)).oldSurfArea = surfArea;
        }
        if Steps > 0 as ::core::ffi::c_int {
            yNew = (1.0f64 - Omega) * yLast + Omega * yNew;
        }
        if isPonded != 0 && yNew < (*Node.offset(i as isize)).fullDepth {
            yNew = (*Node.offset(i as isize)).fullDepth - FUDGE;
        }
    } else {
        corr = 1.0f64;
        if (*Node.offset(i as isize)).degree < 0 as ::core::ffi::c_int {
            corr = 0.6f64;
        }
        denom = (*Xnode.offset(i as isize)).sumdqdh;
        if yLast < 1.25f64 * yCrown {
            f = (yLast - yCrown) / yCrown;
            denom += ((*Xnode.offset(i as isize)).oldSurfArea / dt
                - (*Xnode.offset(i as isize)).sumdqdh)
                * exp(-15.0f64 * f);
        }
        if denom == 0.0f64 {
            dy = 0.0f64;
        } else {
            dy = corr * dQ / denom;
        }
        yNew = yLast + dy;
        if yNew < yCrown {
            yNew = yCrown - FUDGE;
        }
        if canPond != 0 && yNew > (*Node.offset(i as isize)).fullDepth {
            yNew = (*Node.offset(i as isize)).fullDepth + FUDGE;
        }
    }
    if yNew < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        yNew = 0.0f64;
    }
    yMax = (*Node.offset(i as isize)).fullDepth;
    if canPond == FALSE {
        yMax += (*Node.offset(i as isize)).surDepth;
    }
    if yNew > yMax {
        yNew = getFloodedDepth(i, canPond, dV, yNew, yMax, dt);
    } else {
        (*Node.offset(i as isize)).newVolume = node_getVolume(i, yNew);
    }
    (*Xnode.offset(i as isize)).dYdT = fabs(yNew - yOld) / dt;
    (*Node.offset(i as isize)).newDepth = yNew;
}
unsafe extern "C" fn getFloodedDepth(
    mut i: ::core::ffi::c_int,
    mut canPond: ::core::ffi::c_int,
    mut dV: ::core::ffi::c_double,
    mut yNew: ::core::ffi::c_double,
    mut yMax: ::core::ffi::c_double,
    mut dt: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if canPond == FALSE {
        (*Node.offset(i as isize)).overflow = dV / dt;
        (*Node.offset(i as isize)).newVolume = (*Node.offset(i as isize)).fullVolume;
        yNew = yMax;
    } else {
        (*Node.offset(i as isize)).newVolume =
            if (*Node.offset(i as isize)).oldVolume + dV >= (*Node.offset(i as isize)).fullVolume {
                (*Node.offset(i as isize)).oldVolume + dV
            } else {
                (*Node.offset(i as isize)).fullVolume
            };
        (*Node.offset(i as isize)).overflow = ((*Node.offset(i as isize)).newVolume
            - (if (*Node.offset(i as isize)).oldVolume >= (*Node.offset(i as isize)).fullVolume {
                (*Node.offset(i as isize)).oldVolume
            } else {
                (*Node.offset(i as isize)).fullVolume
            }))
            / dt;
    }
    if (*Node.offset(i as isize)).overflow < FUDGE {
        (*Node.offset(i as isize)).overflow = 0.0f64;
    }
    return yNew;
}
unsafe extern "C" fn getVariableStep(mut maxStep: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut minLink: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut minNode: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut tMin: ::core::ffi::c_double = 0.;
    let mut tMinLink: ::core::ffi::c_double = 0.;
    let mut tMinNode: ::core::ffi::c_double = 0.;
    tMin = maxStep;
    tMinLink = getLinkStep(tMin, &raw mut minLink);
    tMinNode = getNodeStep(tMinLink, &raw mut minNode);
    tMin = tMinLink;
    if tMinNode < tMin {
        tMin = tMinNode;
        minLink = -(1 as ::core::ffi::c_int);
    }
    stats_updateCriticalTimeCount(minNode, minLink);
    if tMin < MinRouteStep {
        tMin = MinRouteStep;
    }
    return tMin;
}
unsafe extern "C" fn getLinkStep(
    mut tMin: ::core::ffi::c_double,
    mut minLink: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_double = 0.;
    let mut t: ::core::ffi::c_double = 0.;
    let mut tLink: ::core::ffi::c_double = tMin;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LINK as ::core::ffi::c_int as usize] {
        if (*Link.offset(i as isize)).type_0 == CONDUIT as ::core::ffi::c_int {
            k = (*Link.offset(i as isize)).subIndex;
            q = fabs((*Link.offset(i as isize)).newFlow)
                / (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int
                    as ::core::ffi::c_double;
            if !(q <= FUDGE
                || (*Conduit.offset(k as isize)).a1 <= FUDGE
                || (*Link.offset(i as isize)).froude <= 0.01f64)
            {
                t = (*Link.offset(i as isize)).newVolume
                    / (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_int
                        as ::core::ffi::c_double
                    / q;
                t = t * (*Conduit.offset(k as isize)).modLength / link_getLength(i);
                t = t * (*Link.offset(i as isize)).froude
                    / (1.0f64 + (*Link.offset(i as isize)).froude)
                    * CourantFactor;
                if t < tLink {
                    tLink = t;
                    *minLink = i;
                }
            }
        }
        i += 1;
    }
    return tLink;
}
unsafe extern "C" fn getNodeStep(
    mut tMin: ::core::ffi::c_double,
    mut minNode: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut maxDepth: ::core::ffi::c_double = 0.;
    let mut dYdT: ::core::ffi::c_double = 0.;
    let mut t1: ::core::ffi::c_double = 0.;
    let mut tNode: ::core::ffi::c_double = tMin;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[NODE as ::core::ffi::c_int as usize] {
        if !((*Node.offset(i as isize)).type_0 == OUTFALL as ::core::ffi::c_int) {
            if !((*Node.offset(i as isize)).newDepth <= FUDGE) {
                if !((*Node.offset(i as isize)).newDepth + FUDGE
                    >= (*Node.offset(i as isize)).crownElev - (*Node.offset(i as isize)).invertElev)
                {
                    maxDepth = ((*Node.offset(i as isize)).crownElev
                        - (*Node.offset(i as isize)).invertElev)
                        * 0.25f64;
                    if !(maxDepth < FUDGE) {
                        dYdT = (*Xnode.offset(i as isize)).dYdT;
                        if !(dYdT < FUDGE) {
                            t1 = maxDepth / dYdT;
                            if t1 < tNode {
                                tNode = t1;
                                *minNode = i;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return tNode;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
