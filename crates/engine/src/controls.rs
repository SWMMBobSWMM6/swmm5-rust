//! controls.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until controls.c is translated.

use crate::prelude::*;

/// The file-scope variables of controls.c.
#[derive(Default)]
pub struct ControlsState {
}

/// Called by: input.
pub fn controls_addExpression(prj: &mut Project, tok: &[Vec<u8>], nToks: i32) -> i32 {
    todo!("controls_addExpression")
}

/// Called by: input.
pub fn controls_addRuleClause(prj: &mut Project, r: i32, keyword: i32, tok: &[Vec<u8>], nToks: i32) -> i32 {
    todo!("controls_addRuleClause")
}

/// Called by: input.
pub fn controls_addToCount(prj: &mut Project, s: &[u8]) {
    todo!("controls_addToCount")
}

/// Called by: input.
pub fn controls_addVariable(prj: &mut Project, tok: &[Vec<u8>], nToks: i32) -> i32 {
    todo!("controls_addVariable")
}

/// Called by: project.
pub fn controls_create(prj: &mut Project, n: i32) -> i32 {
    todo!("controls_create")
}

/// Called by: project.
pub fn controls_delete(prj: &mut Project) {
    todo!("controls_delete")
}

/// Called by: routing.
pub fn controls_evaluate(prj: &mut Project, currentTime: DateTime, elapsedTime: DateTime, tStep: f64) -> i32 {
    todo!("controls_evaluate")
}

/// Called by: input.
pub fn controls_init(prj: &mut Project) {
    todo!("controls_init")
}
