//! swmm5.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   swmm5.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//
//   This is the main module of the computational engine for Version 5 of
//   the U.S. Environmental Protection Agency's Storm Water Management Model
//   (SWMM). It contains functions that control the flow of computations.
//
//   This engine should be compiled into a shared object library whose API
//   functions are listed in swmm5.h.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Support added for the MinGW compiler.
//   - Reporting of project options moved to swmm_start.
//   - Hot start file now read before routing system opened.
//   - Final routing step adjusted so that total duration not exceeded.
//   Build 5.1.011:
//   - Made sure that MS exception handling only used with MS C compiler.
//   - Added name of module handling an exception to error report.
//   - Elapsed simulation time now saved to new global variable ElaspedTime.
//   - Added swmm_getError() function that retrieves error code and message.
//   - Changed WarningCode to Warnings (# warnings issued).
//   - Added swmm_getWarnings() function to retrieve value of Warnings.
//   - Fixed error code returned on swmm_xxx functions.
//   Build 5.1.012:
//   - #include <direct.h> only used when compiled for Windows.
//   Build 5.1.013:
//   - Support added for saving average results within a reporting period.
//   - SWMM engine now always compiled to a shared object library.
//   Build 5.1.015:
//   - Fixes bug in summary statistics when Report Start date > Start Date.
//   Build 5.2.0:
//   - Added additional API functions.
//   - Set max. number of open files to 8192.
//   - Changed getElapsedTime function to use report start as base date/time.
//   - Prevented possible infinite loop if swmm_step() called when ErrorCode > 0.
//   - Prevented early exit from swmm_end() when ErrorCode > 0.
//   - Support added for relative file names.
//-----------------------------------------------------------------------------
//
// The Windows-only parts (exception handling, _fpreset, _setmaxstdio,
// _tempnam, GetFullPathName) are not compiled on Linux and are left out.

use crate::prelude::*;
use std::io::Write;

/// max. number of exceptions handled (used only by the Windows build)
const MAX_EXCEPTIONS: i32 = 100;

// swmm5.h: object type codes used by the API
const swmm_GAGE: i32 = 0;
const swmm_SUBCATCH: i32 = 1;
const swmm_NODE: i32 = 2;
const swmm_LINK: i32 = 3;
const swmm_SYSTEM: i32 = 100;

// swmm5.h: swmm_GageProperty
const swmm_GAGE_RAINFALL: i32 = 100;

// swmm5.h: swmm_SubcatchProperty
const swmm_SUBCATCH_AREA: i32 = 200;
const swmm_SUBCATCH_RAINGAGE: i32 = 201;
const swmm_SUBCATCH_RAINFALL: i32 = 202;
const swmm_SUBCATCH_EVAP: i32 = 203;
const swmm_SUBCATCH_INFIL: i32 = 204;
const swmm_SUBCATCH_RUNOFF: i32 = 205;
const swmm_SUBCATCH_RPTFLAG: i32 = 206;

// swmm5.h: swmm_NodeProperty
const swmm_NODE_TYPE: i32 = 300;
const swmm_NODE_ELEV: i32 = 301;
const swmm_NODE_MAXDEPTH: i32 = 302;
const swmm_NODE_DEPTH: i32 = 303;
const swmm_NODE_HEAD: i32 = 304;
const swmm_NODE_VOLUME: i32 = 305;
const swmm_NODE_LATFLOW: i32 = 306;
const swmm_NODE_INFLOW: i32 = 307;
const swmm_NODE_OVERFLOW: i32 = 308;
const swmm_NODE_RPTFLAG: i32 = 309;

// swmm5.h: swmm_LinkProperty
const swmm_LINK_TYPE: i32 = 400;
const swmm_LINK_NODE1: i32 = 401;
const swmm_LINK_NODE2: i32 = 402;
const swmm_LINK_LENGTH: i32 = 403;
const swmm_LINK_SLOPE: i32 = 404;
const swmm_LINK_FULLDEPTH: i32 = 405;
const swmm_LINK_FULLFLOW: i32 = 406;
const swmm_LINK_SETTING: i32 = 407;
const swmm_LINK_TIMEOPEN: i32 = 408;
const swmm_LINK_TIMECLOSED: i32 = 409;
const swmm_LINK_FLOW: i32 = 410;
const swmm_LINK_DEPTH: i32 = 411;
const swmm_LINK_VELOCITY: i32 = 412;
const swmm_LINK_TOPWIDTH: i32 = 413;
const swmm_LINK_RPTFLAG: i32 = 414;

// swmm5.h: swmm_SystemProperty
const swmm_STARTDATE: i32 = 0;
const swmm_CURRENTDATE: i32 = 1;
const swmm_ELAPSEDTIME: i32 = 2;
const swmm_ROUTESTEP: i32 = 3;
const swmm_MAXROUTESTEP: i32 = 4;
const swmm_REPORTSTEP: i32 = 5;
const swmm_TOTALSTEPS: i32 = 6;
const swmm_NOREPORT: i32 = 7;
const swmm_FLOWUNITS: i32 = 8;

//-----------------------------------------------------------------------------
//  Unit conversion factors
//-----------------------------------------------------------------------------
pub const Ucf: [[f64; 2]; 10] = [
    //  US      SI
    [43200.0, 1097280.0],    // RAINFALL (in/hr, mm/hr --> ft/sec)
    [12.0, 304.8],           // RAINDEPTH (in, mm --> ft)
    [1036800.0, 26334720.0], // EVAPRATE (in/day, mm/day --> ft/sec)
    [1.0, 0.3048],           // LENGTH (ft, m --> ft)
    [2.2956e-5, 0.92903e-5], // LANDAREA (ac, ha --> ft2)
    [1.0, 0.02832],          // VOLUME (ft3, m3 --> ft3)
    [1.0, 1.608],            // WINDSPEED (mph, km/hr --> mph)
    [1.0, 1.8],              // TEMPERATURE (deg F, deg C --> deg F)
    [2.203e-6, 1.0e-6],      // MASS (lb, kg --> mg)
    [43560.0, 3048.0],       // GWFLOW (cfs/ac, cms/ha --> ft/sec)
];
/// Flow Conversion Factors:
pub const Qcf: [f64; 6] = [
    1.0, 448.831, 0.64632, // cfs, gpm, mgd --> cfs
    0.02832, 28.317, 2.4466, // cms, lps, mld --> cfs
];

/// The file-scope variables of swmm5.c.
#[derive(Default)]
pub struct Swmm5State {
    /// TRUE if a project has been opened
    IsOpenFlag: i32,
    /// TRUE if a simulation has been started
    IsStartedFlag: i32,
    /// TRUE if output to be saved to binary file
    SaveResultsFlag: i32,
    /// number of exceptions handled
    ExceptionCount: i32,
    /// TRUE if runoff is computed
    DoRunoff: i32,
    /// TRUE if flow routing is computed
    DoRouting: i32,
    /// duration of a set of routing steps (msecs)
    RoutingDuration: f64,
    /// state of the temporary file name generator (stands in for mkstemp's)
    TempNameSeed: u64,
}

impl Project {
    /// A new, empty project (all of C's globals zeroed, as at program start).
    pub fn new() -> Box<Project> {
        Box::default()
    }
}

//=============================================================================

