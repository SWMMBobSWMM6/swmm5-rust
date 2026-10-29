//! project.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   project.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     07/17/23  (Build 5.2.4)
//   Author:   L. Rossman
//
//   Project management functions.
//
//   This module provides project-related services such as:
//   o opening a new project and reading its input data
//   o allocating and freeing memory for project objects
//   o setting default values for object properties and options
//   o initializing the internal state of all objects
//   o managing hash tables for identifying objects by ID name
//
//   Update History
//   ==============
//   Build 5.1.004:
//   - Ignore RDII option added.
//   Build 5.1.007:
//   - Default monthly adjustments for climate variables included.
//   - User-supplied GW flow equations initialized to NULL.
//   - Storage node exfiltration object initialized to NULL.
//   - Freeing of memory used for storage node exfiltration included.
//   Build 5.1.008:
//   - Constants used for dynamic wave routing moved to dynwave.c.
//   - Input processing of minimum time step & number of
//     parallel threads for dynamic wave routing added.
//   - Default values of hyd. conductivity adjustments added.
//   - Freeing of memory used for outfall pollutant load added.
//   Build 5.1.009:
//   - Fixed bug in computing total duration introduced in 5.1.008.
//   Build 5.1.011:
//   - Memory management of hydraulic event dates array added.
//   Build 5.1.012:
//   - Minimum conduit slope option initialized to 0 (none).
//   - NO/YES no longer accepted as options for NORMAL_FLOW_LIMITED.
//   Build 5.1.013:
//   - omp_get_num_threads function protected against lack of compiler
//     support for OpenMP.
//   - Rain gage validation now performed after subcatchment validation.
//   - More robust parsing of MinSurfarea option provided.
//   - Support added for new RuleStep analysis option.
//   Build 5.1.015:
//   - Support added for multiple infiltration methods within a project.
//   Build 5.2.0:
//   - Support added for Streets and Inlets.
//   - Support added for RptFlags.disabled option.
//   - Object's rptFlag changed to record its index in output file.
//   Build 5.2.2:
//   - Default number of threads changed from OMP max. number to 1
//     to be consistent with User Manual.
//   Build 5.2.4:
//   - Default Inertial Damping changed from SOME to PARTIAL_DAMPING.
//   - Default CourantFactor changed from 0 (fixed routing time step)
//   - to 0.75 (variable time step)
//-----------------------------------------------------------------------------

use crate::prelude::*;

// Protect against lack of compiler support for OpenMP
// (the reference build is compiled without OpenMP)
fn omp_get_max_threads() -> i32 {
    1
}

/// The file-scope variables of project.c.
#[derive(Default)]
pub struct ProjectState {
    /// Hash tables for object ID names (one per object type;
    /// empty until createHashTables, as C's NULL pointers)
    pub Htable: Vec<HTtable>,
    /// TRUE if memory pool allocated
    MemPoolAllocated: i8,
}

//-----------------------------------------------------------------------------
//  External Functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  project_open           (called from swmm_open in swmm5.c)
//  project_close          (called from swmm_close in swmm5.c)
//  project_readInput      (called from swmm_open in swmm5.c)
//  project_readOption     (called from readOption in input.c)
//  project_validate       (called from swmm_open in swmm5.c)
//  project_init           (called from swmm_start in swmm5.c)
//  project_addObject      (called from addObject in input.c)
//  project_createMatrix   (called from openFileForInput in iface.c)
//  project_freeMatrix     (called from iface_closeRoutingFiles)
//  project_findObject
//  project_findID

//=============================================================================

pub fn project_open(prj: &mut Project, f1: &[u8], f2: &[u8], f3: &[u8])
//
//  Input:   f1 = pointer to name of input file
//           f2 = pointer to name of report file
//           f3 = pointer to name of binary output file
//  Output:  none
//  Purpose: opens a new SWMM project.
//
{
    initPointers(prj);
    setDefaults(prj);
    openFiles(prj, f1, f2, f3);
}

//=============================================================================

