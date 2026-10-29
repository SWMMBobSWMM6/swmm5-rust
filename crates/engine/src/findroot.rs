//! findroot.c of EPA SWMM 5.2.4, in safe Rust.
//-----------------------------------------------------------------------------
//   findroot.c
//
//   Finds solution of func(x) = 0 using either the Newton-Raphson
//   method or Ridder's Method.
//   Based on code from Numerical Recipes in C (Cambridge University
//   Press, 1992).
//
//   Date:     11/19/13
//   Author:   L. Rossman
//-----------------------------------------------------------------------------
//
// The C functions take a function pointer plus a `void* p` context; here the
// callback is a closure that captures whatever `p` carried.

use crate::prelude::*;

/// The file-scope variables of findroot.c.
#[derive(Default)]
pub struct FindrootState {}

// #define SIGN(a,b) ((b) >= 0.0 ? fabs(a) : -fabs(a))   (macros::SIGN)
const MAXIT: i32 = 60;

/// C: `int findroot_Newton(double x1, double x2, double* rts, double xacc,
///      void (*func) (double x, double* f, double* df, void* p), void* p)`
//
//  Using a combination of Newton-Raphson and bisection, find the root of a
//  function func bracketed between x1 and x2. The root, returned in rts,
//  will be refined until its accuracy is known within +/-xacc. func is a
//  user-supplied routine, that returns both the function value and the first
//  derivative of the function. p is a pointer to any auxilary data structure
//  that func may require. It can be NULL if not needed. The function returns
//  the number of function evaluations used or 0 if the maximum allowed
//  iterations were exceeded.
//
// NOTES:
// 1. The calling program must insure that the signs of func(x1) and func(x2)
//    are not the same, otherwise x1 and x2 do not bracket the root.
// 2. If func(x1) > func(x2) then the order of x1 and x2 should be
//    switched in the call to Newton.
//
pub fn findroot_Newton(x1: f64, x2: f64, rts: &mut f64, xacc: f64, func: &mut dyn FnMut(f64, &mut f64, &mut f64)) -> i32 {
    let mut j: i32;
    let mut n: i32 = 0;
    // NOTE: f and df are written by func before they are read; C leaves
    // them uninitialized, so they start at 0 here only to satisfy Rust.
    let mut df: f64 = 0.0;
    let mut dx: f64;
    let mut dxold: f64;
    let mut f: f64 = 0.0;
    let mut x: f64;
    let mut temp: f64;
    let mut xhi: f64;
    let mut xlo: f64;

    // Initialize the "stepsize before last" and the last step.
    x = *rts;
    xlo = x1;
    xhi = x2;
    dxold = (x2 - x1).abs();
    dx = dxold;
    func(x, &mut f, &mut df);
    n += 1;

    // Loop over allowed iterations.
    j = 1;
    while j <= MAXIT {
        // Bisect if Newton out of range or not decreasing fast enough.
        if (((x - xhi) * df - f) * ((x - xlo) * df - f) >= 0.0) || ((2.0 * f).abs() > (dxold * df).abs()) {
            dxold = dx;
            dx = 0.5 * (xhi - xlo);
            x = xlo + dx;
            if xlo == x {
                break;
            }
        }
        // Newton step acceptable. Take it.
        else {
            dxold = dx;
            dx = f / df;
            temp = x;
            x -= dx;
            if temp == x {
                break;
            }
        }

        // Convergence criterion.
        if dx.abs() < xacc {
            break;
        }

        // Evaluate function. Maintain bracket on the root.
        func(x, &mut f, &mut df);
        n += 1;
        if f < 0.0 {
            xlo = x;
        } else {
            xhi = x;
        }
        j += 1;
    }
    *rts = x;
    if n <= MAXIT {
        n
    } else {
        0
    }
}

/// C: `double findroot_Ridder(double x1, double x2, double xacc,
///      double (*func)(double, void* p), void* p)`
pub fn findroot_Ridder(x1: f64, x2: f64, xacc: f64, func: &mut dyn FnMut(f64) -> f64) -> f64 {
    let mut j: i32;
    let mut ans: f64;
    let mut fhi: f64;
    let mut flo: f64;
    let mut fm: f64;
    let mut fnew: f64;
    let mut s: f64;
    let mut xhi: f64;
    let mut xlo: f64;
    let mut xm: f64;
    let mut xnew: f64;

    flo = func(x1);
    fhi = func(x2);
    if flo == 0.0 {
        return x1;
    }
    if fhi == 0.0 {
        return x2;
    }
    ans = 0.5 * (x1 + x2);
    if (flo > 0.0 && fhi < 0.0) || (flo < 0.0 && fhi > 0.0) {
        xlo = x1;
        xhi = x2;
        j = 1;
        while j <= MAXIT {
            xm = 0.5 * (xlo + xhi);
            fm = func(xm);
            s = (fm * fm - flo * fhi).sqrt();
            if s == 0.0 {
                return ans;
            }
            xnew = xm + (xm - xlo) * ((if flo >= fhi { 1.0 } else { -1.0 }) * fm / s);
            if (xnew - ans).abs() <= xacc {
                break;
            }
            ans = xnew;
            fnew = func(ans);
            if SIGN(fm, fnew) != fm {
                xlo = xm;
                flo = fm;
                xhi = ans;
                fhi = fnew;
            } else if SIGN(flo, fnew) != flo {
                xhi = ans;
                fhi = fnew;
            } else if SIGN(fhi, fnew) != fhi {
                xlo = ans;
                flo = fnew;
            } else {
                return ans;
            }
            if (xhi - xlo).abs() <= xacc {
                return ans;
            }
            j += 1;
        }
        return ans;
    }
    -1.0e20
}
