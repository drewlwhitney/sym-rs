use super::display_helpers;
use crate::expr::Expr;
use crate::helper_macros::*;
use std::fmt::{self, Display};
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct AddOp<'a>(pub(crate) Expr<'a>, pub(crate) Expr<'a>);

impl<'a> AddOp<'a> {
    /// Create a new instance that adds `left` to `right`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self(left.into(), right.into())
    }
}

impl Display for AddOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + ", self.0)?;
        display_helpers::write_maybe_negative_operand(f, &self.1)
    }
}

impl_subs_simple!(AddOp, Add::add, self.0, self.1);
impl_from_op_expr!(AddOp, Add);
impl_binary_operator!(AddOp, Add, add);
