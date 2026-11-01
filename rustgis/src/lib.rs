//! swmm5-rust in RustGIS.
//!
//! A RustGIS WebAssembly plugin (ABI 1) that runs EPA SWMM 5.2.4, as
//! translated into safe Rust by swmm5-rust, inside RustGIS's plugin sandbox,
//! and hands the model back as map layers whose attributes are the engine's
//! own summary results. The same module runs in the RustGIS desktop app, on
//! its command line and in its web app, and in the companion HTML page, which
//! plays RustGIS's part of the ABI in JavaScript.
//!
//! Tools (`plugin.json`):
//! * `run-swmm`: run a .inp, return subcatchment, link, node and rain-gage
//!   layers with results, and the run's messages.
//! * `map-swmm`: the same layers with input attributes only, no run.
//!
//! Command: `about`, the engine and build this module carries.

pub mod epsg;
pub mod inp;
pub mod rpt;

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use swmm5_rust::{Project, memfs, swmm5};

use rustgis_plugin::{Kind, Request};

/// This plugin's version (plugin.json).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Where a model with no real coordinates is drawn: the University of Florida
/// in Gainesville, one of the three places SWMM was first written (1969-71).
pub const ANCHOR: [f64; 2] = [-82.3474, 29.6476];
const EARTH_R: f64 = 6_371_008.8;

/// One run's inputs, as the tool's form gives them.
pub struct Job {
    pub name: String,
    pub bytes: Vec<u8>,
    /// Files the model names (rainfall, inflows, hotstart...), by their own names.
    pub data: Vec<(String, Vec<u8>)>,
    pub epsg: Option<u32>,
    pub anchor: [f64; 2],
    pub report: bool,
    pub run: bool,
}

/// What a run produced, beside the layers. The output file stays in memfs (as
/// `model.out`) until the next run; only its size and hash are taken here.
pub struct Ran {
    pub code: i32,
    pub rpt: Vec<u8>,
    pub out_len: usize,
    pub out_sha: String,
}

/// The names the engine's own three files have in the sandbox.
pub const ENGINE_FILES: [&str; 3] = ["model.inp", "model.rpt", "model.out"];

/// Data files must not share a name with each other or with the engine's own files
/// (names compared as the engine finds them: by the file's own name, without regard to case).
pub fn check_data(data: &[(String, Vec<u8>)]) -> Result<(), String> {
    let base = |n: &str| n.rsplit(['/', '\\']).next().unwrap_or(n).to_ascii_lowercase();
    let mut seen: Vec<String> = Vec::new();
    for (name, _) in data {
        let b = base(name);
        if ENGINE_FILES.contains(&b.as_str()) {
            return Err(format!("a data file cannot be named {name}: the engine's own input, report and output files use that name in the sandbox"));
        }
        if seen.contains(&b) {
            return Err(format!("two data files are both named {name}; the model could read only one of them"));
        }
        seen.push(b);
    }
    Ok(())
}

/// The engine version as EPA's swmm_getVersion() gives it (52004 for 5.2.4).
pub fn swmm_version() -> i32 {
    let mut p = Project::new();
    swmm5::swmm_getVersion(&mut p)
}

/// Runs the model in memory, exactly as runswmm does on a disk: swmm_run()
/// with an input, a report and an output file.
pub fn run_engine(bytes: &[u8], data: &[(String, Vec<u8>)]) -> Ran {
    memfs::clear();
    for (name, b) in data {
        memfs::put(name.as_bytes(), b.clone());
    }
    memfs::put(b"model.inp", bytes.to_vec());
    let mut p = Project::new();
    let code = swmm5::swmm_run(&mut p, b"model.inp", b"model.rpt", b"model.out");
    drop(p);
    let (out_len, out_sha) = memfs::with(b"model.out", |b| (b.len(), sha256_hex(b))).unwrap_or((0, sha256_hex(b"")));
    Ran { code, rpt: memfs::get(b"model.rpt").unwrap_or_default(), out_len, out_sha }
}

