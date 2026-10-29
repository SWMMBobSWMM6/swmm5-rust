//! report.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   report.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21    (Build 5.2.0)
//   Author:   L. Rossman
//
//   Report writing functions.
//
//   Update History
//   ==============
//   Build 5.1.004:
//   - Ignore RDII option reported.
//   Build 5.1.007:
//   - Total exfiltration loss reported.
//   Build 5.1.008:
//   - Number of threads option reported.
//   - LID drainage volume and outfall runon reported.
//   - "Internal Outflow" label changed to "Flooding Loss" in Flow Routing
//     Continuity table.
//   - Exfiltration loss added into Quality Routing Continuity table.
//   Build 5.1.011:
//   - Blank line added after writing project title.
//   - Text of error message saved to global variable ErrorMsg.
//   - Global variable Warnings incremented after warning message issued.
//   Build 5.1.012:
//   - System time step statistics adjusted for time in steady state.
//   Build 5.1.013:
//   - Parsing of AVERAGES report option added to report_readOptions().
//   - Name of surcharge method reported in report_writeOptions().
//   - Missing format specifier added to fprintf() in report_writeErrorCode.
//   Build 5.1.014:
//   - Fixed bug in confusing keywords with ID names in report_readOptions().
//   Build 5.1.015:
//   - Fixes bug in summary statistics when Report Start date > Start Date.
//   - Support added for grouped freqency table of routing time steps.
//   Build 5.2.0:
//   - Support added for reporting most frequent non-converging links.
//   - Support added for RptFlags.disabled flag.
//   - Refactored report_readOptions().
//-----------------------------------------------------------------------------

use crate::prelude::*;

const LINE_10: &[u8] = b"----------";
const LINE_12: &[u8] = b"------------";
const LINE_51: &[u8] = b"---------------------------------------------------";
const LINE_64: &[u8] = b"----------------------------------------------------------------";

/// The file-scope variables of report.c.
#[derive(Default)]
pub struct ReportState {
    //-------------------------------------------------------------------------
    //  Shared variables
    //-------------------------------------------------------------------------
    SysTime: i64, // time_t
}

//-----------------------------------------------------------------------------
//  Imported variables
//-----------------------------------------------------------------------------
//  SubcatchResults, NodeResults, LinkResults: prj.output (defined in OUTPUT.C)
//  ErrString: prj.error (defined in ERROR.C)
//  NodeStats: prj.stats

/// `#define WRITE(x) (report_writeLine((x)))`
#[inline]
fn WRITE(prj: &mut Project, x: &[u8]) {
    report_writeLine(prj, x);
}

//-----------------------------------------------------------------------------
//  time() and ctime() of the C library
//-----------------------------------------------------------------------------

/// C `time(NULL)`: seconds since the Unix epoch.
fn c_time() -> i64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => 0,
    }
}

/// C `ctime(&t)`, formatted as glibc's asctime does
/// ("Wed Oct 28 20:46:07 2026\n"). std has no local time zone, so the
/// time is printed in UTC (C prints local time).
fn c_ctime(t: i64) -> Vec<u8> {
    const WDAY: [&[u8]; 7] = [b"Sun", b"Mon", b"Tue", b"Wed", b"Thu", b"Fri", b"Sat"];
    const MON: [&[u8]; 12] = [
        b"Jan", b"Feb", b"Mar", b"Apr", b"May", b"Jun",
        b"Jul", b"Aug", b"Sep", b"Oct", b"Nov", b"Dec",
    ];
    let days = t.div_euclid(86400);
    let secs = t.rem_euclid(86400);

    // --- civil date from days since 1970-01-01
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    if m <= 2 {
        y += 1;
    }
    let wday = (days + 4).rem_euclid(7); // 1970-01-01 was a Thursday

    cformat!(
        b"%.3s %.3s%3d %.2d:%.2d:%.2d %d\n",
        WDAY[wday as usize],
        MON[(m - 1) as usize],
        d as i32,
        (secs / 3600) as i32,
        (secs % 3600 / 60) as i32,
        (secs % 60) as i32,
        y as i32
    )
}

//=============================================================================

/// Called by: input.
pub fn report_readOptions(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads reporting options from a line of input
//
    let mut k: i8;
    let mut j: i32;
    let mut m: i32;
    let mut t: i32;
    if ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    k = findmatch(&tok[0], ReportWords) as i8;
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[0]);
    }

    // --- keyword not SUBCATCHMENT, NODE, or LINK
    if k < 2 || k > 4 {
        m = findmatch(&tok[1], NoYesWords);
        if m < NO || m > YES {
            return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
        }
        match k {
            0 => {
                prj.RptFlags.disabled = m as i8;
                return 0;
            } // DISABLED
            1 => {
                prj.RptFlags.input = m as i8;
                return 0;
            } // INPUT
            5 => {
                prj.RptFlags.continuity = m as i8;
                return 0;
            } // CONTINUITY
            6 => {
                prj.RptFlags.flowStats = m as i8;
                return 0;
            } // FLOWSTATS
            7 => {
                prj.RptFlags.controls = m as i8;
                return 0;
            } // CONTROLS
            8 => {
                prj.RptFlags.averages = m as i8;
                return 0;
            } // AVERAGES
            9 => return 0, // NODESTATS deprecated
            _ => return error_setInpError(prj, ERR_KEYWORD, &tok[1]),
        }
    }
    // --- SUBCATCHMENT, NODE, & LINK keywords
    else {
        m = match k {
            2 => SUBCATCH, // Subcatchments
            3 => NODE,     // Nodes
            _ => LINK,     // Links (k == 4)
        };
    }

    // --- determine object's reporting flag
    if strcomp(&tok[1], w_NONE) != 0 {
        k = NONE as i8;
    } else if strcomp(&tok[1], w_ALL) != 0 {
        k = ALL as i8;
    } else {
        // --- set reporting flag for individual objects
        k = SOME as i8;
        t = 1;
        while t < ntoks {
            j = project_findObject(prj, m, &tok[t as usize]);
            if j < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[t as usize]);
            }
            match m {
                SUBCATCH => prj.Subcatch[j as usize].rptFlag = TRUE,
                NODE => prj.Node[j as usize].rptFlag = TRUE,
                LINK => prj.Link[j as usize].rptFlag = TRUE,
                _ => {}
            }
            t += 1;
        }
    }

    // --- assign object type's reporting flag
    match m {
        SUBCATCH => prj.RptFlags.subcatchments = k,
        NODE => prj.RptFlags.nodes = k,
        LINK => prj.RptFlags.links = k,
        _ => {}
    }
    0
}

//=============================================================================

/// Called by: inlet, input, inputrpt, rain, statsrpt, swmm5.
pub fn report_writeLine(prj: &mut Project, line: &[u8]) {
//
//  Input:   line = line of text
//  Output:  none
//  Purpose: writes line of text to report file.
//
    if prj.Frpt.file.is_some() {
        cprintf!(prj.Frpt, b"\n  %s", line);
    }
}

//=============================================================================

