//! exfil.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   exfil.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//
//   Storage unit exfiltration functions.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Monthly conductivity adjustment applied to exfiltration rate.
//   Build 5.1.010:
//   - New modified Green-Ampt infiltration option used.
//   Build 5.1.011:
//   - Fixed units conversion error for storage units with surface area curves.
//   Build 5.2.0:
//   - Support added for analytical storage shapes.
//-----------------------------------------------------------------------------

use crate::prelude::*;

/// The file-scope variables of exfil.c (it has none).
#[derive(Default)]
pub struct ExfilState {
}

//=============================================================================

/// Called by: node.
pub fn exfil_readStorageParams(prj: &mut Project, k: i32, tok: &[Vec<u8>], ntoks: i32, n: i32) -> i32
//
//  Input:   k = storage unit index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//           n = last token processed
//  Output:  returns an error code
//  Purpose: reads a storage unit's exfiltration parameters from a
//           tokenized line of input.
//
{
    let mut i: i32;
    let mut x: [f64; 3] = [0.0; 3]; //suction head, Ksat, IMDmax

    // --- read Ksat if it's the only remaining token
    if ntoks == n + 1 {
        if getDouble(&tok[n as usize], &mut x[1]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[n as usize]);
        }
        x[0] = 0.0;
        x[2] = 0.0;
    }
    // --- otherwise read Green-Ampt infiltration parameters from input tokens
    else if ntoks < n + 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    } else {
        i = 0;
        while i < 3 {
            if getDouble(&tok[(n + i) as usize], &mut x[i as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[(n + i) as usize]);
            }
            i += 1;
        }
    }

    // --- no exfiltration if Ksat is 0
    if x[1] == 0.0 {
        return 0;
    }

    // --- create an exfiltration object
    createStorageExfil(prj, k, &x)
}

//=============================================================================

/// Called by: node.
pub fn exfil_initState(prj: &mut Project, k: i32)
//
//  Input:   k = storage unit index
//  Output:  none
//  Purpose: initializes the state of a storage unit's exfiltration object.
//
{
    let ku = k as usize;
    let i: i32;
    let mut a: f64 = 0.0;
    let mut alast: f64;
    let mut d: f64 = 0.0;

    // --- initialize exfiltration object
    //     (taken out of Storage[k] while it is worked on, and put back;
    //      nothing below reads Storage[k].exfil through prj)
    let mut exfilBox = prj.Storage[ku].exfil.take();
    if let Some(exfil) = exfilBox.as_deref_mut() {
        // --- initialize the Green-Ampt infil. parameters
        //     (grnampt_initState returns at once for a NULL pointer)
        if let Some(g) = exfil.btmExfil.as_deref_mut() {
            grnampt_initState(g);
        }
        if let Some(g) = exfil.bankExfil.as_deref_mut() {
            grnampt_initState(g);
        }

        match prj.Storage[ku].shape {
            // --- shape given by a Storage Curve
            TABULAR => {
                i = prj.Storage[ku].aCurve;
                exfil.btmArea = 0.0;
                exfil.bankMinDepth = 0.0;
                exfil.bankMaxDepth = 0.0;
                exfil.bankMaxArea = 0.0;
                if i >= 0 {
                    // --- get bottom area
                    let aCurve: &mut TTable = &mut prj.Curve[i as usize];
                    exfil.btmArea = table_lookupEx(aCurve, 0.0);

                    // --- find min/max bank depths and max. bank area
                    table_getFirstEntry(aCurve, &mut d, &mut a);
                    alast = a;
                    while table_getNextEntry(aCurve, &mut d, &mut a) != 0 {
                        if a < alast {
                            break;
                        } else if a > alast {
                            exfil.bankMaxArea = a;
                            exfil.bankMaxDepth = d;
                        } else if exfil.bankMaxArea == 0.0 {
                            exfil.bankMinDepth = d;
                        } else {
                            break;
                        }
                        alast = a;
                    }

                    // --- convert from user units to internal units
                    exfil.btmArea /= UCF(prj, LENGTH) * UCF(prj, LENGTH);
                    exfil.bankMaxArea /= UCF(prj, LENGTH) * UCF(prj, LENGTH);
                    exfil.bankMinDepth /= UCF(prj, LENGTH);
                    exfil.bankMaxDepth /= UCF(prj, LENGTH);
                }
            }

            // --- functional storage shape curve
            FUNCTIONAL => {
                exfil.btmArea = prj.Storage[ku].a0;
                if prj.Storage[ku].a2 == 0.0 {
                    exfil.btmArea += prj.Storage[ku].a1;
                }
                exfil.bankMinDepth = 0.0;
                exfil.bankMaxDepth = BIG;
                exfil.bankMaxArea = BIG;
            }

            // --- cylindrical, conical & prismatic shapes
            CYLINDRICAL | CONICAL | PYRAMIDAL => {
                exfil.btmArea = prj.Storage[ku].a0;
                exfil.bankMinDepth = 0.0;
                exfil.bankMaxDepth = BIG;
                exfil.bankMaxArea = BIG;
            }

            // C: no case for PARABOLOID; the fields keep whatever malloc
            // gave them (here: zero, see createStorageExfil)
            _ => {}
        }
    }
    prj.Storage[ku].exfil = exfilBox;
}

