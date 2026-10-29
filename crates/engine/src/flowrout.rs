//! flowrout.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   flowrout.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     06/12/23  (Build 5.2.4)
//   Author:   L. Rossman
//             M. Tryby (EPA)
//
//   Flow routing functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - updateStorageState() modified in response to node outflow being
//     initialized with current evap & seepage losses in routing_execute().
//   Build 5.1.008:
//   - Determination of node crown elevations moved to dynwave.c.
//   - Support added for new way of recording conduit's fullness state.
//   Build 5.1.012:
//   - Overflow computed in updateStorageState() must be non-negative.
//   - Terminal storage nodes now updated corectly.
//   Build 5.1.014:
//   - Arguments to function link_getLossRate changed.
//   Build 5.2.0:
//   - Correction made to updating state of terminal storage nodes.
//   Build 5.2.1:
//   - For storage routing, after convergence the reported depth is now
//     based on the last volume found rather than the next trial depth.
//   Build 5.2.4:
//   - Arguments to link_getLossRate changed.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const OMEGA: f64 = 0.55;    // under-relaxation parameter
const MAXITER: i32 = 10;    // max. iterations for storage updating
const STOPTOL: f64 = 0.005; // storage updating stopping tolerance

/// The file-scope variables of flowrout.c.
#[derive(Default)]
pub struct FlowroutState {
}

//=============================================================================

/// Called by: routing.
pub fn flowrout_init(prj: &mut Project, routingModel: i32) {
//
//  Input:   routingModel = routing model code
//  Output:  none
//  Purpose: initializes flow routing system.
//
    // --- initialize for dynamic wave routing
    if routingModel == DW {
        // --- check for valid conveyance network layout
        validateGeneralLayout(prj);
        dynwave_init(prj);

        // --- initialize node & link depths if not using a hotstart file
        if prj.Fhotstart1.mode as i32 == NO_FILE {
            initNodeDepths(prj);
            initLinkDepths(prj);
        }
    }
    // --- validate network layout for kinematic wave routing
    else {
        validateTreeLayout(prj);
    }

    // --- initialize node & link volumes
    initNodes(prj);
    initLinks(prj, routingModel);
}

//=============================================================================

/// Called by: routing.
pub fn flowrout_close(prj: &mut Project, routingModel: i32) {
//
//  Input:   routingModel = routing method code
//  Output:  none
//  Purpose: closes down routing method used.
//
    if routingModel == DW {
        dynwave_close(prj);
    }
}

//=============================================================================

/// Called by: routing.
pub fn flowrout_getRoutingStep(prj: &mut Project, routingModel: i32, fixedStep: f64) -> f64 {
//
//  Input:   routingModel = type of routing method used
//           fixedStep = user-assigned max. routing step (sec)
//  Output:  returns adjusted value of routing time step (sec)
//  Purpose: finds variable time step for dynamic wave routing.
//
    if routingModel == DW {
        return dynwave_getRoutingStep(prj, fixedStep);
    }
    fixedStep
}

//=============================================================================

