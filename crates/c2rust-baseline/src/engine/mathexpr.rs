#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn asin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type __int32_t = i32;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprNode {
    pub opcode: ::core::ffi::c_int,
    pub ivar: ::core::ffi::c_int,
    pub fvalue: ::core::ffi::c_double,
    pub prev: *mut ExprNode,
    pub next: *mut ExprNode,
}
pub type MathExpr = ExprNode;
pub type ExprTree = TreeNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeNode {
    pub opcode: ::core::ffi::c_int,
    pub ivar: ::core::ffi::c_int,
    pub fvalue: ::core::ffi::c_double,
    pub left: *mut TreeNode,
    pub right: *mut TreeNode,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut ::core::ffi::c_char);
}
pub const MAX_STACK_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut Err_0: ::core::ffi::c_int = 0;
static mut Bc: ::core::ffi::c_int = 0;
static mut CurLex: ::core::ffi::c_int = 0;
static mut PrevLex: ::core::ffi::c_int = 0;
static mut Pos: ::core::ffi::c_int = 0;
static mut Len: ::core::ffi::c_int = 0;
static mut S: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut Token: [::core::ffi::c_char; 255] = [0; 255];
static mut Ivar: ::core::ffi::c_int = 0;
static mut Fvalue: ::core::ffi::c_double = 0.;
#[no_mangle]
pub static mut MathFunc: [*mut ::core::ffi::c_char; 20] = [
    b"COS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SIN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TAN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"COT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ABS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SGN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SQRT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"LOG\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"EXP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ASIN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ACOS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ATAN\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ACOT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SINH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"COSH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"TANH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"COTH\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"LOG10\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"STEP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
static mut getVariableIndex: Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
> = None;
unsafe extern "C" fn sametext(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while ({
        let mut __res: ::core::ffi::c_int = 0;
        if ::core::mem::size_of::<::core::ffi::c_char>() as usize > 1 as usize {
            if 0 != 0 {
                let mut __c: ::core::ffi::c_int = *s1.offset(i as isize) as ::core::ffi::c_int;
                __res = (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                    __c as __int32_t
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                }) as ::core::ffi::c_int;
            } else {
                __res = toupper(*s1.offset(i as isize) as ::core::ffi::c_int);
            }
        } else {
            __res = *(*__ctype_toupper_loc())
                .offset(*s1.offset(i as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int;
        }
        __res
    }) == ({
        let mut __res: ::core::ffi::c_int = 0;
        if ::core::mem::size_of::<::core::ffi::c_char>() as usize > 1 as usize {
            if 0 != 0 {
                let mut __c: ::core::ffi::c_int = *s2.offset(i as isize) as ::core::ffi::c_int;
                __res = (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                    __c as __int32_t
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                }) as ::core::ffi::c_int;
            } else {
                __res = toupper(*s2.offset(i as isize) as ::core::ffi::c_int);
            }
        } else {
            __res = *(*__ctype_toupper_loc())
                .offset(*s2.offset(i as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int;
        }
        __res
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
unsafe extern "C" fn isDigit(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= '1' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
        return 1 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int == '0' as i32 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn isLetter(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'z' as i32 {
        return 1 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'Z' as i32 {
        return 1 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int == '_' as i32 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn getToken() {
    let mut c: [::core::ffi::c_char; 2] =
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0");
    Token[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    while Pos <= Len
        && (isLetter(*S.offset(Pos as isize)) != 0 || isDigit(*S.offset(Pos as isize)) != 0)
    {
        c[0 as ::core::ffi::c_int as usize] = *S.offset(Pos as isize);
        strcat(
            &raw mut Token as *mut ::core::ffi::c_char,
            &raw mut c as *mut ::core::ffi::c_char,
        );
        Pos += 1;
    }
    Pos -= 1;
}
unsafe extern "C" fn getMathFunc() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !MathFunc[i as usize].is_null() {
        if sametext(
            MathFunc[i as usize],
            &raw mut Token as *mut ::core::ffi::c_char,
        ) != 0
        {
            return i + 10 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn getVariable() -> ::core::ffi::c_int {
    if getVariableIndex.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    Ivar = getVariableIndex.expect("non-null function pointer")(
        &raw mut Token as *mut ::core::ffi::c_char,
    );
    if Ivar >= 0 as ::core::ffi::c_int {
        return 8 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn getNumber() -> ::core::ffi::c_double {
    let mut c: [::core::ffi::c_char; 2] =
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0");
    let mut sNumber: [::core::ffi::c_char; 255] = [0; 255];
    let mut errflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut decimalCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sNumber[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    while Pos < Len && isDigit(*S.offset(Pos as isize)) != 0 {
        c[0 as ::core::ffi::c_int as usize] = *S.offset(Pos as isize);
        strcat(
            &raw mut sNumber as *mut ::core::ffi::c_char,
            &raw mut c as *mut ::core::ffi::c_char,
        );
        Pos += 1;
    }
    if Pos < Len {
        if *S.offset(Pos as isize) as ::core::ffi::c_int == '.' as i32 {
            decimalCount += 1;
            if decimalCount > 1 as ::core::ffi::c_int {
                Err_0 = 1 as ::core::ffi::c_int;
            }
            strcat(
                &raw mut sNumber as *mut ::core::ffi::c_char,
                b".\0" as *const u8 as *const ::core::ffi::c_char,
            );
            Pos += 1;
            while Pos < Len && isDigit(*S.offset(Pos as isize)) != 0 {
                c[0 as ::core::ffi::c_int as usize] = *S.offset(Pos as isize);
                strcat(
                    &raw mut sNumber as *mut ::core::ffi::c_char,
                    &raw mut c as *mut ::core::ffi::c_char,
                );
                Pos += 1;
            }
        }
        if Pos < Len
            && (*S.offset(Pos as isize) as ::core::ffi::c_int == 'e' as i32
                || *S.offset(Pos as isize) as ::core::ffi::c_int == 'E' as i32)
        {
            strcat(
                &raw mut sNumber as *mut ::core::ffi::c_char,
                b"E\0" as *const u8 as *const ::core::ffi::c_char,
            );
            Pos += 1;
            if Pos >= Len {
                errflag = 1 as ::core::ffi::c_int;
            } else {
                if *S.offset(Pos as isize) as ::core::ffi::c_int == '-' as i32
                    || *S.offset(Pos as isize) as ::core::ffi::c_int == '+' as i32
                {
                    c[0 as ::core::ffi::c_int as usize] = *S.offset(Pos as isize);
                    strcat(
                        &raw mut sNumber as *mut ::core::ffi::c_char,
                        &raw mut c as *mut ::core::ffi::c_char,
                    );
                    Pos += 1;
                }
                if Pos >= Len || isDigit(*S.offset(Pos as isize)) == 0 {
                    errflag = 1 as ::core::ffi::c_int;
                } else {
                    while Pos < Len && isDigit(*S.offset(Pos as isize)) != 0 {
                        c[0 as ::core::ffi::c_int as usize] = *S.offset(Pos as isize);
                        strcat(
                            &raw mut sNumber as *mut ::core::ffi::c_char,
                            &raw mut c as *mut ::core::ffi::c_char,
                        );
                        Pos += 1;
                    }
                }
            }
        }
    }
    Pos -= 1;
    if errflag != 0 {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        return atof(&raw mut sNumber as *mut ::core::ffi::c_char);
    };
}
unsafe extern "C" fn getOperand() -> ::core::ffi::c_int {
    let mut code: ::core::ffi::c_int = 0;
    match *S.offset(Pos as isize) as ::core::ffi::c_int {
        40 => {
            code = 1 as ::core::ffi::c_int;
        }
        41 => {
            code = 2 as ::core::ffi::c_int;
        }
        43 => {
            code = 3 as ::core::ffi::c_int;
        }
        45 => {
            code = 4 as ::core::ffi::c_int;
            if Pos < Len - 1 as ::core::ffi::c_int
                && isDigit(*S.offset((Pos + 1 as ::core::ffi::c_int) as isize)) != 0
                && (CurLex <= 6 as ::core::ffi::c_int || CurLex == 31 as ::core::ffi::c_int)
            {
                Pos += 1;
                Fvalue = -getNumber();
                code = 7 as ::core::ffi::c_int;
            }
        }
        42 => {
            code = 5 as ::core::ffi::c_int;
        }
        47 => {
            code = 6 as ::core::ffi::c_int;
        }
        94 => {
            code = 31 as ::core::ffi::c_int;
        }
        _ => {
            code = 0 as ::core::ffi::c_int;
        }
    }
    return code;
}
unsafe extern "C" fn getLex() -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    while Pos < Len && *S.offset(Pos as isize) as ::core::ffi::c_int == ' ' as i32 {
        Pos += 1;
    }
    if Pos >= Len {
        return 0 as ::core::ffi::c_int;
    }
    n = getOperand();
    if n == 0 as ::core::ffi::c_int {
        if isLetter(*S.offset(Pos as isize)) != 0 {
            getToken();
            n = getMathFunc();
            if n == 0 as ::core::ffi::c_int {
                n = getVariable();
            }
        } else if *S.offset(Pos as isize) as ::core::ffi::c_int == '.' as i32
            || isDigit(*S.offset(Pos as isize)) != 0
        {
            n = 7 as ::core::ffi::c_int;
            Fvalue = getNumber();
        }
    }
    Pos += 1;
    PrevLex = CurLex;
    CurLex = n;
    return n;
}
unsafe extern "C" fn newNode() -> *mut ExprTree {
    let mut node: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    node = malloc(::core::mem::size_of::<ExprTree>() as size_t) as *mut ExprTree;
    if node.is_null() {
        Err_0 = 2 as ::core::ffi::c_int;
    } else {
        (*node).opcode = 0 as ::core::ffi::c_int;
        (*node).ivar = -(1 as ::core::ffi::c_int);
        (*node).fvalue = 0.0f64;
        (*node).left = ::core::ptr::null_mut::<TreeNode>();
        (*node).right = ::core::ptr::null_mut::<TreeNode>();
    }
    return node;
}
unsafe extern "C" fn getSingleOp(mut lex: *mut ::core::ffi::c_int) -> *mut ExprTree {
    let mut opcode: ::core::ffi::c_int = 0;
    let mut left: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    let mut node: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    if *lex == 1 as ::core::ffi::c_int {
        Bc += 1;
        left = getTree();
    } else {
        if *lex < 7 as ::core::ffi::c_int
            || *lex == 9 as ::core::ffi::c_int
            || *lex > 30 as ::core::ffi::c_int
        {
            Err_0 = 1 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<ExprTree>();
        }
        opcode = *lex;
        if *lex == 7 as ::core::ffi::c_int || *lex == 8 as ::core::ffi::c_int {
            left = newNode();
            (*left).opcode = opcode;
            if *lex == 7 as ::core::ffi::c_int {
                (*left).fvalue = Fvalue;
            }
            if *lex == 8 as ::core::ffi::c_int {
                (*left).ivar = Ivar;
            }
        } else {
            *lex = getLex();
            if *lex != 1 as ::core::ffi::c_int {
                Err_0 = 1 as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<ExprTree>();
            }
            Bc += 1;
            left = newNode();
            (*left).left = getTree() as *mut TreeNode;
            (*left).opcode = opcode;
        }
    }
    *lex = getLex();
    if *lex == 31 as ::core::ffi::c_int {
        node = newNode();
        (*node).left = left as *mut TreeNode;
        (*node).opcode = *lex;
        *lex = getLex();
        (*node).right = getSingleOp(lex) as *mut TreeNode;
        left = node;
    }
    return left;
}
unsafe extern "C" fn getOp(mut lex: *mut ::core::ffi::c_int) -> *mut ExprTree {
    let mut opcode: ::core::ffi::c_int = 0;
    let mut left: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    let mut right: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    let mut node: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *lex = getLex();
    if PrevLex == 0 as ::core::ffi::c_int || PrevLex == 1 as ::core::ffi::c_int {
        if *lex == 4 as ::core::ffi::c_int {
            neg = 1 as ::core::ffi::c_int;
            *lex = getLex();
        } else if *lex == 3 as ::core::ffi::c_int {
            *lex = getLex();
        }
    }
    left = getSingleOp(lex);
    while *lex == 5 as ::core::ffi::c_int || *lex == 6 as ::core::ffi::c_int {
        opcode = *lex;
        *lex = getLex();
        right = getSingleOp(lex);
        node = newNode();
        if Err_0 != 0 {
            return ::core::ptr::null_mut::<ExprTree>();
        }
        (*node).left = left as *mut TreeNode;
        (*node).right = right as *mut TreeNode;
        (*node).opcode = opcode;
        left = node;
    }
    if neg != 0 {
        node = newNode();
        if Err_0 != 0 {
            return ::core::ptr::null_mut::<ExprTree>();
        }
        (*node).left = left as *mut TreeNode;
        (*node).right = ::core::ptr::null_mut::<TreeNode>();
        (*node).opcode = 9 as ::core::ffi::c_int;
        left = node;
    }
    return left;
}
unsafe extern "C" fn getTree() -> *mut ExprTree {
    let mut lex: ::core::ffi::c_int = 0;
    let mut opcode: ::core::ffi::c_int = 0;
    let mut left: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    let mut right: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    let mut node: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    left = getOp(&raw mut lex);
    loop {
        if lex == 0 as ::core::ffi::c_int || lex == 2 as ::core::ffi::c_int {
            if lex == 2 as ::core::ffi::c_int {
                Bc -= 1;
            }
            break;
        } else if lex != 3 as ::core::ffi::c_int && lex != 4 as ::core::ffi::c_int {
            Err_0 = 1 as ::core::ffi::c_int;
            break;
        } else {
            opcode = lex;
            right = getOp(&raw mut lex);
            node = newNode();
            if Err_0 != 0 {
                break;
            }
            (*node).left = left as *mut TreeNode;
            (*node).right = right as *mut TreeNode;
            (*node).opcode = opcode;
            left = node;
        }
    }
    return left;
}
unsafe extern "C" fn traverseTree(mut tree: *mut ExprTree, mut expr: *mut *mut MathExpr) {
    let mut node: *mut MathExpr = ::core::ptr::null_mut::<MathExpr>();
    if tree.is_null() {
        return;
    }
    traverseTree((*tree).left as *mut ExprTree, expr);
    traverseTree((*tree).right as *mut ExprTree, expr);
    node = malloc(::core::mem::size_of::<MathExpr>() as size_t) as *mut MathExpr;
    if !node.is_null() {
        (*node).fvalue = (*tree).fvalue;
        (*node).opcode = (*tree).opcode;
        (*node).ivar = (*tree).ivar;
        (*node).next = ::core::ptr::null_mut::<ExprNode>();
        (*node).prev = *expr as *mut ExprNode;
    }
    if !(*expr).is_null() {
        (**expr).next = node as *mut ExprNode;
    }
    *expr = node;
}
unsafe extern "C" fn deleteTree(mut tree: *mut ExprTree) {
    if !tree.is_null() {
        if !(*tree).left.is_null() {
            deleteTree((*tree).left as *mut ExprTree);
        }
        if !(*tree).right.is_null() {
            deleteTree((*tree).right as *mut ExprTree);
        }
        free(tree as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mathexpr_eval(
    mut expr: *mut MathExpr,
    mut getVariableValue: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_double>,
) -> ::core::ffi::c_double {
    let mut ExprStack: [::core::ffi::c_double; 1024] = [0.; 1024];
    let mut node: *mut MathExpr = expr;
    let mut r1: ::core::ffi::c_double = 0.;
    let mut r2: ::core::ffi::c_double = 0.;
    let mut stackindex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ExprStack[0 as ::core::ffi::c_int as usize] = 0.0f64;
    while !node.is_null() && stackindex >= 0 as ::core::ffi::c_int {
        match (*node).opcode {
            3 => {
                r1 = ExprStack[stackindex as usize];
                stackindex -= 1;
                if !(stackindex < 0 as ::core::ffi::c_int) {
                    r2 = ExprStack[stackindex as usize];
                    ExprStack[stackindex as usize] = r2 + r1;
                }
            }
            4 => {
                r1 = ExprStack[stackindex as usize];
                stackindex -= 1;
                if !(stackindex < 0 as ::core::ffi::c_int) {
                    r2 = ExprStack[stackindex as usize];
                    ExprStack[stackindex as usize] = r2 - r1;
                }
            }
            5 => {
                r1 = ExprStack[stackindex as usize];
                stackindex -= 1;
                if !(stackindex < 0 as ::core::ffi::c_int) {
                    r2 = ExprStack[stackindex as usize];
                    ExprStack[stackindex as usize] = r2 * r1;
                }
            }
            6 => {
                r1 = ExprStack[stackindex as usize];
                stackindex -= 1;
                if !(stackindex < 0 as ::core::ffi::c_int) {
                    r2 = ExprStack[stackindex as usize];
                    ExprStack[stackindex as usize] = r2 / r1;
                }
            }
            7 => {
                stackindex += 1;
                if !(stackindex >= MAX_STACK_SIZE) {
                    ExprStack[stackindex as usize] = (*node).fvalue;
                }
            }
            8 => {
                if getVariableValue.is_some() {
                    r1 = getVariableValue.expect("non-null function pointer")((*node).ivar);
                } else {
                    r1 = 0.0f64;
                }
                stackindex += 1;
                if !(stackindex >= MAX_STACK_SIZE) {
                    ExprStack[stackindex as usize] = r1;
                }
            }
            9 => {
                ExprStack[stackindex as usize] = -ExprStack[stackindex as usize];
            }
            10 => {
                r1 = ExprStack[stackindex as usize];
                r2 = cos(r1);
                ExprStack[stackindex as usize] = r2;
            }
            11 => {
                r1 = ExprStack[stackindex as usize];
                r2 = sin(r1);
                ExprStack[stackindex as usize] = r2;
            }
            12 => {
                r1 = ExprStack[stackindex as usize];
                r2 = tan(r1);
                ExprStack[stackindex as usize] = r2;
            }
            13 => {
                r1 = ExprStack[stackindex as usize];
                if r1 == 0.0f64 {
                    r2 = 0.0f64;
                } else {
                    r2 = 1.0f64 / tan(r1);
                }
                ExprStack[stackindex as usize] = r2;
            }
            14 => {
                r1 = ExprStack[stackindex as usize];
                r2 = fabs(r1);
                ExprStack[stackindex as usize] = r2;
            }
            15 => {
                r1 = ExprStack[stackindex as usize];
                if r1 < 0.0f64 {
                    r2 = -1.0f64;
                } else if r1 > 0.0f64 {
                    r2 = 1.0f64;
                } else {
                    r2 = 0.0f64;
                }
                ExprStack[stackindex as usize] = r2;
            }
            16 => {
                r1 = ExprStack[stackindex as usize];
                if r1 < 0.0f64 {
                    r2 = 0.0f64;
                } else {
                    r2 = sqrt(r1);
                }
                ExprStack[stackindex as usize] = r2;
            }
            17 => {
                r1 = ExprStack[stackindex as usize];
                if r1 <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    r2 = 0.0f64;
                } else {
                    r2 = log(r1);
                }
                ExprStack[stackindex as usize] = r2;
            }
            18 => {
                r1 = ExprStack[stackindex as usize];
                r2 = exp(r1);
                ExprStack[stackindex as usize] = r2;
            }
            19 => {
                r1 = ExprStack[stackindex as usize];
                r2 = asin(r1);
                ExprStack[stackindex as usize] = r2;
            }
            20 => {
                r1 = ExprStack[stackindex as usize];
                r2 = acos(r1);
                ExprStack[stackindex as usize] = r2;
            }
            21 => {
                r1 = ExprStack[stackindex as usize];
                r2 = atan(r1);
                ExprStack[stackindex as usize] = r2;
            }
            22 => {
                r1 = ExprStack[stackindex as usize];
                r2 = 1.57079632679489661923f64 - atan(r1);
                ExprStack[stackindex as usize] = r2;
            }
            23 => {
                r1 = ExprStack[stackindex as usize];
                r2 = (exp(r1) - exp(-r1)) / 2.0f64;
                ExprStack[stackindex as usize] = r2;
            }
            24 => {
                r1 = ExprStack[stackindex as usize];
                r2 = (exp(r1) + exp(-r1)) / 2.0f64;
                ExprStack[stackindex as usize] = r2;
            }
            25 => {
                r1 = ExprStack[stackindex as usize];
                r2 = (exp(r1) - exp(-r1)) / (exp(r1) + exp(-r1));
                ExprStack[stackindex as usize] = r2;
            }
            26 => {
                r1 = ExprStack[stackindex as usize];
                r2 = (exp(r1) + exp(-r1)) / (exp(r1) - exp(-r1));
                ExprStack[stackindex as usize] = r2;
            }
            27 => {
                r1 = ExprStack[stackindex as usize];
                if r1 == 0.0f64 {
                    r2 = 0.0f64;
                } else {
                    r2 = log10(r1);
                }
                ExprStack[stackindex as usize] = r2;
            }
            28 => {
                r1 = ExprStack[stackindex as usize];
                if r1 <= 0.0f64 {
                    r2 = 0.0f64;
                } else {
                    r2 = 1.0f64;
                }
                ExprStack[stackindex as usize] = r2;
            }
            31 => {
                r1 = ExprStack[stackindex as usize];
                stackindex -= 1;
                if !(stackindex < 0 as ::core::ffi::c_int) {
                    r2 = ExprStack[stackindex as usize];
                    if r2 <= 0.0f64 {
                        r2 = 0.0f64;
                    } else {
                        r2 = pow(r2, r1);
                    }
                    ExprStack[stackindex as usize] = r2;
                }
            }
            _ => {}
        }
        node = (*node).next as *mut MathExpr;
    }
    if stackindex >= 0 as ::core::ffi::c_int {
        r1 = ExprStack[stackindex as usize];
    } else {
        r1 = 0.0f64;
    }
    if r1 != r1 {
        r1 = 0.0f64;
    }
    return r1;
}
#[no_mangle]
pub unsafe extern "C" fn mathexpr_delete(mut expr: *mut MathExpr) {
    if !expr.is_null() {
        mathexpr_delete((*expr).next as *mut MathExpr);
    }
    free(expr as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mathexpr_create(
    mut formula: *mut ::core::ffi::c_char,
    mut getVar: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>,
) -> *mut MathExpr {
    let mut tree: *mut ExprTree = ::core::ptr::null_mut::<ExprTree>();
    let mut expr: *mut MathExpr = ::core::ptr::null_mut::<MathExpr>();
    let mut result: *mut MathExpr = ::core::ptr::null_mut::<MathExpr>();
    getVariableIndex = getVar;
    Err_0 = 0 as ::core::ffi::c_int;
    PrevLex = 0 as ::core::ffi::c_int;
    CurLex = 0 as ::core::ffi::c_int;
    S = formula;
    Len = strlen(S) as ::core::ffi::c_int;
    Pos = 0 as ::core::ffi::c_int;
    Bc = 0 as ::core::ffi::c_int;
    tree = getTree();
    if Bc == 0 as ::core::ffi::c_int && Err_0 == 0 as ::core::ffi::c_int {
        traverseTree(tree, &raw mut expr);
        while !expr.is_null() {
            result = expr;
            expr = (*expr).prev as *mut MathExpr;
        }
    }
    deleteTree(tree);
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
