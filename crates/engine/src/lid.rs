//! lid.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   lid.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     07/13/23   (Build 5.2.4)
//   Author:   L. Rossman
//
//   This module handles all data processing involving LID (Low Impact
//   Development) practices used to treat runoff for individual subcatchments
//   within a project. The actual computation of LID performance is made by
//   functions within the lidproc.c module. See LidTypes below for the types
//   of LIDs that can be modeled.
//
//   An LID process is described by the TLidProc data structure and consists of
//   size-independent design data for the different vertical layers that make
//   up a specific type of LID. The collection of these LID process designs is
//   stored in the LidProcs array.
//
//   When a member of LidProcs is to be deployed in a particular subcatchment,
//   its sizing and treatment data are stored in a TLidUnit data structure.
//   The collection of all TLidUnits deployed in a subcatchment is held in a
//   TLidGroup list data structure. The LidGroups array contains a TLidGroup
//   list for each subcatchment in the project.
//
//   During a runoff time step, each subcatchment calls the lid_getRunoff()
//   function to compute flux rates and a water balance through each layer
//   of each LID unit in the subcatchment. The resulting outflows (runoff,
//   drain flow, evaporation and infiltration) are added to those computed
//   for the non-LID portion of the subcatchment.
//
//   An option exists for the detailed time series of flux rates and storage
//   levels for a specific LID unit to be written to a text file named by the
//   user for viewing outside of the SWMM program.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - More input error reporting added.
//   - Rooftop Disconnection added to the types of LIDs.
//   - LID drain flows are now tracked separately.
//   - LID drain flows can now be routed to separate outlets.
//   - Check added to insure LID flows not returned to nonexistent pervious area.
//   Build 5.1.009:
//   - Fixed bug where LID's could return outflow to non-LID area when LIDs
//     make up entire subcatchment.
//   Build 5.1.010:
//   - Support for new Modified Green Ampt infiltration model added.
//   - Imported variable HasWetLids now properly initialized.
//   - Initial state of reporting (lidUnit->rptFile->wasDry) changed to
//     prevent duplicate printing of first line of detailed report file.
//   Build 5.1.011:
//   - The top of the storage layer is no longer used as a limit for an
//     underdrain offset thus allowing upturned drains to be modeled.
//   - Column headings for the detailed LID report file were modified.
//   Build 5.1.012:
//   - Redefined initialization of wasDry for LID reporting.
//   Build 5.1.013:
//   - Support added for LID units treating pervious area runoff.
//   - Support added for open/closed head levels and multiplier v. head
//     control curve for underdrain flow.
//   - Support added for unclogging permeable pavement at fixed intervals.
//   - Support added for pollutant removal in underdrain flow.
//   Build 5.1.014:
//   - Fixed bug in creating LidProcs when there are no subcatchments.
//   - Fixed bug in adding underdrain pollutant loads to mass balances.
//   Build 5.1.015:
//   - Support added for mutiple infiltration methods within a project.
//   Build 5.2.0:
//   - Covered property added to RAIN_BARREL parameters
//   Build 5.2.3
//   - Fixed double counting of initial water volume in green roof drain mat.
//   Build 5.2.4
//   - Fixed test for invalid data in readDrainData function.
//-----------------------------------------------------------------------------
//
//  Translation notes:
//  - LidGroups is a Vec<Option<Box<LidGroup>>> (None = C's NULL group); a
//    group's TLidList is a Vec<TLidUnit> in C list order (C prepends each
//    new unit, so new units are inserted at index 0).
//  - Where a function walks a group's list while calling functions that take
//    the model, the group is taken out of prj.lid.LidGroups and put back
//    afterwards; none of those callees reads the LID groups.
//  - The LID process passed to lidproc_getOutflow is taken out of
//    prj.lid.LidProcs for the call and put back right after it.

use crate::prelude::*;

const ERR_PAVE_LAYER: &[u8] = b" - check pavement layer parameters";
const ERR_SOIL_LAYER: &[u8] = b" - check soil layer parameters";
const ERR_STOR_LAYER: &[u8] = b" - check storage layer parameters";
const ERR_SWALE_SURF: &[u8] = b" - check swale surface parameters";
const ERR_GREEN_AMPT: &[u8] = b" - check subcatchment Green-Ampt parameters";
#[allow(dead_code)] // defined but unused in lid.c
const ERR_DRAIN_OFFSET: &[u8] = b" - drain offset exceeds storage height";
const ERR_DRAIN_HEADS: &[u8] = b" - invalid drain open/closed heads";
const ERR_SWALE_WIDTH: &[u8] = b" - invalid swale width";

//-----------------------------------------------------------------------------
//  Enumerations
//-----------------------------------------------------------------------------
// enum LidLayerTypes
const SURF: i32 = 0; // surface layer
const SOIL: i32 = 1; // soil layer
const STOR: i32 = 2; // storage layer
const PAVE: i32 = 3; // pavement layer
const DRAINMAT: i32 = 4; // drainage mat layer
const DRAIN: i32 = 5; // underdrain system
const REMOVALS: i32 = 6; // pollutant removals

//// Note: DRAINMAT must be placed before DRAIN so the two keywords can
///        be distinguished from one another when parsing a line of input.

const LidLayerWords: &[&[u8]] = &[b"SURFACE", b"SOIL", b"STORAGE", b"PAVEMENT", b"DRAINMAT", b"DRAIN", b"REMOVALS"];

const LidTypeWords: &[&[u8]] = &[
    b"BC", //bio-retention cell
    b"RG", //rain garden
    b"GR", //green roof
    b"IT", //infiltration trench
    b"PP", //porous pavement
    b"RB", //rain barrel
    b"VS", //vegetative swale
    b"RD", //rooftop disconnection
];

//-----------------------------------------------------------------------------
//  Data Structures
//-----------------------------------------------------------------------------

// LID Group - collection of LID units applied to a specific subcatchment
// (C's TLidList of units is the Vec `lidList`, head first)
#[derive(Default)]
struct LidGroup {
    pervArea: f64,          // amount of pervious area in group (ft2)
    flowToPerv: f64,        // total flow sent to pervious area (cfs)
    oldDrainFlow: f64,      // total drain flow in previous period (cfs)
    newDrainFlow: f64,      // total drain flow in current period (cfs)
    lidList: Vec<TLidUnit>, // list of LID units in the group
}

/// The file-scope variables of lid.c.
#[derive(Default)]
pub struct LidState {
    LidProcs: Vec<TLidProc>,               // array of LID processes
    LidCount: i32,                         // number of LID processes
    LidGroups: Vec<Option<Box<LidGroup>>>, // array of LID process groups
    GroupCount: i32,                       // number of LID groups (subcatchments)

    EvapRate: f64,       // evaporation rate (ft/s)
    NativeInfil: f64,    // native soil infil. rate (ft/s)
    MaxNativeInfil: f64, // native soil infil. rate limit (ft/s)
}

//-----------------------------------------------------------------------------
//  Imported Variables (from SUBCATCH.C)
//-----------------------------------------------------------------------------
// Volumes (ft3) for a subcatchment over a time step: prj.subcatch.Vevap,
// Vpevap, Vinfil, VlidInfil, VlidIn, VlidOut, VlidDrain, VlidReturn;
// HasWetLids (from RUNOFF.C): prj.runoff.HasWetLids.

//=============================================================================

/// Called by: project.
pub fn lid_create(prj: &mut Project, lidCount: i32, subcatchCount: i32) {
    //
    //  Purpose: creates an array of LID objects.
    //  Input:   n = number of LID processes
    //  Output:  none
    //
    let mut j: i32;

    //... assign NULL values to LID arrays
    prj.lid.LidProcs = Vec::new();
    prj.lid.LidGroups = Vec::new();
    prj.lid.LidCount = lidCount;

    //... create LID groups
    prj.lid.GroupCount = subcatchCount;
    if prj.lid.GroupCount > 0 {
        prj.lid.LidGroups = (0..prj.lid.GroupCount).map(|_| None).collect();
    }

    //... initialize LID groups
    j = 0;
    while j < prj.lid.GroupCount {
        prj.lid.LidGroups[j as usize] = None;
        j += 1;
    }

    //... create LID objects
    if prj.lid.LidCount == 0 {
        return;
    }
    prj.lid.LidProcs = (0..prj.lid.LidCount).map(|_| TLidProc::default()).collect();

    //... initialize LID objects
    j = 0;
    while j < prj.lid.LidCount {
        let npollut = prj.Nobjects[POLLUT as usize];
        let lp = &mut prj.lid.LidProcs[j as usize];
        lp.lidType = -1;
        lp.surface.thickness = 0.0;
        lp.surface.voidFrac = 1.0;
        lp.surface.roughness = 0.0;
        lp.surface.surfSlope = 0.0;
        lp.pavement.thickness = 0.0;
        lp.soil.thickness = 0.0;
        lp.storage.thickness = 0.0;
        lp.storage.kSat = 0.0;
        lp.drain.coeff = 0.0;
        lp.drain.offset = 0.0;
        lp.drainMat.thickness = 0.0;
        lp.drainMat.roughness = 0.0;
        lp.drainRmvl = Vec::new();
        lp.drainRmvl = vec![0.0; npollut as usize];
        j += 1;
    }
}

//=============================================================================

/// Called by: project.
pub fn lid_delete(prj: &mut Project) {
    //
    //  Purpose: deletes all LID objects
    //  Input:   none
    //  Output:  none
    //
    let mut j: i32;
    j = 0;
    while j < prj.lid.GroupCount {
        freeLidGroup(prj, j);
        j += 1;
    }
    prj.lid.LidGroups = Vec::new();
    j = 0;
    while j < prj.lid.LidCount {
        prj.lid.LidProcs[j as usize].drainRmvl = Vec::new();
        j += 1;
    }
    prj.lid.LidProcs = Vec::new();
    prj.lid.GroupCount = 0;
    prj.lid.LidCount = 0;
}

//=============================================================================

