//! landuse.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   landuse.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//
//   Pollutant buildup and washoff functions.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - landuse_getWashoffMass() re-named to landuse_getWashoffQual() and
//     modified to return concentration instead of mass load.
//   - landuse_getRunoffLoad() re-named to landuse_getWashoffLoad() and
//     modified to work with landuse_getWashoffQual().
//-----------------------------------------------------------------------------

use crate::prelude::*;

/// The file-scope variables of landuse.c.
#[derive(Default)]
pub struct LanduseState {}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  landuse_readParams        (called by parseLine in input.c)
//  landuse_readPollutParams  (called by parseLine in input.c)
//  landuse_readBuildupParams (called by parseLine in input.c)
//  landuse_readWashoffParams (called by parseLine in input.c)

//  landuse_getInitBuildup    (called by subcatch_initState)
//  landuse_getBuildup        (called by surfqual_getBuildup)
//  landuse_getWashoffLoad    (called by surfqual_getWashoff)
//  landuse_getCoPollutLoad   (called by surfqual_getwashoff));
//  landuse_getAvgBMPEffic    (called by updatePondedQual in surfqual.c)

//=============================================================================

/// Called by: input.
pub fn landuse_readParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = land use index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads landuse parameters from a tokenized line of input.
//
//  Data format is:
//    landuseID  (sweepInterval sweepRemoval sweepDays0)
//
{
    let id: Option<Vec<u8>>;
    let ju = j as usize;
    if ntoks < 1 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, LANDUSE, &tok[0]);
    let id = match id {
        None => return error_setInpError(prj, ERR_NAME, &tok[0]),
        Some(id) => id,
    };
    prj.Landuse[ju].ID = id;
    if ntoks > 1 {
        if ntoks < 4 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if getDouble(&tok[1], &mut prj.Landuse[ju].sweepInterval) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[1]);
        }
        if getDouble(&tok[2], &mut prj.Landuse[ju].sweepRemoval) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[2]);
        }
        if getDouble(&tok[3], &mut prj.Landuse[ju].sweepDays0) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[3]);
        }
    } else {
        prj.Landuse[ju].sweepInterval = 0.0;
        prj.Landuse[ju].sweepRemoval = 0.0;
        prj.Landuse[ju].sweepDays0 = 0.0;
    }
    if prj.Landuse[ju].sweepRemoval < 0.0 || prj.Landuse[ju].sweepRemoval > 1.0 {
        // (reached only with ntoks >= 4: otherwise sweepRemoval is 0)
        return error_setInpError(prj, ERR_NUMBER, &tok[2]);
    }
    0
}

//=============================================================================

/// Called by: input.
pub fn landuse_readPollutParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = pollutant index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads pollutant parameters from a tokenized line of input.
//
//  Data format is:
//    ID Units cRain cGW cRDII kDecay (snowOnly coPollut coFrac cDWF cInit)
//
{
    let k: i32;
    let mut coPollut: i32;
    let mut snowFlag: i32;
    let mut x: [f64; 4] = [0.0; 4];
    let mut coFrac: f64;
    let mut cDWF: f64;
    let mut cInit: f64;
    let id: Vec<u8>;

    // --- extract pollutant name & units
    if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = match project_findID(prj, POLLUT, &tok[0]) {
        None => return error_setInpError(prj, ERR_NAME, &tok[0]),
        Some(id) => id,
    };
    k = findmatch(&tok[1], QualUnitsWords);
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }

    // --- extract concen. in rain, gwater, & I&I
    for i in 2..=4usize {
        if getDouble(&tok[i], &mut x[i - 2]) == 0 || x[i - 2] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i]);
        }
    }

    // --- extract decay coeff. (which can be negative for growth)
    if getDouble(&tok[5], &mut x[3]) == 0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[5]);
    }

    // --- set defaults for snow only flag & co-pollut. parameters
    snowFlag = 0;
    coPollut = -1;
    coFrac = 0.0;
    cDWF = 0.0;
    cInit = 0.0;

    // --- check for snow only flag
    if ntoks >= 7 {
        snowFlag = findmatch(&tok[6], NoYesWords);
        if snowFlag < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[6]);
        }
    }

    // --- check for co-pollutant
    if ntoks >= 9 {
        if strcomp(&tok[7], b"*") == 0 {
            coPollut = project_findObject(prj, POLLUT, &tok[7]);
            if coPollut < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[7]);
            }
            if getDouble(&tok[8], &mut coFrac) == 0 || coFrac < 0.0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[8]);
            }
        }
    }

    // --- check for DWF concen.
    if ntoks >= 10 {
        if getDouble(&tok[9], &mut cDWF) == 0 || cDWF < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[9]);
        }
    }

    // --- check for initial concen.
    if ntoks >= 11 {
        if getDouble(&tok[10], &mut cInit) == 0 || cInit < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[9]);
        }
    }

    // --- save values for pollutant object
    let mcf_mass = UCF(prj, MASS);
    let p = &mut prj.Pollut[j as usize];
    p.ID = id;
    p.units = k;
    if p.units == MG {
        p.mcf = mcf_mass;
    } else if p.units == UG {
        p.mcf = mcf_mass / 1000.0;
    } else {
        p.mcf = 1.0;
    }
    p.pptConcen = x[0];
    p.gwConcen = x[1];
    p.rdiiConcen = x[2];
    p.kDecay = x[3] / SECperDAY as f64;
    p.snowOnly = snowFlag;
    p.coPollut = coPollut;
    p.coFraction = coFrac;
    p.dwfConcen = cDWF;
    p.initConcen = cInit;
    0
}