/// Called by: swmm5.
pub fn report_writeSysTime(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes starting/ending processing times to report file.
//
    let mut theTime: Vec<u8> = Vec::new();
    let mut elapsedTime: f64;
    let endTime: i64;
    if prj.Frpt.file.is_some() {
        let sysTime = prj.report.SysTime;
        cprintf!(prj.Frpt, FMT20, c_ctime(sysTime));
        endTime = c_time();
        cprintf!(prj.Frpt, FMT20a, c_ctime(endTime));
        elapsedTime = (endTime - sysTime) as f64; // difftime
        cprintf!(prj.Frpt, FMT21);
        if elapsedTime < 1.0 {
            cprintf!(prj.Frpt, b"< 1 sec");
        } else {
            elapsedTime /= SECperDAY as f64;
            if elapsedTime >= 1.0 {
                cprintf!(prj.Frpt, b"%d.", f64_to_i32(elapsedTime.floor()));
                elapsedTime -= elapsedTime.floor();
            }
            datetime_timeToStr(elapsedTime, &mut theTime);
            cprintf!(prj.Frpt, b"%s", theTime);
        }
    }
}

//=============================================================================
//      SIMULATION OPTIONS REPORTING
//=============================================================================

/// Called by: swmm5.
pub fn report_writeLogo(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes report header lines to report file.
//
    cprintf!(prj.Frpt, FMT08);
    cprintf!(prj.Frpt, FMT09);
    cprintf!(prj.Frpt, FMT10);
    prj.report.SysTime = c_time(); // Save starting wall clock time
}

//=============================================================================

/// Called by: swmm5.
pub fn report_writeTitle(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes project title to report file.
//
    let mut i: i32;
    let mut lineCount: i32 = 0;
    if prj.ErrorCode != 0 {
        return;
    }
    i = 0;
    while i < MAXTITLE {
        if cio::strlen(&prj.Title[i as usize]) > 0 {
            let title = prj.Title[i as usize];
            WRITE(prj, &title);
            lineCount += 1;
        }
        i += 1;
    }
    let _ = lineCount; // (C never reads it)
}

//=============================================================================

/// Called by: swmm5.
pub fn report_writeOptions(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes analysis options in use to report file.
//
    let mut str: Vec<u8> = Vec::new();
    WRITE(prj, b"");
    WRITE(prj, b"****************");
    WRITE(prj, b"Analysis Options");
    WRITE(prj, b"****************");
    cprintf!(prj.Frpt, b"\n  Flow Units ............... %s",
        FlowUnitWords[prj.FlowUnits as usize]);
    cprintf!(prj.Frpt, b"\n  Process Models:");
    cprintf!(prj.Frpt, b"\n    Rainfall/Runoff ........ ");
    if prj.IgnoreRainfall != 0 || prj.Nobjects[GAGE as usize] == 0 {
        cprintf!(prj.Frpt, b"NO");
    } else {
        cprintf!(prj.Frpt, b"YES");
    }

    cprintf!(prj.Frpt, b"\n    RDII ................... ");
    if prj.IgnoreRDII != 0 || prj.Nobjects[UNITHYD as usize] == 0 {
        cprintf!(prj.Frpt, b"NO");
    } else {
        cprintf!(prj.Frpt, b"YES");
    }

    cprintf!(prj.Frpt, b"\n    Snowmelt ............... ");
    if prj.IgnoreSnowmelt != 0 || prj.Nobjects[SNOWMELT as usize] == 0 {
        cprintf!(prj.Frpt, b"NO");
    } else {
        cprintf!(prj.Frpt, b"YES");
    }
    cprintf!(prj.Frpt, b"\n    Groundwater ............ ");
    if prj.IgnoreGwater != 0 || prj.Nobjects[AQUIFER as usize] == 0 {
        cprintf!(prj.Frpt, b"NO");
    } else {
        cprintf!(prj.Frpt, b"YES");
    }
    cprintf!(prj.Frpt, b"\n    Flow Routing ........... ");
    if prj.IgnoreRouting != 0 || prj.Nobjects[LINK as usize] == 0 {
        cprintf!(prj.Frpt, b"NO");
    } else {
        cprintf!(prj.Frpt, b"YES");
        cprintf!(prj.Frpt, b"\n    Ponding Allowed ........ ");
        if prj.AllowPonding != 0 {
            cprintf!(prj.Frpt, b"YES");
        } else {
            cprintf!(prj.Frpt, b"NO");
        }
    }
    cprintf!(prj.Frpt, b"\n    Water Quality .......... ");
    if prj.IgnoreQuality != 0 || prj.Nobjects[POLLUT as usize] == 0 {
        cprintf!(prj.Frpt, b"NO");
    } else {
        cprintf!(prj.Frpt, b"YES");
    }

    if prj.Nobjects[SUBCATCH as usize] > 0 {
        cprintf!(prj.Frpt, b"\n  Infiltration Method ...... %s",
            InfilModelWords[prj.InfilModel as usize]);
    }
    if prj.Nobjects[LINK as usize] > 0 {
        cprintf!(prj.Frpt, b"\n  Flow Routing Method ...... %s",
            RouteModelWords[prj.RouteModel as usize]);
    }

    if prj.RouteModel == DW {
        cprintf!(prj.Frpt, b"\n  Surcharge Method ......... %s",
            SurchargeWords[prj.SurchargeMethod as usize]);
    }

    datetime_dateToStr(prj, prj.StartDate, &mut str);
    cprintf!(prj.Frpt, b"\n  Starting Date ............ %s", str);
    datetime_timeToStr(prj.StartTime, &mut str);
    cprintf!(prj.Frpt, b" %s", str);
    datetime_dateToStr(prj, prj.EndDate, &mut str);
    cprintf!(prj.Frpt, b"\n  Ending Date .............. %s", str);
    datetime_timeToStr(prj.EndTime, &mut str);
    cprintf!(prj.Frpt, b" %s", str);
    cprintf!(prj.Frpt, b"\n  Antecedent Dry Days ...... %.1f", prj.StartDryDays);
    datetime_timeToStr(datetime_encodeTime(0, 0, prj.ReportStep as f64), &mut str);
    cprintf!(prj.Frpt, b"\n  Report Time Step ......... %s", str);
    if prj.Nobjects[SUBCATCH as usize] > 0 {
        datetime_timeToStr(datetime_encodeTime(0, 0, prj.WetStep as f64), &mut str);
        cprintf!(prj.Frpt, b"\n  Wet Time Step ............ %s", str);
        datetime_timeToStr(datetime_encodeTime(0, 0, prj.DryStep as f64), &mut str);
        cprintf!(prj.Frpt, b"\n  Dry Time Step ............ %s", str);
    }
    if prj.Nobjects[LINK as usize] > 0 {
        cprintf!(prj.Frpt, b"\n  Routing Time Step ........ %.2f sec", prj.RouteStep);
        if prj.RouteModel == DW {
            cprintf!(prj.Frpt, b"\n  Variable Time Step ....... ");
            if prj.CourantFactor > 0.0 {
                cprintf!(prj.Frpt, b"YES");
            } else {
                cprintf!(prj.Frpt, b"NO");
            }
            cprintf!(prj.Frpt, b"\n  Maximum Trials ........... %d", prj.MaxTrials);
            cprintf!(prj.Frpt, b"\n  Number of Threads ........ %d", prj.NumThreads);
            let headTol = prj.HeadTol * UCF(prj, LENGTH);
            cprintf!(prj.Frpt, b"\n  Head Tolerance ........... %.6f ", headTol);
            if prj.UnitSystem == US {
                cprintf!(prj.Frpt, b"ft");
            } else {
                cprintf!(prj.Frpt, b"m");
            }
        }
    }
    WRITE(prj, b"");
}


