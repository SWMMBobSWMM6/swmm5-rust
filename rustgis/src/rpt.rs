//! The engine's own summary tables, read back from its report (.rpt).
//!
//! Every result attribute on the map is a number EPA SWMM 5.2.4 printed in
//! its report, so a value in RustGIS's attribute table can be found, digit
//! for digit, in the .rpt.
//!
//! A row is read from the right. For each table statsrpt.c prints a fixed run
//! of cells after the object's name, so the name is whatever comes before
//! them; a quoted SWMM name may hold spaces. Where a row leaves cells blank (a
//! pump's velocity, an orifice's full flow), the row grammar of statsrpt.c
//! decides which cells are there. Every cell must be the number (or the
//! `days hr:min` time) its column promises; a row that is not (two wide
//! numbers run together, say) is left out whole and listed in `skipped`,
//! never read as something else.
//!
//! Results are keyed by the name in upper case, the way SWMM matches names.

use std::collections::HashMap;

use serde_json::{Map, Value, json};

pub type Attrs = Map<String, Value>;

#[derive(Default, Debug)]
pub struct Results {
    pub subcatch: HashMap<String, Attrs>,
    pub node: HashMap<String, Attrs>,
    pub link: HashMap<String, Attrs>,
    /// Rows that did not fit their table's grammar (table, row).
    pub skipped: Vec<(String, String)>,
    pub tables: Vec<String>,
}

/// Unit labels for field names, by unit system and flow unit.
pub struct Units {
    pub us: bool,
    pub q: String,
}

impl Units {
    pub fn new(flow_units: &str) -> Units {
        let fu = flow_units.to_ascii_uppercase();
        let us = matches!(fu.as_str(), "CFS" | "GPM" | "MGD");
        Units { us, q: fu.to_ascii_lowercase() }
    }
    fn len(&self) -> &'static str {
        if self.us { "ft" } else { "m" }
    }
    fn depth_in(&self) -> &'static str {
        if self.us { "in" } else { "mm" }
    }
    /// Millions of gallons or of liters (the report's "10^6 gal", "10^6 ltr").
    fn vol6(&self) -> &'static str {
        if self.us { "MG" } else { "ML" }
    }
    /// Thousands of cubic feet or meters (the report's "1000 ft3", "1000 m3").
    fn vol3(&self) -> &'static str {
        if self.us { "1000ft3" } else { "1000m3" }
    }
    fn vel(&self) -> &'static str {
        if self.us { "fps" } else { "mps" }
    }
}

const NODE_TYPES: [&str; 4] = ["JUNCTION", "OUTFALL", "STORAGE", "DIVIDER"];
/// As writeLinkFlows prints them: DUMMY and CHANNEL are conduits with a dummy or an
/// irregular cross section.
const LINK_TYPES: [&str; 7] = ["CONDUIT", "CHANNEL", "DUMMY", "PUMP", "ORIFICE", "WEIR", "OUTLET"];

/// A cell as a finite number.
fn num(t: &str) -> Option<f64> {
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

/// Cells as numbers, or None if any one is not.
fn nums(cells: &[&str]) -> Option<Vec<f64>> {
    cells.iter().map(|c| num(c)).collect()
}

/// "days hr:min", as getElapsedTime() prints it: time since the start of the
/// reporting period (REPORT_START), in hours, to the minute the report printed.
fn hours(days: &str, hm: &str) -> Option<f64> {
    let d: u32 = days.parse().ok()?;
    let (h, m) = hm.split_once(':')?;
    let (h, m): (u32, u32) = (h.parse().ok()?, m.parse().ok()?);
    if m >= 60 {
        return None;
    }
    Some(((d as f64 * 24.0 + h as f64 + m as f64 / 60.0) * 1e4).round() / 1e4)
}

/// A report row: its text and where each blank-separated cell starts and ends.
struct Row<'a> {
    line: &'a str,
    cells: Vec<(usize, usize)>,
}

