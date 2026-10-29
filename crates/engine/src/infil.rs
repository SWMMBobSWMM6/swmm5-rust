//! infil.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until infil.c is translated.

use crate::prelude::*;

/// The file-scope variables of infil.c.
#[derive(Default)]
pub struct InfilState {
}

/// Called by: exfil, lidproc.
pub fn grnampt_getInfil(ist: &mut crate::infil::InfilState, infil: &mut TGrnAmpt, tstep: f64, irate: f64, depth: f64, modelType: i32) -> f64 {
    todo!("grnampt_getInfil")
}

/// Called by: lid.
pub fn grnampt_getParams(prj: &Project, j: i32, p: &mut [f64]) {
    todo!("grnampt_getParams")
}

/// Called by: exfil, lid.
pub fn grnampt_initState(infil: &mut TGrnAmpt) {
    todo!("grnampt_initState")
}

/// Called by: exfil, lid.
pub fn grnampt_setParams(infil: &mut TGrnAmpt, p: &[f64]) -> i32 {
    todo!("grnampt_setParams")
}

/// Called by: project.
pub fn infil_create(prj: &mut Project, n: i32) {
    todo!("infil_create")
}

/// Called by: project.
pub fn infil_delete(prj: &mut Project) {
    todo!("infil_delete")
}

/// Called by: lid, subcatch.
pub fn infil_getInfil(prj: &mut Project, j: i32, tstep: f64, rainfall: f64, runon: f64, depth: f64) -> f64 {
    todo!("infil_getInfil")
}

/// Called by: hotstart.
pub fn infil_getState(prj: &Project, j: i32, x: &mut [f64]) {
    todo!("infil_getState")
}

/// Called by: subcatch.
pub fn infil_initState(prj: &mut Project, j: i32) {
    todo!("infil_initState")
}

/// Called by: input.
pub fn infil_readParams(prj: &mut Project, m: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("infil_readParams")
}

/// Called by: routing, subcatch.
pub fn infil_setInfilFactor(prj: &mut Project, j: i32) {
    todo!("infil_setInfilFactor")
}

/// Called by: hotstart.
pub fn infil_setState(prj: &mut Project, j: i32, x: &[f64]) {
    todo!("infil_setState")
}
