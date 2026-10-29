//! rain.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   rain.c
//
//   Project: EPA SWMM5
//   Version: 5.2
//   Date:    11/01/21  (Build 5.2.0)
//   Author:  L. Rossman
//
//   Places rainfall data from external files into a SWMM rainfall
//   interface file.
//
//   The following types of external data files are supported:
//   NWS_TAPE:            NCDC NWS TD 3240 or 3260 data in fixed field widths
//   NWS_SPACE_DELIMITED: NCDC NWS TD (DSI) 3240 or 3260 data in space delimited
//                        format, with or without header lines, with or without
//                        station name
//   NWS_COMMA_DELIMITED: NCDC NWS TD (DSI) 3240 or 3260 data in comma delimited
//                        format, with or without header lines
//   NWS_ONLINE_60:       NCDC NWS hourly space delimited online format
//   NWS_ONLINE_15:       NCDC NWS fifteen minute space delimited online format
//   AES_HLY:             Canadian AES hourly data with 3-digit year
//   CMC_HLY:             Canadian CMC hourly data in HLY03 or HLY21 format
//   CMC_FIF:             Canadian CMC fifteen minute data in in FIF21 format
//   STD_SPACE_DELIMITED: standard SWMM space delimted format:
//                        StaID  Year  Month  Day  Hour  Minute  Rainfall
//
//   The layout of the SWMM binary rainfall interface file is:
//     File stamp ("SWMM5-RAIN") (10 bytes)
//     Number of SWMM rain gages in file (4-byte int)
//     Repeated for each rain gage:
//       recording station ID (not SWMM rain gage ID) (MAXMSG+1 (=80) bytes)
//       gage recording interval (seconds) (4-byte int)
//       starting byte of rain data in file (4-byte int)
//       ending byte+1 of rain data in file (4-byte int)
//     For each gage:
//       For each time period with non-zero rain:
//         Date/time for start of period (8-byte double)
//         Rain depth (inches) (4-byte float)
//
//   Update History
//   ==============
//   Release 5.1.010:
//   - Modified error message for records out of sequence in std. format file.
//   Release 5.1.011:
//   - Can now read decimal rainfall values in newer NWS online format.
//   Release 5.1.013:
//   - Variable x properly initialized with float value in readNwsOnlineValue().
//   Release 5.1.014:
//   - Fixed indexing bug in rainFileConflict() function.
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
// enum RainFileFormat
const UNKNOWN_FORMAT: i32 = 0;
const NWS_TAPE: i32 = 1;
const NWS_SPACE_DELIMITED: i32 = 2;
const NWS_COMMA_DELIMITED: i32 = 3;
const NWS_ONLINE_60: i32 = 4;
const NWS_ONLINE_15: i32 = 5;
const AES_HLY: i32 = 6;
const CMC_HLY: i32 = 7;
const CMC_FIF: i32 = 8;
const STD_SPACE_DELIMITED: i32 = 9;
// enum ConditionCodes
const NO_CONDITION: i32 = 0;
const ACCUMULATED_PERIOD: i32 = 1;
const DELETED_PERIOD: i32 = 2;
const MISSING_PERIOD: i32 = 3;

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of rain.c.
#[derive(Default)]
pub struct RainState {
    RainStats: TRainStats,    // see objects.h for definition
    Condition: i32,           // rainfall condition code
    TimeOffset: i32,          // time offset of rainfall reading (sec)
    DataOffset: i32,          // start of data on line of input
    ValueOffset: i32,         // start of rain value on input line
    RainType: i32,            // rain measurement type code
    Interval: i32,            // rain measurement interval (sec)
    UnitsFactor: f64,         // units conversion factor
    RainAccum: f32,           // rainfall depth accumulation
    StationID: Option<usize>, // station ID appearing in rain file
    //   (C: char* pointing at Gage[i].staID; here the gage index i)
    AccumStartDate: DateTime, // date when accumulation begins
    PreviousDate: DateTime,   // date of previous rainfall record
    GageIndex: i32,           // index of rain gage analyzed
    hasStationName: i32,      // true if data contains station name
}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//  rain_open   (called by swmm_start in swmm5.c)
//  rain_close  (called by swmm_end in swmm5.c)

//-----------------------------------------------------------------------------
//  Helpers for C library behaviour
//-----------------------------------------------------------------------------

/// `fgets(buf, n, f) != NULL` into a fixed C buffer: on success the line and
/// its terminator overwrite the start of `buf` and the bytes after the
/// terminator keep what they held, as in C; at EOF `buf` is untouched.
fn fgets_into(f: &mut CFile, buf: &mut [u8], n: usize) -> bool {
    match f.fgets(n) {
        Some(line) => {
            buf[..line.len()].copy_from_slice(&line);
            buf[line.len()] = 0;
            true
        }
        None => false,
    }
}

/// `fread(&v, sizeof(int), 1, f)`: bytes read land in `v` even when the
/// item is incomplete, as glibc copies them; returns the items read.
fn fread_i32(f: &mut CFile, v: &mut i32) -> usize {
    let mut b = v.to_le_bytes();
    let n = f.read_bytes(&mut b);
    *v = i32::from_le_bytes(b);
    n / 4
}

//=============================================================================