impl<'a> Row<'a> {
    fn new(line: &'a str) -> Row<'a> {
        // (blanks are ASCII, so these offsets are char boundaries in any text)
        let b = line.as_bytes();
        let mut cells = Vec::new();
        let mut i = 0;
        while i < b.len() {
            if b[i] == b' ' || b[i] == b'\t' {
                i += 1;
                continue;
            }
            let s = i;
            while i < b.len() && b[i] != b' ' && b[i] != b'\t' {
                i += 1;
            }
            cells.push((s, i));
        }
        Row { line, cells }
    }
    fn len(&self) -> usize {
        self.cells.len()
    }
    fn cell(&self, i: usize) -> &'a str {
        let (s, e) = self.cells[i];
        &self.line[s..e]
    }
    /// The text of cells 0..k, the name.
    fn name(&self, k: usize) -> &'a str {
        &self.line[self.cells[0].0..self.cells[k - 1].1]
    }
    /// The name and the last `n` cells, when there is a name before them.
    fn split(&self, n: usize) -> Option<(&'a str, Vec<&'a str>)> {
        if self.len() < n + 1 {
            return None;
        }
        let k = self.len() - n;
        Some((self.name(k), (k..self.len()).map(|i| self.cell(i)).collect()))
    }
}

/// The data rows of the table titled `title`: the lines after its second dashed
/// line (the end of its column headings) up to the next blank or dashed line.
/// (A third dashed line sets off a "System" total row, which is not an object.)
fn rows<'a>(lines: &[&'a str], title: &str) -> Option<Vec<&'a str>> {
    let start = lines.iter().position(|l| l.trim() == title)?;
    let mut dashes = 0;
    let mut out = Vec::new();
    for l in &lines[start + 1..] {
        let tr = l.trim();
        if tr.starts_with("No ") && dashes == 0 {
            return Some(out); // "No nodes were surcharged."
        }
        // a rule is a line of dashes only ("--- Hours Full ---" is a heading)
        if tr.len() >= 3 && tr.bytes().all(|c| c == b'-') {
            dashes += 1;
            if dashes > 2 {
                break;
            }
            continue;
        }
        if dashes == 2 {
            if tr.is_empty() {
                break;
            }
            out.push(*l);
        } else if tr.starts_with("*****") && dashes == 0 && !out.is_empty() {
            break;
        }
        // a new section before any table: the title was a different section's name
        if dashes == 0 && tr.ends_with("Summary") && tr != title {
            return None;
        }
    }
    Some(out)
}

#[derive(Clone, Copy)]
enum Into {
    Subcatch,
    Node,
    Link,
}

type Read<'a> = Option<(&'a str, Vec<(String, Value)>)>;

fn table<'a>(r: &mut Results, lines: &[&'a str], title: &str, into: Into, read: impl Fn(&Row<'a>) -> Read<'a>) {
    let Some(rs) = rows(lines, title) else { return };
    r.tables.push(title.to_string());
    for line in rs {
        let row = Row::new(line);
        if row.len() == 0 {
            continue;
        }
        match read(&row) {
            Some((name, kv)) => {
                let map = match into {
                    Into::Subcatch => &mut r.subcatch,
                    Into::Node => &mut r.node,
                    Into::Link => &mut r.link,
                };
                let e = map.entry(name.to_ascii_uppercase()).or_default();
                for (k, v) in kv {
                    e.insert(k, v);
                }
            }
            None => r.skipped.push((title.to_string(), line.trim().to_string())),
        }
    }
}