fn freeLidGroup(prj: &mut Project, j: i32) {
    //
    //  Purpose: frees all LID units associated with a subcatchment.
    //  Input:   j = group (or subcatchment) index
    //  Output:  none
    //
    //  (Dropping a unit's report file closes it, as C's fclose does.)
    //
    if prj.lid.LidGroups[j as usize].is_none() {
        return;
    }
    if let Some(lidGroup) = prj.lid.LidGroups[j as usize].as_mut() {
        for lidUnit in lidGroup.lidList.iter_mut() {
            if let Some(rptFile) = lidUnit.rptFile.as_mut() {
                if rptFile.file.is_some() {
                    rptFile.file = None;
                }
            }
            lidUnit.rptFile = None;
        }
        lidGroup.lidList.clear();
    }
    prj.lid.LidGroups[j as usize] = None;
}

//=============================================================================

/// Called by: input.
pub fn lid_readProcParams(prj: &mut Project, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads LID process information from line of input data file
    //  Input:   toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format for first line that defines a LID process is:
    //    LID_ID  LID_Type
    //
    //  Followed by some combination of lines below depending on LID_Type:
    //    LID_ID  SURFACE   <parameters>
    //    LID_ID  PAVEMENT  <parameters>
    //    LID_ID  SOIL      <parameters>
    //    LID_ID  STORAGE   <parameters>
    //    LID_ID  DRAIN     <parameters>
    //    LID_ID  DRAINMAT  <parameters>
    //    LID_ID  REMOVALS  <parameters>
    //
    let j: i32;
    let mut m: i32;

    // --- check for minimum number of tokens
    if ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that LID exists in database
    j = project_findObject(prj, LID, &toks[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &toks[0]);
    }

    // --- assign ID if not done yet
    //     (C: ID == NULL; a found ID is never empty)
    if prj.lid.LidProcs[j as usize].ID.is_empty() {
        if let Some(id) = project_findID(prj, LID, &toks[0]) {
            prj.lid.LidProcs[j as usize].ID = id;
        }
    }

    // --- check if second token is the type of LID
    m = findmatch(&toks[1], LidTypeWords);
    if m >= 0 {
        prj.lid.LidProcs[j as usize].lidType = m;
        return 0;
    }
    // --- check if second token is name of LID layer
    else {
        m = findmatch(&toks[1], LidLayerWords);
    }

    // --- read input parameters for the identified layer
    match m {
        SURF => return readSurfaceData(prj, j, toks, ntoks),
        SOIL => return readSoilData(prj, j, toks, ntoks),
        STOR => return readStorageData(prj, j, toks, ntoks),
        PAVE => return readPavementData(prj, j, toks, ntoks),
        DRAIN => return readDrainData(prj, j, toks, ntoks),
        DRAINMAT => return readDrainMatData(prj, j, toks, ntoks),
        REMOVALS => return readRemovalsData(prj, j, toks, ntoks),
        _ => {}
    }
    error_setInpError(prj, ERR_KEYWORD, &toks[1])
}

//=============================================================================

/// Called by: input.
pub fn lid_readGroupParams(prj: &mut Project, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads input data for a LID unit placed in a subcatchment.
    //  Input:   toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format of input data line is:
    //    Subcatch_ID  LID_ID  Number  Area  Width  InitSat  FromImp  ToPerv
    //                                              (RptFile  DrainTo  FromPerv)
    //  where:
    //    Subcatch_ID    = name of subcatchment
    //    LID_ID         = name of LID process
    //    Number     (n) = number of replicate units
    //    Area    (x[0]) = area of each unit
    //    Width   (x[1]) = outflow width of each unit
    //    InitSat (x[2]) = % that LID is initially saturated
    //    FromImp (x[3]) = % of impervious runoff sent to LID
    //    ToPerv  (x[4]) = 1 if outflow goes to pervious sub-area; 0 if not
    //    RptFile        = name of detailed results file (optional)
    //    DrainTo        = name of subcatch/node for drain flow (optional)
    //    FromPerv (x[5]) = % of pervious runoff sent to LID
    //
    let mut i: i32;
    let j: i32;
    let k: i32;
    let n: i32;
    let mut x = [0.0f64; 6];
    let mut fname: Option<&[u8]> = None;
    let mut drainSubcatch: i32 = -1;
    let mut drainNode: i32 = -1;

    //... check for valid number of input tokens
    if ntoks < 8 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    //... find subcatchment
    j = project_findObject(prj, SUBCATCH, &toks[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &toks[0]);
    }

    //... find LID process in list of LID processes
    k = project_findObject(prj, LID, &toks[1]);
    if k < 0 {
        return error_setInpError(prj, ERR_NAME, &toks[1]);
    }

    //... get number of replicates
    n = cio::atoi(&toks[2]);
    if n < 0 {
        return error_setInpError(prj, ERR_NUMBER, &toks[2]);
    }
    if n == 0 {
        return 0;
    }

    //... convert next 4 tokens to doubles
    i = 3;
    while i <= 7 {
        if getDouble(&toks[i as usize], &mut x[(i - 3) as usize]) == 0 || x[(i - 3) as usize] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[i as usize]);
        }
        i += 1;
    }

    //... check for valid percentages on tokens 5 & 6 (x[2] & x[3])
    i = 2;
    while i <= 3 {
        if x[i as usize] > 100.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[(i + 3) as usize]);
        }
        i += 1;
    }

    //... read optional report file name
    if ntoks >= 9 && cio::strcmp(&toks[8], b"*") != 0 {
        fname = Some(toks[8].as_slice());
    }

    //... read optional underdrain outlet
    if ntoks >= 10 && cio::strcmp(&toks[9], b"*") != 0 {
        drainSubcatch = project_findObject(prj, SUBCATCH, &toks[9]);
        if drainSubcatch < 0 {
            drainNode = project_findObject(prj, NODE, &toks[9]);
            if drainNode < 0 {
                return error_setInpError(prj, ERR_NAME, &toks[9]);
            }
        }
    }

    //... read percent of pervious area treated by LID unit
    x[5] = 0.0;
    if ntoks >= 11 {
        if getDouble(&toks[10], &mut x[5]) == 0 || x[5] < 0.0 || x[5] > 100.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[10]);
        }
    }

    //... create a new LID unit and add it to the subcatchment's LID group
    addLidUnit(prj, j, k, n, &x, fname, drainSubcatch, drainNode)
}

//=============================================================================

fn addLidUnit(
    prj: &mut Project,
    j: i32,
    k: i32,
    n: i32,
    x: &[f64],
    fname: Option<&[u8]>,
    drainSubcatch: i32,
    drainNode: i32,
) -> i32 {
    //
    //  Purpose: adds an LID unit to a subcatchment's LID group.
    //  Input:   j = subcatchment index
    //           k = LID control index
    //           n = number of replicate units
    //           x = LID unit's parameters
    //           fname = name of detailed performance report file
    //           drainSubcatch = index of subcatchment receiving underdrain flow
    //           drainNode = index of node receiving underdrain flow
    //  Output:  returns an error code
    //
    //  (C's malloc'd group and unit start out uninitialized apart from the
    //   fields set here; every other field is written in lid_validate or
    //   lid_initState before it is read, so Default zeros are equivalent.)
    //
    let ucfLength = UCF(prj, LENGTH);

    //... create a LID group (pointer to an LidGroup struct)
    //    if one doesn't already exist
    if prj.lid.LidGroups[j as usize].is_none() {
        let mut lidGroup = Box::new(LidGroup::default());
        lidGroup.lidList = Vec::new();
        prj.lid.LidGroups[j as usize] = Some(lidGroup);
    }
    let lidGroup = prj.lid.LidGroups[j as usize].as_mut().unwrap();

    //... create a new LID unit to add to the group
    let mut lidUnit = TLidUnit::default();
    lidUnit.rptFile = None;

    //... add the LID unit to the group
    lidGroup.lidList.insert(0, lidUnit);
    let lidUnit = &mut lidGroup.lidList[0];

    //... assign parameter values to LID unit
    lidUnit.lidIndex = k;
    lidUnit.number = n;
    lidUnit.area = x[0] / SQR(ucfLength);
    lidUnit.fullWidth = x[1] / ucfLength;
    lidUnit.initSat = x[2] / 100.0;
    lidUnit.fromImperv = x[3] / 100.0;
    lidUnit.toPerv = (x[4] > 0.0) as i32;
    lidUnit.fromPerv = x[5] / 100.0;
    lidUnit.drainSubcatch = drainSubcatch;
    lidUnit.drainNode = drainNode;

    //... open report file if it was supplied
    if let Some(fname) = fname {
        if createLidRptFile(lidUnit, fname) == 0 {
            return error_setInpError(prj, ERR_RPT_FILE, fname);
        }
    }
    0
}

//=============================================================================

fn createLidRptFile(lidUnit: &mut TLidUnit, fname: &[u8]) -> i32 {
    let mut rptFile = Box::new(TLidRptFile::default());
    rptFile.file = CFile::open(fname, "wt");
    let opened = rptFile.file.is_some();
    lidUnit.rptFile = Some(rptFile);
    if !opened {
        return 0;
    }
    1
}

//=============================================================================

fn readSurfaceData(prj: &mut Project, j: i32, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads surface layer data for a LID process from line of input
    //           data file
    //  Input:   j = LID process index
    //           toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format of data is:
    //  LID_ID  SURFACE  StorageHt  VegVolFrac  Roughness  SurfSlope  SideSlope
    //
    let mut i: i32;
    let mut x = [0.0f64; 5];

    if ntoks < 7 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    i = 2;
    while i < 7 {
        if getDouble(&toks[i as usize], &mut x[(i - 2) as usize]) == 0 || x[(i - 2) as usize] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[i as usize]);
        }
        i += 1;
    }
    if x[1] >= 1.0 {
        return error_setInpError(prj, ERR_NUMBER, &toks[3]);
    }
    if x[0] == 0.0 {
        x[1] = 0.0;
    }

    let ucfRainDepth = UCF(prj, RAINDEPTH);
    let lp = &mut prj.lid.LidProcs[j as usize];
    lp.surface.thickness = x[0] / ucfRainDepth;
    lp.surface.voidFrac = 1.0 - x[1];
    lp.surface.roughness = x[2];
    lp.surface.surfSlope = x[3] / 100.0;
    lp.surface.sideSlope = x[4];
    0
}

