use super::display_helpers;
use crate::error::{OperationError, OperationResult};
use crate::expr::Expr;
use crate::helper_macros::*;
use crate::subs::SubsMap;
use std::fmt::{self, Display};
use std::ops::Div;

#[derive(Clone, Debug, PartialEq)]
pub struct DivideOp(pub(crate) Expr, pub(crate) Expr);

impl DivideOp {
    /// Create a new instance that divides `left` by `right`.
    pub fn new(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
        Self(left.into(), right.into())
    }

    /// Substitute symbols and try to evaluate the expression.
    pub fn subs(&self, map: &SubsMap) -> OperationResult {
        Ok(match self.1.subs(map)? {
            Expr::Number(divisor) => {
                if divisor == 0.0 {
                    // NOTE: this `.clone()` is essentially `Arc::clone()` so it's cheap
                    return Err(OperationError::DivideByZero(self.0.clone()));
                }
                match self.0.subs(map)? {
                    Expr::Number(dividend) => (dividend / divisor).into(),
                    expr => Self::new(expr, divisor).into(),
                }
            }
            expr => Self(self.0.subs(map)?, expr).into(),
        })
    }
}

impl Display for DivideOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_helpers::write_multiply_divide_operand(f, &self.0)?;
        write!(f, " / ")?;
        display_helpers::write_multiply_divide_operand(f, &self.1)
    }
}

impl_from_op_expr!(DivideOp, Divide);
impl_binary_operator!(DivideOp, Div, div);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::symbol::Symbol;

    #[test]
    fn operator() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");
        let c = Symbol::new("c");
        let d = Symbol::new("d");

        // symbol / symbol
        assert_eq!(&a / &b, DivideOp::new(&a, &b).into());
        // symbol / expr
        assert_eq!(
            &a / (&b / &c),
            DivideOp::new(&a, DivideOp::new(&b, &c)).into()
        );
        // symbol / number
        assert_eq!(&a / 2.0, DivideOp::new(&a, 2.0).into());
        // expr / expr
        assert_eq!(
            (&a / &b) / (&c / &d),
            DivideOp::new(DivideOp::new(&a, &b), DivideOp::new(&c, &d)).into()
        );
        // expr / symbol
        assert_eq!(
            (&a / &b) / &c,
            DivideOp::new(DivideOp::new(&a, &b), &c).into()
        );
        // expr / number
        assert_eq!(
            (&a / &b) / 2.0,
            DivideOp::new(DivideOp::new(&a, &b), 2.0).into()
        );
        // number / symbol
        assert_eq!(2.0 / &a, DivideOp::new(2.0, &a).into());
        // number / expr
        assert_eq!(
            2.0 / (&a / &b),
            DivideOp::new(2.0, DivideOp::new(&a, &b)).into()
        );
    }

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");
        let c = Symbol::new("c");
        let d = Symbol::new("d");

        let map = HashMap::from([
            (a.clone(), 2.0),
            (b.clone(), 3.0),
            (c.clone(), 4.0),
            (d.clone(), 5.0),
        ]);

        // symbol / symbol
        assert_eq!((&a / &b).subs(&map).unwrap().num_or_panic(), 2.0 / 3.0);
        // symbol / expr
        assert_eq!(
            (&a / (&b / &c)).subs(&map).unwrap().num_or_panic(),
            2.0 / (3.0 / 4.0)
        );
        // symbol / number
        assert_eq!((&a / 2.0).subs(&map).unwrap().num_or_panic(), 2.0 / 2.0);
        // expr / expr
        assert_eq!(
            (((&a / &b) / (&c / &d)).subs(&map).unwrap().num_or_panic()),
            (2.0 / 3.0) / (4.0 / 5.0)
        );
        // expr / symbol
        assert_eq!(
            ((&a / &b) / &c).subs(&map).unwrap().num_or_panic(),
            (2.0 / 3.0) / 4.0
        );
        // expr / number
        assert_eq!(
            ((&a / &b) / 2.0).subs(&map).unwrap().num_or_panic(),
            (2.0 / 3.0) / 2.0
        );
        // number / symbol
        assert_eq!((2.0 / &a).subs(&map).unwrap().num_or_panic(), 2.0 / 2.0);
        // number / expr
        assert_eq!(
            (2.0 / (&a / &b)).subs(&map).unwrap().num_or_panic(),
            2.0 / (2.0 / 3.0)
        );

        // error cases (divide by zero)
        // symbol / 0.0
        assert_eq!(
            (&a / 0.0).subs(&map).unwrap_err(),
            OperationError::DivideByZero(a.clone().into())
        );
        // expr / 0.0
        assert_eq!(
            ((&a / &b) / 0.0).subs(&map).unwrap_err(),
            OperationError::DivideByZero(DivideOp::new(&a, &b).into())
        );
        // number / 0.0
        assert_eq!(
            (Expr::from(55.0) / 0.0).subs(&map).unwrap_err(),
            OperationError::DivideByZero(55.0.into())
        );
    }
}
