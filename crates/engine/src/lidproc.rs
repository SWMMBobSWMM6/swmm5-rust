//! lidproc.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   lidproc.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     07/13/23   (Build 5.2.4)
//   Author:   L. Rossman
//
//   This module computes the hydrologic performance of an LID (Low Impact
//   Development) unit at a given point in time.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Euler integration now applied to all LID types except Vegetative
//     Swale which continues to use successive approximation.
//   - LID layer flux routines were re-written to more accurately model
//     flooded conditions.
//   Build 5.1.008:
//   - MAX_STATE_VARS replaced with MAX_LAYERS.
//   - Optional soil layer added to Porous Pavement LID.
//   - Rooftop Disconnection added to types of LIDs.
//   - Separate accounting of drain flows added.
//   - Indicator for currently wet LIDs added.
//   - Detailed reporting procedure fixed.
//   - Possibile negative head on Bioretention Cell drain avoided.
//   - Bug in computing flow through Green Roof drainage mat fixed.
//   Build 5.1.009:
//   - Fixed typo in net flux rate for vegetative swale LID.
//   Build 5.1.010:
//   - New modified version of Green-Ampt used for surface layer infiltration.
//   Build 5.1.011:
//   - Re-named STOR_INFIL to STOR_EXFIL and StorageInfil to StorageExfil to
//     better reflect their meaning.
//   - Evaporation rates from sub-surface layers reduced by fraction of
//     surface that is pervious (applies to block paver systems)
//   - Flux rate routines for LIDs with underdrains modified to produce more
//     physically meaningful results.
//   - Reporting of detailed results re-written.
//   Build 5.1.012:
//   - Modified upper limit for soil layer percolation.
//   - Modified upper limit on surface infiltration into rain gardens.
//   - Modified upper limit on drain flow for LIDs with storage layers.
//   - Used re-defined wasDry variable for LID reports to fix duplicate lines.
//   Build 5.1.013:
//   - Support added for open/closed head levels and multiplier v. head curve
//     to control underdrain flow.
//   - Support added for regenerating pavement permeability at fixed intervals.
//   Build 5.1.014:
//   - Fixed failure to initialize all LID layer moisture volumes to 0 before
//     computing LID unit performance in lidproc_getOutflow.
//   Build 5.2.0:
//   - Fixed failure to account for effect of Impervious Surface Fraction on
//     pavement permeability for Permeable Pavement LID
//   - Fixed units conversion for pavement depth in detailed report file.
//   Build 5.2.4:
//   - Modified flux limits in biocellFluxRates, pavementFluxRates and
//     trenchFluxRates.
//   - Corrected head calculation in getStorageDrainRate when unit has both
//     a soil and pavement layer.
//-----------------------------------------------------------------------------
//
//  Translation notes:
//  - C's module pointers theLidUnit / theLidProc are passed down as the
//    parameters `lidUnit` / `lidProc` (lid.c takes them out of the model
//    for the call). lidproc_saveResults uses its `lidUnit` argument where C
//    used theLidUnit; lid.c always passes the same unit to both calls.
//  - The flux-rate function pointer passed to modpuls_solve is a closure
//    over (prj, lidUnit, lidProc).

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const STOPTOL: f64 = 0.00328; // integration error tolerance in ft (= 1 mm)
const MINFLOW: f64 = 2.3e-8; // flow cutoff for dry conditions (= 0.001 in/hr)

//-----------------------------------------------------------------------------
//  Enumerations
//-----------------------------------------------------------------------------
// enum LidLayerTypes
const SURF: i32 = 0; // surface layer
const SOIL: i32 = 1; // soil layer
const STOR: i32 = 2; // storage layer
const PAVE: i32 = 3; // pavement layer
#[allow(dead_code)] // unused in lidproc.c
const DRAIN: i32 = 4; // underdrain system

// enum LidRptVars
const SURF_INFLOW: i32 = 0; // inflow to surface layer
const TOTAL_EVAP: i32 = 1; // evaporation rate from all layers
const SURF_INFIL: i32 = 2; // infiltration into surface layer
const PAVE_PERC: i32 = 3; // percolation through pavement layer
const SOIL_PERC: i32 = 4; // percolation through soil layer
const STOR_EXFIL: i32 = 5; // exfiltration out of storage layer
const SURF_OUTFLOW: i32 = 6; // outflow from surface layer
const STOR_DRAIN: i32 = 7; // outflow from storage layer
const SURF_DEPTH: i32 = 8; // ponded depth on surface layer
const PAVE_DEPTH: i32 = 9; // water level in pavement layer
const SOIL_MOIST: i32 = 10; // moisture content of soil layer
const STOR_DEPTH: i32 = 11; // water level in storage layer
const MAX_RPT_VARS: i32 = 12;

/// The file-scope variables of lidproc.c.
/// (theLidUnit and theLidProc are passed as parameters instead.)
#[derive(Default)]
#[allow(dead_code)] // Xold is written but never read, as in C
pub struct LidprocState {
    Tstep: f64,          // current time step (sec)
    EvapRate: f64,       // evaporation rate (ft/s)
    MaxNativeInfil: f64, // native soil infil. rate limit (ft/s)

    SurfaceInflow: f64,  // precip. + runon to LID unit (ft/s)
    SurfaceInfil: f64,   // infil. rate from surface layer (ft/s)
    SurfaceEvap: f64,    // evap. rate from surface layer (ft/s)
    SurfaceOutflow: f64, // outflow from surface layer (ft/s)
    SurfaceVolume: f64,  // volume in surface storage (ft)

    PaveEvap: f64,   // evap. from pavement layer (ft/s)
    PavePerc: f64,   // percolation from pavement layer (ft/s)
    PaveVolume: f64, // volume stored in pavement layer  (ft)

    SoilEvap: f64,   // evap. from soil layer (ft/s)
    SoilPerc: f64,   // percolation from soil layer (ft/s)
    SoilVolume: f64, // volume in soil/pavement storage (ft)

    StorageInflow: f64, // inflow rate to storage layer (ft/s)
    StorageExfil: f64,  // exfil. rate from storage layer (ft/s)
    StorageEvap: f64,   // evap.rate from storage layer (ft/s)
    StorageDrain: f64,  // underdrain flow rate layer (ft/s)
    StorageVolume: f64, // volume in storage layer (ft)

    Xold: [f64; MAX_LAYERS as usize], // previous moisture level in LID layers
}

/// C: `void (*derivs)(double*, double*)` bound to the current unit/process.
type FluxRatesFn = fn(&mut Project, &mut TLidUnit, &TLidProc, &[f64], &mut [f64]);

//=============================================================================

/// Called by: lid.
pub fn lidproc_initWaterBalance(lidUnit: &mut TLidUnit, initVol: f64) {
    //
    //  Purpose: initializes the water balance components of a LID unit.
    //  Input:   lidUnit = a particular LID unit
    //           initVol = initial water volume stored in the unit (ft)
    //  Output:  none
    //
    lidUnit.waterBalance.inflow = 0.0;
    lidUnit.waterBalance.evap = 0.0;
    lidUnit.waterBalance.infil = 0.0;
    lidUnit.waterBalance.surfFlow = 0.0;
    lidUnit.waterBalance.drainFlow = 0.0;
    lidUnit.waterBalance.initVol = initVol;
    lidUnit.waterBalance.finalVol = initVol;
}

//=============================================================================

