//! routing.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   routing.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//             M. Tryby (EPA)
//
//   Conveyance system routing functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Nodal evap/seepage losses computed using conditions at start of time step.
//   - DWF pollutant concentrations ignored if DWF is negative.
//   - Separate mass balance accounting made for storage evap. & seepage.
//   - Nodal mass balance accounting for negative lateral inflows corrected.
//   Build 5.1.008:
//   - Initialization of flow and quality routing systems moved here from swmm5.c.
//   - Lateral inflows now evaluated at start (not end) of time step.
//   - Flows from LID drains included in lateral inflows.
//   - Conduit evap/seepage losses multiplied by number of barrels before
//     being added into mass balances.
//   Build 5.1.010:
//   - Time when a link's setting is changed is recorded.
//   Build 5.1.011:
//   - Support added for limiting flow routing to specific events.
//   Build 5.1.012:
//   - routing_execute() was re-written so that Routing Events and
//     Skip Steady Flow options work together correctly.
//   Build 5.1.013:
//   - Support added for evaluating controls rules at RuleStep time interval.
//   - Back flow through Outfall nodes now treated as External Inflows for
//     mass balance purposes.
//   - Global infiltration factor for storage seepage set in routing_execute.
//   Build 5.2.0:
//   - Support added for street flow capture and sewer backflow thru inlets.
//   - Shell sort replaces insertion sort for sorting Event array.
//-----------------------------------------------------------------------------

use crate::prelude::*;

/// The file-scope variables of routing.c.
#[derive(Default)]
pub struct RoutingState {
    SortedLinks: Vec<i32>,
    NextEvent: i32,
    BetweenEvents: i32,
    NewRuleTime: f64,
}

//=============================================================================

/// Called by: swmm5.
pub fn routing_open(prj: &mut Project) -> i32 {
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: initializes the routing analyzer.
//
    // --- open treatment system
    if treatmnt_open(prj) == 0 {
        return prj.ErrorCode;
    }

    // --- topologically sort the links
    prj.routing.SortedLinks = Vec::new();
    if prj.Nobjects[LINK as usize] > 0 {
        let mut sortedLinks = vec![0i32; prj.Nobjects[LINK as usize] as usize];
        toposort_sortLinks(prj, &mut sortedLinks);
        prj.routing.SortedLinks = sortedLinks;
        if prj.ErrorCode != 0 {
            return prj.ErrorCode;
        }
    }

    // --- open any routing interface files
    iface_openRoutingFiles(prj);

    // --- initialize flow and quality routing systems
    let routeModel = prj.RouteModel;
    flowrout_init(prj, routeModel);
    if prj.Fhotstart1.mode as i32 == NO_FILE {
        qualrout_init(prj);
    }

    // --- initialize routing events
    if prj.NumEvents > 0 {
        sortEvents(prj);
    }
    prj.routing.NextEvent = 0;
    prj.routing.BetweenEvents = (prj.NumEvents > 0) as i32;
    prj.routing.NewRuleTime = 0.0;
    prj.ErrorCode
}

//=============================================================================

/// Called by: swmm5.
pub fn routing_close(prj: &mut Project, routingModel: i32) {
//
//  Input:   routingModel = routing method code
//  Output:  none
//  Purpose: closes down the routing analyzer.
//
    // --- close any routing interface files
    iface_closeRoutingFiles(prj);

    // --- free allocated memory
    flowrout_close(prj, routingModel);
    treatmnt_close(prj);
    prj.routing.SortedLinks = Vec::new();
}

//=============================================================================

