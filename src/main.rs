use derive_more::Display;
use std::collections::HashMap;
use symrs::Symbol;
use symrs::expr::Expr;
use symrs::operator::Operand;
use symrs::subs::Substitute;

#[derive(Clone, Display, Debug, PartialEq)]
#[display("{_0} + 1")]
struct AddOneOp<'a>(Expr<'a>);

impl<'a> Substitute<'a> for AddOneOp<'a> {
    fn subs(self: Box<Self>, map: &symrs::subs::SubsMap) -> symrs::error::OperationResult<'a> {
        Ok(match self.0.subs(map)? {
            Expr::Number(n) => Expr::Number(n + 1.0),
            expr => Self(expr).into(),
        })
    }
}

fn main() {
    let x = Symbol::new("x");
    // or you can use an existing string
    let y_id = "y".to_string();
    let y = Symbol::new(&y_id);

    // 2x + 3y² - exp(3x/y)
    let equation = 2.0 * x + 3.0 * y.pow(2.0) - (3.0 * x / y).exp();

    let left: Expr = AddOneOp(x + y).into();
    let right: Expr = AddOneOp(x + y).into();

    assert_eq!(left, right);

    // let equation = Expr::Operation(symrs::operator::Operation::Custom(Box::new(AddOneOp(
    //     equation,
    // ))));

    // shows "2 * x + 3 * y^2 - exp(3 * x / y)"
    println!("{}", equation);

    let subs_map = HashMap::from([(x, 5.3), (y, -22.8)]);
    // outputs 1569.62210617179
    println!("{}", equation.subs(&subs_map).unwrap());
}
