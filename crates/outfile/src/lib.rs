//! swmm-out: reader for the binary results (`.out`) file of EPA SWMM 5.
//!
//! The layout is the one `output.c` writes (here `crates/engine/src/output.rs`,
//! which is held byte for byte to EPA SWMM 5.2.4):
//!
//! ```text
//! header   7 x INT4   magic, version, flow units, counts of reported
//!                     subcatchments, nodes, links, and of pollutants
//! names    for each subcatchment, node, link, pollutant: INT4 length + bytes
//!          then one INT4 concentration-unit code per pollutant
//! props    for subcatchments, nodes, links: INT4 count, that many INT4 codes,
//!          then each object's values (the type code is an INT4, the rest REAL4)
//! vars     for subcatchments, nodes, links, system: INT4 count + INT4 codes
//!          REAL8 start date, INT4 report step (s)
//! results  per reporting period: REAL8 date, then REAL4 values for every
//!          subcatchment, node, link, and the system
//! closing  6 x INT4   names position, props position, results position,
//!                     number of periods, error code, magic
//! ```
//!
//! Every count is taken from the file, so a file with more reporting variables
//! than SWMM 5.2 writes (OpenSWMM 6 stamps version 60000 on this layout) still
//! reads; variables this reader has no name for are called `code_N`.
//!
//! The reader walks the file from byte 0 and uses the closing record only as a
//! check. EPA writes the three positions as the low 32 bits of a 64-bit file
//! offset, so above 4 GiB the closing record cannot be trusted for seeking.
//!
//! What has been checked, and against what, is in this crate's README.
#![forbid(unsafe_code)]

use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;

/// First and last four bytes of a complete file.
pub const MAGIC: i32 = 516114522;

/// Property codes (`output.c`: INPUT_TYPE_CODE ... INPUT_LENGTH).
pub const PROP_TYPE: i32 = 0;
pub const PROP_AREA: i32 = 1;
pub const PROP_INVERT: i32 = 2;
pub const PROP_MAX_DEPTH: i32 = 3;
pub const PROP_OFFSET: i32 = 4;
pub const PROP_LENGTH: i32 = 5;

pub const FLOW_UNITS: [&str; 6] = ["CFS", "GPM", "MGD", "CMS", "LPS", "MLD"];
pub const NODE_TYPES: [&str; 4] = ["JUNCTION", "OUTFALL", "STORAGE", "DIVIDER"];
pub const LINK_TYPES: [&str; 5] = ["CONDUIT", "PUMP", "ORIFICE", "WEIR", "OUTLET"];
pub const QUALITY_UNITS: [&str; 3] = ["mg/L", "ug/L", "#/L"];

// Variable names by code, as SWMM 5.1 and 5.2 use the codes. They are the
// names the swmm-run-record JSON uses, so a peak here and a peak there carry
// the same key.
const SUB_VARS: [&str; 8] = [
    "rainfall",
    "snow_depth",
    "evaporation_loss",
    "infiltration_loss",
    "runoff",
    "groundwater_flow",
    "groundwater_elevation",
    "soil_moisture",
];
const NODE_VARS: [&str; 6] = [
    "depth",
    "head",
    "volume",
    "lateral_inflow",
    "total_inflow",
    "flooding",
];
const LINK_VARS: [&str; 5] = ["flow", "depth", "velocity", "volume", "capacity"];
const SYS_VARS: [&str; 15] = [
    "air_temperature",
    "rainfall",
    "snow_depth",
    "infiltration",
    "runoff",
    "dry_weather_inflow",
    "groundwater_inflow",
    "rdii_inflow",
    "external_inflow",
    "total_lateral_inflow",
    "flooding",
    "outfall_outflow",
    "storage_volume",
    "evaporation",
    "pet",
];

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    /// The bytes are not a SWMM output file, or not a complete one.
    Format(String),
    /// An object, variable or period that the file does not hold.
    NotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{e}"),
            Error::Format(s) | Error::NotFound(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// The four kinds of thing a results record holds values for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Subcatchment,
    Node,
    Link,
    System,
}

