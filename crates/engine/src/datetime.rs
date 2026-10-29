//! datetime.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   datetime.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21  (Build 5.2.0)
//   Author:   L. Rossman
//
//   DateTime functions.
//
//   Update History
//   ==============
//   Build 5.1.011:
//   - decodeTime() no longer rounds up.
//   - New getTimeStamp function added.
//-----------------------------------------------------------------------------
//
// Strings produced by this module (dateToStr, timeToStr, getTimeStamp) are
// returned in a Vec<u8> holding the characters C's snprintf would have
// written, without the terminator, truncated as snprintf truncates.

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const MonthTxt: [&[u8; 3]; 12] = [
    b"JAN", b"FEB", b"MAR", b"APR", //
    b"MAY", b"JUN", b"JUL", b"AUG", //
    b"SEP", b"OCT", b"NOV", b"DEC",
];
const DaysPerMonth: [[i32; 12]; 2] = // days per month
    [
        [31, 28, 31, 30, 31, 30, // normal years
         31, 31, 30, 31, 30, 31],
        [31, 29, 31, 30, 31, 30, // leap years
         31, 31, 30, 31, 30, 31],
    ];
const DateDelta: i32 = 693594; // days since 01/01/00
const SecsPerDay: f64 = 86400.0; // seconds per day

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of datetime.c.
#[derive(Default)]
pub struct DatetimeState {
    /// static int DateFormat;
    pub DateFormat: i32,
}

/// C `snprintf(s, size, ...)`: what lands in the buffer is the formatted
/// text cut to size-1 characters.
fn snprintf_into(s: &mut Vec<u8>, size: i32, text: Vec<u8>) {
    let mut text = text;
    let max = if size > 0 { (size - 1) as usize } else { 0 };
    if text.len() > max {
        text.truncate(max);
    }
    *s = text;
}

/// Byte i of a C string held in a slice (NUL past the end of the slice).
#[inline]
fn at(s: &[u8], i: usize) -> u8 {
    if i < s.len() {
        s[i]
    } else {
        0
    }
}

//=============================================================================

pub fn divMod(n: i32, d: i32, result: &mut i32, remainder: &mut i32) {
    //  Input:   n = numerator
    //           d = denominator
    //  Output:  result = integer part of n/d
    //           remainder = remainder of n/d
    //  Purpose: finds integer part and remainder of n/d.
    if d == 0 {
        *result = 0;
        *remainder = 0;
    } else {
        *result = n.wrapping_div(d);
        *remainder = n.wrapping_sub(d.wrapping_mul(*result));
    }
}

//=============================================================================

fn isLeapYear(year: i32) -> i32 {
    //  Input:   year = a year
    //  Output:  returns 1 if year is a leap year, 0 if not
    //  Purpose: determines if year is a leap year.
    if (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0)) {
        1
    } else {
        0
    }
}

//=============================================================================

pub fn datetime_findMonth(month: &[u8]) -> i32 {
    //  Input:   month = month of year as character string
    //  Output:  returns: month of year as a number (1-12)
    //  Purpose: finds number (1-12) of month.
    let mut i: i32 = 0;
    while i < 12 {
        let t = MonthTxt[i as usize];
        if UCHAR(at(month, 0)) == t[0] && UCHAR(at(month, 1)) == t[1] && UCHAR(at(month, 2)) == t[2] {
            return i + 1;
        }
        i += 1;
    }
    0
}

//=============================================================================

pub fn datetime_encodeDate(year: i32, month: i32, day: i32) -> DateTime {
    //  Input:   year = a year
    //           month = a month (1 to 12)
    //           day = a day of month
    //  Output:  returns encoded value of year-month-day
    //  Purpose: encodes year-month-day to a DateTime value.
    let mut day = day;
    let mut i: i32;
    let mut j: i32;
    i = isLeapYear(year);
    if (year >= 1) && (year <= 9999) && (month >= 1) && (month <= 12) && (day >= 1) && (day <= DaysPerMonth[i as usize][(month - 1) as usize]) {
        j = 0;
        while j < month - 1 {
            day += DaysPerMonth[i as usize][j as usize];
            j += 1;
        }
        i = year - 1;
        i = i * 365 + i / 4 - i / 100 + i / 400 + day - DateDelta;
        i as f64
    } else {
        -DateDelta as f64
    }
}