/// Called by: lid.
#[allow(unused_assignments)] // C also discards modpuls_solve's result
pub fn lidproc_getOutflow(
    prj: &mut Project,
    lidUnit: &mut TLidUnit,
    lidProc: &TLidProc,
    inflow: f64,
    evap: f64,
    infil: f64,
    maxInfil: f64,
    tStep: f64,
    lidEvap: &mut f64,
    lidInfil: &mut f64,
    lidDrain: &mut f64,
) -> f64 {
    //
    //  Purpose: computes runoff outflow from a single LID unit.
    //  Input:   lidUnit  = ptr. to specific LID unit being analyzed
    //           lidProc  = ptr. to generic LID process of the LID unit
    //           inflow   = runoff rate captured by LID unit (ft/s)
    //           evap     = potential evaporation rate (ft/s)
    //           infil    = infiltration rate to native soil (ft/s)
    //           maxInfil = max. infiltration rate to native soil (ft/s)
    //           tStep    = time step (sec)
    //  Output:  lidEvap  = evaporation rate for LID unit (ft/s)
    //           lidInfil = infiltration rate for LID unit (ft/s)
    //           lidDrain = drain flow for LID unit (ft/s)
    //           returns surface runoff rate from the LID unit (ft/s)
    //
    let mut i: i32;
    let mut x = [0.0f64; MAX_LAYERS as usize]; // layer moisture levels
    let mut xOld = [0.0f64; MAX_LAYERS as usize]; // work vector
    let mut xPrev = [0.0f64; MAX_LAYERS as usize]; // work vector
    let mut xMin = [0.0f64; MAX_LAYERS as usize]; // lower limit on moisture levels
    let mut xMax = [0.0f64; MAX_LAYERS as usize]; // upper limit on moisture levels
    let mut fOld = [0.0f64; MAX_LAYERS as usize]; // previously computed flux rates
    let mut f = [0.0f64; MAX_LAYERS as usize]; // newly computed flux rates

    // convergence tolerance on moisture levels (ft, moisture fraction , ft)
    let xTol: [f64; MAX_LAYERS as usize] = [STOPTOL, STOPTOL, STOPTOL, STOPTOL];

    let mut omega = 0.0f64; // integration time weighting

    //... define a pointer to function that computes flux rates through the LID
    let fluxRates: FluxRatesFn;

    //... save references to the LID process and LID unit
    //    (theLidProc = lidProc; theLidUnit = lidUnit; -- passed as parameters)

    //... save evap, max. infil. & time step to shared variables
    prj.lidproc.EvapRate = evap;
    prj.lidproc.MaxNativeInfil = maxInfil;
    prj.lidproc.Tstep = tStep;

    //... store current moisture levels in vector x
    x[SURF as usize] = lidUnit.surfaceDepth;
    x[SOIL as usize] = lidUnit.soilMoisture;
    x[STOR as usize] = lidUnit.storageDepth;
    x[PAVE as usize] = lidUnit.paveDepth;

    //... initialize layer moisture volumes, flux rates and moisture limits
    prj.lidproc.SurfaceVolume = 0.0;
    prj.lidproc.PaveVolume = 0.0;
    prj.lidproc.SoilVolume = 0.0;
    prj.lidproc.StorageVolume = 0.0;
    prj.lidproc.SurfaceInflow = inflow;
    prj.lidproc.SurfaceInfil = 0.0;
    prj.lidproc.SurfaceEvap = 0.0;
    prj.lidproc.SurfaceOutflow = 0.0;
    prj.lidproc.PaveEvap = 0.0;
    prj.lidproc.PavePerc = 0.0;
    prj.lidproc.SoilEvap = 0.0;
    prj.lidproc.SoilPerc = 0.0;
    prj.lidproc.StorageInflow = 0.0;
    prj.lidproc.StorageExfil = 0.0;
    prj.lidproc.StorageEvap = 0.0;
    prj.lidproc.StorageDrain = 0.0;
    i = 0;
    while i < MAX_LAYERS {
        f[i as usize] = 0.0;
        fOld[i as usize] = lidUnit.oldFluxRates[i as usize];
        xMin[i as usize] = 0.0;
        xMax[i as usize] = BIG;
        prj.lidproc.Xold[i as usize] = x[i as usize];
        i += 1;
    }

    //... find Green-Ampt infiltration from surface layer
    if lidProc.lidType == POROUS_PAVEMENT {
        prj.lidproc.SurfaceInfil = 0.0;
    } else if lidUnit.soilInfil.Ks > 0.0 {
        let Tstep = prj.lidproc.Tstep;
        let SurfaceInflow = prj.lidproc.SurfaceInflow;
        let surfaceDepth = lidUnit.surfaceDepth;
        prj.lidproc.SurfaceInfil = grnampt_getInfil(
            &mut prj.infil,
            &mut lidUnit.soilInfil,
            Tstep,
            SurfaceInflow,
            surfaceDepth,
            MOD_GREEN_AMPT,
        );
    } else {
        prj.lidproc.SurfaceInfil = infil;
    }

    //... set moisture limits for soil & storage layers
    if lidProc.soil.thickness > 0.0 {
        xMin[SOIL as usize] = lidProc.soil.wiltPoint;
        xMax[SOIL as usize] = lidProc.soil.porosity;
    }
    if lidProc.pavement.thickness > 0.0 {
        xMax[PAVE as usize] = lidProc.pavement.thickness;
    }
    if lidProc.storage.thickness > 0.0 {
        xMax[STOR as usize] = lidProc.storage.thickness;
    }
    if lidProc.lidType == GREEN_ROOF {
        xMax[STOR as usize] = lidProc.drainMat.thickness;
    }

    //... determine which flux rate function to use
    match lidProc.lidType {
        BIO_CELL | RAIN_GARDEN => fluxRates = biocellFluxRates,
        GREEN_ROOF => fluxRates = greenRoofFluxRates,
        INFIL_TRENCH => fluxRates = trenchFluxRates,
        POROUS_PAVEMENT => fluxRates = pavementFluxRates,
        RAIN_BARREL => fluxRates = barrelFluxRates,
        ROOF_DISCON => fluxRates = roofFluxRates,
        VEG_SWALE => {
            fluxRates = swaleFluxRates;
            omega = 0.5;
        }
        _ => return 0.0,
    }

    //... update moisture levels and flux rates over the time step
    i = modpuls_solve(
        MAX_LAYERS,
        &mut x,
        &mut xOld,
        &mut xPrev,
        &xMin,
        &xMax,
        &xTol,
        &fOld,
        &mut f,
        tStep,
        omega,
        &mut |xx: &[f64], ff: &mut [f64]| fluxRates(prj, lidUnit, lidProc, xx, ff),
    );

    /* For debugging only ********************************************
        if  (i == 0)
        {
            fprintf(Frpt.file,
            "\n  WARNING 09: integration failed to converge at %s %s",
                theDate, theTime);
            fprintf(Frpt.file,
            "\n              for LID %s placed in subcatchment %s.",
                theLidProc->ID, theSubcatch->ID);
        }
    *******************************************************************/

    //... add any surface overflow to surface outflow
    if lidProc.surface.canOverflow != 0 || lidUnit.fullWidth == 0.0 {
        let r = getSurfaceOverflowRate(prj, lidProc, &mut x[SURF as usize]);
        prj.lidproc.SurfaceOutflow += r;
    }

    //... save updated results
    lidUnit.surfaceDepth = x[SURF as usize];
    lidUnit.paveDepth = x[PAVE as usize];
    lidUnit.soilMoisture = x[SOIL as usize];
    lidUnit.storageDepth = x[STOR as usize];
    i = 0;
    while i < MAX_LAYERS {
        lidUnit.oldFluxRates[i as usize] = f[i as usize];
        i += 1;
    }

    //... assign values to LID unit evaporation, infiltration & drain flow
    *lidEvap = prj.lidproc.SurfaceEvap + prj.lidproc.PaveEvap + prj.lidproc.SoilEvap + prj.lidproc.StorageEvap;
    *lidInfil = prj.lidproc.StorageExfil;
    *lidDrain = prj.lidproc.StorageDrain;

    //... return surface outflow (per unit area) from unit
    prj.lidproc.SurfaceOutflow
}

//=============================================================================