//=============================================================================
//      RAINFALL FILE REPORTING
//=============================================================================

/// Called by: rain.
pub fn report_writeRainStats(prj: &mut Project, i: i32, r: &TRainStats) {
//
//  Input:   i = rain gage index
//           r = rain file summary statistics
//  Output:  none
//  Purpose: writes summary of rain data read from file to report file.
//
    let mut date1: Vec<u8> = b"***********".to_vec();
    let mut date2: Vec<u8> = b"***********".to_vec();
    if prj.RptFlags.disabled != 0 {
        return;
    }
    if i < 0 {
        WRITE(prj, b"");
        WRITE(prj, b"*********************");
        WRITE(prj, b"Rainfall File Summary");
        WRITE(prj, b"*********************");
        cprintf!(prj.Frpt,
b"\n  Station    First        Last         Recording   Periods    Periods    Periods");
        cprintf!(prj.Frpt,
b"\n  ID         Date         Date         Frequency  w/Precip    Missing    Malfunc.");
        cprintf!(prj.Frpt,
b"\n  -------------------------------------------------------------------------------");
    } else {
        if r.startDate != NO_DATE as f64 {
            datetime_dateToStr(prj, r.startDate, &mut date1);
        }
        if r.endDate != NO_DATE as f64 {
            datetime_dateToStr(prj, r.endDate, &mut date2);
        }
        cprintf!(prj.Frpt, b"%-10s %10s   %-10s   %5d min    %6ld     %6ld     %6ld\n",
            prj.Gage[i as usize].staID, date1, date2, prj.Gage[i as usize].rainInterval / 60,
            r.periodsRain, r.periodsMissing, r.periodsMalfunc);
    }
    WRITE(prj, b"");
}


//=============================================================================
//      RDII REPORTING
//=============================================================================

/// Called by: rdii.
pub fn report_writeRdiiStats(prj: &mut Project, rainVol: f64, rdiiVol: f64) {
//
//  Input:   rainVol = total rainfall volume over sewershed
//           rdiiVol = total RDII volume produced
//  Output:  none
//  Purpose: writes summary of RDII inflow to report file.
//
    let ratio: f64;
    let ucf1: f64;
    let ucf2: f64;

    if prj.RptFlags.disabled != 0 {
        return;
    }

    ucf1 = UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    if prj.UnitSystem == US {
        ucf2 = MGDperCFS / SECperDAY as f64;
    } else {
        ucf2 = MLDperCFS / SECperDAY as f64;
    }

    WRITE(prj, b"");
    cprintf!(prj.Frpt,
    b"\n  **********************           Volume        Volume");
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
    b"\n  Rainfall Dependent I/I        acre-feet      10^6 gal");
    } else {
        cprintf!(prj.Frpt,
    b"\n  Rainfall Dependent I/I        hectare-m      10^6 ltr");
    }
    cprintf!(prj.Frpt,
    b"\n  **********************        ---------     ---------");

    cprintf!(prj.Frpt, b"\n  Sewershed Rainfall ......%14.3f%14.3f",
        rainVol * ucf1, rainVol * ucf2);

    cprintf!(prj.Frpt, b"\n  RDII Produced ...........%14.3f%14.3f",
        rdiiVol * ucf1, rdiiVol * ucf2);

    if rainVol == 0.0 {
        ratio = 0.0;
    } else {
        ratio = rdiiVol / rainVol;
    }
    cprintf!(prj.Frpt, b"\n  RDII Ratio ..............%14.3f", ratio);
    WRITE(prj, b"");
}


//=============================================================================
//      CONTROL ACTIONS REPORTING
//=============================================================================

/// Called by: swmm5.
pub fn report_writeControlActionsHeading(prj: &mut Project) {
    WRITE(prj, b"");
    WRITE(prj, b"*********************");
    WRITE(prj, b"Control Actions Taken");
    WRITE(prj, b"*********************");
    cprintf!(prj.Frpt, b"\n");
}

//=============================================================================

/// Called by: controls.
pub fn report_writeControlAction(prj: &mut Project, aDate: DateTime, linkID: &[u8], value: f64, ruleID: &[u8]) {
//
//  Input:   aDate  = date/time of rule action
//           linkID = ID of link being controlled
//           value  = new status value of link
//           ruleID = ID of rule implementing the action
//  Output:  none
//  Purpose: reports action taken by a control rule.
//
    let mut theDate: Vec<u8> = Vec::new();
    let mut theTime: Vec<u8> = Vec::new();

    if prj.RptFlags.disabled != 0 {
        return;
    }
    datetime_dateToStr(prj, aDate, &mut theDate);
    datetime_timeToStr(aDate, &mut theTime);
    cprintf!(prj.Frpt,
        b"  %11s: %8s Link %s setting changed to %6.2f by Control %s\n",
        theDate, theTime, linkID, value, ruleID);
}


//=============================================================================
//      CONTINUITY ERROR REPORTING
//=============================================================================

/// Called by: massbal.
pub fn report_writeRunoffError(prj: &mut Project, totals: &TRunoffTotals, totalArea: f64) {
//
//  Input:  totals = accumulated runoff totals
//          totalArea = total area of all subcatchments
//  Output:  none
//  Purpose: writes runoff continuity error to report file.
//
    if prj.Frunoff.mode as i32 == USE_FILE {
        WRITE(prj, b"");
        cprintf!(prj.Frpt,
        b"\n  **************************\
\n  Runoff Quantity Continuity\
\n  **************************\
\n  Runoff supplied by interface file %s", prj.Frunoff.name);
        WRITE(prj, b"");
        return;
    }

    if totalArea == 0.0 {
        return;
    }
    WRITE(prj, b"");

    cprintf!(prj.Frpt,
    b"\n  **************************        Volume         Depth");
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
    b"\n  Runoff Quantity Continuity     acre-feet        inches");
    } else {
        cprintf!(prj.Frpt,
    b"\n  Runoff Quantity Continuity     hectare-m            mm");
    }
    cprintf!(prj.Frpt,
    b"\n  **************************     ---------       -------");

    if totals.initStorage > 0.0 {
        let a = totals.initStorage * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
        let b = totals.initStorage / totalArea * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b"\n  Initial LID Storage ......%14.3f%14.3f", a, b);
    }

    if prj.Nobjects[SNOWMELT as usize] > 0 {
        let a = totals.initSnowCover * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
        let b = totals.initSnowCover / totalArea * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b"\n  Initial Snow Cover .......%14.3f%14.3f", a, b);
    }

    let a = totals.rainfall * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.rainfall / totalArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Total Precipitation ......%14.3f%14.3f", a, b);

    if totals.runon > 0.0 {
        let a = totals.runon * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
        let b = totals.runon / totalArea * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b"\n  Outfall Runon ............%14.3f%14.3f", a, b);
    }

    let a = totals.evap * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.evap / totalArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Evaporation Loss .........%14.3f%14.3f", a, b);

    let a = totals.infil * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.infil / totalArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Infiltration Loss ........%14.3f%14.3f", a, b);

    let a = totals.runoff * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.runoff / totalArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Surface Runoff ...........%14.3f%14.3f", a, b);

    if totals.drains > 0.0 {
        let a = totals.drains * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
        let b = totals.drains / totalArea * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b"\n  LID Drainage .............%14.3f%14.3f", a, b);
    }

    if prj.Nobjects[SNOWMELT as usize] > 0 {
        let a = totals.snowRemoved * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
        let b = totals.snowRemoved / totalArea * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b"\n  Snow Removed .............%14.3f%14.3f", a, b);
        let a = totals.finalSnowCover * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
        let b = totals.finalSnowCover / totalArea * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b"\n  Final Snow Cover .........%14.3f%14.3f", a, b);
    }

    let a = totals.finalStorage * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.finalStorage / totalArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Final Storage ............%14.3f%14.3f", a, b);

    cprintf!(prj.Frpt, b"\n  Continuity Error (%%) .....%14.3f",
        totals.pctError);
    WRITE(prj, b"");
}

