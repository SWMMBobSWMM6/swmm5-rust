//! # swmm5-rust C ABI
//!
//! EPA SWMM 5.2.4's `swmm_*` entry points over the safe engine, plus a
//! handle-based twin of each (`swmm5rs_*`) so one process or one WebAssembly
//! instance can hold many models. This file is the only `unsafe` code in
//! swmm5-rust: it turns C pointers from the caller into Rust slices and
//! back. Every call into the engine is safe Rust.
use std::ffi::{c_char, c_int, CStr};
use std::sync::Mutex;
use swmm5_rust::swmm5 as api;
use swmm5_rust::Project;

static MODELS: Mutex<Vec<Option<Box<Project>>>> = Mutex::new(Vec::new());

fn bytes<'a>(p: *const c_char) -> &'a [u8] {
    if p.is_null() {
        return b"";
    }
    // SAFETY: the caller passes a NUL-terminated string that outlives the call.
    unsafe { CStr::from_ptr(p) }.to_bytes()
}

fn with<R>(h: c_int, f: impl FnOnce(&mut Project) -> R, dflt: R) -> R {
    let mut m = MODELS.lock().unwrap_or_else(|e| e.into_inner());
    match m.get_mut(h as usize).and_then(|x| x.as_mut()) {
        Some(p) => f(p),
        None => dflt,
    }
}

fn copy_out(buf: *mut c_char, len: c_int, msg: &[u8]) {
    if buf.is_null() || len <= 0 {
        return;
    }
    let n = msg.len().min(len as usize);
    // SAFETY: the caller provides at least len+1 writable bytes (EPA's contract).
    unsafe {
        std::ptr::copy_nonoverlapping(msg.as_ptr(), buf as *mut u8, n);
        *buf.add(n) = 0;
    }
}

/// Create a model; returns its handle (>= 0).
#[no_mangle]
pub extern "C" fn swmm5rs_create() -> c_int {
    let mut m = MODELS.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(i) = m.iter().position(|x| x.is_none()) {
        m[i] = Some(Project::new());
        return i as c_int;
    }
    m.push(Some(Project::new()));
    (m.len() - 1) as c_int
}

/// Close (if open) and free a model.
#[no_mangle]
pub extern "C" fn swmm5rs_destroy(h: c_int) {
    let mut m = MODELS.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(slot) = m.get_mut(h as usize) {
        if let Some(p) = slot.as_mut() {
            api::swmm_close(p);
        }
        *slot = None;
    }
}

/// Number of live models.
#[no_mangle]
pub extern "C" fn swmm5rs_count() -> c_int {
    let m = MODELS.lock().unwrap_or_else(|e| e.into_inner());
    m.iter().filter(|x| x.is_some()).count() as c_int
}

#[no_mangle]
pub extern "C" fn swmm5rs_run(h: c_int, f1: *const c_char, f2: *const c_char, f3: *const c_char) -> c_int {
    let (a, b, c) = (bytes(f1), bytes(f2), bytes(f3));
    with(h, |p| api::swmm_run(p, a, b, c), -1)
}
#[no_mangle]
pub extern "C" fn swmm5rs_open(h: c_int, f1: *const c_char, f2: *const c_char, f3: *const c_char) -> c_int {
    let (a, b, c) = (bytes(f1), bytes(f2), bytes(f3));
    with(h, |p| api::swmm_open(p, a, b, c), -1)
}
#[no_mangle]
pub extern "C" fn swmm5rs_start(h: c_int, save: c_int) -> c_int {
    with(h, |p| api::swmm_start(p, save), -1)
}
#[no_mangle]
pub extern "C" fn swmm5rs_step(h: c_int, elapsed: *mut f64) -> c_int {
    let mut t = 0.0;
    let rc = with(h, |p| api::swmm_step(p, &mut t), -1);
    if !elapsed.is_null() {
        // SAFETY: caller passes a valid pointer to a double.
        unsafe { *elapsed = t };
    }
    rc
}
#[no_mangle]
pub extern "C" fn swmm5rs_end(h: c_int) -> c_int {
    with(h, api::swmm_end, -1)
}
#[no_mangle]
pub extern "C" fn swmm5rs_report(h: c_int) -> c_int {
    with(h, api::swmm_report, -1)
}
#[no_mangle]
pub extern "C" fn swmm5rs_close(h: c_int) -> c_int {
    with(h, api::swmm_close, -1)
}
#[no_mangle]
pub extern "C" fn swmm5rs_getWarnings(h: c_int) -> c_int {
    with(h, api::swmm_getWarnings, 0)
}
#[no_mangle]
pub extern "C" fn swmm5rs_getError(h: c_int, buf: *mut c_char, len: c_int) -> c_int {
    let mut msg = Vec::new();
    let rc = with(h, |p| api::swmm_getError(p, &mut msg, len), 0);
    copy_out(buf, len, &msg);
    rc
}
#[no_mangle]
pub extern "C" fn swmm5rs_getMassBalErr(h: c_int, runoff: *mut f32, flow: *mut f32, qual: *mut f32) -> c_int {
    let (mut a, mut b, mut c) = (0f32, 0f32, 0f32);
    let rc = with(h, |p| api::swmm_getMassBalErr(p, &mut a, &mut b, &mut c), -1);
    // SAFETY: caller passes valid pointers (or null).
    unsafe {
        if !runoff.is_null() { *runoff = a; }
        if !flow.is_null() { *flow = b; }
        if !qual.is_null() { *qual = c; }
    }
    rc
}

