//! qualrout.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   qualrout.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     08/01/22   (Build 5.2.1)
//   Author:   L. Rossman
//
//   Water quality routing functions.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Pollutant mass lost to seepage flow added to mass balance totals.
//   - Pollutant concen. increased when evaporation occurs.
//   Build 5.1.009:
//   - Criterion for dry link/storage node changed to avoid concen. blowup.
//   Build 5.1.010:
//   - Entire module re-written to be more compact and easier to follow.
//   - Neglible depth limit replaced with a negligible volume limit.
//   Build 5.1.015:
//   - Fixed mass balance issue for empty storage nodes that flood.
//   Build 5.2.0:
//   - Support added for flow capture by inlet structures.
//   - Definition of a dry node/link modified.
//   Build 5.2.1:
//   - Dry non-storage nodes now have quality determined by inflow.
//   - Wet non-storage nodes with no inflow now have no change in quality.
//-----------------------------------------------------------------------------

use crate::prelude::*;

/// The file-scope variables of qualrout.c.
#[derive(Default)]
pub struct QualroutState {}

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const ZeroVolume: f64 = 0.0353147; // 1 liter in ft3
const ZeroDepth: f64 = 0.003281; // 1 mm in ft

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  qualrout_init            (called by swmm_start)
//  qualrout_execute         (called by routing_execute)

//=============================================================================

/// Called by: routing.
pub fn qualrout_init(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: initializes water quality concentrations in all nodes and links.
//
{
    let mut isWet: i32;
    let mut c: f64;

    for i in 0..prj.Nobjects[NODE as usize] as usize {
        isWet = (prj.Node[i].newDepth > ZeroDepth) as i32;
        for p in 0..prj.Nobjects[POLLUT as usize] as usize {
            c = 0.0;
            if isWet != 0 {
                c = prj.Pollut[p].initConcen;
            }
            prj.Node[i].oldQual[p] = c;
            prj.Node[i].newQual[p] = c;
        }
    }

    for i in 0..prj.Nobjects[LINK as usize] as usize {
        isWet = (prj.Link[i].newDepth > ZeroDepth) as i32;
        for p in 0..prj.Nobjects[POLLUT as usize] as usize {
            c = 0.0;
            if isWet != 0 {
                c = prj.Pollut[p].initConcen;
            }
            prj.Link[i].oldQual[p] = c;
            prj.Link[i].newQual[p] = c;
        }
    }
}

//=============================================================================

/// Called by: routing.
pub fn qualrout_execute(prj: &mut Project, tStep: f64)
//
//  Input:   tStep = routing time step (sec)
//  Output:  none
//  Purpose: routes water quality constituents through the drainage
//           network over the current time step.
//
{
    let mut qIn: f64;
    let mut vAvg: f64;

    // --- find mass flow each link contributes to its downstream node
    for i in 0..prj.Nobjects[LINK as usize] {
        findLinkMassFlow(prj, i, tStep);
    }

    // --- find new water quality concentration at each node
    for j in 0..prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        // --- get node inflow and average volume
        prj.Node[ju].qualInflow = prj.Node[ju].inflow;
        qIn = prj.Node[ju].qualInflow;
        vAvg = (prj.Node[ju].oldVolume + prj.Node[ju].newVolume) / 2.0;

        // --- save inflow concentrations if treatment applied
        //     (C: Node[j].treatment != NULL; an empty Vec is NULL)
        if !prj.Node[ju].treatment.is_empty() {
            if qIn < ZERO {
                qIn = 0.0;
            }
            let newQual = std::mem::take(&mut prj.Node[ju].newQual);
            treatmnt_setInflow(prj, qIn, &newQual);
            prj.Node[ju].newQual = newQual;
        }

        // --- find new quality at the node
        if prj.Node[ju].type_ == STORAGE || prj.Node[ju].oldVolume > ZeroVolume {
            findStorageQual(prj, j, tStep);
        } else {
            findNodeQual(prj, j);
        }

        // --- apply treatment to new quality values
        if !prj.Node[ju].treatment.is_empty() {
            treatmnt_treat(prj, j, qIn, vAvg, tStep);
        }
    }

    // --- find new water quality in each link
    for i in 0..prj.Nobjects[LINK as usize] {
        findLinkQual(prj, i, tStep);
    }
}

//=============================================================================

