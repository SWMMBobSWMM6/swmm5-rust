//! statsrpt.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   statsrpt.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/19/22 (Build 5.2.2)
//   Author:   L. Rossman
//
//   Report writing functions for summary statistics.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - New Groundwater Summary table added.
//   - Reported Max. Depth added to Node Depth Summary table.
//   Build 5.1.009:
//   - Units on column heading in Node Inflow Summary table fixed.
//   Build 5.1.011:
//   - Redundant units conversion on max. reported node depth removed.
//   - Node Surcharge table only produced for dynamic wave routing.
//   Build 5.1.013:
//   - Pervious and impervious runoff added to Subcatchment Runoff Summary.
//   Build 5.1.015:
//   - Fixes bug in summary statistics when Report Start date > Start Date.
//   Build 5.2.0:
//   - Adds a new Street & Inlet Summary table.
//   - Fixes value used for total reporting time.
//   Build 5.2.1:
//   - Replaces the "3" in "ft3" and "m3" with ANSI superscript (\xB3).
//   Build 5.2.2
//   - Calculation of % Evaporation and % Exfiltration losses for storage
//     units was corrected.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Imported variables
//-----------------------------------------------------------------------------
//  SubcatchStats, NodeStats, LinkStats, StorageStats, OutfallStats,
//  PumpStats, MaxOutfallFlow, MaxRunoffFlow, RoutingTimeSpan: prj.stats
//  NodeInflow, NodeOutflow: prj.massbal

/// The file-scope variables of statsrpt.c.
#[derive(Default)]
pub struct StatsrptState {
    FlowFmt: [u8; 6],
    Vcf: f64,
}

/// `#define WRITE(x) (report_writeLine((x)))`
#[inline]
fn WRITE(prj: &mut Project, x: &[u8]) {
    report_writeLine(prj, x);
}

//=============================================================================

/// Called by: stats.
pub fn statsrpt_writeReport(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: reports simulation summary statistics.
//
    // --- set number of decimal places for reporting flow values
    if prj.FlowUnits == MGD || prj.FlowUnits == CMS {
        cio::sstrncpy(&mut prj.statsrpt.FlowFmt, b"%9.3f", 5);
    } else {
        cio::sstrncpy(&mut prj.statsrpt.FlowFmt, b"%9.2f", 5);
    }

    // --- conversion factor from cu. ft. to mil. gallons or megaliters
    if prj.UnitSystem == US {
        prj.statsrpt.Vcf = 7.48 / 1.0e6;
    } else {
        prj.statsrpt.Vcf = 28.317 / 1.0e6;
    }

    // --- report summary results for subcatchment runoff
    if prj.Nobjects[SUBCATCH as usize] > 0 {
        if prj.IgnoreRainfall == 0
            || (prj.Nobjects[SNOWMELT as usize] > 0 && prj.IgnoreSnowmelt == 0)
            || (prj.Nobjects[AQUIFER as usize] > 0 && prj.IgnoreGwater == 0)
        {
            writeSubcatchRunoff(prj);
            lid_writeWaterBalance(prj);
            if prj.IgnoreGwater == 0 {
                writeGroundwater(prj);
            }
            if prj.Nobjects[POLLUT as usize] > 0 && prj.IgnoreQuality == 0 {
                writeSubcatchLoads(prj);
            }
        }
    }

    // --- report summary results for flow routing
    if prj.Nobjects[LINK as usize] > 0 && prj.IgnoreRouting == 0 {
        writeNodeDepths(prj);
        writeNodeFlows(prj);
        if prj.RouteModel == DW {
            writeNodeSurcharge(prj);
        }
        writeNodeFlooding(prj);
        writeStorageVolumes(prj);
        writeOutfallLoads(prj);
        inlet_writeStatsReport(prj);
        writeLinkFlows(prj);
        writeFlowClass(prj);
        writeLinkSurcharge(prj);
        writePumpFlows(prj);
        if prj.Nobjects[POLLUT as usize] > 0 && prj.IgnoreQuality == 0 {
            writeLinkLoads(prj);
        }
    }
}

//=============================================================================

