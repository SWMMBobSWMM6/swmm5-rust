//! surfqual.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   surfqual.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     07/14/23  (Build 5.2.4)
//   Author:   L. Rossman
//
//   Subcatchment water quality functions.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Pollutant surface buildup and washoff functions were moved here from
//     subcatch.c.
//   - Support for separate accounting of LID drain flows included.
//   Build 5.1.014:
//   - Fixed bug in computing effective BMP removal by LIDs.
//   Build 5.2.4:
//   - Set low runoff flow concentrations to zero before computing runoff
//     mass loads rather than after so that they match wet weather mass
//     inflows reported for conveyance system nodes.
//-----------------------------------------------------------------------------

use crate::prelude::*;

/// The file-scope variables of surfqual.c.
#[derive(Default)]
pub struct SurfqualState {}

//-----------------------------------------------------------------------------
//  Imported variables
//-----------------------------------------------------------------------------
// Declared in RUNOFF.C
//   OutflowLoad  -> prj.runoff.OutflowLoad   (exported pollutant mass load)
//
// Volumes (ft3) for a subcatchment over a time step declared in SUBCATCH.C
//   Vinfil      -> prj.subcatch.Vinfil      (non-LID infiltration)
//   Vinflow     -> prj.subcatch.Vinflow     (non-LID precip + snowmelt + runon + ponded water)
//   Voutflow    -> prj.subcatch.Voutflow    (non-LID runoff to subcatchment's outlet)
//   VlidIn      -> prj.subcatch.VlidIn      (inflow to LID units)
//   VlidInfil   -> prj.subcatch.VlidInfil   (infiltration from LID units)
//   VlidOut     -> prj.subcatch.VlidOut     (surface outflow from LID units)
//   VlidDrain   -> prj.subcatch.VlidDrain   (drain outflow from LID units)
//   VlidReturn  -> prj.subcatch.VlidReturn  (LID outflow returned to pervious area)

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  surfqual_initState         (called from subcatch_initState)
//  surfqual_getWashoff        (called from runoff_execute)
//  surfqual_getBuildup        (called from runoff_execute)
//  surfqual_sweepBuildup      (called from runoff_execute)
//  surfqual_getWtdWashoff     (called from addWetWeatherInflows in routing.c)

//=============================================================================

/// Called by: subcatch.
pub fn surfqual_initState(prj: &mut Project, j: i32)
//
//  Input:   j = subcatchment index
//  Output:  none
//  Purpose: initializes pollutant buildup, ponded mass, and washoff.
//
{
    let ju = j as usize;

    // --- initialize washoff quality
    for p in 0..prj.Nobjects[POLLUT as usize] as usize {
        prj.Subcatch[ju].oldQual[p] = 0.0;
        prj.Subcatch[ju].newQual[p] = 0.0;
        prj.Subcatch[ju].pondedQual[p] = 0.0;
    }

    // --- initialize pollutant buildup
    //     (the land factors and initial buildups are taken out of the
    //      subcatchment for the call; landuse_getInitBuildup does not
    //      read them through prj)
    let mut landFactor = std::mem::take(&mut prj.Subcatch[ju].landFactor);
    let initBuildup = std::mem::take(&mut prj.Subcatch[ju].initBuildup);
    let area = prj.Subcatch[ju].area;
    let curb = prj.Subcatch[ju].curbLength;
    landuse_getInitBuildup(prj, &mut landFactor, &initBuildup, area, curb);
    prj.Subcatch[ju].landFactor = landFactor;
    prj.Subcatch[ju].initBuildup = initBuildup;
}

//=============================================================================

