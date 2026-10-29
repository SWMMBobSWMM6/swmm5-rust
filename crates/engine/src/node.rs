//! node.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   node.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     10/29/22   (Build 5.2.2)
//   Author:   L. Rossman
//
//   Conveyance system node functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Ponded area property for storage nodes deprecated.
//   - Support for Green-Ampt seepage from bottom and sides of storage node added.
//   - Storage node evap. & seepage losses now computed at start of each routing
//     time step.
//   Build 5.1.008:
//   - Support added for sending outfall discharge to a subcatchment.
//   Build 5.1.010:
//   - Storage losses now based on node's new volume instead of old volume.
//   Build 5.1.013:
//   - A surcharge depth can now be applied to storage nodes.
//   - A negative inflow is now assigned to an Outfall node with backflow.
//   Build 5.1.014:
//   - Fixed bug in storage_losses() that affected storage exfiltration.
//   Build 5.1.015:
//   - Fatal error issued if a storage node's area curve produces a negative
//     volume when extrapolated to the node's full depth.
//   Build 5.2.0:
//   - Support added Streets and Inlets.
//   - Support added for analytical storage shapes.
//   Build 5.2.1:
//   - Warning no longer issued when node full depth is increased to match
//     crown of highest connecting link.
//   - a2 term for paraboloid shaped storage units was corrected
//   Build 5.2.2:
//   - Warning restored for node full depth being increased to crown of highest
//     connecting link.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Local Declarations
//-----------------------------------------------------------------------------
#[derive(Clone, Copy, Default)]
struct TStorageVol {
    k: i32, // storage unit index
    v: f64, // storage unit volume (ft3)
}

/// The file-scope variables of node.c (it has none).
#[derive(Default)]
pub struct NodeState {
}

//=============================================================================

