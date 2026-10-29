//! flowrout.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until flowrout.c is translated.

use crate::prelude::*;

/// The file-scope variables of flowrout.c.
#[derive(Default)]
pub struct FlowroutState {
}

/// Called by: routing.
pub fn flowrout_close(prj: &mut Project, routingModel: i32) {
    todo!("flowrout_close")
}

/// Called by: routing.
pub fn flowrout_execute(prj: &mut Project, links: &[i32], routingModel: i32, tStep: f64) -> i32 {
    todo!("flowrout_execute")
}

/// Called by: routing.
pub fn flowrout_getRoutingStep(prj: &mut Project, routingModel: i32, fixedStep: f64) -> f64 {
    todo!("flowrout_getRoutingStep")
}

/// Called by: routing.
pub fn flowrout_init(prj: &mut Project, routingModel: i32) {
    todo!("flowrout_init")
}
