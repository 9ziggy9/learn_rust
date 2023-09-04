use std::env;

const MSG_WELCOME:     &str = "\nHello, welcome to command line experiments!";
const ERR_BAD_ARG_NO:  &str = "BAD ASSERT: must supply 1 command line arg.";

fn cmd_line_to_string(args: Vec<String>) -> String {
    return args
        .iter().skip(1).enumerate()
        .map(|(n, arg)| format!("arg {}: {arg}", n+1))
        .collect::<Vec<String>>()
        .join("\n");
}

fn msg_intro(msgs: &[&str]) { msgs.iter().for_each(|m| println!("{m}")) }

fn main() {
    // Equivalent: let args = env::args().collect::<Vec<String>>();
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "{}", ERR_BAD_ARG_NO);
    msg_intro(&[
        MSG_WELCOME,
        "\nYou entered the follow command line args:",
        &cmd_line_to_string(args)
    ]);
}
