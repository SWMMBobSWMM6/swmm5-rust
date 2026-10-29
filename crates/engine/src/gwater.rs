//! gwater.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   gwater.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21   (Build 5.2.0)
//   Author:   L. Rossman
//
//   Groundwater functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - User-supplied function for deep GW seepage flow added.
//   - New variable names for use in user-supplied GW flow equations added.
//   Build 5.1.008:
//   - More variable names for user-supplied GW flow equations added.
//   - Subcatchment area made into a shared variable.
//   - Evaporation loss initialized to 0.
//   - Support for collecting GW statistics added.
//   Build 5.1.010:
//   - Unsaturated hydraulic conductivity added to GW flow equation variables.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const GWTOL: f64 = 0.0001; // ODE solver tolerance
const XTOL: f64 = 0.001; // tolerance for moisture & depth

// enum GWstates
const THETA: usize = 0; // moisture content of upper GW zone
const LOWERDEPTH: usize = 1; // depth of lower saturated GW zone

// enum GWvariables
const gwvHGW: i32 = 0; // water table height (ft)
const gwvHSW: i32 = 1; // surface water height (ft)
const gwvHCB: i32 = 2; // channel bottom height (ft)
const gwvHGS: i32 = 3; // ground surface height (ft)
const gwvKS: i32 = 4; // sat. hyd. condutivity (ft/s)
const gwvK: i32 = 5; // unsat. hyd. conductivity (ft/s)
const gwvTHETA: i32 = 6; // upper zone moisture content
const gwvPHI: i32 = 7; // soil porosity
const gwvFI: i32 = 8; // surface infiltration (ft/s)
const gwvFU: i32 = 9; // uper zone percolation rate (ft/s)
const gwvA: i32 = 10; // subcatchment area (ft2)
const gwvMAX: i32 = 11;

// Names of GW variables that can be used in GW outflow expression
const GWVarWords: &[&[u8]] = &[b"HGW", b"HSW", b"HCB", b"HGS", b"KS", b"K", b"THETA", b"PHI", b"FI", b"FU", b"A"];

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of gwater.c.
///  NOTE: all flux rates are in ft/sec, all depths are in ft.
#[derive(Default)]
pub struct GwaterState {
    Area: f64,         // subcatchment area (ft2)
    Infil: f64,        // infiltration rate from surface
    MaxEvap: f64,      // max. evaporation rate
    AvailEvap: f64,    // available evaporation rate
    UpperEvap: f64,    // evaporation rate from upper GW zone
    LowerEvap: f64,    // evaporation rate from lower GW zone
    UpperPerc: f64,    // percolation rate from upper to lower zone
    LowerLoss: f64,    // loss rate from lower GW zone
    GWFlow: f64,       // flow rate from lower zone to conveyance node
    MaxUpperPerc: f64, // upper limit on UpperPerc
    MaxGWFlowPos: f64, // upper limit on GWFlow when its positve
    MaxGWFlowNeg: f64, // upper limit on GWFlow when its negative
    FracPerv: f64,     // fraction of surface that is pervious
    TotalDepth: f64,   // total depth of GW aquifer
    Theta: f64,        // moisture content of upper zone
    HydCon: f64,       // unsaturated hydraulic conductivity (ft/s)
    Hgw: f64,          // ht. of saturated zone
    Hstar: f64,        // ht. from aquifer bottom to node invert
    Hsw: f64,          // ht. from aquifer bottom to water surface
    Tstep: f64,        // current time step (sec)
    /// aquifer being analyzed (C: `static TAquifer A;`, a copy of
    /// Aquifer[k]; here the index k, since Aquifer does not change while
    /// the copy is in use)
    A: i32,
    /// groundwater object being analyzed (C: `TGroundwater* GW` pointing at
    /// Subcatch[j].groundwater; here j, or -1 for NULL)
    GW: i32,
    /// user-supplied lateral GW flow expression (C: pointer to
    /// Subcatch[j].gwLatFlowExpr; here j, or -1 for NULL)
    LatFlowExpr: i32,
    /// user-supplied deep GW flow expression (C: pointer to
    /// Subcatch[j].gwDeepFlowExpr; here j, or -1 for NULL)
    DeepFlowExpr: i32,
}

//-----------------------------------------------------------------------------
//  External Functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  gwater_readAquiferParams     (called by input_readLine)
//  gwater_readGroundwaterParams (called by input_readLine)
//  gwater_readFlowExpression    (called by input_readLine)
//  gwater_deleteFlowExpression  (called by deleteObjects in project.c)
//  gwater_validateAquifer       (called by swmm_open)
//  gwater_validate              (called by subcatch_validate)
//  gwater_initState             (called by subcatch_initState)
//  gwater_getVolume             (called by massbal_open & massbal_getGwaterError)
//  gwater_getGroundwater        (called by getSubareaRunoff in subcatch.c)
//  gwater_getState              (called by saveRunoff in hotstart.c)
//  gwater_setState              (called by readRunoff in hotstart.c)

