use crate::cli::maintenance::base_command::BaseCommand;
use std::error::Error;

#[allow(dead_code)]
pub struct ListCommand {
    pub base: BaseCommand,
}

impl ListCommand {
    pub const fn new() -> Self {
        ListCommand {
            base: BaseCommand {
                name: "minici list",
                description: "Displays all available commands, optionally filtered by directory",
                synopsis: "minici list [<directory>...]",
                options: "
    <directory>...      (argument)
    One or more directories to list available commands from.
    If omitted, lists commands from all available directories.
                ",
                usage: "
    minici list
        [<directory>...]
                ",
            },
        }
    }

    pub fn run(&self, paths: Vec<String>) -> Result<(), Box<dyn Error>> {
        if paths.is_empty() {
            println!("Listing all available commands...");
        } else {
            println!("Listing commands for paths: {}", paths.join(", "));
        }

        Ok(())
    }
}

pub const LIST_COMMAND: ListCommand = ListCommand::new();