/// Called by: swmm5.
pub fn routing_getRoutingStep(prj: &mut Project, routingModel: i32, fixedStep: f64) -> f64 {
//
//  Input:   routingModel = routing method code
//           fixedStep = user-supplied time step (sec)
//  Output:  returns a routing time step (sec)
//  Purpose: determines time step used for flow routing at current time period.
//
    let mut date1: f64;
    let date2: f64;
    let nextTime: f64;
    let mut routingStep: f64 = 0.0;
    let nextRuleTime: f64;
    let nextRoutingTime: f64;

    if prj.Nobjects[LINK as usize] == 0 {
        return fixedStep;
    }

    // --- find largest step possible if between routing events
    if prj.NumEvents > 0 && prj.routing.BetweenEvents != 0 {
        nextTime = MIN(prj.NewRunoffTime, prj.ReportTime);
        date1 = getDateTime(prj, prj.NewRoutingTime);
        date2 = getDateTime(prj, nextTime);
        if date2 > date1 && date2 < prj.Event[prj.routing.NextEvent as usize].start {
            routingStep = (nextTime - prj.NewRoutingTime) / 1000.0;
        } else {
            date1 = getDateTime(prj, prj.NewRoutingTime + 1000.0 * fixedStep);
            if date1 < prj.Event[prj.routing.NextEvent as usize].start {
                return fixedStep;
            }
        }
    }

    // --- otherwise use a regular flow-routing based time step
    if routingStep == 0.0 {
        routingStep = flowrout_getRoutingStep(prj, routingModel, fixedStep);
    }

    // --- determine if control rule time interval reached
    if prj.RuleStep > 0 {
        nextRuleTime = prj.routing.NewRuleTime + 1000. * prj.RuleStep as f64;
        nextRoutingTime = prj.NewRoutingTime + 1000. * routingStep;
        if nextRoutingTime >= nextRuleTime {
            routingStep = (nextRuleTime - prj.NewRoutingTime) / 1000.0;
        }
    }
    routingStep
}

//=============================================================================

/// Called by: swmm5.
pub fn routing_execute(prj: &mut Project, routingModel: i32, routingStep: f64) {
//
//  Input:   routingModel = routing method code
//           routingStep = routing time step (sec)
//  Output:  none
//  Purpose: executes the routing process at the current time period.
//
    let mut trialsCount: i32 = 1;          // trials required to solve flow routing
    let actionCount: i32;                  // number of control actions taken
    let mut inSteadyState: i32 = TRUE;     // system is in steady state
    let currentDate: DateTime;             // date at start of routing step
    let stepFlowError: f64;                // 1 - (system outflow) / (system inflow)

    if prj.ErrorCode != 0 {
        return;
    }

    // --- update mass balance totals over previous half time step
    massbal_updateRoutingTotals(prj, routingStep / 2.);

    // --- take any applicable control rule actions
    currentDate = getDateTime(prj, prj.NewRoutingTime);
    actionCount = evaluateControlRules(prj, currentDate, routingStep);

    // --- initialize mass balance and system inflow variables
    stepFlowError = massbal_getStepFlowError(prj);
    massbal_initTimeStepTotals(prj);
    initSystemInflows(prj);

    // --- check that current date falls within a user-speficied event period
    prj.routing.BetweenEvents = isBetweenEvents(prj, currentDate);
    if prj.routing.BetweenEvents == FALSE {
        // --- apply current inflows to conveyance system
        addSystemInflows(prj, currentDate, routingStep);
        inlet_findCapturedFlows(prj, routingStep);

        // --- route flows if system is not in steady state
        inSteadyState = isInSteadyState(prj, actionCount, stepFlowError);
        if inSteadyState == FALSE {
            trialsCount = routeFlow(prj, routingModel, routingStep);
        }

        // --- route water quality constituents
        if prj.Nobjects[POLLUT as usize] > 0 && prj.IgnoreQuality == 0 {
            inlet_adjustQualInflows(prj);
            qualrout_execute(prj, routingStep);
        }

        // --- update mass balance totals for flows leaving the system
        removeSystemOutflows(prj, routingStep);
        inlet_adjustQualOutflows(prj);

        // --- update time step & flow routing statistics
        if prj.Nobjects[LINK as usize] > 0 {
            let aDate = getDateTime(prj, prj.NewRoutingTime);
            stats_updateFlowStats(prj, routingStep, aDate);
            stats_updateTimeStepStats(prj, routingStep, trialsCount, inSteadyState);
        }
    }

    // --- update mass balance totals over the current half time step
    massbal_updateRoutingTotals(prj, routingStep / 2.);
}

//=============================================================================

