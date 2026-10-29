//! massbal.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   massbal.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//             M. Tryby (EPA)
//
//   Mass balance functions
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Mass balances modified to to correctly handle negative external inflows.
//   - Volume from minimum surface area at nodes included in mass balances.
//   Build 5.1.008:
//   - massbal_updateRunoffTotals() modified.
//   - LID drain flows and returned outfall flows added to components of
//     runoff mass balance.
//   - Seepage pollutant loss added into mass balances.
//   Build 5.1.010:
//   - Remaining pollutant mass in "dry" elements now added to final storage.
//   Build 5.1.011:
//   - Final stored pollutant mass in links ignored for Steady Flow routing.
//   Build 5.1.012:
//   - Terminal storage nodes no longer treated as non-storage terminal
//     nodes are when updating total outflow volume.
//   Build 5.1.013:
//   - Volume from MinSurfArea no longer included in initial & final storage.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const MAX_RUNOFF_BALANCE_ERR: f64 = 10.0;
const MAX_FLOW_BALANCE_ERR: f64 = 10.0;

/// The file-scope variables of massbal.c.
#[derive(Default)]
pub struct MassbalState {
    //-------------------------------------------------------------------------
    //  Shared variables
    //-------------------------------------------------------------------------
    pub RunoffTotals: TRunoffTotals,        // overall surface runoff continuity totals
    pub LoadingTotals: Vec<TLoadingTotals>, // overall WQ washoff continuity totals
    pub GwaterTotals: TGwaterTotals,        // overall groundwater continuity totals
    pub FlowTotals: TRoutingTotals,         // overall routed flow continuity totals
    pub QualTotals: Vec<TRoutingTotals>,    // overall routed WQ continuity totals
    pub StepFlowTotals: TRoutingTotals,     // routed flow totals over time step
    pub OldStepFlowTotals: TRoutingTotals,
    pub StepQualTotals: Vec<TRoutingTotals>, // routed WQ totals over time step

    //-------------------------------------------------------------------------
    //  Exportable variables
    //-------------------------------------------------------------------------
    pub NodeInflow: Vec<f64>,  // total inflow volume to each node (ft3)
    pub NodeOutflow: Vec<f64>, // total outflow volume from each node (ft3)
    pub TotalArea: f64,        // total drainage area (ft2)
}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  massbal_open                (called from swmm_start in swmm5.c)
//  massbal_close               (called from swmm_end in swmm5.c)
//  massbal_report              (called from swmm_end in swmm5.c)
//  massbal_updateRunoffTotals  (called from subcatch_getRunoff)
//  massbal_updateDrainTotals   (called from evalLidUnit in lid.c)
//  massbal_updateLoadingTotals (called from subcatch_getBuildup)
//  massbal_updateGwaterTotals  (called from updateMassBal in gwater.c)
//  massbal_updateRoutingTotals (called from routing_execute)
//  massbal_initTimeStepTotals  (called from routing_execute)
//  massbal_addInflowFlow       (called from routing.c)
//  massbal_addInflowQual       (called from routing.c)
//  massbal_addOutflowFlow      (called from removeOutflows in routing.c)
//  massbal_addOutflowQual      (called from removeOutflows in routing.c)
//  massbal_addNodeLosses       (called from removeStorageLosses in routing.c)
//  massbal_addLinkLosses       (called from removeConduitLosses in routing.c)
//  massbal_addReactedMass      (called from qualrout.c & treatmnt.c)
//  massbal_addSeepageLoss      (called from routing.c)
//  massbal_addToFinalStorage   (called from qualrout.c)
//  massbal_getStepFlowError    (called from routing.c)

//=============================================================================