/// The aquifer being analyzed (C: `A`).
#[inline]
fn A(prj: &Project) -> &TAquifer {
    &prj.Aquifer[prj.gwater.A as usize]
}

/// The groundwater object being analyzed (C: `GW`).
#[inline]
fn GW(prj: &Project) -> &TGroundwater {
    prj.Subcatch[prj.gwater.GW as usize].groundwater.as_deref().unwrap()
}

/// The groundwater object being analyzed (C: `GW`), for writing.
#[inline]
fn GW_mut(prj: &mut Project) -> &mut TGroundwater {
    let j = prj.gwater.GW as usize;
    prj.Subcatch[j].groundwater.as_deref_mut().unwrap()
}

//=============================================================================

/// Called by: input.
pub fn gwater_readAquiferParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   j = aquifer index
    //           tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error message
    //  Purpose: reads aquifer parameter values from line of input data
    //
    //  Data line contains following parameters:
    //    ID, porosity, wiltingPoint, fieldCapacity,     conductivity,
    //    conductSlope, tensionSlope, upperEvapFraction, lowerEvapDepth,
    //    gwRecession,  bottomElev,   waterTableElev,    upperMoisture
    //    (evapPattern)
    //
    let mut i: i32;
    let mut p: i32;
    let mut x = [0.0f64; 12];
    let id: Option<Vec<u8>>;

    // --- check that aquifer exists
    if ntoks < 13 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, AQUIFER, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- read remaining tokens as numbers
    i = 0;
    while i < 11 {
        x[i as usize] = 0.0;
        i += 1;
    }
    i = 1;
    while i < 13 {
        if getDouble(&tok[i as usize], &mut x[(i - 1) as usize]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
        }
        i += 1;
    }

    // --- read upper evap pattern if present
    p = -1;
    if ntoks > 13 {
        p = project_findObject(prj, TIMEPATTERN, &tok[13]);
        if p < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[13]);
        }
    }

    // --- assign parameters to aquifer object
    let ucfRainfall = UCF(prj, RAINFALL);
    let ucfLength = UCF(prj, LENGTH);
    let a = &mut prj.Aquifer[j as usize];
    a.ID = id.unwrap();
    a.porosity = x[0];
    a.wiltingPoint = x[1];
    a.fieldCapacity = x[2];
    a.conductivity = x[3] / ucfRainfall;
    a.conductSlope = x[4];
    a.tensionSlope = x[5] / ucfLength;
    a.upperEvapFrac = x[6];
    a.lowerEvapDepth = x[7] / ucfLength;
    a.lowerLossCoeff = x[8] / ucfRainfall;
    a.bottomElev = x[9] / ucfLength;
    a.waterTableElev = x[10] / ucfLength;
    a.upperMoisture = x[11];
    a.upperEvapPat = p;
    return 0;
}

//=============================================================================

/// Called by: input.
pub fn gwater_readGroundwaterParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //  Purpose: reads groundwater inflow parameters for a subcatchment from
    //           a line of input data.
    //
    //  Data format is:
    //  subcatch  aquifer  node  surfElev  a1  b1  a2  b2  a3  fixedDepth +
    //            (nodeElev  bottomElev  waterTableElev  upperMoisture )
    //
    let mut i: i32;
    let j: i32;
    let k: i32;
    let mut m: i32;
    let n: i32;
    let mut x = [0.0f64; 11];

    // --- check that specified subcatchment, aquifer & node exist
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    j = project_findObject(prj, SUBCATCH, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- check for enough tokens
    if ntoks < 11 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that specified aquifer and node exists
    k = project_findObject(prj, AQUIFER, &tok[1]);
    if k < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    n = project_findObject(prj, NODE, &tok[2]);
    if n < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }

    // -- read in the flow parameters
    i = 0;
    while i < 7 {
        if getDouble(&tok[(i + 3) as usize], &mut x[i as usize]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[(i + 3) as usize]);
        }
        i += 1;
    }

    // --- read in optional depth parameters
    i = 7;
    while i < 11 {
        x[i as usize] = MISSING;
        m = i + 3;
        if ntoks > m && tok[m as usize].first().copied().unwrap_or(0) != b'*' {
            if getDouble(&tok[m as usize], &mut x[i as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[m as usize]);
            }
            if i < 10 {
                x[i as usize] /= UCF(prj, LENGTH);
            }
        }
        i += 1;
    }

    // --- create a groundwater flow object
    //     (C mallocs it uninitialized; every field is written below, in
    //      gwater_initState or in stats_open before it is read)
    let ju = j as usize;
    if prj.Subcatch[ju].groundwater.is_none() {
        prj.Subcatch[ju].groundwater = Some(Box::new(TGroundwater::default()));
    }

    // --- populate the groundwater flow object with its parameters
    let ucfLength = UCF(prj, LENGTH);
    let gw = prj.Subcatch[ju].groundwater.as_deref_mut().unwrap();
    gw.aquifer = k;
    gw.node = n;
    gw.surfElev = x[0] / ucfLength;
    gw.a1 = x[1];
    gw.b1 = x[2];
    gw.a2 = x[3];
    gw.b2 = x[4];
    gw.a3 = x[5];
    gw.fixedDepth = x[6] / ucfLength;
    gw.nodeElev = x[7]; //already converted to ft.
    gw.bottomElev = x[8];
    gw.waterTableElev = x[9];
    gw.upperMoisture = x[10];
    return 0;
}

