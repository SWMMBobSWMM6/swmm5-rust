//! inlet.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   inlet.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     07/13/23 (Build 5.2.4)
//   Author:   L. Rossman
//
//   Street/Channel Inlet Functions
//
//   Computes capture efficiency of inlets placed in Street conduits
//   or Rectangular/Trapezoidal channels using FHWA HEC-22 methods (see
//   Brown, S.A. et al., Urban Drainage Design Manual, Federal Highway
//   Administration Hydraulic Engineering Circular No. 22, 3rd Edition,
//   FHWA-NHI-10-009, August 2013).
//
//   Build 5.2.1:
//   - Substitutes the constant BIG for HUGE.
//   Build 5.2.2:
//   - Additional statistics added to Street Flow Summary table.
//   Build 5.2.4:
//   - Fixed expression for equivalent gutter slope in getCurbInletCapture.
//   - Corrected sign in equation for effective head in a curb inlet
//     with an inclined throat opening in getCurbOrificeFlow.
//-----------------------------------------------------------------------------
//
// Representation notes:
// * C keeps the deployed inlets in a linked list headed by FirstInlet, built
//   by prepending (inlet_readUsageParams). Here `FirstInlet` is a Vec in C
//   list order (element 0 is the head). `TLink.inlet` is the position of the
//   link's inlet in that Vec; when the list changes shape (prepend, removal)
//   the positions stored in the links are shifted to match.
// * A `TInlet*` parameter of a C function is the inlet's position in
//   `FirstInlet` here.
// * The module-level pointer `xsect` (to the cross section of the current
//   inlet's conduit) is a copy of that TXsect: cross sections do not change
//   once the project is validated.

use crate::prelude::*;

// Grate inlet
#[derive(Clone, Copy, Default)]
struct TGrateInlet {
    type_: i32,              // type of grate used
    length: f64,             // length (parallel to flow) (ft)
    width: f64,              // width (perpendicular to flow) (ft)
    fracOpenArea: f64,       // fraction of grate area that is open
    splashVeloc: f64,        // splash-over velocity (ft/s)
}

// Slotted drain inlet
#[derive(Clone, Copy, Default)]
struct TSlottedInlet {
    length: f64,             // length (parallel to flow) (ft)
    width: f64,              // width (perpendicular to flow) (ft)
}

// Curb opening inlet
#[derive(Clone, Copy, Default)]
struct TCurbInlet {
    length: f64,             // length of curb opening (ft)
    height: f64,             // height of curb opening (ft)
    throatAngle: i32,        // type of throat angle
}

// Custom inlet
#[derive(Clone, Copy, Default)]
struct TCustomInlet {
    onGradeCurve: i32,       // flow diversion curve index
    onSagCurve: i32,         // flow rating curve index
}

// Inlet design object
#[derive(Clone, Default)]
struct TInletDesign {
    ID: Option<Vec<u8>>,           // name assigned to inlet design
    type_: i32,                    // type of inlet used (grate, curb, etc)
    grateInlet: TGrateInlet,       // length = 0 if not used
    slottedInlet: TSlottedInlet,   // length = 0 if not used
    curbInlet: TCurbInlet,         // length = 0 if not used
    customCurve: i32,              // curve index = -1 if not used
}

// Inlet performance statistics
#[derive(Clone, Copy, Default)]
struct TInletStats {
    flowPeriods: i32,        // # periods with approach flow
    capturePeriods: i32,     // # periods with captured flow
    backflowPeriods: i32,    // # periods with backflow
    peakFlow: f64,           // peak flow seen by inlet (cfs)
    peakFlowCapture: f64,    // capture efficiency at peak flow
    avgFlowCapture: f64,     // average capture efficiency
    bypassFreq: f64,         // frequency of bypass flow
}

// Inlet list object
#[derive(Clone, Copy, Default)]
struct TInlet {
    linkIndex: i32,          // index of conduit link with the inlet
    designIndex: i32,        // index of inlet's design
    nodeIndex: i32,          // index of node receiving captured flow
    numInlets: i32,          // # inlets on each side of street or in channel
    placement: i32,          // whether inlet is on-grade or on-sag
    clogFactor: f64,         // fractional degree of inlet clogging
    flowLimit: f64,          // inlet flow restriction (cfs)
    localDepress: f64,       // local gutter depression (ft)
    localWidth: f64,         // local depression width (ft)

    flowFactor: f64,         // flow = flowFactor * (flow spread)^2.67
    flowCapture: f64,        // captured flow rate (cfs)
    backflow: f64,           // backflow from capture node (cfs)
    backflowRatio: f64,      // inlet backflow / capture node overflow
    stats: TInletStats,      // inlet performance statistics
    // nextInlet: the next element of InletState.FirstInlet
}

/// The file-scope variables of inlet.c.
#[derive(Default)]
pub struct InletState {
    // Shared inlet variables
    InletDesigns: Vec<TInletDesign>, // array of available inlet designs
    InletDesignCount: i32,           // number of inlet designs
    UsesInlets: i32,                 // TRUE if project uses inlets

    // Variables as named in the HEC-22 manual.
    Sx: f64,            // street cross slope
    SL: f64,            // conduit longitudinal slope
    Sw: f64,            // gutter + cross slope
    a: f64,             // street gutter depression (ft)
    W: f64,             // street gutter width (ft)
    T: f64,             // top width of flow spread (ft)
    n: f64,             // Manning's roughness coeff.

    // Additional variables
    Nsides: i32,        // 1- or 2-sided street
    Tcrown: f64,        // distance from street curb to crown (ft)
    Beta: f64,          // = 1.486 * sqrt(SL) / n
    Qfactor: f64,       // factor f in Izzard's eqn. Q = f*T^2.67
    xsect: TXsect,      // cross-section data of inlet's conduit (C: a pointer to it)
    InletFlow: Vec<f64>,    // captured inlet flow received by each node
    FirstInlet: Vec<TInlet>, // list of deployed inlets (element 0 = head)
}

//-----------------------------------------------------------------------------
//  Enumerations
//-----------------------------------------------------------------------------

// enum InletType
const GRATE_INLET: i32 = 0;
const CURB_INLET: i32 = 1;
const COMBO_INLET: i32 = 2;
const SLOTTED_INLET: i32 = 3;
const DROP_GRATE_INLET: i32 = 4;
const DROP_CURB_INLET: i32 = 5;
const CUSTOM_INLET: i32 = 6;

// enum GrateType
const P50: i32 = 0;
const P50x100: i32 = 1;
const P30: i32 = 2;
const CURVED_VANE: i32 = 3;
const TILT_BAR_45: i32 = 4;
const TILT_BAR_30: i32 = 5;
const RETICULINE: i32 = 6;
const GENERIC: i32 = 7;

// enum InletPlacementType
const AUTOMATIC: i32 = 0;
const ON_GRADE: i32 = 1;
const ON_SAG: i32 = 2;

// enum ThroatAngleType
const HORIZONTAL_THROAT: i32 = 0;
const INCLINED_THROAT: i32 = 1;
const VERTICAL_THROAT: i32 = 2;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
static InletTypeWords: &[&[u8]] =
    &[b"GRATE", b"CURB", b"", b"SLOTTED", b"DROP_GRATE", b"DROP_CURB", b"CUSTOM"];

static GrateTypeWords: &[&[u8]] =
  &[b"P_BAR-50", b"P_BAR-50x100", b"P_BAR-30", b"CURVED_VANE", b"TILT_BAR-45", b"TILT_BAR-30",
    b"RETICULINE", b"GENERIC"];

static ThroatAngleWords: &[&[u8]] =
    &[b"HORIZONTAL", b"INCLINED", b"VERTICAL"];

// C declares this array without a NULL terminator; findmatch() reads one
// element past its end. In EPA's x86-64 GCC build the array is followed by
// 8 bytes of zero padding (checked in the reference binary), so the read
// finds NULL and the search stops after the 3 words, as it does here.
static PlacementTypeWords: &[&[u8]] =
    &[b"AUTOMATIC", b"ON_GRADE", b"ON_SAG"];

// Coefficients for cubic polynomials fitted to Splash Over Velocity v.
// Grate Length curves in Chart 5B of HEC-22 manual taken from Denver
// UDFCD manual.
static SplashCoeffs: [[f64; 4]; 7] = [
    [2.22, 4.03, 0.65, 0.06],     //P_BAR-50
    [0.74, 2.44, 0.27, 0.02],     //P_BAR-50x100
    [1.76, 3.12, 0.45, 0.03],     //P_BAR-30
    [0.30, 4.85, 1.31, 0.15],     //Curved_Vane
    [0.99, 2.64, 0.36, 0.03],     //Tilt_Bar-45
    [0.51, 2.34, 0.2, 0.01],      //Tilt_Bar-30
    [0.28, 2.28, 0.18, 0.01]];    //Reticuline

// Grate opening ratios (Chart 9B of HEC-22 manual)
static GrateOpeningRatios: [f64; 8] = [
    0.90,     //P_BAR-50
    0.80,     //P_BAR-50x100
    0.60,     //P_BAR-30
    0.35,     //Curved_Vane
    0.17,     //Tilt_Bar-45 (assumed)
    0.34,     //Tilt_Bar-30
    0.80,     //Reticuline
    1.00];    //Generic

//-----------------------------------------------------------------------------
//  External functions (declared in inlet.h)
//-----------------------------------------------------------------------------
// inlet_create              called by createObjects in project.c
// inlet_delete              called by deleteObjects in project.c
// inlet_readDesignParams    called by parseLine in input.c
// inlet_readUsageParams     called by parseLine in input.c
// inlet_validate            called by project_validate
// inlet_findCapturedFlows   called by routing_execute
// inlet_adjustQualInflows   called by routing_execute
// inlet_adjustQualOutflows  called by routing execute
// inlet_writeStatsReport    called by statsrpt_writeReport
// inlet_capturedFlow        called by findLinkMassFlow in qualrout.c

