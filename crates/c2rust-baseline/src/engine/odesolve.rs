#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub const MAXSTP: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const TINY: ::core::ffi::c_double = 1.0e-30f64;
pub const SAFETY: ::core::ffi::c_double = 0.9f64;
pub const PGROW: ::core::ffi::c_double = -0.2f64;
pub const PSHRNK: ::core::ffi::c_double = -0.25f64;
pub const ERRCON: ::core::ffi::c_double = 1.89e-4f64;
#[no_mangle]
pub static mut nmax: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut y: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub static mut yscal: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub static mut yerr: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub static mut ytemp: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub static mut dydx: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub static mut ak: *mut ::core::ffi::c_double =
    ::core::ptr::null::<::core::ffi::c_double>() as *mut ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn odesolve_open(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut n5: ::core::ffi::c_int = n * 5 as ::core::ffi::c_int;
    nmax = 0 as ::core::ffi::c_int;
    y = calloc(
        n as size_t,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    ) as *mut ::core::ffi::c_double;
    yscal = calloc(
        n as size_t,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    ) as *mut ::core::ffi::c_double;
    dydx = calloc(
        n as size_t,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    ) as *mut ::core::ffi::c_double;
    yerr = calloc(
        n as size_t,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    ) as *mut ::core::ffi::c_double;
    ytemp = calloc(
        n as size_t,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    ) as *mut ::core::ffi::c_double;
    ak = calloc(
        n5 as size_t,
        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    ) as *mut ::core::ffi::c_double;
    if y.is_null()
        || yscal.is_null()
        || dydx.is_null()
        || yerr.is_null()
        || ytemp.is_null()
        || ak.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    nmax = n;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn odesolve_close() {
    if !y.is_null() {
        free(y as *mut ::core::ffi::c_void);
    }
    y = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if !yscal.is_null() {
        free(yscal as *mut ::core::ffi::c_void);
    }
    yscal = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if !dydx.is_null() {
        free(dydx as *mut ::core::ffi::c_void);
    }
    dydx = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if !yerr.is_null() {
        free(yerr as *mut ::core::ffi::c_void);
    }
    yerr = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if !ytemp.is_null() {
        free(ytemp as *mut ::core::ffi::c_void);
    }
    ytemp = ::core::ptr::null_mut::<::core::ffi::c_double>();
    if !ak.is_null() {
        free(ak as *mut ::core::ffi::c_void);
    }
    ak = ::core::ptr::null_mut::<::core::ffi::c_double>();
    nmax = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn odesolve_integrate(
    mut ystart: *mut ::core::ffi::c_double,
    mut n: ::core::ffi::c_int,
    mut x1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut eps: ::core::ffi::c_double,
    mut h1: ::core::ffi::c_double,
    mut derivs: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
        ) -> (),
    >,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut errcode: ::core::ffi::c_int = 0;
    let mut nstp: ::core::ffi::c_int = 0;
    let mut hdid: ::core::ffi::c_double = 0.;
    let mut hnext: ::core::ffi::c_double = 0.;
    let mut x: ::core::ffi::c_double = x1;
    let mut h: ::core::ffi::c_double = h1;
    if nmax < n {
        return 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *y.offset(i as isize) = *ystart.offset(i as isize);
        i += 1;
    }
    nstp = 1 as ::core::ffi::c_int;
    while nstp <= MAXSTP {
        derivs.expect("non-null function pointer")(x, y, dydx);
        i = 0 as ::core::ffi::c_int;
        while i < n {
            *yscal.offset(i as isize) =
                fabs(*y.offset(i as isize)) + fabs(*dydx.offset(i as isize) * h) + TINY;
            i += 1;
        }
        if (x + h - x2) * (x + h - x1) > 0.0f64 {
            h = x2 - x;
        }
        errcode = rkqs(&raw mut x, n, h, eps, &raw mut hdid, &raw mut hnext, derivs);
        if errcode != 0 {
            break;
        }
        if (x - x2) * (x2 - x1) >= 0.0f64 {
            i = 0 as ::core::ffi::c_int;
            while i < n {
                *ystart.offset(i as isize) = *y.offset(i as isize);
                i += 1;
            }
            return 0 as ::core::ffi::c_int;
        }
        if fabs(hnext) <= 0.0f64 {
            return 2 as ::core::ffi::c_int;
        }
        h = hnext;
        nstp += 1;
    }
    return 3 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rkqs(
    mut x: *mut ::core::ffi::c_double,
    mut n: ::core::ffi::c_int,
    mut htry: ::core::ffi::c_double,
    mut eps: ::core::ffi::c_double,
    mut hdid: *mut ::core::ffi::c_double,
    mut hnext: *mut ::core::ffi::c_double,
    mut derivs: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
        ) -> (),
    >,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_double = 0.;
    let mut errmax: ::core::ffi::c_double = 0.;
    let mut h: ::core::ffi::c_double = 0.;
    let mut htemp: ::core::ffi::c_double = 0.;
    let mut xnew: ::core::ffi::c_double = 0.;
    let mut xold: ::core::ffi::c_double = *x;
    h = htry;
    loop {
        rkck(xold, n, h, derivs);
        errmax = 0.0f64;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            err = fabs(*yerr.offset(i as isize) / *yscal.offset(i as isize));
            if err > errmax {
                errmax = err;
            }
            i += 1;
        }
        errmax /= eps;
        if errmax > 1.0f64 {
            htemp = SAFETY * h * pow(errmax, PSHRNK);
            if h >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                if htemp > 0.1f64 * h {
                    h = htemp;
                } else {
                    h = 0.1f64 * h;
                }
            } else if htemp < 0.1f64 * h {
                h = htemp;
            } else {
                h = 0.1f64 * h;
            }
            xnew = xold + h;
            if xnew == xold {
                return 2 as ::core::ffi::c_int;
            }
        } else {
            if errmax > ERRCON {
                *hnext = SAFETY * h * pow(errmax, PGROW);
            } else {
                *hnext = 5.0f64 * h;
            }
            *hdid = h;
            *x += *hdid;
            i = 0 as ::core::ffi::c_int;
            while i < n {
                *y.offset(i as isize) = *ytemp.offset(i as isize);
                i += 1;
            }
            return 0 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn rkck(
    mut x: ::core::ffi::c_double,
    mut n: ::core::ffi::c_int,
    mut h: ::core::ffi::c_double,
    mut derivs: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
        ) -> (),
    >,
) {
    let mut a2: ::core::ffi::c_double = 0.2f64;
    let mut a3: ::core::ffi::c_double = 0.3f64;
    let mut a4: ::core::ffi::c_double = 0.6f64;
    let mut a5: ::core::ffi::c_double = 1.0f64;
    let mut a6: ::core::ffi::c_double = 0.875f64;
    let mut b21: ::core::ffi::c_double = 0.2f64;
    let mut b31: ::core::ffi::c_double = 3.0f64 / 40.0f64;
    let mut b32: ::core::ffi::c_double = 9.0f64 / 40.0f64;
    let mut b41: ::core::ffi::c_double = 0.3f64;
    let mut b42: ::core::ffi::c_double = -0.9f64;
    let mut b43: ::core::ffi::c_double = 1.2f64;
    let mut b51: ::core::ffi::c_double = -11.0f64 / 54.0f64;
    let mut b52: ::core::ffi::c_double = 2.5f64;
    let mut b53: ::core::ffi::c_double = -70.0f64 / 27.0f64;
    let mut b54: ::core::ffi::c_double = 35.0f64 / 27.0f64;
    let mut b61: ::core::ffi::c_double = 1631.0f64 / 55296.0f64;
    let mut b62: ::core::ffi::c_double = 175.0f64 / 512.0f64;
    let mut b63: ::core::ffi::c_double = 575.0f64 / 13824.0f64;
    let mut b64: ::core::ffi::c_double = 44275.0f64 / 110592.0f64;
    let mut b65: ::core::ffi::c_double = 253.0f64 / 4096.0f64;
    let mut c1: ::core::ffi::c_double = 37.0f64 / 378.0f64;
    let mut c3: ::core::ffi::c_double = 250.0f64 / 621.0f64;
    let mut c4: ::core::ffi::c_double = 125.0f64 / 594.0f64;
    let mut c6: ::core::ffi::c_double = 512.0f64 / 1771.0f64;
    let mut dc5: ::core::ffi::c_double = -277.0f64 / 14336.0f64;
    let mut dc1: ::core::ffi::c_double = c1 - 2825.0f64 / 27648.0f64;
    let mut dc3: ::core::ffi::c_double = c3 - 18575.0f64 / 48384.0f64;
    let mut dc4: ::core::ffi::c_double = c4 - 13525.0f64 / 55296.0f64;
    let mut dc6: ::core::ffi::c_double = c6 - 0.25f64;
    let mut i: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = n * 2 as ::core::ffi::c_int;
    let mut n3: ::core::ffi::c_int = n * 3 as ::core::ffi::c_int;
    let mut n4: ::core::ffi::c_int = n * 4 as ::core::ffi::c_int;
    let mut ak2: *mut ::core::ffi::c_double = ak;
    let mut ak3: *mut ::core::ffi::c_double = ak.offset(n as isize);
    let mut ak4: *mut ::core::ffi::c_double = ak.offset(n2 as isize);
    let mut ak5: *mut ::core::ffi::c_double = ak.offset(n3 as isize);
    let mut ak6: *mut ::core::ffi::c_double = ak.offset(n4 as isize);
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *ytemp.offset(i as isize) = *y.offset(i as isize) + b21 * h * *dydx.offset(i as isize);
        i += 1;
    }
    derivs.expect("non-null function pointer")(x + a2 * h, ytemp, ak2);
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *ytemp.offset(i as isize) = *y.offset(i as isize)
            + h * (b31 * *dydx.offset(i as isize) + b32 * *ak2.offset(i as isize));
        i += 1;
    }
    derivs.expect("non-null function pointer")(x + a3 * h, ytemp, ak3);
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *ytemp.offset(i as isize) = *y.offset(i as isize)
            + h * (b41 * *dydx.offset(i as isize)
                + b42 * *ak2.offset(i as isize)
                + b43 * *ak3.offset(i as isize));
        i += 1;
    }
    derivs.expect("non-null function pointer")(x + a4 * h, ytemp, ak4);
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *ytemp.offset(i as isize) = *y.offset(i as isize)
            + h * (b51 * *dydx.offset(i as isize)
                + b52 * *ak2.offset(i as isize)
                + b53 * *ak3.offset(i as isize)
                + b54 * *ak4.offset(i as isize));
        i += 1;
    }
    derivs.expect("non-null function pointer")(x + a5 * h, ytemp, ak5);
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *ytemp.offset(i as isize) = *y.offset(i as isize)
            + h * (b61 * *dydx.offset(i as isize)
                + b62 * *ak2.offset(i as isize)
                + b63 * *ak3.offset(i as isize)
                + b64 * *ak4.offset(i as isize)
                + b65 * *ak5.offset(i as isize));
        i += 1;
    }
    derivs.expect("non-null function pointer")(x + a6 * h, ytemp, ak6);
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *ytemp.offset(i as isize) = *y.offset(i as isize)
            + h * (c1 * *dydx.offset(i as isize)
                + c3 * *ak3.offset(i as isize)
                + c4 * *ak4.offset(i as isize)
                + c6 * *ak6.offset(i as isize));
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *yerr.offset(i as isize) = h
            * (dc1 * *dydx.offset(i as isize)
                + dc3 * *ak3.offset(i as isize)
                + dc4 * *ak4.offset(i as isize)
                + dc5 * *ak5.offset(i as isize)
                + dc6 * *ak6.offset(i as isize));
        i += 1;
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
