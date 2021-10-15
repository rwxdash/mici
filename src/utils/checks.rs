use std::process;

pub fn check_help_and_version(args: &Vec<String>) {
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
            process::exit(0);
        }
        _ => {}
    }
}