/// Called by: input.
pub fn node_readParams(prj: &mut Project, j: i32, type_: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = node index
//           type = node type code
//           k = index of node type
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads node properties from a tokenized line of input.
//
{
    match type_ {
        JUNCTION => return junc_readParams(prj, j, k, tok, ntoks),
        OUTFALL => return outfall_readParams(prj, j, k, tok, ntoks),
        STORAGE => return storage_readParams(prj, j, k, tok, ntoks),
        DIVIDER => return divider_readParams(prj, j, k, tok, ntoks),
        _ => return 0,
    }
}

//=============================================================================

fn node_setParams(prj: &mut Project, j: i32, type_: i32, k: i32, x: &[f64])
//
//  Input:   j = node index
//           type = node type code
//           k = index of node type
//           x[] = array of property values
//  Output:  none
//  Purpose: assigns property values to a node.
//
{
    let ju = j as usize;
    let ku = k as usize;
    prj.Node[ju].type_ = type_;
    prj.Node[ju].subIndex = k;
    prj.Node[ju].invertElev = x[0] / UCF(prj, LENGTH);
    prj.Node[ju].crownElev = prj.Node[ju].invertElev;
    prj.Node[ju].initDepth = 0.0;
    prj.Node[ju].newVolume = 0.0;
    prj.Node[ju].fullVolume = 0.0;
    prj.Node[ju].fullDepth = 0.0;
    prj.Node[ju].surDepth = 0.0;
    prj.Node[ju].pondedArea = 0.0;
    prj.Node[ju].degree = 0;
    prj.Node[ju].inlet = NO_INLET;
    match type_ {
        JUNCTION => {
            prj.Node[ju].fullDepth = x[1] / UCF(prj, LENGTH);
            prj.Node[ju].initDepth = x[2] / UCF(prj, LENGTH);
            prj.Node[ju].surDepth = x[3] / UCF(prj, LENGTH);
            prj.Node[ju].pondedArea = x[4] / (UCF(prj, LENGTH) * UCF(prj, LENGTH));
        }

        OUTFALL => {
            prj.Outfall[ku].type_ = f64_to_i32(x[1]);
            prj.Outfall[ku].fixedStage = x[2] / UCF(prj, LENGTH);
            prj.Outfall[ku].tideCurve = f64_to_i32(x[3]);
            prj.Outfall[ku].stageSeries = f64_to_i32(x[4]);
            prj.Outfall[ku].hasFlapGate = f64_to_i8(x[5]);
            prj.Outfall[ku].routeTo = f64_to_i32(x[6]);
            prj.Outfall[ku].wRouted = Vec::new();
            if prj.Outfall[ku].routeTo >= 0 {
                prj.Outfall[ku].wRouted = vec![0.0; prj.Nobjects[POLLUT as usize] as usize];
            }
        }

        STORAGE => {
            prj.Node[ju].fullDepth = x[1] / UCF(prj, LENGTH);
            prj.Node[ju].initDepth = x[2] / UCF(prj, LENGTH);
            prj.Storage[ku].shape = f64_to_i32(x[3]);
            prj.Storage[ku].a1 = x[4];
            prj.Storage[ku].a2 = x[5];
            prj.Storage[ku].a0 = x[6];
            prj.Storage[ku].aCurve = f64_to_i32(x[7]);
            prj.Node[ju].surDepth = x[8] / UCF(prj, LENGTH);
            prj.Storage[ku].fEvap = x[9];
        }

        DIVIDER => {
            prj.Divider[ku].link = f64_to_i32(x[1]);
            prj.Divider[ku].type_ = f64_to_i32(x[2]);
            prj.Divider[ku].flowCurve = f64_to_i32(x[3]);
            prj.Divider[ku].qMin = x[4] / UCF(prj, FLOW);
            prj.Divider[ku].dhMax = x[5];
            prj.Divider[ku].cWeir = x[6];
            prj.Node[ju].fullDepth = x[7] / UCF(prj, LENGTH);
            prj.Node[ju].initDepth = x[8] / UCF(prj, LENGTH);
            prj.Node[ju].surDepth = x[9] / UCF(prj, LENGTH);
            prj.Node[ju].pondedArea = x[10] / (UCF(prj, LENGTH) * UCF(prj, LENGTH));
        }
        _ => {}
    }
}

//=============================================================================

/// Called by: project.
pub fn node_validate(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: validates a node's properties.
//
{
    let ju = j as usize;

    // --- see if full depth was increased to accommodate conduit crown
    if prj.Node[ju].fullDepth > prj.Node[ju].oldDepth && prj.Node[ju].oldDepth > 0.0 {
        let id = prj.Node[ju].ID.clone();
        report_writeWarningMsg(prj, WARN02, &id);
    }

    // --- check that initial depth does not exceed max. depth
    if prj.Node[ju].initDepth > prj.Node[ju].fullDepth + prj.Node[ju].surDepth {
        let id = prj.Node[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_NODE_DEPTH, &id);
    }

    // --- check for negative volume for storage node at full depth
    if prj.Node[ju].type_ == STORAGE {
        let fullDepth = prj.Node[ju].fullDepth;
        if node_getVolume(prj, j, fullDepth) < 0.0 {
            let id = prj.Node[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_STORAGE_VOLUME, &id);
        }
    }

    if prj.Node[ju].type_ == DIVIDER {
        divider_validate(prj, j);
    }

    // --- initialize dry weather inflows
    let mut i = 0usize;
    while i < prj.Node[ju].dwfInflow.len() {
        inflow_initDwfInflow(&prj.Pattern, &mut prj.Node[ju].dwfInflow[i]);
        i += 1;
    }
}

//=============================================================================

/// Called by: project.
pub fn node_initState(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: initializes a node's state variables at start of simulation.
//
{
    let ju = j as usize;
    let mut p: i32;
    let k: i32;

    // --- initialize depth
    prj.Node[ju].oldDepth = prj.Node[ju].initDepth;
    prj.Node[ju].newDepth = prj.Node[ju].oldDepth;
    prj.Node[ju].crownElev = prj.Node[ju].invertElev;

    let fullDepth = prj.Node[ju].fullDepth;
    prj.Node[ju].fullVolume = node_getVolume(prj, j, fullDepth);
    let oldDepth = prj.Node[ju].oldDepth;
    prj.Node[ju].oldVolume = node_getVolume(prj, j, oldDepth);
    prj.Node[ju].newVolume = prj.Node[ju].oldVolume;

    // --- initialize water quality state
    p = 0;
    while p < prj.Nobjects[POLLUT as usize] {
        prj.Node[ju].oldQual[p as usize] = 0.0;
        prj.Node[ju].newQual[p as usize] = 0.0;
        p += 1;
    }

    // --- initialize any inflow
    prj.Node[ju].oldLatFlow = 0.0;
    prj.Node[ju].newLatFlow = 0.0;
    prj.Node[ju].apiExtInflow = 0.0;
    prj.Node[ju].losses = 0.0;

    // --- initialize storage nodes
    if prj.Node[ju].type_ == STORAGE {
        // --- set hydraulic residence time to 0
        k = prj.Node[ju].subIndex;
        prj.Storage[k as usize].hrt = 0.0;

        // --- initialize exfiltration properties
        if prj.Storage[k as usize].exfil.is_some() {
            exfil_initState(prj, k);
        }
    }

    // --- initialize flow stream routed from outfall onto a subcatchment
    if prj.Node[ju].type_ == OUTFALL {
        let k = prj.Node[ju].subIndex as usize;
        if prj.Outfall[k].routeTo >= 0 {
            prj.Outfall[k].vRouted = 0.0;
            p = 0;
            while p < prj.Nobjects[POLLUT as usize] {
                prj.Outfall[k].wRouted[p as usize] = 0.0;
                p += 1;
            }
        }
    }
}

//=============================================================================

/// Called by: routing.
pub fn node_setOldHydState(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: replaces a node's old hydraulic state values with new ones.
//
{
    let node = &mut prj.Node[j as usize];
    node.oldDepth = node.newDepth;
    node.oldVolume = node.newVolume;
    node.oldFlowInflow = node.inflow;
    node.oldNetInflow = node.inflow - node.outflow;
}

//=============================================================================

/// Called by: routing.
pub fn node_setOldQualState(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: replaces a node's old water quality state values with new ones.
//
{
    let ju = j as usize;
    let mut p: i32 = 0;
    while p < prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        prj.Node[ju].oldQual[pu] = prj.Node[ju].newQual[pu];
        prj.Node[ju].newQual[pu] = 0.0;
        p += 1;
    }
}

//=============================================================================

/// Called by: routing.
pub fn node_initFlows(prj: &mut Project, j: i32, tStep: f64)
//
//  Input:   j = node index
//           tStep = time step (sec)
//  Output:  none
//  Purpose: initializes a node's inflow/outflow/overflow at start of time step.
//
{
    let node = &mut prj.Node[j as usize];

    // --- initialize inflow & outflow
    node.inflow = node.newLatFlow;
    node.outflow = node.losses;

    // --- set overflow to any excess stored volume
    if node.newVolume > node.fullVolume {
        node.overflow = (node.newVolume - node.fullVolume) / tStep;
    } else {
        node.overflow = 0.0;
    }
}

//=============================================================================

/// Called by: flowrout.
pub fn node_getDepth(prj: &mut Project, j: i32, v: f64) -> f64
//
//  Input:   j = node index
//           v = volume (ft3)
//  Output:  returns depth of water at a node (ft)
//  Purpose: computes a node's water depth from its volume.
//
{
    match prj.Node[j as usize].type_ {
        STORAGE => storage_getDepth(prj, j, v),
        _ => 0.0,
    }
}

//=============================================================================

/// Called by: dynwave, flowrout.
pub fn node_getVolume(prj: &mut Project, j: i32, d: f64) -> f64
//
//  Input:   j = node index
//           d = water depth (ft)
//  Output:  returns volume of water at a node (ft3)
//  Purpose: computes volume stored at a node from its water depth.
//
{
    let ju = j as usize;
    match prj.Node[ju].type_ {
        STORAGE => storage_getVolume(prj, j, d),

        _ => {
            if prj.Node[ju].fullDepth > 0.0 {
                prj.Node[ju].fullVolume * (d / prj.Node[ju].fullDepth)
            } else {
                0.0
            }
        }
    }
}

//=============================================================================

/// Called by: dynwave, treatmnt.
pub fn node_getSurfArea(prj: &mut Project, j: i32, d: f64) -> f64
//
//  Input:   j = node index
//           d = water depth (ft)
//  Output:  returns surface area of water at a node (ft2)
//  Purpose: computes surface area of water stored at a node from water depth.
//
{
    match prj.Node[j as usize].type_ {
        STORAGE => storage_getSurfArea(prj, j, d),
        _ => 0.0,
    }
}

//=============================================================================

/// Called by: link.
pub fn node_getOutflow(prj: &mut Project, j: i32, k: i32) -> f64
//
//  Input:   j = node index
//           k = link index
//  Output:  returns flow rate (cfs)
//  Purpose: computes outflow from node available for inflow into a link.
//
{
    let ju = j as usize;
    match prj.Node[ju].type_ {
        DIVIDER => divider_getOutflow(prj, j, k),
        STORAGE => storage_getOutflow(prj, j, k),
        _ => prj.Node[ju].inflow + prj.Node[ju].overflow,
    }
}

//=============================================================================

/// Called by: dynwave, flowrout.
pub fn node_getMaxOutflow(prj: &mut Project, j: i32, mut q: f64, tStep: f64) -> f64
//
//  Input:   j = node index
//           q = original outflow rate (cfs)
//           tStep = time step (sec)
//  Output:  returns modified flow rate (cfs)
//  Purpose: limits outflow rate from a node with storage volume.
//
{
    let ju = j as usize;
    let qMax: f64;
    if prj.Node[ju].fullVolume > 0.0 {
        qMax = prj.Node[ju].inflow + prj.Node[ju].oldVolume / tStep;
        if q > qMax {
            q = qMax;
        }
    }
    MAX(0.0, q)
}

//=============================================================================

/// Called by: routing.
pub fn node_getSystemOutflow(prj: &mut Project, j: i32, isFlooded: &mut i32) -> f64
//
//  Input:   j = node index
//           isFlooded = TRUE if node becomes flooded
//  Output:  returns flow rate lost from system (cfs)
//  Purpose: computes flow rate at outfalls and flooded nodes.
//
{
    let routeModel = prj.RouteModel;
    let node = &mut prj.Node[j as usize];
    let mut outflow: f64 = 0.0;

    // --- assume there is no flooding
    *isFlooded = FALSE;

    // --- if node is an outfall
    if node.type_ == OUTFALL {
        // --- node receives inflow from outfall conduit
        if node.outflow == 0.0 {
            outflow = node.inflow;
        }
        // --- node sends flow into outfall conduit
        //     (therefore it has a negative outflow)
        else {
            if node.inflow == 0.0 {
                outflow = -node.outflow;
                node.inflow = outflow.abs();
            }
        }

        // --- set overflow and volume to 0
        node.overflow = 0.0;
        node.newVolume = 0.0;
    }
    // --- node is a terminal node under Steady or Kin. Wave routing
    else if routeModel != DW && node.degree == 0 && node.type_ != STORAGE {
        if node.outflow == 0.0 {
            outflow = node.inflow;
        }
        node.overflow = 0.0;
        node.newVolume = 0.0;
    }
    // --- otherwise node is an interior node and any
    //     overflow is considered as system outflow and flooding
    else {
        if node.newVolume <= node.fullVolume {
            outflow = node.overflow;
        }
        if outflow > 0.0 {
            *isFlooded = TRUE;
        }
    }
    outflow
}

//=============================================================================

/// Called by: output.
pub fn node_getResults(prj: &mut Project, j: i32, wt: f64, x: &mut [f32])
//
//  Input:   j = node index
//           f = weighting factor
//           x[] = array of nodal reporting variables
//  Output:  none
//  Purpose: computes weighted average of old and new results at a node.
//
{
    let f = wt;
    let ju = j as usize;
    let mut p: i32;
    let mut z: f64;
    let f1: f64 = 1.0 - f;

    z = (f1 * prj.Node[ju].oldDepth + f * prj.Node[ju].newDepth) * UCF(prj, LENGTH);
    x[NODE_DEPTH as usize] = z as f32;
    z = prj.Node[ju].invertElev * UCF(prj, LENGTH);
    x[NODE_HEAD as usize] = x[NODE_DEPTH as usize] + z as f32;
    z = (f1 * prj.Node[ju].oldVolume + f * prj.Node[ju].newVolume) * UCF(prj, VOLUME);
    x[NODE_VOLUME as usize] = z as f32;
    z = (f1 * prj.Node[ju].oldLatFlow + f * prj.Node[ju].newLatFlow) * UCF(prj, FLOW);
    x[NODE_LATFLOW as usize] = z as f32;
    z = (f1 * prj.Node[ju].oldFlowInflow + f * prj.Node[ju].inflow) * UCF(prj, FLOW);
    x[NODE_INFLOW as usize] = z as f32;
    z = prj.Node[ju].overflow * UCF(prj, FLOW);
    x[NODE_OVERFLOW as usize] = z as f32;

    if prj.IgnoreQuality == 0 {
        p = 0;
        while p < prj.Nobjects[POLLUT as usize] {
            z = f1 * prj.Node[ju].oldQual[p as usize] + f * prj.Node[ju].newQual[p as usize];
            x[(NODE_QUAL + p) as usize] = z as f32;
            p += 1;
        }
    }
}

//=============================================================================

/// Called by: link.
pub fn node_setOutletDepth(prj: &mut Project, j: i32, yNorm: f64, yCrit: f64, z: f64)
//
//  Input:   j = node index
//           yNorm = normal flow depth (ft)
//           yCrit = critical flow depth (ft)
//           z = offset of connecting outfall link from node invert (ft)
//  Output:  none
//  Purpose: sets water depth at a node that serves as an outlet point.
//
{
    match prj.Node[j as usize].type_ {
        // --- do nothing if outlet is a storage unit
        STORAGE => {
            return;
        }

        // --- if outlet is a designated outfall then use outfall's specs
        OUTFALL => {
            outfall_setOutletDepth(prj, j, yNorm, yCrit, z);
        }

        // --- for all other nodes, use min. of critical & normal depths
        _ => {
            if z > 0.0 {
                prj.Node[j as usize].newDepth = 0.0;
            } else {
                prj.Node[j as usize].newDepth = MIN(yNorm, yCrit);
            }
        }
    }
}

//=============================================================================

/// Called by: dynwave.
#[allow(unused_assignments)] // C computes the ponded depth d and never uses it
pub fn node_getPondedArea(prj: &mut Project, j: i32, mut d: f64) -> f64
//
//  Input:   j = node index
//           d = water depth (ft)
//  Output:  returns surface area of water at a node (ft2)
//  Purpose: computes surface area of water at a node based on depth.
//
{
    let ju = j as usize;
    let mut a: f64;

    // --- use regular getSurfArea function if node not flooded
    if d <= prj.Node[ju].fullDepth || prj.Node[ju].pondedArea == 0.0 {
        return node_getSurfArea(prj, j, d);
    }

    // --- compute ponded depth
    d = d - prj.Node[ju].fullDepth;

    // --- use ponded area for flooded node
    a = prj.Node[ju].pondedArea;
    if a <= 0.0 {
        let fullDepth = prj.Node[ju].fullDepth;
        a = node_getSurfArea(prj, j, fullDepth);
    }
    a
}

//=============================================================================

/// Called by: routing.
pub fn node_getLosses(prj: &mut Project, j: i32, tStep: f64) -> f64
//
//  Input:   j = node index
//           tStep = time step (sec)
//  Output:  returns water loss rate at node (ft3)
//  Purpose: computes the rates of evaporation and infiltration over a given
//           time step for a node.
//
{
    if prj.Node[j as usize].type_ == STORAGE {
        storage_getLosses(prj, j, tStep)
    } else {
        0.0
    }
}

//=============================================================================
//                   J U N C T I O N   M E T H O D S
//=============================================================================

fn junc_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = node index
//           k = junction index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error message
//  Purpose: reads a junction's properties from a tokenized line of input.
//
//  Format of input line is:
//     nodeID  elev  maxDepth  initDepth  surDepth  aPond
{
    let mut i: i32;
    // C's x[6]: x[5] is never written or read.
    let mut x: [f64; 6] = [0.0; 6];
    let id: Option<Vec<u8>>;

    if ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, NODE, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- parse invert elev., max. depth, init. depth, surcharged depth,
    //     & ponded area values
    i = 1;
    while i <= 5 {
        x[(i - 1) as usize] = 0.0;
        if i < ntoks {
            if getDouble(&tok[i as usize], &mut x[(i - 1) as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
            }
        }
        i += 1;
    }

    // --- check for non-negative values (except for invert elev.)
    i = 1;
    while i <= 4 {
        if x[i as usize] < 0.0 {
            // tok[i+1] exists here: a negative x[i] was parsed from tok[i+1]
            return error_setInpError(prj, ERR_NUMBER, &tok[(i + 1) as usize]);
        }
        i += 1;
    }

    // --- add parameters to junction object
    prj.Node[j as usize].ID = id.unwrap();
    node_setParams(prj, j, JUNCTION, k, &x);
    0
}

//=============================================================================
//                   S T O R A G E   M E T H O D S
//=============================================================================

fn storage_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = node index
//           k = storage unit index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error message
//  Purpose: reads a storage unit's properties from a tokenized line of input.
//
//  Format of input line is:
//     nodeID  elev  maxDepth  initDepth  curveType a1 a2 a0 surDepth fEvap (infil)
//     nodeID  elev  maxDepth  initDepth  TABULAR   curveID  surDepth fEvap (infil)
//             x[0]  x[1]      x[2]       x[3]       x[4..7]  x[8]     x[9]
{
    let mut i: i32;
    let m: i32;
    let mut n: i32;
    let mut x: [f64; 10] = [0.0; 10];
    // C leaves y[] uninitialized for a TABULAR shape; it is then never read.
    let mut y: [f64; 3] = [0.0; 3];
    let A: f64; //base semi-axis length & width for conical shape
    let B: f64;
    let L: f64; //base length & width for pyramidal shape
    let W: f64;
    let Z: f64; //run over rise for conical & pyramidal sides
    let id: Option<Vec<u8>>;

    // --- get ID name
    if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, NODE, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- get invert elev, max. depth, & init. depth
    i = 1;
    while i <= 3 {
        if getDouble(&tok[i as usize], &mut x[(i - 1) as usize]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
        }
        i += 1;
    }

    // --- get surf. area relation type
    m = findmatch(&tok[4], RelationWords);
    if m < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[4]);
    }
    x[3] = m as f64;
    x[4] = 0.0; // a1
    x[5] = 0.0; // a2
    x[6] = 0.0; // a0
    x[7] = -1.0; // curveID
    x[8] = 0.0; // surDepth
    x[9] = 0.0; // fEvap

    // --- get surf. area curve name
    if m == TABULAR {
        i = project_findObject(prj, CURVE, &tok[5]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[5]);
        }
        x[7] = i as f64;
        n = 6;
    }
    // --- get surf. area function coeffs.
    else {
        if ntoks < 8 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = 5;
        while i <= 7 {
            if getDouble(&tok[i as usize], &mut y[(i - 5) as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
            }
            i += 1;
        }
        n = 8;
    }

    // --- check for valid data
    match m {
        FUNCTIONAL => {
            // area at 0 depth can't be negative
            if y[2] < 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[7]);
            }
        }

        CYLINDRICAL | CONICAL | PARABOLOID | PYRAMIDAL => {
            // length or width can't be <= 0, slope can't be < 0
            if y[0] <= 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[5]);
            }
            if y[1] <= 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[6]);
            }
            if y[2] < 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[7]);
            }
        }
        _ => {}
    }
    // --- height of paraboloid shape can't be 0
    if m == PARABOLOID && y[2] == 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[7]);
    }

    // --- convert supplied parameters to coeffs. in surface area equation
    match m {
        FUNCTIONAL => {
            x[4] = y[0];
            x[5] = y[1];
            x[6] = y[2];
        }

        CYLINDRICAL => {
            A = y[0] / 2.; // base semi-axis length
            B = y[1] / 2.; // base semi axis width
            x[4] = 0.0; // a1 term
            x[5] = 0.0; // a2 term
            x[6] = PI * A * B; // a0 term
        }

        CONICAL => {
            A = y[0] / 2.; // base semi-axis length
            B = y[1] / 2.; // base semi axis width
            Z = y[2]; // side slope
            x[4] = 2.0 * PI * B * Z; // linear coeff.
            x[5] = PI * B / A * Z * Z; // quadratic coeff.
            x[6] = PI * A * B; // constant
        }

        PARABOLOID => {
            A = y[0] / 2.; // top semi-axis length
            B = y[1] / 2.; // top semi-axis width
            Z = y[2]; // top height
            x[4] = PI * A * B / Z; // a1 term
            x[5] = 0.0; // a2 term
            x[6] = 0.0; // a0 term
        }

        PYRAMIDAL => {
            L = y[0];
            W = y[1];
            Z = y[2];
            x[4] = 2.0 * (L + W) * Z; // linear coeff.
            x[5] = 4.0 * Z * Z; // quadratic coeff.
            x[6] = L * W; // constant
        }
        _ => {}
    }

    // --- get surcharge depth if present
    if ntoks > n {
        if getDouble(&tok[n as usize], &mut x[8]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[n as usize]);
        }
        n += 1;
    }

    // --- get evaporation fraction if present
    if ntoks > n {
        if getDouble(&tok[n as usize], &mut x[9]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[n as usize]);
        }
        n += 1;
    }

    // --- add parameters to storage unit object
    prj.Node[j as usize].ID = id.unwrap();
    node_setParams(prj, j, STORAGE, k, &x);

    // --- read exfiltration parameters if present
    if ntoks > n {
        return exfil_readStorageParams(prj, k, tok, ntoks, n);
    }
    0
}

