//! inflow.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   inflow.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21 (Build 5.2.0)
//   Author:   L. Rossman
//
//   Manages any Direct External or Dry Weather Flow inflows
//   that have been assigned to nodes of the drainage system.
//
//   Update History
//   ==============
//   Build 5.2.0:
//   - Removed references to unused extIfaceInflow member of ExtInflow struct.
//-----------------------------------------------------------------------------

use crate::prelude::*;

/// The file-scope variables of inflow.c.
#[derive(Default)]
pub struct InflowState {}

//-----------------------------------------------------------------------------
//  External Functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  inflow_initDwfPattern   (called createObjects in project.c)
//  inflow_readExtInflow    (called by input_readLine)
//  inflow_readDwfInflow    (called by input_readLine)
//  inflow_deleteExtInflows (called by deleteObjects in project.c)
//  inflow_deleteDwfInflows (called by deleteObjects in project.c)
//  inflow_getExtInflow     (called by addExternalInflows in routing.c)
//  inflow_setExtInflow     (called by setNodeInflow in swmm5.c)
//  inflow_getDwfInflow     (called by addDryWeatherInflows in routing.c)

/// input.c `match()` (not exported by input.rs): true if `substr` matches
/// the start of `str` after leading blanks, ignoring case.
fn r#match(str: &[u8], substr: &[u8]) -> i32 {
    let at = |s: &[u8], i: usize| -> u8 { s.get(i).copied().unwrap_or(0) };

    // --- fail if substring is empty
    if at(substr, 0) == 0 {
        return 0;
    }

    // --- skip leading blanks of str
    let mut k = 0usize;
    while at(str, k) != 0 {
        if at(str, k) != b' ' {
            break;
        }
        k += 1;
    }

    // --- check if substr matches remainder of str
    let mut i = k;
    let mut j = 0usize;
    while at(substr, j) != 0 {
        if at(str, i) == 0 || UCHAR(at(str, i)) != UCHAR(at(substr, j)) {
            return 0;
        }
        i += 1;
        j += 1;
    }
    1
}

//=============================================================================

/// Called by: input.
pub fn inflow_readExtInflow(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error message
//  Purpose: reads parameters of a direct external inflow from a line of input.
//
//  Formats of data line are:
//     nodeID  FLOW      tSeriesID  (FLOW         1.0          scaleFactor  baseline  basePat)
//     nodeID  pollutID  tSeriesID  (CONCEN/MASS  unitsFactor  scaleFactor  baseline  basePat)
//
{
    let j: i32; // object index
    let mut param: i32; // FLOW (-1) or pollutant index
    let mut type_: i32 = CONCEN_INFLOW; // FLOW, CONCEN or MASS inflow
    let mut tseries: i32 = -1; // time series index
    let mut basePat: i32 = -1; // baseline pattern
    let mut cf: f64 = 1.0; // units conversion factor
    let mut sf: f64 = 1.0; // scaling factor
    let mut baseline: f64 = 0.0; // baseline value

    // --- find index of node receiving the inflow
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    j = project_findObject(prj, NODE, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- find index of inflow pollutant or use -1 for FLOW
    param = project_findObject(prj, POLLUT, &tok[1]);
    if param < 0 {
        if r#match(&tok[1], w_FLOW) != 0 {
            param = -1;
        } else {
            return error_setInpError(prj, ERR_NAME, &tok[1]);
        }
    }

    // --- find index of inflow time series (if supplied) in data base
    if cio::strlen(&tok[2]) > 0 {
        tseries = project_findObject(prj, TSERIES, &tok[2]);
        if tseries < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[2]);
        }
        prj.Tseries[tseries as usize].refersTo = EXTERNAL_INFLOW;
    }

    // --- assign type & cf values for a FLOW inflow
    if param == -1 {
        type_ = FLOW_INFLOW;
        cf = 1.0 / UCF(prj, FLOW);
    }

    // --- do the same for a pollutant inflow
    if ntoks >= 4 && param > -1 {
        if r#match(&tok[3], w_CONCEN) != 0 {
            type_ = CONCEN_INFLOW;
        } else if r#match(&tok[3], w_MASS) != 0 {
            type_ = MASS_INFLOW;
        } else {
            return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
        }
        if ntoks >= 5 && type_ == MASS_INFLOW {
            if getDouble(&tok[4], &mut cf) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[4]);
            }
            if cf <= 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[4]);
            }
        }
    }

    // --- get sf and baseline values
    if ntoks >= 6 {
        if getDouble(&tok[5], &mut sf) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[5]);
        }
    }
    if ntoks >= 7 {
        if getDouble(&tok[6], &mut baseline) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[6]);
        }
    }

    // --- get baseline time pattern
    if ntoks >= 8 {
        basePat = project_findObject(prj, TIMEPATTERN, &tok[7]);
        if basePat < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[7]);
        }
    }

    // --- include LperFT3 term in conversion factor for MASS_INFLOW
    if type_ == MASS_INFLOW {
        cf /= LperFT3;
    }

    inflow_setExtInflow(prj, j, param, type_, tseries, basePat, cf, baseline, sf)
}

