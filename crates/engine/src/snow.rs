//! snow.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   snow.c
//
//   Project: EPA SWMM5
//   Version: 5.2
//   Date:    11/01/21   (Build 5.2.0)
//   Author:  L. Rossman
//
//   Models snow melt processes.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Adjustment of snowmelt and subcatchment's net precipitation for area
//     covered by snow was corrected.
//   - Area covered by snow now included in calculation of rate that liquid
//     water leaves a snowpack.
//   Build 5.2.0:
//   - Subcatchment snow pack area should not include LID area.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------
// These symbolize the keywords listed in SnowmeltWords in keywords.c
// enum SnowKeywords {SNOW_PLOWABLE, SNOW_IMPERV, SNOW_PERV, SNOW_REMOVAL};
const SNOW_PLOWABLE: i32 = 0;
const SNOW_IMPERV: i32 = 1;
const SNOW_PERV: i32 = 2;
const SNOW_REMOVAL: i32 = 3;

/// The file-scope variables of snow.c (it has none).
#[derive(Default)]
pub struct SnowState {
}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  snow_createSnowpack  (called from subcatch_setParams)
//  snow_initSnowpack    (called from subcatch_initState)
//  snow_initSnowmelt    (called from createObjects in project.c)
//  snow_validateSnowmelt(called from project_validate)
//  snow_readMeltParams  (called from parseLine in input.c)
//  snow_setMeltCoeffs   (called from setTemp in climate.c)
//  snow_plowSnow        (called from runoff_execute)
//  snow_getSnowMelt     (called from subcatch_getRunoff)
//  snow_getSnowCover    (called from massbal_open)
//  snow_getState        (called from saveRunoff in hotstart.c)

//=============================================================================

/// Called by: input.
pub fn snow_readMeltParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //  Purpose: reads snow melt parameters from a tokenized line of input data.
    //
    //  Format of data are:
    //   Name  SubArea   Cmin  Cmax  Tbase  FWF  SD0  FW0  SNN0/SD100
    //   Name  REMOVAL   SDplow Fout Fimperv Fperv Fimelt Fsubcatch (Subcatch)
    //
    let mut i: i32;
    let j: i32;
    let k: i32;
    let m: i32;
    let mut n: i32;
    let mut x = [0.0f64; 7];
    if ntoks < 8 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- save snow melt parameter set name if not already done so
    j = project_findObject(prj, SNOWMELT, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    // (C: Snowmelt[j].ID == NULL; an unnamed set has an empty ID here)
    if prj.Snowmelt[j as usize].ID.is_empty() {
        prj.Snowmelt[j as usize].ID = project_findID(prj, SNOWMELT, &tok[0]).unwrap_or_default();
    }

    // --- identify data keyword
    k = findmatch(&tok[1], SnowmeltWords);
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }

    // --- number of parameters to read
    n = 7; // 7 for subareas
    if k == SNOW_REMOVAL {
        n = 6; // 6 for Removal
    }
    if ntoks < n + 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    i = 0;
    while i < 7 {
        x[i as usize] = 0.0;
        i += 1;
    }

    // --- parse each parameter
    i = 0;
    while i < n {
        if getDouble(&tok[(i + 2) as usize], &mut x[i as usize]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[(i + 2) as usize]);
        }
        i += 1;
    }

    // --- parse name of subcatch receiving snow plowed from current subcatch
    if k == SNOW_REMOVAL {
        x[6] = -1.0;
        if ntoks >= 9 {
            m = project_findObject(prj, SUBCATCH, &tok[8]);
            if m < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[8]);
            }
            x[6] = m as f64;
        }
    }

    // --- save snow melt parameters
    setMeltParams(prj, j, k, &mut x);
    return 0;
}

//=============================================================================

/// Called by: subcatch.
pub fn snow_createSnowpack(prj: &mut Project, j: i32, k: i32) -> i32 {
    //
    //  Input:   j = subcatchment index
    //           k = snow melt parameter set index
    //  Output:  returns TRUE if successful
    //  Purpose: creates a snowpack object for a subcacthment.
    //
    // NOTE: C mallocs the snowpack without initializing it. snow_initSnowpack
    // sets fArea, wsnow, fw, coldc, ati and awe before the run; imelt, sba
    // and sbws are left as whatever malloc returned. Zeros are used here
    // (see the notes at snow_getSnowMelt and getArealDepletion).
    let mut snowpack: TSnowpack = TSnowpack::default();
    snowpack.snowmeltIndex = k;
    prj.Subcatch[j as usize].snowpack = Some(Box::new(snowpack));
    return TRUE;
}

