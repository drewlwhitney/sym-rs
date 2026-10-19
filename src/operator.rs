pub mod add;
pub mod divide;
pub mod exponent;
pub mod multiply;
pub mod power;
pub mod sqrt;
pub mod subtract;

use super::expr::Expr;
use super::subs::{SubsMap, Substitute};
pub use add::AddOp;
use derive_more::Display;
pub use divide::DivideOp;
pub use exponent::ExponentOp;
pub use multiply::MultiplyOp;
pub use power::PowerOp;
pub use sqrt::SqrtOp;
pub use subtract::SubtractOp;

#[derive(Clone, Display)]
pub enum Operation<'a> {
    Add(Box<AddOp<'a>>),
    Subtract(Box<SubtractOp<'a>>),
    Multiply(Box<MultiplyOp<'a>>),
    Divide(Box<DivideOp<'a>>),
    Exp(Box<ExponentOp<'a>>),
    Power(Box<PowerOp<'a>>),
    Sqrt(Box<SqrtOp<'a>>),
    Custom(Box<dyn Substitute>),
}
impl<'a> Operation<'a> {
    pub fn subs(self, map: &'a SubsMap) -> Expr<'a> {
        match self {
            Self::Add(o) => o.subs(map),
            Self::Subtract(o) => o.subs(map),
            Self::Multiply(o) => o.subs(map),
            Self::Divide(o) => o.subs(map),
            Self::Exp(o) => o.subs(map),
            Self::Power(o) => o.subs(map),
            Self::Sqrt(o) => o.subs(map),
            Self::Custom(o) => o.subs(map),
        }
    }
}

pub trait Operable<'a>: Into<Expr<'a>> {
    fn exp(self) -> Expr<'a> {
        ExponentOp::new(self).into()
    }

    fn pow(self, power: impl Into<Expr<'a>>) -> Expr<'a> {
        PowerOp::new(self, power).into()
    }

    fn sqrt(self) -> Expr<'a> {
        SqrtOp::new(self).into()
    }
}

impl<'a, T: Into<Expr<'a>>> Operable<'a> for T {}
