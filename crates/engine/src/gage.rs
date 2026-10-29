//! gage.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   gage.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21 (Build 5.2.0)
//   Author:   L. Rossman
//
//   Rain gage functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Support for monthly rainfall adjustments added.
//   Build 5.1.013:
//   - Validation no longer performed on unused gages.
//   Build 5.2.0:
//   - Support added for tracking a gage's prior n-hour rainfall total.
//   - Support added for relative file names.
//   - Support added for setting rainfall through API call.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const OneSecond: f64 = 1.1574074e-5;

/// The file-scope variables of gage.c (it has none).
#[derive(Default)]
pub struct GageState {}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  gage_readParams        (called by input_readLine)
//  gage_validate          (called by project_validate)
//  gage_initState         (called by project_init)
//  gage_setState          (called by runoff_execute & getRainfall in rdii.c)
//  gage_getPrecip         (called by subcatch_getRunoff)
//  gage_getNextRainDate   (called by runoff_getTimeStep)
//  gage_updatePastRain    (called by runoff_execute)
//  gage_getPastRain       (called by getRainValue in controls.c)
//  gage_setReportRainfall (called by output_saveSubcatchResults)

//-----------------------------------------------------------------------------
//  Helpers for C library behaviour
//-----------------------------------------------------------------------------

/// `fread(&v, sizeof(double), 1, f)`: the bytes read land in `v` even when
/// the item is incomplete, as glibc copies them.
fn fread_f64(f: &mut CFile, v: &mut f64) -> usize {
    let mut b = v.to_le_bytes();
    let n = f.read_bytes(&mut b);
    *v = f64::from_le_bytes(b);
    n / 8
}

/// `fread(&v, sizeof(float), 1, f)`, as `fread_f64`.
fn fread_f32(f: &mut CFile, v: &mut f32) -> usize {
    let mut b = v.to_le_bytes();
    let n = f.read_bytes(&mut b);
    *v = f32::from_le_bytes(b);
    n / 4
}

/// First character of a C string (`*s`), 0 for an empty string.
#[inline]
fn ch0(s: &[u8]) -> u8 {
    s.first().copied().unwrap_or(0)
}

//=============================================================================

