//! transect.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until transect.c is translated.

use crate::prelude::*;

/// The file-scope variables of transect.c.
#[derive(Default)]
pub struct TransectState {
}

/// Called by: project.
pub fn transect_create(prj: &mut Project, n: i32) -> i32 {
    todo!("transect_create")
}

/// Called by: street.
pub fn transect_createStreetTransect(prj: &mut Project, street: &mut TStreet) {
    todo!("transect_createStreetTransect")
}

/// Called by: project.
pub fn transect_delete(prj: &mut Project) {
    todo!("transect_delete")
}

/// Called by: input.
pub fn transect_readParams(prj: &mut Project, count: &mut i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("transect_readParams")
}

/// Called by: input.
pub fn transect_validate(prj: &mut Project, j: i32) {
    todo!("transect_validate")
}
