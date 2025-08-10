pub mod cli;
pub mod utils;
extern crate dirs;
extern crate getopts;
extern crate handlebars;
extern crate serde_json;

#[cfg(not(target_os = "windows"))]
extern crate pager;

use crate::cli::core::{base_command::InitConfiguration, init_command::INIT_COMMAND};
use crate::utils::{checks::catch_help_and_version_commands, fs::*};
use cli::core::{
    config_command::CONFIG_COMMAND, fetch_command::FETCH_COMMAND, list_command::LIST_COMMAND,
    new_command::NEW_COMMAND,
};
use colored::Colorize;
use getopts::Options;
use indoc::printdoc;
use std::{env, fs, path::Path, sync::OnceLock};

static PROJECT_DIR: &str = ".minici";
static EXECUTABLE: OnceLock<String> = OnceLock::new();

fn main() {
    let args: Vec<String> = env::args().collect();

    // Set which executable is called the command
    let executable: String = Path::new(&args[0])
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("minici")
        .to_string();
    EXECUTABLE.set(executable).unwrap();

    // Read existing configuration file
    let config_exist = Path::new(&get_config_file()).exists();
    if config_exist {
        let config_yaml_str = fs::read_to_string(&get_config_file()).unwrap();
        let config: InitConfiguration = serde_yaml::from_str(&config_yaml_str).unwrap();

        // Control terminal colors
        match config.disable_cli_color {
            Some(true) => {
                colored::control::set_override(false);
            }
            _ => {
                colored::control::set_override(true);
            }
        }

        // Control pager
        match config.disable_pager {
            Some(true) => unsafe {
                std::env::set_var("NOPAGER", "1");
            },
            _ => {}
        }
    }

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
        Some("fetch") => {
            opts.optopt("b", "branch", "", "");
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(err) => {
                    println!("> {}\n", err);
                    return;
                }
            };
            println!("{:?}", &matches);

            match FETCH_COMMAND.run(matches.opt_str("b")) {
                Ok(()) | Err(_) => return,
            };
        }
        Some("new") => {
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(err) => {
                    println!("> {}\n", err);
                    return;
                }
            };

            let command_args = matches.free[1..].to_vec();

            match NEW_COMMAND.run(command_args) {
                Ok(()) => return,
                Err(err) => {
                    println!("> {}\n", err);
                    return;
                }
            };
        }
        Some("config") => {
            match CONFIG_COMMAND.run() {
                Ok(()) | Err(_) => return,
            };
        }
        Some("list") => {
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(err) => {
                    println!("> {}\n", err);
                    return;
                }
            };

            let command_args = matches.free[1..].to_vec();
            match LIST_COMMAND.run(command_args) {
                Ok(()) | Err(_) => return,
            };
        }
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
                      Try running {} {} to see what's available
                ",
                    ">".bright_black(),
                    EXECUTABLE.get().unwrap().underline().bold(),
                    &get_project_folder().underline().bold(),
                    EXECUTABLE.get().unwrap().bright_yellow().bold(),
                    "--help".bright_yellow().bold(),
                };
            } else {
                printdoc! {"
                    {} This is {}!

                      I don't see any existing configuration at {}
                      Try running {} {} to initialize minici
                ",
                    ">".bright_black(),
                    EXECUTABLE.get().unwrap().underline().bold(),
                    &get_project_folder().underline().bold(),
                    EXECUTABLE.get().unwrap().bright_yellow().bold(),
                    "init".bright_yellow().bold(),
                };
            }
        }
    }
}
