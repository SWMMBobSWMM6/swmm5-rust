//! transect_readParams / transect_validate: the safe engine against the
//! c2rust baseline. The baseline reads a generated .inp file through
//! swmm_open; the safe engine gets the same tokens line by line, as
//! input.c's parseLine would hand them over.
//!
//! The safe side needs findmatch, getDouble (input.rs), project_findID
//! (project.rs), UCF (swmm5.rs) and report_writeErrorMsg (report.rs); until
//! those modules are translated the test reports itself skipped.
#![allow(non_snake_case)]

use std::ffi::CString;
use swmm5_c2rust::engine as c;
use swmm5_difftest::{same, Rng};
use swmm5_rust::prelude::*;

fn stubs_ready() -> bool {
    std::panic::catch_unwind(|| {
        let mut prj = Project::default();
        let mut v = 0.0;
        getDouble(b"1", &mut v);
        findmatch(b"NC", TransectKeyWords);
        UCF(&prj, LENGTH);
        let _ = project_findID(&prj, TRANSECT, b"x");
        report_writeErrorMsg(&mut prj, ERR_TRANSECT_TOO_FEW, b"x");
    })
    .is_ok()
}

fn fmt(x: f64) -> String {
    format!("{}", x)
}

/// One model's [TRANSECTS] section: a mix of ordinary and odd transects.
fn gen_transects(rng: &mut Rng, ntr: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for t in 0..ntr {
        // NC line (sometimes omitted, so the previous n values carry over)
        if t == 0 || rng.next() % 4 != 0 {
            let nl = if rng.next() % 4 == 0 { 0.0 } else { (rng.range(0.02, 0.1) * 1000.0).round() / 1000.0 };
            let nr = if rng.next() % 4 == 0 { 0.0 } else { (rng.range(0.02, 0.1) * 1000.0).round() / 1000.0 };
            let nc = if rng.next() % 25 == 0 { 0.0 } else { (rng.range(0.01, 0.05) * 1000.0).round() / 1000.0 };
            lines.push(format!("NC {} {} {}", fmt(nl), fmt(nr), fmt(nc)));
        }
        // stations
        let nsta = match rng.next() % 20 {
            0 => 1,
            1 => 2,
            _ => 3 + (rng.next() % 40) as usize,
        };
        let mut sta = Vec::new();
        let mut x = (rng.range(-50.0, 50.0) * 10.0).round() / 10.0;
        for _ in 0..nsta {
            sta.push(x);
            x += match rng.next() % 8 {
                0 => 0.0,
                _ => (rng.range(0.1, 30.0) * 10.0).round() / 10.0,
            };
        }
        let base = (rng.range(0.0, 500.0) * 10.0).round() / 10.0;
        let mut elev: Vec<f64> = (0..nsta)
            .map(|i| {
                let f = i as f64 / (nsta.max(2) - 1) as f64;
                let bowl = (2.0 * f - 1.0).powi(2) * rng.range(2.0, 20.0);
                ((base + bowl + rng.range(-1.0, 1.0)) * 100.0).round() / 100.0
            })
            .collect();
        if rng.next() % 30 == 0 {
            for e in elev.iter_mut() {
                *e = base; // flat: no depth
            }
        }
        let li = (rng.next() as usize) % nsta;
        let ri = li + (rng.next() as usize) % (nsta - li);
        let (mut xl, mut xr) = (sta[li], sta[ri]);
        if rng.next() % 6 == 0 {
            xl = rng.range(-60.0, 60.0);
        }
        if rng.next() % 6 == 0 {
            xr = rng.range(-60.0, 600.0);
        }
        if rng.next() % 20 == 0 {
            std::mem::swap(&mut xl, &mut xr);
        }
        let lf = if rng.next() % 3 == 0 { 0.0 } else { (rng.range(0.5, 2.0) * 100.0).round() / 100.0 };
        let xf = if rng.next() % 3 == 0 { 0.0 } else { (rng.range(0.5, 2.0) * 100.0).round() / 100.0 };
        let yf = if rng.next() % 3 == 0 { 0.0 } else { (rng.range(-5.0, 5.0) * 100.0).round() / 100.0 };
        lines.push(format!("X1 T{t} {nsta} {} {} 0.0 0.0 {} {} {}", fmt(xl), fmt(xr), fmt(lf), fmt(xf), fmt(yf)));
        let mut gr = String::from("GR");
        for i in 0..nsta {
            gr.push_str(&format!(" {} {}", fmt(elev[i]), fmt(sta[i])));
            if i % 5 == 4 && i + 1 < nsta {
                lines.push(gr);
                gr = String::from("GR");
            }
        }
        lines.push(gr);
    }
    lines
}

