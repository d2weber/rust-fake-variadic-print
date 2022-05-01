use std::fmt::Display;
use std::io::{self, Write};

enum Arg<T, U, V, W> {
    T(T),
    U(U),
    V(V),
    W(W),
}

use Arg::*;

fn concat<T, U, V, W>(args: &[Arg<T, U, V, W>]) -> String
where
    T: Display,
    U: Display,
    V: Display,
    W: Display,
{
    let mut out = String::new();
    for s in args {
        out.push_str(&match s {
            T(x) => x.to_string(),
            U(x) => x.to_string(),
            V(x) => x.to_string(),
            W(x) => x.to_string(),
        });
    }
    out
}

fn main() {
    let my_string = String::from(" and float: ");
    let concat_string = concat(&[
        V("Hello, world! Integer: "),
        U(123),
        W(my_string),
        T(1.2),
        V("\n"),
    ]);
    io::stdout().write_all(concat_string.as_bytes()).unwrap();
}
