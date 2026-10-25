use std::collections::HashMap;
use symrs::Symbol;
use symrs::operator::Operand;

fn main() {
    let x = Symbol::new("x");
    // or you can use an existing string
    let y_id = "y".to_string();
    let y = Symbol::new(&y_id);

    // 2x + 3y² - exp(3x/y)
    let equation = 2.0 * x + -(-x + -1.0) * y.pow(2.0) - -(3.0 * x / y).exp();

    // shows "2 * x + 3 * y^2 - exp(3 * x / y)"
    println!("{}", equation);

    let subs_map = HashMap::from([(x, 5.3), (y, -22.8)]);
    // outputs 1569.62210617179
    println!("{}", equation.subs(&subs_map).unwrap());
}