//=============================================================================

/// Called by: massbal.
pub fn report_writeLoadingError(prj: &mut Project, totals: &[TLoadingTotals]) {
//
//  Input:   totals = accumulated pollutant loading totals
//           area = total area of all subcatchments
//  Output:  none
//  Purpose: writes runoff loading continuity error to report file.
//
    let mut p1: i32;
    let mut p2: i32;
    p1 = 1;
    p2 = MIN(5, prj.Nobjects[POLLUT as usize]);
    while p1 <= prj.Nobjects[POLLUT as usize] {
        report_LoadingErrors(prj, p1 - 1, p2 - 1, totals);
        p1 = p2 + 1;
        p2 = p1 + 4;
        p2 = MIN(p2, prj.Nobjects[POLLUT as usize]);
    }
}

//=============================================================================

fn report_LoadingErrors(prj: &mut Project, p1: i32, p2: i32, totals: &[TLoadingTotals]) {
//
//  Input:   p1 = index of first pollutant to report
//           p2 = index of last pollutant to report
//           totals = accumulated pollutant loading totals
//           area = total area of all subcatchments
//  Output:  none
//  Purpose: writes runoff loading continuity error to report file for
//           up to 5 pollutants at a time.
//
    let mut i: i32;
    let mut p: i32;
    let cf: f64 = 1.0;
    let mut units: [u8; 15] = [0; 15];

    WRITE(prj, b"");
    cprintf!(prj.Frpt, b"\n  **************************");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14s", prj.Pollut[p as usize].ID);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Runoff Quality Continuity ");
    p = p1;
    while p <= p2 {
        i = prj.UnitSystem;
        if prj.Pollut[p as usize].units == COUNT {
            i = 2;
        }
        cio::sstrncpy(&mut units, LoadUnitsWords[i as usize], 14);
        cprintf!(prj.Frpt, b"%14s", units);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  **************************");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"    ----------");
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Initial Buildup ..........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].initLoad * cf);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Surface Buildup ..........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].buildup * cf);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Wet Deposition ...........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].deposition * cf);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Sweeping Removal .........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].sweeping * cf);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Infiltration Loss ........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].infil * cf);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  BMP Removal ..............");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].bmpRemoval * cf);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Surface Runoff ...........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].runoff * cf);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Remaining Buildup ........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].finalLoad * cf);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Continuity Error (%%) .....");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", totals[p as usize].pctError);
        p += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

/// Called by: massbal.
pub fn report_writeGwaterError(prj: &mut Project, totals: &TGwaterTotals, gwArea: f64) {
//
//  Input:   totals = accumulated groundwater totals
//           gwArea = total area of all subcatchments with groundwater
//  Output:  none
//  Purpose: writes groundwater continuity error to report file.
//
    WRITE(prj, b"");
    cprintf!(prj.Frpt,
    b"\n  **************************        Volume         Depth");
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
    b"\n  Groundwater Continuity         acre-feet        inches");
    } else {
        cprintf!(prj.Frpt,
    b"\n  Groundwater Continuity         hectare-m            mm");
    }
    cprintf!(prj.Frpt,
    b"\n  **************************     ---------       -------");
    let a = totals.initStorage * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.initStorage / gwArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Initial Storage ..........%14.3f%14.3f", a, b);

    let a = totals.infil * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.infil / gwArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Infiltration .............%14.3f%14.3f", a, b);

    let a = totals.upperEvap * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.upperEvap / gwArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Upper Zone ET ............%14.3f%14.3f", a, b);

    let a = totals.lowerEvap * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.lowerEvap / gwArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Lower Zone ET ............%14.3f%14.3f", a, b);

    let a = totals.lowerPerc * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.lowerPerc / gwArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Deep Percolation .........%14.3f%14.3f", a, b);

    let a = totals.gwater * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.gwater / gwArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Groundwater Flow .........%14.3f%14.3f", a, b);

    let a = totals.finalStorage * UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    let b = totals.finalStorage / gwArea * UCF(prj, RAINDEPTH);
    cprintf!(prj.Frpt, b"\n  Final Storage ............%14.3f%14.3f", a, b);

    cprintf!(prj.Frpt, b"\n  Continuity Error (%%) .....%14.3f",
        totals.pctError);
    WRITE(prj, b"");
}

//=============================================================================

/// Called by: massbal.
pub fn report_writeFlowError(prj: &mut Project, totals: &TRoutingTotals) {
//
//  Input:  totals = accumulated flow routing totals
//  Output:  none
//  Purpose: writes flow routing continuity error to report file.
//
    let ucf1: f64;
    let ucf2: f64;

    ucf1 = UCF(prj, LENGTH) * UCF(prj, LANDAREA);
    if prj.UnitSystem == US {
        ucf2 = MGDperCFS / SECperDAY as f64;
    } else {
        ucf2 = MLDperCFS / SECperDAY as f64;
    }

    WRITE(prj, b"");
    cprintf!(prj.Frpt,
    b"\n  **************************        Volume        Volume");
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
    b"\n  Flow Routing Continuity        acre-feet      10^6 gal");
    } else {
        cprintf!(prj.Frpt,
    b"\n  Flow Routing Continuity        hectare-m      10^6 ltr");
    }
    cprintf!(prj.Frpt,
    b"\n  **************************     ---------     ---------");

    cprintf!(prj.Frpt, b"\n  Dry Weather Inflow .......%14.3f%14.3f",
        totals.dwInflow * ucf1, totals.dwInflow * ucf2);

    cprintf!(prj.Frpt, b"\n  Wet Weather Inflow .......%14.3f%14.3f",
        totals.wwInflow * ucf1, totals.wwInflow * ucf2);

    cprintf!(prj.Frpt, b"\n  Groundwater Inflow .......%14.3f%14.3f",
        totals.gwInflow * ucf1, totals.gwInflow * ucf2);

    cprintf!(prj.Frpt, b"\n  RDII Inflow ..............%14.3f%14.3f",
        totals.iiInflow * ucf1, totals.iiInflow * ucf2);

    cprintf!(prj.Frpt, b"\n  External Inflow ..........%14.3f%14.3f",
        totals.exInflow * ucf1, totals.exInflow * ucf2);

    cprintf!(prj.Frpt, b"\n  External Outflow .........%14.3f%14.3f",
        totals.outflow * ucf1, totals.outflow * ucf2);

    cprintf!(prj.Frpt, b"\n  Flooding Loss ............%14.3f%14.3f",
        totals.flooding * ucf1, totals.flooding * ucf2);

    cprintf!(prj.Frpt, b"\n  Evaporation Loss .........%14.3f%14.3f",
        totals.evapLoss * ucf1, totals.evapLoss * ucf2);

    cprintf!(prj.Frpt, b"\n  Exfiltration Loss ........%14.3f%14.3f",
        totals.seepLoss * ucf1, totals.seepLoss * ucf2);

    cprintf!(prj.Frpt, b"\n  Initial Stored Volume ....%14.3f%14.3f",
        totals.initStorage * ucf1, totals.initStorage * ucf2);

    cprintf!(prj.Frpt, b"\n  Final Stored Volume ......%14.3f%14.3f",
        totals.finalStorage * ucf1, totals.finalStorage * ucf2);

    cprintf!(prj.Frpt, b"\n  Continuity Error (%%) .....%14.3f",
        totals.pctError);
    WRITE(prj, b"");
}

