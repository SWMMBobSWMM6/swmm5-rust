//! odesolve.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until odesolve.c is translated.

use crate::prelude::*;

/// The file-scope variables of odesolve.c.
#[derive(Default)]
pub struct OdesolveState {
}

/// Called by: runoff.
pub fn odesolve_close() {
    todo!("odesolve_close")
}

/// Called by: gwater, subcatch.
pub fn odesolve_integrate(ystart: &mut [f64], n: i32, x1: f64, x2: f64, eps: f64, h1: f64, derivs: &mut dyn FnMut(f64, &[f64], &mut [f64])) -> i32 {
    todo!("odesolve_integrate")
}

/// Called by: runoff.
pub fn odesolve_open(maxn: i32) -> i32 {
    todo!("odesolve_open")
}
