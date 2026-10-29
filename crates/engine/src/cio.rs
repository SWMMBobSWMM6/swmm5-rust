//! The C library, as far as EPA SWMM uses it, in safe Rust.
//!
//! EPA's engine writes its report with `fprintf`, reads its input with
//! `fgets`, `sscanf`, `strtod` and `atoi`, and keeps names in NUL-terminated
//! `char` arrays. Byte-identical output needs each of those to behave exactly
//! as glibc does for the inputs SWMM gives them, so this module implements
//! them from their C definitions rather than mapping them onto Rust's own
//! formatting and parsing, whose rules differ (`{:e}` writes `1e5`, C writes
//! `1.000000e+05`; Rust's `as` saturates; `str::parse` rejects `1.5abc`).
//!
//! Strings here are bytes (`&[u8]`), never `str`: a model's names and comments
//! are whatever bytes its author typed, and C never checked them for UTF-8.
//! A slice may carry a NUL terminator; every function stops at the first NUL,
//! as C would.

use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};

// ---------------------------------------------------------------------------
//  C strings in byte buffers
// ---------------------------------------------------------------------------

/// Length up to the first NUL (C `strlen`), or the whole slice if none.
#[inline]
pub fn strlen(s: &[u8]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

/// The bytes of a C string, without its terminator.
#[inline]
pub fn cstr(s: &[u8]) -> &[u8] {
    &s[..strlen(s)]
}

/// Copy a C string into a fixed buffer, as `strcpy` would if the buffer
/// were big enough; truncates (keeping the terminator) if it is not.
pub fn strcpy(dst: &mut [u8], src: &[u8]) {
    let src = cstr(src);
    let n = src.len().min(dst.len().saturating_sub(1));
    dst[..n].copy_from_slice(&src[..n]);
    if n < dst.len() {
        dst[n] = 0;
    }
}

/// `strncpy(dst, src, n)` followed by `dst[n] = 0`, which is SWMM's
/// `sstrncpy`: copy at most n bytes and always terminate.
pub fn sstrncpy(dst: &mut [u8], src: &[u8], n: usize) {
    let src = cstr(src);
    let m = src.len().min(n).min(dst.len().saturating_sub(1));
    dst[..m].copy_from_slice(&src[..m]);
    dst[m] = 0;
}

/// `strcat` into a fixed buffer (truncating rather than overrunning).
pub fn strcat(dst: &mut [u8], src: &[u8]) {
    let start = strlen(dst);
    let src = cstr(src);
    let room = dst.len().saturating_sub(start + 1);
    let n = src.len().min(room);
    dst[start..start + n].copy_from_slice(&src[..n]);
    if start + n < dst.len() {
        dst[start + n] = 0;
    }
}

/// Owned copy of a C string's bytes.
#[inline]
pub fn to_vec(s: &[u8]) -> Vec<u8> {
    cstr(s).to_vec()
}

/// C `strcmp` sign.
pub fn strcmp(a: &[u8], b: &[u8]) -> i32 {
    let (a, b) = (cstr(a), cstr(b));
    for i in 0..a.len().min(b.len()) {
        if a[i] != b[i] {
            return a[i] as i32 - b[i] as i32;
        }
    }
    a.len() as i32 - b.len() as i32
}

/// C `strncmp` sign.
pub fn strncmp(a: &[u8], b: &[u8], n: usize) -> i32 {
    let (a, b) = (cstr(a), cstr(b));
    for i in 0..n {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        if x != y {
            return x as i32 - y as i32;
        }
        if x == 0 {
            return 0;
        }
    }
    0
}

/// C `toupper` in the "C" locale.
#[inline]
pub fn toupper(c: u8) -> u8 {
    c.to_ascii_uppercase()
}

/// C `isspace` in the "C" locale: space, \t, \n, \v, \f, \r.
#[inline]
pub fn isspace(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

#[inline]
pub fn isdigit(c: u8) -> bool {
    c.is_ascii_digit()
}

/// C `strstr`: index of the first occurrence of `needle` in `hay`.
pub fn strstr(hay: &[u8], needle: &[u8]) -> Option<usize> {
    let (h, n) = (cstr(hay), cstr(needle));
    if n.is_empty() {
        return Some(0);
    }
    h.windows(n.len()).position(|w| w == n)
}

/// C `strchr`: index of the first `c` (NUL finds the terminator).
pub fn strchr(s: &[u8], c: u8) -> Option<usize> {
    let s2 = cstr(s);
    if c == 0 {
        return Some(s2.len());
    }
    s2.iter().position(|&x| x == c)
}

/// C `strrchr`.
pub fn strrchr(s: &[u8], c: u8) -> Option<usize> {
    let s2 = cstr(s);
    if c == 0 {
        return Some(s2.len());
    }
    s2.iter().rposition(|&x| x == c)
}

// ---------------------------------------------------------------------------
//  Number parsing: strtod, atof, strtol, atoi
// ---------------------------------------------------------------------------

/// glibc `strtod`: returns the value and the number of bytes consumed
/// (0 if no conversion, as when `endptr == s`).
pub fn strtod(s: &[u8]) -> (f64, usize) {
    let s = cstr(s);
    let mut i = 0;
    while i < s.len() && isspace(s[i]) {
        i += 1;
    }
    let start = i;
    let mut neg = false;
    if i < s.len() && (s[i] == b'+' || s[i] == b'-') {
        neg = s[i] == b'-';
        i += 1;
    }
    let lower = |k: usize| s.get(k).map(|c| c.to_ascii_lowercase()).unwrap_or(0);
    // inf / infinity / nan / nan(chars)
    if lower(i) == b'i' && lower(i + 1) == b'n' && lower(i + 2) == b'f' {
        let mut e = i + 3;
        if (0..5).all(|k| lower(e + k) == b"inity"[k]) {
            e += 5;
        }
        return (if neg { f64::NEG_INFINITY } else { f64::INFINITY }, e);
    }
    if lower(i) == b'n' && lower(i + 1) == b'a' && lower(i + 2) == b'n' {
        let mut e = i + 3;
        if s.get(e) == Some(&b'(') {
            let mut k = e + 1;
            while k < s.len() && (s[k].is_ascii_alphanumeric() || s[k] == b'_') {
                k += 1;
            }
            if s.get(k) == Some(&b')') {
                e = k + 1;
            }
        }
        // glibc's NAN is the positive quiet NaN; "-nan" flips the sign bit
        let v = f64::from_bits(0x7ff8_0000_0000_0000);
        return (if neg { -v } else { v }, e);
    }
    // hexadecimal floating point (0x1.8p3); rare in a model, kept for fidelity
    if s.get(i) == Some(&b'0') && lower(i + 1) == b'x' {
        let mut k = i + 2;
        let mut mant: f64 = 0.0;
        let mut any = false;
        let mut exp2: i32 = 0;
        while k < s.len() && s[k].is_ascii_hexdigit() {
            mant = mant * 16.0 + (s[k] as char).to_digit(16).unwrap() as f64;
            any = true;
            k += 1;
        }
        if s.get(k) == Some(&b'.') {
            k += 1;
            while k < s.len() && s[k].is_ascii_hexdigit() {
                mant = mant * 16.0 + (s[k] as char).to_digit(16).unwrap() as f64;
                exp2 -= 4;
                any = true;
                k += 1;
            }
        }
        if any {
            if lower(k) == b'p' {
                let mut j = k + 1;
                let mut eneg = false;
                if j < s.len() && (s[j] == b'+' || s[j] == b'-') {
                    eneg = s[j] == b'-';
                    j += 1;
                }
                if j < s.len() && isdigit(s[j]) {
                    let mut e: i32 = 0;
                    while j < s.len() && isdigit(s[j]) {
                        e = e.saturating_mul(10).saturating_add((s[j] - b'0') as i32);
                        j += 1;
                    }
                    exp2 = exp2.saturating_add(if eneg { -e } else { e });
                    k = j;
                }
            }
            let v = mant * 2f64.powi(exp2);
            return (if neg { -v } else { v }, k);
        }
        // "0x" with no hex digits: the "0" alone is the number
        return (if neg { -0.0 } else { 0.0 }, i + 1);
    }
    // decimal
    let mant_start = i;
    let mut digits = 0;
    while i < s.len() && isdigit(s[i]) {
        i += 1;
        digits += 1;
    }
    if i < s.len() && s[i] == b'.' {
        i += 1;
        while i < s.len() && isdigit(s[i]) {
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return (0.0, 0);
    }
    let mut end = i;
    if i < s.len() && (s[i] == b'e' || s[i] == b'E') {
        let mut j = i + 1;
        if j < s.len() && (s[j] == b'+' || s[j] == b'-') {
            j += 1;
        }
        if j < s.len() && isdigit(s[j]) {
            while j < s.len() && isdigit(s[j]) {
                j += 1;
            }
            end = j;
        }
    }
    // Rust's parser is correctly rounded, as glibc's strtod is.
    let text = std::str::from_utf8(&s[mant_start..end]).unwrap_or("0");
    let v: f64 = parse_decimal(text);
    let _ = start;
    (if neg { -v } else { v }, end)
}

fn parse_decimal(text: &str) -> f64 {
    // Rust rejects a lone trailing '.' after digits? No: "1." parses. It does
    // reject an exponent with too many digits for i64 only by saturating, and
    // it accepts what the scanner above admits.
    match text.parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            // exponent digit strings longer than Rust accepts: saturate as strtod does
            let (m, e) = match text.find(['e', 'E']) {
                Some(k) => (&text[..k], &text[k + 1..]),
                None => (text, "0"),
            };
            let mv: f64 = m.parse().unwrap_or(0.0);
            if mv == 0.0 {
                0.0
            } else if e.starts_with('-') {
                0.0
            } else {
                f64::INFINITY
            }
        }
    }
}

/// C `atof`.
#[inline]
pub fn atof(s: &[u8]) -> f64 {
    strtod(s).0
}

/// glibc `strtol` with base 10 on a 64-bit `long`: value (saturated on
/// overflow) and bytes consumed.
pub fn strtol(s: &[u8]) -> (i64, usize) {
    let s = cstr(s);
    let mut i = 0;
    while i < s.len() && isspace(s[i]) {
        i += 1;
    }
    let mut neg = false;
    if i < s.len() && (s[i] == b'+' || s[i] == b'-') {
        neg = s[i] == b'-';
        i += 1;
    }
    let d0 = i;
    let mut v: i128 = 0;
    while i < s.len() && isdigit(s[i]) {
        if v < (1i128 << 70) {
            v = v * 10 + (s[i] - b'0') as i128;
        }
        i += 1;
    }
    if i == d0 {
        return (0, 0);
    }
    let v = if neg { -v } else { v };
    (v.clamp(i64::MIN as i128, i64::MAX as i128) as i64, i)
}

/// C `atoi`: glibc computes `(int) strtol(s, NULL, 10)`.
#[inline]
pub fn atoi(s: &[u8]) -> i32 {
    strtol(s).0 as i32
}

// ---------------------------------------------------------------------------
//  printf
// ---------------------------------------------------------------------------

/// One argument of a variadic C call, after C's default promotions
/// (float becomes double, char and short become int).
#[derive(Clone, Copy, Debug)]
pub enum Arg<'a> {
    I(i64),
    U(u64),
    F(f64),
    S(&'a [u8]),
}

/// Conversion of a Rust value into the argument C would have received.
pub trait ToArg {
    fn to_arg(&self) -> Arg<'_>;
}
impl ToArg for i32 {
    fn to_arg(&self) -> Arg<'_> {
        Arg::I(*self as i64)
    }
}
impl ToArg for i64 {
    fn to_arg(&self) -> Arg<'_> {
        Arg::I(*self)
    }
}
impl ToArg for u32 {
    fn to_arg(&self) -> Arg<'_> {
        Arg::U(*self as u64)
    }
}
impl ToArg for u64 {
    fn to_arg(&self) -> Arg<'_> {
        Arg::U(*self)
    }
}
impl ToArg for usize {
    fn to_arg(&self) -> Arg<'_> {
        Arg::U(*self as u64)
    }
}
/// A C `char` passed to `%c` (or `%d`).
impl ToArg for u8 {
    fn to_arg(&self) -> Arg<'_> {
        Arg::I(*self as i64)
    }
}
impl ToArg for f64 {
    fn to_arg(&self) -> Arg<'_> {
        Arg::F(*self)
    }
}
impl ToArg for f32 {
    fn to_arg(&self) -> Arg<'_> {
        Arg::F(*self as f64)
    }
}
impl ToArg for [u8] {
    fn to_arg(&self) -> Arg<'_> {
        Arg::S(self)
    }
}
impl ToArg for Vec<u8> {
    fn to_arg(&self) -> Arg<'_> {
        Arg::S(self.as_slice())
    }
}
impl<const N: usize> ToArg for [u8; N] {
    fn to_arg(&self) -> Arg<'_> {
        Arg::S(&self[..])
    }
}
impl ToArg for str {
    fn to_arg(&self) -> Arg<'_> {
        Arg::S(self.as_bytes())
    }
}
impl<T: ToArg + ?Sized> ToArg for &T {
    fn to_arg(&self) -> Arg<'_> {
        (**self).to_arg()
    }
}
impl<T: ToArg + ?Sized> ToArg for &mut T {
    fn to_arg(&self) -> Arg<'_> {
        (**self).to_arg()
    }
}

