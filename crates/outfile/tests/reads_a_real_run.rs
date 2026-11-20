//! The reader against a real run of a model whose answers are known from its input:
//! a steady 1 cfs inflow, 1 in/hr on one impervious acre, a 5 cfs pump.
//!
//! The wider checks (every value against OWA's swmm-toolkit and against the engine's own
//! report tables, on real models) are in tools/check_outfile.py; see the crate README.
use std::path::PathBuf;
use swmm_out::{iso, Kind, OutFile, PROP_AREA, PROP_INVERT, PROP_LENGTH, PROP_MAX_DEPTH};

fn run(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("swmm-out-test-{}-{tag}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let inp = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/tiny.inp");
    let b = |p: &PathBuf| p.to_string_lossy().into_owned().into_bytes();
    let mut prj = swmm5_rust::Project::new();
    let code = swmm5_rust::swmm5::swmm_run(
        &mut prj,
        &b(&inp),
        &b(&dir.join("tiny.rpt")),
        &b(&dir.join("tiny.out")),
    );
    assert_eq!(code, 0, "the engine reported an error");
    dir.join("tiny.out")
}

#[test]
fn header_names_types_and_properties() {
    let f = OutFile::open(run("header")).unwrap();
    assert_eq!(f.version, 52004);
    assert_eq!(f.flow_units(), "CFS");
    assert!(f.is_us() && f.complete);
    assert_eq!(f.error_code, Some(0));
    assert_eq!(f.subcatchments, ["S1"]);
    assert_eq!(f.nodes, ["J1", "J2", "J3", "O1", "WET1"]);
    assert_eq!(f.links, ["C1", "C2", "C3", "P1"]);
    let types: Vec<_> = (0..5).map(|i| f.node_type(i).unwrap()).collect();
    assert_eq!(
        types,
        ["JUNCTION", "JUNCTION", "JUNCTION", "OUTFALL", "STORAGE"]
    );
    let types: Vec<_> = (0..4).map(|i| f.link_type(i).unwrap()).collect();
    assert_eq!(types, ["CONDUIT", "CONDUIT", "CONDUIT", "PUMP"]);
    // properties as the input file gives them
    assert_eq!(f.subcatchment_props.get(0, PROP_AREA, 0), Some(1.0));
    assert_eq!(f.node_props.get(0, PROP_INVERT, 0), Some(100.0));
    assert_eq!(f.node_props.get(4, PROP_MAX_DEPTH, 0), Some(12.0));
    assert_eq!(f.link_props.get(0, PROP_LENGTH, 0), Some(400.0));
    assert_eq!(f.link_props.get(0, PROP_MAX_DEPTH, 0), Some(1.5));
    assert_eq!(f.link_props.get(3, PROP_LENGTH, 0), Some(0.0)); // a pump has no length
    let names: Vec<_> = f
        .variables(Kind::Node)
        .into_iter()
        .map(|v| (v.name, v.unit))
        .collect();
    assert_eq!(names[0], ("depth".to_string(), "ft".to_string()));
    assert_eq!(names[5], ("flooding".to_string(), "CFS".to_string()));
    assert_eq!(f.variables(Kind::System).len(), 15);
}

#[test]
fn time_base_is_two_hours_of_one_minute_periods() {
    let mut f = OutFile::open(run("time")).unwrap();
    assert_eq!((f.n_periods, f.report_step), (120, 60));
    let dates = f.dates().unwrap();
    assert_eq!(iso(dates[0]), "2026-06-01T00:01:00");
    assert_eq!(iso(dates[119]), "2026-06-01T02:00:00");
    assert_eq!(iso(f.start_date), "2026-06-01T00:00:00");
    // the stored dates are the reporting clock
    for (k, d) in dates.iter().enumerate() {
        assert!((d - f.nominal_date(k)).abs() * 86400.0 < 0.01);
    }
}

