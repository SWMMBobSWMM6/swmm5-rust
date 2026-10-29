//! # swmm5-c2rust
//!
//! The commit-zero baseline of swmm5-rust: EPA SWMM 5.2.4's 54 C files
//! translated statement by statement by c2rust 0.22.1. Every function is
//! `unsafe`, every global is a `static mut`, and modules find each other's
//! symbols through the linker exactly as the C files do. It exists to be the
//! oracle the safe engine is checked against, statement for statement.
#![allow(clippy::all)]
pub mod ccast;
pub mod engine;
