//! dynwave.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   dynwave.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     07/13/23  (Build 5.2.4)
//   Author:   L. Rossman
//             M. Tryby (EPA)
//             R. Dickinson (CDM)
//
//   Dynamic wave flow routing functions.
//
//   This module solves the dynamic wave flow routing equations using
//   Picard Iterations (i.e., a method of successive approximations)
//   to solve the explicit form of the continuity and momentum equations
//   for conduits.
//
//   Update History
//   ==============
//   Build 5.1.002:
//   - Only non-ponded nodal surface area is saved for use in
//     surcharge algorithm.
//   Build 5.1.007:
//   - Node losses added to node outflow variable instead of treated
//     as a separate item when computing change in node flow volume.
//   Build 5.1.008:
//   - Module-specific constants moved here from project.c.
//   - Support added for user-specified minimum variable time step.
//   - Node crown elevations found here instead of in flowrout.c module.
//   - OpenMP use to parallelize findLinkFlows() & findNodeDepths().
//   - Bug in finding complete list of capacity limited links fixed.
//   Build 5.1.011:
//   - Added test for failed memory allocation.
//   - Fixed illegal array index bug for Ideal Pumps.
//   Build 5.1.013:
//   - Include omp.h protected against lack of compiler support for OpenMP.
//   - SurchargeMethod option used to decide how node surcharging is handled.
//   - Storage nodes allowed to pressurize if their surcharge depth > 0.
//   - Minimum flow needed to compute a Courant time step modified.
//   Build 5.1.014:
//   - updateNodeFlows() modified to subtract conduit evap. and seepage losses
//     from downstream node inflow instead of upstream node outflow.
//   Build 5.1.015:
//   - Roll back the 5.1.014 change for conduit losses in updateNodeFlows().
//   Build 5.2.0:
//   - Support added for reporting most frequent non-converging links.
//   Build 5.2.4:
//   - Conduit evap+seepage outflow split evenly between outflow from
//     conduit's upstream and non-outfall downstream nodes.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//     Constants
//-----------------------------------------------------------------------------
const MINTIMESTEP: f64 = 0.001;  // min. time step (sec)
const OMEGA: f64 = 0.5;    // under-relaxation parameter
const DEFAULT_SURFAREA: f64 = 12.566; // Min. nodal surface area (~4 ft diam.)
const DEFAULT_HEADTOL: f64 = 0.005;  // Default head tolerance (ft)
const EXTRAN_CROWN_CUTOFF: f64 = 0.96;   // crown cutoff for EXTRAN
const SLOT_CROWN_CUTOFF: f64 = 0.985257; // crown cutoff for SLOT
const DEFAULT_MAXTRIALS: i32 = 8;      // Max. trials per time step

//-----------------------------------------------------------------------------
//  Data Structures
//-----------------------------------------------------------------------------
#[derive(Clone, Copy, Default)]
struct TXnode {
    converged: i8,                 // TRUE if iterations for a node done
    newSurfArea: f64,              // current surface area (ft2)
    oldSurfArea: f64,              // previous surface area (ft2)
    sumdqdh: f64,                  // sum of dqdh from adjoining links
    dYdT: f64,                     // change in depth w.r.t. time (ft/sec)
}

/// The file-scope variables of dynwave.c.
#[derive(Default)]
pub struct DynwaveState {
    VariableStep: f64,                 // size of variable time step (sec)
    Xnode: Vec<TXnode>,                // extended nodal information

    Omega: f64,                        // actual under-relaxation parameter
    Steps: i32,                        // number of Picard iterations
}

//=============================================================================