//=============================================================================

fn readPavementData(prj: &mut Project, j: i32, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads pavement layer data for a LID process from line of input
    //           data file
    //  Input:   j = LID process index
    //           toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format of data is:
    //    LID_ID PAVEMENT  Thickness  VoidRatio  FracImperv  Permeability  ClogFactor
    //                                                        (RegenDays RegenDegree)
    //
    let mut i: i32;
    let mut x = [0.0f64; 7];

    if ntoks < 7 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    i = 2;
    while i < 7 {
        if getDouble(&toks[i as usize], &mut x[(i - 2) as usize]) == 0 || x[(i - 2) as usize] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[i as usize]);
        }
        i += 1;
    }

    // ... read optional clogging regeneration properties
    x[5] = 0.0;
    if ntoks > 7 {
        if getDouble(&toks[7], &mut x[5]) == 0 || x[5] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[7]);
        }
    }
    x[6] = 0.0;
    if ntoks > 8 {
        if getDouble(&toks[8], &mut x[6]) == 0 || x[6] < 0.0 || x[6] > 1.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[8]);
        }
    }

    //... convert void ratio to void fraction
    x[1] = x[1] / (x[1] + 1.0);

    let ucfRainDepth = UCF(prj, RAINDEPTH);
    let ucfRainfall = UCF(prj, RAINFALL);
    let lp = &mut prj.lid.LidProcs[j as usize];
    lp.pavement.thickness = x[0] / ucfRainDepth;
    lp.pavement.voidFrac = x[1];
    lp.pavement.impervFrac = x[2];
    lp.pavement.kSat = x[3] / ucfRainfall;
    lp.pavement.clogFactor = x[4];
    lp.pavement.regenDays = x[5];
    lp.pavement.regenDegree = x[6];
    0
}

//=============================================================================

fn readSoilData(prj: &mut Project, j: i32, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads soil layer data for a LID process from line of input
    //           data file
    //  Input:   j = LID process index
    //           toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format of data is:
    //    LID_ID  SOIL  Thickness  Porosity  FieldCap  WiltPt Ksat  Kslope  Suction
    //
    let mut i: i32;
    let mut x = [0.0f64; 7];

    if ntoks < 9 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    i = 2;
    while i < 9 {
        if getDouble(&toks[i as usize], &mut x[(i - 2) as usize]) == 0 || x[(i - 2) as usize] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[i as usize]);
        }
        i += 1;
    }
    let ucfRainDepth = UCF(prj, RAINDEPTH);
    let ucfRainfall = UCF(prj, RAINFALL);
    let lp = &mut prj.lid.LidProcs[j as usize];
    lp.soil.thickness = x[0] / ucfRainDepth;
    lp.soil.porosity = x[1];
    lp.soil.fieldCap = x[2];
    lp.soil.wiltPoint = x[3];
    lp.soil.kSat = x[4] / ucfRainfall;
    lp.soil.kSlope = x[5];
    lp.soil.suction = x[6] / ucfRainDepth;
    0
}

//=============================================================================

fn readStorageData(prj: &mut Project, j: i32, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads drainage layer data for a LID process from line of input
    //           data file
    //  Input:   j = LID process index
    //           toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format of data is:
    //    LID_ID STORAGE  Thickness  VoidRatio  Ksat  ClogFactor  (YES/NO)
    //
    let mut i: i32;
    let mut covered = FALSE;
    let mut x = [0.0f64; 6];

    //... read numerical parameters
    if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    i = 2;
    while i < 6 {
        if getDouble(&toks[i as usize], &mut x[(i - 2) as usize]) == 0 || x[(i - 2) as usize] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[i as usize]);
        }
        i += 1;
    }

    //... check if rain barrel is covered
    if ntoks > 6 {
        if match_(&toks[6], w_YES) != 0 {
            covered = TRUE;
        }
    }

    //... convert void ratio to void fraction
    x[1] = x[1] / (x[1] + 1.0);

    //... save parameters to LID storage layer structure
    let ucfRainDepth = UCF(prj, RAINDEPTH);
    let ucfRainfall = UCF(prj, RAINFALL);
    let lp = &mut prj.lid.LidProcs[j as usize];
    lp.storage.thickness = x[0] / ucfRainDepth;
    lp.storage.voidFrac = x[1];
    lp.storage.kSat = x[2] / ucfRainfall;
    lp.storage.clogFactor = x[3];
    lp.storage.covered = covered;
    0
}

//=============================================================================

fn readDrainData(prj: &mut Project, j: i32, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads underdrain data for a LID process from line of input
    //           data file
    //  Input:   j = LID process index
    //           toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format of data is:
    //    LID_ID DRAIN  coeff  expon  offset  delay hOpen hClose curve
    //
    let mut i: i32;
    let mut x = [0.0f64; 6];

    //... read numerical parameters
    if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    i = 0;
    while i < 6 {
        x[i as usize] = 0.0;
        i += 1;
    }
    i = 2;
    while i < 8 {
        if (ntoks > i) && (getDouble(&toks[i as usize], &mut x[(i - 2) as usize]) == 0 || x[(i - 2) as usize] < 0.0) {
            return error_setInpError(prj, ERR_NUMBER, &toks[i as usize]);
        }
        i += 1;
    }

    i = -1;
    if ntoks >= 9 {
        i = project_findObject(prj, CURVE, &toks[8]);
        if i < 0 {
            return error_setInpError(prj, ERR_NAME, &toks[8]);
        }
    }

    //... save parameters to LID drain layer structure
    let ucfRainDepth = UCF(prj, RAINDEPTH);
    let lp = &mut prj.lid.LidProcs[j as usize];
    lp.drain.coeff = x[0];
    lp.drain.expon = x[1];
    lp.drain.offset = x[2] / ucfRainDepth;
    lp.drain.delay = x[3] * 3600.0;
    lp.drain.hOpen = x[4] / ucfRainDepth;
    lp.drain.hClose = x[5] / ucfRainDepth;
    lp.drain.qCurve = i;
    0
}

//=============================================================================

fn readDrainMatData(prj: &mut Project, j: i32, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads drainage mat data for a LID process from line of input
    //           data file
    //  Input:   j = LID process index
    //           toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format of data is:
    //    LID_ID DRAINMAT  thickness  voidRatio  roughness
    //
    let mut i: i32;
    let mut x = [0.0f64; 3];

    //... read numerical parameters
    if ntoks < 5 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    if prj.lid.LidProcs[j as usize].lidType != GREEN_ROOF {
        return 0;
    }
    i = 2;
    while i < 5 {
        if getDouble(&toks[i as usize], &mut x[(i - 2) as usize]) == 0 || x[(i - 2) as usize] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[i as usize]);
        }
        i += 1;
    }

    //... save parameters to LID drain layer structure
    let ucfRainDepth = UCF(prj, RAINDEPTH);
    let lp = &mut prj.lid.LidProcs[j as usize];
    lp.drainMat.thickness = x[0] / ucfRainDepth;
    lp.drainMat.voidFrac = x[1];
    lp.drainMat.roughness = x[2];
    0
}

//=============================================================================

fn readRemovalsData(prj: &mut Project, j: i32, toks: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Purpose: reads pollutant removal data for a LID process from line of input
    //           data file
    //  Input:   j = LID process index
    //           toks = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns error code
    //
    //  Format of data is:
    //    LID_ID REMOVALS  pollut1  %removal1  pollut2  %removal2  ...
    //
    let mut i: i32 = 2;
    let mut p: i32;
    let mut rmvl: f64 = 0.0;

    //... start with 3rd token
    if ntoks < 4 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    while ntoks > i {
        //... find pollutant index from its name
        p = project_findObject(prj, POLLUT, &toks[i as usize]);
        if p < 0 {
            return error_setInpError(prj, ERR_NAME, &toks[i as usize]);
        }

        //... check that a next token exists
        i += 1;
        if ntoks == i {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }

        //... get the % removal value from the next token
        if getDouble(&toks[i as usize], &mut rmvl) == 0 || rmvl < 0.0 || rmvl > 100.0 {
            return error_setInpError(prj, ERR_NUMBER, &toks[i as usize]);
        }

        //... save the pollutant removal for the LID process as a fraction
        prj.lid.LidProcs[j as usize].drainRmvl[p as usize] = rmvl / 100.0;
        i += 1;
    }
    0
}

//=============================================================================

/// input.c's `match` (not exported by the input stub; private copy).
fn match_(str: &[u8], substr: &[u8]) -> i32 {
    //
    //  Input:   str = character string being searched
    //           substr = sub-string being searched for
    //  Output:  returns 1 if sub-string found, 0 if not
    //  Purpose: sees if a sub-string of characters appears in a string
    //           (not case sensitive).
    //
    let s = cio::cstr(str);
    let sub = cio::cstr(substr);
    let at = |v: &[u8], n: usize| -> u8 { if n < v.len() { v[n] } else { 0 } };
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;

    // --- fail if substring is empty
    if at(sub, 0) == 0 {
        return 0;
    }

    // --- skip leading blanks of str
    k = 0;
    while at(s, k) != 0 {
        if at(s, k) != b' ' {
            break;
        }
        k += 1;
    }

    // --- check if substr matches remainder of str
    i = k;
    j = 0;
    while at(sub, j) != 0 {
        if at(s, i) == 0 || UCHAR(at(s, i)) != UCHAR(at(sub, j)) {
            return 0;
        }
        i += 1;
        j += 1;
    }
    1
}

//=============================================================================

