use super::super::expr::Expr;
use super::Operation;
use crate::helper_macros::*;
use std::fmt::{self, Display};
use std::ops::Mul;

#[derive(Clone)]
pub struct MultiplyOp<'a>(Expr<'a>, Expr<'a>);
impl<'a> Display for MultiplyOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(
            &self.0,
            Expr::Operation(o) if matches!(*o, Operation::Add(_) | Operation::Subtract(_))
        ) {
            write!(f, "({})", self.0)?;
        } else {
            write!(f, "{}", self.0)?;
        }

        write!(f, " * ")?;

        if matches!(
            &self.1,
            Expr::Operation(o) if matches!(*o, Operation::Add(_) | Operation::Subtract(_))
        ) {
            write!(f, "({})", self.1)?;
        } else {
            write!(f, "{}", self.1)?;
        }

        Ok(())
    }
}

impl_subs_simple!(MultiplyOp, *);
impl_from_op_expr!(MultiplyOp, Multiply);
impl_operator!(MultiplyOp, Mul, mul);
