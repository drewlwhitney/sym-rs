use super::operator::Operation;
use super::subs::SubsMap;
use super::subs::Substitute;
use super::symbol::Symbol;
use derive_more::Display;

#[derive(Clone, Display)]
pub enum Expr<'a> {
    Number(f64),
    Symbol(Symbol<'a>),
    Operation(Operation<'a>),
}
impl<'a> Expr<'a> {
    pub fn subs(self, map: &'a SubsMap) -> Expr<'a> {
        match self {
            number @ Self::Number(_) => number,
            Self::Symbol(symbol) => symbol.subs(map),
            Self::Operation(op) => op.subs(map),
        }
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
impl<'a, T: Substitute + 'static> From<T> for Expr<'a> {
    fn from(value: T) -> Self {
        Self::Operation(Operation::Custom(Box::new(value)))
    }
}