/// `sprintf` into a new byte vector: `cformat!(b"%10.3f", x)`.
#[macro_export]
macro_rules! cformat {
    ($fmt:expr $(, $a:expr)* $(,)?) => {
        $crate::cio::format($fmt, &[$($crate::cio::ToArg::to_arg(&$a)),*])
    };
}

/// `fprintf` to anything with a `write_bytes` sink: `cprintf!(f, b"%d", n)`.
#[macro_export]
macro_rules! cprintf {
    ($f:expr, $fmt:expr $(, $a:expr)* $(,)?) => {
        $f.write_bytes(&$crate::cio::format($fmt, &[$($crate::cio::ToArg::to_arg(&$a)),*]))
    };
}

struct Spec {
    left: bool,
    plus: bool,
    space: bool,
    alt: bool,
    zero: bool,
    width: usize,
    prec: Option<usize>,
}

/// C `sprintf`. Panics on a conversion whose argument type does not match,
/// which in C would be undefined behaviour: better a loud failure in the
/// gate than a quietly different report.
pub fn format(fmt: &[u8], args: &[Arg]) -> Vec<u8> {
    let fmt = cstr(fmt);
    let mut out = Vec::with_capacity(fmt.len() + 16);
    let mut ai = 0usize;
    let mut i = 0usize;
    let next = |ai: &mut usize| -> Arg {
        let a = *args.get(*ai).unwrap_or_else(|| panic!("printf: missing argument {} for {:?}", *ai, String::from_utf8_lossy(fmt)));
        *ai += 1;
        a
    };
    while i < fmt.len() {
        let c = fmt[i];
        if c != b'%' {
            out.push(c);
            i += 1;
            continue;
        }
        i += 1;
        if i >= fmt.len() {
            break;
        }
        if fmt[i] == b'%' {
            out.push(b'%');
            i += 1;
            continue;
        }
        let mut sp = Spec { left: false, plus: false, space: false, alt: false, zero: false, width: 0, prec: None };
        while i < fmt.len() {
            match fmt[i] {
                b'-' => sp.left = true,
                b'+' => sp.plus = true,
                b' ' => sp.space = true,
                b'#' => sp.alt = true,
                b'0' => sp.zero = true,
                _ => break,
            }
            i += 1;
        }
        if i < fmt.len() && fmt[i] == b'*' {
            let w = match next(&mut ai) {
                Arg::I(v) => v,
                Arg::U(v) => v as i64,
                a => panic!("printf: * width needs int, got {:?}", a),
            };
            if w < 0 {
                sp.left = true;
                sp.width = (-w) as usize;
            } else {
                sp.width = w as usize;
            }
            i += 1;
        } else {
            while i < fmt.len() && isdigit(fmt[i]) {
                sp.width = sp.width * 10 + (fmt[i] - b'0') as usize;
                i += 1;
            }
        }
        if i < fmt.len() && fmt[i] == b'.' {
            i += 1;
            if i < fmt.len() && fmt[i] == b'*' {
                let p = match next(&mut ai) {
                    Arg::I(v) => v,
                    Arg::U(v) => v as i64,
                    a => panic!("printf: * precision needs int, got {:?}", a),
                };
                sp.prec = if p < 0 { None } else { Some(p as usize) };
                i += 1;
            } else {
                let mut p = 0usize;
                while i < fmt.len() && isdigit(fmt[i]) {
                    p = p * 10 + (fmt[i] - b'0') as usize;
                    i += 1;
                }
                sp.prec = Some(p);
            }
        }
        // length modifiers: without `l`, C reads an int (32 bits)
        let mut long = false;
        while i < fmt.len() && matches!(fmt[i], b'h' | b'l' | b'L' | b'q' | b'j' | b'z' | b't') {
            if matches!(fmt[i], b'l' | b'L' | b'q' | b'j' | b'z' | b't') {
                long = true;
            }
            i += 1;
        }
        if i >= fmt.len() {
            break;
        }
        let conv = fmt[i];
        i += 1;
        match conv {
            b'd' | b'i' => {
                let v = match next(&mut ai) {
                    Arg::I(v) => v as i128,
                    Arg::U(v) => v as i128,
                    a => panic!("printf: %d needs an integer, got {:?} in {:?}", a, String::from_utf8_lossy(fmt)),
                };
                let v = if long { v as i64 as i128 } else { v as i32 as i128 };
                emit_int(&mut out, &sp, v, 10, false);
            }
            b'u' | b'x' | b'X' | b'o' => {
                let v = match next(&mut ai) {
                    Arg::I(v) => v as u64 as i128,
                    Arg::U(v) => v as i128,
                    a => panic!("printf: %u needs an integer, got {:?}", a),
                };
                let v = if long { v as u64 as i128 } else { v as u32 as i128 };
                let base = match conv {
                    b'o' => 8,
                    b'u' => 10,
                    _ => 16,
                };
                emit_int(&mut out, &sp, v, base, conv == b'X');
            }
            b'c' => {
                let ch = match next(&mut ai) {
                    Arg::I(v) => v as u8,
                    Arg::U(v) => v as u8,
                    a => panic!("printf: %c needs a char, got {:?}", a),
                };
                pad(&mut out, &sp, &[], &[ch], false);
            }
            b's' => {
                let s = match next(&mut ai) {
                    Arg::S(s) => cstr(s),
                    a => panic!("printf: %s needs a string, got {:?} in {:?}", a, String::from_utf8_lossy(fmt)),
                };
                let s = match sp.prec {
                    Some(p) if p < s.len() => &s[..p],
                    _ => s,
                };
                pad(&mut out, &sp, &[], s, false);
            }
            b'f' | b'F' | b'e' | b'E' | b'g' | b'G' => {
                let x = match next(&mut ai) {
                    Arg::F(x) => x,
                    a => panic!("printf: %{} needs a double, got {:?} in {:?}", conv as char, a, String::from_utf8_lossy(fmt)),
                };
                emit_float(&mut out, &sp, x, conv);
            }
            b'p' => {
                let _ = next(&mut ai);
                out.extend_from_slice(b"(nil)");
            }
            _ => {
                // unknown conversion: glibc prints it literally
                out.push(b'%');
                out.push(conv);
            }
        }
    }
    out
}

