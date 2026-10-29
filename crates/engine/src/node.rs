//! node.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until node.c is translated.

use crate::prelude::*;

/// The file-scope variables of node.c.
#[derive(Default)]
pub struct NodeState {
}

/// Called by: flowrout.
pub fn node_getDepth(prj: &mut Project, j: i32, v: f64) -> f64 {
    todo!("node_getDepth")
}

/// Called by: routing.
pub fn node_getLosses(prj: &mut Project, j: i32, tStep: f64) -> f64 {
    todo!("node_getLosses")
}

/// Called by: dynwave, flowrout.
pub fn node_getMaxOutflow(prj: &mut Project, j: i32, q: f64, tStep: f64) -> f64 {
    todo!("node_getMaxOutflow")
}

/// Called by: link.
pub fn node_getOutflow(prj: &mut Project, j: i32, k: i32) -> f64 {
    todo!("node_getOutflow")
}

/// Called by: dynwave.
pub fn node_getPondedArea(prj: &mut Project, j: i32, d: f64) -> f64 {
    todo!("node_getPondedArea")
}

/// Called by: output.
pub fn node_getResults(prj: &mut Project, j: i32, wt: f64, x: &mut [f32]) {
    todo!("node_getResults")
}

/// Called by: dynwave, treatmnt.
pub fn node_getSurfArea(prj: &mut Project, j: i32, d: f64) -> f64 {
    todo!("node_getSurfArea")
}

/// Called by: routing.
pub fn node_getSystemOutflow(prj: &mut Project, j: i32, isFlooded: &mut i32) -> f64 {
    todo!("node_getSystemOutflow")
}

/// Called by: dynwave, flowrout.
pub fn node_getVolume(prj: &mut Project, j: i32, d: f64) -> f64 {
    todo!("node_getVolume")
}

/// Called by: routing.
pub fn node_initFlows(prj: &mut Project, j: i32, tStep: f64) {
    todo!("node_initFlows")
}

/// Called by: project.
pub fn node_initState(prj: &mut Project, j: i32) {
    todo!("node_initState")
}

/// Called by: input.
pub fn node_readParams(prj: &mut Project, j: i32, type_: i32, k: i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("node_readParams")
}

/// Called by: routing.
pub fn node_setOldHydState(prj: &mut Project, j: i32) {
    todo!("node_setOldHydState")
}

/// Called by: routing.
pub fn node_setOldQualState(prj: &mut Project, j: i32) {
    todo!("node_setOldQualState")
}

/// Called by: link.
pub fn node_setOutletDepth(prj: &mut Project, j: i32, yNorm: f64, yCrit: f64, z: f64) {
    todo!("node_setOutletDepth")
}

/// Called by: project.
pub fn node_validate(prj: &mut Project, j: i32) {
    todo!("node_validate")
}