pub fn sha256_hex(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

/// Where the model's map coordinates go.
enum Place {
    /// Native coordinates, declared to RustGIS by EPSG code (RustGIS reprojects).
    Epsg(u32),
    /// Already longitude/latitude ([MAP] UNITS DEGREES).
    LonLat,
    /// A local frame drawn at true size around an anchor.
    Local { lon0: f64, lat0: f64, cx: f64, cy: f64, m_per_unit: f64, assumed: bool },
}

impl Place {
    fn pt(&self, p: [f64; 2]) -> [f64; 2] {
        match self {
            Place::Epsg(_) | Place::LonLat => p,
            Place::Local { lon0, lat0, cx, cy, m_per_unit, .. } => {
                let dx = (p[0] - cx) * m_per_unit;
                let dy = (p[1] - cy) * m_per_unit;
                let lat = lat0 + (dy / EARTH_R).to_degrees();
                let lon = lon0 + (dx / (EARTH_R * lat0.to_radians().cos())).to_degrees();
                [lon, lat]
            }
        }
    }
    fn describe(&self) -> String {
        match self {
            Place::Epsg(c) => format!(
                "coordinates declared as EPSG:{c}, which RustGIS reprojects; check that the layers land where you expect (coordinates that are not in that system are drawn in the wrong place, or left out where they cannot be converted, and RustGIS does not say so)"
            ),
            Place::LonLat => "coordinates read as longitude/latitude ([MAP] UNITS Degrees)".into(),
            Place::Local { lon0, lat0, m_per_unit, assumed, .. } => format!(
                "no coordinate system given: the model is drawn at true size ({}{}) around {lat0:.4}, {lon0:.4}",
                if (*m_per_unit - 0.3048).abs() < 1e-12 { "1 ft per map unit" } else { "1 m per map unit" },
                if *assumed { ", assumed from the flow units since [MAP] UNITS is None" } else { "" }
            ),
        }
    }
    fn crs(&self) -> Option<Value> {
        match self {
            Place::Epsg(c) => Some(json!({"type": "name", "properties": {"name": format!("urn:ogc:def:crs:EPSG::{c}")}})),
            _ => None,
        }
    }
}

fn placement(m: &inp::Model, epsg: Option<u32>, anchor: [f64; 2]) -> Place {
    if let Some(c) = epsg {
        return Place::Epsg(c);
    }
    if m.map_units == "DEGREES" {
        return Place::LonLat;
    }
    // Center of everything drawn.
    let mut b = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
    let mut grow = |p: &[f64; 2]| {
        b[0] = b[0].min(p[0]);
        b[1] = b[1].min(p[1]);
        b[2] = b[2].max(p[0]);
        b[3] = b[3].max(p[1]);
    };
    m.coords.values().for_each(&mut grow);
    m.vertices.values().flatten().for_each(&mut grow);
    m.polygons.values().flatten().for_each(&mut grow);
    m.symbols.values().for_each(&mut grow);
    let (cx, cy) = if b[0].is_finite() { ((b[0] + b[2]) / 2.0, (b[1] + b[3]) / 2.0) } else { (0.0, 0.0) };
    let (m_per_unit, assumed) = match m.map_units.as_str() {
        "FEET" => (0.3048, false),
        "METERS" => (1.0, false),
        _ => (if m.us() { 0.3048 } else { 1.0 }, true),
    };
    Place::Local { lon0: anchor[0], lat0: anchor[1], cx, cy, m_per_unit, assumed }
}

fn feature(geometry: Value, props: Map<String, Value>) -> Value {
    json!({"type": "Feature", "geometry": geometry, "properties": Value::Object(props)})
}

fn collection(features: Vec<Value>, crs: &Option<Value>) -> Value {
    let mut fc = json!({"type": "FeatureCollection", "features": features});
    if let Some(c) = crs {
        fc["crs"] = c.clone();
    }
    fc
}

fn opt(v: Option<f64>) -> Value {
    v.map(|x| json!(x)).unwrap_or(Value::Null)
}

fn merge(props: &mut Map<String, Value>, extra: Option<&rpt::Attrs>) {
    if let Some(e) = extra {
        for (k, v) in e {
            props.insert(k.clone(), v.clone());
        }
    }
}

/// The run's layers, tables and messages, as a tool's reply.
pub fn execute(job: &Job) -> Result<Value, String> {
    let m = inp::parse(&job.bytes);
    let stem = job.name.rsplit_once('.').map(|(a, _)| a).unwrap_or(&job.name).to_string();
    let stem = if stem.trim().is_empty() { "model".to_string() } else { stem };
    let mut units = rpt::Units::new(&m.flow_units);
    let mut messages: Vec<String> = Vec::new();
    let mut tables: Vec<Value> = Vec::new();

    // --- run the engine
    let mut res = rpt::Results::default();
    if job.run {
        check_data(&job.data)?;
        let r = run_engine(&job.bytes, &job.data);
        let text = inp::latin1(&r.rpt);
        if r.code > 0 {
            let errs: Vec<&str> = text.lines().filter(|l| l.trim_start().starts_with("ERROR")).take(20).collect();
            let mut msg = format!("swmm5-rust stopped with error {} in {}", r.code, job.name);
            for e in errs {
                msg.push('\n');
                msg.push_str(e.trim());
            }
            return Err(msg);
        }
        // the units and the routing the engine used, as its report states them
        if let Some(fu) = rpt::option(&text, "Flow Units") {
            units = rpt::Units::new(fu);
        }
        let dynwave = match rpt::option(&text, "Flow Routing Method") {
            Some(rm) => rm == "DYNWAVE",
            None => m.dynwave(),
        };
        res = rpt::parse(&text, &units, dynwave, m.pollutants);
        messages.push(format!(
            "swmm5-rust {VERSION}: EPA SWMM 5.2.4 in safe Rust (engine version {}), run inside RustGIS's WebAssembly sandbox",
            swmm_version()
        ));
        let ce: Vec<String> = [("runoff", "Runoff Quantity Continuity"), ("flow routing", "Flow Routing Continuity"), ("quality routing", "Quality Routing Continuity")]
            .iter()
            .filter_map(|(k, t)| rpt::continuity(&text, t).map(|v| format!("{k} {v}")))
            .collect();
        if !ce.is_empty() {
            messages.push(format!("continuity error (%) as reported: {}", ce.join(", ")));
        }
        let warnings = text.lines().filter(|l| l.trim_start().starts_with("WARNING")).count();
        if warnings > 0 {
            messages.push(format!("{warnings} WARNING lines in the report"));
        }
        messages.push(format!("output file: {} bytes, SHA-256 {}", r.out_len, r.out_sha));
        messages.push(format!(
            "report: {} bytes, SHA-256 {} (wall-clock lines removed)",
            r.rpt.len(),
            sha256_hex(&rpt::without_clock(&r.rpt))
        ));
        if !job.data.is_empty() {
            let names: Vec<&str> = job.data.iter().map(|(n, _)| n.as_str()).collect();
            messages.push(format!("data files given to the model: {}", names.join(", ")));
        }
        for (asked, used) in memfs::resolved() {
            messages.push(format!("the model asked for {}; it read the data file {}", inp::latin1(&asked), inp::latin1(&used)));
        }
        let ours: Vec<String> = ENGINE_FILES.iter().map(|s| s.to_string()).chain(job.data.iter().map(|(n, _)| n.clone())).collect();
        let wrote: Vec<String> = memfs::list().into_iter().map(|(n, _)| inp::latin1(&n)).filter(|n| !ours.contains(n)).collect();
        if !wrote.is_empty() {
            messages.push(format!("files the model wrote, kept only for the run: {}", wrote.join(", ")));
        }
        messages.push(format!("summary tables read: {}", res.tables.join(", ")));
        if !res.skipped.is_empty() {
            let mut per: Vec<(String, usize)> = Vec::new();
            for (t, _) in &res.skipped {
                match per.iter_mut().find(|(n, _)| n == t) {
                    Some(e) => e.1 += 1,
                    None => per.push((t.clone(), 1)),
                }
            }
            let per: Vec<String> = per.iter().map(|(t, n)| format!("{t} {n}")).collect();
            messages.push(format!("{} report rows not read, their attributes left out: {}", res.skipped.len(), per.join(", ")));
        }
        for (t, row) in res.skipped.iter().take(10) {
            messages.push(format!("row not read in {t}: {row}"));
        }
        if job.report {
            let feats: Vec<Value> = text
                .lines()
                .enumerate()
                .map(|(i, l)| json!({"type": "Feature", "geometry": null, "properties": {"line": i + 1, "text": l}}))
                .collect();
            tables.push(json!({"name": format!("{stem} report"), "geojson": collection(feats, &None)}));
        }
    }
    let (len, area) = if units.us { ("ft", "ac") } else { ("m", "ha") };

    // --- placement
    let place = placement(&m, job.epsg, job.anchor);
    let crs = place.crs();
    messages.push(place.describe());

    // --- subcatchments
    let mut subs = Vec::new();
    let mut unplaced_subs = Vec::new();
    for s in &m.subcatchments {
        let mut p = Map::new();
        p.insert("id".into(), json!(s.id));
        p.insert("rain_gage".into(), json!(s.gage));
        p.insert("outlet".into(), json!(s.outlet));
        p.insert(format!("area_{area}"), opt(s.area));
        p.insert("pct_imperv".into(), opt(s.pct_imperv));
        p.insert(format!("width_{len}"), opt(s.width));
        p.insert("slope_pct".into(), opt(s.slope));
        merge(&mut p, res.subcatch.get(&inp::key(&s.id)));
        match m.polygons.get(&inp::key(&s.id)) {
            Some(ring) if ring.len() >= 3 => {
                let mut r: Vec<[f64; 2]> = ring.iter().map(|q| place.pt(*q)).collect();
                if r.first() != r.last() {
                    r.push(r[0]);
                }
                subs.push(feature(json!({"type": "Polygon", "coordinates": [r]}), p));
            }
            _ => unplaced_subs.push(json!({"type": "Feature", "geometry": null, "properties": Value::Object(p)})),
        }
    }

    // --- links
    let mut links = Vec::new();
    let mut unplaced_links = Vec::new();
    for k in &m.links {
        let (a, b) = match (m.coords.get(&inp::key(&k.from)), m.coords.get(&inp::key(&k.to))) {
            (Some(a), Some(b)) => (*a, *b),
            _ => {
                unplaced_links.push(k);
                continue;
            }
        };
        let mut line = vec![place.pt(a)];
        if let Some(v) = m.vertices.get(&inp::key(&k.id)) {
            line.extend(v.iter().map(|q| place.pt(*q)));
        }
        line.push(place.pt(b));
        links.push(feature(json!({"type": "LineString", "coordinates": line}), link_props(k, &m, &res, len)));
    }
    let unplaced_links: Vec<Value> = unplaced_links
        .iter()
        .map(|k| json!({"type": "Feature", "geometry": null, "properties": Value::Object(link_props(k, &m, &res, len))}))
        .collect();


    // --- nodes
    let mut nodes = Vec::new();
    let mut unplaced_nodes = Vec::new();
    for n in &m.nodes {
        let mut p = Map::new();
        p.insert("id".into(), json!(n.id));
        p.insert("type".into(), json!(n.kind));
        p.insert(format!("invert_{len}"), opt(n.invert));
        if n.full_depth.is_some() {
            p.insert(format!("full_depth_{len}"), opt(n.full_depth));
        }
        merge(&mut p, res.node.get(&inp::key(&n.id)));
        match m.coords.get(&inp::key(&n.id)) {
            Some(c) => nodes.push(feature(json!({"type": "Point", "coordinates": place.pt(*c)}), p)),
            None => unplaced_nodes.push(json!({"type": "Feature", "geometry": null, "properties": Value::Object(p)})),
        }
    }

    // --- rain gages
    let mut gages = Vec::new();
    let mut unplaced_gages = Vec::new();
    for g in &m.gages {
        let mut p = Map::new();
        p.insert("id".into(), json!(g.id));
        p.insert("format".into(), json!(g.format));
        p.insert("interval".into(), json!(g.interval));
        p.insert("source".into(), json!(g.source));
        match m.symbols.get(&inp::key(&g.id)) {
            Some(c) => gages.push(feature(json!({"type": "Point", "coordinates": place.pt(*c)}), p)),
            None => unplaced_gages.push(json!({"type": "Feature", "geometry": null, "properties": Value::Object(p)})),
        }
    }

    // Objects with nothing to draw still carry their attributes: they go in tables.
    for (rows, what) in [
        (unplaced_subs, "subcatchments (no polygon)"),
        (unplaced_links, "links (no coordinates)"),
        (unplaced_nodes, "nodes (no coordinates)"),
        (unplaced_gages, "rain gages (no symbol)"),
    ] {
        if !rows.is_empty() {
            messages.push(format!("{} {what}: listed in a table instead of drawn", rows.len()));
            tables.push(json!({"name": format!("{stem} {what}"), "geojson": collection(rows, &None)}));
        }
    }

    let mut layers = Vec::new();
    for (feats, what) in [(subs, "subcatchments"), (links, "links"), (nodes, "nodes"), (gages, "rain gages")] {
        if !feats.is_empty() {
            layers.push(json!({"name": format!("{stem} {what}"), "geojson": collection(feats, &crs)}));
        }
    }
    Ok(json!({"layers": layers, "tables": tables, "messages": messages}))
}

fn link_props(k: &inp::Link, m: &inp::Model, res: &rpt::Results, len: &str) -> Map<String, Value> {
    let mut p = Map::new();
    p.insert("id".into(), json!(k.id));
    // the input section's kind; a run adds rpt_type, the report's word (CHANNEL, DUMMY...)
    p.insert("type".into(), json!(k.kind));
    p.insert("from_node".into(), json!(k.from));
    p.insert("to_node".into(), json!(k.to));
    if k.kind == "CONDUIT" {
        p.insert(format!("length_{len}"), opt(k.length));
        p.insert("roughness".into(), opt(k.roughness));
    }
    if let Some(x) = m.xsects.get(&inp::key(&k.id)) {
        p.insert("shape".into(), json!(x.shape));
        p.insert(format!("geom1_{len}"), opt(x.geom1));
        if k.kind == "CONDUIT" {
            p.insert("barrels".into(), opt(x.barrels.or(Some(1.0))));
        }
    }
    merge(&mut p, res.link.get(&inp::key(&k.id)));
    p
}

/// The EPSG code, refused unless RustGIS can project it (an unknown code would put the
/// model in the wrong place without a word).
pub fn epsg_check(raw: &str) -> Result<Option<u32>, String> {
    let raw = raw.trim();
    let raw = raw.strip_prefix("EPSG:").or_else(|| raw.strip_prefix("epsg:")).unwrap_or(raw).trim();
    if raw.is_empty() {
        return Ok(None);
    }
    let Ok(c) = raw.parse::<u32>() else {
        return Err(format!("'{raw}' is not an EPSG code (a whole number such as 3435)"));
    };
    if epsg::known(c) {
        return Ok(Some(c));
    }
    Err(if c > 65_535 {
        format!(
            "{c} is not an EPSG code (six-digit codes such as 102xxx are Esri's). RustGIS reads a layer's coordinate system only by EPSG code: give the EPSG code for the same system, or leave EPSG empty to draw the model around the anchor"
        )
    } else {
        format!(
            "RustGIS 0.5.0 has no definition for EPSG:{c}, so it would read the coordinates as longitude/latitude and draw the model in the wrong place. Check the code, or leave EPSG empty to draw the model around the anchor"
        )
    })
}

fn handle(mut req: Request) -> Result<Value, String> {
    // Where EPA's C reads memory it does not own (the code-review memory-error decks), safe
    // Rust stops at the bad index instead. A panic aborts the module, so say where first, in
    // the log. RustGIS itself reports only that the tool stopped with an error (it drops what
    // a failed tool logged); hosts that keep the log, such as the companion HTML page, show
    // the line.
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info| rustgis_plugin::host::log(&format!("swmm5-rust stopped: {info}"))));
    match (req.kind, req.id.as_str()) {
        (Kind::Tool, "run-swmm") | (Kind::Tool, "map-swmm") => {
            let run = req.id == "run-swmm";
            let epsg = epsg_check(req.str("epsg").unwrap_or(""))?;
            let anchor = [req.f64("anchor_lon").unwrap_or(ANCHOR[0]), req.f64("anchor_lat").unwrap_or(ANCHOR[1])];
            if !(-180.0..=180.0).contains(&anchor[0]) || !(-85.0..=85.0).contains(&anchor[1]) {
                return Err("the anchor must be a longitude in -180..180 and a latitude in -85..85".into());
            }
            let report = run && req.bool("report").unwrap_or(false);
            let file = req.take_file("model")?;
            let mut data = Vec::new();
            for k in ["data1", "data2", "data3"] {
                if req.files.contains_key(k) {
                    let f = req.take_file(k)?;
                    data.push((f.name, f.bytes));
                }
            }
            execute(&Job { name: file.name, bytes: file.bytes, data, epsg, anchor, report, run })
        }
        (Kind::Command, "about") => {
            let v = swmm_version();
            let text = format!(
                "swmm5-rust {VERSION}: EPA SWMM {}.{}.{} translated into safe Rust (github.com/SWMMBobSWMM6/swmm5-rust), running in RustGIS's plugin sandbox",
                v / 10000,
                (v / 1000) % 10,
                v % 1000
            );
            #[cfg(target_arch = "wasm32")]
            rustgis_plugin::host::notify(&text);
            Ok(json!({"plugin": "swmm5-rust", "version": VERSION, "swmm_version": v, "text": text}))
        }
        (_, other) => Err(format!("swmm5-rust has no '{other}'")),
    }
}

rustgis_plugin::export!(handle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epsg_codes() {
        assert_eq!(epsg_check(" EPSG:3435 "), Ok(Some(3435)));
        assert_eq!(epsg_check(""), Ok(None));
        assert!(epsg_check("102671").unwrap_err().contains("Esri"));
        assert!(epsg_check("1").unwrap_err().contains("no definition"));
        assert!(epsg_check("abc").unwrap_err().contains("not an EPSG code"));
    }

    #[test]
    fn data_names() {
        let f = |n: &str| (n.to_string(), vec![0u8]);
        assert!(check_data(&[f("rain.dat"), f("flows.txt")]).is_ok());
        assert!(check_data(&[f("rain.dat"), f("RAIN.DAT")]).unwrap_err().contains("both named"));
        assert!(check_data(&[f("Model.OUT")]).unwrap_err().contains("cannot be named"));
    }
}
