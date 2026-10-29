//! The C library functions in cio against glibc's own, on many inputs.
use std::ffi::{CStr, CString};
use swmm5_rust::cio;

fn c_fmt_f(fmt: &str, x: f64) -> Vec<u8> {
    let f = CString::new(fmt).unwrap();
    let mut buf = vec![0u8; 512];
    unsafe { libc::snprintf(buf.as_mut_ptr() as *mut libc::c_char, 512, f.as_ptr(), x) };
    unsafe { CStr::from_ptr(buf.as_ptr() as *const libc::c_char) }.to_bytes().to_vec()
}
fn c_fmt_i(fmt: &str, x: libc::c_long) -> Vec<u8> {
    let f = CString::new(fmt).unwrap();
    let mut buf = vec![0u8; 512];
    unsafe { libc::snprintf(buf.as_mut_ptr() as *mut libc::c_char, 512, f.as_ptr(), x) };
    unsafe { CStr::from_ptr(buf.as_ptr() as *const libc::c_char) }.to_bytes().to_vec()
}
fn c_fmt_s(fmt: &str, s: &str) -> Vec<u8> {
    let f = CString::new(fmt).unwrap();
    let a = CString::new(s).unwrap();
    let mut buf = vec![0u8; 512];
    unsafe { libc::snprintf(buf.as_mut_ptr() as *mut libc::c_char, 512, f.as_ptr(), a.as_ptr()) };
    unsafe { CStr::from_ptr(buf.as_ptr() as *const libc::c_char) }.to_bytes().to_vec()
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn double(&mut self) -> f64 {
        match self.next() % 6 {
            0 => f64::from_bits(self.next()),                         // any bit pattern
            1 => (self.next() % 2_000_001) as f64 / 1000.0 - 1000.0,   // decimal-looking
            2 => (self.next() % 20001) as f64 * 0.0005,                // ties at the 4th place
            3 => ((self.next() % 1000) as f64 + 0.5) / 10f64.powi((self.next() % 6) as i32),
            4 => (self.next() as f64 / u64::MAX as f64 - 0.5) * 10f64.powi((self.next() % 40) as i32 - 20),
            _ => [0.0, -0.0, 0.5, 1.5, 2.5, 0.125, 0.0625, 1e-300, 9.9995, 99.995, 0.9999995,
                  f64::NAN, -f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0/0.0, 1e21, 123456789.0][(self.next() % 18) as usize],
        }
    }
}

#[test]
fn printf_floats_match_glibc() {
    let fmts = ["%14.3f", "%10.2f", "%7.2f", "%9.3f", "%8.3f", "%8.2f", "%10.4f", "%10.3f", "%9.2f", "%.2f",
                "%6.2f", "%12.3f", "%5.1f", "%4.2f", "%f", "%6.3f", "%6.1f", "%10.1f", "%.3f", "%-10f",
                "%8.4f", "%12.2f", "%.6f", "%.1f", "%.0f", "%12.3g", "%14.3e", "%g", "%e", "%.10g", "%010.3f",
                "%+8.2f", "% 8.2f", "%#.0f", "%-12.3g", "%.0e", "%#g", "%.17g", "%.15e", "%G", "%E"];
    let mut r = Rng(0x9E3779B97F4A7C15);
    let mut n = 0;
    for _ in 0..200_000 {
        let x = r.double();
        for f in fmts {
            let want = c_fmt_f(f, x);
            let got = swmm5_rust::cformat!(f.as_bytes(), x);
            assert_eq!(String::from_utf8_lossy(&got), String::from_utf8_lossy(&want), "fmt {} x {:e} bits {:x}", f, x, x.to_bits());
            n += 1;
        }
    }
    eprintln!("{} float conversions identical", n);
}