/// The summary tables. `dynwave`: the run used dynamic wave routing (the Node
/// Flooding table's last column is then a ponded depth, not a ponded volume).
/// `pollutants`: how many load columns follow the flows in Outfall Loading.
pub fn parse(rpt: &str, u: &Units, dynwave: bool, pollutants: usize) -> Results {
    let lines: Vec<&str> = rpt.lines().collect();
    let mut r = Results::default();
    let (q, l, din, v6, v3, vel) = (u.q.as_str(), u.len(), u.depth_in(), u.vol6(), u.vol3(), u.vel());

    table(&mut r, &lines, "Subcatchment Runoff Summary", Into::Subcatch, |row| {
        let (name, c) = row.split(10)?;
        let v = nums(&c)?;
        Some((name, vec![
            (format!("precip_{din}"), json!(v[0])),
            (format!("runon_{din}"), json!(v[1])),
            (format!("evap_{din}"), json!(v[2])),
            (format!("infil_{din}"), json!(v[3])),
            (format!("imperv_runoff_{din}"), json!(v[4])),
            (format!("perv_runoff_{din}"), json!(v[5])),
            (format!("runoff_{din}"), json!(v[6])),
            (format!("runoff_vol_{v6}"), json!(v[7])),
            (format!("peak_runoff_{q}"), json!(v[8])),
            ("runoff_coeff".into(), json!(v[9])),
        ]))
    });

    table(&mut r, &lines, "Node Depth Summary", Into::Node, |row| {
        // type avgDepth maxDepth maxHGL days hh:mm reportedMaxDepth
        let (name, c) = row.split(7)?;
        if !NODE_TYPES.contains(&c[0]) {
            return None;
        }
        let v = nums(&[c[1], c[2], c[3], c[6]])?;
        Some((name, vec![
            (format!("avg_depth_{l}"), json!(v[0])),
            (format!("max_depth_{l}"), json!(v[1])),
            (format!("max_hgl_{l}"), json!(v[2])),
            ("t_max_depth_hr".into(), json!(hours(c[4], c[5])?)),
            (format!("rpt_max_depth_{l}"), json!(v[3])),
        ]))
    });

    table(&mut r, &lines, "Node Inflow Summary", Into::Node, |row| {
        // type maxLat maxTotal days hh:mm latVol totalVol balanceError [gal|ltr]
        // (the error is a volume, with its unit, when the node's outflow is under 1)
        let last = row.cell(row.len() - 1);
        let as_volume = last.bytes().all(|b| b.is_ascii_alphabetic());
        let (name, c) = row.split(if as_volume { 9 } else { 8 })?;
        if !NODE_TYPES.contains(&c[0]) {
            return None;
        }
        let v = nums(&[c[1], c[2], c[5], c[6], c[7]])?;
        let (pct, vol) = if as_volume { (Value::Null, json!(v[4])) } else { (json!(v[4]), Value::Null) };
        let vol_unit = if u.us { "gal" } else { "ltr" };
        Some((name, vec![
            (format!("max_lat_inflow_{q}"), json!(v[0])),
            (format!("max_total_inflow_{q}"), json!(v[1])),
            ("t_max_inflow_hr".into(), json!(hours(c[3], c[4])?)),
            (format!("lat_inflow_vol_{v6}"), json!(v[2])),
            (format!("total_inflow_vol_{v6}"), json!(v[3])),
            ("flow_bal_err_pct".into(), pct),
            (format!("flow_bal_err_{vol_unit}"), vol),
        ]))
    });

    table(&mut r, &lines, "Node Surcharge Summary", Into::Node, |row| {
        // type hours maxAboveCrown minBelowRim
        let (name, c) = row.split(4)?;
        if !NODE_TYPES.contains(&c[0]) {
            return None;
        }
        let v = nums(&c[1..])?;
        Some((name, vec![
            ("surcharge_hr".into(), json!(v[0])),
            (format!("max_above_crown_{l}"), json!(v[1])),
            (format!("min_below_rim_{l}"), json!(v[2])),
        ]))
    });

    table(&mut r, &lines, "Node Flooding Summary", Into::Node, |row| {
        // hours maxRate days hh:mm totalVolume maxPonded(depth under DW, else volume)
        let (name, c) = row.split(6)?;
        let v = nums(&[c[0], c[1], c[4], c[5]])?;
        let last = if dynwave { format!("max_ponded_depth_{l}") } else { format!("max_ponded_vol_{v3}") };
        Some((name, vec![
            ("flood_hr".into(), json!(v[0])),
            (format!("max_flood_rate_{q}"), json!(v[1])),
            ("t_max_flood_hr".into(), json!(hours(c[2], c[3])?)),
            (format!("flood_vol_{v6}"), json!(v[2])),
            (last, json!(v[3])),
        ]))
    });

    table(&mut r, &lines, "Storage Volume Summary", Into::Node, |row| {
        // avgVol avgPct evapPct exfilPct maxVol maxPct days hh:mm maxOutflow
        let (name, c) = row.split(9)?;
        let v = nums(&[c[0], c[1], c[2], c[3], c[4], c[5], c[8]])?;
        Some((name, vec![
            (format!("avg_vol_{v3}"), json!(v[0])),
            ("avg_pct_full".into(), json!(v[1])),
            ("evap_pct_loss".into(), json!(v[2])),
            ("exfil_pct_loss".into(), json!(v[3])),
            (format!("max_vol_{v3}"), json!(v[4])),
            ("max_pct_full".into(), json!(v[5])),
            ("t_max_vol_hr".into(), json!(hours(c[6], c[7])?)),
            (format!("max_outflow_{q}"), json!(v[6])),
        ]))
    });

    table(&mut r, &lines, "Outfall Loading Summary", Into::Node, |row| {
        // freq avgFlow maxFlow totalVol, then one load per pollutant
        let (name, c) = row.split(4 + pollutants)?;
        let v = nums(&c)?;
        Some((name, vec![
            ("flow_freq_pct".into(), json!(v[0])),
            (format!("avg_flow_{q}"), json!(v[1])),
            (format!("max_flow_{q}"), json!(v[2])),
            (format!("total_vol_{v6}"), json!(v[3])),
        ]))
    });

    table(&mut r, &lines, "Link Flow Summary", Into::Link, |row| {
        // type maxFlow days hh:mm, then by type (statsrpt.c writeLinkFlows):
        //   PUMP: [max/full flow] | DUMMY, OUTLET: nothing
        //   CONDUIT/CHANNEL: veloc (or >50.00), max/full flow, [max/full depth]
        //   ORIFICE, WEIR: [max/full depth]
        // The type is the first cell after the name that is a link type followed by a
        // flow and a time.
        let p = (1..row.len().saturating_sub(3)).find(|&p| {
            LINK_TYPES.contains(&row.cell(p)) && num(row.cell(p + 1)).is_some() && hours(row.cell(p + 2), row.cell(p + 3)).is_some()
        })?;
        let name = row.name(p);
        let ty = row.cell(p);
        let rest: Vec<&str> = (p + 4..row.len()).map(|i| row.cell(i)).collect();
        let mut kv = vec![
            ("rpt_type".into(), json!(ty)),
            (format!("max_flow_{q}"), json!(num(row.cell(p + 1))?)),
            ("t_max_flow_hr".into(), json!(hours(row.cell(p + 2), row.cell(p + 3))?)),
        ];
        match ty {
            "PUMP" => {
                if rest.len() > 1 {
                    return None;
                }
                if let Some(x) = rest.first() {
                    kv.push(("max_full_flow".into(), json!(num(x)?)));
                }
            }
            "DUMMY" | "OUTLET" => {
                if !rest.is_empty() {
                    return None;
                }
            }
            "CONDUIT" | "CHANNEL" => {
                if !(2..=3).contains(&rest.len()) {
                    return None;
                }
                if rest[0] == ">50.00" {
                    kv.push((format!("max_veloc_{vel}"), Value::Null));
                    kv.push(("veloc_over_50".into(), json!(true)));
                } else {
                    kv.push((format!("max_veloc_{vel}"), json!(num(rest[0])?)));
                }
                kv.push(("max_full_flow".into(), json!(num(rest[1])?)));
                if let Some(x) = rest.get(2) {
                    kv.push(("max_full_depth".into(), json!(num(x)?)));
                }
            }
            _ => {
                if rest.len() > 1 {
                    return None;
                }
                if let Some(x) = rest.first() {
                    kv.push(("max_full_depth".into(), json!(num(x)?)));
                }
            }
        }
        Some((name, kv))
    });

    table(&mut r, &lines, "Conduit Surcharge Summary", Into::Link, |row| {
        // hours full at both ends, upstream, downstream; above full normal flow; capacity limited
        let (name, c) = row.split(5)?;
        let v = nums(&c)?;
        Some((name, vec![
            ("hrs_full_both".into(), json!(v[0])),
            ("hrs_full_up".into(), json!(v[1])),
            ("hrs_full_dn".into(), json!(v[2])),
            ("hrs_above_full_normal".into(), json!(v[3])),
            ("hrs_capacity_ltd".into(), json!(v[4])),
        ]))
    });

    table(&mut r, &lines, "Pumping Summary", Into::Link, |row| {
        let (name, c) = row.split(9)?;
        let v = nums(&c)?;
        Some((name, vec![
            ("pct_utilized".into(), json!(v[0])),
            ("startups".into(), json!(v[1])),
            (format!("min_flow_{q}"), json!(v[2])),
            (format!("avg_flow_{q}"), json!(v[3])),
            (format!("max_pump_flow_{q}"), json!(v[4])),
            (format!("pumped_vol_{v6}"), json!(v[5])),
            ("power_kwh".into(), json!(v[6])),
            ("pct_off_curve_low".into(), json!(v[7])),
            ("pct_off_curve_high".into(), json!(v[8])),
        ]))
    });

    r
}