pub fn gage_readParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = rain gage index
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads rain gage parameters from a line of input data
//
//  Data formats are:
//    Name RainType RecdFreq SCF TIMESERIES SeriesName
//    Name RainType RecdFreq SCF FILE FileName Station Units StartDate
//
{
    let k: i32;
    let err: i32;
    let id: Option<Vec<u8>>;
    let mut fname: [u8; MAXFNAME as usize + 1] = [0; MAXFNAME as usize + 1];
    let mut staID: [u8; MAXMSG as usize + 1] = [0; MAXMSG as usize + 1];
    let mut x: [f64; 7] = [0.0; 7];

    // --- check that gage exists
    if ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    id = project_findID(prj, GAGE, &tok[0]);
    if id.is_none() {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- assign default parameter values
    x[0] = -1.0; // No time series index
    x[1] = 1.0; // Rain type is volume
    x[2] = 3600.0; // Recording freq. is 3600 sec
    x[3] = 1.0; // Snow catch deficiency factor
    x[4] = NO_DATE as f64; // Default is no start/end date
    x[5] = NO_DATE as f64;
    x[6] = 0.0; // US units
    fname[0] = 0;
    staID[0] = 0;

    if ntoks < 5 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    k = findmatch(&tok[4], GageDataWords);
    if k == RAIN_TSERIES {
        err = readGageSeriesFormat(prj, tok, ntoks, &mut x);
    } else if k == RAIN_FILE {
        if ntoks < 8 {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        sstrncpy(&mut fname, &tok[5], MAXFNAME as usize);
        sstrncpy(&mut staID, &tok[6], MAXMSG as usize);
        err = readGageFileFormat(prj, tok, ntoks, &mut x);
    } else {
        return error_setInpError(prj, ERR_KEYWORD, &tok[4]);
    }

    // --- save parameters to rain gage object
    if err > 0 {
        return err;
    }
    let ju = j as usize;
    prj.Gage[ju].ID = id.unwrap_or_default();
    prj.Gage[ju].tSeries = f64_to_i32(x[0]);
    prj.Gage[ju].rainType = f64_to_i32(x[1]);
    prj.Gage[ju].rainInterval = f64_to_i32(x[2]);
    prj.Gage[ju].snowFactor = x[3];
    prj.Gage[ju].rainUnits = f64_to_i32(x[6]);
    if prj.Gage[ju].tSeries >= 0 {
        prj.Gage[ju].dataSource = RAIN_TSERIES;
    } else {
        prj.Gage[ju].dataSource = RAIN_FILE;
    }
    if prj.Gage[ju].dataSource == RAIN_FILE {
        addAbsolutePath(prj, &mut fname);
        sstrncpy(&mut prj.Gage[ju].fname, &fname, MAXFNAME as usize);
        sstrncpy(&mut prj.Gage[ju].staID, &staID, MAXMSG as usize);
        prj.Gage[ju].startFileDate = x[4];
        prj.Gage[ju].endFileDate = x[5];
    }
    prj.Gage[ju].unitsFactor = 1.0;
    prj.Gage[ju].coGage = -1;
    prj.Gage[ju].isUsed = FALSE;
    0
}

//=============================================================================

fn readGageSeriesFormat(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32, x: &mut [f64]) -> i32 {
    let m: i32;
    let ts: i32;
    let mut aTime: DateTime = 0.0;

    if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- determine type of rain data
    m = findmatch(&tok[1], RainTypeWords);
    if m < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }
    x[1] = m as f64;

    // --- get data time interval & convert to seconds
    if getDouble(&tok[2], &mut x[2]) != 0 {
        x[2] = (x[2] * 3600.0 + 0.5).floor();
    } else if datetime_strToTime(&tok[2], &mut aTime) != 0 {
        x[2] = (aTime * SECperDAY as f64 + 0.5).floor();
    } else {
        return error_setInpError(prj, ERR_DATETIME, &tok[2]);
    }
    if x[2] <= 0.0 {
        return error_setInpError(prj, ERR_DATETIME, &tok[2]);
    }

    // --- get snow catch deficiency factor
    if getDouble(&tok[3], &mut x[3]) == 0 {
        return error_setInpError(prj, ERR_DATETIME, &tok[3]);
    }

    // --- get time series index
    ts = project_findObject(prj, TSERIES, &tok[5]);
    if ts < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[5]);
    }
    x[0] = ts as f64;
    // C: sstrncpy(tok[2], "", 0) empties the token in place. Nothing reads
    // tok[2] after this (input.c re-tokenizes every line), and tok is
    // read-only here, so the write is omitted.
    0
}

//=============================================================================

fn readGageFileFormat(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32, x: &mut [f64]) -> i32 {
    let m: i32;
    let u: i32;
    let mut aDate: DateTime = 0.0;
    let mut aTime: DateTime = 0.0;

    // --- determine type of rain data
    m = findmatch(&tok[1], RainTypeWords);
    if m < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }
    x[1] = m as f64;

    // --- get data time interval & convert to seconds
    if getDouble(&tok[2], &mut x[2]) != 0 {
        x[2] *= 3600.0;
    } else if datetime_strToTime(&tok[2], &mut aTime) != 0 {
        x[2] = (aTime * SECperDAY as f64 + 0.5).floor();
    } else {
        return error_setInpError(prj, ERR_DATETIME, &tok[2]);
    }
    if x[2] <= 0.0 {
        return error_setInpError(prj, ERR_DATETIME, &tok[2]);
    }

    // --- get snow catch deficiency factor
    if getDouble(&tok[3], &mut x[3]) == 0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[3]);
    }

    // --- get rain depth units
    u = findmatch(&tok[7], RainUnitsWords);
    if u < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[7]);
    }
    x[6] = u as f64;

    // --- get start date (if present)
    if ntoks > 8 && ch0(&tok[8]) != b'*' {
        if datetime_strToDate(prj, &tok[8], &mut aDate) == 0 {
            return error_setInpError(prj, ERR_DATETIME, &tok[8]);
        }
        x[4] = aDate as f32 as f64;
    }
    0
}

//=============================================================================

