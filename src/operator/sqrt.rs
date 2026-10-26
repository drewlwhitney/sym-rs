use crate::error::{OperationError, OperationResult};
use crate::expr::Expr;
use crate::helper_macros::*;
use crate::subs::SubsMap;
use derive_more::Display;

#[derive(Clone, Display, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::error::OperationError;
    use crate::expr::Expr;
    use crate::operator::Operand;
    use crate::symbol::Symbol;

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");

        let map = HashMap::from([(a, 66.0), (b, -66.0)]);

        // symbol
        assert_eq!(a.sqrt().subs(&map).unwrap().num_or_panic(), f64::sqrt(66.0));
        // expr
        assert_eq!(
            a.sqrt().sqrt().subs(&map).unwrap().num_or_panic(),
            f64::sqrt(66.0).sqrt()
        );
        // number
        assert_eq!(
            (2.0).sqrt().subs(&map).unwrap().num_or_panic(),
            f64::sqrt(2.0)
        );

        // error cases (square root of negative)
        // symbol
        assert_eq!(
            b.sqrt().subs(&map).unwrap_err(),
            OperationError::SquareRootOfNegative(-66.0)
        );
        // expr
        assert_eq!(
            b.sqrt().sqrt().subs(&map).unwrap_err(),
            OperationError::SquareRootOfNegative(-66.0)
        );
        // number
        assert_eq!(
            Expr::from(-101.00).sqrt().subs(&map).unwrap_err(),
            OperationError::SquareRootOfNegative(-101.00)
        );
    }
}