//=============================================================================

/// Called by: swmm5 (setNodeInflow), and inflow_readExtInflow.
pub fn inflow_setExtInflow(prj: &mut Project, j: i32, param: i32, type_: i32, tseries: i32, basePat: i32, cf: f64, baseline: f64, sf: f64) -> i32
// Purpose:  This function assigns property values to the inflow object
// Inputs:   j = Node index
//           param = FLOW (-1) or pollutant index
//           type = FLOW, CONCEN or MASS inflow
//           tSeries = time series index
//           basePat = baseline pattern
//           cf = units conversion factor
//           baseline = baseline inflow value
//           sf = scaling factor
// Return:   returns Error Code
{
    let ju = j as usize;

    // --- check if an external inflow object for this constituent already exists
    let mut idx: Option<usize> = None;
    for (n, inflow) in prj.Node[ju].extInflow.iter().enumerate() {
        if inflow.param == param {
            idx = Some(n);
            break;
        }
    }

    // --- if it doesn't exist, then create it
    //     (C prepends it to the node's list; malloc leaves its fields
    //      uninitialized, but every field is assigned below)
    let n = match idx {
        Some(n) => n,
        None => {
            prj.Node[ju].extInflow.insert(0, TExtInflow::default());
            0
        }
    };

    // --- assign property values to the inflow object
    let inflow = &mut prj.Node[ju].extInflow[n];
    inflow.param = param;
    inflow.type_ = type_;
    inflow.tSeries = tseries;
    inflow.cFactor = cf;
    inflow.sFactor = sf;
    inflow.baseline = baseline;
    inflow.basePat = basePat;
    0
}

//=============================================================================

