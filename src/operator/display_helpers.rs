//! Helper functions for implementing [`fmt::Display`].

use crate::expr::Expr;
use crate::operator::Operation;
use std::fmt;

/// Write a possibly parenthesized expression, such as `(1 + 2)` when formatting for `3 * (1 + 2)`.
pub fn write_multiply_divide_operand(f: &mut fmt::Formatter, expr: &Expr) -> fmt::Result {
    match expr {
        Expr::Operation(o) => match o {
            Operation::Add(_) | Operation::Subtract(_) => write!(f, "({o})")?,
            Operation::Multiply(_)
            | Operation::Divide(_)
            | Operation::Exp(_)
            | Operation::Power(_)
            | Operation::Sqrt(_)
            | Operation::Abs(_)
            | Operation::Neg(_)
            | Operation::Custom(_) => write!(f, "{o}")?,
        },
        other => write!(f, "{other}")?,
    }

    Ok(())
}

/// Write a possibly parenthesized expression, such as `(5 * 4)` when formatting for `6^(5 * 4)`.
pub fn write_power_operand(f: &mut fmt::Formatter, expr: &Expr) -> fmt::Result {
    match expr {
        Expr::Number(n) if *n < 0.0 => write!(f, "({n})")?, // something like 5^(-2)
        Expr::Operation(o) => match o {
            Operation::Add(_)
            | Operation::Subtract(_)
            | Operation::Multiply(_)
            | Operation::Divide(_)
            | Operation::Neg(_) => write!(f, "({o})")?,
            Operation::Power(_)
            | Operation::Exp(_)
            | Operation::Sqrt(_)
            | Operation::Abs(_)
            | Operation::Custom(_) => write!(f, "{o}")?,
        },
        other => write!(f, "{other}")?,
    }

    Ok(())
}

/// Write a possibly parenthesized expression, such as `(-1)` in `2 - (-1)`.
pub fn write_maybe_negative_operand(f: &mut fmt::Formatter, expr: &Expr) -> fmt::Result {
    match expr {
        Expr::Number(n) if *n < 0.0 => write!(f, "({n})")?,
        Expr::Operation(o) => match o {
            Operation::Neg(neg) => write!(f, "({neg})")?,
            Operation::Add(add) => {
                write_maybe_negative_operand(f, &add.0)?;
                write!(f, " + ")?;
                write_maybe_negative_operand(f, &add.1)?;
            }
            Operation::Subtract(sub) => {
                write_maybe_negative_operand(f, &sub.0)?;
                write!(f, " - ")?;
                write_maybe_negative_operand(f, &sub.1)?;
            }
            Operation::Multiply(mult) => {
                write_maybe_negative_operand(f, &mult.0)?;
                write!(f, " * {}", mult.1)?;
            }
            Operation::Divide(div) => {
                write_maybe_negative_operand(f, &div.0)?;
                write!(f, " / {}", div.1)?;
            }
            Operation::Power(_)
            | Operation::Exp(_)
            | Operation::Sqrt(_)
            | Operation::Abs(_)
            | Operation::Custom(_) => write!(f, "{o}")?,
        },
        other => write!(f, "{other}")?,
    }

    Ok(())
}
