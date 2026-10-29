//! table.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until table.c is translated.

use crate::prelude::*;

/// The file-scope variables of table.c.
#[derive(Default)]
pub struct TableState {
}

/// Called by: (entry point).
pub fn table_addEntry(table: &mut TTable, x: f64, y: f64) -> i32 {
    todo!("table_addEntry")
}

/// Called by: project.
pub fn table_deleteEntries(table: &mut TTable) {
    todo!("table_deleteEntries")
}

/// Called by: climate, exfil, gage, link, node, shape.
pub fn table_getFirstEntry(table: &mut TTable, x: &mut f64, y: &mut f64) -> i32 {
    todo!("table_getFirstEntry")
}

/// Called by: (entry point).
pub fn table_getMaxY(table: &mut TTable, x: f64) -> f64 {
    todo!("table_getMaxY")
}

/// Called by: climate, exfil, gage, link, shape.
pub fn table_getNextEntry(table: &mut TTable, x: &mut f64, y: &mut f64) -> i32 {
    todo!("table_getNextEntry")
}

/// Called by: link.
pub fn table_getSlope(table: &mut TTable, x: f64) -> f64 {
    todo!("table_getSlope")
}

/// Called by: node.
pub fn table_getStorageDepth(table: &mut TTable, v: f64) -> f64 {
    todo!("table_getStorageDepth")
}

/// Called by: node.
pub fn table_getStorageVolume(table: &mut TTable, x: f64) -> f64 {
    todo!("table_getStorageVolume")
}

/// Called by: project.
pub fn table_init(table: &mut TTable) {
    todo!("table_init")
}

/// Called by: (entry point).
pub fn table_interpolate(x: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    todo!("table_interpolate")
}

/// Called by: link.
pub fn table_intervalLookup(table: &mut TTable, x: f64) -> f64 {
    todo!("table_intervalLookup")
}

/// Called by: (entry point).
pub fn table_inverseLookup(table: &mut TTable, y: f64) -> f64 {
    todo!("table_inverseLookup")
}

/// Called by: controls, lidproc, link, node.
pub fn table_lookup(table: &mut TTable, x: f64) -> f64 {
    todo!("table_lookup")
}

/// Called by: exfil, inlet, node.
pub fn table_lookupEx(table: &mut TTable, x: f64) -> f64 {
    todo!("table_lookupEx")
}

/// Called by: input.
pub fn table_readCurve(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("table_readCurve")
}

/// Called by: input.
pub fn table_readTimeseries(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("table_readTimeseries")
}

/// Called by: project, rdii.
pub fn table_tseriesInit(table: &mut TTable) {
    todo!("table_tseriesInit")
}

/// Called by: climate, controls, inflow, landuse, node.
pub fn table_tseriesLookup(table: &mut TTable, x: f64, extend: i8) -> f64 {
    todo!("table_tseriesLookup")
}

/// Called by: project.
pub fn table_validate(table: &mut TTable) -> i32 {
    todo!("table_validate")
}