/// Called by: swmm5.
pub fn massbal_open(prj: &mut Project) -> i32 {
//
//  Input:   none
//  Output:  returns error code
//  Purpose: opens and initializes mass balance continuity checking.
//
    let mut j: i32;
    let n: i32;

    // --- initialize global continuity errors
    prj.RunoffError = 0.0;
    prj.GwaterError = 0.0;
    prj.FlowError = 0.0;
    prj.QualError = 0.0;

    // --- initialize runoff totals
    prj.massbal.RunoffTotals.rainfall = 0.0;
    prj.massbal.RunoffTotals.evap = 0.0;
    prj.massbal.RunoffTotals.infil = 0.0;
    prj.massbal.RunoffTotals.runoff = 0.0;
    prj.massbal.RunoffTotals.runon = 0.0;
    prj.massbal.RunoffTotals.drains = 0.0;
    prj.massbal.RunoffTotals.snowRemoved = 0.0;
    prj.massbal.RunoffTotals.initStorage = 0.0;
    prj.massbal.RunoffTotals.initSnowCover = 0.0;
    prj.massbal.TotalArea = 0.0;
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        let s = subcatch_getStorage(prj, j);
        prj.massbal.RunoffTotals.initStorage += s;
        let c = snow_getSnowCover(prj, j);
        prj.massbal.RunoffTotals.initSnowCover += c;
        prj.massbal.TotalArea += prj.Subcatch[j as usize].area;
        j += 1;
    }

    // --- initialize groundwater totals
    prj.massbal.GwaterTotals.infil = 0.0;
    prj.massbal.GwaterTotals.upperEvap = 0.0;
    prj.massbal.GwaterTotals.lowerEvap = 0.0;
    prj.massbal.GwaterTotals.lowerPerc = 0.0;
    prj.massbal.GwaterTotals.gwater = 0.0;
    prj.massbal.GwaterTotals.initStorage = 0.0;
    prj.massbal.GwaterTotals.finalStorage = 0.0;
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        let v = gwater_getVolume(prj, j) * prj.Subcatch[j as usize].area;
        prj.massbal.GwaterTotals.initStorage += v;
        j += 1;
    }

    // --- initialize node flow & storage totals
    prj.massbal.FlowTotals.dwInflow = 0.0;
    prj.massbal.FlowTotals.wwInflow = 0.0;
    prj.massbal.FlowTotals.gwInflow = 0.0;
    prj.massbal.FlowTotals.iiInflow = 0.0;
    prj.massbal.FlowTotals.exInflow = 0.0;
    prj.massbal.FlowTotals.flooding = 0.0;
    prj.massbal.FlowTotals.outflow = 0.0;
    prj.massbal.FlowTotals.evapLoss = 0.0;
    prj.massbal.FlowTotals.seepLoss = 0.0;
    prj.massbal.FlowTotals.reacted = 0.0;
    prj.massbal.FlowTotals.initStorage = 0.0;
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        prj.massbal.FlowTotals.initStorage += prj.Node[j as usize].newVolume;
        j += 1;
    }
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        prj.massbal.FlowTotals.initStorage += prj.Link[j as usize].newVolume;
        j += 1;
    }
    prj.massbal.StepFlowTotals = prj.massbal.FlowTotals;

    // --- initialize arrays to null
    prj.massbal.LoadingTotals = Vec::new();
    prj.massbal.QualTotals = Vec::new();
    prj.massbal.StepQualTotals = Vec::new();
    prj.massbal.NodeInflow = Vec::new();
    prj.massbal.NodeOutflow = Vec::new();

    // --- allocate memory for WQ washoff continuity totals
    n = prj.Nobjects[POLLUT as usize];
    if n > 0 {
        prj.massbal.LoadingTotals = vec![TLoadingTotals::default(); n as usize];
        j = 0;
        while j < n {
            let b = massbal_getBuildup(prj, j);
            let lt = &mut prj.massbal.LoadingTotals[j as usize];
            lt.initLoad = b;
            lt.buildup = 0.0;
            lt.deposition = 0.0;
            lt.sweeping = 0.0;
            lt.infil = 0.0;
            lt.bmpRemoval = 0.0;
            lt.runoff = 0.0;
            lt.finalLoad = 0.0;
            j += 1;
        }
    }

    // --- allocate memory for nodal WQ continuity totals
    if n > 0 {
        prj.massbal.QualTotals = vec![TRoutingTotals::default(); n as usize];
        prj.massbal.StepQualTotals = vec![TRoutingTotals::default(); n as usize];
    }

    // --- initialize WQ totals
    j = 0;
    while j < n {
        let m = massbal_getStoredMass(prj, j);
        let qt = &mut prj.massbal.QualTotals[j as usize];
        qt.dwInflow = 0.0;
        qt.wwInflow = 0.0;
        qt.gwInflow = 0.0;
        qt.exInflow = 0.0;
        qt.flooding = 0.0;
        qt.outflow = 0.0;
        qt.evapLoss = 0.0;
        qt.seepLoss = 0.0;
        qt.reacted = 0.0;
        qt.initStorage = m;
        j += 1;
    }

    // --- initialize totals used over a single time step
    massbal_initTimeStepTotals(prj);

    // --- allocate memory for nodal flow continuity
    if prj.Nobjects[NODE as usize] > 0 {
        prj.massbal.NodeInflow = vec![0.0; prj.Nobjects[NODE as usize] as usize];
        prj.massbal.NodeOutflow = vec![0.0; prj.Nobjects[NODE as usize] as usize];
        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            prj.massbal.NodeInflow[j as usize] = prj.Node[j as usize].newVolume;
            j += 1;
        }
    }
    prj.ErrorCode
}

//=============================================================================

/// Called by: swmm5.
pub fn massbal_close(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: frees memory used by mass balance system.
//
    prj.massbal.LoadingTotals = Vec::new();
    prj.massbal.QualTotals = Vec::new();
    prj.massbal.StepQualTotals = Vec::new();
    prj.massbal.NodeInflow = Vec::new();
    prj.massbal.NodeOutflow = Vec::new();
}

//=============================================================================

