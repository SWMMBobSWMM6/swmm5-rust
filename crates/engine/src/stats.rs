//! stats.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   stats.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21   (Build 5.2.0)
//   Author:   L. Rossman
//             R. Dickinson (CDM)
//
//   Simulation statistics functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Exfiltration losses added to storage node statistics.
//   Build 5.1.008:
//   - Support for updating groundwater statistics added.
//   - Support for updating maximum reported nodal depths added.
//   - OpenMP parallelization applied to updating node and link flow statistics.
//   - Updating of time that conduit is upstrm/dnstrm full was modified.
//   Build 5.1.011:
//   - Surcharging is now evaluated only under dynamic wave flow routing and
//     storage nodes cannot be classified as surcharged.
//   Build 5.1.012:
//   - Time step statistics now evaluated only in non-steady state periods.
//   - Check for full conduit flow now accounts for number of barrels.
//   Build 5.1.013:
//   - Include omp.h protected against lack of compiler support for OpenMP.
//   - Statistics on impervious and pervious runoff totals added.
//   - Storage nodes with a non-zero surcharge depth (e.g. enclosed tanks)
//     can now be classified as being surcharged.
//   Build 5.1.015:
//   - Fixes bug in summary statistics when Report Start date > Start Date.
//   - Fixes failure to initialize all subcatchment groundwater statistics.
//   - Support added for grouped freqency table of routing time steps.
//   Build 5.2.0:
//   - Support added for reporting most frequent non-converging nodes.
//   - Support added for RptFlags.disabled option.
//   - Fixed display of routing statistics report for RptFlags.flowStats = FALSE.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
const MAX_STATS: i32 = 5;

/// The file-scope variables of stats.c.
#[derive(Default)]
pub struct StatsState {
    //-------------------------------------------------------------------------
    //  Exportable variables (shared with statsrpt.c)
    //-------------------------------------------------------------------------
    pub SubcatchStats: Vec<TSubcatchStats>,
    pub NodeStats: Vec<TNodeStats>,
    pub LinkStats: Vec<TLinkStats>,
    pub StorageStats: Vec<TStorageStats>,
    pub OutfallStats: Vec<TOutfallStats>,
    pub PumpStats: Vec<TPumpStats>,
    pub MaxOutfallFlow: f64,
    pub RoutingTimeSpan: f64,
    pub MaxRunoffFlow: f64,

    //-------------------------------------------------------------------------
    //  static variables
    //-------------------------------------------------------------------------
    TimeStepStats: TTimeStepStats,
    MaxMassBalErrs: [TMaxStats; MAX_STATS as usize],
    MaxCourantCrit: [TMaxStats; MAX_STATS as usize],
    MaxFlowTurns: [TMaxStats; MAX_STATS as usize],
    MaxNonConverged: [TMaxStats; MAX_STATS as usize],
    SysOutfallFlow: f64,
}

//-----------------------------------------------------------------------------
//  Imported variables
//-----------------------------------------------------------------------------
//  NodeInflow, NodeOutflow: prj.massbal (defined in massbal.c)

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  stats_open                    (called from swmm_start in swmm5.c)
//  stats_close                   (called from swmm_end in swmm5.c)
//  stats_report                  (called from swmm_end in swmm5.c)
//  stats_updateSubcatchStats     (called from subcatch_getRunoff)
//  stats_updateGwaterStats       (called from gwater_getGroundwater)
//  stats_updateFlowStats         (called from routing_execute)
//  stats_updateTimeStepStats     (called from routing_execute)
//  stats_updateCriticalTimeCount (called from getVariableStep in dynwave.c)
//  stats_updateMaxNodeDepth      (called from output_saveNodeResults)
//  stats_updateConvergenceStats  (called from updateConvergenceStats in dynwave.c)

//=============================================================================