//=============================================================================

#[allow(unused_assignments)] // i: C's initial value, reassigned in the TABULAR case
fn storage_getDepth(prj: &mut Project, j: i32, mut v: f64) -> f64
//
//  Input:   j = node index
//           v = volume (ft3)
//  Output:  returns depth of water at a storage node (ft)
//  Purpose: computes a storage node's water depth from its volume.
//
{
    let ju = j as usize;
    let k: i32 = prj.Node[ju].subIndex;
    let mut i: i32 = prj.Storage[k as usize].aCurve;
    let shape: i32 = prj.Storage[k as usize].shape;
    let a0: f64 = prj.Storage[k as usize].a0;
    let a1: f64 = prj.Storage[k as usize].a1;
    let a2: f64 = prj.Storage[k as usize].a2;
    let mut d: f64;
    let e: f64;
    let mut storageVol = TStorageVol::default();

    // --- return max depth if a max. volume has been computed
    //     and volume is > max. volume
    if prj.Node[ju].fullVolume > 0.0 && v >= prj.Node[ju].fullVolume {
        return prj.Node[ju].fullDepth;
    }
    if v == 0.0 {
        return 0.0;
    }

    // --- convert volume to user's units
    v *= UCF(prj, VOLUME);
    storageVol.k = k;
    storageVol.v = v;

    match shape {
        TABULAR => {
            i = prj.Storage[k as usize].aCurve;
            if i >= 0 {
                d = table_getStorageDepth(&mut prj.Curve[i as usize], v);
            } else {
                d = 0.0;
            }
        }

        CYLINDRICAL => {
            // area = a0; v = a0*d;
            d = v / a0;
        }

        PARABOLOID => {
            // area = a1*d; v = (a1/2)*d^2
            d = (2.0 * v / a1).sqrt();
        }

        FUNCTIONAL => {
            // area = a0 + a1; v = (a0 + a1) * d
            if a2 == 0.0 {
                d = v / (a0 + a1);
            }
            // area = a1*d^a2; v = a1/(a2+1)*d^(a2+1)
            else if a0 == 0.0 {
                e = 1.0 / (a2 + 1.0);
                d = (v / (a1 * e)).powf(e);
            }
            // area = a0 + a1*d; v = a0*d + (a1/2)*d^2
            else if a2 == 1.0 && a1 > 0.0 {
                d = ((a0 * a0 + 2. * a1 * v).sqrt() - a0) / a1;
            } else
            // area = a0 + a1*d^a2
            {
                d = v / (a0 + a1);
                let x2 = prj.Node[ju].fullDepth * UCF(prj, LENGTH);
                findroot_Newton(0.0, x2, &mut d, 0.001, &mut |y: f64, f: &mut f64, df: &mut f64| {
                    storage_getVolDiff(prj, y, f, df, &storageVol)
                });
            }
        }

        CONICAL | PYRAMIDAL => {
            // area = a0 + a1*d + a2*d^2; v = a0*d + (a1/2)*d^2 + (a2/3)*d^3
            d = v / a0;
            let x2 = prj.Node[ju].fullDepth * UCF(prj, LENGTH);
            findroot_Newton(0.0, x2, &mut d, 0.001, &mut |y: f64, f: &mut f64, df: &mut f64| {
                storage_getVolDiff(prj, y, f, df, &storageVol)
            });
        }

        _ => {
            d = 0.0;
        }
    }

    d /= UCF(prj, LENGTH);
    if d > prj.Node[ju].fullDepth {
        d = prj.Node[ju].fullDepth;
    }
    d
}

