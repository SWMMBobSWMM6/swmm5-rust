//! rdii.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   rdii.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21   (Build 5.2.0)
//   Author:   L. Rossman
//             R. Dickinson (CDM)
//
//   RDII processing functions.
//
//   Note: RDII means rainfall dependent infiltration/inflow,
//         UH means unit hydrograph.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Ignore RDII option implemented.
//   - Rainfall climate adjustment implemented.
//   Build 5.1.014:
//   - Fixes bug related to isUsed property of a unit hydrograph's rain gage.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
// Definition of 4-byte integer, 4-byte real and 8-byte real types
//-----------------------------------------------------------------------------
// INT4 = i32, REAL4 = f32, REAL8 = f64
const FILE_STAMP: &[u8] = b"SWMM5-RDII";

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------
const ZERO_RDII: f64 = 0.0001; // Minimum non-zero RDII inflow (cfs)
const FileStamp: &[u8] = FILE_STAMP;

// swmm5.c: `const double Qcf[6]` (flow units conversion factors);
// swmm5.rs does not export it, so a copy of its values is kept here.
const Qcf: [f64; 6] = [1.0, 448.831, 0.64632, 0.02832, 28.317, 2.4466];

//-----------------------------------------------------------------------------
// Data Structures
//-----------------------------------------------------------------------------
// enum FileTypes {BINARY, TEXT};         // File mode types
const BINARY: i32 = 0;
const TEXT: i32 = 1;

/// Data for a single unit hydrograph
#[derive(Clone, Default)]
struct TUHData {
    pastRain: Vec<f64>, // array of past rainfall values
    pastMonth: Vec<i8>, // month in which past rainfall occurred
    period: i32,        // current UH time period
    hasPastRain: i32,   // true if > 0 past periods with rain
    maxPeriods: i32,    // max. past rainfall periods
    drySeconds: i64,    // time since last nonzero rainfall
    iaUsed: f64,        // initial abstraction used (in or mm)
}

/// Data for a unit hydrograph group
#[derive(Clone, Default)]
struct TUHGroup {
    isUsed: i32,        // true if UH group used by any nodes
    rainInterval: i32,  // time interval for RDII processing (sec)
    area: f64,          // sewered area covered by UH's gage (ft2)
    rdii: f64,          // rdii flow (in rainfall units)
    gageDate: DateTime, // calendar date of rain gage period
    lastDate: DateTime, // date of last rdii computed
    uh: [TUHData; 3],   // data for each unit hydrograph
}

//-----------------------------------------------------------------------------
// Shared Variables
//-----------------------------------------------------------------------------
/// The file-scope variables of rdii.c.
#[derive(Default)]
pub struct RdiiState {
    UHGroup: Vec<TUHGroup>,   // processing data for each UH group
    RdiiStep: i32,            // RDII time step (sec)
    NumRdiiNodes: i32,        // number of nodes w/ RDII data
    RdiiNodeIndex: Vec<i32>,  // indexes of nodes w/ RDII data
    RdiiNodeFlow: Vec<f32>,   // inflows for nodes with RDII
    RdiiFlowUnits: i32,       // RDII flow units code
    RdiiStartDate: DateTime,  // start date of RDII inflow period
    RdiiEndDate: DateTime,    // end date of RDII inflow period
    TotalRainVol: f64,        // total rainfall volume (ft3)
    TotalRdiiVol: f64,        // total RDII volume (ft3)
    RdiiFileType: i32,        // type (binary/text) of RDII file
}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  rdii_readRdiiInflow     (called from parseLine in input.c)
//  rdii_deleteRdiiInflow   (called from deleteObjects in project.c)
//  rdii_initUnitHyd        (called from createObjects in project.c)
//  rdii_readUnitHydParams  (called from parseLine in input.c)
//  rdii_openRdii           (called from rain_open)
//  rdii_closeRdii          (called from rain_close)
//  rdii_getNumRdiiFlows    (called from addRdiiInflows in routing.c)
//  rdii_getRdiiFlow        (called from addRdiiInflows in routing.c)

//-----------------------------------------------------------------------------
//  Local helpers standing in for C library / input.c behaviour
//-----------------------------------------------------------------------------

/// input.c `match()` (not exported by input.rs): true if `substr` matches
/// the start of `str` after leading blanks, ignoring case.
fn r#match(str: &[u8], substr: &[u8]) -> i32 {
    let at = |s: &[u8], i: usize| -> u8 { s.get(i).copied().unwrap_or(0) };

    // --- fail if substring is empty
    if at(substr, 0) == 0 {
        return 0;
    }

    // --- skip leading blanks of str
    let mut k = 0usize;
    while at(str, k) != 0 {
        if at(str, k) != b' ' {
            break;
        }
        k += 1;
    }

    // --- check if substr matches remainder of str
    let mut i = k;
    let mut j = 0usize;
    while at(substr, j) != 0 {
        if at(str, i) == 0 || UCHAR(at(str, i)) != UCHAR(at(substr, j)) {
            return 0;
        }
        i += 1;
        j += 1;
    }
    1
}

/// `fgets(line, MAXLINE, f)`: on end of file with nothing read C leaves the
/// buffer unchanged, so `line` keeps its previous contents.
fn fgets_line(f: &mut CFile, line: &mut Vec<u8>) {
    if let Some(l) = f.fgets(MAXLINE as usize) {
        *line = l;
    }
}

/// `fread(&v, 4, 1, f)` of an int. glibc copies whatever bytes it could
/// read, so a short read replaces only the leading bytes. Returns items read.
fn fread_i32(f: &mut CFile, v: &mut i32) -> usize {
    let mut b = v.to_le_bytes();
    let n = f.read_bytes(&mut b);
    *v = i32::from_le_bytes(b);
    n / 4
}

/// `fread(&v, 8, 1, f)` of a double (short reads as for `fread_i32`).
fn fread_f64(f: &mut CFile, v: &mut f64) -> usize {
    let mut b = v.to_le_bytes();
    let n = f.read_bytes(&mut b);
    *v = f64::from_le_bytes(b);
    n / 8
}

/// `fread(a, 4, n, f)` of an int array (short reads as for `fread_i32`).
fn fread_i32s(f: &mut CFile, a: &mut [i32]) -> usize {
    let mut b: Vec<u8> = Vec::with_capacity(a.len() * 4);
    for x in a.iter() {
        b.extend_from_slice(&x.to_le_bytes());
    }
    let n = f.read_bytes(&mut b);
    for (i, x) in a.iter_mut().enumerate() {
        *x = i32::from_le_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]]);
    }
    n / 4
}

/// `fread(a, 4, n, f)` of a float array (short reads as for `fread_i32`).
fn fread_f32s(f: &mut CFile, a: &mut [f32]) -> usize {
    let mut b: Vec<u8> = Vec::with_capacity(a.len() * 4);
    for x in a.iter() {
        b.extend_from_slice(&x.to_le_bytes());
    }
    let n = f.read_bytes(&mut b);
    for (i, x) in a.iter_mut().enumerate() {
        *x = f32::from_le_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]]);
    }
    n / 4
}

