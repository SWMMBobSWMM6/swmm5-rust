//! input.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   input.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     06/01/22  (Build 5.2.1)
//   Author:   L. Rossman
//
//   Input data processing functions.
//
//   Update History
//   ==============
//   Build 5.1.007:
//   - Support added for climate adjustment input data.
//   Build 5.1.011:
//   - Support added for reading hydraulic event dates.
//   Build 5.1.015:
//   - Support added for multiple infiltration methods within a project.
//   Build 5.2.0:
//   - Support added for Streets and Inlets.
//   - Support added for named variables & math expressions in control rules.
//   Build 5.2.1:
//   - Possible integer underflow avoided in getTokens() function.
//-----------------------------------------------------------------------------
//
// Lines are kept as C strings: a Vec<u8> holding the bytes fgets read plus a
// terminating NUL, so that the NULs getTokens and strtok write into a line
// cut it exactly where they cut C's buffer.

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const MAXERRS: i32 = 100; // Max. input errors reported

/// The file-scope variables of input.c.
pub struct InputState {
    /// String tokens from line of input (C: pointers into the line; here
    /// copies of the first Ntokens tokens)
    Tok: Vec<Vec<u8>>,
    /// Number of tokens in line of input
    Ntokens: i32,
    /// Working number of objects of each type
    Mobjects: [i32; MAX_OBJ_TYPES as usize],
    /// Working number of node objects
    Mnodes: [i32; MAX_NODE_TYPES as usize],
    /// Working number of link objects
    Mlinks: [i32; MAX_LINK_TYPES as usize],
    /// Working number of event periods
    Mevents: i32,
}
impl Default for InputState {
    fn default() -> Self {
        InputState {
            Tok: Vec::new(),
            Ntokens: 0,
            Mobjects: [0; MAX_OBJ_TYPES as usize],
            Mnodes: [0; MAX_NODE_TYPES as usize],
            Mlinks: [0; MAX_LINK_TYPES as usize],
            Mevents: 0,
        }
    }
}

//-----------------------------------------------------------------------------
//  External Functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  input_countObjects  (called by swmm_open in swmm5.c)
//  input_readData      (called by swmm_open in swmm5.c)

/// A line read by fgets, as a C string (the bytes plus a terminating NUL).
fn cline(bytes: Vec<u8>) -> Vec<u8> {
    let mut v = bytes;
    v.push(0);
    v
}

/// `sstrncpy(wLine, line, MAXLINE)` into a new C string buffer.
fn working_copy(line: &[u8]) -> Vec<u8> {
    let s = cio::cstr(line);
    let n = s.len().min(MAXLINE as usize);
    let mut w = s[..n].to_vec();
    w.push(0);
    w
}

/// C `strcspn(s, reject)` on the C string starting at `s[pos]`.
fn strcspn(s: &[u8], pos: usize, reject: &[u8]) -> usize {
    let mut i = pos;
    while i < s.len() && s[i] != 0 && !reject.contains(&s[i]) {
        i += 1;
    }
    i - pos
}

/// C `strspn(s, accept)` on the C string starting at `s[pos]`.
fn strspn(s: &[u8], pos: usize, accept: &[u8]) -> usize {
    let mut i = pos;
    while i < s.len() && s[i] != 0 && accept.contains(&s[i]) {
        i += 1;
    }
    i - pos
}

/// glibc `strtok` over one C string buffer: `next` is the saved position
/// (None once the string is exhausted). Writes a NUL after each token, as
/// strtok does, and returns the token's start position.
fn strtok(s: &mut [u8], next: &mut Option<usize>, delim: &[u8]) -> Option<usize> {
    let mut pos = match *next {
        Some(p) => p,
        None => return None,
    };
    // --- skip leading delimiters
    pos += strspn(s, pos, delim);
    if pos >= s.len() || s[pos] == 0 {
        *next = None;
        return None;
    }
    // --- find the end of the token
    let start = pos;
    let end = start + strcspn(s, start, delim);
    if end < s.len() && s[end] != 0 {
        s[end] = 0;
        *next = Some(end + 1);
    } else {
        *next = None;
    }
    Some(start)
}

//=============================================================================

