use crate::expr::Expr;
use crate::helper_macros::*;
use derive_more::Display;

#[derive(Clone, Display, Debug)]
#[display("exp({_0})")]
pub struct ExponentOp<'a>(Expr<'a>);
impl<'a> ExponentOp<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self(expr.into())
    }
}

impl_subs_simple!(ExponentOp, f64::exp, self.0);
impl_from_op_expr!(ExponentOp, Exp);