/// Called by: flowrout.
pub fn dynwave_init(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: initializes dynamic wave routing method.
//
    let mut j: i32;
    let mut z: f64;

    prj.dynwave.VariableStep = 0.0;
    prj.dynwave.Xnode = vec![TXnode::default(); prj.Nobjects[NODE as usize] as usize];

    // --- initialize node surface areas & crown elev.
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        prj.dynwave.Xnode[iu].newSurfArea = 0.0;
        prj.dynwave.Xnode[iu].oldSurfArea = 0.0;
        prj.Node[iu].crownElev = prj.Node[iu].invertElev;
    }

    // --- initialize links & update node crown elevations
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        j = prj.Link[iu].node1;
        z = prj.Node[j as usize].invertElev + prj.Link[iu].offset1 + prj.Link[iu].xsect.yFull;
        prj.Node[j as usize].crownElev = MAX(prj.Node[j as usize].crownElev, z);

        j = prj.Link[iu].node2;
        z = prj.Node[j as usize].invertElev + prj.Link[iu].offset2 + prj.Link[iu].xsect.yFull;
        prj.Node[j as usize].crownElev = MAX(prj.Node[j as usize].crownElev, z);
        prj.Link[iu].flowClass = DRY;
        prj.Link[iu].dqdh = 0.0;
    }

    // --- set crown cutoff for finding top width of closed conduits
    if prj.SurchargeMethod == SLOT {
        prj.CrownCutoff = SLOT_CROWN_CUTOFF;
    } else {
        prj.CrownCutoff = EXTRAN_CROWN_CUTOFF;
    }
}

//=============================================================================

/// Called by: flowrout.
pub fn dynwave_close(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: frees memory allocated for dynamic wave routing method.
//
    prj.dynwave.Xnode = Vec::new();
}

//=============================================================================

/// Called by: project.
pub fn dynwave_validate(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: adjusts dynamic wave routing options.
//
    if prj.MinRouteStep > prj.RouteStep {
        prj.MinRouteStep = prj.RouteStep;
    }
    if prj.MinRouteStep < MINTIMESTEP {
        prj.MinRouteStep = MINTIMESTEP;
    }
    if prj.MinSurfArea == 0.0 {
        prj.MinSurfArea = DEFAULT_SURFAREA;
    } else {
        prj.MinSurfArea /= UCF(prj, LENGTH) * UCF(prj, LENGTH);
    }
    if prj.HeadTol == 0.0 {
        prj.HeadTol = DEFAULT_HEADTOL;
    } else {
        prj.HeadTol /= UCF(prj, LENGTH);
    }
    if prj.MaxTrials == 0 {
        prj.MaxTrials = DEFAULT_MAXTRIALS;
    }
}

//=============================================================================

/// Called by: flowrout.
pub fn dynwave_getRoutingStep(prj: &mut Project, fixedStep: f64) -> f64 {
//
//  Input:   fixedStep = user-supplied fixed time step (sec)
//  Output:  returns routing time step (sec)
//  Purpose: computes variable routing time step if applicable.
//
    // --- use user-supplied fixed step if variable step option turned off
    //     or if its smaller than the min. allowable variable time step
    if prj.CourantFactor == 0.0 {
        return fixedStep;
    }
    if fixedStep < MINTIMESTEP {
        return fixedStep;
    }

    // --- at start of simulation (when current variable step is zero)
    //     use the minimum allowable time step
    if prj.dynwave.VariableStep == 0.0 {
        prj.dynwave.VariableStep = prj.MinRouteStep;
    }
    // --- otherwise compute variable step based on current flow solution
    else {
        prj.dynwave.VariableStep = getVariableStep(prj, fixedStep);
    }

    // --- adjust step to be a multiple of a millisecond
    prj.dynwave.VariableStep = (1000.0 * prj.dynwave.VariableStep).floor() / 1000.0;
    prj.dynwave.VariableStep
}

//=============================================================================

/// Called by: flowrout.
pub fn dynwave_execute(prj: &mut Project, tStep: f64) -> i32 {
//
//  Input:   links = array of topo sorted links indexes
//           tStep = time step (sec)
//  Output:  returns number of iterations used
//  Purpose: routes flows through drainage network over current time step.
//
    let mut converged: i32;

    // --- initialize
    if prj.ErrorCode != 0 {
        return 0;
    }
    prj.dynwave.Steps = 0;
    converged = FALSE;
    prj.dynwave.Omega = OMEGA;
    initRoutingStep(prj);

    // --- keep iterating until convergence
    while prj.dynwave.Steps < prj.MaxTrials {
        // --- execute a routing step & check for nodal convergence
        initNodeStates(prj);
        findLinkFlows(prj, tStep);
        converged = findNodeDepths(prj, tStep);
        prj.dynwave.Steps += 1;
        if prj.dynwave.Steps > 1 {
            if converged != 0 {
                break;
            }

            // --- check if link calculations can be skipped in next step
            findBypassedLinks(prj);
        }
    }
    if converged == 0 {
        updateConvergenceStats(prj);
    }

    //  --- identify any capacity-limited conduits
    findLimitedLinks(prj);
    prj.dynwave.Steps
}

