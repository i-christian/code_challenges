mod core;
use core::commands_handler;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("$ ");
        stdout().flush().expect("shell initiliasation failure");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read input");
        commands_handler(input);
    }
}
