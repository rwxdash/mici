use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

use crate::lib::maintenance::base_command::BaseCommand;
use crate::utils::fs::project_folder;
use crate::PROJECT_DIR;

extern crate colored;
use colored::*;

#[allow(dead_code)]
pub struct InitCommand {
    base: BaseCommand,
}

impl InitCommand {
    pub const fn new() -> Self {
        Self {
            base: BaseCommand {
                name: "minici init",
                description: "Used for initializing the project",
                synopsis: "
                minici init
                    [--clean]
                ",
                options: "
                --clean (flag)
                    This flag will remove the existing minici setup
                    and do an empty setup.
                ",
                usage: "minici init [--clean]",
            },
        }
    }

    pub fn run(&self, clean: bool) -> Result<(), Box<dyn Error>> {
        let minici_exist = Path::new(&project_folder()).exists();

        if minici_exist {
            println!("> Found existing minici setup");
            if clean {
                println!("> Doing the cleanup...");
                println!("  {}", "Removing ~/.minici".bright_yellow());
                if let Err(e) = fs::remove_dir_all(&project_folder()) {
                    println!("  {}", "Error while removing ~/.minici".on_red());
                    println!("  {}\n", e.to_string().on_red());
                    process::exit(1)
                } else {
                    println!("  {}", "Cleanup finished!".bright_green());
                }
            } else {
                println!("  Skipping minici setup...");
                println!(
                    "  {} {}",
                    "To do a clean setup, call this with".bright_black(),
                    "--clean".bright_yellow()
                );
                println!(
                    "  {} {}",
                    "For further information, call this with".bright_black(),
                    "--help\n".bright_yellow()
                );

                return Ok(());
            }
        }

        println!("> Setting up minici...");
        if let Err(e) = fs::create_dir_all(&project_folder()) {
            println!(
                "  {} {}",
                "Error while creating".bright_red(),
                &PROJECT_DIR.bright_red()
            );
            println!("  {}\n", e.to_string().on_red());

            process::exit(1)
        }
        println!("  {}\n", "Successfully set up minici!".green());

        return Ok(());
    }
}

pub const INIT_COMMAND: InitCommand = InitCommand::new();
