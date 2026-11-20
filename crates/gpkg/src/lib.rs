//! swmm5-gpkg: a SWMM 5 model and its results as a GeoPackage.
//!
//! Geometry comes from the input file's map sections, results from the binary
//! `.out` through the `swmm-out` reader, and the file is written by terrio
//! (pure Rust: no GDAL, no SQLite library). The GeoPackage holds
//!
//! * `nodes` (points), `links` (lines) and `subcatchments` (polygons), each
//!   object with its type, its saved properties, and the peak and time of peak
//!   of every reporting variable;
//! * `swmm_run`, a key/value table saying what was run and how to read the numbers;
//! * `swmm_fields`, the unit and meaning of every attribute column.
//!
//! Two sets of peaks are written, and they are different things:
//!
//! * `max_*` are over the reporting periods saved in the `.out` (one value per
//!   report step). Any engine's `.out` gives these.
//! * `rpt_*` are the engine's own running maxima over every routing step, the
//!   numbers its `.rpt` summary tables print. They exist only when swmm5-rust
//!   ran the model in this process ([`run_engine`]); they are read from the
//!   engine's memory after the last step, not parsed from the report text.
//!
//! On a model with a long report step the two differ a good deal.
#![forbid(unsafe_code)]

pub mod inpmap;

use inpmap::InpMap;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{Read, Seek};
use std::path::Path;
use swmm_out::{iso, Kind, OutFile, Stat};
use terrio::vector::Coord;
use terrio::{Feature, FieldDef, FieldType, FieldValue, Geometry, GeometryType, Layer};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Out(swmm_out::Error),
    Terrio(terrio::Error),
    /// The input file and the results file do not describe the same model.
    Mismatch(String),
    /// The engine stopped with an error; the text is its ERROR lines.
    Engine(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{e}"),
            Error::Out(e) => write!(f, "{e}"),
            Error::Terrio(e) => write!(f, "{e}"),
            Error::Mismatch(s) | Error::Engine(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<swmm_out::Error> for Error {
    fn from(e: swmm_out::Error) -> Self {
        Error::Out(e)
    }
}
impl From<terrio::Error> for Error {
    fn from(e: terrio::Error) -> Self {
        Error::Terrio(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Times are written as ISO 8601 text on the model's clock (`1997-01-01T14:05:00`), not as
/// GeoPackage DATETIME: the standard requires a DATETIME to be UTC, and a SWMM model has no
/// time zone, so a DATETIME would either be invalid or claim a zone the model never had.
/// As text they still sort in time order.
const TIME_UNIT: &str = "model clock, ISO 8601 text";

#[derive(Clone, Debug, Default)]
pub struct Options {
    /// EPSG code of the model's coordinates. A SWMM input file does not say what its
    /// coordinates are, so without this the layers are written with no reference system
    /// (terrio records GDAL's "Undefined SRS", srs_id 99999) and a GIS will ask.
    pub epsg: Option<u32>,
    /// Names for the `swmm_run` table.
    pub model_name: String,
    pub results_name: String,
}

/// A node's maxima over every routing step, in the model's units, as the `.rpt` summaries print them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeMaxima {
    pub max_depth: f64,
    pub max_hgl: f64,
    /// When the depth peaked (days since 30 December 1899); `None` when the engine never set it.
    pub max_depth_date: Option<f64>,
    pub max_lateral_inflow: f64,
    pub max_total_inflow: f64,
    pub max_flooding: f64,
    pub hours_flooded: f64,
}

/// A link's maxima over every routing step, as the Link Flow Summary prints them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinkMaxima {
    /// The largest flow in either direction ("Maximum |Flow|").
    pub max_flow: f64,
    /// `None` when the engine never set it (a link that never carried flow).
    pub max_flow_date: Option<f64>,
    /// Conduits only.
    pub max_velocity: Option<f64>,
    /// "Max/Full Depth"; absent where the report leaves it blank (pumps, outlets, dummy and bottom-orifice links).
    pub max_depth_ratio: Option<f64>,
}

/// The engine's running maxima for one run, by object name.
#[derive(Clone, Debug, Default)]
pub struct EngineMaxima {
    pub nodes: HashMap<String, NodeMaxima>,
    pub links: HashMap<String, LinkMaxima>,
    pub warnings: i32,
}

/// What went into the GeoPackage, and what could not.
#[derive(Clone, Debug, Default)]
pub struct Report {
    pub nodes: usize,
    pub nodes_with_results: usize,
    pub links: usize,
    pub links_with_results: usize,
    pub subcatchments: usize,
    pub subcatchments_with_results: usize,
    /// Nodes with no [COORDINATES] line: not drawn.
    pub nodes_without_coordinates: Vec<String>,
    /// Links with an end node that has no coordinates: not drawn.
    pub links_without_geometry: Vec<String>,
    /// Subcatchments with fewer than three [POLYGONS] points: not drawn.
    pub subcatchments_without_polygon: Vec<String>,
    pub notes: Vec<String>,
}

/// A layer under construction: the schema, the unit of each column, and the rows.
struct Table {
    layer: Layer,
    docs: Vec<(String, String, String)>,
}

impl Table {
    fn new(name: &str, geom: Option<GeometryType>, epsg: Option<u32>) -> Table {
        let mut layer = Layer::new(name);
        if let Some(g) = geom {
            layer = layer.with_geom_type(g);
            if let Some(e) = epsg {
                layer = layer.with_epsg(e);
            }
        }
        Table {
            layer,
            docs: Vec::new(),
        }
    }

    fn field(&mut self, name: &str, t: FieldType, unit: &str, what: &str) {
        self.layer.add_field(FieldDef::new(name, t));
        self.docs
            .push((name.to_string(), unit.to_string(), what.to_string()));
    }

    fn push(&mut self, geometry: Option<Geometry>, attributes: Vec<FieldValue>) {
        debug_assert_eq!(attributes.len(), self.docs.len());
        let fid = self.layer.features.len() as u64 + 1;
        self.layer.features.push(Feature {
            fid,
            geometry,
            attributes,
        });
    }
}

fn real(v: f64) -> FieldValue {
    if v.is_finite() {
        FieldValue::Float(v)
    } else {
        FieldValue::Null
    }
}

fn opt_real(v: Option<f64>) -> FieldValue {
    v.map_or(FieldValue::Null, real)
}

fn when(date: Option<f64>) -> FieldValue {
    date.map_or(FieldValue::Null, |d| FieldValue::Text(iso(d)))
}

fn text(s: &str) -> FieldValue {
    FieldValue::Text(s.to_string())
}

/// `quality:TSS` -> `quality_tss`: a column name any GIS and any SQL accepts.
fn column(name: &str) -> String {
    let mut s: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    while s.contains("__") {
        s = s.replace("__", "_");
    }
    s.trim_matches('_').to_string()
}

/// The result columns of one kind of object: for every reporting variable its peak and when.
struct ResultColumns {
    names: Vec<String>,
    stats: Vec<Vec<Stat>>,
    index: HashMap<String, usize>,
}

impl ResultColumns {
    fn new<R: Read + Seek>(
        out: &mut OutFile<R>,
        kind: Kind,
        table: &mut Table,
    ) -> Result<ResultColumns> {
        let vars = out.variables(kind);
        let mut names = Vec::new();
        let mut seen = HashSet::new();
        for v in &vars {
            let mut c = column(&v.name);
            while !seen.insert(c.clone()) {
                c.push('_');
            }
            table.field(
                &format!("max_{c}"),
                FieldType::Float,
                &v.unit,
                &format!("highest saved {} ({})", v.name, kind.as_str()),
            );
            table.field(
                &format!("max_{c}_time"),
                FieldType::Text,
                TIME_UNIT,
                &format!("first time of the highest saved {}", v.name),
            );
            names.push(c);
        }
        let stats = out.stats(kind)?;
        let index = out
            .ids(kind)
            .iter()
            .enumerate()
            .map(|(i, id)| (id.clone(), i))
            .collect();
        Ok(ResultColumns {
            names,
            stats,
            index,
        })
    }

    fn row(&self, id: &str) -> Option<&Vec<Stat>> {
        self.stats.get(*self.index.get(id)?)
    }

    fn values(&self, id: &str, dates: &[f64], out: &mut Vec<FieldValue>) {
        match self.row(id) {
            Some(row) => {
                for s in row {
                    if s.max.is_nan() {
                        out.push(FieldValue::Null);
                        out.push(FieldValue::Null);
                    } else {
                        out.push(real(s.max as f64));
                        out.push(FieldValue::Text(iso(dates[s.max_period])));
                    }
                }
            }
            None => out.extend(std::iter::repeat_n(FieldValue::Null, 2 * self.names.len())),
        }
    }

    fn stat(&self, id: &str, name: &str) -> Option<&Stat> {
        let v = self.names.iter().position(|n| n == name)?;
        self.row(id)?.get(v)
    }
}

/// Builds the layers of the GeoPackage from a parsed input file and an open results file.
///
/// `maxima` are the engine's all-routing-step maxima from [`run_engine`]; without them the
/// `rpt_*` columns are still there, and empty.
pub fn build_layers<R: Read + Seek>(
    map: &InpMap,
    out: &mut OutFile<R>,
    opt: &Options,
    maxima: Option<&EngineMaxima>,
) -> Result<(Vec<Layer>, Report)> {
    let mut report = Report::default();

    // The two files must be the same model: every object with results has to be in the input file.
    let known = |ids: &[String], have: &HashSet<&str>| -> Vec<String> {
        ids.iter()
            .filter(|i| !have.contains(i.as_str()))
            .cloned()
            .collect()
    };
    let node_ids: HashSet<&str> = map.nodes.iter().map(|n| n.id.as_str()).collect();
    let link_ids: HashSet<&str> = map.links.iter().map(|l| l.id.as_str()).collect();
    let sub_ids: HashSet<&str> = map.subcatchments.iter().map(|s| s.id.as_str()).collect();
    let mut strangers = known(&out.nodes, &node_ids);
    strangers.extend(known(&out.links, &link_ids));
    strangers.extend(known(&out.subcatchments, &sub_ids));
    if !strangers.is_empty() {
        return Err(Error::Mismatch(format!(
            "the results file names {} object(s) the input file does not have ({}{}): these are not the same model",
            strangers.len(),
            strangers.iter().take(5).cloned().collect::<Vec<_>>().join(", "),
            if strangers.len() > 5 { ", ..." } else { "" }
        )));
    }

    let dates = out.dates()?;
    let us = out.is_us();
    let (len_unit, area_unit) = if us { ("ft", "ac") } else { ("m", "ha") };
    let step_hours = out.report_step as f64 / 3600.0;

    // --- nodes ---------------------------------------------------------------
    let mut nodes = Table::new("nodes", Some(GeometryType::Point), opt.epsg);
    nodes.field("id", FieldType::Text, "", "node name");
    nodes.field(
        "type",
        FieldType::Text,
        "",
        "JUNCTION, OUTFALL, STORAGE or DIVIDER",
    );
    nodes.field(
        "has_results",
        FieldType::Boolean,
        "",
        "whether the results file reports this node",
    );
    nodes.field(
        "invert_elev",
        FieldType::Float,
        len_unit,
        "invert elevation, as saved in the results file",
    );
    nodes.field(
        "full_depth",
        FieldType::Float,
        len_unit,
        "maximum depth, as saved in the results file",
    );
    let node_res = ResultColumns::new(out, Kind::Node, &mut nodes)?;
    nodes.field(
        "hours_flooded",
        FieldType::Float,
        "h",
        "saved periods with flooding above zero, times the report step",
    );
    let flow_unit = out.flow_units();
    let every = "over every routing step, as the .rpt prints it (swmm5-rust runs only)";
    nodes.field(
        "rpt_max_depth",
        FieldType::Float,
        len_unit,
        &format!("maximum depth {every}"),
    );
    nodes.field(
        "rpt_max_hgl",
        FieldType::Float,
        len_unit,
        &format!("maximum hydraulic grade line {every}"),
    );
    nodes.field(
        "rpt_max_depth_time",
        FieldType::Text,
        TIME_UNIT,
        &format!("time of the maximum depth {every}"),
    );
    nodes.field(
        "rpt_max_lateral_inflow",
        FieldType::Float,
        flow_unit,
        &format!("maximum lateral inflow {every}"),
    );
    nodes.field(
        "rpt_max_total_inflow",
        FieldType::Float,
        flow_unit,
        &format!("maximum total inflow {every}"),
    );
    nodes.field(
        "rpt_max_flooding",
        FieldType::Float,
        flow_unit,
        &format!("maximum flooding rate {every}"),
    );
    nodes.field(
        "rpt_hours_flooded",
        FieldType::Float,
        "h",
        &format!("hours flooded {every}"),
    );
    for n in &map.nodes {
        let Some(&(x, y)) = map.coordinates.get(&n.id) else {
            report.nodes_without_coordinates.push(n.id.clone());
            continue;
        };
        let i = out.index_of(Kind::Node, &n.id);
        let mut a = vec![
            text(&n.id),
            text(i.and_then(|i| out.node_type(i)).unwrap_or(n.kind)),
            FieldValue::Boolean(i.is_some()),
            opt_real(i.and_then(|i| out.node_invert(i))),
            opt_real(i.and_then(|i| out.node_full_depth(i))),
        ];
        node_res.values(&n.id, &dates, &mut a);
        a.push(opt_real(
            node_res
                .stat(&n.id, "flooding")
                .map(|s| s.positive_periods as f64 * step_hours),
        ));
        match maxima.and_then(|m| m.nodes.get(&n.id)) {
            Some(m) => a.extend([
                real(m.max_depth),
                real(m.max_hgl),
                when(m.max_depth_date),
                real(m.max_lateral_inflow),
                real(m.max_total_inflow),
                real(m.max_flooding),
                real(m.hours_flooded),
            ]),
            None => a.extend(std::iter::repeat_n(FieldValue::Null, 7)),
        }
        nodes.push(Some(Geometry::point(x, y)), a);
        report.nodes += 1;
        report.nodes_with_results += i.is_some() as usize;
    }

    // --- links ---------------------------------------------------------------
    let mut links = Table::new("links", Some(GeometryType::LineString), opt.epsg);
    links.field("id", FieldType::Text, "", "link name");
    links.field(
        "type",
        FieldType::Text,
        "",
        "CONDUIT, PUMP, ORIFICE, WEIR or OUTLET",
    );
    links.field(
        "from_node",
        FieldType::Text,
        "",
        "upstream node in the input file",
    );
    links.field(
        "to_node",
        FieldType::Text,
        "",
        "downstream node in the input file",
    );
    links.field(
        "has_results",
        FieldType::Boolean,
        "",
        "whether the results file reports this link",
    );
    links.field(
        "length",
        FieldType::Float,
        len_unit,
        "conduit length, as saved in the results file (0 for other links)",
    );
    links.field(
        "full_depth",
        FieldType::Float,
        len_unit,
        "maximum depth, as saved in the results file",
    );
    let link_res = ResultColumns::new(out, Kind::Link, &mut links)?;
    links.field(
        "min_flow",
        FieldType::Float,
        out.flow_units(),
        "lowest saved flow; below zero means the link ran backwards",
    );
    links.field(
        "min_flow_time",
        FieldType::Text,
        TIME_UNIT,
        "first time of the lowest saved flow",
    );
    links.field(
        "rpt_max_flow",
        FieldType::Float,
        flow_unit,
        &format!("maximum |flow| {every}"),
    );
    links.field(
        "rpt_max_flow_time",
        FieldType::Text,
        TIME_UNIT,
        &format!("time of the maximum |flow| {every}"),
    );
    links.field(
        "rpt_max_velocity",
        FieldType::Float,
        if us { "ft/s" } else { "m/s" },
        &format!("maximum velocity, conduits only, {every}"),
    );
    links.field(
        "rpt_max_depth_ratio",
        FieldType::Float,
        "fraction",
        &format!("maximum depth over full depth {every}"),
    );
    for l in &map.links {
        let (Some(&a0), Some(&a1)) = (map.coordinates.get(&l.from), map.coordinates.get(&l.to))
        else {
            report.links_without_geometry.push(l.id.clone());
            continue;
        };
        let mut line = vec![Coord::xy(a0.0, a0.1)];
        if let Some(v) = map.vertices.get(&l.id) {
            line.extend(v.iter().map(|&(x, y)| Coord::xy(x, y)));
        }
        line.push(Coord::xy(a1.0, a1.1));
        let i = out.index_of(Kind::Link, &l.id);
        let mut a = vec![
            text(&l.id),
            text(i.and_then(|i| out.link_type(i)).unwrap_or(l.kind)),
            text(&l.from),
            text(&l.to),
            FieldValue::Boolean(i.is_some()),
            opt_real(i.and_then(|i| out.link_length(i))),
            opt_real(i.and_then(|i| out.link_full_depth(i))),
        ];
        link_res.values(&l.id, &dates, &mut a);
        match link_res.stat(&l.id, "flow").filter(|s| !s.min.is_nan()) {
            Some(s) => {
                a.push(real(s.min as f64));
                a.push(FieldValue::Text(iso(dates[s.min_period])));
            }
            None => a.extend([FieldValue::Null, FieldValue::Null]),
        }
        match maxima.and_then(|m| m.links.get(&l.id)) {
            Some(m) => a.extend([
                real(m.max_flow),
                when(m.max_flow_date),
                opt_real(m.max_velocity),
                opt_real(m.max_depth_ratio),
            ]),
            None => a.extend(std::iter::repeat_n(FieldValue::Null, 4)),
        }
        links.push(Some(Geometry::line_string(line)), a);
        report.links += 1;
        report.links_with_results += i.is_some() as usize;
    }

    // --- subcatchments -------------------------------------------------------
    let mut subs = Table::new("subcatchments", Some(GeometryType::Polygon), opt.epsg);
    subs.field("id", FieldType::Text, "", "subcatchment name");
    subs.field(
        "rain_gage",
        FieldType::Text,
        "",
        "rain gage in the input file",
    );
    subs.field(
        "outlet",
        FieldType::Text,
        "",
        "node or subcatchment that receives the runoff",
    );
    subs.field(
        "has_results",
        FieldType::Boolean,
        "",
        "whether the results file reports this subcatchment",
    );
    subs.field(
        "area",
        FieldType::Float,
        area_unit,
        "area, as saved in the results file",
    );
    let sub_res = ResultColumns::new(out, Kind::Subcatchment, &mut subs)?;
    for s in &map.subcatchments {
        let mut ring: Vec<(f64, f64)> = map.polygons.get(&s.id).cloned().unwrap_or_default();
        if ring.len() >= 2 && ring.first() == ring.last() {
            ring.pop();
        }
        if ring.len() < 3 {
            report.subcatchments_without_polygon.push(s.id.clone());
            continue;
        }
        ring.push(ring[0]);
        let i = out.index_of(Kind::Subcatchment, &s.id);
        let mut a = vec![
            text(&s.id),
            text(&s.rain_gage),
            text(&s.outlet),
            FieldValue::Boolean(i.is_some()),
            opt_real(i.and_then(|i| out.subcatchment_area(i))),
        ];
        sub_res.values(&s.id, &dates, &mut a);
        let exterior = ring.iter().map(|&(x, y)| Coord::xy(x, y)).collect();
        subs.push(Some(Geometry::polygon(exterior, Vec::new())), a);
        report.subcatchments += 1;
        report.subcatchments_with_results += i.is_some() as usize;
    }

    if map.skipped_lines > 0 {
        report.notes.push(format!(
            "{} line(s) of the input file's map sections could not be read",
            map.skipped_lines
        ));
    }
    report.notes.extend(out.notes.iter().cloned());

    // --- what was run ----------------------------------------------------------
    let mut run = Table::new("swmm_run", None, None);
    run.field("key", FieldType::Text, "", "");
    run.field("value", FieldType::Text, "", "");
    let mut kv = |k: &str, v: String| run.push(None, vec![text(k), FieldValue::Text(v)]);
    kv("model", opt.model_name.clone());
    kv("results", opt.results_name.clone());
    kv("engine_version", out.version.to_string());
    kv(
        "error_code",
        out.error_code
            .map_or("none: the results file has no closing record".into(), |e| {
                e.to_string()
            }),
    );
    kv("flow_units", out.flow_units().to_string());
    kv("unit_system", if us { "US" } else { "SI" }.to_string());
    kv("report_step_seconds", out.report_step.to_string());
    kv("periods", out.n_periods.to_string());
    if let (Some(a), Some(b)) = (dates.first(), dates.last()) {
        kv("first_report", iso(*a));
        kv("last_report", iso(*b));
    }
    kv(
        "coordinate_reference",
        match opt.epsg {
            Some(e) => {
                format!("EPSG:{e}, as given to swmm5-gpkg; the input file does not state one")
            }
            None => "undefined: the input file does not state one and none was given".to_string(),
        },
    );
    kv(
        "map_units",
        map.map_units.clone().unwrap_or_else(|| "not stated".into()),
    );
    kv("times", "ISO 8601 text on the model clock, as the engine stamps them; no time zone, so not stored as GeoPackage DATETIME (which must be UTC)".to_string());
    kv(
        "peaks",
        "max_* and min_* are over the reporting periods saved in the results file, one value per report step; \
         rpt_* are the engine's maxima over every routing step, the numbers the .rpt summary tables print, and can be higher"
            .to_string(),
    );
    kv(
        "rpt_columns",
        match maxima {
            Some(_) => "filled: swmm5-rust ran this model here and its running maxima were read from memory after the last step".to_string(),
            None => "empty: the results came from a file, and a .out does not hold the every-step maxima".to_string(),
        },
    );
    kv(
        "nodes_drawn",
        format!(
            "{} of {} in the input file; {} with results",
            report.nodes,
            map.nodes.len(),
            report.nodes_with_results
        ),
    );
    kv(
        "links_drawn",
        format!(
            "{} of {} in the input file; {} with results",
            report.links,
            map.links.len(),
            report.links_with_results
        ),
    );
    kv(
        "subcatchments_drawn",
        format!(
            "{} of {} in the input file; {} with results",
            report.subcatchments,
            map.subcatchments.len(),
            report.subcatchments_with_results
        ),
    );
    for n in &report.notes {
        kv("note", n.clone());
    }
    kv(
        "written_by",
        format!(
            "swmm5-gpkg {} (swmm-out reader, terrio GeoPackage writer)",
            env!("CARGO_PKG_VERSION")
        ),
    );

    // --- what each column means ------------------------------------------------
    let mut fields = Table::new("swmm_fields", None, None);
    for name in ["layer", "field", "unit", "meaning"] {
        fields.field(name, FieldType::Text, "", "");
    }
    for t in [&nodes, &links, &subs] {
        for (name, unit, what) in &t.docs {
            fields.push(
                None,
                vec![text(&t.layer.name), text(name), text(unit), text(what)],
            );
        }
    }

    Ok((
        vec![
            nodes.layer,
            links.layer,
            subs.layer,
            run.layer,
            fields.layer,
        ],
        report,
    ))
}

/// Writes `gpkg` from an input file and the results file of any engine that writes SWMM 5's layout.
/// Pass the maxima [`run_engine`] returned to fill the `rpt_*` columns.
pub fn export(
    inp: &Path,
    out: &Path,
    gpkg: &Path,
    opt: &Options,
    maxima: Option<&EngineMaxima>,
) -> Result<Report> {
    let map = InpMap::parse(&std::fs::read(inp)?);
    let mut results = OutFile::open(out)?;
    let mut opt = opt.clone();
    let name = |p: &Path| {
        p.file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default()
    };
    if opt.model_name.is_empty() {
        opt.model_name = name(inp);
    }
    if opt.results_name.is_empty() {
        opt.results_name = name(out);
    }
    let (layers, report) = build_layers(&map, &mut results, &opt, maxima)?;
    terrio::write_vector(&layers, &gpkg.to_string_lossy(), &Default::default())?;
    Ok(report)
}

/// Runs a model with the swmm5-rust engine, writing `rpt` and `out`, and returns the engine's
/// running maxima for every node and link.
///
/// The calls are the ones `swmm_run` makes, in its order (open, start, step until the clock
/// returns to zero, end, close), so the two files are what `runswmm-rs` writes. The maxima are
/// copied between the last step and `swmm_end`, which is where the engine prints them and then
/// frees them. Each is converted as `statsrpt.c` converts it for the summary tables.
pub fn run_engine(inp: &Path, rpt: &Path, out: &Path) -> Result<EngineMaxima> {
    use swmm5_rust::enums::{
        BOTTOM_ORIFICE, CONDUIT, DUMMY, FLOW, LENGTH, LINK, NODE, ORIFICE, OUTLET, PUMP,
    };
    use swmm5_rust::swmm5::{
        swmm_close, swmm_end, swmm_getError, swmm_open, swmm_start, swmm_step, UCF,
    };
    // The engine takes paths as bytes and resolves relative ones itself; hand it absolute ones.
    let abs = |p: &Path| -> std::io::Result<Vec<u8>> {
        Ok(std::path::absolute(p)?
            .to_string_lossy()
            .into_owned()
            .into_bytes())
    };
    let id =
        |b: &[u8]| swmm_out::decode_name(&b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]);
    // The engine starts every "date of the maximum" at 0.0 and the report prints such a date as
    // day 0, 00:00. Zero is not a time the model reached, so it is kept as "not set".
    let set = |date: f64| (date > 0.0).then_some(date);

    let mut prj = swmm5_rust::Project::new();
    let mut maxima = EngineMaxima::default();
    swmm_open(&mut prj, &abs(inp)?, &abs(rpt)?, &abs(out)?);
    if prj.ErrorCode == 0 {
        swmm_start(&mut prj, 1);
        if prj.ErrorCode == 0 {
            let mut elapsed = 0.0f64;
            loop {
                swmm_step(&mut prj, &mut elapsed);
                if !(elapsed > 0.0 && prj.ErrorCode == 0) {
                    break;
                }
            }
        }
        if prj.ErrorCode == 0 {
            let (len, flow) = (UCF(&prj, LENGTH), UCF(&prj, FLOW));
            for j in 0..prj.Nobjects[NODE as usize] as usize {
                let (Some(ns), Some(node)) = (prj.stats.NodeStats.get(j), prj.Node.get(j)) else {
                    break;
                };
                maxima.nodes.insert(
                    id(&node.ID),
                    NodeMaxima {
                        max_depth: ns.maxDepth * len,
                        max_hgl: (ns.maxDepth + node.invertElev) * len,
                        max_depth_date: set(ns.maxDepthDate),
                        max_lateral_inflow: ns.maxLatFlow * flow,
                        max_total_inflow: ns.maxInflow * flow,
                        max_flooding: ns.maxOverflow * flow,
                        hours_flooded: ns.timeFlooded / 3600.0,
                    },
                );
            }
            for j in 0..prj.Nobjects[LINK as usize] as usize {
                let (Some(ls), Some(link)) = (prj.stats.LinkStats.get(j), prj.Link.get(j)) else {
                    break;
                };
                let k = link.subIndex as usize;
                // The Link Flow Summary stops after the flow for pumps, dummy conduits and outlets.
                let short_row =
                    link.type_ == PUMP || link.xsect.type_ == DUMMY || link.type_ == OUTLET;
                let mut full_depth = link.xsect.yFull;
                if link.type_ == ORIFICE
                    && prj
                        .Orifice
                        .get(k)
                        .is_some_and(|o| o.type_ == BOTTOM_ORIFICE)
                {
                    full_depth = 0.0;
                }
                maxima.links.insert(
                    id(&link.ID),
                    LinkMaxima {
                        max_flow: ls.maxFlow * flow,
                        max_flow_date: set(ls.maxFlowDate),
                        max_velocity: (!short_row && link.type_ == CONDUIT)
                            .then_some(ls.maxVeloc * len),
                        max_depth_ratio: (!short_row && full_depth > 0.0)
                            .then(|| ls.maxDepth / full_depth),
                    },
                );
            }
        }
        swmm_end(&mut prj);
    }
    let mut msg = Vec::new();
    let code = swmm_getError(&mut prj, &mut msg, 240);
    maxima.warnings = prj.Warnings;
    swmm_close(&mut prj);
    if code > 0 {
        return Err(Error::Engine(format!(
            "the engine stopped: {}",
            String::from_utf8_lossy(&msg).trim()
        )));
    }
    Ok(maxima)
}
