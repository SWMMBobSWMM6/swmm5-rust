//! xsect.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   xsect.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     10/17/22   (Build 5.2.2)
//   Author:   L. Rossman
//             M. Tryby (EPA)
//
//   Cross section geometry functions.
//
//   The primary functions are:
//      getAofY   -- returns area given depth
//      getWofY   -- returns top width given depth
//      getRofY   -- returns hyd. radius given depth
//      getYofA   -- returns flow depth given area
//      getRofA   -- returns hyd. radius given area
//      getSofA   -- returns section factor given area
//      getAofS   -- returns area given section factor
//      getdSdA   -- returns derivative of section factor w.r.t. area
//   where
//      Y = flow depth
//      A = flow area
//      R = hyd. radius
//      S = section factor = A*R^(2/3)
//
//   Update History
//   ==============
//   Build 5.1.012:
//   - Height at max. width for Modified Baskethandle shape corrected.
//   Build 5.1.013:
//   - Width at full height set to 0 for closed rectangular shape.
//   Build 5.2.0:
//   - Support added for Street cross sections.
//   Build 5.2.2:
//   - Feasibility check added to Mod. Baskethandle & Rect.-Round shapes.
//-----------------------------------------------------------------------------
//
// Translation notes:
// * The getters take `prj` because IRREGULAR, STREET_XSECT and CUSTOM shapes
//   read prj.Transect, prj.Street, prj.Shape and prj.Curve.
// * C's TXsectStar (a section factor or critical flow plus a pointer to the
//   cross section, handed to the root finders as void*) becomes the values
//   captured by the closures passed to findroot_Newton / findroot_Ridder.
// * The FILLED_CIRCULAR helpers in C temporarily add the filled bottom to
//   the caller's xsect->yFull / aFull and subtract it back afterwards, and
//   (yFull+yBot)-yBot can differ from yFull in the last bit; that changed
//   value stays in the caller's cross section. The getters that can reach
//   those helpers therefore exist twice: `xsect_getXofY_mut(prj, &mut
//   TXsect, ..)`, which writes the cross section exactly as C does, and the
//   stub-signature `xsect_getXofY(prj, &TXsect, ..)`, which works on a copy.
// * lookup() and tabular_getdSdA() index their table with (int)(x/delta);
//   C reads out of bounds for x <= -delta or a NaN x; see tbl().

use crate::prelude::*;

/// The file-scope variables of xsect.c (none).
#[derive(Default)]
pub struct XsectState {}

const RECT_ALFMAX: f64 = 0.97;
const RECT_TRIANG_ALFMAX: f64 = 0.98;
const RECT_ROUND_ALFMAX: f64 = 0.98;


//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------
// Ratio of area at max. flow to full area
// (= 1.0 for open shapes, < 1.0 for closed shapes)
const Amax: [f64; 26] = [
    1.0,    //  DUMMY
    0.9756, //  CIRCULAR
    0.9756, //  FILLED_CIRCULAR
    0.97,   //  RECT_CLOSED
    1.0,    //  RECT_OPEN
    1.0,    //  TRAPEZOIDAL
    1.0,    //  TRIANGULAR
    1.0,    //  PARABOLIC
    1.0,    //  POWERFUNC
    0.98,   //  RECT_TRIANG
    0.98,   //  RECT_ROUND
    0.96,   //  MOD_BASKET
    0.96,   //  HORIZ_ELLIPSE
    0.96,   //  VERT_ELLIPSE
    0.92,   //  ARCH
    0.96,   //  EGGSHAPED
    0.96,   //  HORSESHOE
    0.96,   //  GOTHIC
    0.98,   //  CATENARY
    0.98,   //  SEMIELLIPTICAL
    0.96,   //  BASKETHANDLE
    0.96,   //  SEMICIRCULAR
    1.0,    //  IRREGULAR
    0.96,   //  CUSTOM
    0.9756, //  FORCE_MAIN
    1.0,    //  STREET_XSECT
];

//=============================================================================

pub fn xsect_isOpen(type_: i32) -> i32 {
    //
    //  Input:   type = type of xsection shape
    //  Output:  returns 1 if xsection is open, 0 if not
    //  Purpose: determines if a xsection type is open or closed.
    //
    if Amax[type_ as usize] >= 1.0 {
        1
    } else {
        0
    }
}

//=============================================================================

/// C: `int xsect_setParams(TXsect *xsect, int type, double p[], double ucf)`.
/// C may overwrite p[2] (RECT_ROUND, MOD_BASKET and the standard-size
/// shapes); no caller reads p afterwards, so p is copied and the copy edited.
pub fn xsect_setParams(prj: &Project, xsect: &mut TXsect, type_: i32, p: &[f64], ucf: f64) -> i32 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           type = xsection shape type
    //           p[] = vector of xsection parameters
    //           ucf = units correction factor
    //  Output:  returns TRUE if successful, FALSE if not
    //  Purpose: assigns parameters to a cross section's data structure.
    //
    let mut p: Vec<f64> = p.to_vec();
    let index: i32;
    let aMax: f64;
    let theta: f64;

    if type_ != DUMMY && p[0] <= 0.0 {
        return FALSE;
    }
    xsect.type_ = type_;
    match xsect.type_ {
        DUMMY => {
            xsect.yFull = TINY;
            xsect.wMax = TINY;
            xsect.aFull = TINY;
            xsect.rFull = TINY;
            xsect.sFull = TINY;
            xsect.sMax = TINY;
        }

        CIRCULAR => {
            xsect.yFull = p[0] / ucf;
            xsect.wMax = xsect.yFull;
            xsect.aFull = PI / 4.0 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.2500 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.08 * xsect.sFull;
            xsect.ywMax = 0.5 * xsect.yFull;
        }

        FORCE_MAIN => {
            xsect.yFull = p[0] / ucf;
            xsect.wMax = xsect.yFull;
            xsect.aFull = PI / 4.0 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.2500 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(0.63);
            xsect.sMax = 1.06949 * xsect.sFull;
            xsect.ywMax = 0.5 * xsect.yFull;

            // --- save C-factor or roughness in rBot position
            xsect.rBot = p[1];
        }

        FILLED_CIRCULAR => {
            if p[1] >= p[0] {
                return FALSE;
            }

            // --- initially compute full values for unfilled pipe
            xsect.yFull = p[0] / ucf;
            xsect.wMax = xsect.yFull;
            xsect.aFull = PI / 4.0 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.2500 * xsect.yFull;

            // --- find:
            //     yBot = depth of filled bottom
            //     aBot = area of filled bottom
            //     sBot = width of filled bottom
            //     rBot = wetted perimeter of filled bottom
            xsect.yBot = p[1] / ucf;
            xsect.aBot = circ_getAofY(xsect, xsect.yBot);
            xsect.sBot = xsect_getWofY(prj, xsect, xsect.yBot);
            xsect.rBot = xsect.aBot / (xsect.rFull * lookup(xsect.yBot / xsect.yFull, &R_Circ, N_R_Circ));

            // --- revise full values for filled bottom
            xsect.aFull -= xsect.aBot;
            xsect.rFull = xsect.aFull / (PI * xsect.yFull - xsect.rBot + xsect.sBot);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.08 * xsect.sFull;
            xsect.yFull -= xsect.yBot;
            xsect.ywMax = 0.5 * xsect.yFull;
        }

        EGGSHAPED => {
            xsect.yFull = p[0] / ucf;
            xsect.aFull = 0.5105 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.1931 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.065 * xsect.sFull;
            xsect.wMax = 2.0 / 3.0 * xsect.yFull;
            xsect.ywMax = 0.64 * xsect.yFull;
        }

        HORSESHOE => {
            xsect.yFull = p[0] / ucf;
            xsect.aFull = 0.8293 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.2538 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.077 * xsect.sFull;
            xsect.wMax = 1.0 * xsect.yFull;
            xsect.ywMax = 0.5 * xsect.yFull;
        }

        GOTHIC => {
            xsect.yFull = p[0] / ucf;
            xsect.aFull = 0.6554 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.2269 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.065 * xsect.sFull;
            xsect.wMax = 0.84 * xsect.yFull;
            xsect.ywMax = 0.45 * xsect.yFull;
        }

        CATENARY => {
            xsect.yFull = p[0] / ucf;
            xsect.aFull = 0.70277 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.23172 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.05 * xsect.sFull;
            xsect.wMax = 0.9 * xsect.yFull;
            xsect.ywMax = 0.25 * xsect.yFull;
        }

        SEMIELLIPTICAL => {
            xsect.yFull = p[0] / ucf;
            xsect.aFull = 0.785 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.242 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.045 * xsect.sFull;
            xsect.wMax = 1.0 * xsect.yFull;
            xsect.ywMax = 0.15 * xsect.yFull;
        }

        BASKETHANDLE => {
            xsect.yFull = p[0] / ucf;
            xsect.aFull = 0.7862 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.2464 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.06078 * xsect.sFull;
            xsect.wMax = 0.944 * xsect.yFull;
            xsect.ywMax = 0.2 * xsect.yFull;
        }

        SEMICIRCULAR => {
            xsect.yFull = p[0] / ucf;
            xsect.aFull = 1.2697 * xsect.yFull * xsect.yFull;
            xsect.rFull = 0.2946 * xsect.yFull;
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = 1.06637 * xsect.sFull;
            xsect.wMax = 1.64 * xsect.yFull;
            xsect.ywMax = 0.15 * xsect.yFull;
        }

        RECT_CLOSED => {
            if p[1] <= 0.0 {
                return FALSE;
            }
            xsect.yFull = p[0] / ucf;
            xsect.wMax = p[1] / ucf;
            xsect.aFull = xsect.yFull * xsect.wMax;
            xsect.rFull = xsect.aFull / (2.0 * (xsect.yFull + xsect.wMax));
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            aMax = RECT_ALFMAX * xsect.aFull;
            xsect.sMax = aMax * rect_closed_getRofA(xsect, aMax).powf(2.0 / 3.0);
            xsect.ywMax = xsect.yFull;
        }

        RECT_OPEN => {
            if p[1] <= 0.0 {
                return FALSE;
            }
            xsect.yFull = p[0] / ucf;
            xsect.wMax = p[1] / ucf;
            if p[2] < 0.0 || p[2] > 2.0 {
                return FALSE; //# sides to ignore
            }
            xsect.sBot = p[2];
            xsect.aFull = xsect.yFull * xsect.wMax;
            xsect.rFull = xsect.aFull / ((2.0 - xsect.sBot) * xsect.yFull + xsect.wMax);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = xsect.sFull;
            xsect.ywMax = xsect.yFull;
        }

        RECT_TRIANG => {
            if p[1] <= 0.0 || p[2] <= 0.0 {
                return FALSE;
            }
            xsect.yFull = p[0] / ucf;
            xsect.wMax = p[1] / ucf;
            xsect.yBot = p[2] / ucf;
            xsect.ywMax = xsect.yFull;

            // --- area of bottom triangle
            xsect.aBot = xsect.yBot * xsect.wMax / 2.0;

            // --- slope of bottom side wall
            xsect.sBot = xsect.wMax / xsect.yBot / 2.0;

            // --- length of side wall per unit of depth
            xsect.rBot = (1.0 + xsect.sBot * xsect.sBot).sqrt();

            xsect.aFull = xsect.wMax * (xsect.yFull - xsect.yBot / 2.0);
            xsect.rFull = xsect.aFull / (2.0 * xsect.yBot * xsect.rBot + 2.0 * (xsect.yFull - xsect.yBot) + xsect.wMax);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            aMax = RECT_TRIANG_ALFMAX * xsect.aFull;
            xsect.sMax = aMax * rect_triang_getRofA(xsect, aMax).powf(2.0 / 3.0);
        }

        RECT_ROUND => {
            if p[1] <= 0.0 {
                return FALSE;
            }
            if p[2] < p[1] / 2.0 {
                p[2] = p[1] / 2.0;
            }
            xsect.yFull = p[0] / ucf;
            xsect.wMax = p[1] / ucf;
            xsect.rBot = p[2] / ucf;

            // --- angle of circular arc
            theta = 2.0 * (xsect.wMax / 2.0 / xsect.rBot).asin();

            // --- area of circular bottom
            xsect.aBot = xsect.rBot * xsect.rBot / 2.0 * (theta - theta.sin());

            // --- section factor for circular bottom
            xsect.sBot = PI * xsect.rBot * xsect.rBot * (xsect.rBot / 2.0).powf(2.0 / 3.0);

            // --- depth of circular bottom
            xsect.yBot = xsect.rBot * (1.0 - (theta / 2.0).cos());
            if xsect.yBot > xsect.yFull {
                return FALSE;
            }
            xsect.ywMax = xsect.yFull;

            xsect.aFull = xsect.wMax * (xsect.yFull - xsect.yBot) + xsect.aBot;
            xsect.rFull = xsect.aFull / (xsect.rBot * theta + 2.0 * (xsect.yFull - xsect.yBot) + xsect.wMax);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            aMax = RECT_ROUND_ALFMAX * xsect.aFull;
            xsect.sMax = aMax * rect_round_getRofA(xsect, aMax).powf(2.0 / 3.0);
        }

        MOD_BASKET => {
            if p[1] <= 0.0 {
                return FALSE;
            }
            if p[2] < p[1] / 2.0 {
                p[2] = p[1] / 2.0;
            }
            xsect.yFull = p[0] / ucf;
            xsect.wMax = p[1] / ucf;

            // --- radius of circular arc
            xsect.rBot = p[2] / ucf;

            // --- angle of circular arc
            theta = 2.0 * (xsect.wMax / 2.0 / xsect.rBot).asin();
            xsect.sBot = theta;

            // --- height of circular arc
            xsect.yBot = xsect.rBot * (1.0 - (theta / 2.0).cos());
            if xsect.yBot > xsect.yFull {
                return FALSE;
            }
            xsect.ywMax = xsect.yFull - xsect.yBot;

            // --- area of circular arc
            xsect.aBot = xsect.rBot * xsect.rBot / 2.0 * (theta - theta.sin());

            // --- full area
            xsect.aFull = (xsect.yFull - xsect.yBot) * xsect.wMax + xsect.aBot;

            // --- full hydraulic radius & section factor
            xsect.rFull = xsect.aFull / (xsect.rBot * theta + 2.0 * (xsect.yFull - xsect.yBot) + xsect.wMax);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);

            // --- area corresponding to max. section factor
            xsect.sMax = xsect_getSofA_mut(prj, xsect, Amax[MOD_BASKET as usize] * xsect.aFull);
        }

        TRAPEZOIDAL => {
            if p[1] < 0.0 || p[2] < 0.0 || p[3] < 0.0 {
                return FALSE;
            }
            xsect.yFull = p[0] / ucf;
            xsect.ywMax = xsect.yFull;

            // --- bottom width
            xsect.yBot = p[1] / ucf;

            // --- avg. slope of side walls
            xsect.sBot = (p[2] + p[3]) / 2.0;
            if xsect.yBot == 0.0 && xsect.sBot == 0.0 {
                return FALSE;
            }

            // --- length of side walls per unit of depth
            xsect.rBot = (1.0 + p[2] * p[2]).sqrt() + (1.0 + p[3] * p[3]).sqrt();

            // --- top width
            xsect.wMax = xsect.yBot + xsect.yFull * (p[2] + p[3]);

            xsect.aFull = (xsect.yBot + xsect.sBot * xsect.yFull) * xsect.yFull;
            xsect.rFull = xsect.aFull / (xsect.yBot + xsect.yFull * xsect.rBot);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = xsect.sFull;
        }

        TRIANGULAR => {
            if p[1] <= 0.0 {
                return FALSE;
            }
            xsect.yFull = p[0] / ucf;
            xsect.wMax = p[1] / ucf;
            xsect.ywMax = xsect.yFull;

            // --- slope of side walls
            xsect.sBot = xsect.wMax / xsect.yFull / 2.0;

            // --- length of side wall per unit of depth
            xsect.rBot = (1.0 + xsect.sBot * xsect.sBot).sqrt();

            xsect.aFull = xsect.yFull * xsect.yFull * xsect.sBot;
            xsect.rFull = xsect.aFull / (2.0 * xsect.yFull * xsect.rBot);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = xsect.sFull;
        }

        PARABOLIC => {
            if p[1] <= 0.0 {
                return FALSE;
            }
            xsect.yFull = p[0] / ucf;
            xsect.wMax = p[1] / ucf;
            xsect.ywMax = xsect.yFull;

            // --- rBot :: 1/c^.5, where y = c*x^2 is eqn. of parabolic shape
            xsect.rBot = xsect.wMax / 2.0 / xsect.yFull.sqrt();

            xsect.aFull = (2.0 / 3.0) * xsect.yFull * xsect.wMax;
            xsect.rFull = xsect_getRofY_mut(prj, xsect, xsect.yFull);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = xsect.sFull;
        }

        POWERFUNC => {
            if p[1] <= 0.0 || p[2] <= 0.0 {
                return FALSE;
            }
            xsect.yFull = p[0] / ucf;
            xsect.wMax = p[1] / ucf;
            xsect.ywMax = xsect.yFull;
            xsect.sBot = 1.0 / p[2];
            xsect.rBot = xsect.wMax / (xsect.sBot + 1.0) / xsect.yFull.powf(xsect.sBot);
            xsect.aFull = xsect.yFull * xsect.wMax / (xsect.sBot + 1.0);
            xsect.rFull = xsect_getRofY_mut(prj, xsect, xsect.yFull);
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = xsect.sFull;
        }

        HORIZ_ELLIPSE => {
            if p[1] == 0.0 {
                p[2] = p[0];
            }
            if p[2] > 0.0 {
                // std. ellipse pipe
                index = f64_to_i32(p[2].floor()).wrapping_sub(1); // size code
                if index < 0 || index >= NumCodesEllipse {
                    return FALSE;
                }
                xsect.yFull = MinorAxis_Ellipse[index as usize] / 12.0;
                xsect.wMax = MajorAxis_Ellipse[index as usize] / 12.0;
                xsect.aFull = Afull_Ellipse[index as usize];
                xsect.rFull = Rfull_Ellipse[index as usize];
            } else {
                // --- length of minor axis
                xsect.yFull = p[0] / ucf;

                // --- length of major axis
                if p[1] < 0.0 {
                    return FALSE;
                }
                xsect.wMax = p[1] / ucf;
                xsect.aFull = 1.2692 * xsect.yFull * xsect.yFull;
                xsect.rFull = 0.3061 * xsect.yFull;
            }
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = xsect.sFull;
            xsect.ywMax = 0.48 * xsect.yFull;
        }

        VERT_ELLIPSE => {
            if p[1] == 0.0 {
                p[2] = p[0];
            }
            if p[2] > 0.0 {
                // std. ellipse pipe
                index = f64_to_i32(p[2].floor()).wrapping_sub(1); // size code
                if index < 0 || index >= NumCodesEllipse {
                    return FALSE;
                }
                xsect.yFull = MajorAxis_Ellipse[index as usize] / 12.0;
                xsect.wMax = MinorAxis_Ellipse[index as usize] / 12.0;
                xsect.aFull = Afull_Ellipse[index as usize];
                xsect.rFull = Rfull_Ellipse[index as usize];
            } else {
                // --- length of major axis
                if p[1] < 0.0 {
                    return FALSE;
                }

                // --- length of minor axis
                xsect.yFull = p[0] / ucf;
                xsect.wMax = p[1] / ucf;
                xsect.aFull = 1.2692 * xsect.wMax * xsect.wMax;
                xsect.rFull = 0.3061 * xsect.wMax;
            }
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = xsect.sFull;
            xsect.ywMax = 0.48 * xsect.yFull;
        }

        ARCH => {
            if p[1] == 0.0 {
                p[2] = p[0];
            }
            if p[2] > 0.0 {
                // std. arch pipe
                index = f64_to_i32(p[2].floor()).wrapping_sub(1); // size code
                if index < 0 || index >= NumCodesArch {
                    return FALSE;
                }
                xsect.yFull = Yfull_Arch[index as usize] / 12.0; // Yfull units are inches
                xsect.wMax = Wmax_Arch[index as usize] / 12.0; // Wmax units are inches
                xsect.aFull = Afull_Arch[index as usize];
                xsect.rFull = Rfull_Arch[index as usize];
            } else {
                // non-std. arch pipe
                if p[1] < 0.0 {
                    return FALSE;
                }
                xsect.yFull = p[0] / ucf;
                xsect.wMax = p[1] / ucf;
                xsect.aFull = 0.7879 * xsect.yFull * xsect.wMax;
                xsect.rFull = 0.2991 * xsect.yFull;
            }
            xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
            xsect.sMax = xsect.sFull;
            xsect.ywMax = 0.28 * xsect.yFull;
        }
        _ => {}
    }
    TRUE
}