/// Called by: runoff.
pub fn surfqual_getBuildup(prj: &mut Project, j: i32, tStep: f64)
//
//  Input:   j = subcatchment index
//           tStep = time step (sec)
//  Output:  none
//  Purpose: adds to pollutant buildup on subcatchment surface.
//
{
    let mut f: f64; // land use fraction
    let mut area: f64; // land use area (acres or hectares)
    let mut curb: f64; // land use curb length (user units)
    let mut oldBuildup: f64; // buildup at start of time step
    let mut newBuildup: f64; // buildup at end of time step
    let ju = j as usize;

    // --- consider each landuse
    for i in 0..prj.Nobjects[LANDUSE as usize] {
        let iu = i as usize;
        // --- skip landuse if not in subcatch
        f = prj.Subcatch[ju].landFactor[iu].fraction;
        if f == 0.0 {
            continue;
        }

        // --- get land area (in acres or hectares) & curb length
        area = f * prj.Subcatch[ju].area * UCF(prj, LANDAREA);
        curb = f * prj.Subcatch[ju].curbLength;

        // --- examine each pollutant
        for p in 0..prj.Nobjects[POLLUT as usize] {
            let pu = p as usize;
            // --- see if snow-only buildup is in effect
            if prj.Pollut[pu].snowOnly != 0 && prj.Subcatch[ju].newSnowDepth < 0.001 / 12.0 {
                continue;
            }

            // --- use land use's buildup function to update buildup amount
            oldBuildup = prj.Subcatch[ju].landFactor[iu].buildup[pu];
            newBuildup = landuse_getBuildup(prj, i, p, area, curb, oldBuildup, tStep);
            newBuildup = MAX(newBuildup, oldBuildup);
            prj.Subcatch[ju].landFactor[iu].buildup[pu] = newBuildup;
            massbal_updateLoadingTotals(prj, BUILDUP_LOAD, p, newBuildup - oldBuildup);
        }
    }
}

//=============================================================================

/// Called by: runoff.
pub fn surfqual_sweepBuildup(prj: &mut Project, j: i32, aDate: DateTime)
//
//  Input:   j = subcatchment index
//           aDate = current date/time
//  Output:  none
//  Purpose: reduces pollutant buildup over a subcatchment if sweeping occurs.
//
{
    let mut oldBuildup: f64; // buildup before sweeping (lbs or kg)
    let mut newBuildup: f64; // buildup after sweeping (lbs or kg)
    let ju = j as usize;

    // --- no sweeping if there is snow on plowable impervious area
    if let Some(sp) = prj.Subcatch[ju].snowpack.as_ref() {
        if sp.wsnow[IMPERV0 as usize] > MIN_TOTAL_DEPTH {
            return;
        }
    }

    // --- consider each land use
    for i in 0..prj.Nobjects[LANDUSE as usize] as usize {
        // --- skip land use if not in subcatchment
        if prj.Subcatch[ju].landFactor[i].fraction == 0.0 {
            continue;
        }

        // --- see if land use is subject to sweeping
        if prj.Landuse[i].sweepInterval == 0.0 {
            continue;
        }

        // --- see if sweep interval has been reached
        if aDate - prj.Subcatch[ju].landFactor[i].lastSwept >= prj.Landuse[i].sweepInterval {
            // --- update time when last swept
            prj.Subcatch[ju].landFactor[i].lastSwept = aDate;

            // --- examine each pollutant
            for p in 0..prj.Nobjects[POLLUT as usize] {
                let pu = p as usize;
                // --- reduce buildup by the fraction available
                //     times the sweeping effic.
                oldBuildup = prj.Subcatch[ju].landFactor[i].buildup[pu];
                newBuildup = oldBuildup * (1.0 - prj.Landuse[i].sweepRemoval * prj.Landuse[i].washoffFunc[pu].sweepEffic);
                newBuildup = MIN(oldBuildup, newBuildup);
                newBuildup = MAX(0.0, newBuildup);
                prj.Subcatch[ju].landFactor[i].buildup[pu] = newBuildup;

                // --- update mass balance totals
                massbal_updateLoadingTotals(prj, SWEEPING_LOAD, p, oldBuildup - newBuildup);
            }
        }
    }
}

//=============================================================================

