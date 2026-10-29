//! dwflow.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   dwflow.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     06/12/23  (Build 5.2.4)
//   Author:   L. Rossman
//             M. Tryby (EPA)
//             R. Dickinson (CDM)
//
//   Solves the momentum equation for flow in a conduit under dynamic wave
//   flow routing.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Bug in finding if conduit was upstrm/dnstrm full was fixed.
//   Build 5.1.012:
//   - Modified uniform loss rate term of conduit momentum equation.
//   Build 5.1.013:
//   - Preissmann slot surcharge option implemented.
//   - Changed sign of uniform loss rate term (dq6) in flow updating equation.
//   Build 5.1.014:
//   - Conduit evap. and seepage loss initialized to 0 in dwflow_findConduitFlow.
//   - Most current flow (qLast) used instead of previous time period flow
//     (qOld) in call to link_getLossRate.
//   Build 5.2.1:
//   - Implements the new option to skip checking for normal flow limitations.
//   Build 5.2.4:
//   - Arguments to function link_getLossRate changed.
//-----------------------------------------------------------------------------

use crate::prelude::*;

const MAXVELOCITY: f64 = 50.;     // max. allowable velocity (ft/sec)

/// The file-scope variables of dwflow.c.
#[derive(Default)]
pub struct DwflowState {
}

//=============================================================================

