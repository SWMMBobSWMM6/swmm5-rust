//! odesolve.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   odesolve.c
//
//   Fifth-order Runge-Kutta integration with adaptive step size control
//   based on code from Numerical Recipes in C (Cambridge University
//   Press, 1992).
//
//   Date:     11/15/06
//   Author:   L. Rossman
//-----------------------------------------------------------------------------
//
// C keeps its work arrays (y, yscal, yerr, ytemp, dydx, ak) as file-scope
// pointers allocated by odesolve_open(n). Every element a call reads is
// written earlier in that same call, so nothing carries from one call to the
// next: here they are locals of odesolve_integrate, sized MAXODES (the only
// size SWMM ever opens the solver with), and passed to rkqs/rkck.

use crate::prelude::*;

/// The file-scope variables of odesolve.c.
#[derive(Default)]
pub struct OdesolveState {}

const MAXSTP: i32 = 10000;
const TINY: f64 = 1.0e-30;
const SAFETY: f64 = 0.9;
const PGROW: f64 = -0.2;
const PSHRNK: f64 = -0.25;
const ERRCON: f64 = 1.89e-4; // = (5/SAFETY)^(1/PGROW)

const NMAX: usize = MAXODES as usize;

/// The solver's work arrays (C: file-scope y, yscal, yerr, ytemp, dydx, ak).
struct Work {
    y: [f64; NMAX],     // dependent variable
    yscal: [f64; NMAX], // scaling factors
    yerr: [f64; NMAX],  // integration errors
    ytemp: [f64; NMAX], // temporary values of y
    dydx: [f64; NMAX],  // derivatives of y
    ak: [f64; NMAX * 5], // derivatives at intermediate points
}

//-----------------------------------------------------------------------------
//    open the ODE solver to solve system of n equations
//    (return 1 if successful, 0 if not)
//-----------------------------------------------------------------------------
pub fn odesolve_open(maxn: i32) -> i32 {
    // C allocates the work arrays here; they are locals of
    // odesolve_integrate in this translation.
    1
}

//-----------------------------------------------------------------------------
//    close the ODE solver
//-----------------------------------------------------------------------------
pub fn odesolve_close() {}

/// C: `int odesolve_integrate(double ystart[], int n, double x1, double x2,
///      double eps, double h1, void (*derivs)(double, double*, double*))`
//---------------------------------------------------------------
//   Driver function for Runge-Kutta integration with adaptive
//   stepsize control. Integrates starting n values in ystart[]
//   from x1 to x2 with accuracy eps. h1 is the initial stepsize
//   guess and derivs is a user-supplied function that computes
//   derivatives dy/dx of y. On completion, ystart[] contains the
//   new values of y at the end of the integration interval.
//---------------------------------------------------------------
pub fn odesolve_integrate(ystart: &mut [f64], n: i32, x1: f64, x2: f64, eps: f64, h1: f64, derivs: &mut dyn FnMut(f64, &[f64], &mut [f64])) -> i32 {
    let mut i: i32;
    let mut errcode: i32;
    let mut nstp: i32;
    let mut hdid: f64 = 0.0;
    let mut hnext: f64 = 0.0;
    let mut x: f64 = x1;
    let mut h: f64 = h1;
    // C: if (nmax < n) return 1;  (nmax == MAXODES once runoff opened the solver)
    if (NMAX as i32) < n {
        return 1;
    }
    let mut w = Work {
        y: [0.0; NMAX],
        yscal: [0.0; NMAX],
        yerr: [0.0; NMAX],
        ytemp: [0.0; NMAX],
        dydx: [0.0; NMAX],
        ak: [0.0; NMAX * 5],
    };
    let nu = n as usize;
    i = 0;
    while i < n {
        w.y[i as usize] = ystart[i as usize];
        i += 1;
    }
    nstp = 1;
    while nstp <= MAXSTP {
        derivs(x, &w.y[..nu], &mut w.dydx[..nu]);
        i = 0;
        while i < n {
            let iu = i as usize;
            w.yscal[iu] = w.y[iu].abs() + (w.dydx[iu] * h).abs() + TINY;
            i += 1;
        }
        if (x + h - x2) * (x + h - x1) > 0.0 {
            h = x2 - x;
        }
        errcode = rkqs(&mut w, &mut x, n, h, eps, &mut hdid, &mut hnext, derivs);
        if errcode != 0 {
            break;
        }
        if (x - x2) * (x2 - x1) >= 0.0 {
            i = 0;
            while i < n {
                ystart[i as usize] = w.y[i as usize];
                i += 1;
            }
            return 0;
        }
        if hnext.abs() <= 0.0 {
            return 2;
        }
        h = hnext;
        nstp += 1;
    }
    3
}