fn pad(out: &mut Vec<u8>, sp: &Spec, sign: &[u8], body: &[u8], zero_ok: bool) {
    let len = sign.len() + body.len();
    let fill = sp.width.saturating_sub(len);
    if sp.left {
        out.extend_from_slice(sign);
        out.extend_from_slice(body);
        out.extend(std::iter::repeat_n(b' ', fill));
    } else if sp.zero && zero_ok {
        out.extend_from_slice(sign);
        out.extend(std::iter::repeat_n(b'0', fill));
        out.extend_from_slice(body);
    } else {
        out.extend(std::iter::repeat_n(b' ', fill));
        out.extend_from_slice(sign);
        out.extend_from_slice(body);
    }
}

fn emit_int(out: &mut Vec<u8>, sp: &Spec, v: i128, base: u32, upper: bool) {
    let neg = v < 0;
    let mag = v.unsigned_abs();
    let mut digits: Vec<u8> = if mag == 0 && sp.prec == Some(0) {
        Vec::new()
    } else {
        let s = match base {
            8 => format!("{:o}", mag),
            16 if upper => format!("{:X}", mag),
            16 => format!("{:x}", mag),
            _ => format!("{}", mag),
        };
        s.into_bytes()
    };
    if let Some(p) = sp.prec {
        while digits.len() < p {
            digits.insert(0, b'0');
        }
    }
    let sign: &[u8] = if neg {
        b"-"
    } else if sp.plus && base == 10 {
        b"+"
    } else if sp.space && base == 10 {
        b" "
    } else {
        b""
    };
    // the 0 flag is ignored when a precision is given
    pad(out, sp, sign, &digits, sp.prec.is_none());
}

