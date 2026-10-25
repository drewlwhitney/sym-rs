use crate::error::{OperationError, OperationResult};
use crate::expr::Expr;
use crate::helper_macros::*;
use crate::subs::SubsMap;
use derive_more::Display;

#[derive(Clone, Display, Debug)]
#[display("sqrt({_0})")]
pub struct SqrtOp<'a>(Expr<'a>);
impl<'a> SqrtOp<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self(expr.into())
    }

    pub fn subs(self, map: &SubsMap) -> OperationResult<'a> {
        Ok(match self.0.subs(map)? {
            Expr::Number(n) => {
                if n < 0.0 {
                    return Err(OperationError::SquareRootOfNegative(n));
                }
                n.sqrt().into()
            }
            expr => Self(expr).into(),
        })
    }
}

impl_from_op_expr!(SqrtOp, Sqrt);
