use super::display_helpers;
use crate::expr::Expr;
use crate::helper_macros::*;
use std::fmt::{self, Display};
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct MultiplyOp<'a>(pub(crate) Expr<'a>, pub(crate) Expr<'a>);
impl<'a> Display for MultiplyOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_helpers::write_multiply_divide_operand(f, &self.0)?;
        write!(f, " * ")?;
        display_helpers::write_multiply_divide_operand(f, &self.1)
    }
}

impl<'a> MultiplyOp<'a> {
    /// Create a new instance that multiplies `left` by `right`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self(left.into(), right.into())
    }
}

impl_subs_simple!(MultiplyOp, Mul::mul, self.0, self.1);
impl_from_op_expr!(MultiplyOp, Multiply);
impl_binary_operator!(MultiplyOp, Mul, mul);
