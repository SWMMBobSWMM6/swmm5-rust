//! subcatch.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until subcatch.c is translated.

use crate::prelude::*;

/// The file-scope variables of subcatch.c.
#[derive(Default)]
pub struct SubcatchState {
    pub Vevap: f64,
    pub Vpevap: f64,
    pub Vinfil: f64,
    pub Vinflow: f64,
    pub Voutflow: f64,
    pub VlidIn: f64,
    pub VlidInfil: f64,
    pub VlidOut: f64,
    pub VlidDrain: f64,
    pub VlidReturn: f64,
}

/// Called by: lid, runoff.
pub fn subcatch_addRunonFlow(prj: &mut Project, k: i32, q: f64) {
    todo!("subcatch_addRunonFlow")
}

/// Called by: surfqual.
pub fn subcatch_getDepth(prj: &Project, j: i32) -> f64 {
    todo!("subcatch_getDepth")
}

/// Called by: gwater.
pub fn subcatch_getFracPerv(prj: &Project, j: i32) -> f64 {
    todo!("subcatch_getFracPerv")
}

/// Called by: output, runoff.
pub fn subcatch_getResults(prj: &mut Project, j: i32, wt: f64, x: &mut [f32]) {
    todo!("subcatch_getResults")
}

/// Called by: runoff.
pub fn subcatch_getRunoff(prj: &mut Project, j: i32, tStep: f64) -> f64 {
    todo!("subcatch_getRunoff")
}

/// Called by: runoff.
pub fn subcatch_getRunon(prj: &mut Project, j: i32) {
    todo!("subcatch_getRunon")
}

/// Called by: massbal.
pub fn subcatch_getStorage(prj: &Project, j: i32) -> f64 {
    todo!("subcatch_getStorage")
}

/// Called by: routing.
pub fn subcatch_getWtdOutflow(prj: &Project, j: i32, f: f64) -> f64 {
    todo!("subcatch_getWtdOutflow")
}

/// Called by: project.
pub fn subcatch_initState(prj: &mut Project, j: i32) {
    todo!("subcatch_initState")
}

/// Called by: input.
pub fn subcatch_readInitBuildup(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("subcatch_readInitBuildup")
}

/// Called by: input.
pub fn subcatch_readLanduseParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("subcatch_readLanduseParams")
}

/// Called by: input.
pub fn subcatch_readParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("subcatch_readParams")
}

/// Called by: input.
pub fn subcatch_readSubareaParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("subcatch_readSubareaParams")
}

/// Called by: runoff.
pub fn subcatch_setOldState(prj: &mut Project, j: i32) {
    todo!("subcatch_setOldState")
}

/// Called by: project.
pub fn subcatch_validate(prj: &mut Project, j: i32) {
    todo!("subcatch_validate")
}
