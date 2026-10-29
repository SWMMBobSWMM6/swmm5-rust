//! table.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   table.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     10/17/22   (Build 5.2.2)
//   Author:   L. Rossman
//
//   Table (curve and time series) functions.
//
//   Curve and Time Series objects in SWMM 5 are both modeled with
//   TTable data structures.
//
//   The table_getFirstEntry and table_getNextEntry functions, as well as the
//   Time Series functions that use them, are not thread safe.
//
//   Update History
//   ==============
//   Build 5.1.008:
//   - The lookup functions used for Curve tables (table_lookup, table_lookupEx,
//     table_intervalLookup, table_inverseLookup, table_getSlope, table_getMaxY,
//     table_getArea, and table_getInverseArea) were made thread-safe (thanks to
//     suggestions by CHI).
//   Build 5.2.0:
//   - First line of Curve's input data can contain just the curve name and type.
//   - The table_getArea function was renamed table_getStorageVolume and was
//   - refactored.
//   - The table_getInverseArea function was renamed table_getStorageDepth and
//     was refactored.
//   - Support added for relative file names.
//   Build 5.2.2:
//   - Prevent re-reading a time series file from start once end is reached.
//-----------------------------------------------------------------------------
//
// Translation notes:
// * A table's linked list of entries is `table.entries` in list order;
//   firstEntry is entries[0], lastEntry the last element, and the cursor
//   thisEntry an index. `entry->next` of element i is element i+1.
// * C dereferences a NULL thisEntry in table_getNextEntry (and a NULL
//   firstEntry in table_getMaxY) when a table has no entries; that crashes
//   EPA's engine. Here it panics.
// * Time series read from an external file parse dates with
//   datetime_strToDate, which reads datetime.c's DateFormat. The table
//   functions are not given the project, so they use M_D_Y: swmm_open sets
//   DateFormat to M_D_Y before anything is read, and no other code changes
//   it for good (datetime_getTimeStamp restores it before returning).

use crate::prelude::*;

/// The file-scope variables of table.c (none).
#[derive(Default)]
pub struct TableState {}

/// datetime.c's DateFormat while a project is open (see the note above).
const FILE_DATE_FORMAT: i32 = M_D_Y;

//=============================================================================

pub fn table_interpolate(x: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    //
    //  Input:   x = x value being interpolated
    //           x1, x2 = x values on either side of x
    //           y1, y2 = y values corrresponding to x1 and x2, respectively
    //  Output:  returns the y value corresponding to x
    //  Purpose: interpolates a y value for a given x value.
    //
    let dx: f64 = x2 - x1;
    if dx.abs() < 1.0e-20 {
        return (y1 + y2) / 2.0;
    }
    y1 + (x - x1) * (y2 - y1) / dx
}

//=============================================================================

pub fn table_readCurve(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns an error code
    //  Purpose: reads a tokenized line of data for a curve table.
    //
    let j: i32;
    let m: i32;
    let mut k: i32;
    let mut k1: i32 = 1;
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    // --- check for minimum number of tokens
    if ntoks < 2 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that curve exists in database
    j = project_findObject(prj, CURVE, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }

    // --- check if this is first line of curve's data
    //     (curve's ID will not have been assigned yet)
    if prj.Curve[j as usize].ID.is_empty() {
        // --- assign ID pointer & curve type
        prj.Curve[j as usize].ID = project_findID(prj, CURVE, &tok[0]).unwrap_or_default();
        m = findmatch(&tok[1], CurveTypeWords);
        if m < 0 {
            return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
        }
        prj.Curve[j as usize].curveType = m;
        if ntoks == 2 {
            return 0;
        }
        k1 = 2;
    }

    // --- start reading pairs of X-Y value tokens
    k = k1;
    while k < ntoks {
        if k + 1 >= ntoks {
            return error_setInpError(prj, ERR_ITEMS, b"");
        }
        if getDouble(&tok[k as usize], &mut x) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[k as usize]);
        }
        if getDouble(&tok[(k + 1) as usize], &mut y) == 0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[(k + 1) as usize]);
        }
        table_addEntry(&mut prj.Curve[j as usize], x, y);
        k = k + 2;
    }
    0
}

