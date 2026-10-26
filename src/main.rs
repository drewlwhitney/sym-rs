use derive_more::Display;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use symrs::Symbol;
use symrs::expr::Expr;
use symrs::operator::Operand;
use symrs::subs::Substitute;

#[derive(Clone, Display, Debug, PartialEq)]
#[display("{_0} + 1")]
struct AddOneOp(Expr);

impl Substitute for AddOneOp {
    fn subs(&self, map: &symrs::subs::SubsMap) -> symrs::error::OperationResult {
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
    let equation = 2.0 * &x + 3.0 * y.pow(2.0) - (3.0 * &x / &y).exp();

    let left: Expr = AddOneOp(&x + &y).into();
    let right: Expr = AddOneOp(&x + &y).into();

    thread::spawn({
        let equation = equation.clone();
        let x = x.clone();
        let y = y.clone();
        move || {
            let subs_map = HashMap::from([(x, 5.3), (y, -22.8)]);
            // outputs 1569.62210617179
            println!("{}", equation.subs(&subs_map).unwrap());
        }
    });

    assert_eq!(left, right);

    // let equation = Expr::Operation(symrs::operator::Operation::Custom(Box::new(AddOneOp(
    //     equation,
    // ))));

    // shows "2 * x + 3 * y^2 - exp(3 * x / y)"
    println!("{}", equation);

    let subs_map = HashMap::from([(x, 5.3), (y, -22.8)]);
    // outputs 1569.62210617179
    println!("{}", equation.subs(&subs_map).unwrap());

    thread::sleep(Duration::from_secs(1));
}
