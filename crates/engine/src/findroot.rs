//! findroot.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until findroot.c is translated.

use crate::prelude::*;

/// The file-scope variables of findroot.c.
#[derive(Default)]
pub struct FindrootState {
}

/// Called by: kinwave, node, xsect.
pub fn findroot_Newton(x1: f64, x2: f64, rts: &mut f64, xacc: f64, func: &mut dyn FnMut(f64, &mut f64, &mut f64)) -> i32 {
    todo!("findroot_Newton")
}

/// Called by: culvert, xsect.
pub fn findroot_Ridder(x1: f64, x2: f64, xacc: f64, func: &mut dyn FnMut(f64) -> f64) -> f64 {
    todo!("findroot_Ridder")
}