pub fn project_readInput(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: retrieves project data from input file.
//
{
    // --- create hash tables for fast retrieval of objects by ID names
    createHashTables(prj);

    // --- count number of objects in input file and create them
    input_countObjects(prj);
    createObjects(prj);

    // --- read project data from input file
    input_readData(prj);
    if prj.ErrorCode != 0 {
        return;
    }

    // --- establish starting & ending date/time
    prj.StartDateTime = prj.StartDate + prj.StartTime;
    prj.EndDateTime = prj.EndDate + prj.EndTime;
    prj.ReportStart = prj.ReportStartDate + prj.ReportStartTime;
    prj.ReportStart = MAX(prj.ReportStart, prj.StartDateTime);

    // --- check for valid starting & ending date/times
    if prj.EndDateTime <= prj.StartDateTime {
        report_writeErrorMsg(prj, ERR_START_DATE, b"");
    } else if prj.EndDateTime <= prj.ReportStart {
        report_writeErrorMsg(prj, ERR_REPORT_DATE, b"");
    } else {
        // --- compute total duration of simulation in seconds
        prj.TotalDuration = ((prj.EndDateTime - prj.StartDateTime) * SECperDAY as f64).floor();

        // --- reporting step must be <= total duration
        if prj.ReportStep as f64 > prj.TotalDuration {
            prj.ReportStep = f64_to_i32(prj.TotalDuration);
        }

        // --- reporting step can't be < routing step
        if (prj.ReportStep as f64) < prj.RouteStep {
            report_writeErrorMsg(prj, ERR_REPORT_STEP, b"");
        }

        // --- convert total duration to milliseconds
        prj.TotalDuration *= 1000.0;
    }
}

//=============================================================================

pub fn project_validate(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: checks validity of project data.
//
{
    let mut i: i32;
    let mut j: i32;
    let mut err: i32;

    // --- validate Curves and TimeSeries
    i = 0;
    while i < prj.Nobjects[CURVE as usize] {
        err = table_validate(&mut prj.Curve[i as usize]);
        if err != 0 {
            let id = prj.Curve[i as usize].ID.clone();
            report_writeErrorMsg(prj, ERR_CURVE_SEQUENCE, &id);
        }
        i += 1;
    }
    i = 0;
    while i < prj.Nobjects[TSERIES as usize] {
        err = table_validate(&mut prj.Tseries[i as usize]);
        if err != 0 {
            report_writeTseriesErrorMsg(prj, err, i);
        }
        i += 1;
    }

    // --- validate hydrology objects
    //     (NOTE: order is important !!!!)
    climate_validate(prj);
    lid_validate(prj);
    if prj.Nobjects[SNOWMELT as usize] == 0 {
        prj.IgnoreSnowmelt = TRUE;
    }
    if prj.Nobjects[AQUIFER as usize] == 0 {
        prj.IgnoreGwater = TRUE;
    }
    i = 0;
    while i < prj.Nobjects[AQUIFER as usize] {
        gwater_validateAquifer(prj, i);
        i += 1;
    }
    i = 0;
    while i < prj.Nobjects[SUBCATCH as usize] {
        subcatch_validate(prj, i);
        i += 1;
    }
    i = 0;
    while i < prj.Nobjects[GAGE as usize] {
        gage_validate(prj, i);
        i += 1;
    }
    i = 0;
    while i < prj.Nobjects[SNOWMELT as usize] {
        snow_validateSnowmelt(prj, i);
        i += 1;
    }

    // --- compute geometry tables for each shape curve
    j = 0;
    i = 0;
    while i < prj.Nobjects[CURVE as usize] {
        if prj.Curve[i as usize].curveType == SHAPE_CURVE {
            prj.Curve[i as usize].refersTo = j;
            prj.Shape[j as usize].curve = i;
            if shape_validate(&mut prj.Shape[j as usize], &mut prj.Curve[i as usize]) == 0 {
                let id = prj.Curve[i as usize].ID.clone();
                report_writeErrorMsg(prj, ERR_CURVE_SEQUENCE, &id);
            }
            j += 1;
        }
        i += 1;
    }

    // --- validate links before nodes, since the latter can
    //     result in adjustment of node depths
    i = 0;
    while i < prj.Nobjects[NODE as usize] {
        prj.Node[i as usize].oldDepth = prj.Node[i as usize].fullDepth;
        i += 1;
    }
    i = 0;
    while i < prj.Nobjects[LINK as usize] {
        link_validate(prj, i);
        i += 1;
    }
    i = 0;
    while i < prj.Nobjects[NODE as usize] {
        node_validate(prj, i);
        i += 1;
    }

    // --- adjust time steps if necessary
    if prj.DryStep < prj.WetStep {
        report_writeWarningMsg(prj, WARN06, b"");
        prj.DryStep = prj.WetStep;
    }
    if prj.RouteStep > prj.WetStep as f64 {
        report_writeWarningMsg(prj, WARN07, b"");
        prj.RouteStep = prj.WetStep as f64;
    }

    // --- adjust individual reporting flags to match global reporting flag
    if prj.RptFlags.subcatchments as i32 == ALL {
        i = 0;
        while i < prj.Nobjects[SUBCATCH as usize] {
            prj.Subcatch[i as usize].rptFlag = 1;
            i += 1;
        }
    }
    if prj.RptFlags.nodes as i32 == ALL {
        i = 0;
        while i < prj.Nobjects[NODE as usize] {
            prj.Node[i as usize].rptFlag = 1;
            i += 1;
        }
    }
    if prj.RptFlags.links as i32 == ALL {
        i = 0;
        while i < prj.Nobjects[LINK as usize] {
            prj.Link[i as usize].rptFlag = 1;
            i += 1;
        }
    }

    // --- validate dynamic wave options
    if prj.RouteModel == DW {
        dynwave_validate(prj);
    }

    // --- validate street/channel inlets
    inlet_validate(prj);

    // --- adjust number of parallel threads to be used
    if prj.NumThreads == 0 {
        prj.NumThreads = omp_get_max_threads();
    } else {
        prj.NumThreads = MIN(prj.NumThreads, omp_get_max_threads());
    }
    if prj.Nobjects[LINK as usize] < 4 * prj.NumThreads {
        prj.NumThreads = 1;
    }
}

//=============================================================================

pub fn project_close(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: closes a SWMM project.
//
{
    deleteObjects(prj);
    deleteHashTables(prj);
}

//=============================================================================

pub fn project_init(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: initializes the internal state of all objects.
//
{
    let mut j: i32;
    let mut k: i32;
    climate_initState(prj);
    lid_initState(prj);
    j = 0;
    while j < prj.Nobjects[TSERIES as usize] {
        table_tseriesInit(&mut prj.Tseries[j as usize]);
        j += 1;
    }
    j = 0;
    while j < prj.Nobjects[GAGE as usize] {
        gage_initState(prj, j);
        j += 1;
    }
    k = 1;
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        subcatch_initState(prj, j);
        if prj.Subcatch[j as usize].rptFlag > 0 {
            prj.Subcatch[j as usize].rptFlag = k;
            k += 1;
        }
        j += 1;
    }
    k = 1;
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        node_initState(prj, j);
        if prj.Node[j as usize].rptFlag > 0 {
            prj.Node[j as usize].rptFlag = k;
            k += 1;
        }
        j += 1;
    }
    k = 1;
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        link_initState(prj, j);
        if prj.Link[j as usize].rptFlag > 0 {
            prj.Link[j as usize].rptFlag = k;
            k += 1;
        }
        j += 1;
    }
    prj.ErrorCode
}

//=============================================================================

pub fn project_addObject(prj: &mut Project, type_: i32, id: &[u8], n: i32) -> i32
//
//  Input:   type = object type
//           id   = object ID string
//           n    = object index
//  Output:  returns 0 if object already added, 1 if not, -1 if hashing fails
//  Purpose: adds an object ID to a hash table
//
{
    let mut result: i32;
    let len: i64;

    // --- do nothing if object already placed in hash table
    if project_findObject(prj, type_, id) >= 0 {
        return 0;
    }

    // --- use memory from the hash tables' common memory pool to store
    //     a copy of the object's ID string
    //     (here: an owned copy of the ID's bytes)
    len = cio::strlen(id) as i64;
    let newID: Vec<u8> = id[..len as usize].to_vec();

    // --- insert object's ID into the hash table for that type of object
    result = HTinsert(&mut prj.project.Htable[type_ as usize], &newID, n);
    if result == 0 {
        result = -1;
    }
    result
}

//=============================================================================

pub fn project_findObject(prj: &Project, type_: i32, id: &[u8]) -> i32
//
//  Input:   type = object type
//           id   = object ID
//  Output:  returns index of object with given ID, or -1 if ID not found
//  Purpose: uses hash table to find index of an object with a given ID.
//
{
    HTfind(&prj.project.Htable[type_ as usize], id)
}

//=============================================================================

pub fn project_findID(prj: &Project, type_: i32, id: &[u8]) -> Option<Vec<u8>>
//
//  Input:   type = object type
//           id   = ID name being sought
//  Output:  returns pointer to location where object's ID string is stored
//           (here: a copy of the stored ID)
//  Purpose: uses hash table to find address of given string entry.
//
{
    HTfindKey(&prj.project.Htable[type_ as usize], id)
}

//=============================================================================

pub fn project_createMatrix(nrows: i32, ncols: i32) -> Vec<Vec<f64>>
//
//  Input:   nrows = number of rows (0-based)
//           ncols = number of columns (0-based)
//  Output:  returns a pointer to a matrix
//  Purpose: allocates memory for a matrix of doubles.
//
//  (C returns NULL for nrows < 1 or ncols < 1; here an empty matrix.
//  C's memset clears only nrows*ncols bytes, leaving the rest of the
//  malloc'd doubles uninitialized; here every element is 0.0.)
{
    // --- allocate pointers to rows
    if nrows < 1 || ncols < 1 {
        return Vec::new();
    }

    // --- allocate rows and fill matrix with zeroes
    let a: Vec<Vec<f64>> = (0..nrows).map(|_| vec![0.0f64; ncols as usize]).collect();

    // --- return pointer to array of pointers to rows
    a
}

//=============================================================================

pub fn project_freeMatrix(a: Vec<Vec<f64>>)
//
//  Input:   a = matrix of floats
//  Output:  none
//  Purpose: frees memory allocated for a matrix of doubles.
//
{
    drop(a);
}

//=============================================================================

pub fn project_readOption(prj: &mut Project, s1: &[u8], s2: &[u8]) -> i32
//
//  Input:   s1 = option keyword
//           s2 = string representation of option's value
//  Output:  returns error code
//  Purpose: reads a project option from a pair of string tokens.
//
//  NOTE:    all project options have default values assigned in setDefaults().
//
{
    let k: i32;
    let mut m: i32 = 0;
    let mut h: i32 = 0;
    let mut s: i32 = 0;
    let mut tStep: f64 = 0.0;
    // (C: char strDate[25]. A 24-character SWEEP date makes sstrcat write
    // "/1947" past the array, into the padding of its 32-byte stack slot;
    // the larger buffer here reproduces the string C then parses.)
    let mut strDate = [0u8; 32];
    let mut aTime: DateTime = 0.0;
    let mut aDate: DateTime = 0.0;

    // --- determine which option is being read
    k = findmatch(s1, OptionWords);
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, s1);
    }
    match k {
        // --- choice of flow units
        FLOW_UNITS => {
            m = findmatch(s2, FlowUnitWords);
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
            prj.FlowUnits = m;
            if prj.FlowUnits <= MGD {
                prj.UnitSystem = US;
            } else {
                prj.UnitSystem = SI;
            }
        }

        // --- choice of infiltration modeling method
        INFIL_MODEL => {
            m = findmatch(s2, InfilModelWords);
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
            prj.InfilModel = m;
        }

        // --- choice of flow routing method
        ROUTE_MODEL => {
            m = findmatch(s2, RouteModelWords);
            if m < 0 {
                m = findmatch(s2, OldRouteModelWords);
            }
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
            if m == NO_ROUTING {
                prj.IgnoreRouting = TRUE;
            } else {
                prj.RouteModel = m;
            }
            if prj.RouteModel == EKW {
                prj.RouteModel = KW;
            }
        }

        // --- simulation start date
        START_DATE => {
            let mut d = prj.StartDate;
            let ok = datetime_strToDate(prj, s2, &mut d);
            prj.StartDate = d;
            if ok == 0 {
                return error_setInpError(prj, ERR_DATETIME, s2);
            }
        }

        // --- simulation start time of day
        START_TIME => {
            if datetime_strToTime(s2, &mut prj.StartTime) == 0 {
                return error_setInpError(prj, ERR_DATETIME, s2);
            }
        }

        // --- simulation ending date
        END_DATE => {
            let mut d = prj.EndDate;
            let ok = datetime_strToDate(prj, s2, &mut d);
            prj.EndDate = d;
            if ok == 0 {
                return error_setInpError(prj, ERR_DATETIME, s2);
            }
        }

        // --- simulation ending time of day
        END_TIME => {
            if datetime_strToTime(s2, &mut prj.EndTime) == 0 {
                return error_setInpError(prj, ERR_DATETIME, s2);
            }
        }

        // --- reporting start date
        REPORT_START_DATE => {
            let mut d = prj.ReportStartDate;
            let ok = datetime_strToDate(prj, s2, &mut d);
            prj.ReportStartDate = d;
            if ok == 0 {
                return error_setInpError(prj, ERR_DATETIME, s2);
            }
        }

        // --- reporting start time of day
        REPORT_START_TIME => {
            if datetime_strToTime(s2, &mut prj.ReportStartTime) == 0 {
                return error_setInpError(prj, ERR_DATETIME, s2);
            }
        }

        // --- day of year when street sweeping begins or when it ends
        //     (year is arbitrarily set to 1947 so that the dayOfYear
        //      function can be applied)
        SWEEP_START | SWEEP_END => {
            sstrncpy(&mut strDate, s2, 24);
            sstrcat(&mut strDate, b"/1947", 25);
            if datetime_strToDate(prj, &strDate, &mut aDate) == 0 {
                return error_setInpError(prj, ERR_DATETIME, s2);
            }
            m = datetime_dayOfYear(aDate);
            if k == SWEEP_START {
                prj.SweepStart = m;
            } else {
                prj.SweepEnd = m;
            }
        }

        // --- number of antecedent dry days
        START_DRY_DAYS => {
            prj.StartDryDays = cio::atof(s2);
            if prj.StartDryDays < 0.0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
        }

        // --- runoff or reporting time steps
        //     (input is in hrs:min:sec format, time step saved as seconds)
        WET_STEP | DRY_STEP | REPORT_STEP | RULE_STEP => {
            if datetime_strToTime(s2, &mut aTime) == 0 {
                return error_setInpError(prj, ERR_DATETIME, s2);
            }
            datetime_decodeTime(aTime, &mut h, &mut m, &mut s);
            h = h.wrapping_add(24i32.wrapping_mul(f64_to_i32(aTime)));
            s = s.wrapping_add(60i32.wrapping_mul(m)).wrapping_add(3600i32.wrapping_mul(h));

            // --- RuleStep allowed to be 0 while other time steps must be > 0
            if k == RULE_STEP {
                if s < 0 {
                    return error_setInpError(prj, ERR_NUMBER, s2);
                }
            } else if s <= 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }

            match k {
                WET_STEP => prj.WetStep = s,
                DRY_STEP => prj.DryStep = s,
                REPORT_STEP => prj.ReportStep = s,
                RULE_STEP => prj.RuleStep = s,
                _ => {}
            }
        }

        // --- type of damping applied to inertial terms of dynamic wave routing
        INERT_DAMPING => {
            m = findmatch(s2, InertDampingWords);
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            } else {
                prj.InertDamping = m;
            }
        }

        // --- Yes/No options (NO = 0, YES = 1)
        ALLOW_PONDING | SLOPE_WEIGHTING | SKIP_STEADY_STATE | IGNORE_RAINFALL
        | IGNORE_SNOWMELT | IGNORE_GWATER | IGNORE_ROUTING | IGNORE_QUALITY
        | IGNORE_RDII => {
            m = findmatch(s2, NoYesWords);
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
            match k {
                ALLOW_PONDING => prj.AllowPonding = m,
                SLOPE_WEIGHTING => prj.SlopeWeighting = m,
                SKIP_STEADY_STATE => prj.SkipSteadyState = m,
                IGNORE_RAINFALL => prj.IgnoreRainfall = m,
                IGNORE_SNOWMELT => prj.IgnoreSnowmelt = m,
                IGNORE_GWATER => prj.IgnoreGwater = m,
                IGNORE_ROUTING => prj.IgnoreRouting = m,
                IGNORE_QUALITY => prj.IgnoreQuality = m,
                IGNORE_RDII => prj.IgnoreRDII = m,
                _ => {}
            }
        }

        NORMAL_FLOW_LTD => {
            m = findmatch(s2, NormalFlowWords);
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
            prj.NormalFlowLtd = m;
        }

        FORCE_MAIN_EQN => {
            m = findmatch(s2, ForceMainEqnWords);
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
            prj.ForceMainEqn = m;
        }

        LINK_OFFSETS => {
            m = findmatch(s2, LinkOffsetWords);
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
            prj.LinkOffsets = m;
        }

        // --- compatibility option for selecting solution method for
        //     dynamic wave flow routing (NOT CURRENTLY USED)
        COMPATIBILITY => {
            if strcomp(s2, b"3") != 0 {
                prj.Compatibility = SWMM3;
            } else if strcomp(s2, b"4") != 0 {
                prj.Compatibility = SWMM4;
            } else if strcomp(s2, b"5") != 0 {
                prj.Compatibility = SWMM5;
            } else {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
        }

        // --- routing or lengthening time step (in decimal seconds)
        //     (lengthening time step is used in Courant stability formula
        //     to artificially lengthen conduits for dynamic wave flow routing
        //     (a value of 0 means that no lengthening is used))
        ROUTE_STEP | LENGTHENING_STEP => {
            if getDouble(s2, &mut tStep) == 0 {
                if datetime_strToTime(s2, &mut aTime) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, s2);
                } else {
                    datetime_decodeTime(aTime, &mut h, &mut m, &mut s);
                    h = h.wrapping_add(24i32.wrapping_mul(f64_to_i32(aTime)));
                    s = s.wrapping_add(60i32.wrapping_mul(m)).wrapping_add(3600i32.wrapping_mul(h));
                    tStep = s as f64;
                }
            }
            if k == ROUTE_STEP {
                if tStep <= 0.0 {
                    return error_setInpError(prj, ERR_NUMBER, s2);
                }
                prj.RouteStep = tStep;
            } else {
                prj.LengtheningStep = MAX(0.0, tStep);
            }
        }

        // --- minimum variable time step for dynamic wave routing
        MIN_ROUTE_STEP => {
            if getDouble(s2, &mut prj.MinRouteStep) == 0 || prj.MinRouteStep < 0.0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
        }

        NUM_THREADS => {
            m = cio::atoi(s2);
            if m < 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
            prj.NumThreads = m;
        }

        // --- safety factor applied to variable time step estimates under
        //     dynamic wave flow routing (value of 0 indicates that variable
        //     time step option not used)
        VARIABLE_STEP => {
            if getDouble(s2, &mut prj.CourantFactor) == 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
            if prj.CourantFactor < 0.0 || prj.CourantFactor > 2.0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
        }

        // --- minimum surface area (ft2 or sq. meters) associated with nodes
        //     under dynamic wave flow routing
        MIN_SURFAREA => {
            if getDouble(s2, &mut prj.MinSurfArea) == 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
            if prj.MinSurfArea < 0.0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
        }

        // --- minimum conduit slope (%)
        MIN_SLOPE => {
            if getDouble(s2, &mut prj.MinSlope) == 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
            if prj.MinSlope < 0.0 || prj.MinSlope >= 100.0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
            prj.MinSlope /= 100.0;
        }

        // --- maximum trials / time step for dynamic wave routing
        MAX_TRIALS => {
            m = cio::atoi(s2);
            if m < 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
            prj.MaxTrials = m;
        }

        // --- head convergence tolerance for dynamic wave routing
        HEAD_TOL => {
            if getDouble(s2, &mut prj.HeadTol) == 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
        }

        // --- steady state tolerance on system inflow - outflow
        SYS_FLOW_TOL => {
            if getDouble(s2, &mut prj.SysFlowTol) == 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
            prj.SysFlowTol /= 100.0;
        }

        // --- steady state tolerance on nodal lateral inflow
        LAT_FLOW_TOL => {
            if getDouble(s2, &mut prj.LatFlowTol) == 0 {
                return error_setInpError(prj, ERR_NUMBER, s2);
            }
            prj.LatFlowTol /= 100.0;
        }

        // --- method used for surcharging in dynamic wave flow routing
        SURCHARGE_METHOD => {
            m = findmatch(s2, SurchargeWords);
            if m < 0 {
                return error_setInpError(prj, ERR_KEYWORD, s2);
            }
            prj.SurchargeMethod = m;
        }

        TEMPDIR => {
            // Temporary Directory
            sstrncpy(&mut prj.TempDir, s2, MAXFNAME as usize);
        }

        _ => {}
    }
    0
}

