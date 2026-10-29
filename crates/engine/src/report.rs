//! report.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until report.c is translated.

use crate::prelude::*;

/// The file-scope variables of report.c.
#[derive(Default)]
pub struct ReportState {
}

/// Called by: input.
pub fn report_readOptions(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    todo!("report_readOptions")
}

/// Called by: controls.
pub fn report_writeControlAction(prj: &mut Project, aDate: DateTime, linkID: &[u8], value: f64, ruleID: &[u8]) {
    todo!("report_writeControlAction")
}

/// Called by: swmm5.
pub fn report_writeControlActionsHeading(prj: &mut Project) {
    todo!("report_writeControlActionsHeading")
}

/// Called by: climate, dynwave, flowrout, gage, gwater, hotstart, iface, kinwave, lid, link, massbal, node, output, project, rain, rdii, routing, runoff, snow, stats, subcatch, toposort, transect, treatmnt.
pub fn report_writeErrorMsg(prj: &mut Project, code: i32, s: &[u8]) {
    todo!("report_writeErrorMsg")
}

/// Called by: massbal.
pub fn report_writeFlowError(prj: &mut Project, totals: &TRoutingTotals) {
    todo!("report_writeFlowError")
}

/// Called by: massbal.
pub fn report_writeGwaterError(prj: &mut Project, totals: &TGwaterTotals, gwArea: f64) {
    todo!("report_writeGwaterError")
}

/// Called by: input.
pub fn report_writeInputErrorMsg(prj: &mut Project, k: i32, sect: i32, line: &[u8], lineCount: i64) {
    todo!("report_writeInputErrorMsg")
}

/// Called by: inlet, input, inputrpt, rain, statsrpt, swmm5.
pub fn report_writeLine(prj: &mut Project, line: &[u8]) {
    todo!("report_writeLine")
}

/// Called by: massbal.
pub fn report_writeLoadingError(prj: &mut Project, totals: &TLoadingTotals) {
    todo!("report_writeLoadingError")
}

/// Called by: swmm5.
pub fn report_writeLogo(prj: &mut Project) {
    todo!("report_writeLogo")
}

/// Called by: stats.
pub fn report_writeMaxFlowTurns(prj: &mut Project, flowTurns: &[TMaxStats], nMaxStats: i32) {
    todo!("report_writeMaxFlowTurns")
}

/// Called by: stats.
pub fn report_writeMaxStats(prj: &mut Project, maxMassBalErrs: &[TMaxStats], maxCourantCrit: &[TMaxStats], nMaxStats: i32) {
    todo!("report_writeMaxStats")
}

/// Called by: stats.
pub fn report_writeNonconvergedStats(prj: &mut Project, maxNonconverged: &[TMaxStats], nMaxStats: i32) {
    todo!("report_writeNonconvergedStats")
}

/// Called by: swmm5.
pub fn report_writeOptions(prj: &mut Project) {
    todo!("report_writeOptions")
}

/// Called by: massbal.
pub fn report_writeQualError(prj: &mut Project, QualTotals: &TRoutingTotals) {
    todo!("report_writeQualError")
}

/// Called by: rain.
pub fn report_writeRainStats(prj: &mut Project, i: i32, r: &TRainStats) {
    todo!("report_writeRainStats")
}

/// Called by: rdii.
pub fn report_writeRdiiStats(prj: &mut Project, rainVol: f64, rdiiVol: f64) {
    todo!("report_writeRdiiStats")
}

/// Called by: swmm5.
pub fn report_writeReport(prj: &mut Project) {
    todo!("report_writeReport")
}

/// Called by: massbal.
pub fn report_writeRunoffError(prj: &mut Project, totals: &TRunoffTotals, totalArea: f64) {
    todo!("report_writeRunoffError")
}

/// Called by: swmm5.
pub fn report_writeSysTime(prj: &mut Project) {
    todo!("report_writeSysTime")
}

/// Called by: stats.
pub fn report_writeTimeStepStats(prj: &mut Project, timeStepStats: &TTimeStepStats) {
    todo!("report_writeTimeStepStats")
}

/// Called by: swmm5.
pub fn report_writeTitle(prj: &mut Project) {
    todo!("report_writeTitle")
}

/// Called by: project.
pub fn report_writeTseriesErrorMsg(prj: &mut Project, code: i32, i: i32) {
    todo!("report_writeTseriesErrorMsg")
}

/// Called by: controls, gage, inlet, link, node, project.
pub fn report_writeWarningMsg(prj: &mut Project, msg: &[u8], id: &[u8]) {
    todo!("report_writeWarningMsg")
}