pub fn input_countObjects(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns error code
//  Purpose: reads input file to determine number of system objects.
//
{
    let mut line: Vec<u8>; // line from input data file
    let mut wLine: Vec<u8>; // working copy of input line
    let mut tok: Option<usize>; // first string token of line
    let mut sect: i32 = -1; // input data sections
    let mut newsect: i32;
    let mut errcode: i32 = 0; // error code
    let mut errsum: i32 = 0; // number of errors found
    let mut i: usize;
    let mut lineCount: i64 = 0;

    // --- initialize number of objects & set default values
    if prj.ErrorCode != 0 {
        return prj.ErrorCode;
    }
    error_setInpError(prj, 0, b"");
    i = 0;
    while i < MAX_OBJ_TYPES as usize {
        prj.Nobjects[i] = 0;
        i += 1;
    }
    i = 0;
    while i < MAX_NODE_TYPES as usize {
        prj.Nnodes[i] = 0;
        i += 1;
    }
    i = 0;
    while i < MAX_LINK_TYPES as usize {
        prj.Nlinks[i] = 0;
        i += 1;
    }
    controls_init(prj);

    // --- make pass through data file counting number of each object
    while let Some(l) = prj.Finp.f().fgets(MAXLINE as usize) {
        line = cline(l);

        // --- skip blank lines & those beginning with a comment
        lineCount += 1;
        wLine = working_copy(&line); // make working copy of line
        let mut strtokNext: Option<usize> = Some(0);
        tok = strtok(&mut wLine, &mut strtokNext, SEPSTR); // get first text token on line
        let t = match tok {
            None => continue,
            Some(t) => t,
        };
        if wLine[t] == b';' {
            continue;
        }

        // --- check if line begins with a new section heading
        if wLine[t] == b'[' {
            // --- look for heading in list of section keywords
            newsect = findmatch(&wLine[t..], SectWords);
            if newsect >= 0 {
                sect = newsect;
                continue;
            } else {
                sect = -1;
                errcode = ERR_KEYWORD;
            }
        }

        // --- if in OPTIONS section then read the option setting
        //     otherwise add object and its ID name (tok) to project
        if sect == s_OPTION {
            errcode = readOption(prj, &mut line);
        } else if sect >= 0 {
            errcode = addObject(prj, sect, &mut wLine, t, &mut strtokNext);
        }

        // --- report any error found
        if errcode != 0 {
            report_writeInputErrorMsg(prj, errcode, sect, &line, lineCount);
            errsum += 1;
            if errsum >= MAXERRS {
                break;
            }
        }
    }

    // --- set global error code if input errors were found
    if errsum > 0 {
        prj.ErrorCode = ERR_INPUT;
    }
    prj.ErrorCode
}

//=============================================================================

pub fn input_readData(prj: &mut Project) -> i32
//
//  Input:   none
//  Output:  returns error code
//  Purpose: reads input file to determine input parameters for each object.
//
{
    let mut line: Vec<u8>; // line from input data file
    let mut wLine: Vec<u8>; // working copy of input line
    let mut comment: Option<usize>; // ptr. to start of comment in input line
    let mut sect: i32; // data sections
    let mut newsect: i32;
    let mut inperr: i32; // error code & total error count
    let mut errsum: i32;
    let mut lineLength: i32; // number of characters in input line
    let mut i: usize;
    let mut lineCount: i64 = 0;

    // --- initialize working item count arrays
    //     (final counts in Mobjects, Mnodes & Mlinks should
    //      match those in Nobjects, Nnodes and Nlinks).
    if prj.ErrorCode != 0 {
        return prj.ErrorCode;
    }
    error_setInpError(prj, 0, b"");
    i = 0;
    while i < MAX_OBJ_TYPES as usize {
        prj.input.Mobjects[i] = 0;
        i += 1;
    }
    i = 0;
    while i < MAX_NODE_TYPES as usize {
        prj.input.Mnodes[i] = 0;
        i += 1;
    }
    i = 0;
    while i < MAX_LINK_TYPES as usize {
        prj.input.Mlinks[i] = 0;
        i += 1;
    }
    prj.input.Mevents = 0;

    // --- initialize starting date for all time series
    i = 0;
    while i < prj.Nobjects[TSERIES as usize] as usize {
        prj.Tseries[i].lastDate = prj.StartDate + prj.StartTime;
        i += 1;
    }

    // --- read each line from input file
    sect = 0;
    errsum = 0;
    prj.Finp.f().rewind();
    while let Some(l) = prj.Finp.f().fgets(MAXLINE as usize) {
        line = cline(l);

        // --- make copy of line and scan for tokens
        lineCount += 1;
        wLine = working_copy(&line);
        prj.input.Ntokens = getTokens(prj, &mut wLine);

        // --- skip blank lines and comments
        if prj.input.Ntokens == 0 {
            continue;
        }
        if prj.input.Tok[0].first().copied().unwrap_or(0) == b';' {
            continue;
        }

        // --- check if max. line length exceeded
        lineLength = cio::strlen(&line) as i32;
        if lineLength >= MAXLINE {
            // --- don't count comment if present
            comment = cio::strchr(&line, b';');
            if let Some(c) = comment {
                lineLength = c as i32; // Pointer math here
            }
            if lineLength >= MAXLINE {
                inperr = ERR_LINE_LENGTH;
                report_writeInputErrorMsg(prj, inperr, sect, &line, lineCount);
                errsum += 1;
            }
        }

        // --- check if at start of a new input section
        if prj.input.Tok[0].first().copied().unwrap_or(0) == b'[' {
            // --- match token against list of section keywords
            newsect = findmatch(&prj.input.Tok[0], SectWords);
            if newsect >= 0 {
                // --- SPECIAL CASE FOR TRANSECTS
                //     finish processing the last set of transect data
                if sect == s_TRANSECT {
                    let n = prj.Nobjects[TRANSECT as usize] - 1;
                    transect_validate(prj, n);
                }

                // --- begin a new input section
                sect = newsect;
                continue;
            } else {
                let t0 = prj.input.Tok[0].clone();
                inperr = error_setInpError(prj, ERR_KEYWORD, &t0);
                report_writeInputErrorMsg(prj, inperr, sect, &line, lineCount);
                errsum += 1;
                break;
            }
        }
        // --- otherwise parse tokens from input line
        else {
            inperr = parseLine(prj, sect, &mut line);
            if inperr > 0 {
                errsum += 1;
                if errsum > MAXERRS {
                    report_writeLine(prj, FMT19);
                } else {
                    report_writeInputErrorMsg(prj, inperr, sect, &line, lineCount);
                }
            }
        }

        // --- stop if reach end of file or max. error count
        if errsum > MAXERRS {
            break;
        }
    } /* End of while */

    // --- check for errors
    if errsum > 0 {
        prj.ErrorCode = ERR_INPUT;
    }
    prj.ErrorCode
}

//=============================================================================

fn addObject(prj: &mut Project, objType: i32, wLine: &mut [u8], idPos: usize, strtokNext: &mut Option<usize>) -> i32
//
//  Input:   objType = object type index
//           id = object's ID string
//  Output:  returns an error code
//  Purpose: adds a new object to the project.
//
//  (id is the token at wLine[idPos]; the curve and transect cases continue
//  C's strtok scan of wLine, whose state is strtokNext.)
{
    let mut errcode: i32 = 0;
    let mut id: Vec<u8> = cio::cstr(&wLine[idPos..]).to_vec();

    // project_addObject(type, id, Nobjects[type]); 0 means a duplicate
    macro_rules! add {
        ($type_:expr) => {{
            let n = prj.Nobjects[$type_ as usize];
            project_addObject(prj, $type_, &id, n)
        }};
    }

    match objType {
        s_RAINGAGE => {
            if add!(GAGE) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[GAGE as usize] += 1;
        }

        s_SUBCATCH => {
            if add!(SUBCATCH) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[SUBCATCH as usize] += 1;
        }

        s_AQUIFER => {
            if add!(AQUIFER) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[AQUIFER as usize] += 1;
        }

        s_UNITHYD => {
            // --- the same Unit Hydrograph can span several lines
            if project_findObject(prj, UNITHYD, &id) < 0 {
                if add!(UNITHYD) == 0 {
                    errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
                }
                prj.Nobjects[UNITHYD as usize] += 1;
            }
        }

        s_SNOWMELT => {
            // --- the same Snowmelt object can appear on several lines
            if project_findObject(prj, SNOWMELT, &id) < 0 {
                if add!(SNOWMELT) == 0 {
                    errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
                }
                prj.Nobjects[SNOWMELT as usize] += 1;
            }
        }

        s_JUNCTION => {
            if add!(NODE) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[NODE as usize] += 1;
            prj.Nnodes[JUNCTION as usize] += 1;
        }

        s_OUTFALL => {
            if add!(NODE) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[NODE as usize] += 1;
            prj.Nnodes[OUTFALL as usize] += 1;
        }

        s_STORAGE => {
            if add!(NODE) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[NODE as usize] += 1;
            prj.Nnodes[STORAGE as usize] += 1;
        }

        s_DIVIDER => {
            if add!(NODE) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[NODE as usize] += 1;
            prj.Nnodes[DIVIDER as usize] += 1;
        }

        s_CONDUIT => {
            if add!(LINK) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[LINK as usize] += 1;
            prj.Nlinks[CONDUIT as usize] += 1;
        }

        s_PUMP => {
            if add!(LINK) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[LINK as usize] += 1;
            prj.Nlinks[PUMP as usize] += 1;
        }

        s_ORIFICE => {
            if add!(LINK) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[LINK as usize] += 1;
            prj.Nlinks[ORIFICE as usize] += 1;
        }

        s_WEIR => {
            if add!(LINK) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[LINK as usize] += 1;
            prj.Nlinks[WEIR as usize] += 1;
        }

        s_OUTLET => {
            if add!(LINK) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[LINK as usize] += 1;
            prj.Nlinks[OUTLET as usize] += 1;
        }

        s_POLLUTANT => {
            if add!(POLLUT) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[POLLUT as usize] += 1;
        }

        s_LANDUSE => {
            if add!(LANDUSE) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[LANDUSE as usize] += 1;
        }

        s_PATTERN => {
            // --- a time pattern can span several lines
            if project_findObject(prj, TIMEPATTERN, &id) < 0 {
                if add!(TIMEPATTERN) == 0 {
                    errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
                }
                prj.Nobjects[TIMEPATTERN as usize] += 1;
            }
        }

        s_CURVE => {
            // --- a Curve can span several lines
            if project_findObject(prj, CURVE, &id) < 0 {
                if add!(CURVE) == 0 {
                    errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
                }
                prj.Nobjects[CURVE as usize] += 1;

                // --- check for a conduit shape curve
                //     (C passes strtok's NULL to findmatch when the line has
                //     no second token, and crashes there; a missing token
                //     reads here as an empty string, which matches nothing)
                let next = strtok(wLine, strtokNext, SEPSTR);
                id = match next {
                    Some(p) => cio::cstr(&wLine[p..]).to_vec(),
                    None => Vec::new(),
                };
                if findmatch(&id, CurveTypeWords) == SHAPE_CURVE {
                    prj.Nobjects[SHAPE as usize] += 1;
                }
            }
        }

        s_TIMESERIES => {
            // --- a Time Series can span several lines
            if project_findObject(prj, TSERIES, &id) < 0 {
                if add!(TSERIES) == 0 {
                    errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
                }
                prj.Nobjects[TSERIES as usize] += 1;
            }
        }

        s_CONTROL => {
            if r#match(&id, w_RULE) != 0 {
                prj.Nobjects[CONTROL as usize] += 1;
            } else {
                controls_addToCount(prj, &id);
            }
        }

        s_TRANSECT => {
            // --- for TRANSECTS, ID name appears as second entry on X1 line
            if r#match(&id, b"X1") != 0 {
                let next = strtok(wLine, strtokNext, SEPSTR);
                if let Some(p) = next {
                    id = cio::cstr(&wLine[p..]).to_vec();
                    if add!(TRANSECT) == 0 {
                        errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
                    }
                    prj.Nobjects[TRANSECT as usize] += 1;
                }
            }
        }

        s_LID_CONTROL => {
            // --- an LID object can span several lines
            if project_findObject(prj, LID, &id) < 0 {
                if add!(LID) == 0 {
                    errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
                }
                prj.Nobjects[LID as usize] += 1;
            }
        }

        s_EVENT => {
            prj.NumEvents += 1;
        }

        s_STREET => {
            if add!(STREET) == 0 {
                errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
            }
            prj.Nobjects[STREET as usize] += 1;
        }

        s_INLET => {
            // --- an INLET object can span several lines
            if project_findObject(prj, INLET, &id) < 0 {
                if add!(INLET) == 0 {
                    errcode = error_setInpError(prj, ERR_DUP_NAME, &id);
                }
                prj.Nobjects[INLET as usize] += 1;
            }
        }

        _ => {}
    }
    errcode
}