/// An analysis option as the report prints it: `option(rpt, "Flow Routing Method")`
/// is `Some("DYNWAVE")` for "  Flow Routing Method ...... DYNWAVE".
pub fn option<'a>(rpt: &'a str, name: &str) -> Option<&'a str> {
    rpt.lines().find_map(|l| {
        let rest = l.trim_start().strip_prefix(name)?.trim_start_matches(' ');
        if !rest.starts_with('.') {
            return None;
        }
        rest.trim_start_matches(['.', ' ']).split_whitespace().next()
    })
}

/// The "Continuity Error (%)" values printed in the report section titled
/// `title` (one for runoff and flow routing, one per pollutant for quality),
/// read only up to the next section.
pub fn continuity(rpt: &str, title: &str) -> Option<String> {
    let lines: Vec<&str> = rpt.lines().collect();
    let start = lines.iter().position(|l| l.trim() == title || l.trim().starts_with(&format!("{title} ")))?;
    // below the title, the section's closing row of asterisks; the next row of them starts
    // the next section
    for l in lines.iter().skip(start + 2) {
        let t = l.trim();
        if t.starts_with("***") {
            break;
        }
        if let Some(rest) = t.strip_prefix("Continuity Error (%)") {
            let v: Vec<&str> = rest.trim_start_matches([' ', '.']).split_whitespace().collect();
            return Some(v.join(" "));
        }
    }
    None
}

