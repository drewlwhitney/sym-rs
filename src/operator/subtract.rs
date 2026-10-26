use super::display_helpers;
use crate::expr::Expr;
use crate::helper_macros::*;
use std::fmt::{self, Display};
use std::ops::Sub;

#[derive(Clone, Debug, PartialEq)]
pub struct SubtractOp(pub(crate) Expr, pub(crate) Expr);

impl SubtractOp {
    /// Create a new instance that subtracts `left` from `right`.
    pub fn new(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
        Self(left.into(), right.into())
    }
}

impl Display for SubtractOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - ", self.0)?;
        display_helpers::write_maybe_negative_operand(f, &self.1)
    }
}

impl_subs_simple!(SubtractOp, Sub::sub, self.0, self.1);
impl_from_op_expr!(SubtractOp, Subtract);
impl_binary_operator!(SubtractOp, Sub, sub);

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

        // symbol - symbol
        assert_eq!(&a - &b, SubtractOp::new(&a, &b).into());
        // symbol - expr
        assert_eq!(
            &a - (&b - &c),
            SubtractOp::new(&a, SubtractOp::new(&b, &c)).into()
        );
        // symbol - number
        assert_eq!(&a - 1.0, SubtractOp::new(&a, 1.0).into());
        // expr - expr
        assert_eq!(
            (&a - &b) - (&c - &d),
            SubtractOp::new(SubtractOp::new(&a, &b), SubtractOp::new(&c, &d)).into()
        );
        // expr - symbol
        assert_eq!(
            (&a - &b) - &c,
            SubtractOp::new(SubtractOp::new(&a, &b), &c).into()
        );
        // expr - number
        assert_eq!(
            (&a - &b) - 1.0,
            SubtractOp::new(SubtractOp::new(&a, &b), 1.0).into()
        );
        // number - symbol
        assert_eq!(1.0 - &a, SubtractOp::new(1.0, &a).into());
        // number - expr
        assert_eq!(
            1.0 - (&a - &b),
            SubtractOp::new(1.0, SubtractOp::new(&a, &b)).into()
        );
    }

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");
        let c = Symbol::new("c");
        let d = Symbol::new("d");

        let map = HashMap::from([
            (a.clone(), 1.0),
            (b.clone(), 2.0),
            (c.clone(), 3.0),
            (d.clone(), 4.0),
        ]);

        // symbol - symbol
        assert_eq!((&a - &b).subs(&map).unwrap().num_or_panic(), -1.0);
        // symbol - expr
        assert_eq!((&a - (&b - &c)).subs(&map).unwrap().num_or_panic(), 2.0);
        // symbol - number
        assert_eq!((&a - 1.0).subs(&map).unwrap().num_or_panic(), 0.0);
        // expr - expr
        assert_eq!(
            (((&a - &b) - (&c - &d)).subs(&map).unwrap().num_or_panic()),
            0.0
        );
        // expr - symbol
        assert_eq!(((&a - &b) - &c).subs(&map).unwrap().num_or_panic(), -4.0);
        // expr - number
        assert_eq!(((&a - &b) - 1.0).subs(&map).unwrap().num_or_panic(), -2.0);
        // number - symbol
        assert_eq!((1.0 - &a).subs(&map).unwrap().num_or_panic(), 0.0);
        // number - expr
        assert_eq!((1.0 - (&a - &b)).subs(&map).unwrap().num_or_panic(), 2.0);
    }
}