//=============================================================================

/// Called by: massbal.
pub fn report_writeQualError(prj: &mut Project, QualTotals: &[TRoutingTotals]) {
//
//  Input:   totals = accumulated quality routing totals for each pollutant
//  Output:  none
//  Purpose: writes quality routing continuity error to report file.
//
    let mut p1: i32;
    let mut p2: i32;
    p1 = 1;
    p2 = MIN(5, prj.Nobjects[POLLUT as usize]);
    while p1 <= prj.Nobjects[POLLUT as usize] {
        report_QualErrors(prj, p1 - 1, p2 - 1, QualTotals);
        p1 = p2 + 1;
        p2 = p1 + 4;
        p2 = MIN(p2, prj.Nobjects[POLLUT as usize]);
    }
}

//=============================================================================

fn report_QualErrors(prj: &mut Project, p1: i32, p2: i32, QualTotals: &[TRoutingTotals]) {
    let mut i: i32;
    let mut p: i32;
    let mut units: [u8; 15] = [0; 15];

    WRITE(prj, b"");
    cprintf!(prj.Frpt, b"\n  **************************");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14s", prj.Pollut[p as usize].ID);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Quality Routing Continuity");
    p = p1;
    while p <= p2 {
        i = prj.UnitSystem;
        if prj.Pollut[p as usize].units == COUNT {
            i = 2;
        }
        cio::sstrncpy(&mut units, LoadUnitsWords[i as usize], 14);
        cprintf!(prj.Frpt, b"%14s", units);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  **************************");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"    ----------");
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Dry Weather Inflow .......");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].dwInflow);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Wet Weather Inflow .......");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].wwInflow);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Groundwater Inflow .......");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].gwInflow);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  RDII Inflow ..............");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].iiInflow);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  External Inflow ..........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].exInflow);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  External Outflow .........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].outflow);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Flooding Loss ............");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].flooding);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Exfiltration Loss ........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].seepLoss);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Mass Reacted .............");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].reacted);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Initial Stored Mass ......");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].initStorage);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Final Stored Mass ........");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].finalStorage);
        p += 1;
    }

    cprintf!(prj.Frpt, b"\n  Continuity Error (%%) .....");
    p = p1;
    while p <= p2 {
        cprintf!(prj.Frpt, b"%14.3f", QualTotals[p as usize].pctError);
        p += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

/// Called by: stats.
pub fn report_writeMaxStats(prj: &mut Project, maxMassBalErrs: &[TMaxStats], maxCourantCrit: &[TMaxStats], nMaxStats: i32) {
//
//  Input:   maxMassBal[] = nodes with highest mass balance errors
//           maxCourantCrit[] = nodes most often Courant time step critical
//           maxLinkTimes[] = links most often Courant time step critical
//           nMaxStats = number of most critical nodes/links saved
//  Output:  none
//  Purpose: lists nodes & links with highest mass balance errors and
//           time Courant time step critical
//
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;

    if prj.RouteModel != DW || prj.Nobjects[LINK as usize] == 0 {
        return;
    }
    if nMaxStats <= 0 {
        return;
    }
    if maxMassBalErrs[0].index >= 0 {
        WRITE(prj, b"");
        WRITE(prj, b"*************************");
        WRITE(prj, b"Highest Continuity Errors");
        WRITE(prj, b"*************************");
        i = 0;
        while i < nMaxStats {
            j = maxMassBalErrs[i as usize].index;
            if j < 0 {
                i += 1;
                continue;
            }
            cprintf!(prj.Frpt, b"\n  Node %s (%.2f%%)",
                prj.Node[j as usize].ID, maxMassBalErrs[i as usize].value);
            i += 1;
        }
        WRITE(prj, b"");
    }

    if prj.CourantFactor == 0.0 {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"***************************");
    WRITE(prj, b"Time-Step Critical Elements");
    WRITE(prj, b"***************************");
    k = 0;
    i = 0;
    while i < nMaxStats {
        j = maxCourantCrit[i as usize].index;
        if j < 0 {
            i += 1;
            continue;
        }
        k += 1;
        if maxCourantCrit[i as usize].objType == NODE {
            cprintf!(prj.Frpt, b"\n  Node %s", prj.Node[j as usize].ID);
        } else {
            cprintf!(prj.Frpt, b"\n  Link %s", prj.Link[j as usize].ID);
        }
        cprintf!(prj.Frpt, b" (%.2f%%)", maxCourantCrit[i as usize].value);
        i += 1;
    }
    if k == 0 {
        cprintf!(prj.Frpt, b"\n  None");
    }
    WRITE(prj, b"");
}

//=============================================================================

/// Called by: stats.
pub fn report_writeMaxFlowTurns(prj: &mut Project, flowTurns: &[TMaxStats], nMaxStats: i32) {
//
//  Input:   flowTurns[] = links with highest number of flow turns
//           nMaxStats = number of links in flowTurns[]
//  Output:  none
//  Purpose: lists links with highest number of flow turns (i.e., fraction
//           of time periods where the flow is higher (or lower) than the
//           flows in the previous and following periods).
//
    let mut i: i32;
    let mut j: i32;

    if prj.Nobjects[LINK as usize] == 0 {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"********************************");
    WRITE(prj, b"Highest Flow Instability Indexes");
    WRITE(prj, b"********************************");
    if nMaxStats <= 0 || flowTurns[0].index <= 0 {
        cprintf!(prj.Frpt, b"\n  All links are stable.");
    } else {
        i = 0;
        while i < nMaxStats {
            j = flowTurns[i as usize].index;
            if j < 0 {
                i += 1;
                continue;
            }
            cprintf!(prj.Frpt, b"\n  Link %s (%.0f)",
                prj.Link[j as usize].ID, flowTurns[i as usize].value);
            i += 1;
        }
    }
    WRITE(prj, b"");
}

//=============================================================================

/// Called by: stats.
pub fn report_writeNonconvergedStats(prj: &mut Project, maxNonconverged: &[TMaxStats], nMaxStats: i32) {
    let mut i: i32;
    let mut j: i32;
    if prj.Nobjects[NODE as usize] == 0 || prj.RouteModel != DW {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"*********************************");
    WRITE(prj, b"Most Frequent Nonconverging Nodes");
    WRITE(prj, b"*********************************");
    if nMaxStats <= 0 || maxNonconverged[0].index <= 0 || maxNonconverged[0].value < 0.00005 {
        cprintf!(prj.Frpt, b"\n  Convergence obtained at all time steps.");
    } else {
        i = 0;
        while i < nMaxStats {
            j = maxNonconverged[i as usize].index;
            if j < 0 || maxNonconverged[i as usize].value <= 0.0 {
                i += 1;
                continue;
            }
            cprintf!(prj.Frpt, b"\n  Node %s (%.2f%%)",
                prj.Node[j as usize].ID, 100.0 * maxNonconverged[i as usize].value);
            i += 1;
        }
    }
    WRITE(prj, b"");
}

//=============================================================================

/// Called by: stats.
pub fn report_writeTimeStepStats(prj: &mut Project, timeStepStats: &TTimeStepStats) {
//
//  Input:   timeStepStats = routing time step statistics
//  Output:  none
//  Purpose: writes routing time step statistics to report file.
//
    let timeStepCount: f64 = timeStepStats.timeStepCount as f64;
    let totalRoutingTime: f64; // time taken for all flow routing steps
    let mut fSteadyState: f64 = 0.0; // fraction of reporting time in steady state

    if prj.Nobjects[LINK as usize] == 0 || timeStepStats.timeStepCount as f64 == 0.0 {
        return;
    }

    totalRoutingTime = timeStepStats.steadyStateTime + timeStepStats.routingTime;
    if totalRoutingTime > 0.0 {
        fSteadyState = 100.0 * timeStepStats.steadyStateTime / totalRoutingTime;
    }

    WRITE(prj, b"");
    WRITE(prj, b"*************************");
    WRITE(prj, b"Routing Time Step Summary");
    WRITE(prj, b"*************************");
    cprintf!(prj.Frpt,
        b"\n  Minimum Time Step           :  %7.2f sec",
        timeStepStats.minTimeStep);
    cprintf!(prj.Frpt,
        b"\n  Average Time Step           :  %7.2f sec",
        timeStepStats.routingTime / timeStepCount);
    cprintf!(prj.Frpt,
        b"\n  Maximum Time Step           :  %7.2f sec",
        timeStepStats.maxTimeStep);
    cprintf!(prj.Frpt,
        b"\n  %% of Time in Steady State   :  %7.2f", MIN(fSteadyState, 100.0));
    cprintf!(prj.Frpt,
        b"\n  Average Iterations per Step :  %7.2f",
        timeStepStats.trialsCount / timeStepCount);
    cprintf!(prj.Frpt,
        b"\n  %% of Steps Not Converging   :  %7.2f",
        100.0 * prj.NonConvergeCount as f64 / timeStepCount);

    // --- write grouped frequency table of variable routing time steps
    if prj.RouteModel == DW && prj.CourantFactor > 0.0 {
        report_RouteStepFreq(prj, timeStepStats);
    }
    WRITE(prj, b"");
}

//=============================================================================

fn report_RouteStepFreq(prj: &mut Project, timeStepStats: &TTimeStepStats) {
//
//  Input:   timeStepStats = routing time step statistics
//  Output:  none
//  Purpose: writes grouped frequency table of routing time steps to report file.
//
    let mut totalSteps: f64 = 0.0;
    let mut i: i32;

    i = 1;
    while i < TIMELEVELS {
        totalSteps += timeStepStats.timeStepCounts[i as usize] as f64;
        i += 1;
    }
    if totalSteps == 0.0 {
        return;
    }
    cprintf!(prj.Frpt,
        b"\n  Time Step Frequencies       :");
    i = 1;
    while i < TIMELEVELS {
        cprintf!(prj.Frpt,
            b"\n     %6.3f - %6.3f sec      :  %7.2f %%",
            timeStepStats.timeStepIntervals[(i - 1) as usize],
            timeStepStats.timeStepIntervals[i as usize],
            100.0 * timeStepStats.timeStepCounts[i as usize] as f64 / totalSteps);
        i += 1;
    }
}


//=============================================================================
//      SIMULATION RESULTS REPORTING
//=============================================================================

/// Called by: swmm5.
pub fn report_writeReport(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes simulation results to report file.
//
    if prj.ErrorCode != 0 {
        return;
    }
    if prj.Nperiods == 0 {
        return;
    }
    if prj.RptFlags.subcatchments as i32 != NONE
        && (prj.IgnoreRainfall == FALSE || prj.IgnoreSnowmelt == FALSE || prj.IgnoreGwater == FALSE)
    {
        report_Subcatchments(prj);
    }

    if prj.IgnoreRouting == TRUE && prj.IgnoreQuality == TRUE {
        return;
    }
    if prj.RptFlags.nodes as i32 != NONE {
        report_Nodes(prj);
    }
    if prj.RptFlags.links as i32 != NONE {
        report_Links(prj);
    }
}

//=============================================================================

fn report_Subcatchments(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes results for selected subcatchments to report file.
//
    let mut j: i32;
    let mut p: i32;
    let mut k: i32;
    let mut period: i32;
    let mut days: DateTime = 0.0;
    let mut theDate: Vec<u8> = Vec::new();
    let mut theTime: Vec<u8> = Vec::new();
    let hasSnowmelt: i32 = (prj.Nobjects[SNOWMELT as usize] > 0 && prj.IgnoreSnowmelt == 0) as i32;
    let hasGwater: i32 = (prj.Nobjects[AQUIFER as usize] > 0 && prj.IgnoreGwater == 0) as i32;
    let hasQuality: i32 = (prj.Nobjects[POLLUT as usize] > 0 && prj.IgnoreQuality == 0) as i32;

    if prj.Nobjects[SUBCATCH as usize] == 0 {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"********************************");
    WRITE(prj, b"Subcatchment Time Series Results");
    WRITE(prj, b"********************************");
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        k = prj.Subcatch[j as usize].rptFlag - 1;
        if k >= 0 {
            let id = prj.Subcatch[j as usize].ID.clone();
            report_SubcatchHeader(prj, &id);
            period = 1;
            while period as i64 <= prj.Nperiods {
                output_readDateTime(prj, period as i64, &mut days);
                datetime_dateToStr(prj, days, &mut theDate);
                datetime_timeToStr(days, &mut theTime);
                output_readSubcatchResults(prj, period as i64, k);
                let r = &prj.output.SubcatchResults;
                let rain = r[SUBCATCH_RAINFALL as usize] as f64;
                let losses = r[SUBCATCH_EVAP as usize] as f64 / 24.0 + r[SUBCATCH_INFIL as usize] as f64;
                let runoff = r[SUBCATCH_RUNOFF as usize] as f64;
                cprintf!(prj.Frpt, b"\n  %11s %8s %10.3f%10.3f%10.4f",
                    theDate, theTime, rain, losses, runoff);
                if hasSnowmelt != 0 {
                    let v = prj.output.SubcatchResults[SUBCATCH_SNOWDEPTH as usize] as f64;
                    cprintf!(prj.Frpt, b"  %10.3f", v);
                }
                if hasGwater != 0 {
                    let v1 = prj.output.SubcatchResults[SUBCATCH_GW_ELEV as usize] as f64;
                    let v2 = prj.output.SubcatchResults[SUBCATCH_GW_FLOW as usize] as f64;
                    cprintf!(prj.Frpt, b"%10.3f%10.4f", v1, v2);
                }
                if hasQuality != 0 {
                    p = 0;
                    while p < prj.Nobjects[POLLUT as usize] {
                        let v = prj.output.SubcatchResults[(SUBCATCH_WASHOFF + p) as usize] as f64;
                        cprintf!(prj.Frpt, b"%10.3f", v);
                        p += 1;
                    }
                }
                period += 1;
            }
            WRITE(prj, b"");
        }
        j += 1;
    }
}

