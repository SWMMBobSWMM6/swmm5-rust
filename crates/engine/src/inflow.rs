//! inflow.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until inflow.c is translated.

use crate::prelude::*;

/// The file-scope variables of inflow.c.
#[derive(Default)]
pub struct InflowState {
}

/// Called by: project.
pub fn inflow_deleteDwfInflows(prj: &mut Project, j: i32) {
    todo!("inflow_deleteDwfInflows")
}

/// Called by: project.
pub fn inflow_deleteExtInflows(prj: &mut Project, j: i32) {
    todo!("inflow_deleteExtInflows")
}

/// Called by: routing.
pub fn inflow_getDwfInflow(prj: &Project, inflow: &TDwfInflow, month: i32, day: i32, hour: i32) -> f64 {
    todo!("inflow_getDwfInflow")
}

/// Called by: routing.
pub fn inflow_getExtInflow(prj: &mut Project, inflow: &TExtInflow, aDate: DateTime) -> f64 {
    todo!("inflow_getExtInflow")
}

/// Called by: node.
pub fn inflow_initDwfInflow(patterns: &[TPattern], inflow: &mut TDwfInflow) {
    todo!("inflow_initDwfInflow")
}

/// Called by: project.
pub fn inflow_initDwfPattern(prj: &mut Project, j: i32) {
    todo!("inflow_initDwfPattern")
}

/// Called by: input.
pub fn inflow_readDwfInflow(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("inflow_readDwfInflow")
}

/// Called by: input.
pub fn inflow_readDwfPattern(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("inflow_readDwfPattern")
}

/// Called by: input.
pub fn inflow_readExtInflow(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("inflow_readExtInflow")
}
