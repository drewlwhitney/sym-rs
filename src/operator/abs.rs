use crate::expr::Expr;
use crate::helper_macros::*;
use derive_more::Display;

#[derive(Clone, Display, Debug)]
#[display("|{_0}|")]
pub struct AbsOp<'a>(Expr<'a>);
impl<'a> AbsOp<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self(expr.into())
    }
}

impl_subs_simple!(AbsOp, f64::abs, self.0);
impl_from_op_expr!(AbsOp, Abs);