//=============================================================================

fn updateConvergenceStats(prj: &mut Project) {
    prj.NonConvergeCount += 1;
    for i in 0..prj.Nobjects[NODE as usize] {
        let c = prj.dynwave.Xnode[i as usize].converged as i32;
        stats_updateConvergenceStats(prj, i, c);
    }
}

//=============================================================================

fn initRoutingStep(prj: &mut Project) {
    for i in 0..prj.Nobjects[NODE as usize] {
        prj.dynwave.Xnode[i as usize].converged = FALSE as i8;
        prj.dynwave.Xnode[i as usize].dYdT = 0.0;
    }
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        prj.Link[iu].bypassed = FALSE as i8;
        prj.Link[iu].surfArea1 = 0.0;
        prj.Link[iu].surfArea2 = 0.0;
    }

    // --- a2 preserves conduit area from solution at last time step
    for i in 0..prj.Nlinks[CONDUIT as usize] {
        prj.Conduit[i as usize].a2 = prj.Conduit[i as usize].a1;
    }
}

//=============================================================================

fn initNodeStates(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: initializes node's surface area, inflow & outflow
//
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        // --- initialize nodal surface area
        if prj.AllowPonding != 0 {
            let d = prj.Node[iu].newDepth;
            prj.dynwave.Xnode[iu].newSurfArea = node_getPondedArea(prj, i, d);
        } else {
            let d = prj.Node[iu].newDepth;
            prj.dynwave.Xnode[iu].newSurfArea = node_getSurfArea(prj, i, d);
        }

        // --- initialize nodal inflow & outflow
        prj.Node[iu].inflow = 0.0;
        prj.Node[iu].outflow = prj.Node[iu].losses;
        if prj.Node[iu].newLatFlow >= 0.0 {
            prj.Node[iu].inflow += prj.Node[iu].newLatFlow;
        } else {
            prj.Node[iu].outflow -= prj.Node[iu].newLatFlow;
        }
        prj.dynwave.Xnode[iu].sumdqdh = 0.0;
    }
}

//=============================================================================

fn findBypassedLinks(prj: &mut Project) {
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        if prj.dynwave.Xnode[prj.Link[iu].node1 as usize].converged != 0
            && prj.dynwave.Xnode[prj.Link[iu].node2 as usize].converged != 0
        {
            prj.Link[iu].bypassed = TRUE as i8;
        } else {
            prj.Link[iu].bypassed = FALSE as i8;
        }
    }
}

//=============================================================================

fn findLimitedLinks(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: determines if a conduit link is capacity limited.
//
    let mut n1: i32;
    let mut n2: i32;
    let mut k: i32;
    let mut h1: f64;
    let mut h2: f64;

    for j in 0..prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        // ---- check only non-dummy conduit links
        if isTrueConduit(prj, j) == 0 {
            continue;
        }

        // --- check that upstream end is full
        k = prj.Link[ju].subIndex;
        let ku = k as usize;
        prj.Conduit[ku].capacityLimited = FALSE as i8;
        if prj.Conduit[ku].a1 >= prj.Link[ju].xsect.aFull {
            // --- check if HGL slope > conduit slope
            n1 = prj.Link[ju].node1;
            n2 = prj.Link[ju].node2;
            h1 = prj.Node[n1 as usize].newDepth + prj.Node[n1 as usize].invertElev;
            h2 = prj.Node[n2 as usize].newDepth + prj.Node[n2 as usize].invertElev;
            if (h1 - h2) > prj.Conduit[ku].slope.abs() * prj.Conduit[ku].length {
                prj.Conduit[ku].capacityLimited = TRUE as i8;
            }
        }
    }
}

//=============================================================================