/// Called by: routing.
pub fn flowrout_execute(prj: &mut Project, links: &[i32], routingModel: i32, tStep: f64) -> i32 {
//
//  Input:   links = array of link indexes in topo-sorted order (per routing model)
//           routingModel = type of routing method used
//           tStep = routing time step (sec)
//  Output:  returns number of computational steps taken
//  Purpose: routes flow through conveyance network over current time step.
//
    let mut j: i32;
    let mut n1: i32;                   // upstream node of link
    let mut qin: f64;                  // link inflow (cfs)
    let mut qout: f64 = 0.0;           // link outflow (cfs)
    let mut steps: f64;                // computational step count

    // --- set overflows to drain any ponded water
    if prj.ErrorCode != 0 {
        return 0;
    }
    for j in 0..prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        prj.Node[ju].updated = FALSE as i8;
        prj.Node[ju].overflow = 0.0;
        if prj.Node[ju].type_ != STORAGE && prj.Node[ju].newVolume > prj.Node[ju].fullVolume {
            prj.Node[ju].overflow = (prj.Node[ju].newVolume - prj.Node[ju].fullVolume) / tStep;
        }
    }

    // --- execute dynamic wave routing if called for
    if routingModel == DW {
        return dynwave_execute(prj, tStep);
    }

    // --- otherwise examine each link, moving from upstream to downstream
    steps = 0.0;
    for i in 0..prj.Nobjects[LINK as usize] {
        // --- see if upstream node is a storage unit whose state needs updating
        j = links[i as usize];
        n1 = prj.Link[j as usize].node1;
        if prj.Node[n1 as usize].type_ == STORAGE {
            updateStorageState(prj, n1, i, links, tStep);
        }

        // --- retrieve inflow at upstream end of link
        qin = getLinkInflow(prj, j, tStep);

        // --- route flow through link
        if routingModel == SF {
            steps += steadyflow_execute(prj, j, &mut qin, &mut qout, tStep) as f64;
        } else {
            steps += kinwave_execute(prj, j, &mut qin, &mut qout, tStep) as f64;
        }
        prj.Link[j as usize].newFlow = qout;

        // adjust outflow at upstream node and inflow at downstream node
        let ln1 = prj.Link[j as usize].node1 as usize;
        prj.Node[ln1].outflow += qin;
        let ln2 = prj.Link[j as usize].node2 as usize;
        prj.Node[ln2].inflow += qout;
    }
    if prj.Nobjects[LINK as usize] > 0 {
        steps /= prj.Nobjects[LINK as usize] as f64;
    }

    // --- update state of each non-updated node and link
    for j in 0..prj.Nobjects[NODE as usize] {
        setNewNodeState(prj, j, tStep);
    }
    for j in 0..prj.Nobjects[LINK as usize] {
        setNewLinkState(prj, j);
    }
    f64_to_i32(steps + 0.5)
}

//=============================================================================

fn validateTreeLayout(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: validates tree-like conveyance system layout used for Steady
//           and Kinematic Wave flow routing
//
    // --- check nodes
    for j in 0..prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        match prj.Node[ju].type_ {
            // --- dividers must have only 2 outlet links
            DIVIDER => {
                if prj.Node[ju].degree > 2 {
                    let id = prj.Node[ju].ID.clone();
                    report_writeErrorMsg(prj, ERR_DIVIDER, &id);
                }
            }

            // --- outfalls cannot have any outlet links
            OUTFALL => {
                if prj.Node[ju].degree > 0 {
                    let id = prj.Node[ju].ID.clone();
                    report_writeErrorMsg(prj, ERR_OUTFALL, &id);
                }
            }

            // --- storage nodes can have multiple outlets
            STORAGE => {}

            // --- all other nodes allowed only one outlet link
            _ => {
                if prj.Node[ju].degree > 1 {
                    let id = prj.Node[ju].ID.clone();
                    report_writeErrorMsg(prj, ERR_MULTI_OUTLET, &id);
                }
            }
        }
    }

    // ---  check links
    for j in 0..prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        match prj.Link[ju].type_ {
            // --- non-dummy conduits cannot have adverse slope
            CONDUIT => {
                if prj.Conduit[prj.Link[ju].subIndex as usize].slope < 0.0
                    && prj.Link[ju].xsect.type_ != DUMMY
                {
                    let id = prj.Link[ju].ID.clone();
                    report_writeErrorMsg(prj, ERR_SLOPE, &id);
                }
            }

            // --- regulator links must be outlets of storage nodes
            ORIFICE | WEIR | OUTLET => {
                if prj.Node[prj.Link[ju].node1 as usize].type_ != STORAGE {
                    let id = prj.Link[ju].ID.clone();
                    report_writeErrorMsg(prj, ERR_REGULATOR, &id);
                }
            }
            _ => {}
        }
    }
}

//=============================================================================

