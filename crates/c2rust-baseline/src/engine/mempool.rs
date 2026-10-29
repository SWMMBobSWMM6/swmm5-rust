#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alloc_handle_t {
    pub dummy: ::core::ffi::c_long,
}
pub type alloc_root_t = alloc_root_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alloc_root_s {
    pub first: *mut alloc_hdr_t,
    pub current: *mut alloc_hdr_t,
}
pub type alloc_hdr_t = alloc_hdr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alloc_hdr_s {
    pub next: *mut alloc_hdr_s,
    pub block: *mut ::core::ffi::c_char,
    pub free: *mut ::core::ffi::c_char,
    pub end: *mut ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ALLOC_BLOCK_SIZE: ::core::ffi::c_int = 64000 as ::core::ffi::c_int;
static mut root: *mut alloc_root_t = ::core::ptr::null::<alloc_root_t>() as *mut alloc_root_t;
unsafe extern "C" fn AllocHdr() -> *mut alloc_hdr_t {
    let mut hdr: *mut alloc_hdr_t = ::core::ptr::null_mut::<alloc_hdr_t>();
    let mut block: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    block = malloc(ALLOC_BLOCK_SIZE as size_t) as *mut ::core::ffi::c_char;
    hdr = malloc(::core::mem::size_of::<alloc_hdr_t>() as size_t) as *mut alloc_hdr_t;
    if hdr.is_null() || block.is_null() {
        return ::core::ptr::null_mut::<alloc_hdr_t>();
    }
    (*hdr).block = block;
    (*hdr).free = block;
    (*hdr).next = ::core::ptr::null_mut::<alloc_hdr_s>();
    (*hdr).end = block.offset(ALLOC_BLOCK_SIZE as isize);
    return hdr;
}
#[no_mangle]
pub unsafe extern "C" fn AllocInit() -> *mut alloc_handle_t {
    let mut newpool: *mut alloc_handle_t = ::core::ptr::null_mut::<alloc_handle_t>();
    root = malloc(::core::mem::size_of::<alloc_root_t>() as size_t) as *mut alloc_root_t;
    if root.is_null() {
        return ::core::ptr::null_mut::<alloc_handle_t>();
    }
    (*root).first = AllocHdr();
    if (*root).first.is_null() {
        return ::core::ptr::null_mut::<alloc_handle_t>();
    }
    (*root).current = (*root).first;
    newpool = root as *mut alloc_handle_t;
    return newpool;
}
#[no_mangle]
pub unsafe extern "C" fn Alloc(mut size: ::core::ffi::c_long) -> *mut ::core::ffi::c_char {
    let mut hdr: *mut alloc_hdr_t = (*root).current;
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    size =
        size + 3 as ::core::ffi::c_long & 0xfffffffc as ::core::ffi::c_uint as ::core::ffi::c_long;
    ptr = (*hdr).free;
    (*hdr).free = (*hdr).free.offset(size as isize);
    if (*hdr).free >= (*hdr).end {
        if !(*hdr).next.is_null() {
            (*(*hdr).next).free = (*(*hdr).next).block;
            (*root).current = (*hdr).next as *mut alloc_hdr_t;
        } else {
            (*hdr).next = AllocHdr() as *mut alloc_hdr_s;
            if (*hdr).next.is_null() {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            (*root).current = (*hdr).next as *mut alloc_hdr_t;
        }
        ptr = (*(*root).current).free;
        (*(*root).current).free = (*(*root).current).free.offset(size as isize);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn AllocSetPool(mut newpool: *mut alloc_handle_t) -> *mut alloc_handle_t {
    let mut old: *mut alloc_handle_t = root as *mut alloc_handle_t;
    root = newpool as *mut alloc_root_t;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn AllocReset() {
    (*root).current = (*root).first;
    (*(*root).current).free = (*(*root).current).block;
}
#[no_mangle]
pub unsafe extern "C" fn AllocFreePool() {
    let mut tmp: *mut alloc_hdr_t = ::core::ptr::null_mut::<alloc_hdr_t>();
    let mut hdr: *mut alloc_hdr_t = (*root).first;
    while !hdr.is_null() {
        tmp = (*hdr).next as *mut alloc_hdr_t;
        free((*hdr).block as *mut ::core::ffi::c_void);
        free(hdr as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        hdr = tmp;
    }
    free(root as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    root = ::core::ptr::null_mut::<alloc_root_t>();
}
