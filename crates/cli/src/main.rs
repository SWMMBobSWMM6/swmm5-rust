//! runswmm-rs: EPA's src/run/main.c over swmm5-rust. Same arguments, same
//! console messages, same exit code (0).
#![forbid(unsafe_code)]
use swmm5_rust::swmm5::{swmm_getError, swmm_getVersion, swmm_getWarnings, swmm_run};
use swmm5_rust::Project;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut prj = Project::new();
    let version = swmm_getVersion(&mut prj);
    let v_major = version / 10000;
    let v_minor = (version - 10000 * v_major) / 1000;
    let v_release = version - 10000 * v_major - 1000 * v_minor;
    let start = std::time::Instant::now();
    match args.len() {
        1 => println!("\nNot Enough Arguments (See Help --help)\n"),
        2 => match args[1].as_str() {
            "--help" | "-h" => {
                println!("\n\nSTORMWATER MANAGEMENT MODEL (SWMM) HELP\n");
                println!("COMMANDS:");
                println!("\t--help (-h)       SWMM Help");
                println!("\t--version (-v)    Build Version");
                println!("\nRUNNING A SIMULATION:");
                println!("\t runswmm <input file> <report file> <optional output file>\n");
            }
            "--version" | "-v" => println!("\n{}.{}.{}\n", v_major, v_minor, v_release),
            _ => println!("\nUnknown Argument (See Help --help)\n"),
        },
        _ => {
            let inp = args[1].as_bytes();
            let rpt = args[2].as_bytes();
            let out: &[u8] = if args.len() > 3 { args[3].as_bytes() } else { b"" };
            println!("\n... EPA SWMM {}.{} (Build {}.{}.{}) [swmm5-rust]", v_major, v_minor, v_major, v_minor, v_release);
            swmm_run(&mut prj, inp, rpt, out);
            let run_time = start.elapsed().as_secs() as f64;
            print!("\n\n... EPA SWMM completed in {:.2} seconds.", run_time);
            let mut msg = Vec::new();
            if swmm_getError(&mut prj, &mut msg, 127) > 0 {
                println!(" There are errors.");
            } else if swmm_getWarnings(&mut prj) > 0 {
                println!(" There are warnings.");
            } else {
                println!();
            }
        }
    }
}