fn validateGeneralLayout(prj: &mut Project) {
//
//  Input:   none
//  Output:  nonw
//  Purpose: validates general conveyance system layout.
//
    let mut i: i32;
    let mut outletCount: i32 = 0;

    // --- use node inflow attribute to count inflow connections
    for i in 0..prj.Nobjects[NODE as usize] {
        prj.Node[i as usize].inflow = 0.0;
    }

    // --- examine each link
    for j in 0..prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        // --- update inflow link count of downstream node
        i = prj.Link[ju].node1;
        if prj.Node[i as usize].type_ != OUTFALL {
            i = prj.Link[ju].node2;
        }
        prj.Node[i as usize].inflow += 1.0;

        // --- if link is dummy link or ideal pump then it must
        //     be the only link exiting the upstream node
        if (prj.Link[ju].type_ == CONDUIT && prj.Link[ju].xsect.type_ == DUMMY)
            || (prj.Link[ju].type_ == PUMP
                && prj.Pump[prj.Link[ju].subIndex as usize].type_ == IDEAL_PUMP)
        {
            i = prj.Link[ju].node1;
            if prj.Link[ju].direction < 0 {
                i = prj.Link[ju].node2;
            }
            if prj.Node[i as usize].degree > 1 {
                let id = prj.Node[i as usize].ID.clone();
                report_writeErrorMsg(prj, ERR_DUMMY_LINK, &id);
            }
        }
    }

    // --- check each node to see if it qualifies as an outlet node
    //     (meaning that degree = 0)
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        // --- if node is of type Outfall, check that it has only 1
        //     connecting link (which can either be an outflow or inflow link)
        if prj.Node[iu].type_ == OUTFALL {
            if prj.Node[iu].degree + f64_to_i32(prj.Node[iu].inflow) > 1 {
                let id = prj.Node[iu].ID.clone();
                report_writeErrorMsg(prj, ERR_OUTFALL, &id);
            } else {
                outletCount += 1;
            }
        }
    }
    if outletCount == 0 {
        report_writeErrorMsg(prj, ERR_NO_OUTLETS, b"");
    }

    // --- reset node inflows back to zero
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        if prj.Node[iu].inflow == 0.0 {
            prj.Node[iu].degree = -prj.Node[iu].degree;
        }
        prj.Node[iu].inflow = 0.0;
    }
}

//=============================================================================

fn initNodeDepths(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: sets initial depth at nodes for Dynamic Wave flow routing.
//
    let mut n: i32;                    // node index
    let mut y: f64;                    // node water depth (ft)

    // --- use Node[].inflow as a temporary accumulator for depth in
    //     connecting links and Node[].outflow as a temporary counter
    //     for the number of connecting links
    for i in 0..prj.Nobjects[NODE as usize] {
        prj.Node[i as usize].inflow = 0.0;
        prj.Node[i as usize].outflow = 0.0;
    }

    // --- total up flow depths in all connecting links into nodes
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        if prj.Link[iu].newDepth > FUDGE {
            y = prj.Link[iu].newDepth + prj.Link[iu].offset1;
        } else {
            y = 0.0;
        }
        n = prj.Link[iu].node1;
        prj.Node[n as usize].inflow += y;
        prj.Node[n as usize].outflow += 1.0;
        n = prj.Link[iu].node2;
        prj.Node[n as usize].inflow += y;
        prj.Node[n as usize].outflow += 1.0;
    }

    // --- if no user-supplied depth then set initial depth at non-storage/
    //     non-outfall nodes to average of depths in connecting links
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        if prj.Node[iu].type_ == OUTFALL {
            continue;
        }
        if prj.Node[iu].type_ == STORAGE {
            continue;
        }
        if prj.Node[iu].initDepth > 0.0 {
            continue;
        }
        if prj.Node[iu].outflow > 0.0 {
            prj.Node[iu].newDepth = prj.Node[iu].inflow / prj.Node[iu].outflow;
        }
    }

    // --- compute initial depths at all outfall nodes
    for i in 0..prj.Nobjects[LINK as usize] {
        link_setOutfallDepth(prj, i);
    }
}

