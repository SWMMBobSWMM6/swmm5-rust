//! link.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   link.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     06/12/23   (Build 5.2.4)
//   Author:   L. Rossman
//             M. Tryby (EPA)
//
//   Conveyance system link functions
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Optional surcharging of weirs introduced.
//   Build 5.1.008:
//   - Bug in finding flow through surcharged weir fixed.
//   - Bug in finding if conduit is upstrm/dnstrm full fixed.
//   - Monthly conductivity adjustment applied to conduit seepage.
//   - Conduit seepage limited by conduit's flow rate.
//   Build 5.1.010:
//   - Support added for new ROADWAY_WEIR object.
//   - Time of last setting change initialized for links.
//   Build 5.1.011:
//   - Crest elevation of regulator links raised to downstream invert.
//   - Fixed converting roadWidth weir parameter to internal units.
//   - Weir shape parameter deprecated.
//   - Extra geometric parameters ignored for non-conduit open rectangular
//     cross sections.
//   Build 5.1.012:
//   - Conduit seepage rate now based on flow width, not wetted perimeter.
//   - Formula for side flow weir corrected.
//   - Crest length contraction adjustments corrected.
//   Build 5.1.013:
//   - Maximum depth adjustments made for storage units that can surcharge.
//   - Support added for head-dependent weir coefficient curves.
//   - Adjustment of regulator link crest offset to match downstream node invert
//     now only done for Dynamic Wave flow routing.
//   Build 5.1.014:
//   - Conduit evap. and seepage losses initialized to 0 in conduit_initState()
//     and not allowed to exceed current flow rate in conduit_getLossRate().
//   Build 5.2.0:
//   - Support added for Streets and Inlets.
//   - Support added for variable speed pumps.
//   Build 5.2.1
//   - Warning no longer issued when conduit elevation drop < MIN_DELTA_Z.
//   Build 5.2.2:
//   - Warning for conduit elevation drop < MIN_DELTA_Z restored.
//   Build 5.2.4:
//   - Conduit evap+seepage loss under DW routing limited by conduit volume.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const MIN_DELTA_Z: f64 = 0.001; // minimum elevation change for conduit
                                // slopes (ft)

/// The file-scope variables of link.c (it has none).
#[derive(Default)]
pub struct LinkState {
}

/// `*tok` in C: the first byte of a token (0 for an empty token).
#[inline]
fn tok0(t: &[u8]) -> u8 {
    t.first().copied().unwrap_or(0)
}

//=============================================================================

/// Called by: input.
pub fn link_readParams(prj: &mut Project, j: i32, type_: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j     = link index
//           type  = link type code
//           k     = link type index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads parameters for a specific type of link from a
//           tokenized line of input data.
//
{
    match type_ {
        CONDUIT => return conduit_readParams(prj, j, k, tok, ntoks),
        PUMP => return pump_readParams(prj, j, k, tok, ntoks),
        ORIFICE => return orifice_readParams(prj, j, k, tok, ntoks),
        WEIR => return weir_readParams(prj, j, k, tok, ntoks),
        OUTLET => return outlet_readParams(prj, j, k, tok, ntoks),
        _ => return 0,
    }
}

//=============================================================================

/// Called by: input.
pub fn link_readXsectParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads a link's cross section parameters from a tokenized
//           line of input data.
//  Formats:
//    Link Shape     Geom1   Geom2   Geom3   Geom4   (Barrels  Culvert)
//    Link IRREGULAR TransectID
//    Link STREET    StreetID
//
{
    let mut i: i32;
    let j: i32;
    let k: i32;
    // C leaves x[] uninitialized; only the entries read are ever written
    // first (for CUSTOM, xsect_setParams reads only x[0]).
    let mut x: [f64; 4] = [0.0; 4];

    // --- check for minimum number of tokens
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- get index of link
    j = project_findObject(prj, LINK, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- get code of xsection shape
    k = findmatch(&tok[1], XsectTypeWords);
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }

    // --- assign default number of barrels to conduit
    if prj.Link[j as usize].type_ == CONDUIT {
        let sub = prj.Link[j as usize].subIndex;
        prj.Conduit[sub as usize].barrels = 1;
    }

    // --- assume link is not a culvert
    prj.Link[j as usize].xsect.culvertCode = 0;

    // --- for irregular shape, find index of transect object
    if k == IRREGULAR {
        i = project_findObject(prj, TRANSECT, &tok[2]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[2]);
        }
        prj.Link[j as usize].xsect.type_ = k;
        prj.Link[j as usize].xsect.transect = i;
        return 0;
    }
    // --- for street cross section, find index of Street object
    else if k == STREET_XSECT {
        i = project_findObject(prj, STREET, &tok[2]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[2]);
        }
        prj.Link[j as usize].xsect.type_ = k;
        prj.Link[j as usize].xsect.transect = i;
        return 0;
    } else {
        // --- check that geometric parameters are present
        if ntoks < 6 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }

        // --- parse max. depth & shape curve for a custom shape
        if k == CUSTOM {
            if getDouble(&tok[2], &mut x[0]) == 0 || x[0] <= 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[2]);
            }
            i = project_findObject(prj, CURVE, &tok[3]);
            if i < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[3]);
            }
            prj.Link[j as usize].xsect.type_ = k;
            prj.Link[j as usize].xsect.transect = i;
            prj.Link[j as usize].xsect.yFull = x[0] / UCF(prj, LENGTH);
        }
        // --- parse and save geometric parameters
        else {
            i = 2;
            while i <= 5 {
                if getDouble(&tok[i as usize], &mut x[(i - 2) as usize]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
                }
                i += 1;
            }
        }

        // --- ignore extra parameters for non-conduit open rectangular shapes
        if prj.Link[j as usize].type_ != CONDUIT && k == RECT_OPEN {
            x[2] = 0.0;
            x[3] = 0.0;
        }
        let ucf = UCF(prj, LENGTH);
        let mut xs = prj.Link[j as usize].xsect;
        let ok = xsect_setParams(prj, &mut xs, k, &x, ucf);
        prj.Link[j as usize].xsect = xs;
        if ok == 0 {
            return error_setInpError(prj, ERR_NUMBER, b"");
        }

        // --- parse number of barrels if present
        if prj.Link[j as usize].type_ == CONDUIT && ntoks >= 7 {
            i = cio::atoi(&tok[6]);
            if i <= 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[6]);
            } else {
                let sub = prj.Link[j as usize].subIndex;
                prj.Conduit[sub as usize].barrels = i as i8;
            }
        }

        // --- parse culvert code if present
        if prj.Link[j as usize].type_ == CONDUIT && ntoks >= 8 {
            i = cio::atoi(&tok[7]);
            if i < 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[7]);
            } else {
                prj.Link[j as usize].xsect.culvertCode = i;
            }
        }
    }
    0
}

//=============================================================================

/// Called by: input.
pub fn link_readLossParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads local loss parameters for a link from a tokenized
//           line of input data.
//
//  Format:  LinkID  cInlet  cOutlet  cAvg  FlapGate(YES/NO)  SeepRate
//
{
    let mut i: i32;
    let j: i32;
    let mut k: i32;
    let mut x: [f64; 3] = [0.0; 3];
    let mut seepRate: f64 = 0.0;

    if ntoks < 4 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    j = project_findObject(prj, LINK, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    i = 1;
    while i <= 3 {
        if getDouble(&tok[i as usize], &mut x[(i - 1) as usize]) == 0 || x[(i - 1) as usize] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
        }
        i += 1;
    }
    k = 0;
    if ntoks >= 5 {
        k = findmatch(&tok[4], NoYesWords);
        if k < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[4]);
        }
    }
    if ntoks >= 6 {
        if getDouble(&tok[5], &mut seepRate) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[5]);
        }
    }
    let ucf = UCF(prj, RAINFALL);
    let link = &mut prj.Link[j as usize];
    link.cLossInlet = x[0];
    link.cLossOutlet = x[1];
    link.cLossAvg = x[2];
    link.hasFlapGate = k;
    link.seepRate = seepRate / ucf;
    0
}

//=============================================================================

fn link_setParams(prj: &mut Project, j: i32, type_: i32, n1: i32, n2: i32, k: i32, x: &[f64])
//
//  Input:   j   = link index
//           type = link type code
//           n1   = index of upstream node
//           n2   = index of downstream node
//           k    = index of link's sub-type
//           x    = array of parameter values
//  Output:  none
//  Purpose: sets parameters for a link.
//
{
    let ju = j as usize;
    let ku = k as usize;
    prj.Link[ju].node1 = n1;
    prj.Link[ju].node2 = n2;
    prj.Link[ju].type_ = type_;
    prj.Link[ju].subIndex = k;
    prj.Link[ju].offset1 = 0.0;
    prj.Link[ju].offset2 = 0.0;
    prj.Link[ju].q0 = 0.0;
    prj.Link[ju].qFull = 0.0;
    prj.Link[ju].setting = 1.0;
    prj.Link[ju].targetSetting = 1.0;
    prj.Link[ju].hasFlapGate = 0;
    prj.Link[ju].qLimit = 0.0; // 0 means that no limit is defined
    prj.Link[ju].direction = 1;

    match type_ {
        CONDUIT => {
            prj.Conduit[ku].length = x[0] / UCF(prj, LENGTH);
            prj.Conduit[ku].modLength = prj.Conduit[ku].length;
            prj.Conduit[ku].roughness = x[1];
            prj.Link[ju].offset1 = x[2] / UCF(prj, LENGTH);
            prj.Link[ju].offset2 = x[3] / UCF(prj, LENGTH);
            prj.Link[ju].q0 = x[4] / UCF(prj, FLOW);
            prj.Link[ju].qLimit = x[5] / UCF(prj, FLOW);
        }

        PUMP => {
            prj.Pump[ku].pumpCurve = f64_to_i32(x[0]);
            prj.Link[ju].hasFlapGate = FALSE;
            prj.Pump[ku].initSetting = x[1];
            prj.Pump[ku].yOn = x[2] / UCF(prj, LENGTH);
            prj.Pump[ku].yOff = x[3] / UCF(prj, LENGTH);
            prj.Pump[ku].xMin = 0.0;
            prj.Pump[ku].xMax = 0.0;
        }

        ORIFICE => {
            prj.Orifice[ku].type_ = f64_to_i32(x[0]);
            prj.Link[ju].offset1 = x[1] / UCF(prj, LENGTH);
            prj.Link[ju].offset2 = prj.Link[ju].offset1;
            prj.Orifice[ku].cDisch = x[2];
            prj.Link[ju].hasFlapGate = if x[3] > 0.0 { 1 } else { 0 };
            prj.Orifice[ku].orate = x[4] * 3600.0;
        }

        WEIR => {
            prj.Weir[ku].type_ = f64_to_i32(x[0]);
            prj.Link[ju].offset1 = x[1] / UCF(prj, LENGTH);
            prj.Link[ju].offset2 = prj.Link[ju].offset1;
            prj.Weir[ku].cDisch1 = x[2];
            prj.Link[ju].hasFlapGate = if x[3] > 0.0 { 1 } else { 0 };
            prj.Weir[ku].endCon = x[4];
            prj.Weir[ku].cDisch2 = x[5];
            prj.Weir[ku].canSurcharge = f64_to_i32(x[6]);
            prj.Weir[ku].roadWidth = x[7] / UCF(prj, LENGTH);
            prj.Weir[ku].roadSurface = f64_to_i32(x[8]);
            prj.Weir[ku].cdCurve = f64_to_i32(x[9]);
        }

        OUTLET => {
            prj.Link[ju].offset1 = x[0] / UCF(prj, LENGTH);
            prj.Link[ju].offset2 = prj.Link[ju].offset1;
            prj.Outlet[ku].qCoeff = x[1];
            prj.Outlet[ku].qExpon = x[2];
            prj.Outlet[ku].qCurve = f64_to_i32(x[3]);
            prj.Link[ju].hasFlapGate = if x[4] > 0.0 { 1 } else { 0 };
            prj.Outlet[ku].curveType = f64_to_i32(x[5]);

            // C passes p = NULL; for DUMMY xsect_setParams never reads p.
            let mut xs = prj.Link[ju].xsect;
            xsect_setParams(prj, &mut xs, DUMMY, &[], 0.0);
            prj.Link[ju].xsect = xs;
        }
        _ => {}
    }
}

//=============================================================================

