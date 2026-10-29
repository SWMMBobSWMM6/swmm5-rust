//! mathexpr, findroot, odesolve, hash, error: the safe engine against the
//! c2rust baseline, bit for bit.
#![allow(non_snake_case)]

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::sync::Mutex;
use swmm5_c2rust::engine as c;
use swmm5_difftest::{same, Rng};
use swmm5_rust::prelude::*;

// ---------------------------------------------------------------------------
//  mathexpr
// ---------------------------------------------------------------------------

const VARS: [&str; 6] = ["x", "Depth", "FLOW", "h_1", "q2", "T"];
static VALUES: Mutex<[f64; 6]> = Mutex::new([0.0; 6]);

fn var_index(name: &[u8]) -> i32 {
    for (i, v) in VARS.iter().enumerate() {
        if v.as_bytes().eq_ignore_ascii_case(name) {
            return i as i32;
        }
    }
    -1
}
unsafe extern "C" fn c_getVar(s: *mut c_char) -> c_int {
    var_index(CStr::from_ptr(s).to_bytes())
}
unsafe extern "C" fn c_getVal(i: c_int) -> f64 {
    VALUES.lock().unwrap()[i as usize]
}

/// The postfix list the baseline built, as (opcode, ivar, fvalue).
unsafe fn c_list(mut e: *mut c::mathexpr::MathExpr) -> Vec<(i32, i32, f64)> {
    let mut v = Vec::new();
    while !e.is_null() {
        v.push(((*e).opcode, (*e).ivar, (*e).fvalue));
        e = (*e).next;
    }
    v
}