/// Called by: lid.
pub fn lidproc_saveResults(prj: &mut Project, lidUnit: &mut TLidUnit, ucfRainfall: f64, ucfRainDepth: f64) {
    //
    //  Purpose: updates the mass balance for an LID unit and saves
    //           current flux rates to the LID report file.
    //  Input:   lidUnit = ptr. to LID unit
    //           ucfRainfall = units conversion factor for rainfall rate
    //           ucfDepth = units conversion factor for rainfall depth
    //  Output:  none
    //
    //  (C uses theLidUnit for everything but the rptFile test; it is the
    //   same unit as lidUnit on every call, see the module notes.)
    //
    let mut ucf: f64; // units conversion factor
    let totalEvap: f64; // total evaporation rate (ft/s)
    let totalVolume: f64; // total volume stored in LID (ft)
    let mut rptVars = [0.0f64; MAX_RPT_VARS as usize]; // array of reporting variables
    let mut isDry = FALSE; // true if current state of LID is dry
    let mut timeStamp: Vec<u8> = Vec::new(); // date/time stamp (char[TIME_STAMP_SIZE+1])
    let elapsedHrs: f64; // elapsed hours

    //... find total evap. rate and stored volume
    totalEvap = prj.lidproc.SurfaceEvap + prj.lidproc.PaveEvap + prj.lidproc.SoilEvap + prj.lidproc.StorageEvap;
    totalVolume = prj.lidproc.SurfaceVolume + prj.lidproc.PaveVolume + prj.lidproc.SoilVolume + prj.lidproc.StorageVolume;

    //... update mass balance totals
    let SurfaceInflow = prj.lidproc.SurfaceInflow;
    let StorageExfil = prj.lidproc.StorageExfil;
    let SurfaceOutflow = prj.lidproc.SurfaceOutflow;
    let StorageDrain = prj.lidproc.StorageDrain;
    updateWaterBalance(prj, lidUnit, SurfaceInflow, totalEvap, StorageExfil, SurfaceOutflow, StorageDrain, totalVolume);

    //... check if dry-weather conditions hold
    if prj.lidproc.SurfaceInflow < MINFLOW
        && prj.lidproc.SurfaceOutflow < MINFLOW
        && prj.lidproc.StorageDrain < MINFLOW
        && prj.lidproc.StorageExfil < MINFLOW
        && totalEvap < MINFLOW
    {
        isDry = TRUE;
    }

    //... update status of HasWetLids
    if isDry == 0 {
        prj.runoff.HasWetLids = TRUE as i8;
    }

    //... write results to LID report file
    if lidUnit.rptFile.is_some() {
        //... convert rate results to original units (in/hr or mm/hr)
        ucf = ucfRainfall;
        rptVars[SURF_INFLOW as usize] = prj.lidproc.SurfaceInflow * ucf;
        rptVars[TOTAL_EVAP as usize] = totalEvap * ucf;
        rptVars[SURF_INFIL as usize] = prj.lidproc.SurfaceInfil * ucf;
        rptVars[PAVE_PERC as usize] = prj.lidproc.PavePerc * ucf;
        rptVars[SOIL_PERC as usize] = prj.lidproc.SoilPerc * ucf;
        rptVars[STOR_EXFIL as usize] = prj.lidproc.StorageExfil * ucf;
        rptVars[SURF_OUTFLOW as usize] = prj.lidproc.SurfaceOutflow * ucf;
        rptVars[STOR_DRAIN as usize] = prj.lidproc.StorageDrain * ucf;

        //... convert storage results to original units (in or mm)
        ucf = ucfRainDepth;
        rptVars[SURF_DEPTH as usize] = lidUnit.surfaceDepth * ucf;
        rptVars[PAVE_DEPTH as usize] = lidUnit.paveDepth * ucf;
        rptVars[SOIL_MOIST as usize] = lidUnit.soilMoisture;
        rptVars[STOR_DEPTH as usize] = lidUnit.storageDepth * ucf;

        //... if the current LID state is wet but the previous state was dry
        //    for more than one period then write the saved previous results
        //    to the report file thus marking the end of a dry period
        if isDry == 0 && lidUnit.rptFile.as_ref().unwrap().wasDry > 1 {
            let rf = lidUnit.rptFile.as_mut().unwrap();
            let results = rf.results;
            cprintf!(rf, b"%s", results);
        }

        //... write the current results to a string which is saved between
        //    reporting periods
        elapsedHrs = prj.NewRunoffTime / 1000.0 / 3600.0;
        let aDate = getDateTime(prj, prj.NewRunoffTime);
        datetime_getTimeStamp(prj, M_D_Y, aDate, TIME_STAMP_SIZE, &mut timeStamp);
        {
            // snprintf(results, sizeof(results), ...): at most 255 bytes + NUL
            let s = cformat!(
                b"\n%20s\t %8.3f\t %8.3f\t %8.4f\t %8.3f\t %8.3f\t %8.3f\t %8.3f\t%8.3f\t %8.3f\t %8.3f\t %8.3f\t %8.3f\t %8.3f",
                timeStamp,
                elapsedHrs,
                rptVars[0],
                rptVars[1],
                rptVars[2],
                rptVars[3],
                rptVars[4],
                rptVars[5],
                rptVars[6],
                rptVars[7],
                rptVars[8],
                rptVars[9],
                rptVars[10],
                rptVars[11]
            );
            let rf = lidUnit.rptFile.as_mut().unwrap();
            let size = rf.results.len();
            let n = s.len().min(size - 1);
            rf.results[..n].copy_from_slice(&s[..n]);
            rf.results[n] = 0;
        }

        let rf = lidUnit.rptFile.as_mut().unwrap();
        //... if the current LID state is dry
        if isDry != 0 {
            //... if the previous state was wet then write the current
            //    results to file marking the start of a dry period
            if rf.wasDry == 0 {
                let results = rf.results;
                cprintf!(rf, b"%s", results);
            }

            //... increment the number of successive dry periods
            rf.wasDry += 1;
        }
        //... if the current LID state is wet
        else {
            //... write the current results to the report file
            let results = rf.results;
            cprintf!(rf, b"%s", results);

            //... re-set the number of successive dry periods to 0
            rf.wasDry = 0;
        }
    }
}

//=============================================================================

fn roofFluxRates(prj: &mut Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc, x: &[f64], f: &mut [f64]) {
    //
    //  Purpose: computes flux rates for roof disconnection.
    //  Input:   x = vector of storage levels
    //  Output:  f = vector of flux rates
    //
    let mut surfaceDepth = x[SURF as usize];

    getEvapRates(prj, surfaceDepth, 0.0, 0.0, 0.0, 1.0);
    prj.lidproc.SurfaceVolume = surfaceDepth;
    prj.lidproc.SurfaceInfil = 0.0;
    if lidProc.surface.alpha > 0.0 {
        prj.lidproc.SurfaceOutflow = getSurfaceOutflowRate(prj, lidUnit, lidProc, surfaceDepth);
    } else {
        getSurfaceOverflowRate(prj, lidProc, &mut surfaceDepth);
    }
    prj.lidproc.StorageDrain = MIN(lidProc.drain.coeff / UCF(prj, RAINFALL), prj.lidproc.SurfaceOutflow);
    prj.lidproc.SurfaceOutflow -= prj.lidproc.StorageDrain;
    f[SURF as usize] = prj.lidproc.SurfaceInflow - prj.lidproc.SurfaceEvap - prj.lidproc.StorageDrain - prj.lidproc.SurfaceOutflow;
}

//=============================================================================

fn greenRoofFluxRates(prj: &mut Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc, x: &[f64], f: &mut [f64]) {
    //
    //  Purpose: computes flux rates from the layers of a green roof.
    //  Input:   x = vector of storage levels
    //  Output:  f = vector of flux rates
    //
    // Moisture level variables
    let surfaceDepth: f64;
    let soilTheta: f64;
    let storageDepth: f64;

    // Intermediate variables
    let mut availVolume: f64;
    let mut maxRate: f64;

    // Green roof properties
    let soilThickness = lidProc.soil.thickness;
    let storageThickness = lidProc.storage.thickness;
    let soilPorosity = lidProc.soil.porosity;
    let storageVoidFrac = lidProc.storage.voidFrac;
    let soilFieldCap = lidProc.soil.fieldCap;
    let soilWiltPoint = lidProc.soil.wiltPoint;

    //... retrieve moisture levels from input vector
    surfaceDepth = x[SURF as usize];
    soilTheta = x[SOIL as usize];
    storageDepth = x[STOR as usize];

    //... convert moisture levels to volumes
    prj.lidproc.SurfaceVolume = surfaceDepth * lidProc.surface.voidFrac;
    prj.lidproc.SoilVolume = soilTheta * soilThickness;
    prj.lidproc.StorageVolume = storageDepth * storageVoidFrac;

    //... get ET rates
    availVolume = prj.lidproc.SoilVolume - soilWiltPoint * soilThickness;
    let SurfaceVolume = prj.lidproc.SurfaceVolume;
    let StorageVolume = prj.lidproc.StorageVolume;
    getEvapRates(prj, SurfaceVolume, 0.0, availVolume, StorageVolume, 1.0);
    if soilTheta >= soilPorosity {
        prj.lidproc.StorageEvap = 0.0;
    }

    //... soil layer perc rate
    prj.lidproc.SoilPerc = getSoilPercRate(lidProc, soilTheta);

    //... limit perc rate by available water
    availVolume = (soilTheta - soilFieldCap) * soilThickness;
    maxRate = MAX(availVolume, 0.0) / prj.lidproc.Tstep - prj.lidproc.SoilEvap;
    prj.lidproc.SoilPerc = MIN(prj.lidproc.SoilPerc, maxRate);
    prj.lidproc.SoilPerc = MAX(prj.lidproc.SoilPerc, 0.0);

    //... storage (drain mat) outflow rate
    prj.lidproc.StorageExfil = 0.0;
    prj.lidproc.StorageDrain = getDrainMatOutflow(prj, lidUnit, lidProc, storageDepth);

    //... unit is full
    if soilTheta >= soilPorosity && storageDepth >= storageThickness {
        //... outflow from both layers equals limiting rate
        maxRate = MIN(prj.lidproc.SoilPerc, prj.lidproc.StorageDrain);
        prj.lidproc.SoilPerc = maxRate;
        prj.lidproc.StorageDrain = maxRate;

        //... adjust inflow rate to soil layer
        prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, maxRate);
    }
    //... unit not full
    else {
        //... limit drainmat outflow by available storage volume
        maxRate = storageDepth * storageVoidFrac / prj.lidproc.Tstep - prj.lidproc.StorageEvap;
        if storageDepth >= storageThickness {
            maxRate += prj.lidproc.SoilPerc;
        }
        maxRate = MAX(maxRate, 0.0);
        prj.lidproc.StorageDrain = MIN(prj.lidproc.StorageDrain, maxRate);

        //... limit soil perc inflow by unused storage volume
        maxRate = (storageThickness - storageDepth) * storageVoidFrac / prj.lidproc.Tstep
            + prj.lidproc.StorageDrain
            + prj.lidproc.StorageEvap;
        prj.lidproc.SoilPerc = MIN(prj.lidproc.SoilPerc, maxRate);

        //... adjust surface infil. so soil porosity not exceeded
        maxRate = (soilPorosity - soilTheta) * soilThickness / prj.lidproc.Tstep + prj.lidproc.SoilPerc + prj.lidproc.SoilEvap;
        prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, maxRate);
    }

    // ... find surface outflow rate
    prj.lidproc.SurfaceOutflow = getSurfaceOutflowRate(prj, lidUnit, lidProc, surfaceDepth);

    // ... compute overall layer flux rates
    f[SURF as usize] = (prj.lidproc.SurfaceInflow - prj.lidproc.SurfaceEvap - prj.lidproc.SurfaceInfil - prj.lidproc.SurfaceOutflow)
        / lidProc.surface.voidFrac;
    f[SOIL as usize] = (prj.lidproc.SurfaceInfil - prj.lidproc.SoilEvap - prj.lidproc.SoilPerc) / lidProc.soil.thickness;
    f[STOR as usize] = (prj.lidproc.SoilPerc - prj.lidproc.StorageEvap - prj.lidproc.StorageDrain) / lidProc.storage.voidFrac;
}