//=============================================================================

fn storage_getVolDiff(prj: &mut Project, y: f64, f: &mut f64, df: &mut f64, p: &TStorageVol)
//
//  Input:   y = depth of water (ft)
//           p = pointer to a TStorageVol object
//  Output:  f = volume of water (ft3)
//           df = dVolume/dDepth ( = surface area)(ft2)
//  Purpose: computes volume difference and its derivative at a storage node
//           using the node's area versus depth function.
//
{
    let k: i32;

    // --- cast void pointer p to a TStorageVol object
    let storageVol: &TStorageVol = p;
    k = storageVol.k;

    // --- compute volume & surface area at depth y
    //     (EPA passes the storage unit index k where these functions
    //      expect a node index; reproduced as written. Node[k] is then
    //      some node whose subIndex is <= k < Nnodes[STORAGE], so the
    //      Storage[] read stays in bounds; it is the wrong unit unless
    //      node k is this storage unit. y and v are in user units.)
    *f = storage_getVolume(prj, k, y) - storageVol.v;
    *df = storage_getSurfArea(prj, k, y);
}

//=============================================================================

fn storage_getVolume(prj: &mut Project, j: i32, mut d: f64) -> f64
//
//  Input:   j = node index
//           d = depth (ft)
//  Output:  returns volume of stored water (ft3)
//  Purpose: computes a storage node's water volume from its depth.
//
{
    let ju = j as usize;
    let k = prj.Node[ju].subIndex as usize;
    let i: i32;
    let n: f64;
    let v: f64;

    // --- return full volume if depth >= max. depth
    if d == 0.0 {
        return 0.0;
    }
    if d >= prj.Node[ju].fullDepth && prj.Node[ju].fullVolume > 0.0 {
        return prj.Node[ju].fullVolume;
    }

    match prj.Storage[k].shape {
        // --- for tabular shape function, use end area method
        TABULAR => {
            i = prj.Storage[k].aCurve;
            if i >= 0 {
                let x = d * UCF(prj, LENGTH);
                return table_getStorageVolume(&mut prj.Curve[i as usize], x) / UCF(prj, VOLUME);
            } else {
                return 0.0;
            }
        }

        // --- for FUNCTIONAL relation, integrate a0 + a1*d^a2
        FUNCTIONAL => {
            d *= UCF(prj, LENGTH);
            n = prj.Storage[k].a2 + 1.0;
            v = (prj.Storage[k].a0 * d) + prj.Storage[k].a1 / n * d.powf(n);
            return v / UCF(prj, VOLUME);
        }

        // --- for other shapes evaluate cubic eqn. a0*d + (a1/2)*d^2 + (a2/3)*d^3
        CYLINDRICAL | CONICAL | PARABOLOID | PYRAMIDAL => {
            d *= UCF(prj, LENGTH);
            v = d * (prj.Storage[k].a0 + d * (prj.Storage[k].a1 / 2.0 + d * prj.Storage[k].a2 / 3.0));
            return v / UCF(prj, VOLUME);
        }

        _ => return 0.0,
    }
}

