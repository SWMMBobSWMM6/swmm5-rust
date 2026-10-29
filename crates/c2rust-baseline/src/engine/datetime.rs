#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type DateTime = ::core::ffi::c_double;
pub const Y_M_D: ::core::ffi::c_int = 0;
pub const M_D_Y: ::core::ffi::c_int = 1;
pub const D_M_Y: ::core::ffi::c_int = 2;
pub const DATE_STR_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const TIME_STR_SIZE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const TIME_STAMP_SIZE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
static mut MonthTxt: [*const ::core::ffi::c_char; 12] = [
    b"JAN\0" as *const u8 as *const ::core::ffi::c_char,
    b"FEB\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAR\0" as *const u8 as *const ::core::ffi::c_char,
    b"APR\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAY\0" as *const u8 as *const ::core::ffi::c_char,
    b"JUN\0" as *const u8 as *const ::core::ffi::c_char,
    b"JUL\0" as *const u8 as *const ::core::ffi::c_char,
    b"AUG\0" as *const u8 as *const ::core::ffi::c_char,
    b"SEP\0" as *const u8 as *const ::core::ffi::c_char,
    b"OCT\0" as *const u8 as *const ::core::ffi::c_char,
    b"NOV\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEC\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut DaysPerMonth: [[::core::ffi::c_int; 12]; 2] = [
    [
        31 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
    ],
    [
        31 as ::core::ffi::c_int,
        29 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
    ],
];
static mut DateDelta: ::core::ffi::c_int = 693594 as ::core::ffi::c_int;
static mut SecsPerDay: ::core::ffi::c_double = 86400.0f64;
static mut DateFormat: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn divMod(
    mut n: ::core::ffi::c_int,
    mut d: ::core::ffi::c_int,
    mut result: *mut ::core::ffi::c_int,
    mut remainder: *mut ::core::ffi::c_int,
) {
    if d == 0 as ::core::ffi::c_int {
        *result = 0 as ::core::ffi::c_int;
        *remainder = 0 as ::core::ffi::c_int;
    } else {
        *result = n / d;
        *remainder = n - d * *result;
    };
}
#[no_mangle]
pub unsafe extern "C" fn isLeapYear(mut year: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if year % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (year % 100 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || year % 400 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
    {
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn datetime_findMonth(
    mut month: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        if (if *month.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'a' as i32
            && *month.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'z' as i32
        {
            *month.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & !(32 as ::core::ffi::c_int)
        } else {
            *month.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        }) == *MonthTxt[i as usize].offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            && (if *month.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'a' as i32
                && *month.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'z' as i32
            {
                *month.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & !(32 as ::core::ffi::c_int)
            } else {
                *month.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            }) == *MonthTxt[i as usize].offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
            && (if *month.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'a' as i32
                && *month.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'z' as i32
            {
                *month.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & !(32 as ::core::ffi::c_int)
            } else {
                *month.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            }) == *MonthTxt[i as usize].offset(2 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
        {
            return i + 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn datetime_encodeDate(
    mut year: ::core::ffi::c_int,
    mut month: ::core::ffi::c_int,
    mut day: ::core::ffi::c_int,
) -> DateTime {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = isLeapYear(year);
    if year >= 1 as ::core::ffi::c_int
        && year <= 9999 as ::core::ffi::c_int
        && month >= 1 as ::core::ffi::c_int
        && month <= 12 as ::core::ffi::c_int
        && day >= 1 as ::core::ffi::c_int
        && day <= DaysPerMonth[i as usize][(month - 1 as ::core::ffi::c_int) as usize]
    {
        j = 0 as ::core::ffi::c_int;
        while j < month - 1 as ::core::ffi::c_int {
            day += DaysPerMonth[i as usize][j as usize];
            j += 1;
        }
        i = year - 1 as ::core::ffi::c_int;
        i = i * 365 as ::core::ffi::c_int + i / 4 as ::core::ffi::c_int
            - i / 100 as ::core::ffi::c_int
            + i / 400 as ::core::ffi::c_int
            + day
            - DateDelta;
        return i as DateTime;
    } else {
        return -DateDelta as DateTime;
    };
}
#[no_mangle]
pub unsafe extern "C" fn datetime_encodeTime(
    mut hour: ::core::ffi::c_int,
    mut minute: ::core::ffi::c_int,
    mut second: ::core::ffi::c_int,
) -> DateTime {
    let mut s: ::core::ffi::c_int = 0;
    if hour >= 0 as ::core::ffi::c_int
        && minute >= 0 as ::core::ffi::c_int
        && second >= 0 as ::core::ffi::c_int
    {
        s = hour * 3600 as ::core::ffi::c_int + minute * 60 as ::core::ffi::c_int + second;
        return s as DateTime / SecsPerDay as DateTime;
    } else {
        return 0.0f64;
    };
}
#[no_mangle]
pub unsafe extern "C" fn datetime_decodeDate(
    mut date: DateTime,
    mut year: *mut ::core::ffi::c_int,
    mut month: *mut ::core::ffi::c_int,
    mut day: *mut ::core::ffi::c_int,
) {
    let mut D1: ::core::ffi::c_int = 0;
    let mut D4: ::core::ffi::c_int = 0;
    let mut D100: ::core::ffi::c_int = 0;
    let mut D400: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_int = 0;
    D1 = 365 as ::core::ffi::c_int;
    D4 = D1 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    D100 = D4 * 25 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D400 = D100 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    t = crate::ccast::f64_to_i32(floor(date as ::core::ffi::c_double)) + DateDelta;
    if t <= 0 as ::core::ffi::c_int {
        *year = 0 as ::core::ffi::c_int;
        *month = 1 as ::core::ffi::c_int;
        *day = 1 as ::core::ffi::c_int;
    } else {
        t -= 1;
        y = 1 as ::core::ffi::c_int;
        while t >= D400 {
            t -= D400;
            y += 400 as ::core::ffi::c_int;
        }
        divMod(t, D100, &raw mut i, &raw mut d);
        if i == 4 as ::core::ffi::c_int {
            i -= 1;
            d += D100;
        }
        y += i * 100 as ::core::ffi::c_int;
        divMod(d, D4, &raw mut i, &raw mut d);
        y += i * 4 as ::core::ffi::c_int;
        divMod(d, D1, &raw mut i, &raw mut d);
        if i == 4 as ::core::ffi::c_int {
            i -= 1;
            d += D1;
        }
        y += i;
        k = isLeapYear(y);
        m = 1 as ::core::ffi::c_int;
        loop {
            i = DaysPerMonth[k as usize][(m - 1 as ::core::ffi::c_int) as usize];
            if d < i {
                break;
            }
            d -= i;
            m += 1;
        }
        *year = y;
        *month = m;
        *day = d + 1 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn datetime_decodeTime(
    mut time: DateTime,
    mut h: *mut ::core::ffi::c_int,
    mut m: *mut ::core::ffi::c_int,
    mut s: *mut ::core::ffi::c_int,
) {
    let mut secs: ::core::ffi::c_int = 0;
    let mut mins: ::core::ffi::c_int = 0;
    let mut fracDay: ::core::ffi::c_double =
        (time as ::core::ffi::c_double - floor(time as ::core::ffi::c_double)) * SecsPerDay;
    secs = crate::ccast::f64_to_i32(floor(fracDay + 0.5f64));
    if secs >= 86400 as ::core::ffi::c_int {
        secs = 86399 as ::core::ffi::c_int;
    }
    divMod(secs, 60 as ::core::ffi::c_int, &raw mut mins, s);
    divMod(mins, 60 as ::core::ffi::c_int, h, m);
    if *h > 23 as ::core::ffi::c_int {
        *h = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn datetime_dateToStr(mut date: DateTime, mut s: *mut ::core::ffi::c_char) {
    let mut y: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    datetime_decodeDate(date, &raw mut y, &raw mut m, &raw mut d);
    match DateFormat {
        Y_M_D => {
            snprintf(
                s,
                DATE_STR_SIZE as size_t,
                b"%4d-%3s-%02d\0" as *const u8 as *const ::core::ffi::c_char,
                y,
                MonthTxt[(m - 1 as ::core::ffi::c_int) as usize],
                d,
            );
        }
        M_D_Y => {
            snprintf(
                s,
                DATE_STR_SIZE as size_t,
                b"%02d/%02d/%04d\0" as *const u8 as *const ::core::ffi::c_char,
                m,
                d,
                y,
            );
        }
        _ => {
            snprintf(
                s,
                DATE_STR_SIZE as size_t,
                b"%02d-%3s-%4d\0" as *const u8 as *const ::core::ffi::c_char,
                d,
                MonthTxt[(m - 1 as ::core::ffi::c_int) as usize],
                y,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn datetime_timeToStr(mut time: DateTime, mut s: *mut ::core::ffi::c_char) {
    let mut hr: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut sec: ::core::ffi::c_int = 0;
    datetime_decodeTime(time, &raw mut hr, &raw mut min, &raw mut sec);
    snprintf(
        s,
        TIME_STR_SIZE as size_t,
        b"%02d:%02d:%02d\0" as *const u8 as *const ::core::ffi::c_char,
        hr,
        min,
        sec,
    );
}
#[no_mangle]
pub unsafe extern "C" fn datetime_strToDate(
    mut s: *mut ::core::ffi::c_char,
    mut d: *mut DateTime,
) -> ::core::ffi::c_int {
    let mut yr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut day: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    let mut month: [::core::ffi::c_char; 4] = [0; 4];
    let mut sep1: ::core::ffi::c_char = 0;
    let mut sep2: ::core::ffi::c_char = 0;
    *d = -DateDelta as DateTime;
    if !strchr(s, '-' as i32).is_null() || !strchr(s, '/' as i32).is_null() {
        match DateFormat {
            Y_M_D => {
                n = sscanf(
                    s,
                    b"%d%c%d%c%d\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut yr,
                    &raw mut sep1,
                    &raw mut mon,
                    &raw mut sep2,
                    &raw mut day,
                );
                if n < 3 as ::core::ffi::c_int {
                    mon = 0 as ::core::ffi::c_int;
                    n = sscanf(
                        s,
                        b"%d%c%3s%c%d\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut yr,
                        &raw mut sep1,
                        &raw mut month as *mut ::core::ffi::c_char,
                        &raw mut sep2,
                        &raw mut day,
                    );
                    if n < 3 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
            D_M_Y => {
                n = sscanf(
                    s,
                    b"%d%c%d%c%d\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut day,
                    &raw mut sep1,
                    &raw mut mon,
                    &raw mut sep2,
                    &raw mut yr,
                );
                if n < 3 as ::core::ffi::c_int {
                    mon = 0 as ::core::ffi::c_int;
                    n = sscanf(
                        s,
                        b"%d%c%3s%c%d\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut day,
                        &raw mut sep1,
                        &raw mut month as *mut ::core::ffi::c_char,
                        &raw mut sep2,
                        &raw mut yr,
                    );
                    if n < 3 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
            _ => {
                n = sscanf(
                    s,
                    b"%d%c%d%c%d\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut mon,
                    &raw mut sep1,
                    &raw mut day,
                    &raw mut sep2,
                    &raw mut yr,
                );
                if n < 3 as ::core::ffi::c_int {
                    mon = 0 as ::core::ffi::c_int;
                    n = sscanf(
                        s,
                        b"%3s%c%d%c%d\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut month as *mut ::core::ffi::c_char,
                        &raw mut sep1,
                        &raw mut day,
                        &raw mut sep2,
                        &raw mut yr,
                    );
                    if n < 3 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        if mon == 0 as ::core::ffi::c_int {
            mon = datetime_findMonth(&raw mut month as *mut ::core::ffi::c_char);
        }
        *d = datetime_encodeDate(yr, mon, day);
    }
    if *d == -DateDelta as DateTime {
        return 0 as ::core::ffi::c_int;
    } else {
        return 1 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn datetime_strToTime(
    mut s: *mut ::core::ffi::c_char,
    mut t: *mut DateTime,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut hr: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sec: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *t = strtod(s, &raw mut endptr) as DateTime;
    if *endptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        *t /= 24.0f64;
        return 1 as ::core::ffi::c_int;
    }
    *t = 0.0f64 as DateTime;
    n = sscanf(
        s,
        b"%d:%d:%d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut hr,
        &raw mut min,
        &raw mut sec,
    );
    if n == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    *t = datetime_encodeTime(hr, min, sec);
    if hr >= 0 as ::core::ffi::c_int
        && min >= 0 as ::core::ffi::c_int
        && sec >= 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn datetime_setDateFormat(mut fmt: ::core::ffi::c_int) {
    if fmt >= Y_M_D && fmt <= M_D_Y {
        DateFormat = fmt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn datetime_addSeconds(
    mut date1: DateTime,
    mut seconds: ::core::ffi::c_double,
) -> DateTime {
    let mut d: ::core::ffi::c_double = floor(date1 as ::core::ffi::c_double);
    let mut h: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    datetime_decodeTime(date1, &raw mut h, &raw mut m, &raw mut s);
    return d as DateTime
        + (3600.0f64 * h as DateTime
            + 60.0f64 * m as DateTime
            + s as DateTime
            + seconds as DateTime)
            / SecsPerDay as DateTime;
}
#[no_mangle]
pub unsafe extern "C" fn datetime_addDays(mut date1: DateTime, mut date2: DateTime) -> DateTime {
    let mut d1: ::core::ffi::c_double = floor(date1 as ::core::ffi::c_double);
    let mut d2: ::core::ffi::c_double = floor(date2 as ::core::ffi::c_double);
    let mut h1: ::core::ffi::c_int = 0;
    let mut m1: ::core::ffi::c_int = 0;
    let mut s1: ::core::ffi::c_int = 0;
    let mut h2: ::core::ffi::c_int = 0;
    let mut m2: ::core::ffi::c_int = 0;
    let mut s2: ::core::ffi::c_int = 0;
    datetime_decodeTime(date1, &raw mut h1, &raw mut m1, &raw mut s1);
    datetime_decodeTime(date2, &raw mut h2, &raw mut m2, &raw mut s2);
    return d1 as DateTime + d2 as DateTime + datetime_encodeTime(h1 + h2, m1 + m2, s1 + s2);
}
#[no_mangle]
pub unsafe extern "C" fn datetime_timeDiff(
    mut date1: DateTime,
    mut date2: DateTime,
) -> ::core::ffi::c_long {
    let mut d1: ::core::ffi::c_double = floor(date1 as ::core::ffi::c_double);
    let mut d2: ::core::ffi::c_double = floor(date2 as ::core::ffi::c_double);
    let mut h: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut s1: ::core::ffi::c_long = 0;
    let mut s2: ::core::ffi::c_long = 0;
    let mut secs: ::core::ffi::c_long = 0;
    datetime_decodeTime(date1, &raw mut h, &raw mut m, &raw mut s);
    s1 = (3600 as ::core::ffi::c_int * h + 60 as ::core::ffi::c_int * m + s) as ::core::ffi::c_long;
    datetime_decodeTime(date2, &raw mut h, &raw mut m, &raw mut s);
    s2 = (3600 as ::core::ffi::c_int * h + 60 as ::core::ffi::c_int * m + s) as ::core::ffi::c_long;
    secs = crate::ccast::f64_to_i32(floor((d1 - d2) * SecsPerDay + 0.5f64)) as ::core::ffi::c_long;
    secs += s1 - s2;
    return secs;
}
#[no_mangle]
pub unsafe extern "C" fn datetime_monthOfYear(mut date: DateTime) -> ::core::ffi::c_int {
    let mut year: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    datetime_decodeDate(date, &raw mut year, &raw mut month, &raw mut day);
    return month;
}
#[no_mangle]
pub unsafe extern "C" fn datetime_dayOfYear(mut date: DateTime) -> ::core::ffi::c_int {
    let mut year: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut startOfYear: DateTime = 0.;
    datetime_decodeDate(date, &raw mut year, &raw mut month, &raw mut day);
    startOfYear = datetime_encodeDate(year, 1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
    return crate::ccast::f64_to_i32(floor(date as ::core::ffi::c_double - startOfYear as ::core::ffi::c_double))
        + 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn datetime_dayOfWeek(mut date: DateTime) -> ::core::ffi::c_int {
    let mut t: ::core::ffi::c_int =
        crate::ccast::f64_to_i32(floor(date as ::core::ffi::c_double)) + DateDelta;
    return t % 7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn datetime_hourOfDay(mut date: DateTime) -> ::core::ffi::c_int {
    let mut hour: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut sec: ::core::ffi::c_int = 0;
    datetime_decodeTime(date, &raw mut hour, &raw mut min, &raw mut sec);
    return hour;
}
#[no_mangle]
pub unsafe extern "C" fn datetime_daysPerMonth(
    mut year: ::core::ffi::c_int,
    mut month: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if month < 1 as ::core::ffi::c_int || month > 12 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return DaysPerMonth[isLeapYear(year) as usize][(month - 1 as ::core::ffi::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn datetime_getTimeStamp(
    mut fmt: ::core::ffi::c_int,
    mut aDate: DateTime,
    mut stampSize: ::core::ffi::c_int,
    mut timeStamp: *mut ::core::ffi::c_char,
) {
    let mut dateStr: [::core::ffi::c_char; 12] = [0; 12];
    let mut timeStr: [::core::ffi::c_char; 9] = [0; 9];
    let mut oldDateFormat: ::core::ffi::c_int = DateFormat;
    if stampSize < TIME_STAMP_SIZE {
        return;
    }
    datetime_setDateFormat(fmt);
    datetime_dateToStr(aDate, &raw mut dateStr as *mut ::core::ffi::c_char);
    DateFormat = oldDateFormat;
    datetime_timeToStr(aDate, &raw mut timeStr as *mut ::core::ffi::c_char);
    snprintf(
        timeStamp,
        stampSize as size_t,
        b"%s %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut dateStr as *mut ::core::ffi::c_char,
        &raw mut timeStr as *mut ::core::ffi::c_char,
    );
}