fn findLinkFlows(prj: &mut Project, dt: f64) {
    // --- find new flow in each non-dummy conduit
    // (C: an OpenMP parallel loop; the reference build runs it serially)
    for i in 0..prj.Nobjects[LINK as usize] {
        if isTrueConduit(prj, i) != 0 && prj.Link[i as usize].bypassed == 0 {
            let steps = prj.dynwave.Steps;
            let omega = prj.dynwave.Omega;
            dwflow_findConduitFlow(prj, i, steps, omega, dt);
        }
    }

    // --- update inflow/outflows for nodes attached to non-dummy conduits
    for i in 0..prj.Nobjects[LINK as usize] {
        if isTrueConduit(prj, i) != 0 {
            updateNodeFlows(prj, i);
        }
    }

    // --- find new flows for all dummy conduits, pumps & regulators
    for i in 0..prj.Nobjects[LINK as usize] {
        if isTrueConduit(prj, i) == 0 {
            if prj.Link[i as usize].bypassed == 0 {
                findNonConduitFlow(prj, i, dt);
            }
            updateNodeFlows(prj, i);
        }
    }
}

//=============================================================================

fn isTrueConduit(prj: &Project, j: i32) -> i32 {
    (prj.Link[j as usize].type_ == CONDUIT && prj.Link[j as usize].xsect.type_ != DUMMY) as i32
}

//=============================================================================

fn findNonConduitFlow(prj: &mut Project, i: i32, dt: f64) {
//
//  Input:   i = link index
//           dt = time step (sec)
//  Output:  none
//  Purpose: finds new flow in a non-conduit-type link
//
    let qLast: f64;                    // previous link flow (cfs)
    let mut qNew: f64;                 // new link flow (cfs)
    let iu = i as usize;

    // --- get link flow from last iteration
    qLast = prj.Link[iu].newFlow;
    prj.Link[iu].dqdh = 0.0;

    // --- get new inflow to link from its upstream node
    //     (link_getInflow returns 0 if flap gate closed or pump is offline)
    qNew = link_getInflow(prj, i);
    if prj.Link[iu].type_ == PUMP {
        qNew = getModPumpFlow(prj, i, qNew, dt);
    }

    // --- find surface area at each end of link
    findNonConduitSurfArea(prj, i);

    // --- apply under-relaxation with flow from previous iteration;
    // --- do not allow flow to change direction without first being 0
    if prj.dynwave.Steps > 0 && prj.Link[iu].type_ != PUMP {
        qNew = (1.0 - prj.dynwave.Omega) * qLast + prj.dynwave.Omega * qNew;
        if qNew * qLast < 0.0 {
            qNew = 0.001 * SGN(qNew);
        }
    }
    prj.Link[iu].newFlow = qNew;
}

//=============================================================================

fn getModPumpFlow(prj: &mut Project, i: i32, q: f64, dt: f64) -> f64 {
//
//  Input:   i = link index
//           q = pump flow from pump curve (cfs)
//           dt = time step (sec)
//  Output:  returns modified pump flow rate (cfs)
//  Purpose: modifies pump curve pumping rate depending on amount of water
//           available at pump's inlet node.
//
    let j: i32 = prj.Link[i as usize].node1;          // pump's inlet node index
    let k: i32 = prj.Link[i as usize].subIndex;       // pump's index
    let newNetInflow: f64;             // inflow - outflow rate (cfs)
    let netFlowVolume: f64;            // inflow - outflow volume (ft3)
    let y: f64;                        // node depth (ft)
    let ju = j as usize;

    if q == 0.0 {
        return q;
    }

    // --- case where inlet node is a storage node:
    //     prevent node volume from going negative
    if prj.Node[ju].type_ == STORAGE {
        return node_getMaxOutflow(prj, j, q, dt);
    }

    // --- case where inlet is a non-storage node
    match prj.Pump[k as usize].type_ {
        // --- for Type1 pump, a volume is computed for inlet node,
        //     so make sure it doesn't go negative
        TYPE1_PUMP => {
            return node_getMaxOutflow(prj, j, q, dt);
        }

        // --- for other types of pumps, if pumping rate would make depth
        //     at upstream node negative, then set pumping rate = inflow
        TYPE2_PUMP | TYPE4_PUMP | TYPE3_PUMP => {
            newNetInflow = prj.Node[ju].inflow - prj.Node[ju].outflow - q;
            netFlowVolume = 0.5 * (prj.Node[ju].oldNetInflow + newNetInflow) * dt;
            y = prj.Node[ju].oldDepth + netFlowVolume / prj.dynwave.Xnode[ju].newSurfArea;
            if y <= 0.0 {
                return prj.Node[ju].inflow;
            }
        }
        _ => {}
    }
    q
}