/// Called by: swmm5.
pub fn massbal_report(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: reports mass balance results.
//
    let mut j: i32;
    let mut gwArea: f64 = 0.0;

    if prj.Nobjects[SUBCATCH as usize] > 0 {
        if massbal_getRunoffError(prj) > MAX_RUNOFF_BALANCE_ERR
            || prj.RptFlags.continuity as i32 == TRUE
        {
            let totals = prj.massbal.RunoffTotals;
            let totalArea = prj.massbal.TotalArea;
            report_writeRunoffError(prj, &totals, totalArea);
        }

        if prj.Nobjects[POLLUT as usize] > 0 && prj.IgnoreQuality == 0 {
            if massbal_getLoadingError(prj) > MAX_RUNOFF_BALANCE_ERR
                || prj.RptFlags.continuity as i32 == TRUE
            {
                let totals = prj.massbal.LoadingTotals.clone();
                report_writeLoadingError(prj, &totals);
            }
        }
    }

    if prj.Nobjects[AQUIFER as usize] > 0 && prj.IgnoreGwater == 0 {
        if massbal_getGwaterError(prj) > MAX_RUNOFF_BALANCE_ERR
            || prj.RptFlags.continuity as i32 == TRUE
        {
            j = 0;
            while j < prj.Nobjects[SUBCATCH as usize] {
                if prj.Subcatch[j as usize].groundwater.is_some() {
                    gwArea += prj.Subcatch[j as usize].area;
                }
                j += 1;
            }
            if gwArea > 0.0 {
                let totals = prj.massbal.GwaterTotals;
                report_writeGwaterError(prj, &totals, gwArea);
            }
        }
    }

    if prj.Nobjects[NODE as usize] > 0 && prj.IgnoreRouting == 0 {
        if massbal_getFlowError(prj) > MAX_FLOW_BALANCE_ERR
            || prj.RptFlags.continuity as i32 == TRUE
        {
            let totals = prj.massbal.FlowTotals;
            report_writeFlowError(prj, &totals);
        }

        if prj.Nobjects[POLLUT as usize] > 0 && prj.IgnoreQuality == 0 {
            if massbal_getQualError(prj) > MAX_FLOW_BALANCE_ERR
                || prj.RptFlags.continuity as i32 == TRUE
            {
                let totals = prj.massbal.QualTotals.clone();
                report_writeQualError(prj, &totals);
            }
        }
    }
}

//=============================================================================

fn massbal_getBuildup(prj: &Project, p: i32) -> f64 {
//
//  Input:   p = pollutant index
//  Output:  returns total pollutant buildup (lbs or kg)
//  Purpose: computes current total buildup of a pollutant over study area.
//
    let mut i: i32;
    let mut j: i32;
    let mut load: f64 = 0.0;

    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        i = 0;
        while i < prj.Nobjects[LANDUSE as usize] {
            load += prj.Subcatch[j as usize].landFactor[i as usize].buildup[p as usize];
            i += 1;
        }
        load += prj.Subcatch[j as usize].pondedQual[p as usize] * prj.Pollut[p as usize].mcf;
        j += 1;
    }
    load
}

//=============================================================================

/// Called by: lid, runoff, subcatch.
pub fn massbal_updateRunoffTotals(prj: &mut Project, flowType: i32, v: f64) {
//
//  Input:   flowType = type of flow
//           v = flow volume (ft3)
//  Output:  none
//  Purpose: updates runoff totals after current time step.
//
    match flowType {
        RUNOFF_RAINFALL => prj.massbal.RunoffTotals.rainfall += v,
        RUNOFF_EVAP => prj.massbal.RunoffTotals.evap += v,
        RUNOFF_INFIL => prj.massbal.RunoffTotals.infil += v,
        RUNOFF_RUNOFF => prj.massbal.RunoffTotals.runoff += v,
        RUNOFF_DRAINS => prj.massbal.RunoffTotals.drains += v,
        RUNOFF_RUNON => prj.massbal.RunoffTotals.runon += v,
        _ => {}
    }
}

//=============================================================================

/// Called by: gwater.
pub fn massbal_updateGwaterTotals(prj: &mut Project, vInfil: f64, vUpperEvap: f64, vLowerEvap: f64, vLowerPerc: f64, vGwater: f64) {
//
//  Input:   vInfil = volume depth of infiltrated water (ft)
//           vUpperEvap = volume depth of upper evaporation (ft)
//           vLowerEvap = volume depth of lower evaporation (ft)
//           vLowerPerc = volume depth of percolation to deep GW (ft)
//           vGwater = volume depth of groundwater outflow (ft)
//  Output:  none
//  Purpose: updates groundwater totals after current time step.
//
    prj.massbal.GwaterTotals.infil += vInfil;
    prj.massbal.GwaterTotals.upperEvap += vUpperEvap;
    prj.massbal.GwaterTotals.lowerEvap += vLowerEvap;
    prj.massbal.GwaterTotals.lowerPerc += vLowerPerc;
    prj.massbal.GwaterTotals.gwater += vGwater;
}

//=============================================================================