/// Called by: inputrpt.
pub fn lid_writeSummary(prj: &mut Project) {
    //
    //  Purpose: writes summary of LID processes used to report file.
    //  Input:   none
    //  Output:  none
    //
    let mut j: i32;
    let mut k: i32;
    let mut pctArea: f64;

    cprintf!(prj.Frpt, b"\n");
    cprintf!(prj.Frpt, b"\n");
    cprintf!(prj.Frpt, b"\n  *******************");
    cprintf!(prj.Frpt, b"\n  LID Control Summary");
    cprintf!(prj.Frpt, b"\n  *******************");

    cprintf!(
        prj.Frpt,
        b"\n                                   No. of        Unit        Unit      %% Area    %% Imperv      %% Perv"
    ); //(5.1.013)
    cprintf!(
        prj.Frpt, //
        b"\n  Subcatchment     LID Control      Units        Area       Width     Covered     Treated     Treated"
    ); //
    cprintf!(
        prj.Frpt, //
        b"\n  ---------------------------------------------------------------------------------------------------"
    ); //

    j = 0;
    while j < prj.lid.GroupCount {
        let nUnits = match &prj.lid.LidGroups[j as usize] {
            None => {
                j += 1;
                continue;
            }
            Some(lidGroup) => lidGroup.lidList.len(),
        };
        for idx in 0..nUnits {
            let (s1, s2) = {
                let lidUnit = &prj.lid.LidGroups[j as usize].as_ref().unwrap().lidList[idx];
                k = lidUnit.lidIndex;
                pctArea = lidUnit.area * lidUnit.number as f64 / prj.Subcatch[j as usize].area * 100.0;
                let s1 = cformat!(b"\n  %-16s %-16s", prj.Subcatch[j as usize].ID, prj.lid.LidProcs[k as usize].ID);
                let s2 = cformat!(
                    b"%6d  %10.2f  %10.2f  %10.2f  %10.2f  %10.2f",
                    lidUnit.number,
                    lidUnit.area * SQR(UCF(prj, LENGTH)),
                    lidUnit.fullWidth * UCF(prj, LENGTH),
                    pctArea,
                    lidUnit.fromImperv * 100.0,
                    lidUnit.fromPerv * 100.0
                );
                (s1, s2)
            };
            prj.Frpt.write_bytes(&s1);
            prj.Frpt.write_bytes(&s2);
        }
        j += 1;
    }
}

//=============================================================================

/// Called by: project.
pub fn lid_validate(prj: &mut Project) {
    //
    //  Purpose: validates LID process and group parameters.
    //  Input:   none
    //  Output:  none
    //
    let mut j: i32;
    j = 0;
    while j < prj.lid.LidCount {
        validateLidProc(prj, j);
        j += 1;
    }
    j = 0;
    while j < prj.lid.GroupCount {
        validateLidGroup(prj, j);
        j += 1;
    }
}

//=============================================================================

fn validateLidProc(prj: &mut Project, j: i32) {
    //
    //  Purpose: validates LID process parameters.
    //  Input:   j = LID process index
    //  Output:  none
    //
    //  Note: at each report_writeErrorMsg(ERR_LID_PARAMS, Msg) C passes the
    //  global Msg as the %s argument, and report_writeErrorMsg overwrites Msg
    //  with the error template (error_getMsg) before printing it, so C prints
    //  the template itself for %s. The calls below pass that template.
    //
    let ju = j as usize;
    let mut layerMissing = FALSE;

    //... check that LID type was supplied
    if prj.lid.LidProcs[ju].lidType < 0 {
        let id = prj.lid.LidProcs[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_LID_TYPE, &id);
        return;
    }

    //... check that required layers were defined
    match prj.lid.LidProcs[ju].lidType {
        BIO_CELL | RAIN_GARDEN => {
            if prj.lid.LidProcs[ju].soil.thickness <= 0.0 {
                layerMissing = TRUE;
            }
        }
        GREEN_ROOF => {
            if prj.lid.LidProcs[ju].soil.thickness <= 0.0 {
                layerMissing = TRUE;
            }
            if prj.lid.LidProcs[ju].drainMat.thickness <= 0.0 {
                layerMissing = TRUE;
            }
        }
        POROUS_PAVEMENT => {
            if prj.lid.LidProcs[ju].pavement.thickness <= 0.0 {
                layerMissing = TRUE;
            }
        }
        INFIL_TRENCH => {
            if prj.lid.LidProcs[ju].storage.thickness <= 0.0 {
                layerMissing = TRUE;
            }
        }
        _ => {}
    }
    if layerMissing != 0 {
        let id = prj.lid.LidProcs[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_LID_LAYER, &id);
        return;
    }

    //... check pavement layer parameters
    if prj.lid.LidProcs[ju].lidType == POROUS_PAVEMENT {
        let lp = &prj.lid.LidProcs[ju];
        if lp.pavement.thickness <= 0.0
            || lp.pavement.kSat <= 0.0
            || lp.pavement.voidFrac <= 0.0
            || lp.pavement.voidFrac > 1.0
            || lp.pavement.impervFrac > 1.0
        {
            let id = prj.lid.LidProcs[ju].ID.clone();
            sstrncpy(&mut prj.Msg, &id, MAXMSG as usize);
            sstrcat(&mut prj.Msg, ERR_PAVE_LAYER, MAXMSG as usize);
            // C prints the error template for %s (Msg is overwritten first)
            let mut tpl = Vec::new();
            error_getMsg(ERR_LID_PARAMS, &mut tpl);
            report_writeErrorMsg(prj, ERR_LID_PARAMS, &tpl);
        }
    }

    //... check soil layer parameters
    if prj.lid.LidProcs[ju].soil.thickness > 0.0 {
        let lp = &prj.lid.LidProcs[ju];
        if lp.soil.porosity <= 0.0
            || lp.soil.fieldCap >= lp.soil.porosity
            || lp.soil.wiltPoint >= lp.soil.fieldCap
            || lp.soil.kSat <= 0.0
            || lp.soil.kSlope < 0.0
        {
            let id = prj.lid.LidProcs[ju].ID.clone();
            sstrncpy(&mut prj.Msg, &id, MAXMSG as usize);
            sstrcat(&mut prj.Msg, ERR_SOIL_LAYER, MAXMSG as usize);
            // C prints the error template for %s (Msg is overwritten first)
            let mut tpl = Vec::new();
            error_getMsg(ERR_LID_PARAMS, &mut tpl);
            report_writeErrorMsg(prj, ERR_LID_PARAMS, &tpl);
        }
    }

    //... check storage layer parameters
    if prj.lid.LidProcs[ju].storage.thickness > 0.0 {
        if prj.lid.LidProcs[ju].storage.voidFrac <= 0.0 || prj.lid.LidProcs[ju].storage.voidFrac > 1.0 {
            let id = prj.lid.LidProcs[ju].ID.clone();
            sstrncpy(&mut prj.Msg, &id, MAXMSG as usize);
            sstrcat(&mut prj.Msg, ERR_STOR_LAYER, MAXMSG as usize);
            // C prints the error template for %s (Msg is overwritten first)
            let mut tpl = Vec::new();
            error_getMsg(ERR_LID_PARAMS, &mut tpl);
            report_writeErrorMsg(prj, ERR_LID_PARAMS, &tpl);
        }
    }
    //... if no storage layer adjust void fraction and drain offset
    else {
        prj.lid.LidProcs[ju].storage.voidFrac = 1.0;
        prj.lid.LidProcs[ju].drain.offset = 0.0;
    }

    //... check for invalid drain open/closed heads
    if prj.lid.LidProcs[ju].drain.hOpen > 0.0 && prj.lid.LidProcs[ju].drain.hOpen <= prj.lid.LidProcs[ju].drain.hClose {
        let id = prj.lid.LidProcs[ju].ID.clone();
        sstrncpy(&mut prj.Msg, &id, MAXMSG as usize);
        sstrcat(&mut prj.Msg, ERR_DRAIN_HEADS, MAXMSG as usize);
        // C prints the error template for %s (Msg is overwritten first)
        let mut tpl = Vec::new();
        error_getMsg(ERR_LID_PARAMS, &mut tpl);
        report_writeErrorMsg(prj, ERR_LID_PARAMS, &tpl);
    }

    //... compute the surface layer's overland flow constant (alpha)
    if prj.lid.LidProcs[ju].lidType == VEG_SWALE {
        if prj.lid.LidProcs[ju].surface.roughness * prj.lid.LidProcs[ju].surface.surfSlope <= 0.0
            || prj.lid.LidProcs[ju].surface.thickness == 0.0
        {
            let id = prj.lid.LidProcs[ju].ID.clone();
            sstrncpy(&mut prj.Msg, &id, MAXMSG as usize);
            sstrcat(&mut prj.Msg, ERR_SWALE_SURF, MAXMSG as usize);
            // C prints the error template for %s (Msg is overwritten first)
            let mut tpl = Vec::new();
            error_getMsg(ERR_LID_PARAMS, &mut tpl);
            report_writeErrorMsg(prj, ERR_LID_PARAMS, &tpl);
        } else {
            let lp = &mut prj.lid.LidProcs[ju];
            lp.surface.alpha = 1.49 * lp.surface.surfSlope.sqrt() / lp.surface.roughness;
        }
    } else {
        //... compute surface overland flow coeff.
        let lp = &mut prj.lid.LidProcs[ju];
        if lp.surface.roughness > 0.0 {
            lp.surface.alpha = 1.49 / lp.surface.roughness * lp.surface.surfSlope.sqrt();
        } else {
            lp.surface.alpha = 0.0;
        }
    }

    let lp = &mut prj.lid.LidProcs[ju];

    //... compute drainage mat layer's flow coeff.
    if lp.drainMat.roughness > 0.0 {
        lp.drainMat.alpha = 1.49 / lp.drainMat.roughness * lp.surface.surfSlope.sqrt();
    } else {
        lp.drainMat.alpha = 0.0;
    }

    //... convert clogging factors to void volume basis
    if lp.pavement.thickness > 0.0 {
        lp.pavement.clogFactor *= lp.pavement.thickness * lp.pavement.voidFrac * (1.0 - lp.pavement.impervFrac);
    }
    if lp.storage.thickness > 0.0 {
        lp.storage.clogFactor *= lp.storage.thickness * lp.storage.voidFrac;
    } else {
        lp.storage.clogFactor = 0.0;
    }

    //... for certain LID types, immediate overflow of excess surface water
    //    occurs if either the surface roughness or slope is zero
    lp.surface.canOverflow = TRUE as i8;
    match lp.lidType {
        ROOF_DISCON => lp.surface.canOverflow = FALSE as i8,
        INFIL_TRENCH | POROUS_PAVEMENT | BIO_CELL | RAIN_GARDEN | GREEN_ROOF => {
            if lp.surface.alpha > 0.0 {
                lp.surface.canOverflow = FALSE as i8;
            }
        }
        _ => {}
    }

    //... rain barrels have 100% void space and impermeable bottom
    if lp.lidType == RAIN_BARREL {
        lp.storage.voidFrac = 1.0;
        lp.storage.kSat = 0.0;
    }

    //... set storage layer parameters of a green roof
    if lp.lidType == GREEN_ROOF {
        lp.storage.thickness = lp.drainMat.thickness;
        lp.storage.voidFrac = lp.drainMat.voidFrac;
        lp.storage.clogFactor = 0.0;
        lp.storage.kSat = 0.0;
    }
}

