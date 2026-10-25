use super::error::OperationResult;
use super::operator::Operation;
use super::subs::SubsMap;
use super::subs::Substitute;
use super::symbol::Symbol;
use derive_more::Display;

#[derive(Clone, Display, Debug)]
pub enum Expr<'a> {
    Number(f64),
    Symbol(Symbol<'a>),
    Operation(Operation<'a>),
}
impl<'a> Expr<'a> {
    pub fn subs(self, map: &SubsMap) -> OperationResult<'a> {
        Ok(match self {
            Self::Number(n) => Self::Number(n),
            Self::Symbol(symbol) => symbol.subs(map),
            Self::Operation(op) => op.subs(map)?,
        })
    }
}
impl<'a> From<f64> for Expr<'a> {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl<'a> From<Symbol<'a>> for Expr<'a> {
    fn from(value: Symbol<'a>) -> Self {
        Self::Symbol(value)
    }
}
impl<'a, T: Substitute<'a> + 'a> From<T> for Expr<'a> {
    fn from(value: T) -> Self {
        Self::Operation(Operation::Custom(Box::new(value)))
    }
}