//=============================================================================

pub fn xsect_setIrregXsectParams(prj: &Project, xsect: &mut TXsect) {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //  Output:  none
    //  Purpose: assigns transect parameters to an irregular shaped cross section.
    //
    let index = xsect.transect;
    getTransectParams(xsect, &prj.Transect[index as usize]);
}

//=============================================================================

pub fn xsect_setStreetXsectParams(prj: &Project, xsect: &mut TXsect) {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //  Output:  none
    //  Purpose: assigns transect parameters to a street cross section.
    //
    let index = xsect.transect;
    getTransectParams(xsect, &prj.Street[index as usize].transect);
}

//=============================================================================

pub fn xsect_setCustomXsectParams(prj: &Project, xsect: &mut TXsect) {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //  Output:  none
    //  Purpose: assigns parameters to a custom-shaped cross section.
    //
    let index: i32 = prj.Curve[xsect.transect as usize].refersTo;
    let yFull: f64 = xsect.yFull;
    let mut i: i32;
    let mut iMax: i32;
    let mut wMax: f64;
    let shape = &prj.Shape[index as usize];
    let wTbl: &[f64] = &shape.widthTbl;

    xsect.wMax = shape.wMax * yFull;
    xsect.aFull = shape.aFull * yFull * yFull;
    xsect.rFull = shape.rFull * yFull;
    xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
    xsect.sMax = shape.sMax * yFull * yFull * yFull.powf(2.0 / 3.0);
    xsect.aBot = shape.aMax * yFull * yFull;

    // Search shape's width table up to point where width decreases
    iMax = 0;
    wMax = wTbl[0];
    i = 1;
    while i < N_SHAPE_TBL {
        if wTbl[i as usize] < wMax {
            break;
        }
        wMax = wTbl[i as usize];
        iMax = i;
        i += 1;
    }

    // Determine height at lowest widest point
    xsect.ywMax = yFull * iMax as f64 / (N_SHAPE_TBL - 1) as f64;
}

//=============================================================================

pub fn xsect_getAmax(prj: &Project, xsect: &TXsect) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //  Output:  returns area (ft2)
    //  Purpose: finds xsection area at maximum flow depth.
    //
    if xsect.type_ == IRREGULAR {
        xsect.aBot
    } else if xsect.type_ == CUSTOM {
        xsect.aBot
    } else {
        Amax[xsect.type_ as usize] * xsect.aFull
    }
}

//=============================================================================

pub fn xsect_getSofA_mut(prj: &Project, xsect: &mut TXsect, area: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           a = area (ft2)
    //  Output:  returns section factor (ft^(8/3))
    //  Purpose: computes xsection's section factor at a given area.
    //
    let a = area;
    let alpha: f64 = a / xsect.aFull;
    let r: f64;
    match xsect.type_ {
        FORCE_MAIN | CIRCULAR => circ_getSofA(xsect, a),

        EGGSHAPED => xsect.sFull * lookup(alpha, &S_Egg, N_S_Egg),

        HORSESHOE => xsect.sFull * lookup(alpha, &S_Horseshoe, N_S_Horseshoe),

        GOTHIC => xsect.sFull * lookup(alpha, &S_Gothic, N_S_Gothic),

        CATENARY => xsect.sFull * lookup(alpha, &S_Catenary, N_S_Catenary),

        SEMIELLIPTICAL => xsect.sFull * lookup(alpha, &S_SemiEllip, N_S_SemiEllip),

        BASKETHANDLE => xsect.sFull * lookup(alpha, &S_BasketHandle, N_S_BasketHandle),

        SEMICIRCULAR => xsect.sFull * lookup(alpha, &S_SemiCirc, N_S_SemiCirc),

        RECT_CLOSED => rect_closed_getSofA(prj, xsect, a),

        RECT_OPEN => rect_open_getSofA(xsect, a),

        RECT_TRIANG => rect_triang_getSofA(xsect, a),

        RECT_ROUND => rect_round_getSofA(prj, xsect, a),

        _ => {
            if a == 0.0 {
                return 0.0;
            }
            r = xsect_getRofA_mut(prj, xsect, a);
            if r < TINY {
                return 0.0;
            }
            a * r.powf(2.0 / 3.0)
        }
    }
}

//=============================================================================

pub fn xsect_getYofA_mut(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           a = area (ft2)
    //  Output:  returns depth (ft)
    //  Purpose: computes xsection's depth at a given area.
    //
    let alpha: f64 = a / xsect.aFull;
    match xsect.type_ {
        FORCE_MAIN | CIRCULAR => circ_getYofA(xsect, a),

        FILLED_CIRCULAR => filled_circ_getYofA(xsect, a),

        EGGSHAPED => xsect.yFull * lookup(alpha, &Y_Egg, N_Y_Egg),

        HORSESHOE => xsect.yFull * lookup(alpha, &Y_Horseshoe, N_Y_Horseshoe),

        GOTHIC => xsect.yFull * lookup(alpha, &Y_Gothic, N_Y_Gothic),

        CATENARY => xsect.yFull * lookup(alpha, &Y_Catenary, N_Y_Catenary),

        SEMIELLIPTICAL => xsect.yFull * lookup(alpha, &Y_SemiEllip, N_Y_SemiEllip),

        BASKETHANDLE => xsect.yFull * lookup(alpha, &Y_BasketHandle, N_Y_BasketHandle),

        SEMICIRCULAR => xsect.yFull * lookup(alpha, &Y_SemiCirc, N_Y_SemiCirc),

        HORIZ_ELLIPSE => xsect.yFull * invLookup(alpha, &A_HorizEllipse, N_A_HorizEllipse),

        VERT_ELLIPSE => xsect.yFull * invLookup(alpha, &A_VertEllipse, N_A_VertEllipse),

        IRREGULAR => xsect.yFull * invLookup(alpha, &prj.Transect[xsect.transect as usize].areaTbl, N_TRANSECT_TBL),

        CUSTOM => xsect.yFull * invLookup(alpha, &prj.Shape[prj.Curve[xsect.transect as usize].refersTo as usize].areaTbl, N_SHAPE_TBL),

        STREET_XSECT => xsect.yFull * invLookup(alpha, &prj.Street[xsect.transect as usize].transect.areaTbl, prj.Street[xsect.transect as usize].transect.nTbl),

        ARCH => xsect.yFull * invLookup(alpha, &A_Arch, N_A_Arch),

        RECT_CLOSED => a / xsect.wMax,

        RECT_TRIANG => rect_triang_getYofA(xsect, a),

        RECT_ROUND => rect_round_getYofA(xsect, a),

        RECT_OPEN => a / xsect.wMax,

        MOD_BASKET => mod_basket_getYofA(xsect, a),

        TRAPEZOIDAL => trapez_getYofA(xsect, a),

        TRIANGULAR => triang_getYofA(xsect, a),

        PARABOLIC => parab_getYofA(xsect, a),

        POWERFUNC => powerfunc_getYofA(xsect, a),

        _ => 0.0,
    }
}

//=============================================================================

pub fn xsect_getAofY_mut(prj: &Project, xsect: &mut TXsect, y: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           y = depth (ft)
    //  Output:  returns area (ft2)
    //  Purpose: computes xsection's area at a given depth.
    //
    let yNorm: f64 = y / xsect.yFull;
    if y <= 0.0 {
        return 0.0;
    }
    match xsect.type_ {
        FORCE_MAIN | CIRCULAR => xsect.aFull * lookup(yNorm, &A_Circ, N_A_Circ),

        FILLED_CIRCULAR => filled_circ_getAofY(xsect, y),

        EGGSHAPED => xsect.aFull * lookup(yNorm, &A_Egg, N_A_Egg),

        HORSESHOE => xsect.aFull * lookup(yNorm, &A_Horseshoe, N_A_Horseshoe),

        GOTHIC => xsect.aFull * invLookup(yNorm, &Y_Gothic, N_Y_Gothic),

        CATENARY => xsect.aFull * invLookup(yNorm, &Y_Catenary, N_Y_Catenary),

        SEMIELLIPTICAL => xsect.aFull * invLookup(yNorm, &Y_SemiEllip, N_Y_SemiEllip),

        BASKETHANDLE => xsect.aFull * lookup(yNorm, &A_Baskethandle, N_A_Baskethandle),

        SEMICIRCULAR => xsect.aFull * invLookup(yNorm, &Y_SemiCirc, N_Y_SemiCirc),

        HORIZ_ELLIPSE => xsect.aFull * lookup(yNorm, &A_HorizEllipse, N_A_HorizEllipse),

        VERT_ELLIPSE => xsect.aFull * lookup(yNorm, &A_VertEllipse, N_A_VertEllipse),

        ARCH => xsect.aFull * lookup(yNorm, &A_Arch, N_A_Arch),

        IRREGULAR => xsect.aFull * lookup(yNorm, &prj.Transect[xsect.transect as usize].areaTbl, N_TRANSECT_TBL),

        CUSTOM => xsect.aFull * lookup(yNorm, &prj.Shape[prj.Curve[xsect.transect as usize].refersTo as usize].areaTbl, N_SHAPE_TBL),

        STREET_XSECT => xsect.aFull * lookup(yNorm, &prj.Street[xsect.transect as usize].transect.areaTbl, prj.Street[xsect.transect as usize].transect.nTbl),

        RECT_CLOSED => y * xsect.wMax,

        RECT_TRIANG => rect_triang_getAofY(xsect, y),

        RECT_ROUND => rect_round_getAofY(xsect, y),

        RECT_OPEN => y * xsect.wMax,

        MOD_BASKET => mod_basket_getAofY(xsect, y),

        TRAPEZOIDAL => trapez_getAofY(xsect, y),

        TRIANGULAR => triang_getAofY(xsect, y),

        PARABOLIC => parab_getAofY(xsect, y),

        POWERFUNC => powerfunc_getAofY(xsect, y),

        _ => 0.0,
    }
}

//=============================================================================