fn evaluateControlRules(prj: &mut Project, currentDate: DateTime, routingStep: f64) -> i32 {
    let mut actionCount: i32 = 0;

    // --- find new link target settings that are not related to
    // --- control rules (e.g., pump on/off depth limits)
    for j in 0..prj.Nobjects[LINK as usize] {
        link_setTargetSetting(prj, j);
    }

    // --- evaluate control rules if next evaluation time reached
    if prj.RuleStep == 0 || (prj.NewRoutingTime - prj.routing.NewRuleTime).abs() < 1.0 {
        let elapsed = currentDate - prj.StartDateTime;
        controls_evaluate(prj, currentDate, elapsed, routingStep / SECperDAY as f64);
    }

    // --- change each link's actual setting if it differs from its target
    for j in 0..prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        if prj.Link[ju].targetSetting != prj.Link[ju].setting {
            // --- update time when link was switched between open & closed
            if prj.Link[ju].targetSetting * prj.Link[ju].setting == 0.0 {
                prj.Link[ju].timeLastSet = currentDate;
            }

            // --- implement the change in the link's setting
            link_setSetting(prj, j, routingStep);
            actionCount += 1;
        }
    }

    // --- update value of elapsed routing time (in milliseconds)
    prj.OldRoutingTime = prj.NewRoutingTime;
    prj.NewRoutingTime = prj.NewRoutingTime + 1000.0 * routingStep;

    // --- see if control rule evaluation time should be advanced
    if (prj.NewRoutingTime - (prj.routing.NewRuleTime + 1000.0 * prj.RuleStep as f64)).abs() < 1 as f64 {
        prj.routing.NewRuleTime += 1000.0 * prj.RuleStep as f64;
    }
    actionCount
}

//=============================================================================

fn initSystemInflows(prj: &mut Project) {
    // --- replace old water quality state with new state
    if prj.Nobjects[POLLUT as usize] > 0 {
        for j in 0..prj.Nobjects[NODE as usize] {
            node_setOldQualState(prj, j);
        }
        for j in 0..prj.Nobjects[LINK as usize] {
            link_setOldQualState(prj, j);
        }
    }

    // --- set infiltration factor for storage unit seepage
    //     (-1 argument indicates global factor is used)
    infil_setInfilFactor(prj, -1);

    // --- initialize lateral inflows at nodes
    for j in 0..prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        prj.Node[ju].oldLatFlow = prj.Node[ju].newLatFlow;
        prj.Node[ju].newLatFlow = 0.0;
    }
}

//=============================================================================

fn isBetweenEvents(prj: &mut Project, currentDate: DateTime) -> i32 {
    // --- if no events defined then result is always false
    if prj.NumEvents == 0 {
        return FALSE;
    }

    // --- currrent event period has ended so result is true
    if currentDate > prj.Event[prj.routing.NextEvent as usize].end {
        prj.routing.NextEvent += 1;
        return TRUE;
    }
    // --- we've entered the next event period so result is false
    else if currentDate >= prj.Event[prj.routing.NextEvent as usize].start {
        return FALSE;
    }
    TRUE
}

//=============================================================================

fn addSystemInflows(prj: &mut Project, currentDate: DateTime, routingStep: f64) {
    // --- find evap. & seepage losses from storage nodes
    for j in 0..prj.Nobjects[NODE as usize] {
        prj.Node[j as usize].losses = node_getLosses(prj, j, routingStep);
    }

    // --- add lateral inflows at nodes
    addExternalInflows(prj, currentDate);
    addDryWeatherInflows(prj, currentDate);
    let oldRoutingTime = prj.OldRoutingTime;
    addWetWeatherInflows(prj, oldRoutingTime);
    addGroundwaterInflows(prj, oldRoutingTime);
    addLidDrainInflows(prj, oldRoutingTime);
    addRdiiInflows(prj, currentDate);
    addIfaceInflows(prj, currentDate);

    // --- initialize node inflow for quality routing
    for j in 0..prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        prj.Node[ju].qualInflow = MAX(0.0, prj.Node[ju].newLatFlow);
    }
}

//=============================================================================

