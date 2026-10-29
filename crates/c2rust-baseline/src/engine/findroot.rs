#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub const MAXIT: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn findroot_Newton(
    mut x1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut rts: *mut ::core::ffi::c_double,
    mut xacc: ::core::ffi::c_double,
    mut func: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut df: ::core::ffi::c_double = 0.;
    let mut dx: ::core::ffi::c_double = 0.;
    let mut dxold: ::core::ffi::c_double = 0.;
    let mut f: ::core::ffi::c_double = 0.;
    let mut x: ::core::ffi::c_double = 0.;
    let mut temp: ::core::ffi::c_double = 0.;
    let mut xhi: ::core::ffi::c_double = 0.;
    let mut xlo: ::core::ffi::c_double = 0.;
    x = *rts;
    xlo = x1;
    xhi = x2;
    dxold = fabs(x2 - x1);
    dx = dxold;
    func.expect("non-null function pointer")(x, &raw mut f, &raw mut df, p);
    n += 1;
    j = 1 as ::core::ffi::c_int;
    while j <= MAXIT {
        if ((x - xhi) * df - f) * ((x - xlo) * df - f) >= 0.0f64
            || fabs(2.0f64 * f) > fabs(dxold * df)
        {
            dxold = dx;
            dx = 0.5f64 * (xhi - xlo);
            x = xlo + dx;
            if xlo == x {
                break;
            }
        } else {
            dxold = dx;
            dx = f / df;
            temp = x;
            x -= dx;
            if temp == x {
                break;
            }
        }
        if fabs(dx) < xacc {
            break;
        }
        func.expect("non-null function pointer")(x, &raw mut f, &raw mut df, p);
        n += 1;
        if f < 0.0f64 {
            xlo = x;
        } else {
            xhi = x;
        }
        j += 1;
    }
    *rts = x;
    if n <= MAXIT {
        return n;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn findroot_Ridder(
    mut x1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut xacc: ::core::ffi::c_double,
    mut func: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_double,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_double,
    >,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    let mut j: ::core::ffi::c_int = 0;
    let mut ans: ::core::ffi::c_double = 0.;
    let mut fhi: ::core::ffi::c_double = 0.;
    let mut flo: ::core::ffi::c_double = 0.;
    let mut fm: ::core::ffi::c_double = 0.;
    let mut fnew: ::core::ffi::c_double = 0.;
    let mut s: ::core::ffi::c_double = 0.;
    let mut xhi: ::core::ffi::c_double = 0.;
    let mut xlo: ::core::ffi::c_double = 0.;
    let mut xm: ::core::ffi::c_double = 0.;
    let mut xnew: ::core::ffi::c_double = 0.;
    flo = func.expect("non-null function pointer")(x1, p);
    fhi = func.expect("non-null function pointer")(x2, p);
    if flo == 0.0f64 {
        return x1;
    }
    if fhi == 0.0f64 {
        return x2;
    }
    ans = 0.5f64 * (x1 + x2);
    if flo > 0.0f64 && fhi < 0.0f64 || flo < 0.0f64 && fhi > 0.0f64 {
        xlo = x1;
        xhi = x2;
        j = 1 as ::core::ffi::c_int;
        while j <= MAXIT {
            xm = 0.5f64 * (xlo + xhi);
            fm = func.expect("non-null function pointer")(xm, p);
            s = sqrt(fm * fm - flo * fhi);
            if s == 0.0f64 {
                return ans;
            }
            xnew = xm + (xm - xlo) * ((if flo >= fhi { 1.0f64 } else { -1.0f64 }) * fm / s);
            if fabs(xnew - ans) <= xacc {
                break;
            }
            ans = xnew;
            fnew = func.expect("non-null function pointer")(ans, p);
            if (if fnew >= 0.0f64 { fabs(fm) } else { -fabs(fm) }) != fm {
                xlo = xm;
                flo = fm;
                xhi = ans;
                fhi = fnew;
            } else if (if fnew >= 0.0f64 {
                fabs(flo)
            } else {
                -fabs(flo)
            }) != flo
            {
                xhi = ans;
                fhi = fnew;
            } else if (if fnew >= 0.0f64 {
                fabs(fhi)
            } else {
                -fabs(fhi)
            }) != fhi
            {
                xlo = ans;
                flo = fnew;
            } else {
                return ans;
            }
            if fabs(xhi - xlo) <= xacc {
                return ans;
            }
            j += 1;
        }
        return ans;
    }
    return -1.0e20f64;
}
