//! error.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until error.c is translated.

use crate::prelude::*;

#[derive(Clone)]
pub struct ErrorState {
    /// error.c: `char ErrString[256]`
    pub ErrString: [u8; 256],
}
impl Default for ErrorState {
    fn default() -> Self {
        ErrorState { ErrString: [0; 256] }
    }
}

/// Called by: report, swmm5.
pub fn error_getMsg(errCode: i32, msg: &mut Vec<u8>) {
    todo!("error_getMsg")
}

/// Called by: climate, controls, exfil, gage, gwater, iface, infil, inflow, inlet, input, landuse, lid, link, node, project, rdii, report, snow, street, subcatch, table, transect, treatmnt.
pub fn error_setInpError(prj: &mut Project, errcode: i32, s: &[u8]) -> i32 {
    todo!("error_setInpError")
}
