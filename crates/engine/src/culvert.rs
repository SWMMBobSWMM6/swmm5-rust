//! culvert.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   culvert.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//
//   Culvert equations for SWMM5
//
//   Computes flow reduction in a culvert-type conduit due to
//   inlet control using equations from the FHWA HEC-5 circular.
//
//   Update History
//   ==============
//   Build 5.1.013:
//   - C parameter corrected for Arch, Corrugated Metal, Mitered culvert.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
// enum CulvertParam {FORM, K, M, C, Y};
const FORM: usize = 0;
const K: usize = 1;
const M: usize = 2;
const C: usize = 3;
const Y: usize = 4;
const MAX_CULVERT_CODE: i32 = 57;
const Params: [[f64; 5]; 58] = [
    //   FORM   K       M     C        Y
    //------------------------------------
    [0.0, 0.0, 0.0, 0.0, 0.00],
    //Circular concrete
    [1.0, 0.0098, 2.00, 0.0398, 0.67], //Square edge w/headwall
    [1.0, 0.0018, 2.00, 0.0292, 0.74], //Groove end w/headwall
    [1.0, 0.0045, 2.00, 0.0317, 0.69], //Groove end projecting
    //Circular Corrugated Metal Pipe
    [1.0, 0.0078, 2.00, 0.0379, 0.69], //Headwall
    [1.0, 0.0210, 1.33, 0.0463, 0.75], //Mitered to slope
    [1.0, 0.0340, 1.50, 0.0553, 0.54], //Projecting
    //Circular Pipe, Beveled Ring Entrance
    [1.0, 0.0018, 2.50, 0.0300, 0.74], //Beveled ring, 45 deg bevels
    [1.0, 0.0018, 2.50, 0.0243, 0.83], //Beveled ring, 33.7 deg bevels
    //Rectangular Box with Flared Wingwalls
    [1.0, 0.026, 1.0, 0.0347, 0.81], //30-75 deg. wingwall flares
    [1.0, 0.061, 0.75, 0.0400, 0.80], //90 or 15 deg. wingwall flares
    [1.0, 0.061, 0.75, 0.0423, 0.82], //0 deg. wingwall flares (striaght sides)
    //Rectanglar Box with Flared Wingwalls & Top Edge Bevel
    [2.0, 0.510, 0.667, 0.0309, 0.80], //45 deg. flare; 0.43D top edge bevel
    [2.0, 0.486, 0.667, 0.0249, 0.83], //18-33.7 deg flare; 0.083D top edge bevel
    //Rectangular Box; 90-deg Headwall; Chamfered or Beveled Inlet Edges
    [2.0, 0.515, 0.667, 0.0375, 0.79], //chamfered 3/4-in
    [2.0, 0.495, 0.667, 0.0314, 0.82], //beveled 1/2-in/ft at 45 deg (1:1)
    [2.0, 0.486, 0.667, 0.0252, 0.865], //beveled 1-in/ft at 33.7 deg (1:1.5)
    //Rectangular Box; Skewed Headwall; Chamfered or Beveled Inlet Edges
    [2.0, 0.545, 0.667, 0.04505, 0.73], //3/4" chamfered edge, 45 deg skewed headwall
    [2.0, 0.533, 0.667, 0.0425, 0.705], //3/4" chamfered edge, 30 deg skewed headwall
    [2.0, 0.522, 0.667, 0.0402, 0.68], //3/4" chamfered edge, 15 deg skewed headwall
    [2.0, 0.498, 0.667, 0.0327, 0.75], //45 deg beveled edge, 10-45 deg skewed headwall
    //Rectangular box, Non-offset Flared Wingwalls; 3/4" Chamfer at Top of Inlet
    [2.0, 0.497, 0.667, 0.0339, 0.803], //45 deg (1:1) wingwall flare
    [2.0, 0.493, 0.667, 0.0361, 0.806], //18.4 deg (3:1) wingwall flare
    [2.0, 0.495, 0.667, 0.0386, 0.71], //18.4 deg (3:1) wingwall flare, 30 deg inlet skew
    //Rectangular box, Offset Flared Wingwalls, Beveled Edge at Inlet Top
    [2.0, 0.497, 0.667, 0.0302, 0.835], //45 deg (1:1) flare, 0.042D top edge bevel
    [2.0, 0.495, 0.667, 0.0252, 0.881], //33.7 deg (1.5:1) flare, 0.083D top edge bevel
    [2.0, 0.493, 0.667, 0.0227, 0.887], //18.4 deg (3:1) flare, 0.083D top edge bevel
    // Corrugated Metal Box
    [1.0, 0.0083, 2.00, 0.0379, 0.69], //90 deg headwall
    [1.0, 0.0145, 1.75, 0.0419, 0.64], //Thick wall projecting
    [1.0, 0.0340, 1.50, 0.0496, 0.57], //Thin wall projecting
    // Horizontal Ellipse Concrete
    [1.0, 0.0100, 2.00, 0.0398, 0.67], //Square edge w/headwall
    [1.0, 0.0018, 2.50, 0.0292, 0.74], //Grooved end w/headwall
    [1.0, 0.0045, 2.00, 0.0317, 0.69], //Grooved end projecting
    // Vertical Ellipse Concrete
    [1.0, 0.0100, 2.00, 0.0398, 0.67], //Square edge w/headwall
    [1.0, 0.0018, 2.50, 0.0292, 0.74], //Grooved end w/headwall
    [1.0, 0.0095, 2.00, 0.0317, 0.69], //Grooved end projecting
    // Pipe Arch, 18" Corner Radius, Corrugated Metal
    [1.0, 0.0083, 2.00, 0.0379, 0.69], //90 deg headwall
    [1.0, 0.0300, 1.00, 0.0463, 0.75], //Mitered to slope
    [1.0, 0.0340, 1.50, 0.0496, 0.57], //Projecting
    // Pipe Arch, 18" Corner Radius, Corrugated Metal
    [1.0, 0.0300, 1.50, 0.0496, 0.57], //Projecting
    [1.0, 0.0088, 2.00, 0.0368, 0.68], //No bevels
    [1.0, 0.0030, 2.00, 0.0269, 0.77], //33.7 deg bevels
    // Pipe Arch, 31" Corner Radius, Corrugated Metal
    [1.0, 0.0300, 1.50, 0.0496, 0.57], //Projecting
    [1.0, 0.0088, 2.00, 0.0368, 0.68], //No bevels
    [1.0, 0.0030, 2.00, 0.0269, 0.77], //33.7 deg. bevels
    // Arch, Corrugated Metal
    [1.0, 0.0083, 2.00, 0.0379, 0.69], //90 deg headwall
    [1.0, 0.0300, 1.00, 0.0473, 0.75], //Mitered to slope
    [1.0, 0.0340, 1.50, 0.0496, 0.57], //Thin wall projecting
    // Circular Culvert
    [2.0, 0.534, 0.555, 0.0196, 0.90], //Smooth tapered inlet throat
    [2.0, 0.519, 0.640, 0.0210, 0.90], //Rough tapered inlet throat
    // Elliptical Inlet Face
    [2.0, 0.536, 0.622, 0.0368, 0.83], //Tapered inlet, beveled edges
    [2.0, 0.5035, 0.719, 0.0478, 0.80], //Tapered inlet, square edges
    [2.0, 0.547, 0.800, 0.0598, 0.75], //Tapered inlet, thin edge projecting
    // Rectangular
    [2.0, 0.475, 0.667, 0.0179, 0.97], //Tapered inlet throat
    // Rectangular Concrete
    [2.0, 0.560, 0.667, 0.0446, 0.85], //Side tapered, less favorable edges
    [2.0, 0.560, 0.667, 0.0378, 0.87], //Side tapered, more favorable edges
    // Rectangular Concrete
    [2.0, 0.500, 0.667, 0.0446, 0.65], //Slope tapered, less favorable edges
    [2.0, 0.500, 0.667, 0.0378, 0.71], //Slope tapered, more favorable edges
];

