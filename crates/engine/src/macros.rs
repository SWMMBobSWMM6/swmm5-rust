//! The macros of macros.h as functions, with C's exact semantics.
//!
//! A C macro evaluates its arguments where they appear: `MAX(x, f())` calls
//! `f` twice when `f()` is the larger. These functions evaluate each
//! argument once. Where an argument has a side effect, the translation must
//! say so at the call site (none of SWMM's uses depends on it).
//!
//! `MIN` and `MAX` use `<=` and `>=` exactly as the macros do, so a NaN
//! argument picks the same side it does in C. `ABS(-0.0)` is `-0.0`, as in
//! C (the macro is a comparison, not `fabs`).

/// `MIN(x,y)  (((x)<=(y)) ? (x) : (y))`
#[inline(always)]
pub fn MIN<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

/// `MAX(x,y)  (((x)>=(y)) ? (x) : (y))`
#[inline(always)]
pub fn MAX<T: PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x
    } else {
        y
    }
}

/// `ABS(x)  (((x)<0) ? -(x) : (x))`
#[inline(always)]
pub fn ABS<T: PartialOrd + std::ops::Neg<Output = T> + Default + Copy>(x: T) -> T {
    if x < T::default() {
        -x
    } else {
        x
    }
}

/// `SQR(x)  ((x)*(x))`
#[inline(always)]
pub fn SQR(x: f64) -> f64 {
    x * x
}

/// `SGN(x)  (((x)<0) ? (-1) : (1))`. C yields an int; every SWMM use
/// multiplies a double by it, where int -1/1 and double -1.0/1.0 are the
/// same, so this returns f64. Use `SGNi` for an integer context.
#[inline(always)]
pub fn SGN(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}

/// `SGN` in an integer context.
#[inline(always)]
pub fn SGNi(x: f64) -> i32 {
    if x < 0.0 {
        -1
    } else {
        1
    }
}

/// `SIGN(x,y)  ((y) >= 0.0 ? fabs(x) : -fabs(x))`
#[inline(always)]
pub fn SIGN(x: f64, y: f64) -> f64 {
    if y >= 0.0 {
        x.abs()
    } else {
        -x.abs()
    }
}

/// `LOG10(x)  ((x) > 0.0 ? log10((x)) : (x))`
#[inline(always)]
pub fn LOG10(x: f64) -> f64 {
    if x > 0.0 {
        x.log10()
    } else {
        x
    }
}

/// `UCHAR(x)  (((x) >= 'a' && (x) <= 'z') ? ((x)&~32) : (x))`
#[inline(always)]
pub fn UCHAR(x: u8) -> u8 {
    if (b'a'..=b'z').contains(&x) {
        x & !32
    } else {
        x
    }
}

/// `MOD(x,y)  ((x)%(y))` on ints (C and Rust both truncate toward zero).
#[inline(always)]
pub fn MOD(x: i32, y: i32) -> i32 {
    x % y
}
