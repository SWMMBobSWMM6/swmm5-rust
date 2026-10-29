//! mathexpr.c of EPA SWMM 5.2.4, in safe Rust.
/******************************************************************************
**  MODULE:        MATHEXPR.C
**  PROJECT:       EPA SWMM5
**  DESCRIPTION:   Evaluates symbolic mathematical expression consisting
**                 of numbers, variable names, math functions & arithmetic
**                 operators.
**  AUTHORS:       L. Rossman, US EPA - NRMRL
**                 F. Shang, University of Cincinnati
**  VERSION:       5.2.2
**  LAST UPDATE:   09/02/2022
**  BUG FIXES:     Problems related to '^' operator (F. Shang, 09/02/2022)
******************************************************************************/
/*
**   Operand codes:
** 	   1 = (
** 	   2 = )
** 	   3 = +
** 	   4 = - (subtraction)
** 	   5 = *
** 	   6 = /
** 	   7 = number
** 	   8 = user-defined variable
** 	   9 = - (negative)
**	  10 = cos
**	  11 = sin
**	  12 = tan
**	  13 = cot
**	  14 = abs
**	  15 = sgn
**	  16 = sqrt
**	  17 = log
**	  18 = exp
**	  19 = asin
**	  20 = acos
**	  21 = atan
**	  22 = acot
**        23 = sinh
**	  24 = cosh
**	  25 = tanh
**	  26 = coth
**	  27 = log10
**        28 = step (x<=0 ? 0 : 1)
**	  31 = ^
******************************************************************************/
//
// Translation notes:
// * The parser's file-scope state (Err, Bc, PrevLex, CurLex, Len, Pos, S,
//   Token, Ivar, Fvalue, getVariableIndex) is the struct Parser, local to
//   one call of mathexpr_create.
// * The binary tree (struct TreeNode) lives in an arena (Vec) and children
//   are indexes; NULL is None.
// * The postfix list C builds by traverseTree (prev/next links) is the
//   MathExpr vector, in the same order.

use crate::prelude::*;

/// One term of a tokenized expression in postfix order (C: `struct ExprNode`,
/// whose prev/next links become the order of this vector).
#[derive(Clone, Copy, Debug, Default)]
pub struct ExprNode {
    pub opcode: i32,
    pub ivar: i32,
    pub fvalue: f64,
}
/// C: `typedef struct ExprNode MathExpr;` a list of terms.
pub type MathExpr = Vec<ExprNode>;

#[derive(Default)]
pub struct MathexprState {}

const MAX_STACK_SIZE: usize = 1024;

//  Local declarations
//--------------------
//  Structure for binary tree representation of math expression
#[derive(Clone, Copy)]
struct TreeNode {
    opcode: i32,           // operator code
    ivar: i32,             // variable index
    fvalue: f64,           // numerical value
    left: Option<usize>,   // left sub-tree of tokenized formula
    right: Option<usize>,  // right sub-tree of tokenized formula
}

// math function names
const MathFunc: [&[u8]; 19] = [
    b"COS", b"SIN", b"TAN", b"COT", b"ABS", b"SGN", //
    b"SQRT", b"LOG", b"EXP", b"ASIN", b"ACOS", b"ATAN", //
    b"ACOT", b"SINH", b"COSH", b"TANH", b"COTH", b"LOG10", //
    b"STEP",
];

/// The parser's state (C: the file-scope variables of mathexpr.c).
struct Parser<'a> {
    Err: i32,
    Bc: i32,
    PrevLex: i32,
    CurLex: i32,
    Len: i32,
    Pos: i32,
    S: &'a [u8],
    Token: Vec<u8>,
    Ivar: i32,
    Fvalue: f64,
    // Callback functions
    getVariableIndex: &'a mut dyn FnMut(&[u8]) -> i32, // return index of named variable
    tree: Vec<TreeNode>,
}

/// Byte i of a C string held in a slice (NUL at and past its end).
#[inline]
fn at(s: &[u8], i: i32) -> u8 {
    if i >= 0 && (i as usize) < s.len() {
        s[i as usize]
    } else {
        0
    }
}

//=============================================================================

fn sametext(s1: &[u8], s2: &[u8]) -> i32
/*
**  Purpose:
**    performs case insensitive comparison of two strings.
**
**  Input:
**    s1 = character string
**    s2 = character string.
**
**  Returns:
**    1 if strings are the same, 0 otherwise.
*/
{
    let mut i: i32 = 0;
    while cio::toupper(at(s1, i)) == cio::toupper(at(s2, i)) {
        if at(s1, i + 1) == 0 && at(s2, i + 1) == 0 {
            return 1;
        }
        i += 1;
    }
    0
}

