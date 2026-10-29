//! transect.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   transect.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     07/13/23   (Build 5.2.4)
//   Author:   L. Rossman
//
//   Geometry processing for irregular cross-section transects.
//
//   Update History
//   ==============
//   Build 5.2.0:
//   - Function added to create a transect for a Street cross-section.
//   Build 5.2.4:
//   - Corrected street transect points in transect_createStreetTransect.
//-----------------------------------------------------------------------------
//
// Translation notes:
// * The file-scope variables live in TransectState (prj.transect). They
//   persist from one transect (or street) to the next, as in C: getFlow can
//   read Station[k+1] one past the current transect's last station, which
//   holds whatever an earlier transect or street left there.
// * Station and Elev have one extra element beyond C's MAXSTATION+1 so that
//   C's one-past-the-end read Station[MAXSTATION+1] (possible only for a
//   transect of exactly MAXSTATION-1 stations) reads 0 instead of panicking.

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
const MAXSTATION: i32 = 1500; // max. number of stations in a transect

//-----------------------------------------------------------------------------
//  Shared variables
//-----------------------------------------------------------------------------
/// The file-scope variables of transect.c.
pub struct TransectState {
    pub Ntransects: i32,   // total number of transects
    pub Nstations: i32,    // number of stations in current transect
    pub Station: Vec<f64>, // x-coordinate of each station
    pub Elev: Vec<f64>,    // elevation of each station
    pub Nleft: f64,        // Manning's n for left overbank
    pub Nright: f64,       // Manning's n for right overbank
    pub Nchannel: f64,     // Manning's n for main channel
    pub Xleftbank: f64,    // station where left overbank ends
    pub Xrightbank: f64,   // station where right overbank begins
    pub Xfactor: f64,      // multiplier for station spacing
    pub Yfactor: f64,      // factor added to station elevations
    pub Lfactor: f64,      // main channel/flood plain length
}
impl Default for TransectState {
    fn default() -> Self {
        TransectState {
            Ntransects: 0,
            Nstations: 0,
            Station: vec![0.0; (MAXSTATION + 2) as usize],
            Elev: vec![0.0; (MAXSTATION + 2) as usize],
            Nleft: 0.0,
            Nright: 0.0,
            Nchannel: 0.0,
            Xleftbank: 0.0,
            Xrightbank: 0.0,
            Xfactor: 0.0,
            Yfactor: 0.0,
            Lfactor: 0.0,
        }
    }
}

//=============================================================================

pub fn transect_create(prj: &mut Project, n: i32) -> i32 {
    //
    //  Input:   n = number of transect objects to create
    //  Output:  returns an error code
    //  Purpose: creates an array of cross-section transects.
    //
    prj.transect.Ntransects = n;
    if n == 0 {
        return 0;
    }
    prj.Transect = vec![TTransect::default(); prj.transect.Ntransects as usize];
    prj.transect.Nchannel = 0.0;
    prj.transect.Nleft = 0.0;
    prj.transect.Nright = 0.0;
    prj.transect.Nstations = 0;
    0
}

//=============================================================================

pub fn transect_delete(prj: &mut Project) {
    //
    //  Input:   none
    //  Output:  none
    //  Purpose: deletes memory allocated for all transects.
    //
    if prj.transect.Ntransects == 0 {
        return;
    }
    prj.Transect = Vec::new();
    prj.transect.Ntransects = 0;
}

//=============================================================================

