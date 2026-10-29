//! forcmain.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until forcmain.c is translated.

use crate::prelude::*;

/// The file-scope variables of forcmain.c.
#[derive(Default)]
pub struct ForcmainState {
}

/// Called by: link.
pub fn forcemain_getEquivN(prj: &mut Project, j: i32, k: i32) -> f64 {
    todo!("forcemain_getEquivN")
}

/// Called by: dwflow.
pub fn forcemain_getFricSlope(prj: &mut Project, j: i32, v: f64, hrad: f64) -> f64 {
    todo!("forcemain_getFricSlope")
}

/// Called by: link.
pub fn forcemain_getRoughFactor(prj: &mut Project, j: i32, lengthFactor: f64) -> f64 {
    todo!("forcemain_getRoughFactor")
}
