use super::super::expr::Expr;
use super::Operation;
use crate::helper_macros::*;
use std::fmt::{self, Display};
use std::ops::Div;

#[derive(Clone)]
pub struct DivideOp<'a>(Expr<'a>, Expr<'a>);
impl<'a> Display for DivideOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(
            &self.0,
            Expr::Operation(o) if matches!(*o, Operation::Add(_) | Operation::Subtract(_))
        ) {
            write!(f, "({})", self.0)?;
        } else {
            write!(f, "{}", self.0)?;
        }

        write!(f, " / ")?;

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

impl_subs_simple!(DivideOp, /);
impl_from_op_expr!(DivideOp, Divide);
impl_operator!(DivideOp, Div, div);