//=============================================================================

/// Called by: subcatch.
pub fn snow_initSnowpack(prj: &mut Project, j: i32) {
    //
    //  Input:   j = subcatchment index
    //  Output:  none
    //  Purpose: initializes state of a subcatchment's snow pack.
    //
    let mut i: i32; // snow sub-area index
    let k: i32; // snowmelt parameter set index
    let f: f64; // fraction of impervious area plowable
    let mut snowDepth = 0.0f64; // snow depth on entire subcatchment (ft)

    let ju = j as usize;
    let fracImperv = prj.Subcatch[ju].fracImperv;

    // --- get ptr. to subcatchment's snow pack object
    let snowpack: &mut TSnowpack = match prj.Subcatch[ju].snowpack.as_deref_mut() {
        None => return,
        Some(s) => s,
    };

    // --- identify index of snow melt data set used by snow pack
    k = snowpack.snowmeltIndex;
    let sm = &prj.Snowmelt[k as usize];

    // --- find fractional area of each snow surface
    f = sm.snn;
    snowpack.fArea[SNOW_PLOWABLE as usize] = f * fracImperv;
    snowpack.fArea[SNOW_IMPERV as usize] = (1.0 - f) * fracImperv;
    snowpack.fArea[SNOW_PERV as usize] = 1.0 - fracImperv;

    // --- initialize state of snow pack on each snow surface
    i = SNOW_PLOWABLE;
    while i <= SNOW_PERV {
        let iu = i as usize;
        if snowpack.fArea[iu] > 0.0 {
            snowpack.wsnow[iu] = sm.wsnow[iu];
            snowpack.fw[iu] = sm.fwnow[iu];
        } else {
            snowpack.wsnow[iu] = 0.0;
            snowpack.fw[iu] = 0.0;
        }
        snowpack.coldc[iu] = 0.0;
        snowpack.ati[iu] = sm.tbase[iu];
        snowpack.awe[iu] = 1.0;
        snowDepth += snowpack.wsnow[iu] * snowpack.fArea[iu];
        i += 1;
    }
    prj.Subcatch[ju].newSnowDepth = snowDepth;
}

//=============================================================================

/// Called by: project.
pub fn snow_initSnowmelt(prj: &mut Project, j: i32) {
    //
    //  Input:   j = snowmelt parameter set index
    //  Output:  none
    //  Purpose: initializes values in a snow melt parameter set.
    //
    let mut i: i32;
    let mut k: i32;
    let sm = &mut prj.Snowmelt[j as usize];
    i = 0;
    while i < 3 {
        let iu = i as usize;
        sm.snn = 0.0;
        sm.si[iu] = 0.0;
        sm.dhmin[iu] = 0.0;
        sm.dhmax[iu] = 0.0;
        sm.tbase[iu] = 0.0;
        sm.fwfrac[iu] = 0.0;
        sm.wsnow[iu] = 0.0;
        sm.fwnow[iu] = 0.0;
        sm.weplow = 1.0e6;
        k = 0;
        while k < 5 {
            sm.sfrac[k as usize] = 0.0;
            k += 1;
        }
        sm.toSubcatch = -1;
        i += 1;
    }
}

//=============================================================================

/// Called by: project.
pub fn snow_validateSnowmelt(prj: &mut Project, j: i32) {
    //
    //  Input:   j = snowmelt parameter set index
    //  Output:  none
    //  Purpose: checks for valid values in a snow melt parameter set.
    //
    let mut k: i32;
    let mut err: i8 = FALSE as i8;
    let mut sum = 0.0f64;
    let ju = j as usize;

    k = SNOW_PLOWABLE;
    while k <= SNOW_PERV {
        let ku = k as usize;
        // --- check melt coeffs.
        if prj.Snowmelt[ju].dhmin[ku] > prj.Snowmelt[ju].dhmax[ku] {
            err = TRUE as i8;
        }

        // --- check free water fraction
        if prj.Snowmelt[ju].fwfrac[ku] < 0.0 || prj.Snowmelt[ju].fwfrac[ku] > 1.0 {
            err = TRUE as i8;
        }
        k += 1;
    }

    // --- check fraction of imperv. area plowable
    if prj.Snowmelt[ju].snn < 0.0 || prj.Snowmelt[ju].snn > 1.0 {
        err = TRUE as i8;
    }

    // --- check that removal fractions sum <= 1.0
    k = 0;
    while k < 5 {
        sum += prj.Snowmelt[ju].sfrac[k as usize];
        k += 1;
    }
    if sum > 1.01 {
        err = TRUE as i8;
    }
    if err != 0 {
        let id = prj.Snowmelt[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_SNOWPACK_PARAMS, &id);
    }
}

