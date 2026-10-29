//! xsect (with shape, transect street geometry): the safe engine against the
//! c2rust baseline, bit for bit.
#![allow(non_snake_case)]

use swmm5_c2rust::engine as c;
use swmm5_difftest::{same, Rng};
use swmm5_rust::prelude::*;

type CX = c::xsect::TXsect;

fn cx_fields(x: &CX) -> ([i32; 3], [f64; 11]) {
    ([x.type_0, x.culvertCode, x.transect], [x.yFull, x.wMax, x.ywMax, x.aFull, x.rFull, x.sFull, x.sMax, x.yBot, x.aBot, x.sBot, x.rBot])
}
fn rx_fields(x: &TXsect) -> ([i32; 3], [f64; 11]) {
    ([x.type_, x.culvertCode, x.transect], [x.yFull, x.wMax, x.ywMax, x.aFull, x.rFull, x.sFull, x.sMax, x.yBot, x.aBot, x.sBot, x.rBot])
}
fn assert_same_xsect(r: &TXsect, cxs: &CX, ctx: &str) {
    let (ri, rf) = rx_fields(r);
    let (ci, cf) = cx_fields(cxs);
    assert_eq!(ri, ci, "{ctx}: int fields");
    for k in 0..11 {
        assert!(same(rf[k], cf[k]), "{ctx}: field {k}: rust {:e} c {:e}", rf[k], cf[k]);
    }
}
fn chk(name: &str, ctx: &str, arg: f64, r: f64, cv: f64) {
    assert!(same(r, cv), "{name}({arg:e}) {ctx}: rust {r:e} ({:#x}) c {cv:e} ({:#x})", r.to_bits(), cv.to_bits());
}

fn pick(r: &mut Rng, v: &[f64]) -> f64 {
    v[(r.next() % v.len() as u64) as usize]
}

/// Plausible (and some implausible) parameter sets for each shape.
fn gen_params(r: &mut Rng, t: i32) -> [f64; 4] {
    let d = if r.next() % 4 == 0 { pick(r, &[0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 6.0, 10.0]) } else { r.range(0.1, 15.0) };
    let w = if r.next() % 4 == 0 { pick(r, &[1.0, 2.0, 3.0, 5.0, 8.0]) } else { r.range(0.1, 20.0) };
    match t {
        DUMMY => [0.0; 4],
        CIRCULAR | EGGSHAPED | HORSESHOE | GOTHIC | CATENARY | SEMIELLIPTICAL | BASKETHANDLE | SEMICIRCULAR => [d, 0.0, 0.0, 0.0],
        FORCE_MAIN => [d, pick(r, &[100.0, 120.0, 130.0, 0.01, 0.003]), 0.0, 0.0],
        FILLED_CIRCULAR => {
            let f = if r.next() % 10 == 0 { d * r.range(0.9, 1.2) } else if r.next() % 3 == 0 { (d * r.range(0.0, 0.6) * 100.0).round() / 100.0 } else { d * r.range(0.0, 0.8) };
            [d, f, 0.0, 0.0]
        }
        RECT_CLOSED => [d, w, 0.0, 0.0],
        RECT_OPEN => [d, w, pick(r, &[0.0, 1.0, 2.0, 0.5, -0.5, 2.5]), 0.0],
        RECT_TRIANG => [d, w, d * r.range(0.05, 0.8), 0.0],
        RECT_ROUND | MOD_BASKET => {
            let rad = if r.next() % 4 == 0 { w * r.range(0.1, 0.5) } else { w * r.range(0.5, 3.0) };
            [d, w, rad, 0.0]
        }
        TRAPEZOIDAL => {
            let b = if r.next() % 5 == 0 { 0.0 } else { r.range(0.0, 20.0) };
            let s1 = if r.next() % 5 == 0 { 0.0 } else { r.range(0.0, 5.0) };
            let s2 = if r.next() % 5 == 0 { 0.0 } else { r.range(0.0, 5.0) };
            [d, b, s1, s2]
        }
        TRIANGULAR | PARABOLIC => [d, w, 0.0, 0.0],
        POWERFUNC => [d, w, r.range(0.2, 4.0), 0.0],
        HORIZ_ELLIPSE | VERT_ELLIPSE | ARCH => {
            if r.next() % 2 == 0 {
                // standard size code
                let n = if t == ARCH { 104.0 } else { 25.0 };
                [(r.range(0.0, n)).floor().max(0.5), 0.0, 0.0, 0.0]
            } else {
                [d, w, 0.0, 0.0]
            }
        }
        _ => [d, w, 0.0, 0.0],
    }
}