//---------------------------------------------------------------
//   Fifth-order Runge-Kutta integration step with monitoring of
//   local truncation error to assure accuracy and adjust stepsize.
//   Inputs are current value of x, trial step size (htry), and
//   accuracy (eps). Outputs are stepsize taken (hdid) and estimated
//   next stepsize (hnext). Also updated are the values of y[].
//---------------------------------------------------------------
fn rkqs(w: &mut Work, x: &mut f64, n: i32, htry: f64, eps: f64, hdid: &mut f64, hnext: &mut f64, derivs: &mut dyn FnMut(f64, &[f64], &mut [f64])) -> i32 {
    let mut i: i32;
    let mut err: f64;
    let mut errmax: f64;
    let mut h: f64;
    let mut htemp: f64;
    let mut xnew: f64;
    let xold: f64 = *x;

    // --- set initial stepsize
    h = htry;
    loop {
        // --- take a Runge-Kutta-Cash-Karp step
        rkck(w, xold, n, h, derivs);

        // --- compute scaled maximum error
        errmax = 0.0;
        i = 0;
        while i < n {
            err = (w.yerr[i as usize] / w.yscal[i as usize]).abs();
            if err > errmax {
                errmax = err;
            }
            i += 1;
        }
        errmax /= eps;

        // --- error too large; reduce stepsize & repeat
        if errmax > 1.0 {
            htemp = SAFETY * h * errmax.powf(PSHRNK);
            if h >= 0.0 {
                if htemp > 0.1 * h {
                    h = htemp;
                } else {
                    h = 0.1 * h;
                }
            } else {
                if htemp < 0.1 * h {
                    h = htemp;
                } else {
                    h = 0.1 * h;
                }
            }
            xnew = xold + h;
            if xnew == xold {
                return 2;
            }
            continue;
        }
        // --- step succeeded; compute size of next step
        else {
            if errmax > ERRCON {
                *hnext = SAFETY * h * errmax.powf(PGROW);
            } else {
                *hnext = 5.0 * h;
            }
            *hdid = h;
            *x += *hdid;
            i = 0;
            while i < n {
                w.y[i as usize] = w.ytemp[i as usize];
                i += 1;
            }
            return 0;
        }
    }
}

//----------------------------------------------------------------------
//   Uses the Runge-Kutta-Cash-Karp method to advance y[] at x
//   over stepsize h.
//----------------------------------------------------------------------
fn rkck(w: &mut Work, x: f64, n: i32, h: f64, derivs: &mut dyn FnMut(f64, &[f64], &mut [f64])) {
    let a2 = 0.2;
    let a3 = 0.3;
    let a4 = 0.6;
    let a5 = 1.0;
    let a6 = 0.875;
    let b21 = 0.2;
    let b31 = 3.0 / 40.0;
    let b32 = 9.0 / 40.0;
    let b41 = 0.3;
    let b42 = -0.9;
    let b43 = 1.2;
    let b51 = -11.0 / 54.0;
    let b52 = 2.5;
    let b53 = -70.0 / 27.0;
    let b54 = 35.0 / 27.0;
    let b61 = 1631.0 / 55296.0;
    let b62 = 175.0 / 512.0;
    let b63 = 575.0 / 13824.0;
    let b64 = 44275.0 / 110592.0;
    let b65 = 253.0 / 4096.0;
    let c1 = 37.0 / 378.0;
    let c3 = 250.0 / 621.0;
    let c4 = 125.0 / 594.0;
    let c6 = 512.0 / 1771.0;
    let dc5 = -277.0 / 14336.0;
    let dc1: f64 = c1 - 2825.0 / 27648.0;
    let dc3: f64 = c3 - 18575.0 / 48384.0;
    let dc4: f64 = c4 - 13525.0 / 55296.0;
    let dc6: f64 = c6 - 0.25;
    let nu = n as usize;
    let n2 = nu * 2;
    let n3 = nu * 3;
    let n4 = nu * 4;
    // ak2 = ak, ak3 = ak+n, ak4 = ak+n2, ak5 = ak+n3, ak6 = ak+n4
    let o2 = 0usize;
    let o3 = nu;
    let o4 = n2;
    let o5 = n3;
    let o6 = n4;

    for i in 0..nu {
        w.ytemp[i] = w.y[i] + b21 * h * w.dydx[i];
    }
    derivs(x + a2 * h, &w.ytemp[..nu], &mut w.ak[o2..o2 + nu]);

    for i in 0..nu {
        w.ytemp[i] = w.y[i] + h * (b31 * w.dydx[i] + b32 * w.ak[o2 + i]);
    }
    derivs(x + a3 * h, &w.ytemp[..nu], &mut w.ak[o3..o3 + nu]);

    for i in 0..nu {
        w.ytemp[i] = w.y[i] + h * (b41 * w.dydx[i] + b42 * w.ak[o2 + i] + b43 * w.ak[o3 + i]);
    }
    derivs(x + a4 * h, &w.ytemp[..nu], &mut w.ak[o4..o4 + nu]);

    for i in 0..nu {
        w.ytemp[i] = w.y[i] + h * (b51 * w.dydx[i] + b52 * w.ak[o2 + i] + b53 * w.ak[o3 + i] + b54 * w.ak[o4 + i]);
    }
    derivs(x + a5 * h, &w.ytemp[..nu], &mut w.ak[o5..o5 + nu]);

    for i in 0..nu {
        w.ytemp[i] = w.y[i] + h * (b61 * w.dydx[i] + b62 * w.ak[o2 + i] + b63 * w.ak[o3 + i] + b64 * w.ak[o4 + i] + b65 * w.ak[o5 + i]);
    }
    derivs(x + a6 * h, &w.ytemp[..nu], &mut w.ak[o6..o6 + nu]);

    for i in 0..nu {
        w.ytemp[i] = w.y[i] + h * (c1 * w.dydx[i] + c3 * w.ak[o3 + i] + c4 * w.ak[o4 + i] + c6 * w.ak[o6 + i]);
    }

    for i in 0..nu {
        w.yerr[i] = h * (dc1 * w.dydx[i] + dc3 * w.ak[o3 + i] + dc4 * w.ak[o4 + i] + dc5 * w.ak[o5 + i] + dc6 * w.ak[o6 + i]);
    }
}