// ---- EPA's handle-less API: one default model, as EPA's DLL has ----
static DEFAULT: Mutex<Option<c_int>> = Mutex::new(None);
fn dflt() -> c_int {
    let mut d = DEFAULT.lock().unwrap_or_else(|e| e.into_inner());
    if d.is_none() {
        *d = Some(swmm5rs_create());
    }
    d.unwrap()
}
#[no_mangle]
pub extern "C" fn swmm_run(f1: *const c_char, f2: *const c_char, f3: *const c_char) -> c_int {
    swmm5rs_run(dflt(), f1, f2, f3)
}
#[no_mangle]
pub extern "C" fn swmm_open(f1: *const c_char, f2: *const c_char, f3: *const c_char) -> c_int {
    swmm5rs_open(dflt(), f1, f2, f3)
}
#[no_mangle]
pub extern "C" fn swmm_start(save: c_int) -> c_int {
    swmm5rs_start(dflt(), save)
}
#[no_mangle]
pub extern "C" fn swmm_step(elapsed: *mut f64) -> c_int {
    swmm5rs_step(dflt(), elapsed)
}
#[no_mangle]
pub extern "C" fn swmm_end() -> c_int {
    swmm5rs_end(dflt())
}
#[no_mangle]
pub extern "C" fn swmm_report() -> c_int {
    swmm5rs_report(dflt())
}
#[no_mangle]
pub extern "C" fn swmm_close() -> c_int {
    swmm5rs_close(dflt())
}
#[no_mangle]
pub extern "C" fn swmm_getWarnings() -> c_int {
    swmm5rs_getWarnings(dflt())
}
#[no_mangle]
pub extern "C" fn swmm_getError(buf: *mut c_char, len: c_int) -> c_int {
    swmm5rs_getError(dflt(), buf, len)
}
#[no_mangle]
pub extern "C" fn swmm_getMassBalErr(runoff: *mut f32, flow: *mut f32, qual: *mut f32) -> c_int {
    swmm5rs_getMassBalErr(dflt(), runoff, flow, qual)
}
#[no_mangle]
pub extern "C" fn swmm_getVersion() -> c_int {
    api::swmm_getVersion()
}

// ---- memory for strings passed in from JavaScript ----
/// `malloc` for the host: a block with its size stored in front.
#[no_mangle]
pub extern "C" fn swmm5rs_malloc(n: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(n + 8, 8).unwrap();
    // SAFETY: non-zero size layout; the size is kept for free.
    unsafe {
        let p = std::alloc::alloc(layout);
        if p.is_null() {
            return p;
        }
        *(p as *mut usize) = n;
        p.add(8)
    }
}
/// `free` for blocks from swmm5rs_malloc.
#[no_mangle]
pub extern "C" fn swmm5rs_free(p: *mut u8) {
    if p.is_null() {
        return;
    }
    // SAFETY: p came from swmm5rs_malloc.
    unsafe {
        let base = p.sub(8);
        let n = *(base as *const usize);
        std::alloc::dealloc(base, std::alloc::Layout::from_size_align(n + 8, 8).unwrap());
    }
}
