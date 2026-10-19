macro_rules! impl_from_op_expr {
    ($op:ident, $variant:ident) => {
        impl<'a> From<$op<'a>> for crate::operator::Operation<'a> {
            fn from(value: $op<'a>) -> Self {
                Self::$variant(Box::new(value))
            }
        }

        impl<'a> From<$op<'a>> for Expr<'a> {
            fn from(value: $op<'a>) -> Self {
                Self::Operation(value.into())
            }
        }
    };
}
pub(crate) use impl_from_op_expr;

macro_rules! impl_subs_simple {
    ($op:ident, $symbol:tt) => {
        impl<'a> $op<'a> {
            pub fn subs(self, map: &'a crate::subs::SubsMap) -> Expr<'a> {
                match (self.0.subs(map), self.1.subs(map)) {
                    (Expr::Number(left), Expr::Number(right)) => (left $symbol right).into(),
                    (left, right) => Self(left, right).into(),
                }
            }
        }
    };
}
pub(crate) use impl_subs_simple;

macro_rules! impl_operator {
    ($op:ident, $op_trait:ident, $trait_func_name:tt) => {
        impl<'a, R: Into<Expr<'a>>> $op_trait<R> for Expr<'a> {
            type Output = Expr<'a>;

            fn $trait_func_name(self, rhs: R) -> Self::Output {
                $op(self, rhs.into()).into()
            }
        }
        impl<'a, R: Into<Expr<'a>>> $op_trait<R> for crate::symbol::Symbol<'a> {
            type Output = Expr<'a>;

            fn $trait_func_name(self, rhs: R) -> Self::Output {
                $op(self.into(), rhs.into()).into()
            }
        }
        impl<'a> $op_trait<crate::symbol::Symbol<'a>> for f64 {
            type Output = Expr<'a>;

            fn $trait_func_name(self, rhs: crate::symbol::Symbol<'a>) -> Self::Output {
                $op(self.into(), rhs.into()).into()
            }
        }
        impl<'a> $op_trait<Expr<'a>> for f64 {
            type Output = Expr<'a>;

            fn $trait_func_name(self, rhs: Expr<'a>) -> Self::Output {
                $op(self.into(), rhs).into()
            }
        }
    };
}

pub(crate) use impl_operator;
