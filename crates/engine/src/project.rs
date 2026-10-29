//! project.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until project.c is translated.

use crate::prelude::*;

/// The file-scope variables of project.c.
#[derive(Default)]
pub struct ProjectState {
    pub Htable: Vec<HTtable>,
}

/// Called by: input.
pub fn project_addObject(prj: &mut Project, type_: i32, id: &[u8], n: i32) -> i32 {
    todo!("project_addObject")
}

/// Called by: swmm5.
pub fn project_close(prj: &mut Project) {
    todo!("project_close")
}

/// Called by: iface.
pub fn project_createMatrix(nrows: i32, ncols: i32) -> Vec<Vec<f64>> {
    todo!("project_createMatrix")
}

/// Called by: controls, gage, gwater, inflow, inlet, landuse, lid, link, node, rdii, snow, street, subcatch, table, transect.
pub fn project_findID(prj: &Project, type_: i32, id: &[u8]) -> Option<Vec<u8>> {
    todo!("project_findID")
}

/// Called by: climate, controls, gage, gwater, iface, infil, inflow, inlet, input, landuse, lid, link, node, rdii, report, snow, street, subcatch, swmm5, table, treatmnt.
pub fn project_findObject(prj: &Project, type_: i32, id: &[u8]) -> i32 {
    todo!("project_findObject")
}

/// Called by: iface.
pub fn project_freeMatrix(a: Vec<Vec<f64>>) {
    todo!("project_freeMatrix")
}

/// Called by: swmm5.
pub fn project_init(prj: &mut Project) -> i32 {
    todo!("project_init")
}

/// Called by: swmm5.
pub fn project_open(prj: &mut Project, f1: &[u8], f2: &[u8], f3: &[u8]) {
    todo!("project_open")
}

/// Called by: swmm5.
pub fn project_readInput(prj: &mut Project) {
    todo!("project_readInput")
}

/// Called by: input.
pub fn project_readOption(prj: &mut Project, s1: &[u8], s2: &[u8]) -> i32 {
    todo!("project_readOption")
}

/// Called by: swmm5.
pub fn project_validate(prj: &mut Project) {
    todo!("project_validate")
}