fn writeSubcatchRunoff(prj: &mut Project) {
    let mut j: i32;
    let mut a: f64;
    let mut x: f64;
    let mut r: f64;

    if prj.Nobjects[SUBCATCH as usize] == 0 {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"***************************");
    WRITE(prj, b"Subcatchment Runoff Summary");
    WRITE(prj, b"***************************");
    WRITE(prj, b"");
    cprintf!(prj.Frpt,
b"\n  ------------------------------------------------------------------------------------------------------------------------------\
\n                            Total      Total      Total      Total     Imperv       Perv      Total       Total     Peak  Runoff\
\n                           Precip      Runon       Evap      Infil     Runoff     Runoff     Runoff      Runoff   Runoff   Coeff");
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
b"\n  Subcatchment                 in         in         in         in         in         in         in    %8s      %3s",
            VolUnitsWords[prj.UnitSystem as usize], FlowUnitWords[prj.FlowUnits as usize]);
    } else {
        cprintf!(prj.Frpt,
b"\n  Subcatchment                 mm         mm         mm         mm         mm         mm         mm    %8s      %3s",
            VolUnitsWords[prj.UnitSystem as usize], FlowUnitWords[prj.FlowUnits as usize]);
    }
    cprintf!(prj.Frpt,
b"\n  ------------------------------------------------------------------------------------------------------------------------------");

    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        let ju = j as usize;
        a = prj.Subcatch[ju].area;
        if a == 0.0 {
            j += 1;
            continue;
        }
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Subcatch[ju].ID);
        let ss = prj.stats.SubcatchStats[ju];
        let vcf = prj.statsrpt.Vcf;
        x = ss.precip * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b" %10.2f", x / a);
        x = ss.runon * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b" %10.2f", x / a);
        x = ss.evap * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b" %10.2f", x / a);
        x = ss.infil * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b" %10.2f", x / a);
        x = ss.impervRunoff * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b" %10.2f", x / a);
        x = ss.pervRunoff * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b" %10.2f", x / a);
        x = ss.runoff * UCF(prj, RAINDEPTH);
        cprintf!(prj.Frpt, b" %10.2f", x / a);
        x = ss.runoff * vcf;
        cprintf!(prj.Frpt, b"%12.2f", x);
        x = ss.maxFlow * UCF(prj, FLOW);
        cprintf!(prj.Frpt, b" %8.2f", x);
        r = ss.precip + ss.runon;
        if r > 0.0 {
            r = ss.runoff / r;
        }
        cprintf!(prj.Frpt, b"%8.3f", r);
        j += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeGroundwater(prj: &mut Project) {
    let mut i: i32;
    let mut j: i32;
    let mut count: i32 = 0;
    let totalSeconds: f64 = prj.NewRunoffTime / 1000.0;
    let mut x: [f64; 9] = [0.0; 9];

    if prj.Nobjects[SUBCATCH as usize] == 0 {
        return;
    }
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        if prj.Subcatch[j as usize].groundwater.is_some() {
            count += 1;
        }
        j += 1;
    }
    if count == 0 {
        return;
    }

    WRITE(prj, b"");
    WRITE(prj, b"*******************");
    WRITE(prj, b"Groundwater Summary");
    WRITE(prj, b"*******************");
    WRITE(prj, b"");
    cprintf!(prj.Frpt,
b"\n  -----------------------------------------------------------------------------------------------------\
\n                                            Total    Total  Maximum  Average  Average    Final    Final\
\n                          Total    Total    Lower  Lateral  Lateral    Upper    Water    Upper    Water\
\n                          Infil     Evap  Seepage  Outflow  Outflow   Moist.    Table   Moist.    Table");
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
b"\n  Subcatchment               in       in       in       in      %3s                ft                ft",
            FlowUnitWords[prj.FlowUnits as usize]);
    } else {
        cprintf!(prj.Frpt,
b"\n  Subcatchment               mm       mm       mm       mm      %3s                 m                 m",
            FlowUnitWords[prj.FlowUnits as usize]);
    }
    cprintf!(prj.Frpt,
b"\n  -----------------------------------------------------------------------------------------------------");

    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        let ju = j as usize;
        if prj.Subcatch[ju].area == 0.0 || prj.Subcatch[ju].groundwater.is_none() {
            j += 1;
            continue;
        }
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Subcatch[ju].ID);
        let st = prj.Subcatch[ju].groundwater.as_ref().unwrap().stats;
        let area = prj.Subcatch[ju].area;
        x[0] = st.infil * UCF(prj, RAINDEPTH);
        x[1] = st.evap * UCF(prj, RAINDEPTH);
        x[2] = st.deepFlow * UCF(prj, RAINDEPTH);
        x[3] = st.latFlow * UCF(prj, RAINDEPTH);
        x[4] = st.maxFlow * UCF(prj, FLOW) * area;
        x[5] = st.avgUpperMoist / totalSeconds;
        x[6] = st.avgWaterTable * UCF(prj, LENGTH) / totalSeconds;
        x[7] = st.finalUpperMoist;
        x[8] = st.finalWaterTable * UCF(prj, LENGTH);
        i = 0;
        while i < 9 {
            cprintf!(prj.Frpt, b" %8.2f", x[i as usize]);
            i += 1;
        }
        j += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeSubcatchLoads(prj: &mut Project) {
    let mut i: i32;
    let mut j: i32;
    let mut p: i32;
    let mut x: f64;
    let mut totals: Vec<f64>;
    let mut units: [u8; 15] = [0; 15];
    let subcatchLine: &[u8] = b"--------------------";
    let pollutLine: &[u8] = b"--------------";
    let npollut = prj.Nobjects[POLLUT as usize];

    // --- create an array to hold total loads for each pollutant
    totals = vec![0.0; npollut as usize];
    {
        // --- print the table headings
        WRITE(prj, b"");
        WRITE(prj, b"****************************");
        WRITE(prj, b"Subcatchment Washoff Summary");
        WRITE(prj, b"****************************");
        WRITE(prj, b"");
        cprintf!(prj.Frpt, b"\n  %s", subcatchLine);
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"%s", pollutLine);
            p += 1;
        }
        cprintf!(prj.Frpt, b"\n                      ");
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"%14s", prj.Pollut[p as usize].ID);
            p += 1;
        }
        cprintf!(prj.Frpt, b"\n  Subcatchment        ");
        p = 0;
        while p < npollut {
            i = prj.UnitSystem;
            if prj.Pollut[p as usize].units == COUNT {
                i = 2;
            }
            cio::sstrncpy(&mut units, LoadUnitsWords[i as usize], 14);
            cprintf!(prj.Frpt, b"%14s", units);
            totals[p as usize] = 0.0;
            p += 1;
        }
        cprintf!(prj.Frpt, b"\n  %s", subcatchLine);
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"%s", pollutLine);
            p += 1;
        }

        // --- print the pollutant loadings from each subcatchment
        j = 0;
        while j < prj.Nobjects[SUBCATCH as usize] {
            let ju = j as usize;
            cprintf!(prj.Frpt, b"\n  %-20s", prj.Subcatch[ju].ID);
            p = 0;
            while p < npollut {
                x = prj.Subcatch[ju].totalLoad[p as usize];
                totals[p as usize] += x;
                if prj.Pollut[p as usize].units == COUNT {
                    x = LOG10(x);
                }
                cprintf!(prj.Frpt, b"%14.3f", x);
                p += 1;
            }
            j += 1;
        }

        // --- print the total loading of each pollutant
        cprintf!(prj.Frpt, b"\n  %s", subcatchLine);
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"%s", pollutLine);
            p += 1;
        }
        cprintf!(prj.Frpt, b"\n  System              ");
        p = 0;
        while p < npollut {
            x = totals[p as usize];
            if prj.Pollut[p as usize].units == COUNT {
                x = LOG10(x);
            }
            cprintf!(prj.Frpt, b"%14.3f", x);
            p += 1;
        }
        drop(totals);
        WRITE(prj, b"");
    }
}

