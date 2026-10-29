//! forcmain.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   forcemain.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//
//   Special Non-Manning Force Main functions
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const VISCOS: f64 = 1.1E-5; // Kinematic viscosity of water
                            // @ 20 deg C (sq ft/sec)

/// The file-scope variables of forcmain.c (it has none).
#[derive(Default)]
pub struct ForcmainState {
}

//=============================================================================

/// Called by: link.
pub fn forcemain_getEquivN(prj: &mut Project, j: i32, k: i32) -> f64
//
//  Input:   j = link index
//           k = conduit index
//  Output:  returns an equivalent Manning's n for a force main
//  Purpose: computes a Mannng's n that results in the same normal flow
//           value for a force main flowing full under fully turbulent
//           conditions using either the Hazen-Williams or Dary-Weisbach
//           flow equations.
//
{
    let xsect: TXsect = prj.Link[j as usize].xsect;
    let f: f64;
    let d: f64 = xsect.yFull;
    match prj.ForceMainEqn {
        H_W => {
            return 1.067 / xsect.rBot * (d / prj.Conduit[k as usize].slope).powf(0.04);
        }
        D_W => {
            f = forcemain_getFricFactor(xsect.rBot, d / 4.0, 1.0e12);
            return (f / 185.0).sqrt() * d.powf(1. / 6.);
        }
        _ => {}
    }
    prj.Conduit[k as usize].roughness
}

//=============================================================================

/// Called by: link.
pub fn forcemain_getRoughFactor(prj: &mut Project, j: i32, lengthFactor: f64) -> f64
//
//  Input:   j = link index
//           lengthFactor = factor by which a pipe will be artifically lengthened
//  Output:  returns a roughness adjustment factor for a force main
//  Purpose: computes an adjustment factor for a force main that compensates for
//           any artificial lengthening the pipe may have received.
//
{
    let xsect: TXsect = prj.Link[j as usize].xsect;
    let r: f64;
    match prj.ForceMainEqn {
        H_W => {
            r = 1.318 * xsect.rBot * lengthFactor.powf(0.54);
            return GRAVITY / r.powf(1.852);
        }
        D_W => {
            return 1.0 / 8.0 / lengthFactor;
        }
        _ => {}
    }
    0.0
}

//=============================================================================

/// Called by: dwflow.
pub fn forcemain_getFricSlope(prj: &mut Project, j: i32, v: f64, hrad: f64) -> f64
//
//  Input:   j = link index
//           v = flow velocity (ft/sec)
//           hrad = hydraulic radius (ft)
//  Output:  returns a force main pipe's friction slope
//  Purpose: computes the headloss per unit length used in dynamic wave
//           flow routing for a pressurized force main using either the
//           Hazen-Williams or Darcy-Weisbach flow equations.
//  Note:    the pipe's roughness factor was saved in xsect.sBot in
//           conduit_validate() in LINK.C.
//
{
    let re: f64;
    let f: f64;
    let xsect: TXsect = prj.Link[j as usize].xsect;
    match prj.ForceMainEqn {
        H_W => {
            return xsect.sBot * v.powf(0.852) / hrad.powf(1.1667);
        }
        D_W => {
            re = forcemain_getReynolds(v, hrad);
            f = forcemain_getFricFactor(xsect.rBot, hrad, re);
            return f * xsect.sBot * v / hrad;
        }
        _ => {}
    }
    0.0
}

//=============================================================================

fn forcemain_getReynolds(v: f64, hrad: f64) -> f64
//
//  Input:   v = flow velocity (ft/sec)
//           hrad = hydraulic radius (ft)
//  Output:  returns a flow's Reynolds Number
//  Purpose: computes a flow's Reynolds Number
//
{
    4.0 * hrad * v / VISCOS
}

//=============================================================================

fn forcemain_getFricFactor(e: f64, hrad: f64, mut re: f64) -> f64
//
//  Input:   e = roughness height (ft)
//           hrad = hydraulic radius (ft)
//           re = Reynolds number
//  Output:  returns a Darcy-Weisbach friction factor
//  Purpose: computes the Darcy-Weisbach friction factor for a force main
//           using the Swamee and Jain approximation to the Colebrook-White
//           equation.
//
{
    let mut f: f64;
    if re < 10.0 {
        re = 10.0;
    }
    if re <= 2000.0 {
        f = 64.0 / re;
    } else if re < 4000.0 {
        f = forcemain_getFricFactor(e, hrad, 4000.0);
        f = 0.032 + (f - 0.032) * (re - 2000.0) / 2000.0;
    } else {
        f = e / 3.7 / (4.0 * hrad);
        if re < 1.0e10 {
            f += 5.74 / re.powf(0.9);
        }
        f = f.log10();
        f = 0.25 / f / f;
    }
    f
}