/// Called by: dynwave.
pub fn dwflow_findConduitFlow(prj: &mut Project, j: i32, steps: i32, omega: f64, dt: f64) {
//
//  Input:   j        = link index
//           steps    = number of iteration steps taken
//           omega    = under-relaxation parameter
//           dt       = time step (sec)
//  Output:  returns new flow value (cfs)
//  Purpose: updates flow in conduit link by solving finite difference
//           form of continuity and momentum equations.
//
    let k: i32;                        // index of conduit
    let n1: i32;                       // indexes of end nodes
    let n2: i32;
    let z1: f64;                       // upstream/downstream invert elev. (ft)
    let z2: f64;
    let mut h1: f64;                   // upstream/dounstream flow heads (ft)
    let mut h2: f64;
    let mut y1: f64;                   // upstream/downstream flow depths (ft)
    let mut y2: f64;
    let a1: f64;                       // upstream/downstream flow areas (ft2)
    let a2: f64;
    let r1: f64;                       // upstream hyd. radius (ft)
    let yMid: f64;                     // mid-stream or avg. values of y, r, & a
    let rMid: f64;
    let mut aMid: f64;
    let aWtd: f64;                     // upstream weighted area & hyd. radius
    let rWtd: f64;
    let qLast: f64;                    // flow from previous iteration (cfs)
    let qOld: f64;                     // flow from previous time step (cfs)
    let mut aOld: f64;                 // area from previous time step (ft2)
    let mut v: f64;                    // velocity (ft/sec)
    let mut rho: f64;                  // upstream weighting factor
    let mut sigma: f64;                // inertial damping factor
    let length: f64;                   // effective conduit length (ft)
    let mut wSlot: f64;                // Preissmann slot width (ft)
    let dq1: f64;                      // terms in momentum eqn.
    let dq2: f64;
    let mut dq3: f64;
    let mut dq4: f64;
    let mut dq5: f64;
    let dq6: f64;                      // term for evap and infil losses
    let denom: f64;                    // denominator of flow update formula
    let mut q: f64;                    // new flow value (cfs)
    let barrels: f64;                  // number of barrels in conduit
    let ju = j as usize;
    // C: TXsect* xsect = &Link[j].xsect; nothing below changes Link[j].xsect,
    // so a copy reads the same values.
    let xsect: TXsect = prj.Link[ju].xsect; // ptr. to conduit's cross section data
    let mut isFull: i8 = FALSE as i8;  // TRUE if conduit flowing full
    let mut isClosed: i8 = FALSE as i8; // TRUE if conduit closed

    // --- adjust isClosed status by any control action
    if prj.Link[ju].setting == 0 as f64 {
        isClosed = TRUE as i8;
    }

    // --- get flow from last time step & previous iteration
    k = prj.Link[ju].subIndex;
    let ku = k as usize;
    barrels = prj.Conduit[ku].barrels as f64;
    qOld = prj.Link[ju].oldFlow / barrels;
    qLast = prj.Conduit[ku].q1;
    prj.Conduit[ku].evapLossRate = 0.0;
    prj.Conduit[ku].seepLossRate = 0.0;

    // --- get most current heads at upstream and downstream ends of conduit
    n1 = prj.Link[ju].node1;
    n2 = prj.Link[ju].node2;
    z1 = prj.Node[n1 as usize].invertElev + prj.Link[ju].offset1;
    z2 = prj.Node[n2 as usize].invertElev + prj.Link[ju].offset2;
    h1 = prj.Node[n1 as usize].newDepth + prj.Node[n1 as usize].invertElev;
    h2 = prj.Node[n2 as usize].newDepth + prj.Node[n2 as usize].invertElev;
    h1 = MAX(h1, z1);
    h2 = MAX(h2, z2);

    // --- get unadjusted upstream and downstream flow depths in conduit
    //    (flow depth = head in conduit - elev. of conduit invert)
    y1 = h1 - z1;
    y2 = h2 - z2;
    y1 = MAX(y1, FUDGE);
    y2 = MAX(y2, FUDGE);

    // --- flow depths can't exceed full depth of conduit if slot not used
    if prj.SurchargeMethod != SLOT {
        y1 = MIN(y1, xsect.yFull);
        y2 = MIN(y2, xsect.yFull);
    }

    // -- get area from solution at previous time step
    aOld = prj.Conduit[ku].a2;
    aOld = MAX(aOld, FUDGE);

    // --- use Courant-modified length instead of conduit's actual length
    length = prj.Conduit[ku].modLength;

    // --- find surface area contributions to upstream and downstream nodes
    //     based on previous iteration's flow estimate
    findSurfArea(prj, j, qLast, length, &mut h1, &mut h2, &mut y1, &mut y2);

    // --- compute area at each end of conduit & hyd. radius at upstream end
    wSlot = getSlotWidth(prj, &xsect, y1);
    a1 = getArea(prj, &xsect, y1, wSlot);
    r1 = getHydRad(prj, &xsect, y1);
    wSlot = getSlotWidth(prj, &xsect, y2);
    a2 = getArea(prj, &xsect, y2, wSlot);

    // --- compute area & hyd. radius at midpoint
    yMid = 0.5 * (y1 + y2);
    wSlot = getSlotWidth(prj, &xsect, yMid);
    aMid = getArea(prj, &xsect, yMid, wSlot);
    rMid = getHydRad(prj, &xsect, yMid);

    // --- alternate approach not currently used, but might produce better
    //     Bernoulli energy balance for steady flows
    //aMid = (a1+a2)/2.0;
    //rMid = (r1+getHydRad(xsect,y2))/2.0;

    // --- check if conduit is flowing full
    if y1 >= xsect.yFull && y2 >= xsect.yFull {
        isFull = TRUE as i8;
    }

    // --- set new flow to zero if conduit is dry or if flap gate is closed
    if prj.Link[ju].flowClass == DRY
        || prj.Link[ju].flowClass == UP_DRY
        || prj.Link[ju].flowClass == DN_DRY
        || isClosed != 0
        || aMid <= FUDGE
    {
        prj.Conduit[ku].a1 = 0.5 * (a1 + a2);
        prj.Conduit[ku].q1 = 0.0;
        prj.Conduit[ku].q2 = 0.0;
        prj.Link[ju].dqdh = GRAVITY * dt * aMid / length * barrels;
        prj.Link[ju].froude = 0.0;
        prj.Link[ju].newDepth = MIN(yMid, prj.Link[ju].xsect.yFull);
        prj.Link[ju].newVolume = prj.Conduit[ku].a1 * link_getLength(prj, j) * barrels;
        prj.Link[ju].newFlow = 0.0;
        return;
    }

    // --- compute velocity from last flow estimate
    v = qLast / aMid;
    if v.abs() > MAXVELOCITY {
        v = MAXVELOCITY * SGN(qLast);
    }

    // --- compute Froude No.
    prj.Link[ju].froude = link_getFroude(prj, j, v, yMid);
    if prj.Link[ju].flowClass == SUBCRITICAL && prj.Link[ju].froude > 1.0 {
        prj.Link[ju].flowClass = SUPCRITICAL;
    }

    // --- find inertial damping factor (sigma)
    if prj.Link[ju].froude <= 0.5 {
        sigma = 1.0;
    } else if prj.Link[ju].froude >= 1.0 {
        sigma = 0.0;
    } else {
        sigma = 2.0 * (1.0 - prj.Link[ju].froude);
    }

    // --- get upstream-weighted area & hyd. radius based on damping factor
    //     (modified version of R. Dickinson's slope weighting)
    rho = 1.0;
    if isFull == 0 && qLast > 0.0 && h1 >= h2 {
        rho = sigma;
    }
    aWtd = a1 + (aMid - a1) * rho;
    rWtd = r1 + (rMid - r1) * rho;

    // --- determine how much inertial damping to apply
    if prj.InertDamping == NO_DAMPING {
        sigma = 1.0;
    } else if prj.InertDamping == FULL_DAMPING {
        sigma = 0.0;
    }

    // --- use full inertial damping if closed conduit is surcharged
    if isFull != 0 && xsect_isOpen(xsect.type_) == 0 {
        sigma = 0.0;
    }

    // --- compute terms of momentum eqn.:
    // --- 1. friction slope term
    if xsect.type_ == FORCE_MAIN && isFull != 0 {
        dq1 = dt * forcemain_getFricSlope(prj, j, v.abs(), rMid);
    } else {
        dq1 = dt * prj.Conduit[ku].roughFactor / rWtd.powf(1.33333) * v.abs();
    }

    // --- 2. energy slope term
    dq2 = dt * GRAVITY * aWtd * (h2 - h1) / length;

    // --- 3 & 4. inertial terms
    dq3 = 0.0;
    dq4 = 0.0;
    if sigma > 0.0 {
        dq3 = 2.0 * v * (aMid - aOld) * sigma;
        dq4 = dt * v * v * (a2 - a1) / length * sigma;
    }

    // --- 5. local losses term
    dq5 = 0.0;
    if prj.Conduit[ku].hasLosses != 0 {
        dq5 = findLocalLosses(prj, j, a1, a2, aMid, qLast) / 2.0 / length * dt;
    }

    // --- 6. term for evap and seepage losses per unit length
    dq6 = link_getLossRate(prj, j, DW, qLast, dt) * 2.5 * dt * v / link_getLength(prj, j);

    // --- combine terms to find new conduit flow
    denom = 1.0 + dq1 + dq5;
    q = (qOld - dq2 + dq3 + dq4 + dq6) / denom;

    // --- compute derivative of flow w.r.t. head
    prj.Link[ju].dqdh = 1.0 / denom * GRAVITY * dt * aWtd / length * barrels;

    // --- check if any flow limitation applies
    prj.Link[ju].inletControl = FALSE as i8;
    prj.Link[ju].normalFlow = FALSE as i8;
    if q > 0.0 {
        // --- check for inlet controlled culvert flow
        if xsect.culvertCode > 0 && isFull == 0 {
            q = culvert_getInflow(prj, j, q, h1);
        }
        // --- check for normal flow limitation based on surface slope & Fr
        else if prj.NormalFlowLtd != NEITHER
            && y1 < prj.Link[ju].xsect.yFull
            && (prj.Link[ju].flowClass == SUBCRITICAL || prj.Link[ju].flowClass == SUPCRITICAL)
        {
            q = checkNormalFlow(prj, j, q, y1, y2, a1, r1);
        }
    }

    // --- apply under-relaxation weighting between new & old flows;
    // --- do not allow change in flow direction without first being zero
    if steps > 0 {
        q = (1.0 - omega) * qLast + omega * q;
        if q * qLast < 0.0 {
            q = 0.001 * SGN(q);
        }
    }

    // --- check if user-supplied flow limit applies
    if prj.Link[ju].qLimit > 0.0 {
        if q.abs() > prj.Link[ju].qLimit {
            q = SGN(q) * prj.Link[ju].qLimit;
        }
    }

    // --- check for reverse flow with closed flap gate
    if link_setFlapGate(prj, j, n1, n2, q) != 0 {
        q = 0.0;
    }

    // --- do not allow flow out of a dry node
    //     (as suggested by R. Dickinson)
    if q > FUDGE && prj.Node[n1 as usize].newDepth <= FUDGE {
        q = FUDGE;
    }
    if q < -FUDGE && prj.Node[n2 as usize].newDepth <= FUDGE {
        q = -FUDGE;
    }

    // --- save new values of area, flow, depth, & volume
    prj.Conduit[ku].a1 = aMid;
    prj.Conduit[ku].q1 = q;
    prj.Conduit[ku].q2 = q;
    prj.Link[ju].newDepth = MIN(yMid, xsect.yFull);
    aMid = (a1 + a2) / 2.0;
//  aMid = MIN(aMid, xsect->aFull);  //Slot can have aMid > aFull
    prj.Conduit[ku].fullState = link_getFullState(a1, a2, xsect.aFull);
    prj.Link[ju].newVolume = aMid * link_getLength(prj, j) * barrels;
    prj.Link[ju].newFlow = q * barrels;
}

