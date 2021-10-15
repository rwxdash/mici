use crate::lib::maintenance::init_command::INIT_COMMAND;
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
            println!("caught help");

            println!("{:?}", &args);
            println!("{:?}", &args[1..args.len() - 1]);

            match &args.get(1).map(String::as_ref) {
                Some("init") => {
                    // print_individual_help();
                    println!("\n\nname: {}", INIT_COMMAND.base.name);
                    println!("desc: {}", INIT_COMMAND.base.description);
                    println!("synopsis: {}", INIT_COMMAND.base.synopsis);
                    println!("options: {}", INIT_COMMAND.base.options);
                    println!("usage: {}\n\n", INIT_COMMAND.base.usage);
                }
                Some("seed") => {}
                Some(_) => {
                    println!("custom cmd help")
                    // figure out path by
                    // joining `&args[1..args.len() - 1]` with `/`
                    // check if file exist
                    // if so, print the usage
                    // if not, warn and print general help
                }
                None => {}
            }

            process::exit(0);
        }
        _ => {}
    }
}
