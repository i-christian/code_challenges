use std::str::SplitWhitespace;
use std::{env, path::Path};

pub fn handle_cd(args: SplitWhitespace<'_>) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(root) {
        eprintln!("{}", e);
    }
}