//=============================================================================

/// Called by: input.
#[allow(unused_assignments)] // C's `k = 1;` before the keyword test
pub fn gwater_readFlowExpression(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //  Purpose: reads mathematical expression for lateral or deep groundwater
    //           flow for a subcatchment from a line of input data.
    //
    //  Format is: subcatch LATERAL/DEEP <expr>
    //     where subcatch is the ID of the subcatchment, LATERAL is for lateral
    //     GW flow, DEEP is for deep GW flow and <expr> is any well-formed math
    //     expression.
    //
    let mut i: i32;
    let j: i32;
    let mut k: i32;
    let mut exprStr = [0u8; (MAXLINE + 1) as usize];
    let expr: Option<MathExpr>;

    // --- return if too few tokens
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that subcatchment exists
    j = project_findObject(prj, SUBCATCH, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- check if expression is for lateral or deep GW flow
    k = 1;
    if r#match(&tok[1], b"LAT") != 0 {
        k = 1;
    } else if r#match(&tok[1], b"DEEP") != 0 {
        k = 2;
    } else {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }

    // --- concatenate remaining tokens into a single string
    sstrncpy(&mut exprStr, &tok[2], MAXLINE as usize);
    i = 3;
    while i < ntoks {
        sstrcat(&mut exprStr, b" ", (MAXLINE + 1) as usize);
        sstrcat(&mut exprStr, &tok[i as usize], (MAXLINE + 1) as usize);
        i += 1;
    }

    // --- delete any previous flow eqn.
    //     (C frees the expression but leaves the stale pointer in place
    //      until it is overwritten below; here it is cleared at once)
    let ju = j as usize;
    if k == 1 {
        mathexpr_delete(prj.Subcatch[ju].gwLatFlowExpr.take());
    } else {
        mathexpr_delete(prj.Subcatch[ju].gwDeepFlowExpr.take());
    }

    // --- create a parsed expression tree from the string expr
    //     (getVariableIndex is the function that converts a GW
    //      variable's name into an index number)
    expr = mathexpr_create(cio::cstr(&exprStr), &mut |s: &[u8]| getVariableIndex(s));
    if expr.is_none() {
        return error_setInpError(prj, ERR_MATH_EXPR, b"");
    }

    // --- save expression tree with the subcatchment
    if k == 1 {
        prj.Subcatch[ju].gwLatFlowExpr = expr;
    } else {
        prj.Subcatch[ju].gwDeepFlowExpr = expr;
    }
    return 0;
}

//=============================================================================

/// Called by: project.
pub fn gwater_deleteFlowExpression(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: deletes a subcatchment's custom groundwater flow expressions.
    //
    let ju = j as usize;
    mathexpr_delete(prj.Subcatch[ju].gwLatFlowExpr.take());
    mathexpr_delete(prj.Subcatch[ju].gwDeepFlowExpr.take());
}

//=============================================================================

/// Called by: project.
pub fn gwater_validateAquifer(prj: &mut Project, j: i32) {
    //
    //  Input:   j = aquifer index
    //  Output:  none
    //  Purpose: validates groundwater aquifer properties .
    //
    let p: i32;
    let ju = j as usize;

    let a = &prj.Aquifer[ju];
    if a.porosity <= 0.0
        || a.fieldCapacity >= a.porosity
        || a.wiltingPoint >= a.fieldCapacity
        || a.conductivity <= 0.0
        || a.conductSlope < 0.0
        || a.tensionSlope < 0.0
        || a.upperEvapFrac < 0.0
        || a.lowerEvapDepth < 0.0
        || a.waterTableElev < a.bottomElev
        || a.upperMoisture > a.porosity
        || a.upperMoisture < a.wiltingPoint
    {
        let id = prj.Aquifer[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_AQUIFER_PARAMS, &id);
    }

    p = prj.Aquifer[ju].upperEvapPat;
    if p >= 0 && prj.Pattern[p as usize].type_ != MONTHLY_PATTERN {
        let id = prj.Aquifer[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_AQUIFER_PARAMS, &id);
    }
}

//=============================================================================

