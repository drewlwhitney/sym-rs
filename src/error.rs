use crate::expr::Expr;
use thiserror::Error;

pub type OperationResult<'a> = Result<'a, Expr<'a>>;
pub type Result<'a, T> = std::result::Result<T, OperationError<'a>>;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum OperationError<'a> {
    #[error("Cannot divide {_0} by 0")]
    DivideByZero(Expr<'a>),

    #[error("Cannot take square root of negative number {_0}")]
    SquareRootOfNegative(f64),

    #[error("Cannot raise zero to negative power {_0}")]
    RaiseZeroToNegative(f64),

    #[error("Cannot raise negative base {base} to fractional power {power}")]
    RaiseNegativeToFraction { base: f64, power: f64 },

    #[error("Cannot take log of {_0}")]
    BadLogDomain(f64),
}
