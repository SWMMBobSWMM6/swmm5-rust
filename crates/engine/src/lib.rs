//! # swmm5-rust
//!
//! EPA SWMM 5.2.4 in safe Rust. Every simulation is a value of type
//! [`Project`]; nothing is global, and the crate contains no `unsafe` code.
//! Function and field names are those of EPA's C, so the source can be
//! read side by side with it.
#![forbid(unsafe_code)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![allow(clippy::all)]
// C's own unused #defines, dead stores and unused locals are kept as written,
// so their warnings are off:
#![allow(dead_code, unused_assignments, unused_variables)]

pub mod ccast;
pub mod cio;
pub mod consts;
pub mod enums;
pub mod globals;
pub mod keywords;
pub mod macros;
pub mod objects;
pub mod prelude;
pub mod text;

pub mod climate;
pub mod controls;
pub mod culvert;
pub mod datetime;
pub mod dwflow;
pub mod dynwave;
pub mod error;
pub mod exfil;
pub mod findroot;
pub mod flowrout;
pub mod forcmain;
pub mod gage;
pub mod gwater;
pub mod hash;
pub mod hotstart;
pub mod iface;
pub mod infil;
pub mod inflow;
pub mod inlet;
pub mod input;
pub mod inputrpt;
pub mod kinwave;
pub mod landuse;
pub mod lid;
pub mod lidproc;
pub mod link;
pub mod massbal;
pub mod mathexpr;
pub mod node;
pub mod odesolve;
pub mod output;
pub mod project;
pub mod qualrout;
pub mod rain;
pub mod rdii;
pub mod report;
pub mod roadway;
pub mod routing;
pub mod runoff;
pub mod shape;
pub mod snow;
pub mod stats;
pub mod statsrpt;
pub mod street;
pub mod subcatch;
pub mod surfqual;
pub mod swmm5;
pub mod table;
pub mod toposort;
pub mod transect;
pub mod treatmnt;
pub mod xsect;

pub use globals::Project;
