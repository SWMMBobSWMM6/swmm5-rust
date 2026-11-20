//! swmmout: look inside a SWMM binary results file from the command line.
#![forbid(unsafe_code)]
use std::io::{self, BufWriter, Write};
use std::process::ExitCode;
use swmm_out::{iso, Kind, OutFile};

const HELP: &str = "\
swmmout: read a SWMM 5 binary results (.out) file

  swmmout info   FILE                 what the file holds
  swmmout json   FILE                 the same, as JSON
  swmmout list   FILE KIND            names of the reported objects (KIND: sub, node, link)
  swmmout vars   FILE KIND            the reporting variables and their units (KIND: sub, node, link, sys)
  swmmout series FILE KIND ID VAR     one time series as CSV (for sys leave ID out)
  swmmout stats  FILE KIND            peaks of every variable of every object as CSV
  swmmout dump   FILE                 every value in the file as CSV, one series per line

Add --partial after FILE to read a file whose run did not finish.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("swmmout: {e}");
            ExitCode::from(1)
        }
    }
}

fn kind(s: Option<&String>) -> Result<Kind, Box<dyn std::error::Error>> {
    let s = s.ok_or("which kind? sub, node, link or sys")?;
    Kind::parse(s).ok_or_else(|| format!("unknown kind '{s}' (sub, node, link, sys)").into())
}

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let partial = args.iter().any(|a| a == "--partial");
    let args: Vec<&String> = args.iter().filter(|a| *a != "--partial").collect();
    let (Some(cmd), Some(path)) = (args.first(), args.get(1)) else {
        print!("{HELP}");
        return Ok(());
    };
    let mut f = if partial {
        OutFile::open_partial(path)?
    } else {
        OutFile::open(path)?
    };
    let out = io::stdout();
    let mut w = BufWriter::new(out.lock());
    match cmd.as_str() {
        "info" => {
            writeln!(w, "file            {path} ({} bytes)", f.file_len())?;
            let whose = if f.dialect() == swmm_out::Dialect::OpenSwmm6 {
                " (OpenSWMM 6 numbering)"
            } else {
                ""
            };
            writeln!(w, "engine version  {}{whose}", f.version)?;
            writeln!(
                w,
                "flow units      {} ({})",
                f.flow_units(),
                if f.is_us() { "US" } else { "SI" }
            )?;
            match f.error_code {
                Some(e) => writeln!(w, "error code      {e}")?,
                None => writeln!(w, "error code      none: the file has no closing record")?,
            }
            writeln!(w, "subcatchments   {}", f.subcatchments.len())?;
            let count =
                |names: &[&str], t: &dyn Fn(usize) -> Option<&'static str>, n: usize| -> String {
                    names
                        .iter()
                        .map(|name| (name, (0..n).filter(|&i| t(i) == Some(name)).count()))
                        .filter(|(_, c)| *c > 0)
                        .map(|(name, c)| format!("{c} {name}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
            writeln!(
                w,
                "nodes           {} ({})",
                f.nodes.len(),
                count(&swmm_out::NODE_TYPES, &|i| f.node_type(i), f.nodes.len())
            )?;
            writeln!(
                w,
                "links           {} ({})",
                f.links.len(),
                count(&swmm_out::LINK_TYPES, &|i| f.link_type(i), f.links.len())
            )?;
            writeln!(
                w,
                "pollutants      {} {}",
                f.pollutants.len(),
                f.pollutants.join(", ")
            )?;
            writeln!(w, "report step     {} s", f.report_step)?;
            writeln!(w, "periods         {}", f.n_periods)?;
            if f.n_periods > 0 {
                let (first, last) = (f.date(0)?, f.date(f.n_periods - 1)?);
                writeln!(w, "first, last     {}, {}", iso(first), iso(last))?;
            }
            writeln!(
                w,
                "record          {} bytes a period, results from byte {}",
                f.period_bytes, f.results_pos
            )?;
            for k in Kind::ALL {
                let v: Vec<String> = f.variables(k).iter().map(|v| v.name.clone()).collect();
                writeln!(w, "{:<15} {}", format!("{} vars", k.as_str()), v.join(", "))?;
            }
            for n in &f.notes {
                writeln!(w, "note            {n}")?;
            }
        }
        "json" => {
            let list = |v: &[String]| {
                format!(
                    "[{}]",
                    v.iter().map(|s| json_str(s)).collect::<Vec<_>>().join(",")
                )
            };
            writeln!(w, "{{")?;
            writeln!(w, "  \"version\": {},", f.version)?;
            writeln!(w, "  \"flow_units\": {},", json_str(f.flow_units()))?;
            writeln!(
                w,
                "  \"unit_system\": {},",
                json_str(if f.is_us() { "US" } else { "SI" })
            )?;
            writeln!(w, "  \"complete\": {},", f.complete)?;
            writeln!(
                w,
                "  \"error_code\": {},",
                f.error_code.map_or("null".to_string(), |e| e.to_string())
            )?;
            writeln!(w, "  \"report_step_seconds\": {},", f.report_step)?;
            writeln!(w, "  \"periods\": {},", f.n_periods)?;
            writeln!(w, "  \"output_start\": {},", json_str(&iso(f.start_date)))?;
            if f.n_periods > 0 {
                let (first, last) = (f.date(0)?, f.date(f.n_periods - 1)?);
                writeln!(
                    w,
                    "  \"first_report\": {},\n  \"last_report\": {},",
                    json_str(&iso(first)),
                    json_str(&iso(last))
                )?;
            }
            writeln!(w, "  \"subcatchments\": {},", list(&f.subcatchments))?;
            writeln!(w, "  \"nodes\": {},", list(&f.nodes))?;
            writeln!(
                w,
                "  \"node_types\": {},",
                list(
                    &(0..f.nodes.len())
                        .map(|i| f.node_type(i).unwrap_or("?").to_string())
                        .collect::<Vec<_>>()
                )
            )?;
            writeln!(w, "  \"links\": {},", list(&f.links))?;
            writeln!(
                w,
                "  \"link_types\": {},",
                list(
                    &(0..f.links.len())
                        .map(|i| f.link_type(i).unwrap_or("?").to_string())
                        .collect::<Vec<_>>()
                )
            )?;
            writeln!(w, "  \"pollutants\": {},", list(&f.pollutants))?;
            writeln!(w, "  \"variables\": {{")?;
            for (n, k) in Kind::ALL.iter().enumerate() {
                let v: Vec<String> = f
                    .variables(*k)
                    .iter()
                    .map(|v| {
                        format!(
                            "{{\"name\": {}, \"unit\": {}, \"code\": {}}}",
                            json_str(&v.name),
                            json_str(&v.unit),
                            v.code
                        )
                    })
                    .collect();
                writeln!(
                    w,
                    "    {}: [{}]{}",
                    json_str(k.as_str()),
                    v.join(", "),
                    if n < 3 { "," } else { "" }
                )?;
            }
            writeln!(w, "  }},")?;
            writeln!(w, "  \"notes\": {}", list(&f.notes))?;
            writeln!(w, "}}")?;
        }
        "list" => {
            for id in f.ids(kind(args.get(2).copied())?) {
                writeln!(w, "{id}")?;
            }
        }
        "vars" => {
            for (i, v) in f.variables(kind(args.get(2).copied())?).iter().enumerate() {
                writeln!(w, "{i},{},{}", v.name, v.unit)?;
            }
        }
        "series" => {
            let k = kind(args.get(2).copied())?;
            let (id, var) = if k == Kind::System {
                ("", args.get(3).ok_or("which variable?")?.as_str())
            } else {
                (
                    args.get(3).ok_or("which object?")?.as_str(),
                    args.get(4).ok_or("which variable?")?.as_str(),
                )
            };
            let values = f.series_by_name(k, id, var)?;
            let dates = f.dates()?;
            let v = f
                .var_index(k, var)
                .map(|i| f.variables(k)[i].clone())
                .ok_or("no such variable")?;
            writeln!(
                w,
                "time,{}",
                csv_field(&if v.unit.is_empty() {
                    v.name.clone()
                } else {
                    format!("{} ({})", v.name, v.unit)
                })
            )?;
            for (d, x) in dates.iter().zip(&values) {
                writeln!(w, "{},{x}", iso(*d))?;
            }
        }
        "stats" => {
            let k = kind(args.get(2).copied())?;
            let vars = f.variables(k);
            let stats = f.stats(k)?;
            let dates = f.dates()?;
            writeln!(
                w,
                "id,variable,unit,max,max_time,min,min_time,mean,last,periods_above_zero"
            )?;
            for (i, row) in stats.iter().enumerate() {
                let id = if k == Kind::System {
                    "system"
                } else {
                    f.ids(k)[i].as_str()
                };
                for (v, s) in vars.iter().zip(row) {
                    writeln!(
                        w,
                        "{},{},{},{},{},{},{},{},{},{}",
                        csv_field(id),
                        csv_field(&v.name),
                        v.unit,
                        s.max,
                        iso(dates[s.max_period]),
                        s.min,
                        iso(dates[s.min_period]),
                        s.mean,
                        s.last,
                        s.positive_periods
                    )?;
                }
            }
        }
        "dump" => {
            // kind,id,variable,v0,v1,... with each f32 in its shortest form that reads back to the same bits
            let dates = f.dates()?;
            write!(w, "date,,")?;
            for d in &dates {
                write!(w, ",{d:?}")?;
            }
            writeln!(w)?;
            let mut all: Vec<Vec<f32>> = Vec::with_capacity(f.n_periods);
            for p in 0..f.n_periods {
                all.push(f.period_values(p)?);
            }
            let mut col = 0;
            for k in Kind::ALL {
                let vars = f.variables(k);
                for i in 0..f.count(k) {
                    let id = if k == Kind::System {
                        ""
                    } else {
                        f.ids(k)[i].as_str()
                    };
                    for v in &vars {
                        write!(w, "{},{},{}", k.as_str(), csv_field(id), csv_field(&v.name))?;
                        for p in &all {
                            write!(w, ",{:?}", p[col])?;
                        }
                        writeln!(w)?;
                        col += 1;
                    }
                }
            }
        }
        other => return Err(format!("unknown command '{other}'\n\n{HELP}").into()),
    }
    w.flush()?;
    Ok(())
}