#[test]
fn transect_tables() {
    if !stubs_ready() {
        eprintln!("transect_tables: SKIPPED (input/project/swmm5/report stubs not yet translated)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("swmm5_difftest_transects_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut rng = Rng(0x7a5ec7);
    let mut ntables = 0usize;
    let mut nvalid = 0usize;
    // transect.c's file-scope variables persist from one model to the next
    // in the baseline (one process); carry the safe engine's along likewise.
    let mut tstate = swmm5_rust::transect::TransectState::default();
    for m in 0..40 {
        let si = m % 2 == 1;
        let ntr = 1 + (rng.next() % 30) as usize;
        let lines = gen_transects(&mut rng, ntr);
        let mut inp = String::new();
        inp.push_str("[OPTIONS]\n");
        inp.push_str(if si { "FLOW_UNITS CMS\n" } else { "FLOW_UNITS CFS\n" });
        inp.push_str("[TRANSECTS]\n");
        for l in lines.iter() {
            inp.push_str(l);
            inp.push('\n');
        }
        inp.push_str("[JUNCTIONS]\nJ1 0 5\n[OUTFALLS]\nO1 0 FREE\n");
        let fi = dir.join(format!("m{m}.inp"));
        let fr = dir.join(format!("m{m}.rpt"));
        let fo = dir.join(format!("m{m}.out"));
        std::fs::write(&fi, &inp).unwrap();

        // --- the baseline
        let (c1, c2, c3) = (
            CString::new(fi.to_str().unwrap()).unwrap(),
            CString::new(fr.to_str().unwrap()).unwrap(),
            CString::new(fo.to_str().unwrap()).unwrap(),
        );
        unsafe { c::swmm5::swmm_open(c1.as_ptr(), c2.as_ptr(), c3.as_ptr()) };
        let cn = unsafe { c::swmm5::Nobjects[TRANSECT as usize] } as usize;
        assert_eq!(cn, ntr, "model {m}: baseline transect count");
        let ctrans: Vec<c::swmm5::TTransect> = unsafe { (0..cn).map(|j| *c::swmm5::Transect.add(j)).collect() };

        // --- the safe engine, given the same tokens
        let mut prj = Project::default();
        prj.transect = std::mem::take(&mut tstate);
        prj.UnitSystem = if si { SI } else { US };
        prj.FlowUnits = if si { CMS } else { CFS };
        prj.Nobjects[TRANSECT as usize] = ntr as i32;
        assert_eq!(transect_create(&mut prj, ntr as i32), 0);
        let mut count: i32 = 0;
        for l in lines.iter() {
            let tok: Vec<Vec<u8>> = l.split_whitespace().map(|t| t.as_bytes().to_vec()).collect();
            let _ = transect_readParams(&mut prj, &mut count, &tok, tok.len() as i32);
        }
        // the next section header finishes the last transect
        transect_validate(&mut prj, ntr as i32 - 1);

        for j in 0..ntr {
            let r = &prj.Transect[j];
            let t = &ctrans[j];
            let ctx = format!("model {m} transect {j}");
            let rs = [r.yFull, r.aFull, r.rFull, r.wMax, r.ywMax, r.sMax, r.aMax, r.lengthFactor, r.roughness];
            let cs = [t.yFull, t.aFull, t.rFull, t.wMax, t.ywMax, t.sMax, t.aMax, t.lengthFactor, t.roughness];
            for k in 0..rs.len() {
                assert!(same(rs[k], cs[k]), "{ctx}: scalar {k}: {} vs {}", rs[k], cs[k]);
            }
            assert_eq!(r.nTbl, t.nTbl, "{ctx}: nTbl");
            for i in 0..51 {
                assert!(same(r.areaTbl[i], t.areaTbl[i]), "{ctx}: areaTbl[{i}]");
                assert!(same(r.hradTbl[i], t.hradTbl[i]), "{ctx}: hradTbl[{i}] {} vs {}", r.hradTbl[i], t.hradTbl[i]);
                assert!(same(r.widthTbl[i], t.widthTbl[i]), "{ctx}: widthTbl[{i}]");
            }
            ntables += 1;
            if t.yFull > 0.0 {
                nvalid += 1;
            }
        }
        tstate = std::mem::take(&mut prj.transect);
        unsafe { c::swmm5::swmm_close() };
        let _ = std::fs::remove_file(&fi);
        let _ = std::fs::remove_file(&fr);
        let _ = std::fs::remove_file(&fo);
    }
    let _ = std::fs::remove_dir_all(&dir);
    eprintln!("transect_tables: 40 models, {ntables} transects compared ({nvalid} with geometry tables, the rest rejected by validation in both)");
}