//=============================================================================

/// Called by: input.
pub fn landuse_readBuildupParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads pollutant buildup parameters from a tokenized line of input.
//
//  Data format is:
//    landuseID  pollutID  buildupType  c1  c2  c3  normalizerType
//
{
    let j: i32;
    let k: i32;
    let mut n: i32;
    let p: i32;
    let mut c: [f64; 3] = [0.0, 0.0, 0.0];
    let tmax: f64;

    if ntoks < 3 {
        return 0;
    }
    j = project_findObject(prj, LANDUSE, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    p = project_findObject(prj, POLLUT, &tok[1]);
    if p < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    k = findmatch(&tok[2], BuildupTypeWords);
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[2]);
    }
    let (ju, pu) = (j as usize, p as usize);
    prj.Landuse[ju].buildupFunc[pu].funcType = k;
    if k > NO_BUILDUP {
        if ntoks < 7 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if k != EXTERNAL_BUILDUP {
            for i in 0..3usize {
                if getDouble(&tok[i + 3], &mut c[i]) == 0 || c[i] < 0.0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[i + 3]);
                }
            }
        }
        n = findmatch(&tok[6], NormalizerWords);
        if n < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[6]);
        }
        prj.Landuse[ju].buildupFunc[pu].normalizer = n;
    }

    // Find time until max. buildup (or time series for external buildup)
    match prj.Landuse[ju].buildupFunc[pu].funcType {
        POWER_BUILDUP => {
            // --- check for too small or large an exponent
            if c[2] > 0.0 && (c[2] < 0.01 || c[2] > 10.0) {
                return error_setInpError(prj, ERR_KEYWORD, &tok[5]);
            }

            // --- find time to reach max. buildup
            // --- use zero if coeffs. are 0
            if c[1] * c[2] == 0.0 {
                tmax = 0.0;
            }
            // --- use 10 years if inverse power function tends to blow up
            else if c[0].log10() / c[2] > 3.5 {
                tmax = 3650.0;
            }
            // --- otherwise use inverse power function
            else {
                tmax = (c[0] / c[1]).powf(1.0 / c[2]);
            }
        }

        EXPON_BUILDUP => {
            if c[1] == 0.0 {
                tmax = 0.0;
            } else {
                tmax = -(0.001f64).ln() / c[1];
            }
        }

        SATUR_BUILDUP => {
            tmax = 1000.0 * c[2];
        }

        EXTERNAL_BUILDUP => {
            if getDouble(&tok[3], &mut c[0]) == 0 || c[0] < 0.0 {
                //max. buildup
                return error_setInpError(prj, ERR_NUMBER, &tok[3]);
            }
            if getDouble(&tok[4], &mut c[1]) == 0 || c[1] < 0.0 {
                //scaling factor
                return error_setInpError(prj, ERR_NUMBER, &tok[3]);
            }
            n = project_findObject(prj, TSERIES, &tok[5]); //time series
            if n < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[4]);
            }
            prj.Tseries[n as usize].refersTo = EXTERNAL_BUILDUP;
            c[2] = n as f64;
            tmax = 0.0;
        }

        _ => {
            tmax = 0.0;
        }
    }

    // Assign parameters to buildup object
    let b = &mut prj.Landuse[ju].buildupFunc[pu];
    b.coeff[0] = c[0];
    b.coeff[1] = c[1];
    b.coeff[2] = c[2];
    b.maxDays = tmax;
    0
}