//=============================================================================

fn parseLine(prj: &mut Project, sect: i32, line: &mut Vec<u8>) -> i32
//
//  Input:   sect  = current section of input file
//           *line = line of text read from input file
//  Output:  returns error code or 0 if no error found
//  Purpose: parses contents of a tokenized line of text read from input file.
//
//  (The readers get the shared Tok/Ntokens as their tok/ntoks arguments;
//  Tok is taken out of the input state for the call and put back after.)
{
    let Tok = std::mem::take(&mut prj.input.Tok);
    let Ntokens = prj.input.Ntokens;
    let result = parseLineTokens(prj, sect, line, &Tok, Ntokens);
    prj.input.Tok = Tok;
    result
}

fn parseLineTokens(prj: &mut Project, sect: i32, line: &mut Vec<u8>, Tok: &[Vec<u8>], Ntokens: i32) -> i32 {
    let j: i32;
    let err: i32;
    match sect {
        s_TITLE => readTitle(prj, line),

        s_RAINGAGE => {
            j = prj.input.Mobjects[GAGE as usize];
            err = gage_readParams(prj, j, Tok, Ntokens);
            prj.input.Mobjects[GAGE as usize] += 1;
            err
        }

        s_TEMP => climate_readParams(prj, Tok, Ntokens),

        s_EVAP => climate_readEvapParams(prj, Tok, Ntokens),

        s_ADJUST => climate_readAdjustments(prj, Tok, Ntokens),

        s_SUBCATCH => {
            j = prj.input.Mobjects[SUBCATCH as usize];
            err = subcatch_readParams(prj, j, Tok, Ntokens);
            prj.input.Mobjects[SUBCATCH as usize] += 1;
            err
        }

        s_SUBAREA => subcatch_readSubareaParams(prj, Tok, Ntokens),

        s_INFIL => {
            let m = prj.InfilModel;
            infil_readParams(prj, m, Tok, Ntokens)
        }

        s_AQUIFER => {
            j = prj.input.Mobjects[AQUIFER as usize];
            err = gwater_readAquiferParams(prj, j, Tok, Ntokens);
            prj.input.Mobjects[AQUIFER as usize] += 1;
            err
        }

        s_GROUNDWATER => gwater_readGroundwaterParams(prj, Tok, Ntokens),

        s_GWF => gwater_readFlowExpression(prj, Tok, Ntokens),

        s_SNOWMELT => snow_readMeltParams(prj, Tok, Ntokens),

        s_JUNCTION => readNode(prj, JUNCTION, Tok, Ntokens),

        s_OUTFALL => readNode(prj, OUTFALL, Tok, Ntokens),

        s_STORAGE => readNode(prj, STORAGE, Tok, Ntokens),

        s_DIVIDER => readNode(prj, DIVIDER, Tok, Ntokens),

        s_CONDUIT => readLink(prj, CONDUIT, Tok, Ntokens),

        s_PUMP => readLink(prj, PUMP, Tok, Ntokens),

        s_ORIFICE => readLink(prj, ORIFICE, Tok, Ntokens),

        s_WEIR => readLink(prj, WEIR, Tok, Ntokens),

        s_OUTLET => readLink(prj, OUTLET, Tok, Ntokens),

        s_XSECTION => link_readXsectParams(prj, Tok, Ntokens),

        s_TRANSECT => {
            let mut count = prj.input.Mobjects[TRANSECT as usize];
            let r = transect_readParams(prj, &mut count, Tok, Ntokens);
            prj.input.Mobjects[TRANSECT as usize] = count;
            r
        }

        s_LOSSES => link_readLossParams(prj, Tok, Ntokens),

        s_POLLUTANT => {
            j = prj.input.Mobjects[POLLUT as usize];
            err = landuse_readPollutParams(prj, j, Tok, Ntokens);
            prj.input.Mobjects[POLLUT as usize] += 1;
            err
        }

        s_LANDUSE => {
            j = prj.input.Mobjects[LANDUSE as usize];
            err = landuse_readParams(prj, j, Tok, Ntokens);
            prj.input.Mobjects[LANDUSE as usize] += 1;
            err
        }

        s_BUILDUP => landuse_readBuildupParams(prj, Tok, Ntokens),

        s_WASHOFF => landuse_readWashoffParams(prj, Tok, Ntokens),

        s_COVERAGE => subcatch_readLanduseParams(prj, Tok, Ntokens),

        s_INFLOW => inflow_readExtInflow(prj, Tok, Ntokens),

        s_DWF => inflow_readDwfInflow(prj, Tok, Ntokens),

        s_PATTERN => inflow_readDwfPattern(prj, Tok, Ntokens),

        s_RDII => rdii_readRdiiInflow(prj, Tok, Ntokens),

        s_UNITHYD => rdii_readUnitHydParams(prj, Tok, Ntokens),

        s_LOADING => subcatch_readInitBuildup(prj, Tok, Ntokens),

        s_TREATMENT => treatmnt_readExpression(prj, Tok, Ntokens),

        s_CURVE => table_readCurve(prj, Tok, Ntokens),

        s_TIMESERIES => table_readTimeseries(prj, Tok, Ntokens),

        s_CONTROL => readControl(prj, Tok, Ntokens, Tok, Ntokens),

        s_REPORT => report_readOptions(prj, Tok, Ntokens),

        s_FILE => iface_readFileParams(prj, Tok, Ntokens),

        s_LID_CONTROL => lid_readProcParams(prj, Tok, Ntokens),

        s_LID_USAGE => lid_readGroupParams(prj, Tok, Ntokens),

        s_EVENT => readEvent(prj, Tok, Ntokens),

        s_STREET => street_readParams(prj, Tok, Ntokens),

        s_INLET => inlet_readDesignParams(prj, Tok, Ntokens),

        s_INLET_USAGE => inlet_readUsageParams(prj, Tok, Ntokens),

        _ => 0,
    }
}

