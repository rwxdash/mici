use crate::EXECUTABLE;
use crate::cli::core::base_command::BaseCommand;
use crate::cli::schemas::v1::{
    CommandSchema, CommandSchemaConfiguration, CommandSchemaStep, CommandSchemaStepRun,
};
use crate::utils::fs::{create_folder_at, get_commands_folder, get_project_folder};
use colored::Colorize;
use dialoguer::{Input, theme::ColorfulTheme};
use indoc::printdoc;
use std::error::Error;
use std::path::Path;
use std::{fs, path};

#[allow(dead_code)]
pub struct NewCommand {
    pub base: BaseCommand,
}

impl NewCommand {
    pub const fn new() -> Self {
        NewCommand {
            base: BaseCommand {
                name: "mci new",
                description: "Creates a new command from a template.",
                synopsis: "mci new [<command>...]",
                options: "
    <command>...        (argument)
    The path for the new command (e.g., 'project deploy').

    If omitted, you will be prompted for the path.
                ",
                usage: "
    mci new             # Prompts for creating a new command
    mci new deploy      # Creates a command without prompting at given path
                        # (i.e., .../deploy.yml)
                ",
            },
        }
    }

    pub fn run(&self, command_args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let minici_exist = Path::new(&get_project_folder()).exists();
        if !minici_exist {
            printdoc! {"
                    {} Can't create a new command.

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

        let commands_folder_exist = Path::new(&get_commands_folder()).exists();
        if !commands_folder_exist {
            create_folder_at(&get_commands_folder());
        }
        let commands_folder = get_commands_folder();

        let command_path = if command_args.is_empty() {
            println!("{} Let's create a new command!", ">".bright_black());
            let prompted_path = self.prompt_for_path()?;
            let prompted_args: Vec<&str> = prompted_path.split_whitespace().collect();
            let prompted_args: Vec<String> =
                prompted_args.into_iter().map(|s| s.to_string()).collect();
            let normalized_path = self.normalize_path(prompted_args)?;

            self.validate_path(&normalized_path)?;
            normalized_path
        } else {
            let normalized_path = self.normalize_path(command_args)?;
            self.validate_path(&normalized_path)?;
            normalized_path
        };

        let file_path = Path::new(&commands_folder).join(format!("{}.yml", &command_path));

        if file_path.exists() {
            return Err(format!(
                "Command already exists at {}\nUse a different path or remove the existing command first.",
                file_path.to_string_lossy()
            ).into());
        }

        // TODO: Replace this
        let schema = CommandSchema {
            version: "1.0".to_string(),
            name: command_path.replace(path::MAIN_SEPARATOR_STR, " "),
            inputs: None,
            description: Some("A new minici command".to_string()),
            usage: Some(format!(
                "mci {}",
                command_path.replace(path::MAIN_SEPARATOR_STR, " ")
            )),
            configuration: CommandSchemaConfiguration {
                confirm: Some(false),
                environment: None,
                working_directory: None,
            },
            steps: vec![CommandSchemaStep {
                id: "run".to_string(),
                name: Some("run".to_string()),
                when: None,
                run: CommandSchemaStepRun {
                    shell: Some("/bin/bash".to_string()),
                    always: Some(false),
                    args: None,
                    script: None,
                    environment: None,
                    working_directory: None,
                    command: Some("echo 'Hello, World!'".to_string()),
                },
            }],
        };

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let yaml_content = serde_yaml::to_string(&schema)?;
        fs::write(&file_path, yaml_content)?;

        printdoc! {"
                {} Created new command at {}
                  Edit the file to customize your command and
                  Run {} {} to use it
            ",
            ">".bright_green(),
            file_path.to_string_lossy().bright_cyan().bold(),
            EXECUTABLE.get().unwrap().bright_yellow().bold(),
            &command_path.replace(path::MAIN_SEPARATOR_STR, " ").bright_yellow().bold(),
        };

        Ok(())
    }

    fn normalize_path(&self, path: Vec<String>) -> Result<String, Box<dyn Error>> {
        let binding = path.join(path::MAIN_SEPARATOR_STR);
        let normalized = binding
            .trim_end_matches(".yml")
            .trim_end_matches(".yaml")
            .trim();

        if normalized.is_empty() {
            // TODO: Better error message.
            return Err("Path cannot be empty".into());
        }

        Ok(normalized.to_string())
    }

    fn validate_path(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let separator_str = path::MAIN_SEPARATOR_STR.to_string();

        // Handle invalid formats
        // TODO: Better error messages.
        // Is this necessary?
        if path.starts_with(&separator_str)
            || path.ends_with(&separator_str)
            || path.contains(&format!("{}{}", separator_str, separator_str))
        {
            return Err(
                "Invalid path format. Use format like 'project deploy' or just 'command'".into(),
            );
        }
        Ok(())
    }

    fn prompt_for_path(&self) -> Result<String, Box<dyn Error>> {
        loop {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Command path (e.g., 'project deploy')")
                .interact_text()?;

            if input.is_empty() {
                println!("{} Path cannot be empty", ">".bright_red());
                continue;
            }

            match self.validate_path(&input) {
                Ok(()) => return Ok(input),
                Err(e) => {
                    println!("{} {}", ">".bright_red(), e);
                    continue;
                }
            }
        }
    }
}

pub const NEW_COMMAND: NewCommand = NewCommand::new();
