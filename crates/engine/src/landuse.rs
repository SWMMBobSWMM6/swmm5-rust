//! landuse.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until landuse.c is translated.

use crate::prelude::*;

/// The file-scope variables of landuse.c.
#[derive(Default)]
pub struct LanduseState {
}

/// Called by: surfqual.
pub fn landuse_getAvgBmpEffic(prj: &mut Project, j: i32, p: i32) -> f64 {
    todo!("landuse_getAvgBmpEffic")
}

/// Called by: surfqual.
pub fn landuse_getBuildup(prj: &mut Project, i: i32, p: i32, area: f64, curb: f64, buildup: f64, tStep: f64) -> f64 {
    todo!("landuse_getBuildup")
}

/// Called by: (entry point).
pub fn landuse_getCoPollutLoad(prj: &Project, p: i32, washoff: &[f64]) -> f64 {
    todo!("landuse_getCoPollutLoad")
}

/// Called by: surfqual.
pub fn landuse_getInitBuildup(prj: &Project, landFactor: &mut [TLandFactor], initBuildup: &[f64], area: f64, curb: f64) {
    todo!("landuse_getInitBuildup")
}

/// Called by: surfqual.
pub fn landuse_getWashoffLoad(prj: &mut Project, i: i32, p: i32, area: f64, landFactor: &mut [TLandFactor], runoff: f64, vOutflow: f64) -> f64 {
    todo!("landuse_getWashoffLoad")
}

/// Called by: input.
pub fn landuse_readBuildupParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("landuse_readBuildupParams")
}

/// Called by: input.
pub fn landuse_readParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("landuse_readParams")
}

/// Called by: input.
pub fn landuse_readPollutParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("landuse_readPollutParams")
}

/// Called by: input.
pub fn landuse_readWashoffParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("landuse_readWashoffParams")
}
