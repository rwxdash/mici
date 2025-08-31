use colored::Colorize;
use indoc::printdoc;

use crate::utils::print::print_general_help;
use crate::utils::print::print_individual_help;

use std::path;
use std::process;

pub fn catch_help_and_version_commands(args: &Vec<String>) {
    match &args.get(1).map(String::as_ref) {
        Some("-v" | "--version" | "version") => {
            printdoc! {"
                {} {} {}
            ",
                ">".bright_black(),
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            };
            process::exit(0);
        }
        _ => {}
    }

    match &args.last().map(String::as_ref) {
        Some("-h" | "--help" | "help") => {
            let command_path: &[String] = &args[1..args.len() - 1];

            if command_path.is_empty() {
                print_general_help();
            } else {
                match &args.get(1).map(String::as_ref) {
                    Some("init") | Some("fetch") | Some("new") | Some("list") | Some("edit")
                    | Some("config") => {
                        print_individual_help(args.get(1).unwrap());
                    }
                    _ => {
                        print_individual_help(&command_path.join(path::MAIN_SEPARATOR_STR));
                    }
                }
            }

            process::exit(0);
        }
        _ => {}
    }
}