fn emit_float(out: &mut Vec<u8>, sp: &Spec, x: f64, conv: u8) {
    let upper = conv.is_ascii_uppercase();
    let neg = x.is_sign_negative();
    let sign: &[u8] = if neg {
        b"-"
    } else if sp.plus {
        b"+"
    } else if sp.space {
        b" "
    } else {
        b""
    };
    if !x.is_finite() {
        let body: &[u8] = match (x.is_nan(), upper) {
            (true, false) => b"nan",
            (true, true) => b"NAN",
            (false, false) => b"inf",
            (false, true) => b"INF",
        };
        pad(out, sp, sign, body, false);
        return;
    }
    let a = x.abs();
    let prec = sp.prec.unwrap_or(6);
    let body: Vec<u8> = match conv {
        b'f' | b'F' => fixed(a, prec, sp.alt),
        b'e' | b'E' => expo(a, prec, sp.alt, upper),
        _ => general(a, prec, sp.alt, upper),
    };
    pad(out, sp, sign, &body, true);
}

/// `%.{prec}f` of a non-negative finite value. Rust's fixed-precision
/// formatting is exact and rounds ties to even, as glibc does.
fn fixed(a: f64, prec: usize, alt: bool) -> Vec<u8> {
    let mut s = format!("{:.*}", prec, a).into_bytes();
    if alt && prec == 0 {
        s.push(b'.');
    }
    s
}