//=============================================================================

fn initLinkDepths(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: sets initial flow depths in conduits under Dyn. Wave routing.
//
    let mut y: f64;                    // depths (ft)
    let mut y1: f64;
    let mut y2: f64;

    // --- examine each link
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        // --- examine each conduit
        if prj.Link[iu].type_ == CONDUIT {
            // --- skip conduits with user-assigned initial flows
            //     (their depths have already been set to normal depth)
            if prj.Link[iu].q0 != 0.0 {
                continue;
            }

            // --- set depth to average of depths at end nodes
            y1 = prj.Node[prj.Link[iu].node1 as usize].newDepth - prj.Link[iu].offset1;
            y1 = MAX(y1, 0.0);
            y1 = MIN(y1, prj.Link[iu].xsect.yFull);
            y2 = prj.Node[prj.Link[iu].node2 as usize].newDepth - prj.Link[iu].offset2;
            y2 = MAX(y2, 0.0);
            y2 = MIN(y2, prj.Link[iu].xsect.yFull);
            y = 0.5 * (y1 + y2);
            y = MAX(y, FUDGE);
            prj.Link[iu].newDepth = y;
        }
    }
}

//=============================================================================

fn initNodes(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: sets initial inflow/outflow and volume for each node
//
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        // --- initialize node inflow and outflow
        prj.Node[iu].inflow = prj.Node[iu].newLatFlow;
        prj.Node[iu].outflow = 0.0;

        // --- initialize node volume
        prj.Node[iu].newVolume = 0.0;
        if prj.AllowPonding != 0
            && prj.Node[iu].pondedArea > 0.0
            && prj.Node[iu].newDepth > prj.Node[iu].fullDepth
        {
            prj.Node[iu].newVolume = prj.Node[iu].fullVolume
                + (prj.Node[iu].newDepth - prj.Node[iu].fullDepth) * prj.Node[iu].pondedArea;
        } else {
            let d = prj.Node[iu].newDepth;
            prj.Node[iu].newVolume = node_getVolume(prj, i, d);
        }
    }

    // --- update nodal inflow/outflow at ends of each link
    //     (needed for Steady Flow & Kin. Wave routing)
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        let n1 = prj.Link[iu].node1 as usize;
        let n2 = prj.Link[iu].node2 as usize;
        if prj.Link[iu].newFlow >= 0.0 {
            prj.Node[n1].outflow += prj.Link[iu].newFlow;
            prj.Node[n2].inflow += prj.Link[iu].newFlow;
        } else {
            prj.Node[n1].inflow -= prj.Link[iu].newFlow;
            prj.Node[n2].outflow -= prj.Link[iu].newFlow;
        }
    }
}

//=============================================================================

fn initLinks(prj: &mut Project, routingModel: i32) {
//
//  Input:   none
//  Output:  none
//  Purpose: sets initial upstream/downstream conditions in links.
//
    let mut k: i32;                    // conduit or pump index

    // --- examine each link
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        if routingModel == SF {
            prj.Link[iu].newFlow = 0.0;
        }
        // --- otherwise if link is a conduit
        else if prj.Link[iu].type_ == CONDUIT {
            // --- assign initial flow to both ends of conduit
            k = prj.Link[iu].subIndex;
            let ku = k as usize;
            prj.Conduit[ku].q1 = prj.Link[iu].newFlow / prj.Conduit[ku].barrels as f64;
            prj.Conduit[ku].q2 = prj.Conduit[ku].q1;

            // --- find areas based on initial flow depth
            let y0 = prj.Link[iu].newDepth;
            prj.Conduit[ku].a1 = linkXsect_getAofY(prj, iu as i32, y0);
            prj.Conduit[ku].a2 = prj.Conduit[ku].a1;

            // --- compute initial volume from area
            {
                prj.Link[iu].newVolume = prj.Conduit[ku].a1 * link_getLength(prj, i)
                    * prj.Conduit[ku].barrels as f64;
            }
            prj.Link[iu].oldVolume = prj.Link[iu].newVolume;
        }
    }
}