//=============================================================================
//                   Management of RDII-Related Data
//=============================================================================

/// Called by: input.
pub fn rdii_readRdiiInflow(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads properties of an RDII inflow from a line of input.
//
{
    let j: i32;
    let k: i32;
    let mut a: f64 = 0.0;

    // --- check for proper number of items
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that node receiving RDII exists
    j = project_findObject(prj, NODE, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- check that RDII unit hydrograph exists
    k = project_findObject(prj, UNITHYD, &tok[1]);
    if k < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[1]);
    }

    // --- read in sewer area value
    if getDouble(&tok[2], &mut a) == 0 || a < 0.0 {
        return error_setInpError(prj, ERR_NUMBER, &tok[2]);
    }

    // --- create the RDII inflow object if it doesn't already exist
    let mut inflow = match prj.Node[j as usize].rdiiInflow {
        Some(x) => x,
        None => TRdiiInflow::default(),
    };

    // --- assign UH & area to inflow object
    inflow.unitHyd = k;
    inflow.area = a / UCF(prj, LANDAREA);

    // --- assign inflow object to node
    prj.Node[j as usize].rdiiInflow = Some(inflow);
    0
}

//=============================================================================

/// Called by: project.
pub fn rdii_initUnitHyd(prj: &mut Project, j: i32)
//
//  Input:   j = UH group index
//  Output:  none
//  Purpose: initializes properties of a unit hydrograph group.
//
{
    let uh = &mut prj.UnitHyd[j as usize];
    for m in 0..12usize {
        for i in 0..3usize {
            uh.iaMax[m][i] = 0.0;
            uh.iaRecov[m][i] = 0.0;
            uh.iaInit[m][i] = 0.0;
            uh.r[m][i] = 0.0;
            uh.tPeak[m][i] = 0;
            uh.tBase[m][i] = 0;
        }
    }
}

//=============================================================================

/// Called by: input.
pub fn rdii_readUnitHydParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads parameters of an RDII unit hydrograph from a line of input.
//
{
    let j: i32;
    let k: i32;
    let m: i32;
    let g: i32;
    let mut x: [f64; 6] = [0.0; 6];

    // --- check that RDII UH object exists in database
    j = project_findObject(prj, UNITHYD, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- assign UH ID to name in hash table
    if prj.UnitHyd[j as usize].ID.is_empty() {
        prj.UnitHyd[j as usize].ID = project_findID(prj, UNITHYD, &tok[0]).unwrap_or_default();
    }

    // --- line has 2 tokens; assign rain gage to UH object
    if ntoks == 2 {
        g = project_findObject(prj, GAGE, &tok[1]);
        if g < 0 {
            return error_setInpError(prj, ERR_NAME, &tok[1]);
        }
        prj.UnitHyd[j as usize].rainGage = g;
        prj.Gage[g as usize].isUsed = TRUE;
        return 0;
    } else if ntoks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- find which month UH params apply to
    m = datetime_findMonth(&tok[1]);
    if m == 0 {
        if r#match(&tok[1], w_ALL) == 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
        }
    }

    // --- find type of UH being specified
    k = findmatch(&tok[2], UHTypeWords);

    // --- if no type match, try using older UH line format
    if k < 0 {
        return readOldUHFormat(prj, j, m, tok, ntoks);
    }

    // --- read the R-T-K parameters
    for i in 0..3usize {
        if getDouble(&tok[i + 3], &mut x[i]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i + 3]);
        }
    }

    // --- read the IA parameters if present
    for i in 3..6usize {
        x[i] = 0.0;
        if ntoks > i as i32 + 3 {
            if getDouble(&tok[i + 3], &mut x[i]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i + 2]);
            }
        }
    }

    // --- save UH params
    setUnitHydParams(prj, j, k, m, &x);
    0
}

//=============================================================================

fn readOldUHFormat(prj: &mut Project, j: i32, m: i32, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   j = unit hydrograph index
//           m = month of year (0 = all months)
//           tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads parameters of a set of RDII unit hydrographs from a line of
//           input.
//
{
    let mut p: [f64; 9] = [0.0; 9];
    // C: x[] is uninitialized. The first setUnitHydParams call below (k=0,
    // i=0) reads x[1], x[2] before they are set (and k=0, i=1 reads x[2]);
    // the call for i=2 overwrites the same months' values with x[0..2] all
    // set, so the garbage never survives. Zeros here.
    let mut x: [f64; 6] = [0.0; 6];

    // --- check for proper number of tokens
    if ntoks < 11 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- read 3 sets of r-t-k values
    for i in 0..9usize {
        if getDouble(&tok[i + 2], &mut p[i]) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[i + 2]);
        }
    }

    // --- read initial abstraction parameters
    for i in 0..3usize {
        x[i + 3] = 0.0;
        if ntoks > i as i32 + 11 {
            if getDouble(&tok[i + 11], &mut x[i + 3]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[i + 11]);
            }
        }
    }

    // --- save UH parameters
    for k in 0..3i32 {
        for i in 0..3usize {
            x[i] = p[3 * k as usize + i];
            setUnitHydParams(prj, j, k, m, &x);
        }
    }
    0
}

//=============================================================================

fn setUnitHydParams(prj: &mut Project, j: i32, i: i32, m: i32, x: &[f64])
//
//  Input:   j = unit hydrograph index
//           i = type of UH response (short, medium or long term)
//           m = month of year (0 = all months)
//           x = array of UH parameters
//  Output:  none
//  Purpose: assigns parameters to a unit hydrograph for a specified month of year.
//
{
    let m1: i32; // start/end month indexes
    let m2: i32;
    let mut t: f64; // UH time to peak (hrs)
    let mut k: f64; // UH k-value
    let mut tBase: f64; // UH base time (hrs)

    // --- find range of months that share same parameter values
    if m == 0 {
        m1 = 0;
        m2 = 11;
    } else {
        m1 = m - 1;
        m2 = m1;
    }

    // --- for each month in the range
    let uh = &mut prj.UnitHyd[j as usize];
    let i = i as usize;
    let mut m = m1;
    while m <= m2 {
        let mu = m as usize;
        // --- set UH response ratio, time to peak, & base time
        uh.r[mu][i] = x[0];
        t = x[1];
        k = x[2];
        tBase = t * (1.0 + k); // hours
        uh.tPeak[mu][i] = f64_to_i64(t * 3600.); // seconds
        uh.tBase[mu][i] = f64_to_i64(tBase * 3600.); // seconds

        // -- set initial abstraction parameters
        uh.iaMax[mu][i] = x[3];
        uh.iaRecov[mu][i] = x[4];
        uh.iaInit[mu][i] = x[5];
        m += 1;
    }
}