//=============================================================================

fn storage_getSurfArea(prj: &mut Project, j: i32, mut d: f64) -> f64
//
//  Input:   j = node index
//           d = depth (ft)
//  Output:  returns surface area (ft2)
//  Purpose: computes a storage node's surface area from its water depth.
//
{
    let mut area: f64 = 0.0;
    let k = prj.Node[j as usize].subIndex as usize;
    let i: i32;

    match prj.Storage[k].shape {
        // --- for tabular shape function, use table look-up
        TABULAR => {
            i = prj.Storage[k].aCurve;
            if i >= 0 {
                let x = d * UCF(prj, LENGTH);
                area = table_lookupEx(&mut prj.Curve[i as usize], x);
            }
        }

        // --- for FUNCTIONAL relation, evaluate a0 + a1*d^a2
        FUNCTIONAL => {
            area = prj.Storage[k].a0 + prj.Storage[k].a1 * (d * UCF(prj, LENGTH)).powf(prj.Storage[k].a2);
        }

        // --- for other shapes, evaluate quadratic a0 + a1*d + a2*d^2
        CYLINDRICAL | CONICAL | PARABOLOID | PYRAMIDAL => {
            d *= UCF(prj, LENGTH);
            area = prj.Storage[k].a0 + d * (prj.Storage[k].a1 + d * prj.Storage[k].a2);
        }

        _ => return 0.0,
    }
    area / UCF(prj, LENGTH) / UCF(prj, LENGTH)
}

