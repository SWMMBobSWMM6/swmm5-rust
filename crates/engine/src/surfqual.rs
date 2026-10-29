//! surfqual.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until surfqual.c is translated.

use crate::prelude::*;

/// The file-scope variables of surfqual.c.
#[derive(Default)]
pub struct SurfqualState {
}

/// Called by: runoff.
pub fn surfqual_getBuildup(prj: &mut Project, j: i32, tStep: f64) {
    todo!("surfqual_getBuildup")
}

/// Called by: runoff.
pub fn surfqual_getWashoff(prj: &mut Project, j: i32, runoff: f64, tStep: f64) {
    todo!("surfqual_getWashoff")
}

/// Called by: routing.
pub fn surfqual_getWtdWashoff(prj: &Project, j: i32, p: i32, f: f64) -> f64 {
    todo!("surfqual_getWtdWashoff")
}

/// Called by: subcatch.
pub fn surfqual_initState(prj: &mut Project, j: i32) {
    todo!("surfqual_initState")
}

/// Called by: runoff.
pub fn surfqual_sweepBuildup(prj: &mut Project, j: i32, aDate: DateTime) {
    todo!("surfqual_sweepBuildup")
}