//=============================================================================

fn validateLidGroup(prj: &mut Project, j: i32) {
    //
    //  Purpose: validates properties of LID units grouped in a subcatchment.
    //  Input:   j = subcatchment index
    //  Output:  returns 1 if data are valid, 0 if not
    //
    //  (See validateLidProc for why ERR_LID_PARAMS is passed its template.)
    //
    let ju = j as usize;
    let mut k: i32;
    let mut p = [0.0f64; 3];
    let totalArea = prj.Subcatch[ju].area;
    let mut totalLidArea = 0.0f64;
    let mut fromImperv = 0.0f64;
    let mut fromPerv = 0.0f64;

    let mut lidGroup = match prj.lid.LidGroups[ju].take() {
        None => return,
        Some(g) => g,
    };
    for lidUnit in lidGroup.lidList.iter_mut() {
        k = lidUnit.lidIndex;
        let ku = k as usize;

        //... update contributing fractions
        totalLidArea += lidUnit.area * lidUnit.number as f64;
        fromImperv += lidUnit.fromImperv;
        fromPerv += lidUnit.fromPerv;

        //... assign biocell soil layer infiltration parameters
        lidUnit.soilInfil.Ks = 0.0;
        if prj.lid.LidProcs[ku].soil.thickness > 0.0 {
            p[0] = prj.lid.LidProcs[ku].soil.suction * UCF(prj, RAINDEPTH);
            p[1] = prj.lid.LidProcs[ku].soil.kSat * UCF(prj, RAINFALL);
            p[2] = (prj.lid.LidProcs[ku].soil.porosity - prj.lid.LidProcs[ku].soil.wiltPoint) * (1.0 - lidUnit.initSat);
            if grnampt_setParams(prj, &mut lidUnit.soilInfil, &p) == FALSE {
                let id = prj.lid.LidProcs[ku].ID.clone();
                sstrncpy(&mut prj.Msg, &id, MAXMSG as usize);
                sstrcat(&mut prj.Msg, ERR_SOIL_LAYER, MAXMSG as usize);
                // C prints the error template for %s (Msg is overwritten first)
                let mut tpl = Vec::new();
                error_getMsg(ERR_LID_PARAMS, &mut tpl);
                report_writeErrorMsg(prj, ERR_LID_PARAMS, &tpl);
            }
        }

        //... assign vegetative swale infiltration parameters
        if prj.lid.LidProcs[ku].lidType == VEG_SWALE {
            if prj.Subcatch[ju].infilModel == GREEN_AMPT || prj.Subcatch[ju].infilModel == MOD_GREEN_AMPT {
                grnampt_getParams(prj, j, &mut p);
                if grnampt_setParams(prj, &mut lidUnit.soilInfil, &p) == FALSE {
                    let id = prj.lid.LidProcs[ku].ID.clone();
                    sstrncpy(&mut prj.Msg, &id, MAXMSG as usize);
                    sstrcat(&mut prj.Msg, ERR_GREEN_AMPT, MAXMSG as usize);
                    // C prints the error template for %s (Msg is overwritten first)
                    let mut tpl = Vec::new();
                    error_getMsg(ERR_LID_PARAMS, &mut tpl);
                    report_writeErrorMsg(prj, ERR_LID_PARAMS, &tpl);
                }
            }
            if lidUnit.fullWidth <= 0.0 {
                let id = prj.lid.LidProcs[ku].ID.clone();
                sstrncpy(&mut prj.Msg, &id, MAXMSG as usize);
                sstrcat(&mut prj.Msg, ERR_SWALE_WIDTH, MAXMSG as usize);
                // C prints the error template for %s (Msg is overwritten first)
                let mut tpl = Vec::new();
                error_getMsg(ERR_LID_PARAMS, &mut tpl);
                report_writeErrorMsg(prj, ERR_LID_PARAMS, &tpl);
            }
        }

        //... LID unit cannot send outflow back to subcatchment's
        //    pervious area if none exists
        if prj.Subcatch[ju].fracImperv >= 0.999 {
            lidUnit.toPerv = 0;
        }

        //... assign drain outlet if not set by user
        if lidUnit.drainNode == -1 && lidUnit.drainSubcatch == -1 {
            lidUnit.drainNode = prj.Subcatch[ju].outNode;
            lidUnit.drainSubcatch = prj.Subcatch[ju].outSubcatch;
        }
    }
    prj.lid.LidGroups[ju] = Some(lidGroup);

    //... check contributing area fractions
    if totalLidArea > 1.001 * totalArea {
        let id = prj.Subcatch[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_LID_AREAS, &id);
    }
    if fromImperv > 1.001 || fromPerv > 1.001 {
        let id = prj.Subcatch[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_LID_CAPTURE_AREA, &id);
    }

    //... Make subcatchment LID area equal total area if the two are close
    if totalLidArea > 0.999 * totalArea {
        totalLidArea = totalArea;
    }
    prj.Subcatch[ju].lidArea = totalLidArea;
}

//=============================================================================

/// Called by: project.
pub fn lid_initState(prj: &mut Project) {
    //
    //  Purpose: initializes the internal state of each LID in a subcatchment.
    //  Input:   none
    //  Output:  none
    //
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut initVol: f64;
    let initDryTime = prj.StartDryDays * SECperDAY as f64;

    prj.runoff.HasWetLids = FALSE as i8;
    j = 0;
    while j < prj.lid.GroupCount {
        //... check if group exists
        let mut lidGroup = match prj.lid.LidGroups[j as usize].take() {
            None => {
                j += 1;
                continue;
            }
            Some(g) => g,
        };

        //... initialize group variables
        lidGroup.pervArea = 0.0;
        lidGroup.flowToPerv = 0.0;
        lidGroup.oldDrainFlow = 0.0;
        lidGroup.newDrainFlow = 0.0;

        //... examine each LID in the group
        for lidUnit in lidGroup.lidList.iter_mut() {
            //... initialize depth & moisture content
            k = lidUnit.lidIndex;
            let ku = k as usize;
            lidUnit.surfaceDepth = 0.0;
            lidUnit.storageDepth = 0.0;
            lidUnit.soilMoisture = 0.0;
            lidUnit.paveDepth = 0.0;
            lidUnit.dryTime = initDryTime;
            lidUnit.volTreated = 0.0;
            lidUnit.nextRegenDay = prj.lid.LidProcs[ku].pavement.regenDays;
            initVol = 0.0;
            if prj.lid.LidProcs[ku].soil.thickness > 0.0 {
                lidUnit.soilMoisture = prj.lid.LidProcs[ku].soil.wiltPoint
                    + lidUnit.initSat * (prj.lid.LidProcs[ku].soil.porosity - prj.lid.LidProcs[ku].soil.wiltPoint);
                initVol += lidUnit.soilMoisture * prj.lid.LidProcs[ku].soil.thickness;
            }
            if prj.lid.LidProcs[ku].storage.thickness > 0.0 {
                lidUnit.storageDepth = lidUnit.initSat * prj.lid.LidProcs[ku].storage.thickness;
                initVol += lidUnit.storageDepth * prj.lid.LidProcs[ku].storage.voidFrac;
            }
            if lidUnit.initSat > 0.0 {
                prj.runoff.HasWetLids = TRUE as i8;
            }

            //... initialize water balance totals
            lidproc_initWaterBalance(lidUnit, initVol);
            lidUnit.volTreated = 0.0;

            //... initialize report file for the LID
            if lidUnit.rptFile.is_some() {
                initLidRptFile(&*prj, &prj.Title[0], &prj.lid.LidProcs[ku].ID, &prj.Subcatch[j as usize].ID, lidUnit);
            }

            //... initialize drain flows
            lidUnit.oldDrainFlow = 0.0;
            lidUnit.newDrainFlow = 0.0;

            //... set previous flux rates to 0
            i = 0;
            while i < MAX_LAYERS {
                lidUnit.oldFluxRates[i as usize] = 0.0;
                i += 1;
            }

            //... initialize infiltration state variables
            if lidUnit.soilInfil.Ks > 0.0 {
                grnampt_initState(&mut lidUnit.soilInfil);
            }

            //... add contribution to pervious LID area
            if isLidPervious(prj, lidUnit.lidIndex) != 0 {
                lidGroup.pervArea += lidUnit.area * lidUnit.number as f64;
            }
        }
        prj.lid.LidGroups[j as usize] = Some(lidGroup);
        j += 1;
    }
}

//=============================================================================