//=============================================================================

pub fn table_readTimeseries(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  returns an error code
    //  Purpose: reads a tokenized line of data for a time series table.
    //
    let j: i32; // time series index
    let mut k: i32; // token index
    let mut state: i32; // 1: next token should be a date
                        // 2: next token should be a time
                        // 3: next token should be a value
    let mut x: f64; // time & value table entries
    let mut y: f64 = 0.0;
    let mut d: DateTime = 0.0; // day portion of date/time value
    let mut t: DateTime = 0.0; // time portion of date/time value
    let mut fname: [u8; (MAXFNAME + 1) as usize] = [0; (MAXFNAME + 1) as usize];

    // --- check for minimum number of tokens
    if ntoks < 3 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that time series exists in database
    j = project_findObject(prj, TSERIES, &tok[0]);
    if j < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    let ju = j as usize;

    // --- if first line of data, assign ID pointer
    if prj.Tseries[ju].ID.is_empty() {
        prj.Tseries[ju].ID = project_findID(prj, TSERIES, &tok[0]).unwrap_or_default();
    }

    // --- check if time series data is in an external file
    if strcomp(&tok[1], w_FILE) != 0 {
        cio::sstrncpy(&mut fname, &tok[2], MAXFNAME as usize);
        addAbsolutePath(prj, &mut fname);
        cio::sstrncpy(&mut prj.Tseries[ju].file.name, &fname, MAXFNAME as usize);
        prj.Tseries[ju].file.mode = USE_FILE as i8;
        return 0;
    }

    // --- parse each token of input line
    x = 0.0;
    k = 1;
    state = 1; // start off looking for a date
    while k < ntoks {
        match state {
            1 => {
                // look for a date entry
                if datetime_strToDate(prj, &tok[k as usize], &mut d) != 0 {
                    prj.Tseries[ju].lastDate = d;
                    k += 1;
                }

                // --- next token must be a time
                state = 2;
            }

            2 => {
                // look for a time entry
                if k >= ntoks {
                    return error_setInpError(prj, ERR_ITEMS, b"");
                }

                // --- first check for decimal hours format
                if getDouble(&tok[k as usize], &mut t) != 0 {
                    t /= 24.0;
                }
                // --- then for an hrs:min format
                else if datetime_strToTime(&tok[k as usize], &mut t) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[k as usize]);
                }

                // --- save date + time in x
                x = prj.Tseries[ju].lastDate + t;

                // --- next token must be a numeric value
                k += 1;
                state = 3;
            }

            3 => {
                // --- extract a numeric value from token
                if k >= ntoks {
                    return error_setInpError(prj, ERR_ITEMS, b"");
                }
                if getDouble(&tok[k as usize], &mut y) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[k as usize]);
                }

                // --- add date/time & value to time series
                table_addEntry(&mut prj.Tseries[ju], x, y);

                // --- start over looking first for a date
                k += 1;
                state = 1;
            }
            _ => {}
        }
    }
    0
}

//=============================================================================

pub fn table_addEntry(table: &mut TTable, x: f64, y: f64) -> i32 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = x value
    //           y = y value
    //  Output:  returns TRUE if successful, FALSE if not
    //  Purpose: adds a new x/y entry to a table.
    //
    table.entries.push(TTableEntry { x, y });
    TRUE
}

//=============================================================================

pub fn table_deleteEntries(table: &mut TTable) {
    //
    //  Input:   table = pointer to a TTable structure
    //  Output:  none
    //  Purpose: deletes all x/y entries in a table.
    //
    table.entries.clear();
    table.thisEntry = None;

    if table.file.file.is_some() {
        table.file.file = None;
    }
}

//=============================================================================