fn getMixedQual(c: f64, v1: f64, wIn: f64, qIn: f64, tStep: f64) -> f64
//
//  Input:   c = concentration in reactor at start of time step (mass/ft3)
//           v1 = volume in reactor at start of time step (ft3)
//           wIn = mass inflow rate (mass/sec)
//           qIn = flow inflow rate (cfs)
//           tStep = time step (sec)
//  Output:  returns pollutant concentration at end of time step (mass/ft3)
//  Purpose: finds pollutant concentration within a completely mixed reactor.
//
{
    let vIn: f64;
    let cIn: f64;
    let cMax: f64;
    let mut c = c;

    // --- if no inflow then reactor concentration is unchanged
    if qIn <= ZERO {
        return c;
    }

    // --- compute concentration of any inflow
    vIn = qIn * tStep;
    cIn = wIn * tStep / vIn;

    // --- mixture concen. can't exceed either original or inflow concen.
    cMax = MAX(c, cIn);

    // --- mix inflow with current reactor contents
    c = (c * v1 + wIn * tStep) / (v1 + vIn);
    c = MIN(c, cMax);
    c = MAX(c, 0.0);
    c
}

//=============================================================================

fn findLinkMassFlow(prj: &mut Project, i: i32, tStep: f64)
//
//  Input:   i = link index
//           tStep = time step (sec)
//  Output:  none
//  Purpose: adds constituent mass flow out of link to the total
//           accumulation at the link's downstream node.
//
//  Note:    Node[].newQual[], the accumulator variable, already contains
//           contributions from runoff and other external inflows from
//           calculations made in routing_execute().
{
    let mut j: i32;
    let mut qLink: f64;
    let mut w: f64;
    let iu = i as usize;

    // --- find inflow to downstream node
    qLink = prj.Link[iu].newFlow;

    // --- identify index of downstream node
    j = prj.Link[iu].node2;
    if qLink < 0.0 {
        j = prj.Link[iu].node1;
    }
    let ju = j as usize;

    // --- flow rate into downstream node (adjusted for inlet capture)
    qLink = qLink.abs();
    if prj.RouteModel != DW {
        qLink -= inlet_capturedFlow(prj, i);
    }

    // --- examine each pollutant
    for p in 0..prj.Nobjects[POLLUT as usize] as usize {
        // --- temporarily accumulate inflow load in Node[j].newQual
        w = qLink * prj.Link[iu].oldQual[p];
        prj.Node[ju].newQual[p] += w;

        // --- update total load transported by link
        prj.Link[iu].totalLoad[p] += w * tStep;
    }
    prj.Node[ju].qualInflow += qLink;
}

//=============================================================================