/// Called by: subcatch.
pub fn lid_setOldGroupState(prj: &mut Project, j: i32) {
    //
    //  Purpose: saves the current drain flow rate for the LIDs in a subcatchment.
    //  Input:   j = subcatchment index
    //  Output:  none
    //
    if let Some(g) = prj.lid.LidGroups[j as usize].as_mut() {
        g.oldDrainFlow = g.newDrainFlow;
        g.newDrainFlow = 0.0;
        for lidUnit in g.lidList.iter_mut() {
            lidUnit.oldDrainFlow = lidUnit.newDrainFlow;
            lidUnit.newDrainFlow = 0.0;
        }
    }
}

//=============================================================================

fn isLidPervious(prj: &Project, k: i32) -> i32 {
    //
    //  Purpose: determines if a LID process allows infiltration or not.
    //  Input:   k = LID process index
    //  Output:  returns 1 if process is pervious or 0 if not
    //
    let lp = &prj.lid.LidProcs[k as usize];
    (lp.storage.thickness == 0.0 || lp.storage.kSat > 0.0) as i32
}

//=============================================================================

fn getSurfaceDepth(prj: &Project, j: i32) -> f64 {
    //
    //  Purpose: computes the depth (volume per unit area) of ponded water on the
    //           surface of all LIDs within a subcatchment.
    //  Input:   j = subcatchment index
    //  Output:  returns volumetric depth of ponded water (ft)
    //
    let mut k: i32;
    let mut depth = 0.0f64;

    let lidGroup = match &prj.lid.LidGroups[j as usize] {
        None => return 0.0,
        Some(g) => g,
    };
    if prj.Subcatch[j as usize].lidArea == 0.0 {
        return 0.0;
    }
    for lidUnit in lidGroup.lidList.iter() {
        k = lidUnit.lidIndex;
        depth += lidUnit.surfaceDepth * prj.lid.LidProcs[k as usize].surface.voidFrac * lidUnit.area * lidUnit.number as f64;
    }
    depth / prj.Subcatch[j as usize].lidArea
}

//=============================================================================

/// Called by: subcatch.
pub fn lid_getPervArea(prj: &Project, j: i32) -> f64 {
    //
    //  Purpose: retrieves amount of pervious LID area in a subcatchment.
    //  Input:   j = subcatchment index
    //  Output:  returns amount of pervious LID area (ft2)
    //
    if let Some(g) = &prj.lid.LidGroups[j as usize] {
        g.pervArea
    } else {
        0.0
    }
}

//=============================================================================

/// Called by: subcatch.
pub fn lid_getFlowToPerv(prj: &Project, j: i32) -> f64 {
    //
    //  Purpose: retrieves flow returned from LID treatment to pervious area of
    //           a subcatchment.
    //  Input:   j = subcatchment index
    //  Output:  returns flow returned to pervious area (cfs)
    //
    if let Some(g) = &prj.lid.LidGroups[j as usize] {
        return g.flowToPerv;
    }
    0.0
}

//=============================================================================

/// Called by: subcatch.
pub fn lid_getStoredVolume(prj: &Project, j: i32) -> f64 {
    //
    //  Purpose: computes stored volume of water for all LIDs
    //           grouped within a subcatchment.
    //  Input:   j = subcatchment index
    //  Output:  returns stored volume of water (ft3)
    //
    let mut total = 0.0f64;

    let lidGroup = &prj.lid.LidGroups[j as usize];
    if lidGroup.is_none() || prj.Subcatch[j as usize].lidArea == 0.0 {
        return 0.0;
    }
    for lidUnit in lidGroup.as_ref().unwrap().lidList.iter() {
        total += lidUnit.waterBalance.finalVol * lidUnit.area * lidUnit.number as f64;
    }
    total
}

//=============================================================================

/// Called by: subcatch.
pub fn lid_getDrainFlow(prj: &Project, j: i32, timePeriod: i32) -> f64 {
    //
    //  Purpose: returns flow from all of a subcatchment's LID drains for
    //           a designated time period
    //  Input:   j = subcatchment index
    //           timePeriod = either PREVIOUS or CURRENT
    //  Output:  total drain flow (cfs) from the subcatchment.
    if let Some(g) = &prj.lid.LidGroups[j as usize] {
        if timePeriod == PREVIOUS {
            return g.oldDrainFlow;
        } else {
            return g.newDrainFlow;
        }
    }
    0.0
}

//=============================================================================

/// Called by: surfqual.
/// (Signature change from the stub: prj is `&mut` because
/// massbal_updateLoadingTotals needs it; `c` is only read, so `&[f64]`.)
pub fn lid_addDrainLoads(prj: &mut Project, j: i32, c: &[f64], tStep: f64) {
    //
    //  Purpose: adds pollutant loads routed from drains to system
    //           mass balance totals.
    //  Input:   j = subcatchment index
    //           c = array of pollutant washoff concentrations (mass/L)
    //           tStep =  time step (sec)
    //  Output:  none.
    //
    let mut isRunoffLoad: i32; // true if drain becomes external runoff load
    let mut p: i32; // pollutant index
    let mut r: f64; // pollutant fractional removal
    let mut w: f64; // pollutant mass load (lb or kg)

    //... check if LID group exists
    let nUnits = match &prj.lid.LidGroups[j as usize] {
        None => return,
        Some(g) => g.lidList.len(),
    };

    //... examine each LID unit in the group
    for idx in 0..nUnits {
        let (drainNode, drainSubcatch, toPerv, newDrainFlow, lidIndex) = {
            let lidUnit = &prj.lid.LidGroups[j as usize].as_ref().unwrap().lidList[idx];
            (lidUnit.drainNode, lidUnit.drainSubcatch, lidUnit.toPerv, lidUnit.newDrainFlow, lidUnit.lidIndex)
        };

        //... see if unit's drain flow becomes external runoff
        isRunoffLoad = (drainNode >= 0 || drainSubcatch == j) as i32;

        //... for each pollutant not routed back on to subcatchment surface
        if toPerv == 0 {
            p = 0;
            while p < prj.Nobjects[POLLUT as usize] {
                //... get mass load flowing through the drain
                w = newDrainFlow * c[p as usize] * tStep * LperFT3 * prj.Pollut[p as usize].mcf;

                //... get fractional removal for this load
                r = prj.lid.LidProcs[lidIndex as usize].drainRmvl[p as usize];

                //... update system mass balance totals
                massbal_updateLoadingTotals(prj, BMP_REMOVAL_LOAD, p, r * w);
                if isRunoffLoad != 0 {
                    massbal_updateLoadingTotals(prj, RUNOFF_LOAD, p, w * (1.0 - r));
                }
                p += 1;
            }
        }

        // process next LID unit in the group
    }
}

//=============================================================================

/// Called by: subcatch.
pub fn lid_addDrainRunon(prj: &mut Project, j: i32) {
    //
    //  Purpose: adds drain flows from LIDs in a given subcatchment to the
    //           subcatchments that were designated to receive them
    //  Input:   j = index of subcatchment contributing underdrain flows
    //  Output:  none.
    //
    let mut i: i32; // index of an LID unit's LID process
    let mut k: i32; // index of subcatchment receiving LID drain flow
    let mut p: i32; // pollutant index
    let mut q: f64; // drain flow rate (cfs)
    let mut w: f64; // mass of polllutant from drain flow

    //... check if LID group exists
    let nUnits = match &prj.lid.LidGroups[j as usize] {
        None => return,
        Some(g) => g.lidList.len(),
    };

    //... examine each LID in the group
    for idx in 0..nUnits {
        //... see if LID's drain discharges to another subcatchment
        let (lidIndex, drainSubcatch, oldDrainFlow) = {
            let lidUnit = &prj.lid.LidGroups[j as usize].as_ref().unwrap().lidList[idx];
            (lidUnit.lidIndex, lidUnit.drainSubcatch, lidUnit.oldDrainFlow)
        };
        i = lidIndex;
        k = drainSubcatch;
        if k >= 0 && k != j {
            //... distribute drain flow across subcatchment's areas
            q = oldDrainFlow;
            subcatch_addRunonFlow(prj, k, q);

            //... add pollutant loads from drain to subcatchment
            //    (newQual[] contains loading rate (mass/sec) at this
            //    point which is converted later on to a concentration)
            p = 0;
            while p < prj.Nobjects[POLLUT as usize] {
                w = q * prj.Subcatch[j as usize].oldQual[p as usize] * LperFT3;
                w = w * (1.0 - prj.lid.LidProcs[i as usize].drainRmvl[p as usize]);
                prj.Subcatch[k as usize].newQual[p as usize] += w;
                p += 1;
            }
        }
    }
}

//=============================================================================

/// Called by: routing.
pub fn lid_addDrainInflow(prj: &mut Project, j: i32, f: f64) {
    //
    //  Purpose: adds LID drain flow to conveyance system nodes
    //  Input:   j = subcatchment index
    //           f = time interval weighting factor
    //  Output:  none.
    //
    //  Note:    this function updates the total lateral flow (Node[].newLatFlow)
    //           and pollutant mass (Node[].newQual[]) inflow seen by nodes that
    //           receive drain flow from the LID units in subcatchment j.
    let mut i: i32; // LID process index
    let mut k: i32; // node index
    let mut p: i32; // pollutant index
    let mut q: f64; // drain flow (cfs)
    let mut w: f64; // pollutant mass loads (mass/sec)
    let mut w1: f64;
    let mut w2: f64;

    //... check if LID group exists
    let nUnits = match &prj.lid.LidGroups[j as usize] {
        None => return,
        Some(g) => g.lidList.len(),
    };

    //... examine each LID in the group
    for idx in 0..nUnits {
        //... see if LID's drain discharges to conveyance system node
        let (lidIndex, drainNode, oldDrainFlow, newDrainFlow) = {
            let lidUnit = &prj.lid.LidGroups[j as usize].as_ref().unwrap().lidList[idx];
            (lidUnit.lidIndex, lidUnit.drainNode, lidUnit.oldDrainFlow, lidUnit.newDrainFlow)
        };
        i = lidIndex;
        k = drainNode;
        if k >= 0 {
            //... add drain flow to node's wet weather inflow
            q = (1.0 - f) * oldDrainFlow + f * newDrainFlow;
            prj.Node[k as usize].newLatFlow += q;
            massbal_addInflowFlow(prj, WET_WEATHER_INFLOW, q);

            //... add pollutant load, based on parent subcatchment quality
            p = 0;
            while p < prj.Nobjects[POLLUT as usize] {
                //... get previous & current drain loads
                w1 = oldDrainFlow * prj.Subcatch[j as usize].oldQual[p as usize];
                w2 = newDrainFlow * prj.Subcatch[j as usize].newQual[p as usize];

                //... add interpolated load to node's wet weather loading
                w = (1.0 - f) * w1 + f * w2;
                w = w * (1.0 - prj.lid.LidProcs[i as usize].drainRmvl[p as usize]);
                prj.Node[k as usize].newQual[p as usize] += w;
                massbal_addInflowQual(prj, WET_WEATHER_INFLOW, p, w);
                p += 1;
            }
        }
    }
}

