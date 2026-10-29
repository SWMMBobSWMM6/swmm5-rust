//! iface.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   iface.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21 (Build 5.2.0)
//   Author:   L. Rossman
//
//   Routing interface file functions.
//
//   Build 5.2.0:
//   - Support added for relative file names.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Imported variables
//-----------------------------------------------------------------------------
// swmm5.c: `const double Qcf[6]` (flow units conversion factors);
// swmm5.rs does not export it, so a copy of its values is kept here.
const Qcf: [f64; 6] = [1.0, 448.831, 0.64632, 0.02832, 28.317, 2.4466];

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of iface.c.
#[derive(Default)]
pub struct IfaceState {
    IfaceFlowUnits: i32,             // flow units for routing interface file
    IfaceStep: i32,                  // interface file time step (sec)
    NumIfacePolluts: i32,            // number of pollutants in interface file
    IfacePolluts: Vec<i32>,          // indexes of interface file pollutants
    NumIfaceNodes: i32,              // number of nodes on interface file
    IfaceNodes: Vec<i32>,            // indexes of nodes on interface file
    OldIfaceValues: Vec<Vec<f64>>,   // interface flows & WQ at previous time
    NewIfaceValues: Vec<Vec<f64>>,   // interface flows & WQ at next time
    IfaceFrac: f64,                  // fraction of interface file time step
    OldIfaceDate: DateTime,          // previous date of interface values
    NewIfaceDate: DateTime,          // next date of interface values
}

//-----------------------------------------------------------------------------
//  External Functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  iface_readFileParams     (called by input_readLine)
//  iface_openRoutingFiles   (called by routing_open)
//  iface_closeRoutingFiles  (called by routing_close)
//  iface_getNumIfaceNodes   (called by addIfaceInflows in routing.c)
//  iface_getIfaceNode       (called by addIfaceInflows in routing.c)
//  iface_getIfaceFlow       (called by addIfaceInflows in routing.c)
//  iface_getIfaceQual       (called by addIfaceInflows in routing.c)
//  iface_saveOutletResults  (called by output_saveResults)

//-----------------------------------------------------------------------------
//  Local helpers standing in for C library behaviour
//-----------------------------------------------------------------------------

/// `fgets(line, MAXLINE, f)`: on end of file with nothing read C leaves the
/// buffer unchanged, so `line` keeps its previous contents.
fn fgets_line(f: &mut CFile, line: &mut Vec<u8>) {
    if let Some(l) = f.fgets(MAXLINE as usize) {
        *line = l;
    }
}

/// `fgets(buf, MAXLINE, f)` into a fixed C buffer: the bytes read plus a
/// NUL; the rest of the buffer is left as it was. On end of file with
/// nothing read the buffer is unchanged.
fn fgets_buf(f: &mut CFile, buf: &mut [u8]) {
    if let Some(l) = f.fgets(MAXLINE as usize) {
        buf[..l.len()].copy_from_slice(&l);
        buf[l.len()] = 0;
    }
}

/// glibc `strtok` on a fixed C buffer. `save` is strtok's saved pointer;
/// `first` selects `strtok(buf, delim)` over `strtok(NULL, delim)`.
/// Returns the token's start and end (the end now holds a NUL, unless the
/// token ended the string).
fn strtok(buf: &mut [u8], save: &mut usize, first: bool, delim: &[u8]) -> Option<(usize, usize)> {
    let mut s = if first { 0 } else { *save };
    // --- scan leading delimiters
    while buf[s] != 0 && delim.contains(&buf[s]) {
        s += 1;
    }
    if buf[s] == 0 {
        *save = s;
        return None;
    }
    // --- find the end of the token
    let token = s;
    while buf[s] != 0 && !delim.contains(&buf[s]) {
        s += 1;
    }
    if buf[s] == 0 {
        // this token finishes the string
        *save = s;
    } else {
        // terminate the token and make save point past it
        buf[s] = 0;
        *save = s + 1;
    }
    Some((token, s))
}

//=============================================================================

