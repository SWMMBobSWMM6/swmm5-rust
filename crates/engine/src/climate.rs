//! climate.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   climate.c
//
//   Project: EPA SWMM5
//   Version: 5.2
//   Date:    11/01/21  (Build 5.2.0)
//   Author:  L. Rossman
//
//   Climate related functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - NCDC GHCN climate file format added.
//   - Monthly adjustments for temperature, evaporation & rainfall added.
//   Build 5.1.008:
//   - Monthly adjustments for hyd. conductivity added.
//   - Time series evaporation rates can now vary within a day.
//   - Evaporation rates are now properly updated when only flow routing
//     is being simulated.
//   Build 5.1.010:
//   - Hargreaves evaporation now computed using 7-day average temperatures.
//   Build 5.1.011:
//   - Monthly adjustment for hyd. conductivity <= 0 is ignored.
//   Build 5.1.013:
//   - Reads names of monthly adjustment patterns for various parameters
//     of a subcatchment from the [ADJUSTMENTS] section of input file.
//   Build 5.2.0:
//   - Reads temperature units for use with GHCND climate files.
//   - Support added for relative file names.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
// enum ClimateFileFormats
const UNKNOWN_FORMAT: i32 = 0;
const USER_PREPARED: i32 = 1; // SWMM 5's own user format
const GHCND: i32 = 2; // NCDC GHCN Daily format
const TD3200: i32 = 3; // NCDC TD3200 format
const DLY0204: i32 = 4; // Canadian DLY02 or DLY04 format
const MAXCLIMATEVARS: i32 = 4;
const MAXDAYSPERMONTH: i32 = 32;

// These variables are used when processing climate files.
// enum ClimateVarType
const TMIN: i32 = 0;
const TMAX: i32 = 1;
const EVAP: i32 = 2;
const WIND: i32 = 3;
// enum WindSpeedType
const WDMV: i32 = 0;
const AWND: i32 = 1;
// enum TempUnitsType
const DEG_C10: i32 = 0;
const DEG_C: i32 = 1;
const DEG_F: i32 = 2;
const ClimateVarWords: &[&[u8]] = &[b"TMIN", b"TMAX", b"EVAP", b"WDMV", b"AWND"];
const TempUnitsWords: &[&[u8]] = &[b"C10", b"C", b"F"];

//-----------------------------------------------------------------------------
//  Data Structures
//-----------------------------------------------------------------------------
#[derive(Clone, Copy, Default)]
struct TMovAve {
    tAve: f64,     // moving avg. for daily temperature (deg F)
    tRng: f64,     // moving avg. for daily temp. range (deg F)
    ta: [f64; 7],  // data window for tAve
    tr: [f64; 7],  // data window for tRng
    count: i32,    // length of moving average window
    maxCount: i32, // maximum length of moving average window
    front: i32,    // index of front of moving average window
}

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of climate.c.
pub struct ClimateState {
    // Temperature variables
    Tmin: f64,         // min. daily temperature (deg F)
    Tmax: f64,         // max. daily temperature (deg F)
    Trng: f64,         // 1/2 range of daily temperatures
    Trng1: f64,        // prev. max - current min. temp.
    Tave: f64,         // average daily temperature (deg F)
    Hrsr: f64,         // time of min. temp. (hrs)
    Hrss: f64,         // time of max. temp (hrs)
    Hrday: f64,        // avg. of min/max temp times
    Dhrdy: f64,        // hrs. between min. & max. temp. times
    Dydif: f64,        // hrs. between max. & min. temp. times
    LastDay: DateTime, // date of last day with temp. data
    Tma: TMovAve,      // moving average of daily temperatures

    // Evaporation variables
    NextEvapDate: DateTime, // next date when evap. rate changes
    NextEvapRate: f64,      // next evaporation rate (user units)

    // Climate file variables
    FileFormat: i32,             // file format (see ClimateFileFormats)
    FileYear: i32,               // current year of file data
    FileMonth: i32,              // current month of year of file data
    FileDay: i32,                // current day of month of file data
    FileLastDay: i32,            // last day of current month of file data
    FileElapsedDays: i32,        // number of days read from file
    FileValue: [f64; 4],         // current day's values of climate data
    FileData: [[f64; 32]; 4],    // month's worth of daily climate data
    FileLine: [u8; MAXLINE as usize + 1], // line from climate data file

    FileFieldPos: [i32; 4], // start of data fields for file record
    FileDateFieldPos: i32,  // start of date field for file record
    FileWindType: i32,      // wind speed type
    FileTempUnits: i32,     // GHCND file temperature units (C10, C or F)
}

impl Default for ClimateState {
    fn default() -> Self {
        ClimateState {
            Tmin: 0.0,
            Tmax: 0.0,
            Trng: 0.0,
            Trng1: 0.0,
            Tave: 0.0,
            Hrsr: 0.0,
            Hrss: 0.0,
            Hrday: 0.0,
            Dhrdy: 0.0,
            Dydif: 0.0,
            LastDay: 0.0,
            Tma: TMovAve::default(),
            NextEvapDate: 0.0,
            NextEvapRate: 0.0,
            FileFormat: 0,
            FileYear: 0,
            FileMonth: 0,
            FileDay: 0,
            FileLastDay: 0,
            FileElapsedDays: 0,
            FileValue: [0.0; 4],
            FileData: [[0.0; 32]; 4],
            FileLine: [0; MAXLINE as usize + 1],
            FileFieldPos: [0; 4],
            FileDateFieldPos: 0,
            FileWindType: 0,
            FileTempUnits: 0,
        }
    }
}

//-----------------------------------------------------------------------------
//  External functions (defined in funcs.h)
//-----------------------------------------------------------------------------
//  climate_readParams                 // called by input_parseLine
//  climate_readEvapParams             // called by input_parseLine
//  climate_validate                   // called by project_validate
//  climate_openFile                   // called by runoff_open
//  climate_initState                  // called by project_init
//  climate_setState                   // called by runoff_execute
//  climate_getNextEvapDate            // called by runoff_getTimeStep

//-----------------------------------------------------------------------------
//  Helpers for C library behaviour
//-----------------------------------------------------------------------------

/// `fgets(buf, n, f)` into a fixed C buffer: on success the line and its
/// terminator overwrite the start of `buf`; the bytes after the terminator
/// keep whatever they held, as in C. Returns false (buffer untouched) at EOF.
fn fgets_into(f: &mut CFile, buf: &mut [u8], n: usize) -> bool {
    match f.fgets(n) {
        Some(line) => {
            buf[..line.len()].copy_from_slice(&line);
            buf[line.len()] = 0;
            true
        }
        None => false,
    }
}

/// First character of a C string (`*s`), 0 for an empty string.
#[inline]
fn ch0(s: &[u8]) -> u8 {
    s.first().copied().unwrap_or(0)
}

/// input.c's `match(str, substr)`: 1 if `substr` matches the start of
/// `str` after leading blanks (not case sensitive). There is no stub for
/// it (`match` is a Rust keyword), so climate.c's uses call this copy.
fn match_(str: &[u8], substr: &[u8]) -> i32 {
    let at = |s: &[u8], i: usize| s.get(i).copied().unwrap_or(0);
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;

    // --- fail if substring is empty
    if at(substr, 0) == 0 {
        return 0;
    }

    // --- skip leading blanks of str
    k = 0;
    while at(str, k) != 0 {
        if at(str, k) != b' ' {
            break;
        }
        k += 1;
    }

    // --- check if substr matches remainder of str
    i = k;
    j = 0;
    while at(substr, j) != 0 {
        if at(str, i) == 0 || UCHAR(at(str, i)) != UCHAR(at(substr, j)) {
            return 0;
        }
        i += 1;
        j += 1;
    }
    1
}