pub fn gage_validate(prj: &mut Project, j: i32)
//
//  Input:   j = rain gage index
//  Output:  none
//  Purpose: checks for valid rain gage parameters
//
//  NOTE: assumes that any time series used by a rain gage has been
//        previously validated.
//
{
    let mut i: i32;
    let k: i32;
    let gageInterval: i32;
    let ju = j as usize;

    // --- for gage with time series data:
    if prj.Gage[ju].dataSource == RAIN_TSERIES {
        // --- no validation for an unused gage
        if prj.Gage[ju].isUsed == 0 {
            return;
        }

        // --- see if gage uses same time series as another gage
        k = prj.Gage[ju].tSeries;
        i = 0;
        while i < j {
            let iu = i as usize;
            if prj.Gage[iu].dataSource == RAIN_TSERIES && prj.Gage[iu].tSeries == k && prj.Gage[iu].isUsed != 0 {
                prj.Gage[ju].coGage = i;

                // --- check that both gages record same type of data
                if prj.Gage[ju].rainType != prj.Gage[iu].rainType {
                    let id = prj.Gage[ju].ID.clone();
                    report_writeErrorMsg(prj, ERR_RAIN_GAGE_FORMAT, &id);
                }
                return;
            }
            i += 1;
        }

        // --- check gage's recording interval against that of time series
        if prj.Tseries[k as usize].refersTo >= 0 {
            let id = prj.Gage[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_RAIN_GAGE_TSERIES, &id);
        }
        gageInterval = f64_to_i32((prj.Tseries[k as usize].dxMin * SECperDAY as f64 + 0.5).floor());
        if gageInterval > 0 && prj.Gage[ju].rainInterval > gageInterval {
            let id = prj.Gage[ju].ID.clone();
            report_writeErrorMsg(prj, ERR_RAIN_GAGE_INTERVAL, &id);
        }
        if prj.Gage[ju].rainInterval < gageInterval {
            let id = prj.Gage[ju].ID.clone();
            report_writeWarningMsg(prj, WARN09, &id);
        }
        if prj.Gage[ju].rainInterval < prj.WetStep {
            let id = prj.Gage[ju].ID.clone();
            report_writeWarningMsg(prj, WARN01, &id);
            prj.WetStep = prj.Gage[ju].rainInterval;
        }
    }
}

//=============================================================================

pub fn gage_initState(prj: &mut Project, j: i32)
//
//  Input:   j = rain gage index
//  Output:  none
//  Purpose: initializes state of rain gage.
//
{
    let ju = j as usize;

    // --- initialize actual and reported rainfall
    prj.Gage[ju].rainfall = 0.0;
    prj.Gage[ju].apiRainfall = MISSING;
    prj.Gage[ju].reportRainfall = 0.0;
    if prj.IgnoreRainfall != 0 {
        return;
    }

    // --- for gage with file data:
    if prj.Gage[ju].dataSource == RAIN_FILE {
        // --- set current file position to start of period of record
        prj.Gage[ju].currentFilePos = prj.Gage[ju].startFilePos;

        // --- assign units conversion factor
        //     (rain depths on interface file are in inches)
        if prj.UnitSystem == SI {
            prj.Gage[ju].unitsFactor = MMperINCH;
        }
    }

    // --- get first & next rainfall values
    if getFirstRainfall(prj, j) != 0 {
        // --- find date at end of starting rain interval
        prj.Gage[ju].endDate = datetime_addSeconds(prj.Gage[ju].startDate, prj.Gage[ju].rainInterval as f64);

        // --- if rainfall record begins after start of simulation,
        if prj.Gage[ju].startDate > prj.StartDateTime {
            // --- make next rainfall date the start of the rain record
            prj.Gage[ju].nextDate = prj.Gage[ju].startDate;
            prj.Gage[ju].nextRainfall = prj.Gage[ju].rainfall;

            // --- make start of current rain interval the simulation start
            prj.Gage[ju].startDate = prj.StartDateTime;
            prj.Gage[ju].endDate = prj.Gage[ju].nextDate;
            prj.Gage[ju].rainfall = 0.0;
        }
        // --- otherwise find next recorded rainfall
        else if getNextRainfall(prj, j) == 0 {
            prj.Gage[ju].nextDate = NO_DATE as f64;
        }
    } else {
        prj.Gage[ju].startDate = NO_DATE as f64;
    }
    initPastRain(prj, j);
}

//=============================================================================