//=============================================================================

/// Called by: hotstart.
pub fn snow_getState(prj: &Project, i: i32, j: i32, x: &mut [f64]) {
    //
    //  Input:   i = subcatchment index
    //           j = snow pack sub-area index
    //  Output:  updates array of snow pack state variables x
    //  Purpose: retrieves the current state of a snow pack object.
    //
    let snowpack = match prj.Subcatch[i as usize].snowpack.as_deref() {
        None => return,
        Some(s) => s,
    };
    let ju = j as usize;
    x[0] = snowpack.wsnow[ju];
    x[1] = snowpack.fw[ju];
    x[2] = snowpack.coldc[ju];
    x[3] = snowpack.ati[ju];
    x[4] = snowpack.awe[ju];
}

//=============================================================================

/// Called by: hotstart.
pub fn snow_setState(prj: &mut Project, i: i32, j: i32, x: &[f64]) {
    //
    //  Input:   i = subcatchment index
    //           j = snow pack sub-area index
    //           x = array of snow pack state variables
    //  Output:  none
    //  Purpose: sets the current state of a snow pack object.
    //
    let snowpack = match prj.Subcatch[i as usize].snowpack.as_deref_mut() {
        None => return,
        Some(s) => s,
    };
    let ju = j as usize;
    snowpack.wsnow[ju] = x[0];
    snowpack.fw[ju] = x[1];
    snowpack.coldc[ju] = x[2];
    snowpack.ati[ju] = x[3];
    snowpack.awe[ju] = x[4];
}

//=============================================================================

fn setMeltParams(prj: &mut Project, j: i32, k: i32, x: &mut [f64]) {
    //
    //  Input:   j = snowmelt parameter set index
    //           k = data category index
    //           x = array of snow parameter values
    //  Output:  none
    //  Purpose: assigns values to parameters in a snow melt data set.
    //
    let mut i: i32;
    let ju = j as usize;
    let ku = k as usize;
    let ucfTemperature = UCF(prj, TEMPERATURE);
    let ucfRainfall = UCF(prj, RAINFALL);
    let ucfRainDepth = UCF(prj, RAINDEPTH);
    let unitSystem = prj.UnitSystem;
    let sm = &mut prj.Snowmelt[ju];

    // --- snow pack melt parameters
    if k >= SNOW_PLOWABLE && k <= SNOW_PERV {
        // --- min/max melt coeffs.
        sm.dhmin[ku] = x[0] * ucfTemperature / ucfRainfall;
        sm.dhmax[ku] = x[1] * ucfTemperature / ucfRainfall;

        // --- base melt temp (deg F)
        sm.tbase[ku] = x[2];
        if unitSystem == SI {
            sm.tbase[ku] = (9.0 / 5.0) * sm.tbase[ku] + 32.0;
        }

        // --- free water fractions
        sm.fwfrac[ku] = x[3];

        // --- initial snow depth & free water depth
        sm.wsnow[ku] = x[4] / ucfRainDepth;
        x[5] = MIN(x[5], x[3] * x[4]);
        sm.fwnow[ku] = x[5] / ucfRainDepth;

        // --- fraction of impervious area that is plowable
        if k == SNOW_PLOWABLE {
            sm.snn = x[6];
        }
        // --- min. depth for 100% areal coverage on remaining
        //     impervious area or total pervious area
        else {
            sm.si[ku] = x[6] / ucfRainDepth;
        }
    }
    // --- removal parameters
    else if k == SNOW_REMOVAL {
        sm.weplow = x[0] / ucfRainDepth;
        i = 0;
        while i <= 4 {
            sm.sfrac[i as usize] = x[(i + 1) as usize];
            i += 1;
        }
        if x[6] >= 0.0 {
            sm.toSubcatch = f64_to_i32(x[6] + 0.01);
        } else {
            sm.toSubcatch = -1;
        }
    }
}

//=============================================================================

/// Called by: climate.
pub fn snow_setMeltCoeffs(prj: &mut Project, j: i32, s: f64) {
    //
    //  Input:   j = snowmelt parameter set index
    //           s = snow season of year
    //  Output:  none
    //  Purpose: sets values of snow melt coeffs. for particular time of year.
    //
    let mut k: i32; // snow sub-area index
    let sm = &mut prj.Snowmelt[j as usize];

    k = SNOW_PLOWABLE;
    while k <= SNOW_PERV {
        let ku = k as usize;
        sm.dhm[ku] = 0.5 * (sm.dhmax[ku] * (1.0 + s) + sm.dhmin[ku] * (1.0 - s));
        k += 1;
    }
}

