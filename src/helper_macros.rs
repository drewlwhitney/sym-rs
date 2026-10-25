/// Implement [`From<T>`] for [`crate::operator::Operation`] and [`crate::expr::Expr`].
macro_rules! impl_from_op_expr {
    ($op:ident, $variant:ident) => {
        impl<'a> From<$op<'a>> for crate::operator::Operation<'a> {
            fn from(value: $op<'a>) -> Self {
                Self::$variant(Box::new(value))
            }
        }

        impl<'a> From<$op<'a>> for crate::expr::Expr<'a> {
            fn from(value: $op<'a>) -> Self {
                Self::Operation(value.into())
            }
        }
    };
}
pub(crate) use impl_from_op_expr;

/// Implement a `.subs()` method that evaluates the contained expression(s) and applies either an
/// operator (for two operands) or a function (for one operand).
macro_rules! impl_subs_simple {
    ($op:ident, $op_func:path, self.$left_operand:tt, self.$right_operand:tt) => {
        impl<'a> $op<'a> {
            /// Substitute symbols and try to evaluate the expression.
            pub fn subs(self, map: &$crate::subs::SubsMap) -> $crate::error::OperationResult<'a> {
                Ok(
                    match (
                        self.$left_operand.subs(map)?,
                        self.$right_operand.subs(map)?,
                    ) {
                        (Expr::Number(left), Expr::Number(right)) => $op_func(left, right).into(),
                        (left, right) => Self::new(left, right).into(),
                    },
                )
            }
        }
    };
    ($op:ident, $op_func:path, self.$operand:tt) => {
        impl<'a> $op<'a> {
            /// Substitute symbols and try to evaluate the expression.
            pub fn subs(self, map: &$crate::subs::SubsMap) -> $crate::error::OperationResult<'a> {
                Ok(match self.$operand.subs(map)? {
                    Expr::Number(n) => $op_func(n).into(),
                    expr => Self::new(expr).into(),
                })
            }
        }
    };
}
pub(crate) use impl_subs_simple;

/// Implement a unary operator from [`std::ops`] for an operator type.
macro_rules! impl_unary_operator {
    ($op:ident, $op_trait:ident, $trait_func_name:tt) => {
        impl<'a> $op_trait for $crate::expr::Expr<'a> {
            type Output = $crate::expr::Expr<'a>;

            fn $trait_func_name(self) -> $crate::expr::Expr<'a> {
                $op::new(self).into()
            }
        }
        impl<'a> $op_trait for $crate::symbol::Symbol<'a> {
            type Output = $crate::expr::Expr<'a>;

            fn $trait_func_name(self) -> $crate::expr::Expr<'a> {
                $op::new(self).into()
            }
        }
    };
}
pub(crate) use impl_unary_operator;

/// Implement a binary operator from [`std::ops`] for an operator type.
macro_rules! impl_binary_operator {
    ($op:ident, $op_trait:ident, $trait_func_name:tt) => {
        impl<'a, R: Into<$crate::expr::Expr<'a>>> $op_trait<R> for $crate::expr::Expr<'a> {
            type Output = $crate::expr::Expr<'a>;

            fn $trait_func_name(self, rhs: R) -> $crate::expr::Expr<'a> {
                $op::new(self, rhs).into()
            }
        }
        impl<'a, R: Into<$crate::expr::Expr<'a>>> $op_trait<R> for $crate::symbol::Symbol<'a> {
            type Output = $crate::expr::Expr<'a>;

            fn $trait_func_name(self, rhs: R) -> $crate::expr::Expr<'a> {
                $op::new(self, rhs).into()
            }
        }
        impl<'a> $op_trait<$crate::symbol::Symbol<'a>> for f64 {
            type Output = $crate::expr::Expr<'a>;

            fn $trait_func_name(self, rhs: $crate::symbol::Symbol<'a>) -> $crate::expr::Expr<'a> {
                $op::new(self, rhs).into()
            }
        }
        impl<'a> $op_trait<$crate::expr::Expr<'a>> for f64 {
            type Output = $crate::expr::Expr<'a>;

            fn $trait_func_name(self, rhs: $crate::expr::Expr<'a>) -> $crate::expr::Expr<'a> {
                $op::new(self, rhs).into()
            }
        }
    };
}

pub(crate) use impl_binary_operator;
