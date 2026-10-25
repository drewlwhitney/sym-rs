use super::expr::Expr;
use super::subs::SubsMap;

use derive_more::Display;

#[derive(Hash, Eq, PartialEq, Display, Debug, Clone, Copy)]
pub struct Symbol<'a>(&'a str);
impl<'a> Symbol<'a> {
    pub fn new(id: &'a str) -> Self {
        Self(id)
    }

    pub fn subs(self, map: &SubsMap) -> Expr<'a> {
        if let Some(value) = map.get(&self) {
            return Expr::Number(*value);
        }
        self.into()
    }
}
