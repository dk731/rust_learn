use std::{env, process};

use minigrep;

fn main() {
    let conf = minigrep::InputConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error occured during arguments parsing: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
