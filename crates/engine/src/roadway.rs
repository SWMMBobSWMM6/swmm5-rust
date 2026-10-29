//! roadway.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until roadway.c is translated.

use crate::prelude::*;

/// The file-scope variables of roadway.c.
#[derive(Default)]
pub struct RoadwayState {
}

/// Called by: link.
pub fn roadway_getInflow(prj: &mut Project, j: i32, dir: f64, hRoad: f64, h1: f64, h2: f64) -> f64 {
    todo!("roadway_getInflow")
}