//=============================================================================

fn findNonConduitSurfArea(prj: &mut Project, i: i32) {
//
//  Input:   i = link index
//  Output:  none
//  Purpose: finds the surface area contributed by a non-conduit
//           link to its upstream and downstream nodes.
//
    let iu = i as usize;
    if prj.Link[iu].type_ == ORIFICE {
        prj.Link[iu].surfArea1 = prj.Orifice[prj.Link[iu].subIndex as usize].surfArea / 2.;
    }
    // --- no surface area for weirs to maintain SWMM 4 compatibility
    else {
        prj.Link[iu].surfArea1 = 0.0;
    }

    prj.Link[iu].surfArea2 = prj.Link[iu].surfArea1;
    if prj.Link[iu].flowClass == UP_CRITICAL
        || prj.Node[prj.Link[iu].node1 as usize].type_ == STORAGE
    {
        prj.Link[iu].surfArea1 = 0.0;
    }
    if prj.Link[iu].flowClass == DN_CRITICAL
        || prj.Node[prj.Link[iu].node2 as usize].type_ == STORAGE
    {
        prj.Link[iu].surfArea2 = 0.0;
    }
}

//=============================================================================

fn updateNodeFlows(prj: &mut Project, i: i32) {
//
//  Input:   i = link index
//           q = link flow rate (cfs)
//  Output:  none
//  Purpose: updates cumulative inflow & outflow at link's end nodes.
//
    let mut k: i32;
    let mut barrels: i32 = 1;
    let iu = i as usize;
    let n1: i32 = prj.Link[iu].node1;
    let n2: i32 = prj.Link[iu].node2;
    let q: f64 = prj.Link[iu].newFlow;
    let mut conduitLossRate: f64 = 0.0;
    let n1u = n1 as usize;
    let n2u = n2 as usize;

    // --- update total inflow & outflow at upstream/downstream nodes
    if q >= 0.0 {
        prj.Node[n1u].outflow += q;
        prj.Node[n2u].inflow += q;
    } else {
        prj.Node[n1u].inflow -= q;
        prj.Node[n2u].outflow -= q;
    }

    // --- add any uniform evap & seepage loss from conduit link
    if prj.Link[iu].type_ == CONDUIT {
        k = prj.Link[iu].subIndex;
        let ku = k as usize;
        barrels = prj.Conduit[ku].barrels as i32;
        conduitLossRate = (prj.Conduit[ku].evapLossRate + prj.Conduit[ku].seepLossRate)
            * barrels as f64;
        if conduitLossRate > 0.0 {
            // --- outfall nodes do not share evap & seepage losses
            if prj.Node[n1u].type_ != OUTFALL && prj.Node[n2u].type_ != OUTFALL {
                conduitLossRate /= 2.0;
            }
            if prj.Node[n1u].type_ != OUTFALL {
                prj.Node[n1u].outflow += conduitLossRate;
            }
            if prj.Node[n2u].type_ != OUTFALL {
                prj.Node[n2u].outflow += conduitLossRate;
            }
        }
    }

    // --- add surf. area contributions to upstream/downstream nodes
    let ln1 = prj.Link[iu].node1 as usize;
    let ln2 = prj.Link[iu].node2 as usize;
    prj.dynwave.Xnode[ln1].newSurfArea += prj.Link[iu].surfArea1 * barrels as f64;
    prj.dynwave.Xnode[ln2].newSurfArea += prj.Link[iu].surfArea2 * barrels as f64;

    // --- update summed value of dqdh at each end node
    prj.dynwave.Xnode[ln1].sumdqdh += prj.Link[iu].dqdh;
    if prj.Link[iu].type_ == PUMP {
        k = prj.Link[iu].subIndex;
        if prj.Pump[k as usize].type_ != TYPE4_PUMP {
            prj.dynwave.Xnode[n2u].sumdqdh += prj.Link[iu].dqdh;
        }
    } else {
        prj.dynwave.Xnode[n2u].sumdqdh += prj.Link[iu].dqdh;
    }
}

//=============================================================================

