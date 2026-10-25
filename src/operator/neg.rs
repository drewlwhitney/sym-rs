use super::display_helpers;
use crate::expr::Expr;
use crate::helper_macros::*;
use crate::operator::Operation;
use std::fmt::{self, Display};
use std::ops::Neg;

#[derive(Clone, Debug)]
pub struct NegOp<'a>(Expr<'a>);
impl<'a> NegOp<'a> {
    /// Create a new instance that negates
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self(expr.into())
    }
}
impl Display for NegOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Expr::Number(n) => {
                if n < 0.0 {
                    write!(f, "-({n})")?;
                } else {
                    write!(f, "-{n}")?;
                }
            }
            Expr::Symbol(symbol) => write!(f, "-{symbol}")?,
            Expr::Operation(ref o) => match o {
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