//=============================================================================

/// Called by: runoff.
pub fn snow_plowSnow(prj: &mut Project, j: i32, tStep: f64) {
    //
    //  Input:   j     = subcatchment index
    //           tStep = time step (sec)
    //  Output:  none
    //  Purpose: adds new snow to subcatchment and plows it between sub-areas.
    //
    let mut i: i32; // snow sub-area index
    let k: i32; // snowmelt parameter set index
    let m: i32; // subcatchment index
    let mut rainfall: f64 = 0.0; // rainfall (not used)
    let mut snowfall: f64 = 0.0; // snowfall (ft/sec)
    let exc: f64; // excess snow depth (ft)
    let mut f: f64; // area ratio
    let mut sfracTotal: f64; // total fraction of snow moved

    // (the snowpack is reached through prj.Subcatch[j] at each use, since
    //  snow sent to another subcatchment m may land on this same snowpack)
    let ju = j as usize;
    if prj.Subcatch[ju].snowpack.is_none() {
        return;
    }

    // --- see if there's any snowfall
    let gage = prj.Subcatch[ju].gage;
    gage_getPrecip(prj, gage, &mut rainfall, &mut snowfall);

    // --- add snowfall to snow pack
    {
        let snowpack = prj.Subcatch[ju].snowpack.as_deref_mut().unwrap();
        i = SNOW_PLOWABLE;
        while i <= SNOW_PERV {
            let iu = i as usize;
            if snowpack.fArea[iu] > 0.0 {
                snowpack.wsnow[iu] += snowfall * tStep;
                snowpack.imelt[iu] = 0.0;
            }
            i += 1;
        }
    }

    // --- see if there is excess snow on plowable area to remove
    let PL = SNOW_PLOWABLE as usize;
    let IM = SNOW_IMPERV as usize;
    let PV = SNOW_PERV as usize;
    if prj.Subcatch[ju].snowpack.as_deref().unwrap().fArea[PL] > 0.0 {
        k = prj.Subcatch[ju].snowpack.as_deref().unwrap().snowmeltIndex;
        let ku = k as usize;
        if prj.Subcatch[ju].snowpack.as_deref().unwrap().wsnow[PL] >= prj.Snowmelt[ku].weplow {
            // --- excess snow to be reomoved
            exc = prj.Subcatch[ju].snowpack.as_deref().unwrap().wsnow[PL];

            // --- plow out of system
            f = prj.Subcatch[ju].snowpack.as_deref().unwrap().fArea[PL]
                * (prj.Subcatch[ju].area - prj.Subcatch[ju].lidArea);
            prj.Snow.removed += prj.Snowmelt[ku].sfrac[0] * exc * f;
            sfracTotal = prj.Snowmelt[ku].sfrac[0];

            // --- plow onto non-plowable impervious area
            if prj.Subcatch[ju].snowpack.as_deref().unwrap().fArea[IM] > 0.0 {
                let sfrac1 = prj.Snowmelt[ku].sfrac[1];
                let snowpack = prj.Subcatch[ju].snowpack.as_deref_mut().unwrap();
                f = snowpack.fArea[PL] / snowpack.fArea[IM];
                snowpack.wsnow[IM] += sfrac1 * exc * f;
                sfracTotal += sfrac1;
            }

            // --- plow onto pervious area
            if prj.Subcatch[ju].snowpack.as_deref().unwrap().fArea[PV] > 0.0 {
                let sfrac2 = prj.Snowmelt[ku].sfrac[2];
                let snowpack = prj.Subcatch[ju].snowpack.as_deref_mut().unwrap();
                f = snowpack.fArea[PL] / snowpack.fArea[PV];
                snowpack.wsnow[PV] += sfrac2 * exc * f;
                sfracTotal += sfrac2;
            }

            // --- convert to immediate melt
            {
                let sfrac3 = prj.Snowmelt[ku].sfrac[3];
                let snowpack = prj.Subcatch[ju].snowpack.as_deref_mut().unwrap();
                snowpack.imelt[PL] = sfrac3 * exc / tStep;
                sfracTotal += sfrac3;
            }

            // --- send to another subcatchment
            if prj.Snowmelt[ku].sfrac[4] > 0.0 {
                m = prj.Snowmelt[ku].toSubcatch;
                // NOTE: m is -1 when no receiving subcatchment was named;
                // C then reads Subcatch[-1].snowpack, outside the array
                // (undefined). That read is treated here as a NULL snowpack.
                let mSnowpack = if m >= 0 { prj.Subcatch[m as usize].snowpack.as_deref() } else { None };
                if let Some(msp) = mSnowpack {
                    f = msp.fArea[PV];
                } else {
                    f = 0.0;
                }
                if f > 0.0 {
                    f = prj.Subcatch[ju].snowpack.as_deref().unwrap().fArea[PL] / f;
                    let sfrac4 = prj.Snowmelt[ku].sfrac[4];
                    prj.Subcatch[m as usize].snowpack.as_deref_mut().unwrap().wsnow[PV] += sfrac4 * exc * f;
                    sfracTotal += sfrac4;
                }
            }

            // --- reduce snow depth by amount plowed
            sfracTotal = MIN(sfracTotal, 1.0);
            prj.Subcatch[ju].snowpack.as_deref_mut().unwrap().wsnow[PL] = exc * (1.0 - sfracTotal);
        }
    }
}

