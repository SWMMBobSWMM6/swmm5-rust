//! gwater.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until gwater.c is translated.

use crate::prelude::*;

/// The file-scope variables of gwater.c.
#[derive(Default)]
pub struct GwaterState {
}

/// Called by: project.
pub fn gwater_deleteFlowExpression(prj: &mut Project, j: i32) {
    todo!("gwater_deleteFlowExpression")
}

/// Called by: subcatch.
pub fn gwater_getGroundwater(prj: &mut Project, j: i32, evap: f64, infil: f64, tStep: f64) {
    todo!("gwater_getGroundwater")
}

/// Called by: hotstart.
pub fn gwater_getState(prj: &Project, j: i32, x: &mut [f64]) {
    todo!("gwater_getState")
}

/// Called by: massbal.
pub fn gwater_getVolume(prj: &Project, j: i32) -> f64 {
    todo!("gwater_getVolume")
}

/// Called by: subcatch.
pub fn gwater_initState(prj: &mut Project, j: i32) {
    todo!("gwater_initState")
}

/// Called by: input.
pub fn gwater_readAquiferParams(prj: &mut Project, j: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("gwater_readAquiferParams")
}

/// Called by: input.
pub fn gwater_readFlowExpression(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("gwater_readFlowExpression")
}

/// Called by: input.
pub fn gwater_readGroundwaterParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("gwater_readGroundwaterParams")
}

/// Called by: hotstart.
pub fn gwater_setState(prj: &mut Project, j: i32, x: &[f64]) {
    todo!("gwater_setState")
}

/// Called by: subcatch.
pub fn gwater_validate(prj: &mut Project, j: i32) {
    todo!("gwater_validate")
}

/// Called by: project.
pub fn gwater_validateAquifer(prj: &mut Project, j: i32) {
    todo!("gwater_validateAquifer")
}