#[test]
fn values_the_input_file_fixes() {
    let mut f = OutFile::open(run("values")).unwrap();
    // rain: the series peaks at exactly 1 in/hr
    let rain = f
        .series_by_name(Kind::Subcatchment, "S1", "rainfall")
        .unwrap();
    assert_eq!(rain.iter().cloned().fold(0.0f32, f32::max), 1.0);
    // 1 in/hr on one impervious acre can give at most 43560/12/3600 = 1.0083 cfs
    let runoff = f
        .series_by_name(Kind::Subcatchment, "S1", "runoff")
        .unwrap();
    let peak = runoff.iter().cloned().fold(0.0f32, f32::max);
    assert!(peak > 0.9 && peak < 1.0084, "runoff peak {peak}");
    // long after the storm the pipes carry the steady 1 cfs inflow
    for pipe in ["C1", "C2"] {
        let q = f.series_by_name(Kind::Link, pipe, "flow").unwrap();
        assert!((q[119] - 1.0).abs() < 0.01, "{pipe} ends at {}", q[119]);
    }
    // the pump curve allows 5 cfs and no more; it is on or off
    let p = f.series_by_name(Kind::Link, "P1", "flow").unwrap();
    assert!(p.iter().all(|&q| q == 0.0 || q == 5.0));
    assert!(p.contains(&5.0));
    // head = invert + depth, at every period (single precision)
    let depth = f.series_by_name(Kind::Node, "J2", "depth").unwrap();
    let head = f.series_by_name(Kind::Node, "J2", "head").unwrap();
    assert!(depth
        .iter()
        .zip(&head)
        .all(|(d, h)| (99.0 + d - h).abs() < 1e-4));
    // the system's inflow column is the steady inflow plus the runoff
    let ext = f
        .series_by_name(Kind::System, "", "external_inflow")
        .unwrap();
    assert!(ext.iter().all(|&q| (q - 1.0).abs() < 1e-6));
}

#[test]
fn the_three_ways_of_reading_agree() {
    let mut f = OutFile::open(run("ways")).unwrap();
    let stats = f.stats(Kind::Link).unwrap();
    for (i, id) in f.links.clone().iter().enumerate() {
        for v in 0..f.variables(Kind::Link).len() {
            let s = f.series(Kind::Link, i, v).unwrap();
            let max = s.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            assert_eq!(stats[i][v].max, max, "{id} variable {v}");
            assert_eq!(s[stats[i][v].max_period], max);
            assert_eq!(
                stats[i][v].max_period,
                s.iter().position(|&x| x == max).unwrap()
            );
            assert_eq!(stats[i][v].last, s[119]);
            assert_eq!(
                stats[i][v].positive_periods,
                s.iter().filter(|&&x| x > 0.0).count()
            );
            let snap = f.snapshot(Kind::Link, v, 77).unwrap();
            assert_eq!(snap[i], s[77]);
        }
    }
    assert!(f.series(Kind::Node, 99, 0).is_err());
    assert!(f.series(Kind::Node, 0, 99).is_err());
    assert!(f.series_by_name(Kind::Node, "nobody", "depth").is_err());
}

#[test]
fn a_run_that_did_not_finish() {
    let path = run("partial");
    let bytes = std::fs::read(&path).unwrap();
    // cut the file in the middle of period 51
    let whole = OutFile::from_bytes(bytes.clone()).unwrap();
    let cut = whole.results_pos as usize + 50 * whole.period_bytes as usize + 123;
    assert!(OutFile::from_bytes(bytes[..cut].to_vec()).is_err());
    let mut part = OutFile::new(std::io::Cursor::new(bytes[..cut].to_vec()), true).unwrap();
    assert!(!part.complete);
    assert_eq!((part.n_periods, part.error_code), (50, None));
    let mut whole = whole;
    assert_eq!(
        part.series(Kind::Link, 0, 0).unwrap(),
        whole.series(Kind::Link, 0, 0).unwrap()[..50]
    );
    // a file that lies about its layout is refused: move the results position in the closing record
    let mut lie = bytes.clone();
    let n = lie.len();
    lie[n - 16..n - 12].copy_from_slice(&999i32.to_le_bytes());
    assert!(OutFile::from_bytes(lie).is_err());
}