//=============================================================================

fn biocellFluxRates(prj: &mut Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc, x: &[f64], f: &mut [f64]) {
    //
    //  Purpose: computes flux rates from the layers of a bio-retention cell LID.
    //  Input:   x = vector of storage levels
    //  Output:  f = vector of flux rates
    //
    // Moisture level variables
    let surfaceDepth: f64;
    let soilTheta: f64;
    let storageDepth: f64;

    // Intermediate variables
    let mut availVolume: f64;
    let mut maxRate: f64;

    // LID layer properties
    let soilThickness = lidProc.soil.thickness;
    let soilPorosity = lidProc.soil.porosity;
    let soilFieldCap = lidProc.soil.fieldCap;
    let soilWiltPoint = lidProc.soil.wiltPoint;
    let storageThickness = lidProc.storage.thickness;
    let storageVoidFrac = lidProc.storage.voidFrac;

    //... retrieve moisture levels from input vector
    surfaceDepth = x[SURF as usize];
    soilTheta = x[SOIL as usize];
    storageDepth = x[STOR as usize];

    //... convert moisture levels to volumes
    prj.lidproc.SurfaceVolume = surfaceDepth * lidProc.surface.voidFrac;
    prj.lidproc.SoilVolume = soilTheta * soilThickness;
    prj.lidproc.StorageVolume = storageDepth * storageVoidFrac;

    //... get ET rates
    availVolume = prj.lidproc.SoilVolume - soilWiltPoint * soilThickness;
    let SurfaceVolume = prj.lidproc.SurfaceVolume;
    let StorageVolume = prj.lidproc.StorageVolume;
    getEvapRates(prj, SurfaceVolume, 0.0, availVolume, StorageVolume, 1.0);
    if soilTheta >= soilPorosity {
        prj.lidproc.StorageEvap = 0.0;
    }

    //... soil layer perc rate
    prj.lidproc.SoilPerc = getSoilPercRate(lidProc, soilTheta);

    //... limit perc rate by available water
    availVolume = (soilTheta - soilFieldCap) * soilThickness;
    maxRate = MAX(availVolume, 0.0) / prj.lidproc.Tstep - prj.lidproc.SoilEvap;
    prj.lidproc.SoilPerc = MIN(prj.lidproc.SoilPerc, maxRate);
    prj.lidproc.SoilPerc = MAX(prj.lidproc.SoilPerc, 0.0);

    //... exfiltration rate out of storage layer
    prj.lidproc.StorageExfil = getStorageExfilRate(prj, lidUnit, lidProc);

    //... underdrain flow rate
    prj.lidproc.StorageDrain = 0.0;
    if lidProc.drain.coeff > 0.0 {
        prj.lidproc.StorageDrain = getStorageDrainRate(prj, lidUnit, lidProc, storageDepth, soilTheta, 0.0, surfaceDepth);
    }

    //... special case of no storage layer present
    if storageThickness == 0.0 {
        prj.lidproc.StorageEvap = 0.0;
        maxRate = MIN(prj.lidproc.SoilPerc, prj.lidproc.StorageExfil);
        prj.lidproc.SoilPerc = maxRate;
        prj.lidproc.StorageExfil = maxRate;

        //... limit surface infil. by unused soil volume
        maxRate = (soilPorosity - soilTheta) * soilThickness / prj.lidproc.Tstep + prj.lidproc.SoilPerc + prj.lidproc.SoilEvap;
        prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, maxRate);
    } else {
        //... storage & soil layers are full
        if soilTheta >= soilPorosity && storageDepth >= storageThickness {
            //... limiting rate is smaller of soil perc and storage outflow
            maxRate = prj.lidproc.StorageExfil + prj.lidproc.StorageDrain;
            if prj.lidproc.SoilPerc < maxRate {
                maxRate = prj.lidproc.SoilPerc;
                if maxRate > prj.lidproc.StorageExfil {
                    prj.lidproc.StorageDrain = maxRate - prj.lidproc.StorageExfil;
                } else {
                    prj.lidproc.StorageExfil = maxRate;
                    prj.lidproc.StorageDrain = 0.0;
                }
            } else {
                prj.lidproc.SoilPerc = maxRate;
            }

            //... apply limiting rate to surface infil.
            prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, maxRate);
        }
        //... either layer not full
        else {
            //... limit storage exfiltration by available storage volume
            maxRate = prj.lidproc.SoilPerc - prj.lidproc.StorageEvap + storageDepth * storageVoidFrac / prj.lidproc.Tstep;
            prj.lidproc.StorageExfil = MIN(prj.lidproc.StorageExfil, maxRate);
            prj.lidproc.StorageExfil = MAX(prj.lidproc.StorageExfil, 0.0);

            //... limit underdrain flow by volume above drain offset
            if prj.lidproc.StorageDrain > 0.0 {
                maxRate = -prj.lidproc.StorageExfil - prj.lidproc.StorageEvap;
                if storageDepth >= storageThickness {
                    maxRate += prj.lidproc.SoilPerc;
                }
                if lidProc.drain.offset <= storageDepth {
                    maxRate += (storageDepth - lidProc.drain.offset) * storageVoidFrac / prj.lidproc.Tstep;
                }
                maxRate = MAX(maxRate, 0.0);
                prj.lidproc.StorageDrain = MIN(prj.lidproc.StorageDrain, maxRate);
            }

            //... limit soil perc by unused storage volume
            maxRate = prj.lidproc.StorageExfil
                + prj.lidproc.StorageDrain
                + prj.lidproc.StorageEvap
                + (storageThickness - storageDepth) * storageVoidFrac / prj.lidproc.Tstep;
            prj.lidproc.SoilPerc = MIN(prj.lidproc.SoilPerc, maxRate);

            //... limit surface infil. by unused soil volume
            maxRate = (soilPorosity - soilTheta) * soilThickness / prj.lidproc.Tstep + prj.lidproc.SoilPerc + prj.lidproc.SoilEvap;
            prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, maxRate);
        }
    }

    //... find surface layer outflow rate
    prj.lidproc.SurfaceOutflow = getSurfaceOutflowRate(prj, lidUnit, lidProc, surfaceDepth);

    //... compute overall layer flux rates
    f[SURF as usize] = (prj.lidproc.SurfaceInflow - prj.lidproc.SurfaceEvap - prj.lidproc.SurfaceInfil - prj.lidproc.SurfaceOutflow)
        / lidProc.surface.voidFrac;
    f[SOIL as usize] = (prj.lidproc.SurfaceInfil - prj.lidproc.SoilEvap - prj.lidproc.SoilPerc) / lidProc.soil.thickness;
    if storageThickness == 0.0 {
        f[STOR as usize] = 0.0;
    } else {
        f[STOR as usize] = (prj.lidproc.SoilPerc - prj.lidproc.StorageEvap - prj.lidproc.StorageExfil - prj.lidproc.StorageDrain)
            / lidProc.storage.voidFrac;
    }
}

//=============================================================================