/// Called by: subcatch.
pub fn gwater_validate(prj: &mut Project, j: i32) {
    let ju = j as usize;
    // (C copies the aquifer into a local TAquifer a; only its values are read)
    if prj.Subcatch[ju].groundwater.is_some() {
        let aq = prj.Subcatch[ju].groundwater.as_deref().unwrap().aquifer as usize;
        let aBottomElev = prj.Aquifer[aq].bottomElev;
        let aWaterTableElev = prj.Aquifer[aq].waterTableElev;
        let aUpperMoisture = prj.Aquifer[aq].upperMoisture;
        let gw = prj.Subcatch[ju].groundwater.as_deref_mut().unwrap();

        // ... use aquifer values for missing groundwater parameters
        if gw.bottomElev == MISSING {
            gw.bottomElev = aBottomElev;
        }
        if gw.waterTableElev == MISSING {
            gw.waterTableElev = aWaterTableElev;
        }
        if gw.upperMoisture == MISSING {
            gw.upperMoisture = aUpperMoisture;
        }

        // ... ground elevation can't be below water table elevation
        if gw.surfElev < gw.waterTableElev {
            let id = prj.Subcatch[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_GROUND_ELEV, &id);
        }
    }
}

//=============================================================================

/// Called by: subcatch.
pub fn gwater_initState(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: initializes state of subcatchment's groundwater.
    //
    let ju = j as usize;
    if prj.Subcatch[ju].groundwater.is_some() {
        let aq = prj.Subcatch[ju].groundwater.as_deref().unwrap().aquifer as usize;
        let aPorosity = prj.Aquifer[aq].porosity;
        // (subcatch_getFracPerv only reads; it is evaluated where C calls it,
        //  after every other field below has been set)
        {
            let gw = prj.Subcatch[ju].groundwater.as_deref_mut().unwrap();

            // ... initial moisture content
            gw.theta = gw.upperMoisture;
            if gw.theta >= aPorosity {
                gw.theta = aPorosity - XTOL;
            }

            // ... initial depth of lower (saturated) zone
            gw.lowerDepth = gw.waterTableElev - gw.bottomElev;
            if gw.lowerDepth >= gw.surfElev - gw.bottomElev {
                gw.lowerDepth = gw.surfElev - gw.bottomElev - XTOL;
            }

            // ... initial lateral groundwater outflow
            gw.oldFlow = 0.0;
            gw.newFlow = 0.0;
            gw.evapLoss = 0.0;
        }

        // ... initial available infiltration volume into upper zone
        let fracPerv = subcatch_getFracPerv(prj, j);
        let gw = prj.Subcatch[ju].groundwater.as_deref_mut().unwrap();
        gw.maxInfilVol = (gw.surfElev - gw.waterTableElev) * (aPorosity - gw.theta) / fracPerv;
    }
}

//=============================================================================

/// Called by: hotstart.
pub fn gwater_getState(prj: &Project, j: i32, x: &mut [f64]) {
    //
    //  Input:   j = subcatchment index
    //  Output:  x[] = array of groundwater state variables
    //  Purpose: retrieves state of subcatchment's groundwater.
    //
    let gw = prj.Subcatch[j as usize].groundwater.as_deref().unwrap();
    x[0] = gw.theta;
    x[1] = gw.bottomElev + gw.lowerDepth;
    x[2] = gw.newFlow;
    x[3] = gw.maxInfilVol;
}

//=============================================================================

/// Called by: hotstart.
pub fn gwater_setState(prj: &mut Project, j: i32, x: &[f64]) {
    //
    //  Input:   j = subcatchment index
    //           x[] = array of groundwater state variables
    //  Purpose: assigns values to a subcatchment's groundwater state.
    //
    let gw = match prj.Subcatch[j as usize].groundwater.as_deref_mut() {
        None => return,
        Some(g) => g,
    };
    gw.theta = x[0];
    gw.lowerDepth = x[1] - gw.bottomElev;
    gw.oldFlow = x[2];
    if x[3] != MISSING {
        gw.maxInfilVol = x[3];
    }
}

//=============================================================================

/// Called by: massbal.
pub fn gwater_getVolume(prj: &Project, j: i32) -> f64 {
    //
    //  Input:   j = subcatchment index
    //  Output:  returns total volume of groundwater in ft/ft2
    //  Purpose: finds volume of groundwater stored in upper & lower zones
    //
    let upperDepth: f64;
    let gw = match prj.Subcatch[j as usize].groundwater.as_deref() {
        None => return 0.0,
        Some(g) => g,
    };
    let a = &prj.Aquifer[gw.aquifer as usize];
    upperDepth = gw.surfElev - gw.bottomElev - gw.lowerDepth;
    return (upperDepth * gw.theta) + (gw.lowerDepth * a.porosity);
}

//=============================================================================