pub fn transect_readParams(prj: &mut Project, count: &mut i32, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Input:   count = transect index
    //           tok[] = array of string tokens
    //           ntoks = number of tokens
    //  Output:  updated value of count,
    //           returns an error code
    //  Purpose: read parameters of a transect from a tokenized line of input data.
    //
    //  Format of transect data follows that used for HEC-2 program:
    //    NC  nLeft  nRight  nChannel
    //    X1  name  nSta  xLeftBank  xRightBank  0  0  0  xFactor  yFactor
    //    GR  Elevation  Station  ...
    //
    let mut i: i32;
    let k: i32;
    let index: i32 = *count; // transect index
    let mut errcode: i32; // error code
    let mut x: [f64; 10] = [0.0; 10]; // parameter values
    let id: Vec<u8>; // transect ID name

    // --- match first token to a transect keyword
    k = findmatch(&tok[0], TransectKeyWords);
    if k < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[0]);
    }

    // --- read parameters associated with keyword
    match k {
        // --- NC line: Manning n values
        0 => {
            // --- finish processing the previous transect
            transect_validate(prj, index - 1);

            // --- read Manning's n values
            if ntoks < 4 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            i = 1;
            while i <= 3 {
                if getDouble(&tok[i as usize], &mut x[i as usize]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
                }
                i += 1;
            }
            return setManning(prj, &x);
        }

        // --- X1 line: identifies start of next transect
        1 => {
            // --- check that transect was already added to project
            //     (by input_countObjects)
            if ntoks < 10 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            id = match project_findID(prj, TRANSECT, &tok[1]) {
                None => return error_setInpError(prj, ERR_NAME, &tok[1]),
                Some(id) => id,
            };

            // --- read in rest of numerical values on data line
            i = 2;
            while i < 10 {
                if getDouble(&tok[i as usize], &mut x[i as usize]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
                }
                i += 1;
            }

            // --- update total transect count
            *count = index + 1;

            // --- transfer parameter values to transect's properties
            return setParams(prj, index, id, &x);
        }

        // --- GR line: station elevation & location data
        2 => {
            // --- check that line contains pairs of data values
            if (ntoks - 1) % 2 > 0 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }

            // --- parse each pair of Elevation-Station values
            i = 1;
            while i < ntoks {
                if getDouble(&tok[i as usize], &mut x[1]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[i as usize]);
                }
                if getDouble(&tok[(i + 1) as usize], &mut x[2]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[(i + 1) as usize]);
                }
                errcode = addStation(prj, x[1], x[2]);
                if errcode != 0 {
                    return errcode;
                }
                i += 2;
            }
            return 0;
        }
        _ => {}
    }
    0
}

//=============================================================================

pub fn transect_validate(prj: &mut Project, j: i32) {
    //
    //  Input:   j = transect index
    //  Output:  none
    //  Purpose: validates transect data and creates its geometry tables.
    //
    let mut i: i32;
    let mut ymin: f64;
    let mut ymax: f64;
    let oldNchannel: f64 = prj.transect.Nchannel;

    // --- check for valid transect data
    if j < 0 || j >= prj.transect.Ntransects {
        return;
    }
    let ju = j as usize;
    if prj.transect.Nstations < 2 {
        let id = prj.Transect[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_TRANSECT_TOO_FEW, &id);
        return;
    }
    if prj.transect.Nstations >= MAXSTATION {
        let id = prj.Transect[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_TRANSECT_TOO_MANY, &id);
        return;
    }
    if prj.transect.Nchannel <= 0.0 {
        let id = prj.Transect[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_TRANSECT_MANNING, &id);
        return;
    }
    if prj.transect.Xleftbank > prj.transect.Xrightbank {
        let id = prj.Transect[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_TRANSECT_OVERBANK, &id);
        return;
    }

    // --- adjust main channel's Mannings n to make its equivalent
    //     length equal to that of entire flood plain
    prj.transect.Nchannel = prj.transect.Nchannel * prj.transect.Lfactor.sqrt();
    prj.Transect[ju].lengthFactor = prj.transect.Lfactor;

    // --- find max. depth across transect
    {
        let st = &prj.transect;
        ymax = st.Elev[1];
        ymin = st.Elev[1];
        i = 2;
        while i <= st.Nstations {
            ymax = MAX(st.Elev[i as usize], ymax);
            ymin = MIN(st.Elev[i as usize], ymin);
            i += 1;
        }
    }
    if ymin >= ymax {
        let id = prj.Transect[ju].ID.clone();
        report_writeErrorMsg(prj, ERR_TRANSECT_NO_DEPTH, &id);
        return;
    }

    // --- add vertical sides to transect to reach full ht. on both ends
    {
        let st = &mut prj.transect;
        st.Station[0] = st.Station[1];
        st.Elev[0] = ymax;
        st.Nstations += 1;
        let ns = st.Nstations as usize;
        st.Station[ns] = st.Station[ns - 1];
        st.Elev[ns] = st.Elev[0];
    }

    // --- create geometry tables
    prj.Transect[ju].nTbl = N_TRANSECT_TBL;
    createTables(&prj.transect, &mut prj.Transect[ju], ymin, ymax);

    // --- save unadjusted main channel roughness
    prj.Transect[ju].roughness = oldNchannel;
}