//=============================================================================

/// Called by: subcatch.
pub fn snow_getSnowMelt(prj: &mut Project, j: i32, rainfall: f64, snowfall: f64, tStep: f64, netPrecip: &mut [f64]) -> f64 {
    //
    //  Input:   j = subcatchment index
    //           rainfall = rainfall (ft/sec)
    //           snowfall = snowfall (ft/sec)
    //           tStep = time step (sec)
    //  Output:  netPrecip = rainfall + snowmelt on each runoff sub-area (ft/sec),
    //           returns new snow depth over subcatchment
    //  Purpose: modifies rainfall input to subcatchment's sub-areas based on
    //           possible snow melt and updates snow depth over entire subcatchment.
    //
    let mut i: i32; // snow sub-area index
    let rmelt: f64; // melt rate when rain falling (ft/sec)
    let mut smelt: f64; // snow melt from sub-area (ft/sec)
    let mut asc: f64; // frac. of sub-area snow covered
    let mut snowDepth = 0.0f64; // snow depth on entire subcatchment (ft)
    let impervPrecip: f64; // net precip. on imperv. area (ft/sec)

    // --- get ptr. to subcatchment's snowpack
    //     (a copy, written back below; nothing called here reaches it
    //      through prj)
    let ju = j as usize;
    let mut snowpack: TSnowpack = **prj.Subcatch[ju].snowpack.as_ref().unwrap();

    // --- compute snowmelt over entire subcatchment when rain falling
    rmelt = getRainmelt(prj, rainfall);

    // --- compute snow melt from each type of subarea
    i = SNOW_PLOWABLE;
    while i <= SNOW_PERV {
        let iu = i as usize;
        // --- completely melt pack if its depth is < 0.001 inch
        if snowpack.wsnow[iu] <= 0.001 / 12.0 {
            asc = 0.0;
            smelt = 0.0;
            // NOTE: for a snow surface with no area, imelt[i] is never set
            // by snow_plowSnow and C reads the uninitialized malloc'd value
            // here; it is zero here. That surface's netPrecip is only ever
            // multiplied by its zero area or used by a sub-area of zero
            // area, so it matters only if the garbage were Inf or NaN.
            snowpack.imelt[iu] += (snowpack.wsnow[iu] + snowpack.fw[iu]) / tStep;
            snowpack.wsnow[iu] = 0.0;
            snowpack.fw[iu] = 0.0;
            snowpack.coldc[iu] = 0.0;
        }
        // --- otherwise compute areal depletion, find snow melt
        //     and route it through pack
        else {
            asc = getArealDepletion(prj, &mut snowpack, i, snowfall, tStep);
            smelt = meltSnowpack(prj, &mut snowpack, i, rmelt, asc, snowfall, tStep);
            smelt = routeSnowmelt(prj, &mut snowpack, i, smelt, asc, rainfall, tStep);
        }

        // --- find net precip. over entire subcatch area
        netPrecip[iu] = smelt + snowpack.imelt[iu] // snow pack melt
            + rainfall * (1.0 - asc); // rainfall on non-snow area

        // --- add to total snow depth on subcatchment
        snowDepth += snowpack.wsnow[iu] * snowpack.fArea[iu];
        i += 1;
    }

    // --- combine netPrecip on plowable & non-plowable imperv. areas
    if prj.Subcatch[ju].fracImperv > 0.0 {
        impervPrecip = (netPrecip[SNOW_PLOWABLE as usize] * snowpack.fArea[SNOW_PLOWABLE as usize]
            + netPrecip[SNOW_IMPERV as usize] * snowpack.fArea[SNOW_IMPERV as usize])
            / prj.Subcatch[ju].fracImperv;
        netPrecip[IMPERV0 as usize] = impervPrecip;
        netPrecip[IMPERV1 as usize] = impervPrecip;
    }
    **prj.Subcatch[ju].snowpack.as_mut().unwrap() = snowpack;
    return snowDepth;
}

