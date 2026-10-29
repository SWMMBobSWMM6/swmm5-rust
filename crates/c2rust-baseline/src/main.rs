//! runswmm-c2rust: EPA's src/run/main.c over the c2rust engine.
use std::ffi::CString;
use swmm5_c2rust::engine::swmm5::{swmm_getError, swmm_getVersion, swmm_getWarnings, swmm_run};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let version = unsafe { swmm_getVersion() };
    let v_major = version / 10000;
    let v_minor = (version - 10000 * v_major) / 1000;
    let v_release = version - 10000 * v_major - 1000 * v_minor;
    let start = std::time::Instant::now();
    if args.len() < 3 {
        println!("\nNot Enough Arguments\n");
        return;
    }
    let inp = CString::new(args[1].as_str()).unwrap();
    let rpt = CString::new(args[2].as_str()).unwrap();
    let out = CString::new(if args.len() > 3 { args[3].as_str() } else { "" }).unwrap();
    println!("\n... EPA SWMM {}.{} (Build {}.{}.{}) [c2rust]", v_major, v_minor, v_major, v_minor, v_release);
    unsafe { swmm_run(inp.as_ptr() as *mut _, rpt.as_ptr() as *mut _, out.as_ptr() as *mut _) };
    print!("\n\n... EPA SWMM completed in {:.2} seconds.", start.elapsed().as_secs_f64());
    let mut msg = [0 as std::ffi::c_char; 128];
    if unsafe { swmm_getError(msg.as_mut_ptr(), 127) } > 0 {
        println!(" There are errors.");
    } else if unsafe { swmm_getWarnings() } > 0 {
        println!(" There are warnings.");
    } else {
        println!();
    }
}