#[allow(unused_assignments)] // C computes availVolume here and never uses it
fn trenchFluxRates(prj: &mut Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc, x: &[f64], f: &mut [f64]) {
    //
    //  Purpose: computes flux rates from the layers of an infiltration trench LID.
    //  Input:   x = vector of storage levels
    //  Output:  f = vector of flux rates
    //
    // Moisture level variables
    let surfaceDepth: f64;
    let storageDepth: f64;

    // Intermediate variables
    let availVolume: f64;
    let mut maxRate: f64;

    // Storage layer properties
    let storageThickness = lidProc.storage.thickness;
    let storageVoidFrac = lidProc.storage.voidFrac;

    //... retrieve moisture levels from input vector
    surfaceDepth = x[SURF as usize];
    storageDepth = x[STOR as usize];

    //... convert moisture levels to volumes
    prj.lidproc.SurfaceVolume = surfaceDepth * lidProc.surface.voidFrac;
    prj.lidproc.SoilVolume = 0.0;
    prj.lidproc.StorageVolume = storageDepth * storageVoidFrac;

    //... get ET rates
    availVolume = (storageThickness - storageDepth) * storageVoidFrac;
    let SurfaceVolume = prj.lidproc.SurfaceVolume;
    let StorageVolume = prj.lidproc.StorageVolume;
    getEvapRates(prj, SurfaceVolume, 0.0, 0.0, StorageVolume, 1.0);

    //... no storage evap if surface ponded
    if surfaceDepth > 0.0 {
        prj.lidproc.StorageEvap = 0.0;
    }

    //... nominal storage inflow
    prj.lidproc.StorageInflow = prj.lidproc.SurfaceInflow + prj.lidproc.SurfaceVolume / prj.lidproc.Tstep;

    //... exfiltration rate out of storage layer
    prj.lidproc.StorageExfil = getStorageExfilRate(prj, lidUnit, lidProc);

    //... underdrain flow rate
    prj.lidproc.StorageDrain = 0.0;
    if lidProc.drain.coeff > 0.0 {
        prj.lidproc.StorageDrain = getStorageDrainRate(prj, lidUnit, lidProc, storageDepth, 0.0, 0.0, surfaceDepth);
    }

    //... limit storage exfiltration by available storage volume
    maxRate = prj.lidproc.StorageInflow - prj.lidproc.StorageEvap + storageDepth * storageVoidFrac / prj.lidproc.Tstep;
    prj.lidproc.StorageExfil = MIN(prj.lidproc.StorageExfil, maxRate);
    prj.lidproc.StorageExfil = MAX(prj.lidproc.StorageExfil, 0.0);

    //... limit underdrain flow by volume above drain offset
    if prj.lidproc.StorageDrain > 0.0 {
        maxRate = -prj.lidproc.StorageExfil - prj.lidproc.StorageEvap;
        if storageDepth >= storageThickness {
            maxRate += prj.lidproc.StorageInflow;
        }
        if lidProc.drain.offset <= storageDepth {
            maxRate += (storageDepth - lidProc.drain.offset) * storageVoidFrac / prj.lidproc.Tstep;
        }
        maxRate = MAX(maxRate, 0.0);
        prj.lidproc.StorageDrain = MIN(prj.lidproc.StorageDrain, maxRate);
    }

    //... limit storage inflow to not exceed storage layer capacity
    maxRate = (storageThickness - storageDepth) * storageVoidFrac / prj.lidproc.Tstep
        + prj.lidproc.StorageExfil
        + prj.lidproc.StorageEvap
        + prj.lidproc.StorageDrain;
    prj.lidproc.StorageInflow = MIN(prj.lidproc.StorageInflow, maxRate);

    //... equate surface infil to storage inflow
    prj.lidproc.SurfaceInfil = prj.lidproc.StorageInflow;

    //... find surface outflow rate
    prj.lidproc.SurfaceOutflow = getSurfaceOutflowRate(prj, lidUnit, lidProc, surfaceDepth);

    // ... find net fluxes for each layer
    f[SURF as usize] = (prj.lidproc.SurfaceInflow - prj.lidproc.SurfaceEvap - prj.lidproc.StorageInflow - prj.lidproc.SurfaceOutflow)
        / lidProc.surface.voidFrac;
    f[STOR as usize] = (prj.lidproc.StorageInflow - prj.lidproc.StorageEvap - prj.lidproc.StorageExfil - prj.lidproc.StorageDrain)
        / lidProc.storage.voidFrac;
    f[SOIL as usize] = 0.0;
}

//=============================================================================