/// Called by: swmm5.
pub fn stats_open(prj: &mut Project) -> i32 {
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: opens the simulation statistics system.
//
    let mut j: i32;
    let mut k: i32;
    let timeStepDelta: f64;
    let logMaxTimeStep: f64;
    let logMinTimeStep: f64;

    // --- set all pointers to NULL
    prj.stats.NodeStats = Vec::new();
    prj.stats.LinkStats = Vec::new();
    prj.stats.StorageStats = Vec::new();
    prj.stats.OutfallStats = Vec::new();
    prj.stats.PumpStats = Vec::new();

    // --- allocate memory for & initialize subcatchment statistics
    prj.stats.SubcatchStats = Vec::new();
    if prj.Nobjects[SUBCATCH as usize] > 0 {
        prj.stats.SubcatchStats = vec![TSubcatchStats::default(); prj.Nobjects[SUBCATCH as usize] as usize];
        j = 0;
        while j < prj.Nobjects[SUBCATCH as usize] {
            let s = &mut prj.stats.SubcatchStats[j as usize];
            s.precip = 0.0;
            s.runon = 0.0;
            s.evap = 0.0;
            s.infil = 0.0;
            s.runoff = 0.0;
            s.maxFlow = 0.0;
            s.impervRunoff = 0.0;
            s.pervRunoff = 0.0;
            j += 1;
        }

        j = 0;
        while j < prj.Nobjects[SUBCATCH as usize] {
            if let Some(gw) = prj.Subcatch[j as usize].groundwater.as_mut() {
                gw.stats.avgUpperMoist = 0.0;
                gw.stats.avgWaterTable = 0.0;
                gw.stats.infil = 0.0;
                gw.stats.latFlow = 0.0;
                gw.stats.deepFlow = 0.0;
                gw.stats.evap = 0.0;
                gw.stats.maxFlow = 0.0;
                gw.stats.finalUpperMoist = 0.0;
                gw.stats.finalWaterTable = 0.0;
            }
            j += 1;
        }
    }

    // --- allocate memory for node & link stats
    // (C tests the pointers for NULL below; an empty Vec stands for NULL,
    //  which is only distinguishable when Nobjects[NODE] == 0, where the
    //  loops below do nothing anyway)
    if prj.Nobjects[LINK as usize] > 0 {
        prj.stats.NodeStats = vec![TNodeStats::default(); prj.Nobjects[NODE as usize] as usize];
        prj.stats.LinkStats = vec![TLinkStats::default(); prj.Nobjects[LINK as usize] as usize];
    }

    // --- initialize node stats
    if !prj.stats.NodeStats.is_empty() {
        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            let start = prj.StartDateTime;
            let ns = &mut prj.stats.NodeStats[j as usize];
            ns.avgDepth = 0.0;
            ns.maxDepth = 0.0;
            ns.maxDepthDate = start;
            ns.maxRptDepth = 0.0;
            ns.volFlooded = 0.0;
            ns.timeFlooded = 0.0;
            ns.timeSurcharged = 0.0;
            ns.timeCourantCritical = 0.0;
            ns.totLatFlow = 0.0;
            ns.maxLatFlow = 0.0;
            ns.maxInflow = 0.0;
            ns.maxOverflow = 0.0;
            ns.maxPondedVol = 0.0;
            ns.nonConvergedCount = 0;
            ns.maxInflowDate = start;
            ns.maxOverflowDate = start;
            j += 1;
        }
    }

    // --- initialize link stats
    if !prj.stats.LinkStats.is_empty() {
        j = 0;
        while j < prj.Nobjects[LINK as usize] {
            let ls = &mut prj.stats.LinkStats[j as usize];
            ls.maxFlow = 0.0;
            ls.maxVeloc = 0.0;
            ls.maxDepth = 0.0;
            ls.maxStreetFilled = 0.0;
            ls.timeSurcharged = 0.0;
            ls.timeFullUpstream = 0.0;
            ls.timeFullDnstream = 0.0;
            ls.timeFullFlow = 0.0;
            ls.timeCapacityLimited = 0.0;
            ls.timeCourantCritical = 0.0;
            k = 0;
            while k < MAX_FLOW_CLASSES {
                ls.timeInFlowClass[k as usize] = 0.0;
                k += 1;
            }
            ls.flowTurns = 0;
            ls.flowTurnSign = 0;
            j += 1;
        }
    }

    // --- allocate memory for & initialize storage unit statistics
    if prj.Nnodes[STORAGE as usize] > 0 {
        prj.stats.StorageStats = vec![TStorageStats::default(); prj.Nnodes[STORAGE as usize] as usize];
        k = 0;
        while k < prj.Nobjects[NODE as usize] {
            if prj.Node[k as usize].type_ != STORAGE {
                k += 1;
                continue;
            }
            j = prj.Node[k as usize].subIndex;
            let newVolume = prj.Node[k as usize].newVolume;
            let start = prj.StartDateTime;
            let ss = &mut prj.stats.StorageStats[j as usize];
            ss.initVol = newVolume;
            ss.avgVol = 0.0;
            ss.maxVol = 0.0;
            ss.maxFlow = 0.0;
            ss.evapLosses = 0.0;
            ss.exfilLosses = 0.0;
            ss.maxVolDate = start;
            k += 1;
        }
    }

    // --- allocate memory for & initialize outfall statistics
    if prj.Nnodes[OUTFALL as usize] > 0 {
        prj.stats.OutfallStats = vec![TOutfallStats::default(); prj.Nnodes[OUTFALL as usize] as usize];
        j = 0;
        while j < prj.Nnodes[OUTFALL as usize] {
            let npollut = prj.Nobjects[POLLUT as usize];
            let os = &mut prj.stats.OutfallStats[j as usize];
            os.avgFlow = 0.0;
            os.maxFlow = 0.0;
            os.totalPeriods = 0;
            if npollut > 0 {
                os.totalLoad = vec![0.0; npollut as usize];
                k = 0;
                while k < npollut {
                    os.totalLoad[k as usize] = 0.0;
                    k += 1;
                }
            } else {
                os.totalLoad = Vec::new();
            }
            j += 1;
        }
    }

    // --- allocate memory & initialize pumping statistics
    if prj.Nlinks[PUMP as usize] > 0 {
        prj.stats.PumpStats = vec![TPumpStats::default(); prj.Nlinks[PUMP as usize] as usize];
        j = 0;
        while j < prj.Nlinks[PUMP as usize] {
            let ps = &mut prj.stats.PumpStats[j as usize];
            ps.utilized = 0.0;
            ps.minFlow = 0.0;
            ps.avgFlow = 0.0;
            ps.maxFlow = 0.0;
            ps.volume = 0.0;
            ps.energy = 0.0;
            ps.startUps = 0;
            ps.offCurveLow = 0.0;
            ps.offCurveHigh = 0.0;
            j += 1;
        }
    }

    // --- initialize system stats
    prj.stats.MaxRunoffFlow = 0.0;
    prj.stats.MaxOutfallFlow = 0.0;
    prj.stats.TimeStepStats.maxTimeStep = 0.0;
    prj.stats.TimeStepStats.minTimeStep = prj.RouteStep;
    prj.stats.TimeStepStats.routingTime = 0.0;
    prj.stats.TimeStepStats.trialsCount = 0.0;
    prj.stats.TimeStepStats.steadyStateTime = 0.0;
    prj.stats.TimeStepStats.timeStepCount = 0;

    // --- divide range between min and max routing time steps into
    //     equal intervals using a logarithmic scale
    logMaxTimeStep = prj.RouteStep.log10();
    logMinTimeStep = prj.MinRouteStep.log10();
    timeStepDelta = (logMaxTimeStep - logMinTimeStep) / (TIMELEVELS - 1) as f64;
    prj.stats.TimeStepStats.timeStepIntervals[0] = prj.RouteStep;
    j = 1;
    while j < TIMELEVELS {
        prj.stats.TimeStepStats.timeStepIntervals[j as usize] =
            10.0f64.powf(logMaxTimeStep - j as f64 * timeStepDelta);
        prj.stats.TimeStepStats.timeStepCounts[j as usize] = 0;
        j += 1;
    }
    prj.stats.TimeStepStats.timeStepIntervals[(TIMELEVELS - 1) as usize] = prj.MinRouteStep;
    prj.stats.RoutingTimeSpan = 0.0;
    0
}

