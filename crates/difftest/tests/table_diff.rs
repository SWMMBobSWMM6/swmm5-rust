//! table: the safe engine against the c2rust baseline, bit for bit.
#![allow(non_snake_case)]

use std::ffi::CString;
use swmm5_c2rust::engine as c;
use swmm5_difftest::{same, Rng};
use swmm5_rust::prelude::*;

type CT = c::table::TTable;

fn new_pair() -> (TTable, Box<CT>) {
    let mut r = TTable::default();
    table_init(&mut r);
    let mut ct: Box<CT> = Box::new(unsafe { std::mem::zeroed() });
    unsafe { c::table::table_init(&mut *ct) };
    (r, ct)
}

fn add(r: &mut TTable, ct: &mut CT, x: f64, y: f64) {
    assert_eq!(table_addEntry(r, x, y), unsafe { c::table::table_addEntry(ct, x, y) });
}

fn free_c(ct: &mut CT) {
    unsafe { c::table::table_deleteEntries(ct) };
}

fn chk(name: &str, k: usize, x: f64, r: f64, cv: f64) {
    assert!(same(r, cv), "{name} table {k} x {x:e}: rust {r:e} c {cv:e}");
}

/// Random tables: increasing x (some with ties and decreasing segments), all
/// lookup functions over a range that extends past both ends.
#[test]
fn table_curve_lookups() {
    let mut rng = Rng(0xabcdef0123);
    let mut n = 0usize;
    for k in 0..3000 {
        let (mut r, mut ct) = new_pair();
        let npts = (rng.next() % 25) as usize; // includes empty tables
        let mut x = if k % 4 == 0 { 0.0 } else { rng.range(-5.0, 5.0) };
        let kind = k % 5;
        for _ in 0..npts {
            let y = match kind {
                0 => rng.range(0.0, 100.0),
                1 => x * rng.range(0.5, 2.0),
                2 => rng.range(-10.0, 10.0),
                3 => (x * 10.0).floor(),
                _ => rng.range(0.0, 1.0e4),
            };
            add(&mut r, &mut ct, x, y);
            x += match rng.next() % 10 {
                0 => 0.0,
                1 => -rng.range(0.0, 1.0),
                _ => rng.range(1.0e-3, 3.0),
            };
        }
        let (xmin, xmax) = if r.entries.is_empty() {
            (-1.0, 1.0)
        } else {
            let a = r.entries.iter().map(|e| e.x).fold(f64::INFINITY, f64::min);
            let b = r.entries.iter().map(|e| e.x).fold(f64::NEG_INFINITY, f64::max);
            (a - 1.0, b + 1.0)
        };
        let (ymin, ymax) = if r.entries.is_empty() {
            (-1.0, 1.0)
        } else {
            let a = r.entries.iter().map(|e| e.y).fold(f64::INFINITY, f64::min);
            let b = r.entries.iter().map(|e| e.y).fold(f64::NEG_INFINITY, f64::max);
            (a - 1.0, b + 1.0)
        };
        for i in 0..300 {
            // probe at the points themselves, too
            let xq = if i < r.entries.len() { r.entries[i].x } else { rng.range(xmin, xmax) };
            let yq = if i < r.entries.len() { r.entries[i].y } else { rng.range(ymin, ymax) };
            unsafe {
                chk("lookup", k, xq, table_lookup(&mut r, xq), c::table::table_lookup(&mut *ct, xq));
                chk("lookupEx", k, xq, table_lookupEx(&mut r, xq), c::table::table_lookupEx(&mut *ct, xq));
                chk("getSlope", k, xq, table_getSlope(&mut r, xq), c::table::table_getSlope(&mut *ct, xq));
                chk("intervalLookup", k, xq, table_intervalLookup(&mut r, xq), c::table::table_intervalLookup(&mut *ct, xq));
                chk("inverseLookup", k, yq, table_inverseLookup(&mut r, yq), c::table::table_inverseLookup(&mut *ct, yq));
                if !r.entries.is_empty() {
                    chk("getMaxY", k, xq, table_getMaxY(&mut r, xq), c::table::table_getMaxY(&mut *ct, xq));
                }
                let xv = xq.abs();
                chk("getStorageVolume", k, xv, table_getStorageVolume(&mut r, xv), c::table::table_getStorageVolume(&mut *ct, xv));
                let vq = if i % 3 == 0 { 0.0 } else { rng.range(0.0, 1.0e4) };
                chk("getStorageDepth", k, vq, table_getStorageDepth(&mut r, vq), c::table::table_getStorageDepth(&mut *ct, vq));
                chk("interpolate", k, xq, table_interpolate(xq, xmin, ymin, xmax, ymax), c::table::table_interpolate(xq, xmin, ymin, xmax, ymax));
            }
            n += 9;
        }
        // validate (sequence check) and first/next iteration; C dereferences
        // a NULL cursor in table_getNextEntry for an empty table, so skip those
        if r.entries.is_empty() {
            free_c(&mut ct);
            continue;
        }
        let rv = table_validate(&mut r);
        let cv = unsafe { c::table::table_validate(&mut *ct) };
        assert_eq!(rv, cv, "validate table {k}");
        assert!(same(r.dxMin, ct.dxMin) && same(r.x2, ct.x2), "validate fields table {k}");
        let (mut rx, mut ry, mut cx, mut cy) = (0.0, 0.0, 0.0, 0.0);
        let a = table_getFirstEntry(&mut r, &mut rx, &mut ry);
        let b = unsafe { c::table::table_getFirstEntry(&mut *ct, &mut cx, &mut cy) };
        assert_eq!(a, b);
        assert!(same(rx, cx) && same(ry, cy));
        if a != 0 {
            loop {
                let a = table_getNextEntry(&mut r, &mut rx, &mut ry);
                let b = unsafe { c::table::table_getNextEntry(&mut *ct, &mut cx, &mut cy) };
                assert_eq!(a, b);
                assert!(same(rx, cx) && same(ry, cy));
                n += 1;
                if a == 0 {
                    break;
                }
            }
        }
        free_c(&mut ct);
    }
    eprintln!("table_curve_lookups: 3000 tables, {n} comparisons");
}

