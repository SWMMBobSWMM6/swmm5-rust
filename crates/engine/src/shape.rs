//! shape.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   shape.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21   (Build 5.2.0)
//   Author:   L. Rossman
//
//   Geometry functions for custom cross-section shapes.
//-----------------------------------------------------------------------------
//
// C keeps the running totals Atotal and Ptotal in file-scope variables that
// are set at the start of every computeShapeTables call and only used during
// it; here they are a local struct passed to the helpers that update them.

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of shape.c (Atotal, Ptotal: see `Totals`).
#[derive(Default)]
pub struct ShapeState {}

/// static double Atotal; static double Ptotal;
struct Totals {
    Atotal: f64,
    Ptotal: f64,
}

//=============================================================================

pub fn shape_validate(shape: &mut TShape, curve: &mut TTable) -> i32 {
    //
    //  Input:   shape = pointer to a custom x-section TShape object
    //           curve = pointer to shape's table of width v. height
    //  Output:  returns TRUE if successful. FALSE if not
    //  Purpose: computes the entries in a custom x-section shape's geometry
    //           tables from its user-supplied width v. height curve.
    //
    if computeShapeTables(shape, curve) == 0 {
        return FALSE;
    }

    if normalizeShapeTables(shape) == 0 {
        return FALSE;
    }

    TRUE
}

//=============================================================================

fn computeShapeTables(shape: &mut TShape, curve: &mut TTable) -> i32 {
    //
    //  Input:   shape = pointer to a TShape object
    //           curve = pointer to shape's table of width v. depth
    //  Output:  returns TRUE if successful. FALSE if not
    //  Purpose: computes the entries in a shape's geometry tables from
    //           the shape's width v. height curve normalized with repsect
    //           to full height.
    //
    //  Note:    the shape curve is a user-supplied table of width v. height
    //           for a custom x-section of unit height.
    let mut i: i32;
    let n: i32;
    let dy: f64;
    let mut y: f64;
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.0;
    let mut w: f64;
    let mut w1: f64 = 0.0;
    let mut w2: f64 = 0.0;
    let mut yLast: f64;
    let mut wLast: f64;
    let mut wMax: f64;
    let mut tot = Totals { Atotal: 0.0, Ptotal: 0.0 };

    // --- get first entry of user's shape curve
    if table_getFirstEntry(curve, &mut y1, &mut w1) == 0 {
        return FALSE;
    }

    if y1 < 0.0 || y1 >= 1.0 || w1 < 0.0 {
        return FALSE;
    }

    wMax = w1;

    // --- if first entry not at zero ht. then add an initial entry
    if y1 != 0.0 {
        y2 = y1;
        w2 = w1;
        y1 = 0.0;
        w1 = 0.0;
    }
    // --- otherwise get next entry in the user's shape curve
    else {
        if table_getNextEntry(curve, &mut y2, &mut w2) == 0 {
            return FALSE;
        }

        if y2 < y1 || w2 < 0.0 {
            return FALSE;
        }

        if y2 > 1.0 {
            y2 = 1.0;
        }

        if w2 > wMax {
            wMax = w2;
        }
    }

    // --- determine number of entries & interval size in geom. tables
    shape.nTbl = N_SHAPE_TBL;
    n = shape.nTbl - 1;
    dy = 1.0 / n as f64;

    // --- initialize geometry tables
    shape.areaTbl[0] = 0.0;
    shape.hradTbl[0] = 0.0;
    shape.widthTbl[0] = w1;
    tot.Ptotal = w1;
    tot.Atotal = 0.0;

    // --- fill in rest of geometry tables
    y = 0.0;
    w = w1;

    i = 1;
    while i <= n {
        // --- advance to next relative height level
        yLast = y;
        wLast = w;
        y = y + dy;

        // --- do not allow height to exceed 1.0
        if (y - 1.0).abs() < TINY {
            y = 1.0;
        }

        // --- if height exceeds current shape curve interval,
        //     move to next interval of shape curve
        if y > y2 {
            if getNextInterval(&mut tot, curve, y, yLast, wLast, &mut y1, &mut y2, &mut w1, &mut w2, &mut wMax) == 0 {
                return FALSE;
            }

            yLast = y1;
            wLast = w1;
        }

        // --- get top width, area, & perimeter of current interval
        w = getWidth(y, y1, y2, w1, w2);
        tot.Atotal += getArea(y, w, yLast, wLast);
        tot.Ptotal += getPerim(y, w, yLast, wLast);

        // --- add top width to total perimeter if at top of shape
        if y == 1.0 {
            tot.Ptotal += w2;
        }

        // --- update table values
        shape.widthTbl[i as usize] = w;
        shape.areaTbl[i as usize] = tot.Atotal;
        if tot.Ptotal > 0.0 {
            shape.hradTbl[i as usize] = tot.Atotal / tot.Ptotal;
        } else {
            shape.hradTbl[i as usize] = 0.0;
        }
        i += 1;
    }

    // --- assign values to shape'a area and hyd. radius when full
    shape.aFull = shape.areaTbl[n as usize];
    shape.rFull = shape.hradTbl[n as usize];

    // --- assign values to shape's max. width and section factor
    shape.wMax = wMax;
    getSmax(shape);

    TRUE
}

//=============================================================================

