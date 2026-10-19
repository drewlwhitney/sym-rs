use super::super::expr::Expr;
use crate::helper_macros::*;
use derive_more::Display;
use std::ops::Add;

#[derive(Clone, Display)]
#[display("{_0} + {_1}")]
pub struct AddOp<'a>(Expr<'a>, Expr<'a>);

impl_subs_simple!(AddOp, +);
impl_from_op_expr!(AddOp, Add);
impl_operator!(AddOp, Add, add);