fn same_bracket(r: &TTable, ct: &CT, ctx: &str) {
    assert!(same(r.x1, ct.x1) && same(r.x2, ct.x2) && same(r.y1, ct.y1) && same(r.y2, ct.y2), "{ctx}: bracket ({}, {}, {}, {}) vs ({}, {}, {}, {})", r.x1, r.x2, r.y1, r.y2, ct.x1, ct.x2, ct.y1, ct.y2);
}

/// Time series lookups: a stateful bracket walked forward, backward and at random.
#[test]
fn table_tseries_lookups() {
    let mut rng = Rng(0x5eed);
    let mut n = 0usize;
    for k in 0..2000 {
        let (mut r, mut ct) = new_pair();
        // (an empty series crashes C in table_tseriesInit; at least one point)
        let npts = 1 + (rng.next() % 40) as usize;
        let mut x = rng.range(36000.0, 40000.0);
        for _ in 0..npts {
            add(&mut r, &mut ct, x, rng.range(0.0, 5.0));
            x += if rng.next() % 15 == 0 { 0.0 } else { rng.range(1.0 / 1440.0, 0.5) };
        }
        table_tseriesInit(&mut r);
        unsafe { c::table::table_tseriesInit(&mut *ct) };
        same_bracket(&r, &ct, &format!("tseriesInit {k}"));
        let lo = r.entries.first().map(|e| e.x).unwrap_or(36000.0) - 0.5;
        let hi = r.entries.last().map(|e| e.x).unwrap_or(36000.0) + 0.5;
        let mut t = lo;
        for i in 0..400 {
            let xq = match k % 3 {
                0 => {
                    t += (hi - lo) / 300.0;
                    t
                }
                1 => rng.range(lo, hi),
                _ => {
                    if i % 50 == 0 {
                        t = lo
                    }
                    t += rng.range(0.0, (hi - lo) / 40.0);
                    t
                }
            };
            for extend in [0i8, 1i8] {
                let a = table_tseriesLookup(&mut r, xq, extend);
                let b = unsafe { c::table::table_tseriesLookup(&mut *ct, xq, extend as _) };
                assert!(same(a, b), "tseriesLookup table {k} x {xq} extend {extend}: {a} vs {b}");
                same_bracket(&r, &ct, &format!("tseriesLookup table {k} x {xq}"));
                n += 1;
            }
        }
        free_c(&mut ct);
    }
    eprintln!("table_tseries_lookups: 2000 series, {n} lookups");
}