//=============================================================================

fn getFlowClass(prj: &mut Project, j: i32, q: f64, h1: f64, h2: f64, y1: f64, y2: f64,
    yC: &mut f64, yN: &mut f64, fasnh: &mut f64) -> i32 {
//
//  Input:   j  = conduit link index
//           q  = current conduit flow (cfs)
//           h1 = head at upstream end of conduit (ft)
//           h2 = head at downstream end of conduit (ft)
//           y1 = upstream flow depth in conduit (ft)
//           y2 = downstream flow depth in conduit (ft)
//           yC = critical flow depth (ft)
//           yN = normal flow depth (ft)
//           fasnh = fraction between norm. & crit. depth
//  Output:  returns flow classification code
//  Purpose: determines flow class for a conduit based on depths at each end.
//
    let n1: i32;                       // indexes of upstrm/downstrm nodes
    let n2: i32;
    let mut flowClass: i32;            // flow classification code
    let ycMin: f64;                    // min/max critical depths (ft)
    let ycMax: f64;
    let mut z1: f64;                   // offsets of conduit inverts (ft)
    let mut z2: f64;
    let ju = j as usize;

    // --- get upstream & downstream node indexes
    n1 = prj.Link[ju].node1;
    n2 = prj.Link[ju].node2;

    // --- get upstream & downstream conduit invert offsets
    z1 = prj.Link[ju].offset1;
    z2 = prj.Link[ju].offset2;

    // --- base offset of an outfall conduit on outfall's depth
    if prj.Node[n1 as usize].type_ == OUTFALL {
        z1 = MAX(0.0, z1 - prj.Node[n1 as usize].newDepth);
    }
    if prj.Node[n2 as usize].type_ == OUTFALL {
        z2 = MAX(0.0, z2 - prj.Node[n2 as usize].newDepth);
    }

    // --- default class is SUBCRITICAL
    flowClass = SUBCRITICAL;
    *fasnh = 1.0;

    // --- case where both ends of conduit are wet
    if y1 > FUDGE && y2 > FUDGE {
        if q < 0.0 {
            // --- upstream end at critical depth if flow depth is
            //     below conduit's critical depth and an upstream
            //     conduit offset exists
            if z1 > 0.0 {
                *yN = link_getYnorm(prj, j, q.abs());
                *yC = link_getYcrit(prj, j, q.abs());
                ycMin = MIN(*yN, *yC);
                if y1 < ycMin {
                    flowClass = UP_CRITICAL;
                }
            }
        }
        // --- case of normal direction flow
        else {
            // --- downstream end at smaller of critical and normal depth
            //     if downstream flow depth below this and a downstream
            //     conduit offset exists
            if z2 > 0.0 {
                *yN = link_getYnorm(prj, j, q.abs());
                *yC = link_getYcrit(prj, j, q.abs());
                ycMin = MIN(*yN, *yC);
                ycMax = MAX(*yN, *yC);
                if y2 < ycMin {
                    flowClass = DN_CRITICAL;
                } else if y2 < ycMax {
                    if ycMax - ycMin < FUDGE {
                        *fasnh = 0.0;
                    } else {
                        *fasnh = (ycMax - y2) / (ycMax - ycMin);
                    }
                }
            }
        }
    }
    // --- case where no flow at either end of conduit
    else if y1 <= FUDGE && y2 <= FUDGE {
        flowClass = DRY;
    }
    // --- case where downstream end of pipe is wet, upstream dry
    else if y2 > FUDGE {
        // --- flow classification is UP_DRY if downstream head <
        //     invert of upstream end of conduit
        if h2 < prj.Node[n1 as usize].invertElev + prj.Link[ju].offset1 {
            flowClass = UP_DRY;
        }
        // --- otherwise, the downstream head will be >= upstream
        //     conduit invert creating a flow reversal and upstream end
        //     should be at critical depth, providing that an upstream
        //     offset exists (otherwise subcritical condition is maintained)
        else if z1 > 0.0 {
            *yN = link_getYnorm(prj, j, q.abs());
            *yC = link_getYcrit(prj, j, q.abs());
            flowClass = UP_CRITICAL;
        }
    }
    // --- case where upstream end of pipe is wet, downstream dry
    else {
        // --- flow classification is DN_DRY if upstream head <
        //     invert of downstream end of conduit
        if h1 < prj.Node[n2 as usize].invertElev + prj.Link[ju].offset2 {
            flowClass = DN_DRY;
        }
        // --- otherwise flow at downstream end should be at critical depth
        //     providing that a downstream offset exists (otherwise
        //     subcritical condition is maintained)
        else if z2 > 0.0 {
            *yN = link_getYnorm(prj, j, q.abs());
            *yC = link_getYcrit(prj, j, q.abs());
            flowClass = DN_CRITICAL;
        }
    }
    flowClass
}

