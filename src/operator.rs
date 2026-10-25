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

#[derive(Clone, Display, Debug)]
pub enum Operation<'a> {
    Add(Box<AddOp<'a>>),
    Subtract(Box<SubtractOp<'a>>),
    Multiply(Box<MultiplyOp<'a>>),
    Divide(Box<DivideOp<'a>>),
    Exp(Box<ExponentOp<'a>>),
    Power(Box<PowerOp<'a>>),
    Sqrt(Box<SqrtOp<'a>>),
    Abs(Box<AbsOp<'a>>),
    Neg(Box<NegOp<'a>>),
    Custom(Box<dyn Substitute<'a> + 'a>),
}
impl<'a> Operation<'a> {
    pub fn subs(self, map: &SubsMap) -> OperationResult<'a> {
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
    impl Sealed for Expr<'_> {}
    impl Sealed for Symbol<'_> {}
}

/// An item that can act as an operand to mathematical operations.
/// This trait cannot be implemented outside `symrs`.
pub trait Operand<'a>:
    sealed::Sealed
    + Into<Expr<'a>>
    + Add<Expr<'a>>
    + Add<Symbol<'a>>
    + Add<f64>
    + Sub<Expr<'a>>
    + Sub<Symbol<'a>>
    + Sub<f64>
    + Mul<Expr<'a>>
    + Mul<Symbol<'a>>
    + Mul<f64>
    + Div<Expr<'a>>
    + Div<Symbol<'a>>
    + Div<f64>
    + Neg
{
    /// Return `exp(self)`.
    fn exp(self) -> Expr<'a> {
        ExponentOp::new(self).into()
    }

    /// Return `self^power`.
    fn pow(self, power: impl Into<Expr<'a>>) -> Expr<'a> {
        PowerOp::new(self, power).into()
    }

    /// Return `sqrt(self)`.
    fn sqrt(self) -> Expr<'a> {
        SqrtOp::new(self).into()
    }

    /// Return `|self|`.
    fn abs(self) -> Expr<'a> {
        AbsOp::new(self).into()
    }
}

impl<
    'a,
    T: sealed::Sealed
        + Into<Expr<'a>>
        + Add<Expr<'a>>
        + Add<Symbol<'a>>
        + Add<f64>
        + Sub<Expr<'a>>
        + Sub<Symbol<'a>>
        + Sub<f64>
        + Mul<Expr<'a>>
        + Mul<Symbol<'a>>
        + Mul<f64>
        + Div<Expr<'a>>
        + Div<Symbol<'a>>
        + Div<f64>
        + Neg,
> Operand<'a> for T
{
}