//=============================================================================

fn isDigit(c: u8) -> i32 {
    if c >= b'1' && c <= b'9' {
        return 1;
    }
    if c == b'0' {
        return 1;
    }
    0
}

//=============================================================================

fn isLetter(c: u8) -> i32 {
    if c >= b'a' && c <= b'z' {
        return 1;
    }
    if c >= b'A' && c <= b'Z' {
        return 1;
    }
    if c == b'_' {
        return 1;
    }
    0
}

impl<'a> Parser<'a> {
    #[inline]
    fn S(&self, i: i32) -> u8 {
        at(self.S, i)
    }

    //=========================================================================

    fn getToken(&mut self) {
        // NOTE: C appends to `char Token[255]` with strcat and would overflow
        // on a name of 255+ characters; the Vec here just grows.
        self.Token.clear();
        while self.Pos <= self.Len && (isLetter(self.S(self.Pos)) != 0 || isDigit(self.S(self.Pos)) != 0) {
            let c = self.S(self.Pos);
            self.Token.push(c);
            self.Pos += 1;
        }
        self.Pos -= 1;
    }

    //=========================================================================

    fn getMathFunc(&mut self) -> i32 {
        let mut i: i32 = 0;
        while (i as usize) < MathFunc.len() {
            if sametext(MathFunc[i as usize], &self.Token) != 0 {
                return i + 10;
            }
            i += 1;
        }
        0
    }

    //=========================================================================

    fn getVariable(&mut self) -> i32 {
        // C: if ( !getVariableIndex ) return 0;  (a callback is always given here)
        self.Ivar = (self.getVariableIndex)(&self.Token);
        if self.Ivar >= 0 {
            return 8;
        }
        0
    }

    //=========================================================================

    fn getNumber(&mut self) -> f64 {
        // NOTE: C builds the number in `char sNumber[255]` with strcat and
        // would overflow on 255+ characters; the Vec here just grows.
        let mut sNumber: Vec<u8> = Vec::new();
        let mut errflag: i32 = 0;
        let mut decimalCount: i32 = 0;

        /* --- get whole number portion of number */
        while self.Pos < self.Len && isDigit(self.S(self.Pos)) != 0 {
            sNumber.push(self.S(self.Pos));
            self.Pos += 1;
        }

        /* --- get fractional portion of number */
        if self.Pos < self.Len {
            if self.S(self.Pos) == b'.' {
                decimalCount += 1;
                if decimalCount > 1 {
                    self.Err = 1;
                }
                sNumber.push(b'.');
                self.Pos += 1;
                while self.Pos < self.Len && isDigit(self.S(self.Pos)) != 0 {
                    sNumber.push(self.S(self.Pos));
                    self.Pos += 1;
                }
            }

            /* --- get exponent */
            if self.Pos < self.Len && (self.S(self.Pos) == b'e' || self.S(self.Pos) == b'E') {
                sNumber.push(b'E');
                self.Pos += 1;
                if self.Pos >= self.Len {
                    errflag = 1;
                } else {
                    if self.S(self.Pos) == b'-' || self.S(self.Pos) == b'+' {
                        sNumber.push(self.S(self.Pos));
                        self.Pos += 1;
                    }
                    if self.Pos >= self.Len || isDigit(self.S(self.Pos)) == 0 {
                        errflag = 1;
                    } else {
                        while self.Pos < self.Len && isDigit(self.S(self.Pos)) != 0 {
                            sNumber.push(self.S(self.Pos));
                            self.Pos += 1;
                        }
                    }
                }
            }
        }
        self.Pos -= 1;
        if errflag != 0 {
            0.0
        } else {
            cio::atof(&sNumber)
        }
    }

    //=========================================================================

    fn getOperand(&mut self) -> i32 {
        let mut code: i32;
        match self.S(self.Pos) {
            b'(' => code = 1,
            b')' => code = 2,
            b'+' => code = 3,
            b'-' => {
                code = 4;
                if self.Pos < self.Len - 1 && isDigit(self.S(self.Pos + 1)) != 0 && (self.CurLex <= 6 || self.CurLex == 31) {
                    self.Pos += 1;
                    self.Fvalue = -self.getNumber();
                    code = 7;
                }
            }
            b'*' => code = 5,
            b'/' => code = 6,
            b'^' => code = 31,
            _ => code = 0,
        }
        code
    }

    //=========================================================================