/// Called by: project.
pub fn link_validate(prj: &mut Project, j: i32)
//
//  Input:   j = link index
//  Output:  none
//  Purpose: validates a link's properties.
//
{
    let ju = j as usize;
    let mut n: i32;

    if prj.LinkOffsets == ELEV_OFFSET {
        link_convertOffsets(prj, j);
    }
    let sub = prj.Link[ju].subIndex;
    match prj.Link[ju].type_ {
        CONDUIT => conduit_validate(prj, j, sub),
        PUMP => pump_validate(prj, j, sub),
        ORIFICE => orifice_validate(prj, j, sub),
        WEIR => weir_validate(prj, j, sub),
        _ => {}
    }

    // --- check if crest of regulator opening < invert of downstream node
    match prj.Link[ju].type_ {
        ORIFICE | WEIR | OUTLET => {
            let n1 = prj.Link[ju].node1 as usize;
            let n2 = prj.Link[ju].node2 as usize;
            if prj.Node[n1].invertElev + prj.Link[ju].offset1 < prj.Node[n2].invertElev {
                if prj.RouteModel == DW {
                    prj.Link[ju].offset1 = prj.Node[n2].invertElev - prj.Node[n1].invertElev;
                    let id = prj.Link[ju].ID.clone();
                    report_writeWarningMsg(prj, WARN10b, &id);
                } else {
                    let id = prj.Link[ju].ID.clone();
                    report_writeWarningMsg(prj, WARN10a, &id);
                }
            }
        }
        _ => {}
    }

    // --- force max. depth of end nodes to be >= link crown height
    //     at non-storage nodes

    // --- skip pumps and bottom orifices
    if prj.Link[ju].type_ == PUMP
        || (prj.Link[ju].type_ == ORIFICE
            && prj.Orifice[prj.Link[ju].subIndex as usize].type_ == BOTTOM_ORIFICE)
    {
        return;
    }

    // --- extend upstream node's full depth to link's crown elevation
    n = prj.Link[ju].node1;
    let nu = n as usize;
    if prj.Node[nu].type_ != STORAGE || prj.Node[nu].surDepth > 0.0 {
        prj.Node[nu].fullDepth = MAX(
            prj.Node[nu].fullDepth,
            prj.Link[ju].offset1 + prj.Link[ju].xsect.yFull,
        );
    }

    // --- do same for downstream node only for conduit links
    n = prj.Link[ju].node2;
    let nu = n as usize;
    if (prj.Node[nu].type_ != STORAGE || prj.Node[nu].surDepth > 0.0)
        && prj.Link[ju].type_ == CONDUIT
    {
        prj.Node[nu].fullDepth = MAX(
            prj.Node[nu].fullDepth,
            prj.Link[ju].offset2 + prj.Link[ju].xsect.yFull,
        );
    }
}

//=============================================================================

fn link_convertOffsets(prj: &mut Project, j: i32)
//
//  Input:   j = link index
//  Output:  none
//  Purpose: converts offset elevations to offset heights for a link.
//
{
    let ju = j as usize;
    let mut elev: f64;

    elev = prj.Node[prj.Link[ju].node1 as usize].invertElev;
    let off = prj.Link[ju].offset1;
    prj.Link[ju].offset1 = link_getOffsetHeight(prj, j, off, elev);
    if prj.Link[ju].type_ == CONDUIT {
        elev = prj.Node[prj.Link[ju].node2 as usize].invertElev;
        let off = prj.Link[ju].offset2;
        prj.Link[ju].offset2 = link_getOffsetHeight(prj, j, off, elev);
    } else {
        prj.Link[ju].offset2 = prj.Link[ju].offset1;
    }
}

//=============================================================================

fn link_getOffsetHeight(prj: &mut Project, j: i32, mut offset: f64, elev: f64) -> f64
//
//  Input:   j = link index
//           offset = link elevation offset (ft)
//           elev   = node invert elevation (ft)
//  Output:  returns offset distance above node invert (ft)
//  Purpose: finds offset height for one end of a link.
//
{
    if offset <= MISSING || prj.Link[j as usize].type_ == PUMP {
        return 0.0;
    }
    offset -= elev;
    if offset >= 0.0 {
        return offset;
    }
    if offset >= -MIN_DELTA_Z {
        return 0.0;
    }
    let id = prj.Link[j as usize].ID.clone();
    report_writeWarningMsg(prj, WARN03, &id);
    0.0
}

//=============================================================================

/// Called by: project.
pub fn link_initState(prj: &mut Project, j: i32)
//
//  Input:   j = link index
//  Output:  none
//  Purpose: initializes a link's state variables at start of simulation.
//
{
    let ju = j as usize;
    let mut p: i32;

    // --- initialize hydraulic state
    prj.Link[ju].oldFlow = prj.Link[ju].q0;
    prj.Link[ju].newFlow = prj.Link[ju].q0;
    prj.Link[ju].oldDepth = 0.0;
    prj.Link[ju].newDepth = 0.0;
    prj.Link[ju].oldVolume = 0.0;
    prj.Link[ju].newVolume = 0.0;
    prj.Link[ju].setting = 1.0;
    prj.Link[ju].targetSetting = 1.0;
    prj.Link[ju].timeLastSet = prj.StartDate;
    prj.Link[ju].inletControl = FALSE as i8;
    prj.Link[ju].normalFlow = FALSE as i8;
    if prj.Link[ju].type_ == CONDUIT {
        let sub = prj.Link[ju].subIndex;
        conduit_initState(prj, j, sub);
    }
    if prj.Link[ju].type_ == PUMP {
        let sub = prj.Link[ju].subIndex;
        pump_initState(prj, j, sub);
    }

    // --- initialize water quality state
    p = 0;
    while p < prj.Nobjects[POLLUT as usize] {
        prj.Link[ju].oldQual[p as usize] = 0.0;
        prj.Link[ju].newQual[p as usize] = 0.0;
        prj.Link[ju].totalLoad[p as usize] = 0.0;
        p += 1;
    }
}

//=============================================================================

/// Called by: dynwave, flowrout.
pub fn link_getInflow(prj: &mut Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns link flow rate (cfs)
//  Purpose: finds total flow entering a link during current time step.
//
{
    if prj.Link[j as usize].setting == 0.0 {
        return 0.0;
    }
    match prj.Link[j as usize].type_ {
        CONDUIT => conduit_getInflow(prj, j),
        PUMP => pump_getInflow(prj, j),
        ORIFICE => orifice_getInflow(prj, j),
        WEIR => weir_getInflow(prj, j),
        OUTLET => outlet_getInflow(prj, j),
        _ => {
            let n1 = prj.Link[j as usize].node1;
            node_getOutflow(prj, n1, j)
        }
    }
}

//=============================================================================

/// Called by: routing.
pub fn link_setOldHydState(prj: &mut Project, j: i32)
//
//  Input:   j = link index
//  Output:  none
//  Purpose: replaces link's old hydraulic state values with current ones.
//
{
    let ju = j as usize;
    let k: i32;

    prj.Link[ju].oldDepth = prj.Link[ju].newDepth;
    prj.Link[ju].oldFlow = prj.Link[ju].newFlow;
    prj.Link[ju].oldVolume = prj.Link[ju].newVolume;

    if prj.Link[ju].type_ == CONDUIT {
        k = prj.Link[ju].subIndex;
        let ku = k as usize;
        prj.Conduit[ku].q1Old = prj.Conduit[ku].q1;
        prj.Conduit[ku].q2Old = prj.Conduit[ku].q2;
    }
}

//=============================================================================

/// Called by: routing.
pub fn link_setOldQualState(prj: &mut Project, j: i32)
//
//  Input:   j = link index
//  Output:  none
//  Purpose: replaces link's old water quality state values with current ones.
//
{
    let ju = j as usize;
    let mut p: i32 = 0;
    while p < prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        prj.Link[ju].oldQual[pu] = prj.Link[ju].newQual[pu];
        prj.Link[ju].newQual[pu] = 0.0;
        p += 1;
    }
}

//=============================================================================

/// Called by: hotstart, routing.
pub fn link_setTargetSetting(prj: &mut Project, j: i32)
//
//  Input:   j = link index
//  Output:  none
//  Purpose: updates a link's target setting.
//
{
    let ju = j as usize;
    let k: i32;
    let n1: i32;
    if prj.Link[ju].type_ == PUMP {
        k = prj.Link[ju].subIndex;
        n1 = prj.Link[ju].node1;
        let ku = k as usize;
        let n1u = n1 as usize;
        prj.Link[ju].targetSetting = prj.Link[ju].setting;
        if prj.Pump[ku].yOff > 0.0
            && prj.Link[ju].setting > 0.0
            && prj.Node[n1u].newDepth < prj.Pump[ku].yOff
        {
            prj.Link[ju].targetSetting = 0.0;
        }
        if prj.Pump[ku].yOn > 0.0
            && prj.Link[ju].setting == 0.0
            && prj.Node[n1u].newDepth > prj.Pump[ku].yOn
        {
            prj.Link[ju].targetSetting = 1.0;
        }
    }
}

//=============================================================================

/// Called by: hotstart, routing, swmm5.
pub fn link_setSetting(prj: &mut Project, j: i32, tstep: f64)
//
//  Input:   j = link index
//           tstep = time step over which setting is adjusted
//  Output:  none
//  Purpose: updates a link's setting as a result of a control action.
//
{
    if prj.Link[j as usize].type_ == ORIFICE {
        orifice_setSetting(prj, j, tstep);
    } else if prj.Link[j as usize].type_ == WEIR {
        weir_setSetting(prj, j);
    } else {
        prj.Link[j as usize].setting = prj.Link[j as usize].targetSetting;
    }
}

//=============================================================================

/// Called by: dwflow.
pub fn link_setFlapGate(prj: &mut Project, j: i32, n1: i32, n2: i32, q: f64) -> i32
//
//  Input:   j = link index
//           n1 = index of node on upstream end of link
//           n2 = index of node on downstream end of link
//           q = signed flow value (value and units don't matter)
//  Output:  returns TRUE if there is reverse flow through a flap gate
//           associated with the link.
//  Purpose: based on the sign of the flow, determines if a flap gate
//           associated with the link should close or not.
//
{
    let mut n: i32 = -1;

    // --- check for reverse flow through link's flap gate
    if prj.Link[j as usize].hasFlapGate != 0 {
        if q * (prj.Link[j as usize].direction as f64) < 0.0 {
            return TRUE;
        }
    }

    // --- check for Outfall with flap gate node on inflow end of link
    if q < 0.0 {
        n = n2;
    }
    if q > 0.0 {
        n = n1;
    }
    if n >= 0
        && prj.Node[n as usize].type_ == OUTFALL
        && prj.Outfall[prj.Node[n as usize].subIndex as usize].hasFlapGate != 0
    {
        return TRUE;
    }
    FALSE
}

//=============================================================================

/// Called by: output.
pub fn link_getResults(prj: &mut Project, j: i32, wt: f64, x: &mut [f32])
//
//  Input:   j = link index
//           f = time weighting factor
//  Output:  x = array of weighted results
//  Purpose: retrieves time-weighted average of old and new results for a link.
//
{
    let f = wt;
    let ju = j as usize;
    let mut p: i32; // pollutant index
    let mut y: f64; // depth
    let mut q: f64; // flow
    let mut u: f64; // velocity
    let mut v: f64; // volume
    let mut c: f64; // capacity, setting or concentration
    let f1: f64 = 1.0 - f;

    y = f1 * prj.Link[ju].oldDepth + f * prj.Link[ju].newDepth;
    q = f1 * prj.Link[ju].oldFlow + f * prj.Link[ju].newFlow;
    v = f1 * prj.Link[ju].oldVolume + f * prj.Link[ju].newVolume;
    u = link_getVelocity(prj, j, q, y);
    c = 0.0;
    if prj.Link[ju].type_ == CONDUIT {
        if prj.Link[ju].xsect.type_ != DUMMY {
            c = linkXsect_getAofY(prj, j, y) / prj.Link[ju].xsect.aFull;
        }
    } else {
        c = prj.Link[ju].setting;
    }

    // --- override time weighting for pump flow between on/off states
    if prj.Link[ju].type_ == PUMP && prj.Link[ju].oldFlow * prj.Link[ju].newFlow == 0.0 {
        if f >= f1 {
            q = prj.Link[ju].newFlow;
        } else {
            q = prj.Link[ju].oldFlow;
        }
    }

    y *= UCF(prj, LENGTH);
    v *= UCF(prj, VOLUME);
    q *= UCF(prj, FLOW) * (prj.Link[ju].direction as f64);
    u *= UCF(prj, LENGTH) * (prj.Link[ju].direction as f64);
    x[LINK_DEPTH as usize] = y as f32;
    x[LINK_FLOW as usize] = q as f32;
    x[LINK_VELOCITY as usize] = u as f32;
    x[LINK_VOLUME as usize] = v as f32;
    x[LINK_CAPACITY as usize] = c as f32;

    if prj.IgnoreQuality == 0 {
        p = 0;
        while p < prj.Nobjects[POLLUT as usize] {
            c = f1 * prj.Link[ju].oldQual[p as usize] + f * prj.Link[ju].newQual[p as usize];
            x[(LINK_QUAL + p) as usize] = c as f32;
            p += 1;
        }
    }
}