//=============================================================================

/// Called by: input.
pub fn landuse_readWashoffParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads pollutant washoff parameters from a tokenized line of input.
//
//  Data format is:
//    landuseID  pollutID  washoffType  c1  c2  sweepEffic  bmpRemoval
{
    let j: i32;
    let p: i32;
    let mut func: i32;
    let mut x: [f64; 4] = [0.0; 4];

    if ntoks < 3 {
        return 0;
    }
    for i in 0..4usize {
        x[i] = 0.0;
    }
    func = NO_WASHOFF;
    j = project_findObject(prj, LANDUSE, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    p = project_findObject(prj, POLLUT, &tok[1]);
    if p < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }
    if ntoks > 2 {
        func = findmatch(&tok[2], WashoffTypeWords);
        if func < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[2]);
        }
        if func != NO_WASHOFF {
            if ntoks < 5 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            if getDouble(&tok[3], &mut x[0]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[3]);
            }
            if getDouble(&tok[4], &mut x[1]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[4]);
            }
            if ntoks >= 6 {
                if getDouble(&tok[5], &mut x[2]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[5]);
                }
            }
            if ntoks >= 7 {
                if getDouble(&tok[6], &mut x[3]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[6]);
                }
            }
        }
    }

    // --- check for valid parameter values
    //     x[0] = washoff coeff.
    //     x[1] = washoff expon.
    //     x[2] = sweep effic.
    //     x[3] = BMP effic.
    //     (each x[i] is nonzero only if its token was read, so the token
    //      indexed below always exists)
    if x[0] < 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[3]);
    }
    if x[1] < -10.0 || x[1] > 10.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[4]);
    }
    if x[2] < 0.0 || x[2] > 100.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[5]);
    }
    if x[3] < 0.0 || x[3] > 100.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[6]);
    }

    // --- convert units of washoff coeff.
    if func == EXPON_WASHOFF {
        x[0] /= 3600.0;
    }
    if func == RATING_WASHOFF {
        x[0] *= UCF(prj, FLOW).powf(x[1]);
    }
    if func == EMC_WASHOFF {
        x[0] *= LperFT3;
    }

    // --- assign washoff parameters to washoff object
    let w = &mut prj.Landuse[j as usize].washoffFunc[p as usize];
    w.funcType = func;
    w.coeff = x[0];
    w.expon = x[1];
    w.sweepEffic = x[2] / 100.0;
    w.bmpEffic = x[3] / 100.0;
    0
}

//=============================================================================

/// Called by: surfqual.
pub fn landuse_getInitBuildup(prj: &mut Project, landFactor: &mut [TLandFactor], initBuildup: &[f64], area: f64, curb: f64)
//
//  Input:   landFactor = array of land use factors
//           initBuildup = total initial buildup of each pollutant
//           area = subcatchment's area (ft2)
//           curb = subcatchment's curb length (users units)
//  Output:  modifies each land use factor's initial pollutant buildup
//  Purpose: determines the initial buildup of each pollutant on
//           each land use for a given subcatchment.
//
//  Notes:   Contributions from co-pollutants to initial buildup are not
//           included since the co-pollutant mechanism only applies to
//           washoff.
//
{
    let startDrySeconds: f64; // antecedent dry period (sec)
    let mut f: f64; // faction of total land area
    let mut fArea: f64; // area of land use (ft2)
    let mut fCurb: f64; // curb length of land use
    let mut buildup: f64; // pollutant mass buildup

    // --- convert antecedent dry days into seconds
    startDrySeconds = prj.StartDryDays * SECperDAY as f64;

    // --- examine each land use
    for i in 0..prj.Nobjects[LANDUSE as usize] {
        let iu = i as usize;
        // --- initialize date when last swept
        landFactor[iu].lastSwept = prj.StartDateTime - prj.Landuse[iu].sweepDays0;

        // --- determine area and curb length covered by land use
        f = landFactor[iu].fraction;
        fArea = f * area * UCF(prj, LANDAREA);
        fCurb = f * curb;

        // --- determine buildup of each pollutant
        for p in 0..prj.Nobjects[POLLUT as usize] {
            // --- if an initial loading was supplied, then use it to
            //     find the starting buildup over the land use
            buildup = 0.0;
            if initBuildup[p as usize] > 0.0 {
                buildup = initBuildup[p as usize] * fArea;
            }
            // --- otherwise use the land use's buildup function to
            //     compute a buildup over the antecedent dry period
            else {
                buildup = landuse_getBuildup(prj, i, p, fArea, fCurb, buildup, startDrySeconds);
            }
            landFactor[iu].buildup[p as usize] = buildup;
        }
    }
}

