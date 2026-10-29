//! output.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   output.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     06/01/22  (Build 5.2.1)
//   Author:   L. Rossman
//
//   Binary output file access functions.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - Possible divide by zero for reported system wide variables avoided.
//   - Updating of maximum node depth at reporting times added.
//   Build 5.1.010:
//   - Potentional ET added to list of system-wide variables saved to file.
//   Build 5.1.013:
//   - Names NsubcatchVars, NnodeVars & NlinkVars replaced with
//     NumSubcatchVars, NumNodeVars & NumLinkVars
//   - Support added for saving average node & link routing results to
//     binary file in each reporting period.
//   Build 5.1.014:
//   - Incorrect loop limit fixed in function output_saveAvgResults.
//   Build 5.2.0:
//   - Changed how time step averaged flow is computed.
//   - Object's rptFlag changed to record its index in output file.
//   - Large file support added.
//   Build5.2.1:
//   - Corrects the definition of F_OFF for non-Microsoft C/C++ compilers.
//-----------------------------------------------------------------------------
//
// INT4 is i32, REAL4 f32, REAL8 f64, F_OFF (off_t on Linux) i64. Every value
// is written little-endian, as fwrite writes it on x86-64. The local save
// functions write to Fout, which is what C passes them as `file`.

use crate::prelude::*;

// enum InputDataType
const INPUT_TYPE_CODE: i32 = 0;
const INPUT_AREA: i32 = 1;
const INPUT_INVERT: i32 = 2;
const INPUT_MAX_DEPTH: i32 = 3;
const INPUT_OFFSET: i32 = 4;
const INPUT_LENGTH: i32 = 5;

#[derive(Clone, Default)]
struct TAvgResults {
    xAvg: Vec<f32>,
}

/// The file-scope variables of output.c.
pub struct OutputState {
    // --- Exportable variables (shared with report.c)
    pub SubcatchResults: Vec<f32>,
    pub NodeResults: Vec<f32>,
    pub LinkResults: Vec<f32>,

    // --- Shared variables
    /// starting file position of ID names
    IDStartPos: i64,
    /// starting file position of input data
    InputStartPos: i64,
    /// starting file position of output data
    OutputStartPos: i64,
    /// bytes saved per simulation time period
    BytesPerPeriod: i64,
    /// number of subcatchment output variables
    NumSubcatchVars: i32,
    /// number of node output variables
    NumNodeVars: i32,
    /// number of link output variables
    NumLinkVars: i32,
    /// number of subcatchments reported on
    NumSubcatch: i32,
    /// number of nodes reported on
    NumNodes: i32,
    /// number of links reported on
    NumLinks: i32,
    /// number of pollutants reported on
    NumPolluts: i32,
    /// values of system output vars.
    SysResults: [f32; MAX_SYS_RESULTS as usize],
    /// (C: NULL until allocated; here empty)
    AvgLinkResults: Vec<TAvgResults>,
    AvgNodeResults: Vec<TAvgResults>,
    Nsteps: i32,
}
impl Default for OutputState {
    fn default() -> Self {
        OutputState {
            SubcatchResults: Vec::new(),
            NodeResults: Vec::new(),
            LinkResults: Vec::new(),
            IDStartPos: 0,
            InputStartPos: 0,
            OutputStartPos: 0,
            BytesPerPeriod: 0,
            NumSubcatchVars: 0,
            NumNodeVars: 0,
            NumLinkVars: 0,
            NumSubcatch: 0,
            NumNodes: 0,
            NumLinks: 0,
            NumPolluts: 0,
            SysResults: [0.0; MAX_SYS_RESULTS as usize],
            AvgLinkResults: Vec::new(),
            AvgNodeResults: Vec::new(),
            Nsteps: 0,
        }
    }
}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  output_open                   (called by swmm_start in swmm5.c)
//  output_end                    (called by swmm_end in swmm5.c)
//  output_close                  (called by swmm_close in swmm5.c)
//  output_updateAvgResults       (called by swmm_step in swmm5.c)
//  output_saveResults            (called by swmm_step in swmm5.c)
//  output_checkFileSize          (called by swmm_report)
//  output_readDateTime           (called by routines in report.c)
//  output_readSubcatchResults    (called by report_Subcatchments)
//  output_readNodeResults        (called by report_Nodes)
//  output_readLinkResults        (called by report_Links)

/// `fwrite(&k, sizeof(INT4), 1, Fout.file)`
#[inline]
fn write_int4(prj: &mut Project, k: i32) {
    prj.Fout.write_bytes(&k.to_le_bytes());
}

/// `fwrite(x, sizeof(REAL4), n, Fout.file)`
#[inline]
fn write_real4s(f: &mut TFile, x: &[f32], n: usize) {
    let mut b: Vec<u8> = Vec::with_capacity(4 * n);
    for v in &x[..n] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    f.write_bytes(&b);
}