fn isInSteadyState(prj: &mut Project, actionCount: i32, stepFlowError: f64) -> i32 {
    // --- check if can skip steady state periods based on flows
    if prj.SkipSteadyState != 0 {
        if prj.OldRoutingTime == 0.0
            || actionCount > 0
            || stepFlowError.abs() > prj.SysFlowTol
            || inflowHasChanged(prj) != 0
        {
            return FALSE;
        } else {
            return TRUE;
        }
    }
    FALSE
}

//=============================================================================

fn routeFlow(prj: &mut Project, routingModel: i32, routingStep: f64) -> i32 {
    let mut stepCount: i32 = 1;

    // --- replace old hydraulic state values with current ones
    for j in 0..prj.Nobjects[LINK as usize] {
        link_setOldHydState(prj, j);
    }
    for j in 0..prj.Nobjects[NODE as usize] {
        node_setOldHydState(prj, j);
    }

    // --- initialize node inflows to lateral flows, outflows to evap +
    //     seepage losses, & overflows to excess stored volume
    for j in 0..prj.Nobjects[NODE as usize] {
        node_initFlows(prj, j, routingStep);
    }

    // --- route flow through the drainage network
    if prj.Nobjects[LINK as usize] > 0 {
        // (SortedLinks is not read by anything flowrout_execute calls)
        let sortedLinks = std::mem::take(&mut prj.routing.SortedLinks);
        stepCount = flowrout_execute(prj, &sortedLinks, routingModel, routingStep);
        prj.routing.SortedLinks = sortedLinks;
    }

    // --- save overflows at inlet capture nodes as inlet backflow
    stepCount
}

//=============================================================================

fn removeSystemOutflows(prj: &mut Project, routingStep: f64) {
    // --- remove evaporation, infiltration & outflows from system
    removeStorageLosses(prj, routingStep);
    removeConduitLosses(prj);
    removeOutflows(prj, routingStep);
}

//=============================================================================

fn addExternalInflows(prj: &mut Project, currentDate: DateTime) {
//
//  Input:   currentDate = current date/time
//  Output:  none
//  Purpose: adds direct external inflows to nodes at current date.
//
    let mut p: i32;
    let mut q: f64;
    let mut w: f64;

    // --- for each node with a defined external inflow
    for j in 0..prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        // --- get flow inflow
        q = prj.Node[ju].apiExtInflow;
        let mut m = 0usize;
        while m < prj.Node[ju].extInflow.len() {
            let inflow: TExtInflow = prj.Node[ju].extInflow[m];
            if inflow.type_ == FLOW_INFLOW {
                q += inflow_getExtInflow(prj, &inflow, currentDate);
                break;
            } else {
                m += 1;
            }
        }
        if q.abs() < FLOW_TOL {
            q = 0.0;
        }

        // --- add flow inflow to node's lateral inflow
        prj.Node[ju].newLatFlow += q;
        if q >= 0.0 {
            massbal_addInflowFlow(prj, EXTERNAL_INFLOW, q);
        } else {
            massbal_addOutflowFlow(prj, -q, FALSE);
            continue;
        }

        // --- add on any inflow (i.e., reverse flow) through an outfall
        if prj.Node[ju].type_ == OUTFALL && prj.Node[ju].oldNetInflow < 0.0 {
            q = q - prj.Node[ju].oldNetInflow;
        }

        // --- get pollutant mass inflows
        let mut m = 0usize;
        while m < prj.Node[ju].extInflow.len() {
            let inflow: TExtInflow = prj.Node[ju].extInflow[m];
            if inflow.type_ != FLOW_INFLOW {
                p = inflow.param;
                w = inflow_getExtInflow(prj, &inflow, currentDate);
                if inflow.type_ == CONCEN_INFLOW {
                    w *= q;
                }
                prj.Node[ju].newQual[p as usize] += w;
                massbal_addInflowQual(prj, EXTERNAL_INFLOW, p, w);
            }
            m += 1;
        }
    }
}

//=============================================================================