//=============================================================================

fn storage_getOutflow(prj: &mut Project, j: i32, i: i32) -> f64
//
//  Input:   j = node index
//           i = link index
//  Output:  returns flow from storage node into conduit link (cfs)
//  Purpose: finds outflow from a storage node into its connecting conduit link
//           ( non-conduit links have their own getInflow functions).
//
{
    let iu = i as usize;
    let k: i32;
    let a: f64;
    let y: f64;

    // --- link must be a conduit
    if prj.Link[iu].type_ != CONDUIT {
        return 0.0;
    }

    // --- find depth of water in conduit
    y = prj.Node[j as usize].newDepth - prj.Link[iu].offset1;

    // --- return 0 if conduit empty or full flow if full
    if y <= 0.0 {
        return 0.0;
    }
    if y >= prj.Link[iu].xsect.yFull {
        return prj.Link[iu].qFull;
    }

    // --- if partially full, return normal flow
    k = prj.Link[iu].subIndex;
    a = linkXsect_getAofY(prj, i, y);
    prj.Conduit[k as usize].beta * linkXsect_getSofA(prj, i, a)
}

//=============================================================================

fn storage_getLosses(prj: &mut Project, j: i32, tStep: f64) -> f64
//
//  Input:   j = node index
//           tStep = time step (sec)
//  Output:  returns evaporation + infiltration rate (cfs)
//  Purpose: computes combined rate of water evaporated & infiltrated from
//           a storage node.
//
{
    let ju = j as usize;
    let k: i32;
    let depth: f64;
    let area: f64;
    let mut evapRate: f64;
    let mut exfilRate: f64 = 0.0;
    let totalLoss: f64;
    let lossRatio: f64;

    // --- get node's evap. rate (ft/s) &  exfiltration object
    k = prj.Node[ju].subIndex;
    let ku = k as usize;
    evapRate = prj.Evap.rate * prj.Storage[ku].fEvap;
    let hasExfil = prj.Storage[ku].exfil.is_some();

    // --- if either of these apply
    if evapRate > 0.0 || hasExfil {
        // --- obtain storage depth & surface area
        depth = prj.Node[ju].newDepth;
        area = storage_getSurfArea(prj, j, depth);

        // --- compute evap rate over this area (cfs)
        if prj.Node[ju].newVolume > FUDGE {
            evapRate = area * evapRate;
        }

        // --- find exfiltration rate (cfs) through bottom and side banks
        if hasExfil {
            exfilRate = exfil_getLoss(
                &mut prj.infil,
                &prj.Adjust,
                prj.Storage[ku].exfil.as_deref_mut().unwrap(),
                tStep,
                depth,
                area,
            );
        }

        // --- total loss over time step cannot exceed stored volume
        totalLoss = (evapRate + exfilRate) * tStep;
        if totalLoss > prj.Node[ju].newVolume {
            lossRatio = prj.Node[ju].newVolume / totalLoss;
            evapRate *= lossRatio;
            exfilRate *= lossRatio;
        }
    }

    // --- save evap & infil losses at the node
    let sub = prj.Node[ju].subIndex as usize;
    prj.Storage[sub].evapLoss = evapRate * tStep;
    prj.Storage[sub].exfilLoss = exfilRate * tStep;
    evapRate + exfilRate
}

//=============================================================================
//                   D I V I D E R   M E T H O D S
//=============================================================================