//=============================================================================

fn writeNodeDepths(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes simulation statistics for nodes to report file.
//
    let mut j: i32;
    let mut days: i32 = 0;
    let mut hrs: i32 = 0;
    let mut mins: i32 = 0;
    if prj.Nobjects[LINK as usize] == 0 {
        return;
    }

    WRITE(prj, b"");
    WRITE(prj, b"******************");
    WRITE(prj, b"Node Depth Summary");
    WRITE(prj, b"******************");
    WRITE(prj, b"");

    cprintf!(prj.Frpt,
b"\n  ---------------------------------------------------------------------------------\
\n                                 Average  Maximum  Maximum  Time of Max    Reported\
\n                                   Depth    Depth      HGL   Occurrence   Max Depth");
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
b"\n  Node                 Type         Feet     Feet     Feet  days hr:min        Feet");
    } else {
        cprintf!(prj.Frpt,
b"\n  Node                 Type       Meters   Meters   Meters  days hr:min      Meters");
    }
    cprintf!(prj.Frpt,
b"\n  ---------------------------------------------------------------------------------");

    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Node[ju].ID);
        cprintf!(prj.Frpt, b" %-9s ", NodeTypeWords[prj.Node[ju].type_ as usize]);
        let ns = prj.stats.NodeStats[ju];
        getElapsedTime(prj, ns.maxDepthDate, &mut days, &mut hrs, &mut mins);
        let v1 = ns.avgDepth / prj.ReportStepCount as f64 * UCF(prj, LENGTH);
        let v2 = ns.maxDepth * UCF(prj, LENGTH);
        let v3 = (ns.maxDepth + prj.Node[ju].invertElev) * UCF(prj, LENGTH);
        cprintf!(prj.Frpt, b"%7.2f  %7.2f  %7.2f  %4d  %02d:%02d  %10.2f",
            v1, v2, v3, days, hrs, mins, ns.maxRptDepth);
        j += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeNodeFlows(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes flow statistics for nodes to report file.
//
    let mut j: i32;
    let mut days1: i32 = 0;
    let mut hrs1: i32 = 0;
    let mut mins1: i32 = 0;

    WRITE(prj, b"");
    WRITE(prj, b"*******************");
    WRITE(prj, b"Node Inflow Summary");
    WRITE(prj, b"*******************");
    WRITE(prj, b"");

    cprintf!(prj.Frpt,
b"\n  -------------------------------------------------------------------------------------------------\
\n                                  Maximum  Maximum                  Lateral       Total        Flow\
\n                                  Lateral    Total  Time of Max      Inflow      Inflow     Balance\
\n                                   Inflow   Inflow   Occurrence      Volume      Volume       Error\
\n  Node                 Type           %3s      %3s  days hr:min    %8s    %8s     Percent",
        FlowUnitWords[prj.FlowUnits as usize], FlowUnitWords[prj.FlowUnits as usize],
        VolUnitsWords[prj.UnitSystem as usize], VolUnitsWords[prj.UnitSystem as usize]);
    cprintf!(prj.Frpt,
b"\n  -------------------------------------------------------------------------------------------------");

    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        let flowFmt = prj.statsrpt.FlowFmt;
        let vcf = prj.statsrpt.Vcf;
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Node[ju].ID);
        cprintf!(prj.Frpt, b" %-9s", NodeTypeWords[prj.Node[ju].type_ as usize]);
        let ns = prj.stats.NodeStats[ju];
        getElapsedTime(prj, ns.maxInflowDate, &mut days1, &mut hrs1, &mut mins1);
        let v = ns.maxLatFlow * UCF(prj, FLOW);
        cprintf!(prj.Frpt, &flowFmt, v);
        let v = ns.maxInflow * UCF(prj, FLOW);
        cprintf!(prj.Frpt, &flowFmt, v);
        cprintf!(prj.Frpt, b"  %4d  %02d:%02d", days1, hrs1, mins1);
        cprintf!(prj.Frpt, b"%12.3g", ns.totLatFlow * vcf);
        let nodeInflow = prj.massbal.NodeInflow[ju];
        let nodeOutflow = prj.massbal.NodeOutflow[ju];
        cprintf!(prj.Frpt, b"%12.3g", nodeInflow * vcf);
        if nodeOutflow.abs() < 1.0 {
            cprintf!(prj.Frpt, b"%12.3f %s",
                (nodeInflow - nodeOutflow) * vcf * 1.0e6,
                VolUnitsWords2[prj.UnitSystem as usize]);
        } else {
            cprintf!(prj.Frpt, b"%12.3f", (nodeInflow - nodeOutflow) / nodeOutflow * 100.0);
        }
        j += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeNodeSurcharge(prj: &mut Project) {
    let mut j: i32;
    let mut n: i32 = 0;
    let mut t: f64;
    let mut d1: f64;
    let mut d2: f64;

    WRITE(prj, b"");
    WRITE(prj, b"**********************");
    WRITE(prj, b"Node Surcharge Summary");
    WRITE(prj, b"**********************");
    WRITE(prj, b"");

    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        if prj.Node[ju].type_ == OUTFALL {
            j += 1;
            continue;
        }
        if prj.stats.NodeStats[ju].timeSurcharged == 0.0 {
            j += 1;
            continue;
        }
        t = MAX(0.01, prj.stats.NodeStats[ju].timeSurcharged / 3600.0);
        if n == 0 {
            WRITE(prj, b"Surcharging occurs when water rises above the top of the highest conduit.");
            cprintf!(prj.Frpt,
b"\n  ---------------------------------------------------------------------\
\n                                               Max. Height   Min. Depth\
\n                                   Hours       Above Crown    Below Rim");
            if prj.UnitSystem == US {
                cprintf!(prj.Frpt,
b"\n  Node                 Type      Surcharged           Feet         Feet");
            } else {
                cprintf!(prj.Frpt,
b"\n  Node                 Type      Surcharged         Meters       Meters");
            }
            cprintf!(prj.Frpt,
b"\n  ---------------------------------------------------------------------");
            n = 1;
        }
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Node[ju].ID);
        cprintf!(prj.Frpt, b" %-9s", NodeTypeWords[prj.Node[ju].type_ as usize]);
        let maxDepth = prj.stats.NodeStats[ju].maxDepth;
        d1 = maxDepth + prj.Node[ju].invertElev - prj.Node[ju].crownElev;
        if d1 < 0.0 {
            d1 = 0.0;
        }
        d2 = prj.Node[ju].fullDepth - maxDepth;
        if d2 < 0.0 {
            d2 = 0.0;
        }
        let v1 = d1 * UCF(prj, LENGTH);
        let v2 = d2 * UCF(prj, LENGTH);
        cprintf!(prj.Frpt, b"  %9.2f      %9.3f    %9.3f", t, v1, v2);
        j += 1;
    }
    if n == 0 {
        WRITE(prj, b"No nodes were surcharged.");
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeNodeFlooding(prj: &mut Project) {
    let mut j: i32;
    let mut n: i32 = 0;
    let mut days: i32 = 0;
    let mut hrs: i32 = 0;
    let mut mins: i32 = 0;
    let mut t: f64;

    WRITE(prj, b"");
    WRITE(prj, b"*********************");
    WRITE(prj, b"Node Flooding Summary");
    WRITE(prj, b"*********************");
    WRITE(prj, b"");

    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        let ju = j as usize;
        if prj.Node[ju].type_ == OUTFALL {
            j += 1;
            continue;
        }
        if prj.stats.NodeStats[ju].timeFlooded == 0.0 {
            j += 1;
            continue;
        }
        t = MAX(0.01, prj.stats.NodeStats[ju].timeFlooded / 3600.0);

        if n == 0 {
            WRITE(prj, b"Flooding refers to all water that overflows a node, whether it ponds or not.");
            cprintf!(prj.Frpt,
b"\n  --------------------------------------------------------------------------\
\n                                                             Total   Maximum\
\n                                 Maximum   Time of Max       Flood    Ponded\
\n                        Hours       Rate    Occurrence      Volume");
            if prj.RouteModel == DW {
                cprintf!(prj.Frpt, b"     Depth");
            } else {
                cprintf!(prj.Frpt, b"    Volume");
            }
            cprintf!(prj.Frpt,
b"\n  Node                 Flooded       %3s   days hr:min    %8s",
                FlowUnitWords[prj.FlowUnits as usize], VolUnitsWords[prj.UnitSystem as usize]);
            if prj.RouteModel == DW {
                cprintf!(prj.Frpt, b"    %6s", PondingUnitsWords[prj.UnitSystem as usize]);
            } else if prj.UnitSystem == US {
                cprintf!(prj.Frpt, b"  1000 ft\xB3");
            } else {
                cprintf!(prj.Frpt, b"   1000 m\xB3");
            }
            cprintf!(prj.Frpt,
b"\n  --------------------------------------------------------------------------");
            n = 1;
        }
        let flowFmt = prj.statsrpt.FlowFmt;
        let vcf = prj.statsrpt.Vcf;
        let ns = prj.stats.NodeStats[ju];
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Node[ju].ID);
        cprintf!(prj.Frpt, b" %7.2f ", t);
        let v = ns.maxOverflow * UCF(prj, FLOW);
        cprintf!(prj.Frpt, &flowFmt, v);
        getElapsedTime(prj, ns.maxOverflowDate, &mut days, &mut hrs, &mut mins);
        cprintf!(prj.Frpt, b"   %4d  %02d:%02d", days, hrs, mins);
        cprintf!(prj.Frpt, b"%12.3f", ns.volFlooded * vcf);
        if prj.RouteModel == DW {
            let v = (ns.maxDepth - prj.Node[ju].fullDepth) * UCF(prj, LENGTH);
            cprintf!(prj.Frpt, b" %9.3f", v);
        } else {
            let v = ns.maxPondedVol / 1000.0 * UCF(prj, VOLUME);
            cprintf!(prj.Frpt, b" %9.3f", v);
        }
        j += 1;
    }

    if n == 0 {
        WRITE(prj, b"No nodes were flooded.");
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeStorageVolumes(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes simulation statistics for storage units to report file.
//
    let mut j: i32;
    let mut k: i32;
    let mut days: i32 = 0;
    let mut hrs: i32 = 0;
    let mut mins: i32 = 0;
    let mut avgVol: f64;
    let mut maxVol: f64;
    let mut pctAvgVol: f64;
    let mut pctMaxVol: f64;
    let mut pctEvapLoss: f64;
    let mut pctSeepLoss: f64;

    if prj.Nnodes[STORAGE as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"**********************");
        WRITE(prj, b"Storage Volume Summary");
        WRITE(prj, b"**********************");
        WRITE(prj, b"");

        cprintf!(prj.Frpt,
b"\n  ------------------------------------------------------------------------------------------------\
\n                         Average    Avg   Evap  Exfil     Maximum    Max    Time of Max    Maximum\
\n                          Volume   Pcnt   Pcnt   Pcnt      Volume   Pcnt     Occurrence    Outflow");
        if prj.UnitSystem == US {
            cprintf!(prj.Frpt,
b"\n  Storage Unit          1000 ft\xB3   Full   Loss   Loss    1000 ft\xB3   Full    days hr:min        ");
        } else {
            cprintf!(prj.Frpt,
b"\n  Storage Unit           1000 m\xB3   Full   Loss   Loss     1000 m\xB3   Full    days hr:min        ");
        }
        cprintf!(prj.Frpt, b"%3s", FlowUnitWords[prj.FlowUnits as usize]);
        cprintf!(prj.Frpt,
b"\n  ------------------------------------------------------------------------------------------------");

        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            let ju = j as usize;
            if prj.Node[ju].type_ != STORAGE {
                j += 1;
                continue;
            }
            k = prj.Node[ju].subIndex;
            let ss = prj.stats.StorageStats[k as usize];
            cprintf!(prj.Frpt, b"\n  %-20s", prj.Node[ju].ID);
            avgVol = ss.avgVol / prj.ReportStepCount as f64;
            maxVol = ss.maxVol;
            pctMaxVol = 0.0;
            pctAvgVol = 0.0;
            if prj.Node[ju].fullVolume > 0.0 {
                pctAvgVol = avgVol / prj.Node[ju].fullVolume * 100.0;
                pctMaxVol = maxVol / prj.Node[ju].fullVolume * 100.0;
            }
            pctEvapLoss = 0.0;
            pctSeepLoss = 0.0;
            let nodeInflow = prj.massbal.NodeInflow[ju];
            if nodeInflow > 0.0 {
                pctEvapLoss = ss.evapLosses / nodeInflow * 100.0;
                pctSeepLoss = ss.exfilLosses / nodeInflow * 100.0;
            }

            let v1 = avgVol * UCF(prj, VOLUME) / 1000.0;
            let v2 = maxVol * UCF(prj, VOLUME) / 1000.0;
            cprintf!(prj.Frpt, b"%10.3f  %5.1f  %5.1f  %5.1f  %10.3f  %5.1f",
                v1, pctAvgVol, pctEvapLoss, pctSeepLoss, v2, pctMaxVol);

            getElapsedTime(prj, ss.maxVolDate, &mut days, &mut hrs, &mut mins);
            cprintf!(prj.Frpt, b"    %4d  %02d:%02d  ", days, hrs, mins);
            let flowFmt = prj.statsrpt.FlowFmt;
            let v = ss.maxFlow * UCF(prj, FLOW);
            cprintf!(prj.Frpt, &flowFmt, v);
            j += 1;
        }
        WRITE(prj, b"");
    }
}

