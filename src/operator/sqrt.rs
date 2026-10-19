use super::super::expr::Expr;
use super::super::subs::SubsMap;
use crate::helper_macros::*;
use derive_more::Display;

#[derive(Clone, Display)]
#[display("sqrt({_0})")]
pub struct SqrtOp<'a>(Expr<'a>);
impl<'a> SqrtOp<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self(expr.into())
    }

    pub fn subs(self, map: &'a SubsMap) -> Expr<'a> {
        match self.0.subs(map) {
            Expr::Number(n) => n.sqrt().into(),
            expr => Self(expr).into(),
        }
    }
}

impl_from_op_expr!(SqrtOp, Sqrt);
