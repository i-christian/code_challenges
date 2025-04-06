use std::{env, process::exit};

mod count;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Not enough arguments");
        exit(1)
    }

    let flags = vec!["-l", "-w", "-c"];
    let mut count_value = Vec::new();

    if args.len() > 1 {
        match args.len() {
            3 => {
                if let Some(count) = count::process_flags(&args[1], &args[2]) {
                    println!("    {} {}", count, args[2]);
                } else {
                    eprint!("Failed to process file")
                }
            }

            2 => {
                if args.len() == 2 && !args[1].contains("-") {
                    for flag in flags {
                        if let Some(count) = count::process_flags(flag, &args[1]) {
                            count_value.push(count);
                        } else {
                            eprint!("Failed to process file");
                        }
                    }
                    println!(
                        "    {}  {}  {}  {}",
                        count_value[0], count_value[1], count_value[2], args[1]
                    );
                }
            }

            _ => println!("Default"),
        }
    }
}