//=============================================================================

fn readControl(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32, Tok: &[Vec<u8>], Ntokens: i32) -> i32
//
//  Input:   tok[] = array of string tokens
//           ntoks = number of tokens
//  Output:  returns error code
//  Purpose: reads a line of input for a control rule.
//
//  (Tok and Ntokens are input.c's shared token list, which C uses here
//  alongside the tok/ntoks arguments; they are the same tokens.)
{
    let index: i32;
    let keyword: i32;

    // --- check for minimum number of tokens
    if ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    if r#match(&tok[0], w_VARIABLE) != 0 {
        return controls_addVariable(prj, tok, ntoks);
    }
    if r#match(&tok[0], w_EXPRESSION) != 0 {
        return controls_addExpression(prj, tok, ntoks);
    }

    // --- get index of control rule keyword
    keyword = findmatch(&tok[0], RuleKeyWords);
    if keyword < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[0]);
    }

    // --- if line begins a new control rule, add rule ID to the database
    if keyword == 0 {
        let n = prj.input.Mobjects[CONTROL as usize];
        if project_addObject(prj, CONTROL, &tok[1], n) == 0 {
            return error_setInpError(prj, ERR_DUP_NAME, &Tok[1]);
        }
        prj.input.Mobjects[CONTROL as usize] += 1;
    }

    // --- get index of last control rule processed
    index = prj.input.Mobjects[CONTROL as usize] - 1;
    if index < 0 {
        return error_setInpError(prj, ERR_RULE, b"");
    }

    // --- add current line as a new clause to the control rule
    controls_addRuleClause(prj, index, keyword, Tok, Ntokens)
}