//=============================================================================

/// Called by: node.
pub fn exfil_getLoss(
    ist: &mut crate::infil::InfilState,
    adjust: &TAdjust,
    exfil: &mut TExfil,
    tStep: f64,
    mut depth: f64,
    mut area: f64,
) -> f64
//
//  Input:   exfil = ptr. to a storage exfiltration object
//           tStep = time step (sec)
//           depth = water depth (ft)
//           area = surface area (ft2)
//  Output:  returns exfiltration rate out of storage unit (cfs)
//  Purpose: computes rate of water exfiltrated from a storage node into
//           the soil beneath it.
//
//  (adjust is the global Adjust, whose hydconFactor C reads here)
{
    let mut exfilRate: f64;

    // --- find infiltration through bottom of unit
    if exfil.btmExfil.as_deref().unwrap().IMDmax == 0.0 {
        exfilRate = exfil.btmExfil.as_deref().unwrap().Ks * adjust.hydconFactor;
    } else {
        exfilRate = grnampt_getInfil(
            ist,
            exfil.btmExfil.as_deref_mut().unwrap(),
            tStep,
            0.0,
            depth,
            MOD_GREEN_AMPT,
        );
    }
    exfilRate *= exfil.btmArea;

    // --- find infiltration through sloped banks
    if depth > exfil.bankMinDepth {
        // --- get area of banks
        area = MIN(area, exfil.bankMaxArea) - exfil.btmArea;
        if area > 0.0 {
            // --- if infil. rate not a function of depth
            if exfil.btmExfil.as_deref().unwrap().IMDmax == 0.0 {
                exfilRate += area * exfil.btmExfil.as_deref().unwrap().Ks * adjust.hydconFactor;
            }
            // --- infil. rate depends on depth above bank
            else {
                // --- case where water depth is above the point where the
                //     storage curve no longer has increasing area with depth
                if depth > exfil.bankMaxDepth {
                    depth = depth - exfil.bankMaxDepth + (exfil.bankMaxDepth - exfil.bankMinDepth) / 2.0;
                }
                // --- case where water depth is below top of bank
                else {
                    depth = (depth - exfil.bankMinDepth) / 2.0;
                }

                // --- use Green-Ampt function for bank infiltration
                exfilRate += area
                    * grnampt_getInfil(
                        ist,
                        exfil.bankExfil.as_deref_mut().unwrap(),
                        tStep,
                        0.0,
                        depth,
                        MOD_GREEN_AMPT,
                    );
            }
        }
    }
    exfilRate
}

//=============================================================================

fn createStorageExfil(prj: &mut Project, k: i32, x: &[f64]) -> i32
//
//  Input:   k = index of storage unit node
//           x = array of Green-Ampt infiltration parameters
//  Output:  returns an error code.
//  Purpose: creates an exfiltration object for a storage node.
//
//  Note: the exfiltration object is freed in project.c.
//
{
    let ku = k as usize;

    // --- create an exfiltration object for the storage node
    //     (C mallocs it uninitialized; its area/depth fields are set by
    //      exfil_initState, except for a PARABOLOID shape: zero here)
    if prj.Storage[ku].exfil.is_none() {
        let mut exfil = TExfil::default();

        // --- create Green-Ampt infiltration objects for the bottom & banks
        exfil.btmExfil = None;
        exfil.bankExfil = None;
        exfil.btmExfil = Some(Box::new(TGrnAmpt::default()));
        exfil.bankExfil = Some(Box::new(TGrnAmpt::default()));
        prj.Storage[ku].exfil = Some(Box::new(exfil));
    }

    // --- initialize the Green-Ampt parameters
    //     (each TGrnAmpt is copied out, set, and written back, so no borrow
    //      of prj is held across the call)
    let mut btm: TGrnAmpt = **prj.Storage[ku].exfil.as_ref().unwrap().btmExfil.as_ref().unwrap();
    let ok = grnampt_setParams(&mut btm, x);
    **prj.Storage[ku].exfil.as_mut().unwrap().btmExfil.as_mut().unwrap() = btm;
    if ok == 0 {
        return error_setInpError(prj, ERR_NUMBER, b"");
    }
    let mut bank: TGrnAmpt = **prj.Storage[ku].exfil.as_ref().unwrap().bankExfil.as_ref().unwrap();
    grnampt_setParams(&mut bank, x);
    **prj.Storage[ku].exfil.as_mut().unwrap().bankExfil.as_mut().unwrap() = bank;
    0
}
