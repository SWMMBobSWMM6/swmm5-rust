//! shape.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until shape.c is translated.

use crate::prelude::*;

/// The file-scope variables of shape.c.
#[derive(Default)]
pub struct ShapeState {
}

/// Called by: project.
pub fn shape_validate(shape: &mut TShape, curve: &mut TTable) -> i32 {
    todo!("shape_validate")
}