fn pavementFluxRates(prj: &mut Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc, x: &[f64], f: &mut [f64]) {
    //
    //  Purpose: computes flux rates for the layers of a porous pavement LID.
    //  Input:   x = vector of storage levels
    //  Output:  f = vector of flux rates
    //
    //... Moisture level variables
    let surfaceDepth: f64;
    let paveDepth: f64;
    let soilTheta: f64;
    let storageDepth: f64;

    //... Intermediate variables
    let pervFrac = 1.0 - lidProc.pavement.impervFrac;
    let storageInflow: f64; // inflow rate to storage layer (ft/s)
    let mut availVolume: f64;
    let mut maxRate: f64;

    //... LID layer properties
    let paveVoidFrac = lidProc.pavement.voidFrac * pervFrac;
    let paveThickness = lidProc.pavement.thickness;
    let soilThickness = lidProc.soil.thickness;
    let soilPorosity = lidProc.soil.porosity;
    let soilFieldCap = lidProc.soil.fieldCap;
    let soilWiltPoint = lidProc.soil.wiltPoint;
    let storageThickness = lidProc.storage.thickness;
    let storageVoidFrac = lidProc.storage.voidFrac;

    //... retrieve moisture levels from input vector
    surfaceDepth = x[SURF as usize];
    paveDepth = x[PAVE as usize];
    soilTheta = x[SOIL as usize];
    storageDepth = x[STOR as usize];

    //... convert moisture levels to volumes
    prj.lidproc.SurfaceVolume = surfaceDepth * lidProc.surface.voidFrac;
    prj.lidproc.PaveVolume = paveDepth * paveVoidFrac;
    prj.lidproc.SoilVolume = soilTheta * soilThickness;
    prj.lidproc.StorageVolume = storageDepth * storageVoidFrac;

    //... get ET rates
    availVolume = prj.lidproc.SoilVolume - soilWiltPoint * soilThickness;
    let SurfaceVolume = prj.lidproc.SurfaceVolume;
    let PaveVolume = prj.lidproc.PaveVolume;
    let StorageVolume = prj.lidproc.StorageVolume;
    getEvapRates(prj, SurfaceVolume, PaveVolume, availVolume, StorageVolume, pervFrac);

    //... no storage evap if soil or pavement layer saturated
    if paveDepth >= paveThickness || (soilThickness > 0.0 && soilTheta >= soilPorosity) {
        prj.lidproc.StorageEvap = 0.0;
    }

    //... find nominal rate of surface infiltration into pavement layer
    prj.lidproc.SurfaceInfil = prj.lidproc.SurfaceInflow + (prj.lidproc.SurfaceVolume / prj.lidproc.Tstep);

    //... find perc rate out of pavement layer
    prj.lidproc.PavePerc = getPavementPermRate(prj, lidUnit, lidProc) * pervFrac;

    //... surface infiltration can't exceed pavement permeability
    prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, prj.lidproc.PavePerc);

    //... limit pavement perc by available water
    maxRate = prj.lidproc.PaveVolume / prj.lidproc.Tstep + prj.lidproc.SurfaceInfil - prj.lidproc.PaveEvap;
    maxRate = MAX(maxRate, 0.0);
    prj.lidproc.PavePerc = MIN(prj.lidproc.PavePerc, maxRate);

    //... find soil layer perc rate
    if soilThickness > 0.0 {
        prj.lidproc.SoilPerc = getSoilPercRate(lidProc, soilTheta);
        availVolume = (soilTheta - soilFieldCap) * soilThickness;
        maxRate = MAX(availVolume, 0.0) / prj.lidproc.Tstep - prj.lidproc.SoilEvap;
        prj.lidproc.SoilPerc = MIN(prj.lidproc.SoilPerc, maxRate);
        prj.lidproc.SoilPerc = MAX(prj.lidproc.SoilPerc, 0.0);
    } else {
        prj.lidproc.SoilPerc = prj.lidproc.PavePerc;
    }

    //... exfiltration rate out of storage layer
    prj.lidproc.StorageExfil = getStorageExfilRate(prj, lidUnit, lidProc);

    //... underdrain flow rate
    prj.lidproc.StorageDrain = 0.0;
    if lidProc.drain.coeff > 0.0 {
        prj.lidproc.StorageDrain = getStorageDrainRate(prj, lidUnit, lidProc, storageDepth, soilTheta, paveDepth, surfaceDepth);
    }

    //... check for adjacent saturated layers

    //... no soil layer, pavement & storage layers are full
    if soilThickness == 0.0 && storageDepth >= storageThickness && paveDepth >= paveThickness {
        //... pavement outflow can't exceed storage outflow
        maxRate = prj.lidproc.StorageEvap + prj.lidproc.StorageDrain + prj.lidproc.StorageExfil;
        if prj.lidproc.PavePerc > maxRate {
            prj.lidproc.PavePerc = maxRate;
        }
        //... storage outflow can't exceed pavement outflow
        else {
            //... use up available exfiltration capacity first
            prj.lidproc.StorageExfil = MIN(prj.lidproc.StorageExfil, prj.lidproc.PavePerc);
            prj.lidproc.StorageDrain = prj.lidproc.PavePerc - prj.lidproc.StorageExfil;
        }

        //... set soil perc to pavement perc
        prj.lidproc.SoilPerc = prj.lidproc.PavePerc;

        //... limit surface infil. by pavement perc
        prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, prj.lidproc.PavePerc);
    }
    //... pavement, soil & storage layers are full
    else if soilThickness > 0 as f64
        && storageDepth >= storageThickness
        && soilTheta >= soilPorosity
        && paveDepth >= paveThickness
    {
        //... find which layer has limiting flux rate
        maxRate = prj.lidproc.StorageExfil + prj.lidproc.StorageDrain;
        if prj.lidproc.SoilPerc < maxRate {
            maxRate = prj.lidproc.SoilPerc;
        } else {
            maxRate = MIN(maxRate, prj.lidproc.PavePerc);
        }

        //... use up available storage exfiltration capacity first
        if maxRate > prj.lidproc.StorageExfil {
            prj.lidproc.StorageDrain = maxRate - prj.lidproc.StorageExfil;
        } else {
            prj.lidproc.StorageExfil = maxRate;
            prj.lidproc.StorageDrain = 0.0;
        }
        prj.lidproc.SoilPerc = maxRate;
        prj.lidproc.PavePerc = maxRate;

        //... limit surface infil. by pavement perc
        prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, prj.lidproc.PavePerc);
    }
    //... storage & soil layers are full
    else if soilThickness > 0.0 && storageDepth >= storageThickness && soilTheta >= soilPorosity {
        //... soil perc can't exceed storage outflow
        maxRate = prj.lidproc.StorageDrain + prj.lidproc.StorageExfil;
        if prj.lidproc.SoilPerc > maxRate {
            prj.lidproc.SoilPerc = maxRate;
        }
        //... storage outflow can't exceed soil perc
        else {
            //... use up available exfiltration capacity first
            prj.lidproc.StorageExfil = MIN(prj.lidproc.StorageExfil, prj.lidproc.SoilPerc);
            prj.lidproc.StorageDrain = prj.lidproc.SoilPerc - prj.lidproc.StorageExfil;
        }
        prj.lidproc.PavePerc = MIN(prj.lidproc.PavePerc, prj.lidproc.SoilPerc);

        //... limit surface infil. by available pavement volume
        availVolume = (paveThickness - paveDepth) * paveVoidFrac;
        maxRate = availVolume / prj.lidproc.Tstep + prj.lidproc.PavePerc + prj.lidproc.PaveEvap;
        prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, maxRate);
    }
    //... soil and pavement layers are full
    else if soilThickness > 0.0 && paveDepth >= paveThickness && soilTheta >= soilPorosity {
        prj.lidproc.PavePerc = MIN(prj.lidproc.PavePerc, prj.lidproc.SoilPerc);
        prj.lidproc.SoilPerc = prj.lidproc.PavePerc;
        prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, prj.lidproc.PavePerc);
        maxRate = MAX(prj.lidproc.StorageVolume / prj.lidproc.Tstep + prj.lidproc.SoilPerc - prj.lidproc.StorageEvap, 0.0);
        prj.lidproc.StorageExfil = MIN(prj.lidproc.StorageExfil, maxRate);
    }
    //... no adjoining layers are full
    else {
        //... limit storage exfiltration by available storage volume
        //    (if no soil layer, SoilPerc is same as PavePerc)
        maxRate = prj.lidproc.SoilPerc - prj.lidproc.StorageEvap + prj.lidproc.StorageVolume / prj.lidproc.Tstep;
        maxRate = MAX(0.0, maxRate);
        prj.lidproc.StorageExfil = MIN(prj.lidproc.StorageExfil, maxRate);

        //... limit underdrain flow by volume above drain offset
        if prj.lidproc.StorageDrain > 0.0 {
            maxRate = -prj.lidproc.StorageExfil - prj.lidproc.StorageEvap;
            if storageDepth >= storageThickness {
                maxRate += prj.lidproc.SoilPerc;
            }
            if lidProc.drain.offset <= storageDepth {
                maxRate += (storageDepth - lidProc.drain.offset) * storageVoidFrac / prj.lidproc.Tstep;
            }
            maxRate = MAX(maxRate, 0.0);
            prj.lidproc.StorageDrain = MIN(prj.lidproc.StorageDrain, maxRate);
        }

        //... limit soil & pavement outflow by unused storage volume
        availVolume = (storageThickness - storageDepth) * storageVoidFrac;
        maxRate = availVolume / prj.lidproc.Tstep + prj.lidproc.StorageEvap + prj.lidproc.StorageDrain + prj.lidproc.StorageExfil;
        maxRate = MAX(maxRate, 0.0);
        if soilThickness > 0.0 {
            prj.lidproc.SoilPerc = MIN(prj.lidproc.SoilPerc, maxRate);
            maxRate = (soilPorosity - soilTheta) * soilThickness / prj.lidproc.Tstep + prj.lidproc.SoilPerc;
        }
        prj.lidproc.PavePerc = MIN(prj.lidproc.PavePerc, maxRate);

        //... limit surface infil. by available pavement volume
        availVolume = (paveThickness - paveDepth) * paveVoidFrac;
        maxRate = availVolume / prj.lidproc.Tstep + prj.lidproc.PavePerc + prj.lidproc.PaveEvap;
        prj.lidproc.SurfaceInfil = MIN(prj.lidproc.SurfaceInfil, maxRate);
    }

    //... surface outflow
    prj.lidproc.SurfaceOutflow = getSurfaceOutflowRate(prj, lidUnit, lidProc, surfaceDepth);

    //... compute overall layer flux rates
    f[SURF as usize] = prj.lidproc.SurfaceInflow - prj.lidproc.SurfaceEvap - prj.lidproc.SurfaceInfil - prj.lidproc.SurfaceOutflow;
    f[PAVE as usize] = (prj.lidproc.SurfaceInfil - prj.lidproc.PaveEvap - prj.lidproc.PavePerc) / paveVoidFrac;
    if lidProc.soil.thickness > 0.0 {
        f[SOIL as usize] = (prj.lidproc.PavePerc - prj.lidproc.SoilEvap - prj.lidproc.SoilPerc) / soilThickness;
        storageInflow = prj.lidproc.SoilPerc;
    } else {
        f[SOIL as usize] = 0.0;
        storageInflow = prj.lidproc.PavePerc;
        prj.lidproc.SoilPerc = 0.0;
    }
    f[STOR as usize] = (storageInflow - prj.lidproc.StorageEvap - prj.lidproc.StorageExfil - prj.lidproc.StorageDrain) / storageVoidFrac;
}

//=============================================================================

fn swaleFluxRates(prj: &mut Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc, x: &[f64], f: &mut [f64]) {
    //
    //  Purpose: computes flux rates from a vegetative swale LID.
    //  Input:   x = vector of storage levels
    //  Output:  f = vector of flux rates
    //
    let mut depth: f64; // depth of surface water in swale (ft)
    let mut topWidth: f64; // top width of full swale (ft)
    let mut botWidth: f64; // bottom width of swale (ft)
    let length: f64; // length of swale (ft)
    let surfInflow: f64; // inflow rate to swale (cfs)
    let surfWidth: f64; // top width at current water depth (ft)
    let surfArea: f64; // surface area of current water depth (ft2)
    let mut flowArea: f64; // x-section flow area (ft2)
    let lidArea: f64; // surface area of full swale (ft2)
    let mut hydRadius: f64; // hydraulic radius for current depth (ft)
    let mut slope: f64; // slope of swale side wall (run/rise)
    let volume: f64; // swale volume at current water depth (ft3)
    let mut dVdT: f64; // change in volume w.r.t. time (cfs)
    let dStore: f64; // depression storage depth (ft)
    let xDepth: f64; // depth above depression storage (ft)

    //... retrieve state variable from work vector
    depth = x[SURF as usize];
    depth = MIN(depth, lidProc.surface.thickness);

    //... depression storage depth
    dStore = 0.0;

    //... get swale's bottom width
    //    (0.5 ft minimum to avoid numerical problems)
    slope = lidProc.surface.sideSlope;
    topWidth = lidUnit.fullWidth;
    topWidth = MAX(topWidth, 0.5);
    botWidth = topWidth - 2.0 * slope * lidProc.surface.thickness;
    if botWidth < 0.5 {
        botWidth = 0.5;
        slope = 0.5 * (topWidth - 0.5) / lidProc.surface.thickness;
    }

    //... swale's length
    lidArea = lidUnit.area;
    length = lidArea / topWidth;

    //... top width, surface area and flow area of current ponded depth
    surfWidth = botWidth + 2.0 * slope * depth;
    surfArea = length * surfWidth;
    flowArea = (depth * (botWidth + slope * depth)) * lidProc.surface.voidFrac;

    //... wet volume and effective depth
    volume = length * flowArea;

    //... surface inflow into swale (cfs)
    surfInflow = prj.lidproc.SurfaceInflow * lidArea;

    //... ET rate in cfs
    prj.lidproc.SurfaceEvap = prj.lidproc.EvapRate * surfArea;
    prj.lidproc.SurfaceEvap = MIN(prj.lidproc.SurfaceEvap, volume / prj.lidproc.Tstep);

    //... infiltration rate to native soil in cfs
    prj.lidproc.StorageExfil = prj.lidproc.SurfaceInfil * surfArea;

    //... no surface outflow if depth below depression storage
    xDepth = depth - dStore;
    if xDepth <= ZERO {
        prj.lidproc.SurfaceOutflow = 0.0;
    }
    //... otherwise compute a surface outflow
    else {
        //... modify flow area to remove depression storage,
        flowArea -= (dStore * (botWidth + slope * dStore)) * lidProc.surface.voidFrac;
        if flowArea < ZERO {
            prj.lidproc.SurfaceOutflow = 0.0;
        } else {
            //... compute hydraulic radius
            botWidth = botWidth + 2.0 * dStore * slope;
            hydRadius = botWidth + 2.0 * xDepth * (1.0 + slope * slope).sqrt();
            hydRadius = flowArea / hydRadius;

            //... use Manning Eqn. to find outflow rate in cfs
            prj.lidproc.SurfaceOutflow = lidProc.surface.alpha * flowArea * hydRadius.powf(2. / 3.);
        }
    }

    //... net flux rate (dV/dt) in cfs
    dVdT = surfInflow - prj.lidproc.SurfaceEvap - prj.lidproc.StorageExfil - prj.lidproc.SurfaceOutflow;

    //... when full, any net positive inflow becomes spillage
    if depth == lidProc.surface.thickness && dVdT > 0.0 {
        prj.lidproc.SurfaceOutflow += dVdT;
        dVdT = 0.0;
    }

    //... convert flux rates to ft/s
    prj.lidproc.SurfaceEvap /= lidArea;
    prj.lidproc.StorageExfil /= lidArea;
    prj.lidproc.SurfaceOutflow /= lidArea;
    f[SURF as usize] = dVdT / surfArea;
    f[SOIL as usize] = 0.0;
    f[STOR as usize] = 0.0;

    //... assign values to layer volumes
    prj.lidproc.SurfaceVolume = volume / lidArea;
    prj.lidproc.SoilVolume = 0.0;
    prj.lidproc.StorageVolume = 0.0;
}