//=============================================================================

pub fn climate_readParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns error code
//  Purpose: reads climate/temperature parameters from input line of data
//
//  Format of data can be
//    TIMESERIES  name
//    FILE        name  (start)  (units)
//    WINDSPEED   MONTHLY  v1  v2  ...  v12
//    WINDSPEED   FILE
//    SNOWMELT    v1  v2  ...  v6
//    ADC         IMPERV/PERV  v1  v2  ...  v10
//
{
    let mut i: i32;
    let mut j: i32;
    let k: i32;
    let mut x: [f64; 6] = [0.0; 6];
    let mut y: f64 = 0.0;
    let mut aDate: DateTime = 0.0;
    let mut fname: [u8; MAXFNAME as usize + 1] = [0; MAXFNAME as usize + 1];

    // --- identify keyword
    k = findmatch(&tok[0], TempKeyWords);
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[0]);
    }
    match k {
        0 => {
            // Time series name
            // --- check that time series name exists
            if ntoks < 2 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            i = project_findObject(prj, TSERIES, &tok[1]);
            if i < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[1]);
            }

            // --- record the time series as being the data source for temperature
            prj.Temp.dataSource = TSERIES_TEMP;
            prj.Temp.tSeries = i;
            prj.Tseries[i as usize].refersTo = TSERIES_TEMP;
        }

        1 => {
            // Climate file
            // --- record file as being source of temperature data
            if ntoks < 2 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            prj.Temp.dataSource = FILE_TEMP;

            // --- save name and usage mode of external climate file
            prj.Fclimate.mode = USE_FILE as i8;
            sstrncpy(&mut fname, &tok[1], MAXFNAME as usize);
            addAbsolutePath(prj, &mut fname);
            sstrncpy(&mut prj.Fclimate.name, &fname, MAXFNAME as usize);

            // --- save starting date to read from file if one is provided
            prj.Temp.fileStartDate = NO_DATE as f64;
            if ntoks > 2 {
                if ch0(&tok[2]) != b'*' {
                    if datetime_strToDate(prj, &tok[2], &mut aDate) == 0 {
                        return error_setInpError(prj, ERR_DATETIME, &tok[2]);
                    }
                    prj.Temp.fileStartDate = aDate;
                }
            }

            // --- file temperature units
            prj.climate.FileTempUnits = DEG_F;
            if prj.UnitSystem == SI {
                prj.climate.FileTempUnits = DEG_C;
            }
            if ntoks > 3 {
                i = findmatch(&tok[3], TempUnitsWords);
                if i < 0 {
                    return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
                }
                prj.climate.FileTempUnits = i;
            }
        }

        2 => {
            // Wind speeds
            // --- check if wind speeds will be supplied from climate file
            //     (C reads tok[1] here without checking ntoks)
            if strcomp(&tok[1], w_FILE) != 0 {
                prj.Wind.type_ = FILE_WIND;
            }
            // --- otherwise read 12 monthly avg. wind speed values
            else {
                if ntoks < 14 {
                    return error_setInpError(prj, ERR_ITEMS, b"");
                }
                prj.Wind.type_ = MONTHLY_WIND;
                i = 0;
                while i < 12 {
                    if getDouble(&tok[(i + 2) as usize], &mut y) == 0 {
                        return error_setInpError(prj, ERR_NUMBER, &tok[(i + 2) as usize]);
                    }
                    prj.Wind.aws[i as usize] = y;
                    i += 1;
                }
            }
        }

        3 => {
            // Snowmelt params
            if ntoks < 7 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            i = 1;
            while i < 7 {
                if getDouble(&tok[i as usize], &mut x[(i - 1) as usize]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
                }
                i += 1;
            }
            // --- convert deg. C to deg. F for snowfall temperature
            if prj.UnitSystem == SI {
                x[0] = 9.0 / 5.0 * x[0] + 32.0;
            }
            prj.Snow.snotmp = x[0];
            prj.Snow.tipm = x[1];
            prj.Snow.rnm = x[2];
            prj.Temp.elev = x[3] / UCF(prj, LENGTH);
            prj.Temp.anglat = x[4];
            prj.Temp.dtlong = x[5] / 60.0;
        }

        4 => {
            // Areal Depletion Curve data
            // --- check if data is for impervious or pervious areas
            if ntoks < 12 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            if match_(&tok[1], w_IMPERV) != 0 {
                i = 0;
            } else if match_(&tok[1], w_PERV) != 0 {
                i = 1;
            } else {
                return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
            }

            // --- read 10 fractional values
            j = 0;
            while j < 10 {
                if getDouble(&tok[(j + 2) as usize], &mut y) == 0 || y < 0.0 || y > 1.0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[(j + 2) as usize]);
                }
                prj.Snow.adc[i as usize][j as usize] = y;
                j += 1;
            }
        }
        _ => {}
    }
    0
}

//=============================================================================

pub fn climate_readEvapParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns error code
//  Purpose: reads evaporation parameters from input line of data.
//
//  Data formats are:
//    CONSTANT  value
//    MONTHLY   v1 ... v12
//    TIMESERIES name
//    TEMPERATURE
//    FILE      (v1 ... v12)
//    RECOVERY   name
//    DRY_ONLY   YES/NO
//
{
    let mut i: i32;
    let k: i32;
    let mut x: f64 = 0.0;

    // --- find keyword indicating what form the evaporation data is in
    k = findmatch(&tok[0], EvapTypeWords);
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[0]);
    }

    // --- check for RECOVERY pattern data
    if k == RECOVERY {
        if ntoks < 2 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = project_findObject(prj, TIMEPATTERN, &tok[1]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[1]);
        }
        prj.Evap.recoveryPattern = i;
        return 0;
    }

    // --- check for no evaporation in wet periods
    if k == DRYONLY {
        if ntoks < 2 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if strcomp(&tok[1], w_NO) != 0 {
            prj.Evap.dryOnly = FALSE;
        } else if strcomp(&tok[1], w_YES) != 0 {
            prj.Evap.dryOnly = TRUE;
        } else {
            return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
        }
        return 0;
    }

    // --- process data depending on its form
    prj.Evap.type_ = k;
    if k != TEMPERATURE_EVAP && ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    match k {
        CONSTANT_EVAP => {
            // --- for constant evap., fill monthly avg. values with same number
            if getDouble(&tok[1], &mut x) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[1]);
            }
            i = 0;
            while i < 12 {
                prj.Evap.monthlyEvap[i as usize] = x;
                i += 1;
            }
        }

        MONTHLY_EVAP => {
            // --- for monthly evap., read a value for each month of year
            if ntoks < 13 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            i = 0;
            while i < 12 {
                if getDouble(&tok[(i + 1) as usize], &mut prj.Evap.monthlyEvap[i as usize]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[(i + 1) as usize]);
                }
                i += 1;
            }
        }

        TIMESERIES_EVAP => {
            // --- for time series evap., read name of time series
            i = project_findObject(prj, TSERIES, &tok[1]);
            if i < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[1]);
            }
            prj.Evap.tSeries = i;
            prj.Tseries[i as usize].refersTo = TIMESERIES_EVAP;
        }

        FILE_EVAP => {
            // --- for evap. from climate file, read monthly pan coeffs.
            //     if they are provided (default values are 1.0)
            if ntoks > 1 {
                if ntoks < 13 {
                    return error_setInpError(prj, ERR_ITEMS, b"");
                }
                i = 0;
                while i < 12 {
                    if getDouble(&tok[(i + 1) as usize], &mut prj.Evap.panCoeff[i as usize]) == 0 {
                        return error_setInpError(prj, ERR_NUMBER, &tok[(i + 1) as usize]);
                    }
                    i += 1;
                }
            }
        }
        _ => {}
    }
    0
}

