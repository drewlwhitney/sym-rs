use super::super::expr::Expr;
use crate::helper_macros::*;
use derive_more::Display;
use std::ops::Sub;

#[derive(Clone, Display)]
#[display("{_0} - {_1}")]
pub struct SubtractOp<'a>(Expr<'a>, Expr<'a>);

impl_subs_simple!(SubtractOp, -);
impl_from_op_expr!(SubtractOp, Subtract);
impl_operator!(SubtractOp, Sub, sub);
