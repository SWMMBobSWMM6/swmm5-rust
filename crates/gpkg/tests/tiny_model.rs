//! Run the small fixture with the engine, write its GeoPackage, and read the file back with
//! terrio. What the file must hold is fixed by the input file (where things are, what they
//! are), by the model (a steady 1 cfs, a 5 cfs pump), and by the reader's own direct answers.
//!
//! The wider checks (decoding the file with sqlite3 and GDAL, comparing every attribute with
//! OWA's swmm-toolkit and with the .rpt summary tables, on real models) are in
//! tools/check_gpkg.py; see the crate README.
use std::path::PathBuf;
use swmm5_gpkg::{export, run_engine, Options};
use swmm_out::{iso, Kind, OutFile};
use terrio::{FieldValue, Geometry, Layer};

struct Made {
    dir: PathBuf,
    layers: Vec<Layer>,
}

fn make(tag: &str, epsg: Option<u32>, with_maxima: bool) -> Made {
    let dir = std::env::temp_dir().join(format!("swmm5-gpkg-test-{}-{tag}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let inp = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../outfile/tests/data/tiny.inp");
    let (rpt, out, gpkg) = (
        dir.join("tiny.rpt"),
        dir.join("tiny.out"),
        dir.join("tiny.gpkg"),
    );
    let maxima = run_engine(&inp, &rpt, &out).unwrap();
    let opt = Options {
        epsg,
        ..Default::default()
    };
    let report = export(&inp, &out, &gpkg, &opt, with_maxima.then_some(&maxima)).unwrap();
    assert_eq!(
        (report.nodes, report.links, report.subcatchments),
        (5, 4, 1)
    );
    assert_eq!(
        (report.nodes_with_results, report.links_with_results),
        (5, 4)
    );
    assert!(
        report.nodes_without_coordinates.is_empty() && report.links_without_geometry.is_empty()
    );
    let layers = terrio::open_vector(&gpkg.to_string_lossy(), &Default::default())
        .unwrap()
        .layers;
    Made { dir, layers }
}

fn layer<'a>(m: &'a Made, name: &str) -> &'a Layer {
    m.layers
        .iter()
        .find(|l| l.name == name)
        .unwrap_or_else(|| panic!("no layer {name}"))
}

fn value<'a>(l: &'a Layer, id: &str, field: &str) -> &'a FieldValue {
    let col = l
        .schema
        .fields()
        .iter()
        .position(|f| f.name == field)
        .unwrap_or_else(|| panic!("no field {field}"));
    let idc = l
        .schema
        .fields()
        .iter()
        .position(|f| f.name == "id")
        .unwrap();
    let row = l
        .features
        .iter()
        .find(|f| f.attributes[idc] == FieldValue::Text(id.into()))
        .unwrap_or_else(|| panic!("no {id}"));
    &row.attributes[col]
}

fn num(l: &Layer, id: &str, field: &str) -> f64 {
    match value(l, id, field) {
        FieldValue::Float(v) => *v,
        other => panic!("{id}.{field} is {other:?}"),
    }
}

