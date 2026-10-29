//! rdii.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until rdii.c is translated.

use crate::prelude::*;

/// The file-scope variables of rdii.c.
#[derive(Default)]
pub struct RdiiState {
}

/// Called by: rain.
pub fn rdii_closeRdii(prj: &mut Project) {
    todo!("rdii_closeRdii")
}

/// Called by: project.
pub fn rdii_deleteRdiiInflow(prj: &mut Project, j: i32) {
    todo!("rdii_deleteRdiiInflow")
}

/// Called by: routing.
pub fn rdii_getNumRdiiFlows(prj: &mut Project, aDate: DateTime) -> i32 {
    todo!("rdii_getNumRdiiFlows")
}

/// Called by: routing.
pub fn rdii_getRdiiFlow(prj: &mut Project, i: i32, j: &mut i32, q: &mut f64) {
    todo!("rdii_getRdiiFlow")
}

/// Called by: project.
pub fn rdii_initUnitHyd(prj: &mut Project, j: i32) {
    todo!("rdii_initUnitHyd")
}

/// Called by: rain.
pub fn rdii_openRdii(prj: &mut Project) {
    todo!("rdii_openRdii")
}

/// Called by: input.
pub fn rdii_readRdiiInflow(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("rdii_readRdiiInflow")
}

/// Called by: input.
pub fn rdii_readUnitHydParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("rdii_readUnitHydParams")
}
