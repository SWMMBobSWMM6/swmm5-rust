//! treatmnt.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   treatmnt.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21   (Build 5.2.0)
//   Author:   L. Rossman
//
//   Pollutant treatment functions.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - A bug in evaluating recursive calls to treatment functions was fixed.
//   Build 5.2.0:
//   - Changed enumerated constant used to indicate a math expression error.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const PVMAX: i32 = 5; // number of process variables
// enum   ProcessVarType
const pvHRT: i32 = 0; // hydraulic residence time
const pvDT: i32 = 1; // time step duration
const pvFLOW: i32 = 2; // flow rate
const pvDEPTH: i32 = 3; // water height above invert
const pvAREA: i32 = 4; // storage surface area

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of treatmnt.c.
#[derive(Default)]
pub struct TreatmntState {
    ErrCode: i32,  // treatment error code
    J: i32,        // index of node being analyzed
    Dt: f64,       // curent time step (sec)
    Q: f64,        // node inflow (cfs)
    V: f64,        // node volume (ft3)
    R: Vec<f64>,   // array of pollut. removals
    Cin: Vec<f64>, // node inflow concentrations
}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  treatmnt_open           (called from routing_open)
//  treatment_close         (called from routing_close)
//  treatmnt_readExpression (called from parseLine in input.c)
//  treatmnt_delete         (called from deleteObjects in project.c)
//  treatmnt_setInflow      (called from qualrout_execute)
//  treatmnt_treat          (called from findNodeQual in qualrout.c)

//=============================================================================