//=============================================================================

/// Called by: massbal.
pub fn snow_getSnowCover(prj: &Project, j: i32) -> f64 {
    //
    //  Input:   j = subcatchment index
    //  Output:  returns volume of snow cover (ft3)
    //  Purpose: computes volume of snow on a subcatchment.
    //
    let mut i: i32;
    let mut snowCover = 0.0f64; // snow cover volume (ft3)

    let ju = j as usize;
    let snowpack = match prj.Subcatch[ju].snowpack.as_deref() {
        None => return 0.0,
        Some(s) => s,
    };
    i = SNOW_PLOWABLE;
    while i <= SNOW_PERV {
        let iu = i as usize;
        snowCover += (snowpack.wsnow[iu] + snowpack.fw[iu]) * snowpack.fArea[iu];
        i += 1;
    }
    return snowCover * (prj.Subcatch[ju].area - prj.Subcatch[ju].lidArea);
}

//=============================================================================

fn getArealDepletion(prj: &Project, snowpack: &mut TSnowpack, i: i32, snowfall: f64, tStep: f64) -> f64 {
    //
    //  Input:   snowpack = ptr. to snow pack object
    //           i = snow sub-area index
    //           snowfall = snow fall rate (ft/sec)
    //           tStep = time step (sec)
    //  Output:  returns fraction of sub-area with snow cover
    //  Purpose: depletes snow covered area as snow pack melts.
    //
    let k: i32; // index of snow melt parameter set
    let asc: f64; // fraction of area with 100% cover
    let si: f64; // snow depth at 100% cover
    let awesi: f64; // depth relative to depth at 100% cover
    let mut awe: f64;
    let sba: f64;
    let mut sbws: f64;
    let iu = i as usize;

    // --- plowable sub-area not subject to areal depletion
    if i == SNOW_PLOWABLE {
        return 1.0;
    }
    k = snowpack.snowmeltIndex;
    si = prj.Snowmelt[k as usize].si[iu];

    // --- no depletion if depth zero or above SI
    if si == 0.0 || snowpack.wsnow[iu] >= si {
        snowpack.awe[iu] = 1.0;
        return 1.0;
    }
    if snowpack.wsnow[iu] == 0.0 {
        snowpack.awe[iu] = 1.0;
        return 0.0;
    }

    // --- case of new snowfall
    if snowfall > 0.0 {
        awe = (snowpack.wsnow[iu] - snowfall * tStep) / si;
        awe = MAX(awe, 0.0);
        let sba = getArealSnowCover(prj, i, awe);
        sbws = awe + (0.75 * snowfall * tStep) / si;
        sbws = MIN(sbws, 1.0);
        snowpack.awe[iu] = awe;
        snowpack.sba[iu] = sba;
        snowpack.sbws[iu] = sbws;
        return 1.0;
    }
    // --- case of no new snow
    else {
        // NOTE: sba and sbws are set only by the new-snowfall branch above;
        // C leaves them uninitialized before that (malloc), zeros here.
        // They are used below only once awe[i] < 1, which that branch (or a
        // hot start file) sets.
        awe = snowpack.awe[iu];
        sba = snowpack.sba[iu];
        sbws = snowpack.sbws[iu];
        awesi = snowpack.wsnow[iu] / si;

        // --- relative snow depth is below start of new snow ADC
        if awesi < snowpack.awe[iu] {
            snowpack.awe[iu] = 1.0;
            asc = getArealSnowCover(prj, i, awesi);
        }
        // --- relative snow depth is above end of new snow ADC
        else if awesi >= snowpack.sbws[iu] {
            asc = 1.0;
        }
        // --- relative snow depth is on new snow ADC
        else {
            asc = sba + (1.0 - sba) / (sbws - awe) * (awesi - awe);
        }
        return asc;
    }
}

//=============================================================================

