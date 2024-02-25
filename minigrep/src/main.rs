
use std::{env, process};

use minigrep::Config;

fn main () {
    let arguments_vector : Vec<String> = env::args().collect();

    let config = Config::build(&arguments_vector).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(message) = minigrep::run(&config) {
        eprintln!("Application Error: {}", message);
        process::exit(1);
    }
}