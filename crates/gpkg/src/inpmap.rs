//! The map side of a SWMM 5 input file: where the nodes are, how the links are
//! drawn, the subcatchment outlines, and which node each link joins.
//!
//! The engine ignores these sections ([COORDINATES], [VERTICES], [POLYGONS]),
//! so they have to be read here. Section names are matched the way the engine
//! matches them (input.c `findmatch`): the line starts with the keyword, in
//! any letter case. Tokens follow the engine's rule too: blanks and tabs
//! separate, a double-quoted run is one token, and `;` ends the line.
use std::collections::HashMap;
use swmm_out::decode_name;

#[derive(Clone, Debug, PartialEq)]
pub struct InpNode {
    pub id: String,
    /// JUNCTION, OUTFALL, STORAGE or DIVIDER, by the section the node is in.
    pub kind: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InpLink {
    pub id: String,
    /// CONDUIT, PUMP, ORIFICE, WEIR or OUTLET.
    pub kind: &'static str,
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InpSubcatchment {
    pub id: String,
    pub rain_gage: String,
    pub outlet: String,
}

#[derive(Clone, Debug, Default)]
pub struct InpMap {
    pub nodes: Vec<InpNode>,
    pub links: Vec<InpLink>,
    pub subcatchments: Vec<InpSubcatchment>,
    pub coordinates: HashMap<String, (f64, f64)>,
    /// Interior points of each link's polyline, in order from the upstream end.
    pub vertices: HashMap<String, Vec<(f64, f64)>>,
    pub polygons: HashMap<String, Vec<(f64, f64)>>,
    /// The `Units` line of [MAP], if any (None, Feet, Meters, Degrees).
    pub map_units: Option<String>,
    /// Lines of the map sections that could not be read (too few tokens, a number that is not one).
    pub skipped_lines: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Section {
    Other,
    Node(&'static str),
    Link(&'static str),
    Subcatchment,
    Coordinates,
    Vertices,
    Polygons,
    Map,
}

/// Keywords in the engine's order, so a line matches here what it matches there.
const SECTIONS: [(&str, Section); 15] = [
    ("[SUBCATCHMENT", Section::Subcatchment),
    ("[SUBAREA", Section::Other),
    ("[JUNCTION", Section::Node("JUNCTION")),
    ("[OUTFALL", Section::Node("OUTFALL")),
    ("[STORAGE", Section::Node("STORAGE")),
    ("[DIVIDER", Section::Node("DIVIDER")),
    ("[CONDUIT", Section::Link("CONDUIT")),
    ("[PUMP", Section::Link("PUMP")),
    ("[ORIFICE", Section::Link("ORIFICE")),
    ("[WEIR", Section::Link("WEIR")),
    ("[OUTLET", Section::Link("OUTLET")),
    ("[COORDINATE", Section::Coordinates),
    ("[VERTICES", Section::Vertices),
    ("[POLYGON", Section::Polygons),
    ("[MAP", Section::Map),
];

fn tokens(line: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < line.len() {
        let c = line[i];
        if c == b';' {
            break;
        }
        if c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' {
            i += 1;
            continue;
        }
        let start;
        let end;
        if c == b'"' {
            start = i + 1;
            let mut j = start;
            while j < line.len() && line[j] != b'"' && line[j] != b'\n' {
                j += 1;
            }
            end = j;
            i = j + 1;
        } else {
            start = i;
            let mut j = i;
            while j < line.len() && !matches!(line[j], b' ' | b'\t' | b'\r' | b'\n' | b';') {
                j += 1;
            }
            end = j;
            i = j;
        }
        out.push(decode_name(&line[start..end]));
    }
    out
}

fn number(s: &str) -> Option<f64> {
    s.parse::<f64>().ok().filter(|v| v.is_finite())
}

impl InpMap {
    pub fn parse(bytes: &[u8]) -> InpMap {
        let mut m = InpMap::default();
        let mut section = Section::Other;
        for line in bytes.split(|&c| c == b'\n') {
            let trimmed: &[u8] = {
                let start = line
                    .iter()
                    .position(|c| !matches!(c, b' ' | b'\t'))
                    .unwrap_or(line.len());
                &line[start..]
            };
            if trimmed.first() == Some(&b'[') {
                let upper = String::from_utf8_lossy(trimmed).to_ascii_uppercase();
                section = SECTIONS
                    .iter()
                    .find(|(k, _)| upper.starts_with(k))
                    .map_or(Section::Other, |(_, s)| *s);
                continue;
            }
            if section == Section::Other {
                continue;
            }
            let t = tokens(trimmed);
            if t.is_empty() {
                continue;
            }
            match section {
                Section::Node(kind) => m.nodes.push(InpNode {
                    id: t[0].clone(),
                    kind,
                }),
                Section::Link(kind) => {
                    if t.len() >= 3 {
                        m.links.push(InpLink {
                            id: t[0].clone(),
                            kind,
                            from: t[1].clone(),
                            to: t[2].clone(),
                        });
                    } else {
                        m.skipped_lines += 1;
                    }
                }
                Section::Subcatchment => {
                    if t.len() >= 3 {
                        m.subcatchments.push(InpSubcatchment {
                            id: t[0].clone(),
                            rain_gage: t[1].clone(),
                            outlet: t[2].clone(),
                        });
                    } else {
                        m.skipped_lines += 1;
                    }
                }
                Section::Coordinates | Section::Vertices | Section::Polygons => {
                    let xy = if t.len() >= 3 {
                        number(&t[1]).zip(number(&t[2]))
                    } else {
                        None
                    };
                    match xy {
                        Some(xy) if section == Section::Coordinates => {
                            m.coordinates.insert(t[0].clone(), xy);
                        }
                        Some(xy) if section == Section::Vertices => {
                            m.vertices.entry(t[0].clone()).or_default().push(xy)
                        }
                        Some(xy) => m.polygons.entry(t[0].clone()).or_default().push(xy),
                        None => m.skipped_lines += 1,
                    }
                }
                Section::Map => {
                    if t[0].eq_ignore_ascii_case("UNITS") && t.len() >= 2 {
                        m.map_units = Some(t[1].clone());
                    }
                }
                Section::Other => {}
            }
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_map_sections() {
        let inp = b"[TITLE]\nJ1 is not a node here\n\n[JUNCTIONS]\n;;Name Elev\nJ1 10 ; a comment\n\"J 2\"\t9\n[outfalls]\nO1 0 FREE\n\
[CONDUITS]\nC1 J1 \"J 2\" 400 0.01\n[PUMPS]\nP1 \"J 2\" O1 curve\n[SUBCATCHMENTS]\nS1 RG J1 5 50 500 0.5 0\n[SUBAREAS]\nS1 0.01 0.1\n\
[COORDINATES]\nJ1 0 0\n\"J 2\" 100.5 -3e1\nO1 200 0\n[VERTICES]\nC1 50 10\nC1 60 12\n[Polygons]\nS1 0 0\nS1 0 10\nS1 10 10\nS1 bad 1\n[MAP]\nUnits Feet\n";
        let m = InpMap::parse(inp);
        assert_eq!(
            m.nodes
                .iter()
                .map(|n| (n.id.as_str(), n.kind))
                .collect::<Vec<_>>(),
            [("J1", "JUNCTION"), ("J 2", "JUNCTION"), ("O1", "OUTFALL")]
        );
        assert_eq!(m.links.len(), 2);
        assert_eq!(
            (m.links[0].from.as_str(), m.links[0].to.as_str()),
            ("J1", "J 2")
        );
        assert_eq!(m.links[1].kind, "PUMP");
        assert_eq!(m.subcatchments[0].outlet, "J1");
        assert_eq!(m.subcatchments.len(), 1);
        assert_eq!(m.coordinates["J 2"], (100.5, -30.0));
        assert_eq!(m.vertices["C1"], [(50.0, 10.0), (60.0, 12.0)]);
        assert_eq!(m.polygons["S1"].len(), 3);
        assert_eq!(m.skipped_lines, 1);
        assert_eq!(m.map_units.as_deref(), Some("Feet"));
    }
}