//=============================================================================

/// Called by: dynwave, flowrout.
pub fn link_setOutfallDepth(prj: &mut Project, j: i32)
//
//  Input:   j = link index
//  Output:  none
//  Purpose: sets depth at outfall node connected to link j.
//
{
    let ju = j as usize;
    let k: i32; // conduit index
    let n: i32; // outfall node index
    let z: f64; // invert offset height (ft)
    let q: f64; // flow rate (cfs)
    let mut yCrit: f64 = 0.0; // critical flow depth (ft)
    let mut yNorm: f64 = 0.0; // normal flow depth (ft)

    // --- find which end node of link is an outfall
    if prj.Node[prj.Link[ju].node2 as usize].type_ == OUTFALL {
        n = prj.Link[ju].node2;
        z = prj.Link[ju].offset2;
    } else if prj.Node[prj.Link[ju].node1 as usize].type_ == OUTFALL {
        n = prj.Link[ju].node1;
        z = prj.Link[ju].offset1;
    } else {
        return;
    }

    // --- find both normal & critical depth for current flow
    if prj.Link[ju].type_ == CONDUIT {
        k = prj.Link[ju].subIndex;
        q = (prj.Link[ju].newFlow / (prj.Conduit[k as usize].barrels as f64)).abs();
        yNorm = link_getYnorm(prj, j, q);
        yCrit = link_getYcrit(prj, j, q);
    }

    // --- set new depth at node
    node_setOutletDepth(prj, n, yNorm, yCrit, z);
}

//=============================================================================

/// Called by: dwflow.
pub fn link_getYcrit(prj: &mut Project, j: i32, q: f64) -> f64
//
//  Input:   j = link index
//           q = link flow rate (cfs)
//  Output:  returns critical depth (ft)
//  Purpose: computes critical depth for given flow rate.
//
{
    linkXsect_getYcrit(prj, j, q)
}

//=============================================================================

/// Called by: dwflow.
pub fn link_getYnorm(prj: &mut Project, j: i32, mut q: f64) -> f64
//
//  Input:   j = link index
//           q = link flow rate (cfs)
//  Output:  returns normal depth (ft)
//  Purpose: computes normal depth for given flow rate.
//
{
    let ju = j as usize;
    let k: i32;
    let s: f64;
    let a: f64;
    let y: f64;

    if prj.Link[ju].type_ != CONDUIT {
        return 0.0;
    }
    if prj.Link[ju].xsect.type_ == DUMMY {
        return 0.0;
    }
    q = q.abs();
    k = prj.Link[ju].subIndex;
    if q > prj.Conduit[k as usize].qMax {
        q = prj.Conduit[k as usize].qMax;
    }
    if q <= 0.0 {
        return 0.0;
    }
    s = q / prj.Conduit[k as usize].beta;
    a = linkXsect_getAofS(prj, j, s);
    y = linkXsect_getYofA(prj, j, a);
    y
}

//=============================================================================

/// Called by: dwflow, dynwave, flowrout, kinwave.
pub fn link_getLength(prj: &Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns length (ft)
//  Purpose: finds true length of a link.
//
{
    if prj.Link[j as usize].type_ == CONDUIT {
        return conduit_getLength(prj, j);
    }
    0.0
}

//=============================================================================

/// Called by: controls, stats, swmm5.
pub fn link_getVelocity(prj: &mut Project, j: i32, mut flow: f64, depth: f64) -> f64
//
//  Input:   j     = link index
//           flow  = link flow rate (cfs)
//           depth = link flow depth (ft)
//  Output:  returns flow velocity (fps)
//  Purpose: finds flow velocity given flow and depth.
//
{
    let ju = j as usize;
    let area: f64;
    let mut veloc: f64 = 0.0;
    let k: i32;

    if depth <= 0.01 {
        return 0.0;
    }
    if prj.Link[ju].type_ == CONDUIT {
        k = prj.Link[ju].subIndex;
        flow /= prj.Conduit[k as usize].barrels as f64;
        area = linkXsect_getAofY(prj, j, depth);
        if area > FUDGE {
            veloc = flow / area;
        }
    }
    veloc
}

//=============================================================================

/// Called by: dwflow.
pub fn link_getFroude(prj: &mut Project, j: i32, v: f64, mut y: f64) -> f64
//
//  Input:   j = link index
//           v = flow velocity (fps)
//           y = flow depth (ft)
//  Output:  returns Froude Number
//  Purpose: computes Froude Number for given velocity and flow depth
//
{
    let ju = j as usize;

    // --- return 0 if link is not a conduit
    if prj.Link[j as usize].type_ != CONDUIT {
        return 0.0;
    }

    // --- return 0 if link empty or closed conduit is full
    if y <= FUDGE {
        return 0.0;
    }
    if xsect_isOpen(prj.Link[ju].xsect.type_) == 0 && prj.Link[ju].xsect.yFull - y <= FUDGE {
        return 0.0;
    }

    // --- compute hydraulic depth
    //     (GCC evaluates the area first; for FILLED_CIRCULAR it can edit the
    //      link's yFull, which the width then reads)
    let aHyd = linkXsect_getAofY(prj, j, y);
    y = aHyd / xsect_getWofY(prj, &prj.Link[ju].xsect, y);

    // --- compute Froude No.
    v.abs() / (GRAVITY * y).sqrt()
}

//=============================================================================

/// Called by: stats.
pub fn link_getPower(prj: &Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns power consumed by link in kwatts
//  Purpose: computes power consumed by head loss (or head gain) of
//           water flowing through a link
//
{
    let n1 = prj.Link[j as usize].node1 as usize;
    let n2 = prj.Link[j as usize].node2 as usize;
    let dh: f64 = (prj.Node[n1].invertElev + prj.Node[n1].newDepth)
        - (prj.Node[n2].invertElev + prj.Node[n2].newDepth);
    let q: f64 = prj.Link[j as usize].newFlow.abs();
    dh.abs() * q / 8.814 * KWperHP
}

//=============================================================================

/// Called by: dwflow, flowrout, kinwave.
pub fn link_getLossRate(prj: &mut Project, j: i32, routeModel: i32, q: f64, tstep: f64) -> f64
//
//  Input:   j = link index
//           routeModel = flow routing model type
//           q = flow rate (ft3/sec)
//           tstep = time step (sec)
//  Output:  returns uniform loss rate in link (ft3/sec)
//  Purpose: computes rate at which flow volume is lost in a link due to
//           evaporation and seepage.
//
{
    if prj.Link[j as usize].type_ == CONDUIT {
        conduit_getLossRate(prj, j, routeModel, q, tstep)
    } else {
        0.0
    }
}

//=============================================================================

/// Called by: dwflow, kinwave.
pub fn link_getFullState(a1: f64, a2: f64, aFull: f64) -> i8
//
//  Input:   a1 = upstream link area (ft2)
//           a2 = downstream link area (ft2)
//           aFull = area of full conduit
//  Output:  returns fullness state of a link
//  Purpose: determines if a link is upstream, downstream or completely full.
//
{
    if a1 >= aFull {
        if a2 >= aFull {
            return ALL_FULL as i8;
        } else {
            return UP_FULL as i8;
        }
    }
    if a2 >= aFull {
        return DN_FULL as i8;
    }
    0
}

//=============================================================================
//                    C O N D U I T   M E T H O D S
//=============================================================================

fn conduit_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = link index
//           k = conduit index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads conduit parameters from a tokenzed line of input.
//
{
    let n1: i32;
    let n2: i32;
    let mut x: [f64; 6] = [0.0; 6];
    let id: Option<Vec<u8>>;

    // --- check for valid ID and end node IDs
    if ntoks < 7 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, LINK, &tok[0]); // link ID
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    n1 = project_findObject(prj, NODE, &tok[1]); // upstrm. node
    if n1 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    n2 = project_findObject(prj, NODE, &tok[2]); // dwnstrm. node
    if n2 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }

    // --- parse length & Mannings N
    if getDouble(&tok[3], &mut x[0]) == 0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[3]);
    }
    if getDouble(&tok[4], &mut x[1]) == 0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[4]);
    }

    // --- parse offsets
    if prj.LinkOffsets == ELEV_OFFSET && tok0(&tok[5]) == b'*' {
        x[2] = MISSING;
    } else if getDouble(&tok[5], &mut x[2]) == 0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[5]);
    }
    if prj.LinkOffsets == ELEV_OFFSET && tok0(&tok[6]) == b'*' {
        x[3] = MISSING;
    } else if getDouble(&tok[6], &mut x[3]) == 0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[6]);
    }

    // --- parse optional parameters
    x[4] = 0.0; // init. flow
    if ntoks >= 8 {
        if getDouble(&tok[7], &mut x[4]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[7]);
        }
    }
    x[5] = 0.0;
    if ntoks >= 9 {
        if getDouble(&tok[8], &mut x[5]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[8]);
        }
    }

    // --- add parameters to data base
    prj.Link[j as usize].ID = id.unwrap();
    link_setParams(prj, j, CONDUIT, n1, n2, k, &x);
    0
}

//=============================================================================

