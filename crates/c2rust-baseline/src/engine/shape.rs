#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn table_getFirstEntry(
        table: *mut TTable,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn table_getNextEntry(
        table: *mut TTable,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFile {
    pub name: [::core::ffi::c_char; 260],
    pub mode: ::core::ffi::c_char,
    pub state: ::core::ffi::c_char,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableEntry {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub next: *mut TableEntry,
}
pub type TTableEntry = TableEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTable {
    pub ID: *mut ::core::ffi::c_char,
    pub curveType: ::core::ffi::c_int,
    pub refersTo: ::core::ffi::c_int,
    pub dxMin: ::core::ffi::c_double,
    pub lastDate: ::core::ffi::c_double,
    pub x1: ::core::ffi::c_double,
    pub x2: ::core::ffi::c_double,
    pub y1: ::core::ffi::c_double,
    pub y2: ::core::ffi::c_double,
    pub firstEntry: *mut TTableEntry,
    pub lastEntry: *mut TTableEntry,
    pub thisEntry: *mut TTableEntry,
    pub file: TFile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TShape {
    pub curve: ::core::ffi::c_int,
    pub nTbl: ::core::ffi::c_int,
    pub aFull: ::core::ffi::c_double,
    pub rFull: ::core::ffi::c_double,
    pub wMax: ::core::ffi::c_double,
    pub sMax: ::core::ffi::c_double,
    pub aMax: ::core::ffi::c_double,
    pub areaTbl: [::core::ffi::c_double; 51],
    pub hradTbl: [::core::ffi::c_double; 51],
    pub widthTbl: [::core::ffi::c_double; 51],
}
pub const N_SHAPE_TBL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
static mut Atotal: ::core::ffi::c_double = 0.;
static mut Ptotal: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn shape_validate(
    mut shape: *mut TShape,
    mut curve: *mut TTable,
) -> ::core::ffi::c_int {
    if computeShapeTables(shape, curve) == 0 {
        return FALSE;
    }
    if normalizeShapeTables(shape) == 0 {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn computeShapeTables(
    mut shape: *mut TShape,
    mut curve: *mut TTable,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut w1: ::core::ffi::c_double = 0.;
    let mut w2: ::core::ffi::c_double = 0.;
    let mut yLast: ::core::ffi::c_double = 0.;
    let mut wLast: ::core::ffi::c_double = 0.;
    let mut wMax: ::core::ffi::c_double = 0.;
    if table_getFirstEntry(curve, &raw mut y1, &raw mut w1) == 0 {
        return FALSE;
    }
    if y1 < 0.0f64 || y1 >= 1.0f64 || w1 < 0.0f64 {
        return FALSE;
    }
    wMax = w1;
    if y1 != 0.0f64 {
        y2 = y1;
        w2 = w1;
        y1 = 0.0f64;
        w1 = 0.0f64;
    } else {
        if table_getNextEntry(curve, &raw mut y2, &raw mut w2) == 0 {
            return FALSE;
        }
        if y2 < y1 || w2 < 0.0f64 {
            return FALSE;
        }
        if y2 > 1.0f64 {
            y2 = 1.0f64;
        }
        if w2 > wMax {
            wMax = w2;
        }
    }
    (*shape).nTbl = N_SHAPE_TBL;
    n = (*shape).nTbl - 1 as ::core::ffi::c_int;
    dy = 1.0f64 / n as ::core::ffi::c_double;
    (*shape).areaTbl[0 as ::core::ffi::c_int as usize] = 0.0f64;
    (*shape).hradTbl[0 as ::core::ffi::c_int as usize] = 0.0f64;
    (*shape).widthTbl[0 as ::core::ffi::c_int as usize] = w1;
    Ptotal = w1;
    Atotal = 0.0f64;
    y = 0.0f64;
    w = w1;
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        yLast = y;
        wLast = w;
        y = y + dy;
        if fabs(y - 1.0f64) < TINY {
            y = 1.0f64;
        }
        if y > y2 {
            if getNextInterval(
                curve,
                y,
                yLast,
                wLast,
                &raw mut y1,
                &raw mut y2,
                &raw mut w1,
                &raw mut w2,
                &raw mut wMax,
            ) == 0
            {
                return FALSE;
            }
            yLast = y1;
            wLast = w1;
        }
        w = getWidth(y, y1, y2, w1, w2);
        Atotal += getArea(y, w, yLast, wLast);
        Ptotal += getPerim(y, w, yLast, wLast);
        if y == 1.0f64 {
            Ptotal += w2;
        }
        (*shape).widthTbl[i as usize] = w;
        (*shape).areaTbl[i as usize] = Atotal;
        if Ptotal > 0.0f64 {
            (*shape).hradTbl[i as usize] = Atotal / Ptotal;
        } else {
            (*shape).hradTbl[i as usize] = 0.0f64;
        }
        i += 1;
    }
    (*shape).aFull = (*shape).areaTbl[n as usize];
    (*shape).rFull = (*shape).hradTbl[n as usize];
    (*shape).wMax = wMax;
    getSmax(shape);
    return TRUE;
}
unsafe extern "C" fn getSmax(mut shape: *mut TShape) {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = (*shape).nTbl - 1 as ::core::ffi::c_int;
    let mut sf: ::core::ffi::c_double = 0.;
    (*shape).sMax = 0.0f64;
    (*shape).aMax = 0.0f64;
    i = 1 as ::core::ffi::c_int;
    while i <= n {
        sf = (*shape).areaTbl[i as usize] * pow((*shape).hradTbl[i as usize], 2.0f64 / 3.0f64);
        if sf > (*shape).sMax {
            (*shape).sMax = sf;
            (*shape).aMax = (*shape).areaTbl[i as usize];
        }
        i += 1;
    }
}
unsafe extern "C" fn normalizeShapeTables(mut shape: *mut TShape) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = (*shape).nTbl - 1 as ::core::ffi::c_int;
    let mut aFull: ::core::ffi::c_double = (*shape).aFull;
    let mut rFull: ::core::ffi::c_double = (*shape).rFull;
    let mut wMax: ::core::ffi::c_double = (*shape).wMax;
    if aFull == 0.0f64 || rFull == 0.0f64 || wMax == 0.0f64 {
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int;
    while i <= n {
        (*shape).areaTbl[i as usize] /= aFull;
        (*shape).hradTbl[i as usize] /= rFull;
        (*shape).widthTbl[i as usize] /= wMax;
        i += 1;
    }
    return TRUE;
}
unsafe extern "C" fn getNextInterval(
    mut curve: *mut TTable,
    mut y: ::core::ffi::c_double,
    mut yLast: ::core::ffi::c_double,
    mut wLast: ::core::ffi::c_double,
    mut y1: *mut ::core::ffi::c_double,
    mut y2: *mut ::core::ffi::c_double,
    mut w1: *mut ::core::ffi::c_double,
    mut w2: *mut ::core::ffi::c_double,
    mut wMax: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    while y > *y2 {
        if *y2 > yLast {
            Atotal += getArea(*y2, *w2, yLast, wLast);
            Ptotal += getPerim(*y2, *w2, yLast, wLast);
            yLast = *y2;
            wLast = *w2;
        }
        *y1 = *y2;
        *w1 = *w2;
        if table_getNextEntry(curve, y2, w2) == 0 {
            *y2 = 1.0f64;
            return TRUE;
        }
        if *w2 > *wMax {
            *wMax = *w2;
        }
        if *y2 < *y1 || *w2 < 0.0f64 {
            return FALSE;
        }
        if *y2 > 1.0f64 {
            *y2 = 1.0f64;
        }
    }
    return TRUE;
}
unsafe extern "C" fn getWidth(
    mut y: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    mut w1: ::core::ffi::c_double,
    mut w2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y2 == y1 {
        return w2;
    }
    return w1 + (y - y1) / (y2 - y1) * (w2 - w1);
}
unsafe extern "C" fn getArea(
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut w1: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut wMin: ::core::ffi::c_double = 0.;
    let mut wMax: ::core::ffi::c_double = 0.;
    if w > w1 {
        wMin = w1;
        wMax = w;
    } else {
        wMin = w;
        wMax = w1;
    }
    return (wMin + (wMax - wMin) / 2.0f64) * (y - y1);
}
unsafe extern "C" fn getPerim(
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut w1: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut dy: ::core::ffi::c_double = y - y1;
    let mut dw: ::core::ffi::c_double = fabs(w - w1) / 2.0f64;
    return 2.0f64 * sqrt(dy * dy + dw * dw);
}
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TINY: ::core::ffi::c_double = 1.0E-6f64;

#[repr(C)]
pub struct _IO_wide_data {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _opaque: [u8; 0],
}