fn findNodeDepths(prj: &mut Project, dt: f64) -> i32 {
//
//  Input:   dt = time step (sec)
//  Output:  returns TRUE if depth change at all non-Outfall nodes is
//           within the convergence tolerance and FALSE otherwise
//  Purpose: finds new depth at all nodes and checks if convergence achieved.
//
    let mut yOld: f64;       // previous node depth (ft)

    // --- compute outfall depths based on flow in connecting link
    for i in 0..prj.Nobjects[LINK as usize] {
        link_setOutfallDepth(prj, i);
    }

    // --- compute new depth for all non-outfall nodes and determine if
    //     depth change from previous iteration is below tolerance
    // (C: an OpenMP parallel loop; the reference build runs it serially)
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        if prj.Node[iu].type_ == OUTFALL {
            continue;
        }
        yOld = prj.Node[iu].newDepth;
        setNodeDepth(prj, i, dt);
        prj.dynwave.Xnode[iu].converged = TRUE as i8;
        if (yOld - prj.Node[iu].newDepth).abs() > prj.HeadTol {
            prj.dynwave.Xnode[iu].converged = FALSE as i8;
        }
    }

    // --- return FALSE if any non-Outfall node failed to converge
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        if prj.Node[iu].type_ == OUTFALL {
            continue;
        }
        if prj.dynwave.Xnode[iu].converged as i32 == FALSE {
            return FALSE;
        }
    }
    TRUE
}

//=============================================================================

fn setNodeDepth(prj: &mut Project, i: i32, dt: f64) {
//
//  Input:   i  = node index
//           dt = time step (sec)
//  Output:  none
//  Purpose: sets depth at non-outfall node after current time step.
//
    let canPond: i32;                  // TRUE if node can pond overflows
    let isPonded: i32;                 // TRUE if node is currently ponded
    let mut isSurcharged: i32 = FALSE; // TRUE if node is surcharged
    let dQ: f64;                       // inflow minus outflow at node (cfs)
    let dV: f64;                       // change in node volume (ft3)
    let mut dy: f64;                   // change in node depth (ft)
    let mut yMax: f64;                 // max. depth at node (ft)
    let yOld: f64;                     // node depth at previous time step (ft)
    let yLast: f64;                    // previous node depth (ft)
    let mut yNew: f64;                 // new node depth (ft)
    let yCrown: f64;                   // depth to node crown (ft)
    let mut surfArea: f64;             // node surface area (ft2)
    let mut denom: f64;                // denominator term
    let mut corr: f64;                 // correction factor
    let f: f64;                        // relative surcharge depth
    let iu = i as usize;

    // --- see if node can pond water above it
    canPond = (prj.AllowPonding != 0 && prj.Node[iu].pondedArea > 0.0) as i32;
    isPonded = (canPond != 0 && prj.Node[iu].newDepth > prj.Node[iu].fullDepth) as i32;

    // --- initialize values
    yCrown = prj.Node[iu].crownElev - prj.Node[iu].invertElev;
    yOld = prj.Node[iu].oldDepth;
    yLast = prj.Node[iu].newDepth;
    prj.Node[iu].overflow = 0.0;
    surfArea = prj.dynwave.Xnode[iu].newSurfArea;
    surfArea = MAX(surfArea, prj.MinSurfArea);

    // --- determine average net flow volume into node over the time step
    dQ = prj.Node[iu].inflow - prj.Node[iu].outflow;
    dV = 0.5 * (prj.Node[iu].oldNetInflow + dQ) * dt;

    // --- determine if node is EXTRAN surcharged
    if prj.SurchargeMethod == EXTRAN {
        // --- ponded nodes don't surcharge
        if isPonded != 0 {
            isSurcharged = FALSE;
        }
        // --- closed storage units that are full are in surcharge
        else if prj.Node[iu].type_ == STORAGE {
            isSurcharged = (prj.Node[iu].surDepth > 0.0 && yLast > prj.Node[iu].fullDepth) as i32;
        }
        // --- surcharge occurs when node depth exceeds top of its highest link
        else {
            isSurcharged = (yCrown > 0.0 && yLast > yCrown) as i32;
        }
    }

    // --- if node not surcharged, base depth change on surface area
    if isSurcharged == 0 {
        dy = dV / surfArea;
        yNew = yOld + dy;

        // --- save non-ponded surface area for use in surcharge algorithm
        if isPonded == 0 {
            prj.dynwave.Xnode[iu].oldSurfArea = surfArea;
        }

        // --- apply under-relaxation to new depth estimate
        if prj.dynwave.Steps > 0 {
            yNew = (1.0 - prj.dynwave.Omega) * yLast + prj.dynwave.Omega * yNew;
        }

        // --- don't allow a ponded node to drop much below full depth
        if isPonded != 0 && yNew < prj.Node[iu].fullDepth {
            yNew = prj.Node[iu].fullDepth - FUDGE;
        }
    }
    // --- if node surcharged, base depth change on dqdh
    //     NOTE: depth change is w.r.t depth from previous
    //     iteration; also, do not apply under-relaxation.
    else {
        // --- apply correction factor for upstream terminal nodes
        corr = 1.0;
        if prj.Node[iu].degree < 0 {
            corr = 0.6;
        }

        // --- allow surface area from last non-surcharged condition
        //     to influence dqdh if depth close to crown depth
        denom = prj.dynwave.Xnode[iu].sumdqdh;
        if yLast < 1.25 * yCrown {
            f = (yLast - yCrown) / yCrown;
            denom += (prj.dynwave.Xnode[iu].oldSurfArea / dt - prj.dynwave.Xnode[iu].sumdqdh)
                * (-15.0 * f).exp();
        }

        // --- compute new estimate of node depth
        if denom == 0.0 {
            dy = 0.0;
        } else {
            dy = corr * dQ / denom;
        }
        yNew = yLast + dy;
        if yNew < yCrown {
            yNew = yCrown - FUDGE;
        }

        // --- don't allow a newly ponded node to rise much above full depth
        if canPond != 0 && yNew > prj.Node[iu].fullDepth {
            yNew = prj.Node[iu].fullDepth + FUDGE;
        }
    }

    // --- depth cannot be negative
    if yNew < 0 as f64 {
        yNew = 0.0;
    }

    // --- determine max. non-flooded depth
    yMax = prj.Node[iu].fullDepth;
    if canPond == FALSE {
        yMax += prj.Node[iu].surDepth;
    }

    // --- find flooded depth & volume
    if yNew > yMax {
        yNew = getFloodedDepth(prj, i, canPond, dV, yNew, yMax, dt);
    } else {
        prj.Node[iu].newVolume = node_getVolume(prj, i, yNew);
    }

    // --- compute change in depth w.r.t. time
    prj.dynwave.Xnode[iu].dYdT = (yNew - yOld).abs() / dt;

    // --- save new depth for node
    prj.Node[iu].newDepth = yNew;
}