//=============================================================================

/// Called by: swmm5.
pub fn stats_close(prj: &mut Project) {
//
//  Input:   none
//  Output:
//  Purpose: closes the simulation statistics system.
//
    prj.stats.SubcatchStats = Vec::new();
    prj.stats.NodeStats = Vec::new();
    prj.stats.LinkStats = Vec::new();
    prj.stats.StorageStats = Vec::new();
    // (each OutfallStats[j].totalLoad is freed with OutfallStats)
    prj.stats.OutfallStats = Vec::new();
    prj.stats.PumpStats = Vec::new();
}

//=============================================================================

/// Called by: swmm5.
pub fn stats_report(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: reports simulation statistics.
//
    // --- report flow routing accuracy statistics
    if prj.Nobjects[LINK as usize] > 0 && prj.RouteModel != NO_ROUTING {
        stats_findMaxStats(prj);
        if prj.RptFlags.disabled == 0 && prj.RptFlags.flowStats != 0 {
            let maxMassBalErrs = prj.stats.MaxMassBalErrs;
            let maxCourantCrit = prj.stats.MaxCourantCrit;
            let maxFlowTurns = prj.stats.MaxFlowTurns;
            let maxNonConverged = prj.stats.MaxNonConverged;
            let timeStepStats = prj.stats.TimeStepStats;
            report_writeMaxStats(prj, &maxMassBalErrs, &maxCourantCrit, MAX_STATS);
            report_writeMaxFlowTurns(prj, &maxFlowTurns, MAX_STATS);
            report_writeNonconvergedStats(prj, &maxNonConverged, MAX_STATS);
            report_writeTimeStepStats(prj, &timeStepStats);
        }
    }

    // --- report summary statistics
    if prj.RptFlags.disabled == 0 {
        statsrpt_writeReport(prj);
    }
}

