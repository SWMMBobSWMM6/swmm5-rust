//! hotstart.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   hotstart.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21 (Build 5.2.0)
//   Author:   L. Rossman
//
//   Hot Start file functions.
//
//   A SWMM hot start file contains the state of a SWMM project after
//   a simulation has been run, allowing it to be used to initialize
//   a subsequent simulation that picks up where the previous run ended.
//
//   An abridged version of the hot start file (version 2) is available
//   that contains only variables that appear in the binary output file
//   (groundwater upper moisture and water table elevation, node depth,
//   lateral inflow, and quality, and link flow, depth, setting and quality).
//
//   When reading a previously saved hot start file checks are made to
//   insure the the current SWMM project has the same number of major
//   components (subcatchments, land uses, nodes, links, and pollutants)
//   and unit system as does the hot start file. No test is made to
//   insure that these components are of the same sub-type and maintain
//   the same order as when the hot start file was created.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Storage node hydraulic residence time (HRT) was added to the file.
//   - Link control settings are now applied when reading a hot start file.
//   - Runoff read from file assigned to newRunoff property instead of oldRunoff.
//   - Array indexing bug when reading snowpack state from file fixed.
//   Build 5.1.011:
//   - Link control setting bug when reading a hot start file fixed.
//   Build 5.1.015:
//   - Support added for multiple infiltration methods within a project.
//-----------------------------------------------------------------------------
//
// Values are read and written little-endian, as fread/fwrite do on x86-64.
// A short fread leaves the bytes it did not reach as they were, as glibc's.

use crate::prelude::*;

/// The file-scope variables of hotstart.c.
#[derive(Default)]
pub struct HotstartState {
    fileVersion: i32,
}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
// hotstart_open                          (called by swmm_start in swmm5.c)
// hotstart_close                         (called by swmm_end in swmm5.c)

/// `fread(&k, sizeof(int), 1, Fhotstart1.file)`
fn fread_int(f: &mut TFile, k: &mut i32) {
    let mut b = k.to_le_bytes();
    f.f().read_bytes(&mut b);
    *k = i32::from_le_bytes(b);
}

//=============================================================================

pub fn hotstart_open(prj: &mut Project) -> i32 {
    // --- open hot start files
    if openHotstartFile1(prj) == 0 {
        return FALSE; //input hot start file
    }
    if openHotstartFile2(prj) == 0 {
        return FALSE; //output hot start file
    }
    TRUE
}

//=============================================================================

pub fn hotstart_close(prj: &mut Project) {
    if prj.Fhotstart2.file.is_some() {
        saveRunoff(prj);
        saveRouting(prj);
        prj.Fhotstart2.file = None;
    }
}

//=============================================================================

