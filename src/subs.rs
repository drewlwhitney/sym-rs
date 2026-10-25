use super::symbol::Symbol;
use crate::error::OperationResult;
use dyn_clone::DynClone;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub type SubsMap<'a> = HashMap<Symbol<'a>, f64>;

pub trait Substitute<'a>: Display + Debug + DynClone + Send + Sync {
    fn subs(self: Box<Self>, map: &SubsMap) -> OperationResult<'a>;
}
dyn_clone::clone_trait_object!(Substitute<'_>);