fn addDryWeatherInflows(prj: &mut Project, currentDate: DateTime) {
//
//  Input:   currentDate = current date/time
//  Output:  none
//  Purpose: adds dry weather inflows to nodes at current date.
//
    let mut p: i32;
    let month: i32;
    let day: i32;
    let hour: i32;
    let mut q: f64;
    let mut w: f64;

    // --- get month (zero-based), day-of-week (zero-based),
    //     & hour-of-day for routing date/time
    month = datetime_monthOfYear(currentDate) - 1;
    day = datetime_dayOfWeek(currentDate) - 1;
    hour = datetime_hourOfDay(currentDate);

    // --- for each node with a defined dry weather inflow
    for j in 0..prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        if prj.Node[ju].dwfInflow.is_empty() {
            continue;
        }

        // --- get flow inflow (i.e., the inflow whose param code is -1)
        q = 0.0;
        let mut m = 0usize;
        while m < prj.Node[ju].dwfInflow.len() {
            let inflow: TDwfInflow = prj.Node[ju].dwfInflow[m];
            if inflow.param < 0 {
                q = inflow_getDwfInflow(prj, &inflow, month, day, hour);
                break;
            }
            m += 1;
        }
        if q.abs() < FLOW_TOL {
            q = 0.0;
        }

        // --- add flow inflow to node's lateral inflow
        prj.Node[ju].newLatFlow += q;
        massbal_addInflowFlow(prj, DRY_WEATHER_INFLOW, q);

        // --- stop if inflow is non-positive
        if q <= 0.0 {
            continue;
        }

        // --- add default DWF pollutant inflows
        for p in 0..prj.Nobjects[POLLUT as usize] {
            let pu = p as usize;
            if prj.Pollut[pu].dwfConcen > 0.0 {
                w = q * prj.Pollut[pu].dwfConcen;
                prj.Node[ju].newQual[pu] += w;
                massbal_addInflowQual(prj, DRY_WEATHER_INFLOW, p, w);
            }
        }

        // --- get pollutant mass inflows
        let mut m = 0usize;
        while m < prj.Node[ju].dwfInflow.len() {
            let inflow: TDwfInflow = prj.Node[ju].dwfInflow[m];
            if inflow.param >= 0 {
                p = inflow.param;
                let pu = p as usize;
                w = q * inflow_getDwfInflow(prj, &inflow, month, day, hour);
                prj.Node[ju].newQual[pu] += w;
                massbal_addInflowQual(prj, DRY_WEATHER_INFLOW, p, w);

                // --- subtract off any default inflow
                if prj.Pollut[pu].dwfConcen > 0.0 {
                    w = q * prj.Pollut[pu].dwfConcen;
                    prj.Node[ju].newQual[pu] -= w;
                    massbal_addInflowQual(prj, DRY_WEATHER_INFLOW, p, -w);
                }
            }
            m += 1;
        }
    }
}

//=============================================================================

fn addWetWeatherInflows(prj: &mut Project, routingTime: f64) {
//
//  Input:   routingTime = elasped time (millisec)
//  Output:  none
//  Purpose: adds runoff inflows to nodes at current elapsed time.
//
    let mut j: i32;
    let mut q: f64;
    let mut w: f64;
    let mut f: f64;

    // --- find where current routing time lies between latest runoff times
    if prj.Nobjects[SUBCATCH as usize] == 0 {
        return;
    }
    f = (routingTime - prj.OldRunoffTime) / (prj.NewRunoffTime - prj.OldRunoffTime);
    if f < 0.0 {
        f = 0.0;
    }
    if f > 1.0 {
        f = 1.0;
    }

    // for each subcatchment outlet node,
    // add interpolated runoff flow & pollutant load to node's inflow
    for i in 0..prj.Nobjects[SUBCATCH as usize] {
        j = prj.Subcatch[i as usize].outNode;
        if j >= 0 {
            let ju = j as usize;
            // add runoff flow to lateral inflow
            q = subcatch_getWtdOutflow(prj, i, f);     // current runoff flow
            prj.Node[ju].newLatFlow += q;
            massbal_addInflowFlow(prj, WET_WEATHER_INFLOW, q);

            // add pollutant load
            for p in 0..prj.Nobjects[POLLUT as usize] {
                w = surfqual_getWtdWashoff(prj, i, p, f);
                prj.Node[ju].newQual[p as usize] += w;
                massbal_addInflowQual(prj, WET_WEATHER_INFLOW, p, w);
            }
        }
    }
}

