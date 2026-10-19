use super::super::expr::Expr;
use super::super::subs::SubsMap;
use super::Operation;
use crate::helper_macros::*;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct PowerOp<'a> {
    base: Expr<'a>,
    power: Expr<'a>,
}
impl<'a> PowerOp<'a> {
    pub fn new(base: impl Into<Expr<'a>>, power: impl Into<Expr<'a>>) -> Self {
        Self {
            base: base.into(),
            power: power.into(),
        }
    }

    pub fn subs(self, map: &'a SubsMap) -> Expr<'a> {
        match (self.base.subs(map), self.power.subs(map)) {
            (Expr::Number(b), Expr::Number(p)) => b.powf(p).into(),
            (base, power) => Self { base, power }.into(),
        }
    }
}
impl<'a> Display for PowerOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(
            &self.base,
            Expr::Operation(o) if !matches!(o, Operation::Exp(_) | Operation::Custom(_))
        ) {
            write!(f, "({})", self.base)?;
        } else {
            write!(f, "{}", self.base)?;
        }

        write!(f, "^")?;

        if matches!(
            &self.power,
            Expr::Operation(o) if !matches!(o, Operation::Exp(_) | Operation::Custom(_))
        ) {
            write!(f, "({})", self.power)?;
        } else {
            write!(f, "{}", self.power)?;
        }

        Ok(())
    }
}

impl_from_op_expr!(PowerOp, Power);