/// Called by: project.
pub fn inflow_deleteExtInflows(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: deletes all time series inflow data for a node.
//
//  (C frees the list but leaves Node[j].extInflow dangling; the node is
//   deleted next, so the Vec is simply emptied.)
{
    prj.Node[j as usize].extInflow.clear();
}

//=============================================================================

/// Called by: routing.
pub fn inflow_getExtInflow(prj: &mut Project, inflow: &TExtInflow, aDate: DateTime) -> f64
//
//  Input:   inflow = external inflow data structure
//           aDate = current simulation date/time
//  Output:  returns current value of external inflow parameter
//  Purpose: retrieves the value of an external inflow at a specific
//           date and time.
//
{
    let month: i32;
    let day: i32;
    let hour: i32;
    let p = inflow.basePat; // baseline pattern
    let k = inflow.tSeries; // time series index
    let cf = inflow.cFactor; // units conversion factor
    let sf = inflow.sFactor; // scaling factor
    let mut blv = inflow.baseline; // baseline value
    let mut tsv: f64 = 0.0; // time series value

    if p >= 0 {
        month = datetime_monthOfYear(aDate) - 1;
        day = datetime_dayOfWeek(aDate) - 1;
        hour = datetime_hourOfDay(aDate);
        blv *= getPatternFactor(prj, p, month, day, hour);
    }
    if k >= 0 {
        tsv = table_tseriesLookup(&mut prj.Tseries[k as usize], aDate, FALSE as i8) * sf;
    }
    cf * (tsv + blv)
}

//=============================================================================

/// Called by: input.
pub fn inflow_readDwfInflow(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error message
//  Purpose: reads dry weather inflow parameters from line of input data.
//
//  Format of data line is:
//    nodeID  FLOW/pollutID  avgValue  (pattern1 pattern2  ... pattern4)
//
{
    let j: i32; // node index
    let mut k: i32; // pollutant index (-1 for flow)
    let mut m: i32; // time pattern index
    let mut pats: [i32; 4] = [0; 4]; // time pattern index array
    let mut x: f64 = 0.0; // avg. DWF value

    // --- find index of node receiving the inflow
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    j = project_findObject(prj, NODE, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- find index of inflow pollutant (-1 for FLOW)
    k = project_findObject(prj, POLLUT, &tok[1]);
    if k < 0 {
        if r#match(&tok[1], w_FLOW) != 0 {
            k = -1;
        } else {
            return error_setInpError(prj, ERR_NAME, &tok[1]);
        }
    }

    // --- get avg. value of DWF inflow
    if getDouble(&tok[2], &mut x) == 0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[2]);
    }
    if k == -1 {
        x /= UCF(prj, FLOW);
    }

    // --- get time patterns assigned to the inflow
    for i in 0..4usize {
        pats[i] = -1;
    }
    for i in 3..7usize {
        if i as i32 >= ntoks {
            break;
        }
        if cio::strlen(&tok[i]) == 0 {
            continue;
        }
        m = project_findObject(prj, TIMEPATTERN, &tok[i]);
        if m < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[i]);
        }
        pats[i - 3] = m;
    }

    // --- check if inflow for this constituent already exists
    let ju = j as usize;
    let mut idx: Option<usize> = None;
    for (n, inflow) in prj.Node[ju].dwfInflow.iter().enumerate() {
        if inflow.param == k {
            idx = Some(n);
            break;
        }
    }

    // --- if it doesn't exist, then create it
    //     (C prepends it to the node's list)
    let n = match idx {
        Some(n) => n,
        None => {
            prj.Node[ju].dwfInflow.insert(0, TDwfInflow::default());
            0
        }
    };

    // --- assign property values to the inflow object
    let inflow = &mut prj.Node[ju].dwfInflow[n];
    inflow.param = k;
    inflow.avgValue = x;
    for i in 0..4usize {
        inflow.patterns[i] = pats[i];
    }
    0
}

//=============================================================================

