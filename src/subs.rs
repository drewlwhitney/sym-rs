use super::symbol::Symbol;
use crate::error::OperationResult;
use dyn_clone::DynClone;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub type SubsMap<'a> = HashMap<Symbol<'a>, f64>;

#[allow(private_bounds)]
pub trait Substitute<'a>: Display + Debug + DynClone + Send + Sync {
    fn subs(self: Box<Self>, map: &SubsMap) -> OperationResult<'a>;
}
dyn_clone::clone_trait_object!(Substitute<'_>);

// trait CompareSubs<'a> {
//     fn compare_subs(&self, other: &(dyn Substitute + 'a)) -> bool;
// }
//
// impl<'a, T: PartialEq + Substitute<'a>> CompareSubs<'a> for T {
//     fn compare_subs(&self, other: &(dyn Substitute + 'a)) -> bool {
//         // SAFETY: TODO
//         let other: *const dyn Any = unsafe { mem::transmute(other) };
//         let other: &dyn Any = &other;
//         if other.type_id() != typeid::of::<T>() {
//             println!("{:?}", other.type_id());
//             println!("{:?}", typeid::of::<T>());
//             return false;
//         }
//
//         // SAFETY: we just checked that `other` is `T`
//         let other = unsafe { &*(other as *const dyn Any as *const T) };
//
//         self == other
//     }
// }

// TODO: find some way to compare these
impl<'a> PartialEq for Box<dyn Substitute<'a> + 'a> {
    fn eq(&self, _other: &Self) -> bool {
        panic!("Cannot compare trait object types");
        // self.compare_subs(other.as_ref())
    }
}
