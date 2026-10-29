//! subcatch.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   subcatch.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//
//   Subcatchment runoff functions.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Support added for keeping separate track of drain outflows from LIDs.
//   - Processing of inflow/outflow volumes over a time step was refactored.
//   - Reported subcatchment runoff includes both surface runoff and LID
//     drain flows, even though latter can be routed elsewhere.
//   - Runon now distributed only over non-LID area of a subcatchment, unless
//     LID covers full area.
//   - Pollutant buildup and washoff functions were moved to surfqual.c.
//   Build 5.1.009:
//   - Runon for full LID subcatchment added to statistical summary.
//   Build 5.1.010:
//   - Fixed a bug introduced in 5.1.008 that forgot to include LID
//     exfiltration as inflow sent to GW routine.
//   Build 5.1.011:
//   - Subcatchment percent imperviousness not allowed to exceed 100.
//   Build 5.1.012:
//   - Subcatchment bottom elevation used instead of aquifer's when
//     saving water table value to results file.
//   Build 5.1.013:
//   - Rain gage isUsed property now set in subcatch_validate().
//   - Cumulative impervious and pervious area runoff volumes added
//     to subcatchment statistics.
//   - Support added for monthly adjustment of subcatchment's depression
//     storage, pervious N, and infiltration.
//   Build 5.1.015:
//   - Support added for multiple infiltration methods within a project.
//   - Only pervious area depression storage receives monthly adjustment.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------
const MCOEFF: f64 = 1.49; // constant in Manning Eq.
const MEXP: f64 = 1.6666667; // exponent in Manning Eq.
const ODETOL: f64 = 0.0001; // acceptable error for ODE solver

/// The file-scope variables of subcatch.c.
#[derive(Default)]
pub struct SubcatchState {
    //-------------------------------------------------------------------------
    // Globally shared variables
    //-------------------------------------------------------------------------
    // Volumes (ft3) for a subcatchment over a time step
    pub Vevap: f64,      // evaporation
    pub Vpevap: f64,     // pervious area evaporation
    pub Vinfil: f64,     // non-LID infiltration
    pub Vinflow: f64,    // non-LID precip + snowmelt + runon + ponded water
    pub Voutflow: f64,   // non-LID runoff to subcatchment's outlet
    pub VlidIn: f64,     // impervious area flow to LID units
    pub VlidInfil: f64,  // infiltration from LID units
    pub VlidOut: f64,    // surface outflow from LID units
    pub VlidDrain: f64,  // drain outflow from LID units
    pub VlidReturn: f64, // LID outflow returned to pervious area

    //-------------------------------------------------------------------------
    // Locally shared variables
    //-------------------------------------------------------------------------
    // (C: static TSubarea* theSubarea -- the subarea to which getDdDt() is
    //  applied; it is passed to getDdDt as a copy, see updatePondedDepth)
    Dstore: f64, // monthly adjusted depression storage (ft)
    Alpha: f64,  // monthly adjusted runoff coeff.
}

// static char *RunoffRoutingWords[] = { w_OUTLET,  w_IMPERV, w_PERV, NULL};
const RunoffRoutingWords: &[&[u8]] = &[w_OUTLET, w_IMPERV, w_PERV];

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  subcatch_readParams        (called from parseLine in input.c)
//  subcatch_readSubareaParams (called from parseLine in input.c)
//  subcatch_readLanduseParams (called from parseLine in input.c)
//  subcatch_readInitBuildup   (called from parseLine in input.c)

//  subcatch_validate          (called from project_validate)
//  subcatch_initState         (called from project_init)

//  subcatch_setOldState       (called from runoff_execute)
//  subcatch_getRunon          (called from runoff_execute)
//  subcatch_addRunon          (called from subcatch_getRunon,
//                              lid_addDrainRunon, & runoff_getOutfallRunon)
//  subcatch_getRunoff         (called from runoff_execute)
//  subcatch_hadRunoff         (called from runoff_execute)

//  subcatch_getFracPerv       (called from gwater_initState)
//  subcatch_getStorage        (called from massbal_getRunoffError)
//  subcatch_getDepth          (called from findPondedLoads in surfqual.c)

//  subcatch_getWtdOutflow     (called from addWetWeatherInflows in routing.c)
//  subcatch_getResults        (called from output_saveSubcatchResults)

//=============================================================================

/// Called by: input.
pub fn subcatch_readParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   j = subcatchment index
    //           tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns an error code
    //  Purpose: reads subcatchment parameters from a tokenized  line of input data.
    //
    //  Data has format:
    //    Name  RainGage  Outlet  Area  %Imperv  Width  Slope CurbLength  Snowpack
    //
    let mut i: i32;
    let mut k: i32;
    let mut m: i32;
    let id: Option<Vec<u8>>;
    let mut x = [0.0f64; 9];

    // --- check for enough tokens
    if ntoks < 8 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that named subcatch exists
    id = project_findID(prj, SUBCATCH, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- check that rain gage exists
    k = project_findObject(prj, GAGE, &tok[1]);
    if k < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    x[0] = k as f64;

    // --- check that outlet node or subcatch exists
    m = project_findObject(prj, NODE, &tok[2]);
    x[1] = m as f64;
    m = project_findObject(prj, SUBCATCH, &tok[2]);
    x[2] = m as f64;
    if x[1] < 0.0 && x[2] < 0.0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }

    // --- read area, %imperv, width, slope, & curb length
    i = 3;
    while i < 8 {
        if getDouble(&tok[i as usize], &mut x[i as usize]) == 0 || x[i as usize] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
        }
        i += 1;
    }

    // --- if snowmelt object named, check that it exists
    x[8] = -1.0;
    if ntoks > 8 {
        k = project_findObject(prj, SNOWMELT, &tok[8]);
        if k < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[8]);
        }
        x[8] = k as f64;
    }

    // --- assign input values to subcatch's properties
    let ucfLandArea = UCF(prj, LANDAREA);
    let ucfLength = UCF(prj, LENGTH);
    let sc = &mut prj.Subcatch[j as usize];
    sc.ID = id.unwrap();
    sc.gage = f64_to_i32(x[0]);
    sc.outNode = f64_to_i32(x[1]);
    sc.outSubcatch = f64_to_i32(x[2]);
    sc.area = x[3] / ucfLandArea;
    sc.fracImperv = MIN(x[4], 100.0) / 100.0;
    sc.width = x[5] / ucfLength;
    sc.slope = x[6] / 100.0;
    sc.curbLength = x[7];
    sc.nPervPattern = -1;
    sc.dStorePattern = -1;
    sc.infilPattern = -1;

    // --- create the snow pack object if it hasn't already been created
    if x[8] >= 0.0 {
        if snow_createSnowpack(prj, j, f64_to_i32(x[8])) == 0 {
            return error_setInpError(prj, ERR_MEMORY, b"");
        }
    }
    return 0;
}