#[test]
fn the_geopackage_is_valid_and_holds_the_model() {
    let m = make("valid", Some(2229), true);
    let check = terrio::validate_gpkg(
        m.dir.join("tiny.gpkg"),
        &terrio::GpkgValidationOptions {
            full_check: true,
            strict: true,
            unicode_case: false,
        },
    )
    .unwrap();
    assert!(
        check.is_valid && check.is_complete,
        "errors {:?} warnings {:?} unsupported {:?}",
        check.errors,
        check.warnings,
        check.unsupported_checks
    );

    let (nodes, links, subs) = (
        layer(&m, "nodes"),
        layer(&m, "links"),
        layer(&m, "subcatchments"),
    );
    assert_eq!(
        (
            nodes.features.len(),
            links.features.len(),
            subs.features.len()
        ),
        (5, 4, 1)
    );
    for l in [nodes, links, subs] {
        assert_eq!(l.crs.as_ref().and_then(|c| c.epsg), Some(2229));
    }
    assert_eq!(
        layer(&m, "swmm_fields").features.len(),
        nodes.schema.len() + links.schema.len() + subs.schema.len()
    );

    // where things are: the numbers of [COORDINATES], [VERTICES] and [POLYGONS]
    let geom = |l: &Layer, id: &str| {
        let idc = l
            .schema
            .fields()
            .iter()
            .position(|f| f.name == "id")
            .unwrap();
        l.features
            .iter()
            .find(|f| f.attributes[idc] == FieldValue::Text(id.into()))
            .unwrap()
            .geometry
            .clone()
            .unwrap()
    };
    assert_eq!(geom(nodes, "O1"), Geometry::point(1100.0, 50.0));
    let xy = |g: &Geometry| -> Vec<(f64, f64)> {
        match g {
            Geometry::LineString(c) => c.iter().map(|c| (c.x, c.y)).collect(),
            Geometry::Polygon { exterior, .. } => exterior.0.iter().map(|c| (c.x, c.y)).collect(),
            other => panic!("{other:?}"),
        }
    };
    assert_eq!(
        xy(&geom(links, "C2")),
        [(400.0, 0.0), (600.0, 40.0), (700.0, 40.0), (800.0, 0.0)]
    );
    assert_eq!(xy(&geom(links, "P1")), [(1000.0, 0.0), (1100.0, 50.0)]);
    // (terrio hands a ring back without its repeated closing point; the file holds it closed,
    //  which tools/check_gpkg.py reads from the bytes)
    let mut ring = xy(&geom(subs, "S1"));
    if ring.first() == ring.last() {
        ring.pop();
    }
    assert_eq!(
        ring,
        [
            (-300.0, -150.0),
            (-300.0, 150.0),
            (0.0, 150.0),
            (0.0, -150.0)
        ]
    );

    // what things are
    assert_eq!(
        value(nodes, "WET1", "type"),
        &FieldValue::Text("STORAGE".into())
    );
    assert_eq!(
        value(nodes, "O1", "type"),
        &FieldValue::Text("OUTFALL".into())
    );
    assert_eq!(value(links, "P1", "type"), &FieldValue::Text("PUMP".into()));
    assert_eq!(
        value(links, "C3", "to_node"),
        &FieldValue::Text("WET1".into())
    );
    assert_eq!(value(subs, "S1", "outlet"), &FieldValue::Text("J1".into()));
    assert_eq!(num(nodes, "J1", "invert_elev"), 100.0);
    assert_eq!(num(links, "C1", "length"), 400.0);
    assert_eq!(num(subs, "S1", "area"), 1.0);

    // results the model fixes
    assert_eq!(num(subs, "S1", "max_rainfall"), 1.0);
    assert_eq!(num(links, "P1", "max_flow"), 5.0);
    assert_eq!(num(links, "P1", "rpt_max_flow"), 5.0);
    assert_eq!(value(links, "P1", "rpt_max_velocity"), &FieldValue::Null); // the report prints none for a pump
    assert_eq!(num(nodes, "J1", "max_flooding"), 0.0);
    assert_eq!(num(nodes, "J1", "hours_flooded"), 0.0);
    // a free outfall with nothing behind it never has depth: the engine's maximum is 0, dated at the start
    assert_eq!(num(nodes, "O1", "rpt_max_depth"), 0.0);
    assert_eq!(
        value(nodes, "O1", "rpt_max_depth_time"),
        &FieldValue::Text("2026-06-01T00:00:00".into())
    );

    // the saved-period peaks are the reader's, value and time
    let mut out = OutFile::open(m.dir.join("tiny.out")).unwrap();
    let dates = out.dates().unwrap();
    let stats = out.stats(Kind::Node).unwrap();
    for (i, id) in out.nodes.clone().iter().enumerate() {
        for (v, var) in out.variables(Kind::Node).iter().enumerate() {
            assert_eq!(
                num(nodes, id, &format!("max_{}", var.name)),
                stats[i][v].max as f64
            );
            let t = value(nodes, id, &format!("max_{}_time", var.name));
            assert_eq!(
                t,
                &FieldValue::Text(iso(dates[stats[i][v].max_period])),
                "{id} {}",
                var.name
            );
        }
    }
    // the every-step maximum can only be at or above the saved-period one
    for id in ["J1", "J2", "J3", "WET1"] {
        assert!(
            num(nodes, id, "rpt_max_depth") >= num(nodes, id, "max_depth") - 1e-6,
            "{id}"
        );
        assert_eq!(
            num(nodes, id, "rpt_max_hgl"),
            num(nodes, id, "rpt_max_depth") + num(nodes, id, "invert_elev")
        );
    }
    for id in ["C1", "C2", "C3"] {
        assert!(
            num(links, id, "rpt_max_flow") >= num(links, id, "max_flow") - 1e-6,
            "{id}"
        );
    }
}