pub fn table_init(table: &mut TTable) {
    //
    //  Input:   table = pointer to a TTable structure
    //  Output:  none
    //  Purpose: initializes properties when table is first created.
    //
    table.ID = Vec::new();
    table.refersTo = -1;
    table.entries = Vec::new();
    table.thisEntry = None;
    table.lastDate = 0.0;
    table.x1 = 0.0;
    table.x2 = 0.0;
    table.y1 = 0.0;
    table.y2 = 0.0;
    table.dxMin = 0.0;
    table.file.mode = NO_FILE as i8;
    table.file.file = None;
    table.curveType = -1;
}

//=============================================================================

pub fn table_validate(table: &mut TTable) -> i32 {
    //
    //  Input:   table = pointer to a TTable structure
    //  Output:  returns error code
    //  Purpose: checks that table's x-values are in ascending order.
    //
    let result: i32;
    let mut x1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.0;
    let mut dx: f64;
    let mut dxMin: f64 = BIG;

    // --- open external file if used as the table's data source
    if table.file.mode as i32 == USE_FILE {
        table.file.file = CFile::open(&table.file.name, "rt");
        if table.file.file.is_none() {
            return ERR_TABLE_FILE_OPEN;
        }
    }

    // --- retrieve the first data entry in the table
    result = table_getFirstEntry(table, &mut x1, &mut y1);

    // --- return error condition if external file has no valid data
    if result == 0 && table.file.mode as i32 == USE_FILE {
        return ERR_TABLE_FILE_READ;
    }

    // --- retrieve successive table entries and check for non-increasing x-values
    while table_getNextEntry(table, &mut x2, &mut y2) != 0 {
        dx = x2 - x1;
        if dx <= 0.0 {
            table.x2 = x2;
            return ERR_CURVE_SEQUENCE;
        }
        dxMin = MIN(dxMin, dx);
        x1 = x2;
    }
    table.dxMin = dxMin;

    // --- return error if external file could not be read completely
    if table.file.mode as i32 == USE_FILE && !table.file.f().feof() {
        return ERR_TABLE_FILE_READ;
    }
    0
}

//=============================================================================

pub fn table_getFirstEntry(table: &mut TTable, x: &mut f64, y: &mut f64) -> i32 {
    //
    //  Input:   table = pointer to a TTable structure
    //  Output:  x = x-value of first table entry
    //           y = y-value of first table entry
    //           returns TRUE if successful, FALSE if not
    //  Purpose: retrieves the first x/y entry in a table.
    //
    //  NOTE: also moves the current position pointer (thisEntry) to the 1st entry.
    //
    *x = 0.0;
    *y = 0.0;

    if table.file.mode as i32 == USE_FILE {
        match table.file.file.as_mut() {
            None => return FALSE,
            Some(f) => f.rewind(),
        }
        return table_getNextFileEntry(table, x, y);
    }

    if !table.entries.is_empty() {
        let entry = table.entries[0];
        *x = entry.x;
        *y = entry.y;
        table.thisEntry = Some(0);
        TRUE
    } else {
        FALSE
    }
}

//=============================================================================

pub fn table_getNextEntry(table: &mut TTable, x: &mut f64, y: &mut f64) -> i32 {
    //
    //  Input:   table = pointer to a TTable structure
    //  Output:  x = x-value of next table entry
    //           y = y-value of next table entry
    //           returns TRUE if successful, FALSE if not
    //  Purpose: retrieves the next x/y entry in a table.
    //
    //  NOTE: also updates the current position pointer (thisEntry).
    //
    if table.file.mode as i32 == USE_FILE {
        return table_getNextFileEntry(table, x, y);
    }

    // entry = table->thisEntry->next;  (C dereferences NULL if thisEntry is NULL)
    let this = table.thisEntry.expect("table_getNextEntry: thisEntry is NULL (C dereferences a NULL pointer)");
    let next = this + 1;
    if next < table.entries.len() {
        let entry = table.entries[next];
        *x = entry.x;
        *y = entry.y;
        table.thisEntry = Some(next);
        TRUE
    } else {
        FALSE
    }
}

//=============================================================================