/// Called by: routing.
pub fn massbal_initTimeStepTotals(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: initializes routing totals for current time step.
//
    let mut j: i32;
    prj.massbal.OldStepFlowTotals = prj.massbal.StepFlowTotals;
    prj.massbal.StepFlowTotals.dwInflow = 0.0;
    prj.massbal.StepFlowTotals.wwInflow = 0.0;
    prj.massbal.StepFlowTotals.gwInflow = 0.0;
    prj.massbal.StepFlowTotals.iiInflow = 0.0;
    prj.massbal.StepFlowTotals.exInflow = 0.0;
    prj.massbal.StepFlowTotals.flooding = 0.0;
    prj.massbal.StepFlowTotals.outflow = 0.0;
    prj.massbal.StepFlowTotals.evapLoss = 0.0;
    prj.massbal.StepFlowTotals.seepLoss = 0.0;
    prj.massbal.StepFlowTotals.reacted = 0.0;
    j = 0;
    while j < prj.Nobjects[POLLUT as usize] {
        let sq = &mut prj.massbal.StepQualTotals[j as usize];
        sq.dwInflow = 0.0;
        sq.wwInflow = 0.0;
        sq.gwInflow = 0.0;
        sq.iiInflow = 0.0;
        sq.exInflow = 0.0;
        sq.flooding = 0.0;
        sq.outflow = 0.0;
        sq.reacted = 0.0;
        sq.seepLoss = 0.0;
        sq.initStorage = 0.0;
        sq.finalStorage = 0.0;
        j += 1;
    }
}

//=============================================================================

/// Called by: lid, routing.
pub fn massbal_addInflowFlow(prj: &mut Project, type_: i32, q: f64) {
//
//  Input:   type = type of inflow
//           q    = inflow rate (cfs)
//  Output:  none
//  Purpose: adds flow inflow to routing totals for current time step.
//
    match type_ {
        DRY_WEATHER_INFLOW => prj.massbal.StepFlowTotals.dwInflow += q,
        WET_WEATHER_INFLOW => prj.massbal.StepFlowTotals.wwInflow += q,
        GROUNDWATER_INFLOW => prj.massbal.StepFlowTotals.gwInflow += q,
        RDII_INFLOW => prj.massbal.StepFlowTotals.iiInflow += q,
        EXTERNAL_INFLOW => prj.massbal.StepFlowTotals.exInflow += q,
        _ => {}
    }
}

//=============================================================================

/// Called by: landuse, lid, runoff, surfqual.
pub fn massbal_updateLoadingTotals(prj: &mut Project, type_: i32, p: i32, w: f64) {
//
//  Input:   type = type of inflow
//           p    = pollutant index
//           w    = mass loading
//  Output:  none
//  Purpose: adds inflow mass loading to loading totals for current time step.
//
    match type_ {
        BUILDUP_LOAD => prj.massbal.LoadingTotals[p as usize].buildup += w,
        DEPOSITION_LOAD => prj.massbal.LoadingTotals[p as usize].deposition += w,
        SWEEPING_LOAD => prj.massbal.LoadingTotals[p as usize].sweeping += w,
        INFIL_LOAD => prj.massbal.LoadingTotals[p as usize].infil += w,
        BMP_REMOVAL_LOAD => prj.massbal.LoadingTotals[p as usize].bmpRemoval += w,
        RUNOFF_LOAD => prj.massbal.LoadingTotals[p as usize].runoff += w,
        FINAL_LOAD => prj.massbal.LoadingTotals[p as usize].finalLoad += w,
        _ => {}
    }
}

//=============================================================================

/// Called by: lid, routing.
pub fn massbal_addInflowQual(prj: &mut Project, type_: i32, p: i32, w: f64) {
//
//  Input:   type = type of inflow
//           p    = pollutant index
//           w    = mass flow rate (mass/sec)
//  Output:  none
//  Purpose: adds quality inflow to routing totals for current time step.
//
    if p < 0 || p >= prj.Nobjects[POLLUT as usize] {
        return;
    }
    match type_ {
        DRY_WEATHER_INFLOW => prj.massbal.StepQualTotals[p as usize].dwInflow += w,
        WET_WEATHER_INFLOW => prj.massbal.StepQualTotals[p as usize].wwInflow += w,
        GROUNDWATER_INFLOW => prj.massbal.StepQualTotals[p as usize].gwInflow += w,
        EXTERNAL_INFLOW => prj.massbal.StepQualTotals[p as usize].exInflow += w,
        RDII_INFLOW => prj.massbal.StepQualTotals[p as usize].iiInflow += w,
        _ => {}
    }
}

//=============================================================================

/// Called by: routing.
pub fn massbal_addOutflowFlow(prj: &mut Project, q: f64, isFlooded: i32) {
//
//  Input:   q = outflow flow rate (cfs)
//           isFlooded = TRUE if outflow represents internal flooding
//  Output:  none
//  Purpose: adds flow outflow over current time step to routing totals.
//
    if isFlooded != 0 {
        prj.massbal.StepFlowTotals.flooding += q;
    } else {
        prj.massbal.StepFlowTotals.outflow += q;
    }
}

//=============================================================================