/// Called by: runoff.
pub fn surfqual_getWashoff(prj: &mut Project, j: i32, runoff: f64, tStep: f64)
//
//  Input:   j = subcatchment index
//           runoff = total subcatchment runoff before internal re-routing or
//                    LID controls (ft/sec)
//           tStep = time step (sec)
//  Output:  none
//  Purpose: computes new runoff quality for a subcatchment.
//
//  Considers three pollutant generating streams that are combined together:
//  1. washoff of pollutant buildup as described by the project's land use
//     washoff functions,
//  2. complete mix mass balance of pollutants in surface ponding on
//     non-LID area due to runon, wet deposition, infiltration, & evaporation,
//  3. wet deposition and runon over LID areas.
//
{
    let hasOutflow: i32; // TRUE if subcatchment has outflow
    let mut cOut: f64; // final washoff concentration (mass/ft3)
    let mut massLoad: f64; // pollut. mass load (mass)
    let vLidRain: f64; // rainfall volume on LID area (ft3)
    let mut vLidRunon: f64; // external runon volume to LID area (ft3)
    let vSurfOut: f64; // surface runoff volume leaving subcatchment (ft3)
    let vOut1: f64; // runoff volume prior to LID treatment (ft3)
    let vOut2: f64; // runoff volume after LID treatment (ft3)
    let area: f64; // subcatchment area (ft2)
    let ju = j as usize;

    // --- return if there is no area or no pollutants
    area = prj.Subcatch[ju].area;
    if prj.Nobjects[POLLUT as usize] == 0 || area == 0.0 {
        return;
    }

    // --- find contributions from washoff, runon and wet precip. to OutflowLoad
    for p in 0..prj.Nobjects[POLLUT as usize] as usize {
        prj.runoff.OutflowLoad[p] = 0.0;
    }
    findWashoffLoads(prj, j, runoff);
    findPondedLoads(prj, j, tStep);
    findLidLoads(prj, j, tStep);

    // --- contribution from direct rainfall on LID areas
    vLidRain = prj.Subcatch[ju].rainfall * prj.Subcatch[ju].lidArea * tStep;

    // --- contribution from upstream runon onto LID areas
    //     (only if LIDs occupy full subcatchment)
    vLidRunon = 0.0;
    if area == prj.Subcatch[ju].lidArea {
        vLidRunon = prj.Subcatch[ju].runon * area * tStep;
    }

    // --- runoff volume before LID treatment (ft3)
    //     (Voutflow, computed in subcatch_getRunoff, is subcatchment
    //      runoff volume before LID treatment)
    vOut1 = prj.subcatch.Voutflow + vLidRain + vLidRunon;

    // --- surface runoff + LID drain flow volume leaving the subcatchment
    //     (Subcatch.newRunoff, computed in subcatch_getRunoff, includes
    //      any surface runoff reduction from LID treatment)
    vSurfOut = prj.Subcatch[ju].newRunoff * tStep;
    vOut2 = vSurfOut + prj.subcatch.VlidDrain;

    // --- determine if subcatchment outflow is below a small cutoff
    hasOutflow = (vOut2 > MIN_RUNOFF * area * tStep) as i32;

    // --- for each pollutant
    for p in 0..prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        // --- convert washoff load to a concentration
        cOut = 0.0;
        if vOut1 > 0.0 && hasOutflow != 0 {
            cOut = prj.runoff.OutflowLoad[pu] / vOut1;
        }

        // --- assign any difference between pre- and post-LID
        //     subcatchment outflow loads to BMP removal
        if prj.Subcatch[ju].lidArea > 0.0 {
            massLoad = cOut * (vOut1 - vOut2) * prj.Pollut[pu].mcf;
            if massLoad > 0.0 {
                massbal_updateLoadingTotals(prj, BMP_REMOVAL_LOAD, p, massLoad);
            }
        }

        // --- update subcatchment's cumulative runoff load in lbs (or kg)
        massLoad = cOut * vOut2 * prj.Pollut[pu].mcf;
        prj.Subcatch[ju].totalLoad[pu] += massLoad;

        // --- update mass balance for surface runoff load routed to a
        //     conveyance system node
        //     (loads from LID drains are accounted for below since they
        //     can go to different outlets than parent subcatchment)
        if prj.Subcatch[ju].outNode >= 0 || prj.Subcatch[ju].outSubcatch == j {
            massLoad = cOut * vSurfOut * prj.Pollut[pu].mcf;
            massbal_updateLoadingTotals(prj, RUNOFF_LOAD, p, massLoad);
        }

        // --- save new washoff concentration
        prj.Subcatch[ju].newQual[pu] = cOut / LperFT3;
    }

    // --- add contribution of LID drain flows to mass balance
    if prj.Subcatch[ju].lidArea > 0.0 {
        // newQual is taken out for the call (lid_addDrainLoads only reads
        // the array it is given) and put back
        let mut newQual = std::mem::take(&mut prj.Subcatch[ju].newQual);
        lid_addDrainLoads(prj, j, &mut newQual, tStep);
        prj.Subcatch[ju].newQual = newQual;
    }
}

