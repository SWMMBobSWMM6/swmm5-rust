//! stats.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until stats.c is translated.

use crate::prelude::*;

/// The file-scope variables of stats.c.
#[derive(Default)]
pub struct StatsState {
    pub SubcatchStats: Vec<TSubcatchStats>,
    pub NodeStats: Vec<TNodeStats>,
    pub LinkStats: Vec<TLinkStats>,
    pub StorageStats: Vec<TStorageStats>,
    pub OutfallStats: Vec<TOutfallStats>,
    pub PumpStats: Vec<TPumpStats>,
    pub MaxOutfallFlow: f64,
    pub RoutingTimeSpan: f64,
}

/// Called by: swmm5.
pub fn stats_close(prj: &mut Project) {
    todo!("stats_close")
}

/// Called by: swmm5.
pub fn stats_open(prj: &mut Project) -> i32 {
    todo!("stats_open")
}

/// Called by: swmm5.
pub fn stats_report(prj: &mut Project) {
    todo!("stats_report")
}

/// Called by: dynwave.
pub fn stats_updateConvergenceStats(prj: &mut Project, node: i32, converged: i32) {
    todo!("stats_updateConvergenceStats")
}

/// Called by: dynwave.
pub fn stats_updateCriticalTimeCount(prj: &mut Project, node: i32, link: i32) {
    todo!("stats_updateCriticalTimeCount")
}

/// Called by: routing.
pub fn stats_updateFlowStats(prj: &mut Project, tStep: f64, aDate: DateTime) {
    todo!("stats_updateFlowStats")
}

/// Called by: gwater.
pub fn stats_updateGwaterStats(prj: &mut Project, j: i32, infil: f64, evap: f64, latFlow: f64, deepFlow: f64, theta: f64, waterTable: f64, tStep: f64) {
    todo!("stats_updateGwaterStats")
}

/// Called by: output.
pub fn stats_updateMaxNodeDepth(prj: &mut Project, j: i32, depth: f64) {
    todo!("stats_updateMaxNodeDepth")
}

/// Called by: runoff.
pub fn stats_updateMaxRunoff(prj: &mut Project) {
    todo!("stats_updateMaxRunoff")
}

/// Called by: subcatch.
pub fn stats_updateSubcatchStats(prj: &mut Project, j: i32, rainVol: f64, runonVol: f64, evapVol: f64, infilVol: f64, impervVol: f64, pervVol: f64, runoffVol: f64, runoff: f64) {
    todo!("stats_updateSubcatchStats")
}

/// Called by: routing.
pub fn stats_updateTimeStepStats(prj: &mut Project, tStep: f64, trialsCount: i32, steadyState: i32) {
    todo!("stats_updateTimeStepStats")
}
