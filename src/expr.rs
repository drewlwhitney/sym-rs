use std::sync::Arc;

use super::error::OperationResult;
use super::operator::Operation;
use super::subs::SubsMap;
use super::subs::Substitute;
use super::symbol::Symbol;
use derive_more::Display;

/// Basic unit for any mathematical expression.
#[derive(Clone, Display, Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Symbol(Symbol),
    Operation(Operation),
}

impl Expr {
    pub fn subs(&self, map: &SubsMap) -> OperationResult {
        Ok(match self {
            Self::Number(n) => Self::Number(*n),
            Self::Symbol(symbol) => symbol.subs(map),
            Self::Operation(op) => op.subs(map)?,
        })
    }

    /// Get the inner number or panic if this expression is not a number.
    pub fn num_or_panic(self) -> f64 {
        let Self::Number(n) = self else {
            panic!(
                "Called `.num_or_panic()` on non-number expression `{}`",
                self
            );
        };
        n
    }
}

impl From<&Expr> for Expr {
    fn from(value: &Expr) -> Self {
        // NOTE: this `.clone()` is essentially `Arc::clone()` so it's cheap
        value.clone()
    }
}
impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl From<&f64> for Expr {
    fn from(value: &f64) -> Self {
        Self::Number(*value)
    }
}
impl<'a> From<Symbol> for Expr {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}
impl<'a> From<&Symbol> for Expr {
    fn from(value: &Symbol) -> Self {
        // NOTE: this `.clone()` is essentially `Arc::clone()` so it's cheap
        Self::Symbol(value.clone())
    }
}
impl<T: Substitute + 'static> From<T> for Expr {
    fn from(value: T) -> Self {
        Self::Operation(Operation::Custom(Arc::new(value)))
    }
}