//=============================================================================

/// Called by: input.
pub fn subcatch_readSubareaParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns an error code
    //  Purpose: reads subcatchment's subarea parameters from a tokenized
    //           line of input data.
    //
    //  Data has format:
    //    Subcatch  Imperv_N  Perv_N  Imperv_S  Perv_S  PctZero  RouteTo (PctRouted)
    //
    let mut i: i32;
    let j: i32;
    let mut k: i32;
    let m: i32;
    let mut x = [0.0f64; 7];

    // --- check for enough tokens
    if ntoks < 7 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that named subcatch exists
    j = project_findObject(prj, SUBCATCH, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- read in Mannings n, depression storage, & PctZero values
    i = 0;
    while i < 5 {
        if getDouble(&tok[(i + 1) as usize], &mut x[i as usize]) == 0 || x[i as usize] < 0.0 {
            return error_setInpError(prj, ERR_NAME, &tok[(i + 1) as usize]);
        }
        i += 1;
    }

    // --- check for valid runoff routing keyword
    m = findmatch(&tok[6], RunoffRoutingWords);
    if m < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[6]);
    }

    // --- get percent routed parameter if present (default is 100)
    x[5] = m as f64;
    x[6] = 1.0;
    if ntoks >= 8 {
        if getDouble(&tok[7], &mut x[6]) == 0 || x[6] < 0.0 || x[6] > 100.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[7]);
        }
        x[6] /= 100.0;
    }

    // --- assign input values to each type of subarea
    let ucfRainDepth = UCF(prj, RAINDEPTH);
    let sc = &mut prj.Subcatch[j as usize];
    let I0 = IMPERV0 as usize;
    let I1 = IMPERV1 as usize;
    let PV = PERV as usize;
    sc.subArea[I0].N = x[0];
    sc.subArea[I1].N = x[0];
    sc.subArea[PV].N = x[1];

    sc.subArea[I0].dStore = 0.0;
    sc.subArea[I1].dStore = x[2] / ucfRainDepth;
    sc.subArea[PV].dStore = x[3] / ucfRainDepth;

    sc.subArea[I0].fArea = sc.fracImperv * x[4] / 100.0;
    sc.subArea[I1].fArea = sc.fracImperv * (1.0 - x[4] / 100.0);
    sc.subArea[PV].fArea = 1.0 - sc.fracImperv;

    // --- assume that all runoff from each subarea goes to subcatch outlet
    i = IMPERV0;
    while i <= PERV {
        sc.subArea[i as usize].routeTo = TO_OUTLET;
        sc.subArea[i as usize].fOutlet = 1.0;
        i += 1;
    }

    // --- modify routing if pervious runoff routed to impervious area
    //     (fOutlet is the fraction of runoff not routed)

    k = f64_to_i32(x[5]);
    if sc.fracImperv == 0.0 || sc.fracImperv == 1.0 {
        k = TO_OUTLET;
    }
    if k == TO_IMPERV && sc.fracImperv != 0.0 {
        sc.subArea[PV].routeTo = k;
        sc.subArea[PV].fOutlet = 1.0 - x[6];
    }

    // --- modify routing if impervious runoff routed to pervious area
    if k == TO_PERV {
        sc.subArea[I0].routeTo = k;
        sc.subArea[I1].routeTo = k;
        sc.subArea[I0].fOutlet = 1.0 - x[6];
        sc.subArea[I1].fOutlet = 1.0 - x[6];
    }
    return 0;
}

//=============================================================================

/// Called by: input.
pub fn subcatch_readLanduseParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns an error code
    //  Purpose: reads assignment of landuses to subcatchment from a tokenized
    //           line of input data.
    //
    //  Data has format:
    //    Subcatch  landuse  percent .... landuse  percent
    //
    let j: i32;
    let mut k: i32;
    let mut m: i32;
    let mut f: f64 = 0.0;

    // --- check for enough tokens
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that named subcatch exists
    j = project_findObject(prj, SUBCATCH, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- process each pair of landuse - percent items
    k = 2;
    while k <= ntoks {
        // --- check that named land use exists and is followed by a percent
        m = project_findObject(prj, LANDUSE, &tok[(k - 1) as usize]);
        if m < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[(k - 1) as usize]);
        }
        if k + 1 > ntoks {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if getDouble(&tok[k as usize], &mut f) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[k as usize]);
        }

        // --- store land use fraction in subcatch's landFactor property
        prj.Subcatch[j as usize].landFactor[m as usize].fraction = f / 100.0;
        k = k + 2;
    }
    return 0;
}

//=============================================================================