//=============================================================================

fn findSurfArea(prj: &mut Project, j: i32, q: f64, length: f64, h1: &mut f64,
                h2: &mut f64, y1: &mut f64, y2: &mut f64) {
//
//  Input:   j  = conduit link index
//           q  = current conduit flow (cfs)
//           length = conduit length (ft)
//           h1 = head at upstream end of conduit (ft)
//           h2 = head at downstream end of conduit (ft)
//           y1 = upstream flow depth (ft)
//           y2 = downstream flow depth (ft)
//  Output:  updated values of h1, h2, y1, & y2;
//  Purpose: assigns surface area of conduit to its up and downstream nodes.
//
    let n1: i32;                       // indexes of upstrm/downstrm nodes
    let n2: i32;
    let mut flowDepth1: f64;           // flow depth at upstrm end (ft)
    let mut flowDepth2: f64;           // flow depth at downstrm end (ft)
    let mut flowDepthMid: f64;         // flow depth at midpt. (ft)
    let width1: f64;                   // top width at upstrm end (ft)
    let width2: f64;                   // top width at downstrm end (ft)
    let widthMid: f64;                 // top width at midpt. (ft)
    let mut surfArea1: f64 = 0.0;      // surface area at upstream node (ft2)
    let mut surfArea2: f64 = 0.0;      // surface area st downstrm node (ft2)
    let mut criticalDepth: f64;        // critical flow depth (ft)
    let mut normalDepth: f64;          // normal flow depth (ft)
    let fullDepth: f64;                // full depth (ft)
    let mut fasnh: f64 = 1.0;          // fraction between norm. & crit. depth
    let ju = j as usize;
    let xsect: TXsect = prj.Link[ju].xsect; // pointer to cross-section data

    // --- get node indexes & current flow depths
    n1 = prj.Link[ju].node1;
    n2 = prj.Link[ju].node2;
    flowDepth1 = *y1;
    flowDepth2 = *y2;

    normalDepth = (flowDepth1 + flowDepth2) / 2.0;
    criticalDepth = normalDepth;

    // --- find conduit's flow classification
    fullDepth = xsect.yFull;
    if flowDepth1 >= fullDepth && flowDepth2 >= fullDepth {
        prj.Link[ju].flowClass = SUBCRITICAL;
    } else {
        prj.Link[ju].flowClass = getFlowClass(prj, j, q, *h1, *h2, *y1, *y2,
            &mut criticalDepth, &mut normalDepth, &mut fasnh);
    }

    // --- add conduit's surface area to its end nodes depending on flow class
    match prj.Link[ju].flowClass {
        SUBCRITICAL => {
            flowDepthMid = 0.5 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            width1 = getWidth(prj, &xsect, flowDepth1);
            width2 = getWidth(prj, &xsect, flowDepth2);
            widthMid = getWidth(prj, &xsect, flowDepthMid);
            surfArea1 = (width1 + widthMid) * length / 4.;
            surfArea2 = (widthMid + width2) * length / 4. * fasnh;
        }

        UP_CRITICAL => {
            flowDepth1 = criticalDepth;
            if normalDepth < criticalDepth {
                flowDepth1 = normalDepth;
            }
            flowDepth1 = MAX(flowDepth1, FUDGE);
            *h1 = prj.Node[n1 as usize].invertElev + prj.Link[ju].offset1 + flowDepth1;
            flowDepthMid = 0.5 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            width2 = getWidth(prj, &xsect, flowDepth2);
            widthMid = getWidth(prj, &xsect, flowDepthMid);
            surfArea2 = (widthMid + width2) * length * 0.5;
        }

        DN_CRITICAL => {
            flowDepth2 = criticalDepth;
            if normalDepth < criticalDepth {
                flowDepth2 = normalDepth;
            }
            flowDepth2 = MAX(flowDepth2, FUDGE);
            *h2 = prj.Node[n2 as usize].invertElev + prj.Link[ju].offset2 + flowDepth2;
            width1 = getWidth(prj, &xsect, flowDepth1);
            flowDepthMid = 0.5 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            widthMid = getWidth(prj, &xsect, flowDepthMid);
            surfArea1 = (width1 + widthMid) * length * 0.5;
        }

        UP_DRY => {
            flowDepth1 = FUDGE;
            flowDepthMid = 0.5 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            width1 = getWidth(prj, &xsect, flowDepth1);
            width2 = getWidth(prj, &xsect, flowDepth2);
            widthMid = getWidth(prj, &xsect, flowDepthMid);

            // --- assign avg. surface area of downstream half of conduit
            //     to the downstream node
            surfArea2 = (widthMid + width2) * length / 4.;

            // --- if there is no free-fall at upstream end, assign the
            //     upstream node the avg. surface area of the upstream half
            if prj.Link[ju].offset1 <= 0.0 {
                surfArea1 = (width1 + widthMid) * length / 4.;
            }
        }

        DN_DRY => {
            flowDepth2 = FUDGE;
            flowDepthMid = 0.5 * (flowDepth1 + flowDepth2);
            if flowDepthMid < FUDGE {
                flowDepthMid = FUDGE;
            }
            width1 = getWidth(prj, &xsect, flowDepth1);
            width2 = getWidth(prj, &xsect, flowDepth2);
            widthMid = getWidth(prj, &xsect, flowDepthMid);

            // --- assign avg. surface area of upstream half of conduit
            //     to the upstream node
            surfArea1 = (widthMid + width1) * length / 4.;

            // --- if there is no free-fall at downstream end, assign the
            //     downstream node the avg. surface area of the downstream half
            if prj.Link[ju].offset2 <= 0.0 {
                surfArea2 = (width2 + widthMid) * length / 4.;
            }
        }

        DRY => {
            surfArea1 = FUDGE * length / 2.0;
            surfArea2 = surfArea1;
        }
        _ => {}
    }
    prj.Link[ju].surfArea1 = surfArea1;
    prj.Link[ju].surfArea2 = surfArea2;
    *y1 = flowDepth1;
    *y2 = flowDepth2;
}

