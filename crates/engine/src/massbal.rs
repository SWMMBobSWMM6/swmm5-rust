//! massbal.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until massbal.c is translated.

use crate::prelude::*;

/// The file-scope variables of massbal.c.
#[derive(Default)]
pub struct MassbalState {
    pub StepFlowTotals: TRoutingTotals,
    pub StepQualTotals: Vec<TRoutingTotals>,
    pub NodeInflow: Vec<f64>,
    pub NodeOutflow: Vec<f64>,
}

/// Called by: lid, routing.
pub fn massbal_addInflowFlow(prj: &mut Project, type_: i32, q: f64) {
    todo!("massbal_addInflowFlow")
}

/// Called by: lid, routing.
pub fn massbal_addInflowQual(prj: &mut Project, type_: i32, p: i32, w: f64) {
    todo!("massbal_addInflowQual")
}

/// Called by: routing.
pub fn massbal_addLinkLosses(prj: &mut Project, evapLoss: f64, seepLoss: f64) {
    todo!("massbal_addLinkLosses")
}

/// Called by: routing.
pub fn massbal_addNodeLosses(prj: &mut Project, evapLoss: f64, seepLoss: f64) {
    todo!("massbal_addNodeLosses")
}

/// Called by: routing.
pub fn massbal_addOutflowFlow(prj: &mut Project, q: f64, isFlooded: i32) {
    todo!("massbal_addOutflowFlow")
}

/// Called by: routing.
pub fn massbal_addOutflowQual(prj: &mut Project, p: i32, w: f64, isFlooded: i32) {
    todo!("massbal_addOutflowQual")
}

/// Called by: qualrout, treatmnt.
pub fn massbal_addReactedMass(prj: &mut Project, p: i32, w: f64) {
    todo!("massbal_addReactedMass")
}

/// Called by: qualrout.
pub fn massbal_addSeepageLoss(prj: &mut Project, p: i32, w: f64) {
    todo!("massbal_addSeepageLoss")
}

/// Called by: qualrout.
pub fn massbal_addToFinalStorage(prj: &mut Project, p: i32, w: f64) {
    todo!("massbal_addToFinalStorage")
}

/// Called by: swmm5.
pub fn massbal_close(prj: &mut Project) {
    todo!("massbal_close")
}

/// Called by: routing.
pub fn massbal_getStepFlowError(prj: &Project) -> f64 {
    todo!("massbal_getStepFlowError")
}

/// Called by: (entry point).
pub fn massbal_getSysFlows(prj: &Project, f: f64, sysFlows: &mut [f64]) {
    todo!("massbal_getSysFlows")
}

/// Called by: routing.
pub fn massbal_initTimeStepTotals(prj: &mut Project) {
    todo!("massbal_initTimeStepTotals")
}

/// Called by: swmm5.
pub fn massbal_open(prj: &mut Project) -> i32 {
    todo!("massbal_open")
}

/// Called by: swmm5.
pub fn massbal_report(prj: &mut Project) {
    todo!("massbal_report")
}

/// Called by: gwater.
pub fn massbal_updateGwaterTotals(prj: &mut Project, vInfil: f64, vUpperEvap: f64, vLowerEvap: f64, vLowerPerc: f64, vGwater: f64) {
    todo!("massbal_updateGwaterTotals")
}

/// Called by: landuse, lid, runoff, surfqual.
pub fn massbal_updateLoadingTotals(prj: &mut Project, type_: i32, p: i32, w: f64) {
    todo!("massbal_updateLoadingTotals")
}

/// Called by: routing.
pub fn massbal_updateRoutingTotals(prj: &mut Project, tStep: f64) {
    todo!("massbal_updateRoutingTotals")
}

/// Called by: lid, runoff, subcatch.
pub fn massbal_updateRunoffTotals(prj: &mut Project, flowType: i32, v: f64) {
    todo!("massbal_updateRunoffTotals")
}