fn divider_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = node index
//           k = divider index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error message
//  Purpose: reads a flow divider's properties from a tokenized line of input.
//
//  Format of input line is:
//    nodeID  elev  divLink  TABULAR  curveID (optional params)
//    nodeID  elev  divLink  OVERFLOW (optional params)
//    nodeID  elev  divLink  CUTOFF  qCutoff (optional params)
//    nodeID  elev  divLink  WEIR    qMin  dhMax  cWeir (optional params)
//  where optional params are:
//    maxDepth  initDepth  surDepth  aPond
//
{
    let mut i: i32;
    let mut m: i32;
    let m1: i32;
    let m2: i32;
    let mut n: i32;
    let mut x: [f64; 11] = [0.0; 11];
    let id: Option<Vec<u8>>;

    // --- get ID name
    if ntoks < 4 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, NODE, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- get invert elev.
    if getDouble(&tok[1], &mut x[0]) == 0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[1]);
    }

    // --- initialize parameter values
    i = 1;
    while i < 11 {
        x[i as usize] = 0.0;
        i += 1;
    }

    // --- check if no diverted link supplied
    if cio::strlen(&tok[2]) == 0 || cio::strcmp(&tok[2], b"*") == 0 {
        x[1] = -1.0;
    }
    // --- otherwise get index of diverted link
    else {
        let m1 = project_findObject(prj, LINK, &tok[2]);
        if m1 < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[2]);
        }
        x[1] = m1 as f64;
    }

    // --- get divider type
    n = 4;
    m1 = findmatch(&tok[3], DividerTypeWords);
    if m1 < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
    }
    x[2] = m1 as f64;

    // --- get index of flow diversion curve for Tabular divider
    x[3] = -1.0;
    if m1 == TABULAR_DIVIDER {
        if ntoks < 5 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        m2 = project_findObject(prj, CURVE, &tok[4]);
        if m2 < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[4]);
        }
        x[3] = m2 as f64;
        n = 5;
    }

    // --- get cutoff flow for Cutoff divider
    if m1 == CUTOFF_DIVIDER {
        if ntoks < 5 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if getDouble(&tok[4], &mut x[4]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[4]);
        }
        n = 5;
    }

    // --- get qmin, dhMax, & cWeir for Weir divider
    if m1 == WEIR_DIVIDER {
        if ntoks < 7 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = 4;
        while i < 7 {
            if getDouble(&tok[i as usize], &mut x[i as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
            }
            i += 1;
        }
        n = 7;
    }

    // --- no parameters needed for Overflow divider
    if m1 == OVERFLOW_DIVIDER {
        n = 4;
    }

    // --- retrieve optional full depth, init. depth, surcharged depth
    //      & ponded area
    m = 7;
    i = n;
    while i < ntoks && m < 11 {
        if getDouble(&tok[i as usize], &mut x[m as usize]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
        }
        m += 1;
        i += 1;
    }

    // --- add parameters to data base
    prj.Node[j as usize].ID = id.unwrap();
    node_setParams(prj, j, DIVIDER, k, &x);
    0
}

//=============================================================================

fn divider_validate(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: validates a flow divider's properties.
//
{
    let ju = j as usize;
    let i: i32;
    let k: i32;

    // --- check that diverted link is attached to divider
    k = prj.Node[ju].subIndex;
    let ku = k as usize;
    i = prj.Divider[ku].link;
    if i < 0 || (prj.Link[i as usize].node1 != j && prj.Link[i as usize].node2 != j) {
        let id = prj.Node[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_DIVIDER_LINK, &id);
    }

    // --- validate parameters supplied for weir-type divider
    if prj.Divider[ku].type_ == WEIR_DIVIDER {
        if prj.Divider[ku].dhMax <= 0.0 || prj.Divider[ku].cWeir <= 0.0 {
            let id = prj.Node[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_WEIR_DIVIDER, &id);
        } else {
            // --- find flow when weir is full
            prj.Divider[ku].qMax =
                prj.Divider[ku].cWeir * prj.Divider[ku].dhMax.powf(1.5) / UCF(prj, FLOW);
            if prj.Divider[ku].qMin > prj.Divider[ku].qMax {
                let id = prj.Node[ju].ID.clone();
                report_writeErrorMsg(prj, ERR_WEIR_DIVIDER, &id);
            }
        }
    }
}

//=============================================================================

fn divider_getOutflow(prj: &mut Project, j: i32, k: i32) -> f64
//
//  Input:   j = node index
//           k = index of diversion link
//  Output:  returns diverted flow rate (cfs)
//  Purpose: computes flow sent through divider node into its diversion link.
//
//  NOTE: requires that links be previously sorted so that the non-diversion
//        link always gets evaluated before the diversion link
{
    let ju = j as usize;
    let i: i32; // index of divider node
    let m: i32; // index of diverted flow table
    let qIn: f64; // inflow to divider
    let mut qOut: f64; // diverted outflow
    let f: f64; // fraction of weir divider full

    qIn = prj.Node[ju].inflow + prj.Node[ju].overflow;
    i = prj.Node[ju].subIndex;
    let iu = i as usize;
    match prj.Divider[iu].type_ {
        CUTOFF_DIVIDER => {
            if qIn <= prj.Divider[iu].qMin {
                qOut = 0.0;
            } else {
                qOut = qIn - prj.Divider[iu].qMin;
            }
        }

        OVERFLOW_DIVIDER => {
            // --- outflow sent into non-diversion link is simply node's inflow
            if k != prj.Divider[iu].link {
                qOut = qIn;
            }
            // --- diversion link receives any excess of node's inflow and
            //     outflow sent previously into non-diversion link
            else {
                qOut = qIn - prj.Node[ju].outflow;
            }
            if qOut < FLOW_TOL {
                qOut = 0.0;
            }
            return qOut;
        }

        WEIR_DIVIDER => {
            // --- no flow if inflow < qMin
            if qIn <= prj.Divider[iu].qMin {
                qOut = 0.0;
            }
            // --- otherwise use weir eqn.
            else {
                // --- find fractional depth of flow over weir
                f = (qIn - prj.Divider[iu].qMin) / (prj.Divider[iu].qMax - prj.Divider[iu].qMin);

                // --- if weir surcharged, use orifice eqn.
                if f > 1.0 {
                    qOut = prj.Divider[iu].qMax * f.sqrt();
                }
                // --- otherwise use weir eqn.
                else {
                    qOut = prj.Divider[iu].cWeir * (f * prj.Divider[iu].dhMax).powf(1.5) / UCF(prj, FLOW);
                }
            }
        }

        TABULAR_DIVIDER => {
            m = prj.Divider[iu].flowCurve;
            if m >= 0 {
                let x = qIn * UCF(prj, FLOW);
                qOut = table_lookup(&mut prj.Curve[m as usize], x) / UCF(prj, FLOW);
            } else {
                qOut = 0.0;
            }
        }

        _ => {
            qOut = 0.0;
        }
    }

    // --- make sure outflow doesn't exceed inflow
    if qOut > qIn {
        qOut = qIn;
    }

    // --- if link k not the diversion link, then re-define qOut as
    //     the undiverted flow
    if k != prj.Divider[iu].link {
        qOut = qIn - qOut;
    }
    qOut
}

//=============================================================================
//                    O U T F A L L   M E T H O D S
//=============================================================================

fn outfall_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = node index
//           k = outfall index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error message
//  Purpose: reads an outfall's properties from a tokenized line of input.
//
//  Format of input line is:
//    nodeID  elev  FIXED  fixedStage (flapGate) (routeTo)
//    nodeID  elev  TIDAL  curveID (flapGate) (routeTo)
//    nodeID  elev  TIMESERIES  tseriesID (flapGate) (routTo)
//    nodeID  elev  FREE (flapGate) (routeTo)
//    nodeID  elev  NORMAL (flapGate) (routeTo)
//
{
    let i: i32;
    let mut m: i32;
    let mut n: i32;
    let mut x: [f64; 7] = [0.0; 7];
    let id: Option<Vec<u8>>;

    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, NODE, &tok[0]); // node ID
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    if getDouble(&tok[1], &mut x[0]) == 0 {
        // invert elev.
        return error_setInpError(prj, ERR_NUMBER, &tok[1]);
    }
    i = findmatch(&tok[2], OutfallTypeWords); // outfall type
    if i < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[2]);
    }
    x[1] = i as f64; // outfall type
    x[2] = 0.0; // fixed stage
    x[3] = -1.0; // tidal curve
    x[4] = -1.0; // tide series
    x[5] = 0.0; // flap gate
    x[6] = -1.0; // route to subcatch

    n = 4;
    if i >= FIXED_OUTFALL {
        if ntoks < 4 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        n = 5;
        match i {
            FIXED_OUTFALL => {
                // fixed stage
                if getDouble(&tok[3], &mut x[2]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[3]);
                }
            }
            TIDAL_OUTFALL => {
                // tidal curve
                m = project_findObject(prj, CURVE, &tok[3]);
                if m < 0 {
                    return error_setInpError(prj, ERR_NAME, &tok[3]);
                }
                x[3] = m as f64;
            }
            TIMESERIES_OUTFALL => {
                // stage time series
                m = project_findObject(prj, TSERIES, &tok[3]);
                if m < 0 {
                    return error_setInpError(prj, ERR_NAME, &tok[3]);
                }
                x[4] = m as f64;
                prj.Tseries[m as usize].refersTo = TIMESERIES_OUTFALL;
            }
            _ => {}
        }
    }
    if ntoks == n {
        m = findmatch(&tok[(n - 1) as usize], NoYesWords); // flap gate
        if m < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[(n - 1) as usize]);
        }
        x[5] = m as f64;
    }

    if ntoks == n + 1 {
        m = project_findObject(prj, SUBCATCH, &tok[n as usize]);
        if m < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[n as usize]);
        }
        x[6] = m as f64;
    }

    prj.Node[j as usize].ID = id.unwrap();
    node_setParams(prj, j, OUTFALL, k, &x);
    0
}

