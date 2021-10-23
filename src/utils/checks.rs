use crate::utils::print::print_individual_help;

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
            print_individual_help(&args[1..args.len() - 1].join("/"));

            process::exit(0);
        }
        _ => {}
    }
}
