use std::env::{self, Args};
use std::iter::Enumerate;

const MSG_WELCOME: &str = "\nHello, welcome to command line experiments!";

fn cmd_line_enumerate() -> Enumerate<Args> { env::args().enumerate() }

fn cmd_line_to_string(args: Enumerate<Args>) -> String {
    return args
        .map(|(n, arg)| format!("Arg-{n}: {arg}"))
        .collect::<Vec<String>>()
        .join("\n");
}

fn msg_intro(msgs: &[&str]) { msgs.iter().for_each(|m| println!("{m}")) }

fn main() {
    let args = cmd_line_enumerate();
    msg_intro(&[
        MSG_WELCOME,
        "\nYou entered the follow command line args:",
        &cmd_line_to_string(args)
    ]);
}