pub fn rain_open(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: opens binary rain interface file and RDII processor.
//
{
    let mut i: i32;
    let mut count: i32;

    // --- see how many gages get their data from a file
    count = 0;
    i = 0;
    while i < prj.Nobjects[GAGE as usize] {
        if prj.Gage[i as usize].dataSource == RAIN_FILE {
            count += 1;
        }
        i += 1;
    }
    prj.Frain.file = None;
    if count == 0 {
        prj.Frain.mode = NO_FILE as i8;
    }
    // --- see what kind of rain interface file to open
    else {
        match prj.Frain.mode as i32 {
            SCRATCH_FILE => {
                let mut name = prj.Frain.name;
                getTempFileName(prj, &mut name);
                prj.Frain.name = name;
                prj.Frain.file = CFile::open(&prj.Frain.name, "w+b");
                if prj.Frain.file.is_none() {
                    report_writeErrorMsg(prj, ERR_RAIN_FILE_SCRATCH, b"");
                    return;
                }
            }

            USE_FILE => {
                prj.Frain.file = CFile::open(&prj.Frain.name, "r+b");
                if prj.Frain.file.is_none() {
                    let name = prj.Frain.name;
                    report_writeErrorMsg(prj, ERR_RAIN_FILE_OPEN, &name);
                    return;
                }
            }

            SAVE_FILE => {
                prj.Frain.file = CFile::open(&prj.Frain.name, "w+b");
                if prj.Frain.file.is_none() {
                    let name = prj.Frain.name;
                    report_writeErrorMsg(prj, ERR_RAIN_FILE_OPEN, &name);
                    return;
                }
            }
            _ => {}
        }
    }

    // --- create new rain file if required
    if prj.Frain.mode as i32 == SCRATCH_FILE || prj.Frain.mode as i32 == SAVE_FILE {
        createRainFile(prj, count);
    }

    // --- initialize rain file
    if prj.Frain.mode as i32 != NO_FILE {
        initRainFile(prj);
    }

    // --- open RDII processor (creates/opens a RDII interface file)
    rdii_openRdii(prj);
}

//=============================================================================

pub fn rain_close(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: closes rain interface file and RDII processor.
//
{
    if prj.Frain.file.is_some() {
        prj.Frain.file = None; // fclose
        if prj.Frain.mode as i32 == SCRATCH_FILE {
            cio::remove(&prj.Frain.name);
        }
    }
    prj.Frain.file = None;
    rdii_closeRdii(prj);
}

//=============================================================================

fn createRainFile(prj: &mut Project, count: i32)
//
//  Input:   count = number of files to include in rain interface file
//  Output:  none
//  Purpose: adds rain data from all rain gage files to the interface file.
//
{
    let mut i: i32;
    let mut k: i32;
    let kount: i32 = count; // number of gages in data file
    let mut filePos1: i32; // starting byte of gage's header data
    let mut filePos2: i32; // starting byte of gage's rain data
    let mut filePos3: i32; // starting byte of next gage's data
    let mut interval: i32; // recording interval (sec)
    let dummy: i32 = -1;
    // (C's staID is uninitialized stack beyond the bytes strcpy/sstrncpy
    //  set, and all MAXMSG+1 bytes are written to the file; those trailing
    //  bytes are zeros here. They are never read back: only the C string
    //  up to its terminator is compared in findGageInFile.)
    let mut staID: [u8; MAXMSG as usize + 1] = [0; MAXMSG as usize + 1]; // gage's ID name
    let fileStamp: &[u8] = b"SWMM5-RAIN";

    // --- make sure interface file is open and no error condition
    if prj.ErrorCode != 0 || prj.Frain.file.is_none() {
        return;
    }

    // --- write file stamp & # gages to file
    prj.Frain.f().write_bytes(fileStamp);
    prj.Frain.f().write_bytes(&kount.to_le_bytes());
    filePos1 = prj.Frain.f().ftell() as i32;

    // --- write default fill-in header records to file for each gage
    //     (will be replaced later with actual records)
    if count > 0 {
        let rs = prj.rain.RainStats;
        report_writeRainStats(prj, -1, &rs);
    }
    cio::strcpy(&mut staID, b" ");
    i = 0;
    while i < count {
        prj.Frain.f().write_bytes(&staID);
        k = 1;
        while k <= 3 {
            prj.Frain.f().write_bytes(&dummy.to_le_bytes());
            k += 1;
        }
        i += 1;
    }
    filePos2 = prj.Frain.f().ftell() as i32;

    // --- loop through project's  rain gages,
    //     looking for ones using rain files
    i = 0;
    while i < prj.Nobjects[GAGE as usize] {
        if prj.ErrorCode != 0 || prj.Gage[i as usize].dataSource != RAIN_FILE {
            i += 1;
            continue;
        }
        if rainFileConflict(prj, i) != 0 {
            break;
        }

        // --- position rain file to where data for gage will begin
        prj.Frain.f().fseek_set(filePos2 as i64);

        // --- add gage's data to rain file
        if addGageToRainFile(prj, i) != 0 {
            // --- write header records for gage to beginning of rain file
            filePos3 = prj.Frain.f().ftell() as i32;
            prj.Frain.f().fseek_set(filePos1 as i64);
            sstrncpy(&mut staID, &prj.Gage[i as usize].staID, MAXMSG as usize);
            interval = prj.rain.Interval;
            prj.Frain.f().write_bytes(&staID);
            prj.Frain.f().write_bytes(&interval.to_le_bytes());
            prj.Frain.f().write_bytes(&filePos2.to_le_bytes());
            prj.Frain.f().write_bytes(&filePos3.to_le_bytes());
            filePos1 = prj.Frain.f().ftell() as i32;
            filePos2 = filePos3;
            let rs = prj.rain.RainStats;
            report_writeRainStats(prj, i, &rs);
        }
        i += 1;
    }

    // --- if there was an error condition, then delete newly created file
    if prj.ErrorCode != 0 {
        prj.Frain.file = None; // fclose
        cio::remove(&prj.Frain.name);
    }
}

//=============================================================================

fn rainFileConflict(prj: &mut Project, i: i32) -> i32
//
//  Input:   i = rain gage index
//  Output:  returns 1 if file conflict found, 0 if not
//  Purpose: checks if a rain gage's station ID matches another gage's
//           station ID but the two use different rain data files.
//
{
    let mut j: i32;
    let iu = i as usize;
    j = 0;
    while j < i {
        let ju = j as usize;
        if strcomp(&prj.Gage[ju].staID, &prj.Gage[iu].staID) != 0
            && strcomp(&prj.Gage[ju].fname, &prj.Gage[iu].fname) == 0
        {
            let id = prj.Gage[iu].ID.clone();
            report_writeErrorMsg(prj, ERR_RAIN_FILE_CONFLICT, &id);
            return 1;
        }
        j += 1;
    }
    0
}

//=============================================================================

fn addGageToRainFile(prj: &mut Project, i: i32) -> i32
//
//  Input:   i = rain gage index
//  Output:  returns 1 if successful, 0 if not
//  Purpose: adds a gage's rainfall record to rain interface file
//
{
    let fileFormat: i32; // file format code
    let mut hdrLines: i32 = 0; // number of header lines skipped

    // --- let StationID point to NULL
    prj.rain.StationID = None;

    // --- check that rain file exists
    let fname = prj.Gage[i as usize].fname;
    match CFile::open(&fname, "rt") {
        None => report_writeErrorMsg(prj, ERR_RAIN_FILE_DATA, &fname),
        Some(mut f) => {
            fileFormat = findFileFormat(prj, &mut f, i, &mut hdrLines);
            if fileFormat == UNKNOWN_FORMAT {
                report_writeErrorMsg(prj, ERR_RAIN_FILE_FORMAT, &fname);
            } else {
                prj.rain.GageIndex = i;
                let day1 = prj.Gage[i as usize].startFileDate;
                let day2 = prj.Gage[i as usize].endFileDate;
                readFile(prj, &mut f, fileFormat, hdrLines, day1, day2);
            }
            drop(f); // fclose
        }
    }
    if prj.ErrorCode != 0 {
        0
    } else {
        1
    }
}

//=============================================================================

fn initRainFile(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: initializes rain interface file for reading.
//
{
    let fileStamp: &[u8] = b"SWMM5-RAIN\0";
    let mut fStamp: [u8; 11] = *b"SWMM5-RAIN\0";
    let mut i: i32;
    // (C leaves kount uninitialized; it stays so only if the read below
    //  fails, where C would use an indeterminate value)
    let mut kount: i32 = 0;
    let filePos: i64;

    // --- make sure interface file is open and no error condition
    if prj.ErrorCode != 0 || prj.Frain.file.is_none() {
        return;
    }

    // --- check that interface file contains proper file stamp
    prj.Frain.f().rewind();
    let n = cio::strlen(fileStamp);
    prj.Frain.f().read_bytes(&mut fStamp[..n]);
    if cio::strcmp(&fStamp, fileStamp) != 0 {
        report_writeErrorMsg(prj, ERR_RAIN_IFACE_FORMAT, b"");
        return;
    }
    fread_i32(prj.Frain.f(), &mut kount);
    filePos = prj.Frain.f().ftell();

    // --- locate information for each raingage in interface file
    i = 0;
    while i < prj.Nobjects[GAGE as usize] {
        if prj.ErrorCode != 0 || prj.Gage[i as usize].dataSource != RAIN_FILE {
            i += 1;
            continue;
        }

        // --- match station ID for gage with one in file
        prj.Frain.f().fseek_set(filePos);
        if findGageInFile(prj, i, kount) == 0
            || prj.Gage[i as usize].startFilePos == prj.Gage[i as usize].endFilePos
        {
            let id = prj.Gage[i as usize].ID.clone();
            report_writeErrorMsg(prj, ERR_RAIN_FILE_GAGE, &id);
        }
        i += 1;
    }
}

//=============================================================================

fn findGageInFile(prj: &mut Project, i: i32, kount: i32) -> i32
//
//  Input:   i     = rain gage index
//           kount = number of rain gages stored on interface file
//  Output:  returns TRUE if successful, FALSE if not
//  Purpose: checks if rain gage's station ID appears in interface file.
//
{
    let mut k: i32;
    // (C leaves these uninitialized; they stay so only if a read fails)
    let mut interval: i32 = 0;
    let mut filePos1: i32 = 0;
    let mut filePos2: i32 = 0;
    let mut staID: [u8; MAXMSG as usize + 1] = [0; MAXMSG as usize + 1];

    k = 1;
    while k <= kount {
        let f = prj.Frain.f();
        f.read_bytes(&mut staID);
        fread_i32(f, &mut interval);
        fread_i32(f, &mut filePos1);
        fread_i32(f, &mut filePos2);
        if cio::strcmp(&staID, &prj.Gage[i as usize].staID) == 0 {
            // --- match found; save file parameters
            let g = &mut prj.Gage[i as usize];
            g.rainType = RAINFALL_VOLUME;
            g.rainInterval = interval;
            g.startFilePos = filePos1 as i64;
            g.endFilePos = filePos2 as i64;
            g.currentFilePos = g.startFilePos;
            return TRUE;
        }
        k += 1;
    }
    FALSE
}

//=============================================================================

fn findFileFormat(prj: &mut Project, f: &mut CFile, i: i32, hdrLines: &mut i32) -> i32
//
//  Input:   f = ptr. to rain gage's rainfall data file
//           i = rain gage index
//  Output:  hdrLines  = number of header lines found in data file;
//           returns type of format used in a rainfall data file
//  Purpose: finds the format of a gage's rainfall data file.
//
{
    let mut fileFormat: i32;
    let mut lineCount: i32;
    let maxCount: i32 = 5;
    let mut n: i32;
    let mut div: i32 = 0;
    let mut sn2: i64 = 0;
    let mut recdType: Vec<u8> = Vec::new();
    let mut elemType: Vec<u8> = Vec::new();
    let mut coopID: Vec<u8> = Vec::new();
    // (C's line buffer is uninitialized stack: a first line shorter than 37
    //  bytes makes the &line[37] scans below read indeterminate bytes after
    //  its terminator; here they are zeros. After the first line the buffer
    //  holds the previous lines' residue, exactly as in C.)
    let mut line: [u8; MAXLINE as usize] = [0; MAXLINE as usize];
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hour: i32 = 0;
    let mut minute: i32 = 0;
    let mut elem: i32 = 0;
    let mut x: f32 = 0.0;

    // --- check first few lines for known formats
    fileFormat = UNKNOWN_FORMAT;
    prj.rain.hasStationName = FALSE;
    prj.rain.UnitsFactor = 1.0;
    prj.rain.Interval = 0;
    *hdrLines = 0;
    lineCount = 1;
    while lineCount <= maxCount {
        if !fgets_into(f, &mut line, MAXLINE as usize) {
            return fileFormat;
        }

        // --- check for NWS space delimited format
        n = cio::sscanf(
            &line,
            b"%6ld %2d %4s",
            &mut [Scan::Long(&mut sn2), Scan::Int(&mut div), Scan::Str(&mut elemType)],
        );
        if n == 3 {
            prj.rain.Interval = getNWSInterval(&elemType);
            prj.rain.TimeOffset = prj.rain.Interval;
            if prj.rain.Interval > 0 {
                fileFormat = NWS_SPACE_DELIMITED;
                break;
            }
        }

        // --- check for NWS space delimited format w/ station name
        n = cio::sscanf(
            &line[37..],
            b"%2d %4s %2s %4d",
            &mut [
                Scan::Int(&mut div),
                Scan::Str(&mut elemType),
                Scan::Str(&mut recdType),
                Scan::Int(&mut year),
            ],
        );
        if n == 4 {
            prj.rain.Interval = getNWSInterval(&elemType);
            prj.rain.TimeOffset = prj.rain.Interval;
            if prj.rain.Interval > 0 {
                fileFormat = NWS_SPACE_DELIMITED;
                prj.rain.hasStationName = TRUE;
                break;
            }
        }

        // --- check for NWS coma delimited format
        n = cio::sscanf(
            &line,
            b"%6ld,%2d,%4s",
            &mut [Scan::Long(&mut sn2), Scan::Int(&mut div), Scan::Str(&mut elemType)],
        );
        if n == 3 {
            prj.rain.Interval = getNWSInterval(&elemType);
            prj.rain.TimeOffset = prj.rain.Interval;
            if prj.rain.Interval > 0 {
                fileFormat = NWS_COMMA_DELIMITED;
                break;
            }
        }

        // --- check for NWS comma delimited format w/ station name
        n = cio::sscanf(
            &line[37..],
            b"%2d,%4s,%2s,%4d",
            &mut [
                Scan::Int(&mut div),
                Scan::Str(&mut elemType),
                Scan::Str(&mut recdType),
                Scan::Int(&mut year),
            ],
        );
        if n == 4 {
            prj.rain.Interval = getNWSInterval(&elemType);
            prj.rain.TimeOffset = prj.rain.Interval;
            if prj.rain.Interval > 0 {
                fileFormat = NWS_COMMA_DELIMITED;
                prj.rain.hasStationName = TRUE;
                break;
            }
        }

        // --- check for NWS TAPE format
        n = cio::sscanf(
            &line,
            b"%3s%6ld%2d%4s",
            &mut [
                Scan::Str(&mut recdType),
                Scan::Long(&mut sn2),
                Scan::Int(&mut div),
                Scan::Str(&mut elemType),
            ],
        );
        if n == 4 {
            prj.rain.Interval = getNWSInterval(&elemType);
            prj.rain.TimeOffset = prj.rain.Interval;
            if prj.rain.Interval > 0 {
                fileFormat = NWS_TAPE;
                break;
            }
        }

        // --- check for NWS Online Retrieval format
        n = cio::sscanf(&line, b"%5s%6ld", &mut [Scan::Str(&mut coopID), Scan::Long(&mut sn2)]);
        if n == 2 && cio::strcmp(&coopID, b"COOP:") == 0 {
            fileFormat = findNWSOnlineFormat(prj, f, &mut line);
            break;
        }

        // --- check for AES type
        n = cio::sscanf(
            &line,
            b"%7ld%3d%2d%2d%3d",
            &mut [
                Scan::Long(&mut sn2),
                Scan::Int(&mut year),
                Scan::Int(&mut month),
                Scan::Int(&mut day),
                Scan::Int(&mut elem),
            ],
        );
        if n == 5 {
            if elem == 123 && cio::strlen(&line) >= 185 {
                fileFormat = AES_HLY;
                prj.rain.Interval = 3600;
                prj.rain.TimeOffset = prj.rain.Interval;
                prj.rain.UnitsFactor = 1.0 / MMperINCH;
                break;
            }
        }

        // --- check for CMC types
        n = cio::sscanf(
            &line,
            b"%7ld%4d%2d%2d%3d",
            &mut [
                Scan::Long(&mut sn2),
                Scan::Int(&mut year),
                Scan::Int(&mut month),
                Scan::Int(&mut day),
                Scan::Int(&mut elem),
            ],
        );
        if n == 5 {
            if elem == 159 && cio::strlen(&line) >= 691 {
                fileFormat = CMC_FIF;
                prj.rain.Interval = 900;
            } else if elem == 123 && cio::strlen(&line) >= 186 {
                fileFormat = CMC_HLY;
                prj.rain.Interval = 3600;
            }
            if fileFormat == CMC_FIF || fileFormat == CMC_HLY {
                prj.rain.TimeOffset = prj.rain.Interval;
                prj.rain.UnitsFactor = 1.0 / MMperINCH;
                break;
            }
        }

        // --- check for standard format
        if parseStdLine(prj, &line, &mut year, &mut month, &mut day, &mut hour, &mut minute, &mut x) != 0 {
            fileFormat = STD_SPACE_DELIMITED;
            prj.rain.RainType = prj.Gage[i as usize].rainType;
            prj.rain.Interval = prj.Gage[i as usize].rainInterval;
            if prj.Gage[i as usize].rainUnits == SI {
                prj.rain.UnitsFactor = 1.0 / MMperINCH;
            }
            prj.rain.TimeOffset = 0;
            prj.rain.StationID = Some(i as usize);
            break;
        }
        *hdrLines += 1;

        lineCount += 1;
    }
    if fileFormat != UNKNOWN_FORMAT {
        prj.Gage[i as usize].rainInterval = prj.rain.Interval;
    }
    fileFormat
}

//=============================================================================

fn findNWSOnlineFormat(prj: &mut Project, f: &mut CFile, line: &mut [u8]) -> i32
//
//  Input:   f = pointer to rainfall data file
//           line = line read from rainfall data file
//  Output:
//  Purpose: determines the file format for an NWS Online Retrieval data file.
//
{
    let mut n: i32;
    let fileFormat: i32;
    let mut str: Option<usize>;

    // --- read in the first header line of the file
    f.rewind();
    fgets_into(f, line, MAXLINE as usize);

    // --- if 'HPCP' appears then file is for hourly data
    str = cio::strstr(line, b"HPCP");
    if let Some(p) = str {
        prj.rain.Interval = 3600;
        prj.rain.TimeOffset = prj.rain.Interval;
        prj.rain.ValueOffset = p as i32;
        fileFormat = NWS_ONLINE_60;
    }
    // --- if 'QPCP" appears then file is for 15 minute data
    else {
        str = cio::strstr(line, b"QPCP");
        if let Some(p) = str {
            prj.rain.Interval = 900;
            prj.rain.TimeOffset = prj.rain.Interval;
            prj.rain.ValueOffset = p as i32;
            fileFormat = NWS_ONLINE_15;
        } else {
            return UNKNOWN_FORMAT;
        }
    }

    // --- find position in line where rainfall date begins
    //     (11 characters before last occurrence of ':')
    // --- read in first line of data
    n = 1;
    while n <= 5 {
        if !fgets_into(f, line, MAXLINE as usize) {
            return UNKNOWN_FORMAT;
        }
        if cio::strstr(line, b"COOP:").is_none() {
            n += 1;
            continue;
        }

        // --- find pointer to last occurrence of time separator character (':')
        str = cio::strrchr(line, b':');
        match str {
            None => return UNKNOWN_FORMAT,
            Some(p) => {
                // --- use pointer arithmetic to convert pointer to character position
                n = p as i32;
                prj.rain.DataOffset = n - 11;
                return fileFormat;
            }
        }
    }
    UNKNOWN_FORMAT
}

//=============================================================================

fn getNWSInterval(elemType: &[u8]) -> i32
//
//  Input:   elemType = code from NWS rainfall file
//  Output:  returns rainfall recording interval (sec)
//  Purpose: decodes NWS rain gage recording interval value
//
{
    if cio::strcmp(elemType, b"HPCP") == 0 {
        3600 // 1 hr rainfall
    } else if cio::strcmp(elemType, b"QPCP") == 0 {
        900 // 15 min rainfall
    } else if cio::strcmp(elemType, b"QGAG") == 0 {
        900 // 15 min rainfall
    } else {
        0
    }
}

//=============================================================================

fn readFile(prj: &mut Project, f: &mut CFile, fileFormat: i32, hdrLines: i32, day1: DateTime, day2: DateTime)
//
//  Input:   f          = ptr. to gage's rainfall data file
//           fileFormat = code of data file's format
//           hdrLines   = number of header lines in data file
//           day1       = starting day of record of interest
//           day2       = ending day of record of interest
//  Output:  none
//  Purpose: reads rainfall records from gage's data file to interface file.
//
{
    // (C's line buffer is uninitialized stack; only bytes after a short
    //  line's terminator can be read by the fixed-column scans, and after
    //  the first line they are the previous lines' residue, as here)
    let mut line: [u8; MAXLINE as usize] = [0; MAXLINE as usize];
    let mut i: i32;
    let mut n: i32;

    f.rewind();
    prj.rain.RainStats.startDate = NO_DATE as f64;
    prj.rain.RainStats.endDate = NO_DATE as f64;
    prj.rain.RainStats.periodsRain = 0;
    prj.rain.RainStats.periodsMissing = 0;
    prj.rain.RainStats.periodsMalfunc = 0;
    prj.rain.RainAccum = 0.0;
    prj.rain.AccumStartDate = NO_DATE as f64;
    prj.rain.PreviousDate = NO_DATE as f64;

    i = 1;
    while i <= hdrLines {
        if !fgets_into(f, &mut line, MAXLINE as usize) {
            return;
        }
        i += 1;
    }
    while fgets_into(f, &mut line, MAXLINE as usize) {
        match fileFormat {
            STD_SPACE_DELIMITED => {
                n = readStdLine(prj, &line, day1, day2);
            }

            NWS_TAPE | NWS_SPACE_DELIMITED | NWS_COMMA_DELIMITED | NWS_ONLINE_60 | NWS_ONLINE_15 => {
                n = readNWSLine(prj, &line, fileFormat, day1, day2);
            }

            AES_HLY | CMC_FIF | CMC_HLY => {
                n = readCMCLine(prj, &line, fileFormat, day1, day2);
            }

            _ => {
                n = -1;
            }
        }
        if n < 0 {
            break;
        }
    }
}

//=============================================================================

fn readNWSLine(prj: &mut Project, line: &[u8], fileFormat: i32, day1: DateTime, day2: DateTime) -> i32
//
//  Input:   line       = line of data from rainfall data file
//           fileFormat = code of data file's format
//           day1       = starting day of record of interest
//           day2       = ending day of record of interest
//  Output:  returns -1 if past end of desired record, 0 if data line could
//           not be read successfully or 1 if line read successfully
//  Purpose: reads a line of data from a rainfall data file and writes its
//           data to the rain interface file.
//
{
    let mut flag1: u8;
    let mut flag2: u8;
    let mut isMissing: i8;
    let mut date1: DateTime;
    let result: i64 = 1;
    let mut k: i32;
    // (y, m, d are written by the date sscanf before any use)
    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    let mut n: i32 = 0;
    let mut hour: i32;
    let mut minute: i32;
    let mut v: i64;
    let mut x: f32;
    let lineLength: i32 = cio::strlen(line) as i32 - 1;
    let mut nameLength: i32 = 0;

    // --- get year, month, & day from line
    match fileFormat {
        NWS_TAPE => {
            if lineLength <= 30 {
                return 0;
            }
            if cio::sscanf(
                &line[17..],
                b"%4d%2d%4d%3d",
                &mut [Scan::Int(&mut y), Scan::Int(&mut m), Scan::Int(&mut d), Scan::Int(&mut n)],
            ) < 4
            {
                return 0;
            }
            k = 30;
        }

        NWS_SPACE_DELIMITED => {
            if prj.rain.hasStationName != 0 {
                nameLength = 31;
            }
            if lineLength <= 28 + nameLength {
                return 0;
            }
            k = 18 + nameLength;
            if cio::sscanf(
                &line[k as usize..],
                b"%4d %2d %2d",
                &mut [Scan::Int(&mut y), Scan::Int(&mut m), Scan::Int(&mut d)],
            ) < 3
            {
                return 0;
            }
            k = k + 10;
        }

        NWS_COMMA_DELIMITED => {
            if lineLength <= 28 {
                return 0;
            }
            if cio::sscanf(
                &line[18..],
                b"%4d,%2d,%2d",
                &mut [Scan::Int(&mut y), Scan::Int(&mut m), Scan::Int(&mut d)],
            ) < 3
            {
                return 0;
            }
            k = 28;
        }

        NWS_ONLINE_60 | NWS_ONLINE_15 => {
            if lineLength <= prj.rain.DataOffset + 23 {
                return 0;
            }
            // (C indexes line[DataOffset]. A negative DataOffset, possible
            //  only if the first data line's last ':' is in its first 11
            //  columns, makes C read stack bytes before the buffer: undefined.
            //  It is taken here as the date not parsing, the likely C result
            //  since those bytes hold a return address, not ASCII digits.)
            if prj.rain.DataOffset < 0 {
                return 0;
            }
            if cio::sscanf(
                &line[prj.rain.DataOffset as usize..],
                b"%4d%2d%2d",
                &mut [Scan::Int(&mut y), Scan::Int(&mut m), Scan::Int(&mut d)],
            ) < 3
            {
                return 0;
            }
            k = prj.rain.DataOffset + 8;
        }

        _ => return 0,
    }

    // --- see if date is within period of record requested
    date1 = datetime_encodeDate(y, m, d);
    if day1 != NO_DATE as f64 && date1 < day1 {
        return 0;
    }
    if day2 != NO_DATE as f64 && date1 > day2 {
        return -1;
    }

    // --- read each recorded rainfall time, value, & codes from line
    while k < lineLength {
        flag1 = 0;
        flag2 = 0;
        v = 99999;
        hour = 25;
        minute = 0;
        match fileFormat {
            NWS_TAPE => {
                n = cio::sscanf(
                    &line[k as usize..],
                    b"%2d%2d%6ld%c%c",
                    &mut [
                        Scan::Int(&mut hour),
                        Scan::Int(&mut minute),
                        Scan::Long(&mut v),
                        Scan::Char(&mut flag1),
                        Scan::Char(&mut flag2),
                    ],
                );
                k += 12;
            }

            NWS_SPACE_DELIMITED => {
                n = cio::sscanf(
                    &line[k as usize..],
                    b" %2d%2d %6ld %c %c",
                    &mut [
                        Scan::Int(&mut hour),
                        Scan::Int(&mut minute),
                        Scan::Long(&mut v),
                        Scan::Char(&mut flag1),
                        Scan::Char(&mut flag2),
                    ],
                );
                k += 16;
            }

            NWS_COMMA_DELIMITED => {
                n = cio::sscanf(
                    &line[k as usize..],
                    b",%2d%2d,%6ld,%c,%c",
                    &mut [
                        Scan::Int(&mut hour),
                        Scan::Int(&mut minute),
                        Scan::Long(&mut v),
                        Scan::Char(&mut flag1),
                        Scan::Char(&mut flag2),
                    ],
                );
                k += 16;
            }

            NWS_ONLINE_60 | NWS_ONLINE_15 => {
                n = cio::sscanf(
                    &line[k as usize..],
                    b" %2d:%2d",
                    &mut [Scan::Int(&mut hour), Scan::Int(&mut minute)],
                );
                n += readNwsOnlineValue(&line[prj.rain.ValueOffset as usize..], &mut v, &mut flag1);

                // --- ending hour 0 is really hour 24 of previous day
                if hour == 0 {
                    hour = 24;
                    date1 -= 1.0;
                }
                k += lineLength;
            }

            _ => {
                n = 0;
            }
        }

        // --- check that we at least have an hour, minute & value
        //     (codes might be left off of the line)
        if n < 3 || hour >= 25 {
            break;
        }

        // --- set special condition code & update daily & hourly counts

        setCondition(prj, flag1);
        if prj.rain.Condition == DELETED_PERIOD || prj.rain.Condition == MISSING_PERIOD || flag1 == b'M' {
            isMissing = TRUE as i8;
        } else if v >= 9999 {
            isMissing = TRUE as i8;
        } else {
            isMissing = FALSE as i8;
        }

        // --- handle accumulation codes
        if flag1 == b'a' {
            prj.rain.AccumStartDate = date1 + datetime_encodeTime(hour, minute, 0.0);
        } else if flag1 == b'A' {
            saveAccumRainfall(prj, date1, hour, minute, v);
        }
        // --- handle all other conditions
        else {
            // --- convert rain measurement to inches & save it
            x = v as f32 / 100.0f32;
            if x > 0.0f32 || isMissing != 0 {
                saveRainfall(prj, date1, hour, minute, x, isMissing);
            }
        }

        // --- reset condition code if special condition period ended
        if flag1 == b'A' || flag1 == b'}' || flag1 == b']' {
            prj.rain.Condition = 0;
        }
    }
    result as i32
}

//=============================================================================

fn readNwsOnlineValue(s: &[u8], v: &mut i64, flag: &mut u8) -> i32
//
//  Input:   s = portion of rainfall record in NWS online format
//  Output:  v = rainfall amount in hundreths of an inch
//           flag = special condition flag
//           returns number of items read from s.
//  Purpose: reads rainfall value and condition flag from a NWS online
//           rainfall record.
//
{
    let n: i32;
    let mut x: f32 = 99.99f32;

    // --- check for newer format of decimal inches
    if cio::strchr(s, b'.').is_some() {
        n = cio::sscanf(s, b"%f %c", &mut [Scan::Float(&mut x), Scan::Char(flag)]);

        // --- convert to integer hundreths of an inch
        *v = f32_to_i64(100.0f32 * x + 0.5f32);
    }
    // --- older format of hundreths of an inch
    else {
        n = cio::sscanf(s, b"%ld %c", &mut [Scan::Long(v), Scan::Char(flag)]);
    }
    n
}

//=============================================================================

fn setCondition(prj: &mut Project, flag: u8) {
    match flag {
        b'a' | b'A' => {
            prj.rain.Condition = ACCUMULATED_PERIOD;
        }
        b'{' | b'}' => {
            prj.rain.Condition = DELETED_PERIOD;
        }
        b'[' | b']' => {
            prj.rain.Condition = MISSING_PERIOD;
        }
        _ => {
            prj.rain.Condition = NO_CONDITION;
        }
    }
}

//=============================================================================

fn readCMCLine(prj: &mut Project, line: &[u8], fileFormat: i32, day1: DateTime, day2: DateTime) -> i32
//
//  Input:   line = line of data from rainfall data file
//           fileFormat = code of data file's format
//           day1 = starting day of record of interest
//           day2 = ending day of record of interest
//  Output:  returns -1 if past end of desired record, 0 if data line could
//           not be read successfully or 1 if line read successfully
//  Purpose: reads a line of data from an AES or CMC rainfall data file and
//           writes its data to the rain interface file.
//
{
    let mut flag: u8 = 0;
    let mut isMissing: i8;
    let date1: DateTime;
    let mut sn: i64 = 0;
    let mut v: i64 = 0;
    let mut col: i32;
    let mut j: i32;
    let mut jMax: i32;
    // (y, m, d, elem are written by the sscanf before any use)
    let mut elem: i32 = 0;
    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    let mut hour: i32;
    let mut minute: i32;
    let mut x: f32;

    // --- get year, month, day & element code from line
    if fileFormat == AES_HLY {
        if cio::sscanf(
            line,
            b"%7ld%3d%2d%2d%3d",
            &mut [
                Scan::Long(&mut sn),
                Scan::Int(&mut y),
                Scan::Int(&mut m),
                Scan::Int(&mut d),
                Scan::Int(&mut elem),
            ],
        ) < 5
        {
            return 0;
        }
        if y < 100 {
            y = y + 2000;
        } else {
            y = y + 1000;
        }
        col = 17;
    } else {
        if cio::sscanf(
            line,
            b"%7ld%4d%2d%2d%3d",
            &mut [
                Scan::Long(&mut sn),
                Scan::Int(&mut y),
                Scan::Int(&mut m),
                Scan::Int(&mut d),
                Scan::Int(&mut elem),
            ],
        ) < 5
        {
            return 0;
        }
        col = 18;
    }

    // --- see if date is within period of record requested
    date1 = datetime_encodeDate(y, m, d);
    if day1 != NO_DATE as f64 && date1 < day1 {
        return 0;
    }
    if day2 != NO_DATE as f64 && date1 > day2 {
        return -1;
    }

    // --- make sure element code is for rainfall
    if fileFormat == AES_HLY && elem != 123 {
        return 0;
    } else if fileFormat == CMC_FIF && elem != 159 {
        return 0;
    } else if fileFormat == CMC_HLY && elem != 123 {
        return 0;
    }

    // --- read rainfall from each recording interval
    hour = 0; // starting hour
    minute = 0; // starting minute
    jMax = 24; // # recording intervals
    if fileFormat == CMC_FIF {
        jMax = 96;
    }
    j = 1;
    while j <= jMax {
        // (C reads line[col] even past a short line's terminator, i.e. the
        //  residue of earlier lines in the same buffer, as here)
        if cio::sscanf(&line[col as usize..], b"%6ld%c", &mut [Scan::Long(&mut v), Scan::Char(&mut flag)]) < 2 {
            return 0;
        }
        col += 7;
        if v == -99999 {
            isMissing = TRUE as i8;
        } else {
            isMissing = FALSE as i8;
        }

        // --- convert rain measurement from 0.1 mm to inches and save it
        x = (v as f64 / 10.0 / MMperINCH) as f32;
        if x > 0.0f32 || isMissing != 0 {
            saveRainfall(prj, date1, hour, minute, x, isMissing);
        }

        // --- update hour & minute for next interval
        if fileFormat == CMC_FIF {
            minute += 15;
            if minute == 60 {
                minute = 0;
                hour += 1;
            }
        } else {
            hour += 1;
        }
        j += 1;
    }
    1
}

//=============================================================================

fn readStdLine(prj: &mut Project, line: &[u8], day1: DateTime, day2: DateTime) -> i32
//
//  Input:   line = line of data from a standard rainfall data file
//           day1 = starting day of record of interest
//           day2 = ending day of record of interest
//  Output:  returns -1 if past end of desired record, 0 if data line could
//           not be read successfully or 1 if line read successfully
//  Purpose: reads a line of data from a standard rainfall data file and
//           writes its data to the rain interface file.
//
{
    let date1: DateTime;
    let date2: DateTime;
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hour: i32 = 0;
    let mut minute: i32 = 0;
    let mut x: f32 = 0.0;

    // --- parse data from input line
    if parseStdLine(prj, line, &mut year, &mut month, &mut day, &mut hour, &mut minute, &mut x) == 0 {
        return 0;
    }

    // --- see if date is within period of record requested
    date1 = datetime_encodeDate(year, month, day);
    if day1 != NO_DATE as f64 && date1 < day1 {
        return 0;
    }
    if day2 != NO_DATE as f64 && date1 > day2 {
        return -1;
    }

    // --- see if record is out of sequence
    date2 = date1 + datetime_encodeTime(hour, minute, 0.0);
    if date2 <= prj.rain.PreviousDate {
        let fname = prj.Gage[prj.rain.GageIndex as usize].fname;
        report_writeErrorMsg(prj, ERR_RAIN_FILE_SEQUENCE, &fname);
        report_writeLine(prj, cio::cstr(line));
        return -1;
    }
    prj.rain.PreviousDate = date2;

    match prj.rain.RainType {
        RAINFALL_INTENSITY => {
            x = x * prj.rain.Interval as f32 / 3600.0f32;
        }

        CUMULATIVE_RAINFALL => {
            if x >= prj.rain.RainAccum {
                x = x - prj.rain.RainAccum;
                prj.rain.RainAccum += x;
            } else {
                prj.rain.RainAccum = x;
            }
        }
        _ => {}
    }
    x *= prj.rain.UnitsFactor as f32;

    // --- save rainfall to binary interface file
    saveRainfall(prj, date1, hour, minute, x, FALSE as i8);
    1
}

//=============================================================================

fn parseStdLine(
    prj: &mut Project,
    line: &[u8],
    year: &mut i32,
    month: &mut i32,
    day: &mut i32,
    hour: &mut i32,
    minute: &mut i32,
    value: &mut f32,
) -> i32
//
//  Input:   line = line of data from a standard rainfall data file
//  Output:  *year = year when rainfall occurs
//           *month = month of year when rainfall occurs
//           *day = day of month when rainfall occurs
//           *hour = hour of day when rainfall occurs
//           *minute = minute of hour when rainfall occurs
//           *value = rainfall value (user units);
//           returns 0 if data line could not be parsed successfully or
//           1 if line parsed successfully
//  Purpose: parses a line of data from a standard rainfall data file.
//
{
    let n: i32;
    let mut token: Vec<u8> = Vec::new();

    n = cio::sscanf(
        line,
        b"%s %d %d %d %d %d %f",
        &mut [
            Scan::Str(&mut token),
            Scan::Int(year),
            Scan::Int(month),
            Scan::Int(day),
            Scan::Int(hour),
            Scan::Int(minute),
            Scan::Float(value),
        ],
    );
    if n < 7 {
        return 0;
    }
    if let Some(g) = prj.rain.StationID {
        if strcomp(&token, &prj.Gage[g].staID) == 0 {
            return 0;
        }
    }
    1
}

//=============================================================================

fn saveAccumRainfall(prj: &mut Project, date1: DateTime, hour: i32, minute: i32, v: i64)
//
//  Input:   date1 = date of latest rainfall reading (in DateTime format)
//           hour = hour of day of latest rain reading
//           minute = minute of hour of latest rain reading
//           v = accumulated rainfall reading in hundreths of inches
//  Output:  none
//  Purpose: divides accumulated rainfall evenly into individual recording
//           periods over the accumulation period and writes each period's
//           rainfall to the binary rainfall file.
//
{
    let mut date2: DateTime;
    let n: i32;
    let mut j: i32;
    let x: f32;

    // --- return if accumulated start date is missing
    if prj.rain.AccumStartDate == NO_DATE as f64 {
        return;
    }

    // --- find number of recording intervals over accumulation period
    date2 = date1 + datetime_encodeTime(hour, minute, 0.0);
    n = (datetime_timeDiff(date2, prj.rain.AccumStartDate) / prj.rain.Interval as i64 + 1) as i32;

    // --- update count of rain or missing periods
    if v == 99999 {
        prj.rain.RainStats.periodsMissing += n as i64;
        return;
    }
    prj.rain.RainStats.periodsRain += n as i64;

    // --- divide accumulated amount evenly into each period
    x = v as f32 / n as f32 / 100.0f32;

    // --- save this amount to file for each period
    if x > 0.0f32 {
        date2 = datetime_addSeconds(prj.rain.AccumStartDate, (-prj.rain.TimeOffset) as f64);
        if prj.rain.RainStats.startDate == NO_DATE as f64 {
            prj.rain.RainStats.startDate = date2;
        }
        j = 0;
        while j < n {
            prj.Frain.f().write_bytes(&date2.to_le_bytes());
            prj.Frain.f().write_bytes(&x.to_le_bytes());
            date2 = datetime_addSeconds(date2, prj.rain.Interval as f64);
            prj.rain.RainStats.endDate = date2;
            j += 1;
        }
    }

    // --- reset start of accumulation period
    prj.rain.AccumStartDate = NO_DATE as f64;
}

//=============================================================================

fn saveRainfall(prj: &mut Project, date1: DateTime, hour: i32, minute: i32, x: f32, isMissing: i8)
//
//  Input:   date1 = date of rainfall reading (in DateTime format)
//           hour = hour of day of current rain reading
//           minute = minute of hour of current rain reading
//           x = rainfall reading in inches
//           isMissing = TRUE if rainfall value is missing
//  Output:  none
//  Purpose: writes current rainfall reading from an external rainfall file
//           to project's binary rainfall file.
//
{
    let date2: DateTime;
    let seconds: i32;

    if isMissing != 0 {
        prj.rain.RainStats.periodsMissing += 1;
    } else {
        prj.rain.RainStats.periodsRain += 1;
    }

    // --- if rainfall not missing then save it to rainfall interface file
    if isMissing == 0 {
        seconds = 3600 * hour + 60 * minute - prj.rain.TimeOffset;
        date2 = datetime_addSeconds(date1, seconds as f64);

        // --- write date & value (in inches) to interface file
        prj.Frain.f().write_bytes(&date2.to_le_bytes());
        prj.Frain.f().write_bytes(&x.to_le_bytes());

        // --- update actual start & end of record dates
        if prj.rain.RainStats.startDate == NO_DATE as f64 {
            prj.rain.RainStats.startDate = date2;
        }
        prj.rain.RainStats.endDate = date2;
    }
}
//=============================================================================
