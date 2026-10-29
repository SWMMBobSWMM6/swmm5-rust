//! lid.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until lid.c is translated.

use crate::prelude::*;

/// The file-scope variables of lid.c.
#[derive(Default)]
pub struct LidState {
}

/// Called by: routing.
pub fn lid_addDrainInflow(prj: &mut Project, j: i32, f: f64) {
    todo!("lid_addDrainInflow")
}

/// Called by: surfqual.
pub fn lid_addDrainLoads(prj: &Project, j: i32, c: &mut [f64], tStep: f64) {
    todo!("lid_addDrainLoads")
}

/// Called by: subcatch.
pub fn lid_addDrainRunon(prj: &mut Project, j: i32) {
    todo!("lid_addDrainRunon")
}

/// Called by: project.
pub fn lid_create(prj: &mut Project, lidCount: i32, subcatchCount: i32) {
    todo!("lid_create")
}

/// Called by: project.
pub fn lid_delete(prj: &mut Project) {
    todo!("lid_delete")
}

/// Called by: subcatch.
pub fn lid_getDrainFlow(prj: &Project, j: i32, timePeriod: i32) -> f64 {
    todo!("lid_getDrainFlow")
}

/// Called by: subcatch.
pub fn lid_getFlowToPerv(prj: &Project, j: i32) -> f64 {
    todo!("lid_getFlowToPerv")
}

/// Called by: subcatch.
pub fn lid_getPervArea(prj: &Project, j: i32) -> f64 {
    todo!("lid_getPervArea")
}

/// Called by: subcatch.
pub fn lid_getRunoff(prj: &mut Project, j: i32, tStep: f64) {
    todo!("lid_getRunoff")
}

/// Called by: subcatch.
pub fn lid_getStoredVolume(prj: &Project, j: i32) -> f64 {
    todo!("lid_getStoredVolume")
}

/// Called by: project.
pub fn lid_initState(prj: &mut Project) {
    todo!("lid_initState")
}

/// Called by: input.
pub fn lid_readGroupParams(prj: &mut Project, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("lid_readGroupParams")
}

/// Called by: input.
pub fn lid_readProcParams(prj: &mut Project, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("lid_readProcParams")
}

/// Called by: subcatch.
pub fn lid_setOldGroupState(prj: &mut Project, j: i32) {
    todo!("lid_setOldGroupState")
}

/// Called by: project.
pub fn lid_validate(prj: &mut Project) {
    todo!("lid_validate")
}

/// Called by: inputrpt.
pub fn lid_writeSummary(prj: &mut Project) {
    todo!("lid_writeSummary")
}

/// Called by: statsrpt.
pub fn lid_writeWaterBalance(prj: &mut Project) {
    todo!("lid_writeWaterBalance")
}