/// Called by: routing.
pub fn massbal_addOutflowQual(prj: &mut Project, p: i32, w: f64, isFlooded: i32) {
//
//  Input:   p = pollutant index
//           w = mass outflow rate (mass/sec)
//           isFlooded = TRUE if outflow represents internal flooding
//  Output:  none
//  Purpose: adds pollutant outflow over current time step to routing totals.
//
    if p < 0 || p >= prj.Nobjects[POLLUT as usize] {
        return;
    }
    if w >= 0.0 {
        if isFlooded != 0 {
            prj.massbal.StepQualTotals[p as usize].flooding += w;
        } else {
            prj.massbal.StepQualTotals[p as usize].outflow += w;
        }
    } else {
        prj.massbal.StepQualTotals[p as usize].exInflow -= w;
    }
}

//=============================================================================

/// Called by: qualrout, treatmnt.
pub fn massbal_addReactedMass(prj: &mut Project, p: i32, w: f64) {
//
//  Input:   p = pollutant index
//           w = rate of mass reacted (mass/sec)
//  Output:  none
//  Purpose: adds mass reacted during current time step to routing totals.
//
    if p < 0 || p >= prj.Nobjects[POLLUT as usize] {
        return;
    }
    prj.massbal.StepQualTotals[p as usize].reacted += w;
}

//=============================================================================

/// Called by: qualrout.
pub fn massbal_addSeepageLoss(prj: &mut Project, p: i32, w: f64) {
//
//  Input:   p = pollutant index
//           w = mass seepage rate (mass/sec)
//  Output:  none
//  Purpose: adds mass lost to seepage during current time step to routing totals.
//
    if p < 0 || p >= prj.Nobjects[POLLUT as usize] {
        return;
    }
    prj.massbal.StepQualTotals[p as usize].seepLoss += w;
}

//=============================================================================

/// Called by: qualrout.
pub fn massbal_addToFinalStorage(prj: &mut Project, p: i32, w: f64) {
//
//  Input:   p = pollutant index
//           w = pollutant mass
//  Output:  none
//  Purpose: adds mass remaining on dry surface to routing totals.
//
    if p < 0 || p >= prj.Nobjects[POLLUT as usize] {
        return;
    }
    prj.massbal.StepQualTotals[p as usize].finalStorage += w;
}

//=============================================================================

/// Called by: routing.
pub fn massbal_addNodeLosses(prj: &mut Project, evapLoss: f64, seepLoss: f64) {
//
//  Input:   evapLoss = evaporation loss from all nodes (ft3/sec)
//           seepLoss = seepage loss from all nodes (ft3/sec)
//  Output:  none
//  Purpose: adds node losses over current time step to routing totals.
//
    prj.massbal.StepFlowTotals.evapLoss += evapLoss;
    prj.massbal.StepFlowTotals.seepLoss += seepLoss;
}

//=============================================================================

/// Called by: routing.
pub fn massbal_addLinkLosses(prj: &mut Project, evapLoss: f64, seepLoss: f64) {
//
//  Input:   evapLoss = evaporation loss from all links (ft3/sec)
//           infilLoss = infiltration loss from all links (ft3/sec)
//  Output:  none
//  Purpose: adds link losses over current time step to routing totals.
//
    prj.massbal.StepFlowTotals.evapLoss += evapLoss;
    prj.massbal.StepFlowTotals.seepLoss += seepLoss;
}

//=============================================================================

/// Called by: routing.
pub fn massbal_updateRoutingTotals(prj: &mut Project, tStep: f64) {
//
//  Input:   tStep = time step (sec)
//  Output:  none
//  Purpose: updates overall routing totals with totals from current time step.
//
    let mut j: i32;
    let mb = &mut prj.massbal;
    mb.FlowTotals.dwInflow += mb.StepFlowTotals.dwInflow * tStep;
    mb.FlowTotals.wwInflow += mb.StepFlowTotals.wwInflow * tStep;
    mb.FlowTotals.gwInflow += mb.StepFlowTotals.gwInflow * tStep;
    mb.FlowTotals.iiInflow += mb.StepFlowTotals.iiInflow * tStep;
    mb.FlowTotals.exInflow += mb.StepFlowTotals.exInflow * tStep;
    mb.FlowTotals.flooding += mb.StepFlowTotals.flooding * tStep;
    mb.FlowTotals.outflow += mb.StepFlowTotals.outflow * tStep;
    mb.FlowTotals.evapLoss += mb.StepFlowTotals.evapLoss * tStep;
    mb.FlowTotals.seepLoss += mb.StepFlowTotals.seepLoss * tStep;

    j = 0;
    while j < prj.Nobjects[POLLUT as usize] {
        let sq = prj.massbal.StepQualTotals[j as usize];
        let qt = &mut prj.massbal.QualTotals[j as usize];
        qt.dwInflow += sq.dwInflow * tStep;
        qt.wwInflow += sq.wwInflow * tStep;
        qt.gwInflow += sq.gwInflow * tStep;
        qt.iiInflow += sq.iiInflow * tStep;
        qt.exInflow += sq.exInflow * tStep;
        qt.flooding += sq.flooding * tStep;
        qt.outflow += sq.outflow * tStep;
        qt.reacted += sq.reacted * tStep;
        qt.seepLoss += sq.seepLoss * tStep;
        qt.finalStorage += sq.finalStorage;
        j += 1;
    }

    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        let node = &prj.Node[j as usize];
        prj.massbal.NodeInflow[j as usize] += node.inflow * tStep;
        if node.type_ == OUTFALL || (node.degree == 0 && node.type_ != STORAGE) {
            prj.massbal.NodeOutflow[j as usize] += node.inflow * tStep;
        } else {
            prj.massbal.NodeOutflow[j as usize] += node.outflow * tStep;
            if node.newVolume <= node.fullVolume {
                prj.massbal.NodeOutflow[j as usize] += node.overflow * tStep;
            }
        }
        j += 1;
    }
}

