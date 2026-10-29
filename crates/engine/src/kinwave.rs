//! kinwave.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   kinwave.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     06/12/23  (Build 5.2.4)
//   Author:   L. Rossman
//             M. Tryby (EPA)
//
//   Kinematic wave flow routing functions.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Conduit inflow passed to function that computes conduit losses.
//   Build 5.1.014:
//   - Arguments to function link_getLossRate changed.
//   Build 5.2.4:
//   - Arguments to function link_getLossRate changed again.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const WX: f64 = 0.6;     // distance weighting
const WT: f64 = 0.6;     // time weighting
const EPSIL: f64 = 0.001;   // convergence criterion

/// The file-scope variables of kinwave.c.
#[derive(Default)]
pub struct KinwaveState {
    Beta1: f64,
    C1: f64,
    C2: f64,
    Afull: f64,
    Qfull: f64,
    /// C: `TXsect* pXsect` (pointer to the current link's cross-section);
    /// here the index of that link, so lookups work on Link[j].xsect itself
    /// (FILLED_CIRCULAR's round trip edits it, as in C).
    pLink: i32,
}

//=============================================================================

/// Called by: flowrout.
pub fn kinwave_execute(prj: &mut Project, j: i32, qinflow: &mut f64, qoutflow: &mut f64, tStep: f64) -> i32 {
//
//  Input:   j = link index
//           qinflow = inflow at current time (cfs)
//           tStep = time step (sec)
//  Output:  qoutflow = outflow at current time (cfs),
//           returns number of iterations used
//  Purpose: finds outflow over time step tStep given flow entering a
//           conduit using Kinematic Wave flow routing.
//
//
//                               ^ q3
//  t                            |
//  |          qin, ain |-------------------| qout, aout
//  |                   |  Flow --->        |
//  |----> x     q1, a1 |-------------------| q2, a2
//
//
    let k: i32;
    let mut result: i32 = 1;
    let dxdt: f64;
    let dq: f64;
    let ain: f64;
    let mut aout: f64;
    let mut qin: f64;
    let qout: f64;
    let a1: f64;
    let a2: f64;
    let q1: f64;
    let q2: f64;
    let q3: f64;
    let ju = j as usize;

    // --- no routing for non-conduit link
    *qoutflow = *qinflow;
    if prj.Link[ju].type_ != CONDUIT {
        return result;
    }

    // --- no routing for dummy xsection
    if prj.Link[ju].xsect.type_ == DUMMY {
        return result;
    }

    // --- assign module-level variables
    prj.kinwave.pLink = j;
    prj.kinwave.Qfull = prj.Link[ju].qFull;
    prj.kinwave.Afull = prj.Link[ju].xsect.aFull;
    k = prj.Link[ju].subIndex;
    let ku = k as usize;
    prj.kinwave.Beta1 = prj.Conduit[ku].beta / prj.kinwave.Qfull;
    let Qfull = prj.kinwave.Qfull;
    let Afull = prj.kinwave.Afull;

    // --- normalize previous flows
    q1 = prj.Conduit[ku].q1 / Qfull;
    q2 = prj.Conduit[ku].q2 / Qfull;

    // --- normalize inflow
    qin = *qinflow / prj.Conduit[ku].barrels as f64 / Qfull;

    // --- compute evaporation and infiltration loss rate
    q3 = link_getLossRate(prj, j, KW, qin * Qfull, tStep) / Qfull;

    // --- normalize previous areas
    a1 = prj.Conduit[ku].a1 / Afull;
    a2 = prj.Conduit[ku].a2 / Afull;

    // --- use full area when inlet flow >= full flow
    if qin >= 1.0 {
        ain = 1.0;
    }
    // --- get normalized inlet area corresponding to inlet flow
    else {
        let s = qin / prj.kinwave.Beta1;
        ain = linkXsect_getAofS(prj, j, s) / Afull;
    }

    // --- check for no flow
    if qin <= TINY && q2 <= TINY {
        qout = 0.0;
        aout = 0.0;
    }
    // --- otherwise solve finite difference form of continuity eqn.
    else {
        // --- compute constant factors
        dxdt = link_getLength(prj, j) / tStep * Afull / Qfull;
        dq = q2 - q1;
        prj.kinwave.C1 = dxdt * WT / WX;
        let mut C2: f64;
        C2 = (1.0 - WT) * (ain - a1);
        C2 = C2 - WT * a2;
        C2 = C2 * dxdt / WX;
        C2 = C2 + (1.0 - WX) / WX * dq - qin;
        C2 = C2 + q3 / WX;
        prj.kinwave.C2 = C2;

        // --- starting guess for aout is value from previous time step
        aout = a2;

        // --- solve continuity equation for aout
        result = solveContinuity(prj, qin, ain, &mut aout);

        // --- report error if continuity eqn. not solved
        if result == -1 {
            let id = prj.Link[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_KINWAVE, &id);
            return 1;
        }
        if result <= 0 {
            result = 1;
        }

        // --- compute normalized outlet flow from outlet area
        qout = prj.kinwave.Beta1 * linkXsect_getSofA(prj, j, aout * Afull);
        if qin > 1.0 {
            qin = 1.0;
        }
    }

    // --- save new flows and areas
    prj.Conduit[ku].q1 = qin * Qfull;
    prj.Conduit[ku].a1 = ain * Afull;
    prj.Conduit[ku].q2 = qout * Qfull;
    prj.Conduit[ku].a2 = aout * Afull;
    prj.Conduit[ku].fullState =
        link_getFullState(prj.Conduit[ku].a1, prj.Conduit[ku].a2, Afull);
    *qinflow = prj.Conduit[ku].q1 * prj.Conduit[ku].barrels as f64;
    *qoutflow = prj.Conduit[ku].q2 * prj.Conduit[ku].barrels as f64;
    result
}