fn conduit_validate(prj: &mut Project, j: i32, k: i32)
//
//  Input:   j = link index
//           k = conduit index
//  Output:  none
//  Purpose: validates a conduit's properties.
//
{
    let ju = j as usize;
    let ku = k as usize;
    let aa: f64;
    let mut lengthFactor: f64;
    let mut roughness: f64;
    let mut slope: f64;

    // --- a storage node cannot have a dummy outflow link
    if prj.Link[ju].xsect.type_ == DUMMY && prj.RouteModel == DW {
        let n1 = prj.Link[ju].node1 as usize;
        if prj.Node[n1].type_ == STORAGE {
            let id = prj.Node[n1].ID.clone();
            report_writeErrorMsg(prj, ERR_DUMMY_LINK, &id);
            return;
        }
    }

    // --- if custom xsection, then set its parameters
    if prj.Link[ju].xsect.type_ == CUSTOM {
        let mut xs = prj.Link[ju].xsect;
        xsect_setCustomXsectParams(prj, &mut xs);
        prj.Link[ju].xsect = xs;
    }

    // --- if irreg. xsection, assign transect roughness to conduit
    if prj.Link[ju].xsect.type_ == IRREGULAR {
        let mut xs = prj.Link[ju].xsect;
        xsect_setIrregXsectParams(prj, &mut xs);
        prj.Link[ju].xsect = xs;
        prj.Conduit[ku].roughness = prj.Transect[prj.Link[ju].xsect.transect as usize].roughness;
    }

    // --- if street xsection, then set its parameters
    if prj.Link[ju].xsect.type_ == STREET_XSECT {
        let mut xs = prj.Link[ju].xsect;
        xsect_setStreetXsectParams(prj, &mut xs);
        prj.Link[ju].xsect = xs;
        prj.Conduit[ku].roughness = prj.Street[prj.Link[ju].xsect.transect as usize].roughness;
    }

    // --- if force main xsection, adjust units on D-W roughness height
    if prj.Link[ju].xsect.type_ == FORCE_MAIN {
        if prj.ForceMainEqn == D_W {
            let u = UCF(prj, RAINDEPTH);
            prj.Link[ju].xsect.rBot /= u;
        }
        if prj.Link[ju].xsect.rBot <= 0.0 {
            let id = prj.Link[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_XSECT, &id);
        }
    }

    // --- check for valid length & roughness
    if prj.Conduit[ku].length <= 0.0 {
        let id = prj.Link[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_LENGTH, &id);
    }
    if prj.Conduit[ku].roughness <= 0.0 {
        let id = prj.Link[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_ROUGHNESS, &id);
    }
    if prj.Conduit[ku].barrels <= 0 {
        let id = prj.Link[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_BARRELS, &id);
    }

    // --- check for valid xsection
    if prj.Link[ju].xsect.type_ != DUMMY {
        if prj.Link[ju].xsect.type_ < 0 {
            let id = prj.Link[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_NO_XSECT, &id);
        } else if prj.Link[ju].xsect.aFull <= 0.0 {
            let id = prj.Link[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_XSECT, &id);
        }
    }
    if prj.ErrorCode != 0 {
        return;
    }

    // --- check for negative offsets
    if prj.Link[ju].offset1 < 0.0 {
        let id = prj.Link[ju].ID.clone();
        report_writeWarningMsg(prj, WARN03, &id);
        prj.Link[ju].offset1 = 0.0;
    }
    if prj.Link[ju].offset2 < 0.0 {
        let id = prj.Link[ju].ID.clone();
        report_writeWarningMsg(prj, WARN03, &id);
        prj.Link[ju].offset2 = 0.0;
    }

    // --- adjust conduit offsets for partly filled circular xsection
    if prj.Link[ju].xsect.type_ == FILLED_CIRCULAR {
        prj.Link[ju].offset1 += prj.Link[ju].xsect.yBot;
        prj.Link[ju].offset2 += prj.Link[ju].xsect.yBot;
    }

    // --- compute conduit slope
    slope = conduit_getSlope(prj, j);
    prj.Conduit[ku].slope = slope;

    // --- reverse orientation of conduit if using dynamic wave routing
    //     and slope is negative
    if prj.RouteModel == DW && slope < 0.0 && prj.Link[ju].xsect.type_ != DUMMY {
        conduit_reverse(prj, j, k);
    }

    // --- get equivalent Manning roughness for Force Mains
    //     for use when pipe is partly full
    roughness = prj.Conduit[ku].roughness;
    if prj.RouteModel == DW && prj.Link[ju].xsect.type_ == FORCE_MAIN {
        roughness = forcemain_getEquivN(prj, j, k);
    }

    // --- adjust roughness for meandering natural channels
    if prj.Link[ju].xsect.type_ == IRREGULAR {
        lengthFactor = prj.Transect[prj.Link[ju].xsect.transect as usize].lengthFactor;
        roughness *= lengthFactor.sqrt();
    }

    // --- lengthen conduit if lengthening option is in effect
    lengthFactor = 1.0;
    if prj.RouteModel == DW && prj.LengtheningStep > 0.0 && prj.Link[ju].xsect.type_ != DUMMY {
        lengthFactor = conduit_getLengthFactor(prj, j, k, roughness);
    }

    if lengthFactor != 1.0 {
        prj.Conduit[ku].modLength = lengthFactor * conduit_getLength(prj, j);
        slope /= lengthFactor;
        roughness = roughness / lengthFactor.sqrt();
    }

    // --- compute roughness factor used when computing friction
    //     slope term in Dynamic Wave flow routing

    // --- special case for non-Manning Force Mains
    //     (roughness factor for full flow is saved in xsect.sBot)
    if prj.RouteModel == DW && prj.Link[ju].xsect.type_ == FORCE_MAIN {
        prj.Link[ju].xsect.sBot = forcemain_getRoughFactor(prj, j, lengthFactor);
    }
    prj.Conduit[ku].roughFactor = GRAVITY * SQR(roughness / PHI);

    // --- compute full flow through cross section
    if prj.Link[ju].xsect.type_ == DUMMY {
        prj.Conduit[ku].beta = 0.0;
    } else {
        prj.Conduit[ku].beta = PHI * slope.abs().sqrt() / roughness;
    }
    prj.Link[ju].qFull = prj.Link[ju].xsect.sFull * prj.Conduit[ku].beta;
    prj.Conduit[ku].qMax = prj.Link[ju].xsect.sMax * prj.Conduit[ku].beta;

    // --- see if flow is supercritical most of time
    //     by comparing normal & critical velocities.
    //     (factor of 0.3 is for circular pipe 95% full)
    // NOTE: this factor was used in the past for a modified version of
    //       Kinematic Wave routing but is now deprecated.
    aa = prj.Conduit[ku].beta / (32.2f64).sqrt() * prj.Link[ju].xsect.yFull.powf(0.1666667) * 0.3;
    if aa >= 1.0 {
        prj.Conduit[ku].superCritical = TRUE as i8;
    } else {
        prj.Conduit[ku].superCritical = FALSE as i8;
    }

    // --- set value of hasLosses flag
    if prj.Link[ju].cLossInlet == 0.0
        && prj.Link[ju].cLossOutlet == 0.0
        && prj.Link[ju].cLossAvg == 0.0
    {
        prj.Conduit[ku].hasLosses = FALSE as i8;
    } else {
        prj.Conduit[ku].hasLosses = TRUE as i8;
    }
}

//=============================================================================

fn conduit_reverse(prj: &mut Project, j: i32, k: i32)
//
//  Input:   j = link index
//           k = conduit index
//  Output:  none
//  Purpose: reverses direction of a conduit
//
{
    let ju = j as usize;
    let i: i32;
    let z: f64;
    let cLoss: f64;

    // --- reverse end nodes
    i = prj.Link[ju].node1;
    prj.Link[ju].node1 = prj.Link[ju].node2;
    prj.Link[ju].node2 = i;

    // --- reverse node offsets
    z = prj.Link[ju].offset1;
    prj.Link[ju].offset1 = prj.Link[ju].offset2;
    prj.Link[ju].offset2 = z;

    // --- reverse loss coeffs.
    cLoss = prj.Link[ju].cLossInlet;
    prj.Link[ju].cLossInlet = prj.Link[ju].cLossOutlet;
    prj.Link[ju].cLossOutlet = cLoss;

    // --- reverse direction & slope
    prj.Conduit[k as usize].slope = -prj.Conduit[k as usize].slope;
    prj.Link[ju].direction = ((prj.Link[ju].direction as i32) * (-1i8 as i32)) as i8;

    // --- reverse initial flow value
    prj.Link[ju].q0 = -prj.Link[ju].q0;
}

//=============================================================================

fn conduit_getLength(prj: &Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns conduit's length (ft)
//  Purpose: finds true length of a conduit.
//
//  Note: for irregular natural channels, user inputs length of main
//        channel (for FEMA purposes) but program should use length
//        associated with entire flood plain. Transect.lengthFactor
//        is the ratio of these two lengths.
//
{
    let ju = j as usize;
    let k = prj.Link[ju].subIndex as usize;
    let t: i32;
    if prj.Link[ju].xsect.type_ != IRREGULAR {
        return prj.Conduit[k].length;
    }
    t = prj.Link[ju].xsect.transect;
    if t < 0 || t >= prj.Nobjects[TRANSECT as usize] {
        return prj.Conduit[k].length;
    }
    prj.Conduit[k].length / prj.Transect[t as usize].lengthFactor
}

//=============================================================================

fn conduit_getLengthFactor(prj: &mut Project, j: i32, k: i32, roughness: f64) -> f64
//
//  Input:   j = link index
//           k = conduit index
//           roughness = conduit Manning's n
//  Output:  returns factor by which a conduit should be lengthened
//  Purpose: computes amount of conduit lengthing to improve numerical stability.
//
//  The following form of the Courant criterion is used:
//      L = t * v * (1 + Fr) / Fr
//  where L = conduit length, t = time step, v = velocity, & Fr = Froude No.
//  After substituting Fr = v / sqrt(gy), where y = flow depth, we get:
//    L = t * ( sqrt(gy) + v )
//
{
    let ju = j as usize;
    let ratio: f64;
    let mut yFull: f64;
    let vFull: f64;
    let tStep: f64;

    // --- evaluate flow depth and velocity at full normal flow condition
    yFull = prj.Link[ju].xsect.yFull;
    if xsect_isOpen(prj.Link[ju].xsect.type_) != 0 {
        yFull = prj.Link[ju].xsect.aFull / xsect_getWofY(prj, &prj.Link[ju].xsect, yFull);
    }
    vFull = PHI / roughness * prj.Link[ju].xsect.sFull * prj.Conduit[k as usize].slope.abs().sqrt()
        / prj.Link[ju].xsect.aFull;

    // --- determine ratio of Courant length to actual length
    if prj.LengtheningStep == 0.0 {
        tStep = prj.RouteStep;
    } else {
        tStep = MIN(prj.RouteStep, prj.LengtheningStep);
    }
    ratio = ((GRAVITY * yFull).sqrt() + vFull) * tStep / conduit_getLength(prj, j);

    // --- return max. of 1.0 and ratio
    if ratio > 1.0 {
        ratio
    } else {
        1.0
    }
}

//=============================================================================

fn conduit_getSlope(prj: &mut Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns conduit slope
//  Purpose: computes conduit slope.
//
{
    let ju = j as usize;
    let elev1: f64;
    let elev2: f64;
    let mut delta: f64;
    let mut slope: f64;
    let length: f64 = conduit_getLength(prj, j);

    // --- check that elevation drop > minimum allowable drop
    elev1 = prj.Link[ju].offset1 + prj.Node[prj.Link[ju].node1 as usize].invertElev;
    elev2 = prj.Link[ju].offset2 + prj.Node[prj.Link[ju].node2 as usize].invertElev;
    delta = (elev1 - elev2).abs();
    if delta < MIN_DELTA_Z {
        let id = prj.Link[ju].ID.clone();
        report_writeWarningMsg(prj, WARN04, &id);
        delta = MIN_DELTA_Z;
    }

    // --- elevation drop cannot exceed conduit length
    if delta >= length {
        let id = prj.Link[ju].ID.clone();
        report_writeWarningMsg(prj, WARN08, &id);
        slope = delta / length;
    }
    // --- slope = elev. drop / horizontal distance
    else {
        slope = delta / (SQR(length) - SQR(delta)).sqrt();
    }

    // -- check that slope exceeds minimum allowable slope
    if prj.MinSlope > 0.0 && slope < prj.MinSlope {
        let id = prj.Link[ju].ID.clone();
        report_writeWarningMsg(prj, WARN05, &id);
        slope = prj.MinSlope;
        // keep min. slope positive for SF or KW routing
        if prj.RouteModel == SF || prj.RouteModel == KW {
            return slope;
        }
    }

    // --- change sign for adverse slope
    if elev1 < elev2 {
        slope = -slope;
    }
    slope
}

//=============================================================================

fn conduit_initState(prj: &mut Project, j: i32, k: i32)
//
//  Input:   j = link index
//           k = conduit index
//  Output:  none
//  Purpose: sets initial conduit depth to normal depth of initial flow
//
{
    let ju = j as usize;
    let ku = k as usize;
    prj.Link[ju].newDepth = link_getYnorm(prj, j, prj.Link[ju].q0 / (prj.Conduit[ku].barrels as f64));
    prj.Link[ju].oldDepth = prj.Link[ju].newDepth;
    prj.Conduit[ku].evapLossRate = 0.0;
    prj.Conduit[ku].seepLossRate = 0.0;
}

//=============================================================================

fn conduit_getInflow(prj: &mut Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns flow in link (cfs)
//  Purpose: finds inflow to conduit from upstream node.
//
{
    let n1 = prj.Link[j as usize].node1;
    let mut qIn: f64 = node_getOutflow(prj, n1, j);
    if prj.Link[j as usize].qLimit > 0.0 {
        qIn = MIN(qIn, prj.Link[j as usize].qLimit);
    }
    qIn
}

//=============================================================================

fn conduit_getLossRate(prj: &mut Project, j: i32, routeModel: i32, mut q: f64, tstep: f64) -> f64
//
//  Input:   j = link index
//           routeModel = type of flow routing model
//           q = current link flow rate (cfs)
//           tstep = current routing time step (sec)
//  Output:  returns rate of evaporation & seepage losses (ft3/sec)
//  Purpose: computes volumetric rate of water evaporation & seepage
//           from a conduit (per barrel).
//
{
    let ju = j as usize;
    let mut depth: f64 = 0.5 * (prj.Link[ju].oldDepth + prj.Link[ju].newDepth);
    let length: f64;
    let width: f64;
    let topWidth: f64;
    let mut evapLossRate: f64 = 0.0;
    let mut seepLossRate: f64 = 0.0;
    let mut totalLossRate: f64 = 0.0;

    if depth > FUDGE {
        let xsect: &TXsect = &prj.Link[ju].xsect;
        length = conduit_getLength(prj, j);

        // --- find evaporation rate for open conduits
        if xsect_isOpen(xsect.type_) != 0 && prj.Evap.rate > 0.0 {
            topWidth = xsect_getWofY(prj, xsect, depth);
            evapLossRate = topWidth * length * prj.Evap.rate;
        }

        // --- compute seepage loss rate
        if prj.Link[ju].seepRate > 0.0 {
            // limit depth to depth at max width
            if xsect.type_ == RECT_CLOSED {
                width = xsect.wMax;
            } else {
                if depth >= xsect.ywMax {
                    depth = xsect.ywMax;
                }
                width = xsect_getWofY(prj, xsect, depth);
            }

            // compute seepage loss rate across length of conduit
            seepLossRate = prj.Link[ju].seepRate * width * length;
            seepLossRate *= prj.Adjust.hydconFactor;
        }

        // --- compute total loss rate
        totalLossRate = evapLossRate + seepLossRate;

        // --- limit total loss rate to current volume for DW routing
        //     or current link flow rate otherwise
        if routeModel == DW {
            q = prj.Link[ju].newVolume / tstep;
        } else {
            q = ABS(q);
        }
        if totalLossRate > q {
            evapLossRate = evapLossRate * q / totalLossRate;
            seepLossRate = seepLossRate * q / totalLossRate;
            totalLossRate = q;
        }
    }

    let sub = prj.Link[ju].subIndex as usize;
    prj.Conduit[sub].evapLossRate = evapLossRate;
    prj.Conduit[sub].seepLossRate = seepLossRate;
    totalLossRate
}

//=============================================================================
//                        P U M P   M E T H O D S
//=============================================================================

fn pump_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = link index
//           k = pump index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads pump parameters from a tokenized line of input.
//
{
    let mut m: i32;
    let n1: i32;
    let n2: i32;
    let mut x: [f64; 4] = [0.0; 4];
    let id: Option<Vec<u8>>;

    // --- check for valid ID and end node IDs
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, LINK, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    n1 = project_findObject(prj, NODE, &tok[1]);
    if n1 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    n2 = project_findObject(prj, NODE, &tok[2]);
    if n2 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }

    // --- parse curve name
    x[0] = -1.0;
    if ntoks >= 4 {
        if strcomp(&tok[3], b"*") == 0 {
            m = project_findObject(prj, CURVE, &tok[3]);
            if m < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[3]);
            }
            x[0] = m as f64;
        }
    }

    // --- parse init. status if present
    x[1] = 1.0;
    if ntoks >= 5 {
        m = findmatch(&tok[4], OffOnWords);
        if m < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[4]);
        }
        x[1] = m as f64;
    }

    // --- parse startup/shutoff depths if present
    x[2] = 0.0;
    if ntoks >= 6 {
        if getDouble(&tok[5], &mut x[2]) == 0 || x[2] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[5]);
        }
    }
    x[3] = 0.0;
    if ntoks >= 7 {
        if getDouble(&tok[6], &mut x[3]) == 0 || x[3] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[6]);
        }
    }

    // --- add parameters to pump object
    prj.Link[j as usize].ID = id.unwrap();
    link_setParams(prj, j, PUMP, n1, n2, k, &x);
    0
}