//=============================================================================

pub fn climate_readAdjustments(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns error code
//  Purpose: reads adjustments to monthly evaporation or rainfall
//           from input line of data.
//
//  Data formats are:
//    TEMPERATURE   v1 ... v12
//    EVAPORATION   v1 ... v12
//    RAINFALL      v1 ... v12
//    CONDUCTIVITY  v1 ... v12
//    N-PERV        subcatchID  patternID
//    DSTORE        subcatchID  patternID
//    INFIL         subcatchID  patternID
{
    let mut i: i32;
    let mut j: i32;

    if ntoks == 1 {
        return 0;
    }

    if match_(&tok[0], b"TEMP") != 0 {
        if ntoks < 13 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = 1;
        while i < 13 {
            if getDouble(&tok[i as usize], &mut prj.Adjust.temp[(i - 1) as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
            }
            i += 1;
        }
        return 0;
    }

    if match_(&tok[0], b"EVAP") != 0 {
        if ntoks < 13 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = 1;
        while i < 13 {
            if getDouble(&tok[i as usize], &mut prj.Adjust.evap[(i - 1) as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
            }
            i += 1;
        }
        return 0;
    }

    if match_(&tok[0], b"RAIN") != 0 {
        if ntoks < 13 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = 1;
        while i < 13 {
            if getDouble(&tok[i as usize], &mut prj.Adjust.rain[(i - 1) as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
            }
            i += 1;
        }
        return 0;
    }

    if match_(&tok[0], b"CONDUCT") != 0 {
        if ntoks < 13 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = 1;
        while i < 13 {
            if getDouble(&tok[i as usize], &mut prj.Adjust.hydcon[(i - 1) as usize]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
            }
            if prj.Adjust.hydcon[(i - 1) as usize] <= 0.0 {
                prj.Adjust.hydcon[(i - 1) as usize] = 1.0;
            }
            i += 1;
        }
        return 0;
    }

    if match_(&tok[0], b"N-PERV") != 0 {
        if ntoks < 3 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = project_findObject(prj, SUBCATCH, &tok[1]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[1]);
        }
        j = project_findObject(prj, TIMEPATTERN, &tok[2]);
        if j < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[2]);
        }
        prj.Subcatch[i as usize].nPervPattern = j;
        return 0;
    }

    if match_(&tok[0], b"DSTORE") != 0 {
        if ntoks < 3 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = project_findObject(prj, SUBCATCH, &tok[1]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[1]);
        }
        j = project_findObject(prj, TIMEPATTERN, &tok[2]);
        if j < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[2]);
        }
        prj.Subcatch[i as usize].dStorePattern = j;
        return 0;
    }

    if match_(&tok[0], b"INFIL") != 0 {
        if ntoks < 3 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        i = project_findObject(prj, SUBCATCH, &tok[1]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[1]);
        }
        j = project_findObject(prj, TIMEPATTERN, &tok[2]);
        if j < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[2]);
        }
        prj.Subcatch[i as usize].infilPattern = j;
        return 0;
    }
    error_setInpError(prj, ERR_KEYWORD, &tok[0])
}

//=============================================================================

