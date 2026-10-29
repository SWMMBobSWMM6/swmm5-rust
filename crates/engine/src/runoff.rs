//! runoff.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   runoff.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     10/17/22   (Build 5.2.2)
//   Author:   L. Rossman
//             M. Tryby
//
//   Runoff analysis functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Climate file now opened in climate.c module.
//   Build 5.1.008:
//   - Memory for runoff pollutant load now allocated and freed in this module.
//   - Runoff time step chosen so that simulation does not exceed total duration.
//   - State of LIDs considered when choosing wet or dry time step.
//   - More checks added to skip over subcatchments with zero area.
//   - Support added for sending outfall node discharge onto a subcatchment.
//   Build 5.1.011:
//   - Runoff wet time step kept aligned with reporting times.
//   - Prior runoff time step used to convert returned outfall volume to flow.
//   Build 5.1.012:
//   - Runoff wet time step no longer kept aligned with reporting times.
//   Build 5.1.014:
//   - Fixed street sweeping bug.
//   Build 5.2.0:
//   - Support added for saving rainfall amounts in previous 48 hours.
//   Build 5.2.2:
//   - Fixed possible use of canSweep in runoff_execute() with no assigned value.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
// Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of runoff.c.
#[derive(Default)]
pub struct RunoffState {
    IsRaining: i8,    // TRUE if precip. falls on study area
    HasRunoff: i8,    // TRUE if study area generates runoff
    HasSnow: i8,      // TRUE if any snow cover on study area
    Nsteps: i32,      // number of runoff time steps taken
    MaxSteps: i32,    // final number of runoff time steps
    MaxStepsPos: i64, // position in Runoff interface file
    //    where MaxSteps is saved

    //-------------------------------------------------------------------------
    //  Exportable variables
    //-------------------------------------------------------------------------
    pub HasWetLids: i8,        // TRUE if any LIDs are wet (used in lidproc.c)
    pub OutflowLoad: Vec<f64>, // exported pollutant mass load (used in surfqual.c)
}

//-----------------------------------------------------------------------------
//  Imported variables
//-----------------------------------------------------------------------------
// extern float* SubcatchResults;  -> prj.output.SubcatchResults (output.c)

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
// runoff_open     (called from swmm_start in swmm5.c)
// runoff_execute  (called from swmm_step in swmm5.c)
// runoff_close    (called from swmm_end in swmm5.c)

//-----------------------------------------------------------------------------
//  Helpers for C library behaviour
//-----------------------------------------------------------------------------

/// `fread(&v, sizeof(int), 1, f)`: bytes read land in `v` even when the
/// item is incomplete, as glibc copies them; returns the items read.
fn fread_i32(f: &mut CFile, v: &mut i32) -> usize {
    let mut b = v.to_le_bytes();
    let n = f.read_bytes(&mut b);
    *v = i32::from_le_bytes(b);
    n / 4
}

/// `fread(x, sizeof(float), n, f)` into `x[0..n]`; returns the items read.
fn fread_f32s(f: &mut CFile, x: &mut [f32], n: usize) -> usize {
    let mut b = vec![0u8; n * 4];
    for (k, v) in x[..n].iter().enumerate() {
        b[4 * k..4 * k + 4].copy_from_slice(&v.to_le_bytes());
    }
    let got = f.read_bytes(&mut b);
    for k in 0..n {
        x[k] = f32::from_le_bytes([b[4 * k], b[4 * k + 1], b[4 * k + 2], b[4 * k + 3]]);
    }
    got / 4
}

//=============================================================================

