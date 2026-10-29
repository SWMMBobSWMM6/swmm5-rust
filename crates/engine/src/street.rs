//! street.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   street.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     07/13/23  (Build 5.2.4)
//   Author:   L. Rossman
//
//   Street Cross-Section Functions
//
//   Build 5.2.4:
//   - Fixed incorrect index used to retrieve street backing parameters
//     in street_readParams.
//-----------------------------------------------------------------------------

use crate::prelude::*;

/// The file-scope variables of street.c (none).
#[derive(Default)]
pub struct StreetState {}

//-----------------------------------------------------------------------------
//  External functions (declared in street.h)
//-----------------------------------------------------------------------------
// street_create             called by createObjects in project.c
// street_delete             called by deleteObjects in project.c
// street_readParams         called by parseLine in input.c

//=============================================================================

pub fn street_create(prj: &mut Project, nStreets: i32) -> i32 {
    //
    //  Input:   nStreets = number of Street objects to create
    //  Output:  none
    //  Purpose: creats a collection of Street objects.
    //
    prj.Street = Vec::new();
    prj.Nobjects[STREET as usize] = 0;
    prj.Street = vec![TStreet::default(); nStreets as usize];
    prj.Nobjects[STREET as usize] = nStreets;
    0
}

//=============================================================================

pub fn street_delete(prj: &mut Project) {
    //
    //  Input:   none
    //  Output:  none
    //  Purpose: deletes the collection of Street objects.
    //
    prj.Street = Vec::new();
}

//=============================================================================

pub fn street_readParams(prj: &mut Project, tok: &[Vec<u8>], ntoks: i32) -> i32 {
    //
    //  Format is:
    //    ID  Tcrown  Hcurb  Sx  nRoad  (Hdep  Wg  Sides  Tback  Sback  nBack)
    //  where
    //    ID = name assigned to street cross section
    //    Tcrown = distance from curb to street crown (ft or m)
    //    Hcurb  = curb height (ft or m)
    //    Sx     = roadway cross slope (%)
    //    nRoad  = roadway Manning's n
    //    Hdep   = depressed gutter height (ft or m)
    //    Wg     = depressed gutter width (ft or m)
    //    Sides  = 1 or 2 sided
    //    Tback  = width of street backing (ft or m)
    //    Sback  = slope of street backing (ft or m)
    //    nBack  = Manning's n of street backing
    let i: i32;
    let mut k: i32;
    let mut sides: i32;
    // NOTE: C sets x[0..9] to 0 and leaves x[10] uninitialized; x[10] is
    // read (street->backRoughness = x[10]) even when it was not parsed
    // (ntoks <= 8 or x[8] == 0). EPA's build then copies stack garbage
    // into backRoughness; it is used only when backWidth > 0, which implies
    // x[10] was parsed. Here it is 0.
    let mut x: [f64; 11] = [0.0; 11];

    // --- check for minimum number of tokens
    if ntoks < 5 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check that street exists in project
    i = project_findObject(prj, STREET, &tok[0]);
    if i < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[0]);
    }
    prj.Street[i as usize].ID = project_findID(prj, STREET, &tok[0]).unwrap_or_default();

    // --- parse required data
    k = 0;
    while k <= 9 {
        x[k as usize] = 0.0;
        k += 1;
    }
    k = 1;
    while k <= 4 {
        if getDouble(&tok[k as usize], &mut x[k as usize]) == 0 || x[k as usize] <= 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[k as usize]);
        }
        k += 1;
    }

    // --- read gutter depression
    if ntoks > 5 {
        if getDouble(&tok[5], &mut x[5]) == 0 || x[5] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[5]);
        }
    }

    // --- read gutter width
    if ntoks > 6 {
        if getDouble(&tok[6], &mut x[6]) == 0 || x[6] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[6]);
        }
    }

    // --- read if 1- or 2-sided
    sides = 2;
    if ntoks > 7 {
        if getInt(&tok[7], &mut sides) == 0 || sides < 1 || sides > 2 {
            return error_setInpError(prj, ERR_NUMBER, &tok[7]);
        }
    }

    // --- read street backing parameters
    if ntoks > 8 {
        if getDouble(&tok[8], &mut x[8]) == 0 || x[8] < 0.0 {
            return error_setInpError(prj, ERR_NUMBER, &tok[8]);
        }
        if x[8] > 0.0 {
            if ntoks < 11 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            k = 9;
            while k <= 10 {
                if getDouble(&tok[k as usize], &mut x[k as usize]) == 0 || x[k as usize] <= 0.0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[k as usize]);
                }
                k += 1;
            }
        }
    }

    // --- assign input values to street object
    let ucf = UCF(prj, LENGTH);
    let mut street = std::mem::take(&mut prj.Street[i as usize]);
    street.width = x[1] / ucf;
    street.curbHeight = x[2] / ucf;
    street.slope = x[3] / 100.0;
    street.roughness = x[4];
    street.gutterDepression = x[5] / ucf;
    street.gutterWidth = x[6] / ucf;
    street.sides = sides;
    street.backWidth = x[8] / ucf;
    street.backSlope = x[9] / 100.0;
    street.backRoughness = x[10];
    // (transect_createStreetTransect does not read prj.Street)
    transect_createStreetTransect(prj, &mut street);
    prj.Street[i as usize] = street;
    0
}

//=============================================================================

pub fn street_getExtentFilled(prj: &Project, link: i32) -> f64 {
    //
    //  Input:   link = a link index
    //  Output:  degree to which street is filled
    //  Purpose: finds the degree to which a street link is filled based on the
    //           depth (for DW routing) or cross section area at the higher end.
    //
    let k: i32;
    let t: i32;
    let filled: f64;
    let lk = &prj.Link[link as usize];

    t = lk.xsect.transect;
    if t < 0 {
        return 0.0;
    }
    if prj.RouteModel == DW {
        filled = MAX(prj.Node[lk.node1 as usize].newDepth, prj.Node[lk.node2 as usize].newDepth);
    } else {
        k = lk.subIndex;
        filled = MAX(prj.Conduit[k as usize].a1, prj.Conduit[k as usize].a2);
    }
    filled
}