/// Called by: subcatch.
pub fn gwater_getGroundwater(prj: &mut Project, j: i32, evap: f64, infil: f64, tStep: f64) {
    //
    //  Purpose: computes groundwater flow from subcatchment during current time step.
    //  Input:   j     = subcatchment index
    //           evap  = pervious surface evaporation volume consumed (ft3)
    //           infil = surface infiltration volume (ft3)
    //           tStep = time step (sec)
    //  Output:  none
    //
    let mut evap = evap;
    let mut infil = infil;
    let n: i32; // node exchanging groundwater
    let mut x = [0.0f64; 2]; // upper moisture content & lower depth
    let mut vUpper: f64; // upper vol. available for percolation
    let nodeFlow: f64; // max. possible GW flow from node
    let ju = j as usize;

    // --- save subcatchment's groundwater and aquifer objects to
    //     shared variables
    if prj.Subcatch[ju].groundwater.is_none() {
        prj.gwater.GW = -1;
        return;
    }
    prj.gwater.GW = j;
    prj.gwater.LatFlowExpr = if prj.Subcatch[ju].gwLatFlowExpr.is_some() { j } else { -1 };
    prj.gwater.DeepFlowExpr = if prj.Subcatch[ju].gwDeepFlowExpr.is_some() { j } else { -1 };
    prj.gwater.A = GW(prj).aquifer;

    // --- get fraction of total area that is pervious
    prj.gwater.FracPerv = subcatch_getFracPerv(prj, j);
    if prj.gwater.FracPerv <= 0.0 {
        return;
    }
    prj.gwater.Area = prj.Subcatch[ju].area;

    // --- convert infiltration volume (ft3) to equivalent rate
    //     over entire GW (subcatchment) area
    infil = infil / prj.gwater.Area / tStep;
    prj.gwater.Infil = infil;
    prj.gwater.Tstep = tStep;

    // --- convert pervious surface evaporation already exerted (ft3)
    //     to equivalent rate over entire GW (subcatchment) area
    evap = evap / prj.gwater.Area / tStep;

    // --- convert max. surface evap rate (ft/sec) to a rate
    //     that applies to GW evap (GW evap can only occur
    //     through the pervious land surface area)
    prj.gwater.MaxEvap = prj.Evap.rate * prj.gwater.FracPerv;

    // --- available subsurface evaporation is difference between max.
    //     rate and pervious surface evap already exerted
    prj.gwater.AvailEvap = MAX(prj.gwater.MaxEvap - evap, 0.0);

    // --- save total depth & outlet node properties to shared variables
    prj.gwater.TotalDepth = GW(prj).surfElev - GW(prj).bottomElev;
    if prj.gwater.TotalDepth <= 0.0 {
        return;
    }
    n = GW(prj).node;
    let nu = n as usize;

    // --- establish min. water table height above aquifer bottom at which
    //     GW flow can occur (override node's invert if a value was provided
    //     in the GW object)
    if GW(prj).nodeElev != MISSING {
        prj.gwater.Hstar = GW(prj).nodeElev - GW(prj).bottomElev;
    } else {
        prj.gwater.Hstar = prj.Node[nu].invertElev - GW(prj).bottomElev;
    }

    // --- establish surface water height (relative to aquifer bottom)
    //     for drainage system node connected to the GW aquifer
    if GW(prj).fixedDepth > 0.0 {
        prj.gwater.Hsw = GW(prj).fixedDepth + prj.Node[nu].invertElev - GW(prj).bottomElev;
    } else {
        prj.gwater.Hsw = prj.Node[nu].newDepth + prj.Node[nu].invertElev - GW(prj).bottomElev;
    }

    // --- store state variables (upper zone moisture content, lower zone
    //     depth) in work vector x
    x[THETA] = GW(prj).theta;
    x[LOWERDEPTH] = GW(prj).lowerDepth;

    // --- set limit on percolation rate from upper to lower GW zone
    vUpper = (prj.gwater.TotalDepth - x[LOWERDEPTH]) * (x[THETA] - A(prj).fieldCapacity);
    vUpper = MAX(0.0, vUpper);
    prj.gwater.MaxUpperPerc = vUpper / tStep;

    // --- set limit on GW flow out of aquifer based on volume of lower zone
    prj.gwater.MaxGWFlowPos = x[LOWERDEPTH] * A(prj).porosity / tStep;

    // --- set limit on GW flow into aquifer from drainage system node
    //     based on min. of capacity of upper zone and drainage system
    //     inflow to the node
    prj.gwater.MaxGWFlowNeg = (prj.gwater.TotalDepth - x[LOWERDEPTH]) * (A(prj).porosity - x[THETA]) / tStep;
    nodeFlow = (prj.Node[nu].inflow + prj.Node[nu].newVolume / tStep) / prj.gwater.Area;
    prj.gwater.MaxGWFlowNeg = -MIN(prj.gwater.MaxGWFlowNeg, nodeFlow);

    // --- integrate eqns. for d(Theta)/dt and d(LowerDepth)/dt
    //     NOTE: ODE solver must have been initialized previously
    odesolve_integrate(&mut x, 2, 0.0, tStep, GWTOL, tStep, &mut |t: f64, xx: &[f64], dxdt: &mut [f64]| {
        getDxDt(prj, t, xx, dxdt)
    });

    // --- keep state variables within allowable bounds
    x[THETA] = MAX(x[THETA], A(prj).wiltingPoint);
    if x[THETA] >= A(prj).porosity {
        x[THETA] = A(prj).porosity - XTOL;
        x[LOWERDEPTH] = prj.gwater.TotalDepth - XTOL;
    }
    x[LOWERDEPTH] = MAX(x[LOWERDEPTH], 0.0);
    if x[LOWERDEPTH] >= prj.gwater.TotalDepth {
        x[LOWERDEPTH] = prj.gwater.TotalDepth - XTOL;
    }

    // --- save new values of state values
    GW_mut(prj).theta = x[THETA];
    GW_mut(prj).lowerDepth = x[LOWERDEPTH];
    let theta = GW(prj).theta;
    let lowerDepth = GW(prj).lowerDepth;
    getFluxes(prj, theta, lowerDepth);
    let newFlow = GW(prj).newFlow;
    GW_mut(prj).oldFlow = newFlow;
    GW_mut(prj).newFlow = prj.gwater.GWFlow;
    let evapLoss = prj.gwater.UpperEvap + prj.gwater.LowerEvap;
    GW_mut(prj).evapLoss = evapLoss;

    //--- find max. infiltration volume (as depth over
    //    the pervious portion of the subcatchment)
    //    that upper zone can support in next time step
    let maxInfilVol = (prj.gwater.TotalDepth - x[LOWERDEPTH]) * (A(prj).porosity - x[THETA]) / prj.gwater.FracPerv;
    GW_mut(prj).maxInfilVol = maxInfilVol;

    // --- update GW mass balance
    let area = prj.gwater.Area;
    updateMassBal(prj, area, tStep);

    // --- update GW statistics
    let gwEvapLoss = GW(prj).evapLoss;
    let gwFlow = prj.gwater.GWFlow;
    let lowerLoss = prj.gwater.LowerLoss;
    let gwTheta = GW(prj).theta;
    let waterTable = GW(prj).lowerDepth + GW(prj).bottomElev;
    stats_updateGwaterStats(prj, j, infil, gwEvapLoss, gwFlow, lowerLoss, gwTheta, waterTable, tStep);
}