fn getSmax(shape: &mut TShape) {
    //
    //  Input:   shape = pointer to a TShape object
    //  Output:  none
    //  Purpose: computes the max. section factor and corresponding area
    //           for a shape of unit height.
    //
    let mut i: i32;
    let n: i32 = shape.nTbl - 1;
    let mut sf: f64;

    shape.sMax = 0.0;
    shape.aMax = 0.0;

    i = 1;
    while i <= n {
        sf = shape.areaTbl[i as usize] * shape.hradTbl[i as usize].powf(2.0 / 3.0);

        if sf > shape.sMax {
            shape.sMax = sf;
            shape.aMax = shape.areaTbl[i as usize];
        }
        i += 1;
    }
}

//=============================================================================

fn normalizeShapeTables(shape: &mut TShape) -> i32 {
    //
    //  Input:   shape = pointer to a TShape object
    //  Output:  returns TRUE if successful. FALSE if not
    //  Purpose: normalizes a shape's area tables to its full (or max.) condition.
    //
    let mut i: i32;
    let n: i32 = shape.nTbl - 1; // highest table entry index
    let aFull: f64 = shape.aFull; // area when full
    let rFull: f64 = shape.rFull; // hyd. radius when full
    let wMax: f64 = shape.wMax; // max. width

    // --- check that normalizing factors are non-zero
    if aFull == 0.0 || rFull == 0.0 || wMax == 0.0 {
        return FALSE;
    }

    // --- normalize entries in each table by their respective factors
    i = 0;
    while i <= n {
        shape.areaTbl[i as usize] /= aFull;
        shape.hradTbl[i as usize] /= rFull;
        shape.widthTbl[i as usize] /= wMax;
        i += 1;
    }

    TRUE
}

//=============================================================================

fn getNextInterval(tot: &mut Totals, curve: &mut TTable, y: f64, yLast: f64, wLast: f64, y1: &mut f64, y2: &mut f64, w1: &mut f64, w2: &mut f64, wMax: &mut f64) -> i32 {
    //
    //  Input:   curve = pointer to a user-supplied shape curve table
    //           y = current height in a geometry table
    //           yLast = previous height in a geometry table
    //           wLast = previous width in a geometry table
    //           y1 = height at start of current curve interval
    //           y2 = height at end of current curve interval
    //           w1 = width at start of current curve interval
    //           w2 = width at end of current curve interval
    //           wMax = current maximum width of curve
    //  Output:  updated values for yLast, wLast, y1, y2, w1, w2, and wMax;
    //           returns TRUE if successful, FALSE if not.
    //  Purpose: advances to the next height interval of a shape's curve that
    //           contains the current height being evaluated in the shape's
    //           geometry table.
    //
    //  Note:    heights and widths are with repsect to a shape of unit height.
    let mut yLast = yLast;
    let mut wLast = wLast;

    // --- keep advancing while the current geom. table height is
    //     above the end of the curve table interval
    while y > *y2 {
        // --- move start of geom. table interval up to the end of
        //     the current curve table interval
        if *y2 > yLast {
            tot.Atotal += getArea(*y2, *w2, yLast, wLast);
            tot.Ptotal += getPerim(*y2, *w2, yLast, wLast);
            yLast = *y2;
            wLast = *w2;
        }

        // --- move to the next curve table interval
        *y1 = *y2;
        *w1 = *w2;
        if table_getNextEntry(curve, y2, w2) == 0 {
            *y2 = 1.0;
            return TRUE;
        }

        // --- update curve table's max. width
        if *w2 > *wMax {
            *wMax = *w2;
        }

        // --- check for valid curve table values
        if *y2 < *y1 || *w2 < 0.0 {
            return FALSE;
        }

        if *y2 > 1.0 {
            *y2 = 1.0;
        }
    }

    TRUE
}

//=============================================================================

fn getWidth(y: f64, y1: f64, y2: f64, w1: f64, w2: f64) -> f64 {
    //
    //  Input:   y = height along a shape curve
    //           y1 = height at start of a shape curve interval
    //           y2 = height at end of a shape curve interval
    //           w1 = width at start of a shape curve interval
    //           w2 = width at end of a shape curve interval
    //  Output:  returns the width corresponding to height y
    //  Purpose: interpolates a width within a given height interval along a
    //           x-section's shape curve.
    //
    if y2 == y1 {
        return w2;
    }

    w1 + (y - y1) / (y2 - y1) * (w2 - w1)
}

//=============================================================================

fn getArea(y: f64, w: f64, y1: f64, w1: f64) -> f64 {
    //
    //  Input:   y = height along a shape curve
    //           w = width that corresponds to y
    //           y1 = height at start of a shape curve interval
    //           w1 = width at start of a shape curve interval
    //  Output:  returns the area within the trapezoid formed by the input points
    //  Purpose: computes the area of an interval along a x-section's shape curve.
    //
    let wMin: f64;
    let wMax: f64;

    if w > w1 {
        wMin = w1;
        wMax = w;
    } else {
        wMin = w;
        wMax = w1;
    }

    (wMin + (wMax - wMin) / 2.0) * (y - y1)
}

//=============================================================================

fn getPerim(y: f64, w: f64, y1: f64, w1: f64) -> f64 {
    //
    //  Input:   y = height along a shape curve
    //           w = width that corresponds to y
    //           y1 = height at start of a shape curve interval
    //           w1 = width at start of a shape curve interval
    //  Output:  returns the length of the sides of the trapezoid formed by the
    //           input points
    //  Purpose: computes the length of the wetted perimeter contributed by an
    //           interval along a x-section's shape curve.
    //
    let dy: f64 = y - y1;
    let dw: f64 = (w - w1).abs() / 2.0;

    2.0 * (dy * dy + dw * dw).sqrt()
}
