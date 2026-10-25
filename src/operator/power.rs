use super::display_helpers;
use crate::error::{OperationError, OperationResult};
use crate::helper_macros::*;
use crate::{expr::Expr, subs::SubsMap};
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct PowerOp<'a> {
    base: Expr<'a>,
    power: Expr<'a>,
}
impl<'a> PowerOp<'a> {
    /// Create a new instance that raises `base` to `power`.
    pub fn new(base: impl Into<Expr<'a>>, power: impl Into<Expr<'a>>) -> Self {
        Self {
            base: base.into(),
            power: power.into(),
        }
    }

    /// Substitute symbols and try to evaluate the expression.
    pub fn subs(self, map: &SubsMap) -> OperationResult<'a> {
        Ok(match (self.base.subs(map)?, self.power.subs(map)?) {
            (Expr::Number(base), Expr::Number(power)) => {
                if base == 0.0 && power < 0.0 {
                    // can't raise 0 to negative power (same as divide by zero)
                    return Err(OperationError::RaiseZeroToNegative(power));
                } else if base < 0.0 && power.abs() < 1.0 {
                    // can't raise negative number to fractional exponent (same as square root
                    // of negative)
                    return Err(OperationError::RaiseNegativeToFraction { base, power });
                }
                base.powf(power).into()
            }
            (base, power) => Self { base, power }.into(),
        })
    }
}
impl<'a> Display for PowerOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_helpers::write_power_operand(f, &self.base)?;
        write!(f, "^")?;
        display_helpers::write_power_operand(f, &self.power)
    }
}

impl_from_op_expr!(PowerOp, Power);
