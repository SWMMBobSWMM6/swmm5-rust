//! hash.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until hash.c is translated.

use crate::prelude::*;

/// One entry of a hash table bucket (C: `struct HTentry`; the bucket's
/// linked list becomes a vector in the same order).
#[derive(Clone, Default)]
pub struct HTentry {
    pub key: Vec<u8>,
    pub data: i32,
}
/// C: `typedef struct HTentry *HTtable;`, an array of HTMAXSIZE buckets.
pub type HTtable = Vec<Vec<HTentry>>;

#[derive(Default)]
pub struct HashState {}

/// Called by: project.
pub fn HTcreate() -> HTtable {
    todo!("HTcreate")
}

/// Called by: project.
pub fn HTfind(ht: &HTtable, key: &[u8]) -> i32 {
    todo!("HTfind")
}

/// Called by: project.
pub fn HTfindKey(ht: &HTtable, key: &[u8]) -> Option<Vec<u8>> {
    todo!("HTfindKey")
}

/// Called by: project.
pub fn HTfree(ht: &mut HTtable) {
    todo!("HTfree")
}

/// Called by: project.
pub fn HTinsert(ht: &mut HTtable, key: &[u8], data: i32) -> i32 {
    todo!("HTinsert")
}

/// Called by: (entry point).
pub fn hash(str: &[u8]) -> u32 {
    todo!("hash")
}

/// Called by: (entry point).
pub fn samestr(s1: &[u8], s2: &[u8]) -> i32 {
    todo!("samestr")
}