pub fn table_lookup(table: &mut TTable, x: f64) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = an x-value
    //  Output:  returns a y-value
    //  Purpose: retrieves the y-value corresponding to an x-value in a table,
    //           using interploation if necessary.
    //
    //  NOTE: if x is below the first table entry, then the first y-value is
    //        returned; if x is above the last entry, then the last y-value is
    //        returned.
    //
    let mut x1: f64;
    let mut y1: f64;
    let mut x2: f64;
    let mut y2: f64;
    let entries = &table.entries;

    if entries.is_empty() {
        return 0.0;
    }
    x1 = entries[0].x;
    y1 = entries[0].y;
    if x <= x1 {
        return y1;
    }
    for entry in entries[1..].iter() {
        x2 = entry.x;
        y2 = entry.y;
        if x <= x2 {
            return table_interpolate(x, x1, y1, x2, y2);
        }
        x1 = x2;
        y1 = y2;
    }
    y1
}

//=============================================================================

pub fn table_getSlope(table: &mut TTable, x: f64) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = an x-value
    //  Output:  returns the slope of the curve at x
    //  Purpose: retrieves the slope of the curve at the line segment containing x.
    //
    let mut x1: f64;
    let mut y1: f64;
    let mut x2: f64;
    let mut y2: f64;
    let dx: f64;
    let entries = &table.entries;

    if entries.is_empty() {
        return 0.0;
    }
    x1 = entries[0].x;
    y1 = entries[0].y;
    x2 = x1;
    y2 = y1;
    for entry in entries[1..].iter() {
        x2 = entry.x;
        y2 = entry.y;
        if x <= x2 {
            break;
        }
        x1 = x2;
        y1 = y2;
    }
    dx = x2 - x1;
    if dx == 0.0 {
        return 0.0;
    }
    (y2 - y1) / dx
}

//=============================================================================

pub fn table_lookupEx(table: &mut TTable, x: f64) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = an x-value
    //  Output:  returns a y-value
    //  Purpose: retrieves the y-value corresponding to an x-value in a table,
    //           using interploation if necessary within the table and linear
    //           extrapolation outside of the table.
    //
    let mut x1: f64;
    let mut y1: f64;
    let mut x2: f64;
    let mut y2: f64;
    let mut s: f64 = 0.0;
    let entries = &table.entries;

    if entries.is_empty() {
        return 0.0;
    }
    x1 = entries[0].x;
    y1 = entries[0].y;
    if x <= x1 {
        if x1 > 0.0 {
            return x / x1 * y1;
        } else {
            return y1;
        }
    }
    for entry in entries[1..].iter() {
        x2 = entry.x;
        y2 = entry.y;
        if x2 != x1 {
            s = (y2 - y1) / (x2 - x1);
        }
        if x <= x2 {
            return table_interpolate(x, x1, y1, x2, y2);
        }
        x1 = x2;
        y1 = y2;
    }
    if s < 0.0 {
        s = 0.0;
    }
    y1 + s * (x - x1)
}

//=============================================================================

pub fn table_intervalLookup(table: &mut TTable, x: f64) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = an x-value
    //  Output:  returns a y-value
    //  Purpose: retrieves the y-value corresponding to the first table entry
    //           whose x-value is > x.
    //
    let entries = &table.entries;

    if entries.is_empty() {
        return 0.0;
    }
    if x < entries[0].x {
        return entries[0].y;
    }
    for entry in entries[1..].iter() {
        if x < entry.x {
            return entry.y;
        }
    }
    entries[entries.len() - 1].y
}

//=============================================================================

pub fn table_inverseLookup(table: &mut TTable, y: f64) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           y = a y-value
    //  Output:  returns an x-value
    //  Purpose: retrieves the x-value corresponding to an y-value in a table,
    //           using interploation if necessary.
    //
    //  NOTE: if y is below the first table entry, then the first x-value is
    //        returned; if y is above the last entry, then the last x-value is
    //        returned.
    //
    let mut x1: f64;
    let mut y1: f64;
    let mut x2: f64;
    let mut y2: f64;
    let entries = &table.entries;

    if entries.is_empty() {
        return 0.0;
    }
    x1 = entries[0].x;
    y1 = entries[0].y;
    if y <= y1 {
        return x1;
    }
    for entry in entries[1..].iter() {
        x2 = entry.x;
        y2 = entry.y;
        if y <= y2 {
            return table_interpolate(y, y1, x1, y2, x2);
        }
        x1 = x2;
        y1 = y2;
    }
    x1
}

