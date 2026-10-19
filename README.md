# `sym-rs`

A super limited implementation of [`sympy`](https://github.com/sympy/sympy) in Rust (I got bored).

## Example

```rust
use std::collections::HashMap;
use symrs::Symbol;
use symrs::operator::Operable;

fn main() {
    let x = Symbol::new("x");
    // or you can use an existing string
    let y_id = "y".to_string();
    let y = Symbol::new(&y_id);

    // 2x + 3y² - exp(3x/y)
    let equation = 2.0 * x + 3.0 * y.pow(2.0) - (3.0 * x / y).exp();

    // shows "2 * x + 3 * y^2 - exp(3 * x / y)"
    println!("{}", equation);

    let subs_map = HashMap::from([(x, 5.3), (y, -22.8)]);
    // outputs 1569.62210617179
    println!("{}", equation.subs(&subs_map));
}
```

## Limitations

1. Like, 99% of `sympy` is not implemented.
2. Working with threads is a little annoying because `Symbol`s hold a `&str`, so any symbols containing non-static `&str`s cannot be sent across threads.
    - This is necessary so `Symbol` can implement `Copy` and be ergonomic to use. Otherwise, there would be `.clone()`s all over the place or I would need to bastardize `Into<Expr> for &Symbol` for implicit clones.
    - The best workaround is to use [`std::thread::scope()`](https://doc.rust-lang.org/stable/std/thread/fn.scope.html), which lets you borrow non-static data across threads.
    - Something like [`tokio`](https://github.com/tokio-rs/tokio) will probably never work.