pub fn xsect_getWofY(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           y = depth ft)
    //  Output:  returns top width (ft)
    //  Purpose: computes xsection's top width at a given depth.
    //
    let mut yNorm: f64 = y / xsect.yFull;
    match xsect.type_ {
        FORCE_MAIN | CIRCULAR => xsect.wMax * lookup(yNorm, &W_Circ, N_W_Circ),

        FILLED_CIRCULAR => {
            yNorm = (y + xsect.yBot) / (xsect.yFull + xsect.yBot);
            xsect.wMax * lookup(yNorm, &W_Circ, N_W_Circ)
        }

        EGGSHAPED => xsect.wMax * lookup(yNorm, &W_Egg, N_W_Egg),

        HORSESHOE => xsect.wMax * lookup(yNorm, &W_Horseshoe, N_W_Horseshoe),

        GOTHIC => xsect.wMax * lookup(yNorm, &W_Gothic, N_W_Gothic),

        CATENARY => xsect.wMax * lookup(yNorm, &W_Catenary, N_W_Catenary),

        SEMIELLIPTICAL => xsect.wMax * lookup(yNorm, &W_SemiEllip, N_W_SemiEllip),

        BASKETHANDLE => xsect.wMax * lookup(yNorm, &W_BasketHandle, N_W_BasketHandle),

        SEMICIRCULAR => xsect.wMax * lookup(yNorm, &W_SemiCirc, N_W_SemiCirc),

        HORIZ_ELLIPSE => xsect.wMax * lookup(yNorm, &W_HorizEllipse, N_W_HorizEllipse),

        VERT_ELLIPSE => xsect.wMax * lookup(yNorm, &W_VertEllipse, N_W_VertEllipse),

        ARCH => xsect.wMax * lookup(yNorm, &W_Arch, N_W_Arch),

        IRREGULAR => xsect.wMax * lookup(yNorm, &prj.Transect[xsect.transect as usize].widthTbl, N_TRANSECT_TBL),

        CUSTOM => xsect.wMax * lookup(yNorm, &prj.Shape[prj.Curve[xsect.transect as usize].refersTo as usize].widthTbl, N_SHAPE_TBL),

        STREET_XSECT => xsect.wMax * lookup(yNorm, &prj.Street[xsect.transect as usize].transect.widthTbl, prj.Street[xsect.transect as usize].transect.nTbl),

        RECT_CLOSED => {
            if yNorm == 1.0 {
                return 0.0;
            }
            xsect.wMax
        }

        RECT_TRIANG => rect_triang_getWofY(xsect, y),

        RECT_ROUND => rect_round_getWofY(xsect, y),

        RECT_OPEN => xsect.wMax,

        MOD_BASKET => mod_basket_getWofY(xsect, y),

        TRAPEZOIDAL => trapez_getWofY(xsect, y),

        TRIANGULAR => triang_getWofY(xsect, y),

        PARABOLIC => parab_getWofY(xsect, y),

        POWERFUNC => powerfunc_getWofY(xsect, y),

        _ => 0.0,
    }
}

//=============================================================================

pub fn xsect_getRofY_mut(prj: &Project, xsect: &mut TXsect, y: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           y = depth (ft)
    //  Output:  returns hydraulic radius (ft)
    //  Purpose: computes xsection's hydraulic radius at a given depth.
    //
    let yNorm: f64 = y / xsect.yFull;
    match xsect.type_ {
        FORCE_MAIN | CIRCULAR => xsect.rFull * lookup(yNorm, &R_Circ, N_R_Circ),

        FILLED_CIRCULAR => {
            if xsect.yBot == 0.0 {
                return xsect.rFull * lookup(yNorm, &R_Circ, N_R_Circ);
            }
            filled_circ_getRofY(xsect, y)
        }

        EGGSHAPED => xsect.rFull * lookup(yNorm, &R_Egg, N_R_Egg),

        HORSESHOE => xsect.rFull * lookup(yNorm, &R_Horseshoe, N_R_Horseshoe),

        BASKETHANDLE => xsect.rFull * lookup(yNorm, &R_Baskethandle, N_R_Baskethandle),

        HORIZ_ELLIPSE => xsect.rFull * lookup(yNorm, &R_HorizEllipse, N_R_HorizEllipse),

        VERT_ELLIPSE => xsect.rFull * lookup(yNorm, &R_VertEllipse, N_R_VertEllipse),

        ARCH => xsect.rFull * lookup(yNorm, &R_Arch, N_R_Arch),

        IRREGULAR => xsect.rFull * lookup(yNorm, &prj.Transect[xsect.transect as usize].hradTbl, N_TRANSECT_TBL),

        CUSTOM => xsect.rFull * lookup(yNorm, &prj.Shape[prj.Curve[xsect.transect as usize].refersTo as usize].hradTbl, N_SHAPE_TBL),

        STREET_XSECT => xsect.rFull * lookup(yNorm, &prj.Street[xsect.transect as usize].transect.hradTbl, prj.Street[xsect.transect as usize].transect.nTbl),

        RECT_TRIANG => rect_triang_getRofY(xsect, y),

        RECT_ROUND => rect_round_getRofY(xsect, y),

        TRAPEZOIDAL => trapez_getRofY(xsect, y),

        TRIANGULAR => triang_getRofY(xsect, y),

        PARABOLIC => parab_getRofY(xsect, y),

        POWERFUNC => powerfunc_getRofY(xsect, y),

        _ => {
            let a = xsect_getAofY_mut(prj, xsect, y);
            xsect_getRofA_mut(prj, xsect, a)
        }
    }
}

//=============================================================================

pub fn xsect_getRofA_mut(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           a = area (ft2)
    //  Output:  returns hydraulic radius (ft)
    //  Purpose: computes xsection's hydraulic radius at a given area.
    //
    let cathy: f64;
    if a <= 0.0 {
        return 0.0;
    }
    match xsect.type_ {
        HORIZ_ELLIPSE | VERT_ELLIPSE | ARCH | IRREGULAR | FILLED_CIRCULAR | CUSTOM | STREET_XSECT => {
            let y = xsect_getYofA_mut(prj, xsect, a);
            xsect_getRofY_mut(prj, xsect, y)
        }

        RECT_CLOSED => rect_closed_getRofA(xsect, a),

        RECT_OPEN => a / (xsect.wMax + (2.0 - xsect.sBot) * a / xsect.wMax),

        RECT_TRIANG => rect_triang_getRofA(xsect, a),

        RECT_ROUND => rect_round_getRofA(xsect, a),

        MOD_BASKET => mod_basket_getRofA(xsect, a),

        TRAPEZOIDAL => trapez_getRofA(xsect, a),

        TRIANGULAR => triang_getRofA(xsect, a),

        PARABOLIC => parab_getRofA(xsect, a),

        POWERFUNC => powerfunc_getRofA(xsect, a),

        _ => {
            cathy = xsect_getSofA_mut(prj, xsect, a);
            if cathy < TINY || a < TINY {
                return 0.0;
            }
            (cathy / a).powf(3.0 / 2.0)
        }
    }
}

//=============================================================================

pub fn xsect_getAofS_mut(prj: &Project, xsect: &mut TXsect, sFactor: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           s = section factor (ft^(8/3))
    //  Output:  returns area (ft2)
    //  Purpose: computes xsection's area at a given section factor.
    //
    let mut s = sFactor;
    let psi: f64 = s / xsect.sFull;
    if s <= 0.0 {
        return 0.0;
    }
    if s > xsect.sMax {
        s = xsect.sMax;
    }
    match xsect.type_ {
        DUMMY => 0.0,

        FORCE_MAIN | CIRCULAR => circ_getAofS(xsect, s),

        EGGSHAPED => xsect.aFull * invLookup(psi, &S_Egg, N_S_Egg),

        HORSESHOE => xsect.aFull * invLookup(psi, &S_Horseshoe, N_S_Horseshoe),

        GOTHIC => xsect.aFull * invLookup(psi, &S_Gothic, N_S_Gothic),

        CATENARY => xsect.aFull * invLookup(psi, &S_Catenary, N_S_Catenary),

        SEMIELLIPTICAL => xsect.aFull * invLookup(psi, &S_SemiEllip, N_S_SemiEllip),

        BASKETHANDLE => xsect.aFull * invLookup(psi, &S_BasketHandle, N_S_BasketHandle),

        SEMICIRCULAR => xsect.aFull * invLookup(psi, &S_SemiCirc, N_S_SemiCirc),

        _ => generic_getAofS(prj, xsect, s),
    }
}

//=============================================================================

pub fn xsect_getdSdA_mut(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           a = area (ft2)
    //  Output:  returns derivative of section factor w.r.t. area (ft^2/3)
    //  Purpose: computes xsection's derivative of its section factor with
    //           respect to area at a given area.
    //
    match xsect.type_ {
        FORCE_MAIN | CIRCULAR => circ_getdSdA(xsect, a),

        EGGSHAPED => tabular_getdSdA(xsect, a, &S_Egg, N_S_Egg),

        HORSESHOE => tabular_getdSdA(xsect, a, &S_Horseshoe, N_S_Horseshoe),

        GOTHIC => tabular_getdSdA(xsect, a, &S_Gothic, N_S_Gothic),

        CATENARY => tabular_getdSdA(xsect, a, &S_Catenary, N_S_Catenary),

        SEMIELLIPTICAL => tabular_getdSdA(xsect, a, &S_SemiEllip, N_S_SemiEllip),

        BASKETHANDLE => tabular_getdSdA(xsect, a, &S_BasketHandle, N_S_BasketHandle),

        SEMICIRCULAR => tabular_getdSdA(xsect, a, &S_SemiCirc, N_S_SemiCirc),

        RECT_CLOSED => rect_closed_getdSdA(prj, xsect, a),

        RECT_OPEN => rect_open_getdSdA(prj, xsect, a),

        RECT_TRIANG => rect_triang_getdSdA(prj, xsect, a),

        RECT_ROUND => rect_round_getdSdA(prj, xsect, a),

        MOD_BASKET => mod_basket_getdSdA(prj, xsect, a),

        TRAPEZOIDAL => trapez_getdSdA(prj, xsect, a),

        TRIANGULAR => triang_getdSdA(prj, xsect, a),

        _ => generic_getdSdA(prj, xsect, a),
    }
}

//=============================================================================

pub fn xsect_getYcrit_mut(prj: &Project, xsect: &mut TXsect, q: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           q = flow rate (cfs)
    //  Output:  returns critical depth (ft)
    //  Purpose: computes critical depth at a specific flow rate.
    //
    let q2g: f64 = SQR(q) / GRAVITY;
    let mut y: f64;
    let r: f64;

    if q2g == 0.0 {
        return 0.0;
    }
    match xsect.type_ {
        DUMMY => {
            return 0.0;
        }

        RECT_OPEN | RECT_CLOSED => {
            // --- analytical expression for yCritical is
            //     y = (q2g / w^2)^(1/3) where w = width
            y = (q2g / SQR(xsect.wMax)).powf(1.0 / 3.0);
        }

        TRIANGULAR => {
            // --- analytical expression for yCritical is
            //     y = (2 * q2g / s^2)^(1/5) where s = side slope
            y = (2.0 * q2g / SQR(xsect.sBot)).powf(1.0 / 5.0);
        }

        PARABOLIC => {
            // --- analytical expression for yCritical is
            //     y = (27/32 * q2g * c)^(1/4) where y = c*x^2
            //     is eqn. for parabola and 1/sqrt(c) = rBot
            y = (27.0 / 32.0 * q2g / SQR(xsect.rBot)).powf(1.0 / 4.0);
        }

        POWERFUNC => {
            y = 1.0 / (2.0 * xsect.sBot + 3.0);
            y = (q2g * (xsect.sBot + 1.0) / SQR(xsect.rBot)).powf(y);
        }

        _ => {
            // --- first estimate yCritical for an equivalent circular conduit
            //     using 1.01 * (q2g / yFull)^(1/4)
            y = 1.01 * (q2g / xsect.yFull).powf(1.0 / 4.0);
            if y >= xsect.yFull {
                y = 0.97 * xsect.yFull;
            }

            // --- then find ratio of conduit area to equiv. circular area
            r = xsect.aFull / (PI / 4.0 * SQR(xsect.yFull));

            // --- use interval enumeration method to find yCritical if
            //     area ratio not too far from 1.0
            if r >= 0.5 && r <= 2.0 {
                y = getYcritEnum(prj, xsect, q, y);
            }
            // --- otherwise use Ridder's root finding method
            else {
                y = getYcritRidder(prj, xsect, q, y);
            }
        }
    }

    // --- do not allow yCritical to be > yFull
    MIN(y, xsect.yFull)
}

//=============================================================================

fn getTransectParams(xsect: &mut TXsect, transect: &TTransect) {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           transect = ptr. to a transect data structure
    //  Output:  none
    //  Purpose: gets a cross section's properties from its transect's properties.
    //
    let mut i: i32;
    let mut iMax: i32;
    let mut wMax: f64;
    let wTbl: &[f64] = &transect.widthTbl;

    xsect.yFull = transect.yFull;
    xsect.wMax = transect.wMax;
    xsect.aFull = transect.aFull;
    xsect.rFull = transect.rFull;
    xsect.sFull = xsect.aFull * xsect.rFull.powf(2.0 / 3.0);
    xsect.sMax = transect.sMax;
    xsect.aBot = transect.aMax;

    // Search transect's width table up to point where width decreases
    iMax = 0;
    wMax = wTbl[0];
    i = 1;
    while i < transect.nTbl {
        if wTbl[i as usize] < wMax {
            break;
        }
        wMax = wTbl[i as usize];
        iMax = i;
        i += 1;
    }

    // Determine height at lowest widest point
    xsect.ywMax = xsect.yFull * iMax as f64 / (transect.nTbl as f64 - 1.0);
}

//=============================================================================

fn generic_getAofS(prj: &Project, xsect: &mut TXsect, s: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to a cross section data structure
    //           s = section factor (ft^8/3)
    //  Output:  returns area (ft2)
    //  Purpose: finds area given section factor by
    //           solving S = A*(A/P(A))^(2/3) using Newton-Raphson iterations.
    //
    let mut a: f64;
    let a1: f64;
    let a2: f64;
    let tol: f64;

    if s <= 0.0 {
        return 0.0;
    }

    // --- if S is between sMax and sFull then
    //     bracket A between aFull and aMax
    if (s <= xsect.sMax && s >= xsect.sFull) && xsect.sMax != xsect.sFull {
        a1 = xsect.aFull; // do this because sFull < sMax
        a2 = xsect_getAmax(prj, xsect);
    }
    // --- otherwise bracket A between 0 and aMax
    else {
        a1 = 0.0;
        a2 = xsect_getAmax(prj, xsect);
    }

    // --- place S & xsect in xsectStar for access by evalSofA function
    //     (here: captured by the closure)

    // --- compute starting guess for A
    a = 0.5 * (a1 + a2);

    // use the Newton-Raphson root finder function to find A
    tol = 0.0001 * xsect.aFull;
    findroot_Newton(a1, a2, &mut a, tol, &mut |a: f64, f: &mut f64, df: &mut f64| evalSofA(prj, xsect, s, a, f, df));
    a
}

//=============================================================================

fn evalSofA(prj: &Project, xsect: &mut TXsect, sTarget: f64, a: f64, f: &mut f64, df: &mut f64) {
    //
    //  Input:   a = area
    //  Output:  f = root finding function
    //           df = derivative of root finding function
    //  Purpose: function used in conjunction with getAofS() that evaluates
    //           f = S(a) - s and df = dS(a)/dA.
    //
    let s: f64;
    s = xsect_getSofA_mut(prj, xsect, a);
    *f = s - sTarget;
    *df = xsect_getdSdA_mut(prj, xsect, a);
}

//=============================================================================

fn tabular_getdSdA(xsect: &TXsect, a: f64, table: &[f64], nItems: i32) -> f64 {
    //
    //  Input:   xsect = ptr. to cross section data structure
    //           a = area (ft2)
    //           table = ptr. to table of section factor v. normalized area
    //           nItems = number of equally spaced items in table
    //  Output:  returns derivative of section factor w.r.t. area (ft^2/3)
    //  Purpose: computes derivative of section factor w.r.t area
    //           using geometry tables.
    //
    let mut i: i32;
    let alpha: f64 = a / xsect.aFull;
    let delta: f64 = 1.0 / (nItems as f64 - 1.0);
    let dSdA: f64;

    // --- find which segment of table contains alpha
    i = f64_to_i32(alpha / delta);
    if i >= nItems - 1 {
        i = nItems - 2;
    }

    // --- compute slope from this interval of table
    dSdA = (tbl(table, i + 1) - tbl(table, i)) / delta;

    // --- convert slope to un-normalized value
    dSdA * xsect.sFull / xsect.aFull
}

//=============================================================================

fn generic_getdSdA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to cross section data structure
    //           a = area (ft2)
    //  Output:  returns derivative of section factor w.r.t. area (ft^2/3)
    //  Purpose: computes derivative of section factor w.r.t area
    //           using central difference approximation.
    //
    let a1: f64;
    let a2: f64;
    let alpha: f64 = a / xsect.aFull;
    let mut alpha1: f64 = alpha - 0.001;
    let alpha2: f64 = alpha + 0.001;
    if alpha1 < 0.0 {
        alpha1 = 0.0;
    }
    a1 = alpha1 * xsect.aFull;
    a2 = alpha2 * xsect.aFull;
    (xsect_getSofA_mut(prj, xsect, a2) - xsect_getSofA_mut(prj, xsect, a1)) / (a2 - a1)
}