//=============================================================================

fn findLocalLosses(prj: &Project, j: i32, a1: f64, a2: f64, aMid: f64, q: f64) -> f64 {
//
//  Input:   j    = link index
//           a1   = upstream area (ft2)
//           a2   = downstream area (ft2)
//           aMid = midpoint area (ft2)
//           q    = flow rate (cfs)
//  Output:  returns local losses (ft/sec)
//  Purpose: computes local losses term of momentum equation.
//
    let mut losses: f64 = 0.0;
    let q = q.abs();
    let ju = j as usize;
    if a1 > FUDGE {
        losses += prj.Link[ju].cLossInlet * (q / a1);
    }
    if a2 > FUDGE {
        losses += prj.Link[ju].cLossOutlet * (q / a2);
    }
    if aMid > FUDGE {
        losses += prj.Link[ju].cLossAvg * (q / aMid);
    }
    losses
}

//=============================================================================

fn getSlotWidth(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
    let yNorm: f64 = y / xsect.yFull;

    // --- return 0.0 if slot surcharge method not used
    if prj.SurchargeMethod != SLOT || xsect_isOpen(xsect.type_) != 0 || yNorm < prj.CrownCutoff {
        return 0.0;
    }

    // --- for depth > 1.78 * pipe depth, slot width = 1% of max. width
    if yNorm > 1.78 {
        return 0.01 * xsect.wMax;
    }

    // --- otherwise use the Sjoberg formula
    xsect.wMax * 0.5423 * (-yNorm.powf(2.4)).exp()
}

