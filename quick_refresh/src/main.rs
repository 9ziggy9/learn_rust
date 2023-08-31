use std::env::{self, Args};
use std::iter::Enumerate;

const MSG_WELCOME: &str = "Hello, welcome to command line experiments!";

fn cmd_line_enumerate() -> Enumerate<Args> {
    return env::args().enumerate();
}

fn cmd_line_to_string() -> String {
    return cmd_line_enumerate()
        .map(|(n, arg)| format!("Arg-{n}: {arg}\n"))
        .collect::<String>();
}

fn main() {
    println!("{}", MSG_WELCOME);
    println!("{}", cmd_line_to_string());
}