//=============================================================================

/// Called by: project.
pub fn inlet_create(prj: &mut Project, numInlets: i32) -> i32
//
//  Input:   numInlets = number of inlet designs to create
//  Output:  none
//  Purpose: creats a collection of inlet designs.
//
{
    let mut i: i32;

    prj.inlet.InletDesigns = Vec::new();
    prj.inlet.InletFlow = Vec::new();
    prj.inlet.InletDesignCount = 0;
    prj.inlet.UsesInlets = FALSE;
    prj.inlet.FirstInlet = Vec::new();
    prj.inlet.InletDesigns = vec![TInletDesign::default(); numInlets as usize];
    prj.inlet.InletDesignCount = numInlets;

    prj.inlet.InletFlow = vec![0.0; prj.Nobjects[NODE as usize] as usize];

    i = 0;
    while i < prj.inlet.InletDesignCount {
        let d = &mut prj.inlet.InletDesigns[i as usize];
        d.customCurve = -1;
        d.curbInlet.length = 0.0;
        d.grateInlet.length = 0.0;
        d.slottedInlet.length = 0.0;
        d.type_ = CUSTOM_INLET;
        i += 1;
    }
    0
}

//=============================================================================

/// Called by: project.
pub fn inlet_delete(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: frees all memory allocated for inlet analysis.
//
{
    prj.inlet.FirstInlet = Vec::new();
    prj.inlet.InletFlow = Vec::new();
    prj.inlet.InletDesigns = Vec::new();
}

//=============================================================================

/// Called by: input.
pub fn inlet_readDesignParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: extracts a set of inlet design parameters from a tokenized line
//           of the [INLETS] section of a SWMM input file.
//
//  Format of input line is:
//  ID  GRATE       Length  Width  GrateType  (OpenArea)  (SplashVeloc)
//  ID  CURB        Length  Height (ThroatType)
//  ID  SLOTTED     Length  Width
//  ID  DROP_GRATE  Length  Width  GrateType  (OpenArea)  (SplashVeloc)
//  ID  DROP_CURB   Length  Height
//  ID  CUSTOM      CurveID
//
{
    let i: i32;

    // --- check for minimum number of tokens
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that design ID already registered in project
    i = project_findObject(prj, INLET, &tok[0]);
    if i < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    prj.inlet.InletDesigns[i as usize].ID = project_findID(prj, INLET, &tok[0]);

    // --- retrieve type of inlet design
    prj.inlet.InletDesigns[i as usize].type_ = findmatch(&tok[1], InletTypeWords);

    // --- read inlet's design parameters
    match prj.inlet.InletDesigns[i as usize].type_ {
        GRATE_INLET | DROP_GRATE_INLET => readGrateInletParams(prj, i, tok, ntoks),
        CURB_INLET | DROP_CURB_INLET => readCurbInletParams(prj, i, tok, ntoks),
        SLOTTED_INLET => readSlottedInletParams(prj, i, tok, ntoks),
        CUSTOM_INLET => readCustomInletParams(prj, i, tok, ntoks),
        _ => error_setInpError(prj, ERR_KEYWORD, &tok[1]),
    }
}

//=============================================================================

/// Called by: input.
pub fn inlet_readUsageParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: extracts inlet usage parameters from a tokenized line
//           of the [INLET_USAGE] section of a SWMM input file.
//
//  Format of input line is:
//    linkID  inletID  nodeID (#Inlets %Clog  Qmax  aLocal wLocal placement)
//  where
//    linkID = ID name of link containing the inlet
//    inletID = ID name of inlet design being used
//    nodeID  = ID name of node receiving captured flow
//    #Inlets = number of identical inlets used (default = 1)
//    %Clog   = percent that inlet is clogged
//    Qmax    = maximum flow that inlet can capture (default = 0 (no limit))
//    aLocal  = local gutter depression (ft or m) (default = 0)
//    wLocal  = width of local gutter depression (ft or m) (default = 0)
//    placement = ON_GRADE, ON_SAG, or AUTO (the default)
//
{
    let linkIndex: i32;
    let designIndex: i32;
    let nodeIndex: i32;
    let mut numInlets: i32 = 1;
    let mut placement: i32 = AUTOMATIC;
    let mut flowLimit: f64 = 0.0;
    let mut pctClogged: f64 = 0.0;
    let mut aLocal: f64 = 0.0;
    let mut wLocal: f64 = 0.0;
    let inlet: usize;

    // --- check that inlet's link exists
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    linkIndex = project_findObject(prj, LINK, &tok[0]);
    if linkIndex < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- check that inlet design type exists
    designIndex = project_findObject(prj, INLET, &tok[1]);
    if designIndex < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }

    // --- check that receiving node exists
    nodeIndex = project_findObject(prj, NODE, &tok[2]);
    if nodeIndex < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }

    // --- get number of inlets
    if ntoks > 3 {
        if getInt(&tok[3], &mut numInlets) == 0 || numInlets < 1 {
            return error_setInpError(prj, ERR_NUMBER, &tok[3]);
        }
    }

    // --- get flow limit & percent clogged
    if ntoks > 4 {
        if getDouble(&tok[4], &mut pctClogged) == 0 || pctClogged < 0.0
            || pctClogged > 99.
        {
            return error_setInpError(prj, ERR_NUMBER, &tok[4]);
        }
    }
    if ntoks > 5 {
        if getDouble(&tok[5], &mut flowLimit) == 0 || flowLimit < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[5]);
        }
    }

    // --- get local depression parameters
    if ntoks > 6 {
        if getDouble(&tok[6], &mut aLocal) == 0 || aLocal < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[6]);
        }
    }
    if ntoks > 7 {
        if getDouble(&tok[7], &mut wLocal) == 0 || wLocal < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[7]);
        }
    }

    // --- get inlet placement
    if ntoks > 8 {
        placement = findmatch(&tok[8], PlacementTypeWords);
        if placement < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[8]);
        }
    }

    // --- create an inlet usage object for the link
    match prj.Link[linkIndex as usize].inlet {
        Some(k) => inlet = k,
        None => {
            // C: inlet = malloc(...); Link[linkIndex].inlet = inlet;
            //    inlet->nextInlet = FirstInlet; FirstInlet = inlet;
            // Prepending moves every inlet already listed one place down,
            // so the positions held by their links move with them.
            for k in 0..prj.inlet.FirstInlet.len() {
                let li = prj.inlet.FirstInlet[k].linkIndex;
                prj.Link[li as usize].inlet = Some(k + 1);
            }
            prj.inlet.FirstInlet.insert(0, TInlet::default());
            prj.Link[linkIndex as usize].inlet = Some(0);
            inlet = 0;
        }
    }

    // --- save inlet usage parameters
    let ucfFlow = UCF(prj, FLOW);
    let ucfLength = UCF(prj, LENGTH);
    let it = &mut prj.inlet.FirstInlet[inlet];
    it.linkIndex = linkIndex;
    it.designIndex = designIndex;
    it.nodeIndex = nodeIndex;
    it.numInlets = numInlets;
    it.placement = placement;
    it.clogFactor = 1.0 - (pctClogged / 100.);
    it.flowLimit = flowLimit / ucfFlow;
    it.localDepress = aLocal / ucfLength;
    it.localWidth = wLocal / ucfLength;
    it.flowFactor = 0.0;
    it.backflowRatio = 0.0;
    initInletStats(it);
    prj.inlet.UsesInlets = TRUE;
    0
}

//=============================================================================

