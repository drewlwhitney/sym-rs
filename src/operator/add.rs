use super::display_helpers;
use crate::expr::Expr;
use crate::helper_macros::*;
use std::fmt::{self, Display};
use std::ops::Add;

#[derive(Clone, Debug, PartialEq)]
pub struct AddOp<'a>(pub(crate) Expr<'a>, pub(crate) Expr<'a>);

impl<'a> AddOp<'a> {
    /// Create a new instance that adds `left` to `right`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self(left.into(), right.into())
    }
}

impl Display for AddOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + ", self.0)?;
        display_helpers::write_maybe_negative_operand(f, &self.1)
    }
}

impl_subs_simple!(AddOp, Add::add, self.0, self.1);
impl_from_op_expr!(AddOp, Add);
impl_binary_operator!(AddOp, Add, add);

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

        // symbol + symbol
        assert_eq!(a + b, AddOp::new(a, b).into());
        // symbol + expr
        assert_eq!(a + (b + c), AddOp::new(a, AddOp::new(b, c)).into());
        // symbol + number
        assert_eq!(a + 1.0, AddOp::new(a, 1.0).into());
        // expr + expr
        assert_eq!(
            (a + b) + (c + d),
            AddOp::new(AddOp::new(a, b), AddOp::new(c, d)).into()
        );
        // expr + symbol
        assert_eq!((a + b) + c, AddOp::new(AddOp::new(a, b), c).into());
        // expr + number
        assert_eq!((a + b) + 1.0, AddOp::new(AddOp::new(a, b), 1.0).into());
        // number + symbol
        assert_eq!(1.0 + a, AddOp::new(1.0, a).into());
        // number + expr
        assert_eq!(1.0 + (a + b), AddOp::new(1.0, AddOp::new(a, b)).into());
    }

    #[test]
    fn subs() {
        let a = Symbol::new("a");
        let b = Symbol::new("b");
        let c = Symbol::new("c");
        let d = Symbol::new("d");

        let map = HashMap::from([(a, 1.0), (b, 2.0), (c, 3.0), (d, 4.0)]);

        // symbol + symbol
        assert_eq!((a + b).subs(&map).unwrap().num_or_panic(), 3.0);
        // symbol + expr
        assert_eq!((a + (b + c)).subs(&map).unwrap().num_or_panic(), 6.0);
        // symbol + number
        assert_eq!((a + 1.0).subs(&map).unwrap().num_or_panic(), 2.0);
        // expr + expr
        assert_eq!(
            (((a + b) + (c + d)).subs(&map).unwrap().num_or_panic()),
            10.0
        );
        // expr + symbol
        assert_eq!(((a + b) + c).subs(&map).unwrap().num_or_panic(), 6.0);
        // expr + number
        assert_eq!(((a + b) + 1.0).subs(&map).unwrap().num_or_panic(), 4.0);
        // number + symbol
        assert_eq!((1.0 + a).subs(&map).unwrap().num_or_panic(), 2.0);
        // number + expr
        assert_eq!((1.0 + (a + b)).subs(&map).unwrap().num_or_panic(), 4.0);
    }
}
