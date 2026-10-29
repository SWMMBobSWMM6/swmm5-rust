//! datetime.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until datetime.c is translated.

use crate::prelude::*;

/// The file-scope variables of datetime.c.
#[derive(Default)]
pub struct DatetimeState {
}

/// Called by: (entry point).
pub fn datetime_addDays(date1: DateTime, date2: DateTime) -> DateTime {
    todo!("datetime_addDays")
}

/// Called by: gage, rain, rdii, swmm5.
pub fn datetime_addSeconds(date1: DateTime, seconds: f64) -> DateTime {
    todo!("datetime_addSeconds")
}

/// Called by: report.
pub fn datetime_dateToStr(prj: &Project, date: DateTime, s: &mut Vec<u8>) {
    todo!("datetime_dateToStr")
}

/// Called by: controls, inflow, routing, swmm5.
pub fn datetime_dayOfWeek(date: DateTime) -> i32 {
    todo!("datetime_dayOfWeek")
}

/// Called by: climate, controls, project, runoff.
pub fn datetime_dayOfYear(date: DateTime) -> i32 {
    todo!("datetime_dayOfYear")
}

/// Called by: climate.
pub fn datetime_daysPerMonth(year: i32, month: i32) -> i32 {
    todo!("datetime_daysPerMonth")
}

/// Called by: climate, iface, swmm5.
pub fn datetime_decodeDate(date: DateTime, year: &mut i32, month: &mut i32, day: &mut i32) {
    todo!("datetime_decodeDate")
}

/// Called by: iface, project, swmm5.
pub fn datetime_decodeTime(time: DateTime, h: &mut i32, m: &mut i32, s: &mut i32) {
    todo!("datetime_decodeTime")
}

/// Called by: climate, iface, project, rain, rdii.
pub fn datetime_encodeDate(year: i32, month: i32, day: i32) -> DateTime {
    todo!("datetime_encodeDate")
}

/// Called by: iface, project, rain, rdii, report.
pub fn datetime_encodeTime(hour: i32, minute: i32, second: f64) -> DateTime {
    todo!("datetime_encodeTime")
}

/// Called by: rdii.
pub fn datetime_findMonth(month: &[u8]) -> i32 {
    todo!("datetime_findMonth")
}

/// Called by: lidproc.
pub fn datetime_getTimeStamp(prj: &Project, fmt: i32, aDate: DateTime, stampSize: i32, timeStamp: &mut Vec<u8>) {
    todo!("datetime_getTimeStamp")
}

/// Called by: inflow, routing.
pub fn datetime_hourOfDay(date: DateTime) -> i32 {
    todo!("datetime_hourOfDay")
}

/// Called by: climate, controls, gwater, infil, inflow, rdii, routing, subcatch.
pub fn datetime_monthOfYear(date: DateTime) -> i32 {
    todo!("datetime_monthOfYear")
}

/// Called by: swmm5.
pub fn datetime_setDateFormat(prj: &mut Project, fmt: i32) {
    todo!("datetime_setDateFormat")
}

/// Called by: climate, controls, gage, input, project, table.
pub fn datetime_strToDate(prj: &Project, s: &[u8], d: &mut DateTime) -> i32 {
    todo!("datetime_strToDate")
}

/// Called by: controls, gage, input, project, table.
pub fn datetime_strToTime(s: &[u8], t: &mut DateTime) -> i32 {
    todo!("datetime_strToTime")
}

/// Called by: rain, runoff.
pub fn datetime_timeDiff(date1: DateTime, date2: DateTime) -> i64 {
    todo!("datetime_timeDiff")
}

/// Called by: report.
pub fn datetime_timeToStr(time: DateTime, s: &mut Vec<u8>) {
    todo!("datetime_timeToStr")
}

/// Called by: (entry point).
pub fn divMod(n: i32, d: i32, result: &mut i32, remainder: &mut i32) {
    todo!("divMod")
}