fn getArealSnowCover(prj: &Project, i: i32, awesi: f64) -> f64 {
    //
    //  Input:   i = snow sub-area index
    //           awesi = snow depth relative to depth at 100% snow cover
    //  Output:  returns fraction of sub-area with snow cover
    //  Purpose: finds x-value on areal depletion curve (ADC) for given y-value.
    //
    //  Note:    Areal depletion curves are associated with a project's Snow
    //           data structure. They plot relative snow depth (awesi)
    //           as a function of snow covererd area fraction (asc) in 10 equal
    //           awesi increments between 0 and 1.0.
    //
    let k: i32; // type of ADC (impervious or pervious)
    let m: i32; // interval on ADC
    let asc1: f64; // asc values at ends of interval
    let asc2: f64;

    // --- determine which ADC to use
    if i == SNOW_IMPERV {
        k = 0;
    } else if i == SNOW_PERV {
        k = 1;
    } else {
        return 1.0;
    }

    // --- locate interval on ADC that bounds awesi
    if awesi <= 0.0 {
        return 0.0;
    }
    if awesi >= 0.9999 {
        return 1.0;
    }
    m = f64_to_i32(awesi * 10.0 + 0.00001);

    // --- get asc values for either end of interval
    asc1 = prj.Snow.adc[k as usize][m as usize];
    if m >= 9 {
        asc2 = 1.0;
    } else {
        asc2 = prj.Snow.adc[k as usize][(m + 1) as usize];
    }

    // --- return with interpolated asc value
    return asc1 + (asc2 - asc1) / 0.1 * (awesi - 0.1 * (m as f32) as f64);
}

//=============================================================================

fn meltSnowpack(prj: &Project, snowpack: &mut TSnowpack, i: i32, rmelt: f64, asc: f64, snowfall: f64, tStep: f64) -> f64 {
    //
    //  Input:   snowpack = ptr. to snow pack object
    //           i        = snow sub-area index
    //           rmelt    = melt rate if raining (ft/sec)
    //           asc      = fraction of area covered with snow
    //           snowfall = rate of snow fall (ft/sec)
    //           tStep    = time step (sec)
    //  Output:  returns snow melt rate (ft/sec)
    //  Purpose: computes rate of snow melt from snow sub-area.
    //
    let k: i32; // snowmelt parameter set index
    let mut smelt: f64; // melt rate over sub-area (ft/sec)
    let ccFactor: f64; // cold content conversion factor
    let iu = i as usize;

    // --- if raining then use result found from getRainMelt
    k = snowpack.snowmeltIndex;
    let ku = k as usize;
    if rmelt > 0.0 {
        smelt = rmelt;
    }
    // --- else if air temp. >= base melt temp. then use degree-day eqn.
    else if prj.Temp.ta >= prj.Snowmelt[ku].tbase[iu] {
        smelt = prj.Snowmelt[ku].dhm[iu] * (prj.Temp.ta - prj.Snowmelt[ku].tbase[iu]);
    }
    // --- otherwise alter cold content and return 0
    else {
        updateColdContent(prj, snowpack, i, asc, snowfall, tStep);
        return 0.0;
    }

    // --- adjust snowmelt for area of snow cover
    smelt *= asc;

    // --- reduce cold content of melting pack
    ccFactor = tStep * prj.Snow.rnm * asc;
    smelt = reduceColdContent(snowpack, i, smelt, ccFactor);
    snowpack.ati[iu] = prj.Snowmelt[ku].tbase[iu];
    return smelt;
}

//=============================================================================

fn getRainmelt(prj: &Project, rainfall: f64) -> f64 {
    //
    //  Input:   rainfall = rainfall rate (ft/sec)
    //  Output:  returns snow melt rate (ft/sec)
    //  Purpose: computes rate of snow melt when rainfall occurs.
    //
    let mut rainfall = rainfall;
    let uadj: f64; // adjusted wind speed
    let t1: f64;
    let t2: f64;
    let t3: f64;
    let smelt: f64; // snow melt in in/hr

    rainfall = rainfall * 43200.0; // convert rain to in/hr
    if rainfall > 0.02 {
        uadj = 0.006 * prj.Wind.ws;
        t1 = prj.Temp.ta - 32.0;
        t2 = 7.5 * prj.Temp.gamma * uadj;
        t3 = 8.5 * uadj * (prj.Temp.ea - 0.18);
        smelt = t1 * (0.001167 + t2 + 0.007 * rainfall) + t3;
        return smelt / 43200.0;
    } else {
        return 0.0;
    }
}

//=============================================================================

