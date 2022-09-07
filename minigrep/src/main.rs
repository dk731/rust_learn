use std::error::Error;
use std::{env, fs};

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let params = minigrep::InputArgs::new(&args);

    let file_contents = match fs::read_to_string(params.file_path()) {
        Err(_) => return println!("Was not able to open a file"),
        Ok(file_str) => file_str,
    };
}