/// Called by: project.
pub fn inlet_validate(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: checks that inlets have been assigned to conduits with proper
//           cross section shapes and counts the number of inlets that each
//           node receives either bypased or captured flow from.
//
{
    let mut i: i32;
    let mut j: i32;
    let mut inletType: i32;
    let mut inletValid: i32;
    let mut inlet: usize;

    // --- traverse the list of inlets placed in conduits
    if prj.inlet.UsesInlets == 0 {
        return;
    }
    inlet = 0;
    while inlet < prj.inlet.FirstInlet.len() {
        // --- check that inlet's conduit can accept the inlet's type
        inletValid = FALSE;
        i = prj.inlet.FirstInlet[inlet].linkIndex;
        prj.inlet.xsect = prj.Link[i as usize].xsect;
        let designIndex = prj.inlet.FirstInlet[inlet].designIndex;
        inletType = prj.inlet.InletDesigns[designIndex as usize].type_;
        let xtype = prj.inlet.xsect.type_;
        if inletType == CUSTOM_INLET {
            j = prj.inlet.InletDesigns[designIndex as usize].customCurve;
            if j >= 0 {
                if prj.Curve[j as usize].curveType == DIVERSION_CURVE
                    || prj.Curve[j as usize].curveType == RATING_CURVE
                {
                    inletValid = TRUE;
                }
            }
        } else if (xtype == TRAPEZOIDAL || xtype == RECT_OPEN)
            && (inletType == DROP_GRATE_INLET || inletType == DROP_CURB_INLET)
        {
            inletValid = TRUE;
        } else if xtype == STREET_XSECT
            && inletType != DROP_GRATE_INLET
            && inletType != DROP_CURB_INLET
        {
            inletValid = TRUE;
        }

        // --- if inlet placement is valid then
        if inletValid != 0 {
            // --- record that receptor node has inlets
            let node2 = prj.Link[i as usize].node2;
            prj.Node[node2 as usize].inlet = BYPASS;
            let nodeIndex = prj.inlet.FirstInlet[inlet].nodeIndex;
            prj.Node[nodeIndex as usize].inlet = CAPTURE;

            // --- initialize inlet's backflow
            prj.inlet.FirstInlet[inlet].backflow = 0.0;

            // --- compute street inlet's flow factor for Izzard's eqn.
            //     (used in Q = flowFactor * Spread^2.67 equation)
            getConduitGeometry(prj, inlet);
            let s = &prj.inlet;
            let ff = (0.56 / s.n) * s.SL.powf(0.5) * s.Sx.powf(1.67);
            prj.inlet.FirstInlet[inlet].flowFactor = ff;

            // --- save reference to current inlet & continue to next inlet
            inlet += 1;
        }
        // --- if inlet placement is not valid then issue a warning message
        //     and remove the inlet from the conduit
        else {
            let id = prj.Link[i as usize].ID.clone();
            report_writeWarningMsg(prj, WARN12, &id);
            // C unlinks the inlet from the list and frees it
            prj.inlet.FirstInlet.remove(inlet);
            prj.Link[i as usize].inlet = None;
            // the inlets that followed it move up one place
            for k in inlet..prj.inlet.FirstInlet.len() {
                let li = prj.inlet.FirstInlet[k].linkIndex;
                prj.Link[li as usize].inlet = Some(k);
            }
        }
    }

    // --- determine how capture node's overflow is split between its inlets
    getBackflowRatios(prj);
}

//=============================================================================

/// Called by: routing.
pub fn inlet_findCapturedFlows(prj: &mut Project, tStep: f64)
//
//  Input:   tStep = current flow routing time step (sec)
//  Output:  none
//  Purpose: computes flow captured by each inlet and adjusts the
//           lateral flows of the inlet's bypass and capture nodes accordingly.
//
//  This function is called after regular lateral flows to all nodes have been
//  set but before a flow routing step has been taken.
{
    let mut i: i32;
    let mut j: i32;
    let mut m: i32;
    let mut placement: i32;
    let mut q: f64;

    // --- For non-DW routing find conduit flow into each node
    //     (used to limit max. amount of on-sag capture)
    if prj.inlet.UsesInlets == 0 {
        return;
    }
    for x in prj.inlet.InletFlow.iter_mut() {
        *x = 0.0;
    }
    if prj.RouteModel != DW {
        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            prj.Node[j as usize].inflow = MAX(0., prj.Node[j as usize].newLatFlow);
            j += 1;
        }
        i = 0;
        while i < prj.Nobjects[LINK as usize] {
            let node2 = prj.Link[i as usize].node2;
            prj.Node[node2 as usize].inflow += MAX(0.0, prj.Link[i as usize].newFlow);
            i += 1;
        }
    }

    // --- loop through each inlet
    for inlet in 0..prj.inlet.FirstInlet.len() {
        // --- identify indexes of inlet's bypass (j) and capture (m) nodes
        i = prj.inlet.FirstInlet[inlet].linkIndex;
        j = prj.Link[i as usize].node2;
        m = prj.inlet.FirstInlet[inlet].nodeIndex;

        // --- get inlet's placement (ON_GRADE or ON_SAG)
        placement = getInletPlacement(prj, inlet, j);

        // --- find flow captured by a Custom inlet
        let designIndex = prj.inlet.FirstInlet[inlet].designIndex;
        let flowCapture;
        if prj.inlet.InletDesigns[designIndex as usize].type_ == CUSTOM_INLET {
            q = prj.Link[i as usize].newFlow.abs();
            let d = prj.Node[j as usize].newDepth;
            flowCapture = getCustomCapturedFlow(prj, inlet, q, d);
        }
        // --- find flow captured by on-grade inlet
        else if placement == ON_GRADE {
            q = prj.Link[i as usize].newFlow.abs();
            let d = prj.Node[j as usize].newDepth;
            flowCapture = getOnGradeCapturedFlow(prj, inlet, q, d);
        }
        // --- find flow captured by on-sag inlet
        else {
            q = prj.Node[j as usize].inflow;
            let d = prj.Node[j as usize].newDepth;
            flowCapture = getOnSagCapturedFlow(prj, inlet, q, d);
        }
        prj.inlet.FirstInlet[inlet].flowCapture = flowCapture;
        if prj.inlet.FirstInlet[inlet].flowCapture.abs() < FUDGE {
            prj.inlet.FirstInlet[inlet].flowCapture = 0.0;
        }

        // --- add to total flow captured by inlet's node
        prj.inlet.InletFlow[j as usize] += prj.inlet.FirstInlet[inlet].flowCapture;

        // --- capture node's overflow becomes inlet's backflow
        let it = &mut prj.inlet.FirstInlet[inlet];
        it.backflow = prj.Node[m as usize].overflow * it.backflowRatio;
        if it.backflow.abs() < FUDGE {
            it.backflow = 0.0;
        }
    }

    // --- make second pass through each inlet
    for inlet in 0..prj.inlet.FirstInlet.len() {
        // --- identify indexes of inlet's bypass (j) and capture (m) nodes
        i = prj.inlet.FirstInlet[inlet].linkIndex;
        j = prj.Link[i as usize].node2;
        m = prj.inlet.FirstInlet[inlet].nodeIndex;

        // --- for on-sag placement under non-DW routing, captured flow
        //     is limited to inlet's share of bypass node's inflow plus
        //     any stored volume
        if prj.RouteModel != DW && getInletPlacement(prj, inlet, j) == ON_SAG {
            q = prj.Node[j as usize].newVolume / tStep;
            q += MAX(prj.Node[j as usize].inflow, 0.0);
            if prj.inlet.InletFlow[j as usize] > q {
                let r = q / prj.inlet.InletFlow[j as usize];
                prj.inlet.FirstInlet[inlet].flowCapture *= r;
            }
        }

        // --- adjust lateral flows at bypass and capture nodes
        //     (subtract captured flow from bypass node, add it to capture
        //     node, and add any backflow to bypass node)
        let flowCapture = prj.inlet.FirstInlet[inlet].flowCapture;
        let backflow = prj.inlet.FirstInlet[inlet].backflow;
        prj.Node[j as usize].newLatFlow -= flowCapture - backflow;
        prj.Node[m as usize].newLatFlow += flowCapture;

        // --- update inlet's performance if reporting has begun
        if getDateTime(prj, prj.NewRoutingTime) > prj.ReportStart {
            let qa = prj.Link[i as usize].newFlow.abs();
            updateInletStats(&mut prj.inlet.FirstInlet[inlet], qa);
        }
    }
}

//=============================================================================