/// Exercise every getter of a cross section over its range. Returns the
/// number of getter evaluations compared. The `_mut` getters are compared
/// with C, and the cross sections (which C's FILLED_CIRCULAR getters can
/// change in the last bit) are compared after every call; the `&TXsect`
/// wrappers are checked to return what `_mut` returns from the same state.
fn sweep(prj: &Project, r: &mut TXsect, cxs: &mut CX, rng: &mut Rng, ctx: &str, npts: usize) -> usize {
    let mut n = 0;
    let t = r.type_;
    unsafe {
        assert_eq!(xsect_isOpen(t), c::xsect::xsect_isOpen(t));
        chk("getAmax", ctx, 0.0, xsect_getAmax(prj, r), c::xsect::xsect_getAmax(cxs));
    }
    let yFull = r.yFull;
    let aFull = r.aFull;
    let sMax = r.sMax;
    macro_rules! both {
        ($name:expr, $wrap:ident, $mutf:ident, $cf:path, $arg:expr) => {{
            let arg = $arg;
            let w = $wrap(prj, r, arg);
            let a = $mutf(prj, r, arg);
            let b = unsafe { $cf(cxs, arg) };
            chk($name, ctx, arg, a, b);
            assert!(same(w, a), "{} wrapper vs _mut {ctx}", $name);
            assert_same_xsect(r, cxs, &format!("after {}({arg:e}) {ctx}", $name));
            n += 1;
        }};
    }
    // Depths above full only for open shapes: SWMM never asks a closed
    // shape's AofY/RofY above its crown (dwflow caps the depth), and for
    // some (MOD_BASKET, RECT_ROUND) C would then index its tables with
    // (int)NaN and read gigabytes out of bounds.
    let ymaxf = if xsect_isOpen(t) != 0 { 1.1 } else { 1.0 };
    for i in 0..=npts {
        let fy = if i < npts / 2 { i as f64 / (npts / 2) as f64 * ymaxf } else { rng.range(0.0, ymaxf) };
        let y = fy * yFull;
        let fa = if i < npts / 2 { i as f64 / (npts / 2) as f64 * 1.05 } else { rng.range(0.0, 1.05) };
        let a = fa * aFull;
        let fs = if i < npts / 2 { i as f64 / (npts / 2) as f64 * 1.1 } else { rng.range(0.0, 1.1) };
        let s = fs * sMax;
        both!("getAofY", xsect_getAofY, xsect_getAofY_mut, c::xsect::xsect_getAofY, y);
        unsafe {
            chk("getWofY", ctx, y, xsect_getWofY(prj, r, y), c::xsect::xsect_getWofY(cxs, y));
        }
        n += 1;
        both!("getRofY", xsect_getRofY, xsect_getRofY_mut, c::xsect::xsect_getRofY, y);
        both!("getYofA", xsect_getYofA, xsect_getYofA_mut, c::xsect::xsect_getYofA, a);
        both!("getAofS", xsect_getAofS, xsect_getAofS_mut, c::xsect::xsect_getAofS, s);
        // DUMMY's SofA and RofA call each other forever for a > 0 (in C too)
        if t != DUMMY {
            both!("getSofA", xsect_getSofA, xsect_getSofA_mut, c::xsect::xsect_getSofA, a);
            both!("getRofA", xsect_getRofA, xsect_getRofA_mut, c::xsect::xsect_getRofA, a);
            both!("getdSdA", xsect_getdSdA, xsect_getdSdA_mut, c::xsect::xsect_getdSdA, a);
        }
    }
    // critical depth over a wide range of flows
    let qref = aFull * (GRAVITY * yFull).sqrt();
    for i in 0..npts / 4 {
        let q = if i == 0 { 0.0 } else { qref * 10f64.powf(rng.range(-4.0, 0.7)) };
        both!("getYcrit", xsect_getYcrit, xsect_getYcrit_mut, c::xsect::xsect_getYcrit, q);
    }
    n
}

