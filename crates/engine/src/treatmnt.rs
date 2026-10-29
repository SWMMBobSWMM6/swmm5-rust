//! treatmnt.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until treatmnt.c is translated.

use crate::prelude::*;

/// The file-scope variables of treatmnt.c.
#[derive(Default)]
pub struct TreatmntState {
}

/// Called by: routing.
pub fn treatmnt_close(prj: &mut Project) {
    todo!("treatmnt_close")
}

/// Called by: project.
pub fn treatmnt_delete(prj: &mut Project, j: i32) {
    todo!("treatmnt_delete")
}

/// Called by: routing.
pub fn treatmnt_open(prj: &mut Project) -> i32 {
    todo!("treatmnt_open")
}

/// Called by: input.
pub fn treatmnt_readExpression(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("treatmnt_readExpression")
}

/// Called by: qualrout.
pub fn treatmnt_setInflow(prj: &mut Project, qIn: f64, wIn: &[f64]) {
    todo!("treatmnt_setInflow")
}

/// Called by: qualrout.
pub fn treatmnt_treat(prj: &mut Project, j: i32, q: f64, v: f64, tStep: f64) {
    todo!("treatmnt_treat")
}
