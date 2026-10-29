//! hash.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   hash.c
//
//   Implementation of a simple Hash Table for string storage & retrieval
//   CASE INSENSITIVE
//
//   Written by L. Rossman
//   Last Updated on 6/19/03
//
//   The hash table data structure (HTable) is defined in "hash.h".
//   Interface Functions:
//      HTcreate() - creates a hash table
//      HTinsert() - inserts a string & its index value into a hash table
//      HTfind()   - retrieves the index value of a string from a table
//      HTfree()   - frees a hash table
//-----------------------------------------------------------------------------

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

/// The byte at index i of a C string held in a slice (NUL past its end).
#[inline]
fn at(s: &[u8], i: usize) -> u8 {
    if i < s.len() {
        s[i]
    } else {
        0
    }
}

/* Case-insensitive comparison of strings s1 and s2 */
pub fn samestr(s1: &[u8], s2: &[u8]) -> i32 {
    let mut i: usize = 0;
    // for (i=0; UCHAR(s1[i]) == UCHAR(s2[i]); i++)
    //   if (!s1[i+1] && !s2[i+1]) return(1);
    // NOTE: like C, two empty strings compare unequal unless the bytes after
    // the terminators happen to match; here past-the-end bytes read as NUL,
    // so "" vs "" reads s1[1] == s2[1] == 0 and returns 1. SWMM never hashes
    // an empty key.
    while UCHAR(at(s1, i)) == UCHAR(at(s2, i)) {
        if at(s1, i + 1) == 0 && at(s2, i + 1) == 0 {
            return 1;
        }
        i += 1;
    }
    0
} /*  End of samestr  */

/* Use Fletcher's checksum to compute 2-byte hash of string */
pub fn hash(str: &[u8]) -> u32 {
    let mut sum1: u32 = 0;
    let mut check1: u32;
    let mut sum2: u64 = 0;
    let mut k: usize = 0;
    while 0 != at(str, k) {
        // C: sum1 += UCHAR(*str); *str is a (signed) char promoted to int
        // and then converted to unsigned int.
        sum1 = sum1.wrapping_add(UCHAR(at(str, k)) as i8 as i32 as u32);
        k += 1;
        if 255 <= sum1 {
            sum1 = sum1.wrapping_sub(255);
        }
        sum2 = sum2.wrapping_add(sum1 as u64);
    }
    check1 = sum2 as u32;
    check1 %= 255;
    check1 = 255 - (sum1.wrapping_add(check1)) % 255;
    sum1 = 255 - (sum1.wrapping_add(check1)) % 255;
    ((check1 << 8) | sum1) % (HTMAXSIZE as u32)
}

pub fn HTcreate() -> HTtable {
    vec![Vec::new(); HTMAXSIZE as usize]
}

pub fn HTinsert(ht: &mut HTtable, key: &[u8], data: i32) -> i32 {
    let i = hash(key);
    if i >= HTMAXSIZE as u32 {
        return 0;
    }
    // entry->next = ht[i]; ht[i] = entry;  (prepend)
    ht[i as usize].insert(
        0,
        HTentry {
            key: cio::to_vec(key),
            data,
        },
    );
    1
}

pub fn HTfind(ht: &HTtable, key: &[u8]) -> i32 {
    let i = hash(key);
    if i >= HTMAXSIZE as u32 {
        return NOTFOUND;
    }
    for entry in ht[i as usize].iter() {
        if samestr(&entry.key, key) != 0 {
            return entry.data;
        }
    }
    NOTFOUND
}

pub fn HTfindKey(ht: &HTtable, key: &[u8]) -> Option<Vec<u8>> {
    let i = hash(key);
    if i >= HTMAXSIZE as u32 {
        return None;
    }
    for entry in ht[i as usize].iter() {
        if samestr(&entry.key, key) != 0 {
            return Some(entry.key.clone());
        }
    }
    None
}

pub fn HTfree(ht: &mut HTtable) {
    for bucket in ht.iter_mut() {
        bucket.clear();
    }
    ht.clear();
}