//=============================================================================

/// Called by: surfqual.
pub fn landuse_getBuildup(prj: &mut Project, i: i32, p: i32, area: f64, curb: f64, buildup: f64, tStep: f64) -> f64
//
//  Input:   i = land use index
//           p = pollutant index
//           area = land use area (ac or ha)
//           curb = land use curb length (users units)
//           buildup = current pollutant buildup (lbs or kg)
//           tStep = time increment for buildup (sec)
//  Output:  returns new buildup mass (lbs or kg)
//  Purpose: computes new pollutant buildup on a landuse after a time increment.
//
{
    let n: i32; // normalizer code
    let mut days: f64; // accumulated days of buildup
    let mut perUnit: f64; // normalizer value (area or curb length)
    let bf = prj.Landuse[i as usize].buildupFunc[p as usize];

    // --- return current buildup if no buildup function or time increment
    if bf.funcType == NO_BUILDUP || tStep == 0.0 {
        return buildup;
    }

    // --- see what buildup is normalized to
    n = bf.normalizer;
    perUnit = 1.0;
    if n == PER_AREA {
        perUnit = area;
    }
    if n == PER_CURB {
        perUnit = curb;
    }
    if perUnit == 0.0 {
        return 0.0;
    }

    // --- buildup determined by loading time series
    if bf.funcType == EXTERNAL_BUILDUP {
        return landuse_getExternalBuildup(prj, i, p, buildup / perUnit, tStep) * perUnit;
    }

    // --- determine equivalent days of current buildup
    days = landuse_getBuildupDays(prj, i, p, buildup / perUnit);

    // --- compute buildup after adding on time increment
    days += tStep / SECperDAY as f64;
    landuse_getBuildupMass(prj, i, p, days) * perUnit
}

//=============================================================================

fn landuse_getBuildupDays(prj: &Project, i: i32, p: i32, buildup: f64) -> f64
//
//  Input:   i = land use index
//           p = pollutant index
//           buildup = amount of pollutant buildup
//  Output:  returns number of days it takes for buildup to reach a given level
//  Purpose: finds the number of days corresponding to a pollutant buildup.
//
{
    let bf = &prj.Landuse[i as usize].buildupFunc[p as usize];
    let c0 = bf.coeff[0];
    let c1 = bf.coeff[1];
    let c2 = bf.coeff[2];

    if buildup == 0.0 {
        return 0.0;
    }
    if buildup >= c0 {
        return bf.maxDays;
    }
    match bf.funcType {
        POWER_BUILDUP => {
            if c1 * c2 == 0.0 {
                0.0
            } else {
                (buildup / c1).powf(1.0 / c2)
            }
        }

        EXPON_BUILDUP => {
            if c0 * c1 == 0.0 {
                0.0
            } else {
                -(1. - buildup / c0).ln() / c1
            }
        }

        SATUR_BUILDUP => {
            if c0 == 0.0 {
                0.0
            } else {
                buildup * c2 / (c0 - buildup)
            }
        }

        _ => 0.0,
    }
}

//=============================================================================

