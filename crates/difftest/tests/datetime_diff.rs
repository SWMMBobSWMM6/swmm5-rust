//! datetime: the safe engine against the c2rust baseline, bit for bit.
#![allow(non_snake_case)]

use std::ffi::CString;
use swmm5_c2rust::engine::datetime as c;
use swmm5_difftest::{same, Rng};
use swmm5_rust::prelude::*;

fn cbuf_bytes(b: &[std::ffi::c_char]) -> Vec<u8> {
    b.iter().take_while(|&&ch| ch != 0).map(|&ch| ch as u8).collect()
}

#[test]
fn datetime_encode_decode() {
    let mut n = 0usize;
    // encodeDate over a grid that includes invalid values
    for year in (-2..=10002).step_by(1) {
        for month in -1..=13 {
            for &day in &[-1, 0, 1, 2, 15, 28, 29, 30, 31, 32] {
                let r = datetime_encodeDate(year, month, day);
                let cv = unsafe { c::datetime_encodeDate(year, month, day) };
                assert!(same(r, cv), "encodeDate({year},{month},{day})");
                n += 1;
            }
        }
    }
    // decodeDate / decodeTime / derived functions on random dates
    let mut rng = Rng(0x1234_5678_9abc_def1);
    for i in 0..400_000 {
        let d = match i % 5 {
            0 => rng.range(-700_000.0, 3_000_000.0),
            1 => rng.range(30_000.0, 60_000.0),
            2 => (rng.range(30_000.0, 60_000.0) * 86400.0).round() / 86400.0,
            3 => rng.range(-1.0e10, 1.0e10),
            _ => rng.range(-1.0, 2.0),
        };
        let (mut y1, mut m1, mut d1) = (0, 0, 0);
        let (mut y2, mut m2, mut d2) = (0, 0, 0);
        datetime_decodeDate(d, &mut y1, &mut m1, &mut d1);
        unsafe { c::datetime_decodeDate(d, &mut y2, &mut m2, &mut d2) };
        assert_eq!((y1, m1, d1), (y2, m2, d2), "decodeDate({d:e})");
        let (mut h1, mut mi1, mut s1) = (0, 0, 0);
        let (mut h2, mut mi2, mut s2) = (0, 0, 0);
        datetime_decodeTime(d, &mut h1, &mut mi1, &mut s1);
        unsafe { c::datetime_decodeTime(d, &mut h2, &mut mi2, &mut s2) };
        assert_eq!((h1, mi1, s1), (h2, mi2, s2), "decodeTime({d:e})");
        unsafe {
            assert_eq!(datetime_monthOfYear(d), c::datetime_monthOfYear(d), "monthOfYear({d:e})");
            assert_eq!(datetime_dayOfYear(d), c::datetime_dayOfYear(d), "dayOfYear({d:e})");
            assert_eq!(datetime_dayOfWeek(d), c::datetime_dayOfWeek(d), "dayOfWeek({d:e})");
            assert_eq!(datetime_hourOfDay(d), c::datetime_hourOfDay(d), "hourOfDay({d:e})");
            let secs = match i % 3 {
                0 => rng.range(-1.0e6, 1.0e6),
                1 => (rng.range(0.0, 86400.0)).floor(),
                _ => rng.range(0.0, 30.0),
            };
            assert!(same(datetime_addSeconds(d, secs), c::datetime_addSeconds(d, secs)), "addSeconds({d:e},{secs:e})");
            let d2v = rng.range(-1000.0, 1000.0);
            assert!(same(datetime_addDays(d, d2v), c::datetime_addDays(d, d2v)), "addDays({d:e},{d2v:e})");
            let d3 = if i % 2 == 0 { d + rng.range(-10.0, 10.0) } else { rng.range(30_000.0, 60_000.0) };
            assert_eq!(datetime_timeDiff(d, d3), c::datetime_timeDiff(d, d3) as i64, "timeDiff({d:e},{d3:e})");
        }
        n += 9;
    }
    // encodeTime and daysPerMonth
    for h in -2..30 {
        for m in -2..65 {
            for s in [-1, 0, 1, 30, 59, 60, 61, 3599, 86399, 100000] {
                let r = datetime_encodeTime(h, m, s as f64);
                let cv = unsafe { c::datetime_encodeTime(h, m, s) };
                assert!(same(r, cv), "encodeTime({h},{m},{s})");
                n += 1;
            }
        }
    }
    for year in -5..2500 {
        for month in -1..=13 {
            assert_eq!(datetime_daysPerMonth(year, month), unsafe { c::datetime_daysPerMonth(year, month) });
            n += 1;
        }
    }
    eprintln!("datetime_encode_decode: {n} comparisons");
}