impl Kind {
    pub const ALL: [Kind; 4] = [Kind::Subcatchment, Kind::Node, Kind::Link, Kind::System];

    fn slot(self) -> usize {
        match self {
            Kind::Subcatchment => 0,
            Kind::Node => 1,
            Kind::Link => 2,
            Kind::System => 3,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Subcatchment => "subcatchment",
            Kind::Node => "node",
            Kind::Link => "link",
            Kind::System => "system",
        }
    }

    /// `sub`, `subcatch`, `subcatchment(s)`, `node(s)`, `link(s)`, `sys`, `system`.
    pub fn parse(s: &str) -> Option<Kind> {
        let s = s.to_ascii_lowercase();
        match s.as_str() {
            "sub" | "subs" | "subcatch" | "subcatchment" | "subcatchments" => {
                Some(Kind::Subcatchment)
            }
            "node" | "nodes" => Some(Kind::Node),
            "link" | "links" => Some(Kind::Link),
            "sys" | "system" => Some(Kind::System),
            _ => None,
        }
    }
}

/// One reporting variable: its code in the file, a name, and its unit for this file's unit system.
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub code: i32,
    pub name: String,
    pub unit: String,
}

/// The saved properties of one kind of object.
#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Property codes in file order (`PROP_*`). Links carry `PROP_OFFSET` twice: upstream, then downstream.
    pub codes: Vec<i32>,
    /// One row per object, one value per code. A type code is held as its integer value.
    pub values: Vec<Vec<f64>>,
}

impl Props {
    /// The `nth` (0-based) value with property code `code` for object `index`.
    pub fn get(&self, index: usize, code: i32, nth: usize) -> Option<f64> {
        let col = self
            .codes
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == code)
            .nth(nth)?
            .0;
        self.values.get(index)?.get(col).copied()
    }
}

/// Peak and summary of one variable of one object over the reporting periods in the file.
///
/// These are statistics of what was *saved*: one value per reporting step. The
/// summary tables of the `.rpt` are kept by the engine over every routing step
/// and can be higher.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stat {
    pub max: f32,
    /// Period (0-based) of the first occurrence of `max`.
    pub max_period: usize,
    pub min: f32,
    pub min_period: usize,
    pub mean: f64,
    /// Value of the last period.
    pub last: f32,
    /// Periods with a value above zero.
    pub positive_periods: usize,
    /// Periods whose value is NaN (left out of max, min and mean).
    pub nan_periods: usize,
}

/// A SWMM binary results file, open for reading.
pub struct OutFile<R> {
    r: R,
    len: u64,
    /// Version stamp of the engine that wrote the file (52004 for SWMM 5.2.4).
    pub version: i32,
    pub flow_unit_code: i32,
    pub subcatchments: Vec<String>,
    pub nodes: Vec<String>,
    pub links: Vec<String>,
    pub pollutants: Vec<String>,
    /// Concentration-unit code of each pollutant (0 mg/L, 1 ug/L, 2 #/L).
    pub pollutant_unit_codes: Vec<i32>,
    pub subcatchment_props: Props,
    pub node_props: Props,
    pub link_props: Props,
    var_codes: [Vec<i32>; 4],
    /// Date the reporting clock starts from, in days since 30 December 1899.
    /// The first saved period is one report step after it.
    pub start_date: f64,
    /// Seconds between reporting periods.
    pub report_step: i32,
    /// Byte positions found by walking the file.
    pub names_pos: u64,
    pub props_pos: u64,
    pub results_pos: u64,
    /// Bytes in one period's record.
    pub period_bytes: u64,
    pub n_periods: usize,
    /// The engine's error code from the closing record; `None` when the file has no closing record.
    pub error_code: Option<i32>,
    /// False when the closing record is missing (a run that was stopped or crashed).
    pub complete: bool,
    /// Anything about the layout that did not check exactly but did not stop the read.
    pub notes: Vec<String>,
}

