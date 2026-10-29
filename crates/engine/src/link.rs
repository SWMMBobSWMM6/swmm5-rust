//! link.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until link.c is translated.

use crate::prelude::*;

/// The file-scope variables of link.c.
#[derive(Default)]
pub struct LinkState {
}

/// Called by: dwflow.
pub fn link_getFroude(prj: &Project, j: i32, v: f64, y: f64) -> f64 {
    todo!("link_getFroude")
}

/// Called by: dwflow, kinwave.
pub fn link_getFullState(a1: f64, a2: f64, aFull: f64) -> i8 {
    todo!("link_getFullState")
}

/// Called by: dynwave, flowrout.
pub fn link_getInflow(prj: &mut Project, j: i32) -> f64 {
    todo!("link_getInflow")
}

/// Called by: dwflow, dynwave, flowrout, kinwave.
pub fn link_getLength(prj: &Project, j: i32) -> f64 {
    todo!("link_getLength")
}

/// Called by: dwflow, flowrout, kinwave.
pub fn link_getLossRate(prj: &mut Project, j: i32, routeModel: i32, q: f64, tstep: f64) -> f64 {
    todo!("link_getLossRate")
}

/// Called by: stats.
pub fn link_getPower(prj: &Project, j: i32) -> f64 {
    todo!("link_getPower")
}

/// Called by: output.
pub fn link_getResults(prj: &mut Project, j: i32, wt: f64, x: &mut [f32]) {
    todo!("link_getResults")
}

/// Called by: controls, stats, swmm5.
pub fn link_getVelocity(prj: &Project, j: i32, flow: f64, depth: f64) -> f64 {
    todo!("link_getVelocity")
}

/// Called by: dwflow.
pub fn link_getYcrit(prj: &Project, j: i32, q: f64) -> f64 {
    todo!("link_getYcrit")
}

/// Called by: dwflow.
pub fn link_getYnorm(prj: &Project, j: i32, q: f64) -> f64 {
    todo!("link_getYnorm")
}

/// Called by: project.
pub fn link_initState(prj: &mut Project, j: i32) {
    todo!("link_initState")
}

/// Called by: input.
pub fn link_readLossParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("link_readLossParams")
}

/// Called by: input.
pub fn link_readParams(prj: &mut Project, j: i32, type_: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("link_readParams")
}

/// Called by: input.
pub fn link_readXsectParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("link_readXsectParams")
}

/// Called by: dwflow.
pub fn link_setFlapGate(prj: &mut Project, j: i32, n1: i32, n2: i32, q: f64) -> i32 {
    todo!("link_setFlapGate")
}

/// Called by: routing.
pub fn link_setOldHydState(prj: &mut Project, j: i32) {
    todo!("link_setOldHydState")
}

/// Called by: routing.
pub fn link_setOldQualState(prj: &mut Project, j: i32) {
    todo!("link_setOldQualState")
}

/// Called by: dynwave, flowrout.
pub fn link_setOutfallDepth(prj: &mut Project, j: i32) {
    todo!("link_setOutfallDepth")
}

/// Called by: hotstart, routing, swmm5.
pub fn link_setSetting(prj: &mut Project, j: i32, tstep: f64) {
    todo!("link_setSetting")
}

/// Called by: hotstart, routing.
pub fn link_setTargetSetting(prj: &mut Project, j: i32) {
    todo!("link_setTargetSetting")
}

/// Called by: project.
pub fn link_validate(prj: &mut Project, j: i32) {
    todo!("link_validate")
}
