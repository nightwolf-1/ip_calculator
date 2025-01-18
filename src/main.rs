use std::env;
use std::process;

use crate::utils::handle_arguments::handle_arguments;

mod utils;
mod libs;

fn main() {
    let args: Vec<String> = env::args().collect();
    match handle_arguments(args) {
        Ok(message) => println!("{}", message),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}