//-----------------------------------------------------------------------------
//  Culvert data structure
//-----------------------------------------------------------------------------
#[derive(Clone, Copy, Default)]
struct TCulvert {
    yFull: f64, // full depth of culvert (ft)
    scf: f64,   // slope correction factor
    dQdH: f64,  // Derivative of flow w.r.t. head
    qc: f64,    // Unsubmerged critical flow
    kk: f64,
    mm: f64, // Coeffs. for unsubmerged flow
    ad: f64,
    hPlus: f64,    // Intermediate terms
    xsect: TXsect, // Culvert cross section (C: pointer to Link[j].xsect;
                   // edits made through it go back to the link at the end)
}

/// The file-scope variables of culvert.c (it has none).
#[derive(Default)]
pub struct CulvertState {
}

//=============================================================================

/// Called by: dwflow.
pub fn culvert_getInflow(prj: &mut Project, j: i32, q0: f64, h: f64) -> f64
//
//  Input:   j  = link index
//           q0 = unmodified flow rate (cfs)
//           h  = upstream head (ft)
//  Output:  returns modified flow rate through culvert (cfs)
//  Purpose: uses FHWA HEC-5 equations to find flow through inlet
//           controlled culverts
//
{
    let ju = j as usize;
    let code: i32; //culvert type code number
    let k: i32; //conduit index
    let condition: i32; //flow condition
    let y: f64; //current depth (ft)
    let y1: f64; //unsubmerged depth limit (ft)
    let y2: f64; //submerged depth limit (ft)
    let q: f64; //inlet-controlled flow (cfs)
    let mut culvert = TCulvert::default(); //intermediate results

    // --- check that we have a culvert conduit
    if prj.Link[ju].type_ != CONDUIT {
        return q0;
    }
    culvert.xsect = prj.Link[ju].xsect;
    code = culvert.xsect.culvertCode;
    if code <= 0 || code > MAX_CULVERT_CODE {
        return q0;
    }
    let cu = code as usize;

    // --- compute often-used variables
    k = prj.Link[ju].subIndex;
    culvert.yFull = culvert.xsect.yFull;
    culvert.ad = culvert.xsect.aFull * culvert.yFull.sqrt();

    // --- slope correction factor (-7 for mitered inlets, 0.5 for others)
    match code {
        5 | 37 | 46 => culvert.scf = -7.0 * prj.Conduit[k as usize].slope,
        _ => culvert.scf = 0.5 * prj.Conduit[k as usize].slope,
    }

    // --- find head relative to culvert's upstream invert
    //     (can be greater than yFull when inlet is submerged)
    y = h - (prj.Node[prj.Link[ju].node1 as usize].invertElev + prj.Link[ju].offset1);

    // --- check for submerged flow (based on FHWA criteria of Q/AD > 4)
    y2 = culvert.yFull * (16.0 * Params[cu][C] + Params[cu][Y] - culvert.scf);
    {
        let p: &Project = prj;
        if y >= y2 {
            q = getSubmergedFlow(code, y, &mut culvert);
            condition = 2;
        } else {
            // --- check for unsubmerged flow (based on arbitrary limit of 0.95 full)
            y1 = 0.95 * culvert.yFull;
            if y <= y1 {
                q = getUnsubmergedFlow(p, code, y, &mut culvert);
                condition = 1;
            }
            // --- flow is in transition zone
            else {
                q = getTransitionFlow(p, code, y, y1, y2, &mut culvert);
                condition = 0;
            }
        }
    }

    // (condition is read only by the commented-out debug report below)
    let _ = condition;

    // C worked on Link[j].xsect through a pointer: keep any FILLED_CIRCULAR
    // round-trip edit the getters made
    prj.Link[ju].xsect = culvert.xsect;

    // --- check if inlet controls and replace conduit's value of dq/dh
    if q < q0 {
        // --- for debugging only
        //if ( RptFlags.controls ) report_CulvertControl(j, q0, q, condition,
        //                                               y / culvert.yFull);

        prj.Link[ju].inletControl = TRUE as i8;
        prj.Link[ju].dqdh = culvert.dQdH;
        q
    } else {
        q0
    }
}