//=============================================================================

fn writeOutfallLoads(prj: &mut Project) {
//
//  Input:   node
//  Output:  none
//  Purpose: writes simulation statistics for outfall nodess to report file.
//
    let mut units: [u8; 15] = [0; 15];
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut p: i32;
    let mut x: f64;
    let outfallCount: f64;
    let mut flowCount: f64;
    let mut flowSum: f64;
    let mut freqSum: f64;
    let mut volSum: f64;
    let mut totals: Vec<f64>;
    let npollut = prj.Nobjects[POLLUT as usize];

    if prj.Nnodes[OUTFALL as usize] > 0 {
        // --- initial totals
        totals = vec![0.0; npollut as usize];
        p = 0;
        while p < npollut {
            totals[p as usize] = 0.0;
            p += 1;
        }
        flowSum = 0.0;
        freqSum = 0.0;
        volSum = 0.0;

        // --- print table title
        WRITE(prj, b"");
        WRITE(prj, b"***********************");
        WRITE(prj, b"Outfall Loading Summary");
        WRITE(prj, b"***********************");
        WRITE(prj, b"");

        // --- print table column headers
        cprintf!(prj.Frpt,
 b"\n  -----------------------------------------------------------");
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"--------------");
            p += 1;
        }
        cprintf!(prj.Frpt,
 b"\n                         Flow       Avg       Max       Total");
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"         Total");
            p += 1;
        }
        cprintf!(prj.Frpt,
 b"\n                         Freq      Flow      Flow      Volume");
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"%14s", prj.Pollut[p as usize].ID);
            p += 1;
        }
        cprintf!(prj.Frpt,
 b"\n  Outfall Node           Pcnt       %3s       %3s    %8s",
            FlowUnitWords[prj.FlowUnits as usize], FlowUnitWords[prj.FlowUnits as usize],
            VolUnitsWords[prj.UnitSystem as usize]);
        p = 0;
        while p < npollut {
            i = prj.UnitSystem;
            if prj.Pollut[p as usize].units == COUNT {
                i = 2;
            }
            cio::sstrncpy(&mut units, LoadUnitsWords[i as usize], 14);
            cprintf!(prj.Frpt, b"%14s", units);
            p += 1;
        }
        cprintf!(prj.Frpt,
 b"\n  -----------------------------------------------------------");
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"--------------");
            p += 1;
        }

        // --- identify each outfall node
        j = 0;
        while j < prj.Nobjects[NODE as usize] {
            let ju = j as usize;
            if prj.Node[ju].type_ != OUTFALL {
                j += 1;
                continue;
            }
            k = prj.Node[ju].subIndex;
            let ku = k as usize;
            flowCount = prj.stats.OutfallStats[ku].totalPeriods as f64;
            let flowFmt = prj.statsrpt.FlowFmt;
            let vcf = prj.statsrpt.Vcf;

            // --- print node ID, flow freq., avg. flow, max. flow & flow vol.
            cprintf!(prj.Frpt, b"\n  %-20s", prj.Node[ju].ID);
            x = 100.0 * flowCount / prj.ReportStepCount as f64;
            cprintf!(prj.Frpt, b"%7.2f", x);
            freqSum += x;
            if flowCount > 0.0 {
                x = prj.stats.OutfallStats[ku].avgFlow * UCF(prj, FLOW) / flowCount;
            } else {
                x = 0.0;
            }
            flowSum += x;

            cprintf!(prj.Frpt, b" ");
            cprintf!(prj.Frpt, &flowFmt, x);
            cprintf!(prj.Frpt, b" ");
            let v = prj.stats.OutfallStats[ku].maxFlow * UCF(prj, FLOW);
            cprintf!(prj.Frpt, &flowFmt, v);
            let nodeInflow = prj.massbal.NodeInflow[ju];
            cprintf!(prj.Frpt, b"%12.3f", nodeInflow * vcf);
            volSum += nodeInflow;

            // --- print load of each pollutant for outfall
            p = 0;
            while p < npollut {
                x = prj.stats.OutfallStats[ku].totalLoad[p as usize] * LperFT3 * prj.Pollut[p as usize].mcf;
                totals[p as usize] += x;
                if prj.Pollut[p as usize].units == COUNT {
                    x = LOG10(x);
                }
                cprintf!(prj.Frpt, b"%14.3f", x);
                p += 1;
            }
            j += 1;
        }

        // --- print total outfall loads
        outfallCount = prj.Nnodes[OUTFALL as usize] as f64;
        cprintf!(prj.Frpt,
 b"\n  -----------------------------------------------------------");
        p = 0;
        while p < npollut {
            cprintf!(prj.Frpt, b"--------------");
            p += 1;
        }

        let flowFmt = prj.statsrpt.FlowFmt;
        let vcf = prj.statsrpt.Vcf;
        cprintf!(prj.Frpt, b"\n  System              %7.2f ", freqSum / outfallCount);
        cprintf!(prj.Frpt, &flowFmt, flowSum);
        cprintf!(prj.Frpt, b" ");
        let v = prj.stats.MaxOutfallFlow * UCF(prj, FLOW);
        cprintf!(prj.Frpt, &flowFmt, v);
        cprintf!(prj.Frpt, b"%12.3f", volSum * vcf);

        p = 0;
        while p < npollut {
            x = totals[p as usize];
            if prj.Pollut[p as usize].units == COUNT {
                x = LOG10(x);
            }
            cprintf!(prj.Frpt, b"%14.3f", x);
            p += 1;
        }
        WRITE(prj, b"");
        drop(totals);
    }
}