//=============================================================================

fn report_SubcatchHeader(prj: &mut Project, id: &[u8]) {
//
//  Input:   id = subcatchment ID name
//  Output:  none
//  Purpose: writes table headings for subcatchment results to report file.
//
    let mut i: i32;
    let hasSnowmelt: i32 = (prj.Nobjects[SNOWMELT as usize] > 0 && prj.IgnoreSnowmelt == 0) as i32;
    let hasGwater: i32 = (prj.Nobjects[AQUIFER as usize] > 0 && prj.IgnoreGwater == 0) as i32;
    let hasQuality: i32 = (prj.Nobjects[POLLUT as usize] > 0 && prj.IgnoreQuality == 0) as i32;

    // --- print top border of header
    WRITE(prj, b"");
    cprintf!(prj.Frpt, b"\n  <<< Subcatchment %s >>>", id);
    WRITE(prj, LINE_51);
    if hasSnowmelt > 0 {
        cprintf!(prj.Frpt, LINE_12);
    }
    if hasGwater != 0 {
        cprintf!(prj.Frpt, LINE_10);
        cprintf!(prj.Frpt, LINE_10);
    }
    if hasQuality != 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, LINE_10);
            i += 1;
        }
    }

    // --- print first line of column headings
    cprintf!(prj.Frpt,
    b"\n  Date        Time        Precip.    Losses    Runoff");
    if hasSnowmelt != 0 {
        cprintf!(prj.Frpt, b"  Snow Depth");
    }
    if hasGwater != 0 {
        cprintf!(prj.Frpt, b"  GW Elev.   GW Flow");
    }
    if hasQuality != 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, b"%10s", prj.Pollut[i as usize].ID);
            i += 1;
        }
    }

    // --- print second line of column headings
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
    b"\n                            in/hr     in/hr %9s", FlowUnitWords[prj.FlowUnits as usize]);
    } else {
        cprintf!(prj.Frpt,
    b"\n                            mm/hr     mm/hr %9s", FlowUnitWords[prj.FlowUnits as usize]);
    }
    if hasSnowmelt != 0 {
        if prj.UnitSystem == US {
            cprintf!(prj.Frpt, b"      inches");
        } else {
            cprintf!(prj.Frpt, b"     mmeters");
        }
    }
    if hasGwater != 0 {
        if prj.UnitSystem == US {
            cprintf!(prj.Frpt, b"      feet %9s", FlowUnitWords[prj.FlowUnits as usize]);
        } else {
            cprintf!(prj.Frpt, b"    meters %9s", FlowUnitWords[prj.FlowUnits as usize]);
        }
    }
    if hasQuality != 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, b"%10s", QualUnitsWords[prj.Pollut[i as usize].units as usize]);
            i += 1;
        }
    }

    // --- print lower border of header
    WRITE(prj, LINE_51);
    if hasSnowmelt != 0 {
        cprintf!(prj.Frpt, LINE_12);
    }
    if hasGwater != 0 {
        cprintf!(prj.Frpt, LINE_10);
        cprintf!(prj.Frpt, LINE_10);
    }
    if hasQuality != 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, LINE_10);
            i += 1;
        }
    }
}

