//! swmm5-gpkg: run a SWMM 5 model and put its nodes, links and subcatchments, with results, in a GeoPackage.
#![forbid(unsafe_code)]
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use swmm5_gpkg::{export, run_engine, Options, Report};

const HELP: &str = "\
swmm5-gpkg: a SWMM 5 model and its results as a GeoPackage, for RustGIS or any GIS that reads one

  swmm5-gpkg run    MODEL.inp [-o FILE.gpkg] [--epsg N]
      runs the model with swmm5-rust (writes MODEL.rpt and MODEL.out beside it), then writes the GeoPackage,
      including the engine's maxima over every routing step (the rpt_* columns)
  swmm5-gpkg export MODEL.inp RESULTS.out [-o FILE.gpkg] [--epsg N]
      writes the GeoPackage from results you already have, from any engine that writes SWMM 5's .out;
      the rpt_* columns stay empty, because a .out holds only the saved reporting periods

  --epsg N   the EPSG code of the model's coordinates. A SWMM input file does not say what they are;
             without it the layers carry no coordinate reference and the GIS will ask for one.
  -o FILE    the GeoPackage to write (default: MODEL.gpkg beside the model)
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("swmm5-gpkg: {e}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut opt = Options::default();
    let mut gpkg: Option<PathBuf> = None;
    let mut rest: Vec<&str> = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--epsg" => {
                let v = it
                    .next()
                    .ok_or("--epsg needs a number, such as --epsg 3435")?;
                opt.epsg = Some(
                    v.trim_start_matches("EPSG:")
                        .trim_start_matches("epsg:")
                        .parse()
                        .map_err(|_| format!("'{v}' is not an EPSG code"))?,
                );
            }
            "-o" | "--output" => {
                gpkg = Some(PathBuf::from(it.next().ok_or("-o needs a file name")?))
            }
            "-h" | "--help" => {
                print!("{HELP}");
                return Ok(());
            }
            other => rest.push(other),
        }
    }
    let (inp, out, maxima) = match rest.as_slice() {
        ["run", inp] => {
            let inp = Path::new(inp);
            let (rpt, out) = (inp.with_extension("rpt"), inp.with_extension("out"));
            println!("running {} with swmm5-rust ...", inp.display());
            let started = std::time::Instant::now();
            let maxima = run_engine(inp, &rpt, &out)?;
            println!(
                "  done in {:.1} s, {} warning(s); report {}, results {}",
                started.elapsed().as_secs_f64(),
                maxima.warnings,
                rpt.display(),
                out.display()
            );
            (inp.to_path_buf(), out, Some(maxima))
        }
        ["export", inp, out] => (PathBuf::from(inp), PathBuf::from(out), None),
        _ => {
            print!("{HELP}");
            return Ok(());
        }
    };
    let gpkg = gpkg.unwrap_or_else(|| inp.with_extension("gpkg"));
    let report = export(&inp, &out, &gpkg, &opt, maxima.as_ref())?;
    if maxima.is_none() {
        println!("  rpt_* columns left empty: a .out does not hold the every-routing-step maxima (use `run` for those)");
    }
    print_report(&gpkg, &opt, &report);
    Ok(())
}

fn print_report(gpkg: &Path, opt: &Options, r: &Report) {
    println!("wrote {}", gpkg.display());
    println!(
        "  nodes          {:>6} drawn, {} with results",
        r.nodes, r.nodes_with_results
    );
    println!(
        "  links          {:>6} drawn, {} with results",
        r.links, r.links_with_results
    );
    println!(
        "  subcatchments  {:>6} drawn, {} with results",
        r.subcatchments, r.subcatchments_with_results
    );
    let list = |what: &str, ids: &[String]| {
        if !ids.is_empty() {
            let shown: Vec<&str> = ids.iter().take(8).map(String::as_str).collect();
            println!(
                "  not drawn: {} {what} ({}{})",
                ids.len(),
                shown.join(", "),
                if ids.len() > 8 { ", ..." } else { "" }
            );
        }
    };
    list(
        "node(s) with no [COORDINATES] line",
        &r.nodes_without_coordinates,
    );
    list(
        "link(s) with an end node that has no coordinates",
        &r.links_without_geometry,
    );
    list(
        "subcatchment(s) with fewer than three [POLYGONS] points",
        &r.subcatchments_without_polygon,
    );
    for n in &r.notes {
        println!("  note: {n}");
    }
    match opt.epsg {
        Some(e) => println!("  coordinates written as EPSG:{e}"),
        None => println!("  no coordinate reference written (give --epsg N if you know it)"),
    }
}
