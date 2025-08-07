pub mod cli;
pub mod utils;

extern crate dirs;
extern crate getopts;
extern crate handlebars;
extern crate pager;
extern crate serde_json;

use crate::cli::maintenance::init_command::INIT_COMMAND;
use crate::utils::checks::catch_help_and_version_commands;
use crate::utils::fs::*;
use cli::maintenance::seed_command::SEED_COMMAND;
use colored::Colorize;
use getopts::Options;
use indoc::printdoc;
use std::env;
use std::path::Path;

static PROJECT_DIR: &str = ".minici";

fn main() {
    let args: Vec<String> = env::args().collect();

    // override colorize to successfully pass styles to the pager
    colored::control::set_override(true);

    catch_help_and_version_commands(&args);

    let mut opts = Options::new();

    match &args.get(1).map(String::as_ref) {
        Some("init") => {
            opts.optflag("", "clean", "");
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(_) => {
                    println!(
                        "> {}\n",
                        "Couldn't recognize the given command. Try running with --help".on_red()
                    );
                    return;
                }
            };

            match INIT_COMMAND.run(matches.opt_present("clean")) {
                Ok(()) | Err(_) => return,
            };
        }
        Some("seed") => {
            opts.optopt("b", "branch", "", "");
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(err) => {
                    println!("> {}\n", err);
                    return;
                }
            };
            println!("{:?}", &matches);

            match SEED_COMMAND.run(matches.opt_str("b")) {
                Ok(()) | Err(_) => return,
            };
        }
        Some("new") => todo!(),
        Some("list") => todo!(),
        Some(_) => {
            // check command
            println!("{:#?}", &args[1..]);
            match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(err) => {
                    println!("> {}\n", err);
                    return;
                }
            };
        }
        None => {
            // Check if ~/.minici/config.yml exists
            // If not, print first time help
            // Otherwise, print shorter version
            let minici_exist = Path::new(&get_project_folder()).exists();

            if minici_exist {
                printdoc! {"
                    {} This is {}!
                      Found an existing configuration at {}
                      Try running {} to see what's available
                ",
                    ">".bright_black(),
                    "minici".underline().bold(),
                    &get_project_folder().underline().bold(),
                    "minici --help".bright_yellow().bold(),
                };
            } else {
                printdoc! {"
                    {} This is {}!

                      I don't see any existing configuration at {}
                      Try running {}
                ",
                    ">".bright_black(),
                    "minici".underline().bold(),
                    &get_project_folder().underline().bold(),
                    "minici init".bright_yellow().bold(),
                };
            }
        }
    }
}