//=============================================================================

fn createTables(st: &TransectState, transect: &mut TTransect, ymin: f64, ymax: f64) {
    let mut i: i32;
    let nLast: i32;
    let dy: f64;
    let mut y: f64;

    transect.yFull = ymax - ymin;
    transect.wMax = 0.0;

    // --- set 1st table entries to zero
    transect.areaTbl[0] = 0.0;
    transect.hradTbl[0] = 0.0;
    transect.widthTbl[0] = 0.0;

    // --- compute geometry for each depth increment
    dy = (ymax - ymin) / (transect.nTbl as f64 - 1.0);
    y = ymin;
    i = 1;
    while i < transect.nTbl {
        y += dy;
        transect.areaTbl[i as usize] = 0.0;
        transect.hradTbl[i as usize] = 0.0;
        transect.widthTbl[i as usize] = 0.0;
        getGeometry(st, transect, i, y);
        i += 1;
    }

    // --- determine max. section factor
    setMaxSectionFactor(transect);

    // --- normalize geometry table entries
    //     (full cross-section values are last table entries)
    nLast = transect.nTbl - 1;
    transect.aFull = transect.areaTbl[nLast as usize];
    transect.rFull = transect.hradTbl[nLast as usize];
    transect.wMax = transect.widthTbl[nLast as usize];

    i = 1;
    while i <= nLast {
        transect.areaTbl[i as usize] /= transect.aFull;
        transect.hradTbl[i as usize] /= transect.rFull;
        transect.widthTbl[i as usize] /= transect.wMax;
        i += 1;
    }

    // --- set width at 0 height equal to width at 4% of max. height
    transect.widthTbl[0] = transect.widthTbl[1];
}

//=============================================================================

fn setManning(prj: &mut Project, n: &[f64]) -> i32 {
    //
    //  Input:   n[] = array of Manning's n values
    //  Output:  returns an error code
    //  Purpose: sets Manning's n for overbanks and main channel of a transect.
    //
    let st = &mut prj.transect;
    let mut i: i32;
    i = 1;
    while i <= 3 {
        if n[i as usize] < 0.0 {
            return ERR_NUMBER;
        }
        i += 1;
    }
    if n[1] > 0.0 {
        st.Nleft = n[1];
    }
    if n[2] > 0.0 {
        st.Nright = n[2];
    }
    if n[3] > 0.0 {
        st.Nchannel = n[3];
    }
    if st.Nleft == 0.0 {
        st.Nleft = st.Nchannel;
    }
    if st.Nright == 0.0 {
        st.Nright = st.Nchannel;
    }
    0
}

//=============================================================================