/// Called by: project.
pub fn inflow_deleteDwfInflows(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: deletes all dry weather inflow data for a node.
//
//  (C frees the list but leaves Node[j].dwfInflow dangling; the node is
//   deleted next, so the Vec is simply emptied.)
{
    prj.Node[j as usize].dwfInflow.clear();
}

//=============================================================================

/// Called by: node.
pub fn inflow_initDwfInflow(patterns: &[TPattern], inflow: &mut TDwfInflow)
//
//  Input:   inflow = dry weather inflow data structure
//  Output:  none
//  Purpose: initialzes a dry weather inflow by ordering its time patterns.
//
//  This function sorts the user-supplied time patterns for a dry weather
//  inflow in the order of the PatternType enumeration (monthly, daily,
//  weekday hourly, weekend hourly) to help speed up pattern processing.
//
{
    let mut p: i32;
    let mut tmpPattern: [i32; 4] = [0; 4]; // index of each type of DWF pattern

    // --- assume no patterns were supplied
    for i in 0..4usize {
        tmpPattern[i] = -1;
    }

    // --- assign supplied patterns to proper position (by type) in tmpPattern
    //     (a pattern never given a type has type -1 and C writes
    //      tmpPattern[-1], one int before the array on the stack; with
    //      SWMM's input checks a referenced pattern always has a type, so
    //      this is not reached for a valid model. It panics here.)
    for i in 0..4usize {
        p = inflow.patterns[i];
        if p >= 0 {
            tmpPattern[patterns[p as usize].type_ as usize] = p;
        }
    }

    // --- re-fill inflow pattern array by pattern type
    for i in 0..4usize {
        inflow.patterns[i] = tmpPattern[i];
    }
}

//=============================================================================

/// Called by: routing.
pub fn inflow_getDwfInflow(prj: &Project, inflow: &TDwfInflow, month: i32, day: i32, hour: i32) -> f64
//
//  Input:   inflow = dry weather inflow data structure
//           month = current month of year of simulation
//           day = current day of week of simulation
//           hour = current hour of day of simulation
//  Output:  returns value of dry weather inflow parameter
//  Purpose: computes dry weather inflow value at a specific point in time.
//
{
    let mut p1: i32; // pattern index
    let p2: i32;
    let mut f: f64 = 1.0; // pattern factor

    p1 = inflow.patterns[MONTHLY_PATTERN as usize];
    if p1 >= 0 {
        f *= getPatternFactor(prj, p1, month, day, hour);
    }
    p1 = inflow.patterns[DAILY_PATTERN as usize];
    if p1 >= 0 {
        f *= getPatternFactor(prj, p1, month, day, hour);
    }
    p1 = inflow.patterns[HOURLY_PATTERN as usize];
    p2 = inflow.patterns[WEEKEND_PATTERN as usize];
    if p2 >= 0 {
        if day == 0 || day == 6 {
            f *= getPatternFactor(prj, p2, month, day, hour);
        } else if p1 >= 0 {
            f *= getPatternFactor(prj, p1, month, day, hour);
        }
    } else if p1 >= 0 {
        f *= getPatternFactor(prj, p1, month, day, hour);
    }
    f * inflow.avgValue
}

//=============================================================================

/// Called by: project.
pub fn inflow_initDwfPattern(prj: &mut Project, j: i32)
//
//  Input:   j = time pattern index
//  Output:  none
//  Purpose: initialzes a dry weather inflow time pattern.
//
{
    let pat = &mut prj.Pattern[j as usize];
    for i in 0..24usize {
        pat.factor[i] = 1.0;
    }
    pat.count = 0;
    pat.type_ = -1;
    pat.ID = Vec::new();
}

//=============================================================================

/// Called by: input.
pub fn inflow_readDwfPattern(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error message
//
//  Purpose: reads values of a time pattern from a line of input data.
//
//  Format of data line is:
//    patternID  patternType  value(1) value(2) ...
//    patternID  value(n)  value(n+1) ....          (for continuation lines)
{
    let mut i: usize;
    let j: i32;
    let k: i32;
    let mut n: i32 = 1;

    // --- check for minimum number of tokens
    if ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that pattern exists in database
    j = project_findObject(prj, TIMEPATTERN, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    let ju = j as usize;

    // --- check if this is first line of pattern
    //     (ID pointer will not have been assigned yet)
    if prj.Pattern[ju].ID.is_empty() {
        // --- assign ID pointer & pattern type
        prj.Pattern[ju].ID = project_findID(prj, TIMEPATTERN, &tok[0]).unwrap_or_default();
        k = findmatch(&tok[1], PatternTypeWords);
        if k < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
        }
        prj.Pattern[ju].type_ = k;
        n = 2;
    }

    // --- start reading pattern factors from rest of line
    while ntoks > n && prj.Pattern[ju].count < 24 {
        i = prj.Pattern[ju].count as usize;
        if getDouble(&tok[n as usize], &mut prj.Pattern[ju].factor[i]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[n as usize]);
        }
        prj.Pattern[ju].count += 1;
        n += 1;
    }
    0
}

//=============================================================================

fn getPatternFactor(prj: &Project, p: i32, month: i32, day: i32, hour: i32) -> f64
//
//  Input:   p = time pattern index
//           month = current month of year of simulation
//           day = current day of week of simulation
//           hour = current hour of day of simulation
//  Output:  returns value of a time pattern multiplier
//  Purpose: computes time pattern multiplier for a specific point in time.
{
    let pat = &prj.Pattern[p as usize];
    match pat.type_ {
        MONTHLY_PATTERN => {
            if month >= 0 && month < 12 {
                return pat.factor[month as usize];
            }
        }
        DAILY_PATTERN => {
            if day >= 0 && day < 7 {
                return pat.factor[day as usize];
            }
        }
        HOURLY_PATTERN => {
            if hour >= 0 && hour < 24 {
                return pat.factor[hour as usize];
            }
        }
        WEEKEND_PATTERN => {
            if day == 0 || day == 6 {
                if hour >= 0 && hour < 24 {
                    return pat.factor[hour as usize];
                }
            }
        }
        _ => {}
    }
    1.0
}
