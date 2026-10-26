use super::symbol::Symbol;
use crate::error::OperationResult;
use dyn_clone::DynClone;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub type SubsMap = HashMap<Symbol, f64>;

/// Required trait for custom math operations to act as expressions.
#[allow(private_bounds)]
pub trait Substitute: Display + Debug + DynClone + DynPartialEq + Send + Sync {
    /// Substitute the provided values into the expression and return the result.
    fn subs(&self, map: &SubsMap) -> OperationResult;
}
dyn_clone::clone_trait_object!(Substitute);

/// Compare with a trait object.
trait DynPartialEq: Any {
    fn dyn_eq(&self, other: &dyn Any) -> bool;
}

// you must implement `PartialEq` to implement `PartialDynEq` (and therefore `Substitute`)
impl<T: PartialEq + Substitute + 'static> DynPartialEq for T {
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        let Some(other) = other.downcast_ref::<T>() else {
            return false;
        };
        self == other
    }
}

impl PartialEq for dyn Substitute + 'static {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other as &dyn Any)
    }
}