pub fn gage_setState(prj: &mut Project, j: i32, t: DateTime)
//
//  Input:   j = rain gage index
//           t = a calendar date/time
//  Output:  none
//  Purpose: updates state of rain gage for specified date.
//
{
    let mut t = t;
    let ju = j as usize;

    // --- return if gage not used by any subcatchment
    if prj.Gage[ju].isUsed == FALSE {
        return;
    }

    // --- set rainfall to zero if disabled
    if prj.IgnoreRainfall != 0 {
        prj.Gage[ju].rainfall = 0.0;
        return;
    }

    // --- use rainfall from co-gage (gage with lower index that uses
    //     same rainfall time series or file) if it exists
    if prj.Gage[ju].coGage >= 0 {
        prj.Gage[ju].rainfall = prj.Gage[prj.Gage[ju].coGage as usize].rainfall;
        return;
    }

    // --- use rainfall supplied by API function call
    //     (where constant ZERO (1.e-10) is used for 0 rainfall)
    if prj.Gage[ju].apiRainfall != MISSING {
        prj.Gage[ju].rainfall = prj.Gage[ju].apiRainfall;
        return;
    }

    // --- otherwise march through rainfall record until date t is bracketed
    t += OneSecond;
    loop {
        // --- no rainfall if no interval start date
        if prj.Gage[ju].startDate == NO_DATE as f64 {
            prj.Gage[ju].rainfall = 0.0;
            return;
        }

        // --- no rainfall if time is before interval start date
        if t < prj.Gage[ju].startDate {
            prj.Gage[ju].rainfall = 0.0;
            return;
        }

        // --- use current rainfall if time is before interval end date
        if t < prj.Gage[ju].endDate {
            return;
        }

        // --- no rainfall if t >= interval end date & no next interval exists
        if prj.Gage[ju].nextDate == NO_DATE as f64 {
            prj.Gage[ju].rainfall = 0.0;
            return;
        }

        // --- no rainfall if t > interval end date & <  next interval date
        if t < prj.Gage[ju].nextDate {
            prj.Gage[ju].rainfall = 0.0;
            return;
        }

        // --- otherwise update next rainfall interval date
        prj.Gage[ju].startDate = prj.Gage[ju].nextDate;
        prj.Gage[ju].endDate = datetime_addSeconds(prj.Gage[ju].startDate, prj.Gage[ju].rainInterval as f64);
        prj.Gage[ju].rainfall = prj.Gage[ju].nextRainfall;
        if getNextRainfall(prj, j) == 0 {
            prj.Gage[ju].nextDate = NO_DATE as f64;
        }
    }
}

//=============================================================================

fn initPastRain(prj: &mut Project, j: i32) {
    // --- initialize past hourly rain accumulation
    let mut i: i32;
    i = 0;
    while i <= MAXPASTRAIN {
        prj.Gage[j as usize].pastRain[i as usize] = 0.0;
        i += 1;
    }
    prj.Gage[j as usize].pastInterval = 0;
}

//=============================================================================

pub fn gage_updatePastRain(prj: &mut Project, j: i32, tStep: i32)
//
//  Input:   j = rain gage index
//           tStep = current runoff time step (sec)
//  Output:  none
//  Purpose: updates past MAXPASTRAIN hourly rain totals.
//
//  Note: pastRain[0] is past rain volume prior to 1 hour,
//        pastRain[n] is past rain volume after n hours,
//        pastInterval is time since last hour was reached.
{
    let mut tStep = tStep;
    let mut i: i32;
    let mut t: i32;
    let r: f64;
    let g = &mut prj.Gage[j as usize];

    // --- current rainfall intensity (in/sec or mm/sec)
    r = g.rainfall / 3600.0;

    // --- process each hourly interval of current time step
    while tStep > 0 {
        // --- time for most recent rainfall interval to reach 1 hr
        t = 3600 - g.pastInterval;

        // --- remaining time step is greater than this time
        if tStep > t {
            // --- add current rain to most recent interval
            g.pastRain[0] += t as f64 * r;

            // --- shift all prior hourly rain amounts by 1 hour
            i = MAXPASTRAIN;
            while i > 0 {
                g.pastRain[i as usize] = g.pastRain[(i - 1) as usize];
                i -= 1;
            }

            // --- begin a new most recent interval
            g.pastInterval = 0;
            g.pastRain[0] = 0.0;
            tStep -= t;
        }
        // --- time to reach 1 hr in most recent interval is greater
        //     than remaining time step so update most recent interval
        else {
            g.pastRain[0] += tStep as f64 * r;
            g.pastInterval += tStep;
            tStep = 0;
        }
    }
}

//=============================================================================