pub fn climate_validate(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: validates climatological variables
//
{
    let mut i: i32;
    let a: f64;
    let z: f64;
    let pa: f64;

    // --- check if climate data comes from external data file
    if prj.Wind.type_ == FILE_WIND || prj.Evap.type_ == FILE_EVAP || prj.Evap.type_ == TEMPERATURE_EVAP {
        if prj.Fclimate.mode as i32 == NO_FILE {
            report_writeErrorMsg(prj, ERR_NO_CLIMATE_FILE, b"");
        }
    }

    // --- open the climate data file
    if prj.Fclimate.mode as i32 == USE_FILE {
        climate_openFile(prj);
    }

    // --- snow melt parameters tipm & rnm must be fractions
    if prj.Snow.tipm < 0.0 || prj.Snow.tipm > 1.0 || prj.Snow.rnm < 0.0 || prj.Snow.rnm > 1.0 {
        report_writeErrorMsg(prj, ERR_SNOWMELT_PARAMS, b"");
    }

    // --- latitude should be between -90 & 90 degrees
    a = prj.Temp.anglat;
    if a <= -89.99 || a >= 89.99 {
        report_writeErrorMsg(prj, ERR_SNOWMELT_PARAMS, b"");
    } else {
        prj.Temp.tanAnglat = (a * PI / 180.0).tan();
    }

    // --- compute psychrometric constant
    z = prj.Temp.elev / 1000.0;
    if z <= 0.0 {
        pa = 29.9;
    } else {
        pa = 29.9 - 1.02 * z + 0.0032 * z.powf(2.4); // atmos. pressure
    }
    prj.Temp.gamma = 0.000359 * pa;

    // --- convert units of monthly temperature & evap adjustments
    i = 0;
    while i < 12 {
        if prj.UnitSystem == SI {
            prj.Adjust.temp[i as usize] *= 9.0 / 5.0;
        }
        let ucf = UCF(prj, EVAPRATE);
        prj.Adjust.evap[i as usize] /= ucf;
        i += 1;
    }
}

//=============================================================================

/// Declared in funcs.h (called only from this module).
pub fn climate_openFile(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: opens a climate file and reads in first set of values.
//
{
    let mut i: i32;
    // (C leaves m and y uninitialized; readFileLine sets them on every path
    //  that reaches the comparison except at end of file, where C compares
    //  stale values that cannot match. 0 cannot match a year/month either.)
    let mut m: i32 = 0;
    let mut y: i32 = 0;

    // --- open the file
    prj.Fclimate.file = CFile::open(&prj.Fclimate.name, "rt");
    if prj.Fclimate.file.is_none() {
        let name = prj.Fclimate.name;
        report_writeErrorMsg(prj, ERR_CLIMATE_FILE_OPEN, &name);
        return;
    }

    // --- initialize values of file's climate variables
    //     (Temp.ta was previously initialized in project.c)
    prj.climate.FileValue[TMIN as usize] = prj.Temp.ta;
    prj.climate.FileValue[TMAX as usize] = prj.Temp.ta;
    prj.climate.FileValue[EVAP as usize] = 0.0;
    prj.climate.FileValue[WIND as usize] = 0.0;

    // --- find climate file's format
    prj.climate.FileFormat = getFileFormat(prj);
    if prj.climate.FileFormat == UNKNOWN_FORMAT {
        let name = prj.Fclimate.name;
        report_writeErrorMsg(prj, ERR_CLIMATE_FILE_READ, &name);
        return;
    }

    // --- position file to begin reading climate file at either user-specified
    //     month/year or at start of simulation period.
    prj.Fclimate.f().rewind();
    sstrncpy(&mut prj.climate.FileLine, b"", 0);
    if prj.Temp.fileStartDate == NO_DATE as f64 {
        datetime_decodeDate(
            prj.StartDate,
            &mut prj.climate.FileYear,
            &mut prj.climate.FileMonth,
            &mut prj.climate.FileDay,
        );
    } else {
        datetime_decodeDate(
            prj.Temp.fileStartDate,
            &mut prj.climate.FileYear,
            &mut prj.climate.FileMonth,
            &mut prj.climate.FileDay,
        );
    }
    while !prj.Fclimate.f().feof() {
        sstrncpy(&mut prj.climate.FileLine, b"", 0);
        readFileLine(prj, &mut y, &mut m);
        if y == prj.climate.FileYear && m == prj.climate.FileMonth {
            break;
        }
    }
    if prj.Fclimate.f().feof() {
        let name = prj.Fclimate.name;
        report_writeErrorMsg(prj, ERR_CLIMATE_END_OF_FILE, &name);
        return;
    }

    // --- initialize file dates and current climate variable values
    if prj.ErrorCode == 0 {
        prj.climate.FileElapsedDays = 0;
        prj.climate.FileLastDay = datetime_daysPerMonth(prj.climate.FileYear, prj.climate.FileMonth);
        readFileValues(prj);
        i = TMIN;
        while i <= WIND {
            let st = &mut prj.climate;
            if st.FileData[i as usize][st.FileDay as usize] == MISSING {
                i += 1;
                continue;
            }
            st.FileValue[i as usize] = st.FileData[i as usize][st.FileDay as usize];
            i += 1;
        }
    }
}

//=============================================================================

pub fn climate_initState(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: initializes climate state variables.
//
{
    prj.climate.LastDay = NO_DATE as f64;
    prj.Temp.tmax = MISSING;
    prj.Snow.removed = 0.0;
    prj.climate.NextEvapDate = prj.StartDate;
    prj.climate.NextEvapRate = 0.0;

    // --- initialize variables for time series evaporation
    if prj.Evap.type_ == TIMESERIES_EVAP && prj.Evap.tSeries >= 0 {
        // --- initialize NextEvapDate & NextEvapRate to first entry of
        //     time series whose date <= the simulation start date
        let k = prj.Evap.tSeries as usize;
        table_getFirstEntry(
            &mut prj.Tseries[k],
            &mut prj.climate.NextEvapDate,
            &mut prj.climate.NextEvapRate,
        );
        if prj.climate.NextEvapDate < prj.StartDate {
            let d = prj.StartDate;
            setNextEvapDate(prj, d);
        }
        prj.Evap.rate = prj.climate.NextEvapRate / UCF(prj, EVAPRATE);

        // --- find the next time evaporation rates change after this
        let d = prj.climate.NextEvapDate;
        setNextEvapDate(prj, d);
    }

    // --- initialize variables for temperature evaporation
    if prj.Evap.type_ == TEMPERATURE_EVAP {
        let tma = &mut prj.climate.Tma;
        tma.maxCount = tma.ta.len() as i32; // sizeof(Tma.ta) / sizeof(double)
        tma.count = 0;
        tma.front = 0;
        tma.tAve = 0.0;
        tma.tRng = 0.0;
    }
}

//=============================================================================

pub fn climate_setState(prj: &mut Project, theDate: DateTime)
//
//  Input:   theDate = simulation date
//  Output:  none
//  Purpose: sets climate variables for current date.
//
{
    if prj.Fclimate.mode as i32 == USE_FILE {
        updateFileValues(prj, theDate);
    }
    if prj.Temp.dataSource != NO_TEMP {
        setTemp(prj, theDate);
    }
    setEvap(prj, theDate);
    setWind(prj, theDate);
    prj.Adjust.rainFactor = prj.Adjust.rain[(datetime_monthOfYear(theDate) - 1) as usize];
    prj.Adjust.hydconFactor = prj.Adjust.hydcon[(datetime_monthOfYear(theDate) - 1) as usize];
    setNextEvapDate(prj, theDate);
}

//=============================================================================

pub fn climate_getNextEvapDate(prj: &mut Project) -> DateTime
//
//  Input:   none
//  Output:  returns the current value of NextEvapDate
//  Purpose: gets the next date when evaporation rate changes.
//
{
    prj.climate.NextEvapDate
}

//=============================================================================

fn setNextEvapDate(prj: &mut Project, theDate: DateTime)
//
//  Input:   theDate = current simulation date
//  Output:  sets a new value for NextEvapDate
//  Purpose: finds date for next change in evaporation after the current date.
//
{
    let mut yr: i32 = 0;
    let mut mon: i32 = 0;
    let mut day: i32 = 0;
    let k: i32;
    let mut d: f64 = 0.0;
    let mut e: f64 = 0.0;

    // --- do nothing if current date hasn't reached the current next date
    if prj.climate.NextEvapDate > theDate {
        return;
    }

    match prj.Evap.type_ {
        // --- for constant evaporation, use a next date far in the future
        CONSTANT_EVAP => {
            prj.climate.NextEvapDate = theDate + 365.0;
        }

        // --- for monthly evaporation, use the start of the next month
        MONTHLY_EVAP => {
            datetime_decodeDate(theDate, &mut yr, &mut mon, &mut day);
            if mon == 12 {
                mon = 1;
                yr += 1;
            } else {
                mon += 1;
            }
            prj.climate.NextEvapDate = datetime_encodeDate(yr, mon, 1);
        }

        // --- for time series evaporation, find the next entry in the
        //     series on or after the current date
        TIMESERIES_EVAP => {
            k = prj.Evap.tSeries;
            if k >= 0 {
                prj.climate.NextEvapDate = theDate + 365.0;
                while table_getNextEntry(&mut prj.Tseries[k as usize], &mut d, &mut e) != 0
                    && d <= prj.EndDateTime
                {
                    if d >= theDate {
                        prj.climate.NextEvapDate = d;
                        prj.climate.NextEvapRate = e;
                        break;
                    }
                }
            }
        }

        // --- for climate file daily evaporation, use the next day
        FILE_EVAP => {
            prj.climate.NextEvapDate = theDate.floor() + 1.0;
        }

        _ => {
            prj.climate.NextEvapDate = theDate + 365.0;
        }
    }
}

//=============================================================================

fn updateFileValues(prj: &mut Project, theDate: DateTime)
//
//  Input:   theDate = current simulation date
//  Output:  none
//  Purpose: updates daily climate variables for new day or reads in
//           another month worth of values if a new month begins.
//
//  NOTE:    counters FileElapsedDays, FileDay, FileMonth, FileYear and
//           FileLastDay were initialized in climate_openFile().
//
{
    let mut i: i32;
    let deltaDays: i32;

    // --- see if a new day has begun
    deltaDays = f64_to_i32(theDate.floor() - prj.StartDateTime.floor());
    if deltaDays > prj.climate.FileElapsedDays {
        // --- advance day counters
        prj.climate.FileElapsedDays += 1;
        prj.climate.FileDay += 1;

        // --- see if new month of data needs to be read from file
        if prj.climate.FileDay > prj.climate.FileLastDay {
            prj.climate.FileMonth += 1;
            if prj.climate.FileMonth > 12 {
                prj.climate.FileMonth = 1;
                prj.climate.FileYear += 1;
            }
            readFileValues(prj);
            prj.climate.FileDay = 1;
            prj.climate.FileLastDay = datetime_daysPerMonth(prj.climate.FileYear, prj.climate.FileMonth);
        }

        // --- set climate variables for new day
        i = TMIN;
        while i <= WIND {
            let st = &mut prj.climate;
            // --- no change in current value if its missing
            if st.FileData[i as usize][st.FileDay as usize] == MISSING {
                i += 1;
                continue;
            }
            st.FileValue[i as usize] = st.FileData[i as usize][st.FileDay as usize];
            i += 1;
        }
    }
}

//=============================================================================

fn setTemp(prj: &mut Project, theDate: DateTime)
//
//  Input:   theDate = simulation date
//  Output:  none
//  Purpose: updates temperatures for new simulation date.
//
{
    let mut j: i32; // snow data object index
    let k: i32; // time series index
    let mon: i32; // month of year
    let day: i32; // day of year
    let theDay: DateTime; // calendar day
    let hour: f64; // hour of day
    let tmp: f64; // temporary temperature

    // --- see if a new day has started
    mon = datetime_monthOfYear(theDate);
    theDay = theDate.floor();
    if theDay > prj.climate.LastDay {
        // --- update min. & max. temps & their time of day
        day = datetime_dayOfYear(theDate);
        if prj.Temp.dataSource == FILE_TEMP {
            prj.climate.Tmin = prj.climate.FileValue[TMIN as usize] + prj.Adjust.temp[(mon - 1) as usize];
            prj.climate.Tmax = prj.climate.FileValue[TMAX as usize] + prj.Adjust.temp[(mon - 1) as usize];
            if prj.climate.Tmin > prj.climate.Tmax {
                tmp = prj.climate.Tmin;
                prj.climate.Tmin = prj.climate.Tmax;
                prj.climate.Tmax = tmp;
            }
            updateTempTimes(prj, day);
            if prj.Evap.type_ == TEMPERATURE_EVAP {
                let (tmin, tmax) = (prj.climate.Tmin, prj.climate.Tmax);
                updateTempMoveAve(prj, tmin, tmax);
                let (tave, trng) = (prj.climate.Tma.tAve, prj.climate.Tma.tRng);
                prj.climate.FileValue[EVAP as usize] = getTempEvap(prj, day, tave, trng);
            }
        }

        // --- compute snow melt coefficients based on day of year
        prj.Snow.season = (0.0172615 * (day as f64 - 81.0)).sin();
        j = 0;
        while j < prj.Nobjects[SNOWMELT as usize] {
            let season = prj.Snow.season;
            snow_setMeltCoeffs(prj, j, season);
            j += 1;
        }

        // --- update date of last day analyzed
        prj.climate.LastDay = theDate;
    }

    // --- for min/max daily temps. from climate file,
    //     compute hourly temp. by sinusoidal interp.
    if prj.Temp.dataSource == FILE_TEMP {
        let st = &prj.climate;
        hour = (theDate - theDay) * 24.0;
        if hour < st.Hrsr {
            prj.Temp.ta = st.Tmin + st.Trng1 / 2.0 * (PI / st.Dydif * (st.Hrsr - hour)).sin();
        } else if hour >= st.Hrsr && hour <= st.Hrss {
            prj.Temp.ta = st.Tave + st.Trng * (PI / st.Dhrdy * (st.Hrday - hour)).sin();
        } else {
            prj.Temp.ta = st.Tmax - st.Trng * (PI / st.Dydif * (hour - st.Hrss)).sin();
        }
    }

    // --- for user-supplied temperature time series,
    //     get temperature value from time series
    if prj.Temp.dataSource == TSERIES_TEMP {
        k = prj.Temp.tSeries;
        if k >= 0 {
            prj.Temp.ta = table_tseriesLookup(&mut prj.Tseries[k as usize], theDate, TRUE as i8);

            // --- convert from deg. C to deg. F if need be
            if prj.UnitSystem == SI {
                prj.Temp.ta = (9.0 / 5.0) * prj.Temp.ta + 32.0;
            }

            // --- apply climate change adjustment factor
            prj.Temp.ta += prj.Adjust.temp[(mon - 1) as usize];
        }
    }

    // --- compute saturation vapor pressure
    prj.Temp.ea = 8.1175e6 * (-7701.544 / (prj.Temp.ta + 405.0265)).exp();
}

//=============================================================================

fn setEvap(prj: &mut Project, theDate: DateTime)
//
//  Input:   theDate = simulation date
//  Output:  none
//  Purpose: sets evaporation rate (ft/sec) for a specified date.
//
{
    let k: i32;
    let mon: i32 = datetime_monthOfYear(theDate);

    match prj.Evap.type_ {
        CONSTANT_EVAP => {
            prj.Evap.rate = prj.Evap.monthlyEvap[0] / UCF(prj, EVAPRATE);
        }

        MONTHLY_EVAP => {
            prj.Evap.rate = prj.Evap.monthlyEvap[(mon - 1) as usize] / UCF(prj, EVAPRATE);
        }

        TIMESERIES_EVAP => {
            if theDate >= prj.climate.NextEvapDate {
                prj.Evap.rate = prj.climate.NextEvapRate / UCF(prj, EVAPRATE);
            }
        }

        FILE_EVAP => {
            prj.Evap.rate = prj.climate.FileValue[EVAP as usize] / UCF(prj, EVAPRATE);
            prj.Evap.rate *= prj.Evap.panCoeff[(mon - 1) as usize];
        }

        TEMPERATURE_EVAP => {
            prj.Evap.rate = prj.climate.FileValue[EVAP as usize] / UCF(prj, EVAPRATE);
        }

        _ => {
            prj.Evap.rate = 0.0;
        }
    }

    // --- apply climate change adjustment
    prj.Evap.rate += prj.Adjust.evap[(mon - 1) as usize];

    // --- set soil recovery factor
    prj.Evap.recoveryFactor = 1.0;
    k = prj.Evap.recoveryPattern;
    if k >= 0 && prj.Pattern[k as usize].type_ == MONTHLY_PATTERN {
        prj.Evap.recoveryFactor = prj.Pattern[k as usize].factor[(mon - 1) as usize];
    }
}

//=============================================================================

fn setWind(prj: &mut Project, theDate: DateTime)
//
//  Input:   theDate = simulation date
//  Output:  none
//  Purpose: sets wind speed (mph) for a specified date.
//
{
    let mut yr: i32 = 0;
    let mut mon: i32 = 0;
    let mut day: i32 = 0;

    match prj.Wind.type_ {
        MONTHLY_WIND => {
            datetime_decodeDate(theDate, &mut yr, &mut mon, &mut day);
            prj.Wind.ws = prj.Wind.aws[(mon - 1) as usize] / UCF(prj, WINDSPEED);
        }

        FILE_WIND => {
            prj.Wind.ws = prj.climate.FileValue[WIND as usize];
        }

        _ => {
            prj.Wind.ws = 0.0;
        }
    }
}

//=============================================================================

fn updateTempTimes(prj: &mut Project, day: i32)
//
//  Input:   day = day of year
//  Output:  none
//  Purpose: computes time of day when min/max temperatures occur.
//           (min. temp occurs at sunrise, max. temp. at 3 hrs. < sunset)
//
{
    let decl: f64; // earth's declination
    let hrang: f64; // hour angle of sunrise/sunset
    let mut arg: f64;

    decl = 0.40928 * (0.017202 * (172.0 - day as f64)).cos();
    arg = -decl.tan() * prj.Temp.tanAnglat;
    if arg <= -1.0 {
        arg = PI;
    } else if arg >= 1.0 {
        arg = 0.0;
    } else {
        arg = arg.acos();
    }
    hrang = 3.8197 * arg;
    let st = &mut prj.climate;
    st.Hrsr = 12.0 - hrang + prj.Temp.dtlong;
    st.Hrss = 12.0 + hrang + prj.Temp.dtlong - 3.0;
    st.Dhrdy = st.Hrsr - st.Hrss;
    st.Dydif = 24.0 + st.Hrsr - st.Hrss;
    st.Hrday = (st.Hrsr + st.Hrss) / 2.0;
    st.Tave = (st.Tmin + st.Tmax) / 2.0;
    st.Trng = (st.Tmax - st.Tmin) / 2.0;
    if prj.Temp.tmax == MISSING {
        st.Trng1 = st.Tmax - st.Tmin;
    } else {
        st.Trng1 = prj.Temp.tmax - st.Tmin;
    }
    prj.Temp.tmax = st.Tmax;
}

//=============================================================================

fn getTempEvap(prj: &Project, day: i32, tave: f64, trng: f64) -> f64
//
//  Input:   day = day of year
//           tave = 7-day average temperature (deg F)
//           trng = 7-day average daily temperature range (deg F)
//  Output:  returns evaporation rate in user's units (US:in/day, SI:mm/day)
//  Purpose: uses Hargreaves method to compute daily evaporation rate
//           from daily average temperatures and Julian day.
//
{
    let a: f64 = 2.0 * PI / 365.0;
    let ta: f64 = (tave - 32.0) * 5.0 / 9.0; //average temperature (deg C)
    let tr: f64 = trng * 5.0 / 9.0; //temperature range (deg C)
    let lamda: f64 = 2.50 - 0.002361 * ta; //latent heat of vaporization
    let dr: f64 = 1.0 + 0.033 * (a * day as f64).cos(); //relative earth-sun distance
    let phi: f64 = prj.Temp.anglat * 2.0 * PI / 360.0; //latitude angle (rad)
    let del: f64 = 0.4093 * (a * (284.0 + day as f64)).sin(); //solar declination angle (rad)
    let omega: f64 = (-phi.tan() * del.tan()).acos(); //sunset hour angle (rad)
    let ra: f64 = 37.6 * dr * //extraterrestrial radiation
                (omega * phi.sin() * del.sin() +
                 phi.cos() * del.cos() * omega.sin());
    let mut e: f64 = 0.0023 * ra / lamda * tr.sqrt() * (ta + 17.8); //evap. rate (mm/day)
    if e < 0.0 {
        e = 0.0;
    }
    if prj.UnitSystem == US {
        e /= MMperINCH; //evap rate (in/day)
    }
    e
}

//=============================================================================

fn getFileFormat(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns code number of climate file's format
//  Purpose: determines what format the climate file is in.
//
{
    let mut recdType: [u8; 4] = [0; 4];
    let mut elemType: [u8; 4] = [0; 4];
    let mut filler: [u8; 5] = [0; 5];
    let mut staID: Vec<u8> = Vec::new();
    let mut s: Vec<u8> = Vec::new();
    // (C's line buffer is uninitialized stack; bytes past the line's
    //  terminator, read below at line[23] for a short line, are zeros here)
    let mut line: [u8; MAXLINE as usize] = [0; MAXLINE as usize];

    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    let mut n: i32;

    // --- read first line of file
    if !fgets_into(prj.Fclimate.f(), &mut line, MAXLINE as usize) {
        return UNKNOWN_FORMAT;
    }

    // --- check for TD3200 format
    sstrncpy(&mut recdType, &line, 3);
    sstrncpy(&mut filler, &line[23..], 4);
    if cio::strcmp(&recdType, b"DLY") == 0 && cio::strcmp(&filler, b"9999") == 0 {
        return TD3200;
    }

    // --- check for DLY0204 format
    if cio::strlen(&line) >= 233 {
        sstrncpy(&mut elemType, &line[13..], 3);
        n = cio::atoi(&elemType);
        if n == 1 || n == 2 || n == 151 {
            return DLY0204;
        }
    }

    // --- check for USER_PREPARED format
    n = cio::sscanf(
        &line,
        b"%s %d %d %d %s",
        &mut [
            Scan::Str(&mut staID),
            Scan::Int(&mut y),
            Scan::Int(&mut m),
            Scan::Int(&mut d),
            Scan::Str(&mut s),
        ],
    );
    if n == 5 {
        return USER_PREPARED;
    }

    // --- check for GHCND format
    if isGhcndFormat(prj, &line) != 0 {
        return GHCND;
    }

    UNKNOWN_FORMAT
}

//=============================================================================

fn readFileLine(prj: &mut Project, y: &mut i32, m: &mut i32)
//
//  Input:   none
//  Output:  y = year
//           m = month
//  Purpose: reads year & month from next line of climate file.
//
{
    // --- read next line from climate data file
    while cio::strlen(&prj.climate.FileLine) == 0 {
        if !fgets_into(
            prj.Fclimate.file.as_mut().expect("I/O on a file that is not open"),
            &mut prj.climate.FileLine,
            MAXLINE as usize,
        ) {
            return;
        }
        if prj.climate.FileLine[0] == b'\n' {
            prj.climate.FileLine[0] = 0;
        }
    }

    // --- parse year & month from line
    match prj.climate.FileFormat {
        USER_PREPARED => readUserFileLine(prj, y, m),
        TD3200 => readTD3200FileLine(prj, y, m),
        DLY0204 => readDLY0204FileLine(prj, y, m),
        GHCND => readGhcndFileLine(prj, y, m),
        _ => {}
    }
}

//=============================================================================

fn readUserFileLine(prj: &mut Project, y: &mut i32, m: &mut i32)
//
//  Input:   none
//  Output:  y = year
//           m = month
//  Purpose: reads year & month from line of User-Prepared climate file.
//
{
    let n: i32;
    let mut staID: Vec<u8> = Vec::new();
    n = cio::sscanf(
        &prj.climate.FileLine,
        b"%s %d %d",
        &mut [Scan::Str(&mut staID), Scan::Int(y), Scan::Int(m)],
    );
    if n < 3 {
        let name = prj.Fclimate.name;
        report_writeErrorMsg(prj, ERR_CLIMATE_FILE_READ, &name);
    }
}

//=============================================================================

fn readTD3200FileLine(prj: &mut Project, y: &mut i32, m: &mut i32)
//
//  Input:   none
//  Output:  y = year
//           m = month
//  Purpose: reads year & month from line of TD-3200 climate file.
//
{
    let mut recdType: [u8; 4] = [0; 4];
    let mut year: [u8; 5] = [0; 5];
    let mut month: [u8; 3] = [0; 3];

    // --- check for minimum number of characters
    if cio::strlen(&prj.climate.FileLine) < 30 {
        let name = prj.Fclimate.name;
        report_writeErrorMsg(prj, ERR_CLIMATE_FILE_READ, &name);
        return;
    }

    // --- check for proper type of record
    sstrncpy(&mut recdType, &prj.climate.FileLine, 3);
    if cio::strcmp(&recdType, b"DLY") != 0 {
        let name = prj.Fclimate.name;
        report_writeErrorMsg(prj, ERR_CLIMATE_FILE_READ, &name);
        return;
    }

    // --- get record's date
    sstrncpy(&mut year, &prj.climate.FileLine[17..], 4);
    sstrncpy(&mut month, &prj.climate.FileLine[21..], 2);
    *y = cio::atoi(&year);
    *m = cio::atoi(&month);
}

//=============================================================================

fn readDLY0204FileLine(prj: &mut Project, y: &mut i32, m: &mut i32)
//
//  Input:   none
//  Output:  y = year
//           m = month
//  Purpose: reads year & month from line of DLY02 or DLY04 climate file.
//
{
    let mut year: [u8; 5] = [0; 5];
    let mut month: [u8; 3] = [0; 3];

    // --- check for minimum number of characters
    if cio::strlen(&prj.climate.FileLine) < 16 {
        let name = prj.Fclimate.name;
        report_writeErrorMsg(prj, ERR_CLIMATE_FILE_READ, &name);
        return;
    }

    // --- get record's date
    sstrncpy(&mut year, &prj.climate.FileLine[7..], 4);
    sstrncpy(&mut month, &prj.climate.FileLine[11..], 2);
    *y = cio::atoi(&year);
    *m = cio::atoi(&month);
}

//=============================================================================

fn readFileValues(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: reads next month's worth of data from climate file.
//
{
    let mut i: i32;
    let mut j: i32;
    // (C leaves y and m uninitialized; as locals of this call they keep
    //  their values from one loop pass to the next, as here)
    let mut y: i32 = 0;
    let mut m: i32 = 0;

    // --- initialize FileData array to missing values
    i = 0;
    while i < MAXCLIMATEVARS {
        j = 0;
        while j < MAXDAYSPERMONTH {
            prj.climate.FileData[i as usize][j as usize] = MISSING;
            j += 1;
        }
        i += 1;
    }

    while prj.ErrorCode == 0 {
        // --- return when date on line is after current file date
        if prj.Fclimate.f().feof() {
            return;
        }
        readFileLine(prj, &mut y, &mut m);
        if y > prj.climate.FileYear || m > prj.climate.FileMonth {
            return;
        }

        // --- parse climate values from file line
        match prj.climate.FileFormat {
            USER_PREPARED => parseUserFileLine(prj),
            TD3200 => parseTD3200FileLine(prj),
            DLY0204 => parseDLY0204FileLine(prj),
            GHCND => parseGhcndFileLine(prj),
            _ => {}
        }
        sstrncpy(&mut prj.climate.FileLine, b"", 0);
    }
}

//=============================================================================

fn parseUserFileLine(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: parses climate variable values from a line of a user-prepared
//           climate file.
//
{
    let n: i32;
    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    let mut staID: Vec<u8> = Vec::new();
    let mut s0: Vec<u8> = Vec::new();
    let mut s1: Vec<u8> = Vec::new();
    let mut s2: Vec<u8> = Vec::new();
    let mut s3: Vec<u8> = Vec::new();
    let mut x: f64;

    // --- read day, Tmax, Tmin, Evap, & Wind from file line
    n = cio::sscanf(
        &prj.climate.FileLine,
        b"%s %d %d %d %s %s %s %s",
        &mut [
            Scan::Str(&mut staID),
            Scan::Int(&mut y),
            Scan::Int(&mut m),
            Scan::Int(&mut d),
            Scan::Str(&mut s0),
            Scan::Str(&mut s1),
            Scan::Str(&mut s2),
            Scan::Str(&mut s3),
        ],
    );
    if n < 4 {
        return;
    }
    if d < 1 || d > 31 {
        return;
    }

    // --- process TMAX
    if cio::strlen(&s0) > 0 && ch0(&s0) != b'*' {
        x = cio::atof(&s0);
        if prj.UnitSystem == SI {
            x = 9.0 / 5.0 * x + 32.0;
        }
        prj.climate.FileData[TMAX as usize][d as usize] = x;
    }

    // --- process TMIN
    if cio::strlen(&s1) > 0 && ch0(&s1) != b'*' {
        x = cio::atof(&s1);
        if prj.UnitSystem == SI {
            x = 9.0 / 5.0 * x + 32.0;
        }
        prj.climate.FileData[TMIN as usize][d as usize] = x;
    }

    // --- process EVAP
    if cio::strlen(&s2) > 0 && ch0(&s2) != b'*' {
        prj.climate.FileData[EVAP as usize][d as usize] = cio::atof(&s2);
    }

    // --- process WIND
    if cio::strlen(&s3) > 0 && ch0(&s3) != b'*' {
        prj.climate.FileData[WIND as usize][d as usize] = cio::atof(&s3);
    }
}

//=============================================================================

fn parseTD3200FileLine(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: parses climate variable values from a line of a TD3200 file.
//
{
    let mut i: i32;
    let mut param: [u8; 5] = [0; 5];

    // --- parse parameter name
    sstrncpy(&mut param, &prj.climate.FileLine[11..], 4);

    // --- see if parameter is temperature, evaporation or wind speed
    i = 0;
    while i < MAXCLIMATEVARS {
        if cio::strcmp(&param, ClimateVarWords[i as usize]) == 0 {
            setTD3200FileValues(prj, i);
        }
        i += 1;
    }
}

//=============================================================================

fn setTD3200FileValues(prj: &mut Project, i: i32)
//
//  Input:   i = climate variable code
//  Output:  none
//  Purpose: reads month worth of values for climate variable from TD-3200 file.
//
{
    let mut valCount: [u8; 4] = [0; 4];
    let mut day: [u8; 3] = [0; 3];
    let mut sign: [u8; 2] = [0; 2];
    let mut value: [u8; 6] = [0; 6];
    let mut flag2: [u8; 2] = [0; 2];
    let mut x: f64;
    let nValues: i32;
    let mut j: i32;
    let mut k: i32;
    let mut d: i32;

    // --- parse number of days with data from cols. 27-29 of file line
    sstrncpy(&mut valCount, &prj.climate.FileLine[27..], 3);
    nValues = cio::atoi(&valCount);

    // --- check for enough characters on line
    if cio::strlen(&prj.climate.FileLine) as i32 >= 12 * nValues + 30 {
        // --- for each day's value
        j = 0;
        while j < nValues {
            // --- parse day, value & flag from file line
            k = 30 + j * 12;
            sstrncpy(&mut day, &prj.climate.FileLine[k as usize..], 2);
            sstrncpy(&mut sign, &prj.climate.FileLine[(k + 4) as usize..], 1);
            sstrncpy(&mut value, &prj.climate.FileLine[(k + 5) as usize..], 5);
            sstrncpy(&mut flag2, &prj.climate.FileLine[(k + 11) as usize..], 1);

            // --- if value is valid then store it in FileData array
            d = cio::atoi(&day);
            if cio::strcmp(&value, b"99999") != 0
                && (flag2[0] == b'0' || flag2[0] == b'1')
                && d > 0
                && d <= 31
            {
                // --- convert from string value to numerical value
                x = cio::atof(&value);
                if sign[0] == b'-' {
                    x = -x;
                }

                // --- convert evaporation from hundreths of inches
                if i == EVAP {
                    x /= 100.0;

                    // --- convert to mm if using SI units
                    if prj.UnitSystem == SI {
                        x *= MMperINCH;
                    }
                }

                // --- convert wind speed from miles/day to miles/hour
                if i == WIND {
                    x /= 24.0;
                }

                // --- store value
                prj.climate.FileData[i as usize][d as usize] = x;
            }
            j += 1;
        }
    }
}

//=============================================================================

fn parseDLY0204FileLine(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: parses a month's worth of climate variable values from a line of
//           a DLY02 or DLY04 climate file.
//
{
    let mut j: i32;
    let mut k: i32;
    let mut p: i32;
    let mut param: [u8; 4] = [0; 4];
    let mut sign: [u8; 2] = [0; 2];
    let mut value: [u8; 6] = [0; 6];
    let mut code: [u8; 2] = [0; 2];
    let mut x: f64;

    // --- parse parameter name
    sstrncpy(&mut param, &prj.climate.FileLine[13..], 3);

    // --- see if parameter is min or max temperature
    p = cio::atoi(&param);
    if p == 1 {
        p = TMAX;
    } else if p == 2 {
        p = TMIN;
    } else if p == 151 {
        p = EVAP;
    } else {
        return;
    }

    // --- check for 233 characters on line
    if cio::strlen(&prj.climate.FileLine) < 233 {
        return;
    }

    // --- for each of 31 days
    k = 16;
    j = 1;
    while j <= 31 {
        // --- parse value & flag from file line
        sstrncpy(&mut sign, &prj.climate.FileLine[k as usize..], 1);
        sstrncpy(&mut value, &prj.climate.FileLine[(k + 1) as usize..], 5);
        sstrncpy(&mut code, &prj.climate.FileLine[(k + 6) as usize..], 1);
        k += 7;

        // --- if value is valid then store it in FileData array

        if cio::strcmp(&value, b"99999") != 0 && cio::strcmp(&value, b"     ") != 0 {
            match p {
                TMAX | TMIN => {
                    // --- convert from integer tenths of a degree C to degrees F
                    x = cio::atof(&value) / 10.0;
                    if sign[0] == b'-' {
                        x = -x;
                    }
                    x = 9.0 / 5.0 * x + 32.0;
                }
                EVAP => {
                    // --- convert from 0.1 mm to inches or mm
                    x = cio::atof(&value) / 10.0;
                    if prj.UnitSystem == US {
                        x /= MMperINCH;
                    }
                }
                _ => return,
            }
            prj.climate.FileData[p as usize][j as usize] = x;
        }
        j += 1;
    }
}

//=============================================================================

fn isGhcndFormat(prj: &mut Project, line: &[u8]) -> i32
//
//  Input:   line = first line of text from a climate file
//  Output:  returns TRUE if climate file is in NCDC GHCN Daily format.
//  Purpose: Checks if a climate file is in the NCDC GHCN Daily format
//           and determines the position of each climate variable field.
//
{
    let mut i: i32;
    let mut ptr: Option<usize>;
    let st = &mut prj.climate;

    // --- find starting position of the DATE field
    ptr = cio::strstr(line, b"DATE");
    match ptr {
        None => return FALSE,
        Some(p) => st.FileDateFieldPos = p as i32,
    }

    // --- initialize starting position of each data field
    i = TMIN;
    while i <= WIND {
        st.FileFieldPos[i as usize] = -1;
        i += 1;
    }

    // --- find starting position of each climate variable's data field
    ptr = cio::strstr(line, b"TMIN");
    if let Some(p) = ptr {
        st.FileFieldPos[TMIN as usize] = p as i32;
    }
    ptr = cio::strstr(line, b"TMAX");
    if let Some(p) = ptr {
        st.FileFieldPos[TMAX as usize] = p as i32;
    }
    ptr = cio::strstr(line, b"EVAP");
    if let Some(p) = ptr {
        st.FileFieldPos[EVAP as usize] = p as i32;
    }

    // --- WIND can either be daily movement or average speed
    st.FileWindType = WDMV;
    ptr = cio::strstr(line, b"WDMV");
    if ptr.is_none() {
        st.FileWindType = AWND;
        ptr = cio::strstr(line, b"AWND");
    }
    if let Some(p) = ptr {
        st.FileFieldPos[WIND as usize] = p as i32;
    }

    // --- check if at least one climate variable was found
    i = TMIN;
    while i <= WIND {
        if st.FileFieldPos[i as usize] >= 0 {
            return TRUE;
        }
        i += 1;
    }
    FALSE
}

//=============================================================================

fn readGhcndFileLine(prj: &mut Project, y: &mut i32, m: &mut i32)
//
//  Input:   none
//  Output:  y = year
//           m = month
//  Purpose: reads year & month from line of a NCDC GHCN Daily climate file.
//
{
    let st = &prj.climate;
    let n: i32 = cio::sscanf(
        &st.FileLine[st.FileDateFieldPos as usize..],
        b"%4d%2d",
        &mut [Scan::Int(y), Scan::Int(m)],
    );
    if n != 2 {
        *y = -99999;
        *m = -99999;
    }
}

//=============================================================================

fn parseGhcndFileLine(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: parses a line of a NCDC GHCN Daily file for daily
//           values of max/min temperature, pan evaporation and
//           wind speed.
//
{
    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    let n: i32;
    let mut i: i32;
    let mut v: f64 = 0.0;

    // --- parse day of month from date field
    {
        let st = &prj.climate;
        n = cio::sscanf(
            &st.FileLine[st.FileDateFieldPos as usize..],
            b"%4d%2d%2d",
            &mut [Scan::Int(&mut y), Scan::Int(&mut m), Scan::Int(&mut d)],
        );
    }
    if n < 3 {
        return;
    }
    if d < 1 || d > 31 {
        return;
    }

    // --- parse climate variables
    i = TMIN;
    while i <= WIND {
        if prj.climate.FileFieldPos[i as usize] >= 0 {
            let pos = prj.climate.FileFieldPos[i as usize] as usize;
            if cio::sscanf(&prj.climate.FileLine[pos..], b"%8lf", &mut [Scan::Double(&mut v)]) > 0 {
                if v.abs() < 9999.0 {
                    prj.climate.FileData[i as usize][d as usize] = convertGhcndValue(prj, i, v);
                }
            }
        }
        i += 1;
    }
}

//=============================================================================

fn convertGhcndValue(prj: &Project, var: i32, v: f64) -> f64
//
//  Input:   var = climate variable code
//           v = climate variable value
//  Output:  climate variable value in SWMM's internal units
//  Purpose: converts a climate variable value read from a NCDC GHCN Daily file
//           to SWMM's internal units.
//
{
    let mut v = v;
    match var {
        TMIN | TMAX => match prj.climate.FileTempUnits {
            DEG_C10 => {
                // tenths deg. C ==> deg. F
                v / 10.0 * 9.0 / 5.0 + 32.0
            }

            DEG_C => {
                // deg. C ==> deg. F
                v * 9.0 / 5.0 + 32.0
            }

            _ => {
                // deg. F
                v
            }
        },
        EVAP => match prj.climate.FileTempUnits {
            DEG_C10 => {
                // tenths mm ==> inches or mm
                v /= 10.0;
                if prj.UnitSystem == US {
                    v /= MMperINCH;
                }
                v
            }

            DEG_C => {
                // mm ==> inches or mm
                if prj.UnitSystem == US {
                    v /= MMperINCH;
                }
                v
            }

            _ => {
                // inches ==> inches or mm
                if prj.UnitSystem == SI {
                    v *= MMperINCH;
                }
                v
            }
        },
        WIND => match prj.climate.FileTempUnits {
            DEG_C10 => {
                // km/day ==> miles/hr
                if prj.climate.FileWindType == WDMV {
                    v * 0.62137 / 24.0
                }
                // tenths m/s ==> miles/hr
                else {
                    v / 10.0 / 1000.0 * 0.62137 * 3600.0
                }
            }
            DEG_C => {
                // km/day ==> miles/hr
                if prj.climate.FileWindType == WDMV {
                    v * 0.62137 / 24.0
                }
                // m/s ==> miles/hr
                else {
                    v / 1000.0 * 0.62137 * 3600.0
                }
            }

            _ => {
                // miles ==> miles/hr
                if prj.climate.FileWindType == WDMV {
                    v / 24.0
                }
                // miles/hr
                else {
                    v
                }
            }
        },
        _ => v,
    }
}

//=============================================================================

fn updateTempMoveAve(prj: &mut Project, tmin: f64, tmax: f64)
//
//  Input:   tmin = minimum daily temperature (deg F)
//           tmax = maximum daily temperature (deg F)
//  Output:  none
//  Purpose: updates moving averages of average daily temperature
//           and daily temperature range stored in structure Tma.
//
{
    let tma = &mut prj.climate.Tma;
    let ta: f64; // new day's average temperature (deg F)
    let tr: f64; // new day's temperature range (deg F)
    let kount: i32 = tma.count;
    let count: f64 = kount as f64;

    // --- find ta and tr from new day's min and max temperature
    ta = (tmin + tmax) / 2.0;
    tr = (tmax - tmin).abs();

    // --- if the array used to store previous days' temperatures is full
    if kount == tma.maxCount {
        // --- update the moving averages with the new day's value
        tma.tAve = (tma.tAve * count + ta - tma.ta[tma.front as usize]) / count;
        tma.tRng = (tma.tRng * count + tr - tma.tr[tma.front as usize]) / count;

        // --- replace the values at the front of the moving average window
        tma.ta[tma.front as usize] = ta;
        tma.tr[tma.front as usize] = tr;

        // --- move the front one position forward
        tma.front += 1;
        if tma.front as f64 == count {
            tma.front = 0;
        }
    }
    // --- array of previous day's values not full (at start of simulation)
    else {
        // --- find new moving averages by adding new values to previous ones
        tma.tAve = (tma.tAve * count + ta) / (count + 1.0);
        tma.tRng = (tma.tRng * count + tr) / (count + 1.0);

        // --- save new day's values
        tma.ta[tma.front as usize] = ta;
        tma.tr[tma.front as usize] = tr;

        // --- increment count and front of moving average window
        tma.count += 1;
        tma.front += 1;
        if tma.count == tma.maxCount {
            tma.front = 0;
        }
    }
}