//=============================================================================

/// Called by: routing.
pub fn surfqual_getWtdWashoff(prj: &Project, j: i32, p: i32, f: f64) -> f64
//
//  Input:   j = subcatchment index
//           p = pollutant index
//           f = weighting factor
//  Output:  returns pollutant washoff value
//  Purpose: finds wtd. combination of old and new washoff for a pollutant.
//
{
    let s = &prj.Subcatch[j as usize];
    let p = p as usize;
    (1.0 - f) * s.oldRunoff * s.oldQual[p] + f * s.newRunoff * s.newQual[p]
}

//=============================================================================

#[allow(unused_assignments)] // C updates wPonded after its last use
fn findPondedLoads(prj: &mut Project, j: i32, tStep: f64)
//
//  Input:   j = subcatchment index
//           tStep = time step (sec)
//  Output:  updates pondedQual and OutflowLoad
//  Purpose: mixes wet deposition and runon pollutant loading with existing
//           ponded pollutant mass to compute an ouflow loading.
//
{
    let mut cPonded: f64; // ponded concentration (mass/ft3)
    let mut wPonded: f64; // pollutant mass in ponded water (mass)
    let mut bmpRemoval: f64; // load reduction by best mgt. practice (mass)
    let vRain: f64; // volume of direct precipitation (ft3)
    let mut wRain: f64; // wet deposition pollutant load (mass)
    let mut wRunon: f64; // external runon pollutant load (mass)
    let mut wInfil: f64; // pollutant load lost to infiltration (mass)
    let mut wOutflow: f64; // ponded water contribution to runoff load (mass)
    let fullArea: f64; // full subcatchment area (ft2)
    let nonLidArea: f64; // non-LID area (ft2)
    let ju = j as usize;

    // --- subcatchment and non-LID areas
    if prj.Subcatch[ju].area == prj.Subcatch[ju].lidArea {
        return;
    }
    fullArea = prj.Subcatch[ju].area;
    nonLidArea = fullArea - prj.Subcatch[ju].lidArea;

    // --- compute precip. volume over time step (ft3)
    vRain = prj.Subcatch[ju].rainfall * nonLidArea * tStep;

    for p in 0..prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        // --- update mass balance for wet deposition
        wRain = prj.Pollut[pu].pptConcen * LperFT3 * vRain;
        let mcf = prj.Pollut[pu].mcf;
        massbal_updateLoadingTotals(prj, DEPOSITION_LOAD, p, wRain * mcf);

        // --- surface is dry and has no runon -- add any remaining mass
        //     to overall mass balance's FINAL_LOAD category
        if prj.subcatch.Vinflow == 0.0 {
            let w = prj.Subcatch[ju].pondedQual[pu] * mcf;
            massbal_updateLoadingTotals(prj, FINAL_LOAD, p, w);
            prj.Subcatch[ju].pondedQual[pu] = 0.0;
        } else {
            // --- find concen. of ponded water
            //     (newQual[] temporarily holds runon mass loading)
            wRunon = prj.Subcatch[ju].newQual[pu] * tStep;
            wPonded = prj.Subcatch[ju].pondedQual[pu] + wRain + wRunon;
            cPonded = wPonded / prj.subcatch.Vinflow;

            // --- mass lost to infiltration
            wInfil = cPonded * prj.subcatch.Vinfil;
            wInfil = MIN(wInfil, wPonded);
            massbal_updateLoadingTotals(prj, INFIL_LOAD, p, wInfil * mcf);
            wPonded -= wInfil;

            // --- mass lost to runoff
            wOutflow = cPonded * prj.subcatch.Voutflow;
            wOutflow = MIN(wOutflow, wPonded);
            wPonded -= wOutflow;

            // --- reduce outflow load by average BMP removal
            bmpRemoval = landuse_getAvgBmpEffic(prj, j, p) * wOutflow;
            massbal_updateLoadingTotals(prj, BMP_REMOVAL_LOAD, p, bmpRemoval * mcf);
            wOutflow -= bmpRemoval;

            // --- update ponded mass (using newly computed ponded depth)
            prj.Subcatch[ju].pondedQual[pu] = cPonded * subcatch_getDepth(prj, j) * nonLidArea;
            prj.runoff.OutflowLoad[pu] += wOutflow;
        }
    }
}

