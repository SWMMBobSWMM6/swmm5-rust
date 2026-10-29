//! gage.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until gage.c is translated.

use crate::prelude::*;

/// The file-scope variables of gage.c.
#[derive(Default)]
pub struct GageState {
}

/// Called by: runoff.
pub fn gage_getNextRainDate(prj: &mut Project, j: i32, aDate: DateTime) -> DateTime {
    todo!("gage_getNextRainDate")
}

/// Called by: controls.
pub fn gage_getPastRain(prj: &Project, j: i32, n: i32) -> f64 {
    todo!("gage_getPastRain")
}

/// Called by: snow, subcatch.
pub fn gage_getPrecip(prj: &mut Project, j: i32, rainfall: &mut f64, snowfall: &mut f64) -> f64 {
    todo!("gage_getPrecip")
}

/// Called by: project, rdii.
pub fn gage_initState(prj: &mut Project, j: i32) {
    todo!("gage_initState")
}

/// Called by: input.
pub fn gage_readParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("gage_readParams")
}

/// Called by: output.
pub fn gage_setReportRainfall(prj: &mut Project, j: i32, reportDate: DateTime) {
    todo!("gage_setReportRainfall")
}

/// Called by: rdii, runoff.
pub fn gage_setState(prj: &mut Project, j: i32, t: DateTime) {
    todo!("gage_setState")
}

/// Called by: runoff.
pub fn gage_updatePastRain(prj: &mut Project, j: i32, tStep: i32) {
    todo!("gage_updatePastRain")
}

/// Called by: project.
pub fn gage_validate(prj: &mut Project, j: i32) {
    todo!("gage_validate")
}