/// Called by: routing.
pub fn inlet_adjustQualInflows(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: adjusts accumulated flow rates and pollutant mass inflows at each
//           inlet's bypass and capture nodes after a flow routing step has
//           been taken prior to a quality routing step.
//
{
    let mut i: i32;
    let mut j: i32;
    let mut m: i32;
    let mut p: i32;
    let mut qNet: f64;

    if prj.inlet.UsesInlets == 0 {
        return;
    }
    if prj.IgnoreQuality != 0 || prj.Nobjects[POLLUT as usize] == 0 {
        return;
    }
    for inlet in 0..prj.inlet.FirstInlet.len() {
        // --- identify indexes of inlet's bypass (j) and capture (m) nodes
        i = prj.inlet.FirstInlet[inlet].linkIndex;
        j = prj.Link[i as usize].node2;
        m = prj.inlet.FirstInlet[inlet].nodeIndex;

        // --- there's a net flow from the bypass to the capture node
        qNet = prj.inlet.FirstInlet[inlet].flowCapture - prj.inlet.FirstInlet[inlet].backflow;
        if qNet > 0.0 {
            // --- add net capture flow to capture node's accumulated flow
            //     inflow for quality routing
            prj.Node[m as usize].qualInflow += qNet;

            // --- and do the same for pollutant mass flows
            //     (Node[m].newQual is the mass inflow accumulator for node m)
            p = 0;
            while p < prj.Nobjects[POLLUT as usize] {
                let w = qNet * prj.Node[j as usize].oldQual[p as usize];
                prj.Node[m as usize].newQual[p as usize] += w;
                p += 1;
            }
        }
        // --- there's a net backflow from the capture to the bypass node
        else {
            // --- add the backflow flow rate and pollutant mass flow to the
            //     bypass node's accumulated flow and pollutant mass inflow
            qNet = -qNet;
            prj.Node[j as usize].qualInflow += qNet;
            p = 0;
            while p < prj.Nobjects[POLLUT as usize] {
                let w = qNet * prj.Node[m as usize].oldQual[p as usize];
                prj.Node[j as usize].newQual[p as usize] += w;
                p += 1;
            }
        }
    }
}

//=============================================================================

/// Called by: routing.
pub fn inlet_adjustQualOutflows(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: adjusts mass balance totals after a complete routing step has been
//           taken so as not to treat inlet transfer flows as system outflows.
//
{
    let mut j: i32;
    let mut p: i32;
    let mut q: f64;
    let mut w: f64;

    // --- these variables, declared in massbal.c, accumulate system-wide flow and
    //     pollutant mass fluxes over a time step to use in mass balances
    //     (prj.massbal.StepFlowTotals, prj.massbal.StepQualTotals)

    // --- examine each node
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        // --- node receives captured flow from an inlet
        if prj.Node[j as usize].inlet == CAPTURE {
            // --- node also has an overflow (e.g., it's a surcharged sewer node)
            q = prj.Node[j as usize].overflow;
            if q > 0.0 {
                // --- remove overflow from system flooding total since it does
                //     not leave the system (it is sent to inlet's bypass node)
                prj.massbal.StepFlowTotals.flooding -= q;

                // --- also remove pollutant overflow mass from system totals
                if prj.IgnoreQuality == 0 {
                    p = 0;
                    while p < prj.Nobjects[POLLUT as usize] {
                        w = q * prj.Node[j as usize].newQual[p as usize];
                        prj.massbal.StepQualTotals[p as usize].flooding -= w;
                        p += 1;
                    }
                }
            }
        }
        j += 1;
    }

    // --- for WQ analysis, examine each inlet's bypass node
    if prj.IgnoreQuality == 0 && prj.Nobjects[POLLUT as usize] > 0 {
        for inlet in 0..prj.inlet.FirstInlet.len() {
            let li = prj.inlet.FirstInlet[inlet].linkIndex;
            j = prj.Link[li as usize].node2;

            // --- inlet has net positive flow capture leading to
            //      node having a net negative lateral inflow
            q = prj.inlet.FirstInlet[inlet].flowCapture - prj.inlet.FirstInlet[inlet].backflow;
            if q > 0.0 && prj.Node[j as usize].newLatFlow < 0.0 {
                // --- remove the pollutant mass in the captured flow from
                //     the system totals since it does not leave the system
                //     (it is sent to the inlet's capture node)
                p = 0;
                while p < prj.Nobjects[POLLUT as usize] {
                    w = q * prj.Node[j as usize].newQual[p as usize];
                    prj.massbal.StepQualTotals[p as usize].outflow -= w;
                    p += 1;
                }
            }
        }
    }
}

//=============================================================================

/// Called by: statsrpt.
pub fn inlet_writeStatsReport(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: writes table of street & inlet flow statistics to SWMM's report file.
//
{
    let mut j: i32;
    let mut header: i32 = FALSE;

    if prj.Nobjects[STREET as usize] == 0 {
        return;
    }
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        if prj.Link[j as usize].xsect.type_ == STREET_XSECT {
            if header == 0 {
                writeStreetStatsHeader(prj);
                header = TRUE;
            }
            writeStreetStats(prj, j);
        }
        j += 1;
    }
    report_writeLine(prj, b"");
}

//=============================================================================

/// Called by: qualrout.
pub fn inlet_capturedFlow(prj: &Project, i: i32) -> f64
//
//  Input:   i = a link index
//  Output:  returns captured flow rate (cfs)
//  Purpose: gets the current flow captured by an inlet.
//
{
    if let Some(k) = prj.Link[i as usize].inlet {
        return prj.inlet.FirstInlet[k].flowCapture;
    }
    0.0
}

//=============================================================================

fn readGrateInletParams(prj: &mut Project, i: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   i = inlet index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: extracts a grate's inlet parameters from a set of string tokens.
//
{
    let grateType: i32;
    let mut width: f64 = 0.0;
    let mut length: f64 = 0.0;
    let mut areaRatio: f64 = 0.0;
    let mut vSplash: f64 = 0.0;

    // --- check for enough tokens
    if ntoks < 5 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- retrieve length & width
    if getDouble(&tok[2], &mut length) == 0 || length <= 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[2]);
    }
    if getDouble(&tok[3], &mut width) == 0 || width <= 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[3]);
    }

    // --- retrieve grate type
    grateType = findmatch(&tok[4], GrateTypeWords);
    if grateType < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[4]);
    }

    // --- only read open area & splash velocity for GENERIC type grate
    if grateType == GENERIC {
        if ntoks < 6 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if getDouble(&tok[5], &mut areaRatio) == 0 || areaRatio <= 0.0
            || areaRatio > 1.0
        {
            return error_setInpError(prj, ERR_NUMBER, &tok[5]);
        }
        if ntoks > 6 {
            if getDouble(&tok[6], &mut vSplash) == 0 || vSplash < 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[6]);
            }
        }
    }

    // --- save grate inlet parameters
    let ucfLength = UCF(prj, LENGTH);
    let d = &mut prj.inlet.InletDesigns[i as usize];
    d.grateInlet.length = length / ucfLength;
    d.grateInlet.width = width / ucfLength;
    d.grateInlet.type_ = grateType;
    d.grateInlet.fracOpenArea = areaRatio;
    d.grateInlet.splashVeloc = vSplash / ucfLength;

    // --- check if grate is part of a combo inlet
    if d.type_ == GRATE_INLET && d.curbInlet.length > 0.0 {
        d.type_ = COMBO_INLET;
    }
    0
}

//=============================================================================

fn readCurbInletParams(prj: &mut Project, i: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   i = inlet index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: extracts curb opening inlet parameters from a set of string tokens.
//
{
    let mut throatAngle: i32;
    let mut height: f64 = 0.0;
    let mut length: f64 = 0.0;

    // --- check for enough tokens
    if ntoks < 4 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- retrieve length & width of opening
    if getDouble(&tok[2], &mut length) == 0 || length <= 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[2]);
    }
    if getDouble(&tok[3], &mut height) == 0 || height <= 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[3]);
    }

    // --- retrieve type of throat angle for curb inlet
    throatAngle = VERTICAL_THROAT;
    if prj.inlet.InletDesigns[i as usize].type_ == CURB_INLET && ntoks > 4 {
        throatAngle = findmatch(&tok[4], ThroatAngleWords);
        if throatAngle < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[4]);
        }
    }

    // ---- save curb opening inlet parameters
    let ucfLength = UCF(prj, LENGTH);
    let d = &mut prj.inlet.InletDesigns[i as usize];
    d.curbInlet.length = length / ucfLength;
    d.curbInlet.height = height / ucfLength;
    d.curbInlet.throatAngle = throatAngle;

    // --- check if curb inlet is part of a combo inlet
    if d.type_ == CURB_INLET && d.grateInlet.length > 0.0 {
        d.type_ = COMBO_INLET;
    }
    0
}

//=============================================================================

fn readSlottedInletParams(prj: &mut Project, i: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   i = inlet index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: extracts slotted drain inlet parameters from a set of string tokens.
//
{
    let mut width: f64 = 0.0;
    let mut length: f64 = 0.0;

    // --- check for enough tokens
    if ntoks < 4 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- retrieve length and width
    if getDouble(&tok[2], &mut length) == 0 || length <= 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[2]);
    }
    if getDouble(&tok[3], &mut width) == 0 || width <= 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[3]);
    }

    // --- save slotted inlet parameters
    let ucfLength = UCF(prj, LENGTH);
    let d = &mut prj.inlet.InletDesigns[i as usize];
    d.slottedInlet.length = length / ucfLength;
    d.slottedInlet.width = width / ucfLength;
    0
}

//=============================================================================

fn readCustomInletParams(prj: &mut Project, i: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   i = inlet index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: extracts custom inlet parameters from a set of string tokens.
//
{
    let c: i32;    // capture curve index

    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    } else {
        c = project_findObject(prj, CURVE, &tok[2]);
        if c < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[2]);
        }
    }
    prj.inlet.InletDesigns[i as usize].customCurve = c;
    0
}

//=============================================================================

fn initInletStats(inlet: &mut TInlet)
//
//  Input:   inlet = an inlet object placed in a conduit link
//  Output:  none
//  Purpose: initializes the performance statistics of an inlet.
//
{
    // C: if (inlet) -- the inlet always exists here
    inlet.flowCapture = 0.0;
    inlet.backflow = 0.0;
    inlet.stats.flowPeriods = 0;
    inlet.stats.capturePeriods = 0;
    inlet.stats.backflowPeriods = 0;
    inlet.stats.peakFlow = 0.0;
    inlet.stats.peakFlowCapture = 0.0;
    inlet.stats.avgFlowCapture = 0.0;
    inlet.stats.bypassFreq = 0.0;
}

//=============================================================================

fn updateInletStats(inlet: &mut TInlet, q: f64)
//
//  Input:   inlet = an inlet object placed in a conduit link
//           q = inlet's approach flow (cfs)
//  Output:  none
//  Purpose: updates the performance statistics of an inlet.
//
{
    let qCapture = inlet.flowCapture;
    let qBackflow = inlet.backflow;
    let qNet = qCapture - qBackflow;
    let qBypass = q - qNet;
    let mut fCapture = 0.0;

    // --- check for no flow condition
    if q < MIN_RUNOFF_FLOW && qBackflow <= 0.0 {
        return;
    }
    inlet.stats.flowPeriods += 1;

    // --- there is positive net flow from inlet to capture node
    if qNet > 0.0 {
        inlet.stats.capturePeriods += 1;
        fCapture = qNet / q;
        fCapture = MIN(fCapture, 1.0);
        inlet.stats.avgFlowCapture += fCapture;
        if qBypass > MIN_RUNOFF_FLOW {
            inlet.stats.bypassFreq += 1.0;
        }
    }
    // --- otherwise inlet receives backflow from capture node
    else {
        inlet.stats.backflowPeriods += 1;
    }

    // --- update peak flow stats
    if q > inlet.stats.peakFlow {
        inlet.stats.peakFlow = q;
        inlet.stats.peakFlowCapture = fCapture * 100.0;
    }
}

//=============================================================================

fn writeStreetStatsHeader(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: writes column headers for Street Flow Summary table to SWMM's report file.
//
{
    report_writeLine(prj, b"");
    report_writeLine(prj, b"*******************");
    report_writeLine(prj, b"Street Flow Summary");
    report_writeLine(prj, b"*******************");
    report_writeLine(prj, b"");
    cprintf!(prj.Frpt,
b"\n  ---------------------------------------------------------------------------------------------------------------------------------------\n                                                                                        Peak     Avg.   Bypass     Back     Peak     Peak\n                        Peak   Maximum   Maximum                                        Flow     Flow     Flow     Flow  Capture   Bypass\n                        Flow    Spread     Depth  Inlet             Inlet     Inlet  Capture  Capture     Freq     Freq  / Inlet     Flow");
    let fu = FlowUnitWords[prj.FlowUnits as usize];
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
b"\n  Street Conduit         %3s        ft        ft  Design            Location  Count     Pcnt     Pcnt     Pcnt     Pcnt      %3s      %3s",
            fu, fu, fu);
    } else {
        cprintf!(prj.Frpt,
b"\n  Street Conduit         %3s         m         m  Design            Location            Pcnt     Pcnt     Pcnt     Pcnt      %3s      %3s",
            fu, fu, fu);
    }
    cprintf!(prj.Frpt,
b"\n  ---------------------------------------------------------------------------------------------------------------------------------------");
}