//=============================================================================

/// C: `DateTime datetime_encodeTime(int hour, int minute, int second)`.
/// `second` is an `int` in C; this signature (from the stub) takes it as an
/// f64, which callers fill from an int, so it converts back exactly.
pub fn datetime_encodeTime(hour: i32, minute: i32, second: f64) -> DateTime {
    //  Input:   hour = hour of day (0-24)
    //           minute = minute of hour (0-60)
    //           second = seconds of minute (0-60)
    //  Output:  returns time encoded as fractional part of a day
    //  Purpose: encodes hour:minute:second to a DateTime value
    let second: i32 = f64_to_i32(second);
    let s: i32;
    if (hour >= 0) && (minute >= 0) && (second >= 0) {
        s = hour.wrapping_mul(3600).wrapping_add(minute.wrapping_mul(60)).wrapping_add(second);
        s as f64 / SecsPerDay
    } else {
        0.0
    }
}

//=============================================================================

pub fn datetime_decodeDate(date: DateTime, year: &mut i32, month: &mut i32, day: &mut i32) {
    //  Input:   date = encoded date/time value
    //  Output:  year = 4-digit year
    //           month = month of year (1-12)
    //           day   = day of month
    //  Purpose: decodes DateTime value to year-month-day.
    let D1: i32;
    let D4: i32;
    let D100: i32;
    let D400: i32;
    let mut y: i32;
    let mut m: i32;
    let mut d: i32 = 0;
    let mut i: i32 = 0;
    let k: i32;
    let mut t: i32;

    D1 = 365; //365
    D4 = D1 * 4 + 1; //1461
    D100 = D4 * 25 - 1; //36524
    D400 = D100 * 4 + 1; //146097

    t = f64_to_i32(date.floor()).wrapping_add(DateDelta);
    if t <= 0 {
        *year = 0;
        *month = 1;
        *day = 1;
    } else {
        t -= 1;
        y = 1;
        while t >= D400 {
            t -= D400;
            y += 400;
        }
        divMod(t, D100, &mut i, &mut d);
        if i == 4 {
            i -= 1;
            d += D100;
        }
        y += i * 100;
        let dd = d;
        divMod(dd, D4, &mut i, &mut d);
        y += i * 4;
        let dd = d;
        divMod(dd, D1, &mut i, &mut d);
        if i == 4 {
            i -= 1;
            d += D1;
        }
        y += i;
        k = isLeapYear(y);
        m = 1;
        loop {
            i = DaysPerMonth[k as usize][(m - 1) as usize];
            if d < i {
                break;
            }
            d -= i;
            m += 1;
        }
        *year = y;
        *month = m;
        *day = d + 1;
    }
}

//=============================================================================

pub fn datetime_decodeTime(time: DateTime, h: &mut i32, m: &mut i32, s: &mut i32) {
    //  Input:   time = decimal fraction of a day
    //  Output:  h = hour of day (0-23)
    //           m = minute of hour (0-59)
    //           s = second of minute (0-59)
    //  Purpose: decodes DateTime value to hour:minute:second.
    let mut secs: i32;
    let mut mins: i32 = 0;
    let fracDay: f64 = (time - time.floor()) * SecsPerDay;
    secs = f64_to_i32((fracDay + 0.5).floor());
    if secs >= 86400 {
        secs = 86399;
    }
    divMod(secs, 60, &mut mins, s);
    divMod(mins, 60, h, m);
    if *h > 23 {
        *h = 0;
    }
}

//=============================================================================

/// datetime_dateToStr with the date format given explicitly (C reads the
/// file-scope DateFormat).
fn dateToStr(DateFormat: i32, date: DateTime, s: &mut Vec<u8>) {
    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    datetime_decodeDate(date, &mut y, &mut m, &mut d);
    match DateFormat {
        Y_M_D => {
            snprintf_into(s, DATE_STR_SIZE, cformat!(b"%4d-%3s-%02d", y, &MonthTxt[(m - 1) as usize][..], d));
        }
        M_D_Y => {
            //sprintf(dateStr, "%3s-%02d-%4d", MonthTxt[m-1], d, y);
            snprintf_into(s, DATE_STR_SIZE, cformat!(b"%02d/%02d/%04d", m, d, y));
        }
        _ => {
            snprintf_into(s, DATE_STR_SIZE, cformat!(b"%02d-%3s-%4d", d, &MonthTxt[(m - 1) as usize][..], y));
        }
    }
}

