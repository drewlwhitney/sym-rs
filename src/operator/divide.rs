use super::display_helpers;
use crate::error::{OperationError, OperationResult};
use crate::expr::Expr;
use crate::helper_macros::*;
use crate::subs::SubsMap;
use std::fmt::{self, Display};
use std::ops::Div;

#[derive(Clone, Debug)]
pub struct DivideOp<'a>(pub(crate) Expr<'a>, pub(crate) Expr<'a>);
impl<'a> DivideOp<'a> {
    /// Create a new instance that divides `left` by `right`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self(left.into(), right.into())
    }

    /// Substitute symbols and try to evaluate the expression.
    pub fn subs(self, map: &SubsMap) -> OperationResult<'a> {
        Ok(match (self.0.subs(map)?, self.1.subs(map)?) {
            // can't divide by zero
            (expr, Expr::Number(right)) if right == 0.0 => {
                return Err(OperationError::DivideByZero(expr));
            }
            (Expr::Number(left), Expr::Number(right)) => (left / right).into(),
            (left, right) => Self(left, right).into(),
        })
    }
}
impl<'a> Display for DivideOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_helpers::write_multiply_divide_operand(f, &self.0)?;
        write!(f, " / ")?;
        display_helpers::write_multiply_divide_operand(f, &self.1)
    }
}

impl_from_op_expr!(DivideOp, Divide);
impl_binary_operator!(DivideOp, Div, div);