//=============================================================================

/// `table[i]` in lookup() and tabular_getdSdA(). Their index is
/// (int)(x/delta), which is negative for x <= -delta and INT_MIN for a NaN
/// x; C then reads outside the table (EPA's x86-64 build reads 16 GB below
/// it for a NaN). The only such argument SWMM produces is a NaN (e.g. a
/// MOD_BASKET or RECT_ROUND depth beyond the arc, through acos), and then
/// the result is NaN whatever value is read. Out-of-range reads give 0.0.
#[inline]
fn tbl(table: &[f64], i: i32) -> f64 {
    if i >= 0 && (i as usize) < table.len() {
        table[i as usize]
    } else {
        0.0
    }
}

//=============================================================================

fn lookup(x: f64, table: &[f64], nItems: i32) -> f64 {
    //
    //  Input:   x = value of independent variable in a geometry table
    //           table = ptr. to geometry table
    //           nItems = number of equally spaced items in table
    //  Output:  returns value of dependent table variable
    //  Purpose: looks up a value in a geometry table (i.e., finds y given x).
    //
    let delta: f64;
    let x0: f64;
    let x1: f64;
    let mut y: f64;
    let y2: f64;
    let i: i32;

    // --- find which segment of table contains x
    delta = 1.0 / (nItems as f64 - 1.0);
    i = f64_to_i32(x / delta);
    if i >= nItems - 1 {
        return table[(nItems - 1) as usize];
    }

    // --- compute x at start and end of segment
    x0 = i as f64 * delta;
    x1 = (i as f64 + 1.0) * delta;

    // --- linearly interpolate a y-value
    y = tbl(table, i) + (x - x0) * (tbl(table, i + 1) - tbl(table, i)) / delta;

    // --- use quadratic interpolation for low x value
    if i < 2 {
        y2 = y + (x - x0) * (x - x1) / (delta * delta) * (tbl(table, i) / 2.0 - tbl(table, i + 1) + tbl(table, i + 2) / 2.0);
        if y2 > 0.0 {
            y = y2;
        }
    }
    if y < 0.0 {
        y = 0.0;
    }
    y
}

//=============================================================================

fn invLookup(y: f64, table: &[f64], nItems: i32) -> f64 {
    //
    //  Input:   y = value of dependent variable in a geometry table
    //           table = ptr. to geometry table
    //           nItems = number of equally spaced items in table
    //  Output:  returns value of independent table variable
    //  Purpose: performs inverse lookup in a geometry table (i.e., finds
    //           x given y).
    //
    //  Notes:   This function assumes that the geometry table has either strictly
    //           increasing entries or that the maximum entry is always third
    //           from the last (which is true for all section factor tables). In
    //           the latter case, the location of a large y can be ambiguous
    //           -- it can be both below and above the location of the maximum.
    //           In such cases this routine searches only the interval above
    //           the maximum (i.e., the last 2 segments of the table).
    //
    //           nItems-1 is the highest subscript for the table's data.
    //
    //           The x value's in a geometry table lie between 0 and 1.
    //
    let dx: f64; // x-increment of table
    let mut x: f64; // interpolation variables
    let x0: f64;
    let dy: f64;
    let mut n: i32; // # items in increasing portion of table
    let i: i32; // lower table index that brackets y

    // --- compute table's uniform x-increment
    dx = 1.0 / (nItems as f64 - 1.0);

    // --- truncate item count if last 2 table entries are decreasing
    n = nItems;
    if table[(n - 3) as usize] > table[(n - 1) as usize] {
        n = n - 2;
    }

    // --- check if y falls in decreasing portion of table
    if n < nItems && y > table[(nItems - 1) as usize] {
        if y >= table[(nItems - 3) as usize] {
            return (n as f64 - 1.0) * dx;
        }
        if y <= table[(nItems - 2) as usize] {
            i = nItems - 2;
        } else {
            i = nItems - 3;
        }
    }
    // --- otherwise locate the interval where y falls in the table
    else {
        i = locate(y, table, n - 1);
    }
    if i >= n - 1 {
        return (n as f64 - 1.0) * dx;
    }

    // --- compute x at start and end of segment
    x0 = i as f64 * dx;

    // --- linearly interpolate an x value
    dy = table[(i + 1) as usize] - table[i as usize];
    if dy == 0.0 {
        x = x0;
    } else {
        x = x0 + (y - table[i as usize]) * dx / dy;
    }
    if x < 0.0 {
        x = 0.0;
    }
    if x > 1.0 {
        x = 1.0;
    }
    x
}

//=============================================================================

fn locate(y: f64, table: &[f64], jLast: i32) -> i32 {
    //
    //  Input:   y      = value being located in table
    //           table  = ptr. to table with monotonically increasing entries
    //           jLast  = highest table entry index to search over
    //  Output:  returns index j of table such that table[j] <= y <= table[j+1]
    //  Purpose: uses bisection method to locate the highest table index whose
    //           table entry does not exceed a given value.
    //
    //  Notes:   This function is only used in conjunction with invLookup().
    //
    let mut j: i32;
    let mut j1: i32 = 0;
    let mut j2: i32 = jLast;

    // Check if value <= first table entry
    if y <= table[0] {
        return 0;
    }

    // Check if value >= the last entry
    if y >= table[jLast as usize] {
        return jLast;
    }

    // While a portion of the table still remains
    while j2 - j1 > 1 {
        // Find midpoint of remaining portion of table
        j = (j1 + j2) >> 1;

        // Value is greater or equal to midpoint: search from midpoint to j2
        if y >= table[j as usize] {
            j1 = j;
        }
        // Value is less than midpoint: search from j1 to midpoint
        else {
            j2 = j;
        }
    }

    // Return the lower index of the remaining interval,
    j1
}

//=============================================================================

fn getQcritical(prj: &Project, xsect: &mut TXsect, qcTarget: f64, yc: f64) -> f64 {
    //
    //  Input:   yc = critical depth (ft)
    //           p = pointer to a TXsectStar object
    //  Output:  returns flow difference value (cfs)
    //  Purpose: finds difference between critical flow at depth yc and
    //           some target value.
    //
    let a: f64;
    let w: f64;
    let mut qc: f64;

    a = xsect_getAofY_mut(prj, xsect, yc);
    w = xsect_getWofY(prj, xsect, yc);
    qc = -qcTarget;
    if w > 0.0 {
        qc = a * (GRAVITY * a / w).sqrt() - qcTarget;
    }
    qc
}

//=============================================================================

fn getYcritEnum(prj: &Project, xsect: &mut TXsect, q: f64, y0: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to cross section data structure
    //           q = critical flow rate (cfs)
    //           y0 = estimate of critical depth (ft)
    //  Output:  returns true critical depth (ft)
    //  Purpose: solves a * sqrt(a(y)*g / w(y)) - q for y using interval
    //           enumeration with starting guess of y0.
    //
    let mut q0: f64;
    let dy: f64;
    let mut qc: f64;
    let mut yc: f64;
    let i1: i32;
    let mut i: i32;
    // xsectStar.xsect = xsect; xsectStar.qc = 0.0;
    let xsectStarQc: f64 = 0.0;

    // --- divide cross section depth into 25 increments and
    //     locate increment corresponding to initial guess y0
    dy = xsect.yFull / 25.0;
    i1 = f64_to_i32(y0 / dy);

    // --- evaluate critical flow at this increment
    q0 = getQcritical(prj, xsect, xsectStarQc, i1 as f64 * dy);

    // --- initial flow lies below target flow
    if q0 < q {
        // --- search each successive higher depth increment
        yc = xsect.yFull;
        i = i1 + 1;
        while i <= 25 {
            // --- if critical flow at current depth is above target
            //     then use linear interpolation to compute critical depth
            qc = getQcritical(prj, xsect, xsectStarQc, i as f64 * dy);
            if qc >= q {
                yc = ((q - q0) / (qc - q0) + (i as f64 - 1.0)) * dy;
                break;
            }
            q0 = qc;
            i += 1;
        }
    }
    // --- initial flow lies above target flow
    else {
        // --- search each successively lower depth increment
        yc = 0.0;
        i = i1 - 1;
        while i >= 0 {
            // --- if critical flow at current depth is below target
            //     then use linear interpolation to compute critical depth
            qc = getQcritical(prj, xsect, xsectStarQc, i as f64 * dy);
            if qc < q {
                yc = ((q - qc) / (q0 - qc) + i as f64) * dy;
                break;
            }
            q0 = qc;
            i -= 1;
        }
    }
    yc
}

//=============================================================================

fn getYcritRidder(prj: &Project, xsect: &mut TXsect, q: f64, y0: f64) -> f64 {
    //
    //  Input:   xsect = ptr. to cross section data structure
    //           q = critical flow rate (cfs)
    //           y0 = estimate of critical depth (ft)
    //  Output:  returns true critical depth (ft)
    //  Purpose: solves a * sqrt(a(y)*g / w(y)) - q for y using Ridder's
    //           root finding method with starting guess of y0.
    //
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.99 * xsect.yFull;
    let yc: f64;
    let q0: f64;
    let q1: f64;
    let q2: f64;

    // --- store reference to cross section in global pointer
    let mut xsectStarQc: f64 = 0.0;

    // --- check if critical flow at (nearly) full depth < target flow
    q2 = getQcritical(prj, xsect, xsectStarQc, y2);
    if q2 < q {
        return xsect.yFull;
    }

    // --- evaluate critical flow at initial depth guess y0
    //     and at 1/2 of full depth
    q0 = getQcritical(prj, xsect, xsectStarQc, y0);
    q1 = getQcritical(prj, xsect, xsectStarQc, 0.5 * xsect.yFull);

    // --- adjust search interval on depth so it contains flow q
    if q0 > q {
        y2 = y0;
        if q1 < q {
            y1 = 0.5 * xsect.yFull;
        }
    } else {
        y1 = y0;
        if q1 > q {
            y2 = 0.5 * xsect.yFull;
        }
    }

    // --- save value of target critical flow in global variable
    xsectStarQc = q;

    // --- call Ridder root finding procedure with error tolerance
    //     of 0.001 ft. to find critical depth yc
    yc = findroot_Ridder(y1, y2, 0.001, &mut |yc: f64| getQcritical(prj, xsect, xsectStarQc, yc));
    yc
}

//=============================================================================
//  RECT_CLOSED fuctions
//=============================================================================

fn rect_closed_getSofA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    // --- if a > area corresponding to Smax then
    //     interpolate between sMax and Sfull
    let alfMax: f64 = RECT_ALFMAX;
    if a / xsect.aFull > alfMax {
        return xsect.sMax + (xsect.sFull - xsect.sMax) * (a / xsect.aFull - alfMax) / (1.0 - alfMax);
    }

    // --- otherwise use regular formula
    a * xsect_getRofA_mut(prj, xsect, a).powf(2.0 / 3.0)
}

fn rect_closed_getdSdA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    let alpha: f64;
    let alfMax: f64;
    let r: f64;

    // --- if above level corresponding to sMax, then
    //     use slope between sFull & sMax
    alfMax = RECT_ALFMAX;
    alpha = a / xsect.aFull;
    if alpha > alfMax {
        return (xsect.sFull - xsect.sMax) / ((1.0 - alfMax) * xsect.aFull);
    }

    // --- for small a/aFull use generic central difference formula
    if alpha <= 1.0e-30 {
        return generic_getdSdA(prj, xsect, a);
    }

    // --- otherwise evaluate dSdA = [5/3 - (2/3)(dP/dA)R]R^(2/3)
    //     (where P = wetted perimeter & dPdA = 2/width)
    r = xsect_getRofA_mut(prj, xsect, a);
    (5.0 / 3.0 - (2.0 / 3.0) * (2.0 / xsect.wMax) * r) * r.powf(2.0 / 3.0)
}

fn rect_closed_getRofA(xsect: &TXsect, a: f64) -> f64 {
    let mut p: f64;
    if a <= 0.0 {
        return 0.0;
    }
    p = xsect.wMax + 2.0 * a / xsect.wMax; // Wetted Perim = width + 2*area/width
    if a / xsect.aFull > RECT_ALFMAX {
        p += (a / xsect.aFull - RECT_ALFMAX) / (1.0 - RECT_ALFMAX) * xsect.wMax;
    }
    a / p
}

//=============================================================================
//  RECT_OPEN fuctions
//=============================================================================

fn rect_open_getSofA(xsect: &TXsect, a: f64) -> f64 {
    let y: f64 = a / xsect.wMax;
    let r: f64 = a / ((2.0 - xsect.sBot) * y + xsect.wMax);
    a * r.powf(2.0 / 3.0)
}

fn rect_open_getdSdA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    let r: f64;
    let dPdA: f64;

    // --- for small a/aFull use generic central difference formula
    if a / xsect.aFull <= 1.0e-30 {
        return generic_getdSdA(prj, xsect, a);
    }

    // --- otherwise evaluate dSdA = [5/3 - (2/3)(dP/dA)R]R^(2/3)
    //     (where P = wetted perimeter)
    r = xsect_getRofA_mut(prj, xsect, a);
    dPdA = (2.0 - xsect.sBot) / xsect.wMax; // since P = geom2 + 2a/geom2
    (5.0 / 3.0 - (2.0 / 3.0) * dPdA * r) * r.powf(2.0 / 3.0)
}

//=============================================================================
//  RECT_TRIANG fuctions
//=============================================================================

fn rect_triang_getYofA(xsect: &TXsect, a: f64) -> f64 {
    // below upper section
    if a <= xsect.aBot {
        (a / xsect.sBot).sqrt()
    }
    // above bottom section
    else {
        xsect.yBot + (a - xsect.aBot) / xsect.wMax
    }
}

fn rect_triang_getRofA(xsect: &TXsect, a: f64) -> f64 {
    let y: f64;
    let mut p: f64;
    let alf: f64;

    if a <= 0.0 {
        return 0.0;
    }
    y = rect_triang_getYofA(xsect, a);

    // below upper section
    if y <= xsect.yBot {
        return a / (2.0 * y * xsect.rBot);
    }

    // wetted perimeter without contribution of top surface
    p = 2.0 * xsect.yBot * xsect.rBot + 2.0 * (y - xsect.yBot);

    // top-surface contribution
    alf = (a / xsect.aFull) - RECT_TRIANG_ALFMAX;
    if alf > 0.0 {
        p += alf / (1.0 - RECT_TRIANG_ALFMAX) * xsect.wMax;
    }
    a / p
}

fn rect_triang_getSofA(xsect: &TXsect, a: f64) -> f64 {
    // --- if a > area corresponding to sMax, then
    //     interpolate between sMax and Sfull
    let alfMax: f64 = RECT_TRIANG_ALFMAX;
    if a / xsect.aFull > alfMax {
        xsect.sMax + (xsect.sFull - xsect.sMax) * (a / xsect.aFull - alfMax) / (1.0 - alfMax)
    }
    // --- otherwise use regular formula
    else {
        a * rect_triang_getRofA(xsect, a).powf(2.0 / 3.0)
    }
}

fn rect_triang_getdSdA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    let alpha: f64;
    let alfMax: f64;
    let dPdA: f64;
    let r: f64;

    // --- if a > area corresponding to sMax, then
    //     use slope between sFull & sMax
    alfMax = RECT_TRIANG_ALFMAX;
    alpha = a / xsect.aFull;
    if alpha > alfMax {
        return (xsect.sFull - xsect.sMax) / ((1.0 - alfMax) * xsect.aFull);
    }

    // --- use generic central difference method for very small a
    if alpha <= 1.0e-30 {
        return generic_getdSdA(prj, xsect, a);
    }

    // --- find deriv. of wetted perimeter
    if a > xsect.aBot {
        dPdA = 2.0 / xsect.wMax; // for upper rectangle
    } else {
        dPdA = xsect.rBot / (a * xsect.sBot).sqrt(); // for triang. bottom
    }

    // --- get hyd. radius & evaluate section factor derivative formula
    r = rect_triang_getRofA(xsect, a);
    (5.0 / 3.0 - (2.0 / 3.0) * dPdA * r) * r.powf(2.0 / 3.0)
}

