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
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HTentry {
    pub key: *mut ::core::ffi::c_char,
    pub data: ::core::ffi::c_int,
    pub next: *mut HTentry,
}
pub type HTtable = *mut HTentry;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const HTMAXSIZE: ::core::ffi::c_int = 1999 as ::core::ffi::c_int;
pub const NOTFOUND: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn samestr(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (if *s1.offset(i as isize) as ::core::ffi::c_int >= 'a' as i32
        && *s1.offset(i as isize) as ::core::ffi::c_int <= 'z' as i32
    {
        *s1.offset(i as isize) as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
    } else {
        *s1.offset(i as isize) as ::core::ffi::c_int
    }) == (if *s2.offset(i as isize) as ::core::ffi::c_int >= 'a' as i32
        && *s2.offset(i as isize) as ::core::ffi::c_int <= 'z' as i32
    {
        *s2.offset(i as isize) as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
    } else {
        *s2.offset(i as isize) as ::core::ffi::c_int
    }) {
        if *s1.offset((i + 1 as ::core::ffi::c_int) as isize) == 0
            && *s2.offset((i + 1 as ::core::ffi::c_int) as isize) == 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hash(mut str: *const ::core::ffi::c_char) -> ::core::ffi::c_uint {
    let mut sum1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut check1: ::core::ffi::c_uint = 0;
    let mut sum2: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    while '\0' as i32 != *str as ::core::ffi::c_int {
        sum1 = sum1.wrapping_add(
            (if *str as ::core::ffi::c_int >= 'a' as i32 && *str as ::core::ffi::c_int <= 'z' as i32
            {
                *str as ::core::ffi::c_int & !(32 as ::core::ffi::c_int)
            } else {
                *str as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        str = str.offset(1);
        if 255 as ::core::ffi::c_uint <= sum1 {
            sum1 = sum1.wrapping_sub(255 as ::core::ffi::c_uint);
        }
        sum2 = sum2.wrapping_add(sum1 as ::core::ffi::c_ulong);
    }
    check1 = sum2 as ::core::ffi::c_uint;
    check1 = check1.wrapping_rem(255 as ::core::ffi::c_uint);
    check1 = (255 as ::core::ffi::c_uint).wrapping_sub(
        sum1.wrapping_add(check1)
            .wrapping_rem(255 as ::core::ffi::c_uint),
    );
    sum1 = (255 as ::core::ffi::c_uint).wrapping_sub(
        sum1.wrapping_add(check1)
            .wrapping_rem(255 as ::core::ffi::c_uint),
    );
    return (check1 << 8 as ::core::ffi::c_int | sum1)
        .wrapping_rem(HTMAXSIZE as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn HTcreate() -> *mut HTtable {
    let mut i: ::core::ffi::c_int = 0;
    let mut ht: *mut HTtable = calloc(
        HTMAXSIZE as size_t,
        ::core::mem::size_of::<HTtable>() as size_t,
    ) as *mut HTtable;
    if !ht.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < HTMAXSIZE {
            let ref mut fresh0 = *ht.offset(i as isize);
            *fresh0 = ::core::ptr::null_mut::<HTentry>();
            i += 1;
        }
    }
    return ht;
}
#[no_mangle]
pub unsafe extern "C" fn HTinsert(
    mut ht: *mut HTtable,
    mut key: *mut ::core::ffi::c_char,
    mut data: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = hash(key);
    let mut entry: *mut HTentry = ::core::ptr::null_mut::<HTentry>();
    if i >= HTMAXSIZE as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    entry = malloc(::core::mem::size_of::<HTentry>() as size_t) as *mut HTentry;
    if entry.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*entry).key = key;
    (*entry).data = data;
    (*entry).next = *ht.offset(i as isize) as *mut HTentry;
    let ref mut fresh1 = *ht.offset(i as isize);
    *fresh1 = entry as HTtable;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HTfind(
    mut ht: *mut HTtable,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = hash(key);
    let mut entry: *mut HTentry = ::core::ptr::null_mut::<HTentry>();
    if i >= HTMAXSIZE as ::core::ffi::c_uint {
        return -(1 as ::core::ffi::c_int);
    }
    entry = *ht.offset(i as isize) as *mut HTentry;
    while !entry.is_null() {
        if samestr((*entry).key, key) != 0 {
            return (*entry).data;
        }
        entry = (*entry).next;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn HTfindKey(
    mut ht: *mut HTtable,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_uint = hash(key);
    let mut entry: *mut HTentry = ::core::ptr::null_mut::<HTentry>();
    if i >= HTMAXSIZE as ::core::ffi::c_uint {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    entry = *ht.offset(i as isize) as *mut HTentry;
    while !entry.is_null() {
        if samestr((*entry).key, key) != 0 {
            return (*entry).key;
        }
        entry = (*entry).next;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn HTfree(mut ht: *mut HTtable) {
    let mut entry: *mut HTentry = ::core::ptr::null_mut::<HTentry>();
    let mut nextentry: *mut HTentry = ::core::ptr::null_mut::<HTentry>();
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < HTMAXSIZE {
        entry = *ht.offset(i as isize) as *mut HTentry;
        while !entry.is_null() {
            nextentry = (*entry).next;
            free(entry as *mut ::core::ffi::c_void);
            entry = nextentry;
        }
        i += 1;
    }
    free(ht as *mut ::core::ffi::c_void);
}