//=============================================================================

fn getWidth(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
//
//  Input:   xsect = ptr. to conduit cross section
//           y     = flow depth (ft)
//  Output:  returns top width (ft)
//  Purpose: computes top width of flow surface in conduit.
//
    let mut y = y;
    let wSlot: f64 = getSlotWidth(prj, xsect, y);
    if wSlot > 0.0 {
        return wSlot;
    }
    if y / xsect.yFull >= prj.CrownCutoff && xsect_isOpen(xsect.type_) == 0 {
        y = prj.CrownCutoff * xsect.yFull;
    }
    xsect_getWofY(prj, xsect, y)
}

//=============================================================================

fn getArea(prj: &Project, xsect: &TXsect, y: f64, wSlot: f64) -> f64 {
//
//  Input:   xsect = ptr. to conduit cross section
//           y     = flow depth (ft)
//  Output:  returns flow area (ft2)
//  Purpose: computes area of flow cross-section in a conduit.
//
    if y >= xsect.yFull {
        return xsect.aFull + (y - xsect.yFull) * wSlot;
    }
    xsect_getAofY(prj, xsect, y)
}

//=============================================================================

fn getHydRad(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
//
//  Input:   xsect = ptr. to conduit cross section
//           y     = flow depth (ft)
//  Output:  returns hydraulic radius (ft)
//  Purpose: computes hydraulic radius of flow cross-section in a conduit.
//
    if y >= xsect.yFull {
        return xsect.rFull;
    }
    xsect_getRofY(prj, xsect, y)
}