//=============================================================================

fn writeStreetStats(prj: &mut Project, link: i32)
//
//  Input:   link = index of a conduit link containing an inlet
//  Output:  none
//  Purpose: writes flow statistics for a Street conduit and its inlet to
//           SWMM's report file.
//
{
    let k: i32;
    let t: i32;
    let placement: i32;
    let mut maxSpread: f64;
    let maxDepth: f64;
    let maxFlow: f64;
    let fp: f64;
    let cp: f64;
    let mut afc: f64 = 0.0;
    let mut bpf: f64 = 0.0;
    let inlet: Option<usize>;

    // --- retrieve street parameters
    k = prj.Link[link as usize].subIndex;
    t = prj.Link[link as usize].xsect.transect;
    inlet = prj.Link[link as usize].inlet;

    // --- get recorded max flow and depth
    maxFlow = prj.stats.LinkStats[link as usize].maxFlow;
    maxDepth = prj.stats.LinkStats[link as usize].maxDepth;

    // --- SWMM's spread (flow width) at max depth
    maxSpread = xsect_getWofY(prj, &prj.Link[link as usize].xsect, maxDepth)
        / prj.Street[t as usize].sides as f64;
    maxSpread = MIN(maxSpread, prj.Street[t as usize].width);
/*
    // HEC-22's spread based on max flow (doesn't account for backwater)
    Sx = Street[t].slope;
    a = Street[t].gutterDepression;
    W = Street[t].gutterWidth;
    n = Street[t].roughness;
    Qfactor = (0.56 / n) * sqrt(Conduit[k].slope) * pow(Sx, 1.67);
    maxSpread = getFlowSpread(maxFlow / Street[t].sides);
    maxSpread = MIN(maxSpread, Street[t].width);
*/
    // --- write street stats
    let ucfFlow = UCF(prj, FLOW);
    let ucfLength = UCF(prj, LENGTH);
    let id = prj.Link[link as usize].ID.clone();
    cprintf!(prj.Frpt, b"\n  %-16s", id);
    cprintf!(prj.Frpt, b" %9.3f", maxFlow * ucfFlow);
    cprintf!(prj.Frpt, b" %9.3f", maxSpread * ucfLength);
    cprintf!(prj.Frpt, b" %9.3f", maxDepth * ucfLength);

    // --- write inlet stats
    if let Some(inlet) = inlet {
        let it = prj.inlet.FirstInlet[inlet];
        // C prints a NULL ID with %s as "(null)" (glibc)
        let designID: Vec<u8> = match &prj.inlet.InletDesigns[it.designIndex as usize].ID {
            Some(s) => s.clone(),
            None => b"(null)".to_vec(),
        };
        cprintf!(prj.Frpt, b"  %-16s", designID);
        let node2 = prj.Link[it.linkIndex as usize].node2;
        placement = getInletPlacement(prj, inlet, node2);
        if placement == ON_GRADE {
            cprintf!(prj.Frpt, b"  ON-GRADE");
        } else {
            cprintf!(prj.Frpt, b"  ON-SAG  ");
        }
        cprintf!(prj.Frpt, b"  %5d", it.numInlets);
        fp = it.stats.flowPeriods as f64 / 100.0;
        if fp > 0.0 {
            cp = it.stats.capturePeriods as f64 / 100.0;
            cprintf!(prj.Frpt, b"  %7.2f", it.stats.peakFlowCapture);
            if cp > 0.0 {
                afc = it.stats.avgFlowCapture / cp;
                bpf = it.stats.bypassFreq / cp;
            }
            cprintf!(prj.Frpt, b"  %7.2f", afc);
            cprintf!(prj.Frpt, b"  %7.2f", bpf);
            cprintf!(prj.Frpt, b"  %7.2f", it.stats.backflowPeriods as f64 / fp);
            let sides = prj.Street[t as usize].sides;
            cprintf!(prj.Frpt, b"  %7.2f", (maxFlow / sides as f64) * ucfFlow *
                0.01 * it.stats.peakFlowCapture / it.numInlets as f64);
            cprintf!(prj.Frpt, b"  %7.2f", maxFlow * ucfFlow * 0.01 *
                (100.0 - it.stats.peakFlowCapture));
        }
    }
}

//=============================================================================

fn getInletPlacement(prj: &Project, inlet: usize, j: i32) -> i32
//
//  Input:   inlet = an inlet object placed in a conduit link
//           j = index of inlet's bypass node
//  Output:  returns type of inlet placement
//  Purpose: determines actual placement for an inlet with AUTOMATIC placement.
//
{
    let placement = prj.inlet.FirstInlet[inlet].placement;
    if placement == AUTOMATIC {
        if prj.Node[j as usize].degree > 0 {
            ON_GRADE
        } else {
            ON_SAG
        }
    } else {
        placement
    }
}

//=============================================================================

fn getConduitGeometry(prj: &mut Project, inlet: usize)
//
//  Input:   inlet = an inlet object placed in a conduit link
//  Output:  none
//  Purpose: assigns properties of an inlet's conduit to
//           module-level shared variables used by other functions.
//
{
    let linkIndex = prj.inlet.FirstInlet[inlet].linkIndex;
    let t: i32;
    let k = prj.Link[linkIndex as usize].subIndex;

    // C reads Conduit[k] whatever the link's type. A custom inlet (valid on
    // any link) or a drop inlet on a weir/orifice with an open rectangular or
    // trapezoidal shape makes k an index into another link type's array:
    // C then reads another conduit's data, or past the end of Conduit[]
    // (undefined). Out of range, this uses a zeroed conduit instead.
    let cond = prj.Conduit.get(k as usize).copied().unwrap_or_default();
    prj.inlet.SL = cond.slope;                         // longitudinal slope
    prj.inlet.Beta = cond.beta;                        // 1.486 * sqrt(SL) / n
    prj.inlet.xsect = prj.Link[linkIndex as usize].xsect;

    // --- if conduit has a Street cross section
    if prj.inlet.xsect.type_ == STREET_XSECT {
        t = prj.inlet.xsect.transect;
        let st = &prj.Street[t as usize];
        let s = &mut prj.inlet;
        s.Sx = st.slope;                     // street cross slope
        s.a = st.gutterDepression;           // gutter depression
        s.W = st.gutterWidth;                // gutter width
        s.n = st.roughness;                  // street roughness
        s.Nsides = st.sides;                 // 1 or 2 sided street
        s.Tcrown = st.width;                 // distance from curb to crown
        s.Qfactor = s.FirstInlet[inlet].flowFactor; // factor used in Izzard's eqn.

        // --- add inlet's local depression to street's continuous depression
        //     (C: if (inlet && ...) -- the inlet always exists here)
        let localDepress = s.FirstInlet[inlet].localDepress;
        let localWidth = s.FirstInlet[inlet].localWidth;
        if localDepress * localWidth > 0.0 {
            s.a += localDepress;            // inlet depression
            s.W = localWidth;               // inlet depressed width
        }

        // --- slope of depressed gutter section
        if s.W * s.a > 0.0 {
            s.Sw = s.Sx + s.a / s.W;
        } else {
            s.Sw = s.Sx;
        }
    }
    // --- conduit has rectangular or trapezoidal cross section
    else {
        let roughness = cond.roughness;
        let s = &mut prj.inlet;
        s.a = 0.0;
        s.W = 0.0;
        s.n = roughness;
        s.Nsides = 1;
        s.Sx = 0.01;
        s.Sw = s.Sx;
    }
}

//=============================================================================

fn getFlowSpread(prj: &Project, Q: f64) -> f64
//
//  Input:   Q = conduit flow rate (cfs)
//  Output:  returns width of flow spread (ft)
//  Purpose: computes width of flow spread across a Street cross section using
//           HEC-22 equations derived from Izzard's form of the Manning eqn.
//
{
    let s = &prj.inlet;
    let mut iter: i32;
    let f: f64;
    let f1: f64;
    let Sr: f64;
    let mut Ts1: f64;
    // C: uninitialized; the loop below runs at least once and sets it
    let mut Ts2: f64 = 0.0;
    let Tw: f64;
    let mut Qs: f64;
    let mut Eo: f64;

    f = s.Qfactor;   // = (0.56/n) * SL^0.5 * Sx^1.67

    // --- no depressed curb
    if s.a == 0.0 {
        Ts1 = (Q / f).powf(0.375);                         //HEC-22 Eq(4-2)
    } else {
        // --- check if spread is within curb width
        f1 = f * ((s.a / s.W) / s.Sx).powf(1.67);
        Tw = (Q / f1).powf(0.375);                         //HEC-22 Eq(4-2)
        if Tw <= s.W {
            Ts1 = Tw;
        } else {
            // --- spread extends beyond curb width
            Sr = (s.Sx + s.a / s.W) / s.Sx;
            iter = 1;
            Ts1 = (Q / f).powf(0.375) - s.W;
            if Ts1 <= 0.0 {
                Ts1 = Tw - s.W;
            }
            while iter < 11 {
                Eo = getEo(Sr, Ts1, s.W);
                Qs = (1.0 - Eo) * Q;                       //HEC-22 Eq(4-6)
                Ts2 = (Qs / f).powf(0.375);                //HEC-22 Eq(4-2)
                if (Ts2 - Ts1).abs() < 0.01 {
                    break;
                }
                Ts1 = Ts2;
                iter += 1;
            }
            Ts1 = Ts2 + s.W;
        }
    }
    MIN(Ts1, s.Tcrown)
}

//=============================================================================