pub fn gage_getPastRain(prj: &Project, j: i32, n: i32) -> f64
//
//  Input:   j = rain gage index
//           n = number of hours prior to current date
//  Output:  cumulative rain volume (inches or mm) in last n hours
//  Purpose: retrieves rainfall total over some previous number of hours.
//
{
    let mut i: i32;
    let mut result: f64 = 0.0;
    if n < 1 || n > MAXPASTRAIN {
        return 0.0;
    }
    i = 1;
    while i <= n {
        result += prj.Gage[j as usize].pastRain[i as usize];
        i += 1;
    }
    result
}

//=============================================================================

pub fn gage_getNextRainDate(prj: &mut Project, j: i32, aDate: DateTime) -> DateTime
//
//  Input:   j = rain gage index
//           aDate = calendar date/time
//  Output:  next date with rainfall occurring
//  Purpose: finds the next date from  specified date when rainfall occurs.
//
{
    let mut aDate = aDate;
    let g = &prj.Gage[j as usize];
    if g.isUsed == FALSE {
        return aDate;
    }
    aDate += OneSecond;
    if aDate < g.startDate {
        return g.startDate;
    }
    if aDate < g.endDate {
        return g.endDate;
    }
    g.nextDate
}

//=============================================================================

pub fn gage_getPrecip(prj: &mut Project, j: i32, rainfall: &mut f64, snowfall: &mut f64) -> f64
//
//  Input:   j = rain gage index
//  Output:  rainfall = rainfall rate (ft/sec)
//           snowfall = snow fall rate (ft/sec)
//           returns total precipitation (ft/sec)
//  Purpose: determines whether gage's recorded rainfall is rain or snow.
//
{
    *rainfall = 0.0;
    *snowfall = 0.0;
    if prj.IgnoreSnowmelt == 0 && prj.Temp.ta <= prj.Snow.snotmp {
        *snowfall = prj.Gage[j as usize].rainfall * prj.Gage[j as usize].snowFactor / UCF(prj, RAINFALL);
    } else {
        *rainfall = prj.Gage[j as usize].rainfall / UCF(prj, RAINFALL);
    }
    (*rainfall) + (*snowfall)
}

//=============================================================================

pub fn gage_setReportRainfall(prj: &mut Project, j: i32, reportDate: DateTime)
//
//  Input:   j = rain gage index
//           reportDate = date/time value of current reporting time
//  Output:  none
//  Purpose: sets the rainfall value reported at the current reporting time.
//
{
    let mut reportDate = reportDate;
    let result: f64;
    let ju = j as usize;

    // --- use value from co-gage if it exists
    if prj.Gage[ju].coGage >= 0 {
        prj.Gage[ju].reportRainfall = prj.Gage[prj.Gage[ju].coGage as usize].reportRainfall;
        return;
    }

    // --- rainfall set by API call
    if prj.Gage[ju].apiRainfall != MISSING {
        prj.Gage[ju].reportRainfall = prj.Gage[ju].apiRainfall;
        return;
    }

    // --- otherwise increase reporting time by 1 second to avoid
    //     roundoff problems
    reportDate += OneSecond;

    // --- use current rainfall if report date/time is before end
    //     of current rain interval
    if reportDate < prj.Gage[ju].endDate {
        result = prj.Gage[ju].rainfall;
    }
    // --- use 0.0 if report date/time is before start of next rain interval
    else if reportDate < prj.Gage[ju].nextDate {
        result = 0.0;
    }
    // --- otherwise report date/time falls right on end of current rain
    //     interval and start of next interval so use next interval's rainfall
    else {
        result = prj.Gage[ju].nextRainfall;
    }
    prj.Gage[ju].reportRainfall = result;
}

//=============================================================================