//=============================================================================

fn getLinkInflow(prj: &mut Project, j: i32, dt: f64) -> f64 {
//
//  Input:   j  = link index
//           dt = routing time step (sec)
//  Output:  returns link inflow (cfs)
//  Purpose: finds flow into upstream end of link at current time step under
//           Steady or Kin. Wave routing.
//
    let n1: i32 = prj.Link[j as usize].node1;
    let q: f64;
    if prj.Link[j as usize].type_ == CONDUIT
        || prj.Link[j as usize].type_ == PUMP
        || prj.Node[n1 as usize].type_ == STORAGE
    {
        q = link_getInflow(prj, j);
    } else {
        q = 0.0;
    }
    node_getMaxOutflow(prj, n1, q, dt)
}

//=============================================================================

fn updateStorageState(prj: &mut Project, i: i32, j: i32, links: &[i32], dt: f64) {
//
//  Input:   i = index of storage node
//           j = current position in links array
//           links = array of topo-sorted link indexes
//           dt = routing time step (sec)
//  Output:  none
//  Purpose: updates depth and volume of a storage node using successive
//           approximation with under-relaxation for Steady or Kin. Wave
//           routing.
//
    let mut iter: i32;                 // iteration counter
    let mut stopped: i32;              // TRUE when iterations stop
    let vFixed: f64;                   // fixed terms of flow balance eqn.
    let mut v2: f64;                   // new volume estimate (ft3)
    let mut d1: f64;                   // initial value of storage depth (ft)
    let mut d2: f64;                   // updated value of storage depth (ft)
    let iu = i as usize;

    // --- see if storage node needs updating
    if prj.Node[iu].type_ != STORAGE {
        return;
    }
    if prj.Node[iu].updated != 0 {
        return;
    }

    // --- compute terms of flow balance eqn.
    //       v2 = v1 + (inflow - outflow)*dt
    //     that do not depend on storage depth at end of time step
    vFixed = prj.Node[iu].oldVolume
        + 0.5 * (prj.Node[iu].oldNetInflow + prj.Node[iu].inflow - prj.Node[iu].outflow) * dt;
    d1 = prj.Node[iu].newDepth;

    // --- iterate finding outflow (which depends on depth) and subsequent
    //     new volume and depth until negligible depth change occurs
    iter = 1;
    stopped = FALSE;
    while iter < MAXITER && stopped == 0 {
        // --- find new volume from flow balance eqn.
        v2 = vFixed - 0.5 * getStorageOutflow(prj, i, j, links, dt) * dt;

        // --- limit volume to full volume if no ponding
        //     and compute overflow rate
        v2 = MAX(0.0, v2);
        prj.Node[iu].overflow = 0.0;
        if v2 > prj.Node[iu].fullVolume {
            prj.Node[iu].overflow =
                (v2 - MAX(prj.Node[iu].oldVolume, prj.Node[iu].fullVolume)) / dt;
            if prj.Node[iu].overflow < FUDGE {
                prj.Node[iu].overflow = 0.0;
            }
            if prj.AllowPonding == 0 || prj.Node[iu].pondedArea == 0.0 {
                v2 = prj.Node[iu].fullVolume;
            }
        }

        // --- update node's volume & depth
        prj.Node[iu].newVolume = v2;
        d2 = node_getDepth(prj, i, v2);
        prj.Node[iu].newDepth = d2;

        // --- use under-relaxation to estimate new depth value
        //     and stop if close enough to previous value
        d2 = (1.0 - OMEGA) * d1 + OMEGA * d2;
        if (d2 - d1).abs() <= STOPTOL {
            stopped = TRUE;
        }

        // --- update old depth with new value and continue to iterate
        d1 = d2;
        iter += 1;
    }

    // --- mark node as being updated
    prj.Node[iu].updated = TRUE as i8;
}

//=============================================================================