/// Called by: input.
pub fn subcatch_readInitBuildup(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns an error code
    //  Purpose: reads initial pollutant buildup on subcatchment from
    //           tokenized line of input data.
    //
    //  Data has format:
    //    Subcatch  pollut  initLoad .... pollut  initLoad
    //
    let j: i32;
    let mut k: i32;
    let mut m: i32;
    let mut x: f64 = 0.0;

    // --- check for enough tokens
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that named subcatch exists
    j = project_findObject(prj, SUBCATCH, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- process each pair of pollutant - init. load items
    k = 2;
    while k <= ntoks {
        // --- check for valid pollutant name and loading value
        m = project_findObject(prj, POLLUT, &tok[(k - 1) as usize]);
        if m < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[(k - 1) as usize]);
        }
        if k + 1 > ntoks {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if getDouble(&tok[k as usize], &mut x) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[k as usize]);
        }

        // --- store loading in subcatch's initBuildup property
        prj.Subcatch[j as usize].initBuildup[m as usize] = x;
        k = k + 2;
    }
    return 0;
}

//=============================================================================

/// Called by: project.
pub fn subcatch_validate(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: checks for valid subcatchment input parameters.
    //
    let mut i: i32;
    let mut area: f64;
    let ju = j as usize;
    let mut nonLidArea = prj.Subcatch[ju].area;

    // --- check for ambiguous outlet name
    if prj.Subcatch[ju].outNode >= 0 && prj.Subcatch[ju].outSubcatch >= 0 {
        let id = prj.Subcatch[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_SUBCATCH_OUTLET, &id);
    }

    // --- validate subcatchment's groundwater component
    gwater_validate(prj, j);

    // --- validate placement of LIDs in the subcatchment
    nonLidArea -= prj.Subcatch[ju].lidArea;

    // --- compute alpha (i.e. WCON in old SWMM) for overland flow
    //     NOTE: the area which contributes to alpha for both imperv
    //     subareas w/ and w/o depression storage is the total imperv area.
    let sc = &mut prj.Subcatch[ju];
    i = IMPERV0;
    while i <= PERV {
        let iu = i as usize;
        if i == PERV {
            area = (1.0 - sc.fracImperv) * nonLidArea;
        } else {
            area = sc.fracImperv * nonLidArea;
        }
        sc.subArea[iu].alpha = 0.0;

        ////  Possible change to how sub-area width should be assigned.  ////
        ////        area = nonLidArea;
        /////////////////////////////////////////////////////////////////////

        if area > 0.0 && sc.subArea[iu].N > 0.0 {
            sc.subArea[iu].alpha = MCOEFF * sc.width / area * sc.slope.sqrt() / sc.subArea[iu].N;
        }
        i += 1;
    }

    // --- set isUsed property of subcatchment's rain gage
    i = prj.Subcatch[ju].gage;
    if i >= 0 {
        prj.Gage[i as usize].isUsed = TRUE;
    }
}

//=============================================================================

/// Called by: project.
pub fn subcatch_initState(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: Initializes the state of a subcatchment.
    //
    let mut i: i32;
    let ju = j as usize;

    // --- initialize rainfall, runoff, & snow depth
    {
        let sc = &mut prj.Subcatch[ju];
        sc.rainfall = 0.0;
        sc.oldRunoff = 0.0;
        sc.newRunoff = 0.0;
        sc.oldSnowDepth = 0.0;
        sc.newSnowDepth = 0.0;
        sc.runon = 0.0;
        sc.evapLoss = 0.0;
        sc.infilLoss = 0.0;
    }

    // --- initialize state of infiltration, groundwater, & snow pack objects
    if prj.Subcatch[ju].infil == j {
        infil_initState(prj, j);
    }
    if prj.Subcatch[ju].groundwater.is_some() {
        gwater_initState(prj, j);
    }
    if prj.Subcatch[ju].snowpack.is_some() {
        snow_initSnowpack(prj, j);
    }

    // --- initialize state of sub-areas
    i = IMPERV0;
    while i <= PERV {
        let sa = &mut prj.Subcatch[ju].subArea[i as usize];
        sa.depth = 0.0;
        sa.inflow = 0.0;
        sa.runoff = 0.0;
        i += 1;
    }

    // --- initialize runoff quality
    surfqual_initState(prj, j);
}

//=============================================================================

/// Called by: runoff.
pub fn subcatch_setOldState(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: replaces old state of subcatchment with new state.
    //
    let mut i: i32;
    let ju = j as usize;
    let nPollut = prj.Nobjects[POLLUT as usize];
    {
        let sc = &mut prj.Subcatch[ju];
        sc.oldRunoff = sc.newRunoff;
        sc.oldSnowDepth = sc.newSnowDepth;
        i = IMPERV0;
        while i <= PERV {
            sc.subArea[i as usize].inflow = 0.0;
            i += 1;
        }
        i = 0;
        while i < nPollut {
            let iu = i as usize;
            sc.oldQual[iu] = sc.newQual[iu];
            sc.newQual[iu] = 0.0;
            i += 1;
        }
    }
    lid_setOldGroupState(prj, j);
}

//=============================================================================

/// Called by: gwater.
pub fn subcatch_getFracPerv(prj: &Project, j: i32) -> f64 {
    //
    //  Purpose: determines what fraction of subcatchment area, including any LID
    //           area, is pervious.
    //  Input:   j = subcatchment index
    //  Output:  returns fraction of area with pervious cover
    //
    let ju = j as usize;
    let mut fracPerv = 1.0 - prj.Subcatch[ju].fracImperv;

    if prj.Subcatch[ju].lidArea > 0.0 {
        fracPerv = (fracPerv * (prj.Subcatch[ju].area - prj.Subcatch[ju].lidArea) + lid_getPervArea(prj, j))
            / prj.Subcatch[ju].area;
        fracPerv = MIN(fracPerv, 1.0);
    }
    return fracPerv;
}

//=============================================================================

