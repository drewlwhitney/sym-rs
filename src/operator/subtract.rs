use super::display_helpers;
use crate::expr::Expr;
use crate::helper_macros::*;
use std::fmt::{self, Display};
use std::ops::Sub;

#[derive(Clone, Debug)]
pub struct SubtractOp<'a>(pub(crate) Expr<'a>, pub(crate) Expr<'a>);

impl<'a> SubtractOp<'a> {
    /// Create a new instance that subtracts `left` from `right`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self(left.into(), right.into())
    }
}

impl Display for SubtractOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - ", self.0)?;
        display_helpers::write_maybe_negative_operand(f, &self.1)
    }
}

impl_subs_simple!(SubtractOp, Sub::sub, self.0, self.1);
impl_from_op_expr!(SubtractOp, Subtract);
impl_binary_operator!(SubtractOp, Sub, sub);