pub fn swmm_run(prj: &mut Project, f1: &[u8], f2: &[u8], f3: &[u8]) -> i32
//
//  Input:   f1 = name of input file
//           f2 = name of report file
//           f3 = name of binary output file
//  Output:  returns error code
//  Purpose: runs a SWMM simulation.
//
{
    let mut newHour: i64;
    let mut oldHour: i64 = 0;
    let mut theDay: i64;
    let mut theHour: i64;
    let mut elapsedTime: f64 = 0.0;

    // --- initialize flags
    prj.swmm5.IsOpenFlag = FALSE;
    prj.swmm5.IsStartedFlag = FALSE;
    prj.swmm5.SaveResultsFlag = TRUE;

    // --- open the files & read input data
    prj.ErrorCode = 0;
    writecon(prj, b"\n o  Retrieving project data");
    swmm_open(prj, f1, f2, f3);

    // --- run the simulation if input data OK
    if prj.ErrorCode == 0 {
        // --- initialize values
        swmm_start(prj, TRUE);

        // --- execute each time step until elapsed time is re-set to 0
        if prj.ErrorCode == 0 {
            writecon(prj, b"\n o  Simulating day: 0     hour:  0");
            loop {
                swmm_step(prj, &mut elapsedTime);
                newHour = f64_to_i64(elapsedTime * 24.0);
                if newHour > oldHour {
                    theDay = f64_to_i64(elapsedTime);
                    theHour = f64_to_i64((elapsedTime - elapsedTime.floor()) * 24.0);
                    writecon(prj, b"\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08");
                    let s = cformat!(b"%-5ld hour: %-2ld", theDay, theHour);
                    snprintf_into(&mut prj.Msg, MAXMSG as usize, &s);
                    let msg = prj.Msg;
                    writecon(prj, &msg);
                    oldHour = newHour;
                }
                if !(elapsedTime > 0.0 && prj.ErrorCode == 0) {
                    break;
                }
            }
            writecon(
                prj,
                b"\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\
                  \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08",
            );
            writecon(prj, b"Simulation complete           ");
        }

        // --- clean up
        swmm_end(prj);
    }

    // --- report results
    if prj.ErrorCode == 0 && prj.Fout.mode as i32 == SCRATCH_FILE {
        writecon(prj, b"\n o  Writing output report");
        swmm_report(prj);
    }

    // --- close the system
    swmm_close(prj);
    prj.ErrorCode
}

//=============================================================================

pub fn swmm_open(prj: &mut Project, f1: &[u8], f2: &[u8], f3: &[u8]) -> i32
//
//  Input:   f1 = name of input file
//           f2 = name of report file
//           f3 = name of binary output file
//  Output:  returns error code
//  Purpose: opens a SWMM project.
//
{
    {
        // --- initialize error & warning codes
        datetime_setDateFormat(prj, M_D_Y);
        prj.ErrorCode = 0;
        prj.ErrorMsg[0] = 0;
        prj.Warnings = 0;
        prj.swmm5.IsOpenFlag = FALSE;
        prj.swmm5.IsStartedFlag = FALSE;
        prj.swmm5.ExceptionCount = 0;

        // --- open a SWMM project
        prj.InpDir[0] = 0;
        project_open(prj, f1, f2, f3);
        let size = prj.InpDir.len();
        getAbsolutePath(f1, &mut prj.InpDir, size);
        if prj.ErrorCode != 0 {
            return prj.ErrorCode;
        }
        prj.swmm5.IsOpenFlag = TRUE;
        report_writeLogo(prj);

        // --- retrieve project data from input file
        project_readInput(prj);
        if prj.ErrorCode != 0 {
            return prj.ErrorCode;
        }

        // --- write project title to report file & validate data
        report_writeTitle(prj);
        project_validate(prj);
    }
    prj.ErrorCode
}

//=============================================================================

pub fn swmm_start(prj: &mut Project, saveResults: i32) -> i32
//
//  Input:   saveResults = TRUE if simulation results saved to binary file
//  Output:  returns an error code
//  Purpose: starts a SWMM simulation.
//
{
    // --- check that a project is open & no run started
    if prj.ErrorCode != 0 {
        return prj.ErrorCode;
    }
    if prj.swmm5.IsOpenFlag == 0 {
        prj.ErrorCode = ERR_API_NOT_OPEN;
        return prj.ErrorCode;
    }
    if prj.swmm5.IsStartedFlag != 0 {
        prj.ErrorCode = ERR_API_NOT_ENDED;
        return prj.ErrorCode;
    }

    // --- write input summary & project options to report file if requested
    if prj.RptFlags.disabled == 0 {
        if prj.RptFlags.input != 0 {
            inputrpt_writeInput(prj);
        }
        report_writeOptions(prj);
    }

    // --- save saveResults flag to global variable
    prj.swmm5.SaveResultsFlag = saveResults;
    prj.swmm5.ExceptionCount = 0;

    {
        // --- initialize elapsed time in decimal days
        prj.ElapsedTime = 0.0;
        prj.swmm5.RoutingDuration = prj.TotalDuration;

        // --- initialize runoff, routing & reporting time (in milliseconds)
        prj.NewRunoffTime = 0.0;
        prj.NewRoutingTime = 0.0;
        prj.ReportTime = 1000.0 * prj.ReportStep as f64;
        prj.TotalStepCount = 0;
        prj.ReportStepCount = 0;
        prj.NonConvergeCount = 0;
        prj.swmm5.IsStartedFlag = TRUE;

        // --- initialize global continuity errors
        prj.RunoffError = 0.0;
        prj.GwaterError = 0.0;
        prj.FlowError = 0.0;
        prj.QualError = 0.0;

        // --- open rainfall processor (creates/opens a rainfall
        //     interface file and generates any RDII flows)
        if prj.IgnoreRainfall == 0 {
            rain_open(prj);
        }
        if prj.ErrorCode != 0 {
            return prj.ErrorCode;
        }

        // --- initialize state of each major system component
        project_init(prj);

        // --- see if runoff & routing needs to be computed
        if prj.Nobjects[SUBCATCH as usize] > 0 {
            prj.swmm5.DoRunoff = TRUE;
        } else {
            prj.swmm5.DoRunoff = FALSE;
        }
        if prj.Nobjects[NODE as usize] > 0 && prj.IgnoreRouting == 0 {
            prj.swmm5.DoRouting = TRUE;
        } else {
            prj.swmm5.DoRouting = FALSE;
        }

        // --- open binary output file
        output_open(prj);

        // --- open runoff processor
        if prj.swmm5.DoRunoff != 0 {
            runoff_open(prj);
        }

        // --- open & read hot start file if present
        if hotstart_open(prj) == 0 {
            return prj.ErrorCode;
        }

        // --- open routing processor
        if prj.swmm5.DoRouting != 0 {
            routing_open(prj);
        }

        // --- open mass balance and statistics processors
        massbal_open(prj);
        stats_open(prj);

        // --- write heading for control actions listing
        if prj.RptFlags.disabled == 0 && prj.RptFlags.controls != 0 {
            report_writeControlActionsHeading(prj);
        }
    }
    prj.ErrorCode
}

//=============================================================================

pub fn swmm_step(prj: &mut Project, elapsedTime: &mut f64) -> i32
//
//  Input:   elapsedTime = current elapsed time in decimal days
//  Output:  updated value of elapsedTime,
//           returns error code
//  Purpose: advances the simulation by one routing time step.
//
{
    // --- check that simulation can proceed
    *elapsedTime = 0.0;
    if prj.ErrorCode != 0 {
        return prj.ErrorCode;
    }
    if prj.swmm5.IsOpenFlag == 0 {
        prj.ErrorCode = ERR_API_NOT_OPEN;
        return prj.ErrorCode;
    }
    if prj.swmm5.IsStartedFlag == 0 {
        prj.ErrorCode = ERR_API_NOT_STARTED;
        return prj.ErrorCode;
    }

    {
        // --- if routing time has not exceeded total duration
        if prj.NewRoutingTime < prj.swmm5.RoutingDuration {
            // --- route flow & WQ through drainage system
            //     (runoff will be calculated as needed)
            //     (NewRoutingTime is updated)
            execRouting(prj);
        }

        // --- if saving results to the binary file
        if prj.swmm5.SaveResultsFlag != 0 {
            saveResults(prj);
        }

        // --- update elapsed time (days)
        if prj.NewRoutingTime < prj.swmm5.RoutingDuration {
            prj.ElapsedTime = prj.NewRoutingTime / MSECperDAY;
        }
        // --- otherwise end the simulation
        else {
            prj.ElapsedTime = 0.0;
        }
        *elapsedTime = prj.ElapsedTime;
    }
    prj.ErrorCode
}

