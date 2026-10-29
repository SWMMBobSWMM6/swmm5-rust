//! runoff.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until runoff.c is translated.

use crate::prelude::*;

/// The file-scope variables of runoff.c.
#[derive(Default)]
pub struct RunoffState {
    pub HasWetLids: i8,
    pub OutflowLoad: Vec<f64>,
}

/// Called by: swmm5.
pub fn runoff_close(prj: &mut Project) {
    todo!("runoff_close")
}

/// Called by: swmm5.
pub fn runoff_execute(prj: &mut Project) {
    todo!("runoff_execute")
}

/// Called by: swmm5.
pub fn runoff_open(prj: &mut Project) -> i32 {
    todo!("runoff_open")
}