/// Mantissa digits and decimal exponent of `%.{prec}e`.
fn expo_parts(a: f64, prec: usize) -> (Vec<u8>, i32) {
    let s = format!("{:.*e}", prec, a);
    let k = s.find('e').unwrap();
    let e: i32 = s[k + 1..].parse().unwrap();
    (s[..k].as_bytes().to_vec(), e)
}

fn exp_suffix(e: i32, upper: bool) -> Vec<u8> {
    let mut v = vec![if upper { b'E' } else { b'e' }];
    v.push(if e < 0 { b'-' } else { b'+' });
    let m = e.unsigned_abs();
    if m < 10 {
        v.push(b'0');
    }
    v.extend_from_slice(m.to_string().as_bytes());
    v
}

fn expo(a: f64, prec: usize, alt: bool, upper: bool) -> Vec<u8> {
    let (mut m, e) = expo_parts(a, prec);
    if alt && prec == 0 {
        m.push(b'.');
    }
    m.extend(exp_suffix(e, upper));
    m
}

fn general(a: f64, prec: usize, alt: bool, upper: bool) -> Vec<u8> {
    let p = if prec == 0 { 1 } else { prec };
    let x_exp = if a == 0.0 { 0 } else { expo_parts(a, p - 1).1 };
    let mut body;
    if (x_exp as i64) < p as i64 && x_exp >= -4 {
        body = fixed(a, (p as i64 - 1 - x_exp as i64) as usize, alt);
        if !alt {
            strip_zeros(&mut body);
        }
    } else {
        let (mut m, e) = expo_parts(a, p - 1);
        if !alt {
            strip_zeros(&mut m);
        } else if p - 1 == 0 {
            m.push(b'.');
        }
        m.extend(exp_suffix(e, upper));
        body = m;
    }
    body
}

