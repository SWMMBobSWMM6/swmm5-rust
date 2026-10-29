#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut InertDamping: ::core::ffi::c_int;
    static mut SurchargeMethod: ::core::ffi::c_int;
    static mut NormalFlowLtd: ::core::ffi::c_int;
    static mut CrownCutoff: ::core::ffi::c_double;
    static mut Node: *mut TNode;
    static mut Link: *mut TLink;
    static mut Conduit: *mut TConduit;
    fn link_setFlapGate(
        link: ::core::ffi::c_int,
        n1: ::core::ffi::c_int,
        n2: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn link_getLength(link: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn link_getYcrit(link: ::core::ffi::c_int, q: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn link_getYnorm(link: ::core::ffi::c_int, q: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn link_getFroude(
        link: ::core::ffi::c_int,
        v: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn link_getLossRate(
        link: ::core::ffi::c_int,
        routeModel: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        tstep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn link_getFullState(
        a1: ::core::ffi::c_double,
        a2: ::core::ffi::c_double,
        aFull: ::core::ffi::c_double,
    ) -> ::core::ffi::c_char;
    fn xsect_isOpen(type_0: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xsect_getAofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getRofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xsect_getWofY(xsect: *mut TXsect, y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn culvert_getInflow(
        link: ::core::ffi::c_int,
        q: ::core::ffi::c_double,
        h: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn forcemain_getFricSlope(
        j: ::core::ffi::c_int,
        v: ::core::ffi::c_double,
        hrad: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
}
pub type NodeType = ::core::ffi::c_uint;
pub const DIVIDER: NodeType = 3;
pub const STORAGE: NodeType = 2;
pub const OUTFALL: NodeType = 1;
pub const JUNCTION: NodeType = 0;
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
pub type NormalFlowType = ::core::ffi::c_uint;
pub const NEITHER: NormalFlowType = 3;
pub const BOTH: NormalFlowType = 2;
pub const FROUDE: NormalFlowType = 1;
pub const SLOPE: NormalFlowType = 0;
pub type InertialDampingType = ::core::ffi::c_uint;
pub const FULL_DAMPING: InertialDampingType = 2;
pub const PARTIAL_DAMPING: InertialDampingType = 1;
pub const NO_DAMPING: InertialDampingType = 0;
pub type SurchargeMethodType = ::core::ffi::c_uint;
pub const SLOT: SurchargeMethodType = 1;
pub const EXTRAN: SurchargeMethodType = 0;
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
static mut MAXVELOCITY: ::core::ffi::c_double = 50.0f64;
#[no_mangle]
pub unsafe extern "C" fn dwflow_findConduitFlow(
    mut j: ::core::ffi::c_int,
    mut steps: ::core::ffi::c_int,
    mut omega: ::core::ffi::c_double,
    mut dt: ::core::ffi::c_double,
) {
    let mut k: ::core::ffi::c_int = 0;
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut z1: ::core::ffi::c_double = 0.;
    let mut z2: ::core::ffi::c_double = 0.;
    let mut h1: ::core::ffi::c_double = 0.;
    let mut h2: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut a1: ::core::ffi::c_double = 0.;
    let mut a2: ::core::ffi::c_double = 0.;
    let mut r1: ::core::ffi::c_double = 0.;
    let mut yMid: ::core::ffi::c_double = 0.;
    let mut rMid: ::core::ffi::c_double = 0.;
    let mut aMid: ::core::ffi::c_double = 0.;
    let mut aWtd: ::core::ffi::c_double = 0.;
    let mut rWtd: ::core::ffi::c_double = 0.;
    let mut qLast: ::core::ffi::c_double = 0.;
    let mut qOld: ::core::ffi::c_double = 0.;
    let mut aOld: ::core::ffi::c_double = 0.;
    let mut v: ::core::ffi::c_double = 0.;
    let mut rho: ::core::ffi::c_double = 0.;
    let mut sigma: ::core::ffi::c_double = 0.;
    let mut length: ::core::ffi::c_double = 0.;
    let mut wSlot: ::core::ffi::c_double = 0.;
    let mut dq1: ::core::ffi::c_double = 0.;
    let mut dq2: ::core::ffi::c_double = 0.;
    let mut dq3: ::core::ffi::c_double = 0.;
    let mut dq4: ::core::ffi::c_double = 0.;
    let mut dq5: ::core::ffi::c_double = 0.;
    let mut dq6: ::core::ffi::c_double = 0.;
    let mut denom: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut barrels: ::core::ffi::c_double = 0.;
    let mut xsect: *mut TXsect = &raw mut (*Link.offset(j as isize)).xsect;
    let mut isFull: ::core::ffi::c_char = FALSE as ::core::ffi::c_char;
    let mut isClosed: ::core::ffi::c_char = FALSE as ::core::ffi::c_char;
    if (*Link.offset(j as isize)).setting == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        isClosed = TRUE as ::core::ffi::c_char;
    }
    k = (*Link.offset(j as isize)).subIndex;
    barrels = (*Conduit.offset(k as isize)).barrels as ::core::ffi::c_double;
    qOld = (*Link.offset(j as isize)).oldFlow / barrels;
    qLast = (*Conduit.offset(k as isize)).q1;
    (*Conduit.offset(k as isize)).evapLossRate = 0.0f64;
    (*Conduit.offset(k as isize)).seepLossRate = 0.0f64;
    n1 = (*Link.offset(j as isize)).node1;
    n2 = (*Link.offset(j as isize)).node2;
    z1 = (*Node.offset(n1 as isize)).invertElev + (*Link.offset(j as isize)).offset1;
    z2 = (*Node.offset(n2 as isize)).invertElev + (*Link.offset(j as isize)).offset2;
    h1 = (*Node.offset(n1 as isize)).newDepth + (*Node.offset(n1 as isize)).invertElev;
    h2 = (*Node.offset(n2 as isize)).newDepth + (*Node.offset(n2 as isize)).invertElev;
    h1 = if h1 >= z1 { h1 } else { z1 };
    h2 = if h2 >= z2 { h2 } else { z2 };
    y1 = h1 - z1;
    y2 = h2 - z2;
    y1 = if y1 >= 0.0001f64 { y1 } else { 0.0001f64 };
    y2 = if y2 >= 0.0001f64 { y2 } else { 0.0001f64 };
    if SurchargeMethod != SLOT as ::core::ffi::c_int {
        y1 = if y1 <= (*xsect).yFull {
            y1
        } else {
            (*xsect).yFull
        };
        y2 = if y2 <= (*xsect).yFull {
            y2
        } else {
            (*xsect).yFull
        };
    }
    aOld = (*Conduit.offset(k as isize)).a2;
    aOld = if aOld >= 0.0001f64 { aOld } else { 0.0001f64 };
    length = (*Conduit.offset(k as isize)).modLength;
    findSurfArea(
        j,
        qLast,
        length,
        &raw mut h1,
        &raw mut h2,
        &raw mut y1,
        &raw mut y2,
    );
    wSlot = getSlotWidth(xsect, y1);
    a1 = getArea(xsect, y1, wSlot);
    r1 = getHydRad(xsect, y1);
    wSlot = getSlotWidth(xsect, y2);
    a2 = getArea(xsect, y2, wSlot);
    yMid = 0.5f64 * (y1 + y2);
    wSlot = getSlotWidth(xsect, yMid);
    aMid = getArea(xsect, yMid, wSlot);
    rMid = getHydRad(xsect, yMid);
    if y1 >= (*xsect).yFull && y2 >= (*xsect).yFull {
        isFull = TRUE as ::core::ffi::c_char;
    }
    if (*Link.offset(j as isize)).flowClass == DRY as ::core::ffi::c_int
        || (*Link.offset(j as isize)).flowClass == UP_DRY as ::core::ffi::c_int
        || (*Link.offset(j as isize)).flowClass == DN_DRY as ::core::ffi::c_int
        || isClosed as ::core::ffi::c_int != 0
        || aMid <= FUDGE
    {
        (*Conduit.offset(k as isize)).a1 = 0.5f64 * (a1 + a2);
        (*Conduit.offset(k as isize)).q1 = 0.0f64;
        (*Conduit.offset(k as isize)).q2 = 0.0f64;
        (*Link.offset(j as isize)).dqdh = GRAVITY * dt * aMid / length * barrels;
        (*Link.offset(j as isize)).froude = 0.0f64;
        (*Link.offset(j as isize)).newDepth = if yMid <= (*Link.offset(j as isize)).xsect.yFull {
            yMid
        } else {
            (*Link.offset(j as isize)).xsect.yFull
        };
        (*Link.offset(j as isize)).newVolume =
            (*Conduit.offset(k as isize)).a1 * link_getLength(j) * barrels;
        (*Link.offset(j as isize)).newFlow = 0.0f64;
        return;
    }
    v = qLast / aMid;
    if fabs(v) > MAXVELOCITY {
        v = MAXVELOCITY
            * (if qLast < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_double;
    }
    (*Link.offset(j as isize)).froude = link_getFroude(j, v, yMid);
    if (*Link.offset(j as isize)).flowClass == SUBCRITICAL as ::core::ffi::c_int
        && (*Link.offset(j as isize)).froude > 1.0f64
    {
        (*Link.offset(j as isize)).flowClass = SUPCRITICAL as ::core::ffi::c_int;
    }
    if (*Link.offset(j as isize)).froude <= 0.5f64 {
        sigma = 1.0f64;
    } else if (*Link.offset(j as isize)).froude >= 1.0f64 {
        sigma = 0.0f64;
    } else {
        sigma = 2.0f64 * (1.0f64 - (*Link.offset(j as isize)).froude);
    }
    rho = 1.0f64;
    if isFull == 0 && qLast > 0.0f64 && h1 >= h2 {
        rho = sigma;
    }
    aWtd = a1 + (aMid - a1) * rho;
    rWtd = r1 + (rMid - r1) * rho;
    if InertDamping == NO_DAMPING as ::core::ffi::c_int {
        sigma = 1.0f64;
    } else if InertDamping == FULL_DAMPING as ::core::ffi::c_int {
        sigma = 0.0f64;
    }
    if isFull as ::core::ffi::c_int != 0 && xsect_isOpen((*xsect).type_0) == 0 {
        sigma = 0.0f64;
    }
    if (*xsect).type_0 == FORCE_MAIN as ::core::ffi::c_int && isFull as ::core::ffi::c_int != 0 {
        dq1 = dt * forcemain_getFricSlope(j, fabs(v), rMid);
    } else {
        dq1 = dt * (*Conduit.offset(k as isize)).roughFactor / pow(rWtd, 1.33333f64) * fabs(v);
    }
    dq2 = dt * GRAVITY * aWtd * (h2 - h1) / length;
    dq3 = 0.0f64;
    dq4 = 0.0f64;
    if sigma > 0.0f64 {
        dq3 = 2.0f64 * v * (aMid - aOld) * sigma;
        dq4 = dt * v * v * (a2 - a1) / length * sigma;
    }
    dq5 = 0.0f64;
    if (*Conduit.offset(k as isize)).hasLosses != 0 {
        dq5 = findLocalLosses(j, a1, a2, aMid, qLast) / 2.0f64 / length * dt;
    }
    dq6 = link_getLossRate(j, DW as ::core::ffi::c_int, qLast, dt) * 2.5f64 * dt * v
        / link_getLength(j);
    denom = 1.0f64 + dq1 + dq5;
    q = (qOld - dq2 + dq3 + dq4 + dq6) / denom;
    (*Link.offset(j as isize)).dqdh = 1.0f64 / denom * GRAVITY * dt * aWtd / length * barrels;
    (*Link.offset(j as isize)).inletControl = FALSE as ::core::ffi::c_char;
    (*Link.offset(j as isize)).normalFlow = FALSE as ::core::ffi::c_char;
    if q > 0.0f64 {
        if (*xsect).culvertCode > 0 as ::core::ffi::c_int && isFull == 0 {
            q = culvert_getInflow(j, q, h1);
        } else if NormalFlowLtd != NEITHER as ::core::ffi::c_int
            && y1 < (*Link.offset(j as isize)).xsect.yFull
            && ((*Link.offset(j as isize)).flowClass == SUBCRITICAL as ::core::ffi::c_int
                || (*Link.offset(j as isize)).flowClass == SUPCRITICAL as ::core::ffi::c_int)
        {
            q = checkNormalFlow(j, q, y1, y2, a1, r1);
        }
    }
    if steps > 0 as ::core::ffi::c_int {
        q = (1.0f64 - omega) * qLast + omega * q;
        if q * qLast < 0.0f64 {
            q = 0.001f64
                * (if q < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_double;
        }
    }
    if (*Link.offset(j as isize)).qLimit > 0.0f64 {
        if fabs(q) > (*Link.offset(j as isize)).qLimit {
            q = (if q < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_double
                * (*Link.offset(j as isize)).qLimit;
        }
    }
    if link_setFlapGate(j, n1, n2, q) != 0 {
        q = 0.0f64;
    }
    if q > FUDGE && (*Node.offset(n1 as isize)).newDepth <= FUDGE {
        q = FUDGE;
    }
    if q < -FUDGE && (*Node.offset(n2 as isize)).newDepth <= FUDGE {
        q = -FUDGE;
    }
    (*Conduit.offset(k as isize)).a1 = aMid;
    (*Conduit.offset(k as isize)).q1 = q;
    (*Conduit.offset(k as isize)).q2 = q;
    (*Link.offset(j as isize)).newDepth = if yMid <= (*xsect).yFull {
        yMid
    } else {
        (*xsect).yFull
    };
    aMid = (a1 + a2) / 2.0f64;
    (*Conduit.offset(k as isize)).fullState = link_getFullState(a1, a2, (*xsect).aFull);
    (*Link.offset(j as isize)).newVolume = aMid * link_getLength(j) * barrels;
    (*Link.offset(j as isize)).newFlow = q * barrels;
}
unsafe extern "C" fn getFlowClass(
    mut j: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
    mut h1: ::core::ffi::c_double,
    mut h2: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    mut yC: *mut ::core::ffi::c_double,
    mut yN: *mut ::core::ffi::c_double,
    mut fasnh: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut flowClass: ::core::ffi::c_int = 0;
    let mut ycMin: ::core::ffi::c_double = 0.;
    let mut ycMax: ::core::ffi::c_double = 0.;
    let mut z1: ::core::ffi::c_double = 0.;
    let mut z2: ::core::ffi::c_double = 0.;
    n1 = (*Link.offset(j as isize)).node1;
    n2 = (*Link.offset(j as isize)).node2;
    z1 = (*Link.offset(j as isize)).offset1;
    z2 = (*Link.offset(j as isize)).offset2;
    if (*Node.offset(n1 as isize)).type_0 == OUTFALL as ::core::ffi::c_int {
        z1 = if 0.0f64 >= z1 - (*Node.offset(n1 as isize)).newDepth {
            0.0f64
        } else {
            z1 - (*Node.offset(n1 as isize)).newDepth
        };
    }
    if (*Node.offset(n2 as isize)).type_0 == OUTFALL as ::core::ffi::c_int {
        z2 = if 0.0f64 >= z2 - (*Node.offset(n2 as isize)).newDepth {
            0.0f64
        } else {
            z2 - (*Node.offset(n2 as isize)).newDepth
        };
    }
    flowClass = SUBCRITICAL as ::core::ffi::c_int;
    *fasnh = 1.0f64;
    if y1 > FUDGE && y2 > FUDGE {
        if q < 0.0f64 {
            if z1 > 0.0f64 {
                *yN = link_getYnorm(j, fabs(q));
                *yC = link_getYcrit(j, fabs(q));
                ycMin = if *yN <= *yC { *yN } else { *yC };
                if y1 < ycMin {
                    flowClass = UP_CRITICAL as ::core::ffi::c_int;
                }
            }
        } else if z2 > 0.0f64 {
            *yN = link_getYnorm(j, fabs(q));
            *yC = link_getYcrit(j, fabs(q));
            ycMin = if *yN <= *yC { *yN } else { *yC };
            ycMax = if *yN >= *yC { *yN } else { *yC };
            if y2 < ycMin {
                flowClass = DN_CRITICAL as ::core::ffi::c_int;
            } else if y2 < ycMax {
                if ycMax - ycMin < FUDGE {
                    *fasnh = 0.0f64;
                } else {
                    *fasnh = (ycMax - y2) / (ycMax - ycMin);
                }
            }
        }
    } else if y1 <= FUDGE && y2 <= FUDGE {
        flowClass = DRY as ::core::ffi::c_int;
    } else if y2 > FUDGE {
        if h2 < (*Node.offset(n1 as isize)).invertElev + (*Link.offset(j as isize)).offset1 {
            flowClass = UP_DRY as ::core::ffi::c_int;
        } else if z1 > 0.0f64 {
            *yN = link_getYnorm(j, fabs(q));
            *yC = link_getYcrit(j, fabs(q));
            flowClass = UP_CRITICAL as ::core::ffi::c_int;
        }
    } else if h1 < (*Node.offset(n2 as isize)).invertElev + (*Link.offset(j as isize)).offset2 {
        flowClass = DN_DRY as ::core::ffi::c_int;
    } else if z2 > 0.0f64 {
        *yN = link_getYnorm(j, fabs(q));
        *yC = link_getYcrit(j, fabs(q));
        flowClass = DN_CRITICAL as ::core::ffi::c_int;
    }
    return flowClass;
}
unsafe extern "C" fn findSurfArea(
    mut j: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
    mut length: ::core::ffi::c_double,
    mut h1: *mut ::core::ffi::c_double,
    mut h2: *mut ::core::ffi::c_double,
    mut y1: *mut ::core::ffi::c_double,
    mut y2: *mut ::core::ffi::c_double,
) {
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut flowDepth1: ::core::ffi::c_double = 0.;
    let mut flowDepth2: ::core::ffi::c_double = 0.;
    let mut flowDepthMid: ::core::ffi::c_double = 0.;
    let mut width1: ::core::ffi::c_double = 0.;
    let mut width2: ::core::ffi::c_double = 0.;
    let mut widthMid: ::core::ffi::c_double = 0.;
    let mut surfArea1: ::core::ffi::c_double = 0.0f64;
    let mut surfArea2: ::core::ffi::c_double = 0.0f64;
    let mut criticalDepth: ::core::ffi::c_double = 0.;
    let mut normalDepth: ::core::ffi::c_double = 0.;
    let mut fullDepth: ::core::ffi::c_double = 0.;
    let mut fasnh: ::core::ffi::c_double = 1.0f64;
    let mut xsect: *mut TXsect = &raw mut (*Link.offset(j as isize)).xsect;
    n1 = (*Link.offset(j as isize)).node1;
    n2 = (*Link.offset(j as isize)).node2;
    flowDepth1 = *y1;
    flowDepth2 = *y2;
    normalDepth = (flowDepth1 + flowDepth2) / 2.0f64;
    criticalDepth = normalDepth;
    fullDepth = (*xsect).yFull;
    if flowDepth1 >= fullDepth && flowDepth2 >= fullDepth {
        (*Link.offset(j as isize)).flowClass = SUBCRITICAL as ::core::ffi::c_int;
    } else {
        (*Link.offset(j as isize)).flowClass = getFlowClass(
            j,
            q,
            *h1,
            *h2,
            *y1,
            *y2,
            &raw mut criticalDepth,
            &raw mut normalDepth,
            &raw mut fasnh,
        );
    }
    match (*Link.offset(j as isize)).flowClass {
        3 => {
            flowDepthMid = 0.5f64 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            width1 = getWidth(xsect, flowDepth1);
            width2 = getWidth(xsect, flowDepth2);
            widthMid = getWidth(xsect, flowDepthMid);
            surfArea1 = (width1 + widthMid) * length / 4.0f64;
            surfArea2 = (widthMid + width2) * length / 4.0f64 * fasnh;
        }
        5 => {
            flowDepth1 = criticalDepth;
            if normalDepth < criticalDepth {
                flowDepth1 = normalDepth;
            }
            flowDepth1 = if flowDepth1 >= 0.0001f64 {
                flowDepth1
            } else {
                0.0001f64
            };
            *h1 = (*Node.offset(n1 as isize)).invertElev
                + (*Link.offset(j as isize)).offset1
                + flowDepth1;
            flowDepthMid = 0.5f64 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            width2 = getWidth(xsect, flowDepth2);
            widthMid = getWidth(xsect, flowDepthMid);
            surfArea2 = (widthMid + width2) * length * 0.5f64;
        }
        6 => {
            flowDepth2 = criticalDepth;
            if normalDepth < criticalDepth {
                flowDepth2 = normalDepth;
            }
            flowDepth2 = if flowDepth2 >= 0.0001f64 {
                flowDepth2
            } else {
                0.0001f64
            };
            *h2 = (*Node.offset(n2 as isize)).invertElev
                + (*Link.offset(j as isize)).offset2
                + flowDepth2;
            width1 = getWidth(xsect, flowDepth1);
            flowDepthMid = 0.5f64 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            widthMid = getWidth(xsect, flowDepthMid);
            surfArea1 = (width1 + widthMid) * length * 0.5f64;
        }
        1 => {
            flowDepth1 = FUDGE;
            flowDepthMid = 0.5f64 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            width1 = getWidth(xsect, flowDepth1);
            width2 = getWidth(xsect, flowDepth2);
            widthMid = getWidth(xsect, flowDepthMid);
            surfArea2 = (widthMid + width2) * length / 4.0f64;
            if (*Link.offset(j as isize)).offset1 <= 0.0f64 {
                surfArea1 = (width1 + widthMid) * length / 4.0f64;
            }
        }
        2 => {
            flowDepth2 = FUDGE;
            flowDepthMid = 0.5f64 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            width1 = getWidth(xsect, flowDepth1);
            width2 = getWidth(xsect, flowDepth2);
            widthMid = getWidth(xsect, flowDepthMid);
            surfArea1 = (widthMid + width1) * length / 4.0f64;
            if (*Link.offset(j as isize)).offset2 <= 0.0f64 {
                surfArea2 = (width2 + widthMid) * length / 4.0f64;
            }
        }
        0 => {
            surfArea1 = FUDGE * length / 2.0f64;
            surfArea2 = surfArea1;
        }
        _ => {}
    }
    (*Link.offset(j as isize)).surfArea1 = surfArea1;
    (*Link.offset(j as isize)).surfArea2 = surfArea2;
    *y1 = flowDepth1;
    *y2 = flowDepth2;
}
unsafe extern "C" fn findLocalLosses(
    mut j: ::core::ffi::c_int,
    mut a1: ::core::ffi::c_double,
    mut a2: ::core::ffi::c_double,
    mut aMid: ::core::ffi::c_double,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut losses: ::core::ffi::c_double = 0.0f64;
    q = fabs(q);
    if a1 > FUDGE {
        losses += (*Link.offset(j as isize)).cLossInlet * (q / a1);
    }
    if a2 > FUDGE {
        losses += (*Link.offset(j as isize)).cLossOutlet * (q / a2);
    }
    if aMid > FUDGE {
        losses += (*Link.offset(j as isize)).cLossAvg * (q / aMid);
    }
    return losses;
}
unsafe extern "C" fn getSlotWidth(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut yNorm: ::core::ffi::c_double = y / (*xsect).yFull;
    if SurchargeMethod != SLOT as ::core::ffi::c_int
        || xsect_isOpen((*xsect).type_0) != 0
        || yNorm < CrownCutoff
    {
        return 0.0f64;
    }
    if yNorm > 1.78f64 {
        return 0.01f64 * (*xsect).wMax;
    }
    return (*xsect).wMax * 0.5423f64 * exp(-pow(yNorm, 2.4f64));
}
unsafe extern "C" fn getWidth(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut wSlot: ::core::ffi::c_double = getSlotWidth(xsect, y);
    if wSlot > 0.0f64 {
        return wSlot;
    }
    if y / (*xsect).yFull >= CrownCutoff && xsect_isOpen((*xsect).type_0) == 0 {
        y = CrownCutoff * (*xsect).yFull;
    }
    return xsect_getWofY(xsect, y);
}
unsafe extern "C" fn getArea(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
    mut wSlot: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y >= (*xsect).yFull {
        return (*xsect).aFull + (y - (*xsect).yFull) * wSlot;
    }
    return xsect_getAofY(xsect, y);
}
unsafe extern "C" fn getHydRad(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y >= (*xsect).yFull {
        return (*xsect).rFull;
    }
    return xsect_getRofY(xsect, y);
}
unsafe extern "C" fn checkNormalFlow(
    mut j: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    mut a1: ::core::ffi::c_double,
    mut r1: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut check: ::core::ffi::c_int = FALSE;
    let mut k: ::core::ffi::c_int = (*Link.offset(j as isize)).subIndex;
    let mut n1: ::core::ffi::c_int = (*Link.offset(j as isize)).node1;
    let mut n2: ::core::ffi::c_int = (*Link.offset(j as isize)).node2;
    let mut hasOutfall: ::core::ffi::c_int = ((*Node.offset(n1 as isize)).type_0
        == OUTFALL as ::core::ffi::c_int
        || (*Node.offset(n2 as isize)).type_0 == OUTFALL as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mut qNorm: ::core::ffi::c_double = 0.;
    let mut f1: ::core::ffi::c_double = 0.;
    if NormalFlowLtd == SLOPE as ::core::ffi::c_int
        || NormalFlowLtd == BOTH as ::core::ffi::c_int
        || hasOutfall != 0
    {
        if y1 < y2 {
            check = TRUE;
        }
    }
    if check == 0
        && (NormalFlowLtd == FROUDE as ::core::ffi::c_int
            || NormalFlowLtd == BOTH as ::core::ffi::c_int)
        && hasOutfall == 0
    {
        if y1 > FUDGE && y2 > FUDGE {
            f1 = link_getFroude(j, q / a1, y1);
            if f1 >= 1.0f64 {
                check = TRUE;
            }
        }
    }
    if check != 0 {
        qNorm = (*Conduit.offset(k as isize)).beta * a1 * pow(r1, 2.0f64 / 3.0f64);
        if qNorm < q {
            (*Link.offset(j as isize)).normalFlow = TRUE as ::core::ffi::c_char;
            return qNorm;
        }
    }
    return q;
}
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GRAVITY: ::core::ffi::c_double = 32.2f64;
pub const FUDGE: ::core::ffi::c_double = 0.0001f64;

#[repr(C)]
pub struct TInlet {
    _opaque: [u8; 0],
}
