//! mathexpr.c of EPA SWMM 5.2.4, in safe Rust.
//!
//! TRANSLATION STUB: bodies are `todo!()` until mathexpr.c is translated.

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

/// Called by: controls, gwater, treatmnt.
pub fn mathexpr_create(formula: &[u8], getVar: &mut dyn FnMut(&[u8]) -> i32) -> Option<MathExpr> {
    todo!("mathexpr_create")
}

/// Called by: controls, gwater, treatmnt.
pub fn mathexpr_delete(expr: Option<MathExpr>) {
    todo!("mathexpr_delete")
}

/// Called by: controls, gwater, treatmnt.
pub fn mathexpr_eval(expr: &MathExpr, getVariableValue: &mut dyn FnMut(i32) -> f64) -> f64 {
    todo!("mathexpr_eval")
}