//=============================================================================

fn pump_validate(prj: &mut Project, j: i32, k: i32)
//
//  Input:   j = link index
//           k = pump index
//  Output:  none
//  Purpose: validates a pump's properties
//
{
    let ju = j as usize;
    let ku = k as usize;
    let m: i32;
    let n1: i32;
    // x and y are written by table_getFirstEntry before they are read
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    prj.Link[ju].xsect.yFull = 0.0;

    // --- check for valid curve type
    m = prj.Pump[ku].pumpCurve;
    if m < 0 {
        prj.Pump[ku].type_ = IDEAL_PUMP;
    } else {
        let mu = m as usize;
        if prj.Curve[mu].curveType < PUMP1_CURVE || prj.Curve[mu].curveType > PUMP5_CURVE {
            let id = prj.Link[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_NO_CURVE, &id);
        }
        // --- store pump curve type with pump's parameters
        else {
            prj.Pump[ku].type_ = prj.Curve[mu].curveType - PUMP1_CURVE;
            if table_getFirstEntry(&mut prj.Curve[mu], &mut x, &mut y) != 0 {
                prj.Link[ju].qFull = y;
                prj.Pump[ku].xMin = x;
                prj.Pump[ku].xMax = x;
                while table_getNextEntry(&mut prj.Curve[mu], &mut x, &mut y) != 0 {
                    prj.Link[ju].qFull = MAX(y, prj.Link[ju].qFull);
                    prj.Pump[ku].xMax = x;
                }
            }
            let u = UCF(prj, FLOW);
            prj.Link[ju].qFull /= u;
        }
    }

    // --- check that shutoff depth < startup depth
    if prj.Pump[ku].yOn > 0.0 && prj.Pump[ku].yOn <= prj.Pump[ku].yOff {
        let id = prj.Link[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_PUMP_LIMITS, &id);
    }

    // --- assign wet well volume to inlet node of Type 1 pump
    if prj.Pump[ku].type_ == TYPE1_PUMP {
        n1 = prj.Link[ju].node1;
        let n1u = n1 as usize;
        if prj.Node[n1u].type_ != STORAGE {
            prj.Node[n1u].fullVolume = MAX(
                prj.Node[n1u].fullVolume,
                prj.Pump[ku].xMax / UCF(prj, VOLUME),
            );
        }
    }
}

//=============================================================================

fn pump_initState(prj: &mut Project, j: i32, k: i32)
//
//  Input:   j = link index
//           k = pump index
//  Output:  none
//  Purpose: initializes pump conditions at start of a simulation
//
{
    prj.Link[j as usize].setting = prj.Pump[k as usize].initSetting;
    prj.Link[j as usize].targetSetting = prj.Pump[k as usize].initSetting;
}

//=============================================================================

fn pump_getInflow(prj: &mut Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns pump flow (cfs)
//  Purpose: finds flow produced by a pump.
//
{
    let ju = j as usize;
    let k: i32;
    let m: i32;
    let n1: i32;
    let n2: i32;
    let vol: f64;
    let mut depth: f64;
    let mut head: f64;
    let mut qIn: f64;
    let qIn1: f64;
    let dh: f64 = 0.001;
    let mut s: f64 = 1.0; // speed setting

    k = prj.Link[ju].subIndex;
    m = prj.Pump[k as usize].pumpCurve;
    n1 = prj.Link[ju].node1;
    n2 = prj.Link[ju].node2;
    let ku = k as usize;
    let n1u = n1 as usize;
    let n2u = n2 as usize;

    // --- no flow if setting is closed
    prj.Link[ju].flowClass = NO;
    prj.Link[ju].setting = prj.Link[ju].targetSetting;
    if prj.Link[ju].setting == 0.0 {
        return 0.0;
    }

    // --- pump flow = node inflow for IDEAL_PUMP
    if prj.Pump[ku].type_ == IDEAL_PUMP {
        qIn = prj.Node[n1u].inflow + prj.Node[n1u].overflow;
    }
    // --- pumping rate depends on pump curve type
    else {
        let mu = m as usize;
        match prj.Curve[mu].curveType {
            PUMP1_CURVE => {
                vol = prj.Node[n1u].newVolume * UCF(prj, VOLUME);
                qIn = table_intervalLookup(&mut prj.Curve[mu], vol) / UCF(prj, FLOW);

                // --- check if off of pump curve
                if vol < prj.Pump[ku].xMin || vol > prj.Pump[ku].xMax {
                    prj.Link[ju].flowClass = YES;
                }
            }

            PUMP2_CURVE => {
                depth = prj.Node[n1u].newDepth * UCF(prj, LENGTH);
                qIn = table_intervalLookup(&mut prj.Curve[mu], depth) / UCF(prj, FLOW);

                // --- check if off of pump curve
                if depth < prj.Pump[ku].xMin || depth > prj.Pump[ku].xMax {
                    prj.Link[ju].flowClass = YES;
                }
            }

            PUMP3_CURVE | PUMP5_CURVE => {
                if prj.Curve[mu].curveType == PUMP5_CURVE {
                    s = prj.Link[ju].setting;
                }
                head = ((prj.Node[n2u].newDepth + prj.Node[n2u].invertElev)
                    - (prj.Node[n1u].newDepth + prj.Node[n1u].invertElev))
                    / s
                    / s;
                head = MAX(head, 0.0) * UCF(prj, LENGTH);
                qIn = table_lookup(&mut prj.Curve[mu], head) / UCF(prj, FLOW);

                // --- compute dQ/dh (slope of pump curve) and
                //     reverse sign since flow decreases with increasing head
                prj.Link[ju].dqdh =
                    -table_getSlope(&mut prj.Curve[mu], head) * UCF(prj, LENGTH) / UCF(prj, FLOW) / s;

                // --- check if off of pump curve
                if head < prj.Pump[ku].xMin || head > prj.Pump[ku].xMax {
                    prj.Link[ju].flowClass = YES;
                }
            }

            PUMP4_CURVE => {
                depth = prj.Node[n1u].newDepth;
                let arg = depth * UCF(prj, LENGTH);
                qIn = table_lookup(&mut prj.Curve[mu], arg) / UCF(prj, FLOW);

                // --- compute dQ/dh (slope of pump curve)
                let arg = (depth + dh) * UCF(prj, LENGTH);
                qIn1 = table_lookup(&mut prj.Curve[mu], arg) / UCF(prj, FLOW);
                prj.Link[ju].dqdh = (qIn1 - qIn) / dh;

                // --- check if off of pump curve
                depth *= UCF(prj, LENGTH);
                if depth < prj.Pump[ku].xMin {
                    prj.Link[ju].flowClass = DN_DRY;
                }
                if depth > prj.Pump[ku].xMax {
                    prj.Link[ju].flowClass = UP_DRY;
                }
            }

            _ => {
                qIn = 0.0;
            }
        }
    }

    // --- do not allow reverse flow through pump
    if qIn < 0.0 {
        qIn = 0.0;
    }
    qIn * prj.Link[ju].setting
}

//=============================================================================
//                    O R I F I C E   M E T H O D S
//=============================================================================

fn orifice_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = link index
//           k = orifice index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads orifice parameters from a tokenized line of input.
//
{
    let mut m: i32;
    let n1: i32;
    let n2: i32;
    let mut x: [f64; 5] = [0.0; 5];
    let id: Option<Vec<u8>>;

    // --- check for valid ID and end node IDs
    if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, LINK, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    n1 = project_findObject(prj, NODE, &tok[1]);
    if n1 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    n2 = project_findObject(prj, NODE, &tok[2]);
    if n2 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }

    // --- parse orifice parameters
    m = findmatch(&tok[3], OrificeTypeWords);
    if m < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
    }
    x[0] = m as f64; // type
    if prj.LinkOffsets == ELEV_OFFSET && tok0(&tok[4]) == b'*' {
        x[1] = MISSING;
    } else if getDouble(&tok[4], &mut x[1]) == 0 {
        // crest height
        return error_setInpError(prj, ERR_NUMBER, &tok[4]);
    }
    if getDouble(&tok[5], &mut x[2]) == 0 || x[2] < 0.0 {
        // cDisch
        return error_setInpError(prj, ERR_NUMBER, &tok[5]);
    }
    x[3] = 0.0;
    if ntoks >= 7 {
        m = findmatch(&tok[6], NoYesWords);
        if m < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[6]);
        }
        x[3] = m as f64; // flap gate
    }
    x[4] = 0.0;
    if ntoks >= 8 {
        if getDouble(&tok[7], &mut x[4]) == 0 || x[4] < 0.0 {
            // orate
            return error_setInpError(prj, ERR_NUMBER, &tok[7]);
        }
    }

    // --- add parameters to orifice object
    prj.Link[j as usize].ID = id.unwrap();
    link_setParams(prj, j, ORIFICE, n1, n2, k, &x);
    0
}

//=============================================================================

