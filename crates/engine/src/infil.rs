//! infil.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   infil.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//
//   Infiltration functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Revised formula for infiltration capacity recovery for the Modified
//     Horton method.
//   - The Green-Ampt functions were re-written.
//   Build 5.1.008:
//   - Monthly adjustment factors applied to hydraulic conductivity.
//   Build 5.1.010:
//   - Support for Modified Green Ampt model added.
//   - Green-Ampt initial recovery time set to 0.
//   Build 5.1.011:
//   - Monthly hydraulic conductivity factor also applied to Fu parameter
//     for Green-Ampt infiltration.
//   - Prevented computed Horton infiltration from dropping below 0.
//   Build 5.1.013:
//   - Support added for subcatchment-specific time patterns that adjust
//     hydraulic conductivity.
//   Build 5.1.015:
//   - Support added for multiple infiltration methods within a project.
//   Build 5.2.0:
//   - Additional validity check for G-A initial deficit added.
//   - New error message 235 added for invalid infiltration parameters.
//   - Conversion of runon to ponded depth fixed for Curve Number infiltration.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Local Variables
//-----------------------------------------------------------------------------
/// C: `typedef union TInfil { THorton horton; TGrnAmpt grnAmpt; TCurveNum
/// curveNum; } TInfil;` -- a subcatchment uses only the member its
/// infiltration model selects, and every field of that member is written
/// (by its setParams and initState) before it is read, so a struct holding
/// all three behaves as the union does.
#[derive(Clone, Copy, Default)]
struct TInfil {
    horton: THorton,
    grnAmpt: TGrnAmpt,
    curveNum: TCurveNum,
}

/// The file-scope variables of infil.c.
pub struct InfilState {
    /// C: `TInfil *Infil;` (one per subcatchment)
    Infil: Vec<TInfil>,
    /// saturated water volume in upper soil zone (ft)
    Fumax: f64,
    InfilFactor: f64,
    /// Not in C: a copy of the global `Evap.recoveryFactor`, which the
    /// Green-Ampt functions read but which `grnampt_getInfil` cannot reach
    /// (its callers in exfil.c and lidproc.c pass only this state). It is
    /// refreshed by `infil_setInfilFactor` and `infil_getInfil`, which C
    /// calls after the last change of `Evap.recoveryFactor` (climate_setState)
    /// and before every Green-Ampt evaluation (subcatch_getRunoff, LID units;
    /// routing's initSystemInflows before storage exfiltration).
    EvapRecoveryFactor: f64,
}

impl Default for InfilState {
    fn default() -> Self {
        InfilState {
            Infil: Vec::new(),
            Fumax: 0.0,
            InfilFactor: 0.0,
            // project.c's default for Evap.recoveryFactor
            EvapRecoveryFactor: 1.0,
        }
    }
}

//-----------------------------------------------------------------------------
//  External Functions (declared in infil.h)
//-----------------------------------------------------------------------------
//  infil_create     (called by createObjects in project.c)
//  infil_delete     (called by deleteObjects in project.c)
//  infil_readParams (called by input_readLine)
//  infil_initState  (called by subcatch_initState)
//  infil_getState   (called by writeRunoffFile in hotstart.c)
//  infil_setState   (called by readRunoffFile in hotstart.c)
//  infil_getInfil   (called by getSubareaRunoff in subcatch.c)

//  Called locally and by storage node methods in node.c
//  grnampt_setParams
//  grnampt_initState
//  grnampt_getInfil

//=============================================================================

/// Called by: project.
pub fn infil_create(prj: &mut Project, n: i32) {
    //
    //  Purpose: creates an array of infiltration objects.
    //  Input:   n = number of subcatchments
    //  Output:  none
    //
    prj.infil.Infil = vec![TInfil::default(); n.max(0) as usize];
    prj.infil.InfilFactor = 1.0;
    return;
}

//=============================================================================

/// Called by: project.
pub fn infil_delete(prj: &mut Project) {
    //
    //  Purpose: deletes infiltration objects associated with subcatchments
    //  Input:   none
    //  Output:  none
    //
    prj.infil.Infil = Vec::new();
}

//=============================================================================