impl OutFile<BufReader<File>> {
    /// Opens a complete results file. A file without its closing record is an error; see [`OutFile::open_partial`].
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::new(BufReader::new(File::open(path)?), false)
    }

    /// Opens a results file whose run did not finish: the periods that were written in full are readable.
    pub fn open_partial(path: impl AsRef<Path>) -> Result<Self> {
        Self::new(BufReader::new(File::open(path)?), true)
    }
}

impl OutFile<Cursor<Vec<u8>>> {
    /// Reads a results file held in memory (a browser, a test).
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Self::new(Cursor::new(bytes), false)
    }
}

/// An object name as text. Names are whatever bytes the input file held: read as UTF-8 when
/// they are UTF-8, else one character per byte (Latin-1). Anything that pairs names from an
/// `.inp` with names from an `.out` should decode both with this.
pub fn decode_name(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => bytes.iter().map(|&c| c as char).collect(),
    }
}

fn fmt_err<T>(msg: impl Into<String>) -> Result<T> {
    Err(Error::Format(msg.into()))
}

struct Walk<'a, R> {
    r: &'a mut R,
    pos: u64,
    len: u64,
}

impl<R: Read> Walk<'_, R> {
    fn need(&self, n: u64, what: &str) -> Result<()> {
        if self.pos + n > self.len {
            return fmt_err(format!(
                "the file ends inside {what} (byte {} of {})",
                self.pos, self.len
            ));
        }
        Ok(())
    }

    fn bytes(&mut self, n: usize, what: &str) -> Result<Vec<u8>> {
        self.need(n as u64, what)?;
        let mut b = vec![0u8; n];
        self.r.read_exact(&mut b)?;
        self.pos += n as u64;
        Ok(b)
    }

    fn i32(&mut self, what: &str) -> Result<i32> {
        let b = self.bytes(4, what)?;
        Ok(i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn f32(&mut self, what: &str) -> Result<f32> {
        let b = self.bytes(4, what)?;
        Ok(f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn f64(&mut self, what: &str) -> Result<f64> {
        let b = self.bytes(8, what)?;
        Ok(f64::from_le_bytes([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        ]))
    }

    /// A count read from the file, refused when it could not fit in what is left of it.
    fn count(&mut self, what: &str, bytes_each: u64) -> Result<usize> {
        let n = self.i32(what)?;
        if n < 0 || n as u64 * bytes_each > self.len - self.pos {
            return fmt_err(format!("impossible count for {what}: {n}"));
        }
        Ok(n as usize)
    }

    fn name(&mut self) -> Result<String> {
        let n = self.count("an object name", 1)?;
        let b = self.bytes(n, "an object name")?;
        Ok(decode_name(&b))
    }

    fn props(&mut self, n_objects: usize, what: &str) -> Result<Props> {
        let k = self.count(what, 4)?;
        let mut codes = Vec::with_capacity(k);
        for _ in 0..k {
            codes.push(self.i32(what)?);
        }
        self.need(4 * k as u64 * n_objects as u64, what)?;
        let mut values = Vec::with_capacity(n_objects);
        for _ in 0..n_objects {
            let mut row = Vec::with_capacity(k);
            for &code in &codes {
                // The engine writes the type code with fwrite(&k, sizeof(INT4), ...) and every
                // other property as a REAL4. Reading the type as a float gives 1.4e-45 for
                // OUTFALL, which truncates to 0 (JUNCTION).
                row.push(if code == PROP_TYPE {
                    self.i32(what)? as f64
                } else {
                    self.f32(what)? as f64
                });
            }
            values.push(row);
        }
        Ok(Props { codes, values })
    }
}

impl<R: Read + Seek> OutFile<R> {
    /// Reads the header of a results file from any seekable source. With `allow_partial`,
    /// a file that lacks its closing record is accepted and its whole periods are readable.
    pub fn new(mut r: R, allow_partial: bool) -> Result<Self> {
        let len = r.seek(SeekFrom::End(0))?;
        r.seek(SeekFrom::Start(0))?;
        if len < 28 {
            return fmt_err(format!("too short to be a SWMM output file ({len} bytes)"));
        }
        let mut w = Walk {
            r: &mut r,
            pos: 0,
            len,
        };
        let magic = w.i32("the header")?;
        if magic != MAGIC {
            return fmt_err(format!(
                "not a SWMM output file (it starts with {magic}, not {MAGIC})"
            ));
        }
        let version = w.i32("the header")?;
        let flow_unit_code = w.i32("the header")?;
        let n_sub = w.count("the subcatchment count", 4)?;
        let n_node = w.count("the node count", 4)?;
        let n_link = w.count("the link count", 4)?;
        let n_poll = w.count("the pollutant count", 4)?;

        let names_pos = w.pos;
        let names = |n: usize, w: &mut Walk<R>| -> Result<Vec<String>> {
            (0..n).map(|_| w.name()).collect()
        };
        let subcatchments = names(n_sub, &mut w)?;
        let nodes = names(n_node, &mut w)?;
        let links = names(n_link, &mut w)?;
        let pollutants = names(n_poll, &mut w)?;
        let mut pollutant_unit_codes = Vec::with_capacity(n_poll);
        for _ in 0..n_poll {
            pollutant_unit_codes.push(w.i32("the pollutant units")?);
        }

        let props_pos = w.pos;
        let subcatchment_props = w.props(n_sub, "the subcatchment properties")?;
        let node_props = w.props(n_node, "the node properties")?;
        let link_props = w.props(n_link, "the link properties")?;

        let mut var_codes: [Vec<i32>; 4] = Default::default();
        for (slot, what) in [
            "the subcatchment variables",
            "the node variables",
            "the link variables",
            "the system variables",
        ]
        .iter()
        .enumerate()
        {
            let k = w.count(what, 4)?;
            for _ in 0..k {
                var_codes[slot].push(w.i32(what)?);
            }
        }
        let start_date = w.f64("the start date")?;
        let report_step = w.i32("the report step")?;
        let results_pos = w.pos;

        let values = n_sub as u64 * var_codes[0].len() as u64
            + n_node as u64 * var_codes[1].len() as u64
            + n_link as u64 * var_codes[2].len() as u64
            + var_codes[3].len() as u64;
        let period_bytes = 8 + 4 * values;

        // The closing record, if the run got as far as writing it.
        let mut notes = Vec::new();
        let mut closing: Option<[i32; 6]> = None;
        if len >= results_pos + 24 {
            r.seek(SeekFrom::Start(len - 24))?;
            let mut b = [0u8; 24];
            r.read_exact(&mut b)?;
            let mut t = [0i32; 6];
            for (i, v) in t.iter_mut().enumerate() {
                *v = i32::from_le_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]]);
            }
            if t[5] == MAGIC {
                closing = Some(t);
            }
        }

        let (n_periods, error_code, complete) = match closing {
            Some(t) => {
                // Positions are the low 32 bits of the real ones.
                for (name, walked, said) in [
                    ("names", names_pos, t[0]),
                    ("properties", props_pos, t[1]),
                    ("results", results_pos, t[2]),
                ] {
                    if walked as u32 != said as u32 {
                        return fmt_err(format!(
                            "layout check failed: the {name} start at byte {walked} by the header, {} by the closing record",
                            said as u32
                        ));
                    }
                }
                let room = (len - 24 - results_pos) / period_bytes;
                let said = t[3] as u32 as u64;
                if said > room {
                    return fmt_err(format!(
                        "the closing record counts {said} periods of {period_bytes} bytes; only {room} fit in the file"
                    ));
                }
                let spare = len - 24 - results_pos - said * period_bytes;
                if spare != 0 {
                    notes.push(format!(
                        "{spare} bytes lie between the last counted period and the closing record"
                    ));
                }
                (said as usize, Some(t[4]), true)
            }
            None if allow_partial => {
                let room = (len - results_pos) / period_bytes;
                notes.push(format!(
                    "no closing record: the run did not finish; {room} whole periods are readable ({} trailing bytes ignored)",
                    len - results_pos - room * period_bytes
                ));
                (room as usize, None, false)
            }
            None => {
                return fmt_err("not a complete SWMM output file: the closing record is missing (the run did not finish)");
            }
        };

        Ok(OutFile {
            r,
            len,
            version,
            flow_unit_code,
            subcatchments,
            nodes,
            links,
            pollutants,
            pollutant_unit_codes,
            subcatchment_props,
            node_props,
            link_props,
            var_codes,
            start_date,
            report_step,
            names_pos,
            props_pos,
            results_pos,
            period_bytes,
            n_periods,
            error_code,
            complete,
            notes,
        })
    }

    /// Size of the file in bytes.
    pub fn file_len(&self) -> u64 {
        self.len
    }

    /// `CFS`, `GPM`, `MGD`, `CMS`, `LPS` or `MLD`.
    pub fn flow_units(&self) -> &'static str {
        FLOW_UNITS
            .get(self.flow_unit_code as usize)
            .copied()
            .unwrap_or("?")
    }

    /// True for CFS, GPM and MGD: lengths in feet, areas in acres.
    pub fn is_us(&self) -> bool {
        self.flow_unit_code < 3
    }

    /// Names of the reported objects of a kind, in file order. The system has none.
    pub fn ids(&self, kind: Kind) -> &[String] {
        match kind {
            Kind::Subcatchment => &self.subcatchments,
            Kind::Node => &self.nodes,
            Kind::Link => &self.links,
            Kind::System => &[],
        }
    }

    /// Number of objects of a kind that have results (1 for the system).
    pub fn count(&self, kind: Kind) -> usize {
        match kind {
            Kind::System => 1,
            k => self.ids(k).len(),
        }
    }

    pub fn index_of(&self, kind: Kind, id: &str) -> Option<usize> {
        self.ids(kind).iter().position(|s| s == id)
    }

    pub fn props(&self, kind: Kind) -> Option<&Props> {
        match kind {
            Kind::Subcatchment => Some(&self.subcatchment_props),
            Kind::Node => Some(&self.node_props),
            Kind::Link => Some(&self.link_props),
            Kind::System => None,
        }
    }

    /// `JUNCTION`, `OUTFALL`, `STORAGE` or `DIVIDER`.
    pub fn node_type(&self, index: usize) -> Option<&'static str> {
        let t = self.node_props.get(index, PROP_TYPE, 0)? as usize;
        NODE_TYPES.get(t).copied()
    }

    /// `CONDUIT`, `PUMP`, `ORIFICE`, `WEIR` or `OUTLET`.
    pub fn link_type(&self, index: usize) -> Option<&'static str> {
        let t = self.link_props.get(index, PROP_TYPE, 0)? as usize;
        LINK_TYPES.get(t).copied()
    }

    /// The reporting variables of a kind, in file order, named and with units.
    pub fn variables(&self, kind: Kind) -> Vec<Variable> {
        let base: &[&str] = match kind {
            Kind::Subcatchment => &SUB_VARS,
            Kind::Node => &NODE_VARS,
            Kind::Link => &LINK_VARS,
            Kind::System => &SYS_VARS,
        };
        self.var_codes[kind.slot()]
            .iter()
            .map(|&code| {
                let c = code as usize;
                if code >= 0 && c < base.len() {
                    Variable {
                        code,
                        name: base[c].to_string(),
                        unit: self.unit_of(kind, code).to_string(),
                    }
                } else if kind != Kind::System
                    && code >= 0
                    && c - base.len() < self.pollutants.len()
                {
                    let p = c - base.len();
                    let unit = QUALITY_UNITS
                        .get(self.pollutant_unit_codes[p] as usize)
                        .copied()
                        .unwrap_or("?");
                    Variable {
                        code,
                        name: format!("quality:{}", self.pollutants[p]),
                        unit: unit.to_string(),
                    }
                } else {
                    Variable {
                        code,
                        name: format!("code_{code}"),
                        unit: String::new(),
                    }
                }
            })
            .collect()
    }

    fn unit_of(&self, kind: Kind, code: i32) -> &'static str {
        let us = self.is_us();
        let flow = self.flow_units();
        let len = if us { "ft" } else { "m" };
        let vol = if us { "ft3" } else { "m3" };
        let rain = if us { "in/hr" } else { "mm/hr" };
        let depth_small = if us { "in" } else { "mm" };
        let per_day = if us { "in/day" } else { "mm/day" };
        match (kind, code) {
            (Kind::Subcatchment, 0 | 3) => rain,
            (Kind::Subcatchment, 1) => depth_small,
            (Kind::Subcatchment, 2) => per_day,
            (Kind::Subcatchment, 4 | 5) => flow,
            (Kind::Subcatchment, 6) => len,
            (Kind::Subcatchment, 7) => "fraction",
            (Kind::Node, 0 | 1) => len,
            (Kind::Node, 2) => vol,
            (Kind::Node, 3..=5) => flow,
            (Kind::Link, 0) => flow,
            (Kind::Link, 1) => len,
            (Kind::Link, 2) => {
                if us {
                    "ft/s"
                } else {
                    "m/s"
                }
            }
            (Kind::Link, 3) => vol,
            (Kind::Link, 4) => "fraction",
            (Kind::System, 0) => {
                if us {
                    "deg F"
                } else {
                    "deg C"
                }
            }
            (Kind::System, 1 | 3) => rain,
            (Kind::System, 2) => depth_small,
            (Kind::System, 4..=11) => flow,
            (Kind::System, 12) => vol,
            (Kind::System, 13 | 14) => per_day,
            _ => "",
        }
    }

    /// Position of a variable among a kind's variables, by name (`depth`, `quality:TSS`, a bare
    /// pollutant name, `code_7`) or by number.
    pub fn var_index(&self, kind: Kind, name: &str) -> Option<usize> {
        let vars = self.variables(kind);
        if let Some(i) = vars.iter().position(|v| v.name.eq_ignore_ascii_case(name)) {
            return Some(i);
        }
        if let Some(i) = vars
            .iter()
            .position(|v| v.name.strip_prefix("quality:").is_some_and(|p| p == name))
        {
            return Some(i);
        }
        name.parse::<usize>().ok().filter(|&i| i < vars.len())
    }

    /// Byte offset, inside one period's record, of a value.
    fn offset_in_period(&self, kind: Kind, index: usize, var: usize) -> Result<u64> {
        let n = [
            self.subcatchments.len(),
            self.nodes.len(),
            self.links.len(),
            1,
        ];
        let k: Vec<usize> = self.var_codes.iter().map(Vec::len).collect();
        let slot = kind.slot();
        if index >= n[slot] {
            return Err(Error::NotFound(format!(
                "{} index {index}: the file holds {}",
                kind.as_str(),
                n[slot]
            )));
        }
        if var >= k[slot] {
            return Err(Error::NotFound(format!(
                "{} variable {var}: the file holds {}",
                kind.as_str(),
                k[slot]
            )));
        }
        let before: usize = (0..slot).map(|s| n[s] * k[s]).sum();
        Ok(8 + 4 * (before + index * k[slot] + var) as u64)
    }

    fn check_period(&self, period: usize) -> Result<()> {
        if period >= self.n_periods {
            return Err(Error::NotFound(format!(
                "period {period}: the file holds {}",
                self.n_periods
            )));
        }
        Ok(())
    }

    /// The date of a period by the reporting clock: start date + (period + 1) report steps.
    /// [`OutFile::dates`] reads the dates the engine stored, which is what the time base is.
    pub fn nominal_date(&self, period: usize) -> f64 {
        self.start_date + (period as f64 + 1.0) * self.report_step as f64 / 86400.0
    }

    /// The date stored with each period, in days since 30 December 1899.
    pub fn dates(&mut self) -> Result<Vec<f64>> {
        let mut out = Vec::with_capacity(self.n_periods);
        let mut b = [0u8; 8];
        for k in 0..self.n_periods {
            self.r.seek(SeekFrom::Start(
                self.results_pos + k as u64 * self.period_bytes,
            ))?;
            self.r.read_exact(&mut b)?;
            out.push(f64::from_le_bytes(b));
        }
        Ok(out)
    }

    /// The date stored with one period.
    pub fn date(&mut self, period: usize) -> Result<f64> {
        self.check_period(period)?;
        let mut b = [0u8; 8];
        self.r.seek(SeekFrom::Start(
            self.results_pos + period as u64 * self.period_bytes,
        ))?;
        self.r.read_exact(&mut b)?;
        Ok(f64::from_le_bytes(b))
    }

    /// One variable of one object over every period, as stored (single precision).
    /// For the system pass index 0. Pair it with [`OutFile::dates`].
    pub fn series(&mut self, kind: Kind, index: usize, var: usize) -> Result<Vec<f32>> {
        let off = self.results_pos + self.offset_in_period(kind, index, var)?;
        let mut out = Vec::with_capacity(self.n_periods);
        let mut b = [0u8; 4];
        for k in 0..self.n_periods {
            self.r
                .seek(SeekFrom::Start(off + k as u64 * self.period_bytes))?;
            self.r.read_exact(&mut b)?;
            out.push(f32::from_le_bytes(b));
        }
        Ok(out)
    }

    /// A series by names: `series_by_name(Kind::Node, "J1", "depth")`. For the system the id is ignored.
    pub fn series_by_name(&mut self, kind: Kind, id: &str, var: &str) -> Result<Vec<f32>> {
        let index = match kind {
            Kind::System => 0,
            k => self.index_of(k, id).ok_or_else(|| {
                Error::NotFound(format!("no {} named '{id}' in the file", k.as_str()))
            })?,
        };
        let v = self.var_index(kind, var).ok_or_else(|| {
            Error::NotFound(format!("no {} variable '{var}' in the file", kind.as_str()))
        })?;
        self.series(kind, index, v)
    }

    /// Every value of one period after its date, in file order: subcatchments, nodes, links, system.
    pub fn period_values(&mut self, period: usize) -> Result<Vec<f32>> {
        self.check_period(period)?;
        let n = (self.period_bytes - 8) as usize;
        let mut b = vec![0u8; n];
        self.r.seek(SeekFrom::Start(
            self.results_pos + period as u64 * self.period_bytes + 8,
        ))?;
        self.r.read_exact(&mut b)?;
        Ok(b.chunks_exact(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect())
    }

    /// One variable of every object of a kind at one period: the values a map of that moment shows.
    pub fn snapshot(&mut self, kind: Kind, var: usize, period: usize) -> Result<Vec<f32>> {
        let first = self.offset_in_period(kind, 0, var)? as usize - 8;
        let stride = self.var_codes[kind.slot()].len();
        let all = self.period_values(period)?;
        Ok((0..self.count(kind))
            .map(|i| all[first / 4 + i * stride])
            .collect())
    }

    /// Peaks of every variable of every object of a kind, in one pass over the file:
    /// `stats(kind)?[object][variable]`. Empty when the file holds no periods.
    pub fn stats(&mut self, kind: Kind) -> Result<Vec<Vec<Stat>>> {
        let n_obj = self.count(kind);
        let n_var = self.var_codes[kind.slot()].len();
        if self.n_periods == 0 || n_obj == 0 || n_var == 0 {
            return Ok(Vec::new());
        }
        let first = self.offset_in_period(kind, 0, 0)?;
        let block = 4 * n_obj * n_var;
        let blank = Stat {
            max: f32::NEG_INFINITY,
            max_period: 0,
            min: f32::INFINITY,
            min_period: 0,
            mean: 0.0,
            last: 0.0,
            positive_periods: 0,
            nan_periods: 0,
        };
        let mut st = vec![blank; n_obj * n_var];
        let mut b = vec![0u8; block];
        for k in 0..self.n_periods {
            self.r.seek(SeekFrom::Start(
                self.results_pos + k as u64 * self.period_bytes + first,
            ))?;
            self.r.read_exact(&mut b)?;
            for (s, c) in st.iter_mut().zip(b.chunks_exact(4)) {
                let v = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                s.last = v;
                if v.is_nan() {
                    s.nan_periods += 1;
                    continue;
                }
                if v > s.max {
                    s.max = v;
                    s.max_period = k;
                }
                if v < s.min {
                    s.min = v;
                    s.min_period = k;
                }
                if v > 0.0 {
                    s.positive_periods += 1;
                }
                s.mean += v as f64;
            }
        }
        for s in st.iter_mut() {
            let n = self.n_periods - s.nan_periods;
            if n > 0 {
                s.mean /= n as f64;
            } else {
                s.max = f32::NAN;
                s.min = f32::NAN;
                s.mean = f64::NAN;
            }
        }
        Ok(st.chunks(n_var).map(<[Stat]>::to_vec).collect())
    }
}