/// The three lines that print the wall clock (in RustGIS's sandbox the clock reads zero).
pub const CLOCK: [&[u8]; 3] = [b"Analysis begun on", b"Analysis ended on", b"Total elapsed time"];

/// The report with its wall-clock lines removed, so two runs of one model hash the same.
pub fn without_clock(rpt: &[u8]) -> Vec<u8> {
    let kept: Vec<&[u8]> = rpt
        .split(|&c| c == b'\n')
        .filter(|line| !CLOCK.iter().any(|c| line.windows(c.len()).any(|w| w == *c)))
        .collect();
    kept.join(&b'\n')
}

#[cfg(test)]
mod tests {
    use super::*;

    const RPT: &str = "
  ******************
  Node Depth Summary
  ******************

  ---------------------------------------------------------------------------------
                                 Average  Maximum  Maximum  Time of Max    Reported
                                   Depth    Depth      HGL   Occurrence   Max Depth
  Node                 Type         Feet     Feet     Feet  days hr:min        Feet
  ---------------------------------------------------------------------------------
  9                    JUNCTION     0.17     0.23  1000.23     0  06:31        0.23
  26                   OUTFALL      0.31     0.92   970.92     1  04:02        0.92
  Main St MH 4         JUNCTION     0.10     0.20   990.20     0  05:00        0.20
  27                   JUNCTION     0.10 12345.6712345.67     0  05:00        0.20