/// Called by: massbal.
pub fn subcatch_getStorage(prj: &Project, j: i32) -> f64 {
    //
    //  Input:   j = subcatchment index
    //  Output:  returns total volume of stored water (ft3)
    //  Purpose: finds total volume of water stored on a subcatchment's surface
    //           and its LIDs at the current time.
    //
    let mut i: i32;
    let mut v = 0.0f64;
    let ju = j as usize;

    i = IMPERV0;
    while i <= PERV {
        v += prj.Subcatch[ju].subArea[i as usize].depth * prj.Subcatch[ju].subArea[i as usize].fArea;
        i += 1;
    }
    return v * (prj.Subcatch[ju].area - prj.Subcatch[ju].lidArea) + lid_getStoredVolume(prj, j);
}

//=============================================================================

/// Called by: runoff.
pub fn subcatch_getRunon(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: Routes runoff from a subcatchment to its outlet subcatchment
    //           or between its subareas.
    //
    let k: i32; // outlet subcatchment index
    let mut p: i32; // pollutant index
    let mut q: f64; // runon to outlet subcatchment (ft/sec)
    let q1: f64; // runoff from imperv. areas (ft/sec)
    let q2: f64;
    let pervArea: f64; // subcatchment pervious area (ft2)
    let ju = j as usize;
    let I0 = IMPERV0 as usize;
    let I1 = IMPERV1 as usize;
    let PV = PERV as usize;

    // --- add previous period's runoff from this subcatchment to the
    //     runon of the outflow subcatchment, if it exists
    k = prj.Subcatch[ju].outSubcatch;
    q = prj.Subcatch[ju].oldRunoff;
    if k >= 0 && k != j {
        subcatch_addRunonFlow(prj, k, q);
        let ku = k as usize;
        p = 0;
        while p < prj.Nobjects[POLLUT as usize] {
            let pu = p as usize;
            let w = q * prj.Subcatch[ju].oldQual[pu] * LperFT3;
            prj.Subcatch[ku].newQual[pu] += w;
            p += 1;
        }
    }

    // --- add any LID underdrain flow sent from this subcatchment to
    //     other subcatchments
    if prj.Subcatch[ju].lidArea > 0.0 {
        lid_addDrainRunon(prj, j);
    }

    // --- add to sub-area inflow any outflow from other subarea in previous period
    //     (NOTE: no transfer of runoff pollutant load, since runoff loads are
    //     based on runoff flow from entire subcatchment.)

    // --- Case 1: imperv --> perv
    if prj.Subcatch[ju].fracImperv < 1.0 && prj.Subcatch[ju].subArea[I0].routeTo == TO_PERV {
        let sc = &mut prj.Subcatch[ju];
        // --- add area-wtd. outflow from imperv1 subarea to perv area inflow
        q1 = sc.subArea[I0].runoff * sc.subArea[I0].fArea;
        q2 = sc.subArea[I1].runoff * sc.subArea[I1].fArea;
        q = q1 + q2;
        sc.subArea[PV].inflow += q * (1.0 - sc.subArea[I0].fOutlet) / sc.subArea[PV].fArea;
    }

    // --- Case 2: perv --> imperv
    if prj.Subcatch[ju].fracImperv > 0.0
        && prj.Subcatch[ju].subArea[PV].routeTo == TO_IMPERV
        && prj.Subcatch[ju].subArea[I1].fArea > 0.0
    {
        let sc = &mut prj.Subcatch[ju];
        q = sc.subArea[PV].runoff;
        sc.subArea[I1].inflow += q * (1.0 - sc.subArea[PV].fOutlet) * sc.subArea[PV].fArea / sc.subArea[I1].fArea;
    }

    // --- Add any return flow from LID units to pervious subarea
    if prj.Subcatch[ju].lidArea > 0.0 && prj.Subcatch[ju].fracImperv < 1.0 {
        pervArea = prj.Subcatch[ju].subArea[PV].fArea * (prj.Subcatch[ju].area - prj.Subcatch[ju].lidArea);
        q = lid_getFlowToPerv(prj, j);
        if pervArea > 0.0 {
            prj.Subcatch[ju].subArea[PV].inflow += q / pervArea;
        }
    }
}

//=============================================================================

/// Called by: lid, runoff.
pub fn subcatch_addRunonFlow(prj: &mut Project, k: i32, q: f64) {
    //
    //  Input:   k = subcatchment index
    //           q = runon flow rate (cfs) to subcatchment k
    //  Output:  none
    //  Purpose: Updates the total runon flow (ft/s) seen by a subcatchment that
    //           receives runon flow from an upstream subcatchment.
    //
    let mut q = q;
    let mut i: i32;
    let nonLidArea: f64;
    let sc = &mut prj.Subcatch[k as usize];

    // --- distribute runoff from upstream subcatchment (in cfs)
    //     uniformly over the non-LID area of current subcatchment (ft/sec)
    if sc.area <= 0.0 {
        return;
    }
    nonLidArea = sc.area - sc.lidArea;
    if nonLidArea > 0.0 {
        q = q / nonLidArea;
    } else {
        q = q / sc.area;
    }
    sc.runon += q;

    // --- assign this flow to the 3 types of subareas
    i = IMPERV0;
    while i <= PERV {
        sc.subArea[i as usize].inflow += q;
        i += 1;
    }
}

//=============================================================================