fn getEo(Sr: f64, Ts: f64, w: f64) -> f64
//
//  Input:   Sr = ratio of gutter slope to street cross slope
//           Ts = amount of flow spread outside of gutter width (ft)
//           w  = gutter width (ft)
//  Output:  returns ratio of gutter flow to total flow in street cross section
//  Purpose: solves HEC-22 Eq. (4-4) for Eo with Ts/w substituted for
//           (T/w) - 1 where Ts = T - w.
//
{
    let mut x: f64;
    x = Sr / (Ts / w);
    x = (1.0 + x).powf(2.67) - 1.0;
    x = 1.0 + Sr / x;
    1.0 / x
}

//=============================================================================

fn getOnGradeCapturedFlow(prj: &mut Project, inlet: usize, q: f64, d: f64) -> f64
//
//  Input:   inlet = an inlet object placed in a conduit link
//           q = flow in link prior to any inlet capture (cfs)
//           d = flow depth seen by inlet (ft)
//  Output:  returns flow captured by the inlet (cfs)
//  Purpose: computes flow captured by an inlet placed on-grade.
//
//  An inlet object placed in a conduit can have multiple inlets of
//  the same type distributed along the conduit's length that all
//  send their captured flow to the same sewer node. This function
//  finds the total captured flow as each individual inlet is analyzed
//  sequentially, where its approach flow has been reduced by the
//  amount of flow captured by prior inlets.
{
    let mut i: i32;
    let linkIndex: i32;      // index of link containing inlets
    let mut qApproach: f64;  // single inlet's approach flow (cfs)
    let mut qc: f64;         // single inlet's captured flow (cfs)
    let mut qCaptured: f64;  // total flow captured by link's inlets (cfs)
    let mut qBypassed: f64;  // total flow bypassed by link's inlets (cfs)
    let mut qMax: f64;       // max. flow that a single inlet can capture (cfs)

    if prj.inlet.FirstInlet[inlet].numInlets == 0 {
        return 0.0;
    }
    linkIndex = prj.inlet.FirstInlet[inlet].linkIndex;

    // --- check that link has flow
    qApproach = q;
    if qApproach < MIN_RUNOFF_FLOW {
        return 0.0;
    }

    // --- store conduit geometry in shared variables
    getConduitGeometry(prj, inlet);

    // --- adjust flow for 2-sided street
    qApproach /= prj.inlet.Nsides as f64;
    qBypassed = qApproach;
    qCaptured = 0.0;

    // --- set limit on max. flow captured per inlet
    qMax = BIG;
    if prj.inlet.FirstInlet[inlet].flowLimit > 0.0 {
        qMax = prj.inlet.FirstInlet[inlet].flowLimit;
    }

    // --- evaluate each inlet
    i = 1;
    while i <= prj.inlet.FirstInlet[inlet].numInlets {
        let designIndex = prj.inlet.FirstInlet[inlet].designIndex;
        qc = getOnGradeInletCapture(prj, designIndex, qBypassed, d) *
            prj.inlet.FirstInlet[inlet].clogFactor;
        qc = MIN(qc, qMax);
        qc = MIN(qc, qBypassed);
        qCaptured += qc;
        qBypassed -= qc;
        if qBypassed < MIN_RUNOFF_FLOW {
            break;
        }
        i += 1;
    }
    qCaptured *= prj.inlet.Nsides as f64;
    qCaptured
}

//=============================================================================

fn getOnGradeInletCapture(prj: &mut Project, i: i32, Q: f64, d: f64) -> f64
//
//  Input:   i = an InletDesigns index
//           Q = flow rate seen by inlet (cfs)
//           d = flow depth seen by inlet (ft)
//  Output:  returns captured flow rate (cfs)
//  Purpose: finds the flow captured by a single on-grade inlet.
//
{
    let mut Q1 = Q;
    let mut Qc = 0.0;
    let mut Lsweep: f64 = 0.0;
    let mut Lcurb: f64 = 0.0;
    let mut Lgrate: f64 = 0.0;

    // --- drop curb inlet (in non-Street conduit) only operates in on sag mode
    if prj.inlet.InletDesigns[i as usize].type_ == DROP_CURB_INLET {
        Qc = getOnSagInletCapture(prj, i, d);
        return MIN(Qc, Q);
    }

    // --- drop grate inlet (in non-Street conduit)
    if prj.inlet.InletDesigns[i as usize].type_ == DROP_GRATE_INLET {
        Qc = getGrateInletCapture(prj, i, Q);
        return MIN(Qc, Q);
    }

    // --- Remaining inlet types apply to Street conduits

    // --- find flow spread
    prj.inlet.T = getFlowSpread(prj, Q);

    // --- slotted inlet (behaves as a curb opening inlet per HEC-22)
    if prj.inlet.InletDesigns[i as usize].type_ == SLOTTED_INLET {
        let L = prj.inlet.InletDesigns[i as usize].slottedInlet.length;
        Qc = getCurbInletCapture(prj, Q, L);
        return MIN(Qc, Q);
    }

    Lcurb = prj.inlet.InletDesigns[i as usize].curbInlet.length;
    Lgrate = prj.inlet.InletDesigns[i as usize].grateInlet.length;

    // --- curb opening inlet
    if Lcurb > 0.0 {
        Lsweep = Lcurb - Lgrate;
        if Lsweep > 0.0 {
            Qc = getCurbInletCapture(prj, Q1, Lsweep);
            Q1 -= Qc;
        }
    }

    // --- grate inlet
    if Lgrate > 0.0 && Q1 > 0.0 {
        if Q1 != Q {
            prj.inlet.T = getFlowSpread(prj, Q1);
        }
        Qc += getGrateInletCapture(prj, i, Q1);
    }
    Qc
}

//=============================================================================

fn getGrateInletCapture(prj: &mut Project, i: i32, Q: f64) -> f64
//
//  Input:   i = inlet type index
//           Q = flow rate seen by inlet (cfs)
//  Output:  returns captured flow rate (cfs)
//  Purpose: finds the flow captured by an on-grade grate inlet.
//
{
    let grateType: i32;
    let Lg: f64;         // grate length (ft)
    let mut Wg: f64;     // grate width (ft)
    let A: f64;          // total cross section flow area (ft2)
    let Y: f64;          // flow depth (ft)
    let mut Eo: f64;     // ratio of gutter to total flow
    let V: f64;          // flow velocity (ft/s)
    let Vo: f64;         // splash-over velocity (ft/s)
    let mut Qo = Q;      // flow over street area (cfs)
    let mut Rf = 1.0;    // ratio of intercepted to total frontal flow
    let mut Rs = 0.0;    // ratio of intercepted to total side flow

// xsect, a, W, & Sx were from getConduitGeometry(). T was from getFlowSpread().

    Lg = prj.inlet.InletDesigns[i as usize].grateInlet.length;
    Wg = prj.inlet.InletDesigns[i as usize].grateInlet.width;
    let xsect = prj.inlet.xsect;

    // --- flow ratio for drop inlet
    if xsect.type_ == TRAPEZOIDAL || xsect.type_ == RECT_OPEN {
        A = xsect_getAofS(prj, &xsect, Q / prj.inlet.Beta);
        Y = xsect_getYofA(prj, &xsect, A);
        prj.inlet.T = xsect_getWofY(prj, &xsect, Y);
        Eo = prj.inlet.Beta * (Y * Wg).powf(1.67) / (Wg + 2.0 * Y).powf(0.67) / Q;
        if Wg > 0.99 * xsect.yBot && xsect.type_ == TRAPEZOIDAL && xsect.sBot > 0.0 {
            Wg = xsect.yBot;
            prj.inlet.Sx = 1.0 / xsect.sBot;
        }
    }
    // --- flow ratio & area for conventional street gutter
    else if prj.inlet.a == 0.0 {
        let s = &prj.inlet;
        A = s.T * s.T * s.Sx / 2.0;
        Eo = getGutterFlowRatio(prj, Wg);   // flow ratio based on grate width
        let s = &prj.inlet;
        if s.T >= s.Tcrown {
            Qo = s.Qfactor * s.Tcrown.powf(2.67);
        }
    }
    // --- flow ratio & area for composite street gutter
    else {
        let s = &prj.inlet;
        // --- spread confined to gutter
        if s.T <= s.W {
            A = s.T * s.T * s.Sw / 2.0;
        }
        // --- spread beyond gutter width
        else {
            A = (s.T * s.T * s.Sx + s.a * s.W) / 2.0;
        }

        // flow ratio based on gutter width corrected for grate width
        Eo = getGutterFlowRatio(prj, s.W);
        if Eo < 1.0 {
            if s.T >= s.Tcrown {
                Qo = s.Qfactor * s.Tcrown.powf(2.67) / (1.0 - Eo);
            }
            Eo = Eo * getGutterAreaRatio(prj, Wg, A);         //HEC-22 Eq(4-20a)
        }
    }

    // --- flow and splash-over velocities
    V = Qo / A;
    grateType = prj.inlet.InletDesigns[i as usize].grateInlet.type_;
    if grateType < 0 || grateType == GENERIC {
        Vo = prj.inlet.InletDesigns[i as usize].grateInlet.splashVeloc;
    } else {
        Vo = getSplashOverVelocity(grateType, Lg);
    }

    // --- frontal flow capture efficiency
    if V > Vo {
        Rf = 1.0 - 0.09 * (V - Vo);                       //HEC-22 Eq(4-18)
    }

    // --- side flow capture efficiency
    if Eo < 1.0 {
        Rs = 1.0 / (1.0 + (0.15 * V.powf(1.8) /
            prj.inlet.Sx / Lg.powf(2.3)));                 //HEC-22 Eq(4-19)
    }

    // --- return total flow captured
    Q * (Rf * Eo + Rs * (1.0 - Eo))                        //HEC-22 Eq(4-21)
}

//=============================================================================