//=============================================================================

fn addGroundwaterInflows(prj: &mut Project, routingTime: f64) {
//
//  Input:   routingTime = elasped time (millisec)
//  Output:  none
//  Purpose: adds groundwater inflows to nodes at current elapsed time.
//
    let mut j: i32;
    let mut q: f64;
    let mut w: f64;
    let mut f: f64;

    // --- find where current routing time lies between latest runoff times
    if prj.Nobjects[SUBCATCH as usize] == 0 {
        return;
    }
    f = (routingTime - prj.OldRunoffTime) / (prj.NewRunoffTime - prj.OldRunoffTime);
    if f < 0.0 {
        f = 0.0;
    }
    if f > 1.0 {
        f = 1.0;
    }

    // --- for each subcatchment
    for i in 0..prj.Nobjects[SUBCATCH as usize] {
        let iu = i as usize;
        // --- see if subcatch contains groundwater
        // (a copy of *gw: nothing below changes the subcatchment's groundwater)
        let gwCopy: Option<TGroundwater> = prj.Subcatch[iu].groundwater.as_deref().copied();
        if let Some(gw) = gwCopy {
            // --- identify node receiving groundwater flow
            j = gw.node;
            if j >= 0 {
                // add groundwater flow to lateral inflow
                q = ((1.0 - f) * gw.oldFlow + f * gw.newFlow) * prj.Subcatch[iu].area;
                if q.abs() < FLOW_TOL {
                    continue;
                }
                let ju = j as usize;
                prj.Node[ju].newLatFlow += q;
                massbal_addInflowFlow(prj, GROUNDWATER_INFLOW, q);

                // add pollutant load (for positive inflow)
                if q > 0.0 {
                    for p in 0..prj.Nobjects[POLLUT as usize] {
                        w = q * prj.Pollut[p as usize].gwConcen;
                        prj.Node[ju].newQual[p as usize] += w;
                        massbal_addInflowQual(prj, GROUNDWATER_INFLOW, p, w);
                    }
                }
            }
        }
    }
}

//=============================================================================

fn addLidDrainInflows(prj: &mut Project, routingTime: f64) {
//
//  Input:   routingTime = elasped time (millisec)
//  Output:  none
//  Purpose: adds inflows to nodes receiving LID drain flow.
//
    let mut f: f64;

    // for each subcatchment
    if prj.Nobjects[SUBCATCH as usize] == 0 {
        return;
    }
    f = (routingTime - prj.OldRunoffTime) / (prj.NewRunoffTime - prj.OldRunoffTime);
    if f < 0.0 {
        f = 0.0;
    }
    if f > 1.0 {
        f = 1.0;
    }
    for j in 0..prj.Nobjects[SUBCATCH as usize] {
        if prj.Subcatch[j as usize].area > 0.0 && prj.Subcatch[j as usize].lidArea > 0.0 {
            lid_addDrainInflow(prj, j, f);
        }
    }
}

//=============================================================================

fn addRdiiInflows(prj: &mut Project, currentDate: DateTime) {
//
//  Input:   currentDate = current date/time
//  Output:  none
//  Purpose: adds RDII inflows to nodes at current date.
//
    // C leaves j and q uninitialized; rdii_getRdiiFlow always sets both
    // here because 0 <= i < numRdiiNodes.
    let mut j: i32 = 0;
    let mut q: f64 = 0.0;
    let mut w: f64;
    let numRdiiNodes: i32;

    // --- see if any nodes have RDII at current date
    numRdiiNodes = rdii_getNumRdiiFlows(prj, currentDate);

    // --- add RDII flow to each node's lateral inflow
    for i in 0..numRdiiNodes {
        rdii_getRdiiFlow(prj, i, &mut j, &mut q);
        if j < 0 {
            continue;
        }
        if q.abs() < FLOW_TOL {
            continue;
        }
        let ju = j as usize;
        prj.Node[ju].newLatFlow += q;
        massbal_addInflowFlow(prj, RDII_INFLOW, q);

        // add pollutant load (for positive inflow)
        if q > 0.0 {
            for p in 0..prj.Nobjects[POLLUT as usize] {
                w = q * prj.Pollut[p as usize].rdiiConcen;
                prj.Node[ju].newQual[p as usize] += w;
                massbal_addInflowQual(prj, RDII_INFLOW, p, w);
            }
        }
    }
}