/// Called by: runoff.
#[allow(unused_assignments)] // C's initializers of vRunon and vOutflow
pub fn subcatch_getRunoff(prj: &mut Project, j: i32, tStep: f64) -> f64 {
    //
    //  Input:   j = subcatchment index
    //           tStep = time step (sec)
    //  Output:  returns total runoff produced by subcatchment (ft/sec)
    //  Purpose: Computes runoff & new storage depth for subcatchment.
    //
    //  The 'runoff' value returned by this function is the total runoff
    //  generated (in ft/sec) by the subcatchment before any internal
    //  re-routing is applied. It is used to compute pollutant washoff.
    //
    //  The 'outflow' value computed here (in cfs) is the surface runoff
    //  that actually leaves the subcatchment after any LID controls are
    //  applied and is saved to Subcatch[j].newRunoff.
    //
    let mut i: i32; // subarea index
    let nonLidArea: f64; // non-LID portion of subcatch area (ft2)
    let mut area: f64; // sub-area or subcatchment area (ft2)
    let mut netPrecip = [0.0f64; 3]; // subarea net precipitation (ft/sec)
    let vRain: f64; // rainfall (+ snowfall) volume (ft3)
    let mut vRunon: f64 = 0.0; // runon volume from other areas (ft3)
    let mut vOutflow: f64 = 0.0; // runoff volume leaving subcatch (ft3)
    let mut runoff: f64 = 0.0; // total runoff flow on subcatch (cfs)
    let evapRate: f64; // potential evaporation rate (ft/sec)
    let mut subAreaRunoff: f64; // sub-area runoff rate (cfs)
    let mut vImpervRunoff: f64 = 0.0; // impervious area runoff volume (ft3)
    let mut vPervRunoff: f64 = 0.0; // pervious area runoff volume (ft3)
    let ju = j as usize;

    // --- initialize shared water balance variables
    prj.subcatch.Vevap = 0.0;
    prj.subcatch.Vpevap = 0.0;
    prj.subcatch.Vinfil = 0.0;
    prj.subcatch.Voutflow = 0.0;
    prj.subcatch.VlidIn = 0.0;
    prj.subcatch.VlidInfil = 0.0;
    prj.subcatch.VlidOut = 0.0;
    prj.subcatch.VlidDrain = 0.0;
    prj.subcatch.VlidReturn = 0.0;

    // --- find volume of inflow to non-LID portion of subcatchment as existing
    //     ponded water + any runon volume from upstream areas;
    //     rainfall and snowmelt will be added as each sub-area is analyzed
    nonLidArea = prj.Subcatch[ju].area - prj.Subcatch[ju].lidArea;
    vRunon = prj.Subcatch[ju].runon * tStep * nonLidArea;
    prj.subcatch.Vinflow = vRunon + subcatch_getDepth(prj, j) * nonLidArea;

    // --- find LID runon only if LID occupies full subcatchment
    if nonLidArea == 0.0 {
        vRunon = prj.Subcatch[ju].runon * tStep * prj.Subcatch[ju].area;
    }

    // --- get net precip. (rainfall + snowfall + snowmelt) on the 3 types
    //     of subcatchment sub-areas and update Vinflow with it
    getNetPrecip(prj, j, &mut netPrecip, tStep);

    // --- find potential evaporation rate
    if prj.Evap.dryOnly != 0 && prj.Subcatch[ju].rainfall > 0.0 {
        evapRate = 0.0;
    } else {
        evapRate = prj.Evap.rate;
    }

    // --- set monthly infiltration adjustment factor
    infil_setInfilFactor(prj, j);

    // --- examine each type of sub-area (impervious w/o depression storage,
    //     impervious w/ depression storage, and pervious)
    if nonLidArea > 0.0 {
        i = IMPERV0;
        while i <= PERV {
            let iu = i as usize;
            // --- get runoff from sub-area updating Vevap, Vpevap,
            //     Vinfil & Voutflow)
            area = nonLidArea * prj.Subcatch[ju].subArea[iu].fArea;
            let r = getSubareaRunoff(prj, j, i, area, netPrecip[iu], evapRate, tStep);
            prj.Subcatch[ju].subArea[iu].runoff = r;
            subAreaRunoff = prj.Subcatch[ju].subArea[iu].runoff * area;
            if i == PERV {
                vPervRunoff = subAreaRunoff * tStep;
            } else {
                vImpervRunoff += subAreaRunoff * tStep;
            }
            runoff += subAreaRunoff;
            i += 1;
        }
    }

    // --- evaluate any LID treatment provided (updating Vevap,
    //     Vpevap, VlidInfil, VlidIn, VlidOut, & VlidDrain)
    if prj.Subcatch[ju].lidArea > 0.0 {
        lid_getRunoff(prj, j, tStep);
    }

    // --- update groundwater levels & flows if applicable
    if prj.IgnoreGwater == 0 && prj.Subcatch[ju].groundwater.is_some() {
        let Vpevap = prj.subcatch.Vpevap;
        let vInfil = prj.subcatch.Vinfil + prj.subcatch.VlidInfil;
        gwater_getGroundwater(prj, j, Vpevap, vInfil, tStep);
    }

    // --- save subcatchment's total loss rates (ft/s)
    area = prj.Subcatch[ju].area;
    prj.Subcatch[ju].evapLoss = prj.subcatch.Vevap / tStep / area;
    prj.Subcatch[ju].infilLoss = (prj.subcatch.Vinfil + prj.subcatch.VlidInfil) / tStep / area;

    // --- find net surface runoff volume
    //     (VlidDrain accounts for LID drain flows)
    vOutflow = prj.subcatch.Voutflow // runoff from all non-LID areas
        - prj.subcatch.VlidIn // runoff treated by LID units
        + prj.subcatch.VlidOut; // runoff from LID units
    prj.Subcatch[ju].newRunoff = vOutflow / tStep;

    // --- obtain external precip. volume (without any snowmelt)
    vRain = prj.Subcatch[ju].rainfall * tStep * area;

    // --- update the cumulative stats for this subcatchment
    let Vevap = prj.subcatch.Vevap;
    let vInfilTotal = prj.subcatch.Vinfil + prj.subcatch.VlidInfil;
    let VlidDrain = prj.subcatch.VlidDrain;
    let runoffRate = prj.Subcatch[ju].newRunoff + VlidDrain / tStep;
    stats_updateSubcatchStats(
        prj,
        j,
        vRain,
        vRunon,
        Vevap,
        vInfilTotal,
        vImpervRunoff,
        vPervRunoff,
        vOutflow + VlidDrain,
        runoffRate,
    );

    // --- include this subcatchment's contribution to overall flow balance
    //     only if its outlet is a drainage system node
    if prj.Subcatch[ju].outNode == -1 && prj.Subcatch[ju].outSubcatch != j {
        vOutflow = 0.0;
    }

    // --- update mass balances
    massbal_updateRunoffTotals(prj, RUNOFF_RAINFALL, vRain);
    let Vevap = prj.subcatch.Vevap;
    massbal_updateRunoffTotals(prj, RUNOFF_EVAP, Vevap);
    let vInfilTotal = prj.subcatch.Vinfil + prj.subcatch.VlidInfil;
    massbal_updateRunoffTotals(prj, RUNOFF_INFIL, vInfilTotal);
    massbal_updateRunoffTotals(prj, RUNOFF_RUNOFF, vOutflow);

    // --- return area-averaged runoff (ft/s)
    return runoff / area;
}