//=============================================================================

fn report_Nodes(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes results for selected nodes to report file.
//
    let mut j: i32;
    let mut p: i32;
    let mut k: i32;
    let mut period: i32;
    let mut days: DateTime = 0.0;
    let mut theDate: Vec<u8> = Vec::new();
    let mut theTime: Vec<u8> = Vec::new();

    if prj.Nobjects[NODE as usize] == 0 {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"************************");
    WRITE(prj, b"Node Time Series Results");
    WRITE(prj, b"************************");
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        k = prj.Node[j as usize].rptFlag - 1;
        if k >= 0 {
            let id = prj.Node[j as usize].ID.clone();
            report_NodeHeader(prj, &id);
            period = 1;
            while period as i64 <= prj.Nperiods {
                output_readDateTime(prj, period as i64, &mut days);
                datetime_dateToStr(prj, days, &mut theDate);
                datetime_timeToStr(days, &mut theTime);
                output_readNodeResults(prj, period as i64, k);
                let r = &prj.output.NodeResults;
                let v1 = r[NODE_INFLOW as usize] as f64;
                let v2 = r[NODE_OVERFLOW as usize] as f64;
                let v3 = r[NODE_DEPTH as usize] as f64;
                let v4 = r[NODE_HEAD as usize] as f64;
                cprintf!(prj.Frpt, b"\n  %11s %8s  %9.3f %9.3f %9.3f %9.3f",
                    theDate, theTime, v1, v2, v3, v4);
                if prj.IgnoreQuality == 0 {
                    p = 0;
                    while p < prj.Nobjects[POLLUT as usize] {
                        let v = prj.output.NodeResults[(NODE_QUAL + p) as usize] as f64;
                        cprintf!(prj.Frpt, b" %9.3f", v);
                        p += 1;
                    }
                }
                period += 1;
            }
            WRITE(prj, b"");
        }
        j += 1;
    }
}

//=============================================================================

fn report_NodeHeader(prj: &mut Project, id: &[u8]) {
//
//  Input:   id = node ID name
//  Output:  none
//  Purpose: writes table headings for node results to report file.
//
    let mut i: i32;
    let mut lengthUnits: [u8; 9] = [0; 9];
    WRITE(prj, b"");
    cprintf!(prj.Frpt, b"\n  <<< Node %s >>>", id);
    WRITE(prj, LINE_64);
    i = 0;
    while i < prj.Nobjects[POLLUT as usize] {
        cprintf!(prj.Frpt, LINE_10);
        i += 1;
    }

    cprintf!(prj.Frpt,
    b"\n                           Inflow  Flooding     Depth      Head");
    if prj.IgnoreQuality == 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, b"%10s", prj.Pollut[i as usize].ID);
            i += 1;
        }
    }
    if prj.UnitSystem == US {
        cio::sstrncpy(&mut lengthUnits, b"feet", 8);
    } else {
        cio::sstrncpy(&mut lengthUnits, b"meters", 8);
    }
    cprintf!(prj.Frpt,
    b"\n  Date        Time      %9s %9s %9s %9s",
        FlowUnitWords[prj.FlowUnits as usize], FlowUnitWords[prj.FlowUnits as usize],
        lengthUnits, lengthUnits);
    if prj.IgnoreQuality == 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, b"%10s", QualUnitsWords[prj.Pollut[i as usize].units as usize]);
            i += 1;
        }
    }

    WRITE(prj, LINE_64);
    if prj.IgnoreQuality == 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, LINE_10);
            i += 1;
        }
    }
}

//=============================================================================

fn report_Links(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes results for selected links to report file.
//
    let mut j: i32;
    let mut p: i32;
    let mut k: i32;
    let mut period: i32;
    let mut days: DateTime = 0.0;
    let mut theDate: Vec<u8> = Vec::new();
    let mut theTime: Vec<u8> = Vec::new();

    if prj.Nobjects[LINK as usize] == 0 {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"************************");
    WRITE(prj, b"Link Time Series Results");
    WRITE(prj, b"************************");
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        k = prj.Link[j as usize].rptFlag - 1;
        if k >= 0 {
            let id = prj.Link[j as usize].ID.clone();
            report_LinkHeader(prj, &id);
            period = 1;
            while period as i64 <= prj.Nperiods {
                output_readDateTime(prj, period as i64, &mut days);
                datetime_dateToStr(prj, days, &mut theDate);
                datetime_timeToStr(days, &mut theTime);
                output_readLinkResults(prj, period as i64, k);
                let r = &prj.output.LinkResults;
                let v1 = r[LINK_FLOW as usize] as f64;
                let v2 = r[LINK_VELOCITY as usize] as f64;
                let v3 = r[LINK_DEPTH as usize] as f64;
                let v4 = r[LINK_CAPACITY as usize] as f64;
                cprintf!(prj.Frpt, b"\n  %11s %8s  %9.3f %9.3f %9.3f %9.3f",
                    theDate, theTime, v1, v2, v3, v4);
                if prj.IgnoreQuality == 0 {
                    p = 0;
                    while p < prj.Nobjects[POLLUT as usize] {
                        let v = prj.output.LinkResults[(LINK_QUAL + p) as usize] as f64;
                        cprintf!(prj.Frpt, b" %9.3f", v);
                        p += 1;
                    }
                }
                period += 1;
            }
            WRITE(prj, b"");
        }
        j += 1;
    }
}