//=============================================================================

fn writeLinkFlows(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes simulation statistics for links to report file.
//
    let mut j: i32;
    let mut k: i32;
    let mut days: i32 = 0;
    let mut hrs: i32 = 0;
    let mut mins: i32 = 0;
    let mut v: f64;
    let mut fullDepth: f64;

    if prj.Nobjects[LINK as usize] == 0 {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"********************");
    WRITE(prj, b"Link Flow Summary");
    WRITE(prj, b"********************");
    WRITE(prj, b"");

    cprintf!(prj.Frpt,
        b"\n  -----------------------------------------------------------------------------\
\n                                 Maximum  Time of Max   Maximum    Max/    Max/\
\n                                  |Flow|   Occurrence   |Veloc|    Full    Full");
    if prj.UnitSystem == US {
        cprintf!(prj.Frpt,
        b"\n  Link                 Type          %3s  days hr:min    ft/sec    Flow   Depth",
            FlowUnitWords[prj.FlowUnits as usize]);
    } else {
        cprintf!(prj.Frpt,
        b"\n  Link                 Type          %3s  days hr:min     m/sec    Flow   Depth",
            FlowUnitWords[prj.FlowUnits as usize]);
    }
    cprintf!(prj.Frpt,
        b"\n  -----------------------------------------------------------------------------");

    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        let flowFmt = prj.statsrpt.FlowFmt;
        let ls = prj.stats.LinkStats[ju];

        // --- print link ID
        k = prj.Link[ju].subIndex;
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Link[ju].ID);

        // --- print link type
        if prj.Link[ju].xsect.type_ == DUMMY {
            cprintf!(prj.Frpt, b" DUMMY   ");
        } else if prj.Link[ju].xsect.type_ == IRREGULAR {
            cprintf!(prj.Frpt, b" CHANNEL ");
        } else {
            cprintf!(prj.Frpt, b" %-7s ", LinkTypeWords[prj.Link[ju].type_ as usize]);
        }

        // --- print max. flow & time of occurrence
        getElapsedTime(prj, ls.maxFlowDate, &mut days, &mut hrs, &mut mins);
        let vq = ls.maxFlow * UCF(prj, FLOW);
        cprintf!(prj.Frpt, &flowFmt, vq);
        cprintf!(prj.Frpt, b"  %4d  %02d:%02d", days, hrs, mins);

        // --- print max flow / flow capacity for pumps
        if prj.Link[ju].type_ == PUMP && prj.Link[ju].qFull > 0.0 {
            cprintf!(prj.Frpt, b"          ");
            cprintf!(prj.Frpt, b"  %6.2f", ls.maxFlow / prj.Link[ju].qFull);
            j += 1;
            continue;
        }

        // --- stop printing for dummy conduits
        if prj.Link[ju].xsect.type_ == DUMMY {
            j += 1;
            continue;
        }

        // --- stop printing for outlet links (since they don't have xsections)
        if prj.Link[ju].type_ == OUTLET {
            j += 1;
            continue;
        }

        // --- print max velocity & max/full flow for conduits
        if prj.Link[ju].type_ == CONDUIT {
            v = ls.maxVeloc * UCF(prj, LENGTH);
            if v > 50.0 {
                cprintf!(prj.Frpt, b"    >50.00");
            } else {
                cprintf!(prj.Frpt, b"   %7.2f", v);
            }
            cprintf!(prj.Frpt, b"  %6.2f",
                ls.maxFlow / prj.Link[ju].qFull / prj.Conduit[k as usize].barrels as f64);
        } else {
            cprintf!(prj.Frpt, b"                  ");
        }

        // --- print max/full depth
        fullDepth = prj.Link[ju].xsect.yFull;
        // (C reads Orifice[k] only when the link is an orifice)
        if prj.Link[ju].type_ == ORIFICE && prj.Orifice[k as usize].type_ == BOTTOM_ORIFICE {
            fullDepth = 0.0;
        }
        if fullDepth > 0.0 {
            cprintf!(prj.Frpt, b"  %6.2f", ls.maxDepth / fullDepth);
        } else {
            cprintf!(prj.Frpt, b"        ");
        }
        j += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeFlowClass(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes flow classification for each conduit to report file.
//
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let totalSeconds: f64 = prj.stats.RoutingTimeSpan;

    if prj.RouteModel != DW {
        return;
    }
    WRITE(prj, b"");
    WRITE(prj, b"***************************");
    WRITE(prj, b"Flow Classification Summary");
    WRITE(prj, b"***************************");
    WRITE(prj, b"");
    cprintf!(prj.Frpt,
b"\n  -------------------------------------------------------------------------------------\
\n                      Adjusted    ---------- Fraction of Time in Flow Class ---------- \
\n                       /Actual         Up    Down  Sub   Sup   Up    Down  Norm  Inlet \
\n  Conduit               Length    Dry  Dry   Dry   Crit  Crit  Crit  Crit  Ltd   Ctrl  \
\n  -------------------------------------------------------------------------------------");
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        if prj.Link[ju].type_ != CONDUIT {
            j += 1;
            continue;
        }
        if prj.Link[ju].xsect.type_ == DUMMY {
            j += 1;
            continue;
        }
        k = prj.Link[ju].subIndex;
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Link[ju].ID);
        cprintf!(prj.Frpt, b"  %6.2f ",
            prj.Conduit[k as usize].modLength / prj.Conduit[k as usize].length);
        i = 0;
        while i < MAX_FLOW_CLASSES {
            // (C divides the stored statistic in place)
            prj.stats.LinkStats[ju].timeInFlowClass[i as usize] /= totalSeconds; //5.2
                                                    //(double)ReportStepCount);
            cprintf!(prj.Frpt, b"  %4.2f", prj.stats.LinkStats[ju].timeInFlowClass[i as usize]);
            i += 1;
        }
        cprintf!(prj.Frpt, b"  %4.2f", prj.stats.LinkStats[ju].timeNormalFlow / totalSeconds);
        cprintf!(prj.Frpt, b"  %4.2f", prj.stats.LinkStats[ju].timeInletControl / totalSeconds);
        j += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeLinkSurcharge(prj: &mut Project) {
    let mut i: i32;
    let mut j: i32;
    let mut n: i32 = 0;
    let mut t: [f64; 5] = [0.0; 5];

    WRITE(prj, b"");
    WRITE(prj, b"*************************");
    WRITE(prj, b"Conduit Surcharge Summary");
    WRITE(prj, b"*************************");
    WRITE(prj, b"");
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        if prj.Link[ju].type_ != CONDUIT || prj.Link[ju].xsect.type_ == DUMMY {
            j += 1;
            continue;
        }
        let ls = prj.stats.LinkStats[ju];
        t[0] = ls.timeSurcharged / 3600.0;
        t[1] = ls.timeFullUpstream / 3600.0;
        t[2] = ls.timeFullDnstream / 3600.0;
        t[3] = ls.timeFullFlow / 3600.0;
        if t[0] + t[1] + t[2] + t[3] == 0.0 {
            j += 1;
            continue;
        }
        t[4] = ls.timeCapacityLimited / 3600.0;
        i = 0;
        while i < 5 {
            t[i as usize] = MAX(0.01, t[i as usize]);
            i += 1;
        }
        if n == 0 {
            cprintf!(prj.Frpt,
b"\n  ----------------------------------------------------------------------------\
\n                                                           Hours        Hours \
\n                         --------- Hours Full --------   Above Full   Capacity\
\n  Conduit                Both Ends  Upstream  Dnstream   Normal Flow   Limited\
\n  ----------------------------------------------------------------------------");
            n = 1;
        }
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Link[ju].ID);
        cprintf!(prj.Frpt, b"    %8.2f  %8.2f  %8.2f  %8.2f     %8.2f",
            t[0], t[1], t[2], t[3], t[4]);
        j += 1;
    }
    if n == 0 {
        WRITE(prj, b"No conduits were surcharged.");
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writePumpFlows(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: writes simulation statistics for pumps to report file.
//
    let mut j: i32;
    let mut k: i32;
    let mut avgFlow: f64;
    let mut pctUtilized: f64;
    let mut pctOffCurve1: f64;
    let mut pctOffCurve2: f64;
    let totalSeconds: f64 = prj.stats.RoutingTimeSpan;

    if prj.Nlinks[PUMP as usize] == 0 {
        return;
    }

    WRITE(prj, b"");
    WRITE(prj, b"***************");
    WRITE(prj, b"Pumping Summary");
    WRITE(prj, b"***************");
    WRITE(prj, b"");

    cprintf!(prj.Frpt,
b"\n  ---------------------------------------------------------------------------------------------------------\
\n                                                  Min       Avg       Max     Total     Power    %% Time Off\
\n                        Percent   Number of      Flow      Flow      Flow    Volume     Usage    Pump Curve\
\n  Pump                 Utilized   Start-Ups       %3s       %3s       %3s  %8s     Kw-hr    Low   High\
\n  ---------------------------------------------------------------------------------------------------------",
        FlowUnitWords[prj.FlowUnits as usize], FlowUnitWords[prj.FlowUnits as usize],
        FlowUnitWords[prj.FlowUnits as usize], VolUnitsWords[prj.UnitSystem as usize]);
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        if prj.Link[ju].type_ != PUMP {
            j += 1;
            continue;
        }
        k = prj.Link[ju].subIndex;
        let ps = prj.stats.PumpStats[k as usize];
        let vcf = prj.statsrpt.Vcf;
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Link[ju].ID);
        pctUtilized = ps.utilized / totalSeconds * 100.0;
        avgFlow = ps.avgFlow;
        if ps.totalPeriods > 0 {
            avgFlow /= ps.totalPeriods as f64;
        }
        let v1 = ps.minFlow * UCF(prj, FLOW);
        let v2 = avgFlow * UCF(prj, FLOW);
        let v3 = ps.maxFlow * UCF(prj, FLOW);
        cprintf!(prj.Frpt, b" %8.2f  %10d %9.2f %9.2f %9.2f %9.3f %9.2f",
            pctUtilized, ps.startUps, v1, v2, v3, ps.volume * vcf, ps.energy);
        pctOffCurve1 = ps.offCurveLow;
        pctOffCurve2 = ps.offCurveHigh;
        if ps.utilized > 0.0 {
            pctOffCurve1 = pctOffCurve1 / ps.utilized * 100.0;
            pctOffCurve2 = pctOffCurve2 / ps.utilized * 100.0;
        }
        cprintf!(prj.Frpt, b" %6.1f %6.1f", pctOffCurve1, pctOffCurve2);
        j += 1;
    }
    WRITE(prj, b"");
}

