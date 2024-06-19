use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
        };

        let _hello = output.stdout;

        print!("$ ");
        stdout().flush().expect("print error");

        //running program
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read input");
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().expect("Failed to split command");
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .expect("failed to parse command");

                child.wait().expect("failed to initialise child process");
            }
        }
    }
}
