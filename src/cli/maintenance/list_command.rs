use colored::Colorize;
use indoc::printdoc;

use crate::{
    EXECUTABLE,
    cli::maintenance::base_command::BaseCommand,
    utils::fs::{create_folder_at, get_commands_folder, get_project_folder},
};
use std::{
    error::Error,
    fs,
    path::{self, Path},
};

#[allow(dead_code)]
pub struct ListCommand {
    pub base: BaseCommand,
}

impl ListCommand {
    pub const fn new() -> Self {
        ListCommand {
            base: BaseCommand {
                name: "minici list",
                description: "Displays all available commands, optionally filtered by directory.",
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

    fn collect_commands(
        &self,
        base_path: &Path,
        relative_path: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut commands = Vec::new();

        if !base_path.exists() {
            return Ok(commands);
        }

        let entries = fs::read_dir(base_path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if path.is_file() && file_name_str.ends_with(".yml") || file_name_str.ends_with(".yaml")
            {
                let command_name = file_name_str
                    .trim_end_matches(".yml")
                    .trim_end_matches(".yaml");

                let full_command = if relative_path == "." {
                    command_name.to_string()
                } else {
                    Path::new(relative_path)
                        .join(command_name)
                        .to_string_lossy()
                        .to_string()
                };

                commands.push(full_command);
            } else if path.is_dir() && !file_name_str.starts_with('.') {
                let sub_relative_path = if relative_path == "." {
                    file_name_str.to_string()
                } else {
                    Path::new(relative_path)
                        .join(file_name_str.to_string())
                        .to_string_lossy()
                        .to_string()
                };
                let mut subcommands = self.collect_commands(&path, &sub_relative_path)?;
                commands.append(&mut subcommands);
            }
        }

        Ok(commands)
    }

    fn list_commands_from_path(
        &self,
        path_filter: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let commands_folder = get_commands_folder();
        let base_path = Path::new(&commands_folder);

        if path_filter.is_none() {
            // List all commands
            self.collect_commands(base_path, ".")
        } else {
            // List commands from specific subdirectory
            let filter_path = base_path.join(path_filter.unwrap());
            self.collect_commands(&filter_path, path_filter.unwrap())
        }
    }

    fn display_commands(&self, commands: &Vec<String>, path_filter: Option<&str>) {
        if commands.is_empty() {
            if let Some(filter) = path_filter {
                println!(
                    "{} No commands found in path: {}",
                    ">".bright_black(),
                    filter.bright_yellow()
                );
            } else {
                println!("{} No commands found", ">".bright_black());
            }

            return;
        }

        let header_path = match path_filter {
            Some(_) => {
                let binding = Path::new(&get_commands_folder()).join(path_filter.unwrap());
                binding.to_string_lossy().to_string()
            }
            None => Path::new(&get_commands_folder())
                .to_string_lossy()
                .to_string(),
        };

        printdoc! {"
            {} Found {} commands in {}
        ",
            ">".bright_black(),
            commands.len().to_string().bright_cyan(),
            header_path.bright_yellow().bold()
        }

        println!();

        let mut sorted_commands = commands.clone();

        // Sorting logic:
        // 1. Split commands into two groups: depth = 0 vs depth > 0
        // 2. Commands with depth = 0 appear at top
        // 3. Sort alphabetically within each group (depth = 0 vs depth > 0)
        sorted_commands.sort_by(|a, b| {
            let a_depth = a.matches(path::MAIN_SEPARATOR).count();
            let b_depth = b.matches(path::MAIN_SEPARATOR).count();

            match (a_depth == 0, b_depth == 0) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });

        let mut current_prefix: Option<String> = None::<String>;

        for command in &sorted_commands {
            let command_depth = command.matches(path::MAIN_SEPARATOR).count();

            let command_prefix = if command_depth > 0 {
                command
                    .split(path::MAIN_SEPARATOR)
                    .next()
                    .unwrap_or("")
                    .to_string()
            } else {
                String::new()
            };

            if let Some(previous_prefix) = current_prefix {
                if command_prefix != previous_prefix {
                    println!(
                        "  {}",
                        "-".repeat(EXECUTABLE.get().unwrap().len()).bright_black()
                    );
                }
            }

            current_prefix = Some(command_prefix);

            println!(
                "  {} {}",
                EXECUTABLE.get().unwrap().bright_black(),
                command.replace(path::MAIN_SEPARATOR, " ")
            );
        }
        println!();
    }

    pub fn run(&self, command_args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let minici_exist = Path::new(&get_project_folder()).exists();
        let commands_folder_exist = Path::new(&get_commands_folder()).exists();

        if !minici_exist {
            printdoc! {"
                    {} Can't list commands.

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
            create_folder_at(&get_commands_folder());
        }

        if command_args.is_empty() {
            let commands = self.list_commands_from_path(None)?;
            self.display_commands(&commands, None);
        } else {
            let path_filter = command_args.join(path::MAIN_SEPARATOR_STR);

            let commands = self.list_commands_from_path(Some(path_filter.as_str()))?;
            self.display_commands(&commands, Some(path_filter.as_str()));
        }

        Ok(())
    }
}

pub const LIST_COMMAND: ListCommand = ListCommand::new();
