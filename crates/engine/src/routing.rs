//! routing.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until routing.c is translated.

use crate::prelude::*;

/// The file-scope variables of routing.c.
#[derive(Default)]
pub struct RoutingState {
}

/// Called by: swmm5.
pub fn routing_close(prj: &mut Project, routingModel: i32) {
    todo!("routing_close")
}

/// Called by: swmm5.
pub fn routing_execute(prj: &mut Project, routingModel: i32, routingStep: f64) {
    todo!("routing_execute")
}

/// Called by: swmm5.
pub fn routing_getRoutingStep(prj: &mut Project, routingModel: i32, fixedStep: f64) -> f64 {
    todo!("routing_getRoutingStep")
}

/// Called by: swmm5.
pub fn routing_open(prj: &mut Project) -> i32 {
    todo!("routing_open")
}
