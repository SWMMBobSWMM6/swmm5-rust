//! snow.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until snow.c is translated.

use crate::prelude::*;

/// The file-scope variables of snow.c.
#[derive(Default)]
pub struct SnowState {
}

/// Called by: subcatch.
pub fn snow_createSnowpack(prj: &mut Project, j: i32, k: i32) -> i32 {
    todo!("snow_createSnowpack")
}

/// Called by: massbal.
pub fn snow_getSnowCover(prj: &Project, j: i32) -> f64 {
    todo!("snow_getSnowCover")
}

/// Called by: subcatch.
pub fn snow_getSnowMelt(prj: &mut Project, j: i32, rainfall: f64, snowfall: f64, tStep: f64, netPrecip: &mut [f64]) -> f64 {
    todo!("snow_getSnowMelt")
}

/// Called by: hotstart.
pub fn snow_getState(prj: &Project, i: i32, j: i32, x: &mut [f64]) {
    todo!("snow_getState")
}

/// Called by: project.
pub fn snow_initSnowmelt(prj: &mut Project, j: i32) {
    todo!("snow_initSnowmelt")
}

/// Called by: subcatch.
pub fn snow_initSnowpack(prj: &mut Project, j: i32) {
    todo!("snow_initSnowpack")
}

/// Called by: runoff.
pub fn snow_plowSnow(prj: &mut Project, j: i32, tStep: f64) {
    todo!("snow_plowSnow")
}

/// Called by: input.
pub fn snow_readMeltParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("snow_readMeltParams")
}

/// Called by: climate.
pub fn snow_setMeltCoeffs(prj: &mut Project, j: i32, s: f64) {
    todo!("snow_setMeltCoeffs")
}

/// Called by: hotstart.
pub fn snow_setState(prj: &mut Project, i: i32, j: i32, x: &[f64]) {
    todo!("snow_setState")
}

/// Called by: project.
pub fn snow_validateSnowmelt(prj: &mut Project, j: i32) {
    todo!("snow_validateSnowmelt")
}