//=============================================================================

fn addIfaceInflows(prj: &mut Project, currentDate: DateTime) {
//
//  Input:   currentDate = current date/time
//  Output:  none
//  Purpose: adds inflows from routing interface file to nodes at current date.
//
    let mut j: i32;
    let mut q: f64;
    let mut w: f64;
    let numIfaceNodes: i32;

    // --- see if any nodes have interface inflows at current date
    if prj.Finflows.mode as i32 != USE_FILE {
        return;
    }
    numIfaceNodes = iface_getNumIfaceNodes(prj, currentDate);

    // --- add interface flow to each node's lateral inflow
    for i in 0..numIfaceNodes {
        j = iface_getIfaceNode(prj, i);
        if j < 0 {
            continue;
        }
        q = iface_getIfaceFlow(prj, i);
        if q.abs() < FLOW_TOL {
            continue;
        }
        let ju = j as usize;
        prj.Node[ju].newLatFlow += q;
        massbal_addInflowFlow(prj, EXTERNAL_INFLOW, q);

        // add pollutant load (for positive inflow)
        if q > 0.0 {
            for p in 0..prj.Nobjects[POLLUT as usize] {
                w = q * iface_getIfaceQual(prj, i, p);
                prj.Node[ju].newQual[p as usize] += w;
                massbal_addInflowQual(prj, EXTERNAL_INFLOW, p, w);
            }
        }
    }
}

//=============================================================================

fn inflowHasChanged(prj: &mut Project) -> i32 {
//
//  Input:   none
//  Output:  returns TRUE if external inflows or outfall flows have changed
//           from the previous time step
//  Purpose: checks if the hydraulic state of the system has changed from
//           the previous time step.
//
    let mut diff: f64;
    let mut qOld: f64;
    let mut qNew: f64;

    // --- check if external inflows or outfall flows have changed
    for j in 0..prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        qOld = prj.Node[ju].oldLatFlow;
        qNew = prj.Node[ju].newLatFlow;
        if qOld.abs() > TINY {
            diff = (qNew / qOld) - 1.0;
        } else if qNew.abs() > TINY {
            diff = 1.0;
        } else {
            diff = 0.0;
        }
        if diff.abs() > prj.LatFlowTol {
            return TRUE;
        }
        if prj.Node[ju].type_ == OUTFALL || prj.Node[ju].degree == 0 {
            qOld = prj.Node[ju].oldFlowInflow;
            qNew = prj.Node[ju].inflow;
            if qOld.abs() > TINY {
                diff = (qNew / qOld) - 1.0;
            } else if qNew.abs() > TINY {
                diff = 1.0;
            } else {
                diff = 0.0;
            }
            if diff.abs() > prj.LatFlowTol {
                return TRUE;
            }
        }
    }
    FALSE
}

//=============================================================================

fn removeStorageLosses(prj: &mut Project, tStep: f64) {
//
//  Input:   tStep = routing time step (sec)
//  Output:  none
//  Purpose: adds flow rate lost from all storage nodes due to evaporation
//           & seepage in current time step to overall mass balance totals.
//
    let mut evapLoss: f64 = 0.0;
    let mut exfilLoss: f64 = 0.0;

    // --- check each storage node
    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        if prj.Node[iu].type_ == STORAGE {
            // --- update total system storage losses
            let k = prj.Node[iu].subIndex as usize;
            evapLoss += prj.Storage[k].evapLoss;
            exfilLoss += prj.Storage[k].exfilLoss;
        }
    }

    // --- add loss rates (ft3/sec) to time step's mass balance
    massbal_addNodeLosses(prj, evapLoss / tStep, exfilLoss / tStep);
}