//=============================================================================

fn report_LinkHeader(prj: &mut Project, id: &[u8]) {
//
//  Input:   id = link ID name
//  Output:  none
//  Purpose: writes table headings for link results to report file.
//
    let mut i: i32;
    WRITE(prj, b"");
    cprintf!(prj.Frpt, b"\n  <<< Link %s >>>", id);
    WRITE(prj, LINE_64);
    i = 0;
    while i < prj.Nobjects[POLLUT as usize] {
        cprintf!(prj.Frpt, LINE_10);
        i += 1;
    }

    cprintf!(prj.Frpt,
    b"\n                             Flow  Velocity     Depth  Capacity/");
    if prj.IgnoreQuality == 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, b"%10s", prj.Pollut[i as usize].ID);
            i += 1;
        }
    }

    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
        b"\n  Date        Time     %10s    ft/sec      feet   Setting ",
            FlowUnitWords[prj.FlowUnits as usize]);
    } else {
        cprintf!(prj.Frpt,
        b"\n  Date        Time     %10s     m/sec    meters   Setting ",
            FlowUnitWords[prj.FlowUnits as usize]);
    }
    if prj.IgnoreQuality == 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, b" %9s", QualUnitsWords[prj.Pollut[i as usize].units as usize]);
            i += 1;
        }
    }

    WRITE(prj, LINE_64);
    if prj.IgnoreQuality == 0 {
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            cprintf!(prj.Frpt, LINE_10);
            i += 1;
        }
    }
}


//=============================================================================
//      ERROR REPORTING
//=============================================================================

/// `error_getMsg(code, Msg)`: C copies the message template into the global
/// Msg and returns Msg; the template is returned here and Msg updated too.
fn getMsgIntoMsg(prj: &mut Project, code: i32) -> Vec<u8> {
    let mut msg: Vec<u8> = Vec::new();
    error_getMsg(code, &mut msg);
    cio::strcpy(&mut prj.Msg, &msg);
    msg
}

/// Called by: climate, dynwave, flowrout, gage, gwater, hotstart, iface, kinwave, lid, link, massbal, node, output, project, rain, rdii, routing, runoff, snow, stats, subcatch, toposort, transect, treatmnt.
pub fn report_writeErrorMsg(prj: &mut Project, code: i32, s: &[u8]) {
//
//  Input:   code = error code
//           s = error message text
//  Output:  none
//  Purpose: writes error message to report file.
//
    // C: lid.c passes the global Msg itself as s, and error_getMsg(code, Msg)
    // overwrites Msg with the message template before fprintf reads s, so
    // what C prints for %s there is the template. lid.rs reproduces that by
    // passing the template as s; here s is printed as given.
    if prj.Frpt.file.is_some() {
        WRITE(prj, b"");
        let fmt = getMsgIntoMsg(prj, code);
        cprintf!(prj.Frpt, &fmt, s);
    }
    prj.ErrorCode = code;

    // --- save message to ErrorMsg if it's not for a line of input data
    if prj.ErrorCode <= ERR_INPUT || prj.ErrorCode >= ERR_FILE_NAME {
        let fmt = getMsgIntoMsg(prj, prj.ErrorCode);
        // snprintf(ErrorMsg, MAXMSG, ...): at most MAXMSG-1 bytes + NUL
        let out = cformat!(&fmt, s);
        let n = out.len().min((MAXMSG - 1) as usize);
        prj.ErrorMsg[..n].copy_from_slice(&out[..n]);
        prj.ErrorMsg[n] = 0;
    }
}

//=============================================================================

/// report_writeErrorCode is declared in funcs.h (no caller in 5.2.4).
pub fn report_writeErrorCode(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes error message to report file.
//
    if prj.Frpt.file.is_some() {
        if (prj.ErrorCode >= ERR_MEMORY && prj.ErrorCode <= ERR_TIMESTEP)
            || (prj.ErrorCode >= ERR_FILE_NAME && prj.ErrorCode <= ERR_OUT_FILE)
            || (prj.ErrorCode == ERR_SYSTEM)
        {
            let msg = getMsgIntoMsg(prj, prj.ErrorCode);
            cprintf!(prj.Frpt, b"%s", msg);
        }
    }
}

//=============================================================================

/// Called by: input.
pub fn report_writeInputErrorMsg(prj: &mut Project, k: i32, sect: i32, line: &[u8], lineCount: i64) {
//
//  Input:   k = error code
//           sect = number of input data section where error occurred
//           line = line of data containing the error
//           lineCount = line number of data file containing the error
//  Output:  none
//  Purpose: writes input error message to report file.
//
    if prj.Frpt.file.is_some() {
        let errString = prj.error.ErrString;
        report_writeErrorMsg(prj, k, &errString);
        if sect < 0 {
            cprintf!(prj.Frpt, FMT17, lineCount);
        } else {
            cprintf!(prj.Frpt, FMT18, lineCount, SectWords[sect as usize]);
        }
        cprintf!(prj.Frpt, b"\n  %s", line);
    }
}

//=============================================================================

/// Called by: controls, gage, inlet, link, node, project.
pub fn report_writeWarningMsg(prj: &mut Project, msg: &[u8], id: &[u8]) {
//
//  Input:   msg = text of warning message
//           id = ID name of object that message refers to
//  Output:  none
//  Purpose: writes a warning message to the report file.
//
    cprintf!(prj.Frpt, b"\n  %s %s", msg, id);
    prj.Warnings += 1;
}

//=============================================================================

/// Called by: project.
pub fn report_writeTseriesErrorMsg(prj: &mut Project, code: i32, i: i32) {
//
//  Input:   tseries = pointer to a time series
//  Output:  none
//  Purpose: writes the date where a time series' data is out of order.
//
    // (C receives &Tseries[i]; here the index i)
    let mut theDate: Vec<u8> = Vec::new();
    let mut theTime: Vec<u8> = Vec::new();
    let x: DateTime;

    if code == ERR_CURVE_SEQUENCE {
        x = prj.Tseries[i as usize].x2;
        datetime_dateToStr(prj, x, &mut theDate);
        datetime_timeToStr(x, &mut theTime);
        let id = prj.Tseries[i as usize].ID.clone();
        report_writeErrorMsg(prj, ERR_TIMESERIES_SEQUENCE, &id);
        cprintf!(prj.Frpt, b" at %s %s.", theDate, theTime);
    } else {
        let id = prj.Tseries[i as usize].ID.clone();
        report_writeErrorMsg(prj, code, &id);
    }
}
