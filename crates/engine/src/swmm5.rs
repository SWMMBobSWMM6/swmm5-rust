//! swmm5.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until swmm5.c is translated.

use crate::prelude::*;

/// The file-scope variables of swmm5.c.
#[derive(Default)]
pub struct Swmm5State {
}

/// Called by: climate, controls, dynwave, exfil, gage, gwater, iface, infil, inflow, inlet, inputrpt, landuse, lid, lidproc, link, massbal, node, output, rdii, report, runoff, snow, statsrpt, street, subcatch, surfqual, transect, treatmnt.
pub fn UCF(prj: &Project, u: i32) -> f64 {
    todo!("UCF")
}

/// Called by: climate, gage, iface, table.
pub fn addAbsolutePath(prj: &mut Project, fname: &mut [u8]) {
    todo!("addAbsolutePath")
}

/// Called by: gwater, infil, inlet, landuse, lidproc, output, routing, runoff, subcatch.
pub fn getDateTime(prj: &Project, elapsedMsec: f64) -> DateTime {
    todo!("getDateTime")
}

/// Called by: statsrpt.
pub fn getElapsedTime(prj: &Project, aDate: DateTime, days: &mut i32, hrs: &mut i32, mins: &mut i32) {
    todo!("getElapsedTime")
}

/// Called by: output, rain, rdii.
pub fn getTempFileName(prj: &mut Project, fname: &mut [u8]) {
    todo!("getTempFileName")
}

/// Called by: (entry point).
pub fn isRelativePath(fname: &[u8]) -> i32 {
    todo!("isRelativePath")
}

/// Called by: controls, gwater, lid, project, treatmnt.
pub fn sstrcat(dest: &mut [u8], src: &[u8], size: usize) -> usize {
    todo!("sstrcat")
}

/// Called by: climate, controls, gage, gwater, iface, input, lid, project, rain, report, statsrpt, table, treatmnt.
pub fn sstrncpy(dest: &mut [u8], src: &[u8], n: usize) -> usize {
    todo!("sstrncpy")
}

/// Called by: climate, controls, iface, landuse, link, project, rain, report, table.
pub fn strcomp(s1: &[u8], s2: &[u8]) -> i32 {
    todo!("strcomp")
}

/// Called by: output, project.
pub fn writecon(prj: &mut Project, s: &[u8]) {
    todo!("writecon")
}