fn landuse_getBuildupMass(prj: &Project, i: i32, p: i32, days: f64) -> f64
//
//  Input:   i = land use index
//           p = pollutant index
//           days = time over which buildup has occurred (days)
//  Output:  returns mass of pollutant buildup (lbs or kg per area or curblength)
//  Purpose: finds amount of buildup of pollutant on a land use.
//
{
    let mut b: f64;
    let bf = &prj.Landuse[i as usize].buildupFunc[p as usize];
    let c0 = bf.coeff[0];
    let c1 = bf.coeff[1];
    let c2 = bf.coeff[2];

    if days == 0.0 {
        return 0.0;
    }
    if days >= bf.maxDays {
        return c0;
    }
    match bf.funcType {
        POWER_BUILDUP => {
            b = c1 * days.powf(c2);
            if b > c0 {
                b = c0;
            }
        }

        EXPON_BUILDUP => {
            b = c0 * (1.0 - (-days * c1).exp());
        }

        SATUR_BUILDUP => {
            b = days * c0 / (c2 + days);
        }

        _ => {
            b = 0.0;
        }
    }
    b
}

//=============================================================================

/// Called by: surfqual.
pub fn landuse_getAvgBmpEffic(prj: &mut Project, j: i32, p: i32) -> f64
//
//  Input:   j = subcatchment index
//           p = pollutant index
//  Output:  returns a BMP removal fraction for pollutant p
//  Purpose: finds the overall average BMP removal achieved for pollutant p
//           treated in subcatchment j.
//
{
    let mut r: f64 = 0.0;
    for i in 0..prj.Nobjects[LANDUSE as usize] as usize {
        r += prj.Subcatch[j as usize].landFactor[i].fraction * prj.Landuse[i].washoffFunc[p as usize].bmpEffic;
    }
    r
}

//=============================================================================

/// Called by: surfqual.
pub fn landuse_getWashoffLoad(prj: &mut Project, i: i32, p: i32, area: f64, landFactor: &mut [TLandFactor], runoff: f64, vOutflow: f64) -> f64
//
//  Input:   i = land use index
//           p = pollut. index
//           area = sucatchment area (ft2)
//           landFactor[] = array of land use data for subcatchment
//           runoff = runoff flow generated by subcatchment (ft/sec)
//           vOutflow = runoff volume leaving the subcatchment (ft3)
//  Output:  returns pollutant runoff load (mass)
//  Purpose: computes pollutant load generated by a land use over a time step.
//
{
    let landuseArea: f64; // area of current land use (ft2)
    let mut buildup: f64; // current pollutant buildup (lb or kg)
    let washoffQual: f64; // pollutant concentration in washoff (mass/ft3)
    let mut washoffLoad: f64; // pollutant washoff load over time step (lb or kg)
    let bmpRemoval: f64; // pollutant load removed by BMP treatment (lb or kg)
    let (iu, pu) = (i as usize, p as usize);

    // --- compute concen. of pollutant in washoff (mass/ft3)
    buildup = landFactor[iu].buildup[pu];
    landuseArea = landFactor[iu].fraction * area;
    washoffQual = landuse_getWashoffQual(prj, i, p, buildup, runoff, landuseArea);

    // --- compute washoff load exported (lbs or kg) from landuse
    //     (Pollut[].mcf converts from mg (or ug) mass units to lbs (or kg)
    washoffLoad = washoffQual * vOutflow * landuseArea / area * prj.Pollut[pu].mcf;

    // --- if buildup modelled, reduce it by amount of washoff
    if prj.Landuse[iu].buildupFunc[pu].funcType != NO_BUILDUP || buildup > washoffLoad {
        washoffLoad = MIN(washoffLoad, buildup);
        buildup -= washoffLoad;
        landFactor[iu].buildup[pu] = buildup;
    }
    // --- otherwise add washoff to buildup mass balance totals
    //     so that things will balance
    else {
        massbal_updateLoadingTotals(prj, BUILDUP_LOAD, p, washoffLoad);
        landFactor[iu].buildup[pu] = 0.0;
    }

    // --- apply any BMP removal to washoff
    bmpRemoval = prj.Landuse[iu].washoffFunc[pu].bmpEffic * washoffLoad;
    if bmpRemoval > 0.0 {
        massbal_updateLoadingTotals(prj, BMP_REMOVAL_LOAD, p, bmpRemoval);
        washoffLoad -= bmpRemoval;
    }

    // --- return washoff load converted back to mass (mg or ug)
    washoffLoad / prj.Pollut[pu].mcf
}

//=============================================================================

