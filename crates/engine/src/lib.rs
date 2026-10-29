//! # swmm5-rust
//!
//! EPA SWMM 5.2.4 in safe Rust. Every simulation is a value of type
//! [`Project`]; nothing is global, and the crate contains no `unsafe` code.
#![forbid(unsafe_code)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

pub mod ccast;
pub mod cio;