//=============================================================================

fn massbal_getStorage(prj: &mut Project, isFinalStorage: i8) -> f64 {
//
//  Input:   isFinalStorage = TRUE if at final time period
//  Output:  returns storage volume used (ft3)
//  Purpose: computes total system storage (nodes + links) filled
//
    let mut j: i32;
    let mut totalStorage: f64 = 0.0;
    let mut nodeStorage: f64;

    // --- get volume in nodes
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        nodeStorage = prj.Node[j as usize].newVolume;
        if isFinalStorage != 0 {
            prj.massbal.NodeOutflow[j as usize] += nodeStorage;
        }
        totalStorage += nodeStorage;
        j += 1;
    }

    // --- skip final link storage for Steady Flow routing
    if isFinalStorage != 0 && prj.RouteModel == SF {
        return totalStorage;
    }

    // --- add on volume stored in links
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        totalStorage += prj.Link[j as usize].newVolume;
        j += 1;
    }
    totalStorage
}

//=============================================================================

/// Called by: (entry point).
pub fn massbal_getSysFlows(prj: &Project, f: f64, sysFlows: &mut [f64]) {
//
//  Input:   f = time weighting factor
//  Output:  sysFlows = array of total system flows
//  Purpose: retrieves time-weighted average of old and new system flows.
//
    let f1: f64 = 1.0 - f;
    let old = &prj.massbal.OldStepFlowTotals;
    let new = &prj.massbal.StepFlowTotals;
    sysFlows[SYS_DWFLOW as usize] = (f1 * old.dwInflow + f * new.dwInflow) * UCF(prj, FLOW);
    sysFlows[SYS_GWFLOW as usize] = (f1 * old.gwInflow + f * new.gwInflow) * UCF(prj, FLOW);
    sysFlows[SYS_IIFLOW as usize] = (f1 * old.iiInflow + f * new.iiInflow) * UCF(prj, FLOW);
    sysFlows[SYS_EXFLOW as usize] = (f1 * old.exInflow + f * new.exInflow) * UCF(prj, FLOW);
    sysFlows[SYS_FLOODING as usize] = (f1 * old.flooding + f * new.flooding) * UCF(prj, FLOW);
    sysFlows[SYS_OUTFLOW as usize] = (f1 * old.outflow + f * new.outflow) * UCF(prj, FLOW);
    sysFlows[SYS_STORAGE as usize] = (f1 * old.finalStorage + f * new.finalStorage) * UCF(prj, VOLUME);
}

//=============================================================================

/// massbal_getRunoffError is declared in funcs.h (no caller outside massbal.c).
pub fn massbal_getRunoffError(prj: &mut Project) -> f64 {
//
//  Input:   none
//  Output:  none
//  Purpose: computes runoff mass balance error.
//
    let mut j: i32;
    let totalInflow: f64;
    let totalOutflow: f64;

    // --- find final storage on all subcatchments
    prj.massbal.RunoffTotals.finalStorage = 0.0;
    prj.massbal.RunoffTotals.finalSnowCover = 0.0;
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        let s = subcatch_getStorage(prj, j);
        prj.massbal.RunoffTotals.finalStorage += s;
        let c = snow_getSnowCover(prj, j);
        prj.massbal.RunoffTotals.finalSnowCover += c;
        j += 1;
    }

    // --- get snow removed from system
    prj.massbal.RunoffTotals.snowRemoved = prj.Snow.removed;

    // --- compute % difference between total inflow and outflow
    let rt = &mut prj.massbal.RunoffTotals;
    totalInflow = rt.rainfall + rt.runon + rt.initStorage + rt.initSnowCover;
    totalOutflow = rt.evap
        + rt.infil
        + rt.runoff
        + rt.drains
        + rt.snowRemoved
        + rt.finalStorage
        + rt.finalSnowCover;
    rt.pctError = 0.0;
    if (totalInflow - totalOutflow).abs() < 1.0 {
        rt.pctError = TINY;
    } else if totalInflow > 0.0 {
        rt.pctError = 100.0 * (1.0 - totalOutflow / totalInflow);
    } else if totalOutflow > 0.0 {
        rt.pctError = 100.0 * (totalInflow / totalOutflow - 1.0);
    }
    prj.RunoffError = prj.massbal.RunoffTotals.pctError;
    prj.massbal.RunoffTotals.pctError
}

//=============================================================================

