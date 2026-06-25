use std::process;

use crate::utils::handle_arguments::handle_arguments;

mod utils;
mod libs;
mod tui;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "--tui" || args[1] == "-t") {
        if let Err(e) = tui::run() {
            eprintln!("TUI error: {}", e);
            process::exit(1);
        }
    } else {
        match handle_arguments(args) {
            Ok(message) => println!("{}", message),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}
