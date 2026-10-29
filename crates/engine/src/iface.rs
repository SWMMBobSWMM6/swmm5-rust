//! iface.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until iface.c is translated.

use crate::prelude::*;

/// The file-scope variables of iface.c.
#[derive(Default)]
pub struct IfaceState {
}

/// Called by: routing.
pub fn iface_closeRoutingFiles(prj: &mut Project) {
    todo!("iface_closeRoutingFiles")
}

/// Called by: routing.
pub fn iface_getIfaceFlow(prj: &Project, index: i32) -> f64 {
    todo!("iface_getIfaceFlow")
}

/// Called by: routing.
pub fn iface_getIfaceNode(prj: &Project, index: i32) -> i32 {
    todo!("iface_getIfaceNode")
}

/// Called by: routing.
pub fn iface_getIfaceQual(prj: &Project, index: i32, pollut: i32) -> f64 {
    todo!("iface_getIfaceQual")
}

/// Called by: routing.
pub fn iface_getNumIfaceNodes(prj: &mut Project, currentDate: DateTime) -> i32 {
    todo!("iface_getNumIfaceNodes")
}

/// Called by: routing.
pub fn iface_openRoutingFiles(prj: &mut Project) {
    todo!("iface_openRoutingFiles")
}

/// Called by: input.
pub fn iface_readFileParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("iface_readFileParams")
}

/// Called by: output.
pub fn iface_saveOutletResults(prj: &mut Project, reportDate: DateTime) {
    todo!("iface_saveOutletResults")
}
