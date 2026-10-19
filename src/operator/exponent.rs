use super::super::expr::Expr;
use super::super::subs::SubsMap;
use crate::helper_macros::*;
use derive_more::Display;

#[derive(Clone, Display)]
#[display("exp({_0})")]
pub struct ExponentOp<'a>(Expr<'a>);
impl<'a> ExponentOp<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self(expr.into())
    }

    pub fn subs(self, map: &'a SubsMap) -> Expr<'a> {
        match self.0.subs(map) {
            Expr::Number(n) => n.exp().into(),
            expr => Self(expr).into(),
        }
    }
}

impl_from_op_expr!(ExponentOp, Exp);