//=============================================================================

/// Called by: subcatch.
pub fn stats_updateSubcatchStats(prj: &mut Project, j: i32, rainVol: f64, runonVol: f64, evapVol: f64, infilVol: f64, impervVol: f64, pervVol: f64, runoffVol: f64, runoff: f64) {
//
//  Input:   j = subcatchment index
//           rainVol   = rainfall + snowfall volume (ft3)
//           runonVol  = runon volume from other subcatchments (ft3)
//           evapVol   = evaporation volume (ft3)
//           infilVol  = infiltration volume (ft3)
//           impervVol = impervious runoff volume (ft3)
//           pervVol   = pervious runoff volume (ft3)
//           runoffVol = runoff volume (ft3)
//           runoff    = runoff rate (cfs)
//  Output:  none
//  Purpose: updates totals of runoff components for a specific subcatchment.
//
    let s = &mut prj.stats.SubcatchStats[j as usize];
    s.precip += rainVol;
    s.runon += runonVol;
    s.evap += evapVol;
    s.infil += infilVol;
    s.runoff += runoffVol;
    s.maxFlow = MAX(s.maxFlow, runoff);
    s.impervRunoff += impervVol;
    s.pervRunoff += pervVol;
}

//=============================================================================

/// Called by: gwater.
pub fn stats_updateGwaterStats(prj: &mut Project, j: i32, infil: f64, evap: f64, latFlow: f64, deepFlow: f64, theta: f64, waterTable: f64, tStep: f64) {
    // (C dereferences Subcatch[j].groundwater without a NULL check)
    let gw = prj.Subcatch[j as usize]
        .groundwater
        .as_mut()
        .expect("stats_updateGwaterStats: subcatchment has no groundwater");
    gw.stats.infil += infil * tStep;
    gw.stats.evap += evap * tStep;
    gw.stats.latFlow += latFlow * tStep;
    gw.stats.deepFlow += deepFlow * tStep;
    gw.stats.avgUpperMoist += theta * tStep;
    gw.stats.avgWaterTable += waterTable * tStep;
    gw.stats.finalUpperMoist = theta;
    gw.stats.finalWaterTable = waterTable;
    if latFlow.abs() > gw.stats.maxFlow.abs() {
        gw.stats.maxFlow = latFlow;
    }
}

//=============================================================================

/// Called by: runoff.
pub fn stats_updateMaxRunoff(prj: &mut Project) {
//
//   Input:   none
//   Output:  updates global variable MaxRunoffFlow
//   Purpose: updates value of maximum system runoff rate.
//
    let mut j: i32;
    let mut sysRunoff: f64 = 0.0;

    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        sysRunoff += prj.Subcatch[j as usize].newRunoff;
        j += 1;
    }
    prj.stats.MaxRunoffFlow = MAX(prj.stats.MaxRunoffFlow, sysRunoff);
}

//=============================================================================

/// Called by: output.
pub fn stats_updateMaxNodeDepth(prj: &mut Project, j: i32, depth: f64) {
//
//   Input:   j = node index
//            depth = water depth at node at current reporting time (ft)
//   Output:  none
//   Purpose: updates a node's maximum depth recorded at reporting times.
//
    if !prj.stats.NodeStats.is_empty() {
        let ns = &mut prj.stats.NodeStats[j as usize];
        ns.maxRptDepth = MAX(ns.maxRptDepth, depth);
    }
}

//=============================================================================