/// Called by: input.
pub fn iface_readFileParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns an error code
//  Purpose: reads interface file information from a line of input data.
//
//  Data format is:
//  USE/SAVE  FileType  FileName
//
{
    let k: i8;
    let j: i32;
    let mut fname: [u8; MAXFNAME as usize + 1] = [0; MAXFNAME as usize + 1];

    // --- determine file disposition and type
    if ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    k = findmatch(&tok[0], FileModeWords) as i8;
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[0]);
    }
    j = findmatch(&tok[1], FileTypeWords);
    if j < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }
    if ntoks < 3 {
        return 0;
    }
    sstrncpy(&mut fname, &tok[2], MAXFNAME as usize);

    // --- process file name
    match j {
        RAINFALL_FILE => {
            prj.Frain.mode = k;
            addAbsolutePath(prj, &mut fname);
            sstrncpy(&mut prj.Frain.name, &fname, MAXFNAME as usize);
        }

        RUNOFF_FILE => {
            prj.Frunoff.mode = k;
            addAbsolutePath(prj, &mut fname);
            sstrncpy(&mut prj.Frunoff.name, &fname, MAXFNAME as usize);
        }

        HOTSTART_FILE => {
            if k as i32 == USE_FILE {
                prj.Fhotstart1.mode = k;
                addAbsolutePath(prj, &mut fname);
                sstrncpy(&mut prj.Fhotstart1.name, &fname, MAXFNAME as usize);
            } else if k as i32 == SAVE_FILE {
                prj.Fhotstart2.mode = k;
                addAbsolutePath(prj, &mut fname);
                sstrncpy(&mut prj.Fhotstart2.name, &fname, MAXFNAME as usize);
            }
        }

        RDII_FILE => {
            prj.Frdii.mode = k;
            sstrncpy(&mut prj.Frdii.name, &fname, MAXFNAME as usize);
        }

        INFLOWS_FILE => {
            if k as i32 != USE_FILE {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            prj.Finflows.mode = k;
            addAbsolutePath(prj, &mut fname);
            sstrncpy(&mut prj.Finflows.name, &fname, MAXFNAME as usize);
        }

        OUTFLOWS_FILE => {
            if k as i32 != SAVE_FILE {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            prj.Foutflows.mode = k;
            addAbsolutePath(prj, &mut fname);
            sstrncpy(&mut prj.Foutflows.name, &fname, MAXFNAME as usize);
        }

        _ => {}
    }
    0
}

//=============================================================================

/// Called by: routing.
pub fn iface_openRoutingFiles(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: opens routing interface files.
//
{
    // --- initialize shared variables
    prj.iface.NumIfacePolluts = 0;
    prj.iface.IfacePolluts = Vec::new();
    prj.iface.NumIfaceNodes = 0;
    prj.iface.IfaceNodes = Vec::new();
    prj.iface.OldIfaceValues = Vec::new();
    prj.iface.NewIfaceValues = Vec::new();

    // --- check that inflows & outflows files are not the same
    if prj.Foutflows.mode as i32 != NO_FILE && prj.Finflows.mode as i32 != NO_FILE {
        if strcomp(&prj.Foutflows.name, &prj.Finflows.name) != 0 {
            report_writeErrorMsg(prj, ERR_ROUTING_FILE_NAMES, b"");
            return;
        }
    }

    // --- open the file for reading or writing
    if prj.Foutflows.mode as i32 == SAVE_FILE {
        openFileForOutput(prj);
    }
    if prj.Finflows.mode as i32 == USE_FILE {
        openFileForInput(prj);
    }
}

//=============================================================================

/// Called by: routing.
pub fn iface_closeRoutingFiles(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: closes routing interface files.
//
{
    prj.iface.IfacePolluts = Vec::new();
    prj.iface.IfaceNodes = Vec::new();
    if !prj.iface.OldIfaceValues.is_empty() {
        let a = std::mem::take(&mut prj.iface.OldIfaceValues);
        project_freeMatrix(a);
    }
    if !prj.iface.NewIfaceValues.is_empty() {
        let a = std::mem::take(&mut prj.iface.NewIfaceValues);
        project_freeMatrix(a);
    }
    // (C fcloses without clearing the FILE pointers)
    if prj.Finflows.file.is_some() {
        prj.Finflows.file = None;
    }
    if prj.Foutflows.file.is_some() {
        prj.Foutflows.file = None;
    }
}

//=============================================================================

/// Called by: routing.
pub fn iface_getNumIfaceNodes(prj: &mut Project, currentDate: DateTime) -> i32
//
//  Input:   currentDate = current date/time
//  Output:  returns number of interface nodes if data exists or
//           0 otherwise
//  Purpose: reads inflow data from interface file at current date.
//
{
    // --- return 0 if file begins after current date
    if prj.iface.OldIfaceDate > currentDate {
        return 0;
    }

    // --- keep updating new interface values until current date bracketed
    while prj.iface.NewIfaceDate < currentDate && prj.iface.NewIfaceDate != NO_DATE as f64 {
        setOldIfaceValues(prj);
        readNewIfaceValues(prj);
    }

    // --- return 0 if no data available
    if prj.iface.NewIfaceDate == NO_DATE as f64 {
        return 0;
    }

    // --- find fraction current date is bewteen old & new interface dates
    let st = &mut prj.iface;
    st.IfaceFrac = (currentDate - st.OldIfaceDate) / (st.NewIfaceDate - st.OldIfaceDate);
    st.IfaceFrac = MAX(0.0, st.IfaceFrac);
    st.IfaceFrac = MIN(st.IfaceFrac, 1.0);

    // --- return number of interface nodes
    st.NumIfaceNodes
}

//=============================================================================

/// Called by: routing.
pub fn iface_getIfaceNode(prj: &Project, index: i32) -> i32
//
//  Input:   index = interface file node index
//  Output:  returns project node index
//  Purpose: finds index of project node associated with interface node index
//
{
    if index >= 0 && index < prj.iface.NumIfaceNodes {
        prj.iface.IfaceNodes[index as usize]
    } else {
        -1
    }
}

//=============================================================================

/// Called by: routing.
pub fn iface_getIfaceFlow(prj: &Project, index: i32) -> f64
//
//  Input:   index = interface file node index
//  Output:  returns inflow to node
//  Purpose: finds interface flow for particular node index.
//
{
    let q1: f64;
    let q2: f64;
    let st = &prj.iface;

    if index >= 0 && index < st.NumIfaceNodes {
        // --- interpolate flow between old and new values
        q1 = st.OldIfaceValues[index as usize][0];
        q2 = st.NewIfaceValues[index as usize][0];
        (1.0 - st.IfaceFrac) * q1 + st.IfaceFrac * q2
    } else {
        0.0
    }
}

//=============================================================================

/// Called by: routing.
pub fn iface_getIfaceQual(prj: &Project, index: i32, pollut: i32) -> f64
//
//  Input:   index = index of node on interface file
//           pollut = index of pollutant on interface file
//  Output:  returns inflow pollutant concentration
//  Purpose: finds interface concentration for particular node index & pollutant.
//
{
    let j: i32;
    let c1: f64;
    let c2: f64;
    let st = &prj.iface;

    if index >= 0 && index < st.NumIfaceNodes {
        // --- find index of pollut on interface file
        j = st.IfacePolluts[pollut as usize];
        if j < 0 {
            return 0.0;
        }

        // --- interpolate flow between old and new values
        //     (remember that 1st col. of values matrix is for flow)
        c1 = st.OldIfaceValues[index as usize][(j + 1) as usize];
        c2 = st.NewIfaceValues[index as usize][(j + 1) as usize];
        (1.0 - st.IfaceFrac) * c1 + st.IfaceFrac * c2
    } else {
        0.0
    }
}

//=============================================================================

/// Called by: output.
pub fn iface_saveOutletResults(prj: &mut Project, reportDate: DateTime)
//
//  Input:   reportDate = reporting date/time
//           file = ptr. to interface file
//  Output:  none
//  Purpose: saves system outflows to routing interface file.
//
//  (C passes the FILE*; both callers pass Foutflows.file, used here.)
//
{
    let (mut yr, mut mon, mut day, mut hr, mut min, mut sec) = (0i32, 0i32, 0i32, 0i32, 0i32, 0i32);
    datetime_decodeDate(reportDate, &mut yr, &mut mon, &mut day);
    datetime_decodeTime(reportDate, &mut hr, &mut min, &mut sec);
    // C: snprintf(theDate, 26, ...) keeps at most 25 characters
    let mut theDate = cformat!(b" %04d %02d  %02d  %02d  %02d  %02d ", yr, mon, day, hr, min, sec);
    theDate.truncate(25);
    let ucfFlow = UCF(prj, FLOW);
    for i in 0..prj.Nobjects[NODE as usize] {
        // --- check that node is an outlet node
        if isOutletNode(prj, i) == 0 {
            continue;
        }

        // --- write node ID, date, flow, and quality to file
        let iu = i as usize;
        let mut out: Vec<u8> = Vec::new();
        out.extend(cformat!(b"\n%-16s", prj.Node[iu].ID));
        out.extend(cformat!(b"%s", theDate));
        out.extend(cformat!(b" %-10f", prj.Node[iu].inflow * ucfFlow));
        for p in 0..prj.Nobjects[POLLUT as usize] as usize {
            out.extend(cformat!(b" %-10f", prj.Node[iu].newQual[p]));
        }
        prj.Foutflows.write_bytes(&out);
    }
}

//=============================================================================

fn openFileForOutput(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: opens a routing interface file for writing.
//
{
    let mut n: i32;

    // --- open the routing file for writing text
    prj.Foutflows.file = CFile::open(&prj.Foutflows.name, "wt");
    if prj.Foutflows.file.is_none() {
        let name = prj.Foutflows.name;
        report_writeErrorMsg(prj, ERR_ROUTING_FILE_OPEN, &name);
        return;
    }

    // --- write title & reporting time step to file
    cprintf!(prj.Foutflows, b"SWMM5 Interface File");
    cprintf!(prj.Foutflows, b"\n%s", prj.Title[0]);
    cprintf!(prj.Foutflows, b"\n%-4d - reporting time step in sec", prj.ReportStep);

    // --- write number & names of each constituent (including flow) to file
    cprintf!(prj.Foutflows, b"\n%-4d - number of constituents as listed below:", prj.Nobjects[POLLUT as usize] + 1);
    cprintf!(prj.Foutflows, b"\nFLOW %s", FlowUnitWords[prj.FlowUnits as usize]);
    for i in 0..prj.Nobjects[POLLUT as usize] as usize {
        cprintf!(prj.Foutflows, b"\n%s %s", prj.Pollut[i].ID, QualUnitsWords[prj.Pollut[i].units as usize]);
    }

    // --- count number of outlet nodes
    n = 0;
    for i in 0..prj.Nobjects[NODE as usize] {
        if isOutletNode(prj, i) != 0 {
            n += 1;
        }
    }

    // --- write number and names of outlet nodes to file
    cprintf!(prj.Foutflows, b"\n%-4d - number of nodes as listed below:", n);
    for i in 0..prj.Nobjects[NODE as usize] {
        if isOutletNode(prj, i) != 0 {
            cprintf!(prj.Foutflows, b"\n%s", prj.Node[i as usize].ID);
        }
    }

    // --- write column headings
    cprintf!(prj.Foutflows, b"\nNode             Year Mon Day Hr  Min Sec FLOW      ");
    for i in 0..prj.Nobjects[POLLUT as usize] as usize {
        cprintf!(prj.Foutflows, b" %-10s", prj.Pollut[i].ID);
    }

    // --- if reporting starts immediately, save initial outlet values
    if prj.ReportStart == prj.StartDateTime {
        let rs = prj.ReportStart;
        iface_saveOutletResults(prj, rs);
    }
}

//=============================================================================

fn openFileForInput(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: opens a routing interface file for reading.
//
{
    let mut err: i32; // error code
    // C: line and s are uninitialized locals; an fgets at end of file leaves
    // them so. They are empty here (only reachable with a truncated file).
    let mut line: Vec<u8> = Vec::new(); // line from Routing interface file
    let mut s: Vec<u8> = Vec::new(); // general string variable

    // --- open the routing interface file for reading text
    prj.Finflows.file = CFile::open(&prj.Finflows.name, "rt");
    if prj.Finflows.file.is_none() {
        let name = prj.Finflows.name;
        report_writeErrorMsg(prj, ERR_ROUTING_FILE_OPEN, &name);
        return;
    }

    // --- check for correct file type
    fgets_line(prj.Finflows.f(), &mut line);
    if cio::sscanf(&line, b"%s", &mut [Scan::Str(&mut s)]) == 0 || strcomp(&s, b"SWMM5") == 0 {
        let name = prj.Finflows.name;
        report_writeErrorMsg(prj, ERR_ROUTING_FILE_FORMAT, &name);
        return;
    }

    // --- skip title line
    fgets_line(prj.Finflows.f(), &mut line);

    // --- read reporting time step (sec)
    prj.iface.IfaceStep = 0;
    fgets_line(prj.Finflows.f(), &mut line);
    if cio::sscanf(&line, b"%d", &mut [Scan::Int(&mut prj.iface.IfaceStep)]) == 0 || prj.iface.IfaceStep <= 0 {
        let name = prj.Finflows.name;
        report_writeErrorMsg(prj, ERR_ROUTING_FILE_FORMAT, &name);
        return;
    }

    // --- match constituents in file with those in project
    err = getIfaceFilePolluts(prj);
    if err > 0 {
        let name = prj.Finflows.name;
        report_writeErrorMsg(prj, err, &name);
        return;
    }

    // --- match nodes in file with those in project
    err = getIfaceFileNodes(prj);
    if err > 0 {
        let name = prj.Finflows.name;
        report_writeErrorMsg(prj, err, &name);
        return;
    }

    // --- create matrices for old & new interface flows & WQ values
    prj.iface.OldIfaceValues = project_createMatrix(prj.iface.NumIfaceNodes, 1 + prj.iface.NumIfacePolluts);
    prj.iface.NewIfaceValues = project_createMatrix(prj.iface.NumIfaceNodes, 1 + prj.iface.NumIfacePolluts);
    if prj.iface.OldIfaceValues.is_empty() || prj.iface.NewIfaceValues.is_empty() {
        report_writeErrorMsg(prj, ERR_MEMORY, b"");
        return;
    }

    // --- read in new interface flows & WQ values
    readNewIfaceValues(prj);
    prj.iface.OldIfaceDate = prj.iface.NewIfaceDate;
}

//=============================================================================

fn getIfaceFilePolluts(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: reads names of pollutants saved on the inflows interface file.
//
{
    let mut j: i32;
    // C: uninitialized locals (see openFileForInput)
    let mut line: Vec<u8> = Vec::new(); // line from inflows interface file
    let mut s1: Vec<u8> = Vec::new(); // general string variable
    let mut s2: Vec<u8> = Vec::new();

    // --- read number of pollutants (minus FLOW)
    fgets_line(prj.Finflows.f(), &mut line);
    prj.iface.NumIfacePolluts = -1;
    if cio::sscanf(&line, b"%d", &mut [Scan::Int(&mut prj.iface.NumIfacePolluts)]) != 0 {
        prj.iface.NumIfacePolluts -= 1;
    }
    if prj.iface.NumIfacePolluts < 0 {
        return ERR_ROUTING_FILE_FORMAT;
    }

    // --- read flow units
    fgets_line(prj.Finflows.f(), &mut line);
    if cio::sscanf(&line, b"%s %s", &mut [Scan::Str(&mut s1), Scan::Str(&mut s2)]) < 2 {
        return ERR_ROUTING_FILE_FORMAT;
    }
    if strcomp(&s1, b"FLOW") == 0 {
        return ERR_ROUTING_FILE_FORMAT;
    }
    prj.iface.IfaceFlowUnits = findmatch(&s2, FlowUnitWords);
    if prj.iface.IfaceFlowUnits < 0 {
        return ERR_ROUTING_FILE_FORMAT;
    }

    // --- allocate memory for pollutant index array
    if prj.Nobjects[POLLUT as usize] > 0 {
        let n = prj.Nobjects[POLLUT as usize] as usize;
        prj.iface.IfacePolluts = vec![0; n];
        for i in 0..n {
            prj.iface.IfacePolluts[i] = -1;
        }
    }

    // --- read pollutant names & units
    if prj.iface.NumIfacePolluts > 0 && prj.Nobjects[POLLUT as usize] > 0 {
        // --- check each pollutant name on file with project's pollutants
        for i in 0..prj.iface.NumIfacePolluts {
            if prj.Finflows.f().feof() {
                return ERR_ROUTING_FILE_FORMAT;
            }
            fgets_line(prj.Finflows.f(), &mut line);
            if cio::sscanf(&line, b"%s %s", &mut [Scan::Str(&mut s1), Scan::Str(&mut s2)]) < 2 {
                return ERR_ROUTING_FILE_FORMAT;
            }
            j = project_findObject(prj, POLLUT, &s1);
            if j < 0 {
                continue;
            }
            if strcomp(&s2, QualUnitsWords[prj.Pollut[j as usize].units as usize]) == 0 {
                return ERR_ROUTING_FILE_NOMATCH;
            }
            prj.iface.IfacePolluts[j as usize] = i;
        }
    }
    0
}

//=============================================================================

fn getIfaceFileNodes(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: reads names of nodes contained on inflows interface file.
//
{
    // C: uninitialized locals (see openFileForInput)
    let mut line: Vec<u8> = Vec::new(); // line from inflows interface file
    let mut s: Vec<u8> = Vec::new(); // general string variable

    // --- read number of interface nodes
    fgets_line(prj.Finflows.f(), &mut line);
    if cio::sscanf(&line, b"%d", &mut [Scan::Int(&mut prj.iface.NumIfaceNodes)]) == 0 || prj.iface.NumIfaceNodes <= 0 {
        return ERR_ROUTING_FILE_FORMAT;
    }

    // --- allocate memory for interface nodes index array
    let n = prj.iface.NumIfaceNodes as usize;
    prj.iface.IfaceNodes = vec![0; n];

    // --- read names of interface nodes from file & save their indexes
    for i in 0..n {
        if prj.Finflows.f().feof() {
            return ERR_ROUTING_FILE_FORMAT;
        }
        prj.iface.IfaceNodes[i] = 0;
        fgets_line(prj.Finflows.f(), &mut line);
        if cio::sscanf(&line, b"%s", &mut [Scan::Str(&mut s)]) == 0 {
            return ERR_ROUTING_FILE_FORMAT;
        }
        prj.iface.IfaceNodes[i] = project_findObject(prj, NODE, &s);
    }

    // --- skip over column headings line
    if prj.Finflows.f().feof() {
        return ERR_ROUTING_FILE_FORMAT;
    }
    fgets_line(prj.Finflows.f(), &mut line);
    0
}

//=============================================================================

fn readNewIfaceValues(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: reads data from inflows interface file for next date.
//
{
    let mut yr: i32 = 0;
    let mut mon: i32 = 0;
    let mut day: i32 = 0;
    let mut hr: i32 = 0;
    let mut min: i32 = 0;
    let mut sec: i32 = 0; // year, month, day, hour, minute, second
    // C: `char line[MAXLINE+1]`, an uninitialized local. When fgets meets
    // end of file without feof already set (a file whose last line ends in
    // a newline), C keeps the buffer as it was: the previous node's line
    // with strtok's NULs (kept here too), or at the first node of a call,
    // stale stack bytes (zeros here). In every such case strtok runs into a
    // NUL by its second call, the function returns with NewIfaceDate =
    // NO_DATE, and feof is now set, so both engines read no further
    // interface inflows.
    let mut line: [u8; MAXLINE as usize + 1] = [0; MAXLINE as usize + 1]; // line from interface file
    let mut save: usize = 0; // strtok's saved position

    // --- read a line for each interface node
    prj.iface.NewIfaceDate = NO_DATE as f64;
    for i in 0..prj.iface.NumIfaceNodes as usize {
        if prj.Finflows.f().feof() {
            return;
        }
        fgets_buf(prj.Finflows.f(), &mut line);

        // --- parse date & time from line
        macro_rules! next_tok {
            ($first:expr) => {
                match strtok(&mut line, &mut save, $first, SEPSTR) {
                    None => return,
                    Some((a, b)) => line[a..b].to_vec(),
                }
            };
        }
        let _ = next_tok!(true);
        let s = next_tok!(false);
        yr = cio::atoi(&s);
        let s = next_tok!(false);
        mon = cio::atoi(&s);
        let s = next_tok!(false);
        day = cio::atoi(&s);
        let s = next_tok!(false);
        hr = cio::atoi(&s);
        let s = next_tok!(false);
        min = cio::atoi(&s);
        let s = next_tok!(false);
        sec = cio::atoi(&s);

        // --- parse flow value
        let s = next_tok!(false);
        prj.iface.NewIfaceValues[i][0] = cio::atof(&s) / Qcf[prj.iface.IfaceFlowUnits as usize];

        // --- parse pollutant values
        for j in 1..=prj.iface.NumIfacePolluts as usize {
            let s = next_tok!(false);
            prj.iface.NewIfaceValues[i][j] = cio::atof(&s);
        }
    }

    // --- encode date & time values
    prj.iface.NewIfaceDate = datetime_encodeDate(yr, mon, day) + datetime_encodeTime(hr, min, sec as f64);
}

//=============================================================================

fn setOldIfaceValues(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: replaces old values read from routing interface file with new ones.
//
{
    let st = &mut prj.iface;
    st.OldIfaceDate = st.NewIfaceDate;
    for i in 0..st.NumIfaceNodes as usize {
        for j in 0..(st.NumIfacePolluts + 1) as usize {
            st.OldIfaceValues[i][j] = st.NewIfaceValues[i][j];
        }
    }
}

//=============================================================================

fn isOutletNode(prj: &Project, i: i32) -> i32
//
//  Input:   i = node index
//  Output:  returns 1 if node is an outlet, 0 if not.
//  Purpose: determines if a node is an outlet point or not.
//
{
    // --- for DW routing only outfalls are outlets
    if prj.RouteModel == DW {
        (prj.Node[i as usize].type_ == OUTFALL) as i32
    }
    // --- otherwise outlets are nodes with no outflow links (degree is 0)
    else {
        (prj.Node[i as usize].degree == 0) as i32
    }
}