  *******************
  Node Inflow Summary
  *******************

  -------------------------------------------------------------------------------------------------
                                  Maximum  Maximum                  Lateral       Total        Flow
                                  Lateral    Total  Time of Max      Inflow      Inflow     Balance
                                   Inflow   Inflow   Occurrence      Volume      Volume       Error
  Node                 Type           CFS      CFS  days hr:min    10^6 gal    10^6 gal     Percent
  -------------------------------------------------------------------------------------------------
  9                    JUNCTION      0.00     0.63     0  06:31           0       0.246       0.049
  11                   JUNCTION      0.00     0.00     0  00:00           0           0       0.000 gal

  **********************
  Node Surcharge Summary
  **********************

  No nodes were surcharged.

  *********************
  Node Flooding Summary
  *********************

  ---------------------------------------------------------------------------
                                                             Total   Maximum
                                 Maximum   Time of Max       Flood    Ponded
                        Hours       Rate    Occurrence      Volume     Depth
  Node                 Flooded       CFS   days hr:min    10^6 gal      Feet
  ---------------------------------------------------------------------------
  9                       0.25      1.50      0  06:31       0.010      2.00

  ***********************
  Outfall Loading Summary
  ***********************

  -----------------------------------------------------------------------
                         Flow       Avg       Max       Total      Total
                         Freq      Flow      Flow      Volume        TSS
  Outfall Node           Pcnt       CFS       CFS    10^6 gal        lbs
  -----------------------------------------------------------------------
  Out 1                 99.00      1.00      2.00       0.500     12.000
  -----------------------------------------------------------------------
  System                99.00      1.00      2.00       0.500     12.000

  ********************
  Link Flow Summary
  ********************

  -----------------------------------------------------------------------------
                                 Maximum  Time of Max   Maximum    Max/    Max/
                                  |Flow|   Occurrence   |Veloc|    Full    Full
  Link                 Type          CFS  days hr:min    ft/sec    Flow   Depth
  -----------------------------------------------------------------------------
  1                    CONDUIT      0.63     0  06:31      4.32    0.12    0.38
  17                   ORIFICE      0.63     0  06:31                      1.00
  P1                   PUMP         2.00     0  01:00              0.50
  C9                   CONDUIT     99.00     0  01:00    >50.00    1.20    1.00
  Pipe WEIR 2          CONDUIT      1.00     0  01:00      1.00    0.10    0.20
  D1                   DUMMY        1.00     0  01:00
  C10                  CONDUIT      1.00     0  01:00      1.00

  *************************
  Conduit Surcharge Summary
  *************************

  ----------------------------------------------------------------------------
                                                           Hours        Hours
                         --------- Hours Full --------   Above Full   Capacity
  Conduit                Both Ends  Upstream  Dnstream   Normal Flow   Limited
  ----------------------------------------------------------------------------
  1285                       13.90     13.90     15.55      0.01         0.01
";

