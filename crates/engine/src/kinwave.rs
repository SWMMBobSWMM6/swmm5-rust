//! kinwave.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until kinwave.c is translated.

use crate::prelude::*;

/// The file-scope variables of kinwave.c.
#[derive(Default)]
pub struct KinwaveState {
}

/// Called by: flowrout.
pub fn kinwave_execute(prj: &mut Project, j: i32, qinflow: &mut f64, qoutflow: &mut f64, tStep: f64) -> i32 {
    todo!("kinwave_execute")
}
