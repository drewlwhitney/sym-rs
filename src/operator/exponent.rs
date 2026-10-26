use crate::expr::Expr;
use crate::helper_macros::*;
use derive_more::Display;

#[derive(Clone, Display, Debug, PartialEq)]
#[display("exp({_0})")]
pub struct ExponentOp<'a>(Expr<'a>);
impl<'a> ExponentOp<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self(expr.into())
    }
}

impl_subs_simple!(ExponentOp, f64::exp, self.0);
impl_from_op_expr!(ExponentOp, Exp);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::operator::Operand;
    use crate::symbol::Symbol;

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");

        let map = HashMap::from([(a, 10.0), (b, -10.0)]);

        // symbol
        assert_eq!(a.exp().subs(&map).unwrap().num_or_panic(), f64::exp(10.0));
        assert_eq!(b.exp().subs(&map).unwrap().num_or_panic(), f64::exp(-10.0));
        // expr
        assert_eq!(
            a.exp().exp().subs(&map).unwrap().num_or_panic(),
            f64::exp(10.0).exp()
        );
        assert_eq!(
            b.exp().exp().subs(&map).unwrap().num_or_panic(),
            f64::exp(-10.0).exp()
        );
        // number
        assert_eq!(
            (2.0).exp().subs(&map).unwrap().num_or_panic(),
            f64::exp(2.0)
        );
        assert_eq!(
            (-2.0).exp().subs(&map).unwrap().num_or_panic(),
            f64::exp(-2.0)
        );
    }
}
