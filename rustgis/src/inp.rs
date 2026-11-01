//! What the map needs from a SWMM 5 input file: the objects, their input
//! attributes, and where they are drawn ([COORDINATES], [VERTICES],
//! [Polygons], [SYMBOLS], [MAP]).
//!
//! This is not the engine's reader (the engine reads the file itself when it
//! runs); it only gathers geometry and the handful of input values worth
//! showing in an attribute table. Bytes are read as Latin-1, so a model whose
//! names are not UTF-8 still opens, and every name survives as one char per
//! byte. Lines are split into tokens by SWMM's own rules (input.c getTokens),
//! and names are matched without regard to case, as SWMM matches them: the
//! maps below are keyed by `key(name)`.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub kind: &'static str,
    pub invert: Option<f64>,
    pub full_depth: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub id: String,
    pub kind: &'static str,
    pub from: String,
    pub to: String,
    pub length: Option<f64>,
    pub roughness: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Xsect {
    pub shape: String,
    pub geom1: Option<f64>,
    pub barrels: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Subcatch {
    pub id: String,
    pub gage: String,
    pub outlet: String,
    pub area: Option<f64>,
    pub pct_imperv: Option<f64>,
    pub width: Option<f64>,
    pub slope: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Gage {
    pub id: String,
    pub format: String,
    pub interval: String,
    pub source: String,
}

#[derive(Debug, Default)]
pub struct Model {
    pub flow_units: String,
    pub routing: String,
    pub map_units: String,
    /// Entries in [POLLUTANTS]: the load columns of the Outfall Loading table.
    pub pollutants: usize,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub subcatchments: Vec<Subcatch>,
    pub gages: Vec<Gage>,
    pub xsects: HashMap<String, Xsect>,
    pub coords: HashMap<String, [f64; 2]>,
    pub vertices: HashMap<String, Vec<[f64; 2]>>,
    pub polygons: HashMap<String, Vec<[f64; 2]>>,
    pub symbols: HashMap<String, [f64; 2]>,
}

impl Model {
    /// US customary units (CFS, GPM, MGD) or SI (CMS, LPS, MLD).
    pub fn us(&self) -> bool {
        matches!(self.flow_units.as_str(), "CFS" | "GPM" | "MGD")
    }
    /// FLOW_ROUTING names dynamic wave: SWMM takes any word that begins with DYNWAVE,
    /// or the old keyword DW (project.c, findmatch on RouteModelWords then OldRouteModelWords).
    pub fn dynwave(&self) -> bool {
        self.routing.starts_with("DYNWAVE") || self.routing.starts_with("DW")
    }
}

/// How SWMM compares names: ASCII letters without regard to case.
pub fn key(name: &str) -> String {
    name.to_ascii_uppercase()
}

/// Bytes as Latin-1 text: one char per byte, always valid.
pub fn latin1(b: &[u8]) -> String {
    b.iter().map(|&c| c as char).collect()
}

/// SWMM's token separators (consts.h SEPSTR): space, tab, newline, carriage return.
fn sep(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

/// A line's tokens, as input.c getTokens splits them: the line is cut at its
/// first `;` (even inside quotes); tokens are separated by SEPSTR only; a token
/// that begins with a double quote runs to the next quote, and may hold spaces.
pub fn tokens(line: &str) -> Vec<String> {
    let line = line.split(';').next().unwrap_or("");
    let c: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < c.len() {
        if sep(c[i]) {
            i += 1;
            continue;
        }
        let s;
        if c[i] == '"' {
            s = i + 1;
            i = s;
            while i < c.len() && c[i] != '"' && c[i] != '\n' {
                i += 1;
            }
        } else {
            s = i;
            while i < c.len() && !sep(c[i]) {
                i += 1;
            }
        }
        out.push(c[s..i].iter().collect());
        i += 1; // past the separator or the closing quote
    }
    out
}

fn num(t: &[String], i: usize) -> Option<f64> {
    t.get(i).and_then(|s| s.parse::<f64>().ok()).filter(|v| v.is_finite())
}

fn s(t: &[String], i: usize) -> String {
    t.get(i).cloned().unwrap_or_default()
}

fn xy(t: &[String]) -> Option<[f64; 2]> {
    Some([num(t, 1)?, num(t, 2)?])
}

pub fn parse(bytes: &[u8]) -> Model {
    let text = latin1(bytes);
    let mut m = Model { flow_units: "CFS".into(), routing: "KINWAVE".into(), map_units: "NONE".into(), ..Default::default() };
    let mut section = String::new();
    for raw in text.lines() {
        let line = raw.trim_matches(sep);
        if line.starts_with('[') {
            section = line.trim_matches(|c| c == '[' || c == ']').trim().to_ascii_uppercase();
            continue;
        }
        if section == "TITLE" {
            continue;
        }
        let t = tokens(line);
        if t.is_empty() {
            continue;
        }
        match section.as_str() {
            "OPTIONS" => {
                let key = t[0].to_ascii_uppercase();
                if key == "FLOW_UNITS" {
                    m.flow_units = s(&t, 1).to_ascii_uppercase();
                } else if key == "FLOW_ROUTING" {
                    m.routing = s(&t, 1).to_ascii_uppercase();
                }
            }
            "MAP" => {
                if t[0].eq_ignore_ascii_case("UNITS") {
                    m.map_units = s(&t, 1).to_ascii_uppercase();
                }
            }
            "JUNCTIONS" => m.nodes.push(Node { id: s(&t, 0), kind: "JUNCTION", invert: num(&t, 1), full_depth: num(&t, 2) }),
            "OUTFALLS" => m.nodes.push(Node { id: s(&t, 0), kind: "OUTFALL", invert: num(&t, 1), full_depth: None }),
            "DIVIDERS" => m.nodes.push(Node { id: s(&t, 0), kind: "DIVIDER", invert: num(&t, 1), full_depth: None }),
            "STORAGE" => m.nodes.push(Node { id: s(&t, 0), kind: "STORAGE", invert: num(&t, 1), full_depth: num(&t, 2) }),
            "CONDUITS" => m.links.push(Link {
                id: s(&t, 0),
                kind: "CONDUIT",
                from: s(&t, 1),
                to: s(&t, 2),
                length: num(&t, 3),
                roughness: num(&t, 4),
            }),
            "PUMPS" | "ORIFICES" | "WEIRS" | "OUTLETS" => {
                let kind = match section.as_str() {
                    "PUMPS" => "PUMP",
                    "ORIFICES" => "ORIFICE",
                    "WEIRS" => "WEIR",
                    _ => "OUTLET",
                };
                m.links.push(Link { id: s(&t, 0), kind, from: s(&t, 1), to: s(&t, 2), length: None, roughness: None });
            }
            "XSECTIONS" => {
                // name shape geom1 geom2 geom3 geom4 [barrels culvert], name CUSTOM height
                // curve 0 0 [barrels] (the engine wants six tokens for CUSTOM too, link.c),
                // name IRREGULAR transect, name STREET street
                let shape = s(&t, 1).to_ascii_uppercase();
                let (geom1, barrels) = match shape.as_str() {
                    "IRREGULAR" | "STREET" => (None, None),
                    _ => (num(&t, 2), num(&t, 6)),
                };
                m.xsects.insert(key(&s(&t, 0)), Xsect { shape, geom1, barrels });
            }
            "SUBCATCHMENTS" => m.subcatchments.push(Subcatch {
                id: s(&t, 0),
                gage: s(&t, 1),
                outlet: s(&t, 2),
                area: num(&t, 3),
                pct_imperv: num(&t, 4),
                width: num(&t, 5),
                slope: num(&t, 6),
            }),
            "RAINGAGES" => m.gages.push(Gage {
                id: s(&t, 0),
                format: s(&t, 1),
                interval: s(&t, 2),
                source: t.iter().skip(4).cloned().collect::<Vec<_>>().join(" "),
            }),
            "POLLUTANTS" => m.pollutants += 1,
            "COORDINATES" => {
                if let Some(p) = xy(&t) {
                    m.coords.insert(key(&s(&t, 0)), p);
                }
            }
            "VERTICES" => {
                if let Some(p) = xy(&t) {
                    m.vertices.entry(key(&s(&t, 0))).or_default().push(p);
                }
            }
            "POLYGONS" => {
                if let Some(p) = xy(&t) {
                    m.polygons.entry(key(&s(&t, 0))).or_default().push(p);
                }
            }
            "SYMBOLS" => {
                if let Some(p) = xy(&t) {
                    m.symbols.insert(key(&s(&t, 0)), p);
                }
            }
            _ => {}
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_like_swmm() {
        assert_eq!(tokens("  J1  10.5 ; comment"), vec!["J1", "10.5"]);
        assert_eq!(tokens(r#"RG1 VOLUME 0:15 1.0 FILE "rain file.dat" RG1 IN"#)[5], "rain file.dat");
        assert!(tokens(";; only a comment").is_empty());
        // only SEPSTR separates: a no-break space (Latin-1 0xA0) is part of a name
        assert_eq!(tokens("J\u{a0}1 5"), vec!["J\u{a0}1", "5"]);
        // the comment cut comes first, even inside quotes
        assert_eq!(tokens(r#""a;b" 1"#), vec!["a"]);
        // a quote inside a token is a character; a closing quote ends the token
        assert_eq!(tokens(r#"ab"c "d e"f"#), vec![r#"ab"c"#, "d e", "f"]);
    }

    #[test]
    fn names_match_without_case() {
        let m = parse(b"[OPTIONS]\nFLOW_ROUTING DW\n[JUNCTIONS]\nMh1 10 5\n[COORDINATES]\nMH1 1 2\n[XSECTIONS]\nC1 CUSTOM 3 Shape1 0 0 2\n[POLLUTANTS]\nTSS MG/L 0\nLead UG/L 0\n");
        assert_eq!(m.coords.get(&key(&m.nodes[0].id)), Some(&[1.0, 2.0]));
        assert_eq!(m.xsects["C1"].barrels, Some(2.0));
        assert!(m.dynwave());
        assert_eq!(m.pollutants, 2);
    }

    #[test]
    fn reads_geometry() {
        let m = parse(b"[OPTIONS]\nFLOW_UNITS LPS\n[JUNCTIONS]\nJ1 10 5\n[OUTFALLS]\nO1 8 FREE\n[CONDUITS]\nC1 J1 O1 100 0.013\n[XSECTIONS]\nC1 CIRCULAR 1.5 0 0 0 2\n[COORDINATES]\nJ1 0 0\nO1 100 0\n[VERTICES]\nC1 50 10\n[MAP]\nUNITS Meters\n");
        assert!(!m.us());
        assert_eq!(m.nodes.len(), 2);
        assert_eq!(m.links[0].length, Some(100.0));
        assert_eq!(m.xsects["C1"].barrels, Some(2.0));
        assert_eq!(m.vertices["C1"], vec![[50.0, 10.0]]);
        assert_eq!(m.map_units, "METERS");
    }
}
