/// Implement [`From<T>`] for [`crate::operator::Operation`] and [`crate::expr::Expr`].
macro_rules! impl_from_op_expr {
    ($op:ident, $variant:ident) => {
        impl From<$op> for crate::operator::Operation {
            fn from(value: $op) -> Self {
                Self::$variant(std::sync::Arc::new(value))
            }
        }

        impl From<$op> for crate::expr::Expr {
            fn from(value: $op) -> Self {
                Self::Operation(value.into())
            }
        }
    };
}
pub(crate) use impl_from_op_expr;

/// Implement a `.subs()` method that evaluates the contained expression(s) and applies either an
/// operator (for two operands) or a function (for one operand).
macro_rules! impl_subs_simple {
    // binary operator
    ($op:ident, $op_func:path, self.$left_operand:tt, self.$right_operand:tt) => {
        impl $op {
            /// Substitute symbols and try to evaluate the expression.
            pub fn subs(&self, map: &$crate::subs::SubsMap) -> $crate::error::OperationResult {
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
    // unary operator
    ($op:ident, $op_func:path, self.$operand:tt) => {
        impl $op {
            /// Substitute symbols and try to evaluate the expression.
            pub fn subs(&self, map: &$crate::subs::SubsMap) -> $crate::error::OperationResult {
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
        // Expr
        impl $op_trait for $crate::expr::Expr {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self) -> $crate::expr::Expr {
                $op::new(self).into()
            }
        }
        // &Expr
        impl $op_trait for &$crate::expr::Expr {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self) -> $crate::expr::Expr {
                $op::new(self).into()
            }
        }
        // Symbol
        impl $op_trait for $crate::symbol::Symbol {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self) -> $crate::expr::Expr {
                $op::new(self).into()
            }
        }
        // &Symbol
        impl $op_trait for &$crate::symbol::Symbol {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self) -> $crate::expr::Expr {
                $op::new(self).into()
            }
        }
    };
}
pub(crate) use impl_unary_operator;

/// Implement a binary operator from [`std::ops`] for an operator type.
macro_rules! impl_binary_operator {
    ($op:ident, $op_trait:ident, $trait_func_name:tt) => {
        // Into<Expr> + Expr
        impl<'a, R: Into<$crate::expr::Expr>> $op_trait<R> for $crate::expr::Expr {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self, rhs: R) -> $crate::expr::Expr {
                $op::new(self, rhs).into()
            }
        }
        // Into<Expr> + &Expr
        impl<'a, R: Into<$crate::expr::Expr>> $op_trait<R> for &$crate::expr::Expr {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self, rhs: R) -> $crate::expr::Expr {
                $op::new(self, rhs).into()
            }
        }
        // Into<Expr> + Symbol
        impl<'a, R: Into<$crate::expr::Expr>> $op_trait<R> for $crate::symbol::Symbol {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self, rhs: R) -> $crate::expr::Expr {
                $op::new(self, rhs).into()
            }
        }
        // Into<Expr> + &Symbol
        impl<'a, R: Into<$crate::expr::Expr>> $op_trait<R> for &$crate::symbol::Symbol {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self, rhs: R) -> $crate::expr::Expr {
                $op::new(self, rhs).into()
            }
        }
        // Symbol + f64
        impl $op_trait<$crate::symbol::Symbol> for f64 {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self, rhs: $crate::symbol::Symbol) -> $crate::expr::Expr {
                $op::new(self, rhs).into()
            }
        }
        // &Symbol + f64
        impl $op_trait<&$crate::symbol::Symbol> for f64 {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self, rhs: &$crate::symbol::Symbol) -> $crate::expr::Expr {
                $op::new(self, rhs).into()
            }
        }
        // Expr + f64
        impl $op_trait<$crate::expr::Expr> for f64 {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self, rhs: $crate::expr::Expr) -> $crate::expr::Expr {
                $op::new(self, rhs).into()
            }
        }
        // &Expr + f64
        impl $op_trait<&$crate::expr::Expr> for f64 {
            type Output = $crate::expr::Expr;

            fn $trait_func_name(self, rhs: &$crate::expr::Expr) -> $crate::expr::Expr {
                $op::new(self, rhs).into()
            }
        }
    };
}

pub(crate) use impl_binary_operator;