//=============================================================================

pub fn table_getMaxY(table: &mut TTable, x: f64) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = an x-value
    //  Output:  returns the maximum y-value for x-values below x.
    //  Purpose: finds the largest y value in the initial non-decreasing
    //           portion of a table that appear before value x.
    //
    let mut ymax: f64;
    let entries = &table.entries;
    let mut entry: usize = 0;

    // (C dereferences a NULL firstEntry for an empty table)
    ymax = entries.first().expect("table_getMaxY: empty table (C dereferences a NULL pointer)").y;
    while x > entries[entry].x && entry + 1 < entries.len() {
        entry += 1;
        if entries[entry].y < ymax {
            return ymax;
        }
        ymax = entries[entry].y;
    }
    0.0
}

//=============================================================================

pub fn table_getStorageVolume(table: &mut TTable, x: f64) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = a depth value
    //  Output:  returns a storage volume
    //  Purpose: finds volume for a given depth in a Storage Curve table.
    //
    let mut a: f64;
    let mut a1: f64;
    let mut x1: f64;
    let mut v: f64;
    let mut dx: f64 = 0.0;
    let mut dy: f64 = 0.0;
    let s: f64;
    let entries = &table.entries;

    // --- get first entry in table
    v = 0.0;
    if entries.is_empty() {
        return 0.0;
    }
    x1 = entries[0].x;
    a1 = entries[0].y;

    // --- target depth is below first tabulated depth
    if x <= x1 {
        if x1 < 1.0e-6 {
            return 0.0;
        }
        return (a1 / x1) * x * x / 2.0;
    }

    // --- otherwise traverse table entries until target depth is bracketed
    for entry in entries[1..].iter() {
        // --- target is bracketed - apply end area method to interpolated area
        if entry.x >= x {
            a = table_interpolate(x, x1, a1, entry.x, entry.y);
            return v + (a1 + a) / 2.0 * (x - x1);
        }
        // --- target not yet bracketed so update volume using end area method
        else {
            dx = entry.x - x1;
            dy = entry.y - a1;
            v = v + (a1 + entry.y) / 2.0 * dx;
            x1 = entry.x;
            a1 = entry.y;
        }
    }

    // --- extrapolate area if table limit exceeded
    if dx > 1.0e-6 {
        s = dy / dx;
        a = a1 + s * (x - x1);
        // --- don't extrapolate below 0 in case s is negative
        if a < 0.0 {
            v = v - a1 * a1 / s / 2.0;
        }
        // --- apply end area method to extrapolated area
        else {
            v = v + (a1 + a) / 2.0 * (x - x1);
        }
    }
    v
}

//=============================================================================

