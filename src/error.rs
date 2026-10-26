use crate::expr::Expr;
use thiserror::Error;

pub type OperationResult = Result<Expr>;
pub type Result<T> = std::result::Result<T, OperationError>;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum OperationError {
    #[error("Cannot divide {_0} by 0")]
    DivideByZero(Expr),

    #[error("Cannot take square root of negative number {_0}")]
    SquareRootOfNegative(f64),

    #[error("Cannot raise zero to negative power {_0}")]
    RaiseZeroToNegative(f64),

    #[error("Cannot raise negative base {base} to fractional power {power}")]
    RaiseNegativeToFraction { base: f64, power: f64 },

    #[error("Cannot take log of {_0}")]
    BadLogDomain(f64),
}
