use colored::Colorize;
use indoc::printdoc;

use crate::{
    EXECUTABLE,
    cli::core::base_command::BaseCommand,
    utils::fs::{get_command_file, get_commands_folder, get_project_folder},
};
use std::{env, error::Error, path, process::Command};

#[allow(dead_code)]
pub struct EditCommand {
    pub base: BaseCommand,
}

impl Default for EditCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl EditCommand {
    pub const fn new() -> Self {
        EditCommand {
            base: BaseCommand {
                name: "mici edit",
                description: "Edit given command with the default editor.",
                synopsis: "mici edit <command>...",
                options: "
    <command>...        (argument)
    Path to a command to edit in the default editor (e.g., 'project deploy').
                ",
                usage: "
    mici edit project deploy     # Opens `.../project/deploy.yml` command
                                # in the default editor
                ",
            },
        }
    }

    pub fn run(&self, command_args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let project_folder = get_project_folder();
        let commands_folder = get_commands_folder();

        if !project_folder.exists() {
            printdoc! {"
                    {} Can't edit commands.

                      I don't see any existing configuration at {}
                      Try running {} {}
                ",
                ">".bright_black(),
                project_folder.display().to_string().underline().bold(),
                EXECUTABLE.get().unwrap(),
                "init".bright_yellow().bold(),
            };
            return Ok(());
        }

        if !commands_folder.exists() {
            printdoc! {"
                    {} Can't edit command.

                      I don't see any existing commands at {}
                      Try creating a command with {} {}
                ",
                ">".bright_black(),
                commands_folder.display().to_string().underline().bold(),
                EXECUTABLE.get().unwrap(),
                "new".bright_yellow().bold(),
            };
            return Ok(());
        }

        if command_args.is_empty() {
            printdoc! {"
                    {} Can't edit command.

                      Expecting a direct path to the command as arguments.
                      Check the exact usage with {} {}
                ",
                ">".bright_black(),
                EXECUTABLE.get().unwrap().bright_yellow().bold(),
                "edit --help".bright_yellow().bold(),
            };

            return Ok(());
        } else {
            let (command_file_path, command_file) =
                get_command_file(command_args.join(path::MAIN_SEPARATOR_STR))?;

            if command_file.is_none() {
                let display_path = command_file_path.display();
                printdoc! {"
                    {} Can't edit command.

                      Command doesn't exists at given path {}.
                      Check the exact usage with {} {}
                ",
                    ">".bright_black(),
                    display_path.to_string().underline().bold(),
                    EXECUTABLE.get().unwrap().bright_yellow().bold(),
                    "edit --help".bright_yellow().bold(),
                };

                return Ok(());
            }

            // Get the editor from environment variable, fallback to sensible defaults
            let editor = env::var("EDITOR")
                .or_else(|_| env::var("VISUAL"))
                .unwrap_or_else(|_| {
                    // Platform-specific defaults
                    if cfg!(target_os = "windows") {
                        "notepad".to_string()
                    } else if cfg!(target_os = "macos") {
                        "open".to_string()
                    } else {
                        // Linux and other Unix-like systems
                        "nano".to_string()
                    }
                });

            let status = Command::new(&editor).arg(&command_file_path).status()?;

            if !status.success() {
                return Err(format!("Failed to open command file with editor '{}'", editor).into());
            }
        }

        Ok(())
    }
}

pub const EDIT_COMMAND: EditCommand = EditCommand::new();