/// Called by: input.
pub fn infil_readParams(prj: &mut Project, m: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   m = default infiltration model
    //           tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns an error code
    //  Purpose: sets infiltration parameters from a line of input data.
    //
    //  Format of data line is:
    //     subcatch  p1  p2 ... (infilMethod)
    let mut m = m;
    let mut ntoks = ntoks;
    let mut i: i32;
    let j: i32;
    let n: i32;
    let status: i32;
    let mut x = [0.0f64; 5];

    // --- check that subcatchment exists
    j = project_findObject(prj, SUBCATCH, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- check for infiltration method keyword is last token
    i = findmatch(&tok[(ntoks - 1) as usize], InfilModelWords);
    if i >= 0 {
        m = i;
        ntoks -= 1;
    }

    // --- number of input tokens depends on infiltration model m
    if m == HORTON {
        n = 5;
    } else if m == MOD_HORTON {
        n = 5;
    } else if m == GREEN_AMPT {
        n = 4;
    } else if m == MOD_GREEN_AMPT {
        n = 4;
    } else if m == CURVE_NUMBER {
        n = 4;
    } else {
        return 0;
    }

    if ntoks < n {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- parse numerical values from tokens
    i = 0;
    while i < 5 {
        x[i as usize] = 0.0;
        i += 1;
    }
    i = 1;
    while i < n {
        if getDouble(&tok[i as usize], &mut x[(i - 1) as usize]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
        }
        i += 1;
    }

    // --- special case for Horton infil. - last parameter is optional
    if (m == HORTON || m == MOD_HORTON) && ntoks > n {
        if getDouble(&tok[n as usize], &mut x[(n - 1) as usize]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[n as usize]);
        }
    }

    // --- assign parameter values to infil, infilModel object
    let ju = j as usize;
    prj.Subcatch[ju].infil = j;
    prj.Subcatch[ju].infilModel = m;
    match m {
        HORTON | MOD_HORTON => {
            let mut h = prj.infil.Infil[ju].horton;
            status = horton_setParams(prj, &mut h, &mut x);
            prj.infil.Infil[ju].horton = h;
        }
        GREEN_AMPT | MOD_GREEN_AMPT => {
            let mut g = prj.infil.Infil[ju].grnAmpt;
            status = grnampt_setParams(prj, &mut g, &x);
            prj.infil.Infil[ju].grnAmpt = g;
        }
        CURVE_NUMBER => {
            let mut c = prj.infil.Infil[ju].curveNum;
            status = curvenum_setParams(&mut c, &mut x);
            prj.infil.Infil[ju].curveNum = c;
        }
        _ => {
            status = TRUE;
        }
    }
    if status == 0 {
        return error_setInpError(prj, ERR_INFIL_PARAMS, b"");
    }
    return 0;
}

//=============================================================================

/// Called by: subcatch.
pub fn infil_initState(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: initializes state of infiltration for a subcatchment.
    //
    let ju = j as usize;
    match prj.Subcatch[ju].infilModel {
        HORTON | MOD_HORTON => horton_initState(&mut prj.infil.Infil[ju].horton),
        GREEN_AMPT | MOD_GREEN_AMPT => grnampt_initState(&mut prj.infil.Infil[ju].grnAmpt),
        CURVE_NUMBER => curvenum_initState(&mut prj.infil.Infil[ju].curveNum),
        _ => {}
    }
}

//=============================================================================

/// Called by: hotstart.
pub fn infil_getState(prj: &Project, j: i32, x: &mut [f64]) {
    //
    //  Input:   j = subcatchment index
    //  Output:  x = subcatchment's infiltration state
    //  Purpose: retrieves the current infiltration state for a subcatchment.
    //
    let ju = j as usize;
    match prj.Subcatch[ju].infilModel {
        HORTON | MOD_HORTON => horton_getState(&prj.infil.Infil[ju].horton, x),
        GREEN_AMPT | MOD_GREEN_AMPT => grnampt_getState(&prj.infil.Infil[ju].grnAmpt, x),
        CURVE_NUMBER => curvenum_getState(&prj.infil.Infil[ju].curveNum, x),
        _ => {}
    }
}

//=============================================================================

/// Called by: hotstart.
pub fn infil_setState(prj: &mut Project, j: i32, x: &[f64]) {
    //
    //  Input:   j = subcatchment index
    //           m = infiltration method code
    //  Output:  none
    //  Purpose: sets the current infiltration state for a subcatchment.
    //
    let ju = j as usize;
    match prj.Subcatch[ju].infilModel {
        HORTON | MOD_HORTON => horton_setState(&mut prj.infil.Infil[ju].horton, x),
        GREEN_AMPT | MOD_GREEN_AMPT => grnampt_setState(&mut prj.infil.Infil[ju].grnAmpt, x),
        CURVE_NUMBER => curvenum_setState(&mut prj.infil.Infil[ju].curveNum, x),
        _ => {}
    }
}

//=============================================================================

/// Called by: routing, subcatch.
pub fn infil_setInfilFactor(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: assigns a value to the infiltration adjustment factor.
    let m: i32;
    let p: i32;

    // (not in C: refresh the copy of Evap.recoveryFactor; see InfilState)
    prj.infil.EvapRecoveryFactor = prj.Evap.recoveryFactor;

    // ... set factor to the global conductivity adjustment factor
    prj.infil.InfilFactor = prj.Adjust.hydconFactor;

    // ... override global factor with subcatchment's adjustment if assigned
    if j >= 0 {
        p = prj.Subcatch[j as usize].infilPattern;
        if p >= 0 && prj.Pattern[p as usize].type_ == MONTHLY_PATTERN {
            m = datetime_monthOfYear(getDateTime(prj, prj.OldRunoffTime)) - 1;
            prj.infil.InfilFactor = prj.Pattern[p as usize].factor[m as usize];
        }
    }
}

//=============================================================================

/// Called by: lid, subcatch.
pub fn infil_getInfil(prj: &mut Project, j: i32, tstep: f64, rainfall: f64, runon: f64, depth: f64) -> f64 {
    //
    //  Input:   j = subcatchment index
    //           tstep = runoff time step (sec)
    //           rainfall = rainfall rate (ft/sec)
    //           runon = runon rate from other sub-areas or subcatchments (ft/sec)
    //           depth = depth of surface water on subcatchment (ft)
    //  Output:  returns infiltration rate (ft/sec)
    //  Purpose: computes infiltration rate depending on infiltration method.
    //
    let mut depth = depth;
    let ju = j as usize;
    match prj.Subcatch[ju].infilModel {
        HORTON => {
            let mut h = prj.infil.Infil[ju].horton;
            let r = horton_getInfil(prj, &mut h, tstep, rainfall + runon, depth);
            prj.infil.Infil[ju].horton = h;
            return r;
        }

        MOD_HORTON => {
            let mut h = prj.infil.Infil[ju].horton;
            let r = modHorton_getInfil(prj, &mut h, tstep, rainfall + runon, depth);
            prj.infil.Infil[ju].horton = h;
            return r;
        }

        GREEN_AMPT | MOD_GREEN_AMPT => {
            // (not in C: refresh the copy of Evap.recoveryFactor)
            prj.infil.EvapRecoveryFactor = prj.Evap.recoveryFactor;
            let modelType = prj.Subcatch[ju].infilModel;
            let mut g = prj.infil.Infil[ju].grnAmpt;
            let r = grnampt_getInfil(&mut prj.infil, &mut g, tstep, rainfall + runon, depth, modelType);
            prj.infil.Infil[ju].grnAmpt = g;
            return r;
        }

        CURVE_NUMBER => {
            depth += runon * tstep;
            let mut c = prj.infil.Infil[ju].curveNum;
            let r = curvenum_getInfil(prj, &mut c, tstep, rainfall, depth);
            prj.infil.Infil[ju].curveNum = c;
            return r;
        }

        _ => {
            return 0.0;
        }
    }
}

//=============================================================================

fn horton_setParams(prj: &Project, infil: &mut THorton, p: &mut [f64]) -> i32 {
    //
    //  Input:   infil = ptr. to Horton infiltration object
    //           p[] = array of parameter values
    //  Output:  returns TRUE if parameters are valid, FALSE otherwise
    //  Purpose: assigns Horton infiltration parameters to a subcatchment.
    //
    let mut k: i32 = 0;
    while k < 5 {
        if p[k as usize] < 0.0 {
            return FALSE;
        }
        k += 1;
    }

    // --- max. & min. infil rates (ft/sec)
    infil.f0 = p[0] / UCF(prj, RAINFALL);
    infil.fmin = p[1] / UCF(prj, RAINFALL);

    // --- convert decay const. to 1/sec
    infil.decay = p[2] / 3600.0;

    // --- convert drying time (days) to a regeneration const. (1/sec)
    //     assuming that former is time to reach 98% dry along an
    //     exponential drying curve
    if p[3] == 0.0 {
        p[3] = TINY;
    }
    infil.regen = -(1.0f64 - 0.98f64).ln() / p[3] / SECperDAY as f64;

    // --- optional max. infil. capacity (ft) (p[4] = 0 if no value supplied)
    infil.Fmax = p[4] / UCF(prj, RAINDEPTH);
    if infil.f0 < infil.fmin {
        return FALSE;
    }
    return TRUE;
}

//=============================================================================

fn horton_initState(infil: &mut THorton) {
    //
    //  Input:   infil = ptr. to Horton infiltration object
    //  Output:  none
    //  Purpose: initializes time on Horton infiltration curve for a subcatchment.
    //
    infil.tp = 0.0;
    infil.Fe = 0.0;
}

//=============================================================================

fn horton_getState(infil: &THorton, x: &mut [f64]) {
    x[0] = infil.tp;
    x[1] = infil.Fe;
}

fn horton_setState(infil: &mut THorton, x: &[f64]) {
    infil.tp = x[0];
    infil.Fe = x[1];
}

//=============================================================================

fn horton_getInfil(prj: &Project, infil: &mut THorton, tstep: f64, irate: f64, depth: f64) -> f64 {
    //
    //  Input:   infil = ptr. to Horton infiltration object
    //           tstep =  runoff time step (sec),
    //           irate = net "rainfall" rate (ft/sec),
    //                 = rainfall + snowmelt + runon - evaporation
    //           depth = depth of ponded water (ft).
    //  Output:  returns infiltration rate (ft/sec)
    //  Purpose: computes Horton infiltration for a subcatchment.
    //
    // --- assign local variables
    let mut iter: i32;
    let fa: f64;
    let mut fp: f64 = 0.0;
    let Fp: f64;
    let mut F1: f64;
    let t1: f64;
    let tlim: f64;
    let mut ex: f64;
    let mut kt: f64;
    let mut FF: f64;
    let mut FF1: f64;
    let mut r: f64;
    let f0 = infil.f0 * prj.infil.InfilFactor;
    let fmin = infil.fmin * prj.infil.InfilFactor;
    let Fmax = infil.Fmax;
    let mut tp = infil.tp;
    let df = f0 - fmin;
    let kd = infil.decay;
    let kr = infil.regen * prj.Evap.recoveryFactor;

    // --- special cases of no infil. or constant infil
    if df < 0.0 || kd < 0.0 || kr < 0.0 {
        return 0.0;
    }
    if df == 0.0 || kd == 0.0 {
        fp = f0;
        let fa = irate + depth / tstep;
        if fp > fa {
            fp = fa;
        }
        return MAX(0.0, fp);
    }

    // --- compute water available for infiltration
    fa = irate + depth / tstep;

    // --- case where there is water to infiltrate
    if fa > ZERO {
        // --- compute average infil. rate over time step
        t1 = tp + tstep; // future cumul. time
        tlim = 16.0 / kd; // for tp >= tlim, f = fmin
        if tp >= tlim {
            Fp = fmin * tp + df / kd;
            F1 = Fp + fmin * tstep;
        } else {
            Fp = fmin * tp + df / kd * (1.0 - (-kd * tp).exp());
            F1 = fmin * t1 + df / kd * (1.0 - (-kd * t1).exp());
        }
        fp = (F1 - Fp) / tstep;
        fp = MAX(fp, fmin);

        // --- limit infil rate to available infil
        if fp > fa {
            fp = fa;
        }

        // --- if fp on flat portion of curve then increase tp by tstep
        if t1 > tlim {
            tp = t1;
        }
        // --- if infil < available capacity then increase tp by tstep
        else if fp < fa {
            tp = t1;
        }
        // --- if infil limited by available capacity then
        //     solve F(tp) - F1 = 0 using Newton-Raphson method
        else {
            F1 = Fp + fp * tstep;
            tp = tp + tstep / 2.0;
            iter = 1;
            while iter <= 20 {
                kt = MIN(60.0, kd * tp);
                ex = (-kt).exp();
                FF = fmin * tp + df / kd * (1.0 - ex) - F1;
                FF1 = fmin + df * ex;
                r = FF / FF1;
                tp = tp - r;
                if r.abs() <= 0.001 * tstep {
                    break;
                }
                iter += 1;
            }
        }

        // --- limit cumulative infiltration to Fmax
        if Fmax > 0.0 {
            if infil.Fe + fp * tstep > Fmax {
                fp = (Fmax - infil.Fe) / tstep;
            }
            fp = MAX(fp, 0.0);
            infil.Fe += fp * tstep;
        }
    }
    // --- case where infil. capacity is regenerating; update tp.
    else if kr > 0.0 {
        r = (-kr * tstep).exp();
        tp = 1.0 - (-kd * tp).exp();
        tp = -(1.0 - r * tp).ln() / kd;

        // reduction in cumulative infiltration
        if Fmax > 0.0 {
            infil.Fe = fmin * tp + (df / kd) * (1.0 - (-kd * tp).exp());
        }
    }
    infil.tp = tp;
    return fp;
}

//=============================================================================

fn modHorton_getInfil(prj: &Project, infil: &mut THorton, tstep: f64, irate: f64, depth: f64) -> f64 {
    //
    //  Input:   infil = ptr. to Horton infiltration object
    //           tstep =  runoff time step (sec),
    //           irate = net "rainfall" rate (ft/sec),
    //                 = rainfall + snowmelt + runon
    //           depth = depth of ponded water (ft).
    //  Output:  returns infiltration rate (ft/sec)
    //  Purpose: computes modified Horton infiltration for a subcatchment.
    //
    // --- assign local variables
    let mut f = 0.0f64;
    let mut fp: f64;
    let fa: f64;
    let f0 = infil.f0 * prj.infil.InfilFactor;
    let fmin = infil.fmin * prj.infil.InfilFactor;
    let df = f0 - fmin;
    let kd = infil.decay;
    let kr = infil.regen * prj.Evap.recoveryFactor;

    // --- special cases of no or constant infiltration
    if df < 0.0 || kd < 0.0 || kr < 0.0 {
        return 0.0;
    }
    if df == 0.0 || kd == 0.0 {
        fp = f0;
        let fa = irate + depth / tstep;
        if fp > fa {
            fp = fa;
        }
        return MAX(0.0, fp);
    }

    // --- compute water available for infiltration
    fa = irate + depth / tstep;

    // --- case where there is water to infiltrate
    if fa > ZERO {
        // --- saturated condition
        if infil.Fmax > 0.0 && infil.Fe >= infil.Fmax {
            return 0.0;
        }

        // --- potential infiltration
        fp = f0 - kd * infil.Fe;
        fp = MAX(fp, fmin);

        // --- actual infiltration
        f = MIN(fa, fp);

        // --- new cumulative infiltration minus seepage
        infil.Fe += MAX(f - fmin, 0.0) * tstep;
        if infil.Fmax > 0.0 {
            infil.Fe = MAX(infil.Fe, infil.Fmax);
        }
    }
    // --- reduce cumulative infiltration for dry condition
    else if kr > 0.0 {
        infil.Fe *= (-kr * tstep).exp();
        infil.Fe = MAX(infil.Fe, 0.0);
    }
    return f;
}

//=============================================================================

/// Called by: lid.
pub fn grnampt_getParams(prj: &Project, j: i32, p: &mut [f64]) {
    //
    //  Input:   j = subcatchment index
    //           p[] = array of parameter values
    //  Output:  none
    //  Purpose: retrieves Green-Ampt infiltration parameters for a subcatchment.
    //
    let ju = j as usize;
    p[0] = prj.infil.Infil[ju].grnAmpt.S * UCF(prj, RAINDEPTH); // Capillary suction head (ft)
    p[1] = prj.infil.Infil[ju].grnAmpt.Ks * UCF(prj, RAINFALL); // Sat. hyd. conductivity (ft/sec)
    p[2] = prj.infil.Infil[ju].grnAmpt.IMDmax; // Max. init. moisture deficit
}

//=============================================================================

/// Called by: exfil, lid.
///
/// Signature changed from the stub: C's `UCF()` needs the model's unit
/// system, so the model is passed first.
pub fn grnampt_setParams(prj: &Project, infil: &mut TGrnAmpt, p: &[f64]) -> i32 {
    //
    //  Input:   infil = ptr. to Green-Ampt infiltration object
    //           p[] = array of parameter values
    //  Output:  returns TRUE if parameters are valid, FALSE otherwise
    //  Purpose: assigns Green-Ampt infiltration parameters to a subcatchment.
    //
    let ksat: f64; // sat. hyd. conductivity in in/hr

    if p[0] < 0.0 || p[1] <= 0.0 || p[2] < 0.0 || p[2] > 1.0 {
        return FALSE;
    }
    infil.S = p[0] / UCF(prj, RAINDEPTH); // Capillary suction head (ft)
    infil.Ks = p[1] / UCF(prj, RAINFALL); // Sat. hyd. conductivity (ft/sec)
    infil.IMDmax = p[2]; // Max. init. moisture deficit

    // --- find depth of upper soil zone (ft) using Mein's eqn.
    ksat = infil.Ks * 12.0 * 3600.0;
    infil.Lu = 4.0 * ksat.sqrt() / 12.0;
    return TRUE;
}

//=============================================================================

/// Called by: exfil, lid.
pub fn grnampt_initState(infil: &mut TGrnAmpt) {
    //
    //  Input:   infil = ptr. to Green-Ampt infiltration object
    //  Output:  none
    //  Purpose: initializes state of Green-Ampt infiltration for a subcatchment.
    //
    // (C: if (infil == NULL) return; -- a reference is never NULL; callers
    //  holding an Option skip the call instead)
    infil.IMD = infil.IMDmax;
    infil.Fu = 0.0;
    infil.F = 0.0;
    infil.Sat = FALSE as i8;
    infil.T = 0.0;
}

fn grnampt_getState(infil: &TGrnAmpt, x: &mut [f64]) {
    x[0] = infil.IMD;
    x[1] = infil.F;
    x[2] = infil.Fu;
    x[3] = infil.Sat as f64;
    x[4] = infil.T;
}

fn grnampt_setState(infil: &mut TGrnAmpt, x: &[f64]) {
    infil.IMD = x[0];
    infil.F = x[1];
    infil.Fu = x[2];
    infil.Sat = f64_to_i8(x[3]);
    infil.T = x[4];
}

//=============================================================================

/// Called by: exfil, lidproc.
pub fn grnampt_getInfil(ist: &mut crate::infil::InfilState, infil: &mut TGrnAmpt, tstep: f64, irate: f64, depth: f64, modelType: i32) -> f64 {
    //
    //  Input:   infil = ptr. to Green-Ampt infiltration object
    //           tstep =  time step (sec),
    //           irate = net "rainfall" rate to upper zone (ft/sec);
    //                 = rainfall + snowmelt + runon,
    //                   does not include ponded water (added on below)
    //           depth = depth of ponded water (ft)
    //           modelType = either GREEN_AMPT or MOD_GREEN_AMPT
    //  Output:  returns infiltration rate (ft/sec)
    //  Purpose: computes Green-Ampt infiltration for a subcatchment
    //           or a storage node.
    //
    // --- find saturated upper soil zone water volume
    ist.Fumax = infil.IMDmax * infil.Lu * ist.InfilFactor.sqrt();

    // --- reduce time until next event
    infil.T -= tstep;

    // --- use different procedures depending on upper soil zone saturation
    if infil.Sat != 0 {
        return grnampt_getSatInfil(ist, infil, tstep, irate, depth);
    } else {
        return grnampt_getUnsatInfil(ist, infil, tstep, irate, depth, modelType);
    }
}

//=============================================================================

fn grnampt_getUnsatInfil(ist: &mut InfilState, infil: &mut TGrnAmpt, tstep: f64, irate: f64, depth: f64, modelType: i32) -> f64 {
    //
    //  Input:   infil = ptr. to Green-Ampt infiltration object
    //           tstep =  runoff time step (sec),
    //           irate = net "rainfall" rate to upper zone (ft/sec);
    //                 = rainfall + snowmelt + runon,
    //                   does not include ponded water (added on below)
    //           depth = depth of ponded water (ft)
    //           modelType = either GREEN_AMPT or MOD_GREEN_AMPT
    //  Output:  returns infiltration rate (ft/sec)
    //  Purpose: computes Green-Ampt infiltration when upper soil zone is
    //           unsaturated.
    //
    let mut ia: f64;
    let c1: f64;
    let mut F2: f64;
    let dF: f64;
    let Fs: f64;
    let kr: f64;
    let mut ts: f64;
    let ks = infil.Ks * ist.InfilFactor;
    let lu = infil.Lu * ist.InfilFactor.sqrt();
    let Fumax = ist.Fumax;

    // --- get available infiltration rate (rainfall + ponded water)
    ia = irate + depth / tstep;
    if ia < ZERO {
        ia = 0.0;
    }

    // --- no rainfall so recover upper zone moisture
    if ia == 0.0 {
        if infil.Fu <= 0.0 {
            return 0.0;
        }
        kr = lu / 90000.0 * ist.EvapRecoveryFactor;
        let dF = kr * Fumax * tstep;
        infil.F -= dF;
        infil.Fu -= dF;
        if infil.Fu <= 0.0 {
            infil.Fu = 0.0;
            infil.F = 0.0;
            infil.IMD = infil.IMDmax;
            return 0.0;
        }

        // --- if new wet event begins then reset IMD & F
        if infil.T <= 0.0 {
            infil.IMD = (Fumax - infil.Fu) / lu;
            infil.F = 0.0;
        }
        return 0.0;
    }

    // --- rainfall does not exceed Ksat
    if ia <= ks {
        let dF = ia * tstep;
        infil.F += dF;
        infil.Fu += dF;
        infil.Fu = MIN(infil.Fu, Fumax);
        if modelType == GREEN_AMPT && infil.T <= 0.0 {
            infil.IMD = (Fumax - infil.Fu) / lu;
            infil.F = 0.0;
        }
        return ia;
    }

    // --- rainfall exceeds Ksat; renew time to drain upper zone
    infil.T = 5400.0 / lu / ist.EvapRecoveryFactor;

    // --- find volume needed to saturate surface layer
    Fs = ks * (infil.S + depth) * infil.IMD / (ia - ks);

    // --- surface layer already saturated
    if infil.F > Fs {
        infil.Sat = TRUE as i8;
        return grnampt_getSatInfil(ist, infil, tstep, irate, depth);
    }

    // --- surface layer remains unsaturated
    if infil.F + ia * tstep < Fs {
        let dF = ia * tstep;
        infil.F += dF;
        infil.Fu += dF;
        infil.Fu = MIN(infil.Fu, Fumax);
        return ia;
    }

    // --- surface layer becomes saturated during time step;
    // --- compute portion of tstep when saturated
    ts = tstep - (Fs - infil.F) / ia;
    if ts <= 0.0 {
        ts = 0.0;
    }

    // --- compute new total volume infiltrated
    c1 = (infil.S + depth) * infil.IMD;
    F2 = grnampt_getF2(Fs, c1, ks, ts);
    if F2 > Fs + ia * ts {
        F2 = Fs + ia * ts;
    }

    // --- compute infiltration rate
    dF = F2 - infil.F;
    infil.F = F2;
    infil.Fu += dF;
    infil.Fu = MIN(infil.Fu, Fumax);
    infil.Sat = TRUE as i8;
    return dF / tstep;
}

//=============================================================================

fn grnampt_getSatInfil(ist: &mut InfilState, infil: &mut TGrnAmpt, tstep: f64, irate: f64, depth: f64) -> f64 {
    //
    //  Input:   infil = ptr. to Green-Ampt infiltration object
    //           tstep =  runoff time step (sec),
    //           irate = net "rainfall" rate to upper zone (ft/sec);
    //                 = rainfall + snowmelt + runon,
    //                   does not include ponded water (added on below)
    //           depth = depth of ponded water (ft).
    //  Output:  returns infiltration rate (ft/sec)
    //  Purpose: computes Green-Ampt infiltration when upper soil zone is
    //           saturated.
    //
    let ia: f64;
    let c1: f64;
    let mut dF: f64;
    let F2: f64;
    let ks = infil.Ks * ist.InfilFactor;
    let lu = infil.Lu * ist.InfilFactor.sqrt();

    // --- get available infiltration rate (rainfall + ponded water)
    ia = irate + depth / tstep;
    if ia < ZERO {
        return 0.0;
    }

    // --- re-set new event recovery time
    infil.T = 5400.0 / lu / ist.EvapRecoveryFactor;

    // --- solve G-A equation for new cumulative infiltration volume (F2)
    c1 = (infil.S + depth) * infil.IMD;
    F2 = grnampt_getF2(infil.F, c1, ks, tstep);
    dF = F2 - infil.F;

    // --- all available water infiltrates -- set saturated state to false
    if dF > ia * tstep {
        dF = ia * tstep;
        infil.Sat = FALSE as i8;
    }

    // --- update total infiltration and upper zone moisture deficit
    infil.F += dF;
    infil.Fu += dF;
    infil.Fu = MIN(infil.Fu, ist.Fumax);
    return dF / tstep;
}

//=============================================================================

fn grnampt_getF2(f1: f64, c1: f64, ks: f64, ts: f64) -> f64 {
    //
    //  Input:   f1 = old infiltration volume (ft)
    //           c1 = head * moisture deficit (ft)
    //           ks = sat. hyd. conductivity (ft/sec)
    //           ts = time step (sec)
    //  Output:  returns infiltration volume at end of time step (ft)
    //  Purpose: computes new infiltration volume over a time step
    //           using Green-Ampt formula for saturated upper soil zone.
    //
    let mut i: i32;
    let mut f2 = f1;
    let f2min: f64;
    let mut df2: f64;
    let c2: f64;

    // --- find min. infil. volume
    f2min = f1 + ks * ts;

    // --- use min. infil. volume for 0 moisture deficit
    if c1 == 0.0 {
        return f2min;
    }

    // --- use direct form of G-A equation for small time steps
    //     and c1/f1 < 100
    if ts < 10.0 && f1 > 0.01 * c1 {
        f2 = f1 + ks * (1.0 + c1 / f1) * ts;
        return MAX(f2, f2min);
    }

    // --- use Newton-Raphson method to solve integrated G-A equation
    //     (convergence limit reduced from that used in previous releases)
    c2 = c1 * (f1 + c1).ln() - ks * ts;
    i = 1;
    while i <= 20 {
        df2 = (f2 - f1 - c1 * (f2 + c1).ln() + c2) / (1.0 - c1 / (f2 + c1));
        if df2.abs() < 0.00001 {
            return MAX(f2, f2min);
        }
        f2 -= df2;
        i += 1;
    }
    return f2min;
}

//=============================================================================

fn curvenum_setParams(infil: &mut TCurveNum, p: &mut [f64]) -> i32 {
    //
    //  Input:   infil = ptr. to Curve Number infiltration object
    //           p[] = array of parameter values
    //  Output:  returns TRUE if parameters are valid, FALSE otherwise
    //  Purpose: assigns Curve Number infiltration parameters to a subcatchment.
    //

    // --- convert Curve Number to max. infil. capacity
    if p[0] < 10.0 {
        p[0] = 10.0;
    }
    if p[0] > 99.0 {
        p[0] = 99.0;
    }
    infil.Smax = (1000.0 / p[0] - 10.0) / 12.0;
    if infil.Smax < 0.0 {
        return FALSE;
    }

    // --- convert drying time (days) to a regeneration const. (1/sec)
    if p[2] > 0.0 {
        infil.regen = 1.0 / (p[2] * SECperDAY as f64);
    } else {
        return FALSE;
    }

    // --- compute inter-event time from regeneration const. as in Green-Ampt
    infil.Tmax = 0.06 / infil.regen;

    return TRUE;
}

//=============================================================================

fn curvenum_initState(infil: &mut TCurveNum) {
    //
    //  Input:   infil = ptr. to Curve Number infiltration object
    //  Output:  none
    //  Purpose: initializes state of Curve Number infiltration for a subcatchment.
    //
    infil.S = infil.Smax;
    infil.P = 0.0;
    infil.F = 0.0;
    infil.T = 0.0;
    infil.Se = infil.Smax;
    infil.f = 0.0;
}

fn curvenum_getState(infil: &TCurveNum, x: &mut [f64]) {
    x[0] = infil.S;
    x[1] = infil.P;
    x[2] = infil.F;
    x[3] = infil.T;
    x[4] = infil.Se;
    x[5] = infil.f;
}

fn curvenum_setState(infil: &mut TCurveNum, x: &[f64]) {
    infil.S = x[0];
    infil.P = x[1];
    infil.F = x[2];
    infil.T = x[3];
    infil.Se = x[4];
    infil.f = x[5];
}

//=============================================================================

fn curvenum_getInfil(prj: &Project, infil: &mut TCurveNum, tstep: f64, irate: f64, depth: f64) -> f64 {
    //
    //  Input:   infil = ptr. to Curve Number infiltration object
    //           tstep = runoff time step (sec),
    //           irate = rainfall rate (ft/sec);
    //           depth = depth of runon + ponded water (ft)
    //  Output:  returns infiltration rate (ft/sec)
    //  Purpose: computes infiltration rate using the Curve Number method.
    //  Note:    this function treats runon from other subcatchments as part
    //           of the ponded depth and not as an effective rainfall rate.
    let F1: f64; // new cumulative infiltration (ft)
    let mut f1 = 0.0f64; // new infiltration rate (ft/sec)
    let fa = irate + depth / tstep; // max. available infil. rate (ft/sec)

    // --- case where there is rainfall
    if irate > ZERO {
        // --- check if new rain event
        if infil.T >= infil.Tmax {
            infil.P = 0.0;
            infil.F = 0.0;
            infil.f = 0.0;
            infil.Se = infil.S;
        }
        infil.T = 0.0;

        // --- update cumulative precip.
        infil.P += irate * tstep;

        // --- find potential new cumulative infiltration
        F1 = infil.P * (1.0 - infil.P / (infil.P + infil.Se));

        // --- compute potential infiltration rate
        f1 = (F1 - infil.F) / tstep;
        if f1 < 0.0 || infil.S <= 0.0 {
            f1 = 0.0;
        }
    }
    // --- case of no rainfall
    else {
        // --- if there is ponded water then use previous infil. rate
        if depth > MIN_TOTAL_DEPTH && infil.S > 0.0 {
            f1 = infil.f;
            if f1 * tstep > infil.S {
                f1 = infil.S / tstep;
            }
        }
        // --- otherwise update inter-event time
        else {
            infil.T += tstep;
        }
    }

    // --- if there is some infiltration
    if f1 > 0.0 {
        // --- limit infil. rate to max. available rate
        f1 = MIN(f1, fa);
        f1 = MAX(f1, 0.0);

        // --- update actual cumulative infiltration
        infil.F += f1 * tstep;

        // --- reduce infil. capacity if a regen. constant was supplied
        if infil.regen > 0.0 {
            infil.S -= f1 * tstep;
            if infil.S < 0.0 {
                infil.S = 0.0;
            }
        }
    }
    // --- otherwise regenerate infil. capacity
    else {
        infil.S += infil.regen * infil.Smax * tstep * prj.Evap.recoveryFactor;
        if infil.S > infil.Smax {
            infil.S = infil.Smax;
        }
    }
    infil.f = f1;
    return f1;
}