//=============================================================================

fn initPointers(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: assigns NULL to all dynamic arrays for a new project.
//
{
    prj.Gage = Vec::new();
    prj.Subcatch = Vec::new();
    prj.Node = Vec::new();
    prj.Outfall = Vec::new();
    prj.Divider = Vec::new();
    prj.Storage = Vec::new();
    prj.Link = Vec::new();
    prj.Conduit = Vec::new();
    prj.Pump = Vec::new();
    prj.Orifice = Vec::new();
    prj.Weir = Vec::new();
    prj.Outlet = Vec::new();
    prj.Pollut = Vec::new();
    prj.Landuse = Vec::new();
    prj.Pattern = Vec::new();
    prj.Curve = Vec::new();
    prj.Tseries = Vec::new();
    prj.Transect = Vec::new();
    prj.Shape = Vec::new();
    prj.Aquifer = Vec::new();
    prj.UnitHyd = Vec::new();
    prj.Snowmelt = Vec::new();
    prj.Event = Vec::new();
    prj.project.MemPoolAllocated = FALSE as i8;
}

//=============================================================================

fn setDefaults(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: assigns default values to project variables.
//
{
    let mut i: usize;
    let mut j: usize;

    // Project title & temp. file path
    i = 0;
    while i < MAXTITLE as usize {
        sstrncpy(&mut prj.Title[i], b"", 0);
        i += 1;
    }
    sstrncpy(&mut prj.TempDir, b"", 0);

    // Interface files
    prj.Frain.mode = SCRATCH_FILE as i8; // Use scratch rainfall file
    prj.Fclimate.mode = NO_FILE as i8;
    prj.Frunoff.mode = NO_FILE as i8;
    prj.Frdii.mode = NO_FILE as i8;
    prj.Fhotstart1.mode = NO_FILE as i8;
    prj.Fhotstart2.mode = NO_FILE as i8;
    prj.Finflows.mode = NO_FILE as i8;
    prj.Foutflows.mode = NO_FILE as i8;
    prj.Frain.file = None;
    prj.Fclimate.file = None;
    prj.Frunoff.file = None;
    prj.Frdii.file = None;
    prj.Fhotstart1.file = None;
    prj.Fhotstart2.file = None;
    prj.Finflows.file = None;
    prj.Foutflows.file = None;
    prj.Fout.file = None;
    prj.Fout.mode = NO_FILE as i8;

    // Analysis options
    prj.UnitSystem = US; // US unit system
    prj.FlowUnits = CFS; // CFS flow units
    prj.InfilModel = HORTON; // Horton infiltration method
    prj.RouteModel = DW; // Dynamic wave flow routing method
    prj.SurchargeMethod = EXTRAN; // Use EXTRAN method for surcharging
    prj.CrownCutoff = 0.96; // Fractional pipe crown cutoff
    prj.AllowPonding = FALSE; // No ponding at nodes
    prj.InertDamping = PARTIAL_DAMPING; // Partial inertial damping
    prj.NormalFlowLtd = BOTH; // Default normal flow limitation
    prj.ForceMainEqn = H_W; // Hazen-Williams eqn. for force mains
    prj.LinkOffsets = DEPTH_OFFSET; // Use depth for link offsets
    prj.LengtheningStep = 0.0; // No lengthening of conduits
    prj.CourantFactor = 0.75; // Variable time step reduced to 75%
    prj.MinSurfArea = 0.0; // Force use of default min. surface area
    prj.MinSlope = 0.0; // No user supplied minimum conduit slope
    prj.SkipSteadyState = FALSE; // Do flow routing in steady state periods
    prj.IgnoreRainfall = FALSE; // Analyze rainfall/runoff
    prj.IgnoreRDII = FALSE; // Analyze RDII
    prj.IgnoreSnowmelt = FALSE; // Analyze snowmelt
    prj.IgnoreGwater = FALSE; // Analyze groundwater
    prj.IgnoreRouting = FALSE; // Analyze flow routing
    prj.IgnoreQuality = FALSE; // Analyze water quality
    prj.WetStep = 300; // Runoff wet time step (secs)
    prj.DryStep = 3600; // Runoff dry time step (secs)
    prj.RuleStep = 0; // Rules evaluated at each routing step
    prj.RouteStep = 20.0; // Routing time step (secs)
    prj.MinRouteStep = 0.5; // Minimum variable time step (sec)
    prj.ReportStep = 900; // Reporting time step (secs)
    prj.StartDryDays = 0.0; // Antecedent dry days
    prj.MaxTrials = 0; // Force use of default max. trials
    prj.HeadTol = 0.0; // Force use of default head tolerance
    prj.SysFlowTol = 0.05; // System flow tolerance for steady state
    prj.LatFlowTol = 0.05; // Lateral flow tolerance for steady state
    prj.NumThreads = 1; // Number of parallel threads to use
    prj.NumEvents = 0; // Number of detailed routing events

    // Deprecated options
    prj.SlopeWeighting = TRUE; // Use slope weighting
    prj.Compatibility = SWMM4; // Use SWMM 4 up/dn weighting method

    // Starting & ending date/time
    prj.StartDate = datetime_encodeDate(2004, 1, 1);
    prj.StartTime = datetime_encodeTime(0, 0, 0.0);
    prj.StartDateTime = prj.StartDate + prj.StartTime;
    prj.EndDate = prj.StartDate;
    prj.EndTime = 0.0;
    prj.ReportStartDate = NO_DATE as f64;
    prj.ReportStartTime = NO_DATE as f64;
    prj.SweepStart = 1;
    prj.SweepEnd = 365;

    // Reporting options
    prj.RptFlags.disabled = FALSE as i8;
    prj.RptFlags.input = FALSE as i8;
    prj.RptFlags.continuity = TRUE as i8;
    prj.RptFlags.flowStats = TRUE as i8;
    prj.RptFlags.controls = FALSE as i8;
    prj.RptFlags.subcatchments = FALSE as i8;
    prj.RptFlags.nodes = FALSE as i8;
    prj.RptFlags.links = FALSE as i8;
    prj.RptFlags.averages = FALSE as i8;

    // Temperature data
    prj.Temp.dataSource = NO_TEMP;
    prj.Temp.tSeries = -1;
    prj.Temp.ta = 70.0;
    prj.Temp.elev = 0.0;
    prj.Temp.anglat = 40.0;
    prj.Temp.dtlong = 0.0;
    prj.Temp.tmax = MISSING;

    // Wind speed data
    prj.Wind.type_ = MONTHLY_WIND;
    i = 0;
    while i < 12 {
        prj.Wind.aws[i] = 0.0;
        i += 1;
    }

    // Snowmelt parameters
    prj.Snow.snotmp = 34.0;
    prj.Snow.tipm = 0.5;
    prj.Snow.rnm = 0.6;

    // Snow areal depletion curves for pervious and impervious surfaces
    i = 0;
    while i < 2 {
        j = 0;
        while j < 10 {
            prj.Snow.adc[i][j] = 1.0;
            j += 1;
        }
        i += 1;
    }

    // Evaporation rates
    prj.Evap.type_ = CONSTANT_EVAP;
    i = 0;
    while i < 12 {
        prj.Evap.monthlyEvap[i] = 0.0;
        prj.Evap.panCoeff[i] = 1.0;
        i += 1;
    }
    prj.Evap.recoveryPattern = -1;
    prj.Evap.recoveryFactor = 1.0;
    prj.Evap.tSeries = -1;
    prj.Evap.dryOnly = FALSE;

    // Climate adjustments
    i = 0;
    while i < 12 {
        prj.Adjust.temp[i] = 0.0; // additive adjustments
        prj.Adjust.evap[i] = 0.0; // additive adjustments
        prj.Adjust.rain[i] = 1.0; // multiplicative adjustments
        prj.Adjust.hydcon[i] = 1.0; // hyd. conductivity adjustments
        i += 1;
    }
    prj.Adjust.rainFactor = 1.0;
    prj.Adjust.hydconFactor = 1.0;
}

//=============================================================================

fn openFiles(prj: &mut Project, f1: &[u8], f2: &[u8], f3: &[u8])
//
//  Input:   f1 = name of input file
//           f2 = name of report file
//           f3 = name of binary output file
//  Output:  none
//  Purpose: opens a project's input and report files.
//
{
    // --- initialize file pointers to NULL
    prj.Finp.file = None;
    prj.Frpt.file = None;
    prj.Fout.file = None;

    // --- save file names
    sstrncpy(&mut prj.Finp.name, f1, MAXFNAME as usize);
    sstrncpy(&mut prj.Frpt.name, f2, MAXFNAME as usize);
    sstrncpy(&mut prj.Fout.name, f3, MAXFNAME as usize);

    // --- check that file names are not identical
    if strcomp(f1, f2) != 0 || strcomp(f1, f3) != 0 || strcomp(f2, f3) != 0 {
        writecon(prj, FMT11);
        prj.ErrorCode = ERR_FILE_NAME;
        return;
    }

    // --- open input and report files
    prj.Finp.file = CFile::open(f1, "rt");
    if prj.Finp.file.is_none() {
        writecon(prj, FMT12);
        writecon(prj, f1);
        prj.ErrorCode = ERR_INP_FILE;
        return;
    }
    prj.Frpt.file = CFile::open(f2, "wt");
    if prj.Frpt.file.is_none() {
        writecon(prj, FMT13);
        prj.ErrorCode = ERR_RPT_FILE;
        return;
    }
}

//=============================================================================

fn createObjects(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: allocates memory for project's objects.
//
//  NOTE: number of each type of object has already been determined in
//        project_readInput().
//
{
    let mut j: usize;
    let mut k: usize;

    // --- allocate memory for each category of object
    if prj.ErrorCode != 0 {
        return;
    }
    let n = |c: i32| -> usize { if c > 0 { c as usize } else { 0 } };
    prj.Gage = vec![TGage::default(); n(prj.Nobjects[GAGE as usize])];
    prj.Subcatch = vec![TSubcatch::default(); n(prj.Nobjects[SUBCATCH as usize])];
    prj.Node = vec![TNode::default(); n(prj.Nobjects[NODE as usize])];
    prj.Outfall = vec![TOutfall::default(); n(prj.Nnodes[OUTFALL as usize])];
    prj.Divider = vec![TDivider::default(); n(prj.Nnodes[DIVIDER as usize])];
    prj.Storage = vec![TStorage::default(); n(prj.Nnodes[STORAGE as usize])];
    prj.Link = vec![TLink::default(); n(prj.Nobjects[LINK as usize])];
    prj.Conduit = vec![TConduit::default(); n(prj.Nlinks[CONDUIT as usize])];
    prj.Pump = vec![TPump::default(); n(prj.Nlinks[PUMP as usize])];
    prj.Orifice = vec![TOrifice::default(); n(prj.Nlinks[ORIFICE as usize])];
    prj.Weir = vec![TWeir::default(); n(prj.Nlinks[WEIR as usize])];
    prj.Outlet = vec![TOutlet::default(); n(prj.Nlinks[OUTLET as usize])];
    prj.Pollut = vec![TPollut::default(); n(prj.Nobjects[POLLUT as usize])];
    prj.Landuse = vec![TLanduse::default(); n(prj.Nobjects[LANDUSE as usize])];
    prj.Pattern = vec![TPattern::default(); n(prj.Nobjects[TIMEPATTERN as usize])];
    prj.Curve = (0..n(prj.Nobjects[CURVE as usize])).map(|_| TTable::default()).collect();
    prj.Tseries = (0..n(prj.Nobjects[TSERIES as usize])).map(|_| TTable::default()).collect();
    prj.Aquifer = vec![TAquifer::default(); n(prj.Nobjects[AQUIFER as usize])];
    prj.UnitHyd = vec![TUnitHyd::default(); n(prj.Nobjects[UNITHYD as usize])];
    prj.Snowmelt = vec![TSnowmelt::default(); n(prj.Nobjects[SNOWMELT as usize])];
    prj.Shape = vec![TShape::default(); n(prj.Nobjects[SHAPE as usize])];

    // --- create array of detailed routing event periods
    prj.Event = vec![TEvent::default(); prj.NumEvents as usize + 1];
    let ne = prj.NumEvents as usize;
    prj.Event[ne].start = BIG;
    prj.Event[ne].end = BIG + 1.0;

    // --- create LID objects
    let (nlid, nsub) = (prj.Nobjects[LID as usize], prj.Nobjects[SUBCATCH as usize]);
    lid_create(prj, nlid, nsub);

    // --- create control rules
    let nc = prj.Nobjects[CONTROL as usize];
    prj.ErrorCode = controls_create(prj, nc);
    if prj.ErrorCode != 0 {
        return;
    }

    // --- create cross section transects
    let nt = prj.Nobjects[TRANSECT as usize];
    prj.ErrorCode = transect_create(prj, nt);
    if prj.ErrorCode != 0 {
        return;
    }

    // --- create street cross sections & inlet designs
    let ns = prj.Nobjects[STREET as usize];
    prj.ErrorCode = street_create(prj, ns);
    if prj.ErrorCode != 0 {
        return;
    }
    let ni = prj.Nobjects[INLET as usize];
    prj.ErrorCode = inlet_create(prj, ni);
    if prj.ErrorCode != 0 {
        return;
    }

    // --- allocate memory for infiltration data
    let nsub = prj.Nobjects[SUBCATCH as usize];
    infil_create(prj, nsub);

    // --- allocate memory for water quality state variables
    let npollut = n(prj.Nobjects[POLLUT as usize]);
    let nlanduse = n(prj.Nobjects[LANDUSE as usize]);
    j = 0;
    while j < n(prj.Nobjects[SUBCATCH as usize]) {
        let sc = &mut prj.Subcatch[j];
        sc.initBuildup = vec![0.0; npollut];
        sc.oldQual = vec![0.0; npollut];
        sc.newQual = vec![0.0; npollut];
        sc.pondedQual = vec![0.0; npollut];
        sc.totalLoad = vec![0.0; npollut];
        j += 1;
    }
    j = 0;
    while j < n(prj.Nobjects[NODE as usize]) {
        let nd = &mut prj.Node[j];
        nd.oldQual = vec![0.0; npollut];
        nd.newQual = vec![0.0; npollut];
        nd.extInflow = Vec::new();
        nd.dwfInflow = Vec::new();
        nd.rdiiInflow = None;
        nd.treatment = Vec::new();
        j += 1;
    }
    j = 0;
    while j < n(prj.Nobjects[LINK as usize]) {
        let lk = &mut prj.Link[j];
        lk.inlet = None;
        lk.oldQual = vec![0.0; npollut];
        lk.newQual = vec![0.0; npollut];
        lk.totalLoad = vec![0.0; npollut];
        j += 1;
    }

    // --- allocate memory for land use buildup/washoff functions
    j = 0;
    while j < nlanduse {
        prj.Landuse[j].buildupFunc = vec![TBuildup::default(); npollut];
        prj.Landuse[j].washoffFunc = vec![TWashoff::default(); npollut];
        j += 1;
    }

    // --- allocate memory for subcatchment landuse factors
    j = 0;
    while j < n(prj.Nobjects[SUBCATCH as usize]) {
        prj.Subcatch[j].landFactor = vec![TLandFactor::default(); nlanduse];
        k = 0;
        while k < nlanduse {
            prj.Subcatch[j].landFactor[k].buildup = vec![0.0; npollut];
            k += 1;
        }
        j += 1;
    }

    // --- initialize buildup & washoff functions
    j = 0;
    while j < nlanduse {
        k = 0;
        while k < npollut {
            prj.Landuse[j].buildupFunc[k].funcType = NO_BUILDUP;
            prj.Landuse[j].buildupFunc[k].normalizer = PER_AREA;
            prj.Landuse[j].washoffFunc[k].funcType = NO_WASHOFF;
            k += 1;
        }
        j += 1;
    }

    // --- initialize rain gage properties
    j = 0;
    while j < n(prj.Nobjects[GAGE as usize]) {
        prj.Gage[j].tSeries = -1;
        sstrncpy(&mut prj.Gage[j].fname, b"", 0);
        j += 1;
    }

    // --- initialize subcatchment properties
    j = 0;
    while j < n(prj.Nobjects[SUBCATCH as usize]) {
        let sc = &mut prj.Subcatch[j];
        sc.outSubcatch = -1;
        sc.outNode = -1;
        sc.infil = -1;
        sc.groundwater = None;
        sc.gwLatFlowExpr = None;
        sc.gwDeepFlowExpr = None;
        sc.snowpack = None;
        sc.lidArea = 0.0;
        k = 0;
        while k < npollut {
            sc.initBuildup[k] = 0.0;
            k += 1;
        }
        j += 1;
    }

    // --- initialize RDII unit hydrograph properties
    j = 0;
    while j < n(prj.Nobjects[UNITHYD as usize]) {
        rdii_initUnitHyd(prj, j as i32);
        j += 1;
    }

    // --- initialize snowmelt properties
    j = 0;
    while j < n(prj.Nobjects[SNOWMELT as usize]) {
        snow_initSnowmelt(prj, j as i32);
        j += 1;
    }

    // --- initialize storage node exfiltration
    j = 0;
    while j < n(prj.Nnodes[STORAGE as usize]) {
        prj.Storage[j].exfil = None;
        j += 1;
    }

    // --- initialize link properties
    j = 0;
    while j < n(prj.Nobjects[LINK as usize]) {
        let lk = &mut prj.Link[j];
        lk.xsect.type_ = -1;
        lk.cLossInlet = 0.0;
        lk.cLossOutlet = 0.0;
        lk.cLossAvg = 0.0;
        lk.hasFlapGate = FALSE;
        j += 1;
    }
    j = 0;
    while j < n(prj.Nlinks[PUMP as usize]) {
        prj.Pump[j].pumpCurve = -1;
        j += 1;
    }

    // --- initialize reporting flags
    j = 0;
    while j < n(prj.Nobjects[SUBCATCH as usize]) {
        prj.Subcatch[j].rptFlag = FALSE;
        j += 1;
    }
    j = 0;
    while j < n(prj.Nobjects[NODE as usize]) {
        prj.Node[j].rptFlag = FALSE;
        j += 1;
    }
    j = 0;
    while j < n(prj.Nobjects[LINK as usize]) {
        prj.Link[j].rptFlag = FALSE;
        j += 1;
    }

    //  --- initialize curves, time series, and time patterns
    j = 0;
    while j < n(prj.Nobjects[CURVE as usize]) {
        table_init(&mut prj.Curve[j]);
        j += 1;
    }
    j = 0;
    while j < n(prj.Nobjects[TSERIES as usize]) {
        table_init(&mut prj.Tseries[j]);
        j += 1;
    }
    j = 0;
    while j < n(prj.Nobjects[TIMEPATTERN as usize]) {
        inflow_initDwfPattern(prj, j as i32);
        j += 1;
    }
}

//=============================================================================

fn deleteObjects(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: frees memory allocated for a project's objects.
//
//  NOTE: care is taken to first free objects that are properties of another
//        object before the latter is freed (e.g., we must free a
//        subcatchment's land use factors before freeing the subcatchment).
//
//  (C's `if ( X )` tests whether array X was allocated; an array that was
//  never allocated is empty here.)
{
    let mut j: i32;
    let mut k: i32;

    // --- free memory for landuse factors & groundwater
    if !prj.Subcatch.is_empty() {
        j = 0;
        while j < prj.Nobjects[SUBCATCH as usize] {
            k = 0;
            while k < prj.Nobjects[LANDUSE as usize] {
                // (C dereferences a NULL landFactor here if createObjects
                //  stopped early on a memory error; that case is skipped)
                if let Some(lf) = prj.Subcatch[j as usize].landFactor.get_mut(k as usize) {
                    lf.buildup = Vec::new();
                }
                k += 1;
            }
            prj.Subcatch[j as usize].landFactor = Vec::new();
            prj.Subcatch[j as usize].groundwater = None;
            gwater_deleteFlowExpression(prj, j);
            prj.Subcatch[j as usize].snowpack = None;
            j += 1;
        }
    }

    // --- free memory for buildup/washoff functions
    if !prj.Landuse.is_empty() {
        j = 0;
        while j < prj.Nobjects[LANDUSE as usize] {
            prj.Landuse[j as usize].buildupFunc = Vec::new();
            prj.Landuse[j as usize].washoffFunc = Vec::new();
            j += 1;
        }
    }

    // --- free memory for water quality state variables
    if !prj.Subcatch.is_empty() {
        j = 0;
        while j < prj.Nobjects[SUBCATCH as usize] {
            let sc = &mut prj.Subcatch[j as usize];
            sc.initBuildup = Vec::new();
            sc.oldQual = Vec::new();
            sc.newQual = Vec::new();
            sc.pondedQual = Vec::new();
            sc.totalLoad = Vec::new();
            j += 1;
        }
    }
    if !prj.Node.is_empty() {
        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            prj.Node[j as usize].oldQual = Vec::new();
            prj.Node[j as usize].newQual = Vec::new();
            j += 1;
        }
    }
    if !prj.Link.is_empty() {
        j = 0;
        while j < prj.Nobjects[LINK as usize] {
            prj.Link[j as usize].oldQual = Vec::new();
            prj.Link[j as usize].newQual = Vec::new();
            prj.Link[j as usize].totalLoad = Vec::new();
            // Any inlet assigned to Link[j].inlet is freed in inlet_delete().
            j += 1;
        }
    }

    // --- free memory used for rainfall infiltration
    infil_delete(prj);

    // --- free memory used for storage exfiltration
    if !prj.Node.is_empty() {
        j = 0;
        while j < prj.Nnodes[STORAGE as usize] {
            if let Some(ex) = prj.Storage[j as usize].exfil.as_mut() {
                ex.btmExfil = None;
                ex.bankExfil = None;
                prj.Storage[j as usize].exfil = None;
            }
            j += 1;
        }
    }

    // --- free memory used for outfall pollutants loads
    if !prj.Node.is_empty() {
        j = 0;
        while j < prj.Nnodes[OUTFALL as usize] {
            prj.Outfall[j as usize].wRouted = Vec::new();
            j += 1;
        }
    }

    // --- free memory used for nodal inflows & treatment functions
    if !prj.Node.is_empty() {
        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            inflow_deleteExtInflows(prj, j);
            inflow_deleteDwfInflows(prj, j);
            rdii_deleteRdiiInflow(prj, j);
            treatmnt_delete(prj, j);
            j += 1;
        }
    }

    // --- delete table entries for curves and time series
    if !prj.Tseries.is_empty() {
        j = 0;
        while j < prj.Nobjects[TSERIES as usize] {
            table_deleteEntries(&mut prj.Tseries[j as usize]);
            j += 1;
        }
    }
    if !prj.Curve.is_empty() {
        j = 0;
        while j < prj.Nobjects[CURVE as usize] {
            table_deleteEntries(&mut prj.Curve[j as usize]);
            j += 1;
        }
    }

    // --- delete cross section transects
    transect_delete(prj);

    // --- delete street and inlet design objects
    street_delete(prj);
    inlet_delete(prj);

    // --- delete control rules
    controls_delete(prj);

    // --- delete LIDs
    lid_delete(prj);

    // --- now free each major category of object
    prj.Gage = Vec::new();
    prj.Subcatch = Vec::new();
    prj.Node = Vec::new();
    prj.Outfall = Vec::new();
    prj.Divider = Vec::new();
    prj.Storage = Vec::new();
    prj.Link = Vec::new();
    prj.Conduit = Vec::new();
    prj.Pump = Vec::new();
    prj.Orifice = Vec::new();
    prj.Weir = Vec::new();
    prj.Outlet = Vec::new();
    prj.Pollut = Vec::new();
    prj.Landuse = Vec::new();
    prj.Pattern = Vec::new();
    prj.Curve = Vec::new();
    prj.Tseries = Vec::new();
    prj.Aquifer = Vec::new();
    prj.UnitHyd = Vec::new();
    prj.Snowmelt = Vec::new();
    prj.Shape = Vec::new();
    prj.Event = Vec::new();
}

//=============================================================================

fn createHashTables(prj: &mut Project)
//
//  Input:   none
//  Output:  returns error code
//  Purpose: allocates memory for object ID hash tables
//
{
    let mut j: i32;
    prj.project.MemPoolAllocated = FALSE as i8;
    prj.project.Htable = Vec::with_capacity(MAX_OBJ_TYPES as usize);
    j = 0;
    while j < MAX_OBJ_TYPES {
        prj.project.Htable.push(HTcreate());
        j += 1;
    }

    // --- initialize memory pool used to store object ID's
    //     (there is no pool: each ID is an owned copy)
    prj.project.MemPoolAllocated = TRUE as i8;
}

//=============================================================================

fn deleteHashTables(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: frees memory allocated for object ID hash tables
//
{
    let mut j: usize;
    j = 0;
    while j < MAX_OBJ_TYPES as usize {
        if let Some(ht) = prj.project.Htable.get_mut(j) {
            HTfree(ht);
        }
        j += 1;
    }

    // --- free object ID memory pool
    if prj.project.MemPoolAllocated != 0 {
        // (nothing to free)
    }
}