fn setParams(prj: &mut Project, j: i32, id: Vec<u8>, x: &[f64]) -> i32 {
    //
    //  Input:   j = transect index
    //           id = transect ID name
    //           x[] = array of parameter values
    //  Output:  returns an error code
    //  Purpose: assigns parameter values to current transect being processed.
    //
    if j < 0 || j >= prj.transect.Ntransects {
        return ERR_NUMBER;
    }
    prj.Transect[j as usize].ID = id; // ID name
    let ucf = UCF(prj, LENGTH); // (UCF only reads the unit system)
    let st = &mut prj.transect;
    st.Xleftbank = x[3] / ucf; // left overbank location
    st.Xrightbank = x[4] / ucf; // right overbank location
    st.Lfactor = x[7]; // channel/bank length
    if st.Lfactor == 0.0 {
        st.Lfactor = 1.0;
    }
    st.Xfactor = x[8]; // station location multiplier
    if st.Xfactor == 0.0 {
        st.Xfactor = 1.0;
    }
    st.Xleftbank *= st.Xfactor; // adjusted left bank
    st.Xrightbank *= st.Xfactor; // adjusted right bank
    st.Yfactor = x[9] / ucf; // elevation offset
    st.Nstations = 0;
    0
}

//=============================================================================

fn addStation(prj: &mut Project, y: f64, x: f64) -> i32 {
    //
    //  Input:   y = station elevation value
    //           x = station distance value
    //  Output:  returns an error code
    //  Purpose: adds a new station to the transect currently being processed.
    //
    // --- check for valid number of stations
    if prj.transect.Nstations < 0 {
        return ERR_TRANSECT_UNKNOWN;
    }
    prj.transect.Nstations += 1;
    if prj.transect.Nstations >= MAXSTATION {
        return 0;
    }
    let ns = prj.transect.Nstations as usize;

    // --- add station distance, modified by distance multiplier
    let ucf = UCF(prj, LENGTH);
    prj.transect.Station[ns] = x * prj.transect.Xfactor / ucf;

    // --- add station elevation, modified by offset elevation
    prj.transect.Elev[ns] = (y + prj.transect.Yfactor) / ucf;

    // --- check if station distances are non-increasing
    if prj.transect.Nstations > 1 && prj.transect.Station[ns] < prj.transect.Station[ns - 1] {
        return ERR_TRANSECT_SEQUENCE;
    }
    0
}

//=============================================================================

fn getGeometry(st: &TransectState, transect: &mut TTransect, i: i32, y: f64) {
    //
    //  Input:   transect = transect being analyzed
    //           i = index of current entry in geometry tables
    //           y = depth of current entry in geometry tables
    //  Output:  none
    //  Purpose: computes entries in a transect's geometry tables at a given depth.
    //
    let mut k: i32; // station index
    let mut ylo: f64; // lower elev. of transect slice
    let mut yhi: f64; // higher elev. of transect slice
    let mut w: f64 = 0.0; // top width of transect slice
    let mut wp: f64 = 0.0; // wetted perimeter of transect slice
    let mut wpSum: f64; // total wetted perimeter across transect
    let mut a: f64 = 0.0; // area of transect slice
    let mut aSum: f64; // total area across transect
    let mut q: f64; // flow across transect slices with same roughness
    let mut qSum: f64; // total flow across transect
    let mut findFlow: i32; // true if flow thru area slice needs updating
    let iu = i as usize;

    // --- initialize
    wpSum = 0.0;
    aSum = 0.0;
    qSum = 0.0;

    // --- examine each horizontal station from left to right
    k = 1;
    while k <= st.Nstations {
        let ku = k as usize;
        // --- determine low & high elevations for transect sub-section
        if st.Elev[ku - 1] >= st.Elev[ku] {
            yhi = st.Elev[ku - 1];
            ylo = st.Elev[ku];
        } else {
            yhi = st.Elev[ku];
            ylo = st.Elev[ku - 1];
        }

        // --- skip station if its totally dry
        if ylo >= y {
            k += 1;
            continue;
        }

        // --- get top width, area & wetted perimeter values for transect
        //     slice between station k and k-1
        getSliceGeom(st, k, y, ylo, yhi, &mut w, &mut a, &mut wp);

        // --- update total transect values
        wpSum += wp;
        aSum += a;
        transect.areaTbl[iu] += a;
        transect.widthTbl[iu] += w;

        // --- must update flow if station elevation is above water level
        if st.Elev[ku] >= y {
            findFlow = TRUE;
        } else {
            findFlow = FALSE;
        }

        // --- update flow across transect if called for
        q = getFlow(st, k, aSum, wpSum, findFlow);
        if q > 0.0 {
            qSum += q;
            aSum = 0.0;
            wpSum = 0.0;
        }
        k += 1;
    } // next station k

    // --- find hyd. radius table entry solving Manning eq. with
    //     total flow, total area, and main channel n
    aSum = transect.areaTbl[iu];
    if aSum == 0.0 {
        transect.hradTbl[iu] = transect.hradTbl[iu - 1];
    } else {
        transect.hradTbl[iu] = (qSum * st.Nchannel / 1.49 / aSum).powf(1.5);
    }
}

