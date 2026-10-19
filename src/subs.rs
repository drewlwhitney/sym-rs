use super::expr::Expr;
use super::symbol::Symbol;
use dyn_clone::DynClone;
use std::collections::HashMap;
use std::fmt::Display;

pub type SubsMap<'a> = HashMap<Symbol<'a>, f64>;

pub trait Substitute: Display + DynClone + Send + Sync {

    fn subs<'a>(self: Box<Self>, map: &SubsMap<'a>) -> Expr<'a>;
}
dyn_clone::clone_trait_object!(Substitute);