/// Called by: routing.
pub fn treatmnt_open(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns TRUE if successful, FALSE if not
//  Purpose: allocates memory for computing pollutant removals by treatment.
//
{
    prj.treatmnt.R = Vec::new();
    prj.treatmnt.Cin = Vec::new();
    if prj.Nobjects[POLLUT as usize] > 0 {
        let n = prj.Nobjects[POLLUT as usize] as usize;
        prj.treatmnt.R = vec![0.0; n];
        prj.treatmnt.Cin = vec![0.0; n];
    }
    TRUE
}

//=============================================================================

/// Called by: routing.
pub fn treatmnt_close(prj: &mut Project)
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: frees memory used for computing pollutant removals by treatment.
//
{
    prj.treatmnt.R = Vec::new();
    prj.treatmnt.Cin = Vec::new();
}

//=============================================================================

/// Called by: input.
pub fn treatmnt_readExpression(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads a treatment expression from a tokenized line of input.
//
{
    let mut s: [u8; MAXLINE as usize + 1] = [0; MAXLINE as usize + 1];
    let j: i32;
    let k: i32;
    let p: i32;

    // --- retrieve node & pollutant
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    j = project_findObject(prj, NODE, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    p = project_findObject(prj, POLLUT, &tok[1]);
    if p < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }

    // --- concatenate remaining tokens into a single string
    sstrncpy(&mut s, &tok[2], MAXLINE as usize);
    for i in 3..ntoks as usize {
        sstrcat(&mut s, b" ", MAXLINE as usize);
        sstrcat(&mut s, &tok[i], MAXLINE as usize);
    }

    // --- check treatment type
    if UCHAR(s[0]) == b'R' {
        k = 0;
    } else if UCHAR(s[0]) == b'C' {
        k = 1;
    } else {
        return error_setInpError(prj, ERR_KEYWORD, &tok[2]);
    }

    // --- start treatment expression after equals sign
    let expr: &[u8] = match cio::strchr(&s, b'=') {
        None => return error_setInpError(prj, ERR_KEYWORD, b""),
        Some(n) => cio::cstr(&s[n + 1..]),
    };

    // --- create treatment objects at node j if they don't already exist
    if prj.Node[j as usize].treatment.is_empty() {
        if createTreatment(prj, j) == 0 {
            return error_setInpError(prj, ERR_MEMORY, b"");
        }
    }

    // --- create a parsed expression tree from the string expr
    //     (getVariableIndex is the function that converts a treatment
    //      variable's name into an index number)
    let equation: Option<MathExpr>; // ptr. to a math. expression
    {
        let prj_r: &Project = prj;
        equation = mathexpr_create(expr, &mut |name: &[u8]| getVariableIndex(prj_r, name));
    }
    let equation = match equation {
        None => return error_setInpError(prj, ERR_MATH_EXPR, b""),
        Some(e) => e,
    };

    // --- save the treatment parameters in the node's treatment object
    //     (C overwrites, and leaks, any equation already there)
    if !prj.Node[j as usize].treatment.is_empty() {
        prj.Node[j as usize].treatment[p as usize].treatType = k;
        prj.Node[j as usize].treatment[p as usize].equation = Some(equation);
    }
    0
}

//=============================================================================

/// Called by: project.
pub fn treatmnt_delete(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: deletes the treatment objects for each pollutant at a node.
//
{
    if !prj.Node[j as usize].treatment.is_empty() {
        for p in 0..prj.Nobjects[POLLUT as usize] as usize {
            let eq = prj.Node[j as usize].treatment[p].equation.take();
            mathexpr_delete(eq);
        }
    }
    prj.Node[j as usize].treatment = Vec::new();
}

//=============================================================================

/// Called by: qualrout.
pub fn treatmnt_setInflow(prj: &mut Project, qIn: f64, wIn: &[f64])
//
//  Input:   j = node index
//           qIn = flow inflow rate (cfs)
//           wIn = pollutant mass inflow rate (mass/sec)
//  Output:  none
//  Purpose: computes and saves array of inflow concentrations to a node.
//
{
    let np = prj.Nobjects[POLLUT as usize] as usize;
    if qIn > 0.0 {
        for p in 0..np {
            prj.treatmnt.Cin[p] = wIn[p] / qIn;
        }
    } else {
        for p in 0..np {
            prj.treatmnt.Cin[p] = 0.0;
        }
    }
}

//=============================================================================

/// Called by: qualrout.
pub fn treatmnt_treat(prj: &mut Project, j: i32, q: f64, v: f64, tStep: f64)
//
//  Input:   j     = node index
//           q     = inflow to node (cfs)
//           v     = volume of node (ft3)
//           tStep = routing time step (sec)
//  Output:  none
//  Purpose: updates pollutant concentrations at a node after treatment.
//
{
    let mut cOut: f64; // concentration after treatment
    let mut massLost: f64; // mass lost by treatment per time step
    let ju = j as usize;
    let np = prj.Nobjects[POLLUT as usize] as usize;

    // --- set locally shared variables for node j
    if prj.Node[ju].treatment.is_empty() {
        return;
    }
    prj.treatmnt.ErrCode = 0;
    prj.treatmnt.J = j; // current node
    prj.treatmnt.Dt = tStep; // current time step
    prj.treatmnt.Q = q; // current inflow rate
    prj.treatmnt.V = v; // current node volume

    // --- initialze each removal to indicate no value
    for p in 0..np {
        prj.treatmnt.R[p] = -1.0;
    }

    // --- determine removal of each pollutant
    for p in 0..np {
        // --- removal is zero if there is no treatment equation
        let treatment = &prj.Node[ju].treatment[p];
        if treatment.equation.is_none() {
            prj.treatmnt.R[p] = 0.0;
        }
        // --- no removal for removal-type expression when there is no inflow
        else if treatment.treatType == REMOVAL && q <= ZERO {
            prj.treatmnt.R[p] = 0.0;
        }
        // --- otherwise evaluate the treatment expression to find R[p]
        else {
            getRemoval(prj, p as i32);
        }
    }

    // --- check for error condition
    //     (getRemoval sets ErrCode to 1, never to ERR_CYCLIC_TREATMENT,
    //      so as in C this branch is never taken)
    if prj.treatmnt.ErrCode == ERR_CYCLIC_TREATMENT {
        let id = prj.Node[prj.treatmnt.J as usize].ID.clone();
        report_writeErrorMsg(prj, ERR_CYCLIC_TREATMENT, &id);
    }
    // --- update nodal concentrations and mass balances
    else {
        for p in 0..np {
            if prj.treatmnt.R[p] == 0.0 {
                continue;
            }
            let treatType = prj.Node[ju].treatment[p].treatType;

            // --- removal-type treatment equations get applied to inflow stream

            if treatType == REMOVAL {
                // --- if no pollutant in inflow then cOut is current nodal concen.
                if prj.treatmnt.Cin[p] == 0.0 {
                    cOut = prj.Node[ju].newQual[p];
                }
                // ---  otherwise apply removal to influent concen.
                else {
                    cOut = (1.0 - prj.treatmnt.R[p]) * prj.treatmnt.Cin[p];
                }

                // --- cOut can't be greater than mixture concen. at node
                //     (i.e., in case node is a storage unit)
                cOut = MIN(cOut, prj.Node[ju].newQual[p]);
            }
            // --- concentration-type equations get applied to nodal concentration
            else {
                cOut = (1.0 - prj.treatmnt.R[p]) * prj.Node[ju].newQual[p];
            }

            // --- mass lost must account for any initial mass in storage
            massLost = (prj.treatmnt.Cin[p] * q * tStep + prj.Node[ju].oldQual[p] * prj.Node[ju].oldVolume
                - cOut * (q * tStep + prj.Node[ju].oldVolume))
                / tStep;
            massLost = MAX(0.0, massLost);

            // --- add mass loss to mass balance totals and revise nodal concentration
            massbal_addReactedMass(prj, p as i32, massLost);
            prj.Node[ju].newQual[p] = cOut;
        }
    }
}

//=============================================================================

fn createTreatment(prj: &mut Project, j: i32) -> i32
//
//  Input:   j = node index
//  Output:  returns TRUE if successful, FALSE if not
//  Purpose: creates a treatment object for each pollutant at a node.
//
{
    let np = prj.Nobjects[POLLUT as usize] as usize;
    prj.Node[j as usize].treatment = vec![TTreatment::default(); np];
    for p in 0..np {
        prj.Node[j as usize].treatment[p].equation = None;
    }
    TRUE
}

//=============================================================================

fn getVariableIndex(prj: &Project, s: &[u8]) -> i32
//
//  Input:   s = name of a process variable or pollutant
//  Output:  returns index of process variable or pollutant
//  Purpose: finds position of process variable/pollutant in list of names.
//
{
    // --- check for a process variable first
    let mut k: i32;
    let m: i32 = PVMAX; // PVMAX is number of process variables

    k = findmatch(s, ProcessVarWords);
    if k >= 0 {
        return k;
    }

    // --- then check for a pollutant concentration
    k = project_findObject(prj, POLLUT, s);
    if k >= 0 {
        return k + m;
    }

    // --- finally check for a pollutant removal
    let at = |i: usize| -> u8 { s.get(i).copied().unwrap_or(0) };
    if UCHAR(at(0)) == b'R' && at(1) == b'_' {
        k = project_findObject(prj, POLLUT, &s[2..]);
        if k >= 0 {
            return prj.Nobjects[POLLUT as usize] + k + m;
        }
    }
    -1
}

//=============================================================================

fn getVariableValue(prj: &mut Project, varCode: i32) -> f64
//
//  Input:   varCode = code number of process variable or pollutant
//  Output:  returns current value of variable
//  Purpose: finds current value of a process variable or pollutant concen.,
//           making reference to the node being evaluated which is stored in
//           shared variable J.
//
{
    let p: i32;
    let a1: f64;
    let a2: f64;
    let y: f64;
    let J = prj.treatmnt.J;
    let Ju = J as usize;

    // --- variable is a process variable
    if varCode < PVMAX {
        match varCode {
            pvHRT => {
                // HRT in hours
                if prj.Node[Ju].type_ == STORAGE {
                    prj.Storage[prj.Node[Ju].subIndex as usize].hrt / 3600.0
                } else {
                    0.0
                }
            }

            pvDT => prj.treatmnt.Dt, // time step in seconds

            pvFLOW => prj.treatmnt.Q * UCF(prj, FLOW), // flow in user's units

            pvDEPTH => {
                y = (prj.Node[Ju].oldDepth + prj.Node[Ju].newDepth) / 2.0;
                y * UCF(prj, LENGTH) // depth in ft or m
            }

            pvAREA => {
                let d1 = prj.Node[Ju].oldDepth;
                a1 = node_getSurfArea(prj, J, d1);
                let d2 = prj.Node[Ju].newDepth;
                a2 = node_getSurfArea(prj, J, d2);
                (a1 + a2) / 2.0 * UCF(prj, LENGTH) * UCF(prj, LENGTH)
            }

            _ => 0.0,
        }
    }
    // --- variable is a pollutant concentration
    else if varCode < PVMAX + prj.Nobjects[POLLUT as usize] {
        p = varCode - PVMAX;
        let treatment = &prj.Node[Ju].treatment[p as usize];
        if treatment.treatType == REMOVAL {
            return prj.treatmnt.Cin[p as usize];
        }
        prj.Node[Ju].newQual[p as usize]
    }
    // --- variable is a pollutant removal
    else {
        p = varCode - PVMAX - prj.Nobjects[POLLUT as usize];
        if p >= prj.Nobjects[POLLUT as usize] {
            return 0.0;
        }
        getRemoval(prj, p)
    }
}

//=============================================================================

fn getRemoval(prj: &mut Project, p: i32) -> f64
//
//  Input:   p = pollutant index
//  Output:  returns fractional removal of pollutant
//  Purpose: computes removal of a specific pollutant
//
{
    let pu = p as usize;
    let Ju = prj.treatmnt.J as usize;
    let c0 = prj.Node[Ju].newQual[pu]; // initial node concentration
    let mut r: f64; // removal value

    // --- case where removal already being computed for another pollutant
    if prj.treatmnt.R[pu] > 1.0 || prj.treatmnt.ErrCode != 0 {
        prj.treatmnt.ErrCode = 1;
        return 0.0;
    }

    // --- case where removal already computed
    if prj.treatmnt.R[pu] >= 0.0 && prj.treatmnt.R[pu] <= 1.0 {
        return prj.treatmnt.R[pu];
    }

    // --- set R[p] to value > 1 to show that value is being sought
    //     (prevents infinite recursive calls in case two removals
    //     depend on each other)
    prj.treatmnt.R[pu] = 10.0;

    // --- case where current concen. is zero
    if c0 == 0.0 {
        prj.treatmnt.R[pu] = 0.0;
        return 0.0;
    }

    // --- apply treatment eqn.
    //     (the expression is taken out of the node for the evaluation so the
    //      evaluator's callback can use the model, and put back after; no
    //      nested call reads it: a nested getRemoval(p) returns early since
    //      R[p] > 1. A pollutant with no equation, reached through an R_name
    //      variable, evaluates an empty list, as C evaluates NULL, to 0.)
    let equation: Option<MathExpr> = prj.Node[Ju].treatment[pu].equation.take();
    {
        let empty: MathExpr = Vec::new();
        let e: &MathExpr = match equation.as_ref() {
            Some(e) => e,
            None => &empty,
        };
        r = mathexpr_eval(e, &mut |varCode: i32| getVariableValue(prj, varCode));
    }
    prj.Node[Ju].treatment[pu].equation = equation;
    r = MAX(0.0, r);

    // --- case where treatment eqn. is for removal
    if prj.Node[Ju].treatment[pu].treatType == REMOVAL {
        r = MIN(1.0, r);
        prj.treatmnt.R[pu] = r;
    }
    // --- case where treatment eqn. is for effluent concen.
    else {
        r = MIN(c0, r);
        prj.treatmnt.R[pu] = 1.0 - r / c0;
    }
    prj.treatmnt.R[pu]
}
