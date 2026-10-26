use crate::expr::Expr;
use crate::helper_macros::*;
use derive_more::Display;

#[derive(Clone, Display, Debug, PartialEq)]
#[display("|{_0}|")]
pub struct AbsOp(Expr);
impl AbsOp {
    pub fn new(expr: impl Into<Expr>) -> Self {
        Self(expr.into())
    }
}

impl_subs_simple!(AbsOp, f64::abs, self.0);
impl_from_op_expr!(AbsOp, Abs);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::operator::Operand;
    use crate::symbol::Symbol;

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");

        let map = HashMap::from([(a.clone(), -32.0), (b.clone(), 32.0)]);

        // symbol
        assert_eq!(a.abs().subs(&map).unwrap().num_or_panic(), 32.0);
        assert_eq!(b.abs().subs(&map).unwrap().num_or_panic(), 32.0);
        // expr
        assert_eq!(a.abs().abs().subs(&map).unwrap().num_or_panic(), 32.0);
        assert_eq!(b.abs().abs().subs(&map).unwrap().num_or_panic(), 32.0);
        // number
        assert_eq!((-1.0).abs().subs(&map).unwrap().num_or_panic(), 1.0);
        assert_eq!((1.0).abs().subs(&map).unwrap().num_or_panic(), 1.0);
    }
}