/// Called by: routing.
pub fn stats_updateFlowStats(prj: &mut Project, tStep: f64, aDate: DateTime) {
//
//  Input:   tStep = routing time step (sec)
//           aDate = current date/time
//  Output:  none
//  Purpose: updates various flow routing statistics at current time period.
//
    let mut j: i32;

    // --- update stats only after reporting period begins
    if aDate < prj.ReportStart {
        return;
    }
    prj.stats.SysOutfallFlow = 0.0;

    // --- update node & link stats
    //#pragma omp parallel num_threads(NumThreads)
    {
        //    #pragma omp for
        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            stats_updateNodeStats(prj, j, tStep, aDate);
            j += 1;
        }
        //    #pragma omp for
        j = 0;
        while j < prj.Nobjects[LINK as usize] {
            stats_updateLinkStats(prj, j, tStep, aDate);
            j += 1;
        }
    }

    // --- update count of time steps taken after reporting begins
    prj.ReportStepCount += 1;
    prj.stats.RoutingTimeSpan += tStep;

    // --- update max. system outfall flow
    prj.stats.MaxOutfallFlow = MAX(prj.stats.MaxOutfallFlow, prj.stats.SysOutfallFlow);
}

//=============================================================================

/// Called by: routing.
pub fn stats_updateTimeStepStats(prj: &mut Project, tStep: f64, trialsCount: i32, steadyState: i32) {
//
//  Input:   tStep = current flow routing time step (sec)
//           trialsCount = number of trials used to solve routing
//           steadyState = TRUE if steady flow conditions exist
//  Output:  none
//  Purpose: updates flow routing time step statistics.
//
    let mut j: i32;
    let oldRoutingTime = prj.OldRoutingTime;
    let ts = &mut prj.stats.TimeStepStats;

    // --- update time step stats if not in steady state
    ts.steadyStateTime += steadyState as f64 * tStep;
    if steadyState == FALSE {
        // --- skip initial time step for min. value)
        if oldRoutingTime > 0.0 {
            ts.minTimeStep = MIN(ts.minTimeStep, tStep);

            // --- locate interval that logged time step falls in
            //     and update its count
            j = 1;
            while j < TIMELEVELS {
                if tStep >= ts.timeStepIntervals[j as usize] {
                    ts.timeStepCounts[j as usize] += 1;
                    break;
                }
                j += 1;
            }
        }
        ts.maxTimeStep = MAX(ts.maxTimeStep, tStep);
        ts.routingTime += tStep;
        ts.timeStepCount += 1;
        ts.trialsCount += trialsCount as f64;
    }
}

//=============================================================================

/// Called by: dynwave.
pub fn stats_updateCriticalTimeCount(prj: &mut Project, node: i32, link: i32) {
//
//  Input:   node = node index
//           link = link index
//  Output:  none
//  Purpose: updates count of times a node or link was time step-critical.
//
    if node >= 0 {
        prj.stats.NodeStats[node as usize].timeCourantCritical += 1.0;
    } else if link >= 0 {
        prj.stats.LinkStats[link as usize].timeCourantCritical += 1.0;
    }
}

//=============================================================================

/// Called by: dynwave.
pub fn stats_updateConvergenceStats(prj: &mut Project, node: i32, converged: i32) {
    if converged == FALSE {
        prj.stats.NodeStats[node as usize].nonConvergedCount += 1;
    }
}

//=============================================================================