//=============================================================================

fn getNetPrecip(prj: &mut Project, j: i32, netPrecip: &mut [f64], tStep: f64) {
    //
    //  Purpose: Finds combined rainfall + snowmelt on a subcatchment.
    //  Input:   j = subcatchment index
    //           tStep = time step (sec)
    //  Output:  netPrecip = rainfall + snowmelt over each type of subarea (ft/s)
    //
    let mut i: i32;
    let k: i32;
    let mut rainfall = 0.0f64; // rainfall (ft/sec)
    let mut snowfall = 0.0f64; // snowfall (ft/sec)
    let ju = j as usize;

    // --- get current rainfall or snowfall from rain gage (in ft/sec)
    k = prj.Subcatch[ju].gage;
    if k >= 0 {
        gage_getPrecip(prj, k, &mut rainfall, &mut snowfall);
    }

    // --- assign total precip. rate to subcatch's rainfall property
    prj.Subcatch[ju].rainfall = rainfall + snowfall;

    // --- determine net precipitation input (netPrecip) to each sub-area

    // --- if subcatch has a snowpack, then base netPrecip on possible snow melt
    if prj.Subcatch[ju].snowpack.is_some() && prj.IgnoreSnowmelt == 0 {
        let d = snow_getSnowMelt(prj, j, rainfall, snowfall, tStep, netPrecip);
        prj.Subcatch[ju].newSnowDepth = d;
    }
    // --- otherwise netPrecip is just sum of rainfall & snowfall
    else {
        i = IMPERV0;
        while i <= PERV {
            netPrecip[i as usize] = rainfall + snowfall;
            i += 1;
        }
    }
}

//=============================================================================

/// Called by: surfqual.
pub fn subcatch_getDepth(prj: &Project, j: i32) -> f64 {
    //
    //  Input:   j = subcatchment index
    //  Output:  returns average depth of ponded water (ft)
    //  Purpose: finds average depth of water over the non-LID portion of a
    //           subcatchment
    //
    let mut i: i32;
    let mut fArea: f64;
    let mut depth = 0.0f64;
    let ju = j as usize;

    i = IMPERV0;
    while i <= PERV {
        fArea = prj.Subcatch[ju].subArea[i as usize].fArea;
        if fArea > 0.0 {
            depth += prj.Subcatch[ju].subArea[i as usize].depth * fArea;
        }
        i += 1;
    }
    return depth;
}

//=============================================================================

/// Called by: routing.
pub fn subcatch_getWtdOutflow(prj: &Project, j: i32, f: f64) -> f64 {
    //
    //  Input:   j = subcatchment index
    //           f = weighting factor.
    //  Output:  returns weighted runoff value
    //  Purpose: computes wtd. combination of old and new subcatchment runoff.
    //
    let ju = j as usize;
    if prj.Subcatch[ju].area == 0.0 {
        return 0.0;
    }
    return (1.0 - f) * prj.Subcatch[ju].oldRunoff + f * prj.Subcatch[ju].newRunoff;
}

//=============================================================================