//=============================================================================

fn removeConduitLosses(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: adds flow rate lost from all conduits due to evaporation
//           & seepage over current time step to overall mass balance.
//
    let mut k: i32;
    let mut barrels: f64;
    let mut evapLoss: f64 = 0.0;
    let mut seepLoss: f64 = 0.0;

    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        if prj.Link[iu].type_ == CONDUIT {
            // --- retrieve number of barrels
            k = prj.Link[iu].subIndex;
            let ku = k as usize;
            barrels = prj.Conduit[ku].barrels as f64;

            // --- update total conduit losses
            evapLoss += prj.Conduit[ku].evapLossRate * barrels;
            seepLoss += prj.Conduit[ku].seepLossRate * barrels;
        }
    }
    massbal_addLinkLosses(prj, evapLoss, seepLoss);
}

//=============================================================================

fn removeOutflows(prj: &mut Project, tStep: f64) {
//
//  Input:   none
//  Output:  none
//  Purpose: finds flows that leave the system and adds these to mass
//           balance totals.
//
    let mut k: i32;
    let mut isFlooded: i32 = 0;
    let mut q: f64;
    let mut w: f64;
    let mut v: f64;

    for i in 0..prj.Nobjects[NODE as usize] {
        let iu = i as usize;
        // --- accumulate inflow volume & pollut. load at outfalls
        if prj.Node[iu].type_ == OUTFALL && prj.Node[iu].inflow > 0.0 {
            k = prj.Node[iu].subIndex;
            let ku = k as usize;
            if prj.Outfall[ku].routeTo >= 0 {
                v = prj.Node[iu].inflow * tStep;
                prj.Outfall[ku].vRouted += v;
                for p in 0..prj.Nobjects[POLLUT as usize] {
                    let pu = p as usize;
                    prj.Outfall[ku].wRouted[pu] += prj.Node[iu].newQual[pu] * v;
                }
            }
        }

        // --- update mass balance with flow and mass leaving the system
        //     through outfalls and flooded interior nodes
        q = node_getSystemOutflow(prj, i, &mut isFlooded);
        if q > 0.0 {
            massbal_addOutflowFlow(prj, q, isFlooded);
            for p in 0..prj.Nobjects[POLLUT as usize] {
                w = q * prj.Node[iu].newQual[p as usize];
                massbal_addOutflowQual(prj, p, w, isFlooded);
            }
        } else {
            massbal_addInflowFlow(prj, EXTERNAL_INFLOW, -q);
        }

        // --- update mass balance with mass leaving system through negative
        //     lateral inflows (lateral flow was previously accounted for)
        q = prj.Node[iu].newLatFlow;
        if q < 0.0 {
            for p in 0..prj.Nobjects[POLLUT as usize] {
                w = -q * prj.Node[iu].newQual[p as usize];
                massbal_addOutflowQual(prj, p, w, FALSE);
            }
        }
    }
}

//=============================================================================

fn sortEvents(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: sorts the entries of the Event array in chronological order.
//
    let mut i: i32;
    let mut j: i32;
    let mut gap: i32;
    let mut temp: TEvent;
    let numEvents = prj.NumEvents;

    // Apply shell sort to event list
    gap = numEvents / 2;
    while gap >= 1 {
        i = gap;
        while i < numEvents {
            temp = prj.Event[i as usize];
            j = i - gap;
            while j >= 0 && prj.Event[j as usize].start > temp.start {
                prj.Event[(j + gap) as usize] = prj.Event[j as usize];
                j -= gap;
            }
            if j != i - gap {
                prj.Event[(j + gap) as usize] = temp;
            }
            i += gap;
        }
        gap /= 2;
    }

    // Adjust for overlapping events
    i = 0;
    while i < numEvents - 1 {
        if prj.Event[i as usize].end > prj.Event[(i + 1) as usize].start {
            prj.Event[i as usize].end = prj.Event[(i + 1) as usize].start;
        }
        i += 1;
    }
}

//=============================================================================