    #[test]
    fn reads_tables() {
        let r = parse(RPT, &Units::new("CFS"), true, 1);
        assert_eq!(r.node["9"]["max_hgl_ft"], json!(1000.23));
        assert_eq!(r.node["26"]["t_max_depth_hr"], json!(28.0333));
        assert_eq!(r.node["MAIN ST MH 4"]["max_hgl_ft"], json!(990.2));
        assert!(!r.node.contains_key("27"), "a row with two numbers run together is not read");
        assert_eq!(r.node["11"]["flow_bal_err_pct"], Value::Null);
        assert_eq!(r.node["11"]["flow_bal_err_gal"], json!(0.0));
        assert_eq!(r.node["9"]["max_ponded_depth_ft"], json!(2.0));
        assert_eq!(r.node["OUT 1"]["total_vol_MG"], json!(0.5));
        assert!(!r.node.contains_key("SYSTEM"));
        assert_eq!(r.link["17"]["max_full_depth"], json!(1.0));
        assert!(r.link["17"].get("max_full_flow").is_none());
        assert_eq!(r.link["P1"]["max_full_flow"], json!(0.5));
        assert_eq!(r.link["C9"]["veloc_over_50"], json!(true));
        assert_eq!(r.link["PIPE WEIR 2"]["rpt_type"], json!("CONDUIT"));
        assert_eq!(r.link["D1"]["rpt_type"], json!("DUMMY"));
        assert!(!r.link.contains_key("C10"), "a conduit row needs velocity and max/full flow");
        assert_eq!(r.link["1285"]["hrs_full_dn"], json!(15.55));
        assert_eq!(r.link["1285"]["hrs_above_full_normal"], json!(0.01));
        let sk: Vec<&str> = r.skipped.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(sk, vec!["Node Depth Summary", "Link Flow Summary"]);
        assert!(r.tables.contains(&"Node Surcharge Summary".to_string()));
        // without dynamic wave the last flooding column is a volume, in thousands of ft3
        let r = parse(RPT, &Units::new("CFS"), false, 1);
        assert_eq!(r.node["9"]["max_ponded_vol_1000ft3"], json!(2.0));
        // the wrong pollutant count leaves the outfall row unread rather than misread
        let r = parse(RPT, &Units::new("CFS"), true, 0);
        assert!(!r.node.get("OUT 1").is_some_and(|a| a.contains_key("total_vol_MG")));
    }

    #[test]
    fn options() {
        let t = "  Flow Units ............... CFS\n  Flow Routing Method ...... DYNWAVE\n";
        assert_eq!(option(t, "Flow Routing Method"), Some("DYNWAVE"));
        assert_eq!(option(t, "Flow Units"), Some("CFS"));
        assert_eq!(option(t, "Surcharge Method"), None);
    }

    #[test]
    fn continuity_values() {
        let t = "  **************************        Volume         Depth\n  Runoff Quantity Continuity     acre-feet        inches\n  **************************     ---------       -------\n  Total Precipitation ......        12.129         2.050\n  Continuity Error (%) .....        -0.008\n";
        assert_eq!(continuity(t, "Runoff Quantity Continuity").as_deref(), Some("-0.008"));
        assert_eq!(continuity(t, "Flow Routing Continuity"), None);
        // a section without the line does not borrow the next section's
        let t2 = "  **************************\n  Runoff Quantity Continuity\n  **************************\n  Total Precipitation ......        12.129\n\n  **************************\n  Flow Routing Continuity\n  **************************\n  Continuity Error (%) .....        0.5\n";
        assert_eq!(continuity(t2, "Runoff Quantity Continuity"), None);
        assert_eq!(continuity(t2, "Flow Routing Continuity").as_deref(), Some("0.5"));
    }

    #[test]
    fn clock_lines_removed() {
        let b = b"a\n  Analysis begun on:  Thu\nb\n  Total elapsed time: 00:00:01\nc";
        assert_eq!(without_clock(b), b"a\nb\nc");
    }
}