pub fn table_getStorageDepth(table: &mut TTable, v: f64) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           v = a storage volume
    //  Output:  returns a storage depth
    //  Purpose: finds depth for a given volume in a Storage Curve table.
    //
    let mut a1: f64;
    let mut a2: f64;
    let mut d1: f64;
    let mut d2: f64;
    let mut dd: f64 = 0.0;
    let mut da: f64 = 0.0;
    let mut v1: f64;
    let mut v2: f64;
    let mut s: f64;
    let entries = &table.entries;

    // --- see if target volume is below that of 1st table entry
    if v == 0.0 {
        return 0.0;
    }
    if entries.is_empty() {
        return 0.0;
    }
    d1 = entries[0].x;
    a1 = entries[0].y;
    v1 = a1 * d1 / 2.0;
    if v <= v1 {
        if a1 > 0.0 {
            return (2.0 * v * d1 / a1).sqrt();
        } else {
            return 0.0;
        }
    }

    // --- add next table entry to volume until target volume is bracketed
    for entry in entries[1..].iter() {
        d2 = entry.x;
        a2 = entry.y;
        dd = d2 - d1;
        da = a2 - a1;
        v2 = v1 + (a1 + a2) / 2.0 * dd;

        // target volume is bracketed
        if v <= v2 {
            // --- target coincides with point on curve
            if dd <= 0.0 {
                return d1;
            }
            if da == 0.0 {
                if (v2 - v1).abs() < 1.0e-6 {
                    return d1;
                } else {
                    return d1 + dd * (v - v1) / (v2 - v1);
                }
            }
            // --- if area decreases with depth then replace point 1 with point 2
            if da < 0.0 {
                d1 = d2;
                a1 = a2;
                v1 = v2;
            }
            // --- interpolate between volumes derived from curve
            s = da / dd;
            return d1 + ((a1 * a1 + 2.0 * s * (v - v1)).sqrt() - a1) / s;
        }

        // --- replace point 1 with point 2
        d1 = d2;
        a1 = a2;
        v1 = v2;
    }

    // --- extrapolate volume if table limit exceeded
    if dd == 0.0 || da == 0.0 {
        if a1 > 0.0 {
            dd = (v - v1) / a1;
        } else {
            dd = 0.0;
        }
    } else {
        s = da / dd;
        dd = ((a1 * a1 + 2.0 * s * (v - v1)).sqrt() - a1) / s;
        if dd < 0.0 {
            dd = 0.0;
        }
    }
    d1 + dd
}

//=============================================================================

pub fn table_tseriesInit(table: &mut TTable) {
    //
    //  Input:   table = pointer to a TTable structure
    //  Output:  none
    //  Purpose: initializes the time bracket within a time series table.
    //
    // (C passes &table->x1 etc.; the callee never reads those fields
    //  through `table`, so copying in and out is the same.)
    let mut x1 = table.x1;
    let mut y1 = table.y1;
    table_getFirstEntry(table, &mut x1, &mut y1);
    table.x1 = x1;
    table.y1 = y1;
    table.x2 = table.x1;
    table.y2 = table.y1;
    let mut x2 = table.x2;
    let mut y2 = table.y2;
    table_getNextEntry(table, &mut x2, &mut y2);
    table.x2 = x2;
    table.y2 = y2;
}

//=============================================================================

pub fn table_tseriesLookup(table: &mut TTable, x: f64, extend: i8) -> f64 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = a date/time value
    //           extend = TRUE if time series extended on either end
    //  Output:  returns a y-value
    //  Purpose: retrieves the y-value corresponding to a time series date,
    //           using interploation if necessary.
    //
    //  NOTE: if extend is FALSE and date x is outside the range of the table
    //        then 0 is returned; if TRUE then the first or last value is
    //        returned.
    //
    // --- x lies within current time bracket
    if table.x1 <= x && table.x2 >= x && table.x1 != table.x2 {
        return table_interpolate(x, table.x1, table.y1, table.x2, table.y2);
    }

    // --- end of external time series file has been reached
    if table.file.mode as i32 == USE_FILE && table.file.f().feof() {
        if extend as i32 == TRUE {
            return table.y1;
        } else {
            return 0.0;
        }
    }

    // --- x lies before current time bracket:
    //     move to start of time series
    if table.x1 == table.x2 || x < table.x1 {
        let mut x1 = table.x1;
        let mut y1 = table.y1;
        table_getFirstEntry(table, &mut x1, &mut y1);
        table.x1 = x1;
        table.y1 = y1;
        if x < table.x1 {
            if extend as i32 == TRUE {
                return table.y1;
            } else {
                return 0.0;
            }
        }
    }

    // --- x lies beyond current time bracket:
    //     update start of next time bracket
    table.x1 = table.x2;
    table.y1 = table.y2;

    // --- get end of next time bracket
    loop {
        let mut x2 = table.x2;
        let mut y2 = table.y2;
        let found = table_getNextEntry(table, &mut x2, &mut y2);
        table.x2 = x2;
        table.y2 = y2;
        if found == 0 {
            break;
        }
        // --- x lies within the bracket
        if x <= table.x2 {
            return table_interpolate(x, table.x1, table.y1, table.x2, table.y2);
        }
        // --- otherwise move to next time bracket
        table.x1 = table.x2;
        table.y1 = table.y2;
    }

    // --- return last value or 0 if beyond last data value
    if extend as i32 == TRUE {
        table.y1
    } else {
        0.0
    }
}

