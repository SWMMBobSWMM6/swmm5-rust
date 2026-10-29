//! exfil.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until exfil.c is translated.

use crate::prelude::*;

/// The file-scope variables of exfil.c.
#[derive(Default)]
pub struct ExfilState {
}

/// Called by: node.
pub fn exfil_getLoss(ist: &mut crate::infil::InfilState, exfil: &mut TExfil, tStep: f64, depth: f64, area: f64) -> f64 {
    todo!("exfil_getLoss")
}

/// Called by: node.
pub fn exfil_initState(prj: &mut Project, k: i32) {
    todo!("exfil_initState")
}

/// Called by: node.
pub fn exfil_readStorageParams(prj: &mut Project, k: i32, tok: &[Vec<u8>], ntoks: i32, n: i32) -> i32 {
    todo!("exfil_readStorageParams")
}