//=============================================================================

fn findWashoffLoads(prj: &mut Project, j: i32, runoff: f64)
//
//  Input:   j = subcatchment index
//           runoff = subcatchment runoff before internal re-routing or
//                    LID controls (ft/sec)
//  Output:  updates OutflowLoad array
//  Purpose: computes pollutant washoff loads for each land use and adds these
//           to the subcatchment's total outflow loads.
//
{
    let mut k: i32; // co-pollutant index
    let mut w: f64; // co-pollutant load (mass)
    let ju = j as usize;
    let area = prj.Subcatch[ju].area; // subcatchment area (ft2)

    // --- examine each land use
    if runoff < MIN_RUNOFF {
        return;
    }
    for i in 0..prj.Nobjects[LANDUSE as usize] {
        if prj.Subcatch[ju].landFactor[i as usize].fraction > 0.0 {
            // --- compute load generated by washoff function
            for p in 0..prj.Nobjects[POLLUT as usize] {
                // the subcatchment's land factors are taken out for the call
                // (landuse_getWashoffLoad does not read them through prj)
                let mut landFactor = std::mem::take(&mut prj.Subcatch[ju].landFactor);
                let vOutflow = prj.subcatch.Voutflow;
                let w = landuse_getWashoffLoad(prj, i, p, area, &mut landFactor, runoff, vOutflow);
                prj.Subcatch[ju].landFactor = landFactor;
                prj.runoff.OutflowLoad[p as usize] += w;
            }
        }
    }

    // --- compute contribution from any co-pollutant
    for p in 0..prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        // --- check if pollutant p has a co-pollutant k
        k = prj.Pollut[pu].coPollut;
        if k >= 0 {
            // --- compute addition to washoff from co-pollutant
            w = prj.Pollut[pu].coFraction * prj.runoff.OutflowLoad[k as usize];

            // --- add this washoff to buildup mass balance totals
            //     so that things will balance
            let mcf = prj.Pollut[pu].mcf;
            massbal_updateLoadingTotals(prj, BUILDUP_LOAD, p, w * mcf);

            // --- then also add it to the total washoff load
            prj.runoff.OutflowLoad[pu] += w;
        }
    }
}

//=============================================================================

fn findLidLoads(prj: &mut Project, j: i32, tStep: f64)
//
//  Input:   j = subcatchment index
//           tStep = time step (sec)
//  Output:  updates OutflowLoad array
//  Purpose: finds addition to subcatchment pollutant loads from wet deposition
//           and upstream runon to LID areas.
//
{
    let useRunon: i32; // = 1 if LIDs receive upstream runon loads
    let lidArea: f64; // area occupied by LID units (ft2)
    let vLidRain: f64; // direct precip. falling on LID areas (ft3)
    let mut wLidRain: f64; // wet deposition pollut. load on LID areas (mass)
    let mut wLidRunon: f64; // runon pollut. load seen by LID areas (mass)
    let ju = j as usize;

    // --- find rainfall volume seen by LIDs
    lidArea = prj.Subcatch[ju].lidArea;
    if lidArea == 0.0 {
        return;
    }
    vLidRain = prj.Subcatch[ju].rainfall * lidArea * tStep;

    // --- use upstream runon load only if LIDs occupy full subcatchment
    //     (for partial LID coverage, runon loads were directed onto non-LID area)
    useRunon = (lidArea == prj.Subcatch[ju].area) as i32;

    for p in 0..prj.Nobjects[POLLUT as usize] {
        let pu = p as usize;
        // --- wet deposition load on LID area
        wLidRain = prj.Pollut[pu].pptConcen * vLidRain * LperFT3;
        let mcf = prj.Pollut[pu].mcf;
        massbal_updateLoadingTotals(prj, DEPOSITION_LOAD, p, wLidRain * mcf);

        // --- runon load to LID area from other subcatchments
        if useRunon != 0 {
            wLidRunon = prj.Subcatch[ju].newQual[pu] * tStep;
        } else {
            wLidRunon = 0.0;
        }

        // --- update total outflow pollutant load (mass)
        prj.runoff.OutflowLoad[pu] += wLidRain + wLidRunon;
    }
}
