//! culvert.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until culvert.c is translated.

use crate::prelude::*;

/// The file-scope variables of culvert.c.
#[derive(Default)]
pub struct CulvertState {
}

/// Called by: dwflow.
pub fn culvert_getInflow(prj: &mut Project, j: i32, q0: f64, h: f64) -> f64 {
    todo!("culvert_getInflow")
}
