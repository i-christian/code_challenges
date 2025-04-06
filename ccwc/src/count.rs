use core::str;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

// help function provide the application help text
pub fn help() -> String {
    let text = "\nUsage: ./target/debug/ccwc [flag] [file name] \nOr cat [filename](optional) | ./target/debug/ccwc[flag]".to_string();
    text
}

// define_mappings links a particular flag to a function that handles it
fn define_mappings() -> HashMap<String, fn(&mut File) -> usize> {
    type Processor = fn(&mut File) -> usize;

    let mut mapped_command: HashMap<String, Processor> = HashMap::new();

    mapped_command.insert("-c".to_string(), count_bytes);
    mapped_command.insert("-l".to_string(), count_lines);
    mapped_command.insert("-w".to_string(), count_words);
    mapped_command.insert("-m".to_string(), count_characters);

    mapped_command
}

// process_flags functions accepts flags and file name.
// It calls the corresponding function to process the file and return the result.
pub fn process_flags<T: AsRef<str>>(flag: T, file_name: T) -> Option<usize> {
    let mut file = File::open(file_name.as_ref());

    let commands = define_mappings();

    if !commands.contains_key(flag.as_ref()) {
        eprintln!("{}", help());
        return None;
    }

    let result = match &mut file {
        Ok(file) => {
            let count = commands[flag.as_ref()](file);
            Some(count)
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };

    result
}

// count_bytes function accepts a mutable reference to a file and returns byte count of a file
fn count_bytes(file: &mut File) -> usize {
    let buf = BufReader::new(file);

    let byte_count = buf.bytes().count();
    byte_count
}

//count_lines function accepts a mutable reference to a file and returns line count of a file
fn count_lines(file: &mut File) -> usize {
    let buf = BufReader::new(file);

    let line_count = buf.lines().count();
    line_count
}

//count_words function accepts a file reference and returns word count
fn count_words(file: &mut File) -> usize {
    let buf = BufReader::new(file);

    let mut total_count = 0;
    let mut words_per_line = 0;

    for line in buf.lines() {
        if let Ok(line) = line {
            words_per_line = line.split_whitespace().count();
        }

        total_count += words_per_line;
    }
    total_count
}

//count_characters function accepts a file reference and returns word count
fn count_characters(file: &mut File) -> usize {
    let mut buf = BufReader::new(file);
    let mut contents = String::new();

    let count = match buf.read_to_string(&mut contents) {
        Ok(_) => {
            let count = contents.chars().count();
            return count;
        }
        Err(_) => 0,
    };

    count
}