pub fn runoff_open(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns the global error code
//  Purpose: opens the runoff analyzer.
//
{
    prj.runoff.IsRaining = FALSE as i8;
    prj.runoff.HasRunoff = FALSE as i8;
    prj.runoff.HasSnow = FALSE as i8;
    prj.runoff.Nsteps = 0;

    // --- open the Ordinary Differential Equation solver
    if odesolve_open(MAXODES) == 0 {
        report_writeErrorMsg(prj, ERR_ODE_SOLVER, b"");
    }

    // --- allocate memory for pollutant runoff loads
    prj.runoff.OutflowLoad = Vec::new();
    if prj.Nobjects[POLLUT as usize] > 0 {
        prj.runoff.OutflowLoad = vec![0.0; prj.Nobjects[POLLUT as usize] as usize];
    }

    // --- see if a runoff interface file should be opened
    match prj.Frunoff.mode as i32 {
        USE_FILE => {
            prj.Frunoff.file = CFile::open(&prj.Frunoff.name, "r+b");
            if prj.Frunoff.file.is_none() {
                let name = prj.Frunoff.name;
                report_writeErrorMsg(prj, ERR_RUNOFF_FILE_OPEN, &name);
            } else {
                runoff_initFile(prj);
            }
        }
        SAVE_FILE => {
            prj.Frunoff.file = CFile::open(&prj.Frunoff.name, "w+b");
            if prj.Frunoff.file.is_none() {
                let name = prj.Frunoff.name;
                report_writeErrorMsg(prj, ERR_RUNOFF_FILE_OPEN, &name);
            } else {
                runoff_initFile(prj);
            }
        }
        _ => {}
    }
    prj.ErrorCode
}

//=============================================================================

pub fn runoff_close(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: closes the runoff analyzer.
//
{
    // --- close the ODE solver
    odesolve_close();

    // --- free memory for pollutant runoff loads
    prj.runoff.OutflowLoad = Vec::new();

    // --- close runoff interface file if in use
    if prj.Frunoff.file.is_some() {
        // --- write to file number of time steps simulated
        if prj.Frunoff.mode as i32 == SAVE_FILE {
            let pos = prj.runoff.MaxStepsPos;
            let nsteps = prj.runoff.Nsteps;
            prj.Frunoff.f().fseek_set(pos);
            prj.Frunoff.f().write_bytes(&nsteps.to_le_bytes());
        }
        // (C leaves the closed FILE* in Frunoff.file; nothing reads it after)
        prj.Frunoff.file = None; // fclose
    }

    // --- close climate file if in use
    if prj.Fclimate.file.is_some() {
        prj.Fclimate.file = None; // fclose
    }
}

//=============================================================================

pub fn runoff_execute(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: computes runoff from each subcatchment at current runoff time.
//
{
    let mut j: i32; // object index
    let day: i32; // day of calendar year
    let mut runoffStep: f64; // runoff time step (sec)
    let oldRunoffStep: f64; // previous runoff time step (sec)
    let mut runoff: f64; // subcatchment runoff (ft/sec)
    let currentDate: DateTime; // current date/time
    let mut canSweep: i8; // TRUE if street sweeping can occur

    if prj.ErrorCode != 0 {
        return;
    }

    // --- find previous runoff time step in sec
    oldRunoffStep = (prj.NewRunoffTime - prj.OldRunoffTime) / 1000.0;

    // --- convert elapsed runoff time in milliseconds to a calendar date
    currentDate = getDateTime(prj, prj.NewRunoffTime);

    // --- update climatological conditions
    climate_setState(prj, currentDate);

    // --- if no subcatchments then simply update runoff elapsed time
    if prj.Nobjects[SUBCATCH as usize] == 0 {
        prj.OldRunoffTime = prj.NewRunoffTime;
        prj.NewRunoffTime += (prj.DryStep) as f64 * 1000.0;
        prj.NewRunoffTime = MIN(prj.NewRunoffTime, prj.TotalDuration);
        return;
    }

    // --- update current rainfall at each raingage
    //     NOTE: must examine gages in sequential order due to possible
    //     presence of co-gages (gages that share same rain time series).
    prj.runoff.IsRaining = FALSE as i8;
    j = 0;
    while j < prj.Nobjects[GAGE as usize] {
        gage_setState(prj, j, currentDate);
        if prj.Gage[j as usize].rainfall > 0.0 {
            prj.runoff.IsRaining = TRUE as i8;
        }
        j += 1;
    }

    // --- read runoff results from interface file if applicable
    if prj.Frunoff.mode as i32 == USE_FILE {
        runoff_readFromFile(prj);
        return;
    }

    // --- see if street sweeping can occur on current date
    day = datetime_dayOfYear(currentDate);
    canSweep = FALSE as i8;
    if prj.SweepStart <= prj.SweepEnd {
        if day >= prj.SweepStart && day <= prj.SweepEnd {
            canSweep = TRUE as i8;
        }
    } else if day <= prj.SweepEnd || day >= prj.SweepStart {
        canSweep = TRUE as i8;
    }

    // --- get runoff time step (in seconds)
    runoffStep = runoff_getTimeStep(prj, currentDate);
    if runoffStep <= 0.0 {
        prj.ErrorCode = ERR_TIMESTEP;
        return;
    }

    // --- update runoff time clock (in milliseconds)
    prj.OldRunoffTime = prj.NewRunoffTime;
    prj.NewRunoffTime += 1000.0 * runoffStep;

    // --- adjust runoff step so that total duration not exceeded
    if prj.NewRunoffTime > prj.TotalDuration {
        runoffStep = (prj.TotalDuration - prj.OldRunoffTime) / 1000.0;
        prj.NewRunoffTime = prj.TotalDuration;
    }

    // --- update past n-hour rain totals
    j = 0;
    while j < prj.Nobjects[GAGE as usize] {
        gage_updatePastRain(prj, j, f64_to_i32(runoffStep));
        j += 1;
    }

    // --- update old state of each subcatchment,
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        subcatch_setOldState(prj, j);
        j += 1;
    }

    // --- determine any runon from drainage system outfall nodes
    if oldRunoffStep > 0.0 {
        runoff_getOutfallRunon(prj, oldRunoffStep);
    }

    // --- determine runon from upstream subcatchments, and implement snow removal
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        if prj.Subcatch[j as usize].area == 0.0 {
            j += 1;
            continue;
        }
        subcatch_getRunon(prj, j);
        if prj.IgnoreSnowmelt == 0 {
            snow_plowSnow(prj, j, runoffStep);
        }
        j += 1;
    }

    // --- determine runoff and pollutant buildup/washoff in each subcatchment
    prj.runoff.HasSnow = FALSE as i8;
    prj.runoff.HasRunoff = FALSE as i8;
    prj.runoff.HasWetLids = FALSE as i8;
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        // --- find total runoff rate (in ft/sec) over the subcatchment
        //     (the amount that actually leaves the subcatchment (in cfs)
        //     is also computed and is stored in Subcatch[j].newRunoff)
        if prj.Subcatch[j as usize].area == 0.0 {
            j += 1;
            continue;
        }
        runoff = subcatch_getRunoff(prj, j, runoffStep);

        // --- update state of study area surfaces
        if runoff > 0.0 {
            prj.runoff.HasRunoff = TRUE as i8;
        }
        if prj.Subcatch[j as usize].newSnowDepth > 0.0 {
            prj.runoff.HasSnow = TRUE as i8;
        }

        // --- skip pollutant buildup/washoff if quality ignored
        if prj.IgnoreQuality != 0 {
            j += 1;
            continue;
        }

        // --- add to pollutant buildup if runoff is negligible
        if runoff < MIN_RUNOFF {
            surfqual_getBuildup(prj, j, runoffStep);
        }

        // --- reduce buildup by street sweeping
        if canSweep != 0 && prj.Subcatch[j as usize].rainfall <= MIN_RUNOFF {
            surfqual_sweepBuildup(prj, j, currentDate);
        }

        // --- compute pollutant washoff
        surfqual_getWashoff(prj, j, runoff, runoffStep);
        j += 1;
    }

    // --- update tracking of system-wide max. runoff rate
    stats_updateMaxRunoff(prj);

    // --- save runoff results to interface file if one is used
    prj.runoff.Nsteps += 1;
    if prj.Frunoff.mode as i32 == SAVE_FILE {
        runoff_saveToFile(prj, runoffStep as f32);
    }

    // --- reset subcatchment runon to 0
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        prj.Subcatch[j as usize].runon = 0.0;
        j += 1;
    }
}