fn findNodeQual(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: finds new quality in a node with no storage volume.
//
{
    let qNode: f64;
    let ju = j as usize;
    let np = prj.Nobjects[POLLUT as usize] as usize;
    let node = &mut prj.Node[ju];

    // --- if there is flow into node then concen. = mass inflow/node flow
    qNode = node.qualInflow;
    if qNode > ZERO {
        for p in 0..np {
            node.newQual[p] /= qNode;
        }
    }
    // --- otherwise concen. remains the same
    else {
        for p in 0..np {
            if node.newDepth > ZeroDepth {
                node.newQual[p] = node.oldQual[p];
            } else {
                node.newQual[p] = 0.0;
            }
        }
    }
}

//=============================================================================

fn findLinkQual(prj: &mut Project, i: i32, tStep: f64)
//
//  Input:   i = link index
//           tStep = routing time step (sec)
//  Output:  none
//  Purpose: finds new quality in a link at end of the current time step.
//
{
    let mut j: i32; // upstream node index
    let k: usize; // conduit index
    let mut wIn: f64; // pollutant mass inflow rate (mass/sec)
    let mut qIn: f64; // inflow rate (cfs)
    let qSeep: f64; // rate of seepage loss (cfs)
    let v1: f64; // link volume at start of time step (ft3)
    let v2: f64; // link volume at end of time step (ft3)
    let mut c1: f64; // current concentration within link (mass/ft3)
    let mut c2: f64; // new concentration within link (mass/ft3)
    let vEvap: f64; // volume lost to evaporation (ft3)
    let vLosses: f64; // evap. + seepage volume loss (ft3)
    let mut fEvap: f64; // evaporation concentration factor
    let barrels: f64; // number of barrels in conduit
    let iu = i as usize;

    // --- identify index of upstream node
    j = prj.Link[iu].node1;
    if prj.Link[iu].newFlow < 0.0 {
        j = prj.Link[iu].node2;
    }
    let ju = j as usize;

    // --- link quality is that of upstream node when
    //     link is not a conduit or is a dummy link
    if prj.Link[iu].type_ != CONDUIT || prj.Link[iu].xsect.type_ == DUMMY {
        for p in 0..prj.Nobjects[POLLUT as usize] as usize {
            prj.Link[iu].newQual[p] = prj.Node[ju].newQual[p];
        }
        return;
    }

    // --- get flow rates and evaporation loss
    k = prj.Link[iu].subIndex as usize;
    barrels = prj.Conduit[k].barrels as f64;
    qIn = prj.Conduit[k].q1.abs() * barrels;
    qSeep = prj.Conduit[k].seepLossRate * barrels;
    vEvap = prj.Conduit[k].evapLossRate * barrels * tStep;

    // --- get starting and ending volumes
    v1 = prj.Link[iu].oldVolume;
    v2 = prj.Link[iu].newVolume;
    vLosses = qSeep * tStep + vEvap;

    // --- compute factor by which concentrations are increased due to
    //     evaporation loss
    fEvap = 1.0;
    if vEvap > 0.0 && v1 > ZeroVolume {
        fEvap += vEvap / v1;
    }

    // --- Steady Flow routing requires special treatment
    if prj.RouteModel == SF {
        findSFLinkQual(prj, i, qSeep, fEvap, tStep);
        return;
    }

    // --- adjust inflow to compensate for volume change under Dynamic
    //     Wave routing (which produces just a single (out)flow rate
    //     for a conduit)
    if prj.RouteModel == DW {
        qIn = qIn + (v2 + vLosses - v1) / tStep;
        qIn = MAX(qIn, 0.0);
    }

    // --- examine each pollutant
    for p in 0..prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        // --- start with concen. at start of time step
        c1 = prj.Link[iu].oldQual[pu];

        // --- update mass balance accounting for seepage loss
        massbal_addSeepageLoss(prj, p, qSeep * c1);

        // --- increase concen. by evaporation factor
        c1 *= fEvap;

        // --- reduce concen. by 1st-order reaction
        c2 = getReactedQual(prj, p, c1, v1, tStep);

        // --- mix resulting contents with inflow from upstream node
        wIn = prj.Node[ju].newQual[pu] * qIn;
        c2 = getMixedQual(c2, v1, wIn, qIn, tStep);

        // --- set concen. to zero if remaining volume is negligible
        if v2 < ZeroVolume || prj.Link[iu].newDepth <= ZeroDepth {
            massbal_addToFinalStorage(prj, p, c2 * v2);
            c2 = 0.0;
        }

        // --- assign new concen. to link
        prj.Link[iu].newQual[pu] = c2;
    }
}

//=============================================================================

fn findSFLinkQual(prj: &mut Project, i: i32, qSeep: f64, fEvap: f64, tStep: f64)
//
//  Input:   i = link index
//           tStep = routing time step (sec)
//  Output:  none
//  Purpose: finds new quality in a link at end of the current time step for
//           Steady Flow routing.
//
{
    let iu = i as usize;
    let j = prj.Link[iu].node1 as usize;
    let mut c1: f64;
    let mut c2: f64;
    let mut lossRate: f64;

    // --- examine each pollutant
    for p in 0..prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        // --- conduit's quality equals upstream node quality
        c1 = prj.Node[j].newQual[pu];

        // --- update mass balance accounting for seepage loss
        massbal_addSeepageLoss(prj, p, qSeep * c1);

        // --- increase concen. by evaporation factor
        c1 *= fEvap;

        // --- apply first-order decay over travel time
        c2 = c1;
        if prj.Pollut[pu].kDecay > 0.0 {
            c2 = c1 * (-prj.Pollut[pu].kDecay * tStep).exp();
            c2 = MAX(0.0, c2);
            lossRate = (c1 - c2) * prj.Link[iu].newFlow;
            massbal_addReactedMass(prj, p, lossRate);
        }
        prj.Link[iu].newQual[pu] = c2;
    }
}

//=============================================================================