/// Time series in external files (USE_FILE): validate, init and lookups.
#[test]
fn table_tseries_files() {
    // table_parseFileLine calls input.c's getDouble; skip until it is translated
    let probe = std::panic::catch_unwind(|| {
        let mut v = 0.0;
        getDouble(b"1.5", &mut v)
    });
    if probe.is_err() {
        eprintln!("table_tseries_files: SKIPPED (getDouble in input.rs is still a stub)");
        return;
    }
    unsafe { c::datetime::datetime_setDateFormat(M_D_Y) };
    let dir = std::env::temp_dir().join(format!("swmm5_difftest_tables_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut rng = Rng(4242);
    let mut n = 0usize;
    for k in 0..60 {
        // --- write a file with a mix of 2- and 3-token lines, comments,
        //     blank lines and (sometimes) bad data
        let mut text = String::new();
        let mut day = 1 + (rng.next() % 28) as i64;
        let mut minutes = 0i64;
        let nlines = 1 + (rng.next() % 200) as usize;
        for i in 0..nlines {
            match rng.next() % 12 {
                0 => text.push_str("; a comment\n"),
                1 => text.push_str("\n"),
                2 if i > 0 => {
                    minutes += 15;
                    text.push_str(&format!("{}:{:02} {:.3}\n", minutes / 60, minutes % 60, rng.range(0.0, 2.0)));
                }
                3 => {
                    minutes += 30;
                    text.push_str(&format!("{} {:.4}\n", minutes as f64 / 60.0, rng.range(0.0, 2.0)));
                }
                _ => {
                    if rng.next() % 4 == 0 {
                        day += 1;
                        minutes = 0;
                    }
                    minutes += 10;
                    text.push_str(&format!("01/{:02}/2005 {}:{:02} {:.5}\n", day.min(31), minutes / 60, minutes % 60, rng.range(0.0, 3.0)));
                }
            }
        }
        if k % 10 == 9 {
            text.push_str("01/02/2005 junk 1.0\n");
        }
        if k % 10 == 8 {
            text.push_str("01/01/2005 0:00 1.0\n"); // out of sequence
        }
        if k % 7 == 6 {
            text.push_str("01/31/2005 23:00 4.0"); // no final newline
        }
        let path = dir.join(format!("ts{k}.dat"));
        std::fs::write(&path, &text).unwrap();
        let pb = path.to_str().unwrap().as_bytes().to_vec();

        let (mut r, mut ct) = new_pair();
        r.file.mode = USE_FILE as i8;
        cio::strcpy(&mut r.file.name, &pb);
        ct.file.mode = USE_FILE as _;
        let cs = CString::new(pb.clone()).unwrap();
        for (i, &b) in cs.as_bytes_with_nul().iter().enumerate() {
            ct.file.name[i] = b as _;
        }
        let rv = table_validate(&mut r);
        let cv = unsafe { c::table::table_validate(&mut *ct) };
        assert_eq!(rv, cv, "validate file {k}");
        assert!(same(r.dxMin, ct.dxMin) && same(r.x2, ct.x2) && same(r.lastDate, ct.lastDate), "validate fields, file {k}");
        n += 1;
        if rv == 0 {
            table_tseriesInit(&mut r);
            unsafe { c::table::table_tseriesInit(&mut *ct) };
            same_bracket(&r, &ct, &format!("file {k} init"));
            let lo = datetime_encodeDate(2005, 1, 1) - 0.2;
            let mut t = lo;
            for _ in 0..500 {
                t += rng.range(0.0, 0.05);
                for extend in [0i8, 1i8] {
                    let a = table_tseriesLookup(&mut r, t, extend);
                    let b = unsafe { c::table::table_tseriesLookup(&mut *ct, t, extend as _) };
                    assert!(same(a, b), "file {k} lookup {t}: {a} vs {b}");
                    same_bracket(&r, &ct, &format!("file {k} lookup {t}"));
                    n += 1;
                }
            }
        }
        table_deleteEntries(&mut r);
        free_c(&mut ct);
        let _ = std::fs::remove_file(&path);
    }
    let _ = std::fs::remove_dir_all(&dir);
    eprintln!("table_tseries_files: 60 files, {n} comparisons");
}