//=============================================================================

fn runoff_getTimeStep(prj: &mut Project, currentDate: DateTime) -> f64
//
//  Input:   currentDate = current simulation date/time
//  Output:  time step (sec)
//  Purpose: computes a time step to use for runoff calculations.
//
{
    let mut j: i32;
    let mut timeStep: i64;
    let mut maxStep: i64 = prj.DryStep as i64;

    // --- find shortest time until next evaporation or rainfall value
    //     (this represents the maximum possible time step)
    timeStep = datetime_timeDiff(climate_getNextEvapDate(prj), currentDate);
    if timeStep as f64 > 0.0 && timeStep < maxStep {
        maxStep = timeStep;
    }
    j = 0;
    while j < prj.Nobjects[GAGE as usize] {
        timeStep = datetime_timeDiff(gage_getNextRainDate(prj, j, currentDate), currentDate);
        if timeStep > 0 && timeStep < maxStep {
            maxStep = timeStep;
        }
        j += 1;
    }

    // --- determine whether wet or dry time step applies
    if prj.runoff.IsRaining != 0 || prj.runoff.HasSnow != 0 || prj.runoff.HasRunoff != 0 || prj.runoff.HasWetLids != 0 {
        timeStep = prj.WetStep as i64;
    } else {
        timeStep = prj.DryStep as i64;
    }

    // --- limit time step if necessary
    if timeStep > maxStep {
        timeStep = maxStep;
    }
    timeStep as f64
}