#[test]
fn printf_ints_and_strings_match_glibc() {
    let fmts = ["%ld", "%lx", "%lu", "%d", "%2d", "%02d", "%4d", "%3d", "%-4d", "%04d", "%5d", "%6d", "%10d", "%6ld", "%7ld", "%ld",
                "%-5ld", "%-2ld", "%i", "%+d", "% d", "%.3d", "%x", "%X", "%u"];
    let mut r = Rng(12345);
    for _ in 0..100_000 {
        let v = match r.next() % 3 { 0 => (r.next() % 2001) as i64 - 1000, 1 => r.next() as i32 as i64, _ => r.next() as i64 };
        for f in fmts {
            let want = c_fmt_i(f, v as libc::c_long);
            // C passes an int unless the format says long
            let got = if f.contains('l') { swmm5_rust::cformat!(f.as_bytes(), v) } else { swmm5_rust::cformat!(f.as_bytes(), v as i32) };
            assert_eq!(got, want, "fmt {} v {}", f, v);
        }
    }
    let sfmts = ["%s", "%3s", "%-20s", "%-16s", "%8s", "%14s", "%9s", "%10s", "%-10s", "%-30s", "%-s", "%.3s", "%20s", "%-5s"];
    for s in ["", "J1", "OUTFALL_LONG_NAME_123456789", "a b", "Ünïcode", "x"] {
        for f in sfmts {
            assert_eq!(swmm5_rust::cformat!(f.as_bytes(), s), c_fmt_s(f, s), "fmt {} s {}", f, s);
        }
    }
    assert_eq!(swmm5_rust::cformat!(b"%c|%5c|%-3c|", b'A', b'z', b'q'), b"A|    z|q  |".to_vec());
    assert_eq!(swmm5_rust::cformat!(b"100%% of %d\0trailing", 5i32), b"100% of 5".to_vec());
}

#[test]
fn strtod_and_atoi_match_glibc() {
    let mut r = Rng(777);
    let pieces = ["", " ", "\t", "+", "-", "0", "1", "9", ".", "e", "E", "x", "0x", "p", "inf", "nan", "infinity", "INF",
                  "123", "4.5", "e10", "e-3", "e+", ",", "abc", "1e400", "1e-400", "00012", "7.", ".25", "nan(1)", "1,5"];
    for _ in 0..300_000 {
        let k = 1 + r.next() % 4;
        let mut s = String::new();
        for _ in 0..k {
            s.push_str(pieces[(r.next() % pieces.len() as u64) as usize]);
        }
        let cs = CString::new(s.clone()).unwrap();
        let mut end: *mut libc::c_char = std::ptr::null_mut();
        let v = unsafe { libc::strtod(cs.as_ptr(), &mut end) };
        let used = unsafe { end.offset_from(cs.as_ptr()) } as usize;
        let (rv, rused) = cio::strtod(s.as_bytes());
        assert_eq!(rused, used, "strtod consumed for {:?}", s);
        assert!(rv.to_bits() == v.to_bits() || (rv.is_nan() && v.is_nan()), "strtod value for {:?}: {} vs {}", s, rv, v);
        let ai = unsafe { libc::atoi(cs.as_ptr()) };
        assert_eq!(cio::atoi(s.as_bytes()), ai, "atoi {:?}", s);
    }
}