//=============================================================================

fn outfall_setOutletDepth(prj: &mut Project, j: i32, yNorm: f64, mut yCrit: f64, z: f64)
//
//  Input:   j = node index
//           yNorm = normal flow depth in outfall conduit (ft)
//           yCrit = critical flow depth in outfall conduit (ft)
//           z = height to outfall conduit invert (ft)
//  Output:  none
//  Purpose: sets water depth at an outfall node.
//
{
    let ju = j as usize;
    let mut x: f64 = 0.0; // x,y values in table
    let mut y: f64 = 0.0;
    let yNew: f64; // new depth above invert elev. (ft)
    let stage: f64; // water elevation at outfall (ft)
    let k: i32; // table index
    let i = prj.Node[ju].subIndex as usize; // outfall index
    let currentDate: DateTime; // current date/time in days

    match prj.Outfall[i].type_ {
        FREE_OUTFALL => {
            if z > 0.0 {
                prj.Node[ju].newDepth = 0.0;
            } else {
                prj.Node[ju].newDepth = MIN(yNorm, yCrit);
            }
            return;
        }

        NORMAL_OUTFALL => {
            if z > 0.0 {
                prj.Node[ju].newDepth = 0.0;
            } else {
                prj.Node[ju].newDepth = yNorm;
            }
            return;
        }

        FIXED_OUTFALL => {
            stage = prj.Outfall[i].fixedStage;
        }

        TIDAL_OUTFALL => {
            k = prj.Outfall[i].tideCurve;
            table_getFirstEntry(&mut prj.Curve[k as usize], &mut x, &mut y);
            currentDate = prj.NewRoutingTime / MSECperDAY;
            x += (currentDate - currentDate.floor()) * 24.0;
            stage = table_lookup(&mut prj.Curve[k as usize], x) / UCF(prj, LENGTH);
        }

        TIMESERIES_OUTFALL => {
            k = prj.Outfall[i].stageSeries;
            currentDate = prj.StartDateTime + prj.NewRoutingTime / MSECperDAY;
            stage = table_tseriesLookup(&mut prj.Tseries[k as usize], currentDate, TRUE as i8) / UCF(prj, LENGTH);
        }
        _ => {
            stage = prj.Node[ju].invertElev;
        }
    }

    // --- now determine depth at node given outfall stage elev.

    // --- let critical flow depth be min. of critical & normal depth
    yCrit = MIN(yCrit, yNorm);

    // --- if elev. of critical depth is below outfall stage elev. then
    //     the outfall stage determines node depth
    if yCrit + z + prj.Node[ju].invertElev < stage {
        yNew = stage - prj.Node[ju].invertElev;
    }
    // --- otherwise if the outfall conduit lies above the outfall invert
    else if z > 0.0 {
        // --- if the outfall stage lies below the bottom of the outfall
        //     conduit then the result is distance from node invert to stage
        if stage < prj.Node[ju].invertElev + z {
            yNew = MAX(0.0, stage - prj.Node[ju].invertElev);
        }
        // --- otherwise stage lies between bottom of conduit and critical
        //     depth in conduit so result is elev. of critical depth
        else {
            yNew = z + yCrit;
        }
    }
    // --- and for case where there is no conduit offset and outfall stage
    //     lies below critical depth, then node depth = critical depth
    else {
        yNew = yCrit;
    }
    prj.Node[ju].newDepth = yNew;
}
