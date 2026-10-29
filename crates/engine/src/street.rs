//! street.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until street.c is translated.

use crate::prelude::*;

/// The file-scope variables of street.c.
#[derive(Default)]
pub struct StreetState {
}

/// Called by: project.
pub fn street_create(prj: &mut Project, nStreets: i32) -> i32 {
    todo!("street_create")
}

/// Called by: project.
pub fn street_delete(prj: &mut Project) {
    todo!("street_delete")
}

/// Called by: stats.
pub fn street_getExtentFilled(prj: &Project, link: i32) -> f64 {
    todo!("street_getExtentFilled")
}

/// Called by: input.
pub fn street_readParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("street_readParams")
}