fn getStorageOutflow(prj: &mut Project, i: i32, j: i32, links: &[i32], dt: f64) -> f64 {
//
//  Input:   i = index of storage node
//           j = current position in links array
//           links = array of topo-sorted link indexes
//           dt = routing time step (sec)
//  Output:  returns total outflow from storage node (cfs)
//  Purpose: computes total flow released from a storage node.
//
    let mut m: i32;
    let mut outflow: f64 = 0.0;

    let mut k = j;
    while k < prj.Nobjects[LINK as usize] {
        m = links[k as usize];
        if prj.Link[m as usize].node1 != i {
            break;
        }
        outflow += getLinkInflow(prj, m, dt);
        k += 1;
    }
    outflow
}

//=============================================================================

fn setNewNodeState(prj: &mut Project, j: i32, dt: f64) {
//
//  Input:   j  = node index
//           dt = time step (sec)
//  Output:  none
//  Purpose: updates state of node after current time step
//           for Steady Flow or Kinematic Wave flow routing.
//
    let canPond: i32;                  // TRUE if ponding can occur at node
    let newNetInflow: f64;             // inflow - outflow at node (cfs)
    let ju = j as usize;

    // --- update terminal storage nodes
    if prj.Node[ju].type_ == STORAGE {
        if prj.Node[ju].updated as i32 == FALSE {
            // C passes links = NULL; with j = Nobjects[LINK] the array is
            // never read, so an empty slice stands in for it.
            let nLinks = prj.Nobjects[LINK as usize];
            updateStorageState(prj, j, nLinks, &[], dt);
        }
        return;
    }

    // --- update stored volume
    newNetInflow = prj.Node[ju].inflow - prj.Node[ju].outflow - prj.Node[ju].losses;
    prj.Node[ju].newVolume = prj.Node[ju].oldVolume + newNetInflow * dt;
    if prj.Node[ju].newVolume < FUDGE {
        prj.Node[ju].newVolume = 0.0;
    }

    // --- determine any overflow lost from system
    prj.Node[ju].overflow = 0.0;
    canPond = (prj.AllowPonding != 0 && prj.Node[ju].pondedArea > 0.0) as i32;
    if prj.Node[ju].newVolume > prj.Node[ju].fullVolume {
        prj.Node[ju].overflow = (prj.Node[ju].newVolume
            - MAX(prj.Node[ju].oldVolume, prj.Node[ju].fullVolume)) / dt;
        if prj.Node[ju].overflow < FUDGE {
            prj.Node[ju].overflow = 0.0;
        }
        if canPond == 0 {
            prj.Node[ju].newVolume = prj.Node[ju].fullVolume;
        }
    }

    // --- compute a depth from volume
    //     (depths at upstream nodes are subsequently adjusted in
    //     setNewLinkState to reflect depths in connected conduit)
    let v = prj.Node[ju].newVolume;
    prj.Node[ju].newDepth = node_getDepth(prj, j, v);
}

//=============================================================================

fn setNewLinkState(prj: &mut Project, j: i32) {
//
//  Input:   j = link index
//  Output:  none
//  Purpose: updates state of link after current time step under
//           Steady Flow or Kinematic Wave flow routing
//
    let k: i32;
    let a: f64;
    let y1: f64;
    let y2: f64;
    let ju = j as usize;

    prj.Link[ju].newDepth = 0.0;
    prj.Link[ju].newVolume = 0.0;

    if prj.Link[ju].type_ == CONDUIT {
        // --- find avg. depth from entry/exit conditions
        k = prj.Link[ju].subIndex;
        let ku = k as usize;
        a = 0.5 * (prj.Conduit[ku].a1 + prj.Conduit[ku].a2);
        prj.Link[ju].newVolume = a * link_getLength(prj, j) * prj.Conduit[ku].barrels as f64;
        let (a1, a2) = (prj.Conduit[ku].a1, prj.Conduit[ku].a2);
        y1 = linkXsect_getYofA(prj, j, a1);
        y2 = linkXsect_getYofA(prj, j, a2);
        prj.Link[ju].newDepth = 0.5 * (y1 + y2);

        // --- update depths at end nodes
        let n1 = prj.Link[ju].node1;
        let off1 = prj.Link[ju].offset1;
        updateNodeDepth(prj, n1, y1 + off1);
        let n2 = prj.Link[ju].node2;
        let off2 = prj.Link[ju].offset2;
        updateNodeDepth(prj, n2, y2 + off2);

        // --- check if capacity limited
        if prj.Conduit[ku].a1 >= prj.Link[ju].xsect.aFull {
            prj.Conduit[ku].capacityLimited = TRUE as i8;
            prj.Conduit[ku].fullState = ALL_FULL as i8;
        } else {
            prj.Conduit[ku].capacityLimited = FALSE as i8;
            prj.Conduit[ku].fullState = 0;
        }
    }
}