fn strip_zeros(s: &mut Vec<u8>) {
    if s.contains(&b'.') {
        while s.last() == Some(&b'0') {
            s.pop();
        }
        if s.last() == Some(&b'.') {
            s.pop();
        }
    }
}

// ---------------------------------------------------------------------------
//  sscanf
// ---------------------------------------------------------------------------

/// A destination of one `sscanf` conversion.
pub enum Scan<'a> {
    /// `%d`, `%i`
    Int(&'a mut i32),
    /// `%ld`
    Long(&'a mut i64),
    /// `%f`
    Float(&'a mut f32),
    /// `%lf`
    Double(&'a mut f64),
    /// `%c`
    Char(&'a mut u8),
    /// `%s`: the C buffer (bytes plus terminator) is replaced by the token
    Str(&'a mut Vec<u8>),
}

/// glibc `sscanf` for the conversions SWMM uses (`%d %ld %f %lf %c %s`,
/// field widths, literal characters, white space). Returns the number of
/// conversions assigned, or -1 (EOF) if the input ran out before the
/// first conversion.
pub fn sscanf(input: &[u8], fmt: &[u8], outs: &mut [Scan]) -> i32 {
    let s = cstr(input);
    let f = cstr(fmt);
    let (mut si, mut fi, mut oi) = (0usize, 0usize, 0usize);
    let mut assigned = 0i32;
    let eof_ret = |assigned: i32| if assigned == 0 { -1 } else { assigned };
    while fi < f.len() {
        let c = f[fi];
        if isspace(c) {
            while fi < f.len() && isspace(f[fi]) {
                fi += 1;
            }
            while si < s.len() && isspace(s[si]) {
                si += 1;
            }
            continue;
        }
        if c != b'%' {
            if si >= s.len() {
                return eof_ret(assigned);
            }
            if s[si] != c {
                return assigned;
            }
            si += 1;
            fi += 1;
            continue;
        }
        fi += 1;
        if fi < f.len() && f[fi] == b'%' {
            while si < s.len() && isspace(s[si]) {
                si += 1;
            }
            if si >= s.len() {
                return eof_ret(assigned);
            }
            if s[si] != b'%' {
                return assigned;
            }
            si += 1;
            fi += 1;
            continue;
        }
        let mut width = 0usize;
        while fi < f.len() && isdigit(f[fi]) {
            width = width * 10 + (f[fi] - b'0') as usize;
            fi += 1;
        }
        let mut longs = 0;
        while fi < f.len() && matches!(f[fi], b'l' | b'h' | b'L') {
            if f[fi] == b'l' {
                longs += 1;
            }
            fi += 1;
        }
        if fi >= f.len() {
            break;
        }
        let conv = f[fi];
        fi += 1;
        if conv != b'c' {
            while si < s.len() && isspace(s[si]) {
                si += 1;
            }
        }
        if si >= s.len() {
            return eof_ret(assigned);
        }
        let lim = if width == 0 { s.len() } else { (si + width).min(s.len()) };
        let field = &s[si..lim];
        match conv {
            b'd' | b'i' => {
                let (v, n) = strtol(field);
                if n == 0 {
                    return assigned;
                }
                si += n;
                match outs.get_mut(oi) {
                    Some(Scan::Int(d)) => **d = v as i32,
                    Some(Scan::Long(d)) => **d = v,
                    _ => panic!("sscanf: %d destination mismatch in {:?}", String::from_utf8_lossy(f)),
                }
                let _ = longs;
            }
            b'f' | b'e' | b'g' | b'E' | b'G' => {
                let (n, ok) = scan_float_len(field);
                if !ok {
                    return assigned;
                }
                let text = &field[..n];
                si += n;
                match outs.get_mut(oi) {
                    Some(Scan::Double(d)) => **d = strtod(text).0,
                    Some(Scan::Float(d)) => **d = strtof(text),
                    _ => panic!("sscanf: %f destination mismatch in {:?}", String::from_utf8_lossy(f)),
                }
            }
            b'c' => {
                match outs.get_mut(oi) {
                    Some(Scan::Char(d)) => **d = s[si],
                    _ => panic!("sscanf: %c destination mismatch"),
                }
                si += 1;
            }
            b's' => {
                let mut k = si;
                while k < lim && !isspace(s[k]) {
                    k += 1;
                }
                match outs.get_mut(oi) {
                    Some(Scan::Str(d)) => {
                        d.clear();
                        d.extend_from_slice(&s[si..k]);
                    }
                    _ => panic!("sscanf: %s destination mismatch"),
                }
                si = k;
            }
            _ => panic!("sscanf: unsupported conversion %{}", conv as char),
        }
        oi += 1;
        assigned += 1;
    }
    assigned
}

/// Length of the longest prefix glibc's scanf would accept as a float, and
/// whether it forms a number. (scanf, unlike strtod, consumes what it
/// reads even if the number then turns out malformed; for the formats SWMM
/// uses the two agree.)
fn scan_float_len(field: &[u8]) -> (usize, bool) {
    let (_, n) = strtod(field);
    (n, n > 0)
}

/// C `strtof`, correctly rounded straight to single precision.
pub fn strtof(s: &[u8]) -> f32 {
    let (_, n) = strtod(s);
    if n == 0 {
        return 0.0;
    }
    let s = cstr(s);
    let mut i = 0;
    while i < n && isspace(s[i]) {
        i += 1;
    }
    let text = std::str::from_utf8(&s[i..n]).unwrap_or("0");
    match text.parse::<f32>() {
        Ok(v) => v,
        Err(_) => strtod(s).0 as f32,
    }
}

// ---------------------------------------------------------------------------
//  FILE
// ---------------------------------------------------------------------------

/// A C `FILE*` over a `std::fs::File`: one buffer, a position, an
/// end-of-file flag, and the stdio rules SWMM relies on (`fgets` keeps the
/// newline; `feof` turns true only after a read hits the end; `rewind`
/// clears it; a write is buffered until a seek, a read or the close).
pub struct CFile {
    f: fs::File,
    rbuf: Vec<u8>,
    rpos: usize,
    wbuf: Vec<u8>,
    eof: bool,
    err: bool,
}

const BUFSZ: usize = 1 << 16;

impl CFile {
    /// `fopen`. Modes as SWMM uses them: "r"/"rt"/"rb", "w"/"wt"/"wb",
    /// "r+b", "w+b". Text and binary are the same on POSIX.
    pub fn open(path: &[u8], mode: &str) -> Option<CFile> {
        let p = path_of(path)?;
        let mut o = fs::OpenOptions::new();
        let m = mode.replace(['t', 'b'], "");
        match m.as_str() {
            "r" => o.read(true),
            "w" => o.write(true).create(true).truncate(true),
            "a" => o.append(true).create(true),
            "r+" => o.read(true).write(true),
            "w+" => o.read(true).write(true).create(true).truncate(true),
            "a+" => o.read(true).append(true).create(true),
            _ => return None,
        };
        let f = o.open(p).ok()?;
        if m == "r" || m == "r+" {
            // fopen succeeds on a directory for reading in glibc; reading it then fails.
            // SWMM never opens one deliberately; treat it as not found.
            if f.metadata().map(|md| md.is_dir()).unwrap_or(false) {
                return None;
            }
        }
        Some(CFile { f, rbuf: Vec::new(), rpos: 0, wbuf: Vec::new(), eof: false, err: false })
    }

    fn flush_write(&mut self) {
        if !self.wbuf.is_empty() {
            if self.f.write_all(&self.wbuf).is_err() {
                self.err = true;
            }
            self.wbuf.clear();
        }
    }

    /// Discard read-ahead, moving the OS position back to the logical one.
    fn drop_read(&mut self) {
        let ahead = self.rbuf.len() - self.rpos;
        if ahead > 0 {
            let _ = self.f.seek(SeekFrom::Current(-(ahead as i64)));
        }
        self.rbuf.clear();
        self.rpos = 0;
    }

    fn fill(&mut self) -> bool {
        self.flush_write();
        if self.rpos < self.rbuf.len() {
            return true;
        }
        self.rbuf.resize(BUFSZ, 0);
        self.rpos = 0;
        match self.f.read(&mut self.rbuf) {
            Ok(n) => {
                self.rbuf.truncate(n);
                n > 0
            }
            Err(_) => {
                self.rbuf.clear();
                self.err = true;
                false
            }
        }
    }

    /// `fgets(buf, n, f)`: reads at most n-1 bytes, stopping after a newline.
    /// Returns the bytes read (with the newline, without a terminator), or
    /// None at end of file with nothing read.
    pub fn fgets(&mut self, n: usize) -> Option<Vec<u8>> {
        let mut line = Vec::new();
        if n <= 1 {
            return if n == 1 { Some(line) } else { None };
        }
        while line.len() < n - 1 {
            if self.rpos >= self.rbuf.len() && !self.fill() {
                self.eof = true;
                break;
            }
            let c = self.rbuf[self.rpos];
            self.rpos += 1;
            line.push(c);
            if c == b'\n' {
                break;
            }
        }
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    }

    /// `fgetc`; None is EOF.
    pub fn fgetc(&mut self) -> Option<u8> {
        if self.rpos >= self.rbuf.len() && !self.fill() {
            self.eof = true;
            return None;
        }
        let c = self.rbuf[self.rpos];
        self.rpos += 1;
        Some(c)
    }

    /// `fread` into `buf`; returns bytes read (C returns items; callers divide).
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> usize {
        let mut k = 0;
        while k < buf.len() {
            if self.rpos >= self.rbuf.len() && !self.fill() {
                self.eof = true;
                break;
            }
            let m = (self.rbuf.len() - self.rpos).min(buf.len() - k);
            buf[k..k + m].copy_from_slice(&self.rbuf[self.rpos..self.rpos + m]);
            self.rpos += m;
            k += m;
        }
        k
    }

    /// `fwrite` / `fputs` / the output side of `fprintf`.
    pub fn write_bytes(&mut self, b: &[u8]) {
        if !self.rbuf.is_empty() {
            self.drop_read();
        }
        self.wbuf.extend_from_slice(b);
        if self.wbuf.len() >= BUFSZ {
            self.flush_write();
        }
    }

    /// `ftell`.
    pub fn ftell(&mut self) -> i64 {
        let os = self.f.stream_position().map(|p| p as i64).unwrap_or(-1);
        os + self.wbuf.len() as i64 - (self.rbuf.len() - self.rpos) as i64
    }

    /// `fseek(f, pos, SEEK_SET)`; clears the end-of-file flag.
    pub fn fseek_set(&mut self, pos: i64) -> i32 {
        self.flush_write();
        self.rbuf.clear();
        self.rpos = 0;
        self.eof = false;
        match self.f.seek(SeekFrom::Start(pos.max(0) as u64)) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    /// `fseek(f, off, SEEK_END)`.
    pub fn fseek_end(&mut self, off: i64) -> i32 {
        self.flush_write();
        self.rbuf.clear();
        self.rpos = 0;
        self.eof = false;
        match self.f.seek(SeekFrom::End(off)) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    /// `rewind`: seek to the start, clear end-of-file and error.
    pub fn rewind(&mut self) {
        self.fseek_set(0);
        self.err = false;
    }

    /// `feof`.
    pub fn feof(&self) -> bool {
        self.eof
    }

    /// `fflush`.
    pub fn fflush(&mut self) {
        self.flush_write();
    }
}

impl Drop for CFile {
    fn drop(&mut self) {
        self.flush_write();
    }
}

/// A file name in C bytes as a path (on Unix any bytes are a valid name).
pub fn path_of(name: &[u8]) -> Option<std::path::PathBuf> {
    let n = cstr(name);
    if n.is_empty() {
        return None;
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        Some(std::path::PathBuf::from(std::ffi::OsStr::from_bytes(n)))
    }
    #[cfg(not(unix))]
    {
        Some(std::path::PathBuf::from(String::from_utf8_lossy(n).into_owned()))
    }
}

/// C `remove`.
pub fn remove(name: &[u8]) -> i32 {
    match path_of(name).map(fs::remove_file) {
        Some(Ok(())) => 0,
        _ => -1,
    }
}