fn rect_triang_getAofY(xsect: &TXsect, y: f64) -> f64 {
    if y <= xsect.yBot {
        y * y * xsect.sBot // below upper section
    } else {
        xsect.aBot + (y - xsect.yBot) * xsect.wMax // above bottom section
    }
}

fn rect_triang_getRofY(xsect: &TXsect, y: f64) -> f64 {
    let mut p: f64;
    let a: f64;
    let alf: f64;

    // y is below upper rectangular section
    if y <= xsect.yBot {
        return y * xsect.sBot / (2.0 * xsect.rBot);
    }

    // area
    a = xsect.aBot + (y - xsect.yBot) * xsect.wMax;

    // wetted perimeter without contribution of top surface
    p = 2.0 * xsect.yBot * xsect.rBot + 2.0 * (y - xsect.yBot);

    // top-surface contribution
    alf = (a / xsect.aFull) - RECT_TRIANG_ALFMAX;
    if alf > 0.0 {
        p += alf / (1.0 - RECT_TRIANG_ALFMAX) * xsect.wMax;
    }
    a / p
}

fn rect_triang_getWofY(xsect: &TXsect, y: f64) -> f64 {
    if y <= xsect.yBot {
        2.0 * xsect.sBot * y // below upper section
    } else {
        xsect.wMax // above bottom section
    }
}

//=============================================================================
//  RECT_ROUND fuctions
//=============================================================================

fn rect_round_getYofA(xsect: &TXsect, a: f64) -> f64 {
    let alpha: f64;

    // --- if above circular bottom:
    if a > xsect.aBot {
        return xsect.yBot + (a - xsect.aBot) / xsect.wMax;
    }

    // --- otherwise use circular xsection method to find height
    alpha = a / (PI * xsect.rBot * xsect.rBot);
    if alpha < 0.04 {
        return (2.0 * xsect.rBot) * getYcircular(alpha);
    }
    (2.0 * xsect.rBot) * lookup(alpha, &Y_Circ, N_Y_Circ)
}

fn rect_round_getRofA(xsect: &TXsect, a: f64) -> f64 {
    let y1: f64;
    let theta1: f64;
    let mut p: f64;
    let arg: f64;

    // --- if above circular invert ...
    if a <= 0.0 {
        return 0.0;
    }
    if a > xsect.aBot {
        // wetted perimeter without contribution of top surface
        let y1: f64 = (a - xsect.aBot) / xsect.wMax;
        let theta1: f64 = 2.0 * (xsect.wMax / 2.0 / xsect.rBot).asin();
        p = xsect.rBot * theta1 + 2.0 * y1;

        // top-surface contribution
        arg = (a / xsect.aFull) - RECT_ROUND_ALFMAX;
        if arg > 0.0 {
            p += arg / (1.0 - RECT_ROUND_ALFMAX) * xsect.wMax;
        }
        return a / p;
    }

    // --- if within circular invert ...
    y1 = rect_round_getYofA(xsect, a);
    theta1 = 2.0 * (1.0 - y1 / xsect.rBot).acos();
    p = xsect.rBot * theta1;
    a / p
}

fn rect_round_getSofA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    let alpha: f64;
    let aFull: f64;
    let sFull: f64;

    // --- if a > area corresponding to sMax,
    //     interpolate between sMax and sFull
    let alfMax: f64 = RECT_ROUND_ALFMAX;
    if a / xsect.aFull > alfMax {
        xsect.sMax + (xsect.sFull - xsect.sMax) * (a / xsect.aFull - alfMax) / (1.0 - alfMax)
    }
    // --- if above circular invert, use generic function
    else if a > xsect.aBot {
        a * xsect_getRofA_mut(prj, xsect, a).powf(2.0 / 3.0)
    }
    // --- otherwise use circular xsection function applied
    //     to full circular shape of bottom section
    else {
        aFull = PI * xsect.rBot * xsect.rBot;
        alpha = a / aFull;
        sFull = xsect.sBot;

        // --- use special function for small a/aFull
        if alpha < 0.04 {
            sFull * getScircular(alpha)
        }
        // --- otherwise use table
        else {
            sFull * lookup(alpha, &S_Circ, N_S_Circ)
        }
    }
}

fn rect_round_getdSdA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    let alfMax: f64;
    let r: f64;
    let dPdA: f64;

    // --- if a > area corresponding to sMax, then
    //     use slope between sFull & sMax
    alfMax = RECT_ROUND_ALFMAX;
    if a / xsect.aFull > alfMax {
        (xsect.sFull - xsect.sMax) / ((1.0 - alfMax) * xsect.aFull)
    }
    // --- if above circular invert, use analytical function for dS/dA
    else if a > xsect.aBot {
        r = rect_round_getRofA(xsect, a);
        dPdA = 2.0 / xsect.wMax; // d(wet perim)/dA for rect.
        (5.0 / 3.0 - (2.0 / 3.0) * dPdA * r) * r.powf(2.0 / 3.0)
    }
    // --- otherwise use generic finite difference function
    else {
        generic_getdSdA(prj, xsect, a)
    }
}

fn rect_round_getAofY(xsect: &TXsect, y: f64) -> f64 {
    let theta1: f64;

    // --- if above circular invert...
    if y > xsect.yBot {
        return xsect.aBot + (y - xsect.yBot) * xsect.wMax;
    }

    // --- find area of circular section
    theta1 = 2.0 * (1.0 - y / xsect.rBot).acos();
    0.5 * xsect.rBot * xsect.rBot * (theta1 - theta1.sin())
}

fn rect_round_getRofY(xsect: &TXsect, y: f64) -> f64 {
    let theta1: f64;

    // --- if above top of circular bottom, use RofA formula
    if y <= 0.0 {
        return 0.0;
    }
    if y > xsect.yBot {
        return rect_round_getRofA(xsect, rect_round_getAofY(xsect, y));
    }

    // --- find hyd. radius of circular section
    theta1 = 2.0 * (1.0 - y / xsect.rBot).acos();
    0.5 * xsect.rBot * (1.0 - theta1.sin()) / theta1
}

fn rect_round_getWofY(xsect: &TXsect, y: f64) -> f64 {
    // --- return width if depth above circular bottom section
    if y > xsect.yBot {
        return xsect.wMax;
    }

    // --- find width of circular section
    2.0 * (y * (2.0 * xsect.rBot - y)).sqrt()
}

//=============================================================================
//  MOD_BASKETHANDLE fuctions
//=============================================================================

// Note: the variables rBot, yBot, and aBot refer to properties of the
//       circular top portion of the cross-section (not the bottom)

fn mod_basket_getYofA(xsect: &TXsect, a: f64) -> f64 {
    let alpha: f64;
    let mut y1: f64;

    // --- water level below top of rectangular bottom
    if a <= xsect.aFull - xsect.aBot {
        return a / xsect.wMax;
    }

    // --- find unfilled top area / area of full circular top
    alpha = (xsect.aFull - a) / (PI * xsect.rBot * xsect.rBot);

    // --- find unfilled height
    if alpha < 0.04 {
        y1 = getYcircular(alpha);
    } else {
        y1 = lookup(alpha, &Y_Circ, N_Y_Circ);
    }
    y1 = 2.0 * xsect.rBot * y1;

    // --- return difference between full height & unfilled height
    xsect.yFull - y1
}

fn mod_basket_getRofA(xsect: &TXsect, a: f64) -> f64 {
    let mut y1: f64;
    let mut p: f64;
    let theta1: f64;

    // --- water level is below top of rectangular bottom;
    //     return hyd. radius of rectangle
    if a <= xsect.aFull - xsect.aBot {
        return a / (xsect.wMax + 2.0 * a / xsect.wMax);
    }

    // --- find height of empty area
    y1 = xsect.yFull - mod_basket_getYofA(xsect, a);

    // --- find angle of circular arc corresponding to this height
    theta1 = 2.0 * (1.0 - y1 / xsect.rBot).acos();

    // --- find perimeter of wetted portion of circular arc
    //     (angle of full circular opening was stored in sBot)
    p = (xsect.sBot - theta1) * xsect.rBot;

    // --- add on wetted perimeter of bottom rectangular area
    y1 = xsect.yFull - xsect.yBot;
    p = p + 2.0 * y1 + xsect.wMax;

    // --- return area / wetted perimeter
    a / p
}

fn mod_basket_getdSdA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    let r: f64;
    let dPdA: f64;

    // --- if water level below top of rectangular bottom but not
    //     empty then use same code as for rectangular xsection
    if a <= xsect.aFull - xsect.aBot && a / xsect.aFull > 1.0e-30 {
        r = a / (xsect.wMax + 2.0 * a / xsect.wMax);
        dPdA = 2.0 / xsect.wMax;
        (5.0 / 3.0 - (2.0 / 3.0) * dPdA * r) * r.powf(2.0 / 3.0)
    }
    // --- otherwise use generic function
    else {
        generic_getdSdA(prj, xsect, a)
    }
}

fn mod_basket_getAofY(xsect: &TXsect, y: f64) -> f64 {
    let a1: f64;
    let theta1: f64;
    let y1: f64;

    // --- if water level is below top of rectangular bottom
    //     return depth * width
    if y <= xsect.yFull - xsect.yBot {
        return y * xsect.wMax;
    }

    // --- find empty top circular area
    y1 = xsect.yFull - y;
    theta1 = 2.0 * (1.0 - y1 / xsect.rBot).acos();
    a1 = 0.5 * xsect.rBot * xsect.rBot * (theta1 - theta1.sin());

    // --- return difference between full and empty areas
    xsect.aFull - a1
}

fn mod_basket_getWofY(xsect: &TXsect, y: f64) -> f64 {
    let y1: f64;

    // --- if water level below top of rectangular bottom then return width
    if y <= 0.0 {
        return 0.0;
    }
    if y <= xsect.yFull - xsect.yBot {
        return xsect.wMax;
    }

    // --- find width of empty top circular section
    y1 = xsect.yFull - y;
    2.0 * (y1 * (2.0 * xsect.rBot - y1)).sqrt()
}

//=============================================================================
//  TRAPEZOIDAL fuctions
//
//  Note: yBot = width of bottom
//        sBot = avg. of side slopes
//        rBot = length of sides per unit of depth
//=============================================================================

fn trapez_getYofA(xsect: &TXsect, a: f64) -> f64 {
    if xsect.sBot == 0.0 {
        return a / xsect.yBot;
    }
    ((xsect.yBot * xsect.yBot + 4.0 * xsect.sBot * a).sqrt() - xsect.yBot) / (2.0 * xsect.sBot)
}

fn trapez_getRofA(xsect: &TXsect, a: f64) -> f64 {
    a / (xsect.yBot + trapez_getYofA(xsect, a) * xsect.rBot)
}

fn trapez_getdSdA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    let r: f64;
    let dPdA: f64;
    // --- use generic central difference method for very small a
    if a / xsect.aFull <= 1.0e-30 {
        return generic_getdSdA(prj, xsect, a);
    }

    // --- otherwise use analytical formula:
    //     dSdA = [5/3 - (2/3)(dP/dA)R]R^(2/3)
    r = trapez_getRofA(xsect, a);
    dPdA = xsect.rBot / (xsect.yBot * xsect.yBot + 4.0 * xsect.sBot * a).sqrt();
    (5.0 / 3.0 - (2.0 / 3.0) * dPdA * r) * r.powf(2.0 / 3.0)
}

fn trapez_getAofY(xsect: &TXsect, y: f64) -> f64 {
    (xsect.yBot + xsect.sBot * y) * y
}

fn trapez_getRofY(xsect: &TXsect, y: f64) -> f64 {
    if y == 0.0 {
        return 0.0;
    }
    trapez_getAofY(xsect, y) / (xsect.yBot + y * xsect.rBot)
}

fn trapez_getWofY(xsect: &TXsect, y: f64) -> f64 {
    xsect.yBot + 2.0 * y * xsect.sBot
}

//=============================================================================
//  TRIANGULAR fuctions
//=============================================================================

fn triang_getYofA(xsect: &TXsect, a: f64) -> f64 {
    (a / xsect.sBot).sqrt()
}

fn triang_getRofA(xsect: &TXsect, a: f64) -> f64 {
    a / (2.0 * triang_getYofA(xsect, a) * xsect.rBot)
}

fn triang_getdSdA(prj: &Project, xsect: &mut TXsect, a: f64) -> f64 {
    let r: f64;
    let dPdA: f64;
    // --- use generic finite difference method for very small 'a'
    if a / xsect.aFull <= 1.0e-30 {
        return generic_getdSdA(prj, xsect, a);
    }

    // --- evaluate dSdA = [5/3 - (2/3)(dP/dA)R]R^(2/3)
    r = triang_getRofA(xsect, a);
    dPdA = xsect.rBot / (a * xsect.sBot).sqrt();
    (5.0 / 3.0 - (2.0 / 3.0) * dPdA * r) * r.powf(2.0 / 3.0)
}

fn triang_getAofY(xsect: &TXsect, y: f64) -> f64 {
    y * y * xsect.sBot
}

fn triang_getRofY(xsect: &TXsect, y: f64) -> f64 {
    (y * xsect.sBot) / (2.0 * xsect.rBot)
}

fn triang_getWofY(xsect: &TXsect, y: f64) -> f64 {
    2.0 * xsect.sBot * y
}

//=============================================================================
//  PARABOLIC fuctions
//=============================================================================

fn parab_getYofA(xsect: &TXsect, a: f64) -> f64 {
    ((3.0 / 4.0) * a / xsect.rBot).powf(2.0 / 3.0)
}

fn parab_getRofA(xsect: &TXsect, a: f64) -> f64 {
    if a <= 0.0 {
        return 0.0;
    }
    a / parab_getPofY(xsect, parab_getYofA(xsect, a))
}

fn parab_getPofY(xsect: &TXsect, y: f64) -> f64 {
    let x: f64 = 2.0 * y.sqrt() / xsect.rBot;
    let t: f64 = (1.0 + x * x).sqrt();
    0.5 * xsect.rBot * xsect.rBot * (x * t + (x + t).ln())
}

fn parab_getAofY(xsect: &TXsect, y: f64) -> f64 {
    4.0 / 3.0 * xsect.rBot * y * y.sqrt()
}

fn parab_getRofY(xsect: &TXsect, y: f64) -> f64 {
    if y <= 0.0 {
        return 0.0;
    }
    parab_getAofY(xsect, y) / parab_getPofY(xsect, y)
}

fn parab_getWofY(xsect: &TXsect, y: f64) -> f64 {
    2.0 * xsect.rBot * y.sqrt()
}

//=============================================================================
//  POWERFUNC fuctions
//=============================================================================

fn powerfunc_getYofA(xsect: &TXsect, a: f64) -> f64 {
    (a / xsect.rBot).powf(1.0 / (xsect.sBot + 1.0))
}

fn powerfunc_getRofA(xsect: &TXsect, a: f64) -> f64 {
    if a <= 0.0 {
        return 0.0;
    }
    a / powerfunc_getPofY(xsect, powerfunc_getYofA(xsect, a))
}

fn powerfunc_getPofY(xsect: &TXsect, y: f64) -> f64 {
    let dy1: f64 = 0.02 * xsect.yFull;
    let h: f64 = (xsect.sBot + 1.0) * xsect.rBot / 2.0;
    let m: f64 = xsect.sBot;
    let mut p: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut x1: f64 = 0.0;
    let mut x2: f64;
    let mut y2: f64;
    let mut dx: f64;
    let mut dy: f64;
    loop {
        y2 = y1 + dy1;
        if y2 > y {
            y2 = y;
        }
        x2 = h * y2.powf(m);
        dx = x2 - x1;
        dy = y2 - y1;
        p += (dx * dx + dy * dy).sqrt();
        x1 = x2;
        y1 = y2;
        if !(y2 < y) {
            break;
        }
    }
    2.0 * p
}

