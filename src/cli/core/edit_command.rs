use colored::Colorize;
use indoc::printdoc;

use crate::{
    EXECUTABLE,
    cli::core::base_command::BaseCommand,
    utils::fs::{get_command_file, get_commands_folder, get_project_folder},
};
use std::{
    env,
    error::Error,
    path::{self, Path},
    process::Command,
};

#[allow(dead_code)]
pub struct EditCommand {
    pub base: BaseCommand,
}

impl EditCommand {
    pub const fn new() -> Self {
        EditCommand {
            base: BaseCommand {
                name: "mci edit",
                description: "Edit given command with the default editor.",
                synopsis: "mci edit <command>...",
                options: "
    <command>...        (argument)
    Path to a command to edit in the default editor (e.g., 'project deploy').
                ",
                usage: "
    mci edit project deploy     # Opens `.../project/deploy.yml` command
                                # in the default editor
                ",
            },
        }
    }

    pub fn run(&self, command_args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let minici_exist: bool = Path::new(&get_project_folder()).exists();
        let commands_folder_exist = Path::new(&get_commands_folder()).exists();

        if !minici_exist {
            printdoc! {"
                    {} Can't edit commands.

                      I don't see any existing configuration at {}
                      Try running {} {}
                ",
                ">".bright_black(),
                &get_project_folder().underline().bold(),
                EXECUTABLE.get().unwrap(),
                "init".bright_yellow().bold(),
            };
            return Ok(());
        }

        if !commands_folder_exist {
            printdoc! {"
                    {} Can't edit command.

                      I don't see any existing commands at {}
                      Try creating a command with {} {}
                ",
                ">".bright_black(),
                &get_commands_folder().underline().bold(),
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
                &get_command_file(command_args.join(path::MAIN_SEPARATOR_STR));

            if command_file.is_none() {
                printdoc! {"
                    {} Can't edit command.

                      Command doesn't exists at given path {}.
                      Check the exact usage with {} {}
                ",
                    ">".bright_black(),
                    &command_file_path.underline().bold(),
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

            // let commands = self.edit_commands_from_path(Some(command_path.as_str()))?;
            // Execute the editor command
            let mut c = Command::new(&editor);
            c.arg(&command_file_path);
            let mut cmd = c;

            let status = cmd.status()?;

            if !status.success() {
                return Err(format!("Failed to open config file with editor '{}'", editor).into());
            }
        }

        Ok(())
    }
}

pub const EDIT_COMMAND: EditCommand = EditCommand::new();