//=============================================================================

fn barrelFluxRates(prj: &mut Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc, x: &[f64], f: &mut [f64]) {
    //
    //  Purpose: computes flux rates for a rain barrel LID.
    //  Input:   x = vector of storage levels
    //  Output:  f = vector of flux rates
    //
    let storageDepth = x[STOR as usize];
    let head: f64;
    let mut maxValue: f64;

    //... assign values to layer volumes
    prj.lidproc.SurfaceVolume = 0.0;
    prj.lidproc.SoilVolume = 0.0;
    prj.lidproc.StorageVolume = storageDepth;

    //... initialize flows
    prj.lidproc.SurfaceInfil = 0.0;
    prj.lidproc.SurfaceOutflow = 0.0;
    prj.lidproc.StorageDrain = 0.0;

    //... compute outflow if time since last rain exceeds drain delay
    //    (dryTime is updated in lid.evalLidUnit at each time step)
    if lidProc.drain.delay == 0.0 || lidUnit.dryTime >= lidProc.drain.delay {
        head = storageDepth - lidProc.drain.offset;
        if head > 0.0 {
            prj.lidproc.StorageDrain = getStorageDrainRate(prj, lidUnit, lidProc, storageDepth, 0.0, 0.0, 0.0);
            maxValue = head / prj.lidproc.Tstep;
            prj.lidproc.StorageDrain = MIN(prj.lidproc.StorageDrain, maxValue);
        }
    }

    //... limit inflow to available storage
    prj.lidproc.StorageInflow = prj.lidproc.SurfaceInflow;
    maxValue = (lidProc.storage.thickness - storageDepth) / prj.lidproc.Tstep + prj.lidproc.StorageDrain;
    prj.lidproc.StorageInflow = MIN(prj.lidproc.StorageInflow, maxValue);
    prj.lidproc.SurfaceInfil = prj.lidproc.StorageInflow;

    //... assign values to layer flux rates
    f[SURF as usize] = prj.lidproc.SurfaceInflow - prj.lidproc.StorageInflow;
    f[STOR as usize] = prj.lidproc.StorageInflow - prj.lidproc.StorageDrain;
    f[SOIL as usize] = 0.0;
}

//=============================================================================

fn getSurfaceOutflowRate(prj: &Project, lidUnit: &TLidUnit, lidProc: &TLidProc, depth: f64) -> f64 {
    //
    //  Purpose: computes outflow rate from a LID's surface layer.
    //  Input:   depth = depth of ponded water on surface layer (ft)
    //  Output:  returns outflow from surface layer (ft/s)
    //
    //  Note: this function should not be applied to swales or rain barrels.
    //
    let delta: f64;
    let mut outflow: f64;

    //... no outflow if ponded depth below storage depth
    delta = depth - lidProc.surface.thickness;
    if delta < 0.0 {
        return 0.0;
    }

    //... compute outflow from overland flow Manning equation
    outflow = lidProc.surface.alpha * delta.powf(5.0 / 3.0) * lidUnit.fullWidth / lidUnit.area;
    outflow = MIN(outflow, delta / prj.lidproc.Tstep);
    outflow
}

//=============================================================================

fn getPavementPermRate(prj: &Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc) -> f64 {
    //
    //  Purpose: computes reduced permeability of a pavement layer due to
    //           clogging.
    //  Input:   none
    //  Output:  returns the reduced permeability of the pavement layer (ft/s).
    //
    let mut permReduction = 0.0f64;
    let clogFactor = lidProc.pavement.clogFactor;
    let regenDays = lidProc.pavement.regenDays;

    // ... find permeability reduction due to clogging
    if clogFactor > 0.0 {
        // ... see if permeability regeneration has occurred
        //     (regeneration is assumed to reduce the total
        //      volumetric loading that the pavement has received)
        if regenDays > 0.0 {
            if prj.OldRunoffTime / 1000.0 / SECperDAY as f64 >= lidUnit.nextRegenDay {
                // ... reduce total volume treated by degree of regeneration
                lidUnit.volTreated *= 1.0 - lidProc.pavement.regenDegree;

                // ... update next day that regenration occurs
                lidUnit.nextRegenDay += regenDays;
            }
        }

        // ... find permeabiity reduction factor
        permReduction = lidUnit.volTreated / clogFactor;
        permReduction = MIN(permReduction, 1.0);
    }

    // ... return the effective pavement permeability
    lidProc.pavement.kSat * (1.0 - permReduction)
}

//=============================================================================

fn getSoilPercRate(lidProc: &TLidProc, theta: f64) -> f64 {
    //
    //  Purpose: computes percolation rate of water through a LID's soil layer.
    //  Input:   theta = moisture content (fraction)
    //  Output:  returns percolation rate within soil layer (ft/s)
    //
    let delta: f64; // moisture deficit

    // ... no percolation if soil moisture <= field capacity
    if theta <= lidProc.soil.fieldCap {
        return 0.0;
    }

    // ... perc rate = unsaturated hydraulic conductivity
    delta = lidProc.soil.porosity - theta;
    lidProc.soil.kSat * (-delta * lidProc.soil.kSlope).exp()
}

//=============================================================================

fn getStorageExfilRate(prj: &Project, lidUnit: &TLidUnit, lidProc: &TLidProc) -> f64 {
    //
    //  Purpose: computes exfiltration rate from storage zone into
    //           native soil beneath a LID.
    //  Input:   depth = depth of water storage zone (ft)
    //  Output:  returns infiltration rate (ft/s)
    //
    let infil: f64;
    let mut clogFactor: f64;

    if lidProc.storage.kSat == 0.0 {
        return 0.0;
    }
    if prj.lidproc.MaxNativeInfil == 0.0 {
        return 0.0;
    }

    //... reduction due to clogging
    clogFactor = lidProc.storage.clogFactor;
    if clogFactor > 0.0 {
        clogFactor = lidUnit.waterBalance.inflow / clogFactor;
        clogFactor = MIN(clogFactor, 1.0);
    }

    //... infiltration rate = storage Ksat reduced by any clogging
    infil = lidProc.storage.kSat * (1.0 - clogFactor);

    //... limit infiltration rate by any groundwater-imposed limit
    MIN(infil, prj.lidproc.MaxNativeInfil)
}

//=============================================================================