fn powerfunc_getAofY(xsect: &TXsect, y: f64) -> f64 {
    xsect.rBot * y.powf(xsect.sBot + 1.0)
}

fn powerfunc_getRofY(xsect: &TXsect, y: f64) -> f64 {
    if y <= 0.0 {
        return 0.0;
    }
    powerfunc_getAofY(xsect, y) / powerfunc_getPofY(xsect, y)
}

fn powerfunc_getWofY(xsect: &TXsect, y: f64) -> f64 {
    (xsect.sBot + 1.0) * xsect.rBot * y.powf(xsect.sBot)
}

//=============================================================================
//  CIRCULAR functions
//=============================================================================

fn circ_getYofA(xsect: &TXsect, a: f64) -> f64 {
    let alpha: f64 = a / xsect.aFull;

    // --- use special function for small a/aFull
    if alpha < 0.04 {
        xsect.yFull * getYcircular(alpha)
    }
    // --- otherwise use table
    else {
        xsect.yFull * lookup(alpha, &Y_Circ, N_Y_Circ)
    }
}

fn circ_getAofS(xsect: &TXsect, s: f64) -> f64 {
    let psi: f64 = s / xsect.sFull;
    if psi == 0.0 {
        return 0.0;
    }
    if psi >= 1.0 {
        return xsect.aFull;
    }

    // --- use special function for small s/sFull
    if psi <= 0.015 {
        xsect.aFull * getAcircular(psi)
    }
    // --- otherwise use table
    else {
        xsect.aFull * invLookup(psi, &S_Circ, N_S_Circ)
    }
}

fn circ_getSofA(xsect: &TXsect, a: f64) -> f64 {
    let alpha: f64 = a / xsect.aFull;

    // --- use special function for small a/aFull
    if alpha < 0.04 {
        xsect.sFull * getScircular(alpha)
    }
    // --- otherwise use table
    else {
        xsect.sFull * lookup(alpha, &S_Circ, N_S_Circ)
    }
}

fn circ_getdSdA(xsect: &TXsect, a: f64) -> f64 {
    let alpha: f64;
    let theta: f64;
    let p: f64;
    let r: f64;
    let dPdA: f64;

    // --- for near-zero area, use generic central difference formula
    alpha = a / xsect.aFull;
    if alpha <= 1.0e-30 {
        1.0e-30 //generic_getdSdA(xsect, a);
    }
    // --- for small a/aFull use analytical derivative
    else if alpha < 0.04 {
        theta = getThetaOfAlpha(alpha);
        p = theta * xsect.yFull / 2.0;
        r = a / p;
        dPdA = 4.0 / xsect.yFull / (1.0 - theta.cos());
        (5.0 / 3.0 - (2.0 / 3.0) * dPdA * r) * r.powf(2.0 / 3.0)
    }
    // --- otherwise use generic tabular getdSdA
    else {
        tabular_getdSdA(xsect, a, &S_Circ, N_S_Circ)
    }
}

////////////////////////////////////////////////
// This is an alternate method used in SWMM 4.4.
// (commented out in the C source; not translated)
////////////////////////////////////////////////

fn circ_getAofY(xsect: &TXsect, y: f64) -> f64 {
    let yNorm: f64;
    yNorm = y / xsect.yFull;
    xsect.aFull * lookup(yNorm, &A_Circ, N_A_Circ)
}

//=============================================================================
//  FILLED_CIRCULAR functions
//=============================================================================
//  These adjust the caller's xsect in place and restore it afterwards, as C
//  does; the restore can leave yFull/aFull 1 ulp off their original values.

fn filled_circ_getYofA(xsect: &mut TXsect, a: f64) -> f64 {
    let mut a = a;
    let mut y: f64;

    // --- temporarily remove filled portion of circle
    xsect.yFull += xsect.yBot;
    xsect.aFull += xsect.aBot;
    a += xsect.aBot;

    // --- find depth in unfilled circle
    y = circ_getYofA(xsect, a);

    // --- restore original values
    y -= xsect.yBot;
    xsect.yFull -= xsect.yBot;
    xsect.aFull -= xsect.aBot;
    y
}

fn filled_circ_getAofY(xsect: &mut TXsect, y: f64) -> f64 {
    let mut y = y;
    let mut a: f64;

    // --- temporarily remove filled portion of circle
    xsect.yFull += xsect.yBot;
    xsect.aFull += xsect.aBot;
    y += xsect.yBot;

    // --- find area of unfilled circle
    a = circ_getAofY(xsect, y);

    // --- restore original values
    a -= xsect.aBot;
    xsect.yFull -= xsect.yBot;
    xsect.aFull -= xsect.aBot;
    a
}

fn filled_circ_getRofY(xsect: &mut TXsect, y: f64) -> f64 {
    let mut y = y;
    let mut a: f64;
    let mut r: f64;
    let mut p: f64;

    // --- temporarily remove filled portion of circle
    xsect.yFull += xsect.yBot;
    xsect.aFull += xsect.aBot;
    y += xsect.yBot;

    // --- get area,  hyd. radius & wetted perimeter of unfilled circle
    a = circ_getAofY(xsect, y);
    r = 0.25 * xsect.yFull * lookup(y / xsect.yFull, &R_Circ, N_R_Circ);
    p = a / r;

    // --- reduce area and wetted perimeter by amount of filled circle
    //     (rBot = filled perimeter, sBot = filled width)
    a = a - xsect.aBot;
    p = p - xsect.rBot + xsect.sBot;

    // --- compute actual hyd. radius & restore xsect parameters
    r = a / p;
    xsect.yFull -= xsect.yBot;
    xsect.aFull -= xsect.aBot;
    r
}

//=============================================================================
//  Special functions for circular cross sections
//=============================================================================

fn getYcircular(alpha: f64) -> f64 {
    let theta: f64;
    if alpha >= 1.0 {
        return 1.0;
    }
    if alpha <= 0.0 {
        return 0.0;
    }
    if alpha <= 1.0e-5 {
        let theta: f64 = (37.6911 * alpha).powf(1.0 / 3.0);
        return theta * theta / 16.0;
    }
    theta = getThetaOfAlpha(alpha);
    (1.0 - (theta / 2.0).cos()) / 2.0
}

fn getScircular(alpha: f64) -> f64 {
    let theta: f64;
    if alpha >= 1.0 {
        return 1.0;
    }
    if alpha <= 0.0 {
        return 0.0;
    }
    if alpha <= 1.0e-5 {
        let theta: f64 = (37.6911 * alpha).powf(1.0 / 3.0);
        return theta.powf(13.0 / 3.0) / 124.4797;
    }
    theta = getThetaOfAlpha(alpha);
    (theta - theta.sin()).powf(5.0 / 3.0) / (2.0 * PI) / theta.powf(2.0 / 3.0)
}

fn getAcircular(psi: f64) -> f64 {
    let theta: f64;
    if psi >= 1.0 {
        return 1.0;
    }
    if psi <= 0.0 {
        return 0.0;
    }
    if psi <= 1.0e-6 {
        let theta: f64 = (124.4797 * psi).powf(3.0 / 13.0);
        return theta * theta * theta / 37.6911;
    }
    theta = getThetaOfPsi(psi);
    (theta - theta.sin()) / (2.0 * PI)
}

fn getThetaOfAlpha(alpha: f64) -> f64 {
    let mut k: i32;
    let mut theta: f64;
    let theta1: f64;
    let ap: f64;
    let mut d: f64;

    if alpha > 0.04 {
        theta = 1.2 + 5.08 * (alpha - 0.04) / 0.96;
    } else {
        theta = 0.031715 - 12.79384 * alpha + 8.28479 * alpha.sqrt();
    }
    theta1 = theta;
    ap = (2.0 * PI) * alpha;
    k = 1;
    while k <= 40 {
        d = -(ap - theta + theta.sin()) / (1.0 - theta.cos());
        // --- modification to improve convergence for large theta
        if d > 1.0 {
            d = SIGN(1.0, d);
        }
        theta = theta - d;
        if d.abs() <= 0.0001 {
            return theta;
        }
        k += 1;
    }
    theta1
}

fn getThetaOfPsi(psi: f64) -> f64 {
    let mut k: i32;
    let mut theta: f64;
    let theta1: f64;
    let ap: f64;
    let mut tt: f64;
    let mut tt23: f64;
    let mut t3: f64;
    let mut d: f64;

    if psi > 0.90 {
        theta = 4.17 + 1.12 * (psi - 0.90) / 0.176;
    } else if psi > 0.5 {
        theta = 3.14 + 1.03 * (psi - 0.5) / 0.4;
    } else if psi > 0.015 {
        theta = 1.2 + 1.94 * (psi - 0.015) / 0.485;
    } else {
        theta = 0.12103 - 55.5075 * psi + 15.62254 * psi.sqrt();
    }
    theta1 = theta;
    ap = (2.0 * PI) * psi;

    k = 1;
    while k <= 40 {
        theta = theta.abs();
        tt = theta - theta.sin();
        tt23 = tt.powf(2.0 / 3.0);
        t3 = theta.powf(1.0 / 3.0);
        d = ap * theta / t3 - tt * tt23;
        d = d / (ap * (2.0 / 3.0) / t3 - (5.0 / 3.0) * tt23 * (1.0 - theta.cos()));
        theta = theta - d;
        if d.abs() <= 0.0001 {
            return theta;
        }
        k += 1;
    }
    theta1
}


//=============================================================================
//  The getters with the stub signatures (`&TXsect`).
//
//  For FILLED_CIRCULAR, C's getters temporarily add the filled bottom to
//  the caller's xsect->yFull/aFull and subtract it again; (yFull+yBot)-yBot
//  can come back 1 ulp different, and C leaves that changed value in the
//  caller's cross section. These wrappers run on a copy, so the caller's
//  TXsect is not changed; the `_mut` versions change it exactly as C does.
//  Every other shape leaves the cross section untouched, so for them the
//  two are identical.
//=============================================================================

/// Called by: kinwave, node. (Exact for every shape; see above for FILLED_CIRCULAR.)
pub fn xsect_getSofA(prj: &Project, xsect: &TXsect, area: f64) -> f64 {
    let mut xs: TXsect = *xsect;
    xsect_getSofA_mut(prj, &mut xs, area)
}

/// Called by: flowrout, inlet, link. (Exact for every shape; see above for FILLED_CIRCULAR.)
pub fn xsect_getYofA(prj: &Project, xsect: &TXsect, a: f64) -> f64 {
    let mut xs: TXsect = *xsect;
    xsect_getYofA_mut(prj, &mut xs, a)
}

/// Called by: culvert, dwflow, flowrout, link, node. (Exact for every shape; see above for FILLED_CIRCULAR.)
pub fn xsect_getAofY(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
    let mut xs: TXsect = *xsect;
    xsect_getAofY_mut(prj, &mut xs, y)
}

/// Called by: dwflow. (Exact for every shape; see above for FILLED_CIRCULAR.)
pub fn xsect_getRofY(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
    let mut xs: TXsect = *xsect;
    xsect_getRofY_mut(prj, &mut xs, y)
}

/// Called by: (entry point). (Exact for every shape; see above for FILLED_CIRCULAR.)
pub fn xsect_getRofA(prj: &Project, xsect: &TXsect, a: f64) -> f64 {
    let mut xs: TXsect = *xsect;
    xsect_getRofA_mut(prj, &mut xs, a)
}

/// Called by: flowrout, inlet, kinwave, link. (Exact for every shape; see above for FILLED_CIRCULAR.)
pub fn xsect_getAofS(prj: &Project, xsect: &TXsect, sFactor: f64) -> f64 {
    let mut xs: TXsect = *xsect;
    xsect_getAofS_mut(prj, &mut xs, sFactor)
}

/// Called by: kinwave. (Exact for every shape; see above for FILLED_CIRCULAR.)
pub fn xsect_getdSdA(prj: &Project, xsect: &TXsect, a: f64) -> f64 {
    let mut xs: TXsect = *xsect;
    xsect_getdSdA_mut(prj, &mut xs, a)
}

/// Called by: link. (Exact for every shape; see above for FILLED_CIRCULAR.)
pub fn xsect_getYcrit(prj: &Project, xsect: &TXsect, q: f64) -> f64 {
    let mut xs: TXsect = *xsect;
    xsect_getYcrit_mut(prj, &mut xs, q)
}

//=============================================================================
//  xsect.dat: geometry tables for rounded shapes
//=============================================================================
//-----------------------------------------------------------------------------
//   xsection.dat
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     08/01/22   (Build 5.2.1)
//   Author:   L. Rossman
//
//   Tables of relative geometric properties for rounded cross-sections.
//
//   Tables are named using the following convention:
//     A_xxx   = area / full area v. depth / full depth for shape xxx
//     Y_xxx   = depth / full depth v. area / full area for shape xxx
//     W_xxx   = width / max. width v. depth / full depth for shape xxx
//     R_xxx   = hyd. radius / hyd. radius at full depth v. depth /
//               full depth for shape xxx
//     S_xxx   = section factor / section factor at full depth v. area /
//               area at full depth for shape xxx
//     N_Z_xxx = number of equal intervals between 0.0 and 1.0 in the
//               table for parameter Z and shape xxx.
//
//   Release 5.2.1:
//   - The second and third entries in tables A_Egg, R_Egg, and W_Egg
//     were corrected.
//-----------------------------------------------------------------------------
//=============================================================================
//  CIRCULAR SHAPE
//=============================================================================
const N_A_Circ: i32 = 51;

const A_Circ: [f64; 51] = [  // A/Afull v. Y/Yfull
    0.0,0.00471,0.0134,0.024446,0.0374,0.05208,0.0680,0.08505,0.1033,0.12236,
    0.1423,0.16310,0.1845,0.20665,0.2292,0.25236,0.2759,0.29985,0.3242,0.34874,
    0.3736,0.39878,0.4237,0.44907,0.4745,0.500,0.5255,0.55093,0.5763,0.60135,
    0.6264,0.65126,0.6758,0.70015,0.7241,0.74764,0.7708,0.79335,0.8154,0.83690,
    0.8576,0.87764,0.8967,0.91495,0.9320,0.94792,0.9626,0.97555,0.9866,0.99516,1.000
];

const N_R_Circ: i32 = 51;

const R_Circ: [f64; 51] = [  // R/Rfull v. Y/Yfull
    0.0100,0.0528,0.1048,0.1556,0.2052,0.254,0.3016,0.3484,0.3944,0.4388,
    0.4824,0.5248,0.5664,0.6064,0.6456,0.6836,0.7204,0.7564,0.7912,0.8244,
    0.8568,0.888,0.9176,0.9464,0.9736,1.00,1.024,1.048,1.070,1.0912,1.110,1.1272,
    1.144,1.1596,1.174,1.1848,1.194,1.2024,1.210,1.2148,
    1.217,1.2172,1.215,1.2104,1.203,1.192,1.178,1.1584,1.132,1.094,1.000
];

const N_Y_Circ: i32 = 51;

const Y_Circ: [f64; 51] = [  // Y/Yfull v. A/Afull
    0.0, 0.05236, 0.08369, 0.11025, 0.13423, 0.15643, 0.17755, 0.19772, 0.21704,
    0.23581, 0.25412, 0.27194, 0.28948, 0.30653, 0.32349, 0.34017, 0.35666,
    0.37298, 0.38915, 0.40521, 0.42117, 0.43704, 0.45284, 0.46858, 0.4843,
    0.50000, 0.51572, 0.53146, 0.54723, 0.56305, 0.57892, 0.59487, 0.61093,
    0.62710, 0.64342, 0.65991, 0.67659, 0.69350, 0.71068, 0.72816, 0.74602,
    0.76424, 0.78297, 0.80235, 0.82240, 0.84353, 0.86563, 0.88970, 0.91444,
    0.94749, 1.0
];

