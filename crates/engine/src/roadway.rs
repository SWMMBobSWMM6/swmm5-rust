//! roadway.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   roadway.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21   (Build 5.2.0)
//   Author:   L. Rossman
//
//   Roadway Weir module for SWMM5
//
//   Computes flow overtopping a roadway (with a ROADWAY_WEIR object) using
//   the FWHA HDS-5 methodology.
//
//   Typically used in conjuction with a culvert crossing where the culvert
//   conduit is placed at zero offset at the upstream node and the Roadway
//   weir has the same upstream node but with an offset equal to the height
//   of the roadway.
//
//   Update History
//   ==============
//   Build 5.1.012:
//   - Entries in discharge coeff. table for gravel roadways corrected.
//-----------------------------------------------------------------------------

use crate::prelude::*;

// enum RoadSurface
const PAVED: i32 = 1;
#[allow(dead_code)] // C never names it either (any surface other than PAVED is gravel)
const GRAVEL: i32 = 2;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------

// The discharge coefficients and submergence factors listed below were
// derived from Figure 10 in "Bridge Waterways Analysis Model: Research
// Report", U.S. Dept. of Transportation Federal Highway Administration
// Report No. FHWA/RD-86/108, McLean, VA, July 1986.

// Discharge Coefficients for (head / road width) <= 0.15
const N_Cr_Low_Paved: i32 = 4;
const Cr_Low_Paved: [[f64; 2]; 4] = [[0.0, 2.85], [0.2, 2.95], [0.7, 3.03], [4.0, 3.05]];

const N_Cr_Low_Gravel: i32 = 8;
const Cr_Low_Gravel: [[f64; 2]; 8] = [
    [0.0, 2.5], [0.5, 2.7], [1.0, 2.8], [1.5, 2.9], [2.0, 2.98],
    [2.5, 3.02], [3.0, 3.03], [4.0, 3.05],
];

// Discharge Coefficients for (head / road width) > 0.15
const N_Cr_High_Paved: i32 = 2;
const Cr_High_Paved: [[f64; 2]; 2] = [[0.15, 3.05], [0.25, 3.10]];

const N_Cr_High_Gravel: i32 = 2;
const Cr_High_Gravel: [[f64; 2]; 2] = [[0.15, 2.95], [0.30, 3.10]];

// Submergence Factors
const N_Kt_Paved: i32 = 9;
const Kt_Paved: [[f64; 2]; 9] = [
    [0.8, 1.0], [0.85, 0.98], [0.90, 0.92], [0.93, 0.85], [0.95, 0.80],
    [0.97, 0.70], [0.98, 0.60], [0.99, 0.50], [1.00, 0.40],
];

const N_Kt_Gravel: i32 = 12;
const Kt_Gravel: [[f64; 2]; 12] = [
    [0.75, 1.00], [0.80, 0.985], [0.83, 0.97], [0.86, 0.93], [0.89, 0.90],
    [0.90, 0.87], [0.92, 0.80], [0.94, 0.70], [0.96, 0.60], [0.98, 0.50],
    [0.99, 0.40], [1.00, 0.24],
];

/// The file-scope variables of roadway.c (it has none).
#[derive(Default)]
pub struct RoadwayState {
}

//=============================================================================