//=============================================================================

fn runoff_initFile(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: initializes a Runoff Interface file for saving results.
//
{
    let mut nSubcatch: i32;
    let mut nPollut: i32;
    let mut flowUnits: i32;
    let fileStamp: &[u8] = b"SWMM5-RUNOFF\0";
    let mut fStamp: [u8; 13] = *b"SWMM5-RUNOFF\0";

    prj.runoff.MaxSteps = 0;
    if prj.Frunoff.mode as i32 == SAVE_FILE {
        // --- write file stamp, # subcatchments & # pollutants to file
        nSubcatch = prj.Nobjects[SUBCATCH as usize];
        nPollut = prj.Nobjects[POLLUT as usize];
        flowUnits = prj.FlowUnits;
        let maxSteps = prj.runoff.MaxSteps;
        let f = prj.Frunoff.f();
        f.write_bytes(cio::cstr(fileStamp));
        f.write_bytes(&nSubcatch.to_le_bytes());
        f.write_bytes(&nPollut.to_le_bytes());
        f.write_bytes(&flowUnits.to_le_bytes());
        prj.runoff.MaxStepsPos = f.ftell();
        f.write_bytes(&maxSteps.to_le_bytes());
    }

    if prj.Frunoff.mode as i32 == USE_FILE {
        // --- check that interface file contains proper header records
        let n = cio::strlen(fileStamp);
        prj.Frunoff.f().read_bytes(&mut fStamp[..n]);
        if cio::strcmp(&fStamp, fileStamp) != 0 {
            report_writeErrorMsg(prj, ERR_RUNOFF_FILE_FORMAT, b"");
            return;
        }
        nSubcatch = -1;
        nPollut = -1;
        flowUnits = -1;
        let f = prj.Frunoff.f();
        fread_i32(f, &mut nSubcatch);
        fread_i32(f, &mut nPollut);
        fread_i32(f, &mut flowUnits);
        fread_i32(f, &mut prj.runoff.MaxSteps);
        if nSubcatch != prj.Nobjects[SUBCATCH as usize]
            || nPollut != prj.Nobjects[POLLUT as usize]
            || flowUnits != prj.FlowUnits
            || prj.runoff.MaxSteps <= 0
        {
            report_writeErrorMsg(prj, ERR_RUNOFF_FILE_FORMAT, b"");
        }
    }
}

//=============================================================================

fn runoff_saveToFile(prj: &mut Project, tStep: f32)
//
//  Input:   tStep = runoff time step (sec)
//  Output:  none
//  Purpose: saves current runoff results to Runoff Interface file.
//
{
    let mut j: i32;
    let n: i32 = MAX_SUBCATCH_RESULTS + prj.Nobjects[POLLUT as usize] - 1;

    prj.Frunoff.f().write_bytes(&tStep.to_le_bytes());
    // SubcatchResults (output.c) holds MAX_SUBCATCH_RESULTS-1+NumPolluts
    // floats, where NumPolluts is 0 when IgnoreQuality is set. C then
    // writes n floats from a shorter heap block (an out-of-bounds read of
    // whatever follows it on the heap); here those extra floats are zeros.
    let mut results = std::mem::take(&mut prj.output.SubcatchResults);
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        subcatch_getResults(prj, j, 1.0, &mut results);
        let mut bytes: Vec<u8> = Vec::with_capacity(n as usize * 4);
        for k in 0..n as usize {
            let v: f32 = results.get(k).copied().unwrap_or(0.0);
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        prj.Frunoff.f().write_bytes(&bytes);
        j += 1;
    }
    prj.output.SubcatchResults = results;
}

//=============================================================================

fn runoff_readFromFile(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: reads runoff results from Runoff Interface file for current time.
//
{
    let mut i: i32;
    let mut j: i32;
    let nResults: i32; // number of results per subcatch.
    let mut kount: i32; // count of items read from file
    // (C leaves tStep uninitialized; it keeps that value only if the
    //  fread below gets nothing, which is then reported as an error before
    //  tStep is used)
    let mut tStep: f32 = 0.0; // runoff time step (sec)

    // --- make sure not past end of file
    if prj.runoff.Nsteps > prj.runoff.MaxSteps {
        report_writeErrorMsg(prj, ERR_RUNOFF_FILE_END, b"");
        return;
    }

    // --- replace old state with current one for all subcatchments
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        subcatch_setOldState(prj, j);
        j += 1;
    }

    // --- read runoff time step
    kount = 0;
    {
        let mut b = tStep.to_le_bytes();
        let got = prj.Frunoff.f().read_bytes(&mut b);
        tStep = f32::from_le_bytes(b);
        kount += (got / 4) as i32;
    }

    // --- compute number of results saved for each subcatchment
    nResults = MAX_SUBCATCH_RESULTS + prj.Nobjects[POLLUT as usize] - 1;

    // SubcatchResults (output.c) has MAX_SUBCATCH_RESULTS-1+NumPolluts
    // floats, NumPolluts being 0 when IgnoreQuality is set; C then freads
    // nResults floats past its end (a heap overflow) and reads them back
    // below. The buffer is lengthened to nResults for this call so the
    // values read are the ones used, then restored to its length.
    let mut results = std::mem::take(&mut prj.output.SubcatchResults);
    let origLen = results.len();
    if results.len() < nResults as usize {
        results.resize(nResults as usize, 0.0);
    }

    // --- for each subcatchment
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        // --- read vector of saved results
        kount += fread_f32s(prj.Frunoff.f(), &mut results, nResults as usize) as i32;

        // --- extract hydrologic results, converting units where necessary
        //     (results were saved to file in user's units)
        let ju = j as usize;
        prj.Subcatch[ju].newSnowDepth = results[SUBCATCH_SNOWDEPTH as usize] as f64 / UCF(prj, RAINDEPTH);
        prj.Subcatch[ju].evapLoss = results[SUBCATCH_EVAP as usize] as f64 / UCF(prj, RAINFALL);
        prj.Subcatch[ju].infilLoss = results[SUBCATCH_INFIL as usize] as f64 / UCF(prj, RAINFALL);
        prj.Subcatch[ju].newRunoff = results[SUBCATCH_RUNOFF as usize] as f64 / UCF(prj, FLOW);
        if prj.Subcatch[ju].groundwater.is_some() {
            let newFlow = results[SUBCATCH_GW_FLOW as usize] as f64 / UCF(prj, FLOW);
            let ucfLength = UCF(prj, LENGTH);
            let aquifer = prj.Subcatch[ju].groundwater.as_ref().map(|gw| gw.aquifer).unwrap_or(0);
            let bottomElev = prj.Aquifer[aquifer as usize].bottomElev;
            let gw = prj.Subcatch[ju].groundwater.as_mut().expect("groundwater");
            gw.newFlow = newFlow;
            gw.lowerDepth = bottomElev - (results[SUBCATCH_GW_ELEV as usize] as f64 / ucfLength);
            gw.theta = results[SUBCATCH_SOIL_MOIST as usize] as f64;
        }

        // --- extract water quality results
        i = 0;
        while i < prj.Nobjects[POLLUT as usize] {
            prj.Subcatch[ju].newQual[i as usize] = results[(SUBCATCH_WASHOFF + i) as usize] as f64;
            i += 1;
        }
        j += 1;
    }
    results.truncate(origLen);
    prj.output.SubcatchResults = results;

    // --- report error if not enough values were read
    if kount < 1 + prj.Nobjects[SUBCATCH as usize] * nResults {
        report_writeErrorMsg(prj, ERR_RUNOFF_FILE_READ, b"");
        return;
    }

    // --- update runoff time clock
    prj.OldRunoffTime = prj.NewRunoffTime;
    prj.NewRunoffTime = prj.OldRunoffTime + (tStep) as f64 * 1000.0;
    prj.NewRunoffTime = MIN(prj.NewRunoffTime, prj.TotalDuration);
    prj.runoff.Nsteps += 1;
}