fn openHotstartFile1(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  none
//  Purpose: opens a previously saved routing hotstart file.
//
{
    let mut nSubcatch: i32;
    let mut nNodes: i32;
    let mut nLinks: i32;
    let mut nPollut: i32;
    let mut nLandUses: i32;
    let mut flowUnits: i32;
    let mut fStamp: [u8; 15] = *b"SWMM5-HOTSTART\0";
    let fileStamp: [u8; 15] = *b"SWMM5-HOTSTART\0";
    let mut fStampx: [u8; 16] = *b"SWMM5-HOTSTARTx\0";
    let fileStamp2: [u8; 16] = *b"SWMM5-HOTSTART2\0";
    let fileStamp3: [u8; 16] = *b"SWMM5-HOTSTART3\0";
    let fileStamp4: [u8; 16] = *b"SWMM5-HOTSTART4\0";

    // --- try to open the file
    if prj.Fhotstart1.mode as i32 != USE_FILE {
        return TRUE;
    }
    prj.Fhotstart1.file = CFile::open(&prj.Fhotstart1.name, "r+b");
    if prj.Fhotstart1.file.is_none() {
        let name = prj.Fhotstart1.name;
        report_writeErrorMsg(prj, ERR_HOTSTART_FILE_OPEN, &name);
        return FALSE;
    }

    // --- check that file contains proper header records
    let n2 = cio::strlen(&fileStamp2);
    prj.Fhotstart1.f().read_bytes(&mut fStampx[..n2]);
    if cio::strcmp(&fStampx, &fileStamp4) == 0 {
        prj.hotstart.fileVersion = 4;
    } else if cio::strcmp(&fStampx, &fileStamp3) == 0 {
        prj.hotstart.fileVersion = 3;
    } else if cio::strcmp(&fStampx, &fileStamp2) == 0 {
        prj.hotstart.fileVersion = 2;
    } else {
        prj.Fhotstart1.f().rewind();
        let n1 = cio::strlen(&fileStamp);
        prj.Fhotstart1.f().read_bytes(&mut fStamp[..n1]);
        if cio::strcmp(&fStamp, &fileStamp) != 0 {
            report_writeErrorMsg(prj, ERR_HOTSTART_FILE_FORMAT, b"");
            return FALSE;
        }
        prj.hotstart.fileVersion = 1;
    }

    nSubcatch = -1;
    nNodes = -1;
    nLinks = -1;
    nPollut = -1;
    nLandUses = -1;
    flowUnits = -1;
    if prj.hotstart.fileVersion >= 2 {
        fread_int(&mut prj.Fhotstart1, &mut nSubcatch);
    } else {
        nSubcatch = prj.Nobjects[SUBCATCH as usize];
    }
    if prj.hotstart.fileVersion >= 3 {
        fread_int(&mut prj.Fhotstart1, &mut nLandUses);
    } else {
        nLandUses = prj.Nobjects[LANDUSE as usize];
    }
    fread_int(&mut prj.Fhotstart1, &mut nNodes);
    fread_int(&mut prj.Fhotstart1, &mut nLinks);
    fread_int(&mut prj.Fhotstart1, &mut nPollut);
    fread_int(&mut prj.Fhotstart1, &mut flowUnits);
    if nSubcatch != prj.Nobjects[SUBCATCH as usize]
        || nLandUses != prj.Nobjects[LANDUSE as usize]
        || nNodes != prj.Nobjects[NODE as usize]
        || nLinks != prj.Nobjects[LINK as usize]
        || nPollut != prj.Nobjects[POLLUT as usize]
        || flowUnits != prj.FlowUnits
    {
        report_writeErrorMsg(prj, ERR_HOTSTART_FILE_FORMAT, b"");
        return FALSE;
    }

    // --- read contents of the file and close it
    if prj.hotstart.fileVersion >= 3 {
        readRunoff(prj);
    }
    readRouting(prj);
    prj.Fhotstart1.file = None;
    if prj.ErrorCode != 0 {
        FALSE
    } else {
        TRUE
    }
}

//=============================================================================

fn openHotstartFile2(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  none
//  Purpose: opens a new routing hotstart file to save results to.
//
{
    let nSubcatch: i32;
    let nLandUses: i32;
    let nNodes: i32;
    let nLinks: i32;
    let nPollut: i32;
    let flowUnits: i32;
    let fileStamp: &[u8] = b"SWMM5-HOTSTART4";

    // --- try to open file
    if prj.Fhotstart2.mode as i32 != SAVE_FILE {
        return TRUE;
    }
    prj.Fhotstart2.file = CFile::open(&prj.Fhotstart2.name, "w+b");
    if prj.Fhotstart2.file.is_none() {
        let name = prj.Fhotstart2.name;
        report_writeErrorMsg(prj, ERR_HOTSTART_FILE_OPEN, &name);
        return FALSE;
    }

    // --- write file stamp & number of objects to file
    nSubcatch = prj.Nobjects[SUBCATCH as usize];
    nLandUses = prj.Nobjects[LANDUSE as usize];
    nNodes = prj.Nobjects[NODE as usize];
    nLinks = prj.Nobjects[LINK as usize];
    nPollut = prj.Nobjects[POLLUT as usize];
    flowUnits = prj.FlowUnits;
    let f = &mut prj.Fhotstart2;
    f.write_bytes(&fileStamp[..cio::strlen(fileStamp)]);
    f.write_bytes(&nSubcatch.to_le_bytes());
    f.write_bytes(&nLandUses.to_le_bytes());
    f.write_bytes(&nNodes.to_le_bytes());
    f.write_bytes(&nLinks.to_le_bytes());
    f.write_bytes(&nPollut.to_le_bytes());
    f.write_bytes(&flowUnits.to_le_bytes());
    TRUE
}

//=============================================================================

fn saveRouting(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: saves current state of all nodes and links to hotstart file.
//
{
    let mut i: usize;
    let mut j: usize;
    let mut x: [f32; 3] = [0.0; 3];

    i = 0;
    while i < prj.Nobjects[NODE as usize] as usize {
        x[0] = prj.Node[i].newDepth as f32;
        x[1] = prj.Node[i].newLatFlow as f32;
        prj.Fhotstart2.write_bytes(&x[0].to_le_bytes());
        prj.Fhotstart2.write_bytes(&x[1].to_le_bytes());

        if prj.Node[i].type_ == STORAGE {
            j = prj.Node[i].subIndex as usize;
            x[0] = prj.Storage[j].hrt as f32;
            prj.Fhotstart2.write_bytes(&x[0].to_le_bytes());
        }

        j = 0;
        while j < prj.Nobjects[POLLUT as usize] as usize {
            x[0] = prj.Node[i].newQual[j] as f32;
            prj.Fhotstart2.write_bytes(&x[0].to_le_bytes());
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < prj.Nobjects[LINK as usize] as usize {
        x[0] = prj.Link[i].newFlow as f32;
        x[1] = prj.Link[i].newDepth as f32;
        x[2] = prj.Link[i].setting as f32;
        prj.Fhotstart2.write_bytes(&x[0].to_le_bytes());
        prj.Fhotstart2.write_bytes(&x[1].to_le_bytes());
        prj.Fhotstart2.write_bytes(&x[2].to_le_bytes());
        j = 0;
        while j < prj.Nobjects[POLLUT as usize] as usize {
            x[0] = prj.Link[i].newQual[j] as f32;
            prj.Fhotstart2.write_bytes(&x[0].to_le_bytes());
            j += 1;
        }
        i += 1;
    }
}

//=============================================================================

fn readRouting(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: reads initial state of all nodes, links and groundwater objects
//           from hotstart file.
//
{
    let mut i: i32;
    let mut j: i32;
    // (uninitialized in C; it is only read after a readFloat stores into
    // it, except if the very first fread gets no bytes at all, where C
    // would test garbage for NaN)
    let mut x: f32 = 0.0;
    let mut xgw: [f64; 4] = [0.0; 4];

    // --- for file format 2, assign GW moisture content and lower depth
    if prj.hotstart.fileVersion == 2 {
        // --- flow and available upper zone volume not used
        xgw[2] = 0.0;
        xgw[3] = MISSING;
        i = 0;
        while i < prj.Nobjects[SUBCATCH as usize] {
            // --- read moisture content and water table elevation as floats
            if readFloat(prj, &mut x) == 0 {
                return;
            }
            xgw[0] = x as f64;
            if readFloat(prj, &mut x) == 0 {
                return;
            }
            xgw[1] = x as f64;

            // --- set GW state
            if prj.Subcatch[i as usize].groundwater.is_some() {
                gwater_setState(prj, i, &xgw);
            }
            i += 1;
        }
    }

    // --- read node states
    i = 0;
    while i < prj.Nobjects[NODE as usize] {
        if readFloat(prj, &mut x) == 0 {
            return;
        }
        prj.Node[i as usize].newDepth = x as f64;
        if readFloat(prj, &mut x) == 0 {
            return;
        }
        prj.Node[i as usize].newLatFlow = x as f64;

        if prj.hotstart.fileVersion >= 4 && prj.Node[i as usize].type_ == STORAGE {
            if readFloat(prj, &mut x) == 0 {
                return;
            }
            j = prj.Node[i as usize].subIndex;
            prj.Storage[j as usize].hrt = x as f64;
        }

        j = 0;
        while j < prj.Nobjects[POLLUT as usize] {
            if readFloat(prj, &mut x) == 0 {
                return;
            }
            prj.Node[i as usize].newQual[j as usize] = x as f64;
            j += 1;
        }

        // --- read in zeros here for backwards compatibility
        if prj.hotstart.fileVersion <= 2 {
            j = 0;
            while j < prj.Nobjects[POLLUT as usize] {
                if readFloat(prj, &mut x) == 0 {
                    return;
                }
                j += 1;
            }
        }
        i += 1;
    }

    // --- read link states
    i = 0;
    while i < prj.Nobjects[LINK as usize] {
        if readFloat(prj, &mut x) == 0 {
            return;
        }
        prj.Link[i as usize].newFlow = x as f64;
        if readFloat(prj, &mut x) == 0 {
            return;
        }
        prj.Link[i as usize].newDepth = x as f64;
        if readFloat(prj, &mut x) == 0 {
            return;
        }
        prj.Link[i as usize].setting = x as f64;

        // --- set link's target setting to saved setting
        prj.Link[i as usize].targetSetting = x as f64;
        link_setTargetSetting(prj, i);
        link_setSetting(prj, i, 0.0);

        j = 0;
        while j < prj.Nobjects[POLLUT as usize] {
            if readFloat(prj, &mut x) == 0 {
                return;
            }
            prj.Link[i as usize].newQual[j as usize] = x as f64;
            j += 1;
        }
        i += 1;
    }
}

//=============================================================================

fn saveRunoff(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: saves current state of all subcatchments to hotstart file.
//
//  (C writes Nobjects[POLLUT] doubles from the 6-element array x for each
//  land use buildup value: x[0] is the buildup, x[1..] whatever x held
//  before. Past x[5] C reads beyond the array (undefined; whatever lies
//  on its stack after x); those doubles are written as 0.0 here.)
{
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;
    // (uninitialized in C; every element is written below before it is
    // read: x[0..3] then all six are set at the top of each subcatchment)
    let mut x: [f64; 6] = [0.0; 6];

    let write_doubles = |f: &mut TFile, x: &[f64; 6], n: usize| {
        let mut b: Vec<u8> = Vec::with_capacity(8 * n);
        for m in 0..n {
            let v = if m < 6 { x[m] } else { 0.0 };
            b.extend_from_slice(&v.to_le_bytes());
        }
        f.write_bytes(&b);
    };

    i = 0;
    while i < prj.Nobjects[SUBCATCH as usize] as usize {
        // Ponded depths for each sub-area & total runoff (4 elements)
        j = 0;
        while j < 3 {
            x[j] = prj.Subcatch[i].subArea[j].depth;
            j += 1;
        }
        x[3] = prj.Subcatch[i].newRunoff;
        write_doubles(&mut prj.Fhotstart2, &x, 4);

        // Infiltration state (max. of 6 elements)
        j = 0;
        while j < 6 {
            x[j] = 0.0;
            j += 1;
        }
        infil_getState(prj, i as i32, &mut x);
        write_doubles(&mut prj.Fhotstart2, &x, 6);

        // Groundwater state (4 elements)
        if prj.Subcatch[i].groundwater.is_some() {
            gwater_getState(prj, i as i32, &mut x);
            write_doubles(&mut prj.Fhotstart2, &x, 4);
        }

        // Snowpack state (5 elements for each of 3 snow surfaces)
        if prj.Subcatch[i].snowpack.is_some() {
            j = 0;
            while j < 3 {
                snow_getState(prj, i as i32, j as i32, &mut x);
                write_doubles(&mut prj.Fhotstart2, &x, 5);
                j += 1;
            }
        }

        // Water quality
        if prj.Nobjects[POLLUT as usize] > 0 {
            let npollut = prj.Nobjects[POLLUT as usize] as usize;

            // Runoff quality
            j = 0;
            while j < npollut {
                x[0] = prj.Subcatch[i].newQual[j];
                write_doubles(&mut prj.Fhotstart2, &x, 1);
                j += 1;
            }

            // Ponded quality
            j = 0;
            while j < npollut {
                x[0] = prj.Subcatch[i].pondedQual[j];
                write_doubles(&mut prj.Fhotstart2, &x, 1);
                j += 1;
            }

            // Buildup and when streets were last swept
            k = 0;
            while k < prj.Nobjects[LANDUSE as usize] as usize {
                j = 0;
                while j < npollut {
                    x[0] = prj.Subcatch[i].landFactor[k].buildup[j];
                    write_doubles(&mut prj.Fhotstart2, &x, npollut);
                    j += 1;
                }
                x[0] = prj.Subcatch[i].landFactor[k].lastSwept;
                write_doubles(&mut prj.Fhotstart2, &x, 1);
                k += 1;
            }
        }
        i += 1;
    }
}

//=============================================================================

fn readRunoff(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: reads saved state of all subcatchments from a hot start file.
//
{
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;
    // (uninitialized in C; each element read is stored by readDouble first)
    let mut x: [f64; 6] = [0.0; 6];

    // readDouble into a field of the model: C passes the field's address
    macro_rules! read_into {
        ($place:expr) => {{
            let mut v: f64 = $place;
            let ok = readDouble(prj, &mut v);
            $place = v;
            ok
        }};
    }

    i = 0;
    while i < prj.Nobjects[SUBCATCH as usize] as usize {
        // Ponded depths & runoff (4 elements)
        j = 0;
        while j < 3 {
            if read_into!(prj.Subcatch[i].subArea[j].depth) == 0 {
                return;
            }
            j += 1;
        }
        if read_into!(prj.Subcatch[i].newRunoff) == 0 {
            return;
        }

        // Infiltration state (max. of 6 elements)
        j = 0;
        while j < 6 {
            if readDouble(prj, &mut x[j]) == 0 {
                return;
            }
            j += 1;
        }
        infil_setState(prj, i as i32, &x);

        // Groundwater state (4 elements)
        if prj.Subcatch[i].groundwater.is_some() {
            j = 0;
            while j < 4 {
                if readDouble(prj, &mut x[j]) == 0 {
                    return;
                }
                j += 1;
            }
            gwater_setState(prj, i as i32, &x);
        }

        // Snowpack state (5 elements for each of 3 snow surfaces)
        if prj.Subcatch[i].snowpack.is_some() {
            j = 0;
            while j < 3 {
                k = 0;
                while k < 5 {
                    if readDouble(prj, &mut x[k]) == 0 {
                        return;
                    }
                    k += 1;
                }
                snow_setState(prj, i as i32, j as i32, &x);
                j += 1;
            }
        }

        // Water quality
        if prj.Nobjects[POLLUT as usize] > 0 {
            // Runoff quality
            j = 0;
            while j < prj.Nobjects[POLLUT as usize] as usize {
                if read_into!(prj.Subcatch[i].newQual[j]) == 0 {
                    return;
                }
                j += 1;
            }

            // Ponded quality
            j = 0;
            while j < prj.Nobjects[POLLUT as usize] as usize {
                if read_into!(prj.Subcatch[i].pondedQual[j]) == 0 {
                    return;
                }
                j += 1;
            }

            // Buildup and when streets were last swept
            k = 0;
            while k < prj.Nobjects[LANDUSE as usize] as usize {
                j = 0;
                while j < prj.Nobjects[POLLUT as usize] as usize {
                    if read_into!(prj.Subcatch[i].landFactor[k].buildup[j]) == 0 {
                        return;
                    }
                    j += 1;
                }
                if read_into!(prj.Subcatch[i].landFactor[k].lastSwept) == 0 {
                    return;
                }
                k += 1;
            }
        }
        i += 1;
    }
}

//=============================================================================

fn readFloat(prj: &mut Project, x: &mut f32) -> i32
//
//  Input:   none
//  Output:  x  = pointer to a float variable
//  Purpose: reads a floating point value from a hot start file
//
{
    // --- read a value from the file
    let mut b = x.to_le_bytes();
    prj.Fhotstart1.f().read_bytes(&mut b);
    *x = f32::from_le_bytes(b);

    // --- test if the value is NaN (not a number)
    if *x != *x {
        report_writeErrorMsg(prj, ERR_HOTSTART_FILE_READ, b"");
        *x = 0.0;
        return FALSE;
    }
    TRUE
}

//=============================================================================

fn readDouble(prj: &mut Project, x: &mut f64) -> i32
//
//  Input:   none
//  Output:  x  = pointer to a double variable
//  Purpose: reads a floating point value from a hot start file
//
{
    // --- read a value from the file
    if prj.Fhotstart1.f().feof() {
        *x = 0.0;
        report_writeErrorMsg(prj, ERR_HOTSTART_FILE_READ, b"");
        return FALSE;
    }
    let mut b = x.to_le_bytes();
    prj.Fhotstart1.f().read_bytes(&mut b);
    *x = f64::from_le_bytes(b);

    // --- test if the value is NaN (not a number)
    if *x != *x {
        *x = 0.0;
        return FALSE;
    }
    TRUE
}
