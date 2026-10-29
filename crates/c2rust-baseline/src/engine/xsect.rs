#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn asin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static mut Curve: *mut TTable;
    static mut Transect: *mut TTransect;
    static mut Street: *mut TStreet;
    static mut Shape: *mut TShape;
    fn findroot_Newton(
        x1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        rts: *mut ::core::ffi::c_double,
        xacc: ::core::ffi::c_double,
        func: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        p: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn findroot_Ridder(
        x1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        xacc: ::core::ffi::c_double,
        func: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_double,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_double,
        >,
        p: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_double;
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
pub type XsectType = ::core::ffi::c_uint;
pub const STREET_XSECT: XsectType = 25;
pub const FORCE_MAIN: XsectType = 24;
pub const CUSTOM: XsectType = 23;
pub const IRREGULAR: XsectType = 22;
pub const SEMICIRCULAR: XsectType = 21;
pub const BASKETHANDLE: XsectType = 20;
pub const SEMIELLIPTICAL: XsectType = 19;
pub const CATENARY: XsectType = 18;
pub const GOTHIC: XsectType = 17;
pub const HORSESHOE: XsectType = 16;
pub const EGGSHAPED: XsectType = 15;
pub const ARCH: XsectType = 14;
pub const VERT_ELLIPSE: XsectType = 13;
pub const HORIZ_ELLIPSE: XsectType = 12;
pub const MOD_BASKET: XsectType = 11;
pub const RECT_ROUND: XsectType = 10;
pub const RECT_TRIANG: XsectType = 9;
pub const POWERFUNC: XsectType = 8;
pub const PARABOLIC: XsectType = 7;
pub const TRIANGULAR: XsectType = 6;
pub const TRAPEZOIDAL: XsectType = 5;
pub const RECT_OPEN: XsectType = 4;
pub const RECT_CLOSED: XsectType = 3;
pub const FILLED_CIRCULAR: XsectType = 2;
pub const CIRCULAR: XsectType = 1;
pub const DUMMY: XsectType = 0;
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
pub struct TXsect {
    pub type_0: ::core::ffi::c_int,
    pub culvertCode: ::core::ffi::c_int,
    pub transect: ::core::ffi::c_int,
    pub yFull: ::core::ffi::c_double,
    pub wMax: ::core::ffi::c_double,
    pub ywMax: ::core::ffi::c_double,
    pub aFull: ::core::ffi::c_double,
    pub rFull: ::core::ffi::c_double,
    pub sFull: ::core::ffi::c_double,
    pub sMax: ::core::ffi::c_double,
    pub yBot: ::core::ffi::c_double,
    pub aBot: ::core::ffi::c_double,
    pub sBot: ::core::ffi::c_double,
    pub rBot: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTransect {
    pub ID: *mut ::core::ffi::c_char,
    pub yFull: ::core::ffi::c_double,
    pub aFull: ::core::ffi::c_double,
    pub rFull: ::core::ffi::c_double,
    pub wMax: ::core::ffi::c_double,
    pub ywMax: ::core::ffi::c_double,
    pub sMax: ::core::ffi::c_double,
    pub aMax: ::core::ffi::c_double,
    pub lengthFactor: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub areaTbl: [::core::ffi::c_double; 51],
    pub hradTbl: [::core::ffi::c_double; 51],
    pub widthTbl: [::core::ffi::c_double; 51],
    pub nTbl: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TStreet {
    pub ID: *mut ::core::ffi::c_char,
    pub sides: ::core::ffi::c_int,
    pub slope: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub curbHeight: ::core::ffi::c_double,
    pub gutterDepression: ::core::ffi::c_double,
    pub gutterWidth: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub backSlope: ::core::ffi::c_double,
    pub backWidth: ::core::ffi::c_double,
    pub backRoughness: ::core::ffi::c_double,
    pub transect: TTransect,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TXsectStar {
    pub s: ::core::ffi::c_double,
    pub qc: ::core::ffi::c_double,
    pub xsect: *mut TXsect,
}
pub const N_TRANSECT_TBL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const N_SHAPE_TBL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const RECT_ALFMAX: ::core::ffi::c_double = 0.97f64;
pub const RECT_TRIANG_ALFMAX: ::core::ffi::c_double = 0.98f64;
pub const RECT_ROUND_ALFMAX: ::core::ffi::c_double = 0.98f64;
#[no_mangle]
pub static mut Amax: [::core::ffi::c_double; 26] = [
    1.0f64, 0.9756f64, 0.9756f64, 0.97f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 0.98f64,
    0.98f64, 0.96f64, 0.96f64, 0.96f64, 0.92f64, 0.96f64, 0.96f64, 0.96f64, 0.98f64, 0.98f64,
    0.96f64, 0.96f64, 1.0f64, 0.96f64, 0.9756f64, 1.0f64,
];
#[no_mangle]
pub unsafe extern "C" fn xsect_isOpen(mut type_0: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if Amax[type_0 as usize] >= 1.0f64 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_setParams(
    mut xsect: *mut TXsect,
    mut type_0: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_double,
    mut ucf: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut aMax: ::core::ffi::c_double = 0.;
    let mut theta: ::core::ffi::c_double = 0.;
    if type_0 != DUMMY as ::core::ffi::c_int
        && *p.offset(0 as ::core::ffi::c_int as isize) <= 0.0f64
    {
        return FALSE;
    }
    (*xsect).type_0 = type_0;
    match (*xsect).type_0 {
        0 => {
            (*xsect).yFull = TINY;
            (*xsect).wMax = TINY;
            (*xsect).aFull = TINY;
            (*xsect).rFull = TINY;
            (*xsect).sFull = TINY;
            (*xsect).sMax = TINY;
        }
        1 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = (*xsect).yFull;
            (*xsect).aFull = PI / 4.0f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.2500f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.08f64 * (*xsect).sFull;
            (*xsect).ywMax = 0.5f64 * (*xsect).yFull;
        }
        24 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = (*xsect).yFull;
            (*xsect).aFull = PI / 4.0f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.2500f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 0.63f64);
            (*xsect).sMax = 1.06949f64 * (*xsect).sFull;
            (*xsect).ywMax = 0.5f64 * (*xsect).yFull;
            (*xsect).rBot = *p.offset(1 as ::core::ffi::c_int as isize);
        }
        2 => {
            if *p.offset(1 as ::core::ffi::c_int as isize)
                >= *p.offset(0 as ::core::ffi::c_int as isize)
            {
                return FALSE;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = (*xsect).yFull;
            (*xsect).aFull = PI / 4.0f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.2500f64 * (*xsect).yFull;
            (*xsect).yBot = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aBot = circ_getAofY(xsect, (*xsect).yBot);
            (*xsect).sBot = xsect_getWofY(xsect, (*xsect).yBot);
            (*xsect).rBot = (*xsect).aBot
                / ((*xsect).rFull
                    * lookup(
                        (*xsect).yBot / (*xsect).yFull,
                        &raw mut R_Circ as *mut ::core::ffi::c_double,
                        N_R_Circ,
                    ));
            (*xsect).aFull -= (*xsect).aBot;
            (*xsect).rFull = (*xsect).aFull / (PI * (*xsect).yFull - (*xsect).rBot + (*xsect).sBot);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.08f64 * (*xsect).sFull;
            (*xsect).yFull -= (*xsect).yBot;
            (*xsect).ywMax = 0.5f64 * (*xsect).yFull;
        }
        15 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aFull = 0.5105f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.1931f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.065f64 * (*xsect).sFull;
            (*xsect).wMax = 2.0f64 / 3.0f64 * (*xsect).yFull;
            (*xsect).ywMax = 0.64f64 * (*xsect).yFull;
        }
        16 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aFull = 0.8293f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.2538f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.077f64 * (*xsect).sFull;
            (*xsect).wMax = 1.0f64 * (*xsect).yFull;
            (*xsect).ywMax = 0.5f64 * (*xsect).yFull;
        }
        17 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aFull = 0.6554f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.2269f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.065f64 * (*xsect).sFull;
            (*xsect).wMax = 0.84f64 * (*xsect).yFull;
            (*xsect).ywMax = 0.45f64 * (*xsect).yFull;
        }
        18 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aFull = 0.70277f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.23172f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.05f64 * (*xsect).sFull;
            (*xsect).wMax = 0.9f64 * (*xsect).yFull;
            (*xsect).ywMax = 0.25f64 * (*xsect).yFull;
        }
        19 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aFull = 0.785f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.242f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.045f64 * (*xsect).sFull;
            (*xsect).wMax = 1.0f64 * (*xsect).yFull;
            (*xsect).ywMax = 0.15f64 * (*xsect).yFull;
        }
        20 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aFull = 0.7862f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.2464f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.06078f64 * (*xsect).sFull;
            (*xsect).wMax = 0.944f64 * (*xsect).yFull;
            (*xsect).ywMax = 0.2f64 * (*xsect).yFull;
        }
        21 => {
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aFull = 1.2697f64 * (*xsect).yFull * (*xsect).yFull;
            (*xsect).rFull = 0.2946f64 * (*xsect).yFull;
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = 1.06637f64 * (*xsect).sFull;
            (*xsect).wMax = 1.64f64 * (*xsect).yFull;
            (*xsect).ywMax = 0.15f64 * (*xsect).yFull;
        }
        3 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64 {
                return FALSE;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).aFull = (*xsect).yFull * (*xsect).wMax;
            (*xsect).rFull = (*xsect).aFull / (2.0f64 * ((*xsect).yFull + (*xsect).wMax));
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            aMax = RECT_ALFMAX * (*xsect).aFull;
            (*xsect).sMax = aMax * pow(rect_closed_getRofA(xsect, aMax), 2.0f64 / 3.0f64);
            (*xsect).ywMax = (*xsect).yFull;
        }
        4 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64 {
                return FALSE;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            if *p.offset(2 as ::core::ffi::c_int as isize) < 0.0f64
                || *p.offset(2 as ::core::ffi::c_int as isize) > 2.0f64
            {
                return FALSE;
            }
            (*xsect).sBot = *p.offset(2 as ::core::ffi::c_int as isize);
            (*xsect).aFull = (*xsect).yFull * (*xsect).wMax;
            (*xsect).rFull =
                (*xsect).aFull / ((2.0f64 - (*xsect).sBot) * (*xsect).yFull + (*xsect).wMax);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = (*xsect).sFull;
            (*xsect).ywMax = (*xsect).yFull;
        }
        9 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64
                || *p.offset(2 as ::core::ffi::c_int as isize) <= 0.0f64
            {
                return FALSE;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).yBot = *p.offset(2 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).ywMax = (*xsect).yFull;
            (*xsect).aBot = (*xsect).yBot * (*xsect).wMax / 2.0f64;
            (*xsect).sBot = (*xsect).wMax / (*xsect).yBot / 2.0f64;
            (*xsect).rBot = sqrt(1.0f64 + (*xsect).sBot * (*xsect).sBot);
            (*xsect).aFull = (*xsect).wMax * ((*xsect).yFull - (*xsect).yBot / 2.0f64);
            (*xsect).rFull = (*xsect).aFull
                / (2.0f64 * (*xsect).yBot * (*xsect).rBot
                    + 2.0f64 * ((*xsect).yFull - (*xsect).yBot)
                    + (*xsect).wMax);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            aMax = RECT_TRIANG_ALFMAX * (*xsect).aFull;
            (*xsect).sMax = aMax * pow(rect_triang_getRofA(xsect, aMax), 2.0f64 / 3.0f64);
        }
        10 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64 {
                return FALSE;
            }
            if *p.offset(2 as ::core::ffi::c_int as isize)
                < *p.offset(1 as ::core::ffi::c_int as isize) / 2.0f64
            {
                *p.offset(2 as ::core::ffi::c_int as isize) =
                    *p.offset(1 as ::core::ffi::c_int as isize) / 2.0f64;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).rBot = *p.offset(2 as ::core::ffi::c_int as isize) / ucf;
            theta = 2.0f64 * asin((*xsect).wMax / 2.0f64 / (*xsect).rBot);
            (*xsect).aBot = (*xsect).rBot * (*xsect).rBot / 2.0f64 * (theta - sin(theta));
            (*xsect).sBot =
                PI * (*xsect).rBot * (*xsect).rBot * pow((*xsect).rBot / 2.0f64, 2.0f64 / 3.0f64);
            (*xsect).yBot = (*xsect).rBot * (1.0f64 - cos(theta / 2.0f64));
            if (*xsect).yBot > (*xsect).yFull {
                return FALSE;
            }
            (*xsect).ywMax = (*xsect).yFull;
            (*xsect).aFull = (*xsect).wMax * ((*xsect).yFull - (*xsect).yBot) + (*xsect).aBot;
            (*xsect).rFull = (*xsect).aFull
                / ((*xsect).rBot * theta
                    + 2.0f64 * ((*xsect).yFull - (*xsect).yBot)
                    + (*xsect).wMax);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            aMax = RECT_ROUND_ALFMAX * (*xsect).aFull;
            (*xsect).sMax = aMax * pow(rect_round_getRofA(xsect, aMax), 2.0f64 / 3.0f64);
        }
        11 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64 {
                return FALSE;
            }
            if *p.offset(2 as ::core::ffi::c_int as isize)
                < *p.offset(1 as ::core::ffi::c_int as isize) / 2.0f64
            {
                *p.offset(2 as ::core::ffi::c_int as isize) =
                    *p.offset(1 as ::core::ffi::c_int as isize) / 2.0f64;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).rBot = *p.offset(2 as ::core::ffi::c_int as isize) / ucf;
            theta = 2.0f64 * asin((*xsect).wMax / 2.0f64 / (*xsect).rBot);
            (*xsect).sBot = theta;
            (*xsect).yBot = (*xsect).rBot * (1.0f64 - cos(theta / 2.0f64));
            if (*xsect).yBot > (*xsect).yFull {
                return FALSE;
            }
            (*xsect).ywMax = (*xsect).yFull - (*xsect).yBot;
            (*xsect).aBot = (*xsect).rBot * (*xsect).rBot / 2.0f64 * (theta - sin(theta));
            (*xsect).aFull = ((*xsect).yFull - (*xsect).yBot) * (*xsect).wMax + (*xsect).aBot;
            (*xsect).rFull = (*xsect).aFull
                / ((*xsect).rBot * theta
                    + 2.0f64 * ((*xsect).yFull - (*xsect).yBot)
                    + (*xsect).wMax);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = xsect_getSofA(
                xsect,
                Amax[MOD_BASKET as ::core::ffi::c_int as usize] * (*xsect).aFull,
            );
        }
        5 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) < 0.0f64
                || *p.offset(2 as ::core::ffi::c_int as isize) < 0.0f64
                || *p.offset(3 as ::core::ffi::c_int as isize) < 0.0f64
            {
                return FALSE;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).ywMax = (*xsect).yFull;
            (*xsect).yBot = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).sBot = (*p.offset(2 as ::core::ffi::c_int as isize)
                + *p.offset(3 as ::core::ffi::c_int as isize))
                / 2.0f64;
            if (*xsect).yBot == 0.0f64 && (*xsect).sBot == 0.0f64 {
                return FALSE;
            }
            (*xsect).rBot = sqrt(
                1.0f64
                    + *p.offset(2 as ::core::ffi::c_int as isize)
                        * *p.offset(2 as ::core::ffi::c_int as isize),
            ) + sqrt(
                1.0f64
                    + *p.offset(3 as ::core::ffi::c_int as isize)
                        * *p.offset(3 as ::core::ffi::c_int as isize),
            );
            (*xsect).wMax = (*xsect).yBot
                + (*xsect).yFull
                    * (*p.offset(2 as ::core::ffi::c_int as isize)
                        + *p.offset(3 as ::core::ffi::c_int as isize));
            (*xsect).aFull = ((*xsect).yBot + (*xsect).sBot * (*xsect).yFull) * (*xsect).yFull;
            (*xsect).rFull = (*xsect).aFull / ((*xsect).yBot + (*xsect).yFull * (*xsect).rBot);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = (*xsect).sFull;
        }
        6 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64 {
                return FALSE;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).ywMax = (*xsect).yFull;
            (*xsect).sBot = (*xsect).wMax / (*xsect).yFull / 2.0f64;
            (*xsect).rBot = sqrt(1.0f64 + (*xsect).sBot * (*xsect).sBot);
            (*xsect).aFull = (*xsect).yFull * (*xsect).yFull * (*xsect).sBot;
            (*xsect).rFull = (*xsect).aFull / (2.0f64 * (*xsect).yFull * (*xsect).rBot);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = (*xsect).sFull;
        }
        7 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64 {
                return FALSE;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).ywMax = (*xsect).yFull;
            (*xsect).rBot = (*xsect).wMax / 2.0f64 / sqrt((*xsect).yFull);
            (*xsect).aFull = 2.0f64 / 3.0f64 * (*xsect).yFull * (*xsect).wMax;
            (*xsect).rFull = xsect_getRofY(xsect, (*xsect).yFull);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = (*xsect).sFull;
        }
        8 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) <= 0.0f64
                || *p.offset(2 as ::core::ffi::c_int as isize) <= 0.0f64
            {
                return FALSE;
            }
            (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
            (*xsect).ywMax = (*xsect).yFull;
            (*xsect).sBot = 1.0f64 / *p.offset(2 as ::core::ffi::c_int as isize);
            (*xsect).rBot = (*xsect).wMax
                / ((*xsect).sBot + 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                / pow((*xsect).yFull, (*xsect).sBot);
            (*xsect).aFull = (*xsect).yFull * (*xsect).wMax
                / ((*xsect).sBot + 1 as ::core::ffi::c_int as ::core::ffi::c_double);
            (*xsect).rFull = xsect_getRofY(xsect, (*xsect).yFull);
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = (*xsect).sFull;
        }
        12 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) == 0.0f64 {
                *p.offset(2 as ::core::ffi::c_int as isize) =
                    *p.offset(0 as ::core::ffi::c_int as isize);
            }
            if *p.offset(2 as ::core::ffi::c_int as isize) > 0.0f64 {
                index = crate::ccast::f64_to_i32(floor(*p.offset(2 as ::core::ffi::c_int as isize)))
                    - 1 as ::core::ffi::c_int;
                if index < 0 as ::core::ffi::c_int || index >= NumCodesEllipse {
                    return FALSE;
                }
                (*xsect).yFull = MinorAxis_Ellipse[index as usize] / 12.0f64;
                (*xsect).wMax = MajorAxis_Ellipse[index as usize] / 12.0f64;
                (*xsect).aFull = Afull_Ellipse[index as usize];
                (*xsect).rFull = Rfull_Ellipse[index as usize];
            } else {
                (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
                if *p.offset(1 as ::core::ffi::c_int as isize) < 0.0f64 {
                    return FALSE;
                }
                (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
                (*xsect).aFull = 1.2692f64 * (*xsect).yFull * (*xsect).yFull;
                (*xsect).rFull = 0.3061f64 * (*xsect).yFull;
            }
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = (*xsect).sFull;
            (*xsect).ywMax = 0.48f64 * (*xsect).yFull;
        }
        13 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) == 0.0f64 {
                *p.offset(2 as ::core::ffi::c_int as isize) =
                    *p.offset(0 as ::core::ffi::c_int as isize);
            }
            if *p.offset(2 as ::core::ffi::c_int as isize) > 0.0f64 {
                index = crate::ccast::f64_to_i32(floor(*p.offset(2 as ::core::ffi::c_int as isize)))
                    - 1 as ::core::ffi::c_int;
                if index < 0 as ::core::ffi::c_int || index >= NumCodesEllipse {
                    return FALSE;
                }
                (*xsect).yFull = MajorAxis_Ellipse[index as usize] / 12.0f64;
                (*xsect).wMax = MinorAxis_Ellipse[index as usize] / 12.0f64;
                (*xsect).aFull = Afull_Ellipse[index as usize];
                (*xsect).rFull = Rfull_Ellipse[index as usize];
            } else {
                if *p.offset(1 as ::core::ffi::c_int as isize) < 0.0f64 {
                    return FALSE;
                }
                (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
                (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
                (*xsect).aFull = 1.2692f64 * (*xsect).wMax * (*xsect).wMax;
                (*xsect).rFull = 0.3061f64 * (*xsect).wMax;
            }
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = (*xsect).sFull;
            (*xsect).ywMax = 0.48f64 * (*xsect).yFull;
        }
        14 => {
            if *p.offset(1 as ::core::ffi::c_int as isize) == 0.0f64 {
                *p.offset(2 as ::core::ffi::c_int as isize) =
                    *p.offset(0 as ::core::ffi::c_int as isize);
            }
            if *p.offset(2 as ::core::ffi::c_int as isize) > 0.0f64 {
                index = crate::ccast::f64_to_i32(floor(*p.offset(2 as ::core::ffi::c_int as isize)))
                    - 1 as ::core::ffi::c_int;
                if index < 0 as ::core::ffi::c_int || index >= NumCodesArch {
                    return FALSE;
                }
                (*xsect).yFull = Yfull_Arch[index as usize] / 12.0f64;
                (*xsect).wMax = Wmax_Arch[index as usize] / 12.0f64;
                (*xsect).aFull = Afull_Arch[index as usize];
                (*xsect).rFull = Rfull_Arch[index as usize];
            } else {
                if *p.offset(1 as ::core::ffi::c_int as isize) < 0.0f64 {
                    return FALSE;
                }
                (*xsect).yFull = *p.offset(0 as ::core::ffi::c_int as isize) / ucf;
                (*xsect).wMax = *p.offset(1 as ::core::ffi::c_int as isize) / ucf;
                (*xsect).aFull = 0.7879f64 * (*xsect).yFull * (*xsect).wMax;
                (*xsect).rFull = 0.2991f64 * (*xsect).yFull;
            }
            (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
            (*xsect).sMax = (*xsect).sFull;
            (*xsect).ywMax = 0.28f64 * (*xsect).yFull;
        }
        _ => {}
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn xsect_setIrregXsectParams(mut xsect: *mut TXsect) {
    let mut index: ::core::ffi::c_int = (*xsect).transect;
    getTransectParams(xsect, Transect.offset(index as isize) as *mut TTransect);
}
#[no_mangle]
pub unsafe extern "C" fn xsect_setStreetXsectParams(mut xsect: *mut TXsect) {
    let mut index: ::core::ffi::c_int = (*xsect).transect;
    getTransectParams(xsect, &raw mut (*Street.offset(index as isize)).transect);
}
#[no_mangle]
pub unsafe extern "C" fn xsect_setCustomXsectParams(mut xsect: *mut TXsect) {
    let mut index: ::core::ffi::c_int = (*Curve.offset((*xsect).transect as isize)).refersTo;
    let mut yFull: ::core::ffi::c_double = (*xsect).yFull;
    let mut i: ::core::ffi::c_int = 0;
    let mut iMax: ::core::ffi::c_int = 0;
    let mut wMax: ::core::ffi::c_double = 0.;
    let mut wTbl: *mut ::core::ffi::c_double =
        &raw mut (*Shape.offset(index as isize)).widthTbl as *mut ::core::ffi::c_double;
    (*xsect).wMax = (*Shape.offset(index as isize)).wMax * yFull;
    (*xsect).aFull = (*Shape.offset(index as isize)).aFull * yFull * yFull;
    (*xsect).rFull = (*Shape.offset(index as isize)).rFull * yFull;
    (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
    (*xsect).sMax =
        (*Shape.offset(index as isize)).sMax * yFull * yFull * pow(yFull, 2.0f64 / 3.0f64);
    (*xsect).aBot = (*Shape.offset(index as isize)).aMax * yFull * yFull;
    iMax = 0 as ::core::ffi::c_int;
    wMax = *wTbl.offset(0 as ::core::ffi::c_int as isize);
    i = 1 as ::core::ffi::c_int;
    while i < N_SHAPE_TBL {
        if *wTbl.offset(i as isize) < wMax {
            break;
        }
        wMax = *wTbl.offset(i as isize);
        iMax = i;
        i += 1;
    }
    (*xsect).ywMax = yFull * iMax as ::core::ffi::c_double
        / (N_SHAPE_TBL - 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getAmax(mut xsect: *mut TXsect) -> ::core::ffi::c_double {
    if (*xsect).type_0 == IRREGULAR as ::core::ffi::c_int {
        return (*xsect).aBot;
    } else if (*xsect).type_0 == CUSTOM as ::core::ffi::c_int {
        return (*xsect).aBot;
    } else {
        return Amax[(*xsect).type_0 as usize] * (*xsect).aFull;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getSofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = a / (*xsect).aFull;
    let mut r: ::core::ffi::c_double = 0.;
    match (*xsect).type_0 {
        24 | 1 => return circ_getSofA(xsect, a),
        15 => {
            return (*xsect).sFull
                * lookup(alpha, &raw mut S_Egg as *mut ::core::ffi::c_double, N_S_Egg);
        }
        16 => {
            return (*xsect).sFull
                * lookup(
                    alpha,
                    &raw mut S_Horseshoe as *mut ::core::ffi::c_double,
                    N_S_Horseshoe,
                );
        }
        17 => {
            return (*xsect).sFull
                * lookup(
                    alpha,
                    &raw mut S_Gothic as *mut ::core::ffi::c_double,
                    N_S_Gothic,
                );
        }
        18 => {
            return (*xsect).sFull
                * lookup(
                    alpha,
                    &raw mut S_Catenary as *mut ::core::ffi::c_double,
                    N_S_Catenary,
                );
        }
        19 => {
            return (*xsect).sFull
                * lookup(
                    alpha,
                    &raw mut S_SemiEllip as *mut ::core::ffi::c_double,
                    N_S_SemiEllip,
                );
        }
        20 => {
            return (*xsect).sFull
                * lookup(
                    alpha,
                    &raw mut S_BasketHandle as *mut ::core::ffi::c_double,
                    N_S_BasketHandle,
                );
        }
        21 => {
            return (*xsect).sFull
                * lookup(
                    alpha,
                    &raw mut S_SemiCirc as *mut ::core::ffi::c_double,
                    N_S_SemiCirc,
                );
        }
        3 => return rect_closed_getSofA(xsect, a),
        4 => return rect_open_getSofA(xsect, a),
        9 => return rect_triang_getSofA(xsect, a),
        10 => return rect_round_getSofA(xsect, a),
        _ => {
            if a == 0.0f64 {
                return 0.0f64;
            }
            r = xsect_getRofA(xsect, a);
            if r < TINY {
                return 0.0f64;
            }
            return a * pow(r, 2.0f64 / 3.0f64);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = a / (*xsect).aFull;
    match (*xsect).type_0 {
        24 | 1 => return circ_getYofA(xsect, a),
        2 => return filled_circ_getYofA(xsect, a),
        15 => {
            return (*xsect).yFull
                * lookup(alpha, &raw mut Y_Egg as *mut ::core::ffi::c_double, N_Y_Egg);
        }
        16 => {
            return (*xsect).yFull
                * lookup(
                    alpha,
                    &raw mut Y_Horseshoe as *mut ::core::ffi::c_double,
                    N_Y_Horseshoe,
                );
        }
        17 => {
            return (*xsect).yFull
                * lookup(
                    alpha,
                    &raw mut Y_Gothic as *mut ::core::ffi::c_double,
                    N_Y_Gothic,
                );
        }
        18 => {
            return (*xsect).yFull
                * lookup(
                    alpha,
                    &raw mut Y_Catenary as *mut ::core::ffi::c_double,
                    N_Y_Catenary,
                );
        }
        19 => {
            return (*xsect).yFull
                * lookup(
                    alpha,
                    &raw mut Y_SemiEllip as *mut ::core::ffi::c_double,
                    N_Y_SemiEllip,
                );
        }
        20 => {
            return (*xsect).yFull
                * lookup(
                    alpha,
                    &raw mut Y_BasketHandle as *mut ::core::ffi::c_double,
                    N_Y_BasketHandle,
                );
        }
        21 => {
            return (*xsect).yFull
                * lookup(
                    alpha,
                    &raw mut Y_SemiCirc as *mut ::core::ffi::c_double,
                    N_Y_SemiCirc,
                );
        }
        12 => {
            return (*xsect).yFull
                * invLookup(
                    alpha,
                    &raw mut A_HorizEllipse as *mut ::core::ffi::c_double,
                    N_A_HorizEllipse,
                );
        }
        13 => {
            return (*xsect).yFull
                * invLookup(
                    alpha,
                    &raw mut A_VertEllipse as *mut ::core::ffi::c_double,
                    N_A_VertEllipse,
                );
        }
        22 => {
            return (*xsect).yFull
                * invLookup(
                    alpha,
                    &raw mut (*Transect.offset((*xsect).transect as isize)).areaTbl
                        as *mut ::core::ffi::c_double,
                    N_TRANSECT_TBL,
                );
        }
        23 => {
            return (*xsect).yFull
                * invLookup(
                    alpha,
                    &raw mut (*Shape
                        .offset((*Curve.offset((*xsect).transect as isize)).refersTo as isize))
                    .areaTbl as *mut ::core::ffi::c_double,
                    N_SHAPE_TBL,
                );
        }
        25 => {
            return (*xsect).yFull
                * invLookup(
                    alpha,
                    &raw mut (*Street.offset((*xsect).transect as isize))
                        .transect
                        .areaTbl as *mut ::core::ffi::c_double,
                    (*Street.offset((*xsect).transect as isize)).transect.nTbl,
                );
        }
        14 => {
            return (*xsect).yFull
                * invLookup(
                    alpha,
                    &raw mut A_Arch as *mut ::core::ffi::c_double,
                    N_A_Arch,
                );
        }
        3 => return a / (*xsect).wMax,
        9 => return rect_triang_getYofA(xsect, a),
        10 => return rect_round_getYofA(xsect, a),
        4 => return a / (*xsect).wMax,
        11 => return mod_basket_getYofA(xsect, a),
        5 => return trapez_getYofA(xsect, a),
        6 => return triang_getYofA(xsect, a),
        7 => return parab_getYofA(xsect, a),
        8 => return powerfunc_getYofA(xsect, a),
        _ => return 0.0f64,
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut yNorm: ::core::ffi::c_double = y / (*xsect).yFull;
    if y <= 0.0f64 {
        return 0.0f64;
    }
    match (*xsect).type_0 {
        24 | 1 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut A_Circ as *mut ::core::ffi::c_double,
                    N_A_Circ,
                );
        }
        2 => return filled_circ_getAofY(xsect, y),
        15 => {
            return (*xsect).aFull
                * lookup(yNorm, &raw mut A_Egg as *mut ::core::ffi::c_double, N_A_Egg);
        }
        16 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut A_Horseshoe as *mut ::core::ffi::c_double,
                    N_A_Horseshoe,
                );
        }
        17 => {
            return (*xsect).aFull
                * invLookup(
                    yNorm,
                    &raw mut Y_Gothic as *mut ::core::ffi::c_double,
                    N_Y_Gothic,
                );
        }
        18 => {
            return (*xsect).aFull
                * invLookup(
                    yNorm,
                    &raw mut Y_Catenary as *mut ::core::ffi::c_double,
                    N_Y_Catenary,
                );
        }
        19 => {
            return (*xsect).aFull
                * invLookup(
                    yNorm,
                    &raw mut Y_SemiEllip as *mut ::core::ffi::c_double,
                    N_Y_SemiEllip,
                );
        }
        20 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut A_Baskethandle as *mut ::core::ffi::c_double,
                    N_A_Baskethandle,
                );
        }
        21 => {
            return (*xsect).aFull
                * invLookup(
                    yNorm,
                    &raw mut Y_SemiCirc as *mut ::core::ffi::c_double,
                    N_Y_SemiCirc,
                );
        }
        12 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut A_HorizEllipse as *mut ::core::ffi::c_double,
                    N_A_HorizEllipse,
                );
        }
        13 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut A_VertEllipse as *mut ::core::ffi::c_double,
                    N_A_VertEllipse,
                );
        }
        14 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut A_Arch as *mut ::core::ffi::c_double,
                    N_A_Arch,
                );
        }
        22 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut (*Transect.offset((*xsect).transect as isize)).areaTbl
                        as *mut ::core::ffi::c_double,
                    N_TRANSECT_TBL,
                );
        }
        23 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut (*Shape
                        .offset((*Curve.offset((*xsect).transect as isize)).refersTo as isize))
                    .areaTbl as *mut ::core::ffi::c_double,
                    N_SHAPE_TBL,
                );
        }
        25 => {
            return (*xsect).aFull
                * lookup(
                    yNorm,
                    &raw mut (*Street.offset((*xsect).transect as isize))
                        .transect
                        .areaTbl as *mut ::core::ffi::c_double,
                    (*Street.offset((*xsect).transect as isize)).transect.nTbl,
                );
        }
        3 => return y * (*xsect).wMax,
        9 => return rect_triang_getAofY(xsect, y),
        10 => return rect_round_getAofY(xsect, y),
        4 => return y * (*xsect).wMax,
        11 => return mod_basket_getAofY(xsect, y),
        5 => return trapez_getAofY(xsect, y),
        6 => return triang_getAofY(xsect, y),
        7 => return parab_getAofY(xsect, y),
        8 => return powerfunc_getAofY(xsect, y),
        _ => return 0.0f64,
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getWofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut yNorm: ::core::ffi::c_double = y / (*xsect).yFull;
    match (*xsect).type_0 {
        24 | 1 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_Circ as *mut ::core::ffi::c_double,
                    N_W_Circ,
                );
        }
        2 => {
            yNorm = (y + (*xsect).yBot) / ((*xsect).yFull + (*xsect).yBot);
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_Circ as *mut ::core::ffi::c_double,
                    N_W_Circ,
                );
        }
        15 => {
            return (*xsect).wMax
                * lookup(yNorm, &raw mut W_Egg as *mut ::core::ffi::c_double, N_W_Egg);
        }
        16 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_Horseshoe as *mut ::core::ffi::c_double,
                    N_W_Horseshoe,
                );
        }
        17 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_Gothic as *mut ::core::ffi::c_double,
                    N_W_Gothic,
                );
        }
        18 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_Catenary as *mut ::core::ffi::c_double,
                    N_W_Catenary,
                );
        }
        19 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_SemiEllip as *mut ::core::ffi::c_double,
                    N_W_SemiEllip,
                );
        }
        20 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_BasketHandle as *mut ::core::ffi::c_double,
                    N_W_BasketHandle,
                );
        }
        21 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_SemiCirc as *mut ::core::ffi::c_double,
                    N_W_SemiCirc,
                );
        }
        12 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_HorizEllipse as *mut ::core::ffi::c_double,
                    N_W_HorizEllipse,
                );
        }
        13 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_VertEllipse as *mut ::core::ffi::c_double,
                    N_W_VertEllipse,
                );
        }
        14 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut W_Arch as *mut ::core::ffi::c_double,
                    N_W_Arch,
                );
        }
        22 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut (*Transect.offset((*xsect).transect as isize)).widthTbl
                        as *mut ::core::ffi::c_double,
                    N_TRANSECT_TBL,
                );
        }
        23 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut (*Shape
                        .offset((*Curve.offset((*xsect).transect as isize)).refersTo as isize))
                    .widthTbl as *mut ::core::ffi::c_double,
                    N_SHAPE_TBL,
                );
        }
        25 => {
            return (*xsect).wMax
                * lookup(
                    yNorm,
                    &raw mut (*Street.offset((*xsect).transect as isize))
                        .transect
                        .widthTbl as *mut ::core::ffi::c_double,
                    (*Street.offset((*xsect).transect as isize)).transect.nTbl,
                );
        }
        3 => {
            if yNorm == 1.0f64 {
                return 0.0f64;
            }
            return (*xsect).wMax;
        }
        9 => return rect_triang_getWofY(xsect, y),
        10 => return rect_round_getWofY(xsect, y),
        4 => return (*xsect).wMax,
        11 => return mod_basket_getWofY(xsect, y),
        5 => return trapez_getWofY(xsect, y),
        6 => return triang_getWofY(xsect, y),
        7 => return parab_getWofY(xsect, y),
        8 => return powerfunc_getWofY(xsect, y),
        _ => return 0.0f64,
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getRofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut yNorm: ::core::ffi::c_double = y / (*xsect).yFull;
    match (*xsect).type_0 {
        24 | 1 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut R_Circ as *mut ::core::ffi::c_double,
                    N_R_Circ,
                );
        }
        2 => {
            if (*xsect).yBot == 0.0f64 {
                return (*xsect).rFull
                    * lookup(
                        yNorm,
                        &raw mut R_Circ as *mut ::core::ffi::c_double,
                        N_R_Circ,
                    );
            }
            return filled_circ_getRofY(xsect, y);
        }
        15 => {
            return (*xsect).rFull
                * lookup(yNorm, &raw mut R_Egg as *mut ::core::ffi::c_double, N_R_Egg);
        }
        16 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut R_Horseshoe as *mut ::core::ffi::c_double,
                    N_R_Horseshoe,
                );
        }
        20 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut R_Baskethandle as *mut ::core::ffi::c_double,
                    N_R_Baskethandle,
                );
        }
        12 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut R_HorizEllipse as *mut ::core::ffi::c_double,
                    N_R_HorizEllipse,
                );
        }
        13 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut R_VertEllipse as *mut ::core::ffi::c_double,
                    N_R_VertEllipse,
                );
        }
        14 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut R_Arch as *mut ::core::ffi::c_double,
                    N_R_Arch,
                );
        }
        22 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut (*Transect.offset((*xsect).transect as isize)).hradTbl
                        as *mut ::core::ffi::c_double,
                    N_TRANSECT_TBL,
                );
        }
        23 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut (*Shape
                        .offset((*Curve.offset((*xsect).transect as isize)).refersTo as isize))
                    .hradTbl as *mut ::core::ffi::c_double,
                    N_SHAPE_TBL,
                );
        }
        25 => {
            return (*xsect).rFull
                * lookup(
                    yNorm,
                    &raw mut (*Street.offset((*xsect).transect as isize))
                        .transect
                        .hradTbl as *mut ::core::ffi::c_double,
                    (*Street.offset((*xsect).transect as isize)).transect.nTbl,
                );
        }
        9 => return rect_triang_getRofY(xsect, y),
        10 => return rect_round_getRofY(xsect, y),
        5 => return trapez_getRofY(xsect, y),
        6 => return triang_getRofY(xsect, y),
        7 => return parab_getRofY(xsect, y),
        8 => return powerfunc_getRofY(xsect, y),
        _ => return xsect_getRofA(xsect, xsect_getAofY(xsect, y)),
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut cathy: ::core::ffi::c_double = 0.;
    if a <= 0.0f64 {
        return 0.0f64;
    }
    match (*xsect).type_0 {
        12 | 13 | 14 | 22 | 2 | 23 | 25 => {
            return xsect_getRofY(xsect, xsect_getYofA(xsect, a));
        }
        3 => return rect_closed_getRofA(xsect, a),
        4 => return a / ((*xsect).wMax + (2.0f64 - (*xsect).sBot) * a / (*xsect).wMax),
        9 => return rect_triang_getRofA(xsect, a),
        10 => return rect_round_getRofA(xsect, a),
        11 => return mod_basket_getRofA(xsect, a),
        5 => return trapez_getRofA(xsect, a),
        6 => return triang_getRofA(xsect, a),
        7 => return parab_getRofA(xsect, a),
        8 => return powerfunc_getRofA(xsect, a),
        _ => {
            cathy = xsect_getSofA(xsect, a);
            if cathy < TINY || a < TINY {
                return 0.0f64;
            }
            return pow(cathy / a, 3.0f64 / 2.0f64);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getAofS(
    mut xsect: *mut TXsect,
    mut s: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut psi: ::core::ffi::c_double = s / (*xsect).sFull;
    if s <= 0.0f64 {
        return 0.0f64;
    }
    if s > (*xsect).sMax {
        s = (*xsect).sMax;
    }
    match (*xsect).type_0 {
        0 => return 0.0f64,
        24 | 1 => return circ_getAofS(xsect, s),
        15 => {
            return (*xsect).aFull
                * invLookup(psi, &raw mut S_Egg as *mut ::core::ffi::c_double, N_S_Egg);
        }
        16 => {
            return (*xsect).aFull
                * invLookup(
                    psi,
                    &raw mut S_Horseshoe as *mut ::core::ffi::c_double,
                    N_S_Horseshoe,
                );
        }
        17 => {
            return (*xsect).aFull
                * invLookup(
                    psi,
                    &raw mut S_Gothic as *mut ::core::ffi::c_double,
                    N_S_Gothic,
                );
        }
        18 => {
            return (*xsect).aFull
                * invLookup(
                    psi,
                    &raw mut S_Catenary as *mut ::core::ffi::c_double,
                    N_S_Catenary,
                );
        }
        19 => {
            return (*xsect).aFull
                * invLookup(
                    psi,
                    &raw mut S_SemiEllip as *mut ::core::ffi::c_double,
                    N_S_SemiEllip,
                );
        }
        20 => {
            return (*xsect).aFull
                * invLookup(
                    psi,
                    &raw mut S_BasketHandle as *mut ::core::ffi::c_double,
                    N_S_BasketHandle,
                );
        }
        21 => {
            return (*xsect).aFull
                * invLookup(
                    psi,
                    &raw mut S_SemiCirc as *mut ::core::ffi::c_double,
                    N_S_SemiCirc,
                );
        }
        _ => return generic_getAofS(xsect, s),
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    match (*xsect).type_0 {
        24 | 1 => return circ_getdSdA(xsect, a),
        15 => {
            return tabular_getdSdA(
                xsect,
                a,
                &raw mut S_Egg as *mut ::core::ffi::c_double,
                N_S_Egg,
            );
        }
        16 => {
            return tabular_getdSdA(
                xsect,
                a,
                &raw mut S_Horseshoe as *mut ::core::ffi::c_double,
                N_S_Horseshoe,
            );
        }
        17 => {
            return tabular_getdSdA(
                xsect,
                a,
                &raw mut S_Gothic as *mut ::core::ffi::c_double,
                N_S_Gothic,
            );
        }
        18 => {
            return tabular_getdSdA(
                xsect,
                a,
                &raw mut S_Catenary as *mut ::core::ffi::c_double,
                N_S_Catenary,
            );
        }
        19 => {
            return tabular_getdSdA(
                xsect,
                a,
                &raw mut S_SemiEllip as *mut ::core::ffi::c_double,
                N_S_SemiEllip,
            );
        }
        20 => {
            return tabular_getdSdA(
                xsect,
                a,
                &raw mut S_BasketHandle as *mut ::core::ffi::c_double,
                N_S_BasketHandle,
            );
        }
        21 => {
            return tabular_getdSdA(
                xsect,
                a,
                &raw mut S_SemiCirc as *mut ::core::ffi::c_double,
                N_S_SemiCirc,
            );
        }
        3 => return rect_closed_getdSdA(xsect, a),
        4 => return rect_open_getdSdA(xsect, a),
        9 => return rect_triang_getdSdA(xsect, a),
        10 => return rect_round_getdSdA(xsect, a),
        11 => return mod_basket_getdSdA(xsect, a),
        5 => return trapez_getdSdA(xsect, a),
        6 => return triang_getdSdA(xsect, a),
        _ => return generic_getdSdA(xsect, a),
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsect_getYcrit(
    mut xsect: *mut TXsect,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut q2g: ::core::ffi::c_double = q * q / GRAVITY;
    let mut y: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    if q2g == 0.0f64 {
        return 0.0f64;
    }
    match (*xsect).type_0 {
        0 => return 0.0f64,
        4 | 3 => {
            y = pow(q2g / ((*xsect).wMax * (*xsect).wMax), 1.0f64 / 3.0f64);
        }
        6 => {
            y = pow(
                2.0f64 * q2g / ((*xsect).sBot * (*xsect).sBot),
                1.0f64 / 5.0f64,
            );
        }
        7 => {
            y = pow(
                27.0f64 / 32.0f64 * q2g / ((*xsect).rBot * (*xsect).rBot),
                1.0f64 / 4.0f64,
            );
        }
        8 => {
            y = 1.0f64 / (2.0f64 * (*xsect).sBot + 3.0f64);
            y = pow(
                q2g * ((*xsect).sBot + 1.0f64) / ((*xsect).rBot * (*xsect).rBot),
                y,
            );
        }
        _ => {
            y = 1.01f64 * pow(q2g / (*xsect).yFull, 1.0f64 / 4.0f64);
            if y >= (*xsect).yFull {
                y = 0.97f64 * (*xsect).yFull;
            }
            r = (*xsect).aFull / (PI / 4.0f64 * ((*xsect).yFull * (*xsect).yFull));
            if r >= 0.5f64 && r <= 2.0f64 {
                y = getYcritEnum(xsect, q, y);
            } else {
                y = getYcritRidder(xsect, q, y);
            }
        }
    }
    return if y <= (*xsect).yFull {
        y
    } else {
        (*xsect).yFull
    };
}
unsafe extern "C" fn getTransectParams(mut xsect: *mut TXsect, mut transect: *mut TTransect) {
    let mut i: ::core::ffi::c_int = 0;
    let mut iMax: ::core::ffi::c_int = 0;
    let mut wMax: ::core::ffi::c_double = 0.;
    let mut wTbl: *mut ::core::ffi::c_double =
        &raw mut (*transect).widthTbl as *mut ::core::ffi::c_double;
    (*xsect).yFull = (*transect).yFull;
    (*xsect).wMax = (*transect).wMax;
    (*xsect).aFull = (*transect).aFull;
    (*xsect).rFull = (*transect).rFull;
    (*xsect).sFull = (*xsect).aFull * pow((*xsect).rFull, 2.0f64 / 3.0f64);
    (*xsect).sMax = (*transect).sMax;
    (*xsect).aBot = (*transect).aMax;
    iMax = 0 as ::core::ffi::c_int;
    wMax = *wTbl.offset(0 as ::core::ffi::c_int as isize);
    i = 1 as ::core::ffi::c_int;
    while i < (*transect).nTbl {
        if *wTbl.offset(i as isize) < wMax {
            break;
        }
        wMax = *wTbl.offset(i as isize);
        iMax = i;
        i += 1;
    }
    (*xsect).ywMax = (*xsect).yFull * iMax as ::core::ffi::c_double
        / ((*transect).nTbl as ::core::ffi::c_double
            - 1 as ::core::ffi::c_int as ::core::ffi::c_double);
}
unsafe extern "C" fn generic_getAofS(
    mut xsect: *mut TXsect,
    mut s: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a: ::core::ffi::c_double = 0.;
    let mut a1: ::core::ffi::c_double = 0.;
    let mut a2: ::core::ffi::c_double = 0.;
    let mut tol: ::core::ffi::c_double = 0.;
    let mut xsectStar: TXsectStar = TXsectStar {
        s: 0.,
        qc: 0.,
        xsect: ::core::ptr::null_mut::<TXsect>(),
    };
    if s <= 0.0f64 {
        return 0.0f64;
    }
    if s <= (*xsect).sMax && s >= (*xsect).sFull && (*xsect).sMax != (*xsect).sFull {
        a1 = (*xsect).aFull;
        a2 = xsect_getAmax(xsect);
    } else {
        a1 = 0.0f64;
        a2 = xsect_getAmax(xsect);
    }
    xsectStar.xsect = xsect;
    xsectStar.s = s;
    a = 0.5f64 * (a1 + a2);
    tol = 0.0001f64 * (*xsect).aFull;
    findroot_Newton(
        a1,
        a2,
        &raw mut a,
        tol,
        Some(
            evalSofA
                as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut xsectStar as *mut ::core::ffi::c_void,
    );
    return a;
}
unsafe extern "C" fn evalSofA(
    mut a: ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
    mut df: *mut ::core::ffi::c_double,
    mut p: *mut ::core::ffi::c_void,
) {
    let mut xsectStar: *mut TXsectStar = ::core::ptr::null_mut::<TXsectStar>();
    let mut s: ::core::ffi::c_double = 0.;
    xsectStar = p as *mut TXsectStar;
    s = xsect_getSofA((*xsectStar).xsect, a);
    *f = s - (*xsectStar).s;
    *df = xsect_getdSdA((*xsectStar).xsect, a);
}
unsafe extern "C" fn tabular_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
    mut table: *mut ::core::ffi::c_double,
    mut nItems: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut alpha: ::core::ffi::c_double = a / (*xsect).aFull;
    let mut delta: ::core::ffi::c_double = 1.0f64
        / (nItems as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double);
    let mut dSdA: ::core::ffi::c_double = 0.;
    i = crate::ccast::f64_to_i32((alpha / delta));
    if i >= nItems - 1 as ::core::ffi::c_int {
        i = nItems - 2 as ::core::ffi::c_int;
    }
    dSdA =
        (*table.offset((i + 1 as ::core::ffi::c_int) as isize) - *table.offset(i as isize)) / delta;
    return dSdA * (*xsect).sFull / (*xsect).aFull;
}
unsafe extern "C" fn generic_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a1: ::core::ffi::c_double = 0.;
    let mut a2: ::core::ffi::c_double = 0.;
    let mut alpha: ::core::ffi::c_double = a / (*xsect).aFull;
    let mut alpha1: ::core::ffi::c_double = alpha - 0.001f64;
    let mut alpha2: ::core::ffi::c_double = alpha + 0.001f64;
    if alpha1 < 0.0f64 {
        alpha1 = 0.0f64;
    }
    a1 = alpha1 * (*xsect).aFull;
    a2 = alpha2 * (*xsect).aFull;
    return (xsect_getSofA(xsect, a2) - xsect_getSofA(xsect, a1)) / (a2 - a1);
}
unsafe extern "C" fn lookup(
    mut x: ::core::ffi::c_double,
    mut table: *mut ::core::ffi::c_double,
    mut nItems: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut delta: ::core::ffi::c_double = 0.;
    let mut x0: ::core::ffi::c_double = 0.;
    let mut x1: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut i: ::core::ffi::c_int = 0;
    delta = 1.0f64
        / (nItems as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double);
    i = crate::ccast::f64_to_i32((x / delta));
    if i >= nItems - 1 as ::core::ffi::c_int {
        return *table.offset((nItems - 1 as ::core::ffi::c_int) as isize);
    }
    x0 = i as ::core::ffi::c_double * delta;
    x1 = (i as ::core::ffi::c_double + 1 as ::core::ffi::c_int as ::core::ffi::c_double) * delta;
    y = *table.offset(i as isize)
        + (x - x0)
            * (*table.offset((i + 1 as ::core::ffi::c_int) as isize) - *table.offset(i as isize))
            / delta;
    if i < 2 as ::core::ffi::c_int {
        y2 = y
            + (x - x0) * (x - x1) / (delta * delta)
                * (*table.offset(i as isize) / 2.0f64
                    - *table.offset((i + 1 as ::core::ffi::c_int) as isize)
                    + *table.offset((i + 2 as ::core::ffi::c_int) as isize) / 2.0f64);
        if y2 > 0.0f64 {
            y = y2;
        }
    }
    if y < 0.0f64 {
        y = 0.0f64;
    }
    return y;
}
unsafe extern "C" fn invLookup(
    mut y: ::core::ffi::c_double,
    mut table: *mut ::core::ffi::c_double,
    mut nItems: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut dx: ::core::ffi::c_double = 0.;
    let mut x: ::core::ffi::c_double = 0.;
    let mut x0: ::core::ffi::c_double = 0.;
    let mut dy: ::core::ffi::c_double = 0.;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    dx = 1.0f64
        / (nItems as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double);
    n = nItems;
    if *table.offset((n - 3 as ::core::ffi::c_int) as isize)
        > *table.offset((n - 1 as ::core::ffi::c_int) as isize)
    {
        n = n - 2 as ::core::ffi::c_int;
    }
    if n < nItems && y > *table.offset((nItems - 1 as ::core::ffi::c_int) as isize) {
        if y >= *table.offset((nItems - 3 as ::core::ffi::c_int) as isize) {
            return (n as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                * dx;
        }
        if y <= *table.offset((nItems - 2 as ::core::ffi::c_int) as isize) {
            i = nItems - 2 as ::core::ffi::c_int;
        } else {
            i = nItems - 3 as ::core::ffi::c_int;
        }
    } else {
        i = locate(y, table, n - 1 as ::core::ffi::c_int);
    }
    if i >= n - 1 as ::core::ffi::c_int {
        return (n as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double)
            * dx;
    }
    x0 = i as ::core::ffi::c_double * dx;
    dy = *table.offset((i + 1 as ::core::ffi::c_int) as isize) - *table.offset(i as isize);
    if dy == 0.0f64 {
        x = x0;
    } else {
        x = x0 + (y - *table.offset(i as isize)) * dx / dy;
    }
    if x < 0.0f64 {
        x = 0.0f64;
    }
    if x > 1.0f64 {
        x = 1.0f64;
    }
    return x;
}
unsafe extern "C" fn locate(
    mut y: ::core::ffi::c_double,
    mut table: *mut ::core::ffi::c_double,
    mut jLast: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut j1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j2: ::core::ffi::c_int = jLast;
    if y <= *table.offset(0 as ::core::ffi::c_int as isize) {
        return 0 as ::core::ffi::c_int;
    }
    if y >= *table.offset(jLast as isize) {
        return jLast;
    }
    while j2 - j1 > 1 as ::core::ffi::c_int {
        j = j1 + j2 >> 1 as ::core::ffi::c_int;
        if y >= *table.offset(j as isize) {
            j1 = j;
        } else {
            j2 = j;
        }
    }
    return j1;
}
unsafe extern "C" fn getQcritical(
    mut yc: ::core::ffi::c_double,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    let mut a: ::core::ffi::c_double = 0.;
    let mut w: ::core::ffi::c_double = 0.;
    let mut qc: ::core::ffi::c_double = 0.;
    let mut xsectStar: *mut TXsectStar = ::core::ptr::null_mut::<TXsectStar>();
    xsectStar = p as *mut TXsectStar;
    a = xsect_getAofY((*xsectStar).xsect, yc);
    w = xsect_getWofY((*xsectStar).xsect, yc);
    qc = -(*xsectStar).qc;
    if w > 0.0f64 {
        qc = a * sqrt(GRAVITY * a / w) - (*xsectStar).qc;
    }
    return qc;
}
unsafe extern "C" fn getYcritEnum(
    mut xsect: *mut TXsect,
    mut q: ::core::ffi::c_double,
    mut y0: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut q0: ::core::ffi::c_double = 0.;
    let mut dy: ::core::ffi::c_double = 0.;
    let mut qc: ::core::ffi::c_double = 0.;
    let mut yc: ::core::ffi::c_double = 0.;
    let mut i1: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut xsectStar: TXsectStar = TXsectStar {
        s: 0.,
        qc: 0.,
        xsect: ::core::ptr::null_mut::<TXsect>(),
    };
    dy = (*xsect).yFull / 25.0f64;
    i1 = crate::ccast::f64_to_i32((y0 / dy));
    xsectStar.xsect = xsect;
    xsectStar.qc = 0.0f64;
    q0 = getQcritical(
        i1 as ::core::ffi::c_double * dy,
        &raw mut xsectStar as *mut ::core::ffi::c_void,
    );
    if q0 < q {
        yc = (*xsect).yFull;
        i = i1 + 1 as ::core::ffi::c_int;
        while i <= 25 as ::core::ffi::c_int {
            qc = getQcritical(
                i as ::core::ffi::c_double * dy,
                &raw mut xsectStar as *mut ::core::ffi::c_void,
            );
            if qc >= q {
                yc = ((q - q0) / (qc - q0)
                    + (i as ::core::ffi::c_double
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_double))
                    * dy;
                break;
            } else {
                q0 = qc;
                i += 1;
            }
        }
    } else {
        yc = 0.0f64;
        i = i1 - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            qc = getQcritical(
                i as ::core::ffi::c_double * dy,
                &raw mut xsectStar as *mut ::core::ffi::c_void,
            );
            if qc < q {
                yc = ((q - qc) / (q0 - qc) + i as ::core::ffi::c_double) * dy;
                break;
            } else {
                q0 = qc;
                i -= 1;
            }
        }
    }
    return yc;
}
unsafe extern "C" fn getYcritRidder(
    mut xsect: *mut TXsect,
    mut q: ::core::ffi::c_double,
    mut y0: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut y1: ::core::ffi::c_double = 0.0f64;
    let mut y2: ::core::ffi::c_double = 0.99f64 * (*xsect).yFull;
    let mut yc: ::core::ffi::c_double = 0.;
    let mut q0: ::core::ffi::c_double = 0.;
    let mut q1: ::core::ffi::c_double = 0.;
    let mut q2: ::core::ffi::c_double = 0.;
    let mut xsectStar: TXsectStar = TXsectStar {
        s: 0.,
        qc: 0.,
        xsect: ::core::ptr::null_mut::<TXsect>(),
    };
    xsectStar.xsect = xsect;
    xsectStar.qc = 0.0f64;
    q2 = getQcritical(y2, &raw mut xsectStar as *mut ::core::ffi::c_void);
    if q2 < q {
        return (*xsect).yFull;
    }
    q0 = getQcritical(y0, &raw mut xsectStar as *mut ::core::ffi::c_void);
    q1 = getQcritical(
        0.5f64 * (*xsect).yFull,
        &raw mut xsectStar as *mut ::core::ffi::c_void,
    );
    if q0 > q {
        y2 = y0;
        if q1 < q {
            y1 = 0.5f64 * (*xsect).yFull;
        }
    } else {
        y1 = y0;
        if q1 > q {
            y2 = 0.5f64 * (*xsect).yFull;
        }
    }
    xsectStar.qc = q;
    yc = findroot_Ridder(
        y1,
        y2,
        0.001f64,
        Some(
            getQcritical
                as unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_double,
        ),
        &raw mut xsectStar as *mut ::core::ffi::c_void,
    );
    return yc;
}
unsafe extern "C" fn rect_closed_getSofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alfMax: ::core::ffi::c_double = RECT_ALFMAX;
    if a / (*xsect).aFull > alfMax {
        return (*xsect).sMax
            + ((*xsect).sFull - (*xsect).sMax) * (a / (*xsect).aFull - alfMax) / (1.0f64 - alfMax);
    }
    return a * pow(xsect_getRofA(xsect, a), 2.0f64 / 3.0f64);
}
unsafe extern "C" fn rect_closed_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut alfMax: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    alfMax = RECT_ALFMAX;
    alpha = a / (*xsect).aFull;
    if alpha > alfMax {
        return ((*xsect).sFull - (*xsect).sMax) / ((1.0f64 - alfMax) * (*xsect).aFull);
    }
    if alpha <= 1.0e-30f64 {
        return generic_getdSdA(xsect, a);
    }
    r = xsect_getRofA(xsect, a);
    return (5.0f64 / 3.0f64 - 2.0f64 / 3.0f64 * (2.0f64 / (*xsect).wMax) * r)
        * pow(r, 2.0f64 / 3.0f64);
}
unsafe extern "C" fn rect_closed_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut p: ::core::ffi::c_double = 0.;
    if a <= 0.0f64 {
        return 0.0f64;
    }
    p = (*xsect).wMax + 2.0f64 * a / (*xsect).wMax;
    if a / (*xsect).aFull > RECT_ALFMAX {
        p += (a / (*xsect).aFull - RECT_ALFMAX) / (1.0f64 - RECT_ALFMAX) * (*xsect).wMax;
    }
    return a / p;
}
unsafe extern "C" fn rect_open_getSofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut y: ::core::ffi::c_double = a / (*xsect).wMax;
    let mut r: ::core::ffi::c_double = a / ((2.0f64 - (*xsect).sBot) * y + (*xsect).wMax);
    return a * pow(r, 2.0f64 / 3.0f64);
}
unsafe extern "C" fn rect_open_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut r: ::core::ffi::c_double = 0.;
    let mut dPdA: ::core::ffi::c_double = 0.;
    if a / (*xsect).aFull <= 1.0e-30f64 {
        return generic_getdSdA(xsect, a);
    }
    r = xsect_getRofA(xsect, a);
    dPdA = (2.0f64 - (*xsect).sBot) / (*xsect).wMax;
    return (5.0f64 / 3.0f64 - 2.0f64 / 3.0f64 * dPdA * r) * pow(r, 2.0f64 / 3.0f64);
}
unsafe extern "C" fn rect_triang_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if a <= (*xsect).aBot {
        return sqrt(a / (*xsect).sBot);
    } else {
        return (*xsect).yBot + (a - (*xsect).aBot) / (*xsect).wMax;
    };
}
unsafe extern "C" fn rect_triang_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut y: ::core::ffi::c_double = 0.;
    let mut p: ::core::ffi::c_double = 0.;
    let mut alf: ::core::ffi::c_double = 0.;
    if a <= 0.0f64 {
        return 0.0f64;
    }
    y = rect_triang_getYofA(xsect, a);
    if y <= (*xsect).yBot {
        return a / (2.0f64 * y * (*xsect).rBot);
    }
    p = 2.0f64 * (*xsect).yBot * (*xsect).rBot + 2.0f64 * (y - (*xsect).yBot);
    alf = a / (*xsect).aFull - RECT_TRIANG_ALFMAX;
    if alf > 0.0f64 {
        p += alf / (1.0f64 - RECT_TRIANG_ALFMAX) * (*xsect).wMax;
    }
    return a / p;
}
unsafe extern "C" fn rect_triang_getSofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alfMax: ::core::ffi::c_double = RECT_TRIANG_ALFMAX;
    if a / (*xsect).aFull > alfMax {
        return (*xsect).sMax
            + ((*xsect).sFull - (*xsect).sMax) * (a / (*xsect).aFull - alfMax) / (1.0f64 - alfMax);
    } else {
        return a * pow(rect_triang_getRofA(xsect, a), 2.0f64 / 3.0f64);
    };
}
unsafe extern "C" fn rect_triang_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut alfMax: ::core::ffi::c_double = 0.;
    let mut dPdA: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    alfMax = RECT_TRIANG_ALFMAX;
    alpha = a / (*xsect).aFull;
    if alpha > alfMax {
        return ((*xsect).sFull - (*xsect).sMax) / ((1.0f64 - alfMax) * (*xsect).aFull);
    }
    if alpha <= 1.0e-30f64 {
        return generic_getdSdA(xsect, a);
    }
    if a > (*xsect).aBot {
        dPdA = 2.0f64 / (*xsect).wMax;
    } else {
        dPdA = (*xsect).rBot / sqrt(a * (*xsect).sBot);
    }
    r = rect_triang_getRofA(xsect, a);
    return (5.0f64 / 3.0f64 - 2.0f64 / 3.0f64 * dPdA * r) * pow(r, 2.0f64 / 3.0f64);
}
unsafe extern "C" fn rect_triang_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y <= (*xsect).yBot {
        return y * y * (*xsect).sBot;
    } else {
        return (*xsect).aBot + (y - (*xsect).yBot) * (*xsect).wMax;
    };
}
unsafe extern "C" fn rect_triang_getRofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut p: ::core::ffi::c_double = 0.;
    let mut a: ::core::ffi::c_double = 0.;
    let mut alf: ::core::ffi::c_double = 0.;
    if y <= (*xsect).yBot {
        return y * (*xsect).sBot / (2.0f64 * (*xsect).rBot);
    }
    a = (*xsect).aBot + (y - (*xsect).yBot) * (*xsect).wMax;
    p = 2.0f64 * (*xsect).yBot * (*xsect).rBot + 2.0f64 * (y - (*xsect).yBot);
    alf = a / (*xsect).aFull - RECT_TRIANG_ALFMAX;
    if alf > 0.0f64 {
        p += alf / (1.0f64 - RECT_TRIANG_ALFMAX) * (*xsect).wMax;
    }
    return a / p;
}
unsafe extern "C" fn rect_triang_getWofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y <= (*xsect).yBot {
        return 2.0f64 * (*xsect).sBot * y;
    } else {
        return (*xsect).wMax;
    };
}
unsafe extern "C" fn rect_round_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = 0.;
    if a > (*xsect).aBot {
        return (*xsect).yBot + (a - (*xsect).aBot) / (*xsect).wMax;
    }
    alpha = a / (PI * (*xsect).rBot * (*xsect).rBot);
    if alpha < 0.04f64 {
        return 2.0f64 * (*xsect).rBot * getYcircular(alpha);
    }
    return 2.0f64
        * (*xsect).rBot
        * lookup(
            alpha,
            &raw mut Y_Circ as *mut ::core::ffi::c_double,
            N_Y_Circ,
        );
}
unsafe extern "C" fn rect_round_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut y1: ::core::ffi::c_double = 0.;
    let mut theta1: ::core::ffi::c_double = 0.;
    let mut p: ::core::ffi::c_double = 0.;
    let mut arg: ::core::ffi::c_double = 0.;
    if a <= 0.0f64 {
        return 0.0f64;
    }
    if a > (*xsect).aBot {
        y1 = (a - (*xsect).aBot) / (*xsect).wMax;
        theta1 = 2.0f64 * asin((*xsect).wMax / 2.0f64 / (*xsect).rBot);
        p = (*xsect).rBot * theta1 + 2.0f64 * y1;
        arg = a / (*xsect).aFull - RECT_ROUND_ALFMAX;
        if arg > 0.0f64 {
            p += arg / (1.0f64 - RECT_ROUND_ALFMAX) * (*xsect).wMax;
        }
        return a / p;
    }
    y1 = rect_round_getYofA(xsect, a);
    theta1 = 2.0f64 * acos(1.0f64 - y1 / (*xsect).rBot);
    p = (*xsect).rBot * theta1;
    return a / p;
}
unsafe extern "C" fn rect_round_getSofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut aFull: ::core::ffi::c_double = 0.;
    let mut sFull: ::core::ffi::c_double = 0.;
    let mut alfMax: ::core::ffi::c_double = RECT_ROUND_ALFMAX;
    if a / (*xsect).aFull > alfMax {
        return (*xsect).sMax
            + ((*xsect).sFull - (*xsect).sMax) * (a / (*xsect).aFull - alfMax) / (1.0f64 - alfMax);
    } else if a > (*xsect).aBot {
        return a * pow(xsect_getRofA(xsect, a), 2.0f64 / 3.0f64);
    } else {
        aFull = PI * (*xsect).rBot * (*xsect).rBot;
        alpha = a / aFull;
        sFull = (*xsect).sBot;
        if alpha < 0.04f64 {
            return sFull * getScircular(alpha);
        } else {
            return sFull
                * lookup(
                    alpha,
                    &raw mut S_Circ as *mut ::core::ffi::c_double,
                    N_S_Circ,
                );
        }
    };
}
unsafe extern "C" fn rect_round_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alfMax: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    let mut dPdA: ::core::ffi::c_double = 0.;
    alfMax = RECT_ROUND_ALFMAX;
    if a / (*xsect).aFull > alfMax {
        return ((*xsect).sFull - (*xsect).sMax) / ((1.0f64 - alfMax) * (*xsect).aFull);
    } else if a > (*xsect).aBot {
        r = rect_round_getRofA(xsect, a);
        dPdA = 2.0f64 / (*xsect).wMax;
        return (5.0f64 / 3.0f64 - 2.0f64 / 3.0f64 * dPdA * r) * pow(r, 2.0f64 / 3.0f64);
    } else {
        return generic_getdSdA(xsect, a);
    };
}
unsafe extern "C" fn rect_round_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut theta1: ::core::ffi::c_double = 0.;
    if y > (*xsect).yBot {
        return (*xsect).aBot + (y - (*xsect).yBot) * (*xsect).wMax;
    }
    theta1 = 2.0f64 * acos(1.0f64 - y / (*xsect).rBot);
    return 0.5f64 * (*xsect).rBot * (*xsect).rBot * (theta1 - sin(theta1));
}
unsafe extern "C" fn rect_round_getRofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut theta1: ::core::ffi::c_double = 0.;
    if y <= 0.0f64 {
        return 0.0f64;
    }
    if y > (*xsect).yBot {
        return rect_round_getRofA(xsect, rect_round_getAofY(xsect, y));
    }
    theta1 = 2.0f64 * acos(1.0f64 - y / (*xsect).rBot);
    return 0.5f64 * (*xsect).rBot * (1.0f64 - sin(theta1)) / theta1;
}
unsafe extern "C" fn rect_round_getWofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y > (*xsect).yBot {
        return (*xsect).wMax;
    }
    return 2.0f64 * sqrt(y * (2.0f64 * (*xsect).rBot - y));
}
unsafe extern "C" fn mod_basket_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    if a <= (*xsect).aFull - (*xsect).aBot {
        return a / (*xsect).wMax;
    }
    alpha = ((*xsect).aFull - a) / (PI * (*xsect).rBot * (*xsect).rBot);
    if alpha < 0.04f64 {
        y1 = getYcircular(alpha);
    } else {
        y1 = lookup(
            alpha,
            &raw mut Y_Circ as *mut ::core::ffi::c_double,
            N_Y_Circ,
        );
    }
    y1 = 2.0f64 * (*xsect).rBot * y1;
    return (*xsect).yFull - y1;
}
unsafe extern "C" fn mod_basket_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut y1: ::core::ffi::c_double = 0.;
    let mut p: ::core::ffi::c_double = 0.;
    let mut theta1: ::core::ffi::c_double = 0.;
    if a <= (*xsect).aFull - (*xsect).aBot {
        return a / ((*xsect).wMax + 2.0f64 * a / (*xsect).wMax);
    }
    y1 = (*xsect).yFull - mod_basket_getYofA(xsect, a);
    theta1 = 2.0f64 * acos(1.0f64 - y1 / (*xsect).rBot);
    p = ((*xsect).sBot - theta1) * (*xsect).rBot;
    y1 = (*xsect).yFull - (*xsect).yBot;
    p = p + 2.0f64 * y1 + (*xsect).wMax;
    return a / p;
}
unsafe extern "C" fn mod_basket_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut r: ::core::ffi::c_double = 0.;
    let mut dPdA: ::core::ffi::c_double = 0.;
    if a <= (*xsect).aFull - (*xsect).aBot && a / (*xsect).aFull > 1.0e-30f64 {
        r = a / ((*xsect).wMax + 2.0f64 * a / (*xsect).wMax);
        dPdA = 2.0f64 / (*xsect).wMax;
        return (5.0f64 / 3.0f64 - 2.0f64 / 3.0f64 * dPdA * r) * pow(r, 2.0f64 / 3.0f64);
    } else {
        return generic_getdSdA(xsect, a);
    };
}
unsafe extern "C" fn mod_basket_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a1: ::core::ffi::c_double = 0.;
    let mut theta1: ::core::ffi::c_double = 0.;
    let mut y1: ::core::ffi::c_double = 0.;
    if y <= (*xsect).yFull - (*xsect).yBot {
        return y * (*xsect).wMax;
    }
    y1 = (*xsect).yFull - y;
    theta1 = 2.0f64 * acos(1.0f64 - y1 / (*xsect).rBot);
    a1 = 0.5f64 * (*xsect).rBot * (*xsect).rBot * (theta1 - sin(theta1));
    return (*xsect).aFull - a1;
}
unsafe extern "C" fn mod_basket_getWofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut y1: ::core::ffi::c_double = 0.;
    if y <= 0.0f64 {
        return 0.0f64;
    }
    if y <= (*xsect).yFull - (*xsect).yBot {
        return (*xsect).wMax;
    }
    y1 = (*xsect).yFull - y;
    return 2.0f64 * sqrt(y1 * (2.0f64 * (*xsect).rBot - y1));
}
unsafe extern "C" fn trapez_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if (*xsect).sBot == 0.0f64 {
        return a / (*xsect).yBot;
    }
    return (sqrt((*xsect).yBot * (*xsect).yBot + 4.0f64 * (*xsect).sBot * a) - (*xsect).yBot)
        / (2.0f64 * (*xsect).sBot);
}
unsafe extern "C" fn trapez_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return a / ((*xsect).yBot + trapez_getYofA(xsect, a) * (*xsect).rBot);
}
unsafe extern "C" fn trapez_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut r: ::core::ffi::c_double = 0.;
    let mut dPdA: ::core::ffi::c_double = 0.;
    if a / (*xsect).aFull <= 1.0e-30f64 {
        return generic_getdSdA(xsect, a);
    }
    r = trapez_getRofA(xsect, a);
    dPdA = (*xsect).rBot / sqrt((*xsect).yBot * (*xsect).yBot + 4.0f64 * (*xsect).sBot * a);
    return (5.0f64 / 3.0f64 - 2.0f64 / 3.0f64 * dPdA * r) * pow(r, 2.0f64 / 3.0f64);
}
unsafe extern "C" fn trapez_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return ((*xsect).yBot + (*xsect).sBot * y) * y;
}
unsafe extern "C" fn trapez_getRofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y == 0.0f64 {
        return 0.0f64;
    }
    return trapez_getAofY(xsect, y) / ((*xsect).yBot + y * (*xsect).rBot);
}
unsafe extern "C" fn trapez_getWofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return (*xsect).yBot + 2.0f64 * y * (*xsect).sBot;
}
unsafe extern "C" fn triang_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return sqrt(a / (*xsect).sBot);
}
unsafe extern "C" fn triang_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return a / (2.0f64 * triang_getYofA(xsect, a) * (*xsect).rBot);
}
unsafe extern "C" fn triang_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut r: ::core::ffi::c_double = 0.;
    let mut dPdA: ::core::ffi::c_double = 0.;
    if a / (*xsect).aFull <= 1.0e-30f64 {
        return generic_getdSdA(xsect, a);
    }
    r = triang_getRofA(xsect, a);
    dPdA = (*xsect).rBot / sqrt(a * (*xsect).sBot);
    return (5.0f64 / 3.0f64 - 2.0f64 / 3.0f64 * dPdA * r) * pow(r, 2.0f64 / 3.0f64);
}
unsafe extern "C" fn triang_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return y * y * (*xsect).sBot;
}
unsafe extern "C" fn triang_getRofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return y * (*xsect).sBot / (2.0f64 * (*xsect).rBot);
}
unsafe extern "C" fn triang_getWofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return 2.0f64 * (*xsect).sBot * y;
}
unsafe extern "C" fn parab_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return pow(3.0f64 / 4.0f64 * a / (*xsect).rBot, 2.0f64 / 3.0f64);
}
unsafe extern "C" fn parab_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if a <= 0.0f64 {
        return 0.0f64;
    }
    return a / parab_getPofY(xsect, parab_getYofA(xsect, a));
}
unsafe extern "C" fn parab_getPofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut x: ::core::ffi::c_double = 2.0f64 * sqrt(y) / (*xsect).rBot;
    let mut t: ::core::ffi::c_double = sqrt(1.0f64 + x * x);
    return 0.5f64 * (*xsect).rBot * (*xsect).rBot * (x * t + log(x + t));
}
unsafe extern "C" fn parab_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return 4.0f64 / 3.0f64 * (*xsect).rBot * y * sqrt(y);
}
unsafe extern "C" fn parab_getRofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y <= 0.0f64 {
        return 0.0f64;
    }
    return parab_getAofY(xsect, y) / parab_getPofY(xsect, y);
}
unsafe extern "C" fn parab_getWofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return 2.0f64 * (*xsect).rBot * sqrt(y);
}
unsafe extern "C" fn powerfunc_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return pow(a / (*xsect).rBot, 1.0f64 / ((*xsect).sBot + 1.0f64));
}
unsafe extern "C" fn powerfunc_getRofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if a <= 0.0f64 {
        return 0.0f64;
    }
    return a / powerfunc_getPofY(xsect, powerfunc_getYofA(xsect, a));
}
unsafe extern "C" fn powerfunc_getPofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut dy1: ::core::ffi::c_double = 0.02f64 * (*xsect).yFull;
    let mut h: ::core::ffi::c_double = ((*xsect).sBot + 1.0f64) * (*xsect).rBot / 2.0f64;
    let mut m: ::core::ffi::c_double = (*xsect).sBot;
    let mut p: ::core::ffi::c_double = 0.0f64;
    let mut y1: ::core::ffi::c_double = 0.0f64;
    let mut x1: ::core::ffi::c_double = 0.0f64;
    let mut x2: ::core::ffi::c_double = 0.;
    let mut y2: ::core::ffi::c_double = 0.;
    let mut dx: ::core::ffi::c_double = 0.;
    let mut dy: ::core::ffi::c_double = 0.;
    loop {
        y2 = y1 + dy1;
        if y2 > y {
            y2 = y;
        }
        x2 = h * pow(y2, m);
        dx = x2 - x1;
        dy = y2 - y1;
        p += sqrt(dx * dx + dy * dy);
        x1 = x2;
        y1 = y2;
        if !(y2 < y) {
            break;
        }
    }
    return 2.0f64 * p;
}
unsafe extern "C" fn powerfunc_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return (*xsect).rBot * pow(y, (*xsect).sBot + 1.0f64);
}
unsafe extern "C" fn powerfunc_getRofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if y <= 0.0f64 {
        return 0.0f64;
    }
    return powerfunc_getAofY(xsect, y) / powerfunc_getPofY(xsect, y);
}
unsafe extern "C" fn powerfunc_getWofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return ((*xsect).sBot + 1.0f64) * (*xsect).rBot * pow(y, (*xsect).sBot);
}
unsafe extern "C" fn circ_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = a / (*xsect).aFull;
    if alpha < 0.04f64 {
        return (*xsect).yFull * getYcircular(alpha);
    } else {
        return (*xsect).yFull
            * lookup(
                alpha,
                &raw mut Y_Circ as *mut ::core::ffi::c_double,
                N_Y_Circ,
            );
    };
}
unsafe extern "C" fn circ_getAofS(
    mut xsect: *mut TXsect,
    mut s: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut psi: ::core::ffi::c_double = s / (*xsect).sFull;
    if psi == 0.0f64 {
        return 0.0f64;
    }
    if psi >= 1.0f64 {
        return (*xsect).aFull;
    }
    if psi <= 0.015f64 {
        return (*xsect).aFull * getAcircular(psi);
    } else {
        return (*xsect).aFull
            * invLookup(psi, &raw mut S_Circ as *mut ::core::ffi::c_double, N_S_Circ);
    };
}
unsafe extern "C" fn circ_getSofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = a / (*xsect).aFull;
    if alpha < 0.04f64 {
        return (*xsect).sFull * getScircular(alpha);
    } else {
        return (*xsect).sFull
            * lookup(
                alpha,
                &raw mut S_Circ as *mut ::core::ffi::c_double,
                N_S_Circ,
            );
    };
}
unsafe extern "C" fn circ_getdSdA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut theta: ::core::ffi::c_double = 0.;
    let mut p: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    let mut dPdA: ::core::ffi::c_double = 0.;
    alpha = a / (*xsect).aFull;
    if alpha <= 1.0e-30f64 {
        return 1.0e-30f64;
    } else if alpha < 0.04f64 {
        theta = getThetaOfAlpha(alpha);
        p = theta * (*xsect).yFull / 2.0f64;
        r = a / p;
        dPdA = 4.0f64 / (*xsect).yFull / (1.0f64 - cos(theta));
        return (5.0f64 / 3.0f64 - 2.0f64 / 3.0f64 * dPdA * r) * pow(r, 2.0f64 / 3.0f64);
    } else {
        return tabular_getdSdA(
            xsect,
            a,
            &raw mut S_Circ as *mut ::core::ffi::c_double,
            N_S_Circ,
        );
    };
}
unsafe extern "C" fn circ_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut yNorm: ::core::ffi::c_double = 0.;
    yNorm = y / (*xsect).yFull;
    return (*xsect).aFull
        * lookup(
            yNorm,
            &raw mut A_Circ as *mut ::core::ffi::c_double,
            N_A_Circ,
        );
}
unsafe extern "C" fn filled_circ_getYofA(
    mut xsect: *mut TXsect,
    mut a: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut y: ::core::ffi::c_double = 0.;
    (*xsect).yFull += (*xsect).yBot;
    (*xsect).aFull += (*xsect).aBot;
    a += (*xsect).aBot;
    y = circ_getYofA(xsect, a);
    y -= (*xsect).yBot;
    (*xsect).yFull -= (*xsect).yBot;
    (*xsect).aFull -= (*xsect).aBot;
    return y;
}
unsafe extern "C" fn filled_circ_getAofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a: ::core::ffi::c_double = 0.;
    (*xsect).yFull += (*xsect).yBot;
    (*xsect).aFull += (*xsect).aBot;
    y += (*xsect).yBot;
    a = circ_getAofY(xsect, y);
    a -= (*xsect).aBot;
    (*xsect).yFull -= (*xsect).yBot;
    (*xsect).aFull -= (*xsect).aBot;
    return a;
}
unsafe extern "C" fn filled_circ_getRofY(
    mut xsect: *mut TXsect,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut a: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    let mut p: ::core::ffi::c_double = 0.;
    (*xsect).yFull += (*xsect).yBot;
    (*xsect).aFull += (*xsect).aBot;
    y += (*xsect).yBot;
    a = circ_getAofY(xsect, y);
    r = 0.25f64
        * (*xsect).yFull
        * lookup(
            y / (*xsect).yFull,
            &raw mut R_Circ as *mut ::core::ffi::c_double,
            N_R_Circ,
        );
    p = a / r;
    a = a - (*xsect).aBot;
    p = p - (*xsect).rBot + (*xsect).sBot;
    r = a / p;
    (*xsect).yFull -= (*xsect).yBot;
    (*xsect).aFull -= (*xsect).aBot;
    return r;
}
unsafe extern "C" fn getYcircular(mut alpha: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut theta: ::core::ffi::c_double = 0.;
    if alpha >= 1.0f64 {
        return 1.0f64;
    }
    if alpha <= 0.0f64 {
        return 0.0f64;
    }
    if alpha <= 1.0e-5f64 {
        theta = pow(37.6911f64 * alpha, 1.0f64 / 3.0f64);
        return theta * theta / 16.0f64;
    }
    theta = getThetaOfAlpha(alpha);
    return (1.0f64 - cos(theta / 2.0f64)) / 2.0f64;
}
unsafe extern "C" fn getScircular(mut alpha: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut theta: ::core::ffi::c_double = 0.;
    if alpha >= 1.0f64 {
        return 1.0f64;
    }
    if alpha <= 0.0f64 {
        return 0.0f64;
    }
    if alpha <= 1.0e-5f64 {
        theta = pow(37.6911f64 * alpha, 1.0f64 / 3.0f64);
        return pow(theta, 13.0f64 / 3.0f64) / 124.4797f64;
    }
    theta = getThetaOfAlpha(alpha);
    return pow(theta - sin(theta), 5.0f64 / 3.0f64) / (2.0f64 * PI) / pow(theta, 2.0f64 / 3.0f64);
}
unsafe extern "C" fn getAcircular(mut psi: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut theta: ::core::ffi::c_double = 0.;
    if psi >= 1.0f64 {
        return 1.0f64;
    }
    if psi <= 0.0f64 {
        return 0.0f64;
    }
    if psi <= 1.0e-6f64 {
        theta = pow(124.4797f64 * psi, 3.0f64 / 13.0f64);
        return theta * theta * theta / 37.6911f64;
    }
    theta = getThetaOfPsi(psi);
    return (theta - sin(theta)) / (2.0f64 * PI);
}
unsafe extern "C" fn getThetaOfAlpha(mut alpha: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut theta: ::core::ffi::c_double = 0.;
    let mut theta1: ::core::ffi::c_double = 0.;
    let mut ap: ::core::ffi::c_double = 0.;
    let mut d: ::core::ffi::c_double = 0.;
    if alpha > 0.04f64 {
        theta = 1.2f64 + 5.08f64 * (alpha - 0.04f64) / 0.96f64;
    } else {
        theta = 0.031715f64 - 12.79384f64 * alpha + 8.28479f64 * sqrt(alpha);
    }
    theta1 = theta;
    ap = 2.0f64 * PI * alpha;
    k = 1 as ::core::ffi::c_int;
    while k <= 40 as ::core::ffi::c_int {
        d = -(ap - theta + sin(theta)) / (1.0f64 - cos(theta));
        if d > 1.0f64 {
            d = if d >= 0.0f64 {
                fabs(1.0f64)
            } else {
                -fabs(1.0f64)
            };
        }
        theta = theta - d;
        if fabs(d) <= 0.0001f64 {
            return theta;
        }
        k += 1;
    }
    return theta1;
}
unsafe extern "C" fn getThetaOfPsi(mut psi: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut k: ::core::ffi::c_int = 0;
    let mut theta: ::core::ffi::c_double = 0.;
    let mut theta1: ::core::ffi::c_double = 0.;
    let mut ap: ::core::ffi::c_double = 0.;
    let mut tt: ::core::ffi::c_double = 0.;
    let mut tt23: ::core::ffi::c_double = 0.;
    let mut t3: ::core::ffi::c_double = 0.;
    let mut d: ::core::ffi::c_double = 0.;
    if psi > 0.90f64 {
        theta = 4.17f64 + 1.12f64 * (psi - 0.90f64) / 0.176f64;
    } else if psi > 0.5f64 {
        theta = 3.14f64 + 1.03f64 * (psi - 0.5f64) / 0.4f64;
    } else if psi > 0.015f64 {
        theta = 1.2f64 + 1.94f64 * (psi - 0.015f64) / 0.485f64;
    } else {
        theta = 0.12103f64 - 55.5075f64 * psi + 15.62254f64 * sqrt(psi);
    }
    theta1 = theta;
    ap = 2.0f64 * PI * psi;
    k = 1 as ::core::ffi::c_int;
    while k <= 40 as ::core::ffi::c_int {
        theta = fabs(theta);
        tt = theta - sin(theta);
        tt23 = pow(tt, 2.0f64 / 3.0f64);
        t3 = pow(theta, 1.0f64 / 3.0f64);
        d = ap * theta / t3 - tt * tt23;
        d = d / (ap * (2.0f64 / 3.0f64) / t3 - 5.0f64 / 3.0f64 * tt23 * (1.0f64 - cos(theta)));
        theta = theta - d;
        if fabs(d) <= 0.0001f64 {
            return theta;
        }
        k += 1;
    }
    return theta1;
}
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TINY: ::core::ffi::c_double = 1.0E-6f64;
pub const PI: ::core::ffi::c_double = 3.141592654f64;
pub const GRAVITY: ::core::ffi::c_double = 32.2f64;
#[no_mangle]
pub static mut N_A_Circ: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut A_Circ: [::core::ffi::c_double; 51] = [
    0.0f64,
    0.00471f64,
    0.0134f64,
    0.024446f64,
    0.0374f64,
    0.05208f64,
    0.0680f64,
    0.08505f64,
    0.1033f64,
    0.12236f64,
    0.1423f64,
    0.16310f64,
    0.1845f64,
    0.20665f64,
    0.2292f64,
    0.25236f64,
    0.2759f64,
    0.29985f64,
    0.3242f64,
    0.34874f64,
    0.3736f64,
    0.39878f64,
    0.4237f64,
    0.44907f64,
    0.4745f64,
    0.500f64,
    0.5255f64,
    0.55093f64,
    0.5763f64,
    0.60135f64,
    0.6264f64,
    0.65126f64,
    0.6758f64,
    0.70015f64,
    0.7241f64,
    0.74764f64,
    0.7708f64,
    0.79335f64,
    0.8154f64,
    0.83690f64,
    0.8576f64,
    0.87764f64,
    0.8967f64,
    0.91495f64,
    0.9320f64,
    0.94792f64,
    0.9626f64,
    0.97555f64,
    0.9866f64,
    0.99516f64,
    1.000f64,
];
#[no_mangle]
pub static mut N_R_Circ: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut R_Circ: [::core::ffi::c_double; 51] = [
    0.0100f64, 0.0528f64, 0.1048f64, 0.1556f64, 0.2052f64, 0.254f64, 0.3016f64, 0.3484f64,
    0.3944f64, 0.4388f64, 0.4824f64, 0.5248f64, 0.5664f64, 0.6064f64, 0.6456f64, 0.6836f64,
    0.7204f64, 0.7564f64, 0.7912f64, 0.8244f64, 0.8568f64, 0.888f64, 0.9176f64, 0.9464f64,
    0.9736f64, 1.00f64, 1.024f64, 1.048f64, 1.070f64, 1.0912f64, 1.110f64, 1.1272f64, 1.144f64,
    1.1596f64, 1.174f64, 1.1848f64, 1.194f64, 1.2024f64, 1.210f64, 1.2148f64, 1.217f64, 1.2172f64,
    1.215f64, 1.2104f64, 1.203f64, 1.192f64, 1.178f64, 1.1584f64, 1.132f64, 1.094f64, 1.000f64,
];
#[no_mangle]
pub static mut N_Y_Circ: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Y_Circ: [::core::ffi::c_double; 51] = [
    0.0f64, 0.05236f64, 0.08369f64, 0.11025f64, 0.13423f64, 0.15643f64, 0.17755f64, 0.19772f64,
    0.21704f64, 0.23581f64, 0.25412f64, 0.27194f64, 0.28948f64, 0.30653f64, 0.32349f64, 0.34017f64,
    0.35666f64, 0.37298f64, 0.38915f64, 0.40521f64, 0.42117f64, 0.43704f64, 0.45284f64, 0.46858f64,
    0.4843f64, 0.50000f64, 0.51572f64, 0.53146f64, 0.54723f64, 0.56305f64, 0.57892f64, 0.59487f64,
    0.61093f64, 0.62710f64, 0.64342f64, 0.65991f64, 0.67659f64, 0.69350f64, 0.71068f64, 0.72816f64,
    0.74602f64, 0.76424f64, 0.78297f64, 0.80235f64, 0.82240f64, 0.84353f64, 0.86563f64, 0.88970f64,
    0.91444f64, 0.94749f64, 1.0f64,
];
#[no_mangle]
pub static mut N_S_Circ: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut S_Circ: [::core::ffi::c_double; 51] = [
    0.0f64, 0.00529f64, 0.01432f64, 0.02559f64, 0.03859f64, 0.05304f64, 0.06877f64, 0.08551f64,
    0.10326f64, 0.12195f64, 0.14144f64, 0.16162f64, 0.18251f64, 0.2041f64, 0.22636f64, 0.24918f64,
    0.27246f64, 0.29614f64, 0.32027f64, 0.34485f64, 0.36989f64, 0.39531f64, 0.42105f64, 0.44704f64,
    0.47329f64, 0.4998f64, 0.52658f64, 0.55354f64, 0.58064f64, 0.60777f64, 0.63499f64, 0.66232f64,
    0.68995f64, 0.7177f64, 0.74538f64, 0.77275f64, 0.79979f64, 0.82658f64, 0.8532f64, 0.87954f64,
    0.90546f64, 0.93095f64, 0.95577f64, 0.97976f64, 1.00291f64, 1.02443f64, 1.04465f64, 1.06135f64,
    1.08208f64, 1.07662f64, 1.0f64,
];
#[no_mangle]
pub static mut N_W_Circ: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_Circ: [::core::ffi::c_double; 51] = [
    0.0f64, 0.2800f64, 0.3919f64, 0.4750f64, 0.5426f64, 0.6000f64, 0.6499f64, 0.6940f64, 0.7332f64,
    0.7684f64, 0.8000f64, 0.8285f64, 0.8542f64, 0.8773f64, 0.8980f64, 0.9165f64, 0.9330f64,
    0.9474f64, 0.9600f64, 0.9708f64, 0.9798f64, 0.9871f64, 0.9928f64, 0.9968f64, 0.9992f64,
    1.000f64, 0.9992f64, 0.9968f64, 0.9928f64, 0.9871f64, 0.9798f64, 0.9708f64, 0.9600f64,
    0.9474f64, 0.9330f64, 0.9165f64, 0.8980f64, 0.8773f64, 0.8542f64, 0.8285f64, 0.8000f64,
    0.7684f64, 0.7332f64, 0.6940f64, 0.6499f64, 0.6000f64, 0.5426f64, 0.4750f64, 0.3919f64,
    0.2800f64, 0.0f64,
];
#[no_mangle]
pub static mut N_A_Egg: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut A_Egg: [::core::ffi::c_double; 26] = [
    0.0000f64, 0.0116f64, 0.0316f64, 0.0550f64, 0.0850f64, 0.1200f64, 0.1555f64, 0.1900f64,
    0.2250f64, 0.2750f64, 0.3200f64, 0.3700f64, 0.4200f64, 0.4700f64, 0.5150f64, 0.5700f64,
    0.6200f64, 0.6800f64, 0.7300f64, 0.7800f64, 0.8350f64, 0.8850f64, 0.9250f64, 0.9550f64,
    0.9800f64, 1.000f64,
];
#[no_mangle]
pub static mut N_R_Egg: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut R_Egg: [::core::ffi::c_double; 26] = [
    0.0100f64, 0.1300f64, 0.2440f64, 0.3020f64, 0.3860f64, 0.4650f64, 0.5360f64, 0.6110f64,
    0.6760f64, 0.7350f64, 0.7910f64, 0.8540f64, 0.9040f64, 0.9410f64, 1.008f64, 1.045f64, 1.076f64,
    1.115f64, 1.146f64, 1.162f64, 1.186f64, 1.193f64, 1.186f64, 1.162f64, 1.107f64, 1.000f64,
];
#[no_mangle]
pub static mut N_Y_Egg: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Y_Egg: [::core::ffi::c_double; 51] = [
    0.0f64, 0.04912f64, 0.08101f64, 0.11128f64, 0.14161f64, 0.16622f64, 0.18811f64, 0.21356f64,
    0.23742f64, 0.25742f64, 0.27742f64, 0.29741f64, 0.31742f64, 0.33742f64, 0.35747f64, 0.37364f64,
    0.4f64, 0.41697f64, 0.43372f64, 0.45f64, 0.46374f64, 0.47747f64, 0.49209f64, 0.50989f64,
    0.53015f64, 0.55f64, 0.56429f64, 0.57675f64, 0.58834f64, 0.6f64, 0.61441f64, 0.62967f64,
    0.64582f64, 0.66368f64, 0.68209f64, 0.7f64, 0.71463f64, 0.72807f64, 0.74074f64, 0.75296f64,
    0.765f64, 0.77784f64, 0.79212f64, 0.80945f64, 0.82936f64, 0.85f64, 0.86731f64, 0.88769f64,
    0.914f64, 0.95f64, 1.0f64,
];
#[no_mangle]
pub static mut N_S_Egg: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut S_Egg: [::core::ffi::c_double; 51] = [
    0.0f64, 0.00295f64, 0.01331f64, 0.02629f64, 0.04f64, 0.05657f64, 0.075f64, 0.09432f64,
    0.11473f64, 0.13657f64, 0.15894f64, 0.1803f64, 0.20036f64, 0.22f64, 0.23919f64, 0.25896f64,
    0.28f64, 0.30504f64, 0.33082f64, 0.35551f64, 0.37692f64, 0.39809f64, 0.42f64, 0.44625f64,
    0.47321f64, 0.5f64, 0.52255f64, 0.54481f64, 0.56785f64, 0.59466f64, 0.62485f64, 0.65518f64,
    0.68181f64, 0.70415f64, 0.72585f64, 0.74819f64, 0.77482f64, 0.80515f64, 0.83534f64, 0.86193f64,
    0.88465f64, 0.9069f64, 0.93f64, 0.95866f64, 0.98673f64, 1.01238f64, 1.03396f64, 1.05f64,
    1.06517f64, 1.0538f64, 1.0f64,
];
#[no_mangle]
pub static mut N_W_Egg: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_Egg: [::core::ffi::c_double; 26] = [
    0.0f64, 0.3250f64, 0.4270f64, 0.5080f64, 0.5820f64, 0.6420f64, 0.6960f64, 0.7460f64, 0.7910f64,
    0.8360f64, 0.8660f64, 0.8960f64, 0.9260f64, 0.9560f64, 0.9700f64, 0.9850f64, 1.000f64,
    0.9850f64, 0.9700f64, 0.9400f64, 0.8960f64, 0.8360f64, 0.7640f64, 0.6420f64, 0.3100f64, 0.0f64,
];
#[no_mangle]
pub static mut N_A_Horseshoe: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut A_Horseshoe: [::core::ffi::c_double; 26] = [
    0.0000f64, 0.0181f64, 0.0508f64, 0.0908f64, 0.1326f64, 0.1757f64, 0.2201f64, 0.2655f64,
    0.3118f64, 0.3587f64, 0.4064f64, 0.4542f64, 0.5023f64, 0.5506f64, 0.5987f64, 0.6462f64,
    0.6931f64, 0.7387f64, 0.7829f64, 0.8253f64, 0.8652f64, 0.9022f64, 0.9356f64, 0.9645f64,
    0.9873f64, 1.000f64,
];
#[no_mangle]
pub static mut N_R_Horseshoe: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut R_Horseshoe: [::core::ffi::c_double; 26] = [
    0.0100f64, 0.1040f64, 0.2065f64, 0.3243f64, 0.4322f64, 0.5284f64, 0.6147f64, 0.6927f64,
    0.7636f64, 0.8268f64, 0.8873f64, 0.9417f64, 0.9905f64, 1.036f64, 1.077f64, 1.113f64, 1.143f64,
    1.169f64, 1.189f64, 1.202f64, 1.208f64, 1.206f64, 1.195f64, 1.170f64, 1.126f64, 1.000f64,
];
#[no_mangle]
pub static mut N_Y_Horseshoe: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Y_Horseshoe: [::core::ffi::c_double; 51] = [
    0.0f64, 0.04146f64, 0.07033f64, 0.09098f64, 0.10962f64, 0.12921f64, 0.14813f64, 0.16701f64,
    0.18565f64, 0.20401f64, 0.22211f64, 0.23998f64, 0.25769f64, 0.27524f64, 0.29265f64, 0.3099f64,
    0.32704f64, 0.34406f64, 0.36101f64, 0.3779f64, 0.39471f64, 0.41147f64, 0.42818f64, 0.44484f64,
    0.46147f64, 0.47807f64, 0.49468f64, 0.51134f64, 0.52803f64, 0.54474f64, 0.56138f64, 0.57804f64,
    0.59478f64, 0.61171f64, 0.62881f64, 0.64609f64, 0.6635f64, 0.68111f64, 0.69901f64, 0.71722f64,
    0.73583f64, 0.7549f64, 0.77447f64, 0.79471f64, 0.81564f64, 0.83759f64, 0.86067f64, 0.88557f64,
    0.91159f64, 0.9452f64, 1.0f64,
];
#[no_mangle]
pub static mut N_S_Horseshoe: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut S_Horseshoe: [::core::ffi::c_double; 51] = [
    0.0f64, 0.00467f64, 0.01237f64, 0.02268f64, 0.03515f64, 0.04943f64, 0.06525f64, 0.08212f64,
    0.10005f64, 0.11891f64, 0.13856f64, 0.15896f64, 0.18004f64, 0.20172f64, 0.22397f64, 0.24677f64,
    0.27006f64, 0.2938f64, 0.3179f64, 0.34237f64, 0.3672f64, 0.39239f64, 0.41792f64, 0.44374f64,
    0.46984f64, 0.49619f64, 0.52276f64, 0.5495f64, 0.5764f64, 0.60345f64, 0.63065f64, 0.65795f64,
    0.68531f64, 0.71271f64, 0.74009f64, 0.76738f64, 0.79451f64, 0.82144f64, 0.84814f64, 0.8745f64,
    0.90057f64, 0.92652f64, 0.95244f64, 0.97724f64, 0.99988f64, 1.02048f64, 1.03989f64, 1.05698f64,
    1.07694f64, 1.07562f64, 1.0f64,
];
#[no_mangle]
pub static mut N_W_Horseshoe: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_Horseshoe: [::core::ffi::c_double; 26] = [
    0.0f64, 0.5878f64, 0.8772f64, 0.8900f64, 0.9028f64, 0.9156f64, 0.9284f64, 0.9412f64, 0.9540f64,
    0.9668f64, 0.9798f64, 0.9928f64, 0.9992f64, 0.9992f64, 0.9928f64, 0.9798f64, 0.9600f64,
    0.9330f64, 0.8980f64, 0.8542f64, 0.8000f64, 0.7332f64, 0.6499f64, 0.5426f64, 0.3919f64, 0.0f64,
];
#[no_mangle]
pub static mut N_Y_Gothic: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Y_Gothic: [::core::ffi::c_double; 51] = [
    0.0f64, 0.04522f64, 0.07825f64, 0.10646f64, 0.12645f64, 0.14645f64, 0.16787f64, 0.18641f64,
    0.20129f64, 0.22425f64, 0.24129f64, 0.25624f64, 0.27344f64, 0.29097f64, 0.30529f64, 0.32607f64,
    0.33755f64, 0.35073f64, 0.36447f64, 0.37558f64, 0.4f64, 0.4181f64, 0.43648f64, 0.45374f64,
    0.46805f64, 0.48195f64, 0.49626f64, 0.51352f64, 0.5319f64, 0.55f64, 0.56416f64, 0.57787f64,
    0.59224f64, 0.6095f64, 0.62941f64, 0.65f64, 0.67064f64, 0.69055f64, 0.70721f64, 0.72031f64,
    0.73286f64, 0.74632f64, 0.76432f64, 0.78448f64, 0.80421f64, 0.82199f64, 0.84363f64, 0.87423f64,
    0.90617f64, 0.93827f64, 1.0f64,
];
#[no_mangle]
pub static mut N_S_Gothic: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut S_Gothic: [::core::ffi::c_double; 51] = [
    0.0f64, 0.005f64, 0.0174f64, 0.03098f64, 0.04272f64, 0.055f64, 0.0698f64, 0.0862f64,
    0.10461f64, 0.12463f64, 0.145f64, 0.16309f64, 0.18118f64, 0.2f64, 0.22181f64, 0.24487f64,
    0.26888f64, 0.2938f64, 0.31901f64, 0.34389f64, 0.36564f64, 0.38612f64, 0.4072f64, 0.43f64,
    0.45868f64, 0.48895f64, 0.52f64, 0.55032f64, 0.5804f64, 0.61f64, 0.63762f64, 0.66505f64,
    0.6929f64, 0.72342f64, 0.75467f64, 0.785f64, 0.81165f64, 0.83654f64, 0.86f64, 0.88253f64,
    0.90414f64, 0.925f64, 0.94486f64, 0.96475f64, 0.98567f64, 1.00833f64, 1.03f64, 1.0536f64,
    1.065f64, 1.055f64, 1.0f64,
];
#[no_mangle]
pub static mut N_W_Gothic: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_Gothic: [::core::ffi::c_double; 21] = [
    0.0f64, 0.286f64, 0.643f64, 0.762f64, 0.833f64, 0.905f64, 0.952f64, 0.976f64, 0.976f64, 1.0f64,
    1.0f64, 0.976f64, 0.976f64, 0.952f64, 0.905f64, 0.833f64, 0.762f64, 0.667f64, 0.524f64,
    0.357f64, 0.0f64,
];
#[no_mangle]
pub static mut N_Y_Catenary: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Y_Catenary: [::core::ffi::c_double; 51] = [
    0.0f64, 0.02974f64, 0.06439f64, 0.08433f64, 0.10549f64, 0.12064f64, 0.13952f64, 0.1556f64,
    0.17032f64, 0.18512f64, 0.20057f64, 0.21995f64, 0.24011f64, 0.25892f64, 0.27595f64, 0.29214f64,
    0.30802f64, 0.32372f64, 0.33894f64, 0.35315f64, 0.36557f64, 0.37833f64, 0.3923f64, 0.4097f64,
    0.42982f64, 0.45f64, 0.46769f64, 0.48431f64, 0.5f64, 0.51466f64, 0.52886f64, 0.54292f64,
    0.55729f64, 0.57223f64, 0.5878f64, 0.60428f64, 0.62197f64, 0.64047f64, 0.6598f64, 0.67976f64,
    0.7f64, 0.71731f64, 0.73769f64, 0.76651f64, 0.8f64, 0.8209f64, 0.84311f64, 0.87978f64,
    0.91576f64, 0.95f64, 1.0f64,
];
#[no_mangle]
pub static mut N_S_Catenary: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut S_Catenary: [::core::ffi::c_double; 51] = [
    0.0f64, 0.00605f64, 0.01455f64, 0.0254f64, 0.03863f64, 0.0543f64, 0.07127f64, 0.08778f64,
    0.10372f64, 0.12081f64, 0.14082f64, 0.16375f64, 0.18779f64, 0.21157f64, 0.23478f64, 0.25818f64,
    0.28244f64, 0.30741f64, 0.33204f64, 0.35505f64, 0.37465f64, 0.39404f64, 0.41426f64, 0.43804f64,
    0.46531f64, 0.49357f64, 0.52187f64, 0.54925f64, 0.57647f64, 0.60321f64, 0.62964f64, 0.65639f64,
    0.68472f64, 0.71425f64, 0.74303f64, 0.76827f64, 0.79168f64, 0.815f64, 0.84094f64, 0.86707f64,
    0.89213f64, 0.91607f64, 0.94f64, 0.96604f64, 0.99f64, 1.00714f64, 1.02158f64, 1.03814f64,
    1.05f64, 1.05f64, 1.0f64,
];
#[no_mangle]
pub static mut N_W_Catenary: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_Catenary: [::core::ffi::c_double; 21] = [
    0.0f64, 0.6667f64, 0.8222f64, 0.9111f64, 0.9778f64, 1.0000f64, 1.0000f64, 0.9889f64, 0.9778f64,
    0.9556f64, 0.9333f64, 0.8889f64, 0.8444f64, 0.8000f64, 0.7556f64, 0.7000f64, 0.6333f64,
    0.5556f64, 0.4444f64, 0.3333f64, 0.0f64,
];
#[no_mangle]
pub static mut N_Y_SemiEllip: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Y_SemiEllip: [::core::ffi::c_double; 51] = [
    0.0f64, 0.03075f64, 0.05137f64, 0.07032f64, 0.09f64, 0.11323f64, 0.13037f64, 0.14519f64,
    0.15968f64, 0.18459f64, 0.19531f64, 0.21354f64, 0.22694f64, 0.23947f64, 0.25296f64, 0.265f64,
    0.27784f64, 0.29212f64, 0.3097f64, 0.32982f64, 0.35f64, 0.36738f64, 0.3839f64, 0.4f64,
    0.41667f64, 0.43333f64, 0.45f64, 0.46697f64, 0.48372f64, 0.5f64, 0.51374f64, 0.52747f64,
    0.54209f64, 0.5595f64, 0.57941f64, 0.6f64, 0.62f64, 0.64f64, 0.66f64, 0.68f64, 0.7f64,
    0.71843f64, 0.73865f64, 0.76365f64, 0.7926f64, 0.82088f64, 0.85f64, 0.88341f64, 0.90998f64,
    0.93871f64, 1.0f64,
];
#[no_mangle]
pub static mut N_S_SemiEllip: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut S_SemiEllip: [::core::ffi::c_double; 51] = [
    0.0f64, 0.00438f64, 0.01227f64, 0.02312f64, 0.03638f64, 0.05145f64, 0.06783f64, 0.085f64,
    0.10093f64, 0.11752f64, 0.1353f64, 0.15626f64, 0.17917f64, 0.20296f64, 0.22654f64, 0.24962f64,
    0.27269f64, 0.29568f64, 0.31848f64, 0.34152f64, 0.365f64, 0.38941f64, 0.41442f64, 0.44f64,
    0.46636f64, 0.49309f64, 0.52f64, 0.54628f64, 0.57285f64, 0.6f64, 0.62949f64, 0.65877f64,
    0.68624f64, 0.71017f64, 0.73304f64, 0.75578f64, 0.77925f64, 0.80368f64, 0.83114f64, 0.8595f64,
    0.88592f64, 0.90848f64, 0.93f64, 0.95292f64, 0.97481f64, 0.99374f64, 1.01084f64, 1.02858f64,
    1.04543f64, 1.05f64, 1.0f64,
];
#[no_mangle]
pub static mut N_W_SemiEllip: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_SemiEllip: [::core::ffi::c_double; 21] = [
    0.0f64, 0.7000f64, 0.9800f64, 1.0000f64, 1.0000f64, 1.0000f64, 0.9900f64, 0.9800f64, 0.9600f64,
    0.9400f64, 0.9100f64, 0.8800f64, 0.8400f64, 0.8000f64, 0.7500f64, 0.7000f64, 0.6400f64,
    0.5600f64, 0.4600f64, 0.3400f64, 0.0f64,
];
#[no_mangle]
pub static mut N_A_Baskethandle: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut A_Baskethandle: [::core::ffi::c_double; 26] = [
    0.0000f64, 0.0173f64, 0.0457f64, 0.0828f64, 0.1271f64, 0.1765f64, 0.2270f64, 0.2775f64,
    0.3280f64, 0.3780f64, 0.4270f64, 0.4765f64, 0.5260f64, 0.5740f64, 0.6220f64, 0.6690f64,
    0.7160f64, 0.7610f64, 0.8030f64, 0.8390f64, 0.8770f64, 0.9110f64, 0.9410f64, 0.9680f64,
    0.9880f64, 1.000f64,
];
#[no_mangle]
pub static mut N_R_Baskethandle: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut R_Baskethandle: [::core::ffi::c_double; 26] = [
    0.0100f64, 0.0952f64, 0.1890f64, 0.2730f64, 0.3690f64, 0.4630f64, 0.5600f64, 0.6530f64,
    0.7430f64, 0.8220f64, 0.8830f64, 0.9490f64, 0.9990f64, 1.055f64, 1.095f64, 1.141f64, 1.161f64,
    1.188f64, 1.206f64, 1.206f64, 1.206f64, 1.205f64, 1.196f64, 1.168f64, 1.127f64, 1.000f64,
];
#[no_mangle]
pub static mut N_Y_BasketHandle: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Y_BasketHandle: [::core::ffi::c_double; 51] = [
    0.0f64, 0.04112f64, 0.0738f64, 0.1f64, 0.12236f64, 0.14141f64, 0.15857f64, 0.17462f64,
    0.18946f64, 0.20315f64, 0.21557f64, 0.22833f64, 0.2423f64, 0.25945f64, 0.27936f64, 0.3f64,
    0.3204f64, 0.34034f64, 0.35892f64, 0.37595f64, 0.39214f64, 0.40802f64, 0.42372f64, 0.43894f64,
    0.45315f64, 0.46557f64, 0.47833f64, 0.4923f64, 0.50945f64, 0.52936f64, 0.55f64, 0.57f64,
    0.59f64, 0.61023f64, 0.63045f64, 0.65f64, 0.66756f64, 0.68413f64, 0.7f64, 0.71481f64,
    0.72984f64, 0.74579f64, 0.76417f64, 0.78422f64, 0.80477f64, 0.82532f64, 0.85f64, 0.88277f64,
    0.915f64, 0.95f64, 1.0f64,
];
#[no_mangle]
pub static mut N_S_BasketHandle: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut S_BasketHandle: [::core::ffi::c_double; 51] = [
    0.0f64, 0.00758f64, 0.01812f64, 0.03f64, 0.03966f64, 0.04957f64, 0.0623f64, 0.07849f64,
    0.09618f64, 0.11416f64, 0.13094f64, 0.14808f64, 0.16583f64, 0.18381f64, 0.20294f64, 0.225f64,
    0.2547f64, 0.28532f64, 0.31006f64, 0.32804f64, 0.34555f64, 0.36944f64, 0.40032f64, 0.43203f64,
    0.46004f64, 0.47849f64, 0.49591f64, 0.51454f64, 0.5381f64, 0.56711f64, 0.6f64, 0.64092f64,
    0.68136f64, 0.71259f64, 0.73438f64, 0.755f64, 0.78625f64, 0.8188f64, 0.85f64, 0.8679f64,
    0.88483f64, 0.90431f64, 0.9369f64, 0.97388f64, 1.00747f64, 1.033f64, 1.05f64, 1.05464f64,
    1.06078f64, 1.055f64, 1.0f64,
];
#[no_mangle]
pub static mut N_W_BasketHandle: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_BasketHandle: [::core::ffi::c_double; 26] = [
    0.0f64, 0.49f64, 0.667f64, 0.82f64, 0.93f64, 1.00f64, 1.00f64, 1.00f64, 0.997f64, 0.994f64,
    0.988f64, 0.982f64, 0.967f64, 0.948f64, 0.928f64, 0.904f64, 0.874f64, 0.842f64, 0.798f64,
    0.75f64, 0.697f64, 0.637f64, 0.567f64, 0.467f64, 0.342f64, 0.0f64,
];
#[no_mangle]
pub static mut N_Y_SemiCirc: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Y_SemiCirc: [::core::ffi::c_double; 51] = [
    0.0f64, 0.04102f64, 0.07407f64, 0.1f64, 0.11769f64, 0.13037f64, 0.14036f64, 0.15f64,
    0.16546f64, 0.18213f64, 0.2f64, 0.22018f64, 0.2403f64, 0.25788f64, 0.27216f64, 0.285f64,
    0.29704f64, 0.30892f64, 0.32128f64, 0.33476f64, 0.35f64, 0.36927f64, 0.38963f64, 0.41023f64,
    0.43045f64, 0.45f64, 0.46769f64, 0.48431f64, 0.5f64, 0.51443f64, 0.52851f64, 0.54271f64,
    0.55774f64, 0.57388f64, 0.59101f64, 0.60989f64, 0.63005f64, 0.65f64, 0.66682f64, 0.68318f64,
    0.7f64, 0.71675f64, 0.73744f64, 0.76651f64, 0.8f64, 0.8209f64, 0.84311f64, 0.87978f64,
    0.91576f64, 0.95f64, 1.0f64,
];
#[no_mangle]
pub static mut N_S_SemiCirc: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
#[no_mangle]
pub static mut S_SemiCirc: [::core::ffi::c_double; 51] = [
    0.0f64, 0.00757f64, 0.01815f64, 0.03f64, 0.0358f64, 0.04037f64, 0.04601f64, 0.055f64,
    0.07475f64, 0.09834f64, 0.125f64, 0.1557f64, 0.18588f64, 0.20883f64, 0.223f64, 0.23472f64,
    0.24667f64, 0.26758f64, 0.29346f64, 0.32124f64, 0.35f64, 0.3772f64, 0.4054f64, 0.43541f64,
    0.46722f64, 0.5f64, 0.53532f64, 0.56935f64, 0.6f64, 0.61544f64, 0.62811f64, 0.6417f64,
    0.66598f64, 0.7001f64, 0.73413f64, 0.76068f64, 0.78027f64, 0.8f64, 0.82891f64, 0.85964f64,
    0.89f64, 0.9127f64, 0.93664f64, 0.96677f64, 1.0f64, 1.02661f64, 1.04631f64, 1.05726f64,
    1.06637f64, 1.06f64, 1.0f64,
];
#[no_mangle]
pub static mut N_W_SemiCirc: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_SemiCirc: [::core::ffi::c_double; 21] = [
    0.0f64, 0.5488f64, 0.8537f64, 1.0000f64, 1.0000f64, 0.9939f64, 0.9878f64, 0.9756f64, 0.9634f64,
    0.9451f64, 0.9207f64, 0.8902f64, 0.8537f64, 0.8171f64, 0.7683f64, 0.7073f64, 0.6463f64,
    0.5732f64, 0.4756f64, 0.3354f64, 0.0f64,
];
#[no_mangle]
pub static mut NumCodesEllipse: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
#[no_mangle]
pub static mut MinorAxis_Ellipse: [::core::ffi::c_double; 23] = [
    14 as ::core::ffi::c_int as ::core::ffi::c_double,
    19 as ::core::ffi::c_int as ::core::ffi::c_double,
    22 as ::core::ffi::c_int as ::core::ffi::c_double,
    24 as ::core::ffi::c_int as ::core::ffi::c_double,
    27 as ::core::ffi::c_int as ::core::ffi::c_double,
    29 as ::core::ffi::c_int as ::core::ffi::c_double,
    32 as ::core::ffi::c_int as ::core::ffi::c_double,
    34 as ::core::ffi::c_int as ::core::ffi::c_double,
    38 as ::core::ffi::c_int as ::core::ffi::c_double,
    43 as ::core::ffi::c_int as ::core::ffi::c_double,
    48 as ::core::ffi::c_int as ::core::ffi::c_double,
    53 as ::core::ffi::c_int as ::core::ffi::c_double,
    58 as ::core::ffi::c_int as ::core::ffi::c_double,
    63 as ::core::ffi::c_int as ::core::ffi::c_double,
    68 as ::core::ffi::c_int as ::core::ffi::c_double,
    72 as ::core::ffi::c_int as ::core::ffi::c_double,
    77 as ::core::ffi::c_int as ::core::ffi::c_double,
    82 as ::core::ffi::c_int as ::core::ffi::c_double,
    87 as ::core::ffi::c_int as ::core::ffi::c_double,
    92 as ::core::ffi::c_int as ::core::ffi::c_double,
    97 as ::core::ffi::c_int as ::core::ffi::c_double,
    106 as ::core::ffi::c_int as ::core::ffi::c_double,
    116 as ::core::ffi::c_int as ::core::ffi::c_double,
];
#[no_mangle]
pub static mut MajorAxis_Ellipse: [::core::ffi::c_double; 23] = [
    23 as ::core::ffi::c_int as ::core::ffi::c_double,
    30 as ::core::ffi::c_int as ::core::ffi::c_double,
    34 as ::core::ffi::c_int as ::core::ffi::c_double,
    38 as ::core::ffi::c_int as ::core::ffi::c_double,
    42 as ::core::ffi::c_int as ::core::ffi::c_double,
    45 as ::core::ffi::c_int as ::core::ffi::c_double,
    49 as ::core::ffi::c_int as ::core::ffi::c_double,
    53 as ::core::ffi::c_int as ::core::ffi::c_double,
    60 as ::core::ffi::c_int as ::core::ffi::c_double,
    68 as ::core::ffi::c_int as ::core::ffi::c_double,
    76 as ::core::ffi::c_int as ::core::ffi::c_double,
    83 as ::core::ffi::c_int as ::core::ffi::c_double,
    91 as ::core::ffi::c_int as ::core::ffi::c_double,
    98 as ::core::ffi::c_int as ::core::ffi::c_double,
    106 as ::core::ffi::c_int as ::core::ffi::c_double,
    113 as ::core::ffi::c_int as ::core::ffi::c_double,
    121 as ::core::ffi::c_int as ::core::ffi::c_double,
    128 as ::core::ffi::c_int as ::core::ffi::c_double,
    136 as ::core::ffi::c_int as ::core::ffi::c_double,
    143 as ::core::ffi::c_int as ::core::ffi::c_double,
    151 as ::core::ffi::c_int as ::core::ffi::c_double,
    166 as ::core::ffi::c_int as ::core::ffi::c_double,
    180 as ::core::ffi::c_int as ::core::ffi::c_double,
];
#[no_mangle]
pub static mut Afull_Ellipse: [::core::ffi::c_double; 23] = [
    1.80f64, 3.30f64, 4.10f64, 5.10f64, 6.30f64, 7.40f64, 8.80f64, 10.20f64, 12.90f64, 16.60f64,
    20.50f64, 24.80f64, 29.50f64, 34.60f64, 40.10f64, 46.10f64, 52.40f64, 59.20f64, 66.40f64,
    74.00f64, 82.00f64, 99.20f64, 118.60f64,
];
#[no_mangle]
pub static mut Rfull_Ellipse: [::core::ffi::c_double; 23] = [
    0.367f64, 0.490f64, 0.546f64, 0.613f64, 0.686f64, 0.736f64, 0.812f64, 0.875f64, 0.969f64,
    1.106f64, 1.229f64, 1.352f64, 1.475f64, 1.598f64, 1.721f64, 1.845f64, 1.967f64, 2.091f64,
    2.215f64, 2.340f64, 2.461f64, 2.707f64, 2.968f64,
];
#[no_mangle]
pub static mut N_A_HorizEllipse: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut A_HorizEllipse: [::core::ffi::c_double; 26] = [
    0.0000f64, 0.0150f64, 0.0400f64, 0.0650f64, 0.0950f64, 0.1300f64, 0.1650f64, 0.2050f64,
    0.2500f64, 0.3000f64, 0.3550f64, 0.4150f64, 0.4800f64, 0.5200f64, 0.5850f64, 0.6450f64,
    0.7000f64, 0.7500f64, 0.7950f64, 0.8350f64, 0.8700f64, 0.9050f64, 0.9350f64, 0.9600f64,
    0.9850f64, 1.000f64,
];
#[no_mangle]
pub static mut N_R_HorizEllipse: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut R_HorizEllipse: [::core::ffi::c_double; 26] = [
    0.0100f64, 0.0764f64, 0.1726f64, 0.2389f64, 0.3274f64, 0.4191f64, 0.5120f64, 0.5983f64,
    0.6757f64, 0.7630f64, 0.8326f64, 0.9114f64, 0.9702f64, 1.030f64, 1.091f64, 1.146f64, 1.185f64,
    1.225f64, 1.257f64, 1.274f64, 1.290f64, 1.282f64, 1.274f64, 1.257f64, 1.185f64, 1.000f64,
];
#[no_mangle]
pub static mut N_W_HorizEllipse: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_HorizEllipse: [::core::ffi::c_double; 26] = [
    0.0f64, 0.3919f64, 0.5426f64, 0.6499f64, 0.7332f64, 0.8000f64, 0.8542f64, 0.8980f64, 0.9330f64,
    0.9600f64, 0.9798f64, 0.9928f64, 0.9992f64, 0.9992f64, 0.9928f64, 0.9798f64, 0.9600f64,
    0.9330f64, 0.8980f64, 0.8542f64, 0.8000f64, 0.7332f64, 0.6499f64, 0.5426f64, 0.3919f64, 0.0f64,
];
#[no_mangle]
pub static mut N_A_VertEllipse: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut A_VertEllipse: [::core::ffi::c_double; 26] = [
    0.0000f64, 0.0100f64, 0.0400f64, 0.0700f64, 0.1000f64, 0.1400f64, 0.1850f64, 0.2300f64,
    0.2800f64, 0.3300f64, 0.3800f64, 0.4300f64, 0.4800f64, 0.5200f64, 0.5700f64, 0.6200f64,
    0.6700f64, 0.7200f64, 0.7700f64, 0.8150f64, 0.8600f64, 0.9000f64, 0.9300f64, 0.9600f64,
    0.9900f64, 1.000f64,
];
#[no_mangle]
pub static mut N_R_VertEllipse: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut R_VertEllipse: [::core::ffi::c_double; 26] = [
    0.0100f64, 0.1250f64, 0.2436f64, 0.3536f64, 0.4474f64, 0.5484f64, 0.6366f64, 0.7155f64,
    0.7768f64, 0.8396f64, 0.8969f64, 0.9480f64, 0.9925f64, 1.023f64, 1.053f64, 1.084f64, 1.107f64,
    1.130f64, 1.154f64, 1.170f64, 1.177f64, 1.177f64, 1.170f64, 1.162f64, 1.122f64, 1.000f64,
];
#[no_mangle]
pub static mut N_W_VertEllipse: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_VertEllipse: [::core::ffi::c_double; 26] = [
    0.0f64, 0.3919f64, 0.5426f64, 0.6499f64, 0.7332f64, 0.8000f64, 0.8542f64, 0.8980f64, 0.9330f64,
    0.9600f64, 0.9798f64, 0.9928f64, 0.9992f64, 0.9992f64, 0.9928f64, 0.9798f64, 0.9600f64,
    0.9330f64, 0.8980f64, 0.8542f64, 0.8000f64, 0.7332f64, 0.6499f64, 0.5426f64, 0.3919f64, 0.0f64,
];
#[no_mangle]
pub static mut NumCodesArch: ::core::ffi::c_int = 102 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Yfull_Arch: [::core::ffi::c_double; 102] = [
    11 as ::core::ffi::c_int as ::core::ffi::c_double,
    13.5f64,
    15.5f64,
    18 as ::core::ffi::c_int as ::core::ffi::c_double,
    22.5f64,
    26.625f64,
    31.3125f64,
    36 as ::core::ffi::c_int as ::core::ffi::c_double,
    40 as ::core::ffi::c_int as ::core::ffi::c_double,
    45 as ::core::ffi::c_int as ::core::ffi::c_double,
    54 as ::core::ffi::c_int as ::core::ffi::c_double,
    62 as ::core::ffi::c_int as ::core::ffi::c_double,
    72 as ::core::ffi::c_int as ::core::ffi::c_double,
    77.5f64,
    87.125f64,
    96.875f64,
    106.5f64,
    13 as ::core::ffi::c_int as ::core::ffi::c_double,
    15 as ::core::ffi::c_int as ::core::ffi::c_double,
    18 as ::core::ffi::c_int as ::core::ffi::c_double,
    20 as ::core::ffi::c_int as ::core::ffi::c_double,
    24 as ::core::ffi::c_int as ::core::ffi::c_double,
    29 as ::core::ffi::c_int as ::core::ffi::c_double,
    33 as ::core::ffi::c_int as ::core::ffi::c_double,
    38 as ::core::ffi::c_int as ::core::ffi::c_double,
    43 as ::core::ffi::c_int as ::core::ffi::c_double,
    47 as ::core::ffi::c_int as ::core::ffi::c_double,
    52 as ::core::ffi::c_int as ::core::ffi::c_double,
    57 as ::core::ffi::c_int as ::core::ffi::c_double,
    31 as ::core::ffi::c_int as ::core::ffi::c_double,
    36 as ::core::ffi::c_int as ::core::ffi::c_double,
    41 as ::core::ffi::c_int as ::core::ffi::c_double,
    46 as ::core::ffi::c_int as ::core::ffi::c_double,
    51 as ::core::ffi::c_int as ::core::ffi::c_double,
    55 as ::core::ffi::c_int as ::core::ffi::c_double,
    59 as ::core::ffi::c_int as ::core::ffi::c_double,
    63 as ::core::ffi::c_int as ::core::ffi::c_double,
    67 as ::core::ffi::c_int as ::core::ffi::c_double,
    71 as ::core::ffi::c_int as ::core::ffi::c_double,
    75 as ::core::ffi::c_int as ::core::ffi::c_double,
    79 as ::core::ffi::c_int as ::core::ffi::c_double,
    83 as ::core::ffi::c_int as ::core::ffi::c_double,
    87 as ::core::ffi::c_int as ::core::ffi::c_double,
    91 as ::core::ffi::c_int as ::core::ffi::c_double,
    55 as ::core::ffi::c_int as ::core::ffi::c_double,
    57 as ::core::ffi::c_int as ::core::ffi::c_double,
    59 as ::core::ffi::c_int as ::core::ffi::c_double,
    61 as ::core::ffi::c_int as ::core::ffi::c_double,
    63 as ::core::ffi::c_int as ::core::ffi::c_double,
    65 as ::core::ffi::c_int as ::core::ffi::c_double,
    67 as ::core::ffi::c_int as ::core::ffi::c_double,
    69 as ::core::ffi::c_int as ::core::ffi::c_double,
    71 as ::core::ffi::c_int as ::core::ffi::c_double,
    73 as ::core::ffi::c_int as ::core::ffi::c_double,
    75 as ::core::ffi::c_int as ::core::ffi::c_double,
    77 as ::core::ffi::c_int as ::core::ffi::c_double,
    79 as ::core::ffi::c_int as ::core::ffi::c_double,
    81 as ::core::ffi::c_int as ::core::ffi::c_double,
    83 as ::core::ffi::c_int as ::core::ffi::c_double,
    85 as ::core::ffi::c_int as ::core::ffi::c_double,
    87 as ::core::ffi::c_int as ::core::ffi::c_double,
    89 as ::core::ffi::c_int as ::core::ffi::c_double,
    91 as ::core::ffi::c_int as ::core::ffi::c_double,
    93 as ::core::ffi::c_int as ::core::ffi::c_double,
    95 as ::core::ffi::c_int as ::core::ffi::c_double,
    97 as ::core::ffi::c_int as ::core::ffi::c_double,
    100 as ::core::ffi::c_int as ::core::ffi::c_double,
    101 as ::core::ffi::c_int as ::core::ffi::c_double,
    103 as ::core::ffi::c_int as ::core::ffi::c_double,
    105 as ::core::ffi::c_int as ::core::ffi::c_double,
    107 as ::core::ffi::c_int as ::core::ffi::c_double,
    109 as ::core::ffi::c_int as ::core::ffi::c_double,
    111 as ::core::ffi::c_int as ::core::ffi::c_double,
    113 as ::core::ffi::c_int as ::core::ffi::c_double,
    115 as ::core::ffi::c_int as ::core::ffi::c_double,
    118 as ::core::ffi::c_int as ::core::ffi::c_double,
    119 as ::core::ffi::c_int as ::core::ffi::c_double,
    121 as ::core::ffi::c_int as ::core::ffi::c_double,
    112 as ::core::ffi::c_int as ::core::ffi::c_double,
    114 as ::core::ffi::c_int as ::core::ffi::c_double,
    116 as ::core::ffi::c_int as ::core::ffi::c_double,
    118 as ::core::ffi::c_int as ::core::ffi::c_double,
    120 as ::core::ffi::c_int as ::core::ffi::c_double,
    122 as ::core::ffi::c_int as ::core::ffi::c_double,
    124 as ::core::ffi::c_int as ::core::ffi::c_double,
    126 as ::core::ffi::c_int as ::core::ffi::c_double,
    128 as ::core::ffi::c_int as ::core::ffi::c_double,
    130 as ::core::ffi::c_int as ::core::ffi::c_double,
    132 as ::core::ffi::c_int as ::core::ffi::c_double,
    134 as ::core::ffi::c_int as ::core::ffi::c_double,
    136 as ::core::ffi::c_int as ::core::ffi::c_double,
    138 as ::core::ffi::c_int as ::core::ffi::c_double,
    140 as ::core::ffi::c_int as ::core::ffi::c_double,
    142 as ::core::ffi::c_int as ::core::ffi::c_double,
    144 as ::core::ffi::c_int as ::core::ffi::c_double,
    146 as ::core::ffi::c_int as ::core::ffi::c_double,
    148 as ::core::ffi::c_int as ::core::ffi::c_double,
    150 as ::core::ffi::c_int as ::core::ffi::c_double,
    152 as ::core::ffi::c_int as ::core::ffi::c_double,
    154 as ::core::ffi::c_int as ::core::ffi::c_double,
    156 as ::core::ffi::c_int as ::core::ffi::c_double,
    158 as ::core::ffi::c_int as ::core::ffi::c_double,
];
#[no_mangle]
pub static mut Wmax_Arch: [::core::ffi::c_double; 102] = [
    18 as ::core::ffi::c_int as ::core::ffi::c_double,
    22 as ::core::ffi::c_int as ::core::ffi::c_double,
    26 as ::core::ffi::c_int as ::core::ffi::c_double,
    28.5f64,
    36.25f64,
    43.75f64,
    51.125f64,
    58.5f64,
    65 as ::core::ffi::c_int as ::core::ffi::c_double,
    73 as ::core::ffi::c_int as ::core::ffi::c_double,
    88 as ::core::ffi::c_int as ::core::ffi::c_double,
    102 as ::core::ffi::c_int as ::core::ffi::c_double,
    115 as ::core::ffi::c_int as ::core::ffi::c_double,
    122 as ::core::ffi::c_int as ::core::ffi::c_double,
    138 as ::core::ffi::c_int as ::core::ffi::c_double,
    154 as ::core::ffi::c_int as ::core::ffi::c_double,
    168.75f64,
    17 as ::core::ffi::c_int as ::core::ffi::c_double,
    21 as ::core::ffi::c_int as ::core::ffi::c_double,
    24 as ::core::ffi::c_int as ::core::ffi::c_double,
    28 as ::core::ffi::c_int as ::core::ffi::c_double,
    35 as ::core::ffi::c_int as ::core::ffi::c_double,
    42 as ::core::ffi::c_int as ::core::ffi::c_double,
    49 as ::core::ffi::c_int as ::core::ffi::c_double,
    57 as ::core::ffi::c_int as ::core::ffi::c_double,
    64 as ::core::ffi::c_int as ::core::ffi::c_double,
    71 as ::core::ffi::c_int as ::core::ffi::c_double,
    77 as ::core::ffi::c_int as ::core::ffi::c_double,
    83 as ::core::ffi::c_int as ::core::ffi::c_double,
    40 as ::core::ffi::c_int as ::core::ffi::c_double,
    46 as ::core::ffi::c_int as ::core::ffi::c_double,
    53 as ::core::ffi::c_int as ::core::ffi::c_double,
    60 as ::core::ffi::c_int as ::core::ffi::c_double,
    66 as ::core::ffi::c_int as ::core::ffi::c_double,
    73 as ::core::ffi::c_int as ::core::ffi::c_double,
    81 as ::core::ffi::c_int as ::core::ffi::c_double,
    87 as ::core::ffi::c_int as ::core::ffi::c_double,
    95 as ::core::ffi::c_int as ::core::ffi::c_double,
    103 as ::core::ffi::c_int as ::core::ffi::c_double,
    112 as ::core::ffi::c_int as ::core::ffi::c_double,
    117 as ::core::ffi::c_int as ::core::ffi::c_double,
    128 as ::core::ffi::c_int as ::core::ffi::c_double,
    137 as ::core::ffi::c_int as ::core::ffi::c_double,
    142 as ::core::ffi::c_int as ::core::ffi::c_double,
    73 as ::core::ffi::c_int as ::core::ffi::c_double,
    76 as ::core::ffi::c_int as ::core::ffi::c_double,
    81 as ::core::ffi::c_int as ::core::ffi::c_double,
    84 as ::core::ffi::c_int as ::core::ffi::c_double,
    87 as ::core::ffi::c_int as ::core::ffi::c_double,
    92 as ::core::ffi::c_int as ::core::ffi::c_double,
    95 as ::core::ffi::c_int as ::core::ffi::c_double,
    98 as ::core::ffi::c_int as ::core::ffi::c_double,
    103 as ::core::ffi::c_int as ::core::ffi::c_double,
    106 as ::core::ffi::c_int as ::core::ffi::c_double,
    112 as ::core::ffi::c_int as ::core::ffi::c_double,
    114 as ::core::ffi::c_int as ::core::ffi::c_double,
    117 as ::core::ffi::c_int as ::core::ffi::c_double,
    123 as ::core::ffi::c_int as ::core::ffi::c_double,
    128 as ::core::ffi::c_int as ::core::ffi::c_double,
    131 as ::core::ffi::c_int as ::core::ffi::c_double,
    137 as ::core::ffi::c_int as ::core::ffi::c_double,
    139 as ::core::ffi::c_int as ::core::ffi::c_double,
    142 as ::core::ffi::c_int as ::core::ffi::c_double,
    148 as ::core::ffi::c_int as ::core::ffi::c_double,
    150 as ::core::ffi::c_int as ::core::ffi::c_double,
    152 as ::core::ffi::c_int as ::core::ffi::c_double,
    154 as ::core::ffi::c_int as ::core::ffi::c_double,
    161 as ::core::ffi::c_int as ::core::ffi::c_double,
    167 as ::core::ffi::c_int as ::core::ffi::c_double,
    169 as ::core::ffi::c_int as ::core::ffi::c_double,
    171 as ::core::ffi::c_int as ::core::ffi::c_double,
    178 as ::core::ffi::c_int as ::core::ffi::c_double,
    184 as ::core::ffi::c_int as ::core::ffi::c_double,
    186 as ::core::ffi::c_int as ::core::ffi::c_double,
    188 as ::core::ffi::c_int as ::core::ffi::c_double,
    190 as ::core::ffi::c_int as ::core::ffi::c_double,
    197 as ::core::ffi::c_int as ::core::ffi::c_double,
    199 as ::core::ffi::c_int as ::core::ffi::c_double,
    159 as ::core::ffi::c_int as ::core::ffi::c_double,
    162 as ::core::ffi::c_int as ::core::ffi::c_double,
    168 as ::core::ffi::c_int as ::core::ffi::c_double,
    170 as ::core::ffi::c_int as ::core::ffi::c_double,
    173 as ::core::ffi::c_int as ::core::ffi::c_double,
    179 as ::core::ffi::c_int as ::core::ffi::c_double,
    184 as ::core::ffi::c_int as ::core::ffi::c_double,
    187 as ::core::ffi::c_int as ::core::ffi::c_double,
    190 as ::core::ffi::c_int as ::core::ffi::c_double,
    195 as ::core::ffi::c_int as ::core::ffi::c_double,
    198 as ::core::ffi::c_int as ::core::ffi::c_double,
    204 as ::core::ffi::c_int as ::core::ffi::c_double,
    206 as ::core::ffi::c_int as ::core::ffi::c_double,
    209 as ::core::ffi::c_int as ::core::ffi::c_double,
    215 as ::core::ffi::c_int as ::core::ffi::c_double,
    217 as ::core::ffi::c_int as ::core::ffi::c_double,
    223 as ::core::ffi::c_int as ::core::ffi::c_double,
    225 as ::core::ffi::c_int as ::core::ffi::c_double,
    231 as ::core::ffi::c_int as ::core::ffi::c_double,
    234 as ::core::ffi::c_int as ::core::ffi::c_double,
    236 as ::core::ffi::c_int as ::core::ffi::c_double,
    239 as ::core::ffi::c_int as ::core::ffi::c_double,
    245 as ::core::ffi::c_int as ::core::ffi::c_double,
    247 as ::core::ffi::c_int as ::core::ffi::c_double,
];
#[no_mangle]
pub static mut Afull_Arch: [::core::ffi::c_double; 102] = [
    1.1f64,
    1.65f64,
    2.2f64,
    2.8f64,
    4.4f64,
    6.4f64,
    8.8f64,
    11.4f64,
    14.3f64,
    17.7f64,
    25.6f64,
    34.6f64,
    44.5f64,
    51.7f64,
    66 as ::core::ffi::c_int as ::core::ffi::c_double,
    81.8f64,
    99.1f64,
    1.1f64,
    1.6f64,
    2.2f64,
    2.9f64,
    4.5f64,
    6.5f64,
    8.9f64,
    11.6f64,
    14.7f64,
    18.1f64,
    21.9f64,
    26 as ::core::ffi::c_int as ::core::ffi::c_double,
    7 as ::core::ffi::c_int as ::core::ffi::c_double,
    9.4f64,
    12.3f64,
    15.6f64,
    19.3f64,
    23.2f64,
    27.4f64,
    32.1f64,
    37 as ::core::ffi::c_int as ::core::ffi::c_double,
    42.4f64,
    48 as ::core::ffi::c_int as ::core::ffi::c_double,
    54.2f64,
    60.5f64,
    67.4f64,
    74.5f64,
    22 as ::core::ffi::c_int as ::core::ffi::c_double,
    24 as ::core::ffi::c_int as ::core::ffi::c_double,
    26 as ::core::ffi::c_int as ::core::ffi::c_double,
    28 as ::core::ffi::c_int as ::core::ffi::c_double,
    31 as ::core::ffi::c_int as ::core::ffi::c_double,
    33 as ::core::ffi::c_int as ::core::ffi::c_double,
    35 as ::core::ffi::c_int as ::core::ffi::c_double,
    38 as ::core::ffi::c_int as ::core::ffi::c_double,
    40 as ::core::ffi::c_int as ::core::ffi::c_double,
    43 as ::core::ffi::c_int as ::core::ffi::c_double,
    46 as ::core::ffi::c_int as ::core::ffi::c_double,
    49 as ::core::ffi::c_int as ::core::ffi::c_double,
    52 as ::core::ffi::c_int as ::core::ffi::c_double,
    55 as ::core::ffi::c_int as ::core::ffi::c_double,
    58 as ::core::ffi::c_int as ::core::ffi::c_double,
    61 as ::core::ffi::c_int as ::core::ffi::c_double,
    64 as ::core::ffi::c_int as ::core::ffi::c_double,
    67 as ::core::ffi::c_int as ::core::ffi::c_double,
    71 as ::core::ffi::c_int as ::core::ffi::c_double,
    74 as ::core::ffi::c_int as ::core::ffi::c_double,
    78 as ::core::ffi::c_int as ::core::ffi::c_double,
    81 as ::core::ffi::c_int as ::core::ffi::c_double,
    85 as ::core::ffi::c_int as ::core::ffi::c_double,
    89 as ::core::ffi::c_int as ::core::ffi::c_double,
    93 as ::core::ffi::c_int as ::core::ffi::c_double,
    97 as ::core::ffi::c_int as ::core::ffi::c_double,
    101 as ::core::ffi::c_int as ::core::ffi::c_double,
    105 as ::core::ffi::c_int as ::core::ffi::c_double,
    109 as ::core::ffi::c_int as ::core::ffi::c_double,
    113 as ::core::ffi::c_int as ::core::ffi::c_double,
    118 as ::core::ffi::c_int as ::core::ffi::c_double,
    122 as ::core::ffi::c_int as ::core::ffi::c_double,
    126 as ::core::ffi::c_int as ::core::ffi::c_double,
    131 as ::core::ffi::c_int as ::core::ffi::c_double,
    97 as ::core::ffi::c_int as ::core::ffi::c_double,
    102 as ::core::ffi::c_int as ::core::ffi::c_double,
    105 as ::core::ffi::c_int as ::core::ffi::c_double,
    109 as ::core::ffi::c_int as ::core::ffi::c_double,
    114 as ::core::ffi::c_int as ::core::ffi::c_double,
    118 as ::core::ffi::c_int as ::core::ffi::c_double,
    123 as ::core::ffi::c_int as ::core::ffi::c_double,
    127 as ::core::ffi::c_int as ::core::ffi::c_double,
    132 as ::core::ffi::c_int as ::core::ffi::c_double,
    137 as ::core::ffi::c_int as ::core::ffi::c_double,
    142 as ::core::ffi::c_int as ::core::ffi::c_double,
    146 as ::core::ffi::c_int as ::core::ffi::c_double,
    151 as ::core::ffi::c_int as ::core::ffi::c_double,
    157 as ::core::ffi::c_int as ::core::ffi::c_double,
    161 as ::core::ffi::c_int as ::core::ffi::c_double,
    167 as ::core::ffi::c_int as ::core::ffi::c_double,
    172 as ::core::ffi::c_int as ::core::ffi::c_double,
    177 as ::core::ffi::c_int as ::core::ffi::c_double,
    182 as ::core::ffi::c_int as ::core::ffi::c_double,
    188 as ::core::ffi::c_int as ::core::ffi::c_double,
    194 as ::core::ffi::c_int as ::core::ffi::c_double,
    200 as ::core::ffi::c_int as ::core::ffi::c_double,
    205 as ::core::ffi::c_int as ::core::ffi::c_double,
    211 as ::core::ffi::c_int as ::core::ffi::c_double,
];
#[no_mangle]
pub static mut Rfull_Arch: [::core::ffi::c_double; 102] = [
    0.25f64, 0.3f64, 0.36f64, 0.45f64, 0.56f64, 0.68f64, 0.8f64, 0.9f64, 1.01f64, 1.13f64, 1.35f64,
    1.57f64, 1.77f64, 1.92f64, 2.17f64, 2.42f64, 2.65f64, 0.324f64, 0.374f64, 0.449f64, 0.499f64,
    0.598f64, 0.723f64, 0.823f64, 0.947f64, 1.072f64, 1.171f64, 1.296f64, 1.421f64, 0.773f64,
    0.773f64, 1.022f64, 1.147f64, 1.271f64, 1.371f64, 1.471f64, 1.570f64, 1.670f64, 1.770f64,
    1.869f64, 1.969f64, 2.069f64, 2.168f64, 2.268f64, 1.371f64, 1.421f64, 1.471f64, 1.520f64,
    1.570f64, 1.620f64, 1.670f64, 1.720f64, 1.770f64, 1.820f64, 1.869f64, 1.919f64, 1.969f64,
    2.019f64, 2.069f64, 2.119f64, 2.168f64, 2.218f64, 2.268f64, 2.318f64, 2.368f64, 2.418f64,
    2.493f64, 2.517f64, 2.567f64, 2.617f64, 2.667f64, 2.717f64, 2.767f64, 2.817f64, 2.866f64,
    2.941f64, 2.966f64, 3.016f64, 2.792f64, 2.841f64, 2.891f64, 2.941f64, 2.991f64, 3.041f64,
    3.091f64, 3.141f64, 3.190f64, 3.240f64, 3.290f64, 3.340f64, 3.390f64, 3.440f64, 3.490f64,
    3.539f64, 3.589f64, 3.639f64, 3.689f64, 3.739f64, 3.789f64, 3.838f64, 3.888f64, 3.938f64,
];
#[no_mangle]
pub static mut N_A_Arch: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut A_Arch: [::core::ffi::c_double; 26] = [
    0.0000f64, 0.0200f64, 0.0600f64, 0.1000f64, 0.1400f64, 0.1900f64, 0.2400f64, 0.2900f64,
    0.3400f64, 0.3900f64, 0.4400f64, 0.4900f64, 0.5400f64, 0.5900f64, 0.6400f64, 0.6900f64,
    0.7350f64, 0.7800f64, 0.8200f64, 0.8600f64, 0.8950f64, 0.9300f64, 0.9600f64, 0.9850f64,
    0.9950f64, 1.000f64,
];
#[no_mangle]
pub static mut N_R_Arch: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut R_Arch: [::core::ffi::c_double; 26] = [
    0.0100f64, 0.0983f64, 0.1965f64, 0.2948f64, 0.3940f64, 0.4962f64, 0.5911f64, 0.6796f64,
    0.7615f64, 0.8364f64, 0.9044f64, 0.9640f64, 1.018f64, 1.065f64, 1.106f64, 1.142f64, 1.170f64,
    1.192f64, 1.208f64, 1.217f64, 1.220f64, 1.213f64, 1.196f64, 1.168f64, 1.112f64, 1.000f64,
];
#[no_mangle]
pub static mut N_W_Arch: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
#[no_mangle]
pub static mut W_Arch: [::core::ffi::c_double; 26] = [
    0.0f64, 0.6272f64, 0.8521f64, 0.9243f64, 0.9645f64, 0.9846f64, 0.9964f64, 0.9988f64, 0.9917f64,
    0.9811f64, 0.9680f64, 0.9515f64, 0.9314f64, 0.9101f64, 0.8864f64, 0.8592f64, 0.8284f64,
    0.7917f64, 0.7527f64, 0.7065f64, 0.6544f64, 0.5953f64, 0.5231f64, 0.4355f64, 0.3195f64, 0.0f64,
];

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