//=============================================================================

fn readOption(prj: &mut Project, line: &mut [u8]) -> i32
//
//  Input:   line = line of input data
//  Output:  returns error code
//  Purpose: reads an input line containing a project option.
//
{
    prj.input.Ntokens = getTokens(prj, line);
    if prj.input.Ntokens < 2 {
        return 0;
    }
    let s1 = prj.input.Tok[0].clone();
    let s2 = prj.input.Tok[1].clone();
    project_readOption(prj, &s1, &s2)
}

//=============================================================================

fn readTitle(prj: &mut Project, line: &mut Vec<u8>) -> i32
//
//  Input:   line = line from input file
//  Output:  returns error code
//  Purpose: reads project title from line of input.
//
{
    let mut i: usize;
    let n: usize;
    i = 0;
    while i < MAXTITLE as usize {
        // --- find next empty Title entry
        if cio::strlen(&prj.Title[i]) == 0 {
            // --- strip line feed character from input line
            //     (the line is never empty here: it has a token)
            n = cio::strlen(line);
            if n >= 1 && line[n - 1] == 10 {
                line[n - 1] = b' ';
            }

            // --- copy input line into Title entry
            sstrncpy(&mut prj.Title[i], line, MAXMSG as usize);
            break;
        }
        i += 1;
    }
    0
}