fn massbal_getLoadingError(prj: &mut Project) -> f64 {
//
//  Input:   none
//  Output:  none
//  Purpose: computes runoff load mass balance error.
//
    let mut j: i32;
    let mut loadIn: f64;
    let mut loadOut: f64;
    let mut maxError: f64 = 0.0;

    j = 0;
    while j < prj.Nobjects[POLLUT as usize] {
        // --- get final pollutant loading remaining on land surface
        let b = massbal_getBuildup(prj, j);
        prj.massbal.LoadingTotals[j as usize].finalLoad += b;

        let isCount = prj.Pollut[j as usize].units == COUNT;
        let lt = &mut prj.massbal.LoadingTotals[j as usize];

        // --- compute total load added to study area
        loadIn = lt.initLoad + lt.buildup + lt.deposition;

        // --- compute total load removed from study area
        loadOut = lt.sweeping + lt.infil + lt.bmpRemoval + lt.runoff + lt.finalLoad;

        // --- compute mass balance error
        lt.pctError = 0.0;
        if (loadIn - loadOut).abs() < 0.001 {
            lt.pctError = TINY;
        } else if loadIn > 0.0 {
            lt.pctError = 100.0 * (1.0 - loadOut / loadIn);
        } else if loadOut > 0.0 {
            lt.pctError = 100.0 * (loadIn / loadOut - 1.0);
        }
        maxError = MAX(maxError, lt.pctError);

        // --- report total counts as log10
        if isCount {
            lt.initLoad = LOG10(lt.initLoad);
            lt.buildup = LOG10(lt.buildup);
            lt.deposition = LOG10(lt.deposition);
            lt.sweeping = LOG10(lt.sweeping);
            lt.infil = LOG10(lt.infil);
            lt.bmpRemoval = LOG10(lt.bmpRemoval);
            lt.runoff = LOG10(lt.runoff);
            lt.finalLoad = LOG10(lt.finalLoad);
        }
        j += 1;
    }
    maxError
}

//=============================================================================

fn massbal_getGwaterError(prj: &mut Project) -> f64 {
//
//  Input:   none
//  Output:  none
//  Purpose: computes groundwater mass balance error.
//
    let mut j: i32;
    let totalInflow: f64;
    let totalOutflow: f64;

    // --- find final storage in groundwater
    prj.massbal.GwaterTotals.finalStorage = 0.0;
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        let v = gwater_getVolume(prj, j) * prj.Subcatch[j as usize].area;
        prj.massbal.GwaterTotals.finalStorage += v;
        j += 1;
    }

    // --- compute % difference between total inflow and outflow
    let gt = &mut prj.massbal.GwaterTotals;
    totalInflow = gt.infil + gt.initStorage;
    totalOutflow = gt.upperEvap + gt.lowerEvap + gt.lowerPerc + gt.gwater + gt.finalStorage;
    gt.pctError = 0.0;
    if (totalInflow - totalOutflow).abs() < 1.0 {
        gt.pctError = TINY;
    } else if totalInflow > 0.0 {
        gt.pctError = 100.0 * (1.0 - totalOutflow / totalInflow);
    } else if totalOutflow > 0.0 {
        gt.pctError = 100.0 * (totalInflow / totalOutflow - 1.0);
    }
    prj.GwaterError = prj.massbal.GwaterTotals.pctError;
    prj.massbal.GwaterTotals.pctError
}

//=============================================================================

/// massbal_getFlowError is declared in funcs.h (no caller outside massbal.c).
pub fn massbal_getFlowError(prj: &mut Project) -> f64 {
//
//  Input:   none
//  Output:  none
//  Purpose: computes flow routing mass balance error.
//
    let mut totalInflow: f64;
    let mut totalOutflow: f64;

    // --- get final volume of nodes and links
    let fs = massbal_getStorage(prj, TRUE as i8);
    prj.massbal.FlowTotals.finalStorage = fs;

    let ft = &mut prj.massbal.FlowTotals;

    // --- add contributions to total inflow and outflow that are always positive
    totalInflow = ft.initStorage + ft.wwInflow + ft.iiInflow;
    totalOutflow = ft.finalStorage + ft.flooding + ft.evapLoss + ft.seepLoss + ft.reacted;

    // --- add on contributions that might be either positive or negative
    if ft.dwInflow >= 0.0 {
        totalInflow += ft.dwInflow;
    } else {
        totalOutflow -= ft.dwInflow;
    }
    if ft.gwInflow >= 0.0 {
        totalInflow += ft.gwInflow;
    } else {
        totalOutflow -= ft.gwInflow;
    }
    if ft.exInflow >= 0.0 {
        totalInflow += ft.exInflow;
    } else {
        totalOutflow -= ft.exInflow;
    }
    if ft.outflow >= 0.0 {
        totalOutflow += ft.outflow;
    } else {
        totalInflow -= ft.outflow;
    }

    // --- find percent difference between total inflow and outflow
    ft.pctError = 0.0;
    if (totalInflow - totalOutflow).abs() < 1.0 {
        ft.pctError = TINY;
    } else if totalInflow.abs() > 0.0 {
        ft.pctError = 100.0 * (1.0 - totalOutflow / totalInflow);
    } else if totalOutflow.abs() > 0.0 {
        ft.pctError = 100.0 * (totalInflow / totalOutflow - 1.0);
    }
    prj.FlowError = prj.massbal.FlowTotals.pctError;
    prj.massbal.FlowTotals.pctError
}

