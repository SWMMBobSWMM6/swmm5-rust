//! inlet.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until inlet.c is translated.

use crate::prelude::*;

/// The file-scope variables of inlet.c.
#[derive(Default)]
pub struct InletState {
}

/// Called by: routing.
pub fn inlet_adjustQualInflows(prj: &mut Project) {
    todo!("inlet_adjustQualInflows")
}

/// Called by: routing.
pub fn inlet_adjustQualOutflows(prj: &mut Project) {
    todo!("inlet_adjustQualOutflows")
}

/// Called by: qualrout.
pub fn inlet_capturedFlow(prj: &Project, i: i32) -> f64 {
    todo!("inlet_capturedFlow")
}

/// Called by: project.
pub fn inlet_create(prj: &mut Project, numInlets: i32) -> i32 {
    todo!("inlet_create")
}

/// Called by: project.
pub fn inlet_delete(prj: &mut Project) {
    todo!("inlet_delete")
}

/// Called by: routing.
pub fn inlet_findCapturedFlows(prj: &mut Project, tStep: f64) {
    todo!("inlet_findCapturedFlows")
}

/// Called by: input.
pub fn inlet_readDesignParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("inlet_readDesignParams")
}

/// Called by: input.
pub fn inlet_readUsageParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("inlet_readUsageParams")
}

/// Called by: project.
pub fn inlet_validate(prj: &mut Project) {
    todo!("inlet_validate")
}

/// Called by: statsrpt.
pub fn inlet_writeStatsReport(prj: &mut Project) {
    todo!("inlet_writeStatsReport")
}