//=============================================================================

fn getSliceGeom(st: &TransectState, k: i32, y: f64, ylo: f64, yhi: f64, w: &mut f64, a: &mut f64, wp: &mut f64) {
    //
    //  Input:   k = station index
    //           y = water elevation
    //           ylo = transect elevation on low side of slice
    //           yhi = transect elevation on high side of slice
    //  Output   w = width of transect slice
    //           a = area of transect slice
    //           wp = wetted perimeter of transect slice
    //  Purpose: finds area, width & wetted perim. for slice of transect that
    //           is covered by given water depth.
    //
    //      yhi  |
    //           |
    //        y  |**********
    //           |********** --> slice of transect being analyzed
    //      ylo  |**********|
    //           |**********|
    //           |**********|
    //         Station    Station
    //           k-1        k
    //
    let width: f64;
    let ratio: f64;
    let ku = k as usize;

    // --- compute width & wetted perimeter of transect slice
    width = (st.Station[ku] - st.Station[ku - 1]).abs();
    *w = width;
    *wp = (width * width + (yhi - ylo) * (yhi - ylo)).sqrt();
    *a = 0.0;

    // --- find area for completely submerged slice
    if y > yhi {
        *a = width * ((y - yhi) + (y - ylo)) / 2.0;
    }
    // --- otherwise find area and adjust width & wetted perim. for
    //     partly submerged slice
    else if yhi > ylo {
        ratio = (y - ylo) / (yhi - ylo);
        *a = width * (yhi - ylo) / 2.0 * ratio * ratio;
        *w *= ratio;
        *wp *= ratio;
    }
}

//=============================================================================

fn getFlow(st: &TransectState, k: i32, a: f64, wp: f64, findFlow: i32) -> f64 {
    //
    //  Input:   k = index of station at end of transect sub-section
    //           a = flow area of sub-section
    //           wp = wetted perimeter of flow area of sub-section
    //           findFlow = TRUE if flow needs updating
    //  Output:  returns normal flow (per unit of slope)
    //  Purpose: finds flow through a sub-section of a transect.
    //
    let mut findFlow = findFlow;
    let mut n: f64; // Manning's n
    let ku = k as usize;

    if findFlow == FALSE {
        // --- flow needs updating if we are at last station
        if k == st.Nstations - 1 {
            findFlow = TRUE;
        }
        // --- flow needs updating if we are at end of left overbank and
        //     there is a change in Manning's n and section not vertical
        else if st.Station[ku] == st.Xleftbank {
            if st.Nleft != st.Nchannel && st.Station[ku] != st.Station[ku - 1] {
                findFlow = TRUE;
            }
        }
        // --- flow needs updating if we are at start of right overbank and
        //     there is a change in Manning's n and section not vertical
        else if st.Station[ku] == st.Xrightbank {
            // (Station[k+1] can lie past the current transect; see the note
            //  at the top of the file)
            if st.Nright != st.Nchannel && st.Station[ku] != st.Station[ku + 1] {
                findFlow = TRUE;
            }
        }
    }

    // --- if flow needs updating
    if findFlow != 0 {
        // --- find value of Manning's n to use
        n = st.Nchannel;
        if st.Station[ku - 1] < st.Xleftbank {
            n = st.Nleft;
        }
        if st.Station[ku] > st.Xrightbank {
            n = st.Nright;
        }

        // --- compute flow through flow area
        //     (PHI is the Manning Eqn. constant defined in consts.h)
        return PHI / n * a * (a / wp).powf(2.0 / 3.0);
    }
    0.0
}