pub fn datetime_dateToStr(prj: &Project, date: DateTime, s: &mut Vec<u8>) {
    //  Input:   date = encoded date/time value
    //  Output:  s = formatted date string
    //  Purpose: represents DateTime date value as a formatted string.
    dateToStr(prj.datetime.DateFormat, date, s);
}

pub fn datetime_timeToStr(time: DateTime, s: &mut Vec<u8>) {
    //  Input:   time = decimal fraction of a day
    //  Output:  s = time in hr:min:sec format
    //  Purpose: represents DateTime time value as a formatted string.
    let mut hr: i32 = 0;
    let mut min: i32 = 0;
    let mut sec: i32 = 0;
    datetime_decodeTime(time, &mut hr, &mut min, &mut sec);
    snprintf_into(s, TIME_STR_SIZE, cformat!(b"%02d:%02d:%02d", hr, min, sec));
}

//=============================================================================

pub fn datetime_strToDate(prj: &Project, s: &[u8], d: &mut DateTime) -> i32 {
    //  Input:   s = date as string
    //  Output:  d = encoded date;
    //           returns 1 if conversion successful, 0 if not
    //  Purpose: converts string date s to DateTime value.
    strToDateFmt(prj.datetime.DateFormat, s, d)
}

/// The body of datetime_strToDate with DateFormat passed in, for table.c's
/// external time series files, whose readers are not given the project.
pub(crate) fn strToDateFmt(DateFormat: i32, s: &[u8], d: &mut DateTime) -> i32 {
    let mut yr: i32 = 0;
    let mut mon: i32 = 0;
    let mut day: i32 = 0;
    let mut n: i32;
    // NOTE: C leaves `char month[4]` uninitialized; it is read by
    // datetime_findMonth only when mon == 0, which can happen without the
    // %3s scan having filled it (a numeric month of 0, e.g. "0/1/2000").
    // C then reads stack garbage; here (as in the c2rust baseline) it is
    // zeros, which match no month, so encodeDate rejects the date.
    let mut month: [u8; 4] = [0; 4];
    let mut sep1: u8 = 0;
    let mut sep2: u8 = 0;
    let mut tok: Vec<u8> = Vec::new();
    *d = -DateDelta as f64;
    if cio::strchr(s, b'-').is_some() || cio::strchr(s, b'/').is_some() {
        match DateFormat {
            Y_M_D => {
                n = cio::sscanf(s, b"%d%c%d%c%d", &mut [Scan::Int(&mut yr), Scan::Char(&mut sep1), Scan::Int(&mut mon), Scan::Char(&mut sep2), Scan::Int(&mut day)]);
                if n < 3 {
                    mon = 0;
                    tok.clear();
                    n = cio::sscanf(s, b"%d%c%3s%c%d", &mut [Scan::Int(&mut yr), Scan::Char(&mut sep1), Scan::Str(&mut tok), Scan::Char(&mut sep2), Scan::Int(&mut day)]);
                    if n >= 3 {
                        put_month(&mut month, &tok);
                    }
                    if n < 3 {
                        return 0;
                    }
                }
            }

            D_M_Y => {
                n = cio::sscanf(s, b"%d%c%d%c%d", &mut [Scan::Int(&mut day), Scan::Char(&mut sep1), Scan::Int(&mut mon), Scan::Char(&mut sep2), Scan::Int(&mut yr)]);
                if n < 3 {
                    mon = 0;
                    tok.clear();
                    n = cio::sscanf(s, b"%d%c%3s%c%d", &mut [Scan::Int(&mut day), Scan::Char(&mut sep1), Scan::Str(&mut tok), Scan::Char(&mut sep2), Scan::Int(&mut yr)]);
                    if n >= 3 {
                        put_month(&mut month, &tok);
                    }
                    if n < 3 {
                        return 0;
                    }
                }
            }

            _ => {
                // M_D_Y
                n = cio::sscanf(s, b"%d%c%d%c%d", &mut [Scan::Int(&mut mon), Scan::Char(&mut sep1), Scan::Int(&mut day), Scan::Char(&mut sep2), Scan::Int(&mut yr)]);
                if n < 3 {
                    mon = 0;
                    tok.clear();
                    n = cio::sscanf(s, b"%3s%c%d%c%d", &mut [Scan::Str(&mut tok), Scan::Char(&mut sep1), Scan::Int(&mut day), Scan::Char(&mut sep2), Scan::Int(&mut yr)]);
                    if n >= 1 {
                        put_month(&mut month, &tok);
                    }
                    if n < 3 {
                        return 0;
                    }
                }
            }
        }
        if mon == 0 {
            mon = datetime_findMonth(&month);
        }
        *d = datetime_encodeDate(yr, mon, day);
    }
    if *d == -DateDelta as f64 {
        0
    } else {
        1
    }
}