//=============================================================================

fn getFloodedDepth(prj: &mut Project, i: i32, canPond: i32, dV: f64, yNew: f64,
                   yMax: f64, dt: f64) -> f64 {
//
//  Input:   i  = node index
//           canPond = TRUE if water can pond over node
//           isPonded = TRUE if water is currently ponded
//           dV = change in volume over time step (ft3)
//           yNew = current depth at node (ft)
//           yMax = max. depth at node before ponding (ft)
//           dt = time step (sec)
//  Output:  returns depth at node when flooded (ft)
//  Purpose: computes depth, volume and overflow for a flooded node.
//
    let mut yNew = yNew;
    let iu = i as usize;
    if canPond == FALSE {
        prj.Node[iu].overflow = dV / dt;
        prj.Node[iu].newVolume = prj.Node[iu].fullVolume;
        yNew = yMax;
    } else {
        prj.Node[iu].newVolume = MAX(prj.Node[iu].oldVolume + dV, prj.Node[iu].fullVolume);
        prj.Node[iu].overflow = (prj.Node[iu].newVolume
            - MAX(prj.Node[iu].oldVolume, prj.Node[iu].fullVolume)) / dt;
    }
    if prj.Node[iu].overflow < FUDGE {
        prj.Node[iu].overflow = 0.0;
    }
    yNew
}

//=============================================================================