#[test]
fn xsect_all_regular_shapes() {
    let prj = Project::default();
    let mut rng = Rng(0x9e3779b97f4a7c15);
    let types = [
        DUMMY, CIRCULAR, FILLED_CIRCULAR, RECT_CLOSED, RECT_OPEN, TRAPEZOIDAL, TRIANGULAR, PARABOLIC, POWERFUNC, RECT_TRIANG, RECT_ROUND, MOD_BASKET, HORIZ_ELLIPSE, VERT_ELLIPSE, ARCH, EGGSHAPED, HORSESHOE, GOTHIC,
        CATENARY, SEMIELLIPTICAL, BASKETHANDLE, SEMICIRCULAR, FORCE_MAIN,
    ];
    let mut total_sets = 0usize;
    let mut total_evals = 0usize;
    for &t in types.iter() {
        let mut ok_sets = 0;
        for k in 0..300 {
            let p = gen_params(&mut rng, t);
            let ucf = if k % 3 == 0 { 0.3048 } else { 1.0 };
            let mut r = TXsect::default();
            let mut cxs: CX = unsafe { std::mem::zeroed() };
            let mut pc = p;
            let rr = xsect_setParams(&prj, &mut r, t, &p, ucf);
            let rc = unsafe { c::xsect::xsect_setParams(&mut cxs, t, pc.as_mut_ptr(), ucf) };
            let ctx = format!("type {t} p {p:?} ucf {ucf}");
            assert_eq!(rr, rc, "setParams return, {ctx}");
            assert_same_xsect(&r, &cxs, &ctx);
            total_sets += 1;
            if rr == 0 {
                continue;
            }
            ok_sets += 1;
            total_evals += sweep(&prj, &mut r, &mut cxs, &mut rng, &ctx, 200);
        }
        assert!(ok_sets > 50 || t == FILLED_CIRCULAR, "type {t}: only {ok_sets} valid parameter sets");
        eprintln!("xsect type {t:2}: {ok_sets} valid parameter sets");
    }
    eprintln!("xsect_all_regular_shapes: {total_sets} setParams calls, {total_evals} getter evaluations");
}

/// FILLED_CIRCULAR on a decimal grid of diameters and fill depths, the
/// inputs most likely to expose C's in-place (yFull+yBot)-yBot restore.
#[test]
fn xsect_filled_circular_grid() {
    let prj = Project::default();
    let mut rng = Rng(12345);
    let mut evals = 0usize;
    let mut sets = 0usize;
    for dc in (10..=600).step_by(3) {
        for fc in (1..dc).step_by(7) {
            let p = [dc as f64 / 100.0, fc as f64 / 100.0, 0.0, 0.0];
            for &ucf in &[1.0, 0.3048] {
                let mut r = TXsect::default();
                let mut cxs: CX = unsafe { std::mem::zeroed() };
                let mut pc = p;
                let rr = xsect_setParams(&prj, &mut r, FILLED_CIRCULAR, &p, ucf);
                let rc = unsafe { c::xsect::xsect_setParams(&mut cxs, FILLED_CIRCULAR, pc.as_mut_ptr(), ucf) };
                let ctx = format!("FILLED_CIRCULAR p {p:?} ucf {ucf}");
                assert_eq!(rr, rc);
                assert_same_xsect(&r, &cxs, &ctx);
                sets += 1;
                evals += sweep(&prj, &mut r, &mut cxs, &mut rng, &ctx, 40);
            }
        }
    }
    eprintln!("xsect_filled_circular_grid: {sets} sections, {evals} getter evaluations");
}

// ---------------------------------------------------------------------------
//  Street transects, IRREGULAR, STREET_XSECT and CUSTOM (shape) sections
// ---------------------------------------------------------------------------

fn same_transect(r: &TTransect, t: &c::swmm5::TTransect, ctx: &str) {
    let rs = [r.yFull, r.aFull, r.rFull, r.wMax, r.ywMax, r.sMax, r.aMax, r.lengthFactor, r.roughness];
    let cs = [t.yFull, t.aFull, t.rFull, t.wMax, t.ywMax, t.sMax, t.aMax, t.lengthFactor, t.roughness];
    for k in 0..rs.len() {
        assert!(same(rs[k], cs[k]), "{ctx}: transect scalar {k}: {} vs {}", rs[k], cs[k]);
    }
    assert_eq!(r.nTbl, t.nTbl, "{ctx}: nTbl");
    for i in 0..51 {
        assert!(same(r.areaTbl[i], t.areaTbl[i]), "{ctx}: areaTbl[{i}]");
        assert!(same(r.hradTbl[i], t.hradTbl[i]), "{ctx}: hradTbl[{i}]");
        assert!(same(r.widthTbl[i], t.widthTbl[i]), "{ctx}: widthTbl[{i}]");
    }
}