//=============================================================================

fn runoff_getOutfallRunon(prj: &mut Project, tStep: f64)
//
//  Input:   tStep = previous runoff time step (sec)
//  Output:  none
//  Purpose: adds flow and pollutant loads leaving drainage system outfalls
//           during the previous runoff time step to designated subcatchments.
//
{
    let mut i: i32;
    let mut k: i32;
    let mut p: i32;
    let mut w: f64;

    // --- examine each outfall node
    i = 0;
    while i < prj.Nnodes[OUTFALL as usize] {
        let iu = i as usize;
        // --- ignore node if outflow not re-routed onto a subcatchment
        k = prj.Outfall[iu].routeTo;
        if k < 0 {
            i += 1;
            continue;
        }
        if prj.Subcatch[k as usize].area == 0.0 {
            i += 1;
            continue;
        }

        // --- add outfall's flow to subcatchment as runon and re-set routed
        //     flow volume to 0
        let vRouted = prj.Outfall[iu].vRouted;
        subcatch_addRunonFlow(prj, k, vRouted / tStep);
        massbal_updateRunoffTotals(prj, RUNOFF_RUNON, vRouted);
        prj.Outfall[iu].vRouted = 0.0;

        // --- add outfall's pollutant load on to subcatchment's wet
        //     deposition load and re-set routed load to 0
        //     (Subcatch.newQual is being used as a temporary load accumulator)
        p = 0;
        while p < prj.Nobjects[POLLUT as usize] {
            w = prj.Outfall[iu].wRouted[p as usize] * LperFT3;
            let mcf = prj.Pollut[p as usize].mcf;
            massbal_updateLoadingTotals(prj, DEPOSITION_LOAD, p, w * mcf);
            prj.Subcatch[k as usize].newQual[p as usize] += w / tStep;
            prj.Outfall[iu].wRouted[p as usize] = 0.0;
            p += 1;
        }
        i += 1;
    }
}