#[allow(unused_assignments)] // C initializes qExfil, vEvap, fEvap at declaration
fn findStorageQual(prj: &mut Project, j: i32, tStep: f64)
//
//  Input:   j = node index
//           tStep = routing time step (sec)
//  Output:  none
//  Purpose: finds new quality in a node with storage volume.
//
{
    let k: usize; // storage unit index
    let qIn: f64; // inflow rate (cfs)
    let mut wIn: f64; // pollutant mass inflow rate (mass)
    let v1: f64; // volume at start of time step (ft3)
    let mut c1: f64; // initial pollutant concentration (mass/ft3)
    let mut c2: f64; // final pollutant concentration (mass/ft3)
    let mut qExfil: f64 = 0.0; // exfiltration rate from storage unit (cfs)
    let mut vEvap: f64 = 0.0; // evaporation loss from storage unit (ft3)
    let mut fEvap: f64 = 1.0; // evaporation concentration factor
    let ju = j as usize;

    // --- get inflow rate & initial volume
    qIn = prj.Node[ju].qualInflow;
    v1 = prj.Node[ju].oldVolume;

    // -- for storage nodes
    if prj.Node[ju].type_ == STORAGE {
        // --- update hydraulic residence time
        //     (HRT can be used in treatment functions)
        let ov = prj.Node[ju].oldVolume;
        updateHRT(prj, j, ov, qIn, tStep);

        // --- get exfiltration rate and evaporation loss
        k = prj.Node[ju].subIndex as usize;
        qExfil = prj.Storage[k].exfilLoss / tStep;
        vEvap = prj.Storage[k].evapLoss;

        // --- compute factor by which concentrations are increased due to
        //     evaporation loss (avoiding huge factors as storage unit
        //     dries out completely)
        if vEvap > 0.0 && v1 > ZeroVolume {
            fEvap += vEvap / v1;
        }
    }

    // --- for each pollutant
    for p in 0..prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        // --- start with concen. at start of time step
        c1 = prj.Node[ju].oldQual[pu];

        // --- update mass balance accounting for exfiltration loss
        massbal_addSeepageLoss(prj, p, qExfil * c1);

        // --- increase concen. by evaporation factor
        c1 *= fEvap;

        // --- apply first order reaction only if no separate treatment function
        if prj.Node[ju].treatment.is_empty() || prj.Node[ju].treatment[pu].equation.is_none() {
            c1 = getReactedQual(prj, p, c1, v1, tStep);
        }

        // --- mix resulting contents with inflow from all sources
        //     (temporarily accumulated in Node[j].newQual)
        wIn = prj.Node[ju].newQual[pu];
        c2 = getMixedQual(c1, v1, wIn, qIn, tStep);

        // --- set concen. to zero if remaining volume & inflow is negligible
        if (prj.Node[ju].newVolume <= ZeroVolume || prj.Node[ju].newDepth <= ZeroDepth) && qIn <= ZERO {
            let nv = prj.Node[ju].newVolume;
            massbal_addToFinalStorage(prj, p, c2 * nv);
            c2 = 0.0;
        }

        // --- assign new concen. to node
        prj.Node[ju].newQual[pu] = c2;
    }
}

//=============================================================================

fn updateHRT(prj: &mut Project, j: i32, v: f64, q: f64, tStep: f64)
//
//  Input:   j = node index
//           v = storage volume (ft3)
//           q = inflow rate (cfs)
//           tStep = time step (sec)
//  Output:  none
//  Purpose: updates hydraulic residence time (i.e., water age) at a
//           storage node.
//
{
    let k = prj.Node[j as usize].subIndex as usize;
    let mut hrt = prj.Storage[k].hrt;
    if v < ZERO {
        hrt = 0.0;
    } else {
        hrt = (hrt + tStep) * v / (v + q * tStep);
    }
    prj.Storage[k].hrt = MAX(hrt, 0.0);
}

//=============================================================================

fn getReactedQual(prj: &mut Project, p: i32, c: f64, v1: f64, tStep: f64) -> f64
//
//  Input:   p = pollutant index
//           c = initial concentration (mass/ft3)
//           v1 = initial volume (ft3)
//           tStep = time step (sec)
//  Output:  none
//  Purpose: applies a first order reaction to a pollutant over a given
//           time step.
//
{
    let mut c2: f64;
    let lossRate: f64;
    let kDecay = prj.Pollut[p as usize].kDecay;

    if kDecay == 0.0 {
        return c;
    }
    c2 = c * (1.0 - kDecay * tStep);
    c2 = MAX(0.0, c2);
    lossRate = (c - c2) * v1 / tStep;
    massbal_addReactedMass(prj, p, lossRate);
    c2
}