//=============================================================================

fn updateNodeDepth(prj: &mut Project, i: i32, y: f64) {
//
//  Input:   i = node index
//           y = flow depth (ft)
//  Output:  none
//  Purpose: updates water depth at a node with a possibly higher value.
//
    let mut y = y;
    let iu = i as usize;

    // --- storage nodes were updated elsewhere
    if prj.Node[iu].type_ == STORAGE {
        return;
    }

    // --- if non-outfall node is flooded, then use full depth
    if prj.Node[iu].type_ != OUTFALL && prj.Node[iu].degree > 0 && prj.Node[iu].overflow > 0.0 {
        y = prj.Node[iu].fullDepth;
    }

    // --- if current new depth below y
    if prj.Node[iu].newDepth < y {
        // --- update new depth
        prj.Node[iu].newDepth = y;

        // --- depth cannot exceed full depth (if value exists)
        if prj.Node[iu].fullDepth > 0.0 && y > prj.Node[iu].fullDepth {
            prj.Node[iu].newDepth = prj.Node[iu].fullDepth;
        }
    }
}

//=============================================================================

fn steadyflow_execute(prj: &mut Project, j: i32, qin: &mut f64, qout: &mut f64, tStep: f64) -> i32 {
//
//  Input:   j = link index
//           qin = inflow to link (cfs)
//           tStep = time step (sec)
//  Output:  qin = adjusted inflow to link (limited by flow capacity) (cfs)
//           qout = link's outflow (cfs)
//           returns 1 if successful
//  Purpose: performs steady flow routing through a single link.
//
    let k: i32;
    let s: f64;
    let mut q: f64;
    let ju = j as usize;

    // --- use Manning eqn. to compute flow area for conduits
    if prj.Link[ju].type_ == CONDUIT {
        k = prj.Link[ju].subIndex;
        let ku = k as usize;
        q = *qin / prj.Conduit[ku].barrels as f64;
        if prj.Link[ju].xsect.type_ == DUMMY {
            prj.Conduit[ku].a1 = 0.0;
        } else {
            // --- adjust flow for evap and infil losses
            q -= link_getLossRate(prj, j, SF, q, tStep);

            // --- flow can't exceed full flow
            if q > prj.Link[ju].qFull {
                q = prj.Link[ju].qFull;
                prj.Conduit[ku].a1 = prj.Link[ju].xsect.aFull;
                *qin = q * prj.Conduit[ku].barrels as f64;
            }
            // --- infer flow area from flow rate
            else {
                s = q / prj.Conduit[ku].beta;
                prj.Conduit[ku].a1 = linkXsect_getAofS(prj, j, s);
            }
        }
        prj.Conduit[ku].a2 = prj.Conduit[ku].a1;

        prj.Conduit[ku].q1Old = prj.Conduit[ku].q1;
        prj.Conduit[ku].q2Old = prj.Conduit[ku].q2;

        prj.Conduit[ku].q1 = q;
        prj.Conduit[ku].q2 = q;
        *qout = q * prj.Conduit[ku].barrels as f64;
    } else {
        *qout = *qin;
    }
    1
}

//=============================================================================