#[test]
fn without_an_epsg_no_reference_system_is_claimed() {
    let m = make("nocrs", None, false);
    for name in ["nodes", "links", "subcatchments"] {
        let crs = layer(&m, name).crs.clone();
        assert_eq!(
            crs.as_ref().and_then(|c| c.epsg),
            None,
            "{name} claims {crs:?}"
        );
    }
    // and with results from a file alone the rpt_ columns are there, empty
    assert_eq!(
        value(layer(&m, "nodes"), "J1", "rpt_max_depth"),
        &FieldValue::Null
    );
    assert_eq!(
        value(layer(&m, "links"), "C1", "rpt_max_flow"),
        &FieldValue::Null
    );
}

#[test]
fn results_of_another_model_are_refused() {
    let dir = std::env::temp_dir().join(format!("swmm5-gpkg-test-{}-mismatch", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let inp = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../outfile/tests/data/tiny.inp");
    run_engine(&inp, &dir.join("tiny.rpt"), &dir.join("tiny.out")).unwrap();
    // the same model with one node renamed in the input file
    let other = dir.join("other.inp");
    std::fs::write(
        &other,
        std::fs::read_to_string(&inp)
            .unwrap()
            .replace("WET1", "WET2"),
    )
    .unwrap();
    let err = export(
        &other,
        &dir.join("tiny.out"),
        &dir.join("x.gpkg"),
        &Options::default(),
        None,
    )
    .unwrap_err();
    assert!(err.to_string().contains("not the same model"), "{err}");
    assert!(!dir.join("x.gpkg").exists());
}

/// `export` from an OpenSWMM 6 results file: the types and the saved properties must be the
/// input file's, though that engine numbers them differently in the header.
#[test]
fn an_openswmm6_results_file_exports_with_the_right_types() {
    let dir = std::env::temp_dir().join(format!("swmm5-gpkg-test-{}-swmm6", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../outfile/tests/data");
    let gpkg = dir.join("tiny6.gpkg");
    export(
        &data.join("tiny.inp"),
        &data.join("tiny_openswmm6.out"),
        &gpkg,
        &Options::default(),
        None,
    )
    .unwrap();
    let m = Made {
        dir,
        layers: terrio::open_vector(&gpkg.to_string_lossy(), &Default::default())
            .unwrap()
            .layers,
    };
    let (nodes, links, subs) = (
        layer(&m, "nodes"),
        layer(&m, "links"),
        layer(&m, "subcatchments"),
    );
    assert_eq!(
        value(nodes, "WET1", "type"),
        &FieldValue::Text("STORAGE".into())
    );
    assert_eq!(
        value(nodes, "O1", "type"),
        &FieldValue::Text("OUTFALL".into())
    );
    assert_eq!(value(links, "P1", "type"), &FieldValue::Text("PUMP".into()));
    assert_eq!(
        (
            num(nodes, "WET1", "invert_elev"),
            num(nodes, "WET1", "full_depth")
        ),
        (90.0, 12.0)
    );
    assert_eq!(
        (num(links, "C1", "length"), num(links, "C1", "full_depth")),
        (400.0, 1.5)
    );
    assert_eq!(num(subs, "S1", "area"), 1.0);
    assert_eq!(num(links, "P1", "max_flow"), 5.0);
}