//=============================================================================

fn readNode(prj: &mut Project, type_: i32, Tok: &[Vec<u8>], Ntokens: i32) -> i32
//
//  Input:   type = type of node
//  Output:  returns error code
//  Purpose: reads data for a node from a line of input.
//
{
    let j = prj.input.Mobjects[NODE as usize];
    let k = prj.input.Mnodes[type_ as usize];
    let err = node_readParams(prj, j, type_, k, Tok, Ntokens);
    prj.input.Mobjects[NODE as usize] += 1;
    prj.input.Mnodes[type_ as usize] += 1;
    err
}

//=============================================================================

fn readLink(prj: &mut Project, type_: i32, Tok: &[Vec<u8>], Ntokens: i32) -> i32
//
//  Input:   type = type of link
//  Output:  returns error code
//  Purpose: reads data for a link from a line of input.
//
{
    let j = prj.input.Mobjects[LINK as usize];
    let k = prj.input.Mlinks[type_ as usize];
    let err = link_readParams(prj, j, type_, k, Tok, Ntokens);
    prj.input.Mobjects[LINK as usize] += 1;
    prj.input.Mlinks[type_ as usize] += 1;
    err
}

//=============================================================================

fn readEvent(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    let mut x: [DateTime; 4] = [0.0; 4];

    if ntoks < 4 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    if datetime_strToDate(prj, &tok[0], &mut x[0]) == 0 {
        return error_setInpError(prj, ERR_DATETIME, &tok[0]);
    }
    if datetime_strToTime(&tok[1], &mut x[1]) == 0 {
        return error_setInpError(prj, ERR_DATETIME, &tok[1]);
    }
    if datetime_strToDate(prj, &tok[2], &mut x[2]) == 0 {
        return error_setInpError(prj, ERR_DATETIME, &tok[2]);
    }
    if datetime_strToTime(&tok[3], &mut x[3]) == 0 {
        return error_setInpError(prj, ERR_DATETIME, &tok[3]);
    }

    let m = prj.input.Mevents as usize;
    prj.Event[m].start = x[0] + x[1];
    prj.Event[m].end = x[2] + x[3];
    if prj.Event[m].start >= prj.Event[m].end {
        return error_setInpError(prj, ERR_DATETIME, b" - start date exceeds end date");
    }
    prj.input.Mevents += 1;
    0
}