fn landuse_getWashoffQual(prj: &Project, i: i32, p: i32, buildup: f64, runoff: f64, area: f64) -> f64
//
//  Input:   i = land use index
//           p = pollutant index
//           buildup = current buildup over land use (lbs or kg)
//           runoff = current runoff on subcatchment (ft/sec)
//           area = area devoted to land use (ft2)
//  Output:  returns pollutant concentration in washoff (mass/ft3)
//  Purpose: finds concentration of pollutant washed off a land use.
//
//  Notes:   "coeff" for each washoff function was previously adjusted to
//           result in units of mass/sec
//
{
    let (iu, pu) = (i as usize, p as usize);
    let mut cWashoff: f64 = 0.0;
    let coeff = prj.Landuse[iu].washoffFunc[pu].coeff;
    let expon = prj.Landuse[iu].washoffFunc[pu].expon;
    let func = prj.Landuse[iu].washoffFunc[pu].funcType;

    // --- if no washoff function or no runoff, return 0
    if func == NO_WASHOFF || runoff == 0.0 {
        return 0.0;
    }

    // --- if buildup function exists but no current buildup, return 0
    if prj.Landuse[iu].buildupFunc[pu].funcType != NO_BUILDUP && buildup == 0.0 {
        return 0.0;
    }

    // --- Exponential Washoff function
    if func == EXPON_WASHOFF {
        // --- evaluate washoff eqn. with runoff in in/hr (or mm/hr)
        //     and buildup converted from lbs (or kg) to concen. mass units
        cWashoff = coeff * (runoff * UCF(prj, RAINFALL)).powf(expon) * buildup / prj.Pollut[pu].mcf;
        cWashoff /= runoff * area;
    }
    // --- Rating Curve Washoff function
    else if func == RATING_WASHOFF {
        cWashoff = coeff * (runoff * area).powf(expon - 1.0);
    }
    // --- Event Mean Concentration Washoff
    else if func == EMC_WASHOFF {
        cWashoff = coeff; // coeff includes LperFT3 factor
    }
    cWashoff
}

//=============================================================================

/// Called by: (entry point).
pub fn landuse_getCoPollutLoad(prj: &mut Project, p: i32, washoff: &[f64]) -> f64
//
//  Input:   p = pollutant index
//           washoff = pollut. washoff rate (mass/sec)
//  Output:  returns washoff mass added by co-pollutant relation (mass)
//  Purpose: finds washoff mass added by a co-pollutant of a given pollutant.
//
{
    let k: i32;
    let w: f64;

    // --- check if pollutant p has a co-pollutant k
    k = prj.Pollut[p as usize].coPollut;
    if k >= 0 {
        // --- compute addition to washoff from co-pollutant
        w = prj.Pollut[p as usize].coFraction * washoff[k as usize];

        // --- add washoff to buildup mass balance totals
        //     so that things will balance
        let mcf = prj.Pollut[p as usize].mcf;
        massbal_updateLoadingTotals(prj, BUILDUP_LOAD, p, w * mcf);
        return w;
    }
    0.0
}

//=============================================================================

fn landuse_getExternalBuildup(prj: &mut Project, i: i32, p: i32, buildup: f64, tStep: f64) -> f64
//
//  Input:   i = landuse index
//           p = pollutant index
//           buildup = buildup at start of time step (mass/unit)
//           tStep = time step (sec)
//  Output:  returns pollutant buildup at end of time interval (mass/unit)
//  Purpose: finds pollutant buildup contributed by external loading over a
//           given time step.
//
{
    let bf = prj.Landuse[i as usize].buildupFunc[p as usize];
    let maxBuildup = bf.coeff[0];
    let sf = bf.coeff[1]; // scaling factor
    let ts = f64_to_i32(bf.coeff[2].floor()); // time series index
    let mut rate: f64 = 0.0;
    let mut buildup = buildup;

    // --- no buildup increment at start of simulation
    if prj.NewRunoffTime == 0.0 {
        return 0.0;
    }

    // --- get buildup rate (mass/unit/day) over the interval
    if ts >= 0 {
        let x = getDateTime(prj, prj.NewRunoffTime);
        rate = sf * table_tseriesLookup(&mut prj.Tseries[ts as usize], x, FALSE as i8);
    }

    // --- compute buildup at end of time interval
    buildup = buildup + rate * tStep / SECperDAY as f64;
    buildup = MIN(buildup, maxBuildup);
    buildup
}