fn getStorageDrainRate(
    prj: &mut Project,
    lidUnit: &TLidUnit,
    lidProc: &TLidProc,
    storageDepth: f64,
    soilTheta: f64,
    paveDepth: f64,
    surfaceDepth: f64,
) -> f64 {
    //
    //  Purpose: computes underdrain flow rate in a LID's storage layer.
    //  Input:   storageDepth = depth of water in storage layer (ft)
    //           soilTheta    = moisture content of soil layer
    //           paveDepth    = effective depth of water in pavement layer (ft)
    //           surfaceDepth = depth of ponded water on surface layer (ft)
    //  Output:  returns flow in underdrain (ft/s)
    //
    //  Note:    drain eqn. is evaluated in user's units.
    //  Note:    head on drain is water depth in storage layer plus the
    //           layers above it (soil, pavement, and surface in that order)
    //           minus the drain outlet offset.
    let curve = lidProc.drain.qCurve;
    let mut head = storageDepth;
    let mut outflow = 0.0f64;
    let paveThickness = lidProc.pavement.thickness;
    let soilThickness = lidProc.soil.thickness;
    let soilPorosity = lidProc.soil.porosity;
    let soilFieldCap = lidProc.soil.fieldCap;
    let storageThickness = lidProc.storage.thickness;

    // --- storage layer is full
    if storageDepth >= storageThickness {
        // --- a soil layer exists
        if soilThickness > 0.0 {
            // --- increase head by fraction of soil layer saturated
            if soilTheta > soilFieldCap {
                head += (soilTheta - soilFieldCap) / (soilPorosity - soilFieldCap) * soilThickness;

                // --- soil layer is saturated, increase head by water
                //     depth in layer above it
                if soilTheta >= soilPorosity {
                    if paveThickness > 0.0 {
                        head += paveDepth;
                        if paveDepth >= paveThickness {
                            head += surfaceDepth;
                        }
                    } else {
                        head += surfaceDepth;
                    }
                }
            }
        }
        // --- no soil layer so increase head by water level in pavement
        //     layer and possibly surface layer
        else if paveThickness > 0.0 {
            head += paveDepth;
            if paveDepth >= paveThickness {
                head += surfaceDepth;
            }
        }
    }

    // --- no outflow if:
    //     a) no prior outflow and head below open threshold
    //     b) prior outflow and head below closed threshold
    if lidUnit.oldDrainFlow == 0.0 && head <= lidProc.drain.hOpen {
        return 0.0;
    }
    if lidUnit.oldDrainFlow > 0.0 && head <= lidProc.drain.hClose {
        return 0.0;
    }

    // --- make head relative to drain offset
    head -= lidProc.drain.offset;

    // --- compute drain outflow from underdrain flow equation in user units
    //     (head in inches or mm, flow rate in in/hr or mm/hr)
    if head > ZERO {
        // --- convert head to user units
        head *= UCF(prj, RAINDEPTH);

        // --- compute drain outflow in user units
        outflow = lidProc.drain.coeff * head.powf(lidProc.drain.expon);

        // --- apply user-supplied control curve to outflow
        if curve >= 0 {
            outflow *= table_lookup(&mut prj.Curve[curve as usize], head);
        }

        // --- convert outflow to ft/s
        outflow /= UCF(prj, RAINFALL);
    }
    outflow
}

//=============================================================================

fn getDrainMatOutflow(prj: &Project, lidUnit: &TLidUnit, lidProc: &TLidProc, depth: f64) -> f64 {
    //
    //  Purpose: computes flow rate through a green roof's drainage mat.
    //  Input:   depth = depth of water in drainage mat (ft)
    //  Output:  returns flow in drainage mat (ft/s)
    //
    //... default is to pass all inflow
    let mut result = prj.lidproc.SoilPerc;

    //... otherwise use Manning eqn. if its parameters were supplied
    if lidProc.drainMat.alpha > 0.0 {
        result = lidProc.drainMat.alpha * depth.powf(5.0 / 3.0) * lidUnit.fullWidth / lidUnit.area * lidProc.drainMat.voidFrac;
    }
    result
}

//=============================================================================

fn getEvapRates(prj: &mut Project, surfaceVol: f64, paveVol: f64, soilVol: f64, storageVol: f64, pervFrac: f64) {
    //
    //  Purpose: computes surface, pavement, soil, and storage evaporation rates.
    //  Input:   surfaceVol = volume/area of ponded water on surface layer (ft)
    //           paveVol    = volume/area of water in pavement pores (ft)
    //           soilVol    = volume/area of water in soil (or pavement) pores (ft)
    //           storageVol = volume/area of water in storage layer (ft)
    //           pervFrac   = fraction of surface layer that is pervious
    //  Output:  none
    //
    let s = &mut prj.lidproc;
    let mut availEvap: f64;

    //... surface evaporation flux
    availEvap = s.EvapRate;
    s.SurfaceEvap = MIN(availEvap, surfaceVol / s.Tstep);
    s.SurfaceEvap = MAX(0.0, s.SurfaceEvap);
    availEvap = MAX(0.0, availEvap - s.SurfaceEvap);
    availEvap *= pervFrac;

    //... no subsurface evap if water is infiltrating
    if s.SurfaceInfil > 0.0 {
        s.PaveEvap = 0.0;
        s.SoilEvap = 0.0;
        s.StorageEvap = 0.0;
    } else {
        //... pavement evaporation flux
        s.PaveEvap = MIN(availEvap, paveVol / s.Tstep);
        availEvap = MAX(0.0, availEvap - s.PaveEvap);

        //... soil evaporation flux
        s.SoilEvap = MIN(availEvap, soilVol / s.Tstep);
        availEvap = MAX(0.0, availEvap - s.SoilEvap);

        //... storage evaporation flux
        s.StorageEvap = MIN(availEvap, storageVol / s.Tstep);
    }
}

//=============================================================================

fn getSurfaceOverflowRate(prj: &Project, lidProc: &TLidProc, surfaceDepth: &mut f64) -> f64 {
    //
    //  Purpose: finds surface overflow rate from a LID unit.
    //  Input:   surfaceDepth = depth of water stored in surface layer (ft)
    //  Output:  returns the overflow rate (ft/s)
    //
    let delta = *surfaceDepth - lidProc.surface.thickness;
    if delta <= 0.0 {
        return 0.0;
    }
    *surfaceDepth = lidProc.surface.thickness;
    delta * lidProc.surface.voidFrac / prj.lidproc.Tstep
}

//=============================================================================

fn updateWaterBalance(
    prj: &Project,
    lidUnit: &mut TLidUnit,
    inflow: f64,
    evap: f64,
    infil: f64,
    surfFlow: f64,
    drainFlow: f64,
    storage: f64,
) {
    //
    //  Purpose: updates components of the water mass balance for a LID unit
    //           over the current time step.
    //  Input:   lidUnit   = a particular LID unit
    //           inflow    = runon + rainfall to the LID unit (ft/s)
    //           evap      = evaporation rate from the unit (ft/s)
    //           infil     = infiltration out the bottom of the unit (ft/s)
    //           surfFlow  = surface runoff from the unit (ft/s)
    //           drainFlow = underdrain flow from the unit
    //           storage   = volume of water stored in the unit (ft)
    //  Output:  none
    //
    let Tstep = prj.lidproc.Tstep;
    lidUnit.volTreated += inflow * Tstep;
    lidUnit.waterBalance.inflow += inflow * Tstep;
    lidUnit.waterBalance.evap += evap * Tstep;
    lidUnit.waterBalance.infil += infil * Tstep;
    lidUnit.waterBalance.surfFlow += surfFlow * Tstep;
    lidUnit.waterBalance.drainFlow += drainFlow * Tstep;
    lidUnit.waterBalance.finalVol = storage;
}

//=============================================================================

fn modpuls_solve(
    n: i32,
    x: &mut [f64],
    xOld: &mut [f64],
    xPrev: &mut [f64],
    xMin: &[f64],
    xMax: &[f64],
    xTol: &[f64],
    qOld: &[f64],
    q: &mut [f64],
    dt: f64,
    omega: f64,
    derivs: &mut dyn FnMut(&[f64], &mut [f64]),
) -> i32 {
    //
    //  Purpose: solves system of equations dx/dt = q(x) for x at end of time step
    //           dt using a modified Puls method.
    //  Input:   n = number of state variables
    //           x = vector of state variables
    //           xOld = state variable values at start of time step
    //           xPrev = state variable values from previous iteration
    //           xMin = lower limits on state variables
    //           xMax = upper limits on state variables
    //           xTol = convergence tolerances on state variables
    //           qOld = flux rates at start of time step
    //           q = flux rates at end of time step
    //           dt = time step (sec)
    //           omega = time weighting parameter (use 0 for Euler method
    //                   or 0.5 for modified Puls method)
    //           derivs = pointer to function that computes flux rates q as a
    //                    function of state variables x
    //  Output:  returns number of steps required for convergence (or 0 if
    //           process doesn't converge)
    //
    let mut i: i32;
    let mut canStop: i32;
    let mut steps: i32 = 1;
    let maxSteps: i32 = 20;

    //... initialize state variable values
    i = 0;
    while i < n {
        xOld[i as usize] = x[i as usize];
        xPrev[i as usize] = x[i as usize];
        i += 1;
    }

    //... repeat until convergence achieved
    while steps < maxSteps {
        //... compute flux rates for current state levels
        canStop = 1;
        derivs(&*x, &mut *q);

        //... update state levels based on current flux rates
        i = 0;
        while i < n {
            let iu = i as usize;
            x[iu] = xOld[iu] + (omega * qOld[iu] + (1.0 - omega) * q[iu]) * dt;
            x[iu] = MIN(x[iu], xMax[iu]);
            x[iu] = MAX(x[iu], xMin[iu]);

            if omega > 0.0 && (x[iu] - xPrev[iu]).abs() > xTol[iu] {
                canStop = 0;
            }
            xPrev[iu] = x[iu];
            i += 1;
        }

        //... return if process converges
        if canStop != 0 {
            return steps;
        }
        steps += 1;
    }

    //... no convergence so return 0
    0
}