/// `fread(x, sizeof(REAL4), n, Fout.file)`: a short read leaves the bytes
/// it did not reach as they were, as fread does.
fn read_real4s(f: &mut TFile, x: &mut [f32], n: usize) {
    let mut b: Vec<u8> = Vec::with_capacity(4 * n);
    for v in &x[..n] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    let mut buf = vec![0u8; 4 * n];
    let got = f.f().read_bytes(&mut buf);
    b[..got].copy_from_slice(&buf[..got]);
    for i in 0..n {
        x[i] = f32::from_le_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]]);
    }
}

//=============================================================================

pub fn output_open(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns an error code
//  Purpose: writes basic project data to binary output file.
//
{
    let mut j: i32;
    let mut m: i32;
    let mut k: i32;
    let mut x: f32;
    let mut z: f64;
    let numResults: i64;

    // --- open binary output file
    output_openOutFile(prj);
    if prj.ErrorCode != 0 {
        return prj.ErrorCode;
    }

    // --- ignore pollutants if no water quality analsis performed
    if prj.IgnoreQuality != 0 {
        prj.output.NumPolluts = 0;
    } else {
        prj.output.NumPolluts = prj.Nobjects[POLLUT as usize];
    }
    let NumPolluts = prj.output.NumPolluts;

    // --- subcatchment results consist of Rainfall, Snowdepth, Evap,
    //     Infil, Runoff, GW Flow, GW Elev, GW Sat, and Washoff
    prj.output.NumSubcatchVars = MAX_SUBCATCH_RESULTS - 1 + NumPolluts;

    // --- node results consist of Depth, Head, Volume, Lateral Inflow,
    //     Total Inflow, Overflow and Quality
    prj.output.NumNodeVars = MAX_NODE_RESULTS - 1 + NumPolluts;

    // --- link results consist of Depth, Flow, Velocity, Volume,
    //     Capacity and Quality
    prj.output.NumLinkVars = MAX_LINK_RESULTS - 1 + NumPolluts;

    // --- get number of objects reported on
    prj.output.NumSubcatch = 0;
    prj.output.NumNodes = 0;
    prj.output.NumLinks = 0;
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        if prj.Subcatch[j as usize].rptFlag != 0 {
            prj.output.NumSubcatch += 1;
        }
        j += 1;
    }
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        if prj.Node[j as usize].rptFlag != 0 {
            prj.output.NumNodes += 1;
        }
        j += 1;
    }
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        if prj.Link[j as usize].rptFlag != 0 {
            prj.output.NumLinks += 1;
        }
        j += 1;
    }

    // --- find size of results saved in each time period
    {
        let o = &prj.output;
        numResults = (o.NumSubcatch as i64 * o.NumSubcatchVars as i64)
            .wrapping_add(o.NumNodes as i64 * o.NumNodeVars as i64)
            .wrapping_add(o.NumLinks as i64 * o.NumLinkVars as i64)
            .wrapping_add(MAX_SYS_RESULTS as i64);
    }
    prj.output.BytesPerPeriod = 8i64.wrapping_add(numResults.wrapping_mul(4));
    prj.Nperiods = 0;

    prj.output.SubcatchResults = Vec::new();
    prj.output.NodeResults = Vec::new();
    prj.output.LinkResults = Vec::new();
    let nz = |n: i32| if n > 0 { n as usize } else { 0 };
    prj.output.SubcatchResults = vec![0.0f32; nz(prj.output.NumSubcatchVars)];
    prj.output.NodeResults = vec![0.0f32; nz(prj.output.NumNodeVars)];
    prj.output.LinkResults = vec![0.0f32; nz(prj.output.NumLinkVars)];

    // --- allocate memory to store average node & link results per period
    prj.output.AvgNodeResults = Vec::new();
    prj.output.AvgLinkResults = Vec::new();
    if prj.RptFlags.averages != 0 && output_openAvgResults(prj) == 0 {
        report_writeErrorMsg(prj, ERR_MEMORY, b"");
        return prj.ErrorCode;
    }

    prj.Fout.f().fseek_set(0);
    k = MAGICNUMBER;
    write_int4(prj, k); // Magic number
    k = VERSION;
    write_int4(prj, k); // Version number
    k = prj.FlowUnits;
    write_int4(prj, k); // Flow units
    k = prj.output.NumSubcatch;
    write_int4(prj, k); // # subcatchments
    k = prj.output.NumNodes;
    write_int4(prj, k); // # nodes
    k = prj.output.NumLinks;
    write_int4(prj, k); // # links
    k = NumPolluts;
    write_int4(prj, k); // # pollutants

    // --- save ID names of subcatchments, nodes, links, & pollutants
    prj.output.IDStartPos = prj.Fout.f().ftell();
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        if prj.Subcatch[j as usize].rptFlag != 0 {
            output_saveID(&prj.Subcatch[j as usize].ID, &mut prj.Fout);
        }
        j += 1;
    }
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        if prj.Node[j as usize].rptFlag != 0 {
            output_saveID(&prj.Node[j as usize].ID, &mut prj.Fout);
        }
        j += 1;
    }
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        if prj.Link[j as usize].rptFlag != 0 {
            output_saveID(&prj.Link[j as usize].ID, &mut prj.Fout);
        }
        j += 1;
    }
    j = 0;
    while j < NumPolluts {
        output_saveID(&prj.Pollut[j as usize].ID, &mut prj.Fout);
        j += 1;
    }

    // --- save codes of pollutant concentration units
    j = 0;
    while j < NumPolluts {
        k = prj.Pollut[j as usize].units;
        write_int4(prj, k);
        j += 1;
    }

    prj.output.InputStartPos = prj.Fout.f().ftell();

    // --- save subcatchment area
    k = 1;
    write_int4(prj, k);
    k = INPUT_AREA;
    write_int4(prj, k);
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        if prj.Subcatch[j as usize].rptFlag == 0 {
            j += 1;
            continue;
        }
        prj.output.SubcatchResults[0] = (prj.Subcatch[j as usize].area * UCF(prj, LANDAREA)) as f32;
        let v = prj.output.SubcatchResults[0];
        prj.Fout.write_bytes(&v.to_le_bytes());
        j += 1;
    }

    // --- save node type, invert, & max. depth
    k = 3;
    write_int4(prj, k);
    k = INPUT_TYPE_CODE;
    write_int4(prj, k);
    k = INPUT_INVERT;
    write_int4(prj, k);
    k = INPUT_MAX_DEPTH;
    write_int4(prj, k);
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        if prj.Node[j as usize].rptFlag == 0 {
            j += 1;
            continue;
        }
        k = prj.Node[j as usize].type_;
        prj.output.NodeResults[0] = (prj.Node[j as usize].invertElev * UCF(prj, LENGTH)) as f32;
        prj.output.NodeResults[1] = (prj.Node[j as usize].fullDepth * UCF(prj, LENGTH)) as f32;
        write_int4(prj, k);
        write_real4s(&mut prj.Fout, &prj.output.NodeResults, 2);
        j += 1;
    }

    // --- save link type, offsets, max. depth, & length
    k = 5;
    write_int4(prj, k);
    k = INPUT_TYPE_CODE;
    write_int4(prj, k);
    k = INPUT_OFFSET;
    write_int4(prj, k);
    k = INPUT_OFFSET;
    write_int4(prj, k);
    k = INPUT_MAX_DEPTH;
    write_int4(prj, k);
    k = INPUT_LENGTH;
    write_int4(prj, k);

    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        if prj.Link[j as usize].rptFlag == 0 {
            j += 1;
            continue;
        }
        k = prj.Link[j as usize].type_;
        if k == PUMP {
            m = 0;
            while m < 4 {
                prj.output.LinkResults[m as usize] = 0.0f32;
                m += 1;
            }
        } else {
            prj.output.LinkResults[0] = (prj.Link[j as usize].offset1 * UCF(prj, LENGTH)) as f32;
            prj.output.LinkResults[1] = (prj.Link[j as usize].offset2 * UCF(prj, LENGTH)) as f32;
            if prj.Link[j as usize].direction < 0 {
                x = prj.output.LinkResults[0];
                prj.output.LinkResults[0] = prj.output.LinkResults[1];
                prj.output.LinkResults[1] = x;
            }
            if k == OUTLET {
                prj.output.LinkResults[2] = 0.0f32;
            } else {
                prj.output.LinkResults[2] = (prj.Link[j as usize].xsect.yFull * UCF(prj, LENGTH)) as f32;
            }
            if k == CONDUIT {
                m = prj.Link[j as usize].subIndex;
                prj.output.LinkResults[3] = (prj.Conduit[m as usize].length * UCF(prj, LENGTH)) as f32;
            } else {
                prj.output.LinkResults[3] = 0.0f32;
            }
        }
        write_int4(prj, k);
        write_real4s(&mut prj.Fout, &prj.output.LinkResults, 4);
        j += 1;
    }

    // --- save number & codes of subcatchment result variables
    k = prj.output.NumSubcatchVars;
    write_int4(prj, k);
    k = SUBCATCH_RAINFALL;
    write_int4(prj, k);
    k = SUBCATCH_SNOWDEPTH;
    write_int4(prj, k);
    k = SUBCATCH_EVAP;
    write_int4(prj, k);
    k = SUBCATCH_INFIL;
    write_int4(prj, k);
    k = SUBCATCH_RUNOFF;
    write_int4(prj, k);
    k = SUBCATCH_GW_FLOW;
    write_int4(prj, k);
    k = SUBCATCH_GW_ELEV;
    write_int4(prj, k);
    k = SUBCATCH_SOIL_MOIST;
    write_int4(prj, k);

    j = 0;
    while j < NumPolluts {
        k = SUBCATCH_WASHOFF + j;
        write_int4(prj, k);
        j += 1;
    }

    // --- save number & codes of node result variables
    k = prj.output.NumNodeVars;
    write_int4(prj, k);
    k = NODE_DEPTH;
    write_int4(prj, k);
    k = NODE_HEAD;
    write_int4(prj, k);
    k = NODE_VOLUME;
    write_int4(prj, k);
    k = NODE_LATFLOW;
    write_int4(prj, k);
    k = NODE_INFLOW;
    write_int4(prj, k);
    k = NODE_OVERFLOW;
    write_int4(prj, k);
    j = 0;
    while j < NumPolluts {
        k = NODE_QUAL + j;
        write_int4(prj, k);
        j += 1;
    }

    // --- save number & codes of link result variables
    k = prj.output.NumLinkVars;
    write_int4(prj, k);
    k = LINK_FLOW;
    write_int4(prj, k);
    k = LINK_DEPTH;
    write_int4(prj, k);
    k = LINK_VELOCITY;
    write_int4(prj, k);
    k = LINK_VOLUME;
    write_int4(prj, k);
    k = LINK_CAPACITY;
    write_int4(prj, k);
    j = 0;
    while j < NumPolluts {
        k = LINK_QUAL + j;
        write_int4(prj, k);
        j += 1;
    }

    // --- save number & codes of system result variables
    k = MAX_SYS_RESULTS;
    write_int4(prj, k);
    k = 0;
    while k < MAX_SYS_RESULTS {
        write_int4(prj, k);
        k += 1;
    }

    // --- save starting report date & report step
    //     (if reporting start date > simulation start date then
    //      make saved starting report date one reporting period
    //      prior to the date of the first reported result)
    z = prj.ReportStep as f64 / 86400.0;
    if prj.StartDateTime + z > prj.ReportStart {
        z = prj.StartDateTime;
    } else {
        z = ((prj.ReportStart - prj.StartDateTime) / z).floor() - 1.0;
        z = prj.StartDateTime + z * prj.ReportStep as f64 / 86400.0;
    }
    prj.Fout.write_bytes(&z.to_le_bytes());
    k = prj.ReportStep;
    // (C checks that fwrite wrote the item; a buffered write cannot
    //  fail here, as it does not in C short of a stdio error)
    write_int4(prj, k);
    prj.output.OutputStartPos = prj.Fout.f().ftell();
    prj.ErrorCode
}