#[test]
fn datetime_strings() {
    let mut prj = Project::default();
    let mut n = 0usize;
    let mut rng = Rng(77);
    // --- dateToStr / timeToStr / getTimeStamp in each format
    for fmt in [Y_M_D, M_D_Y, D_M_Y, 3, -1] {
        unsafe { c::datetime_setDateFormat(Y_M_D) };
        prj.datetime.DateFormat = Y_M_D;
        datetime_setDateFormat(&mut prj, fmt);
        unsafe { c::datetime_setDateFormat(fmt) };
        for i in 0..60_000 {
            let d = match i % 4 {
                0 => rng.range(-700_000.0, 3_000_000.0),
                1 => rng.range(30_000.0, 60_000.0),
                2 => rng.range(-1.0e9, 1.0e9),
                _ => (rng.range(30_000.0, 60_000.0) * 1440.0).round() / 1440.0,
            };
            let mut rs = b"previous contents that must go".to_vec();
            datetime_dateToStr(&prj, d, &mut rs);
            let mut cb = [0 as std::ffi::c_char; 64];
            unsafe { c::datetime_dateToStr(d, cb.as_mut_ptr()) };
            assert_eq!(rs, cbuf_bytes(&cb), "dateToStr({d:e}) fmt {fmt}");
            let mut rt = b"previous contents".to_vec();
            datetime_timeToStr(d, &mut rt);
            let mut ct = [0 as std::ffi::c_char; 64];
            unsafe { c::datetime_timeToStr(d, ct.as_mut_ptr()) };
            assert_eq!(rt, cbuf_bytes(&ct), "timeToStr({d:e})");
            for &(sfmt, size) in &[(Y_M_D, 21), (M_D_Y, 24), (D_M_Y, 30), (5, 21), (Y_M_D, 20), (M_D_Y, 15)] {
                let mut rts = b"old".to_vec();
                datetime_getTimeStamp(&prj, sfmt, d, size, &mut rts);
                let mut cts = [0 as std::ffi::c_char; 64];
                cts[0] = b'o' as _;
                cts[1] = b'l' as _;
                cts[2] = b'd' as _;
                unsafe { c::datetime_getTimeStamp(sfmt, d, size, cts.as_mut_ptr()) };
                assert_eq!(rts, cbuf_bytes(&cts), "getTimeStamp({sfmt},{d:e},{size})");
                n += 1;
            }
            n += 2;
        }
    }

    // --- strToDate in each format
    let mut dates: Vec<String> = vec![
        "1/2/2003", "01/02/2003", "12/31/1999", "2/29/2000", "2/29/1900", "2/30/2004", "13/1/2000", "0/1/2000", "1/0/2000",
        "1-2-2003", "2003-01-02", "2003/1/2", "02-JAN-2003", "JAN-02-2003", "Jan/02/2003", "jan-2-2003", "2003-Jan-02",
        "2003-jan-2", "02-Feb-2003", "31-dec-1999", "1-XYZ-2000", "abc", "", "-", "/", "1/2", "1/2/", "2003", "1.5/2/2003",
        "1/2/2003extra", " 1/2/2003", "1/ 2/2003", "10000/1/1", "1/1/10000", "1/1/0", "1/1/1", "12/31/9999", "-1/2/2003",
        "1--2-2003", "MAR-15-1980", "SEPT-1-2000", "1-SEP-1999", "1999-SEP-1", "1999-12-31", "31-12-1999", "31/12/1999",
        "00-JAN-2000", "JANUARY-1-2000", "2000-FEBRUARY-3", "1x2y2003", "7/4/1776",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let mons = ["JAN", "feb", "Mar", "APR", "may", "JUN", "jul", "AUG", "sep", "OCT", "nov", "DEC", "XXX", "J", "JA"];
    for _ in 0..30_000 {
        let y = (rng.next() % 3000) as i64 - 50;
        let m = (rng.next() % 15) as i64 - 1;
        let d = (rng.next() % 34) as i64 - 1;
        let sep = [b'/', b'-', b'.', b' ', b'x'][(rng.next() % 5) as usize] as char;
        let sep2 = [b'/', b'-'][(rng.next() % 2) as usize] as char;
        let mn = mons[(rng.next() % mons.len() as u64) as usize];
        let s = match rng.next() % 7 {
            0 => format!("{m}{sep}{d}{sep2}{y}"),
            1 => format!("{y}{sep}{m}{sep2}{d}"),
            2 => format!("{d}{sep}{m}{sep2}{y}"),
            3 => format!("{mn}{sep2}{d}{sep}{y}"),
            4 => format!("{y}{sep2}{mn}{sep}{d}"),
            5 => format!("{d}{sep2}{mn}{sep2}{y}"),
            _ => format!("{m:02}{sep2}{d:02}{sep2}{y:04}"),
        };
        dates.push(s);
    }
    // (datetime_setDateFormat accepts only Y_M_D and M_D_Y; D_M_Y is refused
    //  and leaves the previous format in place, in both engines)
    for fmt in [Y_M_D, M_D_Y, D_M_Y] {
        datetime_setDateFormat(&mut prj, fmt);
        unsafe { c::datetime_setDateFormat(fmt) };
        for s in dates.iter() {
            let mut rd: f64 = 123.0;
            let rr = datetime_strToDate(&prj, s.as_bytes(), &mut rd);
            let cs = CString::new(s.as_str()).unwrap();
            let mut cd: f64 = 123.0;
            let cr = unsafe { c::datetime_strToDate(cs.as_ptr() as *mut _, &mut cd) };
            assert_eq!(rr, cr, "strToDate({s:?}) fmt {fmt}");
            assert!(same(rd, cd), "strToDate({s:?}) fmt {fmt}: {rd} vs {cd}");
            n += 1;
        }
    }
    unsafe { c::datetime_setDateFormat(M_D_Y) };

    // --- strToTime
    let mut times: Vec<String> = vec![
        "12", "12.5", "12:30", "12:30:45", "-1:00", "abc", "1e2", " 5", "5 ", "", "24:00", "1:2:3:4", "nan", "inf", ":30", "0:0:0", "23:59:59", "23:59:60",
        "100:00", "1:-1", "1:30:-5", "+3:15", "0x10", "1.5:30", "12:", "12::30", "-0.5", "  ", "12h", "07:05", "7:5:3",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    for _ in 0..30_000 {
        let h = (rng.next() % 40) as i64 - 5;
        let m = (rng.next() % 70) as i64 - 3;
        let s = (rng.next() % 70) as i64 - 3;
        let t = match rng.next() % 5 {
            0 => format!("{h}:{m}"),
            1 => format!("{h}:{m}:{s}"),
            2 => format!("{}", rng.range(-5.0, 30.0)),
            3 => format!("{h:02}:{m:02}"),
            _ => format!("{h}"),
        };
        times.push(t);
    }
    for s in times.iter() {
        let mut rt: f64 = 9.0;
        let rr = datetime_strToTime(s.as_bytes(), &mut rt);
        let cs = CString::new(s.as_str()).unwrap();
        let mut ct: f64 = 9.0;
        let cr = unsafe { c::datetime_strToTime(cs.as_ptr() as *mut _, &mut ct) };
        assert_eq!(rr, cr, "strToTime({s:?})");
        assert!(same(rt, ct), "strToTime({s:?}): {rt} vs {ct}");
        n += 1;
    }

    // --- findMonth
    for s in ["JAN", "jan", "Feb", "MARCH", "apr", "May", "JUNE", "jul", "aug", "SEP", "oct", "NOV", "dec", "XYZ", "ja", "", "j", "DECEMBER"] {
        let cs = CString::new(s).unwrap();
        // (C reads 3 characters; give the baseline a buffer that has them)
        let mut buf = cs.as_bytes_with_nul().to_vec();
        buf.resize(8, 0);
        assert_eq!(datetime_findMonth(s.as_bytes()), unsafe { c::datetime_findMonth(buf.as_mut_ptr() as *mut _) }, "findMonth({s:?})");
        n += 1;
    }
    eprintln!("datetime_strings: {n} comparisons ({} date strings x 3 formats, {} time strings)", dates.len(), times.len());
}
