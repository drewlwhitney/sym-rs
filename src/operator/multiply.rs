use super::display_helpers;
use crate::expr::Expr;
use crate::helper_macros::*;
use std::fmt::{self, Display};
use std::ops::Mul;

#[derive(Clone, Debug, PartialEq)]
pub struct MultiplyOp<'a>(pub(crate) Expr<'a>, pub(crate) Expr<'a>);
impl<'a> MultiplyOp<'a> {
    /// Create a new instance that multiplies `left` by `right`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self(left.into(), right.into())
    }
}

impl<'a> Display for MultiplyOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_helpers::write_multiply_divide_operand(f, &self.0)?;
        write!(f, " * ")?;
        display_helpers::write_multiply_divide_operand(f, &self.1)
    }
}

impl_subs_simple!(MultiplyOp, Mul::mul, self.0, self.1);
impl_from_op_expr!(MultiplyOp, Multiply);
impl_binary_operator!(MultiplyOp, Mul, mul);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::symbol::Symbol;

    #[test]
    fn operator() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");
        let c = Symbol::new("c");
        let d = Symbol::new("d");

        // symbol * symbol
        assert_eq!(a * b, MultiplyOp::new(a, b).into());
        // symbol * expr
        assert_eq!(
            a * (b * c),
            MultiplyOp::new(a, MultiplyOp::new(b, c)).into()
        );
        // symbol * number
        assert_eq!(a * 2.0, MultiplyOp::new(a, 2.0).into());
        // expr * expr
        assert_eq!(
            (a * b) * (c * d),
            MultiplyOp::new(MultiplyOp::new(a, b), MultiplyOp::new(c, d)).into()
        );
        // expr * symbol
        assert_eq!(
            (a * b) * c,
            MultiplyOp::new(MultiplyOp::new(a, b), c).into()
        );
        // expr * number
        assert_eq!(
            (a * b) * 2.0,
            MultiplyOp::new(MultiplyOp::new(a, b), 2.0).into()
        );
        // number * symbol
        assert_eq!(2.0 * a, MultiplyOp::new(2.0, a).into());
        // number * expr
        assert_eq!(
            2.0 * (a * b),
            MultiplyOp::new(2.0, MultiplyOp::new(a, b)).into()
        );
    }

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");
        let c = Symbol::new("c");
        let d = Symbol::new("d");

        let map = HashMap::from([(a, 2.0), (b, 3.0), (c, 4.0), (d, 5.0)]);

        // symbol * symbol
        assert_eq!((a * b).subs(&map).unwrap().num_or_panic(), 6.0);
        // symbol * expr
        assert_eq!((a * (b * c)).subs(&map).unwrap().num_or_panic(), 24.0);
        // symbol * number
        assert_eq!((a * 2.0).subs(&map).unwrap().num_or_panic(), 4.0);
        // expr * expr
        assert_eq!(
            (((a * b) * (c * d)).subs(&map).unwrap().num_or_panic()),
            120.0
        );
        // expr * symbol
        assert_eq!(((a * b) * c).subs(&map).unwrap().num_or_panic(), 24.0);
        // expr * number
        assert_eq!(((a * b) * 2.0).subs(&map).unwrap().num_or_panic(), 12.0);
        // number * symbol
        assert_eq!((2.0 * a).subs(&map).unwrap().num_or_panic(), 4.0);
        // number * expr
        assert_eq!((2.0 * (a * b)).subs(&map).unwrap().num_or_panic(), 12.0);
    }
}