//=============================================================================

fn massbal_getQualError(prj: &mut Project) -> f64 {
//
//  Input:   none
//  Output:  none
//  Purpose: computes water quality routing mass balance error.
//
    let mut p: i32;
    let mut maxQualError: f64 = 0.0;
    let mut totalInflow: f64;
    let mut totalOutflow: f64;
    let mut cf: f64;

    // --- analyze each pollutant
    p = 0;
    while p < prj.Nobjects[POLLUT as usize] {
        // --- get final mass stored in nodes and links
        let m = massbal_getStoredMass(prj, p);
        prj.massbal.QualTotals[p as usize].finalStorage += m;

        let units = prj.Pollut[p as usize].units;
        let ucfMass = UCF(prj, MASS);
        let qt = &mut prj.massbal.QualTotals[p as usize];

        // --- compute % difference between total inflow and outflow
        totalInflow = qt.dwInflow + qt.wwInflow + qt.gwInflow + qt.iiInflow + qt.exInflow + qt.initStorage;
        totalOutflow = qt.flooding + qt.outflow + qt.reacted + qt.seepLoss + qt.finalStorage;
        qt.pctError = 0.0;
        if (totalInflow - totalOutflow).abs() < 0.001 {
            qt.pctError = TINY;
        } else if totalInflow > 0.0 {
            qt.pctError = 100.0 * (1.0 - totalOutflow / totalInflow);
        } else if totalOutflow > 0.0 {
            qt.pctError = 100.0 * (totalInflow / totalOutflow - 1.0);
        }

        // --- update max. error among all pollutants
        if qt.pctError.abs() > maxQualError.abs() {
            maxQualError = qt.pctError;
        }

        // --- convert totals to reporting units (lbs, kg, or Log(Count))
        cf = LperFT3;
        if units == COUNT {
            qt.dwInflow = LOG10(cf * qt.dwInflow);
            qt.wwInflow = LOG10(cf * qt.wwInflow);
            qt.gwInflow = LOG10(cf * qt.gwInflow);
            qt.iiInflow = LOG10(cf * qt.iiInflow);
            qt.exInflow = LOG10(cf * qt.exInflow);
            qt.flooding = LOG10(cf * qt.flooding);
            qt.outflow = LOG10(cf * qt.outflow);
            qt.reacted = LOG10(cf * qt.reacted);
            qt.seepLoss = LOG10(cf * qt.seepLoss);
            qt.initStorage = LOG10(cf * qt.initStorage);
            qt.finalStorage = LOG10(cf * qt.finalStorage);
        } else {
            cf = cf * ucfMass;
            if units == UG {
                cf /= 1000.0;
            }
            qt.dwInflow *= cf;
            qt.wwInflow *= cf;
            qt.gwInflow *= cf;
            qt.iiInflow *= cf;
            qt.exInflow *= cf;
            qt.flooding *= cf;
            qt.outflow *= cf;
            qt.reacted *= cf;
            qt.seepLoss *= cf;
            qt.initStorage *= cf;
            qt.finalStorage *= cf;
        }
        p += 1;
    }
    prj.QualError = maxQualError;
    maxQualError
}

//=============================================================================

/// Called by: routing.
pub fn massbal_getStepFlowError(prj: &Project) -> f64 {
//
//  Input:   none
//  Output:  returns fractional difference between total inflow and outflow.
//  Purpose: computes flow routing mass balance error at current time step.
//
    let totalInflow: f64;
    let totalOutflow: f64;
    let s = &prj.massbal.StepFlowTotals;

    // --- compute % difference between total inflow and outflow
    totalInflow = s.dwInflow + s.wwInflow + s.gwInflow + s.iiInflow + s.exInflow;
    totalOutflow = s.flooding + s.outflow + s.evapLoss + s.seepLoss + s.reacted;
    if totalInflow.abs() > 0.0 {
        1.0 - totalOutflow / totalInflow
    } else if totalOutflow.abs() > 0.0 {
        totalInflow / totalOutflow - 1.0
    } else {
        0.0
    }
}

//=============================================================================

fn massbal_getStoredMass(prj: &Project, p: i32) -> f64 {
//
//  Input:   p = pollutant index
//  Output:  returns mass of pollutant.
//  Purpose: computes mass of pollutant stored in conveyance network.
//
    let mut j: i32;
    let mut storedMass: f64 = 0.0;

    // --- get mass stored in nodes
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        storedMass += prj.Node[j as usize].newVolume * prj.Node[j as usize].newQual[p as usize];
        j += 1;
    }

    // --- get mass stored in links (except for Steady Flow routing)
    if prj.RouteModel != SF {
        j = 0;
        while j < prj.Nobjects[LINK as usize] {
            storedMass += prj.Link[j as usize].newVolume * prj.Link[j as usize].newQual[p as usize];
            j += 1;
        }
    }
    storedMass
}
