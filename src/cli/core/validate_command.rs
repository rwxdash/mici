use colored::Colorize;
use indoc::printdoc;

use crate::{
    EXECUTABLE,
    cli::core::base_command::BaseCommand,
    utils::{
        fs::{get_command_file, get_commands_folder, get_project_folder},
        yaml::parse_command_file,
    },
};
use std::{
    error::Error,
    path::{self},
};

#[allow(dead_code)]
pub struct ValidateCommand {
    pub base: BaseCommand,
}

impl ValidateCommand {
    pub const fn new() -> Self {
        ValidateCommand {
            base: BaseCommand {
                name: "mici validate",
                description: "Validate the given command's specification.",
                synopsis: "mici validate <command>...",
                options: "
    <command>...        (argument)
    Path to a command to validate (e.g., 'project deploy').
                ",
                usage: "
    mici validate project deploy     # Validates `.../project/deploy.yml`
                ",
            },
        }
    }

    pub fn run(&self, command_args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let project_folder = get_project_folder();
        let commands_folder = get_commands_folder();

        if !project_folder.exists() {
            printdoc! {"
                    {} Can't validate commands.

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
                    {} Can't validate command.

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
                    {} Can't validate command.

                      Expecting a direct path to the command as arguments.
                      Check the exact usage with {} {}
                ",
                ">".bright_black(),
                EXECUTABLE.get().unwrap().bright_yellow().bold(),
                "validate --help".bright_yellow().bold(),
            };

            return Ok(());
        } else {
            let (command_file_path, command_file) =
                match get_command_file(command_args.join(path::MAIN_SEPARATOR_STR)) {
                    Ok(result) => result,
                    Err(err) => return Err(err.into()),
                };

            if command_file.is_none() {
                let display_path = command_file_path.display();
                printdoc! {"
                    {} Can't validate command.

                      Command doesn't exists at given path {}.
                      Check the exact usage with {} {}
                ",
                    ">".bright_black(),
                    display_path.to_string().underline().bold(),
                    EXECUTABLE.get().unwrap().bright_yellow().bold(),
                    "validate --help".bright_yellow().bold(),
                };

                return Ok(());
            }

            match parse_command_file(&command_file_path) {
                Ok(_) => {
                    printdoc! {"
                            {} Command is valid! 🎉

                              You can run or check the usage of the command with:
                              {} {} {}

                              {} {}
                        ",
                        "✓".bright_green(),
                    EXECUTABLE.get().unwrap().bright_yellow().bold(),
                        command_args.join(" ").bright_yellow().bold(),
                        "--help".bright_black(),
                        r#"
                        If your command still fails due to some validation error,
  that means our validator is lacking something.

  Feel free to open an issue at
                        "#.trim().bright_black(),
                        env!("CARGO_PKG_REPOSITORY").bright_black().underline(),
                    };
                }
                Err(err) => {
                    let report = miette::Report::new(err);
                    eprintln!("{:?}", report);
                    std::process::exit(1);
                }
            }
        }

        Ok(())
    }
}

pub const VALIDATE_COMMAND: ValidateCommand = ValidateCommand::new();