//=============================================================================

/// Called by: subcatch.
pub fn lid_getRunoff(prj: &mut Project, j: i32, tStep: f64) {
    //
    //  Purpose: computes runoff and drain flows from the LIDs in a subcatchment.
    //  Input:   j     = subcatchment index
    //           tStep = time step (sec)
    //  Output:  updates following global quantities after LID treatment applied:
    //           Vevap, Vpevap, VlidInfil, VlidIn, VlidOut, VlidDrain.
    //
    let ju = j as usize;
    let mut lidArea: f64; // area of an LID unit
    let mut qImperv = 0.0f64; // runoff from impervious areas (cfs)
    let mut qPerv = 0.0f64; // runoff from pervious areas (cfs)
    let mut lidInflow: f64; // inflow to an LID unit (ft/s)
    let mut qRunoff = 0.0f64; // surface runoff from all LID units (cfs)
    let mut qDrain = 0.0f64; // drain flow from all LID units (cfs)
    let mut qReturn = 0.0f64; // LID outflow returned to pervious area (cfs)

    //... return if there are no LID's
    match &prj.lid.LidGroups[ju] {
        None => return,
        Some(g) => {
            if g.lidList.is_empty() {
                return;
            }
        }
    }

    //... determine if evaporation can occur
    prj.lid.EvapRate = prj.Evap.rate;
    if prj.Evap.dryOnly != 0 && prj.Subcatch[ju].rainfall > 0.0 {
        prj.lid.EvapRate = 0.0;
    }

    //... find subcatchment's infiltration rate into native soil
    findNativeInfil(prj, j, tStep);

    //... get impervious and pervious area runoff from non-LID
    //    portion of subcatchment (cfs)
    if prj.Subcatch[ju].area > prj.Subcatch[ju].lidArea {
        qImperv = getImpervAreaRunoff(prj, j);
        qPerv = getPervAreaRunoff(prj, j);
    }

    //... evaluate performance of each LID unit placed in the subcatchment
    //    (the group is taken out of the model while its units are evaluated)
    let mut theLidGroup = prj.lid.LidGroups[ju].take().unwrap();
    for lidUnit in theLidGroup.lidList.iter_mut() {
        //... find area of the LID unit
        lidArea = lidUnit.area * lidUnit.number as f64;

        //... if LID unit has area, evaluate its performance
        if lidArea > 0.0 {
            //... find runoff from non-LID area treated by LID area (ft/sec)
            lidInflow = (qImperv * lidUnit.fromImperv + qPerv * lidUnit.fromPerv) / lidArea;

            //... update total runoff volume treated
            prj.subcatch.VlidIn += lidInflow * lidArea * tStep;

            //... add rainfall onto LID inflow (ft/s)
            lidInflow = lidInflow + getRainInflow(prj, j, lidUnit);

            // ... add upstream runon only if LID occupies full subcatchment
            if prj.Subcatch[ju].area == prj.Subcatch[ju].lidArea {
                lidInflow += prj.Subcatch[ju].runon;
            }

            //... evaluate the LID unit's performance, updating the LID group's
            //    total surface runoff, drain flow, and flow returned to
            //    pervious area
            evalLidUnit(prj, j, lidUnit, lidArea, lidInflow, tStep, &mut qRunoff, &mut qDrain, &mut qReturn);
        }
    }

    //... save the LID group's total drain & return flows
    theLidGroup.newDrainFlow = qDrain;
    theLidGroup.flowToPerv = qReturn;
    prj.lid.LidGroups[ju] = Some(theLidGroup);

    //... save the LID group's total surface, drain and return flow volumes
    prj.subcatch.VlidOut = qRunoff * tStep;
    prj.subcatch.VlidDrain = qDrain * tStep;
    prj.subcatch.VlidReturn = qReturn * tStep;
}

//=============================================================================

fn findNativeInfil(prj: &mut Project, j: i32, tStep: f64) {
    //
    //  Purpose: determines a subcatchment's current infiltration rate into
    //           its native soil.
    //  Input:   j = subcatchment index
    //           tStep    = time step (sec)
    //  Output:  sets values for module-level variables NativeInfil
    //
    let ju = j as usize;
    let nonLidArea: f64;

    //... subcatchment has non-LID pervious area
    nonLidArea = prj.Subcatch[ju].area - prj.Subcatch[ju].lidArea;
    if nonLidArea > 0.0 && prj.Subcatch[ju].fracImperv < 1.0 {
        prj.lid.NativeInfil = prj.subcatch.Vinfil / nonLidArea / tStep;
    }
    //... otherwise find infil. rate for the subcatchment's rainfall + runon
    else {
        let rainfall = prj.Subcatch[ju].rainfall;
        let runon = prj.Subcatch[ju].runon;
        let depth = getSurfaceDepth(prj, j);
        prj.lid.NativeInfil = infil_getInfil(prj, j, tStep, rainfall, runon, depth);
    }

    //... see if there is any groundwater-imposed limit on infil.
    if prj.IgnoreGwater == 0 && prj.Subcatch[ju].groundwater.is_some() {
        prj.lid.MaxNativeInfil = prj.Subcatch[ju].groundwater.as_ref().unwrap().maxInfilVol / tStep;
    } else {
        prj.lid.MaxNativeInfil = BIG;
    }
}

//=============================================================================

fn getRainInflow(prj: &Project, j: i32, lidUnit: &TLidUnit) -> f64 {
    //
    //  Purpose: gets rainfall inflow to an LID unit.
    //  Input:   j = subcatchment index
    //           lidUnit = ptr. to an LID unit
    //  Output:  returns rainfall rate over the LID unit (ft/sec)
    //
    let lidProc = &prj.lid.LidProcs[lidUnit.lidIndex as usize];

    if lidProc.lidType == RAIN_BARREL && lidProc.storage.covered == TRUE {
        return 0.0;
    }
    prj.Subcatch[j as usize].rainfall
}

//=============================================================================

fn getImpervAreaRunoff(prj: &Project, j: i32) -> f64 {
    //
    //  Purpose: computes runoff from impervious area of a subcatchment that
    //           is available for LID treatment.
    //  Input:   j = subcatchment index
    //  Output:  returns runoff flow rate (cfs)
    //
    let sc = &prj.Subcatch[j as usize];
    let mut i: i32;
    let mut q = 0.0f64; // runoff rate (ft/sec)
    let nonLidArea: f64; // non-LID area (ft2)

    // --- runoff from impervious area w/ & w/o depression storage
    i = IMPERV0;
    while i <= IMPERV1 {
        q += sc.subArea[i as usize].runoff * sc.subArea[i as usize].fArea;
        i += 1;
    }

    // --- adjust for any fraction of runoff sent to pervious area
    if sc.subArea[IMPERV0 as usize].routeTo == TO_PERV && sc.fracImperv < 1.0 {
        q *= sc.subArea[IMPERV0 as usize].fOutlet;
    }
    nonLidArea = sc.area - sc.lidArea;
    q * nonLidArea
}

//=============================================================================

fn getPervAreaRunoff(prj: &Project, j: i32) -> f64 {
    //
    //  Purpose: computes runoff from pervious area of a subcatchment that
    //           is available for LID treatment.
    //  Input:   j = subcatchment index
    //  Output:  returns runoff flow rate (cfs)
    //
    let sc = &prj.Subcatch[j as usize];
    let mut q: f64; // runoff rate (ft/sec)
    let nonLidArea: f64; // non-LID area (ft2)

    // --- runoff from pervious area
    q = sc.subArea[PERV as usize].runoff * sc.subArea[PERV as usize].fArea;

    // --- adjust for any fraction of runoff sent to impervious area
    if sc.subArea[PERV as usize].routeTo == TO_IMPERV && sc.fracImperv > 0.0 {
        q *= sc.subArea[PERV as usize].fOutlet;
    }
    nonLidArea = sc.area - sc.lidArea;
    q * nonLidArea
}

//=============================================================================