/// Called by: link.
pub fn roadway_getInflow(
    prj: &mut Project,
    j: i32,     // link index
    dir: f64,   // flow direction (+1 or -1)
    hRoad: f64, // road elev. (ft)
    h1: f64,    // upstream head (ft)
    h2: f64,    // downstream head (ft)
) -> f64 {
    let ju = j as usize;
    let k: i32; // weir array index
    let roadSurf: i32; // type of road surface
    let mut useVariableCd: i32; // true if HDS-5 coeff. curves used
    let length: f64; // length of roadway segment transverse to flow (ft)
    let roadWidth: f64; // width of roadway which receives flow (ft)
    let hWr: f64; // water elevation on upstream side of roadway (ft)
    let ht: f64; // water elevation on downstream side of roadway (ft)
    let mut cD: f64; // discharge coefficient for cfs flow units
    let mut q: f64 = 0.0; // flow across roadway (cfs)
    let mut dqdh: f64 = 0.0; // derivative of flow w.r.t. head (ft2/sec)

    // --- get road width & surface type
    if prj.Link[ju].type_ != WEIR {
        return 0.0;
    }
    k = prj.Link[ju].subIndex;
    let ku = k as usize;
    roadWidth = prj.Weir[ku].roadWidth;
    roadSurf = prj.Weir[ku].roadSurface;

    // --- user-supplied discharge coeff.
    cD = prj.Weir[ku].cDisch1;
    if prj.UnitSystem == SI {
        cD = cD / 0.552;
    }

    // --- check if there's enough info to use a variable cD value
    useVariableCd = FALSE;
    if roadWidth > 0.0 && roadSurf >= 1 {
        useVariableCd = TRUE;
    }

    // --- upstream and downstream heads
    hWr = h1 - hRoad;
    ht = h2 - hRoad;
    if hWr > FUDGE {
        // --- get discharge coeff. as function of heads
        if useVariableCd != 0 {
            cD = getCd(hWr, ht, roadWidth, roadSurf);
        }

        // --- use user-supplied weir length
        length = prj.Link[ju].xsect.wMax;

        // --- weir eqn. for discharge across roadway
        q = cD * length * hWr.powf(1.5);
        dqdh = 1.5 * q / hWr;
    }

    // --- assign output values
    prj.Link[ju].dqdh = dqdh;
    prj.Link[ju].newDepth = MAX(h1 - hRoad, 0.0);
    prj.Link[ju].flowClass = SUBCRITICAL;
    if hRoad > h2 {
        if dir == 1.0 {
            prj.Link[ju].flowClass = DN_CRITICAL;
        } else {
            prj.Link[ju].flowClass = UP_CRITICAL;
        }
    }
    dir * q
}

//=============================================================================

fn getCd(hWr: f64, ht: f64, roadWidth: f64, roadSurf: i32) -> f64 {
    let mut kT: f64 = 1.0; // submergence factor
    let hL: f64; // ratio of water elevation to road width
    let htH: f64; // ratio of downstream to upstream water depth
    let cR: f64; // roadway discharge coeff.

    if hWr <= 0.0 {
        return 0.0;
    }
    hL = hWr / roadWidth;
    if hL <= 0.15 {
        if roadSurf == PAVED {
            cR = getY(hWr, &Cr_Low_Paved, N_Cr_Low_Paved);
        } else {
            cR = getY(hWr, &Cr_Low_Gravel, N_Cr_Low_Gravel);
        }
    } else {
        if roadSurf == PAVED {
            cR = getY(hL, &Cr_High_Paved, N_Cr_High_Paved);
        } else {
            cR = getY(hL, &Cr_High_Gravel, N_Cr_High_Gravel);
        }
    }
    if ht > 0.0 {
        htH = ht / hWr;
        if roadSurf == PAVED {
            kT = getY(htH, &Kt_Paved, N_Kt_Paved);
        } else {
            kT = getY(htH, &Kt_Gravel, N_Kt_Gravel);
        }
    }
    cR * kT
}

//=============================================================================

fn getY(x: f64, table: &[[f64; 2]], n: i32) -> f64 {
    let mut i: i32;
    let x1: f64;
    let y1: f64;
    let dx: f64;
    let dy: f64;
    if x <= table[0][0] {
        return table[0][1];
    }
    if x >= table[(n - 1) as usize][0] {
        return table[(n - 1) as usize][1];
    }
    i = 1;
    while i < n {
        let iu = i as usize;
        if x <= table[iu][0] {
            x1 = table[iu - 1][0];
            dx = table[iu][0] - x1;
            y1 = table[iu - 1][1];
            dy = table[iu][1] - y1;
            return y1 + (x - x1) * dy / dx;
        }
        i += 1;
    }
    table[(n - 1) as usize][1]
}
