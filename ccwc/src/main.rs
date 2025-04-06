use std::{env, process::exit};

mod count;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Not enough arguments");
        exit(1)
    }

    let flag = &args[1];
    let file_name = &args[2];

    if let Some(count) = count::process_flags(flag, file_name) {
        println!("{} {}", count, args[2]);
    } else {
        eprint!("Failed to process file")
    }
}