fn getVariableStep(prj: &mut Project, maxStep: f64) -> f64 {
//
//  Input:   maxStep = user-supplied max. time step (sec)
//  Output:  returns time step (sec)
//  Purpose: finds time step that satisfies stability criterion but
//           is no greater than the user-supplied max. time step.
//
    let mut minLink: i32 = -1;          // index of link w/ min. time step
    let mut minNode: i32 = -1;          // index of node w/ min. time step
    let mut tMin: f64;                  // allowable time step (sec)
    let tMinLink: f64;                  // allowable time step for links (sec)
    let tMinNode: f64;                  // allowable time step for nodes (sec)

    // --- find stable time step for links & then nodes
    tMin = maxStep;
    tMinLink = getLinkStep(prj, tMin, &mut minLink);
    tMinNode = getNodeStep(prj, tMinLink, &mut minNode);

    // --- use smaller of the link and node time step
    tMin = tMinLink;
    if tMinNode < tMin {
        tMin = tMinNode;
        minLink = -1;
    }

    // --- update count of times the minimum node or link was critical
    stats_updateCriticalTimeCount(prj, minNode, minLink);

    // --- don't let time step go below an absolute minimum
    if tMin < prj.MinRouteStep {
        tMin = prj.MinRouteStep;
    }
    tMin
}

//=============================================================================

fn getLinkStep(prj: &Project, tMin: f64, minLink: &mut i32) -> f64 {
//
//  Input:   tMin = critical time step found so far (sec)
//  Output:  minLink = index of link with critical time step;
//           returns critical time step (sec)
//  Purpose: finds critical time step for conduits based on Courant criterion.
//
    let mut k: i32;                     // conduit index
    let mut q: f64;                     // conduit flow (cfs)
    let mut t: f64;                     // time step (sec)
    let mut tLink: f64 = tMin;          // critical link time step (sec)

    // --- examine each conduit link
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        if prj.Link[iu].type_ == CONDUIT {
            // --- skip conduits with negligible flow, area or Fr
            k = prj.Link[iu].subIndex;
            let ku = k as usize;
            q = prj.Link[iu].newFlow.abs() / prj.Conduit[ku].barrels as f64;
            if q <= FUDGE || prj.Conduit[ku].a1 <= FUDGE || prj.Link[iu].froude <= 0.01 {
                continue;
            }

            // --- compute time step to satisfy Courant condition
            t = prj.Link[iu].newVolume / prj.Conduit[ku].barrels as f64 / q;
            t = t * prj.Conduit[ku].modLength / link_getLength(prj, i);
            t = t * prj.Link[iu].froude / (1.0 + prj.Link[iu].froude) * prj.CourantFactor;

            // --- update critical link time step
            if t < tLink {
                tLink = t;
                *minLink = i;
            }
        }
    }
    tLink
}

//=============================================================================

fn getNodeStep(prj: &Project, tMin: f64, minNode: &mut i32) -> f64 {
//
//  Input:   tMin = critical time step found so far (sec)
//  Output:  minNode = index of node with critical time step;
//           returns critical time step (sec)
//  Purpose: finds critical time step for nodes based on max. allowable
//           projected change in depth.
//
    let mut maxDepth: f64;              // max. depth allowed at node (ft)
    let mut dYdT: f64;                  // change in depth per unit time (ft/sec)
    let mut t1: f64;                    // time needed to reach depth limit (sec)
    let mut tNode: f64 = tMin;          // critical node time step (sec)

    // --- find smallest time so that estimated change in nodal depth
    //     does not exceed safety factor * maxdepth
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        // --- see if node can be skipped
        if prj.Node[iu].type_ == OUTFALL {
            continue;
        }
        if prj.Node[iu].newDepth <= FUDGE {
            continue;
        }
        if prj.Node[iu].newDepth + FUDGE >= prj.Node[iu].crownElev - prj.Node[iu].invertElev {
            continue;
        }

        // --- define max. allowable depth change using crown elevation
        maxDepth = (prj.Node[iu].crownElev - prj.Node[iu].invertElev) * 0.25;
        if maxDepth < FUDGE {
            continue;
        }
        dYdT = prj.dynwave.Xnode[iu].dYdT;
        if dYdT < FUDGE {
            continue;
        }

        // --- compute time to reach max. depth & compare with critical time
        t1 = maxDepth / dYdT;
        if t1 < tNode {
            tNode = t1;
            *minNode = i;
        }
    }
    tNode
}

//=============================================================================
