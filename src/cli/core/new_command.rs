use crate::EXECUTABLE;
use crate::cli::core::base_command::BaseCommand;
use crate::cli::schemas::v1::{
    CommandSchema, CommandSchemaConfiguration, CommandSchemaStep, CommandSchemaStepRun,
    CommandSchemaStepRunExecution,
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

        const TEMPLATE: &str = r#"
##
##  A reference command template
##

# [Required] Command schema version
version: "{version}"
# [Required] A human readable command name
name: "{name}"
# [Optional] Command description
description: "{description}"
# [Optional] Command usage
usage: "{usage}"

# [Required] Configuration options
configuration:
  # [Optional] Whether to prompt for confirmation before running
  # Defaults to false
  # On runtime, it'll accept any of the following inputs:
  #     y | yes | true  | 1
  #     n | no  | false | 0
  confirm: {confirm}

  # [Optional] Environment variables to pass to all steps
  # These will override the OS environment variables if any name matches
  environment:
    VAR_ONE: "SOME_VALUE_123"     # Basic value
    VAR_TWO: "SOME_VALUE_123"     # Basic value
    IS_FORCED: "@{inputs.force}"  # Value from inputs
    TOKEN: "${MY_PRIVATE_TOKEN}"  # Value from OS Environment

  # [Optional] Working directory to run the command from
  # Defaults to current working directory where the commmand is called
  working_directory: null

# [Optional] List of inputs for the command
# Allowed input type values are `string`, `choice`, and `bool` or `boolean`
inputs:
  name:
    type: string                                # [Required]
    description: "A name to say hello to!"      # [Required]
    required: true                              # [Optional] Defaults to false
    secret: false                               # [Optional] Defaults to false
    short: -n                                   # [Optional] Defaults to null
    long: --name                                # [Optional] Defaults to input key, ie. `--name`
    default: "World"                            # [Optional] Defaults to null
  force:
    type: boolean                               # [Required]
    description: "Run this with force, maybe?"  # [Required]
    short: -f                                   # [Optional] Defaults to null
    long: --force                               # [Optional] Defaults to input key, ie. `--force`
  environment:
    type: choice                                # [Required]
    description: "Environment to run this!"     # [Required]
    options:                                    # [Optional] Only checked in `choice` type inputs
      - staging                                 #      Takes an array of strings.
      - production                              #      Validated on runtime.
    required: false                             # [Optional] Defaults to false
    secret: false                               # [Optional] Defaults to false
    short: -e                                   # [Optional] Defaults to null
    long: --environment                         # [Optional] Defaults to input key, ie. `--name`
    default: "production"                       # [Optional] Defaults to null

# [Required] Command steps to execute
# [TODO] Add `when`, `script`, and `args` after implementation is done
steps:
  - id: "{step_id}"               # [Required] A short string with no whitespace to identify this step in the runtime
    name: "{step_name}"           # [Optional] A human readable name for the step to be more descriptive
    run:                          # [Required]
      shell: "{shell}"            # [Optional] The shell that will call the command. Defaults to OS's shell
      working_directory: null     # [Optional] Overrides the `configuration.working_directory` only for this step. Defaults to current
      environment:                # [Optional] Overrides the `configuration.environment` only for this step. Defaults to null
        VAR_TWO: "ANOTHER_VALUE_456"
      # [Required] The actual command to run in this step
      # Can be either `command` for inline codes or `script` to reference another file from outside
      command: |
        {command}
"#;

        #[cfg(unix)]
        let default_shell: &'static str = "bash";

        #[cfg(windows)]
        let default_shell: &'static str = "powershell";

        let schema: CommandSchema = CommandSchema {
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
                id: "say_hello".to_string(),
                name: Some("Say hello on terminal".to_string()),
                when: None,
                run: CommandSchemaStepRun {
                    shell: Some(default_shell.to_string()),
                    args: None,
                    environment: None,
                    working_directory: None,
                    execution: CommandSchemaStepRunExecution::Command {
                        command: r#"
echo "Hello, @{inputs.name}!"
"#
                        .trim()
                        .to_string(),
                    },
                },
            }],
        };

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // let yaml_content = serde_yaml::to_string(&schema)?;
        let yaml_content = TEMPLATE
            .trim_start()
            .replace("{version}", &schema.version)
            .replace("{name}", &schema.name)
            .replace("{description}", &schema.description.unwrap())
            .replace("{usage}", &schema.usage.unwrap())
            .replace(
                "{confirm}",
                &schema.configuration.confirm.unwrap_or_default().to_string(),
            )
            .replace("{step_id}", &schema.steps[0].id)
            .replace("{step_name}", &schema.steps[0].name.as_ref().unwrap())
            .replace("{shell}", &schema.steps[0].run.shell.as_ref().unwrap())
            .replace(
                "{command}",
                &schema.steps[0].run.execution.get_command().unwrap(),
            );
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