fn orifice_validate(prj: &mut Project, j: i32, k: i32)
//
//  Input:   j = link index
//           k = orifice index
//  Output:  none
//  Purpose: validates an orifice's properties
//
{
    let ju = j as usize;
    let ku = k as usize;
    let mut err: i32 = 0;

    // --- check for valid xsection
    if prj.Link[ju].xsect.type_ != RECT_CLOSED && prj.Link[ju].xsect.type_ != CIRCULAR {
        err = ERR_REGULATOR_SHAPE;
    }
    if err > 0 {
        let id = prj.Link[ju].ID.clone();
        report_writeErrorMsg(prj, err, &id);
        return;
    }

    // --- check for negative offset
    if prj.Link[ju].offset1 < 0.0 {
        prj.Link[ju].offset1 = 0.0;
    }

    // --- compute partial flow adjustment
    orifice_setSetting(prj, j, 0.0);

    // --- compute an equivalent length
    prj.Orifice[ku].length = 2.0 * prj.RouteStep * (GRAVITY * prj.Link[ju].xsect.yFull).sqrt();
    prj.Orifice[ku].length = MAX(200.0, prj.Orifice[ku].length);
    prj.Orifice[ku].surfArea = 0.0;
}

//=============================================================================

fn orifice_setSetting(prj: &mut Project, j: i32, tstep: f64)
//
//  Input:   j = link index
//           tstep = time step over which setting is adjusted (sec)
//  Output:  none
//  Purpose: updates an orifice's setting as a result of a control action.
//
{
    let ju = j as usize;
    let k = prj.Link[ju].subIndex;
    let ku = k as usize;
    let delta: f64;
    let step: f64;
    let h: f64;
    let f: f64;

    // --- case where adjustment rate is instantaneous
    if prj.Orifice[ku].orate == 0.0 || tstep == 0.0 {
        prj.Link[ju].setting = prj.Link[ju].targetSetting;
    }
    // --- case where orifice setting depends on time step
    else {
        delta = prj.Link[ju].targetSetting - prj.Link[ju].setting;
        step = tstep / prj.Orifice[ku].orate;
        if step + 0.001 >= delta.abs() {
            prj.Link[ju].setting = prj.Link[ju].targetSetting;
        } else {
            prj.Link[ju].setting += SGN(delta) * step;
        }
    }

    // --- find effective orifice discharge coeff.
    h = prj.Link[ju].setting * prj.Link[ju].xsect.yFull;
    f = xsect_getAofY(prj, &prj.Link[ju].xsect, h) * (2.0 * GRAVITY).sqrt();
    prj.Orifice[ku].cOrif = prj.Orifice[ku].cDisch * f;

    // --- find equiv. discharge coeff. for when weir flow occurs
    prj.Orifice[ku].cWeir = orifice_getWeirCoeff(prj, j, k, h) * f;
}

//=============================================================================

fn orifice_getWeirCoeff(prj: &mut Project, j: i32, k: i32, mut h: f64) -> f64
//
//  Input:   j = link index
//           k = orifice index
//           h = height of orifice opening (ft)
//  Output:  returns a discharge coefficient (ft^1/2)
//  Purpose: computes the discharge coefficient for an orifice
//           at the critical depth where weir flow begins.
//
{
    let ju = j as usize;
    let ku = k as usize;
    let w: f64;
    let aOverL: f64;

    // --- this is for bottom orifices
    if prj.Orifice[ku].type_ == BOTTOM_ORIFICE {
        // --- find critical height above opening where orifice flow
        //     turns into weir flow. It equals (Co/Cw)*(Area/Length)
        //     where Co is the orifice coeff., Cw is the weir coeff/sqrt(2g),
        //     Area is the area of the opening, and Length = circumference
        //     of the opening. For a basic sharp crested weir, Cw = 0.414.
        if prj.Link[ju].xsect.type_ == CIRCULAR {
            aOverL = h / 4.0;
        } else {
            w = prj.Link[ju].xsect.wMax;
            aOverL = (h * w) / (2.0 * (h + w));
        }
        h = prj.Orifice[ku].cDisch / 0.414 * aOverL;
        prj.Orifice[ku].hCrit = h;
    }
    // --- this is for side orifices
    else {
        // --- critical height is simply height of opening
        prj.Orifice[ku].hCrit = h;

        // --- head on orifice is distance to center line
        h = h / 2.0;
    }

    // --- return a coefficient for the critical depth
    prj.Orifice[ku].cDisch * h.sqrt()
}

//=============================================================================

#[allow(unused_assignments)] // hcrest, hcrown: C's initial values, overwritten
fn orifice_getInflow(prj: &mut Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns orifice flow rate (cfs)
//  Purpose: finds the flow through an orifice.
//
{
    let ju = j as usize;
    let k: i32;
    let n1: i32;
    let n2: i32;
    let mut head: f64;
    let mut h1: f64;
    let mut h2: f64;
    let mut y1: f64;
    let dir: f64;
    let mut f: f64;
    let mut hcrest: f64 = 0.0;
    let mut hcrown: f64 = 0.0;
    let hmidpt: f64;
    let mut q: f64;
    let ratio: f64;

    // --- get indexes of end nodes and link's orifice
    n1 = prj.Link[ju].node1;
    n2 = prj.Link[ju].node2;
    k = prj.Link[ju].subIndex;
    let n1u = n1 as usize;
    let n2u = n2 as usize;
    let ku = k as usize;

    // --- find heads at upstream & downstream nodes
    if prj.RouteModel == DW {
        h1 = prj.Node[n1u].newDepth + prj.Node[n1u].invertElev;
        h2 = prj.Node[n2u].newDepth + prj.Node[n2u].invertElev;
    } else {
        h1 = prj.Node[n1u].newDepth + prj.Node[n1u].invertElev;
        h2 = prj.Node[n1u].invertElev;
    }
    dir = if h1 >= h2 { 1.0 } else { -1.0 };

    // --- exchange h1 and h2 for reverse flow
    y1 = prj.Node[n1u].newDepth;
    if dir < 0.0 {
        head = h1;
        h1 = h2;
        h2 = head;
        y1 = prj.Node[n2u].newDepth;
    }

    // --- orifice is a bottom orifice (oriented in horizontal plane)
    if prj.Orifice[ku].type_ == BOTTOM_ORIFICE {
        // --- compute crest elevation
        hcrest = prj.Node[n1u].invertElev + prj.Link[ju].offset1;

        // --- compute head on orifice
        if h1 < hcrest {
            head = 0.0;
        } else if h2 > hcrest {
            head = h1 - h2;
        } else {
            head = h1 - hcrest;
        }

        // --- find fraction of critical height for which weir flow occurs
        f = head / prj.Orifice[ku].hCrit;
        f = MIN(f, 1.0);
    }
    // --- otherwise orifice is a side orifice (oriented in vertical plane)
    else {
        // --- compute elevations of orifice crest and crown
        hcrest = prj.Node[n1u].invertElev + prj.Link[ju].offset1;
        hcrown = hcrest + prj.Link[ju].xsect.yFull * prj.Link[ju].setting;
        hmidpt = (hcrest + hcrown) / 2.0;

        // --- compute degree of inlet submergence
        if h1 < hcrown && hcrown > hcrest {
            f = (h1 - hcrest) / (hcrown - hcrest);
        } else {
            f = 1.0;
        }

        // --- compute head on orifice
        if f < 1.0 {
            head = h1 - hcrest;
        } else if h2 < hmidpt {
            head = h1 - hmidpt;
        } else {
            head = h1 - h2;
        }
    }

    // --- return if head is negligible or flap gate closed
    if head <= FUDGE || y1 <= FUDGE || link_setFlapGate(prj, j, n1, n2, dir) != 0 {
        prj.Link[ju].newDepth = 0.0;
        prj.Link[ju].flowClass = DRY;
        prj.Orifice[ku].surfArea = FUDGE * prj.Orifice[ku].length;
        prj.Link[ju].dqdh = 0.0;
        return 0.0;
    }

    // --- determine flow class
    prj.Link[ju].flowClass = SUBCRITICAL;
    if hcrest > h2 {
        if dir == 1.0 {
            prj.Link[ju].flowClass = DN_CRITICAL;
        } else {
            prj.Link[ju].flowClass = UP_CRITICAL;
        }
    }

    // --- compute flow depth and surface area
    y1 = prj.Link[ju].xsect.yFull * prj.Link[ju].setting;
    if prj.Orifice[ku].type_ == SIDE_ORIFICE {
        prj.Link[ju].newDepth = y1 * f;
        prj.Orifice[ku].surfArea =
            xsect_getWofY(prj, &prj.Link[ju].xsect, prj.Link[ju].newDepth) * prj.Orifice[ku].length;
    } else {
        prj.Link[ju].newDepth = y1;
        prj.Orifice[ku].surfArea = xsect_getAofY(prj, &prj.Link[ju].xsect, y1);
    }

    // --- find flow through the orifice
    let hasFlapGate = prj.Link[ju].hasFlapGate;
    q = dir * orifice_getFlow(prj, j, k, head, f, hasFlapGate);

    // --- apply Villemonte eqn. to correct for submergence
    if f < 1.0 && h2 > hcrest {
        ratio = (h2 - hcrest) / (h1 - hcrest);
        q *= (1.0 - ratio.powf(1.5)).powf(0.385);
    }
    q
}

//=============================================================================

fn orifice_getFlow(prj: &mut Project, j: i32, k: i32, mut head: f64, mut f: f64, hasFlapGate: i32) -> f64
//
//  Input:   j = link index
//           k = orifice index
//           head = head across orifice
//           f = fraction of critical depth filled
//           hasFlapGate = flap gate indicator
//  Output:  returns flow through an orifice
//  Purpose: computes flow through an orifice as a function of head.
//
{
    let ju = j as usize;
    let ku = k as usize;
    let area: f64;
    let mut q: f64;
    let veloc: f64;
    let hLoss: f64;

    // --- case where orifice is closed
    if head == 0.0 || f <= 0.0 {
        prj.Link[ju].dqdh = 0.0;
        return 0.0;
    }
    // --- case where inlet depth is below critical depth;
    //     orifice behaves as a weir
    else if f < 1.0 {
        q = prj.Orifice[ku].cWeir * f.powf(1.5);
        prj.Link[ju].dqdh = 1.5 * q / (f * prj.Orifice[ku].hCrit);
    }
    // --- case where normal orifice flow applies
    else {
        q = prj.Orifice[ku].cOrif * head.sqrt();
        prj.Link[ju].dqdh = q / (2.0 * head);
    }

    // --- apply ARMCO adjustment for headloss from flap gate
    if hasFlapGate != 0 {
        // --- compute velocity for current orifice flow
        area = xsect_getAofY(
            prj,
            &prj.Link[ju].xsect,
            prj.Link[ju].setting * prj.Link[ju].xsect.yFull,
        );
        veloc = q / area;

        // --- compute head loss from gate
        hLoss = (4.0 / GRAVITY) * veloc * veloc * (-1.15 * veloc / head.sqrt()).exp();

        // --- update head (for orifice flow)
        //     or critical depth fraction (for weir flow)
        if f < 1.0 {
            f = f - hLoss / prj.Orifice[ku].hCrit;
            if f < 0.0 {
                f = 0.0;
            }
        } else {
            head = head - hLoss;
            if head < 0.0 {
                head = 0.0;
            }
        }

        // --- make recursive call to this function, with hasFlapGate
        //     set to false, to find flow values at adjusted head value
        q = orifice_getFlow(prj, j, k, head, f, FALSE);
    }
    q
}

//=============================================================================
//                           W E I R   M E T H O D S
//=============================================================================

fn weir_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = link index
//           k = weir index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads weir parameters from a tokenized line of input.
//
{
    let mut m: i32;
    let n1: i32;
    let n2: i32;
    let mut x: [f64; 10] = [0.0; 10];
    let id: Option<Vec<u8>>;

    // --- check for valid ID and end node IDs
    if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, LINK, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    n1 = project_findObject(prj, NODE, &tok[1]);
    if n1 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    n2 = project_findObject(prj, NODE, &tok[2]);
    if n2 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }

    // --- parse weir parameters
    m = findmatch(&tok[3], WeirTypeWords);
    if m < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
    }
    x[0] = m as f64; // type
    if prj.LinkOffsets == ELEV_OFFSET && tok0(&tok[4]) == b'*' {
        x[1] = MISSING;
    } else if getDouble(&tok[4], &mut x[1]) == 0 {
        // height
        return error_setInpError(prj, ERR_NUMBER, &tok[4]);
    }
    if getDouble(&tok[5], &mut x[2]) == 0 || x[2] < 0.0 {
        // cDisch1
        return error_setInpError(prj, ERR_NUMBER, &tok[5]);
    }
    x[3] = 0.0;
    x[4] = 0.0;
    x[5] = 0.0;
    x[6] = 1.0;
    x[7] = 0.0;
    x[8] = 0.0;
    x[9] = -1.0;
    if ntoks >= 7 && tok0(&tok[6]) != b'*' {
        m = findmatch(&tok[6], NoYesWords);
        if m < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[6]);
        }
        x[3] = m as f64; // flap gate
    }
    if ntoks >= 8 && tok0(&tok[7]) != b'*' {
        if getDouble(&tok[7], &mut x[4]) == 0 || x[4] < 0.0 {
            // endCon
            return error_setInpError(prj, ERR_NUMBER, &tok[7]);
        }
    }
    if ntoks >= 9 && tok0(&tok[8]) != b'*' {
        if getDouble(&tok[8], &mut x[5]) == 0 || x[5] < 0.0 {
            // cDisch2
            return error_setInpError(prj, ERR_NUMBER, &tok[8]);
        }
    }

    if ntoks >= 10 && tok0(&tok[9]) != b'*' {
        m = findmatch(&tok[9], NoYesWords);
        if m < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[9]);
        }
        x[6] = m as f64; // canSurcharge
    }

    m = f64_to_i32(x[0]);
    if m == ROADWAY_WEIR {
        if ntoks >= 11 {
            // road width
            if getDouble(&tok[10], &mut x[7]) == 0 || x[7] < 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[10]);
            }
        }
        if ntoks >= 12 {
            // road surface
            if strcomp(&tok[11], b"PAVED") != 0 {
                x[8] = 1.0;
            } else if strcomp(&tok[11], b"GRAVEL") != 0 {
                x[8] = 2.0;
            }
        }
    }

    if ntoks >= 13 && tok0(&tok[12]) != b'*' {
        m = project_findObject(prj, CURVE, &tok[12]); // coeff. curve
        if m < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[12]);
        }
        x[9] = m as f64;
    }

    // --- add parameters to weir object
    prj.Link[j as usize].ID = id.unwrap();
    link_setParams(prj, j, WEIR, n1, n2, k, &x);
    0
}