const N_S_Circ: i32 = 51;

const S_Circ: [f64; 51] = [  // S/Sfull v. A/Afull
    0.0, 0.00529, 0.01432, 0.02559, 0.03859, 0.05304, 0.06877, 0.08551, 0.10326,
    0.12195, 0.14144, 0.16162, 0.18251, 0.2041,  0.22636, 0.24918, 0.27246,
    0.29614, 0.32027, 0.34485, 0.36989, 0.39531, 0.42105, 0.44704, 0.47329,
    0.4998,  0.52658, 0.55354, 0.58064, 0.60777, 0.63499, 0.66232, 0.68995,
    0.7177,  0.74538, 0.77275, 0.79979, 0.82658, 0.8532,  0.87954, 0.90546,
    0.93095, 0.95577, 0.97976, 1.00291, 1.02443, 1.04465, 1.06135, 1.08208,
    1.07662, 1.0
];

const N_W_Circ: i32 = 51;

const W_Circ: [f64; 51] = [  // W/Wmax v. Y/Yfull
    0.0,   0.2800,   0.3919,   0.4750,   0.5426,   0.6000,   0.6499,   0.6940,   0.7332,
    0.7684,   0.8000,   0.8285,   0.8542,   0.8773,   0.8980,   0.9165,   0.9330,
    0.9474,   0.9600,   0.9708,   0.9798,   0.9871,   0.9928,   0.9968,   0.9992,
    1.000,   0.9992,   0.9968,   0.9928,   0.9871,   0.9798,   0.9708,   0.9600,
    0.9474,   0.9330,   0.9165,   0.8980,   0.8773,   0.8542,   0.8285,   0.8000,
    0.7684,   0.7332,   0.6940,   0.6499,   0.6000,   0.5426,   0.4750,   0.3919,
    0.2800,   0.0
];


//=============================================================================
//  EGG SHAPE
//=============================================================================
const N_A_Egg: i32 = 26;

const A_Egg: [f64; 26] = [
    0.0000,0.0116,0.0316,0.0550,0.0850,0.1200,0.1555,0.1900,0.2250,0.2750,
    0.3200,0.3700,0.4200,0.4700,0.5150,0.5700,0.6200,0.6800,0.7300,0.7800,
    0.8350,0.8850,0.9250,0.9550,0.9800,1.000
];

const N_R_Egg: i32 = 26;

const R_Egg: [f64; 26] = [
    0.0100,0.1300,0.2440,0.3020,0.3860,0.4650,0.5360,0.6110,0.6760,0.7350,
    0.7910,0.8540,0.9040,0.9410,1.008,1.045,1.076,1.115,1.146,1.162,
    1.186,1.193,1.186,1.162,1.107,1.000
];

const N_Y_Egg: i32 = 51;

const Y_Egg: [f64; 51] = [
    0.0, 0.04912, 0.08101, 0.11128, 0.14161, 0.16622, 0.18811, 0.21356, 0.23742,
    0.25742, 0.27742, 0.29741, 0.31742, 0.33742, 0.35747, 0.37364, 0.4,
    0.41697, 0.43372, 0.45,    0.46374, 0.47747, 0.49209, 0.50989, 0.53015,
    0.55,    0.56429, 0.57675, 0.58834, 0.6,     0.61441, 0.62967, 0.64582,
    0.66368, 0.68209, 0.7,     0.71463, 0.72807, 0.74074, 0.75296, 0.765,
    0.77784, 0.79212, 0.80945, 0.82936, 0.85,    0.86731, 0.88769, 0.914,
    0.95,    1.0
];

const N_S_Egg: i32 = 51;

const S_Egg: [f64; 51] = [
    0.0, 0.00295, 0.01331, 0.02629, 0.04,    0.05657, 0.075,   0.09432, 0.11473,
    0.13657, 0.15894, 0.1803,  0.20036, 0.22,    0.23919, 0.25896, 0.28,
    0.30504, 0.33082, 0.35551, 0.37692, 0.39809, 0.42,    0.44625, 0.47321,
    0.5,     0.52255, 0.54481, 0.56785, 0.59466, 0.62485, 0.65518, 0.68181,
    0.70415, 0.72585, 0.74819, 0.77482, 0.80515, 0.83534, 0.86193, 0.88465,
    0.9069,  0.93,    0.95866, 0.98673, 1.01238, 1.03396, 1.05,    1.06517,
    1.0538,  1.0
];

const N_W_Egg: i32 = 26;

const W_Egg: [f64; 26] = [
    0.0,    0.3250,   0.4270,   0.5080,   0.5820,   0.6420,   0.6960,   0.7460,   0.7910,
    0.8360,   0.8660,   0.8960,   0.9260,   0.9560,   0.9700,   0.9850,   1.000,
    0.9850,   0.9700,   0.9400,   0.8960,   0.8360,   0.7640,   0.6420,   0.3100,
    0.0
];



//=============================================================================
//  HORSESHOE SHAPE
//=============================================================================
const N_A_Horseshoe: i32 = 26;

const A_Horseshoe: [f64; 26] = [
    0.0000,0.0181,0.0508,0.0908,0.1326,0.1757,0.2201,0.2655,0.3118,0.3587,
    0.4064,0.4542,0.5023,0.5506,0.5987,0.6462,0.6931,0.7387,0.7829,0.8253,
    0.8652,0.9022,0.9356,0.9645,0.9873,1.000
];

const N_R_Horseshoe: i32 = 26;

const R_Horseshoe: [f64; 26] = [
    0.0100,0.1040,0.2065,0.3243,0.4322,0.5284,0.6147,0.6927,0.7636,0.8268,
    0.8873,0.9417,0.9905,1.036,1.077,1.113,1.143,1.169,1.189,1.202,
    1.208,1.206,1.195,1.170,1.126,1.000
];

const N_Y_Horseshoe: i32 = 51;

const Y_Horseshoe: [f64; 51] = [
    0.0, 0.04146, 0.07033, 0.09098, 0.10962, 0.12921, 0.14813, 0.16701, 0.18565,
    0.20401, 0.22211, 0.23998, 0.25769, 0.27524, 0.29265, 0.3099,  0.32704,
    0.34406, 0.36101, 0.3779,  0.39471, 0.41147, 0.42818, 0.44484, 0.46147,
    0.47807, 0.49468, 0.51134, 0.52803, 0.54474, 0.56138, 0.57804, 0.59478,
    0.61171, 0.62881, 0.64609, 0.6635,  0.68111, 0.69901, 0.71722, 0.73583,
    0.7549,  0.77447, 0.79471, 0.81564, 0.83759, 0.86067, 0.88557, 0.91159,
    0.9452,  1.0
];

const N_S_Horseshoe: i32 = 51;

const S_Horseshoe: [f64; 51] = [
    0.0, 0.00467, 0.01237, 0.02268, 0.03515, 0.04943, 0.06525, 0.08212, 0.10005,
    0.11891, 0.13856, 0.15896, 0.18004, 0.20172, 0.22397, 0.24677, 0.27006,
    0.2938,  0.3179,  0.34237, 0.3672,  0.39239, 0.41792, 0.44374, 0.46984,
    0.49619, 0.52276, 0.5495,  0.5764,  0.60345, 0.63065, 0.65795, 0.68531,
    0.71271, 0.74009, 0.76738, 0.79451, 0.82144, 0.84814, 0.8745,  0.90057,
    0.92652, 0.95244, 0.97724, 0.99988, 1.02048, 1.03989, 1.05698, 1.07694,
    1.07562, 1.0
];

const N_W_Horseshoe: i32 = 26;

const W_Horseshoe: [f64; 26] = [
    0.0,      0.5878,   0.8772,   0.8900,   0.9028,   0.9156,   0.9284,   0.9412,
    0.9540,   0.9668,   0.9798,   0.9928,   0.9992,   0.9992,   0.9928,   0.9798,
    0.9600,   0.9330,   0.8980,   0.8542,   0.8000,   0.7332,   0.6499,   0.5426,
    0.3919,   0.0
];



//=============================================================================
//  GOTHIC SHAPE
//=============================================================================
const N_Y_Gothic: i32 = 51;

const Y_Gothic: [f64; 51] = [
    0.0, 0.04522, 0.07825, 0.10646, 0.12645, 0.14645, 0.16787, 0.18641, 0.20129,
    0.22425, 0.24129, 0.25624, 0.27344, 0.29097, 0.30529, 0.32607, 0.33755,
    0.35073, 0.36447, 0.37558, 0.4,     0.4181,  0.43648, 0.45374, 0.46805,
    0.48195, 0.49626, 0.51352, 0.5319,  0.55,    0.56416, 0.57787, 0.59224,
    0.6095,  0.62941, 0.65,    0.67064, 0.69055, 0.70721, 0.72031, 0.73286,
    0.74632, 0.76432, 0.78448, 0.80421, 0.82199, 0.84363, 0.87423, 0.90617,
    0.93827, 1.0
];

const N_S_Gothic: i32 = 51;

const S_Gothic: [f64; 51] = [
    0.0, 0.005,   0.0174,  0.03098, 0.04272, 0.055,   0.0698,  0.0862,  0.10461,
    0.12463, 0.145,   0.16309, 0.18118, 0.2,     0.22181, 0.24487, 0.26888,
    0.2938,  0.31901, 0.34389, 0.36564, 0.38612, 0.4072,  0.43,    0.45868,
    0.48895, 0.52,    0.55032, 0.5804,  0.61,    0.63762, 0.66505, 0.6929,
    0.72342, 0.75467, 0.785,   0.81165, 0.83654, 0.86,    0.88253, 0.90414,
    0.925,   0.94486, 0.96475, 0.98567, 1.00833, 1.03,    1.0536,  1.065,
    1.055,   1.0
];

const N_W_Gothic: i32 = 21;

const W_Gothic: [f64; 21] = [
    0.0,   0.286,   0.643,   0.762,   0.833,   0.905,   0.952,   0.976,   0.976,
    1.0,     1.0,     0.976,   0.976,   0.952,   0.905,   0.833,   0.762,
    0.667,   0.524,   0.357,   0.0
];



//=============================================================================
//  CATENARY SHAPE
//=============================================================================
const N_Y_Catenary: i32 = 51;

const Y_Catenary: [f64; 51] = [
    0.0, 0.02974, 0.06439, 0.08433, 0.10549, 0.12064, 0.13952, 0.1556,  0.17032,
    0.18512, 0.20057, 0.21995, 0.24011, 0.25892, 0.27595, 0.29214, 0.30802,
    0.32372, 0.33894, 0.35315, 0.36557, 0.37833, 0.3923,  0.4097,  0.42982,
    0.45,    0.46769, 0.48431, 0.5,     0.51466, 0.52886, 0.54292, 0.55729,
    0.57223, 0.5878,  0.60428, 0.62197, 0.64047, 0.6598,  0.67976, 0.7,
    0.71731, 0.73769, 0.76651, 0.8,     0.8209,  0.84311, 0.87978, 0.91576,
    0.95,    1.0
];

const N_S_Catenary: i32 = 51;

const S_Catenary: [f64; 51] = [
    0.0, 0.00605, 0.01455, 0.0254,  0.03863, 0.0543,  0.07127, 0.08778, 0.10372,
    0.12081, 0.14082, 0.16375, 0.18779, 0.21157, 0.23478, 0.25818, 0.28244,
    0.30741, 0.33204, 0.35505, 0.37465, 0.39404, 0.41426, 0.43804, 0.46531,
    0.49357, 0.52187, 0.54925, 0.57647, 0.60321, 0.62964, 0.65639, 0.68472,
    0.71425, 0.74303, 0.76827, 0.79168, 0.815,   0.84094, 0.86707, 0.89213,
    0.91607, 0.94,    0.96604, 0.99,    1.00714, 1.02158, 1.03814, 1.05,
    1.05,    1.0
];

const N_W_Catenary: i32 = 21;

const W_Catenary: [f64; 21] = [
    0.0,    0.6667,  0.8222,  0.9111,  0.9778,  1.0000,  1.0000,  0.9889,  0.9778,
    0.9556,  0.9333,  0.8889,  0.8444,  0.8000,  0.7556,  0.7000,  0.6333,
    0.5556,  0.4444,  0.3333,  0.0
];



//=============================================================================
//  SEMI-ELLIPTICAL SHAPE
//=============================================================================
const N_Y_SemiEllip: i32 = 51;

const Y_SemiEllip: [f64; 51] = [
    0.0, 0.03075, 0.05137, 0.07032, 0.09,    0.11323, 0.13037, 0.14519, 0.15968,
    0.18459, 0.19531, 0.21354, 0.22694, 0.23947, 0.25296, 0.265,   0.27784,
    0.29212, 0.3097,  0.32982, 0.35,    0.36738, 0.3839,  0.4,     0.41667,
    0.43333, 0.45,    0.46697, 0.48372, 0.5,     0.51374, 0.52747, 0.54209,
    0.5595,  0.57941, 0.6,     0.62,    0.64,    0.66,    0.68,    0.7,
    0.71843, 0.73865, 0.76365, 0.7926,  0.82088, 0.85,    0.88341, 0.90998,
    0.93871, 1.0
];

const N_S_SemiEllip: i32 = 51;

const S_SemiEllip: [f64; 51] = [
    0.0, 0.00438, 0.01227, 0.02312, 0.03638, 0.05145, 0.06783, 0.085,   0.10093,
    0.11752, 0.1353,  0.15626, 0.17917, 0.20296, 0.22654, 0.24962, 0.27269,
    0.29568, 0.31848, 0.34152, 0.365,   0.38941, 0.41442, 0.44,    0.46636,
    0.49309, 0.52,    0.54628, 0.57285, 0.6,     0.62949, 0.65877, 0.68624,
    0.71017, 0.73304, 0.75578, 0.77925, 0.80368, 0.83114, 0.8595,  0.88592,
    0.90848, 0.93,    0.95292, 0.97481, 0.99374, 1.01084, 1.02858, 1.04543,
    1.05,    1.0
];

const N_W_SemiEllip: i32 = 21;

const W_SemiEllip: [f64; 21] = [
    0.0,    0.7000,  0.9800,  1.0000,  1.0000,  1.0000,  0.9900,  0.9800,  0.9600,
    0.9400,  0.9100,  0.8800,  0.8400,  0.8000,  0.7500,  0.7000,  0.6400,
    0.5600,  0.4600,  0.3400,  0.0
];



//=============================================================================
//  BASKETHANDLE SHAPE
//=============================================================================
const N_A_Baskethandle: i32 = 26;

const A_Baskethandle: [f64; 26] = [
    0.0000,0.0173,0.0457,0.0828,0.1271,0.1765,0.2270,0.2775,0.3280,0.3780,
    0.4270,0.4765,0.5260,0.5740,0.6220,0.6690,0.7160,0.7610,0.8030,0.8390,
    0.8770,0.9110,0.9410,0.9680,0.9880,1.000
];

const N_R_Baskethandle: i32 = 26;

const R_Baskethandle: [f64; 26] = [
    0.0100,0.0952,0.1890,0.2730,0.3690,0.4630,0.5600,0.6530,0.7430,0.8220,
    0.8830,0.9490,0.9990,1.055,1.095,1.141,1.161,1.188,1.206,1.206,
    1.206,1.205,1.196,1.168,1.127,1.000
];

const N_Y_BasketHandle: i32 = 51;

const Y_BasketHandle: [f64; 51] = [
    0.0, 0.04112, 0.0738,  0.1,     0.12236, 0.14141, 0.15857, 0.17462, 0.18946,
    0.20315, 0.21557, 0.22833, 0.2423,  0.25945, 0.27936, 0.3,     0.3204,
    0.34034, 0.35892, 0.37595, 0.39214, 0.40802, 0.42372, 0.43894, 0.45315,
    0.46557, 0.47833, 0.4923,  0.50945, 0.52936, 0.55,    0.57,    0.59,
    0.61023, 0.63045, 0.65,    0.66756, 0.68413, 0.7,     0.71481, 0.72984,
    0.74579, 0.76417, 0.78422, 0.80477, 0.82532, 0.85,    0.88277, 0.915,
    0.95,    1.0
];

