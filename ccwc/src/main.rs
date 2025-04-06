use std::{
    env,
    fs::{self, File},
    io::{self, Read},
    path::Path,
    process::exit,
};

mod count;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("{}", count::help());
        exit(1)
    }

    let flags = vec!["-l", "-w", "-c"];
    let mut count_value = Vec::new();

    if args.len() > 1 {
        match args.len() {
            3 => {
                if let Some(count) = count::process_flags(&args[1], &args[2]) {
                    println!("    {} {}", count, args[2]);
                }
            }

            2 => {
                if !args[1].contains("-") {
                    for flag in flags {
                        if let Some(count) = count::process_flags(flag, &args[1]) {
                            count_value.push(count);
                        }
                    }
                    println!(
                        "    {}  {}  {}  {}",
                        count_value[0], count_value[1], count_value[2], args[1]
                    );
                } else if args[1].contains("-") {
                    eprintln!("{}", count::help());
                    exit(1);
                } else {
                    let _ = File::create("temp.txt");
                    let mut buffer = String::new();

                    let mut stdin = io::stdin();
                    if let Ok(_) = stdin.read_to_string(&mut buffer) {
                        fs::write("temp.txt", buffer).expect("Failed to read stdin data");
                    };

                    if let Some(name) = Path::new("temp.txt").file_name() {
                        let temp_file_name = name.to_str().unwrap().to_owned();
                        let flag = &args[1];

                        if let Some(count) = count::process_flags(flag, &temp_file_name) {
                            println!("    {}", count);
                        }
                    }

                    fs::remove_file("temp.txt").expect("failed to remove temp file");
                }
            }
            _ => eprintln!("{}", count::help()),
        }
    }
}