//=============================================================================

fn updateMassBal(prj: &mut Project, area: f64, tStep: f64) {
    //
    //  Input:   area  = subcatchment area (ft2)
    //           tStep = time step (sec)
    //  Output:  none
    //  Purpose: updates GW mass balance with volumes of water fluxes.
    //
    let vInfil: f64; // infiltration volume
    let vUpperEvap: f64; // upper zone evap. volume
    let vLowerEvap: f64; // lower zone evap. volume
    let vLowerPerc: f64; // lower zone deep perc. volume
    let vGwater: f64; // volume of exchanged groundwater
    let ft2sec = area * tStep;

    vInfil = prj.gwater.Infil * ft2sec;
    vUpperEvap = prj.gwater.UpperEvap * ft2sec;
    vLowerEvap = prj.gwater.LowerEvap * ft2sec;
    vLowerPerc = prj.gwater.LowerLoss * ft2sec;
    vGwater = 0.5 * (GW(prj).oldFlow + GW(prj).newFlow) * ft2sec;
    massbal_updateGwaterTotals(prj, vInfil, vUpperEvap, vLowerEvap, vLowerPerc, vGwater);
}

//=============================================================================

fn getFluxes(prj: &mut Project, theta: f64, lowerDepth: f64) {
    //
    //  Input:   upperVolume = vol. depth of upper zone (ft)
    //           upperDepth  = depth of upper zone (ft)
    //  Output:  none
    //  Purpose: computes water fluxes into/out of upper/lower GW zones.
    //
    let mut lowerDepth = lowerDepth;
    let upperDepth: f64;

    // --- find upper zone depth
    lowerDepth = MAX(lowerDepth, 0.0);
    lowerDepth = MIN(lowerDepth, prj.gwater.TotalDepth);
    upperDepth = prj.gwater.TotalDepth - lowerDepth;

    // --- save lower depth and theta to global variables
    prj.gwater.Hgw = lowerDepth;
    prj.gwater.Theta = theta;

    // --- find evaporation rate from both zones
    getEvapRates(prj, theta, upperDepth);

    // --- find percolation rate from upper to lower zone
    prj.gwater.UpperPerc = getUpperPerc(prj, theta, upperDepth);
    prj.gwater.UpperPerc = MIN(prj.gwater.UpperPerc, prj.gwater.MaxUpperPerc);

    // --- find loss rate to deep GW
    if prj.gwater.DeepFlowExpr >= 0 {
        let p: &Project = prj;
        let expr = p.Subcatch[p.gwater.DeepFlowExpr as usize].gwDeepFlowExpr.as_ref().unwrap();
        let v = mathexpr_eval(expr, &mut |varIndex: i32| getVariableValue(p, varIndex));
        prj.gwater.LowerLoss = v / UCF(prj, RAINFALL);
    } else {
        prj.gwater.LowerLoss = A(prj).lowerLossCoeff * lowerDepth / prj.gwater.TotalDepth;
    }
    prj.gwater.LowerLoss = MIN(prj.gwater.LowerLoss, lowerDepth / prj.gwater.Tstep);

    // --- find GW flow rate from lower zone to drainage system node
    prj.gwater.GWFlow = getGWFlow(prj, lowerDepth);
    if prj.gwater.LatFlowExpr >= 0 {
        let p: &Project = prj;
        let expr = p.Subcatch[p.gwater.LatFlowExpr as usize].gwLatFlowExpr.as_ref().unwrap();
        let v = mathexpr_eval(expr, &mut |varIndex: i32| getVariableValue(p, varIndex));
        prj.gwater.GWFlow += v / UCF(prj, GWFLOW);
    }
    if prj.gwater.GWFlow >= 0.0 {
        prj.gwater.GWFlow = MIN(prj.gwater.GWFlow, prj.gwater.MaxGWFlowPos);
    } else {
        prj.gwater.GWFlow = MAX(prj.gwater.GWFlow, prj.gwater.MaxGWFlowNeg);
    }
}

