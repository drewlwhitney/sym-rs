use super::display_helpers;
use crate::expr::Expr;
use crate::helper_macros::*;
use crate::operator::Operation;
use std::fmt::{self, Display};
use std::ops::Neg;

#[derive(Clone, Debug, PartialEq)]
pub struct NegOp(Expr);
impl NegOp {
    /// Create a new instance that negates
    pub fn new(expr: impl Into<Expr>) -> Self {
        Self(expr.into())
    }
}
impl Display for NegOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Expr::Number(n) => {
                if *n < 0.0 {
                    write!(f, "-({n})")?;
                } else {
                    write!(f, "-{n}")?;
                }
            }
            Expr::Symbol(symbol) => write!(f, "-{symbol}")?,
            Expr::Operation(o) => match o {
                Operation::Neg(_) | Operation::Add(_) | Operation::Subtract(_) => {
                    write!(f, "-({o})")?
                }
                Operation::Multiply(mult) => {
                    display_helpers::write_maybe_negative_operand(f, &mult.0)?;
                    write!(f, " * {}", mult.1)?;
                }
                Operation::Divide(div) => {
                    display_helpers::write_maybe_negative_operand(f, &div.0)?;
                    write!(f, " / {}", div.1)?;
                }
                Operation::Exp(_)
                | Operation::Power(_)
                | Operation::Sqrt(_)
                | Operation::Abs(_)
                | Operation::Custom(_) => write!(f, "-{o}")?,
            },
        }

        Ok(())
    }
}

impl_subs_simple!(NegOp, Neg::neg, self.0);
impl_unary_operator!(NegOp, Neg, neg);
impl_from_op_expr!(NegOp, Neg);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Symbol;
    use std::collections::HashMap;

    #[test]
    fn operator() {
        let a = Symbol::new("a");

        // symbol
        assert_eq!(-&a, NegOp::new(&a).into());
        // expr
        assert_eq!(-(-&a), NegOp::new(NegOp::new(&a)).into());
    }

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");

        let map = HashMap::from([(a.clone(), 15.0), (b.clone(), -15.0)]);

        // symbol
        assert_eq!((-&a).subs(&map).unwrap().num_or_panic(), -15.0);
        assert_eq!((-&b).subs(&map).unwrap().num_or_panic(), 15.0);
        // expr
        assert_eq!((-(-&a)).subs(&map).unwrap().num_or_panic(), 15.0);
        assert_eq!((-(-&b)).subs(&map).unwrap().num_or_panic(), -15.0);
        // number
        assert_eq!((-Expr::from(1.0)).subs(&map).unwrap().num_or_panic(), -1.0);
        assert_eq!((-Expr::from(-1.0)).subs(&map).unwrap().num_or_panic(), 1.0);
    }
}
