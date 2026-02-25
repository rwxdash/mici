use crate::EXECUTABLE;
use crate::cli::core::base_command::BaseCommand;
use crate::utils::fs::{create_folder_at, get_commands_folder, get_project_folder};
use colored::Colorize;
use dialoguer::{Input, theme::ColorfulTheme};
use indoc::printdoc;
use std::error::Error;
use std::{fs, path};

#[allow(dead_code)]
pub struct NewCommand {
    pub base: BaseCommand,
}

impl Default for NewCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl NewCommand {
    pub const fn new() -> Self {
        NewCommand {
            base: BaseCommand {
                name: "mici new",
                description: "Creates a new command from a template.",
                synopsis: "mici new [<command>...]",
                options: "
    <command>...        (argument)
    The path for the new command (e.g., 'project deploy').

    If omitted, you will be prompted for the path.
                ",
                usage: "
    mici new             # Prompts for creating a new command
    mici new deploy      # Creates a command without prompting at given path
                         # (i.e., .../deploy.yml)
                ",
            },
        }
    }

    pub fn run(&self, command_args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let project_folder = get_project_folder();
        if !project_folder.exists() {
            printdoc! {"
                    {} Can't create a new command.

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

        let commands_folder = get_commands_folder();
        if !commands_folder.exists() {
            create_folder_at(&commands_folder)?;
        }

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

        let file_path = commands_folder.join(format!("{}.yml", &command_path));

        if file_path.exists() {
            return Err(format!(
                "Command already exists at {}\nUse a different path or remove the existing command first.",
                file_path.display()
            ).into());
        }

        const TEMPLATE: &str = r#"
##  ==================================================
##  mici Command Template
##  A reference template for creating new commands
##  ==================================================

##  Schema Version
#
#   Must match supported version (currently "1.0")
#   version: "1.0"
#
version: "{version}"

##  Command Metadata
#
#   name: String          [Required] Human-readable command name
#   description: String   [Optional] Brief description of what this command does
#   usage: String         [Optional] Usage example showing how to invoke this command
#
name: "{name}"
description: "{description}"
usage: "{usage}"

##  Command Configuration
#
#   configuration:
#     confirm: bool
#           [Optional]  default: false
#           Prompt for confirmation before running the command
#           On runtime, it'll accept any of the following inputs:
#               y | yes | true  | 1
#               n | no  | false | 0
#     environment: Map<String, String>
#           [Optional]  default: null
#           Environment variables to pass to all steps
#           These will override the OS environment variables if any name matches
#            Supports:
#              - Basic values: "SOME_VALUE"
#              - Input references: "@{inputs.force}"
#              - OS environment: "${MY_PRIVATE_TOKEN}"
#     working_directory: String
#           [Optional]  default: null
#           Working directory for command execution
#           Defaults to directory where command is invoked
#           Supports @{inputs.*} variable substitution
#           e.g., working_directory: "/projects/@{inputs.project}"
#
configuration:
  confirm: {confirm}
  environment:
    VAR_ONE: "SOME_VALUE_123"
    VAR_TWO: "SOME_VALUE_123"
    IS_FORCED: "@{inputs.force}"
    TOKEN: "${MY_PRIVATE_TOKEN}"
  working_directory: null

##  Command Inputs
#
#   inputs:
#     ...
#     <input_key>:
#       type: String
#           [Required]
#           Input type: "string" | "choice" | "bool" or "boolean"
#       description: String
#           [Required]
#           Help text shown to user
#       options: Vec<String>
#           [Optional]  default: null
#           Array of valid choices
#           Only required and usable for "choice" type inputs
#       required: bool
#           [Optional]  default: false
#           Whether input must be provided
#       secret: bool
#           [Optional]  default: false
#           Hides value in logs/output
#       short: String
#           [Optional]  default: null
#           Short flag format (e.g., "-n")
#       long: String
#           [Optional]  default: "--<input_key>"
#           Long flag format (e.g., "--name")
#       default: String
#           [Optional]  default: null
#           Default value for the input if not provided
#
inputs:
  name:
    type: string
    description: "A name to say hello to!"
    required: true
    secret: false
    short: -n
    long: --name
    default: "World"
  force:
    type: boolean
    description: "Run this with force, maybe?"
    short: -f
    long: --force
  environment:
    type: choice
    description: "Environment to run this!"
    options:
      - staging
      - production
    required: false
    secret: false
    short: -e
    long: --environment
    default: "production"

##  Command Steps
#
#   steps:
#     ...
#     - id: String
#           [Required]
#           Unique identifier for the step
#           No whitespace allowed
#       name: String
#           [Optional]  default: null
#           Human-readable step description
#       when: String
#           [Optional]  default: null
#           Conditional expression to control the step execution
#       run:
#         shell: String
#           [Optional]  default: OS default shell
#           Shell to execute command (e.g., "bash", "powershell")
#         environment: Map<String, String>
#           [Optional]  default: null
#           Override configuration.environment for this step only
#           Supports same syntax as configuration.environment
#         working_directory: String
#           [Optional]  default: configuration.working_directory
#           Override working directory for this step only
#           Supports @{inputs.*} variable substitution
#         command: String
#           [Required if no script]
#           Inline command to execute
#           Supports @{inputs.*} variable substitution
#         script: String
#           [Required if no command]
#           Path to an external script file, relative to ~/.mici/jobs/scripts/
#           Supports @{inputs.*} variable substitution in the path
#           The shell field doubles as the interpreter:
#             shell: "bash"    → bash    ~/.mici/jobs/scripts/deploy.sh
#             shell: "python3" → python3 ~/.mici/jobs/scripts/deploy.py
#             shell: "node"    → node    ~/.mici/jobs/scripts/deploy.js
#
##  Auto-injected Environment Variables
#
#   All inputs are automatically injected as MICI_INPUT_<NAME> environment
#   variables into every step (both command and script). For example:
#     Input "name"        → MICI_INPUT_NAME
#     Input "force"       → MICI_INPUT_FORCE
#     Input "environment" → MICI_INPUT_ENVIRONMENT
#
#   This allows scripts to access inputs without @{inputs.*} substitution:
#     bash:       echo $MICI_INPUT_NAME
#     python:     os.environ["MICI_INPUT_NAME"]
#     node:       process.env.MICI_INPUT_NAME
#     powershell: $env:MICI_INPUT_NAME
#
steps:
  - id: "{step_id}"
    name: "{step_name}"
    run:
      shell: "{shell}"
      working_directory: null
      environment:
        DEPLOY_TARGET: "@{inputs.environment}"
        VAR_TWO: "OVERRIDDEN_VALUE_123"
      command: |
        {command}
"#;

        #[cfg(unix)]
        let default_shell: &'static str = "bash";
        #[cfg(unix)]
        let command = r#"echo "Hello, @{inputs.name}!"
        echo "Printing: $VAR_ONE"
        echo "Printing: $VAR_TWO"
        echo "Deploying to: $DEPLOY_TARGET""#;

        #[cfg(windows)]
        let default_shell: &'static str = "powershell";
        #[cfg(windows)]
        let command = r#"Write-Output "Hello, @{inputs.name}!"
        Write-Output "Printing: $env:VAR_ONE"
        Write-Output "Printing: $env:VAR_TWO"
        Write-Output "Deploying to: $env:DEPLOY_TARGET""#;

        let name = command_path.replace(path::MAIN_SEPARATOR_STR, " ");
        let usage = format!("mici {}", &name);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let yaml_content = TEMPLATE
            .trim_start()
            .replace("{version}", "1.0")
            .replace("{name}", &name)
            .replace("{description}", "A new mici command")
            .replace("{usage}", &usage)
            .replace("{confirm}", "false")
            .replace("{step_id}", "say_hello")
            .replace("{step_name}", "Say hello on terminal")
            .replace("{shell}", default_shell)
            .replace("{command}", command);
        fs::write(&file_path, yaml_content)?;

        printdoc! {"
                {} Created new command at {}
                  Edit the file with {} {} {} to customize your command
                  Run {} {} to use it
            ",
            ">".bright_green(),
            file_path.display().to_string().bright_cyan().bold(),
            EXECUTABLE.get().unwrap().bright_yellow().bold(),
            "edit".bright_yellow().bold(),
            &command_path.replace(path::MAIN_SEPARATOR_STR, " ").bright_yellow().bold(),
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