//=============================================================================

fn getDxDt(prj: &mut Project, t: f64, x: &[f64], dxdt: &mut [f64]) {
    //
    //  Input:   t    = current time (not used)
    //           x    = array of state variables
    //  Output:  dxdt = array of time derivatives of state variables
    //  Purpose: computes time derivatives of upper moisture content
    //           and lower depth.
    //
    let qUpper: f64; // inflow - outflow for upper zone (ft/sec)
    let qLower: f64; // inflow - outflow for lower zone (ft/sec)
    let mut denom: f64;

    getFluxes(prj, x[THETA], x[LOWERDEPTH]);
    let g = &prj.gwater;
    qUpper = g.Infil - g.UpperEvap - g.UpperPerc;
    qLower = g.UpperPerc - g.LowerLoss - g.LowerEvap - g.GWFlow;

    // --- d(upper zone moisture)/dt = (net upper zone flow) /
    //                                 (upper zone depth)
    denom = g.TotalDepth - x[LOWERDEPTH];
    if denom > 0.0 {
        dxdt[THETA] = qUpper / denom;
    } else {
        dxdt[THETA] = 0.0;
    }

    // --- d(lower zone depth)/dt = (net lower zone flow) /
    //                              (upper zone moisture deficit)
    denom = A(prj).porosity - x[THETA];
    if denom > 0.0 {
        dxdt[LOWERDEPTH] = qLower / denom;
    } else {
        dxdt[LOWERDEPTH] = 0.0;
    }
}

//=============================================================================

fn getEvapRates(prj: &mut Project, theta: f64, upperDepth: f64) {
    //
    //  Input:   theta      = moisture content of upper zone
    //           upperDepth = depth of upper zone (ft)
    //  Output:  none
    //  Purpose: computes evapotranspiration out of upper & lower zones.
    //
    let p: i32;
    let month: i32;
    let mut f: f64;
    let mut lowerFrac: f64;
    let mut upperFrac: f64;

    // --- no GW evaporation when infiltration is occurring
    prj.gwater.UpperEvap = 0.0;
    prj.gwater.LowerEvap = 0.0;
    if prj.gwater.Infil > 0.0 {
        return;
    }

    // --- get monthly-adjusted upper zone evap fraction
    upperFrac = A(prj).upperEvapFrac;
    f = 1.0;
    p = A(prj).upperEvapPat;
    if p >= 0 {
        month = datetime_monthOfYear(getDateTime(prj, prj.NewRunoffTime));
        f = prj.Pattern[p as usize].factor[(month - 1) as usize];
    }
    upperFrac *= f;

    // --- upper zone evaporation requires that soil moisture
    //     be above the wilting point
    if theta > A(prj).wiltingPoint {
        // --- actual evap is upper zone fraction applied to max. potential
        //     rate, limited by the available rate after any surface evap
        prj.gwater.UpperEvap = upperFrac * prj.gwater.MaxEvap;
        prj.gwater.UpperEvap = MIN(prj.gwater.UpperEvap, prj.gwater.AvailEvap);
    }

    // --- check if lower zone evaporation is possible
    if A(prj).lowerEvapDepth > 0.0 {
        // --- find the fraction of the lower evaporation depth that
        //     extends into the saturated lower zone
        lowerFrac = (A(prj).lowerEvapDepth - upperDepth) / A(prj).lowerEvapDepth;
        lowerFrac = MAX(0.0, lowerFrac);
        lowerFrac = MIN(lowerFrac, 1.0);

        // --- make the lower zone evap rate proportional to this fraction
        //     and the evap not used in the upper zone
        prj.gwater.LowerEvap = lowerFrac * (1.0 - upperFrac) * prj.gwater.MaxEvap;
        prj.gwater.LowerEvap = MIN(prj.gwater.LowerEvap, prj.gwater.AvailEvap - prj.gwater.UpperEvap);
    }
}

//=============================================================================