//=============================================================================

fn writeLinkLoads(prj: &mut Project) {
    let mut i: i32;
    let mut j: i32;
    let mut p: i32;
    let mut x: f64;
    let mut units: [u8; 15] = [0; 15];
    let linkLine: &[u8] = b"--------------------";
    let pollutLine: &[u8] = b"--------------";
    let npollut = prj.Nobjects[POLLUT as usize];

    // --- print the table headings
    WRITE(prj, b"");
    WRITE(prj, b"***************************");
    WRITE(prj, b"Link Pollutant Load Summary");
    WRITE(prj, b"***************************");
    WRITE(prj, b"");
    cprintf!(prj.Frpt, b"\n  %s", linkLine);
    p = 0;
    while p < npollut {
        cprintf!(prj.Frpt, b"%s", pollutLine);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n                      ");
    p = 0;
    while p < npollut {
        cprintf!(prj.Frpt, b"%14s", prj.Pollut[p as usize].ID);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  Link                ");
    p = 0;
    while p < npollut {
        i = prj.UnitSystem;
        if prj.Pollut[p as usize].units == COUNT {
            i = 2;
        }
        cio::sstrncpy(&mut units, LoadUnitsWords[i as usize], 14);
        cprintf!(prj.Frpt, b"%14s", units);
        p += 1;
    }
    cprintf!(prj.Frpt, b"\n  %s", linkLine);
    p = 0;
    while p < npollut {
        cprintf!(prj.Frpt, b"%s", pollutLine);
        p += 1;
    }

    // --- print the pollutant loadings carried by each link
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        let ju = j as usize;
        cprintf!(prj.Frpt, b"\n  %-20s", prj.Link[ju].ID);
        p = 0;
        while p < npollut {
            x = prj.Link[ju].totalLoad[p as usize] * LperFT3 * prj.Pollut[p as usize].mcf;
            if prj.Pollut[p as usize].units == COUNT {
                x = LOG10(x);
            }
            if x < 10000.0 {
                cprintf!(prj.Frpt, b"%14.3f", x);
            } else {
                cprintf!(prj.Frpt, b"%14.3e", x);
            }
            p += 1;
        }
        j += 1;
    }
    WRITE(prj, b"");
}