fn updateColdContent(prj: &Project, snowpack: &mut TSnowpack, i: i32, asc: f64, snowfall: f64, tStep: f64) {
    //
    //  Input:   snowpack = ptr. to snow pack object
    //           i        = snow sub-area index
    //           asc      = fraction of area snow covered
    //           snowfall = snow fall rate (ft/sec)
    //           tStep    = time step (sec)
    //  Output:  none
    //  Purpose: updates cold content of snow pack under non-melting conditions.
    //
    let k: i32; // snowmelt parameter set index
    let mut ati: f64; // antecdent temperature index (deg F)
    let mut cc: f64; // snow pack cold content (ft)
    let ccMax: f64; // max. possible cold content (ft)
    let tipm: f64; // adjusted ATI weighting factor
    let iu = i as usize;

    // --- retrieve ATI & CC from snow pack object
    ati = snowpack.ati[iu];
    cc = snowpack.coldc[iu];

    // --- if snowing, ATI = snow (air) temperature
    if snowfall * 43200.0 > 0.02 {
        ati = prj.Temp.ta;
    } else {
        // convert ATI weighting factor from 6-hr to tStep time basis
        tipm = 1.0 - (1.0 - prj.Snow.tipm).powf(tStep / (6.0 * 3600.0));

        // update ATI
        ati += tipm * (prj.Temp.ta - ati);
    }

    // --- ATI cannot exceed snow melt base temperature
    k = snowpack.snowmeltIndex;
    let ku = k as usize;
    ati = MIN(ati, prj.Snowmelt[ku].tbase[iu]);

    // --- update cold content
    cc += prj.Snow.rnm * prj.Snowmelt[ku].dhm[iu] * (ati - prj.Temp.ta) * tStep * asc;
    cc = MAX(cc, 0.0);

    // --- maximum cold content based on assumed specific heat of snow
    //     of 0.007 in. water equiv. per deg. F
    ccMax = snowpack.wsnow[iu] * 0.007 / 12.0 * (prj.Snowmelt[ku].tbase[iu] - ati);
    cc = MIN(cc, ccMax);

    // --- assign updated values to snowpack
    snowpack.coldc[iu] = cc;
    snowpack.ati[iu] = ati;
}

//=============================================================================

fn reduceColdContent(snowpack: &mut TSnowpack, i: i32, smelt: f64, ccFactor: f64) -> f64 {
    //
    //  Input:   snowpack = ptr. to snowpack object
    //           i        = snow sub-area index
    //           smelt    = potential melt rate (ft/sec)
    //           ccFactor = cold content conversion factor
    //  Output:  returns snow melt rate (ft/sec)
    //  Purpose: reduces cold content of snow pack adjusting melt rate accordingly.
    //
    let mut smelt = smelt;
    let mut cc: f64; // cold content of snow pack (ft)
    let iu = i as usize;

    cc = snowpack.coldc[iu];
    if smelt * ccFactor > cc {
        smelt -= cc / ccFactor;
        cc = 0.0;
    } else {
        cc -= smelt * ccFactor;
        smelt = 0.0;
    }
    snowpack.coldc[iu] = cc;
    return smelt;
}

//=============================================================================

fn routeSnowmelt(prj: &Project, snowpack: &mut TSnowpack, i: i32, smelt: f64, asc: f64, rainfall: f64, tStep: f64) -> f64 {
    //
    //  Input:   snowpack = ptr. to snowpack object
    //           i        = snow sub-area index
    //           smelt    = snow melt rate (ft/sec)
    //           asc      = fraction of area snow covered
    //           rainfall = rainfall rate (ft/sec)
    //           tStep    = time step (sec)
    //  Output:  returns rate of liquid snow melt leaving a snow pack (ft/sec)
    //  Purpose: routes snow melt through free water holding capacity of snow pack.
    //
    let k: i32; // snowmelt parameter set index
    let mut vmelt: f64; // snow melt volume (ft)
    let iu = i as usize;

    // --- get volume of snowmelt over time step
    k = snowpack.snowmeltIndex;
    vmelt = smelt * tStep;
    vmelt = MIN(vmelt, snowpack.wsnow[iu]);

    // --- reduce snow depth by volume of snowmelt
    snowpack.wsnow[iu] -= vmelt;

    // --- add snowmelt volume and any rainfall on snow
    //     covered area of sub-area to snow pack's free water content
    snowpack.fw[iu] += vmelt + rainfall * tStep * asc;

    // --- excess free water becomes liquid melt that leaves the pack
    vmelt = snowpack.fw[iu] - prj.Snowmelt[k as usize].fwfrac[iu] * snowpack.wsnow[iu];
    vmelt = MAX(vmelt, 0.0);

    // --- reduce free water by liquid melt volume and return liquid melt rate
    snowpack.fw[iu] -= vmelt;
    return vmelt / tStep;
}
