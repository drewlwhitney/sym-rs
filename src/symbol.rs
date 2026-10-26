use std::sync::Arc;

use super::expr::Expr;
use super::subs::SubsMap;

use derive_more::Display;

#[derive(Hash, Eq, PartialEq, Display, Debug, Clone)]
pub struct Symbol(Arc<String>);
impl Symbol {
    pub fn new(id: impl Into<String>) -> Self {
        Self(Arc::new(id.into()))
    }

    pub fn subs(&self, map: &SubsMap) -> Expr {
        if let Some(value) = map.get(&self) {
            return Expr::Number(*value);
        }
        self.into()
    }
}