fn stats_updateNodeStats(prj: &mut Project, j: i32, tStep: f64, aDate: DateTime) {
//
//  Input:   j = node index
//           tStep = routing time step (sec)
//           aDate = current date/time
//  Output:  none
//  Purpose: updates flow statistics for a node.
//
    let mut k: i32;
    let mut p: i32;
    let mut newVolume: f64 = prj.Node[j as usize].newVolume;
    let newDepth: f64 = prj.Node[j as usize].newDepth;
    let canPond: i32 = (prj.AllowPonding != 0 && prj.Node[j as usize].pondedArea > 0.0) as i32;
    let ju = j as usize;

    // --- update depth statistics
    prj.stats.NodeStats[ju].avgDepth += newDepth;
    if newDepth > prj.stats.NodeStats[ju].maxDepth {
        prj.stats.NodeStats[ju].maxDepth = newDepth;
        prj.stats.NodeStats[ju].maxDepthDate = aDate;
    }

    // --- update flooding, ponding, and surcharge statistics
    if prj.Node[ju].type_ != OUTFALL {
        if newVolume > prj.Node[ju].fullVolume || prj.Node[ju].overflow > 0.0 {
            prj.stats.NodeStats[ju].timeFlooded += tStep;
            prj.stats.NodeStats[ju].volFlooded += prj.Node[ju].overflow * tStep;
            if canPond != 0 {
                prj.stats.NodeStats[ju].maxPondedVol =
                    MAX(prj.stats.NodeStats[ju].maxPondedVol, newVolume - prj.Node[ju].fullVolume);
            }
        }

        // --- for dynamic wave routing, classify a node as
        //     surcharged if its water level exceeds its crown elev.
        if prj.RouteModel == DW {
            if (prj.Node[ju].type_ != STORAGE || prj.Node[ju].surDepth > 0.0)
                && newDepth + prj.Node[ju].invertElev + FUDGE >= prj.Node[ju].crownElev
            {
                prj.stats.NodeStats[ju].timeSurcharged += tStep;
            }
        }
    }

    // --- update storage statistics
    if prj.Node[ju].type_ == STORAGE {
        k = prj.Node[ju].subIndex;
        let ku = k as usize;
        let si = prj.Node[ju].subIndex as usize;
        prj.stats.StorageStats[ku].avgVol += newVolume;
        prj.stats.StorageStats[ku].evapLosses += prj.Storage[si].evapLoss;
        prj.stats.StorageStats[ku].exfilLosses += prj.Storage[si].exfilLoss;

        newVolume = MIN(newVolume, prj.Node[ju].fullVolume);
        if newVolume > prj.stats.StorageStats[ku].maxVol {
            prj.stats.StorageStats[ku].maxVol = newVolume;
            prj.stats.StorageStats[ku].maxVolDate = aDate;
        }
        prj.stats.StorageStats[ku].maxFlow = MAX(prj.stats.StorageStats[ku].maxFlow, prj.Node[ju].outflow);
    }

    // --- update outfall statistics
    if prj.Node[ju].type_ == OUTFALL {
        k = prj.Node[ju].subIndex;
        let ku = k as usize;
        let inflow = prj.Node[ju].inflow;
        if inflow >= MIN_RUNOFF_FLOW {
            prj.stats.OutfallStats[ku].avgFlow += inflow;
            prj.stats.OutfallStats[ku].maxFlow = MAX(prj.stats.OutfallStats[ku].maxFlow, inflow);
            prj.stats.OutfallStats[ku].totalPeriods += 1;
        }
        p = 0;
        while p < prj.Nobjects[POLLUT as usize] {
            prj.stats.OutfallStats[ku].totalLoad[p as usize] +=
                inflow * prj.Node[ju].newQual[p as usize] * tStep;
            p += 1;
        }
        prj.stats.SysOutfallFlow += inflow;
    }

    // --- update inflow statistics
    prj.stats.NodeStats[ju].totLatFlow +=
        (prj.Node[ju].oldLatFlow + prj.Node[ju].newLatFlow) * 0.5 * tStep;
    if prj.Node[ju].newLatFlow.abs() > prj.stats.NodeStats[ju].maxLatFlow.abs() {
        prj.stats.NodeStats[ju].maxLatFlow = prj.Node[ju].newLatFlow;
    }
    if prj.Node[ju].inflow > prj.stats.NodeStats[ju].maxInflow {
        prj.stats.NodeStats[ju].maxInflow = prj.Node[ju].inflow;
        prj.stats.NodeStats[ju].maxInflowDate = aDate;
    }

    // --- update overflow statistics
    if prj.Node[ju].overflow > prj.stats.NodeStats[ju].maxOverflow {
        prj.stats.NodeStats[ju].maxOverflow = prj.Node[ju].overflow;
        prj.stats.NodeStats[ju].maxOverflowDate = aDate;
    }
}

//=============================================================================