/// Called by: output, runoff.
pub fn subcatch_getResults(prj: &mut Project, j: i32, f: f64, x: &mut [f32]) {
    //
    //  Input:   j = subcatchment index
    //           f = weighting factor
    //  Output:  x = array of results
    //  Purpose: computes wtd. combination of old and new subcatchment results.
    //
    let mut p: i32; // pollutant index
    let k: i32; // rain gage index
    let f1 = 1.0 - f;
    let mut z: f64;
    let mut runoff: f64;
    let ju = j as usize;

    // --- retrieve rainfall for current report period
    k = prj.Subcatch[ju].gage;
    if k >= 0 {
        x[SUBCATCH_RAINFALL as usize] = prj.Gage[k as usize].reportRainfall as f32;
    } else {
        x[SUBCATCH_RAINFALL as usize] = 0.0f32;
    }

    // --- retrieve snow depth
    z = (f1 * prj.Subcatch[ju].oldSnowDepth + f * prj.Subcatch[ju].newSnowDepth) * UCF(prj, RAINDEPTH);
    x[SUBCATCH_SNOWDEPTH as usize] = z as f32;

    // --- retrieve runoff and losses
    x[SUBCATCH_EVAP as usize] = (prj.Subcatch[ju].evapLoss * UCF(prj, EVAPRATE)) as f32;
    x[SUBCATCH_INFIL as usize] = (prj.Subcatch[ju].infilLoss * UCF(prj, RAINFALL)) as f32;
    runoff = f1 * prj.Subcatch[ju].oldRunoff + f * prj.Subcatch[ju].newRunoff;

    // --- add any LID drain flow to reported runoff
    if prj.Subcatch[ju].lidArea > 0.0 {
        runoff += f1 * lid_getDrainFlow(prj, j, PREVIOUS) + f * lid_getDrainFlow(prj, j, CURRENT);
    }

    // --- if runoff is really small, report it as zero
    if runoff < MIN_RUNOFF * prj.Subcatch[ju].area {
        runoff = 0.0;
    }
    x[SUBCATCH_RUNOFF as usize] = (runoff * UCF(prj, FLOW)) as f32;

    // --- retrieve groundwater results
    if let Some(gw) = prj.Subcatch[ju].groundwater.as_deref() {
        z = (f1 * gw.oldFlow + f * gw.newFlow) * prj.Subcatch[ju].area * UCF(prj, FLOW);
        x[SUBCATCH_GW_FLOW as usize] = z as f32;
        z = (gw.bottomElev + gw.lowerDepth) * UCF(prj, LENGTH);
        x[SUBCATCH_GW_ELEV as usize] = z as f32;
        z = gw.theta;
        x[SUBCATCH_SOIL_MOIST as usize] = z as f32;
    } else {
        x[SUBCATCH_GW_FLOW as usize] = 0.0f32;
        x[SUBCATCH_GW_ELEV as usize] = 0.0f32;
        x[SUBCATCH_SOIL_MOIST as usize] = 0.0f32;
    }

    // --- retrieve pollutant washoff
    if prj.IgnoreQuality == 0 {
        p = 0;
        while p < prj.Nobjects[POLLUT as usize] {
            let pu = p as usize;
            if runoff == 0.0 {
                z = 0.0;
            } else {
                z = f1 * prj.Subcatch[ju].oldQual[pu] + f * prj.Subcatch[ju].newQual[pu];
            }
            x[(SUBCATCH_WASHOFF + p) as usize] = z as f32;
            p += 1;
        }
    }
}

//=============================================================================
//                              SUB-AREA METHODS
//=============================================================================

fn getSubareaRunoff(prj: &mut Project, j: i32, i: i32, area: f64, precip: f64, evap: f64, tStep: f64) -> f64 {
    //
    //  Purpose: computes runoff & losses from a subarea over the current time step.
    //  Input:   j = subcatchment index
    //           i = subarea index
    //           area = sub-area area (ft2)
    //           precip = rainfall + snowmelt over subarea (ft/sec)
    //           evap = evaporation (ft/sec)
    //           tStep = time step (sec)
    //  Output:  returns runoff rate from the sub-area (cfs);
    //           updates shared variables Vinflow, Vevap, Vpevap, Vinfil & Voutflow.
    //
    let mut tRunoff: f64; // time over which runoff occurs (sec)
    let mut surfMoisture: f64; // surface water available (ft/sec)
    let surfEvap: f64; // evap. used for surface water (ft/sec)
    let mut infil = 0.0f64; // infiltration rate (ft/sec)
    let runoff: f64; // runoff rate (ft/sec)

    // --- no runoff if no area
    if area == 0.0 {
        return 0.0;
    }

    // --- assign pointer to current subarea
    //     (a copy, written back at the end; nothing called below reaches
    //      Subcatch[j].subArea[i] through prj)
    let ju = j as usize;
    let iu = i as usize;
    let mut subarea: TSubarea = prj.Subcatch[ju].subArea[iu];

    // --- assume runoff occurs over entire time step
    tRunoff = tStep;

    // --- determine evaporation loss rate
    surfMoisture = subarea.depth / tStep;
    surfEvap = MIN(surfMoisture, evap);

    // --- compute infiltration loss rate
    if i == PERV {
        infil = getSubareaInfil(prj, j, &subarea, precip, tStep);
    }

    // --- add precip to other subarea inflows
    subarea.inflow += precip;
    surfMoisture += subarea.inflow;

    // --- update total inflow, evaporation & infiltration volumes
    prj.subcatch.Vinflow += precip * area * tStep;
    prj.subcatch.Vevap += surfEvap * area * tStep;
    if i == PERV {
        prj.subcatch.Vpevap += prj.subcatch.Vevap;
    }
    prj.subcatch.Vinfil += infil * area * tStep;

    // --- assign adjusted runoff coeff. & storage to shared variables
    prj.subcatch.Alpha = subarea.alpha;
    prj.subcatch.Dstore = subarea.dStore;
    adjustSubareaParams(prj, i, j);

    // --- if losses exceed available moisture then no ponded water remains
    if surfEvap + infil >= surfMoisture {
        subarea.depth = 0.0;
    }
    // --- otherwise reduce inflow by losses and update depth
    //     of ponded water and time over which runoff occurs
    else {
        subarea.inflow -= surfEvap + infil;
        updatePondedDepth(prj, &mut subarea, &mut tRunoff);
    }

    // --- compute runoff based on updated ponded depth
    runoff = findSubareaRunoff(prj, &mut subarea, tRunoff);

    // --- compute runoff volume leaving subcatchment for mass balance purposes
    //     (fOutlet is the fraction of this subarea's runoff that goes to the
    //     subcatchment outlet as opposed to another subarea of the subcatchment)
    prj.subcatch.Voutflow += subarea.fOutlet * runoff * area * tStep;
    prj.Subcatch[ju].subArea[iu] = subarea;
    return runoff;
}

//=============================================================================

fn getSubareaInfil(prj: &mut Project, j: i32, subarea: &TSubarea, precip: f64, tStep: f64) -> f64 {
    //
    //  Purpose: computes infiltration rate at current time step.
    //  Input:   j = subcatchment index
    //           subarea = ptr. to a subarea
    //           precip = rainfall + snowmelt over subarea (ft/sec)
    //           tStep = time step (sec)
    //  Output:  returns infiltration rate (ft/s)
    //
    let mut infil: f64; // actual infiltration rate (ft/sec)

    // --- compute infiltration rate
    infil = infil_getInfil(prj, j, tStep, precip, subarea.inflow, subarea.depth);

    // --- limit infiltration rate by available void space in unsaturated
    //     zone of any groundwater aquifer
    if prj.IgnoreGwater == 0 {
        if let Some(gw) = prj.Subcatch[j as usize].groundwater.as_deref() {
            infil = MIN(infil, gw.maxInfilVol / tStep);
        }
    }
    return infil;
}

