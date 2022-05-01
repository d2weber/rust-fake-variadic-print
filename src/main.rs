use std::fmt::Display;
use std::io::{self, Write};

enum Arg<T, U, V, W> {
    T(T),
    U(U),
    V(V),
    W(W),
}

impl<T, U, V, W> Display for Arg<T, U, V, W>
where
    T: Display,
    U: Display,
    V: Display,
    W: Display,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str(&match self {
            T(x) => x.to_string(),
            U(x) => x.to_string(),
            V(x) => x.to_string(),
            W(x) => x.to_string(),
        })
    }
}

use Arg::*;

fn concat<A: Display>(args: &[A]) -> String
{
    args.iter().map(|a| a.to_string()).collect::<Vec<String>>().join("")
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
