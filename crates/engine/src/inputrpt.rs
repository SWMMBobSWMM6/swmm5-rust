//! inputrpt.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   inputrpt.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21 (Build 5.2.0)
//   Author:   L. Rossman
//
//   Report writing functions for input data summary.
//
//   Update History
//   ==============
//   Build 5.2.0:
//   - Support added for reporting Street geometry tables.
//-----------------------------------------------------------------------------

use crate::prelude::*;

/// The file-scope variables of inputrpt.c.
#[derive(Default)]
pub struct InputrptState {}

/// `#define WRITE(x) (report_writeLine((x)))`
#[inline]
fn WRITE(prj: &mut Project, x: &[u8]) {
    report_writeLine(prj, x);
}

//=============================================================================

pub fn inputrpt_writeInput(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: writes summary of input data to report file.
//
{
    let mut m: i32;
    let mut i: usize;
    let mut k: usize;
    let mut lidCount: i32 = 0;
    if prj.ErrorCode != 0 {
        return;
    }

    WRITE(prj, b"");
    WRITE(prj, b"*************");
    WRITE(prj, b"Element Count");
    WRITE(prj, b"*************");
    cprintf!(prj.Frpt, b"\n  Number of rain gages ...... %d", prj.Nobjects[GAGE as usize]);
    cprintf!(prj.Frpt, b"\n  Number of subcatchments ... %d", prj.Nobjects[SUBCATCH as usize]);
    cprintf!(prj.Frpt, b"\n  Number of nodes ........... %d", prj.Nobjects[NODE as usize]);
    cprintf!(prj.Frpt, b"\n  Number of links ........... %d", prj.Nobjects[LINK as usize]);
    cprintf!(prj.Frpt, b"\n  Number of pollutants ...... %d", prj.Nobjects[POLLUT as usize]);
    cprintf!(prj.Frpt, b"\n  Number of land uses ....... %d", prj.Nobjects[LANDUSE as usize]);

    if prj.Nobjects[POLLUT as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"*****************");
        WRITE(prj, b"Pollutant Summary");
        WRITE(prj, b"*****************");
        cprintf!(prj.Frpt, b"\n                               Ppt.      GW         Kdecay");
        cprintf!(prj.Frpt, b"\n  Name                 Units   Concen.   Concen.    1/days    CoPollutant");
        cprintf!(prj.Frpt, b"\n  -----------------------------------------------------------------------");
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] as usize {
            let p = &prj.Pollut[i];
            cprintf!(
                prj.Frpt,
                b"\n  %-20s %5s%10.2f%10.2f%10.2f",
                &p.ID,
                QualUnitsWords[p.units as usize],
                p.pptConcen,
                p.gwConcen,
                p.kDecay * SECperDAY as f64
            );
            if p.coPollut >= 0 {
                cprintf!(
                    prj.Frpt,
                    b"    %-s  (%.2f)",
                    &prj.Pollut[p.coPollut as usize].ID,
                    p.coFraction
                );
            }
            i += 1;
        }
    }

    if prj.Nobjects[LANDUSE as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"***************");
        WRITE(prj, b"Landuse Summary");
        WRITE(prj, b"***************");
        cprintf!(prj.Frpt, b"\n                         Sweeping   Maximum      Last");
        cprintf!(prj.Frpt, b"\n  Name                   Interval   Removal     Swept");
        cprintf!(prj.Frpt, b"\n  ---------------------------------------------------");
        i = 0;
        while i < prj.Nobjects[LANDUSE as usize] as usize {
            let l = &prj.Landuse[i];
            cprintf!(
                prj.Frpt,
                b"\n  %-20s %10.2f%10.2f%10.2f",
                &l.ID,
                l.sweepInterval,
                l.sweepRemoval,
                l.sweepDays0
            );
            i += 1;
        }
    }

    if prj.Nobjects[GAGE as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"****************");
        WRITE(prj, b"Raingage Summary");
        WRITE(prj, b"****************");
        cprintf!(
            prj.Frpt,
            b"\n                                                      Data       Recording"
        );
        cprintf!(
            prj.Frpt,
            b"\n  Name                 Data Source                    Type       Interval "
        );
        cprintf!(
            prj.Frpt,
            b"\n  ------------------------------------------------------------------------"
        );
        i = 0;
        while i < prj.Nobjects[GAGE as usize] as usize {
            let g = &prj.Gage[i];
            if g.tSeries >= 0 {
                cprintf!(prj.Frpt, b"\n  %-20s %-30s ", &g.ID, &prj.Tseries[g.tSeries as usize].ID);
                cprintf!(
                    prj.Frpt,
                    b"%-10s %3d min.",
                    RainTypeWords[g.rainType as usize],
                    (g.rainInterval) / 60
                );
            } else {
                cprintf!(prj.Frpt, b"\n  %-20s %-30s", &g.ID, &g.fname[..]);
            }
            i += 1;
        }
    }

    if prj.Nobjects[SUBCATCH as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"********************");
        WRITE(prj, b"Subcatchment Summary");
        WRITE(prj, b"********************");
        cprintf!(
            prj.Frpt,
            b"\n  Name                       Area     Width   %%Imperv    %%Slope Rain Gage            Outlet              "
        );
        cprintf!(
            prj.Frpt,
            b"\n  -----------------------------------------------------------------------------------------------------------"
        );
        i = 0;
        while i < prj.Nobjects[SUBCATCH as usize] as usize {
            let s = &prj.Subcatch[i];
            cprintf!(
                prj.Frpt,
                b"\n  %-20s %10.2f%10.2f%10.2f%10.4f %-20s ",
                &s.ID,
                s.area * UCF(prj, LANDAREA),
                s.width * UCF(prj, LENGTH),
                s.fracImperv * 100.0,
                s.slope * 100.0,
                &prj.Gage[s.gage as usize].ID
            );
            if s.outNode >= 0 {
                cprintf!(prj.Frpt, b"%-20s", &prj.Node[s.outNode as usize].ID);
            } else if s.outSubcatch >= 0 {
                cprintf!(prj.Frpt, b"%-20s", &prj.Subcatch[s.outSubcatch as usize].ID);
            }
            if s.lidArea != 0.0 {
                lidCount += 1;
            }
            i += 1;
        }
    }
    if lidCount > 0 {
        lid_writeSummary(prj);
    }

    if prj.Nobjects[NODE as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"************");
        WRITE(prj, b"Node Summary");
        WRITE(prj, b"************");
        cprintf!(
            prj.Frpt,
            b"\n                                           Invert      Max.    Ponded    External"
        );
        cprintf!(
            prj.Frpt,
            b"\n  Name                 Type                 Elev.     Depth      Area    Inflow  "
        );
        cprintf!(
            prj.Frpt,
            b"\n  -------------------------------------------------------------------------------"
        );
        i = 0;
        while i < prj.Nobjects[NODE as usize] as usize {
            let n = &prj.Node[i];
            cprintf!(
                prj.Frpt,
                b"\n  %-20s %-16s%10.2f%10.2f%10.1f",
                &n.ID,
                NodeTypeWords[(n.type_ - JUNCTION) as usize],
                n.invertElev * UCF(prj, LENGTH),
                n.fullDepth * UCF(prj, LENGTH),
                n.pondedArea * UCF(prj, LENGTH) * UCF(prj, LENGTH)
            );
            if !n.extInflow.is_empty() || !n.dwfInflow.is_empty() || n.rdiiInflow.is_some() {
                cprintf!(prj.Frpt, b"    Yes");
            }
            i += 1;
        }
    }

    if prj.Nobjects[LINK as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"************");
        WRITE(prj, b"Link Summary");
        WRITE(prj, b"************");
        cprintf!(
            prj.Frpt,
            b"\n  Name             From Node        To Node          Type            Length    %%Slope Roughness"
        );
        cprintf!(
            prj.Frpt,
            b"\n  ---------------------------------------------------------------------------------------------"
        );
        i = 0;
        while i < prj.Nobjects[LINK as usize] as usize {
            let l = &prj.Link[i];
            // --- list end nodes in their original orientation
            if l.direction == 1 {
                cprintf!(
                    prj.Frpt,
                    b"\n  %-16s %-16s %-16s ",
                    &l.ID,
                    &prj.Node[l.node1 as usize].ID,
                    &prj.Node[l.node2 as usize].ID
                );
            } else {
                cprintf!(
                    prj.Frpt,
                    b"\n  %-16s %-16s %-16s ",
                    &l.ID,
                    &prj.Node[l.node2 as usize].ID,
                    &prj.Node[l.node1 as usize].ID
                );
            }

            // --- list link type
            if l.type_ == PUMP {
                k = l.subIndex as usize;
                cprintf!(prj.Frpt, b"%-5s PUMP  ", PumpTypeWords[prj.Pump[k].type_ as usize]);
            } else {
                cprintf!(prj.Frpt, b"%-12s", LinkTypeWords[(l.type_ - CONDUIT) as usize]);
            }

            // --- list length, slope and roughness for conduit links
            if l.type_ == CONDUIT {
                k = l.subIndex as usize;
                cprintf!(
                    prj.Frpt,
                    b"%10.1f%10.4f%10.4f",
                    prj.Conduit[k].length * UCF(prj, LENGTH),
                    prj.Conduit[k].slope * 100.0 * l.direction as f64,
                    prj.Conduit[k].roughness
                );
            }
            i += 1;
        }

        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"*********************");
        WRITE(prj, b"Cross Section Summary");
        WRITE(prj, b"*********************");
        cprintf!(
            prj.Frpt,
            b"\n                                        Full     Full     Hyd.     Max.   No. of     Full"
        );
        cprintf!(
            prj.Frpt,
            b"\n  Conduit          Shape               Depth     Area     Rad.    Width  Barrels     Flow"
        );
        cprintf!(
            prj.Frpt,
            b"\n  ---------------------------------------------------------------------------------------"
        );
        i = 0;
        while i < prj.Nobjects[LINK as usize] as usize {
            let l = &prj.Link[i];
            if l.type_ == CONDUIT {
                k = l.subIndex as usize;
                cprintf!(prj.Frpt, b"\n  %-16s ", &l.ID);
                if l.xsect.type_ == CUSTOM {
                    cprintf!(prj.Frpt, b"%-16s ", &prj.Curve[l.xsect.transect as usize].ID);
                } else if l.xsect.type_ == IRREGULAR {
                    cprintf!(prj.Frpt, b"%-16s ", &prj.Transect[l.xsect.transect as usize].ID);
                } else if l.xsect.type_ == STREET_XSECT {
                    cprintf!(prj.Frpt, b"%-16s ", &prj.Street[l.xsect.transect as usize].ID);
                } else {
                    cprintf!(prj.Frpt, b"%-16s ", XsectTypeWords[l.xsect.type_ as usize]);
                }
                cprintf!(
                    prj.Frpt,
                    b"%8.2f %8.2f %8.2f %8.2f      %3d %8.2f",
                    l.xsect.yFull * UCF(prj, LENGTH),
                    l.xsect.aFull * UCF(prj, LENGTH) * UCF(prj, LENGTH),
                    l.xsect.rFull * UCF(prj, LENGTH),
                    l.xsect.wMax * UCF(prj, LENGTH),
                    prj.Conduit[k].barrels as i32,
                    l.qFull * UCF(prj, FLOW)
                );
            }
            i += 1;
        }
    }

    if prj.Nobjects[SHAPE as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"*************");
        WRITE(prj, b"Shape Summary");
        WRITE(prj, b"*************");
        i = 0;
        while i < prj.Nobjects[SHAPE as usize] as usize {
            k = prj.Shape[i].curve as usize;
            cprintf!(prj.Frpt, b"\n\n  Shape %s", &prj.Curve[k].ID);
            cprintf!(prj.Frpt, b"\n  Area:  ");
            m = 1;
            while m < N_SHAPE_TBL {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Shape[i].areaTbl[m as usize]);
                m += 1;
            }
            cprintf!(prj.Frpt, b"\n  Hrad:  ");
            m = 1;
            while m < N_SHAPE_TBL {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Shape[i].hradTbl[m as usize]);
                m += 1;
            }
            cprintf!(prj.Frpt, b"\n  Width: ");
            m = 1;
            while m < N_SHAPE_TBL {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Shape[i].widthTbl[m as usize]);
                m += 1;
            }
            i += 1;
        }
    }

    if prj.Nobjects[TRANSECT as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"****************");
        WRITE(prj, b"Transect Summary");
        WRITE(prj, b"****************");
        i = 0;
        while i < prj.Nobjects[TRANSECT as usize] as usize {
            cprintf!(prj.Frpt, b"\n\n  Transect %s", &prj.Transect[i].ID);
            cprintf!(prj.Frpt, b"\n  Area:  ");
            m = 1;
            while m < N_TRANSECT_TBL {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Transect[i].areaTbl[m as usize]);
                m += 1;
            }
            cprintf!(prj.Frpt, b"\n  Hrad:  ");
            m = 1;
            while m < N_TRANSECT_TBL {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Transect[i].hradTbl[m as usize]);
                m += 1;
            }
            cprintf!(prj.Frpt, b"\n  Width: ");
            m = 1;
            while m < N_TRANSECT_TBL {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Transect[i].widthTbl[m as usize]);
                m += 1;
            }
            i += 1;
        }
    }

    if prj.Nobjects[STREET as usize] > 0 {
        WRITE(prj, b"");
        WRITE(prj, b"");
        WRITE(prj, b"**************");
        WRITE(prj, b"Street Summary");
        WRITE(prj, b"**************");
        i = 0;
        while i < prj.Nobjects[STREET as usize] as usize {
            cprintf!(prj.Frpt, b"\n\n  Street %s", &prj.Street[i].ID);
            cprintf!(prj.Frpt, b"\n  Area:  ");
            m = 1;
            while m < prj.Street[i].transect.nTbl {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Street[i].transect.areaTbl[m as usize]);
                m += 1;
            }
            cprintf!(prj.Frpt, b"\n  Hrad:  ");
            m = 1;
            while m < prj.Street[i].transect.nTbl {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Street[i].transect.hradTbl[m as usize]);
                m += 1;
            }
            cprintf!(prj.Frpt, b"\n  Width: ");
            m = 1;
            while m < prj.Street[i].transect.nTbl {
                if m % 5 == 1 {
                    cprintf!(prj.Frpt, b"\n          ");
                }
                cprintf!(prj.Frpt, b"%10.4f ", prj.Street[i].transect.widthTbl[m as usize]);
                m += 1;
            }
            i += 1;
        }
    }
    WRITE(prj, b"");
}