//=============================================================================

fn getUnsubmergedFlow(prj: &Project, code: i32, h: f64, culvert: &mut TCulvert) -> f64
//
//  Input:   code  = culvert type code number
//           h     = inlet water depth above culvert invert
//           culvert = pointer to a culvert data structure
//  Output:  returns flow rate;
//           computes value of variable Dqdh
//  Purpose: computes flow rate and its derivative for unsubmerged
//           culvert inlet.
//
{
    let cu = code as usize;
    let arg: f64;
    let q: f64;

    // --- assign shared variables
    culvert.kk = Params[cu][K];
    culvert.mm = Params[cu][M];
    arg = h / culvert.yFull / culvert.kk;

    // --- evaluate correct equation form
    if Params[cu][FORM] == 1.0 {
        q = getForm1Flow(prj, h, culvert);
    } else {
        q = culvert.ad * arg.powf(1.0 / culvert.mm);
    }
    culvert.dQdH = q / h / culvert.mm;
    q
}

//=============================================================================

fn getSubmergedFlow(code: i32, h: f64, culvert: &mut TCulvert) -> f64
//
//  Input:   code  = culvert type code number
//           h     = inlet head (ft)
//           culvert = pointer to a culvert data structure
//  Output:  returns flow rate;
//           computes value of Dqdh
//  Purpose: computes flow rate and its derivative for submerged
//           culvert inlet.
//
{
    let cu = code as usize;
    let cc: f64 = Params[cu][C];
    let yy: f64 = Params[cu][Y];
    let arg: f64 = (h / culvert.yFull - yy + culvert.scf) / cc;
    let q: f64;

    if arg <= 0.0 {
        culvert.dQdH = 0.0;
        return BIG;
    }
    q = arg.sqrt() * culvert.ad;
    culvert.dQdH = 0.5 * q / arg / culvert.yFull / cc;
    q
}