//=============================================================================

fn weir_validate(prj: &mut Project, j: i32, k: i32)
//
//  Input:   j = link index
//           k = weir index
//  Output:  none
//  Purpose: validates a weir's properties
//
{
    let ju = j as usize;
    let ku = k as usize;
    let mut err: i32 = 0;
    let q: f64;
    let mut q1: f64 = 0.0;
    let mut q2: f64 = 0.0;
    let mut head: f64;

    // --- check for valid cross section
    match prj.Weir[ku].type_ {
        TRANSVERSE_WEIR | SIDEFLOW_WEIR | ROADWAY_WEIR => {
            if prj.Link[ju].xsect.type_ != RECT_OPEN {
                err = ERR_REGULATOR_SHAPE;
            }
            prj.Weir[ku].slope = 0.0;
        }

        VNOTCH_WEIR => {
            if prj.Link[ju].xsect.type_ != TRIANGULAR {
                err = ERR_REGULATOR_SHAPE;
            } else {
                prj.Weir[ku].slope = prj.Link[ju].xsect.sBot;
            }
        }

        TRAPEZOIDAL_WEIR => {
            if prj.Link[ju].xsect.type_ != TRAPEZOIDAL {
                err = ERR_REGULATOR_SHAPE;
            } else {
                prj.Weir[ku].slope = prj.Link[ju].xsect.sBot;
            }
        }
        _ => {}
    }
    if err > 0 {
        let id = prj.Link[ju].ID.clone();
        report_writeErrorMsg(prj, err, &id);
        return;
    }

    // --- check for negative offset
    if prj.Link[ju].offset1 < 0.0 {
        prj.Link[ju].offset1 = 0.0;
    }

    // --- compute an equivalent length
    prj.Weir[ku].length = 2.0 * prj.RouteStep * (GRAVITY * prj.Link[ju].xsect.yFull).sqrt();
    prj.Weir[ku].length = MAX(200.0, prj.Weir[ku].length);
    prj.Weir[ku].surfArea = 0.0;

    // --- find flow through weir when water level equals weir height
    head = prj.Link[ju].xsect.yFull;
    weir_getFlow(prj, j, k, head, 1.0, FALSE, &mut q1, &mut q2);
    q = q1 + q2;

    // --- compute equivalent orifice coeff. (for CFS flow units)
    head = head / 2.0; // head seen by equivalent orifice
    prj.Weir[ku].cSurcharge = q / head.sqrt();
}

//=============================================================================

fn weir_setSetting(prj: &mut Project, j: i32)
//
//  Input:   j = link index
//  Output:  none
//  Purpose: updates a weir's setting as a result of a control action.
//
{
    let ju = j as usize;
    let k = prj.Link[ju].subIndex;
    let ku = k as usize;
    let mut h: f64;
    let q: f64;
    let mut q1: f64 = 0.0;
    let mut q2: f64 = 0.0;

    // --- adjust weir setting
    prj.Link[ju].setting = prj.Link[ju].targetSetting;
    if prj.Weir[ku].canSurcharge == 0 {
        return;
    }
    if prj.Weir[ku].type_ == ROADWAY_WEIR {
        return;
    }

    // --- find orifice coeff. for surcharged flow
    if prj.Link[ju].setting == 0.0 {
        prj.Weir[ku].cSurcharge = 0.0;
    } else {
        // --- find flow through weir when water level equals weir height
        h = prj.Link[ju].setting * prj.Link[ju].xsect.yFull;
        weir_getFlow(prj, j, k, h, 1.0, FALSE, &mut q1, &mut q2);
        q = q1 + q2;

        // --- compute equivalent orifice coeff. (for CFS flow units)
        h = h / 2.0; // head seen by equivalent orifice
        prj.Weir[ku].cSurcharge = q / h.sqrt();
    }
}

//=============================================================================

