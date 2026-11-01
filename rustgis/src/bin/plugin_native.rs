//! The plugin's code, run natively for testing (memfs, no RustGIS):
//!   plugin-native MODEL.inp [--epsg N] [--report] [--map] [--data FILE]... [--rpt FILE] [--out FILE]
//! prints the tool's reply (`{"ok": ...}` or `{"error": ...}`) as RustGIS would get it,
//! and with --rpt/--out writes the engine's report and output files.
use swmm5_rust::memfs;
use swmm5_rust_rustgis::{ANCHOR, Job, execute, run_engine};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let path = a.get(1).expect("usage: plugin-native MODEL.inp [--epsg N] [--report] [--map] [--data FILE]... [--rpt FILE] [--out FILE]");
    let val = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned();
    let bytes = std::fs::read(path).expect("read the model");
    let data: Vec<(String, Vec<u8>)> = a
        .windows(2)
        .filter(|w| w[0] == "--data")
        .map(|w| {
            let p = std::path::Path::new(&w[1]);
            (p.file_name().unwrap().to_string_lossy().into_owned(), std::fs::read(p).expect("read a data file"))
        })
        .collect();
    if let (Some(r), o) = (val("--rpt"), val("--out")) {
        let ran = run_engine(&bytes, &data);
        std::fs::write(&r, &ran.rpt).unwrap();
        if let Some(o) = o {
            std::fs::write(&o, memfs::get(b"model.out").unwrap_or_default()).unwrap();
        }
        eprintln!("code {}", ran.code);
        return;
    }
    let job = Job {
        name: std::path::Path::new(path).file_name().unwrap().to_string_lossy().into_owned(),
        bytes,
        data,
        epsg: val("--epsg").and_then(|s| s.parse().ok()),
        anchor: ANCHOR,
        report: a.iter().any(|x| x == "--report"),
        run: !a.iter().any(|x| x == "--map"),
    };
    let reply = match execute(&job) {
        Ok(v) => serde_json::json!({ "ok": v }),
        Err(e) => serde_json::json!({ "error": e }),
    };
    // (the engine's console messages go to stdout, as runswmm's do; the reply goes to stderr)
    eprintln!("{reply}");
}
