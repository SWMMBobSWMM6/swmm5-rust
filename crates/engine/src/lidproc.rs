//! lidproc.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until lidproc.c is translated.

use crate::prelude::*;

/// The file-scope variables of lidproc.c.
#[derive(Default)]
pub struct LidprocState {
}

/// Called by: lid.
pub fn lidproc_getOutflow(prj: &mut Project, lidUnit: &mut TLidUnit, lidProc: &TLidProc, inflow: f64, evap: f64, infil: f64, maxInfil: f64, tStep: f64, lidEvap: &mut f64, lidInfil: &mut f64, lidDrain: &mut f64) -> f64 {
    todo!("lidproc_getOutflow")
}

/// Called by: lid.
pub fn lidproc_initWaterBalance(lidUnit: &mut TLidUnit, initVol: f64) {
    todo!("lidproc_initWaterBalance")
}

/// Called by: lid.
pub fn lidproc_saveResults(prj: &mut Project, lidUnit: &mut TLidUnit, ucfRainfall: f64, ucfRainDepth: f64) {
    todo!("lidproc_saveResults")
}
