use std::{collections::HashMap, fs::File, io::Read};

// define_mappings links a particular flag to a function that handles it
fn define_mappings() -> HashMap<String, fn(&mut File) -> usize> {
    type Processor = fn(&mut File) -> usize;

    let mut mapped_command: HashMap<String, Processor> = HashMap::new();

    mapped_command.insert("-c".to_string(), count_bytes);

    mapped_command
}

pub fn process_flags(flag: &str, file_name: &str) -> Option<usize> {
    let mut file = File::open(file_name);

    let commands = define_mappings();

    let result = match &mut file {
        Ok(file) => {
            let count = commands[flag](file);
            Some(count)
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };

    result
}

fn count_bytes(file: &mut File) -> usize {
    let mut buf = Vec::new();
    let count = file
        .read_to_end(&mut buf)
        .expect("Failed to read file into buffer");
    count
}