//=============================================================================

/// Called by: project.
pub fn rdii_deleteRdiiInflow(prj: &mut Project, j: i32)
//
//  Input:   j = node index
//  Output:  none
//  Purpose: deletes the RDII inflow object for a node.
//
{
    if prj.Node[j as usize].rdiiInflow.is_some() {
        prj.Node[j as usize].rdiiInflow = None;
    }
}

//=============================================================================
//                 Reading Inflow Data From a RDII File
//=============================================================================

/// Called by: rain.
pub fn rdii_openRdii(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: opens an exisiting RDII interface file or creates a new one.
//
{
    // C: char fStamp[] = FILE_STAMP;
    let mut fStamp: [u8; 11] = *b"SWMM5-RDII\0";

    prj.rdii.RdiiNodeIndex = Vec::new();
    prj.rdii.RdiiNodeFlow = Vec::new();
    prj.rdii.NumRdiiNodes = 0;
    prj.rdii.RdiiStartDate = NO_DATE as f64;

    // --- create the RDII file if existing file not being used
    if prj.IgnoreRDII != 0 {
        return;
    }
    if prj.Frdii.mode as i32 != USE_FILE {
        createRdiiFile(prj);
    }
    if prj.Frdii.mode as i32 == NO_FILE || prj.ErrorCode != 0 {
        return;
    }

    // --- try to open the RDII file in binary mode
    prj.Frdii.file = CFile::open(&prj.Frdii.name, "rb");
    if prj.Frdii.file.is_none() {
        if prj.Frdii.mode as i32 == SCRATCH_FILE {
            report_writeErrorMsg(prj, ERR_RDII_FILE_SCRATCH, b"");
        } else {
            let name = prj.Frdii.name;
            report_writeErrorMsg(prj, ERR_RDII_FILE_OPEN, &name);
        }
        return;
    }

    // --- check for valid file stamp
    //     (a short fread leaves the rest of fStamp as initialized)
    prj.Frdii.f().read_bytes(&mut fStamp[..FileStamp.len()]);
    if cio::strcmp(&fStamp, FileStamp) == 0 {
        prj.rdii.RdiiFileType = BINARY;
        prj.ErrorCode = readRdiiFileHeader(prj);
    }
    // --- if stamp invalid try to open the file in text mode
    else {
        prj.Frdii.file = None;
        prj.rdii.RdiiFileType = TEXT;
        openRdiiTextFile(prj);
    }

    // --- catch any error
    if prj.ErrorCode != 0 {
        let name = prj.Frdii.name;
        let code = prj.ErrorCode;
        report_writeErrorMsg(prj, code, &name);
    }
    // --- read the first set of RDII flows form the file
    else {
        readRdiiFlows(prj);
    }
}

//=============================================================================

fn openRdiiTextFile(prj: &mut Project) {
    // --- try to open the RDII file in text mode
    prj.Frdii.file = CFile::open(&prj.Frdii.name, "rt");
    if prj.Frdii.file.is_none() {
        if prj.Frdii.mode as i32 == SCRATCH_FILE {
            report_writeErrorMsg(prj, ERR_RDII_FILE_SCRATCH, b"");
        } else {
            let name = prj.Frdii.name;
            report_writeErrorMsg(prj, ERR_RDII_FILE_OPEN, &name);
        }
        return;
    }

    // --- read header records from file
    prj.ErrorCode = readRdiiTextFileHeader(prj);
    if prj.ErrorCode != 0 {
        let name = prj.Frdii.name;
        let code = prj.ErrorCode;
        report_writeErrorMsg(prj, code, &name);
    }
}

//=============================================================================

/// Called by: rain.
pub fn rdii_closeRdii(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: closes the RDII interface file.
//
{
    // C fcloses without setting Frdii.file to NULL; closing it here as None
    // makes a second close a no-op instead of C's double fclose.
    if prj.Frdii.file.is_some() {
        prj.Frdii.file = None;
    }
    if prj.Frdii.mode as i32 == SCRATCH_FILE {
        cio::remove(&prj.Frdii.name);
    }
    prj.rdii.RdiiNodeIndex = Vec::new();
    prj.rdii.RdiiNodeFlow = Vec::new();
}

//=============================================================================

/// Called by: routing.
pub fn rdii_getNumRdiiFlows(prj: &mut Project, aDate: DateTime) -> i32
//
//  Input:   aDate = current date/time
//  Output:  returns 0 if no RDII flow or number of nodes with RDII inflows
//  Purpose: finds number of RDII inflows at a specified date.
//
{
    // --- default result is 0 indicating no RDII inflow at specified date
    if prj.rdii.NumRdiiNodes == 0 {
        return 0;
    }
    if prj.Frdii.file.is_none() {
        return 0;
    }

    // --- keep reading RDII file as need be
    while !prj.Frdii.f().feof() {
        // --- return if date of current RDII inflow not reached yet
        if prj.rdii.RdiiStartDate == NO_DATE as f64 {
            return 0;
        }
        if aDate < prj.rdii.RdiiStartDate {
            return 0;
        }

        // --- return RDII node count if specified date falls
        //     within time interval of current RDII inflow
        if aDate < prj.rdii.RdiiEndDate {
            return prj.rdii.NumRdiiNodes;
        }
        // --- otherwise get next date and RDII flow values from file
        else {
            readRdiiFlows(prj);
        }
    }
    0
}

//=============================================================================

/// Called by: routing.
pub fn rdii_getRdiiFlow(prj: &mut Project, i: i32, j: &mut i32, q: &mut f64)
//
//  Input:   i = RDII node index
//           j = pointer to project node index
//           q = pointer to RDII flow rate
//  Output:  sets node index and RDII inflow for node
//  Purpose: finds index and current RDII inflow for an RDII node.
//
{
    if i >= 0 && i < prj.rdii.NumRdiiNodes {
        *j = prj.rdii.RdiiNodeIndex[i as usize];
        *q = prj.rdii.RdiiNodeFlow[i as usize] as f64;
    }
}

//=============================================================================

fn readRdiiFileHeader(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns error code
//  Purpose: reads header information from a binary RDII file.
//
{
    let mut j: i32;

    // --- extract time step and number of RDII nodes
    fread_i32(prj.Frdii.f(), &mut prj.rdii.RdiiStep);
    if prj.rdii.RdiiStep <= 0 {
        return ERR_RDII_FILE_FORMAT;
    }
    fread_i32(prj.Frdii.f(), &mut prj.rdii.NumRdiiNodes);
    if prj.rdii.NumRdiiNodes <= 0 {
        return ERR_RDII_FILE_FORMAT;
    }

    // --- allocate memory for RdiiNodeIndex & RdiiNodeFlow arrays
    let n = prj.rdii.NumRdiiNodes as usize;
    prj.rdii.RdiiNodeIndex = vec![0; n];
    prj.rdii.RdiiNodeFlow = vec![0.0; n];

    // --- read indexes of RDII nodes
    if prj.Frdii.f().feof() {
        return ERR_RDII_FILE_FORMAT;
    }
    fread_i32s(prj.Frdii.f(), &mut prj.rdii.RdiiNodeIndex[..n]);
    for i in 0..n {
        j = prj.rdii.RdiiNodeIndex[i];
        // C indexes Node[j] unchecked (j comes from the file)
        if prj.Node[j as usize].rdiiInflow.is_none() {
            return ERR_RDII_FILE_FORMAT;
        }
    }
    if prj.Frdii.f().feof() {
        return ERR_RDII_FILE_FORMAT;
    }
    0
}

//=============================================================================

fn readRdiiTextFileHeader(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns error code
//  Purpose: reads header information from a text RDII file.
//
{
    let mut j: i32;
    // C: line and s2 start uninitialized; on an fgets at end of file C keeps
    // the buffer's previous contents, which is what these Vecs model (the
    // first fgets here always reads data: the file is non-empty).
    let mut line: Vec<u8> = Vec::new(); // line from RDII data file
    let mut s1: Vec<u8> = Vec::new(); // general string variable
    let mut s2: Vec<u8> = Vec::new();

    // --- check for correct file type
    fgets_line(prj.Frdii.f(), &mut line);
    if cio::sscanf(&line, b"%s", &mut [Scan::Str(&mut s1)]) == 0 || cio::strcmp(&s1, b"SWMM5") != 0 {
        return ERR_RDII_FILE_FORMAT;
    }

    // --- skip title line
    fgets_line(prj.Frdii.f(), &mut line);

    // --- read RDII UH time step interval (sec)
    prj.rdii.RdiiStep = 0;
    fgets_line(prj.Frdii.f(), &mut line);
    if cio::sscanf(&line, b"%d", &mut [Scan::Int(&mut prj.rdii.RdiiStep)]) == 0 || prj.rdii.RdiiStep <= 0 {
        return ERR_RDII_FILE_FORMAT;
    }

    // --- skip over line with number of constituents (= 1 for RDII)
    fgets_line(prj.Frdii.f(), &mut line);

    // --- read flow units
    fgets_line(prj.Frdii.f(), &mut line);
    if cio::sscanf(&line, b"%s %s", &mut [Scan::Str(&mut s1), Scan::Str(&mut s2)]) < 2 {
        return ERR_RDII_FILE_FORMAT;
    }
    prj.rdii.RdiiFlowUnits = findmatch(&s2, FlowUnitWords);
    if prj.rdii.RdiiFlowUnits < 0 {
        return ERR_RDII_FILE_FORMAT;
    }

    // --- read number of RDII nodes
    fgets_line(prj.Frdii.f(), &mut line);
    if cio::sscanf(&line, b"%d", &mut [Scan::Int(&mut prj.rdii.NumRdiiNodes)]) < 1 || prj.rdii.NumRdiiNodes <= 0 {
        return ERR_RDII_FILE_FORMAT;
    }

    // --- allocate memory for RdiiNodeIndex & RdiiNodeFlow arrays
    let n = prj.rdii.NumRdiiNodes as usize;
    prj.rdii.RdiiNodeIndex = vec![0; n];
    prj.rdii.RdiiNodeFlow = vec![0.0; n];

    // --- read names of RDII nodes from file & save their indexes
    for i in 0..n {
        if prj.Frdii.f().feof() {
            return ERR_RDII_FILE_FORMAT;
        }
        fgets_line(prj.Frdii.f(), &mut line);
        if cio::sscanf(&line, b"%s", &mut [Scan::Str(&mut s1)]) == 0 {
            return ERR_RDII_FILE_FORMAT;
        }
        j = project_findObject(prj, NODE, &s1);
        if j < 0 {
            return ERR_RDII_FILE_FORMAT;
        }
        prj.rdii.RdiiNodeIndex[i] = j;
    }

    // --- skip column heading line
    if prj.Frdii.f().feof() {
        return ERR_RDII_FILE_FORMAT;
    }
    fgets_line(prj.Frdii.f(), &mut line);
    0
}

//=============================================================================

fn readRdiiFlows(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: reads date and flow values of next RDII inflows from RDII file.
//
{
    if prj.rdii.RdiiFileType == TEXT {
        readRdiiTextFlows(prj);
    } else {
        prj.rdii.RdiiStartDate = NO_DATE as f64;
        prj.rdii.RdiiEndDate = NO_DATE as f64;
        if prj.Frdii.f().feof() {
            return;
        }
        fread_f64(prj.Frdii.f(), &mut prj.rdii.RdiiStartDate);
        if prj.rdii.RdiiStartDate == NO_DATE as f64 {
            return;
        }
        let n = prj.rdii.NumRdiiNodes as usize;
        if fread_f32s(prj.Frdii.f(), &mut prj.rdii.RdiiNodeFlow[..n]) < n {
            prj.rdii.RdiiStartDate = NO_DATE as f64;
        } else {
            prj.rdii.RdiiEndDate = datetime_addSeconds(prj.rdii.RdiiStartDate, prj.rdii.RdiiStep as f64);
        }
    }
}

//=============================================================================

fn readRdiiTextFlows(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: reads date and flow values of next RDII inflows from RDII file.
//
{
    let mut n: i32;
    let mut yr: i32 = 0;
    let mut mon: i32 = 0;
    let mut day: i32 = 0;
    let mut hr: i32 = 0;
    let mut min: i32 = 0;
    let mut sec: i32 = 0; // year, month, day, hour, minute, second
    let mut x: f64 = 0.0; // RDII flow in original units
    // C: line is an uninitialized local; if fgets finds end of file without
    // feof having been set (last line ended with a newline), C parses stale
    // stack bytes. It is taken as empty here: either way no further RDII
    // inflow is returned (see rdii_getNumRdiiFlows).
    let mut line: Vec<u8> = Vec::new(); // line from RDII data file
    let mut s: Vec<u8> = Vec::new(); // node ID label (not used)

    prj.rdii.RdiiStartDate = NO_DATE as f64;
    for i in 0..prj.rdii.NumRdiiNodes as usize {
        if prj.Frdii.f().feof() {
            return;
        }
        fgets_line(prj.Frdii.f(), &mut line);
        n = cio::sscanf(
            &line,
            b"%s %d %d %d %d %d %d %lf",
            &mut [
                Scan::Str(&mut s),
                Scan::Int(&mut yr),
                Scan::Int(&mut mon),
                Scan::Int(&mut day),
                Scan::Int(&mut hr),
                Scan::Int(&mut min),
                Scan::Int(&mut sec),
                Scan::Double(&mut x),
            ],
        );
        if n < 8 {
            return;
        }
        prj.rdii.RdiiNodeFlow[i] = (x / Qcf[prj.rdii.RdiiFlowUnits as usize]) as f32;
    }
    prj.rdii.RdiiStartDate = datetime_encodeDate(yr, mon, day) + datetime_encodeTime(hr, min, sec as f64);
    prj.rdii.RdiiEndDate = datetime_addSeconds(prj.rdii.RdiiStartDate, prj.rdii.RdiiStep as f64);
}

//=============================================================================
//                   Creation of a RDII Interface File
//=============================================================================

fn createRdiiFile(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: computes time history of RDII inflows and saves them to file.
//
{
    let mut hasRdii: i32; // true when total RDII > 0
    let mut elapsedTime: f64; // current elapsed time (sec)
    let duration: f64; // duration being analyzed (sec)
    let mut currentDate: DateTime; // current calendar date/time

    // --- set RDII reporting time step to Runoff wet step
    prj.rdii.RdiiStep = prj.WetStep;

    // --- count nodes with RDII data
    prj.rdii.NumRdiiNodes = getNumRdiiNodes(prj);

    // --- if no RDII nodes then re-set RDII file usage to NO_FILE
    if prj.rdii.NumRdiiNodes == 0 {
        prj.Frdii.mode = NO_FILE as i8;
        return;
    }
    // --- otherwise set file usage to SCRATCH if originally set to NO_FILE
    else if prj.Frdii.mode as i32 == NO_FILE {
        prj.Frdii.mode = SCRATCH_FILE as i8;
    }

    // --- validate RDII data
    validateRdii(prj);
    initGageData(prj);
    if prj.ErrorCode != 0 {
        return;
    }

    // --- open RDII processing system
    openRdiiProcessor(prj);
    if prj.ErrorCode == 0 {
        // --- initialize rain gage & UH processing data
        initUnitHydData(prj);

        // --- convert total simulation duration from millisec to sec
        duration = prj.TotalDuration / 1000.0;

        // --- examine rainfall record over each RdiiStep time step
        elapsedTime = 0.0;
        while elapsedTime <= duration && prj.ErrorCode == 0 {
            // --- compute current calendar date/time
            currentDate = prj.StartDateTime + elapsedTime / SECperDAY as f64;

            // --- update rainfall at all rain gages
            getRainfall(prj, currentDate);

            // --- compute convolutions of past rainfall with UH's
            getUnitHydRdii(prj, currentDate);

            // --- find RDII at all nodes
            hasRdii = getNodeRdii(prj);

            // --- save RDII at all nodes to file for current date
            if hasRdii != 0 {
                saveRdiiFlows(prj, currentDate);
            }

            // --- advance one time step
            elapsedTime += prj.rdii.RdiiStep as f64;
        }
    }

    // --- close RDII processing system
    closeRdiiProcessor(prj);
}

//=============================================================================

fn getNumRdiiNodes(prj: &Project) -> i32
//
//  Input:   none
//  Output:  returns node count
//  Purpose: counts number of nodes that receive RDII inflow.
//
{
    let mut n: i32; // node count

    n = 0;
    for j in 0..prj.Nobjects[NODE as usize] as usize {
        if prj.Node[j].rdiiInflow.is_some() {
            n += 1;
        }
    }
    n
}

//=============================================================================

fn validateRdii(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: validates UH and RDII inflow object data.
//
{
    let mut rsum: f64; // sum of UH r-values
                       //  long   gageInterval;               // rain gage time interval

    // --- check each unit hydrograph for consistency
    for j in 0..prj.Nobjects[UNITHYD as usize] as usize {
        for m in 0..12usize {
            rsum = 0.0;
            for k in 0..3usize {
                // --- if no base time then UH doesn't exist
                if prj.UnitHyd[j].tBase[m][k] == 0 {
                    continue;
                }

                // --- restriction on time to peak being less than the
                //     rain gage's recording interval no longer applies

                // --- can't have negative UH parameters
                if (prj.UnitHyd[j].tPeak[m][k] as f64) < 0.0 {
                    let id = prj.UnitHyd[j].ID.clone();
                    report_writeErrorMsg(prj, ERR_UNITHYD_TIMES, &id);
                }

                // --- can't have negative UH response ratio
                if prj.UnitHyd[j].r[m][k] < 0.0 {
                    let id = prj.UnitHyd[j].ID.clone();
                    report_writeErrorMsg(prj, ERR_UNITHYD_RATIOS, &id);
                } else {
                    rsum += prj.UnitHyd[j].r[m][k];
                }
            }
            if rsum > 1.01 {
                let id = prj.UnitHyd[j].ID.clone();
                report_writeErrorMsg(prj, ERR_UNITHYD_RATIOS, &id);
            }
        }
    }

    // --- check each node's RDII inflow object
    for i in 0..prj.Nobjects[NODE as usize] as usize {
        if let Some(inflow) = prj.Node[i].rdiiInflow {
            // --- check that sewer area is non-negative
            if inflow.area < 0.0 {
                let id = prj.Node[i].ID.clone();
                report_writeErrorMsg(prj, ERR_RDII_AREA, &id);
            }
        }
    }
}

//=============================================================================

fn openRdiiProcessor(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: opens RDII processing system.
//
{
    let mut n: usize; // RDII node count

    // --- set RDII processing arrays to NULL
    prj.rdii.UHGroup = Vec::new();
    prj.rdii.RdiiNodeIndex = Vec::new();
    prj.rdii.RdiiNodeFlow = Vec::new();
    prj.rdii.TotalRainVol = 0.0;
    prj.rdii.TotalRdiiVol = 0.0;

    // --- allocate memory used for RDII processing
    if allocRdiiMemory(prj) == 0 {
        report_writeErrorMsg(prj, ERR_MEMORY, b"");
        return;
    }

    // --- open & initialize RDII file
    if openNewRdiiFile(prj) == 0 {
        report_writeErrorMsg(prj, ERR_RDII_FILE_SCRATCH, b"");
        return;
    }

    // --- identify index of each node with RDII inflow
    //     (C also tests RdiiNodeIndex != NULL; it is allocated here)
    n = 0;
    for j in 0..prj.Nobjects[NODE as usize] as usize {
        if prj.Node[j].rdiiInflow.is_some() {
            prj.rdii.RdiiNodeIndex[n] = j as i32;
            n += 1;
        }
    }
}

//=============================================================================

fn allocRdiiMemory(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns TRUE if successful, FALSE if not
//  Purpose: allocates memory used for RDII processing .
//
//
{
    let mut n: i32; // number of past rain periods

    // --- allocate memory for RDII processing data for UH groups
    let nuh = prj.Nobjects[UNITHYD as usize] as usize;
    prj.rdii.UHGroup = vec![TUHGroup::default(); nuh];

    // --- allocate memory for past rainfall data for each UH in each group
    for i in 0..nuh {
        let ri = getRainInterval(prj, i as i32);
        prj.rdii.UHGroup[i].rainInterval = ri;
        for k in 0..3usize {
            prj.rdii.UHGroup[i].uh[k].pastRain = Vec::new();
            prj.rdii.UHGroup[i].uh[k].pastMonth = Vec::new();
            let mp = getMaxPeriods(prj, i as i32, k as i32);
            prj.rdii.UHGroup[i].uh[k].maxPeriods = mp;
            n = prj.rdii.UHGroup[i].uh[k].maxPeriods;
            if n > 0 {
                prj.rdii.UHGroup[i].uh[k].pastRain = vec![0.0; n as usize];
                prj.rdii.UHGroup[i].uh[k].pastMonth = vec![0; n as usize];
            }
        }
    }

    // --- allocate memory for RDII indexes & inflow at each node w/ RDII data
    let nn = prj.rdii.NumRdiiNodes as usize;
    prj.rdii.RdiiNodeIndex = vec![0; nn];
    prj.rdii.RdiiNodeFlow = vec![0.0; nn];
    TRUE
}

//=============================================================================

fn getRainInterval(prj: &Project, i: i32) -> i32
//
//  Input:   i = UH group index
//  Output:  returns a time interval (sec)
//  Purpose: finds rainfall processing time interval for a unit hydrograph group.
//
{
    let mut ri: i32; // rainfal processing time interval for the UH group
    let mut tLimb: i32; // duration of a UH's rising & falling limbs

    // --- begin with UH group time step equal to wet runoff step
    ri = prj.WetStep;

    // --- examine each UH in the group
    let uh = &prj.UnitHyd[i as usize];
    for m in 0..12usize {
        for k in 0..3usize {
            // --- make sure the UH exists
            if uh.tPeak[m][k] > 0 {
                // --- reduce time step if rising/falling limb is smaller
                tLimb = uh.tPeak[m][k] as i32;
                ri = MIN(ri, tLimb);
                tLimb = (uh.tBase[m][k] - tLimb as i64) as i32;
                if tLimb > 0 {
                    ri = MIN(ri, tLimb);
                }
            }
        }
    }
    ri
}

//=============================================================================

fn getMaxPeriods(prj: &Project, i: i32, k: i32) -> i32
//
//  Input:   i = UH group index
//           k = UH index
//  Output:  returns number of past rainfall values
//  Purpose: finds number of past rainfall values to save for a UH.
//
{
    let mut n: i32; // number of time periods
    let mut nMax: i32; // maximum number of time periods
    let rainInterval: i32; // rainfall processing interval (sec)

    // --- examine each monthly set of UHs
    rainInterval = prj.rdii.UHGroup[i as usize].rainInterval;
    nMax = 0;
    for m in 0..12usize {
        // --- compute number of time periods in UH base
        n = (prj.UnitHyd[i as usize].tBase[m][k as usize] / rainInterval as i64 + 1) as i32;

        // --- update number of time periods to be saved
        nMax = MAX(n, nMax);
    }
    nMax
}

//=============================================================================

fn initGageData(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: initializes state of Unit Hydrograph rain gages.
//
{
    let mut g: i32; // rain gage index

    // --- first initialize the state of each rain gage
    for g in 0..prj.Nobjects[GAGE as usize] {
        let ts = prj.Gage[g as usize].tSeries;
        if ts >= 0 {
            table_tseriesInit(&mut prj.Tseries[ts as usize]);
        }
        gage_initState(prj, g);
    }

    // --- then flag each gage that is used by a Unit Hydrograph set
    for i in 0..prj.Nobjects[UNITHYD as usize] as usize {
        g = prj.UnitHyd[i].rainGage;
        if g >= 0 {
            // --- if UH's gage uses same time series as a previous gage,
            //     then assign the latter gage to the UH
            if prj.Gage[g as usize].coGage >= 0 {
                prj.UnitHyd[i].rainGage = prj.Gage[g as usize].coGage;
            }
        }
    }
}

//=============================================================================

fn initUnitHydData(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: initializes unit hydrograph processing data.
//
{
    let mut i: usize; // UH group index
    let mut j: usize; // node index
                      //  int g,                             // rain gage index
    let month: usize; // month index

    // --- initialize UHGroup entries for each Unit Hydrograph
    month = (datetime_monthOfYear(prj.StartDateTime) - 1) as usize;
    let nuh = prj.Nobjects[UNITHYD as usize] as usize;
    for i in 0..nuh {
        for k in 0..3usize {
            // --- make the first recorded rainfall begin a new RDII event
            // --- (new RDII event occurs when dry period > base of longest UH)
            let grp = &mut prj.rdii.UHGroup[i];
            grp.uh[k].drySeconds = (grp.uh[k].maxPeriods * grp.rainInterval + 1) as i64;
            grp.uh[k].period = grp.uh[k].maxPeriods + 1;
            grp.uh[k].hasPastRain = FALSE;

            // --- assign initial abstraction used
            grp.uh[k].iaUsed = prj.UnitHyd[i].iaInit[month][k];
        }

        // --- initialize gage date to simulation start date
        prj.rdii.UHGroup[i].gageDate = prj.StartDateTime;
        prj.rdii.UHGroup[i].area = 0.0;
        prj.rdii.UHGroup[i].rdii = 0.0;
    }

    // --- assume each UH group is not used
    for i in 0..nuh {
        prj.rdii.UHGroup[i].isUsed = FALSE;
    }

    // --- look at each node with RDII inflow
    for n in 0..prj.rdii.NumRdiiNodes as usize {
        // --- mark as used the UH group associated with the node
        j = prj.rdii.RdiiNodeIndex[n] as usize;
        let inflow = prj.Node[j].rdiiInflow.expect("RDII node without RDII inflow");
        i = inflow.unitHyd as usize;
        prj.rdii.UHGroup[i].isUsed = TRUE;

        // --- add node's sewer area to UH group's area
        prj.rdii.UHGroup[i].lastDate = prj.StartDateTime;
        prj.rdii.UHGroup[i].area += inflow.area;
    }
}

//=============================================================================

fn openNewRdiiFile(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns TRUE if successful, FALSE if not
//  Purpose: opens a new RDII interface file.
//
{
    // --- create a temporary file name if scratch file being used
    if prj.Frdii.mode as i32 == SCRATCH_FILE {
        let mut name = prj.Frdii.name;
        getTempFileName(prj, &mut name);
        prj.Frdii.name = name;
    }

    // --- open the RDII file as a formatted text file
    prj.Frdii.file = CFile::open(&prj.Frdii.name, "w+b");
    if prj.Frdii.file.is_none() {
        return FALSE;
    }

    // --- write file stamp to RDII file
    prj.Frdii.write_bytes(FileStamp);

    // --- initialize the contents of the file with RDII time step (sec),
    //     number of RDII nodes, and index of each node
    let b = prj.rdii.RdiiStep.to_le_bytes();
    prj.Frdii.write_bytes(&b);
    let b = prj.rdii.NumRdiiNodes.to_le_bytes();
    prj.Frdii.write_bytes(&b);
    for j in 0..prj.Nobjects[NODE as usize] {
        if prj.Node[j as usize].rdiiInflow.is_some() {
            prj.Frdii.write_bytes(&j.to_le_bytes());
        }
    }
    TRUE
}

//=============================================================================

fn getRainfall(prj: &mut Project, currentDate: DateTime)
//
//  Input:   currentDate = current calendar date/time
//  Output:  none
//  Purpose: determines rainfall at current RDII processing date.
//
//
{
    let mut g: i32; // rain gage index
    let mut i: i32; // past rainfall index
    let month: i32; // month of current date
    let mut rainInterval: i32; // rainfall interval (sec)
    let mut rainDepth: f64; // rainfall depth (inches or mm)
    let mut excessDepth: f64; // excess rainfall depth (inches or mm))
    let mut gageDate: DateTime; // calendar date for rain gage

    // --- examine each UH group
    month = datetime_monthOfYear(currentDate) - 1;
    for g in 0..prj.Nobjects[GAGE as usize] as usize {
        prj.Gage[g].isCurrent = FALSE;
    }
    for j in 0..prj.Nobjects[UNITHYD as usize] {
        let ju = j as usize;
        // --- repeat until gage's date reaches or exceeds current date
        g = prj.UnitHyd[ju].rainGage;
        rainInterval = prj.rdii.UHGroup[ju].rainInterval;
        while prj.rdii.UHGroup[ju].gageDate < currentDate {
            // --- get rainfall volume over gage's recording interval
            //     at gage'a current date (in original depth units)
            gageDate = prj.rdii.UHGroup[ju].gageDate;
            prj.Adjust.rainFactor = prj.Adjust.rain[(datetime_monthOfYear(gageDate) - 1) as usize];
            if prj.Gage[g as usize].isCurrent == 0 {
                gage_setState(prj, g, gageDate);
                prj.Gage[g as usize].isCurrent = TRUE;
            }
            rainDepth = prj.Gage[g as usize].rainfall * rainInterval as f64 / 3600.0;

            // --- update amount of total rainfall volume (ft3)
            prj.rdii.TotalRainVol += rainDepth / UCF(prj, RAINDEPTH) * prj.rdii.UHGroup[ju].area;

            // --- compute rainfall excess for each UH in the group
            for k in 0..3i32 {
                let ku = k as usize;
                // --- adjust rainfall volume for any initial abstraction
                excessDepth = applyIA(prj, j, k, gageDate, rainInterval as f64, rainDepth);

                // --- adjust extent of dry period for the UH
                updateDryPeriod(prj, j, k, excessDepth, rainInterval);

                // --- add rainfall to list of past values,
                //     wrapping array index if necessary
                let uh = &mut prj.rdii.UHGroup[ju].uh[ku];
                i = uh.period;
                if i >= uh.maxPeriods {
                    i = 0;
                }
                uh.pastRain[i as usize] = excessDepth;
                uh.pastMonth[i as usize] = month as i8;
                uh.period = i + 1;
            }

            // --- advance rain date by gage recording interval
            prj.rdii.UHGroup[ju].gageDate = datetime_addSeconds(gageDate, rainInterval as f64);
        }
    }
}

//=============================================================================

fn applyIA(prj: &mut Project, j: i32, k: i32, aDate: DateTime, dt: f64, rainDepth: f64) -> f64
//
//  Input:   j = UH group index
//           k = unit hydrograph index
//           aDate = current date/time
//           dt = time interval (sec)
//           rainDepth = unadjusted rain depth (in or mm)
//  Output:  returns rainfall adjusted for initial abstraction (IA)
//  Purpose: adjusts rainfall for any initial abstraction and updates the
//           amount of available initial abstraction actually used.
//
{
    let m: usize;
    let mut ia: f64;
    let mut netRainDepth: f64;
    let (ju, ku) = (j as usize, k as usize);

    // --- determine amount of unused IA
    m = (datetime_monthOfYear(aDate) - 1) as usize;
    ia = prj.UnitHyd[ju].iaMax[m][ku] - prj.rdii.UHGroup[ju].uh[ku].iaUsed;
    ia = MAX(ia, 0.0);

    // --- case where there's some rainfall
    if rainDepth > 0.0 {
        // --- reduce rain depth by unused IA
        netRainDepth = rainDepth - ia;
        netRainDepth = MAX(netRainDepth, 0.0);

        // --- update amount of IA used up
        ia = rainDepth - netRainDepth;
        prj.rdii.UHGroup[ju].uh[ku].iaUsed += ia;
    }
    // --- case where there's no rainfall
    else {
        // --- recover a portion of the IA already used
        prj.rdii.UHGroup[ju].uh[ku].iaUsed -= dt / 86400. * prj.UnitHyd[ju].iaRecov[m][ku];
        prj.rdii.UHGroup[ju].uh[ku].iaUsed = MAX(prj.rdii.UHGroup[ju].uh[ku].iaUsed, 0.0);
        netRainDepth = 0.0;
    }
    netRainDepth
}

//=============================================================================

fn updateDryPeriod(prj: &mut Project, j: i32, k: i32, rainDepth: f64, rainInterval: i32)
//
//  Input:   j = UH group index
//           k = unit hydrograph index
//           rainDepth = excess rain depth (in or mm)
//           rainInterval = rainfall time interval (sec)
//  Output:  none
//  Purpose: adjusts the length of the dry period between rainfall events.
//
{
    let uh = &mut prj.rdii.UHGroup[j as usize].uh[k as usize];

    // --- if rainfall occurs
    if rainDepth > 0.0 {
        // --- if previous dry period long enough then begin
        //     new RDII event with time period index set to 0
        if uh.drySeconds >= (rainInterval * uh.maxPeriods) as i64 {
            for i in 0..uh.maxPeriods as usize {
                uh.pastRain[i] = 0.0;
            }
            uh.period = 0;
        }
        uh.drySeconds = 0;
        uh.hasPastRain = TRUE;
    }
    // --- if no rainfall, update duration of dry period
    else {
        uh.drySeconds += rainInterval as i64;
        if uh.drySeconds >= (rainInterval * uh.maxPeriods) as i64 {
            uh.hasPastRain = FALSE;
        } else {
            uh.hasPastRain = TRUE;
        }
    }
}

//=============================================================================

fn getUnitHydRdii(prj: &mut Project, currentDate: DateTime)
//
//  Input:   currentDate = current calendar date/time
//  Output:  none
//  Purpose: computes RDII generated by past rainfall for each UH group.
//
{
    let mut rainInterval: i32; // rainfall time interval (sec)

    // --- examine each UH group
    for j in 0..prj.Nobjects[UNITHYD as usize] {
        let ju = j as usize;
        // --- skip calculation if group not used by any RDII node or if
        //     current date hasn't reached last date RDII was computed
        if prj.rdii.UHGroup[ju].isUsed == 0 {
            continue;
        }
        if currentDate < prj.rdii.UHGroup[ju].lastDate {
            continue;
        }

        // --- update date RDII last computed
        prj.rdii.UHGroup[ju].lastDate = prj.rdii.UHGroup[ju].gageDate;

        // --- perform convolution for each UH in the group
        rainInterval = prj.rdii.UHGroup[ju].rainInterval;
        prj.rdii.UHGroup[ju].rdii = 0.0;
        for k in 0..3i32 {
            if prj.rdii.UHGroup[ju].uh[k as usize].hasPastRain != 0 {
                let r = getUnitHydConvol(prj, j, k, rainInterval);
                prj.rdii.UHGroup[ju].rdii += r;
            }
        }
    }
}

//=============================================================================

fn getUnitHydConvol(prj: &Project, j: i32, k: i32, rainInterval: i32) -> f64
//
//  Input:   j = UH group index
//           k = UH index
//           rainInterval = rainfall time interval (sec)
//  Output:  returns a RDII flow value
//  Purpose: computes convolution of Unit Hydrographs with past rainfall.
//
{
    let mut i: i32; // previous rainfall period index
    let mut m: i32; // month of year index
    let mut p: i32; // UH time period index
    let pMax: i32; // max. number of periods
    let mut t: f64; // UH time value (sec)
    let mut u: f64; // UH ordinate
    let mut v: f64; // rainfall volume
    let mut rdii: f64; // RDII flow

    // --- initialize RDII, rain period index and UH period index
    rdii = 0.0;
    let uh = &prj.rdii.UHGroup[j as usize].uh[k as usize]; // UH data
    i = uh.period - 1;
    if i < 0 {
        i = uh.maxPeriods - 1;
    }
    pMax = uh.maxPeriods;
    p = 1;

    // --- evaluate each time period of UH's
    while p < pMax {
        // --- if rain period has rainfall
        v = uh.pastRain[i as usize];
        m = uh.pastMonth[i as usize] as i32;
        if v > 0.0 {
            // --- find mid-point time of UH period in seconds
            t = (p as f64 - 0.5) * rainInterval as f64;

            // --- convolute rain volume with UH ordinate
            u = getUnitHydOrd(prj, j, m, k, t) * prj.UnitHyd[j as usize].r[m as usize][k as usize];
            rdii += u * v;
        }

        // --- move to next UH period & previous rainfall period
        p = p + 1;
        i = i - 1;
        if i < 0 {
            i = uh.maxPeriods - 1;
        }
    }
    rdii
}

//=============================================================================

fn getUnitHydOrd(prj: &Project, h: i32, m: i32, k: i32, t: f64) -> f64
//
//  Input:   h = index of UH group
//           m = month index
//           k = individual UH index
//           t = UH time (sec)
//  Output:  returns ordinate of a unit hydrograph
//  Purpose: gets ordinate of a particular unit hydrograph at specified time.
//
{
    let qPeak: f64; // peak flow of unit hydrograph
    let f: f64; // fraction of time to/from peak on UH
    let t1: f64; // time to peak on UH (sec)
    let t2: f64; // time after peak on UH (sec)
    let tBase: f64; // base time of UH (sec)
    let (h, m, k) = (h as usize, m as usize, k as usize);

    // --- return 0 if past end of UH time base
    tBase = prj.UnitHyd[h].tBase[m][k] as f64;
    if t >= tBase {
        return 0.0;
    }

    // --- compute peak value of UH in original rainfall units (in/hr or mm/hr)
    qPeak = 2. / tBase * 3600.0;

    // --- break UH base into times before & after peak flow
    t1 = prj.UnitHyd[h].tPeak[m][k] as f64;
    t2 = tBase - t1;

    // --- find UH flow at time t
    if t <= t1 {
        f = t / t1;
    } else {
        f = 1.0 - (t - t1) / t2;
    }
    MAX(f, 0.0) * qPeak
}

//=============================================================================

fn getNodeRdii(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns TRUE if any node has RDII inflow, FALSE if not
//  Purpose: computes current RDII inflow at each node.
//
{
    let mut hasRdii: i32 = FALSE; // true if any node has some RDII
    let mut i: usize; // UH group index
    let mut j: usize; // node index
    let mut rdii: f64; // RDII flow (cfs)

    // --- examine each node w/ RDII data
    for n in 0..prj.rdii.NumRdiiNodes as usize {
        // --- identify node's index in project's data base
        j = prj.rdii.RdiiNodeIndex[n] as usize;

        // --- apply node's sewer area to UH RDII to get node RDII in CFS
        let inflow = prj.Node[j].rdiiInflow.expect("RDII node without RDII inflow");
        i = inflow.unitHyd as usize;
        rdii = prj.rdii.UHGroup[i].rdii * inflow.area / UCF(prj, RAINFALL);
        if rdii < ZERO_RDII {
            rdii = 0.0;
        } else {
            hasRdii = TRUE;
        }

        // --- update total RDII volume
        prj.rdii.RdiiNodeFlow[n] = rdii as f32;
        if rdii > 0.0 {
            prj.rdii.TotalRdiiVol += rdii * prj.rdii.RdiiStep as f64;
        }
    }
    hasRdii
}

//=============================================================================

fn saveRdiiFlows(prj: &mut Project, currentDate: DateTime)
//
//  Input:   currentDate = current calendar date/time
//  Output:  none
//  Purpose: saves current set of RDII inflows in current flow units to file.
//
{
    let n = prj.rdii.NumRdiiNodes as usize;
    let mut b: Vec<u8> = Vec::with_capacity(8 + 4 * n);
    b.extend_from_slice(&currentDate.to_le_bytes());
    for x in &prj.rdii.RdiiNodeFlow[..n] {
        b.extend_from_slice(&x.to_le_bytes());
    }
    prj.Frdii.write_bytes(&b);
}

//=============================================================================

fn closeRdiiProcessor(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: closes RDII processing system.
//
{
    // --- write rainfall & RDII totals to report file
    if prj.ErrorCode == 0 {
        let (rainVol, rdiiVol) = (prj.rdii.TotalRainVol, prj.rdii.TotalRdiiVol);
        report_writeRdiiStats(prj, rainVol, rdiiVol);
    }

    // --- free allocated memory and close RDII file
    //     (C fcloses without clearing Frdii.file; see rdii_closeRdii)
    freeRdiiMemory(prj);
    if prj.Frdii.file.is_some() {
        prj.Frdii.file = None;
    }
}

//=============================================================================

fn freeRdiiMemory(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: frees memory used for RDII processing.
//
{
    prj.rdii.UHGroup = Vec::new();
    prj.rdii.RdiiNodeIndex = Vec::new();
    prj.rdii.RdiiNodeFlow = Vec::new();
}