#[test]
fn mathexpr_formulas() {
    let mut formulas: Vec<String> = [
        "1", "x", "x + 1", "x - 1", "-x", "-x + 2", "+x", "2*x", "x/2", "x^2", "2^x", "x^2^3", "-x^2", "(x)", "((x))", "(x+1)*(x-1)", "x*Depth + FLOW/2 - h_1",
        "sin(x)", "cos(x)", "tan(x)", "cot(x)", "abs(x)", "sgn(x)", "sqrt(x)", "log(x)", "exp(x)", "asin(x)", "acos(x)", "atan(x)", "acot(x)", "sinh(x)",
        "cosh(x)", "tanh(x)", "coth(x)", "log10(x)", "step(x)", "STEP(x - 1)", "Sqrt(Abs(x))", "1.5e-3*x", "1.5E+3", "2.5e", "1e", ".5*x", "5.*x", "3.14159",
        "x*-2", "x/-2", "x^-1", "x^-0.5", "2*-x", "x - -1", "x--1", "x+-1", "(-x)^2", "-(x)", "-(x+1)*2", "x*(Depth+(FLOW*(h_1-q2)))/T",
        "0.5*Depth^1.5*FLOW", "x + unknown", "x +", "* x", "(x", "x)", "sin x", "sin()", "", "   ", "1 2", "x y", "x^(1/3)", "exp(-x/T)*FLOW",
        "1.2.3", "1..2", "q2^2 - 4*x*T", "x^2+Depth^2", "-1", "-1.5e2", "x^Depth^FLOW", "abs(-x)", "step(-x)", "log(0)", "sqrt(-1)", "x/0",
        "0/0", "asin(2)", "log10(x)*log10(x)", "100000*x*x*x", "x*1e308*10", "(((((x+1)))))", "x*x*x*x*x*x*x*x*x*x", "1-2-3-4-5", "1/2/3/4",
        "2^3^2", "-2^2", "tanh(x)*coth(x)", "x_y", "_x", "x1", "Depth*0.0", "h_1^0", "0^x", "-0^x",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    // random formulas from a small grammar
    let mut rng = Rng(0x0f0f_1234_5555);
    fn gen(rng: &mut Rng, depth: u32) -> String {
        let leaf = |rng: &mut Rng| -> String {
            match rng.next() % 4 {
                0 => VARS[(rng.next() % 6) as usize].to_string(),
                1 => format!("{}", (rng.range(0.0, 10.0) * 100.0).round() / 100.0),
                2 => format!("{:e}", rng.range(0.0, 5.0)),
                _ => format!("{}", rng.next() % 10),
            }
        };
        if depth == 0 {
            return leaf(rng);
        }
        match rng.next() % 9 {
            0 | 1 => leaf(rng),
            2 => format!("{} + {}", gen(rng, depth - 1), gen(rng, depth - 1)),
            3 => format!("{}-{}", gen(rng, depth - 1), gen(rng, depth - 1)),
            4 => format!("{} * {}", gen(rng, depth - 1), gen(rng, depth - 1)),
            5 => format!("{}/{}", gen(rng, depth - 1), gen(rng, depth - 1)),
            6 => format!("{}^{}", gen(rng, depth - 1), gen(rng, depth - 1)),
            7 => format!("-({})", gen(rng, depth - 1)),
            _ => {
                let f = ["sin", "COS", "tan", "cot", "abs", "sgn", "sqrt", "log", "exp", "asin", "acos", "atan", "acot", "sinh", "cosh", "tanh", "coth", "log10", "step"];
                format!("{}({})", f[(rng.next() % f.len() as u64) as usize], gen(rng, depth - 1))
            }
        }
    }
    for _ in 0..3000 {
        formulas.push(gen(&mut rng, 4));
    }
    let mut parsed = 0usize;
    let mut evals = 0usize;
    for f in formulas.iter() {
        let mut gv = |s: &[u8]| var_index(s);
        let r = mathexpr_create(f.as_bytes(), &mut gv);
        let cs = CString::new(f.as_str()).unwrap();
        let ce = unsafe { c::mathexpr::mathexpr_create(cs.as_ptr() as *mut _, Some(c_getVar)) };
        let cl = unsafe { c_list(ce) };
        match &r {
            None => assert!(ce.is_null(), "formula {f:?}: rust failed, C parsed {cl:?}"),
            Some(list) => {
                assert!(!ce.is_null(), "formula {f:?}: rust parsed, C failed");
                let rl: Vec<(i32, i32, f64)> = list.iter().map(|e| (e.opcode, e.ivar, e.fvalue)).collect();
                assert_eq!(rl.len(), cl.len(), "formula {f:?}: list length");
                for (a, b) in rl.iter().zip(cl.iter()) {
                    assert!(a.0 == b.0 && a.1 == b.1 && same(a.2, b.2), "formula {f:?}: {rl:?} vs {cl:?}");
                }
                parsed += 1;
                for _ in 0..200 {
                    let vals: [f64; 6] = std::array::from_fn(|i| match rng.next() % 5 {
                        0 => 0.0,
                        1 => -rng.range(0.0, 10.0),
                        2 => rng.range(0.0, 1.0),
                        _ => rng.range(0.0, 100.0) * if i % 2 == 0 { 1.0 } else { 0.1 },
                    });
                    *VALUES.lock().unwrap() = vals;
                    let mut getv = |i: i32| vals[i as usize];
                    let a = mathexpr_eval(list, &mut getv);
                    let b = unsafe { c::mathexpr::mathexpr_eval(ce, Some(c_getVal)) };
                    assert!(same(a, b), "formula {f:?} at {vals:?}: rust {a:e} c {b:e}");
                    evals += 1;
                }
            }
        }
        unsafe { c::mathexpr::mathexpr_delete(ce) };
        mathexpr_delete(r);
    }
    eprintln!("mathexpr_formulas: {} formulas ({parsed} parse), {evals} evaluations", formulas.len());
}

// ---------------------------------------------------------------------------
//  findroot
// ---------------------------------------------------------------------------

/// Test functions, each with a parameter: f(x) and f'(x).
fn fdf(k: u32, p: f64, x: f64) -> (f64, f64) {
    match k {
        0 => (x * x - p, 2.0 * x),
        1 => (x * x * x - p * x - 1.0, 3.0 * x * x - p),
        2 => (x.cos() - p * x, -x.sin() - p),
        3 => ((x * p).exp() - 2.0, p * (x * p).exp()),
        4 => (x.powf(5.0 / 3.0) - p, (5.0 / 3.0) * x.powf(2.0 / 3.0)),
        5 => ((x - p).atan(), 1.0 / (1.0 + (x - p) * (x - p))),
        _ => (x.sin() + p * 0.1 * x - 0.3, x.cos() + p * 0.1),
    }
}

#[repr(C)]
struct Ctx {
    k: u32,
    p: f64,
}
unsafe extern "C" fn c_newton_f(x: f64, f: *mut f64, df: *mut f64, p: *mut c_void) {
    let c = &*(p as *const Ctx);
    let (a, b) = fdf(c.k, c.p, x);
    *f = a;
    *df = b;
}
unsafe extern "C" fn c_ridder_f(x: f64, p: *mut c_void) -> f64 {
    let c = &*(p as *const Ctx);
    fdf(c.k, c.p, x).0
}

#[test]
fn findroot_newton_ridder() {
    let mut rng = Rng(31337);
    let mut n = 0usize;
    for i in 0..40_000 {
        let k = (i % 7) as u32;
        let p = rng.range(0.05, 4.0);
        let x1 = rng.range(-3.0, 1.0);
        let x2 = x1 + rng.range(0.01, 5.0);
        let xacc = 10f64.powf(rng.range(-12.0, -1.0));
        let mut ctx = Ctx { k, p };
        // Newton
        let start = rng.range(x1, x2);
        let mut rr = start;
        let mut cr = start;
        let a = findroot_Newton(x1, x2, &mut rr, xacc, &mut |x: f64, f: &mut f64, df: &mut f64| {
            let (u, v) = fdf(k, p, x);
            *f = u;
            *df = v;
        });
        let b = unsafe { c::findroot::findroot_Newton(x1, x2, &mut cr, xacc, Some(c_newton_f), &mut ctx as *mut Ctx as *mut c_void) };
        assert_eq!(a, b, "Newton count k {k} p {p} [{x1},{x2}]");
        assert!(same(rr, cr), "Newton root k {k} p {p} [{x1},{x2}]: {rr} vs {cr}");
        // Ridder
        let a = findroot_Ridder(x1, x2, xacc, &mut |x: f64| fdf(k, p, x).0);
        let b = unsafe { c::findroot::findroot_Ridder(x1, x2, xacc, Some(c_ridder_f), &mut ctx as *mut Ctx as *mut c_void) };
        assert!(same(a, b), "Ridder k {k} p {p} [{x1},{x2}]: {a} vs {b}");
        n += 2;
    }
    eprintln!("findroot_newton_ridder: {n} root solves");
}

// ---------------------------------------------------------------------------
//  odesolve
// ---------------------------------------------------------------------------

static ODE_P: Mutex<(u32, [f64; 4])> = Mutex::new((0, [0.0; 4]));

fn derivs(k: u32, p: &[f64; 4], t: f64, y: &[f64], dy: &mut [f64]) {
    match k {
        0 => {
            // runoff-like: d' = i - a*max(d-ds,0)^(5/3)
            let rx = y[0] - p[1];
            let rx = if rx < 0.0 { 0.0 } else { p[2] * rx.powf(5.0 / 3.0) };
            dy[0] = p[0] - rx;
        }
        1 => {
            dy[0] = -p[0] * y[0] + p[1] * y[1];
            dy[1] = p[2] * y[0] - p[3] * y[1] * y[1];
        }
        2 => {
            dy[0] = y[1];
            dy[1] = -p[0] * y[0] - p[1] * y[1] + (t * p[2]).sin();
            dy[2] = y[0] * y[1] * 0.01;
        }
        _ => {
            for i in 0..4 {
                dy[i] = (p[i] * t).cos() - 0.1 * y[i] * y[(i + 1) % 4];
            }
        }
    }
}
unsafe extern "C" fn c_derivs(t: f64, y: *mut f64, dy: *mut f64) {
    let (k, p) = *ODE_P.lock().unwrap();
    let n = [1, 2, 3, 4][k as usize];
    let ys = std::slice::from_raw_parts(y, n);
    let ds = std::slice::from_raw_parts_mut(dy, n);
    derivs(k, &p, t, ys, ds);
}

#[test]
fn odesolve_integrate_systems() {
    unsafe { c::odesolve::odesolve_open(MAXODES) };
    assert_eq!(odesolve_open(MAXODES), 1);
    let mut rng = Rng(99991);
    let mut n = 0usize;
    for i in 0..20_000 {
        let k = (i % 4) as u32;
        let nn = [1usize, 2, 3, 4][k as usize];
        let p: [f64; 4] = std::array::from_fn(|_| rng.range(0.0, 3.0));
        *ODE_P.lock().unwrap() = (k, p);
        let y0: [f64; 4] = std::array::from_fn(|_| rng.range(-1.0, 2.0));
        let x2 = rng.range(0.1, 3600.0) * if i % 3 == 0 { 0.001 } else { 1.0 };
        let eps = [0.0001, 0.001, 0.00001, 0.05][(i / 4 % 4) as usize];
        let h1 = if i % 2 == 0 { x2 } else { x2 * 0.1 };
        let mut ry = y0;
        let mut cy = y0;
        let a = odesolve_integrate(&mut ry[..nn], nn as i32, 0.0, x2, eps, h1, &mut |t: f64, y: &[f64], dy: &mut [f64]| derivs(k, &p, t, y, dy));
        let b = unsafe { c::odesolve::odesolve_integrate(cy.as_mut_ptr(), nn as i32, 0.0, x2, eps, h1, Some(c_derivs)) };
        assert_eq!(a, b, "odesolve return, system {k} case {i}");
        for j in 0..nn {
            assert!(same(ry[j], cy[j]), "odesolve system {k} case {i} y[{j}]: {} vs {}", ry[j], cy[j]);
        }
        n += 1;
    }
    unsafe { c::odesolve::odesolve_close() };
    odesolve_close();
    eprintln!("odesolve_integrate_systems: {n} integrations");
}

// ---------------------------------------------------------------------------
//  hash
// ---------------------------------------------------------------------------

#[test]
fn hash_tables() {
    let mut rng = Rng(0x4a5b);
    let mut keys: Vec<Vec<u8>> = vec![b"J1".to_vec(), b"j1".to_vec(), b"Outfall".to_vec(), b"OUTFALL".to_vec(), b"a".to_vec(), b"A".to_vec(), b"zz".to_vec(), b"{|}~".to_vec(), b"[\\]^_`".to_vec()];
    for _ in 0..20_000 {
        let len = 1 + (rng.next() % 20) as usize;
        let k: Vec<u8> = (0..len)
            .map(|_| {
                let c = (rng.next() % 254 + 1) as u8;
                if rng.next() % 3 == 0 { c } else { b"abcdefgxyzABCXYZ0123456789_-.@"[(rng.next() % 30) as usize] }
            })
            .collect();
        keys.push(k);
    }
    let mut n = 0usize;
    for k in keys.iter() {
        let ck = CString::new(k.clone()).unwrap();
        assert_eq!(hash(k), unsafe { c::hash::hash(ck.as_ptr()) }, "hash({k:?})");
        n += 1;
    }
    // samestr on pairs (including case variants)
    for i in 0..keys.len() {
        let a = &keys[i];
        let b = if i % 3 == 0 { a.to_ascii_uppercase() } else if i % 3 == 1 { keys[(i * 7919) % keys.len()].clone() } else { a[..a.len() - 1].to_vec() };
        let ca = CString::new(a.clone()).unwrap();
        let cb = CString::new(b.clone()).unwrap();
        if b.is_empty() {
            continue;
        }
        assert_eq!(samestr(a, &b), unsafe { c::hash::samestr(ca.as_ptr(), cb.as_ptr()) }, "samestr({a:?},{b:?})");
        n += 1;
    }
    // insert/find through both tables
    let mut rt = HTcreate();
    let ct = unsafe { c::hash::HTcreate() };
    let mut owned: Vec<CString> = Vec::new();
    for (i, k) in keys.iter().enumerate().take(10_000) {
        let ck = CString::new(k.clone()).unwrap();
        assert_eq!(HTinsert(&mut rt, k, i as i32), unsafe { c::hash::HTinsert(ct, ck.as_ptr() as *mut _, i as i32) });
        owned.push(ck);
    }
    for k in keys.iter() {
        let ck = CString::new(k.clone()).unwrap();
        let mut probe = k.clone();
        if rng.next() % 2 == 0 {
            probe = probe.to_ascii_lowercase();
        }
        let cp = CString::new(probe.clone()).unwrap();
        assert_eq!(HTfind(&rt, &probe), unsafe { c::hash::HTfind(ct, cp.as_ptr()) }, "HTfind({probe:?})");
        let rk = HTfindKey(&rt, &probe);
        let ckey = unsafe { c::hash::HTfindKey(ct, cp.as_ptr()) };
        match rk {
            None => assert!(ckey.is_null()),
            Some(kk) => assert_eq!(kk, unsafe { CStr::from_ptr(ckey) }.to_bytes()),
        }
        let _ = ck;
        n += 2;
    }
    HTfree(&mut rt);
    unsafe { c::hash::HTfree(ct) };
    drop(owned);
    eprintln!("hash_tables: {} keys, {n} comparisons", keys.len());
}

// ---------------------------------------------------------------------------
//  error
// ---------------------------------------------------------------------------

#[test]
fn error_messages() {
    let mut n = 0;
    for code in -5..700 {
        let mut msg = b"previous message".to_vec();
        error_getMsg(code, &mut msg);
        let mut buf = [0 as c_char; 512];
        buf[0] = b'z' as _;
        unsafe { c::error::error_getMsg(code, buf.as_mut_ptr()) };
        let cm = unsafe { CStr::from_ptr(buf.as_ptr()) }.to_bytes();
        assert_eq!(msg, cm, "error_getMsg({code})");
        n += 1;
    }
    let mut prj = Project::default();
    for s in [&b"J1"[..], b"", b"a longer token name"] {
        assert_eq!(error_setInpError(&mut prj, 211, s), 211);
        let cs = CString::new(s).unwrap();
        assert_eq!(unsafe { c::error::error_setInpError(211, cs.as_ptr() as *mut _) }, 211);
        let cb: Vec<u8> = unsafe { (*std::ptr::addr_of!(c::error::ErrString)).iter().take_while(|&&ch| ch != 0).map(|&ch| ch as u8).collect() };
        assert_eq!(cio::cstr(&prj.error.ErrString), &cb[..]);
        n += 1;
    }
    eprintln!("error_messages: {n} comparisons");
}