//=============================================================================

fn getTransitionFlow(prj: &Project, code: i32, h: f64, h1: f64, h2: f64, culvert: &mut TCulvert) -> f64
//
//  Input:   code    = culvert type code number
//           h       = inlet water depth above culvert invert (ft)
//           h1      = head limit for unsubmerged condition (ft)
//           h2      = head limit for submerged condition (ft)
//           culvert = pointer to a culvert data structure
//  Output:  returns flow rate )cfs);
//           computes value of Dqdh (cfs/ft)
//  Purpose: computes flow rate and its derivative for inlet-controlled flow
//           when inlet water depth lies in the transition range between
//           submerged and unsubmerged conditions.
//
{
    let q1: f64 = getUnsubmergedFlow(prj, code, h1, culvert);
    let q2: f64 = getSubmergedFlow(code, h2, culvert);
    let q: f64 = q1 + (q2 - q1) * (h - h1) / (h2 - h1);
    culvert.dQdH = (q2 - q1) / (h2 - h1);
    q
}

//=============================================================================

fn getForm1Flow(prj: &Project, h: f64, culvert: &mut TCulvert) -> f64
//
//  Input:   h       = inlet water depth above culvert invert
//           culvert = pointer to a culvert data structure
//  Output:  returns inlet controlled flow rate
//  Purpose: computes inlet-controlled flow rate for unsubmerged culvert
//           using FHWA Equation Form1.
//
//  See pages 195-196 of FHWA HEC-5 (2001) for details.
//
{
    // --- save re-used terms in culvert structure
    culvert.hPlus = h / culvert.yFull + culvert.scf;

    // --- use Ridder's method to solve Equation Form 1 for critical depth
    //     between a range of 0.01h and h
    findroot_Ridder(0.01 * h, h, 0.001, &mut |yc: f64| form1Eqn(prj, yc, culvert));

    // --- return the flow value used in evaluating Equation Form 1
    culvert.qc
}

//=============================================================================

fn form1Eqn(prj: &Project, yc: f64, p: &mut TCulvert) -> f64
//
//  Input:   yc = critical depth
//           p  = pointer to a TCulvert object
//  Output:  returns residual error
//  Purpose: evaluates the error in satisfying FHWA culvert Equation Form1:
//
//  h/yFull + 0.5*s = yc/yFull + yh/2/yFull + K[ac/aFull*sqrt(g*yh/yFull)]^M
//
//  for a given value of critical depth yc where:
//    h = inlet depth above culvert invert
//    s = culvert slope
//    yFull = full depth of culvert
//    yh = hydraulic depth at critical depth
//    ac = flow area at critical depth
//    g = accel. of gravity
//    K and M = coefficients
//
{
    let ac: f64;
    let wc: f64;
    let yh: f64;
    let culvert: &mut TCulvert = p;

    ac = xsect_getAofY_mut(prj, &mut culvert.xsect, yc);
    wc = xsect_getWofY(prj, &culvert.xsect, yc);
    yh = ac / wc;

    culvert.qc = ac * (GRAVITY * yh).sqrt();
    culvert.hPlus - yc / culvert.yFull - yh / 2.0 / culvert.yFull
        - culvert.kk * (culvert.qc / culvert.ad).powf(culvert.mm)
}

//=============================================================================
/*
void report_CulvertControl(int j, double q0, double q, int condition, double yRatio)
//
//  Used for debugging only
//
{
    static   char* conditionTxt[] = {"transition", "unsubmerged", "submerged"};
    char     theDate[12];
    char     theTime[9];
    DateTime aDate = getDateTime(NewRoutingTime);
    datetime_dateToStr(aDate, theDate);
    datetime_timeToStr(aDate, theTime);
    fprintf(Frpt.file,
            "\n  %11s: %8s Culvert %s flow reduced from %.3f to %.3f cfs for %s flow (%.2f).",
            theDate, theTime, Link[j].ID, q0, q, conditionTxt[condition], yRatio);
}
*/