//=============================================================================

pub fn findmatch(s: &[u8], keyword: &[&[u8]]) -> i32
//
//  Input:   s = character string
//           keyword = array of keyword strings
//  Output:  returns index of matching keyword or -1 if no match found
//  Purpose: finds match between string and array of keyword strings.
//
{
    let mut i: usize = 0;
    while i < keyword.len() {
        if r#match(s, keyword[i]) != 0 {
            return i as i32;
        }
        i += 1;
    }
    -1
}

//=============================================================================

pub fn r#match(str: &[u8], substr: &[u8]) -> i32
//
//  Input:   str = character string being searched
//           substr = sub-string being searched for
//  Output:  returns 1 if sub-string found, 0 if not
//  Purpose: sees if a sub-string of characters appears in a string
//           (not case sensitive).
//
{
    let at = |s: &[u8], i: usize| s.get(i).copied().unwrap_or(0);
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;

    // --- fail if substring is empty
    if at(substr, 0) == 0 {
        return 0;
    }

    // --- skip leading blanks of str
    k = 0;
    while at(str, k) != 0 {
        if at(str, k) != b' ' {
            break;
        }
        k += 1;
    }

    // --- check if substr matches remainder of str
    i = k;
    j = 0;
    while at(substr, j) != 0 {
        if at(str, i) == 0 || UCHAR(at(str, i)) != UCHAR(at(substr, j)) {
            return 0;
        }
        i += 1;
        j += 1;
    }
    1
}

