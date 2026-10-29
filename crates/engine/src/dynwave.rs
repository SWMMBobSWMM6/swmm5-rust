//! dynwave.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until dynwave.c is translated.

use crate::prelude::*;

/// The file-scope variables of dynwave.c.
#[derive(Default)]
pub struct DynwaveState {
}

/// Called by: flowrout.
pub fn dynwave_close(prj: &mut Project) {
    todo!("dynwave_close")
}

/// Called by: flowrout.
pub fn dynwave_execute(prj: &mut Project, tStep: f64) -> i32 {
    todo!("dynwave_execute")
}

/// Called by: flowrout.
pub fn dynwave_getRoutingStep(prj: &mut Project, fixedStep: f64) -> f64 {
    todo!("dynwave_getRoutingStep")
}

/// Called by: flowrout.
pub fn dynwave_init(prj: &mut Project) {
    todo!("dynwave_init")
}

/// Called by: project.
pub fn dynwave_validate(prj: &mut Project) {
    todo!("dynwave_validate")
}
