//! lockstep: run one model in the c2rust baseline (EPA's C, statement for
//! statement) and in the safe engine side by side, one routing step at a
//! time, and report the first state variable whose bits differ.
//!
//!   lockstep MODEL.inp [MAX_REPORTED]
//!
//! This is how a byte difference in a report becomes one line of code: the
//! first diverging variable, object and time step point at the function that
//! computed it.
use std::ffi::CString;
use swmm5_c2rust::engine::swmm5 as c;
use swmm5_rust::swmm5 as r;
use swmm5_rust::Project;

fn same(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
}

struct Diff {
    n: usize,
    max: usize,
}
impl Diff {
    fn check(&mut self, step: i64, t: f64, what: &str, id: &str, a: f64, b: f64) {
        if !same(a, b) {
            if self.n < self.max {
                println!(
                    "step {:>8}  t={:.10}  {:<28} {:<24} C={:<24e} Rust={:<24e}",
                    step, t, what, id, a, b
                );
            }
            self.n += 1;
        }
    }
}

fn cid(p: *mut std::ffi::c_char) -> String {
    if p.is_null() {
        return String::new();
    }
    unsafe { std::ffi::CStr::from_ptr(p) }.to_string_lossy().into_owned()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let inp = &args[1];
    let max: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);
    let dir = std::env::temp_dir().join(format!("lockstep_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let crpt = CString::new(dir.join("c.rpt").to_str().unwrap()).unwrap();
    let cout = CString::new(dir.join("c.out").to_str().unwrap()).unwrap();
    let cinp = CString::new(inp.as_str()).unwrap();
    let rrpt = dir.join("r.rpt");
    let rout = dir.join("r.out");

    // --- open and start both ---
    let ec = unsafe { c::swmm_open(cinp.as_ptr() as *mut _, crpt.as_ptr() as *mut _, cout.as_ptr() as *mut _) };
    let mut prj = Project::new();
    let er = r::swmm_open(&mut prj, inp.as_bytes(), rrpt.to_str().unwrap().as_bytes(), rout.to_str().unwrap().as_bytes());
    println!("open: C={} Rust={}", ec, er);
    if ec != 0 || er != 0 {
        return;
    }
    let ec = unsafe { c::swmm_start(1) };
    let er = r::swmm_start(&mut prj, 1);
    println!("start: C={} Rust={}", ec, er);
    if ec != 0 || er != 0 {
        return;
    }
    let mut d = Diff { n: 0, max };
    let mut step: i64 = 0;
    loop {
        let mut tc = 0.0f64;
        let mut tr = 0.0f64;
        let ec = unsafe { c::swmm_step(&mut tc) };
        let er = r::swmm_step(&mut prj, &mut tr);
        step += 1;
        if ec != er {
            println!("step {} return codes differ: C={} Rust={}", step, ec, er);
            break;
        }
        d.check(step, tc, "elapsedTime", "", tc, tr);
        unsafe {
            let n = c::Nobjects;
            d.check(step, tc, "NewRoutingTime", "", c::NewRoutingTime, prj.NewRoutingTime);
            d.check(step, tc, "NewRunoffTime", "", c::NewRunoffTime, prj.NewRunoffTime);
            let npol = n[swmm5_rust::enums::POLLUT as usize] as usize;
            for j in 0..n[swmm5_rust::enums::GAGE as usize] as usize {
                let g = &*c::Gage.add(j);
                let id = cid(g.ID);
                d.check(step, tc, "Gage.rainfall", &id, g.rainfall, prj.Gage[j].rainfall);
            }
            for j in 0..n[swmm5_rust::enums::SUBCATCH as usize] as usize {
                let s = &*c::Subcatch.add(j);
                let rs = &prj.Subcatch[j];
                let id = cid(s.ID);
                d.check(step, tc, "Subcatch.rainfall", &id, s.rainfall, rs.rainfall);
                d.check(step, tc, "Subcatch.newRunoff", &id, s.newRunoff, rs.newRunoff);
                d.check(step, tc, "Subcatch.evapLoss", &id, s.evapLoss, rs.evapLoss);
                d.check(step, tc, "Subcatch.infilLoss", &id, s.infilLoss, rs.infilLoss);
                d.check(step, tc, "Subcatch.runon", &id, s.runon, rs.runon);
                d.check(step, tc, "Subcatch.newSnowDepth", &id, s.newSnowDepth, rs.newSnowDepth);
                for k in 0..3 {
                    d.check(step, tc, "Subcatch.subArea.depth", &id, s.subArea[k].depth, rs.subArea[k].depth);
                }
                for p in 0..npol {
                    d.check(step, tc, "Subcatch.newQual", &id, *s.newQual.add(p), rs.newQual[p]);
                }
            }
            for j in 0..n[swmm5_rust::enums::NODE as usize] as usize {
                let x = &*c::Node.add(j);
                let y = &prj.Node[j];
                let id = cid(x.ID);
                d.check(step, tc, "Node.newDepth", &id, x.newDepth, y.newDepth);
                d.check(step, tc, "Node.newVolume", &id, x.newVolume, y.newVolume);
                d.check(step, tc, "Node.inflow", &id, x.inflow, y.inflow);
                d.check(step, tc, "Node.outflow", &id, x.outflow, y.outflow);
                d.check(step, tc, "Node.overflow", &id, x.overflow, y.overflow);
                d.check(step, tc, "Node.newLatFlow", &id, x.newLatFlow, y.newLatFlow);
                d.check(step, tc, "Node.losses", &id, x.losses, y.losses);
                for p in 0..npol {
                    d.check(step, tc, "Node.newQual", &id, *x.newQual.add(p), y.newQual[p]);
                }
            }
            for j in 0..n[swmm5_rust::enums::LINK as usize] as usize {
                let x = &*c::Link.add(j);
                let y = &prj.Link[j];
                let id = cid(x.ID);
                d.check(step, tc, "Link.newFlow", &id, x.newFlow, y.newFlow);
                d.check(step, tc, "Link.newDepth", &id, x.newDepth, y.newDepth);
                d.check(step, tc, "Link.newVolume", &id, x.newVolume, y.newVolume);
                d.check(step, tc, "Link.setting", &id, x.setting, y.setting);
                d.check(step, tc, "Link.froude", &id, x.froude, y.froude);
                d.check(step, tc, "Link.dqdh", &id, x.dqdh, y.dqdh);
                for p in 0..npol {
                    d.check(step, tc, "Link.newQual", &id, *x.newQual.add(p), y.newQual[p]);
                }
            }
        }
        if d.n > 0 {
            println!("{} differences in step {}; stopping here.", d.n, step);
            break;
        }
        if tc == 0.0 || ec != 0 {
            println!("no state difference in {} steps", step);
            break;
        }
    }
    unsafe {
        c::swmm_end();
        c::swmm_report();
        c::swmm_close();
    }
    r::swmm_end(&mut prj);
    r::swmm_report(&mut prj);
    r::swmm_close(&mut prj);
    let a = std::fs::read(dir.join("c.out")).unwrap_or_default();
    let b = std::fs::read(&rout).unwrap_or_default();
    println!(".out files: {} ({} vs {} bytes)", if a == b { "identical" } else { "DIFFER" }, a.len(), b.len());
    let norm = |s: Vec<u8>| -> Vec<Vec<u8>> {
        s.split(|&c| c == b'\n')
            .filter(|l| !(l.windows(17).any(|w| w == b"Analysis begun on") || l.windows(17).any(|w| w == b"Analysis ended on") || l.windows(18).any(|w| w == b"Total elapsed time")))
            .map(|l| l.to_vec())
            .collect()
    };
    let ra = norm(std::fs::read(dir.join("c.rpt")).unwrap_or_default());
    let rb = norm(std::fs::read(&rrpt).unwrap_or_default());
    match ra.iter().zip(rb.iter()).position(|(x, y)| x != y) {
        None if ra.len() == rb.len() => println!(".rpt files: identical ({} lines)", ra.len()),
        None => println!(".rpt files: DIFFER in length ({} vs {} lines)", ra.len(), rb.len()),
        Some(i) => {
            println!(".rpt files: first difference at line {}:", i + 1);
            println!("  C:    {}", String::from_utf8_lossy(&ra[i]));
            println!("  Rust: {}", String::from_utf8_lossy(&rb[i]));
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}