//=============================================================================
/*  DEPRECATED
void  output_checkFileSize()
//
//  Input:   none
//  Output:  none
//  Purpose: checks if the size of the binary output file will be too big
//           to access using a file pointer variable for a 32-bit compile.
//
{
    if ( RptFlags.subcatchments != NONE ||
         RptFlags.nodes != NONE ||
         RptFlags.links != NONE )
    {
        if (sizeof(void*) == 4 &&
           (double)OutputStartPos + (double)BytesPerPeriod * TotalDuration
             / 1000.0 / (double)ReportStep >= (double)MAXFILESIZE )
        {
            report_writeErrorMsg(ERR_OUT_SIZE, "");
        }
    }
}
*/

//=============================================================================

fn output_openOutFile(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: opens a project's binary output file.
//
{
    // --- close output file if already opened
    if prj.Fout.file.is_some() {
        prj.Fout.file = None;
    }
    // --- else if file name supplied then set file mode to SAVE
    else if cio::strlen(&prj.Fout.name) != 0 {
        prj.Fout.mode = SAVE_FILE as i8;
    }
    // --- otherwise set file mode to SCRATCH & generate a name
    else {
        prj.Fout.mode = SCRATCH_FILE as i8;
        let mut name = prj.Fout.name;
        getTempFileName(prj, &mut name);
        prj.Fout.name = name;
    }

    // --- try to open the file
    prj.Fout.file = CFile::open(&prj.Fout.name, "w+b");
    if prj.Fout.file.is_none() {
        writecon(prj, FMT14);
        prj.ErrorCode = ERR_OUT_FILE;
    }
}

//=============================================================================

pub fn output_saveResults(prj: &mut Project, reportTime: f64)
//
//  Input:   reportTime = elapsed simulation time (millisec)
//  Output:  none
//  Purpose: writes computed results for current report time to binary file.
//
{
    let mut i: usize;
    let reportDate: DateTime = getDateTime(prj, reportTime);
    let date: f64;

    // --- initialize system-wide results
    if reportDate < prj.ReportStart {
        return;
    }
    i = 0;
    while i < MAX_SYS_RESULTS as usize {
        prj.output.SysResults[i] = 0.0f32;
        i += 1;
    }

    // --- save date corresponding to this elapsed reporting time
    date = reportDate;
    prj.Fout.write_bytes(&date.to_le_bytes());

    // --- save subcatchment results
    if prj.Nobjects[SUBCATCH as usize] > 0 {
        output_saveSubcatchResults(prj, reportTime);
    }

    // --- save average routing results over reporting period if called for
    if prj.RptFlags.averages != 0 {
        output_saveAvgResults(prj);
    }
    // --- otherwise save interpolated point routing results
    else {
        if prj.Nobjects[NODE as usize] > 0 {
            output_saveNodeResults(prj, reportTime);
        }
        if prj.Nobjects[LINK as usize] > 0 {
            output_saveLinkResults(prj, reportTime);
        }
    }

    // --- update & save system-wide flows
    let StepFlowTotals = prj.massbal.StepFlowTotals;
    let ucf = UCF(prj, FLOW);
    let sr = &mut prj.output.SysResults;
    sr[SYS_FLOODING as usize] = (StepFlowTotals.flooding * ucf) as f32;
    sr[SYS_OUTFLOW as usize] = (StepFlowTotals.outflow * ucf) as f32;
    sr[SYS_DWFLOW as usize] = (StepFlowTotals.dwInflow * ucf) as f32;
    sr[SYS_GWFLOW as usize] = (StepFlowTotals.gwInflow * ucf) as f32;
    sr[SYS_IIFLOW as usize] = (StepFlowTotals.iiInflow * ucf) as f32;
    sr[SYS_EXFLOW as usize] = (StepFlowTotals.exInflow * ucf) as f32;
    sr[SYS_INFLOW as usize] = sr[SYS_RUNOFF as usize]
        + sr[SYS_DWFLOW as usize]
        + sr[SYS_GWFLOW as usize]
        + sr[SYS_IIFLOW as usize]
        + sr[SYS_EXFLOW as usize];
    let sys = prj.output.SysResults;
    write_real4s(&mut prj.Fout, &sys, MAX_SYS_RESULTS as usize);

    // --- save outfall flows to interface file if called for
    if prj.Foutflows.mode as i32 == SAVE_FILE && prj.IgnoreRouting == 0 {
        iface_saveOutletResults(prj, reportDate);
    }
    prj.Nperiods += 1;
}

//=============================================================================

pub fn output_end(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: writes closing records to binary file.
//
//  (C writes the first sizeof(INT4) bytes of each 8-byte F_OFF position,
//  i.e. its low 32 bits on a little-endian machine.)
{
    let mut k: i32;
    let (a, b, c) = (prj.output.IDStartPos, prj.output.InputStartPos, prj.output.OutputStartPos);
    write_int4(prj, a as i32);
    write_int4(prj, b as i32);
    write_int4(prj, c as i32);
    k = prj.Nperiods as i32;
    write_int4(prj, k);
    k = prj.ErrorCode;
    write_int4(prj, k);
    k = MAGICNUMBER;
    // (C reports ERR_OUT_WRITE if fwrite fails; a buffered write cannot)
    write_int4(prj, k);
}

//=============================================================================

pub fn output_close(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: frees memory used for accessing the binary file.
//
{
    prj.output.SubcatchResults = Vec::new();
    prj.output.NodeResults = Vec::new();
    prj.output.LinkResults = Vec::new();
    output_closeAvgResults(prj);
}

//=============================================================================

fn output_saveID(id: &[u8], file: &mut TFile)
//
//  Input:   id = name of an object
//           file = ptr. to binary output file
//  Output:  none
//  Purpose: writes an object's name to the binary output file.
//
{
    let n: i32 = cio::strlen(id) as i32;
    file.write_bytes(&n.to_le_bytes());
    file.write_bytes(&id[..n as usize]);
}

//=============================================================================

fn output_saveSubcatchResults(prj: &mut Project, reportTime: f64)
//
//  Input:   reportTime = elapsed simulation time (millisec)
//           file = ptr. to binary output file
//  Output:  none
//  Purpose: writes computed subcatchment results to binary file.
//
{
    let mut j: i32;
    let mut f: f64;
    let mut area: f64;
    let mut totalArea: f32 = 0.0f32;
    let reportDate: DateTime = getDateTime(prj, reportTime);

    // --- update reported rainfall at each rain gage
    j = 0;
    while j < prj.Nobjects[GAGE as usize] {
        gage_setReportRainfall(prj, j, reportDate);
        j += 1;
    }

    // --- find where current reporting time lies between latest runoff times
    f = (reportTime - prj.OldRunoffTime) / (prj.NewRunoffTime - prj.OldRunoffTime);

    // --- write subcatchment results to file
    j = 0;
    while j < prj.Nobjects[SUBCATCH as usize] {
        // --- retrieve interpolated results for reporting time & write to file
        let mut r = std::mem::take(&mut prj.output.SubcatchResults);
        subcatch_getResults(prj, j, f, &mut r);
        prj.output.SubcatchResults = r;
        if prj.Subcatch[j as usize].rptFlag != 0 {
            let n = prj.output.NumSubcatchVars as usize;
            write_real4s(&mut prj.Fout, &prj.output.SubcatchResults, n);
        }

        // --- update system-wide results
        area = prj.Subcatch[j as usize].area * UCF(prj, LANDAREA);
        totalArea += area as f32;
        let sc = &prj.output.SubcatchResults;
        let ucfEvap = UCF(prj, EVAPRATE);
        let gwEvap = prj.Subcatch[j as usize].groundwater.as_ref().map(|gw| gw.evapLoss);
        let sr = &mut prj.output.SysResults;
        sr[SYS_RAINFALL as usize] += (sc[SUBCATCH_RAINFALL as usize] as f64 * area) as f32;
        sr[SYS_SNOWDEPTH as usize] += (sc[SUBCATCH_SNOWDEPTH as usize] as f64 * area) as f32;
        sr[SYS_EVAP as usize] += (sc[SUBCATCH_EVAP as usize] as f64 * area) as f32;
        if let Some(evapLoss) = gwEvap {
            sr[SYS_EVAP as usize] += (evapLoss * ucfEvap * area) as f32;
        }
        sr[SYS_INFIL as usize] += (sc[SUBCATCH_INFIL as usize] as f64 * area) as f32;
        sr[SYS_RUNOFF as usize] += sc[SUBCATCH_RUNOFF as usize];
        j += 1;
    }

    // --- normalize system-wide results to catchment area
    if totalArea as f64 > 0.0 {
        let sr = &mut prj.output.SysResults;
        sr[SYS_EVAP as usize] /= totalArea;
        sr[SYS_RAINFALL as usize] /= totalArea;
        sr[SYS_SNOWDEPTH as usize] /= totalArea;
        sr[SYS_INFIL as usize] /= totalArea;
    }

    // --- update system temperature and PET
    if prj.UnitSystem == SI {
        f = (5.0 / 9.0) * (prj.Temp.ta - 32.0);
    } else {
        f = prj.Temp.ta;
    }
    prj.output.SysResults[SYS_TEMPERATURE as usize] = f as f32;
    f = prj.Evap.rate * UCF(prj, EVAPRATE);
    prj.output.SysResults[SYS_PET as usize] = f as f32;
}

//=============================================================================

fn output_saveNodeResults(prj: &mut Project, reportTime: f64)
//
//  Input:   reportTime = elapsed simulation time (millisec)
//           file = ptr. to binary output file
//  Output:  none
//  Purpose: writes computed node results to binary file.
//
{
    let mut j: i32;

    // --- find where current reporting time lies between latest routing times
    let f: f64 = (reportTime - prj.OldRoutingTime) / (prj.NewRoutingTime - prj.OldRoutingTime);

    // --- write node results to file
    j = 0;
    while j < prj.Nobjects[NODE as usize] {
        // --- retrieve interpolated results for reporting time & write to file
        let mut r = std::mem::take(&mut prj.output.NodeResults);
        node_getResults(prj, j, f, &mut r);
        prj.output.NodeResults = r;
        if prj.Node[j as usize].rptFlag != 0 {
            let n = prj.output.NumNodeVars as usize;
            write_real4s(&mut prj.Fout, &prj.output.NodeResults, n);
        }
        let d = prj.output.NodeResults[NODE_DEPTH as usize] as f64;
        stats_updateMaxNodeDepth(prj, j, d);

        // --- update system-wide storage volume
        prj.output.SysResults[SYS_STORAGE as usize] += prj.output.NodeResults[NODE_VOLUME as usize];
        j += 1;
    }
}

//=============================================================================

fn output_saveLinkResults(prj: &mut Project, reportTime: f64)
//
//  Input:   reportTime = elapsed simulation time (millisec)
//           file = ptr. to binary output file
//  Output:  none
//  Purpose: writes computed link results to binary file.
//
{
    let mut j: i32;
    let f: f64;
    let mut z: f64;

    // --- find where current reporting time lies between latest routing times
    f = (reportTime - prj.OldRoutingTime) / (prj.NewRoutingTime - prj.OldRoutingTime);

    // --- write link results to file
    j = 0;
    while j < prj.Nobjects[LINK as usize] {
        // --- retrieve interpolated results for reporting time & write to file
        if prj.Link[j as usize].rptFlag != 0 {
            let mut r = std::mem::take(&mut prj.output.LinkResults);
            link_getResults(prj, j, f, &mut r);
            prj.output.LinkResults = r;
            let n = prj.output.NumLinkVars as usize;
            write_real4s(&mut prj.Fout, &prj.output.LinkResults, n);
        }

        // --- update system-wide results
        z = ((1.0 - f) * prj.Link[j as usize].oldVolume + f * prj.Link[j as usize].newVolume) * UCF(prj, VOLUME);
        prj.output.SysResults[SYS_STORAGE as usize] += z as f32;
        j += 1;
    }
}

//=============================================================================

pub fn output_readDateTime(prj: &mut Project, period: i64, days: &mut DateTime)
//
//  Input:   period = index of reporting time period
//  Output:  days = date/time value
//  Purpose: retrieves the date/time for a specific reporting period
//           from the binary output file.
//
{
    let p: i64 = period;
    let bytePos: i64 = prj
        .output
        .OutputStartPos
        .wrapping_add((p.wrapping_sub(1)).wrapping_mul(prj.output.BytesPerPeriod));
    prj.Fout.f().fseek_set(bytePos);
    *days = NO_DATE as f64;
    // fread: a short read leaves the bytes it did not reach as they were
    let mut b = days.to_le_bytes();
    let mut buf = [0u8; 8];
    let got = prj.Fout.f().read_bytes(&mut buf);
    b[..got].copy_from_slice(&buf[..got]);
    *days = f64::from_le_bytes(b);
}

//=============================================================================

pub fn output_readSubcatchResults(prj: &mut Project, period: i64, index: i32)
//
//  Input:   period = index of reporting time period
//           index = subcatchment index in binary output file
//  Output:  none
//  Purpose: reads computed results for a subcatchment at a specific time
//           period.
//
{
    let o = &prj.output;
    let offset: i64 = index.wrapping_mul(o.NumSubcatchVars) as i64;
    let p: i64 = period;
    let bytePos: i64 = o
        .OutputStartPos
        .wrapping_add((p.wrapping_sub(1)).wrapping_mul(o.BytesPerPeriod))
        .wrapping_add(8)
        .wrapping_add(offset.wrapping_mul(4));
    prj.Fout.f().fseek_set(bytePos);
    let n = prj.output.NumSubcatchVars as usize;
    read_real4s(&mut prj.Fout, &mut prj.output.SubcatchResults, n);
}

//=============================================================================

pub fn output_readNodeResults(prj: &mut Project, period: i64, index: i32)
//
//  Input:   period = index of reporting time period
//           index = node index in binary output file
//  Output:  none
//  Purpose: reads computed results for a node at a specific time period.
//
{
    let o = &prj.output;
    let offset: i64 = o
        .NumSubcatch
        .wrapping_mul(o.NumSubcatchVars)
        .wrapping_add(index.wrapping_mul(o.NumNodeVars)) as i64;
    let p: i64 = period;
    let bytePos: i64 = o
        .OutputStartPos
        .wrapping_add((p.wrapping_sub(1)).wrapping_mul(o.BytesPerPeriod))
        .wrapping_add(8)
        .wrapping_add(offset.wrapping_mul(4));
    prj.Fout.f().fseek_set(bytePos);
    let n = prj.output.NumNodeVars as usize;
    read_real4s(&mut prj.Fout, &mut prj.output.NodeResults, n);
}

//=============================================================================

pub fn output_readLinkResults(prj: &mut Project, period: i64, index: i32)
//
//  Input:   period = index of reporting time period
//           index = link index in binary output file
//  Output:  none
//  Purpose: reads computed results for a link at a specific time period.
//
{
    let o = &prj.output;
    let offset: i64 = o
        .NumSubcatch
        .wrapping_mul(o.NumSubcatchVars)
        .wrapping_add(o.NumNodes.wrapping_mul(o.NumNodeVars))
        .wrapping_add(index.wrapping_mul(o.NumLinkVars)) as i64;
    let p: i64 = period;
    let bytePos: i64 = o
        .OutputStartPos
        .wrapping_add((p.wrapping_sub(1)).wrapping_mul(o.BytesPerPeriod))
        .wrapping_add(8)
        .wrapping_add(offset.wrapping_mul(4));
    prj.Fout.f().fseek_set(bytePos);
    let n = prj.output.NumLinkVars as usize;
    read_real4s(&mut prj.Fout, &mut prj.output.LinkResults, n);
    read_real4s(&mut prj.Fout, &mut prj.output.SysResults, MAX_SYS_RESULTS as usize);
}

//=============================================================================
//  Functions for saving average results within a reporting period to file.
//=============================================================================

fn output_openAvgResults(prj: &mut Project) -> i32
//
//  Allocates memory for storing average results for nodes and links.
{
    let mut i: i32;
    let o = &mut prj.output;
    let nz = |n: i32| if n > 0 { n as usize } else { 0 };

    // --- allocate memory for averages at reportable nodes
    o.AvgNodeResults = vec![TAvgResults::default(); nz(o.NumNodes)];

    // --- allocate memory for averages at reportable links
    o.AvgLinkResults = vec![TAvgResults::default(); nz(o.NumLinks)];

    // --- allocate memory for each reportable variable for each reportable node
    i = 0;
    while i < o.NumNodes {
        o.AvgNodeResults[i as usize].xAvg = vec![0.0f32; nz(o.NumNodeVars)];
        i += 1;
    }

    // --- allocate memory for each reportable variable for each reportable link
    i = 0;
    while i < o.NumLinks {
        o.AvgLinkResults[i as usize].xAvg = vec![0.0f32; nz(o.NumLinkVars)];
        i += 1;
    }
    TRUE
}

//=============================================================================

fn output_closeAvgResults(prj: &mut Project)
//
//  Frees memory used for storing average results for nodes and links.
{
    prj.output.AvgNodeResults = Vec::new();
    prj.output.AvgLinkResults = Vec::new();
}

//=============================================================================

fn output_initAvgResults(prj: &mut Project)
//
//  Initializes average node & link results.
{
    let mut i: i32;
    let mut j: i32;
    let o = &mut prj.output;
    o.Nsteps = 0;
    i = 0;
    while i < o.NumNodes {
        j = 0;
        while j < o.NumNodeVars {
            o.AvgNodeResults[i as usize].xAvg[j as usize] = 0.0;
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < o.NumLinks {
        j = 0;
        while j < o.NumLinkVars {
            o.AvgLinkResults[i as usize].xAvg[j as usize] = 0.0;
            j += 1;
        }
        i += 1;
    }
}

//=============================================================================

pub fn output_updateAvgResults(prj: &mut Project) {
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut _sign: i32;

    // --- update average accumulations for nodes
    k = 0;
    i = 0;
    while i < prj.Nobjects[NODE as usize] {
        if prj.Node[i as usize].rptFlag == 0 {
            i += 1;
            continue;
        }
        let mut r = std::mem::take(&mut prj.output.NodeResults);
        node_getResults(prj, i, 1.0, &mut r);
        prj.output.NodeResults = r;
        let o = &mut prj.output;
        j = 0;
        while j < o.NumNodeVars {
            o.AvgNodeResults[k as usize].xAvg[j as usize] += o.NodeResults[j as usize];
            j += 1;
        }
        k += 1;
        i += 1;
    }

    // --- update average accumulations for links
    k = 0;
    i = 0;
    while i < prj.Nobjects[LINK as usize] {
        if prj.Link[i as usize].rptFlag == 0 {
            i += 1;
            continue;
        }
        let mut r = std::mem::take(&mut prj.output.LinkResults);
        link_getResults(prj, i, 1.0, &mut r);
        prj.output.LinkResults = r;
        let isConduit = prj.Link[i as usize].type_ == CONDUIT;
        let o = &mut prj.output;

        // --- save sign of current flow rate
        _sign = if o.LinkResults[LINK_FLOW as usize] < 0.0 { -1 } else { 1 };

        // --- add current results to average accumulation
        j = 0;
        while j < o.NumLinkVars {
            if j == LINK_CAPACITY {
                // --- accumulate capacity (fraction full) for conduits
                if isConduit {
                    o.AvgLinkResults[k as usize].xAvg[j as usize] += o.LinkResults[j as usize];
                }
                // --- for other links capacity is pump speed or regulator
                //     opening fraction which shouldn't be averaged
                //     (multiplying by Nsteps+1 will preserve last value
                //     when average results are taken in saveAvgResults())
                else {
                    o.AvgLinkResults[k as usize].xAvg[j as usize] =
                        o.LinkResults[j as usize] * (o.Nsteps.wrapping_add(1)) as f32;
                }
            }
            // --- accumulation for all other reported results
            else {
                o.AvgLinkResults[k as usize].xAvg[j as usize] += o.LinkResults[j as usize];
            }
            j += 1;
        }
        k += 1;
        i += 1;
    }
    prj.output.Nsteps += 1;
}

//=============================================================================

fn output_saveAvgResults(prj: &mut Project) {
    let mut i: i32;
    let mut j: i32;

    // --- examine each reportable node
    i = 0;
    while i < prj.output.NumNodes {
        // --- determine the node's average results
        let o = &mut prj.output;
        j = 0;
        while j < o.NumNodeVars {
            o.NodeResults[j as usize] = o.AvgNodeResults[i as usize].xAvg[j as usize] / o.Nsteps as f32;
            j += 1;
        }

        // --- save average results to file
        let n = prj.output.NumNodeVars as usize;
        write_real4s(&mut prj.Fout, &prj.output.NodeResults, n);
        i += 1;
    }

    // --- update each node's max depth and contribution to system storage
    i = 0;
    while i < prj.Nobjects[NODE as usize] {
        let d = prj.Node[i as usize].newDepth * UCF(prj, LENGTH);
        stats_updateMaxNodeDepth(prj, i, d);
        prj.output.SysResults[SYS_STORAGE as usize] +=
            (prj.Node[i as usize].newVolume * UCF(prj, VOLUME)) as f32;
        i += 1;
    }

    // --- examine each reportable link
    i = 0;
    while i < prj.output.NumLinks {
        // --- determine the link's average results
        let o = &mut prj.output;
        j = 0;
        while j < o.NumLinkVars {
            o.LinkResults[j as usize] = o.AvgLinkResults[i as usize].xAvg[j as usize] / o.Nsteps as f32;
            j += 1;
        }

        // --- save average results to file
        let n = prj.output.NumLinkVars as usize;
        write_real4s(&mut prj.Fout, &prj.output.LinkResults, n);
        i += 1;
    }

    // --- add each link's volume to total system storage
    i = 0;
    while i < prj.Nobjects[LINK as usize] {
        prj.output.SysResults[SYS_STORAGE as usize] +=
            (prj.Link[i as usize].newVolume * UCF(prj, VOLUME)) as f32;
        i += 1;
    }

    // --- re-initialize average results for all nodes and links
    output_initAvgResults(prj);
}