fn stats_updateLinkStats(prj: &mut Project, j: i32, tStep: f64, aDate: DateTime) {
//
//  Input:   j = link index
//           tStep = routing time step (sec)
//           aDate = current date/time
//  Output:  none
//  Purpose: updates flow statistics for a link.
//
    let mut k: i32;
    let q: f64;
    let v: f64;
    let dq: f64;
    let ju = j as usize;

    // --- update max. flow
    dq = prj.Link[ju].newFlow - prj.Link[ju].oldFlow;
    q = prj.Link[ju].newFlow.abs();
    if q > prj.stats.LinkStats[ju].maxFlow {
        prj.stats.LinkStats[ju].maxFlow = q;
        prj.stats.LinkStats[ju].maxFlowDate = aDate;
    }

    // --- update max. velocity
    v = link_getVelocity(prj, j, q, prj.Link[ju].newDepth);
    if v > prj.stats.LinkStats[ju].maxVeloc {
        prj.stats.LinkStats[ju].maxVeloc = v;
    }

    // --- update max. depth
    if prj.Link[ju].newDepth > prj.stats.LinkStats[ju].maxDepth {
        prj.stats.LinkStats[ju].maxDepth = prj.Link[ju].newDepth;
    }

    if prj.Link[ju].type_ == PUMP {
        if q >= prj.Link[ju].qFull {
            prj.stats.LinkStats[ju].timeFullFlow += tStep;
        }
        if q > MIN_RUNOFF_FLOW {
            k = prj.Link[ju].subIndex;
            let ku = k as usize;
            prj.stats.PumpStats[ku].minFlow = MIN(prj.stats.PumpStats[ku].minFlow, q);
            prj.stats.PumpStats[ku].maxFlow = prj.stats.LinkStats[ju].maxFlow;
            prj.stats.PumpStats[ku].avgFlow += q;
            prj.stats.PumpStats[ku].volume += q * tStep;
            prj.stats.PumpStats[ku].utilized += tStep;
            let power = link_getPower(prj, j);
            prj.stats.PumpStats[ku].energy += power * tStep / 3600.0;
            if prj.Link[ju].flowClass == DN_DRY {
                prj.stats.PumpStats[ku].offCurveLow += tStep;
            }
            if prj.Link[ju].flowClass == UP_DRY {
                prj.stats.PumpStats[ku].offCurveHigh += tStep;
            }
            if prj.Link[ju].oldFlow < MIN_RUNOFF_FLOW {
                prj.stats.PumpStats[ku].startUps += 1;
            }
            prj.stats.PumpStats[ku].totalPeriods += 1;
            prj.stats.LinkStats[ju].timeSurcharged += tStep;
            prj.stats.LinkStats[ju].timeFullUpstream += tStep;
            prj.stats.LinkStats[ju].timeFullDnstream += tStep;
        }
    } else if prj.Link[ju].type_ == CONDUIT {
        // --- update time under normal flow & inlet control
        if prj.Link[ju].normalFlow != 0 {
            prj.stats.LinkStats[ju].timeNormalFlow += tStep;
        }
        if prj.Link[ju].inletControl != 0 {
            prj.stats.LinkStats[ju].timeInletControl += tStep;
        }

        // --- update flow classification distribution
        k = prj.Link[ju].flowClass;
        if k >= 0 && k < MAX_FLOW_CLASSES {
            prj.stats.LinkStats[ju].timeInFlowClass[k as usize] += tStep;
        }

        // --- update time conduit is full
        k = prj.Link[ju].subIndex;
        let ku = k as usize;
        if q >= prj.Link[ju].qFull * prj.Conduit[ku].barrels as f64 {
            prj.stats.LinkStats[ju].timeFullFlow += tStep;
        }
        if prj.Conduit[ku].capacityLimited != 0 {
            prj.stats.LinkStats[ju].timeCapacityLimited += tStep;
        }

        match prj.Conduit[ku].fullState as i32 {
            ALL_FULL => {
                prj.stats.LinkStats[ju].timeSurcharged += tStep;
                prj.stats.LinkStats[ju].timeFullUpstream += tStep;
                prj.stats.LinkStats[ju].timeFullDnstream += tStep;
            }
            UP_FULL => {
                prj.stats.LinkStats[ju].timeFullUpstream += tStep;
            }
            DN_FULL => {
                prj.stats.LinkStats[ju].timeFullDnstream += tStep;
            }
            _ => {}
        }

        // --- update max. degree filled for streets
        //     (MAX may call street_getExtentFilled twice in C; it only reads)
        if prj.Link[ju].xsect.type_ == STREET_XSECT {
            let filled = street_getExtentFilled(prj, j);
            prj.stats.LinkStats[ju].maxStreetFilled = MAX(prj.stats.LinkStats[ju].maxStreetFilled, filled);
        }
    }

    // --- update flow turn count
    k = prj.stats.LinkStats[ju].flowTurnSign;
    prj.stats.LinkStats[ju].flowTurnSign = SGNi(dq);
    if dq.abs() > 0.001 && k * prj.stats.LinkStats[ju].flowTurnSign < 0 {
        prj.stats.LinkStats[ju].flowTurns += 1;
    }
}

//=============================================================================