//=============================================================================

pub fn getInt(s: &[u8], y: &mut i32) -> i32
//
//  Input:   s = a character string
//  Output:  y = converted value of s,
//           returns 1 if conversion successful, 0 if not
//  Purpose: converts a string to an integer number.
//
{
    let mut x: f64 = 0.0;
    if getDouble(s, &mut x) != 0 {
        if x < 0.0 {
            x -= 0.01;
        } else {
            x += 0.01;
        }
        *y = f64_to_i32(x);
        return 1;
    }
    *y = 0;
    0
}

//=============================================================================

pub fn getFloat(s: &[u8], y: &mut f32) -> i32
//
//  Input:   s = a character string
//  Output:  y = converted value of s,
//           returns 1 if conversion successful, 0 if not
//  Purpose: converts a string to a single precision floating point number.
//
{
    let (v, n) = cio::strtod(s);
    *y = v as f32;
    // --- *endptr > 0 with a signed char: a byte >= 0x80 does not count
    if endchar(s, n) > 0 {
        return 0;
    }
    1
}

//=============================================================================

pub fn getDouble(s: &[u8], y: &mut f64) -> i32
//
//  Input:   s = a character string
//  Output:  y = converted value of s,
//           returns 1 if conversion successful, 0 if not
//  Purpose: converts a double precision floating point number.
//
{
    let (v, n) = cio::strtod(s);
    *y = v;
    // --- *endptr > 0 with a signed char: a byte >= 0x80 does not count
    if endchar(s, n) > 0 {
        return 0;
    }
    1
}

/// The (signed) C char strtod's endptr points at: the byte after the
/// `n` bytes it consumed, or the terminator.
fn endchar(s: &[u8], n: usize) -> i8 {
    let c = cio::cstr(s);
    c.get(n).copied().unwrap_or(0) as i8
}

//=============================================================================

fn getTokens(prj: &mut Project, s: &mut [u8]) -> i32
//
//  Input:   s = a character string
//  Output:  returns number of tokens found in s
//  Purpose: scans a string for tokens, saving pointers to them
//           in shared variable Tok[].
//
//  Notes:   Tokens can be separated by the characters listed in SEPSTR
//           (spaces, tabs, newline, carriage return) which is defined
//           in CONSTS.H. Text between quotes is treated as a single token.
//
//  (s is a C string buffer; NULs are written into it exactly as C does.
//  Tok receives copies of the tokens.)
{
    let mut len: i32;
    let mut n: i32;
    let mut m: i32;
    let mut pos: usize = 0;

    // --- begin with no tokens
    prj.input.Tok.clear();
    n = 0;

    // --- truncate s at start of comment
    let c = cio::strchr(s, b';');
    if let Some(c) = c {
        s[c] = 0;
    }
    len = cio::strlen(s) as i32;

    // --- scan s for tokens until nothing left
    while len > 0 && n < MAXTOKS {
        m = strcspn(s, pos, SEPSTR) as i32; // find token length
        if m == 0 {
            pos += 1; // no token found
        } else {
            if s[pos] == b'"' {
                // token begins with quote
                pos += 1; // start token after quote
                len -= 1; // reduce length of s
                m = strcspn(s, pos, b"\"\n") as i32; // find end quote or new line
            }
            s[pos + m as usize] = 0; // null-terminate the token
            prj.input.Tok.push(cio::cstr(&s[pos..]).to_vec()); // save pointer to token
            n += 1; // update token count
            pos += m as usize + 1; // begin next token
        }
        len -= m + 1; // update length of s
    }
    n
}