#[test]
fn sscanf_matches_glibc_on_swmm_formats() {
    // the date/time parse formats of datetime.c and a few of rain.c/climate.c
    let inputs = ["2026-10-28", "10/28/2026", "28.OCT.2026", "OCT-28-2026", "12:30:45", "1:5", "",
                  "  7", "2026/02", "199901", "2024JAN15", "x", "12-", "12:30", " 3 4 5 ", "100.5 A", "-3 B"];
    let mut r = Rng(4242);
    for inp in inputs {
        let ci = CString::new(inp).unwrap();
        let (mut a, mut b, mut c) = (0i32, 0i32, 0i32);
        let (mut s1, mut s2) = (0u8, 0u8);
        let n = unsafe { libc::sscanf(ci.as_ptr(), c"%d%c%d%c%d".as_ptr(), &mut a, &mut s1, &mut b, &mut s2, &mut c) };
        let (mut ra, mut rb, mut rc) = (0i32, 0i32, 0i32);
        let (mut rs1, mut rs2) = (0u8, 0u8);
        let rn = cio::sscanf(inp.as_bytes(), b"%d%c%d%c%d", &mut [cio::Scan::Int(&mut ra), cio::Scan::Char(&mut rs1), cio::Scan::Int(&mut rb), cio::Scan::Char(&mut rs2), cio::Scan::Int(&mut rc)]);
        assert_eq!(rn, n, "count {:?}", inp);
        let k = n.max(0);
        if k >= 1 { assert_eq!(ra, a) } if k >= 2 { assert_eq!(rs1, s1) } if k >= 3 { assert_eq!(rb, b) }
        if k >= 4 { assert_eq!(rs2, s2) } if k >= 5 { assert_eq!(rc, c) }
        let (mut h, mut m, mut s) = (0i32, 0i32, 0i32);
        let n = unsafe { libc::sscanf(ci.as_ptr(), c"%d:%d:%d".as_ptr(), &mut h, &mut m, &mut s) };
        let (mut rh, mut rm, mut rs) = (0i32, 0i32, 0i32);
        let rn = cio::sscanf(inp.as_bytes(), b"%d:%d:%d", &mut [cio::Scan::Int(&mut rh), cio::Scan::Int(&mut rm), cio::Scan::Int(&mut rs)]);
        assert_eq!(rn, n, "hms count {:?}", inp);
        let mut fx = 0f32; let mut fc = 0u8;
        let n = unsafe { libc::sscanf(ci.as_ptr(), c"%f %c".as_ptr(), &mut fx, &mut fc) };
        let mut rfx = 0f32; let mut rfc = 0u8;
        let rn = cio::sscanf(inp.as_bytes(), b"%f %c", &mut [cio::Scan::Float(&mut rfx), cio::Scan::Char(&mut rfc)]);
        assert_eq!(rn, n, "f c count {:?}", inp);
        if n >= 1 { assert_eq!(rfx.to_bits(), fx.to_bits()) } if n >= 2 { assert_eq!(rfc, fc) }
        let (mut y, mut mo, mut d) = (0i32, 0i32, 0i32);
        let n = unsafe { libc::sscanf(ci.as_ptr(), c"%4d%2d%2d".as_ptr(), &mut y, &mut mo, &mut d) };
        let (mut ry, mut rmo, mut rd) = (0i32, 0i32, 0i32);
        let rn = cio::sscanf(inp.as_bytes(), b"%4d%2d%2d", &mut [cio::Scan::Int(&mut ry), cio::Scan::Int(&mut rmo), cio::Scan::Int(&mut rd)]);
        assert_eq!(rn, n, "%4d%2d%2d count {:?}", inp);
        if n >= 1 { assert_eq!((ry), y) } if n >= 2 { assert_eq!(rmo, mo) } if n >= 3 { assert_eq!(rd, d) }
        let mut st = vec![0u8; 64]; let mut sm = vec![0u8; 8];
        let (mut x1, mut x2) = (0i32, 0i32);
        let n = unsafe { libc::sscanf(ci.as_ptr(), c"%d%c%3s%c%d".as_ptr(), &mut x1, st.as_mut_ptr(), sm.as_mut_ptr(), st.as_mut_ptr().add(8), &mut x2) };
        let (mut rx1, mut rx2) = (0i32, 0i32); let (mut rc1, mut rc2) = (0u8, 0u8); let mut rsm = Vec::new();
        let rn = cio::sscanf(inp.as_bytes(), b"%d%c%3s%c%d", &mut [cio::Scan::Int(&mut rx1), cio::Scan::Char(&mut rc1), cio::Scan::Str(&mut rsm), cio::Scan::Char(&mut rc2), cio::Scan::Int(&mut rx2)]);
        assert_eq!(rn, n, "%d%c%3s count {:?}", inp);
        if n >= 3 { assert_eq!(rsm, cio::cstr(&sm).to_vec()) }
        let _ = r.next();
    }
}