fn getCurbInletCapture(prj: &Project, Q: f64, L: f64) -> f64
//
//  Input:   Q = flow rate seen by inlet (cfs)
//           L = length of inlet opening (ft)
//  Output:  returns captured flow rate (cfs)
//  Purpose: finds the flow captured by an on-grade curb opening inlet.
//
{
    let s = &prj.inlet;
    let mut Se = s.Sx;   // equivalent gutter slope
    let Lt: f64;         // length for full capture
    let Sr: f64;         // ratio of gutter slope to cross slope
    let mut Eo = 0.0;    // ratio of gutter to total flow
    let mut E = 1.0;     // capture efficiency

// a, W, Sx, Sw, SL, & n were from getConduitGeometry(). T was from getFlowSpread().

    // --- for depressed gutter section
    if s.a > 0.0 {
        Sr = s.Sw / s.Sx;
        Eo = getEo(Sr, s.T - s.W, s.W);
        Se = s.Sx + (s.a / s.W) * Eo;                      //HEC-22 Eq(4-24)
    }

    // --- opening length for full capture
    Lt = 0.6 * Q.powf(0.42) * s.SL.powf(0.3) *
         (1.0 / (s.n * Se)).powf(0.6);                    //HEC-22 Eq(4-22a)

    // --- capture efficiency for actual opening length
    if L < Lt {
        E = 1.0 - (L / Lt);
        E = 1.0 - E.powf(1.8);                             //HEC-22 Eq(4-23)
    }
    E = MIN(E, 1.0);
    E = MAX(E, 0.0);
    E * Q
}

//=============================================================================

fn getGutterFlowRatio(prj: &Project, w: f64) -> f64
//
//  Input:   w = gutter width (ft)
//  Output:  returns a flow ratio
//  Purpose: computes the ratio of flow over a width of gutter to the total
//           flow in a street cross section.
//
{
    let s = &prj.inlet;
    if s.T <= w {
        1.0
    } else if s.a > 0.0 {
        getEo(s.Sw / s.Sx, s.T - w, w)
    } else {
        1.0 - (1.0 - w / s.T).powf(2.67)                   //HEC-22 Eq(4-16)
    }
}

//=============================================================================

fn getGutterAreaRatio(prj: &Project, Wg: f64, A: f64) -> f64
//
//  Input:   Wg = width of grate inlet (ft)
//           A = total flow area (ft2)
//  Output:  returns an area ratio
//  Purpose: computes the ratio of the flow area above a grate to the flow
//           area above depressed gutter in a street cross section.
//
{
    let s = &prj.inlet;
    let As: f64;     // flow area beyond gutter width (ft2)
    let Ag: f64;     // flow area over grate width (ft2)

    if Wg >= s.W {
        return 1.0;
    }
    if s.T <= Wg {
        return 1.0;
    }
    if s.T <= s.W {
        return Wg / s.T;
    }
    As = 0.5 * SQR(s.T - s.W) * s.Sx;
    Ag = Wg * ((s.T * s.Sx) + s.a - (Wg * s.Sw / 2.));
    Ag / (A - As)
}

//=============================================================================

fn getSplashOverVelocity(grateType: i32, L: f64) -> f64
//
//  Input:   grateType = grate inlet type code
//           L = length of grate inlet (ft)
//  Output:  returns a splash over velocity
//  Purpose: computes the splash over velocity for a standard type of grate
//           inlet as a function of its length.
//
{
    let c = &SplashCoeffs[grateType as usize];
    c[0] +
    c[1] * L -
    c[2] * L * L +
    c[3] * L * L * L
}

//=============================================================================

fn getOnSagCapturedFlow(prj: &mut Project, inlet: usize, q: f64, d: f64) -> f64
//
//  Input:   inlet = an inlet object placed in a conduit link
//           q = flow in link prior to any inlet capture (cfs)
//           d = flow depth seen by inlet (ft)
//  Output:  returns flow captured by the inlet (cfs)
//  Purpose: computes flow captured by an inlet placed on-sag.
//
{
    let linkIndex: i32;
    let designIndex: i32;
    let totalInlets: i32;
    let mut qCaptured: f64 = 0.0;
    let mut qMax: f64 = BIG;

    if prj.inlet.FirstInlet[inlet].numInlets == 0 {
        return 0.0;
    }
    // Nsides here is still the value left by the previous inlet's geometry
    // (getConduitGeometry is called below), as in C.
    totalInlets = prj.inlet.Nsides * prj.inlet.FirstInlet[inlet].numInlets;
    linkIndex = prj.inlet.FirstInlet[inlet].linkIndex;
    designIndex = prj.inlet.FirstInlet[inlet].designIndex;

    // --- store conduit geometry in shared variables
    getConduitGeometry(prj, inlet);

    // --- set flow limit per inlet
    if prj.inlet.FirstInlet[inlet].flowLimit > 0.0 {
        qMax = prj.inlet.FirstInlet[inlet].flowLimit;
    }

    // --- find nominal flow captured by inlet
    qCaptured = getOnSagInletCapture(prj, designIndex, d.abs());

    // --- find actual flow captured by the inlet
    qCaptured *= prj.inlet.FirstInlet[inlet].clogFactor;
    qCaptured = MIN(qCaptured, qMax);
    qCaptured *= totalInlets as f64;
    qCaptured
}

//=============================================================================

fn getOnSagInletCapture(prj: &Project, i: i32, d: f64) -> f64
//
//  Input:   i = inlet type index
//           d = water level seen by inlet (ft)
//  Output:  returns captured flow rate (cfs)
//  Purpose: finds the flow captured by an on-sag inlet.
//
{
    let mut Lsweep: f64 = 0.0;
    let mut Lcurb: f64 = 0.0;
    let mut Lgrate: f64 = 0.0;
    let mut Qsw: f64 = 0.0;   //Sweeper curb opening weir flow
    let mut Qso: f64 = 0.0;   //Sweeper curb opening orifice flow
    let mut Qgw: f64 = 0.0;   //Grate weir flow
    let mut Qgo: f64 = 0.0;   //Grate orifice flow
    let mut Qcw: f64 = 0.0;   //Curb opening weir flow
    let mut Qco: f64 = 0.0;   //Curb opening orifice flow

    if prj.inlet.InletDesigns[i as usize].slottedInlet.length > 0.0 {
        return getOnSagSlottedFlow(prj, i, d);
    }

    Lgrate = prj.inlet.InletDesigns[i as usize].grateInlet.length;
    if Lgrate > 0.0 {
        findOnSagGrateFlows(prj, i, d, &mut Qgw, &mut Qgo);
    }

    Lcurb = prj.inlet.InletDesigns[i as usize].curbInlet.length;
    if Lcurb > 0.0 {
        Lsweep = Lcurb - Lgrate;
        if Lsweep > 0.0 {
            findOnSagCurbFlows(prj, i, d, Lsweep, &mut Qsw, &mut Qso);
        }
        if Qgo > 0.0 {
            findOnSagCurbFlows(prj, i, d, Lgrate, &mut Qcw, &mut Qco);
        }
    }
    Qgw + Qgo + Qsw + Qso + Qco
}

//=============================================================================

fn findOnSagGrateFlows(prj: &Project, i: i32, d: f64, Qw: &mut f64, Qo: &mut f64)
//
//  Input:   i = inlet type index
//           d = water level seen by inlet (ft)
//  Output:  Qw = flow captured in weir mode (cfs)
//           Qo = flow captured in orifice mode (cfs)
//  Purpose: finds the flow captured by an on-sag grate inlet.
//
{
    let s = &prj.inlet;
    let grateType = s.InletDesigns[i as usize].grateInlet.type_;
    let Lg = s.InletDesigns[i as usize].grateInlet.length;
    let mut Wg = s.InletDesigns[i as usize].grateInlet.width;
    let P: f64;      // grate perimeter (ft)
    let Ao: f64;     // grate opening area (ft2)
    let di: f64;     // average flow depth across grate (ft)

    // --- for drop grate inlets
    if s.InletDesigns[i as usize].type_ == DROP_GRATE_INLET {
        di = d;
        P = 2.0 * (Lg + Wg);
    }
    // --- for gutter grate inlets:
    else {
        // --- check for spread within grate width
        if d <= Wg * s.Sw {
            Wg = d / s.Sw;
        }

        // --- avergage depth over grate
        di = d - (Wg / 2.0) * s.Sw;

        // --- effective grate perimeter
        P = Lg + 2.0 * Wg;
    }

    if grateType == GENERIC {
        Ao = Lg * Wg * s.InletDesigns[i as usize].grateInlet.fracOpenArea;
    } else {
        Ao = Lg * Wg * GrateOpeningRatios[grateType as usize];
    }

    // --- weir flow applies (based on  depth where result of
    //     weir eqn. equals result of orifice eqn.)

    if d <= 1.79 * Ao / P {
        *Qw = 3.0 * P * di.powf(1.5);                      //HEC-22 Eq(4-26)
    }
    // --- orifice flow applies
    else {
        *Qo = 0.67 * Ao * (2.0 * 32.16 * di).sqrt();       //HEC-22 Eq(4-27)
    }
}

//=============================================================================