fn evalLidUnit(
    prj: &mut Project,
    j: i32,
    lidUnit: &mut TLidUnit,
    lidArea: f64,
    lidInflow: f64,
    tStep: f64,
    qRunoff: &mut f64,
    qDrain: &mut f64,
    qReturn: &mut f64,
) {
    //
    //  Purpose: evaluates performance of a specific LID unit over current time step.
    //  Input:   j         = subcatchment index
    //           lidUnit   = ptr. to LID unit being evaluated
    //           lidArea   = area of LID unit
    //           lidInflow = inflow to LID unit (ft/s)
    //           tStep     = time step (sec)
    //  Output:  qRunoff   = sum of surface runoff from all LIDs (cfs)
    //           qDrain    = sum of drain flows from all LIDs (cfs)
    //           qReturn   = sum of LID flows returned to pervious area (cfs)
    //
    let ju = j as usize;
    let mut lidRunoff: f64; // surface runoff from LID unit (cfs)
    let mut lidEvap: f64; // evaporation rate from LID unit (ft/s)
    let mut lidInfil: f64; // infiltration rate from LID unit (ft/s)
    // drain flow rate from LID unit (ft/s & cfs); C leaves it unset here and
    // lidproc_getOutflow sets it on every path a validated LID type takes
    let mut lidDrain: f64 = 0.0;

    //... identify the LID process of the LID unit being analyzed
    //    (taken out of LidProcs for the call; lidproc does not read it there)
    let k = lidUnit.lidIndex as usize;
    let lidProc = std::mem::take(&mut prj.lid.LidProcs[k]);

    //... initialize evap and infil losses
    lidEvap = 0.0;
    lidInfil = 0.0;

    //... find surface runoff from the LID unit (in cfs)
    let EvapRate = prj.lid.EvapRate;
    let NativeInfil = prj.lid.NativeInfil;
    let MaxNativeInfil = prj.lid.MaxNativeInfil;
    lidRunoff = lidproc_getOutflow(
        prj,
        lidUnit,
        &lidProc,
        lidInflow,
        EvapRate,
        NativeInfil,
        MaxNativeInfil,
        tStep,
        &mut lidEvap,
        &mut lidInfil,
        &mut lidDrain,
    ) * lidArea;
    prj.lid.LidProcs[k] = lidProc;

    //... convert drain flow to CFS
    lidDrain *= lidArea;

    //... revise flows if LID outflow returned to pervious area
    if lidUnit.toPerv != 0 && prj.Subcatch[ju].area > prj.Subcatch[ju].lidArea {
        //... surface runoff is always returned
        *qReturn += lidRunoff;
        lidRunoff = 0.0;

        //... drain flow returned if it has same outlet as subcatchment
        if lidUnit.drainNode == prj.Subcatch[ju].outNode && lidUnit.drainSubcatch == prj.Subcatch[ju].outSubcatch {
            *qReturn += lidDrain;
            lidDrain = 0.0;
        }
    }

    //... update system flow balance if drain flow goes to a
    //    conveyance system node
    if lidUnit.drainNode >= 0 {
        massbal_updateRunoffTotals(prj, RUNOFF_DRAINS, lidDrain * tStep);
    }

    //... save new drain outflow
    lidUnit.newDrainFlow = lidDrain;

    //... update moisture losses (ft3)
    prj.subcatch.Vevap += lidEvap * tStep * lidArea;
    prj.subcatch.VlidInfil += lidInfil * tStep * lidArea;
    if isLidPervious(prj, lidUnit.lidIndex) != 0 {
        prj.subcatch.Vpevap += lidEvap * tStep * lidArea;
    }

    //... update time since last rainfall (for Rain Barrel emptying)
    if prj.Subcatch[ju].rainfall > MIN_RUNOFF {
        lidUnit.dryTime = 0.0;
    } else {
        lidUnit.dryTime += tStep;
    }

    //... update LID water balance and save results
    let ucfRainfall = UCF(prj, RAINFALL);
    let ucfRainDepth = UCF(prj, RAINDEPTH);
    lidproc_saveResults(prj, lidUnit, ucfRainfall, ucfRainDepth);

    //... update LID group totals
    *qRunoff += lidRunoff;
    *qDrain += lidDrain;
}

//=============================================================================

/// Called by: statsrpt.
pub fn lid_writeWaterBalance(prj: &mut Project) {
    //
    //  Purpose: writes a LID performance summary table to the project's report file.
    //  Input:   none
    //  Output:  none
    //
    let mut j: i32;
    let mut k: i32 = 0;
    let ucf = UCF(prj, RAINDEPTH);
    let mut inflow: f64;
    let mut outflow: f64;
    let mut err: f64;

    //... check that project has LIDs
    j = 0;
    while j < prj.lid.GroupCount {
        if prj.lid.LidGroups[j as usize].is_some() {
            k += 1;
        }
        j += 1;
    }
    if k == 0 {
        return;
    }

    //... write table header
    cprintf!(
        prj.Frpt,
        concat!(
            "\n",
            "\n  ***********************",
            "\n  LID Performance Summary",
            "\n  ***********************\n"
        )
        .as_bytes()
    );

    cprintf!(
        prj.Frpt,
        concat!(
            "\n  --------------------------------------------------------------------------------------------------------------------",
            "\n                                         Total      Evap     Infil   Surface    Drain    Initial     Final  Continuity",
            "\n                                        Inflow      Loss      Loss   Outflow   Outflow   Storage   Storage       Error"
        )
        .as_bytes()
    );
    if prj.UnitSystem == US {
        cprintf!(
            prj.Frpt,
            b"\n  Subcatchment      LID Control             in        in        in        in        in        in        in           %%"
        );
    } else {
        cprintf!(
            prj.Frpt,
            b"\n  Subcatchment      LID Control             mm        mm        mm        mm        mm        mm        mm           %%"
        );
    }
    cprintf!(
        prj.Frpt,
        b"\n  --------------------------------------------------------------------------------------------------------------------"
    );

    //... examine each LID unit in each subcatchment
    j = 0;
    while j < prj.lid.GroupCount {
        let ju = j as usize;
        let nUnits = match &prj.lid.LidGroups[ju] {
            Some(g) if prj.Subcatch[ju].lidArea != 0.0 => g.lidList.len(),
            _ => {
                j += 1;
                continue;
            }
        };
        for idx in 0..nUnits {
            //... write water balance components to report file
            let wb = prj.lid.LidGroups[ju].as_ref().unwrap().lidList[idx].waterBalance;
            k = prj.lid.LidGroups[ju].as_ref().unwrap().lidList[idx].lidIndex;
            let s = cformat!(b"\n  %-16s  %-16s", prj.Subcatch[ju].ID, prj.lid.LidProcs[k as usize].ID);
            prj.Frpt.write_bytes(&s);
            cprintf!(
                prj.Frpt,
                b"%10.2f%10.2f%10.2f%10.2f%10.2f%10.2f%10.2f",
                wb.inflow * ucf,
                wb.evap * ucf,
                wb.infil * ucf,
                wb.surfFlow * ucf,
                wb.drainFlow * ucf,
                wb.initVol * ucf,
                wb.finalVol * ucf
            );

            //... compute flow balance error
            inflow = wb.initVol + wb.inflow;
            outflow = wb.finalVol + wb.evap + wb.infil + wb.surfFlow + wb.drainFlow;
            if inflow > 0.0 {
                err = (inflow - outflow) / inflow;
            } else {
                err = 1.0;
            }
            cprintf!(prj.Frpt, b"  %10.2f", err * 100.0);
        }
        j += 1;
    }
}

//=============================================================================

fn initLidRptFile(prj: &Project, title: &[u8], lidID: &[u8], subcatchID: &[u8], lidUnit: &mut TLidUnit) {
    //
    //  Purpose: initializes the report file used for a specific LID unit
    //  Input:   title = project's title
    //           lidID = LID process name
    //           subcatchID = subcatchment ID name
    //           lidUnit = ptr. to LID unit
    //  Output:  none
    //
    const colCount: i32 = 14;
    const head1: [&[u8]; 14] = [
        b"\n                    \t",
        b"  Elapsed\t",
        b"    Total\t",
        b"    Total\t",
        b"  Surface\t",
        b" Pavement\t",
        b"     Soil\t",
        b"  Storage\t",
        b"  Surface\t",
        b"    Drain\t",
        b"  Surface\t",
        b" Pavement\t",
        b"     Soil\t",
        b"  Storage",
    ];
    const head2: [&[u8]; 14] = [
        b"\n                    \t",
        b"     Time\t",
        b"   Inflow\t",
        b"     Evap\t",
        b"    Infil\t",
        b"     Perc\t",
        b"     Perc\t",
        b"    Exfil\t",
        b"   Runoff\t",
        b"  OutFlow\t",
        b"    Level\t",
        b"    Level\t",
        b" Moisture\t",
        b"    Level",
    ];
    const units1: [&[u8]; 14] = [
        b"\nDate        Time    \t",
        b"    Hours\t",
        b"    in/hr\t",
        b"    in/hr\t",
        b"    in/hr\t",
        b"    in/hr\t",
        b"    in/hr\t",
        b"    in/hr\t",
        b"    in/hr\t",
        b"    in/hr\t",
        b"   inches\t",
        b"   inches\t",
        b"  Content\t",
        b"   inches",
    ];
    const units2: [&[u8]; 14] = [
        b"\nDate        Time    \t",
        b"    Hours\t",
        b"    mm/hr\t",
        b"    mm/hr\t",
        b"    mm/hr\t",
        b"    mm/hr\t",
        b"    mm/hr\t",
        b"    mm/hr\t",
        b"    mm/hr\t",
        b"    mm/hr\t",
        b"       mm\t",
        b"       mm\t",
        b"  Content\t",
        b"       mm",
    ];
    const line9: &[u8] = b" ---------";
    let mut i: i32;
    let rptFile = lidUnit.rptFile.as_mut().unwrap();
    let f = match rptFile.file.as_mut() {
        //... check that file was opened
        None => return,
        Some(f) => f,
    };

    //... write title lines
    cprintf!(f, b"SWMM5 LID Report File\n");
    cprintf!(f, b"\nProject:  %s", title);
    cprintf!(f, b"\nLID Unit: %s in Subcatchment %s\n", lidID, subcatchID);

    //... write column headings
    i = 0;
    while i < colCount {
        cprintf!(f, b"%s", head1[i as usize]);
        i += 1;
    }
    i = 0;
    while i < colCount {
        cprintf!(f, b"%s", head2[i as usize]);
        i += 1;
    }
    if prj.UnitSystem == US {
        i = 0;
        while i < colCount {
            cprintf!(f, b"%s", units1[i as usize]);
            i += 1;
        }
    } else {
        i = 0;
        while i < colCount {
            cprintf!(f, b"%s", units2[i as usize]);
            i += 1;
        }
    }
    cprintf!(f, b"\n----------- --------");
    i = 1;
    while i < colCount {
        cprintf!(f, b"\t%s", line9);
        i += 1;
    }

    //... initialize LID dryness state
    rptFile.wasDry = 1;
    sstrncpy(&mut rptFile.results, b"", 0);
}