// --- dates -------------------------------------------------------------------
//
// SWMM keeps a date as days since 30 December 1899 with the time of day as the
// fraction. These two functions follow datetime_decodeDate and
// datetime_decodeTime, so a stamp made here reads as the report file prints it:
// the day is the floor, the seconds are rounded and capped at 23:59:59.

const DATE_DELTA: i64 = 693594;

fn is_leap(y: i64) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

/// Year, month (1-12) and day of a SWMM date.
pub fn decode_date(date: f64) -> (i32, u32, u32) {
    const D1: i64 = 365;
    const D4: i64 = D1 * 4 + 1;
    const D100: i64 = D4 * 25 - 1;
    const D400: i64 = D100 * 4 + 1;
    let mut t = date.floor() as i64 + DATE_DELTA;
    if t <= 0 {
        return (0, 1, 1);
    }
    t -= 1;
    let mut y = 1 + 400 * (t / D400);
    t %= D400;
    let (mut i, mut d) = (t / D100, t % D100);
    if i == 4 {
        i -= 1;
        d += D100;
    }
    y += i * 100;
    y += (d / D4) * 4;
    d %= D4;
    let (mut i, mut d) = (d / D1, d % D1);
    if i == 4 {
        i -= 1;
        d += D1;
    }
    y += i;
    let days = [
        31,
        if is_leap(y) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 0;
    while d >= days[m] {
        d -= days[m];
        m += 1;
    }
    (y as i32, m as u32 + 1, d as u32 + 1)
}

/// Hour, minute and second of a SWMM date.
pub fn decode_time(date: f64) -> (u32, u32, u32) {
    let mut secs = ((date - date.floor()) * 86400.0 + 0.5).floor() as i64;
    if secs >= 86400 {
        secs = 86399;
    }
    let h = secs / 3600;
    (
        (if h > 23 { 0 } else { h }) as u32,
        (secs / 60 % 60) as u32,
        (secs % 60) as u32,
    )
}

/// `1997-01-01T00:05:00`.
pub fn iso(date: f64) -> String {
    let (y, mo, d) = decode_date(date);
    let (h, mi, s) = decode_time(date);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_decode() {
        // 1 January 1997 is day 35431 (Excel's serial number for it).
        assert_eq!(decode_date(35431.0), (1997, 1, 1));
        assert_eq!(iso(35431.0 + 300.0 / 86400.0), "1997-01-01T00:05:00");
        assert_eq!(decode_date(36525.0), (1999, 12, 31));
        assert_eq!(decode_date(36526.0), (2000, 1, 1));
        assert_eq!(decode_date(36585.0), (2000, 2, 29));
        assert_eq!(decode_date(2.0), (1900, 1, 1));
    }

    #[test]
    fn refuses_what_is_not_an_output_file() {
        assert!(OutFile::from_bytes(vec![0u8; 10]).is_err());
        assert!(OutFile::from_bytes(vec![0u8; 100]).is_err());
        let mut b = MAGIC.to_le_bytes().to_vec();
        b.extend_from_slice(&[0xff; 60]);
        assert!(OutFile::from_bytes(b).is_err());
    }
}