//=============================================================================

fn table_getNextFileEntry(table: &mut TTable, x: &mut f64, y: &mut f64) -> i32 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = pointer to a date (as decimal days)
    //           y = pointer to a time series value
    //  Output:  updates values of x and y;
    //           returns TRUE if successful, FALSE if not
    //  Purpose: retrieves the next date and value for a time series
    //           table stored in an external file.
    //
    let mut code: i32;
    if table.file.file.is_none() {
        return FALSE;
    }
    loop {
        let f = table.file.f();
        if f.feof() {
            break;
        }
        let mut line = match f.fgets(MAXLINE as usize) {
            None => break,
            Some(l) => l,
        };
        code = table_parseFileLine(&mut line, table, x, y);
        if code < 0 {
            continue; //skip blank & comment lines
        }
        return code;
    }
    FALSE
}

//=============================================================================

fn table_parseFileLine(line: &mut Vec<u8>, table: &mut TTable, x: &mut f64, y: &mut f64) -> i32 {
    //
    //  Input:   table = pointer to a TTable structure
    //           x = pointer to a date (as decimal days)
    //           y = pointer to a time series value
    //  Output:  updates values of x and y;
    //           returns -1 if line was a comment,
    //           TRUE if line successfully parsed,
    //           FALSE if line could not be parsed
    //  Purpose: parses a line of time series data from an external file.
    //
    let n: i32;
    // NOTE: C's s1, s2, s3 are char[50]; sscanf overflows them for a token
    // of 50+ characters (undefined). Here they hold the whole token.
    let mut s1: Vec<u8> = Vec::new();
    let mut s2: Vec<u8> = Vec::new();
    let mut s3: Vec<u8> = Vec::new();
    let tStr: &[u8]; // time as string
    let yStr: &[u8]; // value as string
    let mut yy: f64 = 0.0; // value as double
    let mut d: DateTime = 0.0; // day portion of date/time value
    let mut t: DateTime = 0.0; // time portion of date/time value

    // --- get 3 string tokens from line and check if its a comment
    n = cio::sscanf(line, b"%s %s %s", &mut [Scan::Str(&mut s1), Scan::Str(&mut s2), Scan::Str(&mut s3)]);

    // --- return if line is blank or is a comment
    // tStr = strtok(line, SEPSTR);
    // if ( tStr == NULL || *tStr == ';' ) return -1;
    {
        let l = cio::cstr(line);
        let mut i = 0;
        while i < l.len() && SEPSTR.contains(&l[i]) {
            i += 1;
        }
        if i >= l.len() || l[i] == b';' {
            return -1;
        }
    }

    // --- line only has a time and a value
    if n == 2 {
        // --- calendar date is same as last recorded date
        d = table.lastDate;
        tStr = &s1;
        yStr = &s2;
    }
    // --- line has date, time and a value
    else if n == 3 {
        // --- convert date string to numeric value
        if crate::datetime::strToDateFmt(FILE_DATE_FORMAT, &s1, &mut d) == 0 {
            return FALSE;
        }

        // --- update last recorded calendar date
        table.lastDate = d;
        tStr = &s2;
        yStr = &s3;
    } else {
        return FALSE;
    }

    // --- convert time string to numeric value
    if getDouble(tStr, &mut t) != 0 {
        t /= 24.0;
    } else if datetime_strToTime(tStr, &mut t) == 0 {
        return FALSE;
    }

    // --- convert value string to numeric value
    if getDouble(yStr, &mut yy) == 0 {
        return FALSE;
    }

    // --- assign values to current date and value
    *x = d + t;
    *y = yy;
    TRUE
}
