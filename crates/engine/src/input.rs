//! input.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until input.c is translated.

use crate::prelude::*;

/// The file-scope variables of input.c.
#[derive(Default)]
pub struct InputState {
}

/// Called by: climate, controls, gage, gwater, iface, infil, inflow, inlet, landuse, lid, link, node, project, rdii, report, snow, subcatch, table, transect, treatmnt.
pub fn findmatch(s: &[u8], keyword: &[&[u8]]) -> i32 {
    todo!("findmatch")
}

/// Called by: climate, controls, exfil, gage, gwater, infil, inflow, inlet, landuse, lid, link, node, project, rdii, snow, street, subcatch, table, transect.
pub fn getDouble(s: &[u8], y: &mut f64) -> i32 {
    todo!("getDouble")
}

/// Called by: (entry point).
pub fn getFloat(s: &[u8], y: &mut f32) -> i32 {
    todo!("getFloat")
}

/// Called by: inlet, street.
pub fn getInt(s: &[u8], y: &mut i32) -> i32 {
    todo!("getInt")
}

/// Called by: project.
pub fn input_countObjects(prj: &mut Project) -> i32 {
    todo!("input_countObjects")
}

/// Called by: project.
pub fn input_readData(prj: &mut Project) -> i32 {
    todo!("input_readData")
}
