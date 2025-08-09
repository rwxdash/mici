use crate::utils::print::print_individual_help;

use std::path;
use std::process;

pub fn catch_help_and_version_commands(args: &Vec<String>) {
    match &args.get(1).map(String::as_ref) {
        Some("-v" | "--version" | "version") => {
            println!("caught version");
            process::exit(0);
        }
        _ => {}
    }

    match &args.last().map(String::as_ref) {
        Some("-h" | "--help" | "help") => {
            let command_path: &[String] = &args[1..args.len() - 1];

            if command_path.is_empty() {
                println!("caught general help")
            } else {
                print_individual_help(&command_path.join(&path::MAIN_SEPARATOR.to_string()));
            }

            process::exit(0);
        }
        _ => {}
    }
}
