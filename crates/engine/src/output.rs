//! output.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until output.c is translated.

use crate::prelude::*;

/// The file-scope variables of output.c.
#[derive(Default)]
pub struct OutputState {
    pub SubcatchResults: Vec<f32>,
    pub NodeResults: Vec<f32>,
    pub LinkResults: Vec<f32>,
}

/// Called by: swmm5.
pub fn output_close(prj: &mut Project) {
    todo!("output_close")
}

/// Called by: swmm5.
pub fn output_end(prj: &mut Project) {
    todo!("output_end")
}

/// Called by: swmm5.
pub fn output_open(prj: &mut Project) -> i32 {
    todo!("output_open")
}

/// Called by: report, swmm5.
pub fn output_readDateTime(prj: &mut Project, period: i64, days: &mut DateTime) {
    todo!("output_readDateTime")
}

/// Called by: report, swmm5.
pub fn output_readLinkResults(prj: &mut Project, period: i64, index: i32) {
    todo!("output_readLinkResults")
}

/// Called by: report, swmm5.
pub fn output_readNodeResults(prj: &mut Project, period: i64, index: i32) {
    todo!("output_readNodeResults")
}

/// Called by: report, swmm5.
pub fn output_readSubcatchResults(prj: &mut Project, period: i64, index: i32) {
    todo!("output_readSubcatchResults")
}

/// Called by: swmm5.
pub fn output_saveResults(prj: &mut Project, reportTime: f64) {
    todo!("output_saveResults")
}

/// Called by: swmm5.
pub fn output_updateAvgResults(prj: &mut Project) {
    todo!("output_updateAvgResults")
}