//=============================================================================

fn setMaxSectionFactor(transect: &mut TTransect) {
    //
    //  Input:   transect = transect being analyzed
    //  Output:  none
    //  Purpose: determines the maximum section factor for a transect and the
    //           area where this maxumum occurs.
    //
    let mut i: i32;
    let mut sf: f64;

    transect.aMax = 0.0;
    transect.sMax = 0.0;
    i = 1;
    while i < transect.nTbl {
        sf = transect.areaTbl[i as usize] * transect.hradTbl[i as usize].powf(2.0 / 3.0);
        if sf > transect.sMax {
            transect.sMax = sf;
            transect.aMax = transect.areaTbl[i as usize];
        }
        i += 1;
    }
}

//=============================================================================

pub fn transect_createStreetTransect(prj: &mut Project, street: &mut TStreet) {
    //
    let ymin: f64;
    let mut ymax: f64;
    let y1: f64;
    let y3: f64;
    let y4: f64;
    let w1: f64;
    let w2: f64;
    let w3: f64;
    let w4: f64;
    let st = &mut prj.transect;

    // Point 0 = top of backing
    // Point 1 = top of curb
    // Point 2 = bottom of curb
    // Point 3 = bottom of depressed gutter
    // Point 4 = top of depressed gutter
    // Point 5 = street crown

    // --- assign height (y) and width (w) to road & gutter sections
    ymin = 0.0;
    w1 = street.backWidth;
    w2 = street.gutterWidth;
    w3 = street.width;
    w4 = w3 - w2;
    y3 = street.gutterDepression + street.slope * w2;
    y1 = street.curbHeight + street.gutterDepression;
    ymax = street.backSlope * street.backWidth + y1;
    y4 = y3 + street.slope * w4;
    ymax = MAX(ymax, y4);

    // --- assign Station,Elevation points to the street's sections
    st.Station[0] = 0.0;
    st.Elev[0] = ymax;
    st.Station[1] = w1;
    st.Elev[1] = y1;
    st.Station[2] = w1;
    st.Elev[2] = 0.0;
    st.Station[3] = w1 + w2;
    st.Elev[3] = y3;
    st.Station[4] = w1 + w3;
    st.Elev[4] = y4;

    // --- a half street ends here
    if street.sides == 1 {
        st.Station[5] = st.Station[4];
        st.Elev[5] = ymax;
        st.Nstations = 5;
        street.transect.nTbl = N_TRANSECT_TBL;
    }
    // --- the right side of a full street mirrors the left side
    else {
        st.Station[5] = st.Station[4] + w4;
        st.Elev[5] = y3;
        st.Station[6] = st.Station[5] + w2;
        st.Elev[6] = 0.0;
        st.Station[7] = st.Station[6];
        st.Elev[7] = y1;
        st.Station[8] = st.Station[7] + w1;
        st.Elev[8] = ymax;
        st.Nstations = 8;
        street.transect.nTbl = N_TRANSECT_TBL;
    }

    // --- assign Manning's N to street
    st.Nchannel = street.roughness;
    if street.backWidth == 0.0 {
        st.Nleft = st.Nchannel;
        st.Nright = st.Nchannel;
        st.Xleftbank = st.Station[0];
        st.Xrightbank = st.Station[st.Nstations as usize];
    } else {
        st.Nleft = street.backRoughness;
        st.Nright = st.Nleft;
        st.Xleftbank = st.Station[1];
        if street.sides == 2 {
            st.Xrightbank = st.Station[(st.Nstations - 1) as usize];
        } else {
            st.Xrightbank = st.Station[st.Nstations as usize];
        }
    }

    // --- create the street's geometry tables
    createTables(&prj.transect, &mut street.transect, ymin, ymax);
    street.transect.roughness = street.roughness;
}