//=============================================================================

pub fn swmm_stride(prj: &mut Project, strideStep: i32, elapsedTime: &mut f64) -> i32
//
//  Input:   strideStep = number of seconds to advance the simulation
//           elapsedTime = current elapsed time in decimal days
//  Output:  updated value of elapsedTime,
//           returns error code
//  Purpose: advances the simulation by a fixed number of seconds.
{
    let realRouteStep: f64 = prj.RouteStep;

    // --- check that simulation can proceed
    *elapsedTime = 0.0;
    if prj.ErrorCode != 0 {
        return prj.ErrorCode;
    }
    if prj.swmm5.IsOpenFlag == 0 {
        prj.ErrorCode = ERR_API_NOT_OPEN;
        return prj.ErrorCode;
    }
    if prj.swmm5.IsStartedFlag == 0 {
        prj.ErrorCode = ERR_API_NOT_STARTED;
        return prj.ErrorCode;
    }

    // --- modify total duration to be strideStep seconds after current time
    prj.swmm5.RoutingDuration = prj.NewRoutingTime + 1000.0 * strideStep as f64;
    prj.swmm5.RoutingDuration = MIN(prj.TotalDuration, prj.swmm5.RoutingDuration);

    // --- modify routing step to not exceed stride time step
    if (strideStep as f64) < prj.RouteStep {
        prj.RouteStep = strideStep as f64;
    }

    // --- step through simulation until next stride step is reached
    loop {
        swmm_step(prj, elapsedTime);
        if !(*elapsedTime > 0.0 && prj.ErrorCode == 0) {
            break;
        }
    }

    // --- restore original routing step and routing duration
    prj.RouteStep = realRouteStep;
    prj.swmm5.RoutingDuration = prj.TotalDuration;

    // --- restore actual elapsed time (days)
    if prj.NewRoutingTime < prj.TotalDuration {
        prj.ElapsedTime = prj.NewRoutingTime / MSECperDAY;
    } else {
        prj.ElapsedTime = 0.0;
    }
    *elapsedTime = prj.ElapsedTime;
    prj.ErrorCode
}

//=============================================================================

