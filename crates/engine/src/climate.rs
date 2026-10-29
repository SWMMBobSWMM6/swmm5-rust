//! climate.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until climate.c is translated.

use crate::prelude::*;

/// The file-scope variables of climate.c.
#[derive(Default)]
pub struct ClimateState {
}

/// Called by: runoff.
pub fn climate_getNextEvapDate(prj: &mut Project) -> DateTime {
    todo!("climate_getNextEvapDate")
}

/// Called by: project.
pub fn climate_initState(prj: &mut Project) {
    todo!("climate_initState")
}

/// Called by: input.
pub fn climate_readAdjustments(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("climate_readAdjustments")
}

/// Called by: input.
pub fn climate_readEvapParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("climate_readEvapParams")
}

/// Called by: input.
pub fn climate_readParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("climate_readParams")
}

/// Called by: runoff, swmm5.
pub fn climate_setState(prj: &mut Project, theDate: DateTime) {
    todo!("climate_setState")
}

/// Called by: project.
pub fn climate_validate(prj: &mut Project) {
    todo!("climate_validate")
}