fn same_shape(r: &TShape, t: &c::swmm5::TShape, ctx: &str) {
    let rs = [r.aFull, r.rFull, r.wMax, r.sMax, r.aMax];
    let cs = [t.aFull, t.rFull, t.wMax, t.sMax, t.aMax];
    for k in 0..rs.len() {
        assert!(same(rs[k], cs[k]), "{ctx}: shape scalar {k}: {} vs {}", rs[k], cs[k]);
    }
    assert_eq!(r.nTbl, t.nTbl, "{ctx}: nTbl");
    for i in 0..51 {
        assert!(same(r.areaTbl[i], t.areaTbl[i]), "{ctx}: areaTbl[{i}] {} vs {}", r.areaTbl[i], t.areaTbl[i]);
        assert!(same(r.hradTbl[i], t.hradTbl[i]), "{ctx}: hradTbl[{i}]");
        assert!(same(r.widthTbl[i], t.widthTbl[i]), "{ctx}: widthTbl[{i}]");
    }
}

#[test]
fn street_transects_and_model_shapes() {
    let mut rng = Rng(0xdeadbeef12345);
    let mut prj = Project::default();
    const NSTREET: usize = 200;
    const NSHAPE: usize = 300;

    // --- streets (transect_createStreetTransect)
    let mut cstreets: Vec<c::swmm5::TStreet> = vec![unsafe { std::mem::zeroed() }; NSTREET];
    let mut rstreets: Vec<TStreet> = vec![TStreet::default(); NSTREET];
    for i in 0..NSTREET {
        let back = rng.next() % 3 != 0;
        let sides = if rng.next() % 2 == 0 { 1 } else { 2 };
        let width = rng.range(5.0, 30.0);
        let gw = if rng.next() % 4 == 0 { 0.0 } else { rng.range(0.5, 3.0).min(width * 0.5) };
        let vals = (
            sides,
            rng.range(0.005, 0.06),
            width,
            rng.range(0.3, 1.0),
            if rng.next() % 4 == 0 { 0.0 } else { rng.range(0.0, 0.3) },
            gw,
            rng.range(0.011, 0.02),
            if back { rng.range(0.01, 0.1) } else { 0.0 },
            if back { rng.range(1.0, 15.0) } else { 0.0 },
            if back { rng.range(0.02, 0.1) } else { 0.0 },
        );
        let r = &mut rstreets[i];
        r.sides = vals.0;
        r.slope = vals.1;
        r.width = vals.2;
        r.curbHeight = vals.3;
        r.gutterDepression = vals.4;
        r.gutterWidth = vals.5;
        r.roughness = vals.6;
        r.backSlope = vals.7;
        r.backWidth = vals.8;
        r.backRoughness = vals.9;
        let cs = &mut cstreets[i];
        cs.sides = vals.0;
        cs.slope = vals.1;
        cs.width = vals.2;
        cs.curbHeight = vals.3;
        cs.gutterDepression = vals.4;
        cs.gutterWidth = vals.5;
        cs.roughness = vals.6;
        cs.backSlope = vals.7;
        cs.backWidth = vals.8;
        cs.backRoughness = vals.9;
        transect_createStreetTransect(&mut prj, r);
        unsafe { c::transect::transect_createStreetTransect(cs as *mut c::swmm5::TStreet as *mut c::transect::TStreet) };
        same_transect(&r.transect, &cs.transect, &format!("street {i}"));
    }
    prj.Street = rstreets.clone();
    // IRREGULAR sections reuse the street transects as transects
    prj.Transect = rstreets.iter().map(|s| s.transect.clone()).collect();
    let mut ctransects: Vec<c::swmm5::TTransect> = cstreets.iter().map(|s| s.transect).collect();

    // --- custom shapes (shape_validate)
    let mut cshapes: Vec<c::swmm5::TShape> = vec![unsafe { std::mem::zeroed() }; NSHAPE];
    let mut ccurves: Vec<c::swmm5::TTable> = vec![unsafe { std::mem::zeroed() }; NSHAPE];
    prj.Shape = vec![TShape::default(); NSHAPE];
    prj.Curve = (0..NSHAPE).map(|_| TTable::default()).collect();
    let mut valid_shapes = Vec::new();
    for i in 0..NSHAPE {
        unsafe { c::table::table_init(&mut ccurves[i] as *mut _ as *mut c::table::TTable) };
        table_init(&mut prj.Curve[i]);
        let npts = 2 + (rng.next() % 12) as usize;
        let mut y = if rng.next() % 3 == 0 { 0.0 } else { rng.range(0.0, 0.3) };
        for k in 0..npts {
            let w = if rng.next() % 15 == 0 { -0.1 } else { rng.range(0.0, 2.0) };
            unsafe { c::table::table_addEntry(&mut ccurves[i] as *mut _ as *mut c::table::TTable, y, w) };
            table_addEntry(&mut prj.Curve[i], y, w);
            y += if rng.next() % 20 == 0 { 0.0 } else { rng.range(0.0, 0.3) };
            if k + 2 == npts && rng.next() % 2 == 0 {
                y = 1.0;
            }
        }
        let mut sh = TShape::default();
        let mut cv = std::mem::take(&mut prj.Curve[i]);
        let rr = shape_validate(&mut sh, &mut cv);
        prj.Curve[i] = cv;
        let rc = unsafe { c::shape::shape_validate(&mut cshapes[i] as *mut _ as *mut c::shape::TShape, &mut ccurves[i] as *mut _ as *mut c::shape::TTable) };
        assert_eq!(rr, rc, "shape {i} validate");
        if rr != 0 {
            same_shape(&sh, &cshapes[i], &format!("shape {i}"));
            valid_shapes.push(i);
        }
        prj.Shape[i] = sh;
        prj.Curve[i].refersTo = i as i32;
        ccurves[i].refersTo = i as i32;
    }
    eprintln!("shapes: {} of {NSHAPE} valid", valid_shapes.len());

    // --- point the baseline's globals at its arrays
    unsafe {
        c::swmm5::Street = cstreets.as_mut_ptr();
        c::swmm5::Transect = ctransects.as_mut_ptr();
        c::swmm5::Shape = cshapes.as_mut_ptr();
        c::swmm5::Curve = ccurves.as_mut_ptr();
    }

    let mut evals = 0usize;
    let mut sections = 0usize;
    // STREET_XSECT and IRREGULAR
    for i in 0..NSTREET {
        for &t in &[STREET_XSECT, IRREGULAR] {
            let mut r = TXsect::default();
            r.type_ = t;
            r.transect = i as i32;
            let mut cxs: CX = unsafe { std::mem::zeroed() };
            cxs.type_0 = t;
            cxs.transect = i as i32;
            if t == STREET_XSECT {
                xsect_setStreetXsectParams(&prj, &mut r);
                unsafe { c::xsect::xsect_setStreetXsectParams(&mut cxs) };
            } else {
                xsect_setIrregXsectParams(&prj, &mut r);
                unsafe { c::xsect::xsect_setIrregXsectParams(&mut cxs) };
            }
            let ctx = format!("type {t} transect {i}");
            assert_same_xsect(&r, &cxs, &ctx);
            evals += sweep(&prj, &mut r, &mut cxs, &mut rng, &ctx, 100);
            sections += 1;
        }
    }
    // CUSTOM
    for &i in valid_shapes.iter() {
        for _ in 0..3 {
            let yFull = rng.range(0.3, 10.0);
            let mut r = TXsect::default();
            r.type_ = CUSTOM;
            r.transect = i as i32;
            r.yFull = yFull;
            let mut cxs: CX = unsafe { std::mem::zeroed() };
            cxs.type_0 = CUSTOM;
            cxs.transect = i as i32;
            cxs.yFull = yFull;
            xsect_setCustomXsectParams(&prj, &mut r);
            unsafe { c::xsect::xsect_setCustomXsectParams(&mut cxs) };
            let ctx = format!("CUSTOM shape {i} yFull {yFull}");
            assert_same_xsect(&r, &cxs, &ctx);
            evals += sweep(&prj, &mut r, &mut cxs, &mut rng, &ctx, 100);
            sections += 1;
        }
    }
    unsafe {
        c::swmm5::Street = std::ptr::null_mut();
        c::swmm5::Transect = std::ptr::null_mut();
        c::swmm5::Shape = std::ptr::null_mut();
        c::swmm5::Curve = std::ptr::null_mut();
    }
    eprintln!("street/irregular/custom: {NSTREET} street transects, {NSHAPE} shape curves, {sections} sections, {evals} getter evaluations");
}