    fn getLex(&mut self) -> i32 {
        let mut n: i32;

        /* --- skip spaces */
        while self.Pos < self.Len && self.S(self.Pos) == b' ' {
            self.Pos += 1;
        }
        if self.Pos >= self.Len {
            return 0;
        }

        /* --- check for operand */
        n = self.getOperand();

        /* --- check for function/variable/number */
        if n == 0 {
            if isLetter(self.S(self.Pos)) != 0 {
                self.getToken();
                n = self.getMathFunc();
                if n == 0 {
                    n = self.getVariable();
                }
            } else if self.S(self.Pos) == b'.' || isDigit(self.S(self.Pos)) != 0 {
                n = 7;
                self.Fvalue = self.getNumber();
            }
        }
        self.Pos += 1;
        self.PrevLex = self.CurLex;
        self.CurLex = n;
        n
    }

    //=========================================================================

    fn newNode(&mut self) -> Option<usize> {
        self.tree.push(TreeNode {
            opcode: 0,
            ivar: -1,
            fvalue: 0.0,
            left: None,
            right: None,
        });
        Some(self.tree.len() - 1)
    }

    //=========================================================================

    fn getSingleOp(&mut self, lex: &mut i32) -> Option<usize> {
        let opcode: i32;
        let mut left: Option<usize>;
        let node: Option<usize>;

        /* --- open parenthesis, so continue to grow the tree */
        if *lex == 1 {
            self.Bc += 1;
            left = self.getTree();
        } else {
            /* --- Error if not a singleton operand */
            if *lex < 7 || *lex == 9 || *lex > 30 {
                self.Err = 1;
                return None;
            }

            opcode = *lex;

            /* --- simple number or variable name */
            if *lex == 7 || *lex == 8 {
                left = self.newNode();
                let l = left.unwrap();
                self.tree[l].opcode = opcode;
                if *lex == 7 {
                    self.tree[l].fvalue = self.Fvalue;
                }
                if *lex == 8 {
                    self.tree[l].ivar = self.Ivar;
                }
            }
            /* --- function which must have a '(' after it */
            else {
                *lex = self.getLex();
                if *lex != 1 {
                    self.Err = 1;
                    return None;
                }
                self.Bc += 1;
                left = self.newNode();
                let sub = self.getTree();
                let l = left.unwrap();
                self.tree[l].left = sub;
                self.tree[l].opcode = opcode;
            }
        }
        *lex = self.getLex();

        /* --- exponentiation */
        if *lex == 31 {
            node = self.newNode();
            let nd = node.unwrap();
            self.tree[nd].left = left;
            self.tree[nd].opcode = *lex;
            *lex = self.getLex();
            let r = self.getSingleOp(lex);
            self.tree[nd].right = r;
            left = node;
        }
        left
    }

    //=========================================================================

    fn getOp(&mut self, lex: &mut i32) -> Option<usize> {
        let mut opcode: i32;
        let mut left: Option<usize>;
        let mut right: Option<usize>;
        let mut node: Option<usize>;
        let mut neg: i32 = 0;

        *lex = self.getLex();
        if self.PrevLex == 0 || self.PrevLex == 1 {
            if *lex == 4 {
                neg = 1;
                *lex = self.getLex();
            } else if *lex == 3 {
                *lex = self.getLex();
            }
        }
        left = self.getSingleOp(lex);
        while *lex == 5 || *lex == 6 {
            opcode = *lex;
            *lex = self.getLex();
            right = self.getSingleOp(lex);
            node = self.newNode();
            if self.Err != 0 {
                return None;
            }
            let nd = node.unwrap();
            self.tree[nd].left = left;
            self.tree[nd].right = right;
            self.tree[nd].opcode = opcode;
            left = node;
        }
        if neg != 0 {
            node = self.newNode();
            if self.Err != 0 {
                return None;
            }
            let nd = node.unwrap();
            self.tree[nd].left = left;
            self.tree[nd].right = None;
            self.tree[nd].opcode = 9;
            left = node;
        }
        left
    }

    //=========================================================================

    fn getTree(&mut self) -> Option<usize> {
        let mut lex: i32 = 0;
        let mut opcode: i32;
        let mut left: Option<usize>;
        let mut right: Option<usize>;
        let mut node: Option<usize>;

        left = self.getOp(&mut lex);
        loop {
            if lex == 0 || lex == 2 {
                if lex == 2 {
                    self.Bc -= 1;
                }
                break;
            }

            if lex != 3 && lex != 4 {
                self.Err = 1;
                break;
            }

            opcode = lex;
            right = self.getOp(&mut lex);
            node = self.newNode();
            if self.Err != 0 {
                break;
            }
            let nd = node.unwrap();
            self.tree[nd].left = left;
            self.tree[nd].right = right;
            self.tree[nd].opcode = opcode;
            left = node;
        }
        left
    }