const N_S_BasketHandle: i32 = 51;

const S_BasketHandle: [f64; 51] = [
    0.0, 0.00758, 0.01812, 0.03,    0.03966, 0.04957, 0.0623,  0.07849, 0.09618,
    0.11416, 0.13094, 0.14808, 0.16583, 0.18381, 0.20294, 0.225,   0.2547,
    0.28532, 0.31006, 0.32804, 0.34555, 0.36944, 0.40032, 0.43203, 0.46004,
    0.47849, 0.49591, 0.51454, 0.5381,  0.56711, 0.6,     0.64092, 0.68136,
    0.71259, 0.73438, 0.755,   0.78625, 0.8188,  0.85,    0.8679,  0.88483,
    0.90431, 0.9369,  0.97388, 1.00747, 1.033,   1.05,    1.05464, 1.06078,
    1.055,   1.0
];

const N_W_BasketHandle: i32 = 26;

const W_BasketHandle: [f64; 26] = [
    0.0, 0.49,    0.667,   0.82,    0.93,    1.00,    1.00,    1.00,    0.997,
    0.994,   0.988,   0.982,   0.967,   0.948,   0.928,   0.904,   0.874,
    0.842,   0.798,   0.75,    0.697,   0.637,   0.567,   0.467,   0.342,
    0.0
];



//=============================================================================
//  SEMI-CIRCULAR SHAPE
//=============================================================================
const N_Y_SemiCirc: i32 = 51;

const Y_SemiCirc: [f64; 51] = [
    0.0, 0.04102, 0.07407, 0.1,     0.11769, 0.13037, 0.14036, 0.15,    0.16546,
    0.18213, 0.2,     0.22018, 0.2403,  0.25788, 0.27216, 0.285,   0.29704,
    0.30892, 0.32128, 0.33476, 0.35,    0.36927, 0.38963, 0.41023, 0.43045,
    0.45,    0.46769, 0.48431, 0.5,     0.51443, 0.52851, 0.54271, 0.55774,
    0.57388, 0.59101, 0.60989, 0.63005, 0.65,    0.66682, 0.68318, 0.7,
    0.71675, 0.73744, 0.76651, 0.8,     0.8209,  0.84311, 0.87978, 0.91576,
    0.95, 1.0
];

const N_S_SemiCirc: i32 = 51;

const S_SemiCirc: [f64; 51] = [
    0.0, 0.00757, 0.01815, 0.03,    0.0358,  0.04037, 0.04601, 0.055,   0.07475,
    0.09834, 0.125,   0.1557,  0.18588, 0.20883, 0.223,   0.23472, 0.24667,
    0.26758, 0.29346, 0.32124, 0.35,    0.3772,  0.4054,  0.43541, 0.46722,
    0.5,     0.53532, 0.56935, 0.6,     0.61544, 0.62811, 0.6417,  0.66598,
    0.7001,  0.73413, 0.76068, 0.78027, 0.8,     0.82891, 0.85964, 0.89,
    0.9127,  0.93664, 0.96677, 1.0,     1.02661, 1.04631, 1.05726, 1.06637,
    1.06,    1.0
];

const N_W_SemiCirc: i32 = 21;

const W_SemiCirc: [f64; 21] = [
    0.0, 0.5488,  0.8537,  1.0000,  1.0000,  0.9939,  0.9878,  0.9756,  0.9634,
    0.9451,  0.9207,  0.8902,  0.8537,  0.8171,  0.7683,  0.7073,  0.6463,
    0.5732,  0.4756,  0.3354,  0.0
];


//=============================================================================
//  SIZES FOR STANDARD ELLIPSE SHAPES
//=============================================================================
const NumCodesEllipse: i32 = 23;


//  Minor  axis (inches)
const MinorAxis_Ellipse: [f64; 23] = [
    14.0,19.0,22.0,24.0,27.0,29.0,32.0,34.0,38.0,43.0,48.0,53.0,58.0,63.0,68.0,72.0,77.0,82.0,87.0,92.0,97.0,106.0,116.0
];


//  Major axis (inches)
const MajorAxis_Ellipse: [f64; 23] = [
    23.0,30.0,34.0,38.0,42.0,45.0,49.0,53.0,60.0,68.0,76.0,83.0,91.0,98.0,106.0,113.0, 121.0,128.0,136.0,143.0,151.0,
    166.0,180.0
];


//  Full area (sq.ft.)
const Afull_Ellipse: [f64; 23] = [
    1.80,3.30,4.10,5.10,6.30,7.40,8.80,10.20,12.90,16.60,20.50,24.80,29.50,34.60,
    40.10,46.10,52.40,59.20,66.40,74.00,82.00,99.20,118.60
];


//  Full hydraulic radius (ft)
const Rfull_Ellipse: [f64; 23] = [
    0.367,0.490,0.546,0.613,0.686,0.736,0.812,0.875,0.969,1.106,1.229,1.352,
    1.475,1.598,1.721,1.845,1.967,2.091,2.215,2.340,2.461,2.707,2.968
];


//=============================================================================
//  HORIZONTAL ELLIPSE SHAPE
//=============================================================================
const N_A_HorizEllipse: i32 = 26;

const A_HorizEllipse: [f64; 26] = [
    0.0000,0.0150,0.0400,0.0650,0.0950,0.1300,0.1650,0.2050,0.2500,0.3000,
    0.3550,0.4150,0.4800,0.5200,0.5850,0.6450,0.7000,0.7500,0.7950,0.8350,
    0.8700,0.9050,0.9350,0.9600,0.9850,1.000
];

const N_R_HorizEllipse: i32 = 26;

const R_HorizEllipse: [f64; 26] = [
    0.0100,0.0764,0.1726,0.2389,0.3274,0.4191,0.5120,0.5983,0.6757,0.7630,
    0.8326,0.9114,0.9702,1.030,1.091,1.146,1.185,1.225,1.257,1.274,
    1.290,1.282,1.274,1.257,1.185,1.000
];

const N_W_HorizEllipse: i32 = 26;

const W_HorizEllipse: [f64; 26] = [
    0.0,0.3919,0.5426,0.6499,0.7332,0.8000,0.8542,0.8980,0.9330,0.9600,
    0.9798,0.9928,0.9992,0.9992,0.9928,0.9798,0.9600,0.9330,0.8980,0.8542,
    0.8000,0.7332,0.6499,0.5426,0.3919,0.0
];



//=============================================================================
//  VERTICAL ELLIPSE SHAPE
//=============================================================================
const N_A_VertEllipse: i32 = 26;

const A_VertEllipse: [f64; 26] = [
    0.0000,0.0100,0.0400,0.0700,0.1000,0.1400,0.1850,0.2300,0.2800,0.3300,
    0.3800,0.4300,0.4800,0.5200,0.5700,0.6200,0.6700,0.7200,0.7700,0.8150,
    0.8600,0.9000,0.9300,0.9600,0.9900,1.000
];

const N_R_VertEllipse: i32 = 26;

const R_VertEllipse: [f64; 26] = [
    0.0100,0.1250,0.2436,0.3536,0.4474,0.5484,0.6366,0.7155,0.7768,0.8396,
    0.8969,0.9480,0.9925,1.023,1.053,1.084,1.107,1.130,1.154,1.170,
    1.177,1.177,1.170,1.162,1.122,1.000
];

const N_W_VertEllipse: i32 = 26;

const W_VertEllipse: [f64; 26] = [
    0.0,0.3919,0.5426,0.6499,0.7332,0.8000,0.8542,0.8980,0.9330,0.9600,
    0.9798,0.9928,0.9992,0.9992,0.9928,0.9798,0.9600,0.9330,0.8980,0.8542,
    0.8000,0.7332,0.6499,0.5426,0.3919,0.0
];



//=============================================================================
//  ARCH SHAPE
//=============================================================================
const NumCodesArch: i32 = 102;

const Yfull_Arch: [f64; 102] = [  // NOTE: these are in inches

    // Concrete
    11.0,13.5,15.5,18.0,22.5,26.625,31.3125,36.0,40.0,45.0,54.0,62.0,
    72.0,77.5,87.125,96.875,106.5,

    // Corrugated Steel (2-2/3 x 1/2 inch Corrugation)
    13.0,15.0,18.0,20.0,24.0,29.0,33.0,38.0,43.0,47.0,52.0,57.0,

    // Corrugated Steel (3 x 1 inch Corrugation)
    31.0,36.0,41.0,46.0,51.0,55.0,59.0,63.0,67.0,71.0,75.0,79.0,83.0,87.0,91.0, // 2nd value corrected

    // Structural Plate (6 x 2 inch Corrugation - Bolted Seams         19-inch Corner Radius
    55.0,57.0,59.0,61.0,63.0,65.0,67.0,69.0,71.0,73.0,75.0,77.0,79.0,81.0,83.0,85.0,87.0,89.0,91.0,
    93.0,95.0,97.0,100.0,101.0,103.0,105.0,107.0,109.0,111.0,113.0,115.0,118.0,119.0,121.0,

    // Structural Plate (6 x 2 inch Corrugation - Bolted Seams         31-inch Corner Radius
    112.0,114.0,116.0,118.0,120.0,122.0,124.0,126.0,128.0,130.0,132.0,134.0,
    136.0,138.0,140.0,142.0,144.0,146.0,148.0,150.0,152.0,154.0,156.0,158.0
];

const Wmax_Arch: [f64; 102] = [  // NOTE: these are in inches

    // Concrete
    18.0,22.0,26.0,28.5,36.25,43.75,51.125,58.5,65.0,73.0,88.0,102.0,
    115.0,122.0,138.0,154.0,168.75,

    // Corrugated Steel (2 2/3 x 1/2 inch corrugation)
    17.0,21.0,24.0,28.0,35.0,42.0,49.0,57.0,64.0,71.0,77.0,83.0,

    // Corrugated Steel (3 x 1 inch Corrugation)
    40.0,46.0,53.0,60.0,66.0,73.0,81.0,87.0,95.0,103.0,112.0,117.0,128.0,137.0,142.0,


    // Structural Plate (6 x 2 inch Corrugation - Bolted Seams         19-inch Corner Radius
    73.0,76.0,81.0,84.0,87.0, 92.0,95.0,98.0,103.0,106.0,112.0,114.0,117.0,123.0,128.0,131.0,137.0,139.0,
    142.0,148.0,150.0,152.0,154.0,161.0,167.0,169.0,171.0,178.0,184.0,186.0,188.0,190.0,197.0,199.0,

    // Structural Plate (6 x 2 inch Corrugation - Bolted Seams         31-inch Corner Radius
    159.0,162.0,168.0,170.0,173.0,179.0,184.0,187.0,190.0,195.0,198.0,204.0,
    206.0,209.0,215.0,217.0,223.0,225.0,231.0,234.0,236.0,239.0,245.0,247.0
];

const Afull_Arch: [f64; 102] = [
    1.1,1.65,2.2,2.8,4.4,6.4,8.8,11.4,14.3,17.7,25.6,34.6,
    44.5,51.7,66.0,81.8,99.1,1.1,1.6,2.2,2.9,4.5,6.5,8.9,11.6,14.7,18.1,
    21.9,26.0,7.0,9.4,12.3,15.6,19.3,23.2,27.4,32.1,37.0,42.4,48.0,54.2,60.5,
    67.4,74.5,22.0,24.0,26.0,28.0,31.0,33.0,35.0,38.0,40.0,43.0,46.0,49.0,52.0,55.0,58.0,61.0,64.0,67.0,
    71.0,74.0,78.0,81.0,85.0,89.0,93.0,97.0,101.0,105.0,109.0,113.0,118.0,122.0,126.0,131.0,97.0,102.0,
    105.0,109.0,114.0,118.0,123.0,127.0,132.0,137.0,142.0,146.0,151.0,157.0,161.0,167.0,172.0,177.0,
    182.0,188.0,194.0,200.0,205.0,211.0
];

const Rfull_Arch: [f64; 102] = [
    0.25,0.3,0.36,0.45,0.56,0.68,0.8,0.9,1.01,1.13,1.35,
    1.57,1.77,1.92,2.17,2.42,2.65,0.324,0.374,0.449,0.499,0.598,0.723,
    0.823,0.947,1.072,1.171,1.296,1.421,0.773,0.773,1.022,1.147,1.271,
    1.371,1.471,1.570,1.670,1.770,1.869,1.969,2.069,2.168,2.268,1.371,
    1.421,1.471,1.520,1.570,1.620,1.670,1.720,1.770,1.820,1.869,1.919,
    1.969,2.019,2.069,2.119,2.168,2.218,2.268,2.318,2.368,2.418,2.493,
    2.517,2.567,2.617,2.667,2.717,2.767,2.817,2.866,2.941,2.966,3.016,
    2.792,2.841,2.891,2.941,2.991,3.041,3.091,3.141,3.190,3.240,3.290,
    3.340,3.390,3.440,3.490,3.539,3.589,3.639,3.689,3.739,3.789,3.838,
    3.888,3.938
];

const N_A_Arch: i32 = 26;

const A_Arch: [f64; 26] = [
    0.0000,0.0200,0.0600,0.1000,0.1400,0.1900,0.2400,0.2900,0.3400,0.3900,
    0.4400,0.4900,0.5400,0.5900,0.6400,0.6900,0.7350,0.7800,0.8200,0.8600,
    0.8950,0.9300,0.9600,0.9850,0.9950,1.000
];

const N_R_Arch: i32 = 26;

const R_Arch: [f64; 26] = [
    0.0100,0.0983,0.1965,0.2948,0.3940,0.4962,0.5911,0.6796,0.7615,0.8364,
    0.9044,0.9640,1.018,1.065,1.106,1.142,1.170,1.192,1.208,1.217,
    1.220,1.213,1.196,1.168,1.112,1.000
];

const N_W_Arch: i32 = 26;

const W_Arch: [f64; 26] = [
    0.0,0.6272,0.8521,0.9243,0.9645,0.9846,0.9964,0.9988,0.9917,0.9811,
    0.9680,0.9515,0.9314,0.9101,0.8864,0.8592,0.8284,0.7917,0.7527,0.7065,
    0.6544,0.5953,0.5231,0.4355,0.3195,0.0
];

//=============================================================================
//  Getters on a link's own cross-section (not in C as functions)
//=============================================================================
//
// C passes `&Link[j].xsect` to the getters above. For a FILLED_CIRCULAR
// section the filled_circ_* helpers add the filled depth and area back into
// yFull and aFull, compute, and take them out again; the round trip can leave
// yFull or aFull one bit different, and C keeps the changed value in the link
// for the rest of the run. These helpers make the same edit on
// prj.Link[j].xsect, so the Rust engine carries the same drift. For every
// other shape they are the plain getters.
macro_rules! link_xsect_getter {
    ($name:ident, $mutfn:ident) => {
        pub fn $name(prj: &mut Project, j: i32, x: f64) -> f64 {
            let mut xs = prj.Link[j as usize].xsect;
            let r = $mutfn(prj, &mut xs, x);
            prj.Link[j as usize].xsect = xs;
            r
        }
    };
}
link_xsect_getter!(linkXsect_getSofA, xsect_getSofA_mut);
link_xsect_getter!(linkXsect_getYofA, xsect_getYofA_mut);
link_xsect_getter!(linkXsect_getAofY, xsect_getAofY_mut);
link_xsect_getter!(linkXsect_getRofY, xsect_getRofY_mut);
link_xsect_getter!(linkXsect_getRofA, xsect_getRofA_mut);
link_xsect_getter!(linkXsect_getAofS, xsect_getAofS_mut);
link_xsect_getter!(linkXsect_getdSdA, xsect_getdSdA_mut);
link_xsect_getter!(linkXsect_getYcrit, xsect_getYcrit_mut);