fn findOnSagCurbFlows(prj: &Project, i: i32, d: f64, mut L: f64, Qw: &mut f64, Qo: &mut f64)
//
//  Input:   i = inlet type index
//           d = water level seen by inlet (ft)
//           L = length of curb opening (ft)
//  Output:  Qw = flow captured in weir mode (cfs)
//           Qo = flow captured in orifice mode (cfs)
//  Purpose: finds the flow captured by an on-sag curb opening inlet.
//
{
    let s = &prj.inlet;
    let throatAngle = s.InletDesigns[i as usize].curbInlet.throatAngle;
    let h = s.InletDesigns[i as usize].curbInlet.height;
    let Qweir: f64;
    let Qorif: f64;
    let P: f64;
    let dweir: f64;
    let dorif: f64;
    let r: f64;

    // --- check for orifice flow
    if L <= 0.0 {
        return;
    }
    if s.InletDesigns[i as usize].type_ == DROP_CURB_INLET {
        L = L * 4.0;
    }
    dorif = 1.4 * h;
    if d > dorif {
        *Qo = getCurbOrificeFlow(d, h, L, throatAngle);
        return;
    }

    // --- for uniform cross slope or very long opening
    if s.a == 0.0 || L > 12.0 {
        // --- check for weir flow
        dweir = h;
        if d < dweir {
            *Qw = 3.0 * L * d.powf(1.5);                   //HEC-22 Eq(4-30)
            return;
        } else {
            Qweir = 3.0 * L * dweir.powf(1.5);
        }
    }
    // --- for depressed gutter
    else {
        // --- check for weir flow
        P = L + 1.8 * s.W;
        dweir = h + s.a;
        if d < dweir {
            *Qw = 2.3 * P * d.powf(1.5);                   //HEC-22 Eq(4-28)
            return;
        } else {
            Qweir = 2.3 * P * dweir.powf(1.5);
        }
    }

    // --- interpolate between Qweir at depth dweir and Qorif at depth dorif
    Qorif = getCurbOrificeFlow(dorif, h, L, throatAngle);
    r = (d - dweir) / (dorif - dweir);
    *Qw = (1.0 - r) * Qweir;
    *Qo = r * Qorif;
}

//=============================================================================

fn getCurbOrificeFlow(di: f64, h: f64, L: f64, throatAngle: i32) -> f64
//
//  Input:   di = water level at lip of inlet opening (ft)
//           h = height of curb opening (ft)
//           L = length of curb opening (ft)
//           throatAngle = type of throat angle in curb opening
//  Output:  return flow captured by inlet (cfs)
//  Purpose: finds the flow captured by an on-sag curb opening inlet under
//           orifice flow conditions.
//
{
    let mut d = di;
    if throatAngle == HORIZONTAL_THROAT {
        d = di - h / 2.0;
    } else if throatAngle == INCLINED_THROAT {
        d = di - (h / 2.0) * 0.7071;
    }
    0.67 * h * L * (2.0 * 32.16 * d).sqrt()                 //HEC-22 Eq(4-31a)
}

//=============================================================================

fn getOnSagSlottedFlow(prj: &Project, i: i32, d: f64) -> f64
//
//  Input:   i = inlet type index
//           d = water level seen by inlet (ft)
//  Output:  returns captured flow rate (cfs)
//  Purpose: finds the flow captured by an on-sag slotted inlet.
//
//  Note: weir flow = orifice flow at d = 2.587 * inlet width
{
    let L = prj.inlet.InletDesigns[i as usize].slottedInlet.length;
    let w = prj.inlet.InletDesigns[i as usize].slottedInlet.width;

    if d <= 2.587 * w {
        2.48 * L * d.powf(1.5)                  //HEC-22 Eq(4-32)
    } else {
        0.8 * L * w * (64.32 * d).sqrt()        //HEC-22 Eq(4-33)
    }
}

//=============================================================================

fn getBackflowRatios(prj: &mut Project)
//
//  Input:   none
//  Output:  overflow ratio for each inlet
//  Purpose: finds the fraction of the overflow produced by an inlet's capture
//           node that becomes backflow into the inlet.
//
//  Note: when a capture node receives flow from two or more inlets
//        its backflow is divided among the inlets based on:
//        i)  the fraction of total open area for standard inlets
//        ii) the fraction of total number of inlets for custom inlets
{
    let mut area: f64;
    let mut f: f64;
    let mut n: i32;

    // --- info for each node receiving flow from an inlet
    #[derive(Clone, Copy, Default)]
    struct TInletNode {
        numInletLinks: i32,          // total # inlet links
        numStdInletLinks: i32,       // total # standard inlet links
        numCustomInlets: i32,        // # custom inlets
        totalInletArea: f64,         // open area of standard inlets
    }
    let mut inletNodes: Vec<TInletNode> =
        vec![TInletNode::default(); prj.Nobjects[NODE as usize] as usize];

    // --- Finds each inlet's contribution to its capture node
    for inlet in 0..prj.inlet.FirstInlet.len() {
        n = prj.inlet.FirstInlet[inlet].nodeIndex;
        inletNodes[n as usize].numInletLinks += 1;
        area = getInletArea(prj, inlet);
        if area > 0.0 {
            inletNodes[n as usize].numStdInletLinks += 1;
            inletNodes[n as usize].totalInletArea += area;
        } else {
            inletNodes[n as usize].numCustomInlets += prj.inlet.FirstInlet[inlet].numInlets;
        }
    }

    // --- find fraction of capture node's overflow that becomes inlet backflow
    for inlet in 0..prj.inlet.FirstInlet.len() {
        // --- f is ratio of links with standard inlets to all inlet links
        //     connected to receptor node n
        n = prj.inlet.FirstInlet[inlet].nodeIndex;
        let nd = inletNodes[n as usize];
        f = nd.numStdInletLinks as f64 / nd.numInletLinks as f64;

        // --- backflow ratio depends if inlet is standard or custom (area = 0)
        area = getInletArea(prj, inlet);
        let it = &mut prj.inlet.FirstInlet[inlet];
        if area == 0.0 {
            it.backflowRatio = it.numInlets as f64 /
                               nd.numCustomInlets as f64 * (1. - f);
        } else {
            it.backflowRatio = area / nd.totalInletArea * f;
        }
    }
}

//=============================================================================

fn getInletArea(prj: &Project, inlet: usize) -> f64
//
//  Input:   inlet = an inlet object placed in a conduit link
//  Output:  returns the unclogged open area of the inlet (ft2)
//  Purpose: finds the total open flow area inlets placed in a conduit.
//
{
    let mut area = 0.0;
    let curbLength: f64;
    let it = &prj.inlet.FirstInlet[inlet];
    let i = it.designIndex;
    let d = &prj.inlet.InletDesigns[i as usize];
    let grateType = d.grateInlet.type_;

    if d.grateInlet.length > 0.0 {
        area = d.grateInlet.length * d.grateInlet.width;
        if grateType == GENERIC {
            area *= d.grateInlet.fracOpenArea;
        } else {
            area *= GrateOpeningRatios[grateType as usize];
        }
    }

    curbLength = d.curbInlet.length - d.grateInlet.length;
    if curbLength > 0.0 {
        area += curbLength * d.curbInlet.height;
    }

    if d.slottedInlet.length > 0.0 {
        area = d.slottedInlet.length * d.slottedInlet.width;
    }
    area * it.numInlets as f64 * it.clogFactor
}

//=============================================================================

fn getCustomCapturedFlow(prj: &mut Project, inlet: usize, q: f64, d: f64) -> f64
{
    let i = prj.inlet.FirstInlet[inlet].designIndex;  // inlet's position in InletDesigns array
    let mut j: i32;                    // counter for replicate inlets
    let mut sides: i32 = 1;            // number of sides for inlet's street (1 or 2)
    let c: i32;                        // an index into the Curve array
    let qApproach: f64;                // inlet's approach flow (cfs)
    let mut qBypassed: f64;            // inlet's bypassed flow (cfs)
    let mut qCaptured: f64;            // inlet's captured flow (cfs)
    let mut qIncrement: f64;           // increment to captured flow (cfs)
    let mut qMax: f64 = BIG;           // user-supplied flow capture limit (cfs)

    if prj.inlet.FirstInlet[inlet].numInlets == 0 {
        return 0.0;
    }

    // --- set limit on max. flow captured per inlet
    qMax = BIG;
    if prj.inlet.FirstInlet[inlet].flowLimit > 0.0 {
        qMax = prj.inlet.FirstInlet[inlet].flowLimit;
    }

    // --- get number of sides to a street xsection
    let li = prj.inlet.FirstInlet[inlet].linkIndex;
    prj.inlet.xsect = prj.Link[li as usize].xsect;
    if prj.inlet.xsect.type_ == STREET_XSECT {
        sides = prj.Street[prj.inlet.xsect.transect as usize].sides;
    }

    // --- adjust flow for 2-sided street
    qApproach = q / sides as f64;
    qBypassed = qApproach;
    qCaptured = 0.0;

    // --- get index of inlet's capture curve
    c = prj.inlet.InletDesigns[i as usize].customCurve;
    if c >= 0 {
        // --- curve is captured flow v. approach flow
        if prj.Curve[c as usize].curveType == DIVERSION_CURVE {
            // --- add up incrmental capture of each replicate inlet
            j = 1;
            while j <= prj.inlet.FirstInlet[inlet].numInlets {
                let ucfFlow = UCF(prj, FLOW);
                let clogFactor = prj.inlet.FirstInlet[inlet].clogFactor;
                qIncrement = clogFactor *
                    table_lookupEx(&mut prj.Curve[c as usize], qBypassed * ucfFlow) / ucfFlow;
                qIncrement = MIN(qIncrement, qMax);
                qIncrement = MIN(qIncrement, qBypassed);
                qCaptured += qIncrement;
                qBypassed -= qIncrement;
                if qBypassed < MIN_RUNOFF_FLOW {
                    break;
                }
                j += 1;
            }
        }
        // --- curve is captured flow v. downstream node depth
        else if prj.Curve[c as usize].curveType == RATING_CURVE {
            let ucfLength = UCF(prj, LENGTH);
            let ucfFlow = UCF(prj, FLOW);
            let numInlets = prj.inlet.FirstInlet[inlet].numInlets;
            let clogFactor = prj.inlet.FirstInlet[inlet].clogFactor;
            qCaptured = numInlets as f64 * clogFactor *
                table_lookupEx(&mut prj.Curve[c as usize], d * ucfLength) / ucfFlow;
        }
        qCaptured *= sides as f64;
    }
    qCaptured
}