fn getFirstRainfall(prj: &mut Project, j: i32) -> i32
//
//  Input:   j = rain gage index
//  Output:  returns TRUE if successful
//  Purpose: positions rainfall record to date with first rainfall.
//
{
    let k: i32; // time series index
    // (C leaves vFirst uninitialized; it keeps that value only if the
    //  fread below gets nothing, which a valid interface file prevents)
    let mut vFirst: f32 = 0.0; // first rain volume (ft or m)
    let mut rFirst: f64 = 0.0; // first rain intensity (in/hr or mm/hr)
    let ju = j as usize;

    // --- assign default values to date & rainfall
    prj.Gage[ju].startDate = NO_DATE as f64;
    prj.Gage[ju].rainfall = 0.0;

    // --- initialize internal cumulative rainfall value
    prj.Gage[ju].rainAccum = 0.0;

    // --- use rain interface file if applicable
    if prj.Gage[ju].dataSource == RAIN_FILE {
        if prj.Frain.file.is_some() && prj.Gage[ju].endFilePos > prj.Gage[ju].startFilePos {
            // --- retrieve 1st date & rainfall volume from file
            let f = prj.Frain.f();
            f.fseek_set(prj.Gage[ju].startFilePos);
            fread_f64(f, &mut prj.Gage[ju].startDate);
            fread_f32(f, &mut vFirst);
            prj.Gage[ju].currentFilePos = f.ftell();

            // --- convert rainfall to intensity
            prj.Gage[ju].rainfall = convertRainfall(prj, j, vFirst as f64);
            return 1;
        }
        0
    }
    // --- otherwise access user-supplied rainfall time series
    else {
        k = prj.Gage[ju].tSeries;
        if k >= 0 {
            // --- retrieve first rainfall value from time series
            if table_getFirstEntry(&mut prj.Tseries[k as usize], &mut prj.Gage[ju].startDate, &mut rFirst) != 0 {
                // --- convert rainfall to intensity
                prj.Gage[ju].rainfall = convertRainfall(prj, j, rFirst);
                return 1;
            }
        }
        0
    }
}

//=============================================================================

fn getNextRainfall(prj: &mut Project, j: i32) -> i32
//
//  Input:   j = rain gage index
//  Output:  returns 1 if successful; 0 if not
//  Purpose: positions rainfall record to date with next non-zero rainfall
//           while updating the gage's next rain intensity value.
//
//  Note: zero rainfall values explicitly entered into a rain file or
//        time series are skipped over so that a proper accounting of
//        wet and dry periods can be maintained.
//
{
    let mut k: i32; // time series index
    // (C leaves vNext uninitialized; as in getFirstRainfall)
    let mut vNext: f32 = 0.0; // next rain volume (ft or m)
    let mut rNext: f64 = 0.0; // next rain intensity (in/hr or mm/hr)
    let ju = j as usize;

    prj.Gage[ju].nextRainfall = 0.0;
    loop {
        if prj.Gage[ju].dataSource == RAIN_FILE {
            if prj.Frain.file.is_some() && prj.Gage[ju].currentFilePos < prj.Gage[ju].endFilePos {
                let f = prj.Frain.f();
                f.fseek_set(prj.Gage[ju].currentFilePos);
                fread_f64(f, &mut prj.Gage[ju].nextDate);
                fread_f32(f, &mut vNext);
                prj.Gage[ju].currentFilePos = f.ftell();
                rNext = convertRainfall(prj, j, vNext as f64);
            } else {
                return 0;
            }
        } else {
            k = prj.Gage[ju].tSeries;
            if k >= 0 {
                if table_getNextEntry(&mut prj.Tseries[k as usize], &mut prj.Gage[ju].nextDate, &mut rNext) == 0 {
                    return 0;
                }
                rNext = convertRainfall(prj, j, rNext);
            } else {
                return 0;
            }
        }
        if !(rNext == 0.0) {
            break;
        }
    }
    prj.Gage[ju].nextRainfall = rNext;
    1
}

//=============================================================================

fn convertRainfall(prj: &mut Project, j: i32, r: f64) -> f64
//
//  Input:   j = rain gage index
//           r = rainfall value (user units)
//  Output:  returns rainfall intensity (user units)
//  Purpose: converts rainfall value to an intensity (depth per hour).
//
{
    let r1: f64;
    let g = &mut prj.Gage[j as usize];
    match g.rainType {
        RAINFALL_INTENSITY => {
            r1 = r;
        }

        RAINFALL_VOLUME => {
            r1 = r / g.rainInterval as f64 * 3600.0;
        }

        CUMULATIVE_RAINFALL => {
            if r < g.rainAccum {
                r1 = r / g.rainInterval as f64 * 3600.0;
            } else {
                r1 = (r - g.rainAccum) / g.rainInterval as f64 * 3600.0;
            }
            g.rainAccum = r;
        }

        _ => {
            r1 = r;
        }
    }
    r1 * g.unitsFactor * prj.Adjust.rainFactor
}

//=============================================================================
