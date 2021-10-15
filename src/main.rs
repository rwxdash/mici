pub mod lib;
pub mod utils;

extern crate dirs;
extern crate getopts;

use crate::lib::maintenance::init_command::INIT_COMMAND;
use crate::utils::checks::check_help_and_version;
use colored::Colorize;
use getopts::Options;
use std::env;

static PROJECT_DIR: &str = ".minici";

fn main() {
    let args: Vec<String> = env::args().collect();
    check_help_and_version(&args);

    let mut opts = Options::new();

    match &args.get(1).map(String::as_ref) {
        Some("init") => {
            opts.optflag("", "clean", "");
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(_) => {
                    println!(
                        "> {}\n",
                        "Couldn't recognize the given command. Try running --help".on_red()
                    );
                    return;
                }
            };

            match INIT_COMMAND.run(matches.opt_present("clean")) {
                Ok(()) | Err(_) => return,
            };
        }
        Some("seed") => {
            // run seed
            let mut branch = String::new();
            opts.optopt("b", "branch", "", "");
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(_) => return,
            };
            if matches.opt_present("b") {
                println!("{:?}", matches.opt_positions("b"));
                branch = matches.opt_str("b").unwrap();
            }

            println!("{}", branch);
        }
        Some(_) => {
            // check command
        }
        None => {
            println!("help none");
        }
    }
}

// minici root project subcommand [flag]

// minici   thundra         foresight       frontend        deploy
//          maintenance     slack-bot       update          --anyflag="123"

/*

> check for help and version flag
> check for other flags
> pop flags
> joint the rest with `/`
> validate file is present
    - error if not
    - print help
> parse the command yaml
> run the steps

*/
