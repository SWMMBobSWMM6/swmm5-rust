//! error.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   error.c
//
//   Project: EPA SWMM5
//   Version: 5.2
//   Date:    11/01/21   (Build 5.2.0)
//   Author:  L. Rossman
//
//   Error messages
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Text of Error 217 for control rules modified.
//   Build 5.1.010:
//   - Text of Error 318 for rainfall data files modified.
//   Build 5.1.015:
//   - Added new Error 140 for storage nodes.
//   Build 5.2.0:
//   - Re-designed error message system.
//   - Added new Error 235 for invalid infiltration parameters.
//-----------------------------------------------------------------------------

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

/// error.txt: the text of each error code (C: `ERR(code,string)` cases of
/// the switch in error_getMsg).
const ERRTXT: &[(i32, &[u8])] = &[
    // SWMM 5.2 Error Messages

    (101, b"\n  ERROR 101: memory allocation error."),
    (103, b"\n  ERROR 103: cannot solve KW equations for Link %s."),
    (105, b"\n  ERROR 105: cannot open ODE solver."),
    (107, b"\n  ERROR 107: cannot compute a valid time step."),

    (108, b"\n  ERROR 108: ambiguous outlet ID name for Subcatchment %s."),
    (109, b"\n  ERROR 109: invalid parameter values for Aquifer %s."),
    (110, b"\n  ERROR 110: ground elevation is below water table for Subcatchment %s."),

    (111, b"\n  ERROR 111: invalid length for Conduit %s."),
    (112, b"\n  ERROR 112: elevation drop exceeds length for Conduit %s."),
    (113, b"\n  ERROR 113: invalid roughness for Conduit %s."),
    (114, b"\n  ERROR 114: invalid number of barrels for Conduit %s."),
    (115, b"\n  ERROR 115: adverse slope for Conduit %s."),
    (117, b"\n  ERROR 117: no cross section defined for Link %s."),
    (119, b"\n  ERROR 119: invalid cross section for Link %s."),
    (121, b"\n  ERROR 121: missing or invalid pump curve assigned to Pump %s."),
    (122, b"\n  ERROR 122: startup depth not higher than shutoff depth for Pump %s."),

    (131, b"\n  ERROR 131: the following links form cyclic loops in the drainage system:"),
    (133, b"\n  ERROR 133: Node %s has more than one outlet link."),
    (134, b"\n  ERROR 134: Node %s has illegal DUMMY link connections."),

    (135, b"\n  ERROR 135: Divider %s does not have two outlet links."),
    (136, b"\n  ERROR 136: Divider %s has invalid diversion link."),
    (137, b"\n  ERROR 137: Weir Divider %s has invalid parameters."),
    (138, b"\n  ERROR 138: Node %s has initial depth greater than maximum depth."),
    (139, b"\n  ERROR 139: Regulator %s is the outlet of a non-storage node."),
    (140, b"\n  ERROR 140: Storage node %s has negative volume at full depth."),
    (141, b"\n  ERROR 141: Outfall %s has more than 1 inlet link or an outlet link."),
    (143, b"\n  ERROR 143: Regulator %s has invalid cross-section shape."),
    (145, b"\n  ERROR 145: Drainage system has no acceptable outlet nodes."),

    (151, b"\n  ERROR 151: a Unit Hydrograph in set %s has invalid time base."),
    (153, b"\n  ERROR 153: a Unit Hydrograph in set %s has invalid response ratios."),
    (155, b"\n  ERROR 155: invalid sewer area for RDII at node %s."),

    (156, b"\n  ERROR 156: ambiguous station ID for Rain Gage %s."),
    (157, b"\n  ERROR 157: inconsistent rainfall format for Rain Gage %s."),
    (158, b"\n  ERROR 158: time series for Rain Gage %s is also used by another object."),
    (159, b"\n  ERROR 159: recording interval greater than time series interval for Rain Gage %s."),

    (161, b"\n  ERROR 161: cyclic dependency in treatment functions at node %s."),

    (171, b"\n  ERROR 171: Curve %s has invalid or out of sequence data."),
    (173, b"\n  ERROR 173: Time Series %s has its data out of sequence."),

    (181, b"\n  ERROR 181: invalid Snow Melt Climatology parameters."),
    (182, b"\n  ERROR 182: invalid parameters for Snow Pack %s."),

    (183, b"\n  ERROR 183: no type specified for LID %s."),
    (184, b"\n  ERROR 184: missing layer for LID %s."),
    (185, b"\n  ERROR 185: invalid parameter value for LID %s."),
    (187, b"\n  ERROR 187: LID area exceeds total area for Subcatchment %s."),
    (188, b"\n  ERROR 188: LID capture area exceeds total impervious area for Subcatchment %s."),

    (191, b"\n  ERROR 191: simulation start date comes after ending date."),
    (193, b"\n  ERROR 193: report start date comes after ending date."),
    (195, b"\n  ERROR 195: reporting time step or duration is less than routing time step."),

    (200, b"\n  ERROR 200: one or more errors in input file."),
    (201, b"\n  ERROR 201: too many characters in input line "),
    (203, b"\n  ERROR 203: too few items "),
    (205, b"\n  ERROR 205: invalid keyword %s "),
    (207, b"\n  ERROR 207: duplicate ID name %s "),
    (209, b"\n  ERROR 209: undefined object %s "),
    (211, b"\n  ERROR 211: invalid number %s "),
    (213, b"\n  ERROR 213: invalid date/time %s "),
    (217, b"\n  ERROR 217: control rule clause invalid or out of sequence "),
    (219, b"\n  ERROR 219: data provided for unidentified transect "),
    (221, b"\n  ERROR 221: transect station out of sequence "),
    (223, b"\n  ERROR 223: Transect %s has too few stations."),
    (225, b"\n  ERROR 225: Transect %s has too many stations."),
    (227, b"\n  ERROR 227: Transect %s has no Manning's N."),
    (229, b"\n  ERROR 229: Transect %s has invalid overbank locations."),
    (231, b"\n  ERROR 231: Transect %s has no depth."),
    (233, b"\n  ERROR 233: invalid math expression "),
    (235, b"\n  ERROR 235: invalid infiltration parameters "),

    (301, b"\n  ERROR 301: files share same names."),
    (303, b"\n  ERROR 303: cannot open input file."),
    (305, b"\n  ERROR 305: cannot open report file."),
    (307, b"\n  ERROR 307: cannot open binary results file."),
    (308, b"\n  ERROR 308: amount of output produced will exceed maximum file size."),

    (309, b"\n  ERROR 309: error writing to binary results file."),
    (311, b"\n  ERROR 311: error reading from binary results file."),

    (313, b"\n  ERROR 313: cannot open scratch rainfall interface file."),
    (315, b"\n  ERROR 315: cannot open rainfall interface file %s."),
    (317, b"\n  ERROR 317: cannot open rainfall data file %s."),
    (318, b"\n  ERROR 318: the following line is out of sequence in rainfall data file %s."),
    (319, b"\n  ERROR 319: unknown format for rainfall data file %s."),
    (320, b"\n  ERROR 320: invalid format for rainfall interface file."),
    (321, b"\n  ERROR 321: no data in rainfall interface file for gage %s."),

    (323, b"\n  ERROR 323: cannot open runoff interface file %s."),
    (325, b"\n  ERROR 325: incompatible data found in runoff interface file."),
    (327, b"\n  ERROR 327: attempting to read beyond end of runoff interface file."),
    (329, b"\n  ERROR 329: error in reading from runoff interface file."),

    (331, b"\n  ERROR 331: cannot open hot start interface file %s."),
    (333, b"\n  ERROR 333: incompatible data found in hot start interface file."),
    (335, b"\n  ERROR 335: error in reading from hot start interface file."),

    (336, b"\n  ERROR 336: no climate file specified for evaporation and/or wind speed."),
    (337, b"\n  ERROR 337: cannot open climate file %s."),
    (338, b"\n  ERROR 338: error in reading from climate file %s."),
    (339, b"\n  ERROR 339: attempt to read beyond end of climate file %s."),

    (341, b"\n  ERROR 341: cannot open scratch RDII interface file."),
    (343, b"\n  ERROR 343: cannot open RDII interface file %s."),
    (345, b"\n  ERROR 345: invalid format for RDII interface file."),

    (351, b"\n  ERROR 351: cannot open routing interface file %s."),
    (353, b"\n  ERROR 353: invalid format for routing interface file %s."),
    (355, b"\n  ERROR 355: mis-matched names in routing interface file %s."),
    (357, b"\n  ERROR 357: inflows and outflows interface files have same name."),

    (361, b"\n  ERROR 361: could not open external file used for Time Series %s."),
    (363, b"\n  ERROR 363: invalid data in external file used for Time Series %s."),

    // API Error Keys
    (500, b"\n  ERROR 500: System exception thrown."),
    (501, b"\n  API Error 501: project not opened."),
    (502, b"\n  API Error 502: simulation not started."),
    (503, b"\n  API Error 503: simulation not ended."),
    (504, b"\n  API Error 504: invalid object type."),
    (505, b"\n  API Error 505: invalid object index."),
    (506, b"\n  API Error 506: invalid object name."),
    (507, b"\n  API Error 507: invalid property type."),
    (508, b"\n  API Error 508: invalid property value."),
    (509, b"\n  API Error 509: invalid time period."),
];

/// C: `char* error_getMsg(int errCode, char* msg)`; the message replaces the
/// contents of `msg` (C returns `msg` itself, which callers already hold).
pub fn error_getMsg(errCode: i32, msg: &mut Vec<u8>) {
    msg.clear();
    for &(code, string) in ERRTXT.iter() {
        if code == errCode {
            // case code: strcpy(msg, string); break;
            msg.extend_from_slice(string);
            return;
        }
    }
    // default: strcpy(msg, "");
}

pub fn error_setInpError(prj: &mut Project, errcode: i32, s: &[u8]) -> i32 {
    // strcpy(ErrString, s);
    // NOTE: C's strcpy into the 256-byte ErrString would overflow for a
    // longer token; cio::strcpy truncates instead (tokens come from input
    // lines of at most MAXLINE chars, and SWMM's tokens are short in practice).
    cio::strcpy(&mut prj.error.ErrString, s);
    errcode
}
