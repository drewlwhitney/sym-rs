pub mod abs;
pub mod add;
mod display_helpers;
pub mod divide;
pub mod exponent;
pub mod multiply;
pub mod neg;
pub mod power;
pub mod sqrt;
pub mod subtract;

use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::Arc;

use super::expr::Expr;
use super::subs::{SubsMap, Substitute};
use crate::Symbol;
use crate::error::OperationResult;
pub use abs::AbsOp;
pub use add::AddOp;
use derive_more::Display;
pub use divide::DivideOp;
pub use exponent::ExponentOp;
pub use multiply::MultiplyOp;
pub use neg::NegOp;
pub use power::PowerOp;
pub use sqrt::SqrtOp;
pub use subtract::SubtractOp;

#[derive(Clone, Display, Debug, PartialEq)]
pub enum Operation {
    Add(Arc<AddOp>),
    Subtract(Arc<SubtractOp>),
    Multiply(Arc<MultiplyOp>),
    Divide(Arc<DivideOp>),
    Exp(Arc<ExponentOp>),
    Power(Arc<PowerOp>),
    Sqrt(Arc<SqrtOp>),
    Abs(Arc<AbsOp>),
    Neg(Arc<NegOp>),
    Custom(Arc<dyn Substitute>),
}

impl Operation {
    pub fn subs(&self, map: &SubsMap) -> OperationResult {
        match self {
            Self::Add(o) => o.subs(map),
            Self::Subtract(o) => o.subs(map),
            Self::Multiply(o) => o.subs(map),
            Self::Divide(o) => o.subs(map),
            Self::Exp(o) => o.subs(map),
            Self::Power(o) => o.subs(map),
            Self::Sqrt(o) => o.subs(map),
            Self::Abs(o) => o.subs(map),
            Self::Neg(o) => o.subs(map),
            Self::Custom(o) => o.subs(map),
        }
    }
}

mod sealed {
    use super::Expr;
    use super::Symbol;

    pub trait Sealed {}
    impl Sealed for Expr {}
    impl Sealed for Symbol {}
    impl Sealed for f64 {}
}

/// An item that can act as an operand to mathematical operations.
/// This trait cannot be implemented outside `symrs`.
pub trait Operand:
    sealed::Sealed
    + Add<Expr>
    + Add<Symbol>
    + Add<f64>
    + Sub<Expr>
    + Sub<Symbol>
    + Sub<f64>
    + Mul<Expr>
    + Mul<Symbol>
    + Mul<f64>
    + Div<Expr>
    + Div<Symbol>
    + Div<f64>
    + Neg
where
    for<'a> &'a Self: Into<Expr>,
{
    /// Return `exp(self)`.
    fn exp(&self) -> Expr {
        ExponentOp::new(self).into()
    }

    /// Return `self^power`.
    fn pow(&self, power: impl Into<Expr>) -> Expr {
        PowerOp::new(self, power).into()
    }

    /// Return `sqrt(self)`.
    fn sqrt(&self) -> Expr {
        SqrtOp::new(self).into()
    }

    /// Return `|self|`.
    fn abs(&self) -> Expr {
        AbsOp::new(self).into()
    }
}

// this gets implemented for `Symbol` and `Expr`.
impl<
    T: sealed::Sealed
        + Add<Expr>
        + Add<Symbol>
        + Add<f64>
        + Sub<Expr>
        + Sub<Symbol>
        + Sub<f64>
        + Mul<Expr>
        + Mul<Symbol>
        + Mul<f64>
        + Div<Expr>
        + Div<Symbol>
        + Div<f64>
        + Neg,
> Operand for T
where
    for<'a> &'a T: Into<Expr>,
{
}
