use super::display_helpers;
use crate::error::{OperationError, OperationResult};
use crate::helper_macros::*;
use crate::{expr::Expr, subs::SubsMap};
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{error::OperationError, operator::Operand, symbol::Symbol};

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");
        let c = Symbol::new("c");
        let d = Symbol::new("d");

        let map = HashMap::from([(a, 2.0), (b, 3.0), (c, 4.0), (d, 5.0)]);

        // symbol + symbol
        assert_eq!(
            a.pow(b).subs(&map).unwrap().num_or_panic(),
            2.0_f64.powf(3.0)
        );
        // symbol + expr
        assert_eq!(
            a.pow(b.pow(c)).subs(&map).unwrap().num_or_panic(),
            2.0_f64.powf(3.0_f64.powf(4.0))
        );
        // symbol + number
        assert_eq!(
            a.pow(2.0).subs(&map).unwrap().num_or_panic(),
            2.0_f64.powf(2.0)
        );
        // expr + expr
        assert_eq!(
            a.pow(b).pow(c.pow(d)).subs(&map).unwrap().num_or_panic(),
            2.0_f64.powf(3.0).powf(4.0_f64.powf(5.0))
        );
        // expr + symbol
        assert_eq!(
            a.pow(b).pow(c).subs(&map).unwrap().num_or_panic(),
            2.0_f64.powf(3.0).powf(4.0)
        );
        // expr + number
        assert_eq!(
            a.pow(b).pow(2.0).subs(&map).unwrap().num_or_panic(),
            2.0_f64.powf(3.0).powf(2.0)
        );
        // number + symbol
        assert_eq!(
            2.0.pow(a).subs(&map).unwrap().num_or_panic(),
            2.0_f64.powf(2.0)
        );
        // number + expr
        assert_eq!(
            2.0.pow(a.pow(b)).subs(&map).unwrap().num_or_panic(),
            2.0_f64.powf(2.0_f64.powf(3.0))
        );

        let error_map = HashMap::from([(a, -1.0), (b, 0.25), (c, -0.25)]);

        // error cases
        // zero to negative power
        assert_eq!(
            0.0.pow(a).subs(&error_map).unwrap_err(),
            OperationError::RaiseZeroToNegative(-1.0)
        );
        // negative number to fractional power
        assert_eq!(
            a.pow(b).subs(&error_map).unwrap_err(),
            OperationError::RaiseNegativeToFraction {
                base: -1.0,
                power: 0.25
            }
        );
        assert_eq!(
            a.pow(c).subs(&error_map).unwrap_err(),
            OperationError::RaiseNegativeToFraction {
                base: -1.0,
                power: -0.25
            }
        );
    }
}