//=============================================================================

fn findSubareaRunoff(prj: &Project, subarea: &mut TSubarea, tRunoff: f64) -> f64 {
    //
    //  Purpose: computes runoff (ft/s) from subarea after current time step.
    //  Input:   subarea = ptr. to a subarea
    //           tRunoff = time step over which runoff occurs (sec)
    //  Output:  returns runoff rate (ft/s)
    //
    let Dstore = prj.subcatch.Dstore;
    let Alpha = prj.subcatch.Alpha;
    let xDepth = subarea.depth - Dstore;
    let mut runoff: f64;

    if xDepth > ZERO {
        // --- case where nonlinear routing is used
        if subarea.N > 0.0 {
            runoff = Alpha * xDepth.powf(MEXP);
        }
        // --- case where no routing is used (Mannings N = 0)
        else {
            runoff = xDepth / tRunoff;
            subarea.depth = Dstore;
        }
    } else {
        runoff = 0.0;
    }
    return runoff;
}

//=============================================================================

fn updatePondedDepth(prj: &Project, subarea: &mut TSubarea, dt: &mut f64) {
    //
    //  Input:   subarea = ptr. to a subarea,
    //           dt = time step (sec)
    //  Output:  dt = time ponded depth is above depression storage (sec)
    //  Purpose: computes new ponded depth over subarea after current time step.
    //
    let Dstore = prj.subcatch.Dstore;
    let Alpha = prj.subcatch.Alpha;
    let ix = subarea.inflow; // excess inflow to subarea (ft/sec)
    let dx: f64; // depth above depression storage (ft)
    let mut tx = *dt; // time over which dx > 0 (sec)

    // --- see if not enough inflow to fill depression storage (dStore)
    if subarea.depth + ix * tx <= Dstore {
        subarea.depth += ix * tx;
    }
    // --- otherwise use the ODE solver to integrate flow depth
    else {
        // --- if depth < Dstore then fill up Dstore & reduce time step
        dx = Dstore - subarea.depth;
        if dx > 0.0 && ix > 0.0 {
            tx -= dx / ix;
            subarea.depth = Dstore;
        }

        // --- now integrate depth over remaining time step tx
        if Alpha > 0.0 && tx > 0.0 {
            // (C: theSubarea = subarea; getDdDt reads only its inflow,
            //  which does not change during the integration)
            let theSubarea: TSubarea = *subarea;
            odesolve_integrate(
                std::slice::from_mut(&mut subarea.depth),
                1,
                0.0,
                tx,
                ODETOL,
                tx,
                &mut |t: f64, d: &[f64], dddt: &mut [f64]| getDdDt(&theSubarea, Dstore, Alpha, t, d, dddt),
            );
        } else {
            if tx < 0.0 {
                tx = 0.0;
            }
            subarea.depth += ix * tx;
        }
    }

    // --- do not allow ponded depth to go negative
    if subarea.depth < 0.0 {
        subarea.depth = 0.0;
    }

    // --- replace original time step with time ponded depth
    //     is above depression storage
    *dt = tx;
}

//=============================================================================

fn getDdDt(theSubarea: &TSubarea, Dstore: f64, Alpha: f64, t: f64, d: &[f64], dddt: &mut [f64]) {
    //
    //  Input:   t = current time (not used)
    //           d = stored depth (ft)
    //  Output   dddt = derivative of d with respect to time
    //  Purpose: evaluates derivative of stored depth w.r.t. time
    //           for the subarea whose runoff is being computed.
    //
    //  (theSubarea, Dstore and Alpha are the module-level variables of C)
    let ix = theSubarea.inflow;
    let mut rx = d[0] - Dstore;
    if rx < 0.0 {
        rx = 0.0;
    } else {
        rx = Alpha * rx.powf(MEXP);
    }
    dddt[0] = ix - rx;
}

//=============================================================================

fn adjustSubareaParams(prj: &mut Project, i: i32, j: i32) {
    //
    //  Input:   i = type of subarea being analyzed
    //           j = index of current subcatchment being analyzed
    //  Output   adjusted values of module-level variables Dstore & Alpha
    //  Purpose: adjusts a pervious subarea's depression storage and its
    //           runoff coeff. by month of the year.
    //
    let mut p: i32; // monthly pattern index
    let mut m: i32; // current month of the year
    let mut f: f64; // adjustment factor
    let ju = j as usize;

    if i == PERV {
        // --- depression storage adjustment
        p = prj.Subcatch[ju].dStorePattern;
        if p >= 0 && prj.Pattern[p as usize].type_ == MONTHLY_PATTERN {
            m = datetime_monthOfYear(getDateTime(prj, prj.OldRunoffTime)) - 1;
            f = prj.Pattern[p as usize].factor[m as usize];
            if f >= 0.0 {
                prj.subcatch.Dstore *= f;
            }
        }

        // --- roughness adjustment to runoff coeff.
        p = prj.Subcatch[ju].nPervPattern;
        if p >= 0 && prj.Pattern[p as usize].type_ == MONTHLY_PATTERN {
            m = datetime_monthOfYear(getDateTime(prj, prj.OldRunoffTime)) - 1;
            f = prj.Pattern[p as usize].factor[m as usize];
            if f <= 0.0 {
                prj.subcatch.Alpha = 0.0;
            } else {
                prj.subcatch.Alpha /= f;
            }
        }
    }
}