//=============================================================================

fn solveContinuity(prj: &mut Project, qin: f64, ain: f64, aout: &mut f64) -> i32 {
//
//  Input:   qin = upstream normalized flow
//           ain = upstream normalized area
//           aout = downstream normalized area
//  Output:  new value for aout; returns an error code
//  Purpose: solves continuity equation f(a) = Beta1*S(a) + C1*a + C2 = 0
//           for 'a' using the Newton-Raphson root finder function.
//           Return code has the following meanings:
//           >= 0 number of function evaluations used
//           -1   Newton function failed
//           -2   flow always above max. flow
//           -3   flow always below zero
//
//     Note: pXsect (pointer to conduit's cross-section), and constants Beta1,
//           C1, and C2 are module-level shared variables assigned values
//           in kinwave_execute().
//
    let mut n: i32;                    // # evaluations or error code
    let mut aLo: f64;                  // lower/upper bounds on a
    let mut aHi: f64;
    let aTmp: f64;
    let mut fLo: f64;                  // lower/upper bounds on f
    let mut fHi: f64;
    let tol: f64 = EPSIL;              // absolute convergence tol.
    let (Beta1, C1, C2, Afull, pl) = (prj.kinwave.Beta1, prj.kinwave.C1, prj.kinwave.C2, prj.kinwave.Afull, prj.kinwave.pLink as usize);

    // --- first determine bounds on 'a' so that f(a) passes through 0.

    // --- set upper bound to area at full flow
    aHi = 1.0;
    fHi = 1.0 + C1 + C2;

    // --- try setting lower bound to area where section factor is maximum
    aLo = xsect_getAmax(prj, &prj.Link[pl].xsect) / Afull;
    if aLo < aHi {
        fLo = (Beta1 * prj.Link[pl].xsect.sMax) + (C1 * aLo) + C2;
    } else {
        fLo = fHi;
    }

    // --- if fLo and fHi have same sign then set lower bound to 0
    if fHi * fLo > 0.0 {
        aHi = aLo;
        fHi = fLo;
        aLo = 0.0;
        fLo = C2;
    }

    // --- proceed with search for root if fLo and fHi have different signs
    if fHi * fLo <= 0.0 {
        // --- start search at midpoint of lower/upper bounds
        //     if initial value outside of these bounds
        if *aout < aLo || *aout > aHi {
            *aout = 0.5 * (aLo + aHi);
        }

        // --- if fLo > fHi then switch aLo and aHi
        if fLo > fHi {
            aTmp = aLo;
            aLo = aHi;
            aHi = aTmp;
        }

        // --- call the Newton root finder method passing it the
        //     evalContinuity function to evaluate the function
        //     and its derivatives
        n = findroot_Newton(aLo, aHi, aout, tol, &mut |a: f64, f: &mut f64, df: &mut f64| {
            evalContinuity(prj, a, f, df)
        });

        // --- check if root finder succeeded
        if n <= 0 {
            n = -1;
        }
    }
    // --- if lower/upper bound functions both negative then use full flow
    else if fLo < 0.0 {
        if qin > 1.0 {
            *aout = ain;
        } else {
            *aout = 1.0;
        }
        n = -2;
    }
    // --- if lower/upper bound functions both positive then use no flow
    else if fLo > 0 as f64 {
        *aout = 0.0;
        n = -3;
    } else {
        n = -1;
    }
    n
}

//=============================================================================

fn evalContinuity(prj: &mut Project, a: f64, f: &mut f64, df: &mut f64) {
//
//  Input:   a = outlet normalized area
//  Output:  f = value of continuity eqn.
//           df = derivative of continuity eqn.
//  Purpose: computes value of continuity equation (f) and its derivative (df)
//           w.r.t. normalized area for link with normalized outlet area 'a'.
//
    let (Beta1, C1, C2, Afull, pl) = (prj.kinwave.Beta1, prj.kinwave.C1, prj.kinwave.C2, prj.kinwave.Afull, prj.kinwave.pLink);
    *f = (Beta1 * linkXsect_getSofA(prj, pl, a * Afull)) + (C1 * a) + C2;
    *df = (Beta1 * Afull * linkXsect_getdSdA(prj, pl, a * Afull)) + C1;
}

//=============================================================================