/// What sscanf's `%3s` leaves in `char month[4]`: the token's bytes and a
/// terminator, later bytes untouched.
fn put_month(month: &mut [u8; 4], tok: &[u8]) {
    let n = tok.len().min(3);
    month[..n].copy_from_slice(&tok[..n]);
    month[n] = 0;
}

//=============================================================================

pub fn datetime_strToTime(s: &[u8], t: &mut DateTime) -> i32 {
    //  Input:   s = time as string
    //  Output:  t = encoded time,
    //           returns 1 if conversion successful, 0 if not
    //  Purpose: converts a string time to a DateTime value.
    //  Note:    accepts time as hr:min:sec or as decimal hours.
    let n: i32;
    // NOTE: C leaves hr uninitialized; it is read uninitialized only when
    // sscanf returns EOF (-1), i.e. for an all-blank string, which a token
    // never is. Here it starts at 0 (as in the c2rust baseline).
    let mut hr: i32 = 0;
    let mut min: i32 = 0;
    let mut sec: i32 = 0;

    // Attempt to read time as decimal hours
    let (v, endptr) = cio::strtod(s);
    *t = v;
    if at(s, endptr) == 0 {
        *t /= 24.0;
        return 1;
    }

    // Read time in hr:min:sec format
    *t = 0.0;
    n = cio::sscanf(s, b"%d:%d:%d", &mut [Scan::Int(&mut hr), Scan::Int(&mut min), Scan::Int(&mut sec)]);
    if n == 0 {
        return 0;
    }
    *t = datetime_encodeTime(hr, min, sec as f64);
    if (hr >= 0) && (min >= 0) && (sec >= 0) {
        1
    } else {
        0
    }
}

//=============================================================================

pub fn datetime_setDateFormat(prj: &mut Project, fmt: i32) {
    //  Input:   fmt = date format code
    //  Output:  none
    //  Purpose: sets date format
    if fmt >= Y_M_D && fmt <= M_D_Y {
        prj.datetime.DateFormat = fmt;
    }
}

//=============================================================================

pub fn datetime_addSeconds(date1: DateTime, seconds: f64) -> DateTime {
    //  Input:   date1 = an encoded date/time value
    //           seconds = number of seconds to add to date1
    //  Output:  returns updated value of date1
    //  Purpose: adds a given number of seconds to a date/time.
    let d: f64 = date1.floor();
    let mut h: i32 = 0;
    let mut m: i32 = 0;
    let mut s: i32 = 0;
    datetime_decodeTime(date1, &mut h, &mut m, &mut s);
    d + (3600.0 * h as f64 + 60.0 * m as f64 + s as f64 + seconds) / SecsPerDay
}

//=============================================================================

pub fn datetime_addDays(date1: DateTime, date2: DateTime) -> DateTime {
    //  Input:   date1 = an encoded date/time value
    //           date2 = decimal days to be added to date1
    //  Output:  returns date1 + date2
    //  Purpose: adds a given number of decimal days to a date/time.
    let d1: f64 = date1.floor();
    let d2: f64 = date2.floor();
    let mut h1: i32 = 0;
    let mut m1: i32 = 0;
    let mut s1: i32 = 0;
    let mut h2: i32 = 0;
    let mut m2: i32 = 0;
    let mut s2: i32 = 0;
    datetime_decodeTime(date1, &mut h1, &mut m1, &mut s1);
    datetime_decodeTime(date2, &mut h2, &mut m2, &mut s2);
    d1 + d2 + datetime_encodeTime(h1 + h2, m1 + m2, (s1 + s2) as f64)
}

//=============================================================================