fn weir_getInflow(prj: &mut Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  returns weir flow rate (cfs)
//  Purpose: finds the flow over a weir.
//
{
    let ju = j as usize;
    let n1: i32; // index of upstream node
    let n2: i32; // index of downstream node
    let k: i32; // index of weir
    let mut q1: f64 = 0.0; // flow through central part of weir (cfs)
    let mut q2: f64 = 0.0; // flow through end sections of weir (cfs)
    let mut head: f64; // head on weir (ft)
    let mut h1: f64; // upstrm nodal head (ft)
    let mut h2: f64; // downstrm nodal head (ft)
    let mut hcrest: f64; // head at weir crest (ft)
    let hcrown: f64; // head at weir crown (ft)
    let mut y: f64; // water depth in weir (ft)
    let dir: f64; // direction multiplier
    let ratio: f64;
    let weirPower: [f64; 4] = [
        1.5,       // transverse weir
        5. / 3.,   // side flow weir
        2.5,       // v-notch weir
        1.5,       // trapezoidal weir
    ];

    n1 = prj.Link[ju].node1;
    n2 = prj.Link[ju].node2;
    k = prj.Link[ju].subIndex;
    let n1u = n1 as usize;
    let n2u = n2 as usize;
    let ku = k as usize;
    if prj.RouteModel == DW {
        h1 = prj.Node[n1u].newDepth + prj.Node[n1u].invertElev;
        h2 = prj.Node[n2u].newDepth + prj.Node[n2u].invertElev;
    } else {
        h1 = prj.Node[n1u].newDepth + prj.Node[n1u].invertElev;
        h2 = prj.Node[n1u].invertElev;
    }
    dir = if h1 > h2 { 1.0 } else { -1.0 };

    // --- exchange h1 and h2 for reverse flow
    if dir < 0.0 {
        head = h1;
        h1 = h2;
        h2 = head;
    }

    // --- find head of weir's crest and crown
    hcrest = prj.Node[n1u].invertElev + prj.Link[ju].offset1;
    hcrown = hcrest + prj.Link[ju].xsect.yFull;

    // --- treat a roadway weir as a special case
    if prj.Weir[ku].type_ == ROADWAY_WEIR {
        return roadway_getInflow(prj, j, dir, hcrest, h1, h2);
    }

    // --- adjust crest ht. for partially open weir
    hcrest += (1.0 - prj.Link[ju].setting) * prj.Link[ju].xsect.yFull;

    // --- compute head relative to weir crest
    head = h1 - hcrest;

    // --- return if head is negligible or flap gate closed
    prj.Link[ju].dqdh = 0.0;
    if head <= FUDGE || hcrest >= hcrown || link_setFlapGate(prj, j, n1, n2, dir) != 0 {
        prj.Link[ju].newDepth = 0.0;
        prj.Link[ju].flowClass = DRY;
        return 0.0;
    }

    // --- determine flow class
    prj.Link[ju].flowClass = SUBCRITICAL;
    if hcrest > h2 {
        if dir == 1.0 {
            prj.Link[ju].flowClass = DN_CRITICAL;
        } else {
            prj.Link[ju].flowClass = UP_CRITICAL;
        }
    }

    // --- compute new equivalent surface area
    y = prj.Link[ju].xsect.yFull - (hcrown - MIN(h1, hcrown));
    prj.Weir[ku].surfArea = xsect_getWofY(prj, &prj.Link[ju].xsect, y) * prj.Weir[ku].length;

    // --- head is above crown
    if h1 >= hcrown {
        // --- use equivalent orifice if weir can surcharge
        if prj.Weir[ku].canSurcharge != 0 {
            y = (hcrest + hcrown) / 2.0;
            if h2 < y {
                head = h1 - y;
            } else {
                head = h1 - h2;
            }
            y = hcrown - hcrest;
            let cSurcharge = prj.Weir[ku].cSurcharge;
            q1 = weir_getOrificeFlow(prj, j, head, y, cSurcharge);
            prj.Link[ju].newDepth = y;
            return dir * q1;
        }
        // --- otherwise limit head to height of weir opening
        else {
            head = hcrown - hcrest;
        }
    }

    // --- use weir eqn. to find flows through central (q1)
    //     and end sections (q2) of weir
    let hasFlapGate = prj.Link[ju].hasFlapGate;
    weir_getFlow(prj, j, k, head, dir, hasFlapGate, &mut q1, &mut q2);

    // --- apply Villemonte eqn. to correct for submergence
    if h2 > hcrest {
        ratio = (h2 - hcrest) / (h1 - hcrest);
        q1 *= (1.0 - ratio.powf(weirPower[prj.Weir[ku].type_ as usize])).powf(0.385);
        if q2 > 0.0 {
            q2 *= (1.0 - ratio.powf(weirPower[VNOTCH_WEIR as usize])).powf(0.385);
        }
    }

    // --- return total flow through weir
    prj.Link[ju].newDepth = MIN(h1 - hcrest, prj.Link[ju].xsect.yFull);
    dir * (q1 + q2)
}

//=============================================================================

fn weir_getFlow(
    prj: &mut Project,
    j: i32,
    k: i32,
    mut head: f64,
    dir: f64,
    hasFlapGate: i32,
    q1: &mut f64,
    q2: &mut f64,
)
//
//  Input:   j    = link index
//           k    = weir index
//           head = head across weir (ft)
//           dir  = flow direction indicator
//           hasFlapGate = flap gate indicator
//  Output:  q1 = flow through central portion of weir (cfs)
//           q2 = flow through end sections of weir (cfs)
//  Purpose: computes flow over weir given head.
//
{
    let ju = j as usize;
    let ku = k as usize;
    let mut length: f64;
    let h: f64;
    let y: f64;
    let hLoss: f64;
    let area: f64;
    let veloc: f64;
    let mut wType: i32;
    let cdCurve: i32 = prj.Weir[ku].cdCurve;
    let mut cDisch1: f64 = prj.Weir[ku].cDisch1;

    // --- q1 = flow through central portion of weir,
    //     q2 = flow through end sections of trapezoidal weir
    *q1 = 0.0;
    *q2 = 0.0;
    prj.Link[ju].dqdh = 0.0;
    if head <= 0.0 {
        return;
    }

    // --- convert weir length & head to original units
    length = prj.Link[ju].xsect.wMax * UCF(prj, LENGTH);
    h = head * UCF(prj, LENGTH);

    // --- lookup tabulated discharge coeff.
    if cdCurve >= 0 {
        cDisch1 = table_lookup(&mut prj.Curve[cdCurve as usize], h);
    }

    // --- use appropriate formula for weir flow
    wType = prj.Weir[ku].type_;
    if wType == VNOTCH_WEIR && prj.Link[ju].setting < 1.0 {
        wType = TRAPEZOIDAL_WEIR;
    }
    match wType {
        TRANSVERSE_WEIR => {
            // --- reduce length when end contractions present
            length -= 0.1 * prj.Weir[ku].endCon * h;
            length = MAX(length, 0.0);
            *q1 = cDisch1 * length * h.powf(1.5);
        }

        SIDEFLOW_WEIR => {
            // --- reduce length when end contractions present
            length -= 0.1 * prj.Weir[ku].endCon * h;
            length = MAX(length, 0.0);

            // --- weir behaves as a transverse weir under reverse flow
            if dir < 0.0 {
                *q1 = cDisch1 * length * h.powf(1.5);
            } else {
                //  Corrected formula (see Metcalf & Eddy, Inc.,
                //  Wastewater Engineering, McGraw-Hill, 1972 p. 164).
                *q1 = cDisch1 * length.powf(0.83) * h.powf(1.67);
            }
        }

        VNOTCH_WEIR => {
            *q1 = cDisch1 * prj.Weir[ku].slope * h.powf(2.5);
        }

        TRAPEZOIDAL_WEIR => {
            y = (1.0 - prj.Link[ju].setting) * prj.Link[ju].xsect.yFull;
            length = xsect_getWofY(prj, &prj.Link[ju].xsect, y) * UCF(prj, LENGTH);
            *q1 = cDisch1 * length * h.powf(1.5);
            *q2 = prj.Weir[ku].cDisch2 * prj.Weir[ku].slope * h.powf(2.5);
        }
        _ => {}
    }

    // --- convert CMS flows to CFS
    if prj.UnitSystem == SI {
        *q1 /= M3perFT3;
        *q2 /= M3perFT3;
    }

    // --- apply ARMCO adjustment for headloss from flap gate
    if hasFlapGate != 0 {
        // --- compute flow area & velocity for current weir flow
        area = weir_getOpenArea(prj, j, head);
        if area > TINY {
            veloc = (*q1 + *q2) / area;

            // --- compute headloss and subtract from original head
            hLoss = (4.0 / GRAVITY) * veloc * veloc * (-1.15 * veloc / head.sqrt()).exp();
            head = head - hLoss;
            if head < 0.0 {
                head = 0.0;
            }

            // --- make recursive call to this function, with hasFlapGate
            //     set to false, to find flow values at adjusted head value
            weir_getFlow(prj, j, k, head, dir, FALSE, q1, q2);
        }
    }
    prj.Link[ju].dqdh = weir_getdqdh(prj, k, dir, head, *q1, *q2);
}

//=============================================================================

fn weir_getOrificeFlow(prj: &mut Project, j: i32, mut head: f64, y: f64, cOrif: f64) -> f64
//
//  Input:   j = link index
//           head = head across weir (ft)
//           y = height of upstream water level above weir crest (ft)
//           cOrif = orifice flow coefficient
//  Output:  returns flow through weir
//  Purpose: finds flow through a surcharged weir using the orifice equation.
//
{
    let ju = j as usize;
    let a: f64;
    let mut q: f64;
    let v: f64;
    let hloss: f64;

    // --- evaluate the orifice flow equation
    q = cOrif * head.sqrt();

    // --- apply Armco adjustment if weir has a flap gate
    if prj.Link[ju].hasFlapGate != 0 {
        a = weir_getOpenArea(prj, j, y);
        if a > 0.0 {
            v = q / a;
            hloss = (4.0 / GRAVITY) * v * v * (-1.15 * v / y.sqrt()).exp();
            head -= hloss;
            head = MAX(head, 0.0);
            q = cOrif * head.sqrt();
        }
    }
    if head > 0.0 {
        prj.Link[ju].dqdh = q / (2.0 * head);
    } else {
        prj.Link[ju].dqdh = 0.0;
    }
    q
}

//=============================================================================

fn weir_getOpenArea(prj: &Project, j: i32, y: f64) -> f64
//
//  Input:   j = link index
//           y = depth of water above weir crest (ft)
//  Output:  returns area between weir crest and y (ft2)
//  Purpose: finds flow area through a weir.
//
{
    let ju = j as usize;
    let z: f64;
    let mut zy: f64;

    // --- find offset of weir crest due to control setting
    z = (1.0 - prj.Link[ju].setting) * prj.Link[ju].xsect.yFull;

    // --- ht. of crest + ht of water above crest
    zy = z + y;
    zy = MIN(zy, prj.Link[ju].xsect.yFull);

    // --- return difference between area of offset + water depth
    //     and area of just the offset
    xsect_getAofY(prj, &prj.Link[ju].xsect, zy) - xsect_getAofY(prj, &prj.Link[ju].xsect, z)
}

//=============================================================================

fn weir_getdqdh(prj: &Project, k: i32, dir: f64, h: f64, q1: f64, q2: f64) -> f64 {
    let q1h: f64;
    let q2h: f64;

    if h.abs() < FUDGE {
        return 0.0;
    }
    q1h = (q1 / h).abs();
    q2h = (q2 / h).abs();

    match prj.Weir[k as usize].type_ {
        TRANSVERSE_WEIR => return 1.5 * q1h,

        SIDEFLOW_WEIR => {
            // --- weir behaves as a transverse weir under reverse flow
            if dir < 0.0 {
                return 1.5 * q1h;
            } else {
                return 1.67 * q1h;
            }
        }

        VNOTCH_WEIR => {
            if q2h == 0.0 {
                return 2.5 * q1h; // Fully open
            } else {
                return 1.5 * q1h + 2.5 * q2h; // Partly open
            }
        }

        TRAPEZOIDAL_WEIR => return 1.5 * q1h + 2.5 * q2h,
        _ => {}
    }
    0.0
}

//=============================================================================
//               O U T L E T    D E V I C E    M E T H O D S
//=============================================================================

fn outlet_readParams(prj: &mut Project, j: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = link index
//           k = outlet index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads outlet parameters from a tokenized  line of input.
//
{
    let mut i: i32;
    let m: i32;
    let n: i32;
    let n1: i32;
    let n2: i32;
    let mut x: [f64; 6] = [0.0; 6];
    let id: Option<Vec<u8>>;

    // --- check for valid ID and end node IDs
    if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, LINK, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    n1 = project_findObject(prj, NODE, &tok[1]);
    if n1 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    n2 = project_findObject(prj, NODE, &tok[2]);
    if n2 < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }

    // --- get height above invert
    if prj.LinkOffsets == ELEV_OFFSET && tok0(&tok[3]) == b'*' {
        x[0] = MISSING;
    } else {
        if getDouble(&tok[3], &mut x[0]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[3]);
        }
        if prj.LinkOffsets == DEPTH_OFFSET && x[0] < 0.0 {
            x[0] = 0.0;
        }
    }

    // --- see if outlet flow relation is tabular or functional
    m = findmatch(&tok[4], RelationWords);
    if m < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[4]);
    }
    x[1] = 0.0;
    x[2] = 0.0;
    x[3] = -1.0;
    x[4] = 0.0;

    // --- see if rating curve is head or depth based
    x[5] = NODE_DEPTH as f64; //default is depth-based
    //  s = strtok(tok[4], "/");   //parse token for
    //  s = strtok(NULL, "/");     //  qualifier term
    // (strtok writes a NUL into tok[4], which is not read again; the
    //  second token is the qualifier, or NULL if there is none)
    let s: Option<&[u8]> = {
        let mut it = cio::cstr(&tok[4]).split(|&c| c == b'/').filter(|t| !t.is_empty());
        let _ = it.next();
        it.next()
    };
    // C: strcomp(NULL, w_HEAD) dereferences NULL when the token has no
    // "/qualifier" (e.g. plain TABULAR or FUNCTIONAL); EPA's engine
    // crashes there (verified: segmentation fault). Mirror the crash.
    let s = s.expect("outlet_readParams: C strcomp(NULL, \"HEAD\") dereferences NULL (EPA SWMM crashes)");
    if strcomp(s, w_HEAD) != 0 {
        x[5] = NODE_HEAD as f64; //check if its "HEAD"
    }

    // --- get params. for functional outlet device
    if m == FUNCTIONAL {
        if ntoks < 7 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if getDouble(&tok[5], &mut x[1]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[5]);
        }
        if getDouble(&tok[6], &mut x[2]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[6]);
        }
        n = 7;
    }
    // --- get name of outlet rating curve
    else {
        i = project_findObject(prj, CURVE, &tok[5]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[5]);
        }
        x[3] = i as f64;
        n = 6;
    }

    // --- check if flap gate specified
    if ntoks > n {
        i = findmatch(&tok[n as usize], NoYesWords);
        if i < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]);
        }
        x[4] = i as f64;
    }

    // --- add parameters to outlet object
    prj.Link[j as usize].ID = id.unwrap();
    link_setParams(prj, j, OUTLET, n1, n2, k, &x);
    0
}

//=============================================================================

fn outlet_getInflow(prj: &mut Project, j: i32) -> f64
//
//  Input:   j = link index
//  Output:  outlet flow rate (cfs)
//  Purpose: finds the flow through an outlet.
//
{
    let ju = j as usize;
    let k: i32;
    let n1: i32;
    let n2: i32;
    let head: f64;
    let hcrest: f64;
    let mut h1: f64;
    let mut h2: f64;
    let mut y1: f64;
    let dir: f64;

    // --- get indexes of end nodes
    n1 = prj.Link[ju].node1;
    n2 = prj.Link[ju].node2;
    k = prj.Link[ju].subIndex;
    let n1u = n1 as usize;
    let n2u = n2 as usize;
    let ku = k as usize;

    // --- find heads at upstream & downstream nodes
    if prj.RouteModel == DW {
        h1 = prj.Node[n1u].newDepth + prj.Node[n1u].invertElev;
        h2 = prj.Node[n2u].newDepth + prj.Node[n2u].invertElev;
    } else {
        h1 = prj.Node[n1u].newDepth + prj.Node[n1u].invertElev;
        h2 = prj.Node[n1u].invertElev;
    }
    dir = if h1 >= h2 { 1.0 } else { -1.0 };

    // --- exchange h1 and h2 for reverse flow
    y1 = prj.Node[n1u].newDepth;
    if dir < 0.0 {
        y1 = h1;
        h1 = h2;
        h2 = y1;
        y1 = prj.Node[n2u].newDepth;
    }

    // --- for a NODE_DEPTH rating curve the effective head across the
    //     outlet is the depth above the crest elev. while for a NODE_HEAD
    //     curve it is the difference between upstream & downstream heads
    hcrest = prj.Node[n1u].invertElev + prj.Link[ju].offset1;
    if prj.Outlet[ku].curveType == NODE_HEAD && prj.RouteModel == DW {
        head = h1 - MAX(h2, hcrest);
    } else {
        head = h1 - hcrest;
    }

    // --- no flow if either no effective head difference,
    //     no upstream water available, or closed flap gate
    if head <= FUDGE || y1 <= FUDGE || link_setFlapGate(prj, j, n1, n2, dir) != 0 {
        prj.Link[ju].newDepth = 0.0;
        prj.Link[ju].flowClass = DRY;
        return 0.0;
    }

    // --- otherwise use rating curve to compute flow
    prj.Link[ju].newDepth = head;
    prj.Link[ju].flowClass = SUBCRITICAL;
    let setting = prj.Link[ju].setting;
    dir * setting * outlet_getFlow(prj, k, head)
}

//=============================================================================

fn outlet_getFlow(prj: &mut Project, k: i32, head: f64) -> f64
//
//  Input:   k    = outlet index
//           head = head across outlet (ft)
//  Output:  returns outlet flow rate (cfs)
//  Purpose: computes flow rate through an outlet given head.
//
{
    let ku = k as usize;
    let m: i32;
    let h: f64;

    // --- convert head to original units
    h = head * UCF(prj, LENGTH);

    // --- look-up flow in rating curve table if provided
    m = prj.Outlet[ku].qCurve;
    if m >= 0 {
        return table_lookup(&mut prj.Curve[m as usize], h) / UCF(prj, FLOW);
    }
    // --- otherwise use function to find flow
    else {
        return prj.Outlet[ku].qCoeff * h.powf(prj.Outlet[ku].qExpon) / UCF(prj, FLOW);
    }
}

//=============================================================================
