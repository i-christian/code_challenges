use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::{Child, Command, Stdio},
};
mod buildin_handlers;

pub fn commands_handler(input: String) {
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().expect("Failed to split command");
        let args = parts;

        match command {
            "cd" => {
                buildin_handlers::handle_cd(args);
                previous_command = None;
            }
            "exit" => return,
            command => {
                let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                    Stdio::from(output.stdout.expect("failed to read command"))
                });

                let stdout = if commands.peek().is_some() {
                    Stdio::piped()
                } else {
                    Stdio::inherit()
                };

                let output = Command::new(command)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                match output {
                    Ok(output) => previous_command = Some(output),
                    Err(e) => {
                        previous_command = None;
                        eprintln!("command not found: {}", e);
                    }
                }
            }
        }
    }

    if let Some(final_command) = previous_command {
        final_command
            .wait_with_output()
            .expect("failed to wait on child");
    }
}

pub fn runner() {
    loop {
        print!("$ ");
        stdout().flush().expect("shell initiliasation failure");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read input");
        commands_handler(input);
    }
}