    //=========================================================================

    // Converts binary tree to linked list (postfix format)
    fn traverseTree(&self, tree: Option<usize>, expr: &mut MathExpr) {
        let t = match tree {
            None => return,
            Some(t) => self.tree[t],
        };
        self.traverseTree(t.left, expr);
        self.traverseTree(t.right, expr);
        expr.push(ExprNode {
            fvalue: t.fvalue,
            opcode: t.opcode,
            ivar: t.ivar,
        });
    }
}

//=============================================================================

/// C: `double mathexpr_eval(MathExpr *expr, double (*getVariableValue) (int))`
//  Mathematica expression evaluation using a stack
pub fn mathexpr_eval(expr: &MathExpr, getVariableValue: &mut dyn FnMut(i32) -> f64) -> f64 {
    // --- Note: the ExprStack array must be declared locally and not globally
    //     since this function can be called recursively.

    // NOTE: C leaves ExprStack[1..] uninitialized; a postfix list built by
    // mathexpr_create never reads an entry it has not written, so zeros
    // here change nothing.
    let mut ExprStack: [f64; MAX_STACK_SIZE] = [0.0; MAX_STACK_SIZE];
    let mut node: usize = 0;
    let mut r1: f64;
    let mut r2: f64;
    let mut stackindex: i32 = 0;

    // NOTE: C lets stackindex reach MAX_STACK_SIZE (after `break` on
    // overflow the loop goes on) and then reads past the array; that needs
    // an expression nested more than 1023 levels deep, longer than any
    // SWMM input line. Here it would panic.
    ExprStack[0] = 0.0;
    while node < expr.len() && stackindex >= 0 {
        let nd = &expr[node];
        let si = |k: i32| k as usize;
        match nd.opcode {
            3 => {
                r1 = ExprStack[si(stackindex)];
                stackindex -= 1;
                if stackindex >= 0 {
                    r2 = ExprStack[si(stackindex)];
                    ExprStack[si(stackindex)] = r2 + r1;
                }
            }

            4 => {
                r1 = ExprStack[si(stackindex)];
                stackindex -= 1;
                if stackindex >= 0 {
                    r2 = ExprStack[si(stackindex)];
                    ExprStack[si(stackindex)] = r2 - r1;
                }
            }

            5 => {
                r1 = ExprStack[si(stackindex)];
                stackindex -= 1;
                if stackindex >= 0 {
                    r2 = ExprStack[si(stackindex)];
                    ExprStack[si(stackindex)] = r2 * r1;
                }
            }

            6 => {
                r1 = ExprStack[si(stackindex)];
                stackindex -= 1;
                if stackindex >= 0 {
                    r2 = ExprStack[si(stackindex)];
                    ExprStack[si(stackindex)] = r2 / r1;
                }
            }

            7 => {
                stackindex += 1;
                if (stackindex as usize) < MAX_STACK_SIZE {
                    ExprStack[si(stackindex)] = nd.fvalue;
                }
            }

            8 => {
                // C: if (getVariableValue != NULL) r1 = getVariableValue(node->ivar);
                r1 = getVariableValue(nd.ivar);
                stackindex += 1;
                if (stackindex as usize) < MAX_STACK_SIZE {
                    ExprStack[si(stackindex)] = r1;
                }
            }

            9 => {
                ExprStack[si(stackindex)] = -ExprStack[si(stackindex)];
            }

            10 => {
                r1 = ExprStack[si(stackindex)];
                r2 = r1.cos();
                ExprStack[si(stackindex)] = r2;
            }

            11 => {
                r1 = ExprStack[si(stackindex)];
                r2 = r1.sin();
                ExprStack[si(stackindex)] = r2;
            }

            12 => {
                r1 = ExprStack[si(stackindex)];
                r2 = r1.tan();
                ExprStack[si(stackindex)] = r2;
            }

            13 => {
                r1 = ExprStack[si(stackindex)];
                if r1 == 0.0 {
                    r2 = 0.0;
                } else {
                    r2 = 1.0 / r1.tan();
                }
                ExprStack[si(stackindex)] = r2;
            }

            14 => {
                r1 = ExprStack[si(stackindex)];
                r2 = r1.abs();
                ExprStack[si(stackindex)] = r2;
            }

            15 => {
                r1 = ExprStack[si(stackindex)];
                if r1 < 0.0 {
                    r2 = -1.0;
                } else if r1 > 0.0 {
                    r2 = 1.0;
                } else {
                    r2 = 0.0;
                }
                ExprStack[si(stackindex)] = r2;
            }

            16 => {
                r1 = ExprStack[si(stackindex)];
                if r1 < 0.0 {
                    r2 = 0.0;
                } else {
                    r2 = r1.sqrt();
                }
                ExprStack[si(stackindex)] = r2;
            }

            17 => {
                r1 = ExprStack[si(stackindex)];
                if r1 <= 0.0 {
                    r2 = 0.0;
                } else {
                    r2 = r1.ln();
                }
                ExprStack[si(stackindex)] = r2;
            }

            18 => {
                r1 = ExprStack[si(stackindex)];
                r2 = r1.exp();
                ExprStack[si(stackindex)] = r2;
            }

            19 => {
                r1 = ExprStack[si(stackindex)];
                r2 = r1.asin();
                ExprStack[si(stackindex)] = r2;
            }

            20 => {
                r1 = ExprStack[si(stackindex)];
                r2 = r1.acos();
                ExprStack[si(stackindex)] = r2;
            }

            21 => {
                r1 = ExprStack[si(stackindex)];
                r2 = r1.atan();
                ExprStack[si(stackindex)] = r2;
            }

            22 => {
                r1 = ExprStack[si(stackindex)];
                r2 = 1.57079632679489661923 - r1.atan();
                ExprStack[si(stackindex)] = r2;
            }

            23 => {
                r1 = ExprStack[si(stackindex)];
                r2 = (r1.exp() - (-r1).exp()) / 2.0;
                ExprStack[si(stackindex)] = r2;
            }

            24 => {
                r1 = ExprStack[si(stackindex)];
                r2 = (r1.exp() + (-r1).exp()) / 2.0;
                ExprStack[si(stackindex)] = r2;
            }

            25 => {
                r1 = ExprStack[si(stackindex)];
                r2 = (r1.exp() - (-r1).exp()) / (r1.exp() + (-r1).exp());
                ExprStack[si(stackindex)] = r2;
            }

            26 => {
                r1 = ExprStack[si(stackindex)];
                r2 = (r1.exp() + (-r1).exp()) / (r1.exp() - (-r1).exp());
                ExprStack[si(stackindex)] = r2;
            }

            27 => {
                r1 = ExprStack[si(stackindex)];
                if r1 == 0.0 {
                    r2 = 0.0;
                } else {
                    r2 = r1.log10();
                }
                ExprStack[si(stackindex)] = r2;
            }

            28 => {
                r1 = ExprStack[si(stackindex)];
                if r1 <= 0.0 {
                    r2 = 0.0;
                } else {
                    r2 = 1.0;
                }
                ExprStack[si(stackindex)] = r2;
            }

            31 => {
                r1 = ExprStack[si(stackindex)];
                stackindex -= 1;
                if stackindex >= 0 {
                    r2 = ExprStack[si(stackindex)];
                    if r2 <= 0.0 {
                        r2 = 0.0;
                    } else {
                        r2 = r2.powf(r1);
                    }
                    ExprStack[si(stackindex)] = r2;
                }
            }
            _ => {}
        }
        node += 1;
    }
    if stackindex >= 0 {
        r1 = ExprStack[stackindex as usize];
    } else {
        r1 = 0.0;
    }

    // Set result to 0 if it is NaN due to an illegal math op
    if r1 != r1 {
        r1 = 0.0;
    }

    r1
}

//=============================================================================

pub fn mathexpr_delete(expr: Option<MathExpr>) {
    drop(expr);
}

//=============================================================================

/// C: `MathExpr * mathexpr_create(char *formula, int (*getVar) (char *))`;
/// None is C's NULL (a formula that does not parse).
pub fn mathexpr_create(formula: &[u8], getVar: &mut dyn FnMut(&[u8]) -> i32) -> Option<MathExpr> {
    let tree: Option<usize>;
    let mut expr: MathExpr = Vec::new();
    let mut result: Option<MathExpr> = None;
    let S = cio::cstr(formula);
    let mut p = Parser {
        Err: 0,
        Bc: 0,
        PrevLex: 0,
        CurLex: 0,
        Len: S.len() as i32,
        Pos: 0,
        S,
        Token: Vec::new(),
        Ivar: 0,
        Fvalue: 0.0,
        getVariableIndex: getVar,
        tree: Vec::new(),
    };
    tree = p.getTree();
    if p.Bc == 0 && p.Err == 0 {
        p.traverseTree(tree, &mut expr);
        // C walks back from the last node to the first: the head of the list
        if !expr.is_empty() {
            result = Some(expr);
        }
    }
    // deleteTree(tree): the arena is dropped with the parser
    result
}