fn getUpperPerc(prj: &mut Project, theta: f64, upperDepth: f64) -> f64 {
    //
    //  Input:   theta      = moisture content of upper zone
    //           upperDepth = depth of upper zone (ft)
    //  Output:  returns percolation rate (ft/sec)
    //  Purpose: finds percolation rate from upper to lower zone.
    //
    let mut delta: f64; // unfilled water content of upper zone
    let dhdz: f64; // avg. change in head with depth
    let hydcon: f64; // unsaturated hydraulic conductivity

    // --- no perc. from upper zone if no depth or moisture content too low
    if upperDepth <= 0.0 || theta <= A(prj).fieldCapacity {
        return 0.0;
    }

    // --- compute hyd. conductivity as function of moisture content
    delta = theta - A(prj).porosity;
    hydcon = A(prj).conductivity * (delta * A(prj).conductSlope).exp();

    // --- compute integral of dh/dz term
    delta = theta - A(prj).fieldCapacity;
    dhdz = 1.0 + A(prj).tensionSlope * 2.0 * delta / upperDepth;

    // --- compute upper zone percolation rate
    prj.gwater.HydCon = hydcon;
    return hydcon * dhdz;
}

//=============================================================================

fn getGWFlow(prj: &Project, lowerDepth: f64) -> f64 {
    //
    //  Input:   lowerDepth = depth of lower zone (ft)
    //  Output:  returns groundwater flow rate (ft/sec)
    //  Purpose: finds groundwater outflow from lower saturated zone.
    //
    let mut q: f64;
    let t1: f64;
    let t2: f64;
    let t3: f64;
    let g = &prj.gwater;
    let gw = GW(prj);

    // --- water table must be above Hstar for flow to occur
    if lowerDepth <= g.Hstar {
        return 0.0;
    }

    // --- compute groundwater component of flow
    if gw.b1 == 0.0 {
        t1 = gw.a1;
    } else {
        t1 = gw.a1 * ((lowerDepth - g.Hstar) * UCF(prj, LENGTH)).powf(gw.b1);
    }

    // --- compute surface water component of flow
    if gw.b2 == 0.0 {
        t2 = gw.a2;
    } else if g.Hsw > g.Hstar {
        t2 = gw.a2 * ((g.Hsw - g.Hstar) * UCF(prj, LENGTH)).powf(gw.b2);
    } else {
        t2 = 0.0;
    }

    // --- compute groundwater/surface water interaction term
    t3 = gw.a3 * lowerDepth * g.Hsw * UCF(prj, LENGTH) * UCF(prj, LENGTH);

    // --- compute total groundwater flow
    q = (t1 - t2 + t3) / UCF(prj, GWFLOW);
    if q < 0.0 && gw.a3 != 0.0 {
        q = 0.0;
    }
    return q;
}

//=============================================================================

fn getVariableIndex(s: &[u8]) -> i32 {
    //
    //  Input:   s = name of a groundwater variable
    //  Output:  returns index of groundwater variable
    //  Purpose: finds position of GW variable in list of GW variable names.
    //
    let k: i32;

    k = findmatch(s, GWVarWords);
    if k >= 0 {
        return k;
    }
    return -1;
}

//=============================================================================

fn getVariableValue(prj: &Project, varIndex: i32) -> f64 {
    //
    //  Input:   varIndex = index of a GW variable
    //  Output:  returns current value of GW variable
    //  Purpose: finds current value of a GW variable.
    //
    let g = &prj.gwater;
    match varIndex {
        gwvHGW => return g.Hgw * UCF(prj, LENGTH),
        gwvHSW => return g.Hsw * UCF(prj, LENGTH),
        gwvHCB => return g.Hstar * UCF(prj, LENGTH),
        gwvHGS => return g.TotalDepth * UCF(prj, LENGTH),
        gwvKS => return A(prj).conductivity * UCF(prj, RAINFALL),
        gwvK => return g.HydCon * UCF(prj, RAINFALL),
        gwvTHETA => return g.Theta,
        gwvPHI => return A(prj).porosity,
        gwvFI => return g.Infil * UCF(prj, RAINFALL),
        gwvFU => return g.UpperPerc * UCF(prj, RAINFALL),
        gwvA => return g.Area * UCF(prj, LANDAREA),
        _ => return 0.0,
    }
}

//=============================================================================

/// input.c's `match` (funcs.h; no stub in input.rs): sees if a sub-string
/// of characters appears in a string (not case sensitive).
fn r#match(str: &[u8], substr: &[u8]) -> i32 {
    //
    //  Input:   str = character string being searched
    //           substr = sub-string being searched for
    //  Output:  returns 1 if sub-string found, 0 if not
    //  Purpose: sees if a sub-string of characters appears in a string
    //           (not case sensitive).
    //
    let str = cio::cstr(str);
    let substr = cio::cstr(substr);
    let at = |s: &[u8], i: usize| -> u8 { if i < s.len() { s[i] } else { 0 } };
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;

    // --- fail if substring is empty
    if at(substr, 0) == 0 {
        return 0;
    }

    // --- skip leading blanks of str
    k = 0;
    while at(str, k) != 0 {
        if at(str, k) != b' ' {
            break;
        }
        k += 1;
    }

    // --- check if substr matches remainder of str
    i = k;
    j = 0;
    while at(substr, j) != 0 {
        if at(str, i) == 0 || UCHAR(at(str, i)) != UCHAR(at(substr, j)) {
            return 0;
        }
        i += 1;
        j += 1;
    }
    return 1;
}
