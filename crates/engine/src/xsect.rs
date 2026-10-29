//! xsect.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until xsect.c is translated.

use crate::prelude::*;

/// The file-scope variables of xsect.c.
#[derive(Default)]
pub struct XsectState {
}

/// Called by: kinwave.
pub fn xsect_getAmax(prj: &Project, xsect: &TXsect) -> f64 {
    todo!("xsect_getAmax")
}

/// Called by: flowrout, inlet, kinwave, link.
pub fn xsect_getAofS(prj: &Project, xsect: &TXsect, sFactor: f64) -> f64 {
    todo!("xsect_getAofS")
}

/// Called by: culvert, dwflow, flowrout, link, node.
pub fn xsect_getAofY(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
    todo!("xsect_getAofY")
}

/// Called by: (entry point).
pub fn xsect_getRofA(prj: &Project, xsect: &TXsect, a: f64) -> f64 {
    todo!("xsect_getRofA")
}

/// Called by: dwflow.
pub fn xsect_getRofY(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
    todo!("xsect_getRofY")
}

/// Called by: kinwave, node.
pub fn xsect_getSofA(prj: &Project, xsect: &TXsect, area: f64) -> f64 {
    todo!("xsect_getSofA")
}

/// Called by: culvert, dwflow, inlet, link, swmm5.
pub fn xsect_getWofY(prj: &Project, xsect: &TXsect, y: f64) -> f64 {
    todo!("xsect_getWofY")
}

/// Called by: link.
pub fn xsect_getYcrit(prj: &Project, xsect: &TXsect, q: f64) -> f64 {
    todo!("xsect_getYcrit")
}

/// Called by: flowrout, inlet, link.
pub fn xsect_getYofA(prj: &Project, xsect: &TXsect, a: f64) -> f64 {
    todo!("xsect_getYofA")
}

/// Called by: kinwave.
pub fn xsect_getdSdA(prj: &Project, xsect: &TXsect, a: f64) -> f64 {
    todo!("xsect_getdSdA")
}

/// Called by: dwflow, link.
pub fn xsect_isOpen(type_: i32) -> i32 {
    todo!("xsect_isOpen")
}

/// Called by: link.
pub fn xsect_setCustomXsectParams(prj: &Project, xsect: &mut TXsect) {
    todo!("xsect_setCustomXsectParams")
}

/// Called by: link.
pub fn xsect_setIrregXsectParams(prj: &Project, xsect: &mut TXsect) {
    todo!("xsect_setIrregXsectParams")
}

/// Called by: link.
pub fn xsect_setParams(prj: &Project, xsect: &mut TXsect, type_: i32, p: &[f64], ucf: f64) -> i32 {
    todo!("xsect_setParams")
}

/// Called by: link.
pub fn xsect_setStreetXsectParams(prj: &Project, xsect: &mut TXsect) {
    todo!("xsect_setStreetXsectParams")
}