pub fn datetime_timeDiff(date1: DateTime, date2: DateTime) -> i64 {
    //  Input:   date1 = an encoded date/time value
    //           date2 = an encoded date/time value
    //  Output:  returns date1 - date2 in seconds
    //  Purpose: finds number of seconds between two dates.
    let d1: f64 = date1.floor();
    let d2: f64 = date2.floor();
    let mut h: i32 = 0;
    let mut m: i32 = 0;
    let mut s: i32 = 0;
    let s1: i64;
    let s2: i64;
    let mut secs: i64;
    datetime_decodeTime(date1, &mut h, &mut m, &mut s);
    s1 = (3600 * h + 60 * m + s) as i64;
    datetime_decodeTime(date2, &mut h, &mut m, &mut s);
    s2 = (3600 * h + 60 * m + s) as i64;
    secs = f64_to_i32(((d1 - d2) * SecsPerDay + 0.5).floor()) as i64;
    secs += s1 - s2;
    secs
}

//=============================================================================

pub fn datetime_monthOfYear(date: DateTime) -> i32 {
    //  Input:   date = an encoded date/time value
    //  Output:  returns index of month of year (1..12)
    //  Purpose: finds month of year (Jan = 1 ...) for a given date.
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    datetime_decodeDate(date, &mut year, &mut month, &mut day);
    month
}

//=============================================================================

pub fn datetime_dayOfYear(date: DateTime) -> i32 {
    //  Input:   date = an encoded date/time value
    //  Output:  returns day of year (1..365)
    //  Purpose: finds day of year (Jan 1 = 1) for a given date.
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let startOfYear: DateTime;
    datetime_decodeDate(date, &mut year, &mut month, &mut day);
    startOfYear = datetime_encodeDate(year, 1, 1);
    f64_to_i32((date - startOfYear).floor()).wrapping_add(1)
}

//=============================================================================

pub fn datetime_dayOfWeek(date: DateTime) -> i32 {
    //  Input:   date = an encoded date/time value
    //  Output:  returns index of day of week (1..7)
    //  Purpose: finds day of week (Sun = 1, ... Sat = 7) for a given date.
    let t: i32 = f64_to_i32(date.floor()).wrapping_add(DateDelta);
    (t % 7) + 1
}

//=============================================================================

pub fn datetime_hourOfDay(date: DateTime) -> i32 {
    //  Input:   date = an encoded date/time value
    //  Output:  returns hour of day (0..23)
    //  Purpose: finds hour of day (0 = 12 AM, ..., 23 = 11 PM) for a given date.
    let mut hour: i32 = 0;
    let mut min: i32 = 0;
    let mut sec: i32 = 0;
    datetime_decodeTime(date, &mut hour, &mut min, &mut sec);
    hour
}

//=============================================================================

pub fn datetime_daysPerMonth(year: i32, month: i32) -> i32 {
    //  Input:   year = year in which month falls
    //           month = month of year (1..12)
    //  Output:  returns number of days in the month
    //  Purpose: finds number of days in a given month of a specified year.
    if month < 1 || month > 12 {
        return 0;
    }
    DaysPerMonth[isLeapYear(year) as usize][(month - 1) as usize]
}

//=============================================================================

pub fn datetime_getTimeStamp(prj: &Project, fmt: i32, aDate: DateTime, stampSize: i32, timeStamp: &mut Vec<u8>) {
    //  Input:   fmt = desired date format code
    //           aDate = a date/time value in decimal days
    //           stampSize = the number of bytes allocated for the time stamp
    //  Output:  returns a time stamp string (e.g., Year-Month-Day Hr:Min:Sec)
    //  Purpose: Expresses a decimal day date by a time stamp.
    let mut dateStr: Vec<u8> = Vec::new();
    let mut timeStr: Vec<u8> = Vec::new();
    let oldDateFormat: i32 = prj.datetime.DateFormat;

    if stampSize < TIME_STAMP_SIZE {
        return;
    }
    // C: datetime_setDateFormat(fmt); datetime_dateToStr(aDate, dateStr);
    //    DateFormat = oldDateFormat;
    let mut DateFormat = oldDateFormat;
    if fmt >= Y_M_D && fmt <= M_D_Y {
        DateFormat = fmt;
    }
    dateToStr(DateFormat, aDate, &mut dateStr);
    datetime_timeToStr(aDate, &mut timeStr);
    snprintf_into(timeStamp, stampSize, cformat!(b"%s %s", dateStr, timeStr));
}