//=============================================================================

fn checkNormalFlow(prj: &mut Project, j: i32, q: f64, y1: f64, y2: f64, a1: f64,
                   r1: f64) -> f64 {
//
//  Input:   j = link index
//           q = link flow found from dynamic wave equations (cfs)
//           y1 = flow depth at upstream end (ft)
//           y2 = flow depth at downstream end (ft)
//           a1 = flow area at upstream end (ft2)
//           r1 = hyd. radius at upstream end (ft)
//  Output:  returns modifed flow in link (cfs)
//  Purpose: checks if flow in link should be replaced by normal flow.
//
    let ju = j as usize;
    let mut check: i32 = FALSE;
    let k: i32 = prj.Link[ju].subIndex;
    let n1: i32 = prj.Link[ju].node1;
    let n2: i32 = prj.Link[ju].node2;
    let hasOutfall: i32 = (prj.Node[n1 as usize].type_ == OUTFALL
        || prj.Node[n2 as usize].type_ == OUTFALL) as i32;
    let qNorm: f64;
    let f1: f64;

    // --- check if water surface slope < conduit slope
    if prj.NormalFlowLtd == SLOPE || prj.NormalFlowLtd == BOTH || hasOutfall != 0 {
        if y1 < y2 {
            check = TRUE;
        }
    }

    // --- check if Fr >= 1.0 at upstream end of conduit
    if check == 0 && (prj.NormalFlowLtd == FROUDE || prj.NormalFlowLtd == BOTH) && hasOutfall == 0 {
        if y1 > FUDGE && y2 > FUDGE {
            f1 = link_getFroude(prj, j, q / a1, y1);
            if f1 >= 1.0 {
                check = TRUE;
            }
        }
    }

    // --- check if normal flow < dynamic flow
    if check != 0 {
        qNorm = prj.Conduit[k as usize].beta * a1 * r1.powf(2. / 3.);
        if qNorm < q {
            prj.Link[ju].normalFlow = TRUE as i8;
            return qNorm;
        }
    }
    q
}

//=============================================================================