fn execRouting(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: routes flow & WQ through drainage system over a single time step.
//
{
    let mut nextRoutingTime: f64; // updated elapsed routing time (msec)
    let mut routingStep: f64; // routing time step (sec)

    {
        // --- determine when next routing time occurs
        prj.TotalStepCount += 1;
        if prj.swmm5.DoRouting == 0 {
            routingStep = MIN(prj.WetStep, prj.ReportStep) as f64;
        } else {
            let (rm, rs) = (prj.RouteModel, prj.RouteStep);
            routingStep = routing_getRoutingStep(prj, rm, rs);
        }
        if routingStep <= 0.0 {
            prj.ErrorCode = ERR_TIMESTEP;
            return;
        }
        nextRoutingTime = prj.NewRoutingTime + 1000.0 * routingStep;

        // --- adjust routing step so that total duration not exceeded
        if nextRoutingTime > prj.swmm5.RoutingDuration {
            routingStep = (prj.swmm5.RoutingDuration - prj.NewRoutingTime) / 1000.0;
            routingStep = MAX(routingStep, 1.0 / 1000.0);
            nextRoutingTime = prj.swmm5.RoutingDuration;
        }

        // --- compute runoff until next routing time reached or exceeded
        if prj.swmm5.DoRunoff != 0 {
            while prj.NewRunoffTime < nextRoutingTime {
                runoff_execute(prj);
                if prj.ErrorCode != 0 {
                    return;
                }
            }
        }
        // --- if no runoff analysis, update climate state (for evaporation)
        else {
            let d = getDateTime(prj, prj.NewRoutingTime);
            climate_setState(prj, d);
        }

        // --- route flows & pollutants through drainage system
        //     (while updating NewRoutingTime)
        if prj.swmm5.DoRouting != 0 {
            let rm = prj.RouteModel;
            routing_execute(prj, rm, routingStep);
        } else {
            prj.NewRoutingTime = nextRoutingTime;
        }
    }
}

//=============================================================================

fn saveResults(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: saves current results to binary output file.
{
    if prj.NewRoutingTime >= prj.ReportTime {
        // --- if user requested that average results be saved:
        if prj.RptFlags.averages != 0 {
            // --- include latest results in current averages
            //     if current time equals the reporting time
            if prj.NewRoutingTime == prj.ReportTime {
                output_updateAvgResults(prj);
            }

            // --- save current average results to binary file
            //     (which will re-set averages to 0)
            let rt = prj.ReportTime;
            output_saveResults(prj, rt);

            // --- if current time exceeds reporting period then
            //     start computing averages for next period
            if prj.NewRoutingTime > prj.ReportTime {
                output_updateAvgResults(prj);
            }
        }
        // --- otherwise save interpolated point results
        else {
            let rt = prj.ReportTime;
            output_saveResults(prj, rt);
        }

        // --- advance to next reporting period
        prj.ReportTime = prj.ReportTime + 1000.0 * prj.ReportStep as f64;
    }
    // --- not a reporting period so update average results if applicable
    else if prj.RptFlags.averages != 0 {
        output_updateAvgResults(prj);
    }
}

//=============================================================================

pub fn swmm_end(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  none
//  Purpose: ends a SWMM simulation.
//
{
    // --- check that project opened and run started
    if prj.swmm5.IsOpenFlag == 0 {
        prj.ErrorCode = ERR_API_NOT_OPEN;
        return prj.ErrorCode;
    }

    if prj.swmm5.IsStartedFlag != 0 {
        // --- write ending records to binary output file
        if prj.Fout.file.is_some() {
            output_end(prj);
        }

        // --- report mass balance results and system statistics
        if prj.ErrorCode == 0 && prj.RptFlags.disabled == 0 {
            massbal_report(prj);
            stats_report(prj);
        }

        // --- close all computing systems
        stats_close(prj);
        massbal_close(prj);
        if prj.IgnoreRainfall == 0 {
            rain_close(prj);
        }
        if prj.swmm5.DoRunoff != 0 {
            runoff_close(prj);
        }
        if prj.swmm5.DoRouting != 0 {
            let rm = prj.RouteModel;
            routing_close(prj, rm);
        }
        hotstart_close(prj);
        prj.swmm5.IsStartedFlag = FALSE;
    }
    prj.ErrorCode
}

//=============================================================================

pub fn swmm_report(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: writes simulation results to the report file.
//
{
    if prj.ErrorCode == 0 {
        report_writeReport(prj);
    }
    prj.ErrorCode
}

//=============================================================================

pub fn swmm_writeLine(prj: &mut Project, line: &[u8])
//
//  Input:   line = a character string
//  Output:  returns an error code
//  Purpose: writes a line of text to the report file.
//
{
    if prj.swmm5.IsOpenFlag != 0 {
        report_writeLine(prj, line);
    }
}

//=============================================================================

pub fn swmm_close(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: closes a SWMM project.
//
{
    if prj.Fout.file.is_some() {
        output_close(prj);
    }
    if prj.swmm5.IsOpenFlag != 0 {
        project_close(prj);
    }
    report_writeSysTime(prj);
    // (C's fclose leaves the FILE pointers dangling; here they become None)
    if prj.Finp.file.is_some() {
        prj.Finp.file = None;
    }
    if prj.Frpt.file.is_some() {
        prj.Frpt.file = None;
    }
    if prj.Fout.file.is_some() {
        prj.Fout.file = None;
        if prj.Fout.mode as i32 == SCRATCH_FILE {
            cio::remove(&prj.Fout.name);
        }
    }
    prj.swmm5.IsOpenFlag = FALSE;
    prj.swmm5.IsStartedFlag = FALSE;
    0
}

//=============================================================================

pub fn swmm_getMassBalErr(prj: &mut Project, runoffErr: &mut f32, flowErr: &mut f32, qualErr: &mut f32) -> i32
//
//  Input:   none
//  Output:  runoffErr = runoff mass balance error (percent)
//           flowErr   = flow routing mass balance error (percent)
//           qualErr   = quality routing mass balance error (percent)
//           returns an error code
//  Purpose: reports a simulation's mass balance errors.
//
{
    *runoffErr = 0.0;
    *flowErr = 0.0;
    *qualErr = 0.0;

    if prj.swmm5.IsOpenFlag != 0 && prj.swmm5.IsStartedFlag == 0 {
        *runoffErr = prj.RunoffError as f32;
        *flowErr = prj.FlowError as f32;
        *qualErr = prj.QualError as f32;
    }
    0
}

//=============================================================================

pub fn swmm_getVersion(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns SWMM engine version number
//  Purpose: retrieves version number of current SWMM engine which
//           uses a format of xyzzz where x = major version number,
//           y = minor version number, and zzz = build number.
//
//  NOTE: Each New Release should be updated in consts.h
{
    let _ = prj;
    VERSION
}

//=============================================================================

pub fn swmm_getWarnings(prj: &mut Project) -> i32
//
//  Input:  none
//  Output: returns number of warning messages issued.
//  Purpose: retrieves number of warning messages issued during an analysis.
{
    prj.Warnings
}

//=============================================================================

pub fn swmm_getError(prj: &mut Project, errMsg: &mut Vec<u8>, msgLen: i32) -> i32
//
//  Input:   errMsg = character array to hold error message text
//           msgLen = maximum size of errMsg
//  Output:  returns error message code number and text of error message.
//  Purpose: retrieves the code number and text of the error condition that
//           caused SWMM to abort its analysis.
//
//  (errMsg receives the message bytes, without a terminator.)
{
    // --- copy text of last error message into errMsg
    if prj.ErrorCode > 0 && cio::strlen(&prj.ErrorMsg) == 0 {
        let mut m: Vec<u8> = Vec::new();
        error_getMsg(prj.ErrorCode, &mut m);
        cio::strcpy(&mut prj.ErrorMsg, &m);
    }
    copy_out(errMsg, &prj.ErrorMsg, msgLen as usize);

    // --- remove leading line feed from errMsg
    if msgLen > 0 && errMsg.first().copied().unwrap_or(0) == b'\n' {
        errMsg[0] = b' ';
    }
    prj.ErrorCode
}

//=============================================================================

pub fn swmm_getCount(prj: &mut Project, objType: i32) -> i32
//
//  Input:   objType = a type of SWMM object
//  Output:  returns the number of objects;
//  Purpose: retrieves the number of objects of a specific type.
{
    if prj.swmm5.IsOpenFlag == 0 {
        return 0;
    }
    if objType < swmm_GAGE || objType > swmm_LINK {
        return 0;
    }
    prj.Nobjects[objType as usize]
}

//=============================================================================

pub fn swmm_getName(prj: &mut Project, objType: i32, index: i32, name: &mut Vec<u8>, size: i32)
//
//  Input:   objType = a type of SWMM object
//           index = the object's index in the array of objects
//           name = a character array
//           size = size of the name array
//  Output:  name = the object's ID name;
//  Purpose: retrieves the ID name of an object.
//
//  (name receives the ID bytes, without a terminator.)
{
    let mut idName: Option<&[u8]> = None;

    name.clear();
    if prj.swmm5.IsOpenFlag == 0 {
        return;
    }
    if objType < swmm_GAGE || objType > swmm_LINK {
        return;
    }
    if index < 0 || index >= prj.Nobjects[objType as usize] {
        return;
    }
    match objType {
        GAGE => idName = Some(&prj.Gage[index as usize].ID),
        SUBCATCH => idName = Some(&prj.Subcatch[index as usize].ID),
        NODE => idName = Some(&prj.Node[index as usize].ID),
        LINK => idName = Some(&prj.Link[index as usize].ID),
        _ => {}
    }
    if let Some(id) = idName {
        copy_out(name, id, size as usize);
    }
}

//=============================================================================

pub fn swmm_getIndex(prj: &mut Project, objType: i32, name: &[u8]) -> i32
//
//  Input:   objType = a type of SWMM object
//           name = the object's ID name
//  Output:  returns the object's position in the array of like objects;
//  Purpose: retrieves the index of a named object.
{
    if prj.swmm5.IsOpenFlag == 0 {
        return -1;
    }
    if objType < swmm_GAGE || objType > swmm_LINK {
        return -1;
    }
    project_findObject(prj, objType, name)
}

//=============================================================================

pub fn swmm_getValue(prj: &mut Project, property: i32, index: i32) -> f64
//
//  Input:   property = an object's property code
//           index = the object's index in the array of like objects
//
//  Output:  returns the property's current value
//  Purpose: retrieves the value of an object's property.
{
    if prj.swmm5.IsOpenFlag == 0 {
        return 0.0;
    }
    if property < 100 {
        return getSystemValue(prj, property);
    }
    if property < 200 {
        return getGageValue(prj, property, index);
    }
    if property < 300 {
        return getSubcatchValue(prj, property, index);
    }
    if property < 400 {
        return getNodeValue(prj, property, index);
    }
    if property < 500 {
        return getLinkValue(prj, property, index);
    }
    0.0
}

//=============================================================================

pub fn swmm_setValue(prj: &mut Project, property: i32, index: i32, value: f64)
//
//  Input:   property = an object's property code
//           index = the object's index in the array of like objects
//           value = the property's new value
//  Output:  none
//  Purpose: sets the value of an object's property.
{
    if prj.swmm5.IsOpenFlag == 0 {
        return;
    }
    match property {
        swmm_GAGE_RAINFALL => {
            if index < 0 || index >= prj.Nobjects[GAGE as usize] {
                return;
            }
            if value >= 0.0 {
                prj.Gage[index as usize].apiRainfall = value;
            }
        }
        swmm_SUBCATCH_RPTFLAG => {
            if prj.swmm5.IsStartedFlag == 0 && index >= 0 && index < prj.Nobjects[SUBCATCH as usize] {
                prj.Subcatch[index as usize].rptFlag = (value > 0.0) as i32;
            }
        }
        swmm_NODE_LATFLOW => {
            setNodeLatFlow(prj, index, value);
        }
        swmm_NODE_HEAD => {
            setOutfallStage(prj, index, value);
        }
        swmm_NODE_RPTFLAG => {
            if prj.swmm5.IsStartedFlag == 0 && index >= 0 && index < prj.Nobjects[NODE as usize] {
                prj.Node[index as usize].rptFlag = (value > 0.0) as i32;
            }
        }
        swmm_LINK_SETTING => {
            setLinkSetting(prj, index, value);
        }
        swmm_LINK_RPTFLAG => {
            if prj.swmm5.IsStartedFlag == 0 && index >= 0 && index < prj.Nobjects[LINK as usize] {
                prj.Link[index as usize].rptFlag = (value > 0.0) as i32;
            }
        }
        swmm_ROUTESTEP => {
            setRoutingStep(prj, value);
        }
        swmm_REPORTSTEP => {
            if prj.swmm5.IsStartedFlag == 0 && value > 0.0 {
                prj.ReportStep = f64_to_i32(value);
            }
        }
        swmm_NOREPORT => {
            if prj.swmm5.IsStartedFlag == 0 {
                prj.RptFlags.disabled = (value > 0.0) as i8;
            }
        }
        _ => {}
    }
}

//=============================================================================

pub fn swmm_getSavedValue(prj: &mut Project, property: i32, index: i32, period: i32) -> f64
//
//  Input:   property = an object's property code
//           index = the object's index in the array of like objects
//           period = a reporting time period (starting from 1)
//  Output:  returns the property's saved value
//  Purpose: retrieves an object's computed value at a specific reporting time period.
{
    if prj.swmm5.IsOpenFlag == 0 {
        return 0.0;
    }
    if prj.swmm5.IsStartedFlag != 0 {
        return 0.0;
    }
    if period < 1 || period as i64 > prj.Nperiods {
        return 0.0;
    }
    if property == swmm_CURRENTDATE {
        return getSavedDate(prj, period);
    }
    if property >= 200 && property < 300 {
        return getSavedSubcatchValue(prj, property, index, period);
    }
    if property < 400 {
        return getSavedNodeValue(prj, property, index, period);
    }
    if property < 500 {
        return getSavedLinkValue(prj, property, index, period);
    }
    0.0
}

//=============================================================================

pub fn swmm_decodeDate(
    prj: &mut Project,
    date: f64,
    year: &mut i32,
    month: &mut i32,
    day: &mut i32,
    hour: &mut i32,
    minute: &mut i32,
    second: &mut i32,
    dayOfWeek: &mut i32,
)
//
//  Input:  date = an encoded date in decimal days
//  Output: date's year, month of year, day of month, time of day (hour,
//           minute, second), and day of weeek
//  Purpose: retrieves the calendar date and clock time of an encoded date.
{
    let _ = prj;
    datetime_decodeDate(date, year, month, day);
    datetime_decodeTime(date, hour, minute, second);
    *dayOfWeek = datetime_dayOfWeek(date);
}

//=============================================================================
//   Object property getters and setters
//=============================================================================

fn getGageValue(prj: &mut Project, property: i32, index: i32) -> f64
//
//  Input:   property = a rain gage property code
//           index = the index of a rain gage
//  Output:  returns current property value
//  Purpose: retrieves current value of a rain gage property.
{
    if index < 0 || index >= prj.Nobjects[GAGE as usize] {
        return 0.0;
    }
    if property == swmm_GAGE_RAINFALL {
        return prj.Gage[index as usize].reportRainfall;
    }
    0.0
}

//=============================================================================

fn getSubcatchValue(prj: &mut Project, property: i32, index: i32) -> f64
//
//  Input:   property = a subcatchment property code
//           index = the index of a subcatchment
//  Output:  returns current property value
//  Purpose: retrieves current value of a subcatchment's property.
{
    if index < 0 || index >= prj.Nobjects[SUBCATCH as usize] {
        return 0.0;
    }
    let subcatch = &prj.Subcatch[index as usize];
    match property {
        swmm_SUBCATCH_AREA => subcatch.area * UCF(prj, LANDAREA),
        swmm_SUBCATCH_RAINGAGE => subcatch.gage as f64,
        swmm_SUBCATCH_RAINFALL => {
            if subcatch.gage >= 0 {
                prj.Gage[subcatch.gage as usize].reportRainfall
            } else {
                0.0
            }
        }
        swmm_SUBCATCH_EVAP => subcatch.evapLoss * UCF(prj, EVAPRATE),
        swmm_SUBCATCH_INFIL => subcatch.infilLoss * UCF(prj, RAINFALL),
        swmm_SUBCATCH_RUNOFF => subcatch.newRunoff * UCF(prj, FLOW),
        swmm_SUBCATCH_RPTFLAG => (subcatch.rptFlag > 0) as i32 as f64,
        _ => 0.0,
    }
}

//=============================================================================

fn getNodeValue(prj: &mut Project, property: i32, index: i32) -> f64
//
//  Input:   property = a node property code
//           index = the index of a node
//  Output:  returns current property value
//  Purpose: retrieves current value of a node's property.
{
    if index < 0 || index >= prj.Nobjects[NODE as usize] {
        return 0.0;
    }
    let node = &prj.Node[index as usize];
    match property {
        swmm_NODE_TYPE => node.type_ as f64,
        swmm_NODE_ELEV => node.invertElev * UCF(prj, LENGTH),
        swmm_NODE_MAXDEPTH => node.fullDepth * UCF(prj, LENGTH),
        swmm_NODE_DEPTH => node.newDepth * UCF(prj, LENGTH),
        swmm_NODE_HEAD => (node.newDepth + node.invertElev) * UCF(prj, LENGTH),
        swmm_NODE_VOLUME => node.newVolume * UCF(prj, VOLUME),
        swmm_NODE_LATFLOW => node.newLatFlow * UCF(prj, FLOW),
        swmm_NODE_INFLOW => node.inflow * UCF(prj, FLOW),
        swmm_NODE_OVERFLOW => node.overflow * UCF(prj, FLOW),
        swmm_NODE_RPTFLAG => (node.rptFlag > 0) as i32 as f64,
        _ => 0.0,
    }
}

//=============================================================================

fn getLinkValue(prj: &mut Project, property: i32, index: i32) -> f64
//
//  Input:   property = a link property code
//           index = the index of a link
//  Output:  returns current property value
//  Purpose: retrieves current value of a link's property.
{
    if index < 0 || index >= prj.Nobjects[LINK as usize] {
        return 0.0;
    }
    let link = &prj.Link[index as usize];
    match property {
        swmm_LINK_TYPE => link.type_ as f64,
        swmm_LINK_NODE1 => link.node1 as f64,
        swmm_LINK_NODE2 => link.node2 as f64,
        swmm_LINK_LENGTH => {
            if link.type_ == CONDUIT {
                prj.Conduit[link.subIndex as usize].length * UCF(prj, LENGTH)
            } else {
                0.0
            }
        }
        swmm_LINK_SLOPE => {
            if link.type_ == CONDUIT {
                prj.Conduit[link.subIndex as usize].slope
            } else {
                0.0
            }
        }
        swmm_LINK_FULLDEPTH => link.xsect.yFull * UCF(prj, LENGTH),
        swmm_LINK_FULLFLOW => link.qFull * UCF(prj, FLOW),
        swmm_LINK_FLOW => link.newFlow * UCF(prj, FLOW) * link.direction as f64,
        swmm_LINK_VELOCITY => {
            link_getVelocity(prj, index, link.newFlow.abs(), link.newDepth) * UCF(prj, LENGTH)
        }
        swmm_LINK_DEPTH => link.newDepth * UCF(prj, LENGTH),
        swmm_LINK_TOPWIDTH => {
            if link.type_ == CONDUIT {
                let xs = link.xsect;
                xsect_getWofY(prj, &xs, link.newDepth) * UCF(prj, LENGTH)
            } else {
                0.0
            }
        }
        swmm_LINK_SETTING => link.setting,
        swmm_LINK_TIMEOPEN => {
            if link.setting > 0.0 {
                (getDateTime(prj, prj.NewRoutingTime) - link.timeLastSet) * 24.0
            } else {
                0.0
            }
        }
        swmm_LINK_TIMECLOSED => {
            if link.setting == 0.0 {
                (getDateTime(prj, prj.NewRoutingTime) - link.timeLastSet) * 24.0
            } else {
                0.0
            }
        }
        swmm_LINK_RPTFLAG => (link.rptFlag > 0) as i32 as f64,
        _ => 0.0,
    }
}

//=============================================================================

fn getSystemValue(prj: &mut Project, property: i32) -> f64
//
//  Input:   property = a system property code
//  Output:  returns current property value
//  Purpose: retrieves current value of a system property.
{
    match property {
        swmm_STARTDATE => prj.StartDateTime,
        swmm_CURRENTDATE => prj.StartDateTime + prj.ElapsedTime,
        swmm_ELAPSEDTIME => prj.ElapsedTime,
        swmm_ROUTESTEP => prj.RouteStep,
        swmm_MAXROUTESTEP => getMaxRouteStep(prj),
        swmm_REPORTSTEP => prj.ReportStep as f64,
        swmm_TOTALSTEPS => prj.Nperiods as f64,
        swmm_NOREPORT => prj.RptFlags.disabled as f64,
        swmm_FLOWUNITS => prj.FlowUnits as f64,
        _ => 0.0,
    }
}

//=============================================================================

fn setNodeLatFlow(prj: &mut Project, index: i32, value: f64)
//
//  Input:   index = the index of a node
//           value = the node's external inflow value
//  Output:  none
//  Purpose: sets the value of a node's external inflow.
{
    if index < 0 || index >= prj.Nobjects[NODE as usize] {
        return;
    }
    prj.Node[index as usize].apiExtInflow = value / UCF(prj, FLOW);
}

//=============================================================================

fn setOutfallStage(prj: &mut Project, index: i32, value: f64)
//
//  Input:   index = the index of an outfall node
//           value = the outfall's fixed stage elevation
//  Output:  none
//  Purpose: sets the value of an outfall node's fixed stage.
{
    if index < 0 || index >= prj.Nobjects[NODE as usize] {
        return;
    }
    let node = &prj.Node[index as usize];
    if node.type_ != OUTFALL {
        return;
    }
    let k = node.subIndex as usize;
    prj.Outfall[k].fixedStage = value / UCF(prj, LENGTH);
    prj.Outfall[k].type_ = FIXED_OUTFALL;
}

//=============================================================================

fn setLinkSetting(prj: &mut Project, index: i32, value: f64)
//
//  Input:   index = the index of a link
//           value = the link's new setting
//  Output:  node
//  Purpose: sets the value of a link's setting.
{
    let mut value = value;
    if index < 0 || index >= prj.Nobjects[LINK as usize] {
        return;
    }
    let StartDateTime = prj.StartDateTime;
    let ElapsedTime = prj.ElapsedTime;
    let link = &mut prj.Link[index as usize];
    if value < 0.0 || link.type_ == CONDUIT {
        return;
    }
    if link.type_ != PUMP && value > 1.0 {
        value = 1.0;
    }
    if link.targetSetting == value {
        return;
    }
    link.targetSetting = value;
    if link.targetSetting * link.setting == 0.0 {
        link.timeLastSet = StartDateTime + ElapsedTime;
    }
    link_setSetting(prj, index, 0.0);
}

//=============================================================================

fn getSavedDate(prj: &mut Project, period: i32) -> f64
//
//  Input:   period = a reporting period (starting at 1)
//  Output:  returns the date/time of the reporting period in decimal days
//  Purpose: retrieves the date/time of a reporting period.
{
    let mut days: f64 = 0.0;
    output_readDateTime(prj, period as i64, &mut days);
    days
}

//=============================================================================

fn getSavedSubcatchValue(prj: &mut Project, property: i32, index: i32, period: i32) -> f64
//
//  Input:   property = index of a computed property
//           index = index of a subcatchment
//           period = a reporting period (starting at 1)
//  Output:  returns the property's value at the recording period
//  Purpose: retrieves the computed value of a subcatchment property at a
//           specific reporting period.
{
    // --- SubcatchResults array is defined in output.c and contains
    //     computed results in user's units

    // --- order in which subcatchment was saved to output results file
    let outIndex = prj.Subcatch[index as usize].rptFlag - 1;
    if outIndex < 0 {
        return 0.0;
    }

    output_readSubcatchResults(prj, period as i64, outIndex);
    let r = &prj.output.SubcatchResults;
    match property {
        swmm_SUBCATCH_RAINFALL => r[SUBCATCH_RAINFALL as usize] as f64,
        swmm_SUBCATCH_EVAP => r[SUBCATCH_EVAP as usize] as f64,
        swmm_SUBCATCH_INFIL => r[SUBCATCH_INFIL as usize] as f64,
        swmm_SUBCATCH_RUNOFF => r[SUBCATCH_RUNOFF as usize] as f64,
        _ => 0.0,
    }
}

//=============================================================================

fn getSavedNodeValue(prj: &mut Project, property: i32, index: i32, period: i32) -> f64
//
//  Input:   property = index of a computed property
//           index = index of a node
//           period = a reporting period (starting at 1)
//  Output:  returns the property's value at the recording period
//  Purpose: retrieves the computed value of a node property at a
//           specific reporting period.
{
    // --- NodeResults array is defined in output.c and contains
    //     computed results in user's units

    // --- order in which node was saved to output results file
    let outIndex = prj.Node[index as usize].rptFlag - 1;
    if outIndex < 0 {
        return 0.0;
    }

    output_readNodeResults(prj, period as i64, outIndex);
    let r = &prj.output.NodeResults;
    match property {
        swmm_NODE_DEPTH => r[NODE_DEPTH as usize] as f64,
        swmm_NODE_HEAD => r[NODE_HEAD as usize] as f64,
        swmm_NODE_VOLUME => r[NODE_VOLUME as usize] as f64,
        swmm_NODE_LATFLOW => r[NODE_LATFLOW as usize] as f64,
        swmm_NODE_INFLOW => r[NODE_INFLOW as usize] as f64,
        swmm_NODE_OVERFLOW => r[NODE_OVERFLOW as usize] as f64,
        _ => 0.0,
    }
}

//=============================================================================

fn getSavedLinkValue(prj: &mut Project, property: i32, index: i32, period: i32) -> f64
//
//  Input:   property = index of a computed property
//           index = index of a link
//           period = a reporting period (starting at 1)
//  Output:  returns the property's value at the recording period
//  Purpose: retrieves the computed value of a link property at a
//           specific reporting period.
{
    let y: f64;
    let w: f64;

    // --- LinkResults array is defined in output.c and contains
    //     computed results in user's units

    // --- order in which link was saved to output results file
    let outIndex = prj.Link[index as usize].rptFlag - 1;
    if outIndex < 0 {
        return 0.0;
    }

    output_readLinkResults(prj, period as i64, outIndex);
    match property {
        swmm_LINK_FLOW => prj.output.LinkResults[LINK_FLOW as usize] as f64,
        swmm_LINK_DEPTH => prj.output.LinkResults[LINK_DEPTH as usize] as f64,
        swmm_LINK_VELOCITY => prj.output.LinkResults[LINK_VELOCITY as usize] as f64,
        swmm_LINK_TOPWIDTH => {
            y = prj.output.LinkResults[LINK_DEPTH as usize] as f64 / UCF(prj, LENGTH);
            let xs = prj.Link[index as usize].xsect;
            w = xsect_getWofY(prj, &xs, y);
            w * UCF(prj, LENGTH)
        }
        swmm_LINK_SETTING => prj.output.LinkResults[LINK_CAPACITY as usize] as f64,
        _ => 0.0,
    }
}

//=============================================================================

fn getMaxRouteStep(prj: &mut Project) -> f64 {
    let tmpCourantFactor: f64 = prj.CourantFactor;
    let mut result: f64 = prj.RouteStep;

    if prj.swmm5.IsStartedFlag == 0 || prj.RouteModel != DW {
        return result;
    }
    prj.CourantFactor = 1.0;
    let (rm, ms) = (prj.RouteModel, prj.MinRouteStep);
    result = routing_getRoutingStep(prj, rm, ms);
    prj.CourantFactor = tmpCourantFactor;
    result
}

//=============================================================================

fn setRoutingStep(prj: &mut Project, value: f64)
//
//  Input:   value = a routing time step (in decimal seconds)
//  Output:  none
//  Purpose: sets the value of the current flow routing time step.
{
    let mut value = value;
    if value <= 0.0 {
        return;
    }
    if value <= prj.MinRouteStep {
        value = prj.MinRouteStep;
    }
    prj.CourantFactor = 0.0;
    prj.RouteStep = value;
}

//=============================================================================
//   General purpose functions
//=============================================================================

pub fn UCF(prj: &Project, u: i32) -> f64
//
//  Input:   u = integer code of quantity being converted
//  Output:  returns a units conversion factor
//  Purpose: computes a conversion factor from SWMM's internal
//           units to user's units
//
{
    if u < FLOW {
        Ucf[u as usize][prj.UnitSystem as usize]
    } else {
        Qcf[prj.FlowUnits as usize]
    }
}

//=============================================================================

pub fn sstrncpy(dest: &mut [u8], src: &[u8], n: usize) -> usize
//
//  Input:   dest = string to be copied to
//           src = string to be copied from
//           n = number of bytes to copy
//  Output:  returns the size of dest
//  Purpose: better version of standard strncpy function
//
//  (A byte past the end of src reads as the terminator. C would overrun a
//  too-small dest; here copying stops one byte short of dest's end.)
{
    let mut offset: usize = 0;
    if n > 0 {
        while src.get(offset).copied().unwrap_or(0) != 0 {
            if offset == n {
                break;
            }
            if offset + 1 >= dest.len() {
                break;
            }
            dest[offset] = src[offset];
            offset += 1;
        }
    }
    if offset < dest.len() {
        dest[offset] = 0;
    }
    cio::strlen(dest)
}

//=============================================================================

pub fn sstrcat(dest: &mut [u8], src: &[u8], size: usize) -> usize
//
//  Input:   dest = string to be appended
//           src = string to append to dest
//           size = allocated size of dest (including nul terminator)
//  Output:  returns new size of dest
//  Purpose: safe version of standard strcat function
//
//  (C would overrun a too-small dest; here copying stops at dest's end.)
{
    let dest_len: usize;
    let _src_len: usize;
    let mut offset: usize;
    let mut src_index: usize;

    // obtain initial sizes
    dest_len = cio::strlen(dest);
    _src_len = cio::strlen(src);

    // get the end of dest
    offset = dest_len;

    // append src
    src_index = 0;
    while src.get(src_index).copied().unwrap_or(0) != 0 {
        if offset + 1 >= dest.len() {
            break;
        }
        dest[offset] = src[src_index];
        offset += 1;
        src_index += 1;
        // don't copy more than size - dest_len - 1 characters
        if offset == size.wrapping_sub(1) {
            break;
        }
    }
    if offset < dest.len() {
        dest[offset] = 0;
    }
    cio::strlen(dest)
}

//=============================================================================

pub fn strcomp(s1: &[u8], s2: &[u8]) -> i32
//
//  Input:   s1 = a character string
//           s2 = a character string
//  Output:  returns 1 if s1 is same as s2, 0 otherwise
//  Purpose: does a case insensitive comparison of two strings.
//
//  (C reads s1[1] and s2[1] when both strings are empty, one byte past
//  their terminators; a byte past the end of a slice reads as 0 here.)
{
    let at = |s: &[u8], i: usize| s.get(i).copied().unwrap_or(0);
    let mut i: usize = 0;
    while UCHAR(at(s1, i)) == UCHAR(at(s2, i)) {
        if at(s1, i + 1) == 0 && at(s2, i + 1) == 0 {
            return 1;
        }
        i += 1;
    }
    0
}

//=============================================================================

pub fn getTempFileName(prj: &mut Project, fname: &mut [u8])
//
//  Input:   fname = file name string (with max size of MAXFNAME)
//  Output:  returns pointer to file name
//  Purpose: creates a temporary file name with path prepended to it.
//
//  (C's Linux branch calls mkstemp("swmmXXXXXX"), creating the file in the
//  current directory. As the integrator asked, the file is created in the
//  user's TempDir if given (and creatable), as the Windows branch does, or
//  else in the system's temporary directory. The name never reaches the
//  .rpt or .out file.)
{
    let mut dir: Option<Vec<u8>> = None;

    // --- set dir to user's choice of a temporary directory
    if cio::strlen(&prj.TempDir) > 0 {
        if let Some(p) = cio::path_of(&prj.TempDir) {
            // (as the Windows branch: _mkdir(TempDir) == 0 || errno == EEXIST)
            match std::fs::create_dir(&p) {
                Ok(()) => dir = Some(cio::to_vec(&prj.TempDir)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists && p.is_dir() => {
                    dir = Some(cio::to_vec(&prj.TempDir))
                }
                Err(_) => {}
            }
        }
    }
    // (WASI has no system temporary directory, and std::env::temp_dir()
    //  panics there; the file goes in the current directory, as C's
    //  Linux branch puts it.)
    if dir.is_none() && cfg!(target_os = "wasi") {
        dir = Some(Vec::new());
    }
    if dir.is_none() {
        let t = std::env::temp_dir();
        #[cfg(unix)]
        let b: Vec<u8> = {
            use std::os::unix::ffi::OsStrExt;
            t.as_os_str().as_bytes().to_vec()
        };
        #[cfg(not(unix))]
        let b: Vec<u8> = t.to_string_lossy().into_owned().into_bytes();
        dir = Some(b);
    }
    let mut template = dir.unwrap_or_default();
    if !template.is_empty() && template.last() != Some(&b'/') {
        template.push(b'/');
    }
    template.extend_from_slice(b"swmmXXXXXX");

    // --- use system function mkstemp() to create a temporary file name
    sstrncpy(fname, &template, MAXFNAME as usize);
    mkstemp(prj, fname);
}

/// glibc `mkstemp`: replaces the trailing "XXXXXX" of the template with
/// characters that make a new file name, and creates that file (mode 0600,
/// O_EXCL). On failure the template is left as it was.
fn mkstemp(prj: &mut Project, template: &mut [u8]) {
    const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let len = cio::strlen(template);
    if len < 6 || &template[len - 6..len] != b"XXXXXX" {
        return;
    }
    if prj.swmm5.TempNameSeed == 0 {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        // (std::process::id() panics on WASI, which has no process ids)
        let pid: u64 = if cfg!(target_os = "wasi") { 0 } else { std::process::id() as u64 };
        prj.swmm5.TempNameSeed = t ^ (pid << 32) ^ 0x9E37_79B9_7F4A_7C15;
    }
    let orig: Vec<u8> = template[len - 6..len].to_vec();
    for _attempt in 0..238328 {
        let mut v = prj.swmm5.TempNameSeed;
        for k in 0..6 {
            // xorshift64*
            v ^= v >> 12;
            v ^= v << 25;
            v ^= v >> 27;
            let r = v.wrapping_mul(0x2545_F491_4F6C_DD1D);
            template[len - 6 + k] = LETTERS[(r % 62) as usize];
        }
        prj.swmm5.TempNameSeed = v;
        let path = match cio::path_of(&template[..len]) {
            Some(p) => p,
            None => break,
        };
        let mut o = std::fs::OpenOptions::new();
        o.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            o.mode(0o600);
        }
        match o.open(&path) {
            Ok(_) => return,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => break,
        }
    }
    template[len - 6..len].copy_from_slice(&orig);
}

//=============================================================================

pub fn getElapsedTime(prj: &Project, aDate: DateTime, days: &mut i32, hrs: &mut i32, mins: &mut i32)
//
//  Input:   aDate = simulation calendar date + time
//  Output:  days, hrs, mins = elapsed days, hours & minutes for aDate
//  Purpose: finds elapsed simulation time for a given calendar date
//
{
    let x: DateTime;
    let mut secs: i32 = 0;
    x = aDate - prj.ReportStart;
    if x <= 0.0 {
        *days = 0;
        *hrs = 0;
        *mins = 0;
    } else {
        *days = f64_to_i32(x);
        datetime_decodeTime(x, hrs, mins, &mut secs);
    }
}

//=============================================================================

pub fn getDateTime(prj: &Project, elapsedMsec: f64) -> DateTime
//
//  Input:   elapsedMsec = elapsed milliseconds
//  Output:  returns date/time value
//  Purpose: finds calendar date/time value for elapsed milliseconds of
//           simulation time.
//
{
    datetime_addSeconds(prj.StartDateTime, (elapsedMsec + 1.0) / 1000.0)
}

//=============================================================================

pub fn isRelativePath(fname: &[u8]) -> i32
//
//  Input:   fname = a file name
//  Output:  returns 1 if fname's path is relative or 0 if absolute
//  Purpose: determines if a file name contains a relative or absolute path.
//
{
    if cio::strchr(fname, b':').is_some() {
        return 0;
    }
    let c0 = cio::cstr(fname).first().copied().unwrap_or(0);
    if c0 == b'\\' {
        return 0;
    }
    if c0 == b'/' {
        return 0;
    }
    1
}

//=============================================================================

fn getAbsolutePath(fname: &[u8], absPath: &mut [u8], size: usize)
//
//  Input:   fname = a file name
//           absPath = string to hold the absolute path
//           size = max. size of absPath
//  Output:  absPath = string containing absolute path of fname
//                     (including ending path delimiter)
//  Purpose: finds the full path of the directory for file fname
//
{
    let _ = size;

    // --- case of empty file anme
    if cio::strlen(fname) == 0 {
        return;
    }

    // --- if fname has a relative path then retrieve its full path
    if isRelativePath(fname) != 0 {
        realpath(fname, absPath);
    }
    // --- otherwise copy fname to absPath
    else {
        sstrncpy(absPath, fname, cio::strlen(fname));
    }

    // --- trim file name portion of absPath
    let endOfDir = cio::strrchr(absPath, b'/');
    if let Some(e) = endOfDir {
        if e + 1 < absPath.len() {
            absPath[e + 1] = 0;
        }
    }
}

/// glibc `realpath(fname, absPath)`: std::fs::canonicalize when the path
/// resolves. When it does not, glibc (2.33 and later) still copies into
/// absPath the path it had built up to and including the component that
/// failed; that is emulated here for paths without symbolic links (a
/// symbolic link met on the way is resolved by canonicalizing the prefix).
/// C writes up to PATH_MAX bytes into the 260-byte InpDir; here the result
/// is truncated to the buffer.
fn realpath(fname: &[u8], absPath: &mut [u8]) {
    let p = match cio::path_of(fname) {
        Some(p) => p,
        None => return,
    };
    #[cfg(unix)]
    fn bytes_of(p: &std::path::Path) -> Vec<u8> {
        use std::os::unix::ffi::OsStrExt;
        p.as_os_str().as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    fn bytes_of(p: &std::path::Path) -> Vec<u8> {
        p.to_string_lossy().into_owned().into_bytes()
    }
    if let Ok(full) = std::fs::canonicalize(&p) {
        cio::strcpy(absPath, &bytes_of(&full));
        return;
    }

    // --- error path: the partially resolved name
    let name = cio::cstr(fname);
    let mut dest: Vec<u8> = if name.first() == Some(&b'/') {
        b"/".to_vec()
    } else {
        match std::env::current_dir() {
            Ok(d) => bytes_of(&d),
            Err(_) => return, // glibc returns NULL without touching absPath
        }
    };
    let comps: Vec<&[u8]> = name.split(|&c| c == b'/').filter(|c| !c.is_empty()).collect();
    for (ci, comp) in comps.iter().enumerate() {
        if *comp == b"." {
            continue;
        }
        if *comp == b".." {
            if dest.len() > 1 {
                while dest.len() > 1 && dest.last() != Some(&b'/') {
                    dest.pop();
                }
                if dest.len() > 1 {
                    dest.pop();
                }
            }
            continue;
        }
        if dest.last() != Some(&b'/') {
            dest.push(b'/');
        }
        dest.extend_from_slice(comp);
        let path = match cio::path_of(&dest) {
            Some(p) => p,
            None => break,
        };
        match std::fs::symlink_metadata(&path) {
            Err(_) => break,
            Ok(md) => {
                if md.file_type().is_symlink() {
                    match std::fs::canonicalize(&path) {
                        Ok(t) => dest = bytes_of(&t),
                        Err(_) => break,
                    }
                }
                let more = ci + 1 < comps.len();
                if more && !path.is_dir() {
                    break;
                }
            }
        }
    }
    cio::strcpy(absPath, &dest);
}

//=============================================================================

pub fn addAbsolutePath(prj: &mut Project, fname: &mut [u8])
//
//  Input:   fname = a file name
//  Output:  returns fname with a full path prepended to it
//  Purpose: adds an absolute path name to a file name.
//  Note:    fname must have been dimensioned to accept MAXFNAME characters.
//
{
    let n: usize;
    if isRelativePath(fname) != 0 {
        let mut buffer = cformat!(b"%s%s", &prj.InpDir[..], cio::cstr(fname));
        n = buffer.len();
        // snprintf(buffer, MAXFNAME, ...) keeps at most MAXFNAME-1 characters
        buffer.truncate(MAXFNAME as usize - 1);
        if n > 0 {
            sstrncpy(fname, &buffer, MAXFNAME as usize);
        }
    }
}

//=============================================================================

pub fn writecon(prj: &mut Project, s: &[u8])
//
//  Input:   s = a character string
//  Output:  none
//  Purpose: writes string of characters to the console.
//
{
    let _ = prj;
    let mut out = std::io::stdout();
    let _ = out.write_all(cio::cstr(s));
    let _ = out.flush();
}

//=============================================================================
//  Helpers for C's fixed buffers
//=============================================================================

/// `snprintf(buf, size, ...)` of an already formatted string: keeps at
/// most size-1 bytes and terminates.
fn snprintf_into(buf: &mut [u8], size: usize, s: &[u8]) {
    let n = s.len().min(size.saturating_sub(1)).min(buf.len().saturating_sub(1));
    buf[..n].copy_from_slice(&s[..n]);
    buf[n] = 0;
}

/// `sstrncpy(out, src, n)` into a caller's buffer that is a Vec: the copied
/// bytes, without a terminator.
fn copy_out(out: &mut Vec<u8>, src: &[u8], n: usize) {
    out.clear();
    if n > 0 {
        let s = cio::cstr(src);
        let m = s.len().min(n);
        out.extend_from_slice(&s[..m]);
    }
}