fn stats_findMaxStats(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: finds nodes & links with highest mass balance errors
//           & highest times Courant time-step critical.
//
    let mut j: i32;
    let mut x: f64;
    let z: f64;
    let mut stepCount: f64;

    // --- initialize max. stats arrays
    j = 0;
    while j < MAX_STATS {
        let ju = j as usize;
        prj.stats.MaxMassBalErrs[ju].objType = NODE;
        prj.stats.MaxMassBalErrs[ju].index = -1;
        prj.stats.MaxMassBalErrs[ju].value = -1.0;
        prj.stats.MaxCourantCrit[ju].index = -1;
        prj.stats.MaxCourantCrit[ju].value = -1.0;
        prj.stats.MaxFlowTurns[ju].index = -1;
        prj.stats.MaxFlowTurns[ju].value = -1.0;
        prj.stats.MaxNonConverged[ju].index = -1;
        prj.stats.MaxNonConverged[ju].value = 0.0;
        j += 1;
    }

    // --- find links with most flow turns during reporting period
    if prj.ReportStepCount > 2 {
        stepCount = prj.ReportStepCount as f64;
        z = 100.0 / (2.0 / 3.0 * (stepCount - 2.0));
        j = 0;
        while j < prj.Nobjects[LINK as usize] {
            x = prj.stats.LinkStats[j as usize].flowTurns as f64 * z;
            stats_updateMaxStats(&mut prj.stats.MaxFlowTurns, LINK, j, x);
            j += 1;
        }
    }

    // --- find nodes with largest mass balance errors
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        // --- skip terminal nodes and nodes with negligible inflow
        if prj.Node[ju].degree <= 0 {
            j += 1;
            continue;
        }
        if prj.massbal.NodeInflow[ju] <= 0.1 {
            j += 1;
            continue;
        }

        // --- evaluate mass balance error
        //     (Note: NodeInflow & NodeOutflow include any initial and final
        //            stored volumes, respectively).
        if prj.massbal.NodeInflow[ju] > 0.0 {
            x = 1.0 - prj.massbal.NodeOutflow[ju] / prj.massbal.NodeInflow[ju];
        } else if prj.massbal.NodeOutflow[ju] > 0.0 {
            x = -1.0;
        } else {
            x = 0.0;
        }
        stats_updateMaxStats(&mut prj.stats.MaxMassBalErrs, NODE, j, 100.0 * x);
        j += 1;
    }

    // --- following stats apply to all (startup + reporting) time periods
    stepCount = prj.stats.TimeStepStats.timeStepCount as f64;

    // --- find nodes with highest nonconvergence frequency
    if prj.RouteModel == DW {
        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            let v = prj.stats.NodeStats[j as usize].nonConvergedCount as f64 / stepCount;
            stats_updateMaxStats(&mut prj.stats.MaxNonConverged, NODE, j, v);
            j += 1;
        }
    }

    // --- stop if not using a variable time step
    if prj.RouteModel != DW || prj.CourantFactor == 0.0 {
        return;
    }

    // --- find nodes most frequently Courant critical
    if stepCount == 0.0 {
        return;
    }
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        x = prj.stats.NodeStats[j as usize].timeCourantCritical / stepCount;
        stats_updateMaxStats(&mut prj.stats.MaxCourantCrit, NODE, j, 100.0 * x);
        j += 1;
    }

    // --- find links most frequently Courant critical
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        x = prj.stats.LinkStats[j as usize].timeCourantCritical / stepCount;
        stats_updateMaxStats(&mut prj.stats.MaxCourantCrit, LINK, j, 100.0 * x);
        j += 1;
    }
}

//=============================================================================

fn stats_updateMaxStats(maxStats: &mut [TMaxStats], i: i32, j: i32, x: f64) {
//
//  Input:   maxStats[] = array of critical statistics values
//           i = object category (NODE or LINK)
//           j = object index
//           x = value of statistic for the object
//  Output:  none
//  Purpose: updates the collection of most critical statistics
//
    let mut k: i32;
    let mut maxStats1 = TMaxStats::default();
    let mut maxStats2: TMaxStats;
    maxStats1.objType = i;
    maxStats1.index = j;
    maxStats1.value = x;
    k = 0;
    while k < MAX_STATS {
        if maxStats1.value.abs() > maxStats[k as usize].value.abs() {
            maxStats2 = maxStats[k as usize];
            maxStats[k as usize] = maxStats1;
            maxStats1 = maxStats2;
        }
        k += 1;
    }
}
