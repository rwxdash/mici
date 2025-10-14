use crate::{
    cli::schemas::v1::{CommandSchemaStep, CommandSchemaStepRunExecution},
    runner::context::ExecutionContext,
    utils::resolver::{resolve_environment_variables, resolve_input_variables},
};
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::{collections::BTreeMap, io::IsTerminal, process::Command};

pub struct Coordinator<'a> {
    context: ExecutionContext<'a>,
}

impl<'a> Coordinator<'a> {
    pub fn with_context(context: ExecutionContext<'a>) -> Self {
        Self { context }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // println!("{:?}", self.context.inputs);
        // println!("{:?}", self.schema);

        println!("> Starting execution of: {}", self.context.command.name);

        if let Some(description) = &self.context.command.description {
            println!("  {}", description);
        }

        if self.context.command.configuration.confirm.unwrap_or(false) {
            let confirmation = if std::io::stdin().is_terminal() {
                println!("> This command requires your confirmation!");

                Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you want to continue with the execution?")
                    .wait_for_newline(true)
                    .interact()
                    .unwrap()
            } else {
                // Non-interactive (piped/scripted) - read from stdin
                println!("> Command confirmation is piped into the command!");

                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let trimmed = input.trim().to_lowercase();

                        // TODO: Handle logging in a better way
                        match trimmed.as_str() {
                            "y" | "yes" | "true" | "1" => {
                                println!("> Command is confirmed with {}", &trimmed);

                                true
                            }
                            "n" | "no" | "false" | "0" => {
                                println!("> Command is not confirmed with {}", &trimmed);

                                false
                            }
                            _ => {
                                println!("> Piped value {} is invalid", &trimmed);
                                println!("  Acceptable values are:");
                                println!("      y | yes | true  | 1");
                                println!("      n | no  | false | 0");

                                false
                            }
                        }
                    }
                    Err(_) => false, // Default to false for safety on read errors
                }
            };

            if !confirmation {
                println!("> Command execution cancelled...");
                return Ok(());
            }
        }

        println!("> Executing {} steps", self.context.command.steps.len());

        // TODO: check when/always conditions for steps
        // I'll need an expression evaluator for the full funcitonality
        // We'll skip this for now.

        for (index, step) in self.context.command.steps.iter().enumerate() {
            println!(
                "> {}/{}: {}",
                index + 1,
                self.context.command.steps.len(),
                step.id
            );

            // Exec step
            let _ = self.execute_step(step);

            println!("  Step completed: {}", step.id);
            println!()
        }

        println!("Done!");
        Ok(())
    }

    fn execute_step(&self, step: &CommandSchemaStep) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Handle shell not being present and default to OS choice
        let shell = match &step.run.shell {
            Some(s) => s.as_str(),
            None => {
                #[cfg(unix)]
                {
                    "bash"
                }
                #[cfg(windows)]
                {
                    "powershell"
                }
            }
        };

        let mut cmd = match &step.run.execution {
            CommandSchemaStepRunExecution::Command { command } => {
                let mut c = Command::new(shell);

                let flag = match shell {
                    "bash" | "sh" | "zsh" | "fish" => "-c",
                    "powershell" | "pwsh" => "-Command",
                    "cmd" => "/c",
                    _ => "-c", // Default to POSIX
                };

                // Resolve @{inputs.} variables in the command
                let empty_inputs = BTreeMap::new();
                let resolved_command = resolve_input_variables(
                    command,
                    self.context
                        .command
                        .inputs
                        .as_ref()
                        .unwrap_or(&empty_inputs),
                    &self.context.matches,
                );

                c.arg(flag).arg(&resolved_command);
                c
            }
            CommandSchemaStepRunExecution::Script { script } => {
                let mut c = Command::new(shell);
                c.arg(script);
                c
            }
        };

        // Set Working Directories
        if let Some(command_wd) = &self.context.command.configuration.working_directory {
            cmd.current_dir(command_wd);
        }

        if let Some(step_wd) = &step.run.working_directory {
            cmd.current_dir(step_wd);
        }

        // Set Environment Variables
        if let Some(command_environment_variables) = &self.context.command.configuration.environment
        {
            let empty_inputs = BTreeMap::new();

            let resolved_env = resolve_environment_variables(
                command_environment_variables,
                self.context
                    .command
                    .inputs
                    .as_ref()
                    .unwrap_or(&empty_inputs),
                &self.context.matches,
            );

            for (key, value) in resolved_env {
                cmd.env(key, value);
            }
        }

        if let Some(step_environment_variables) = &step.run.environment {
            let empty_inputs = BTreeMap::new();

            let resolved_env = resolve_environment_variables(
                step_environment_variables,
                self.context
                    .command
                    .inputs
                    .as_ref()
                    .unwrap_or(&empty_inputs),
                &self.context.matches,
            );

            for (key, value) in resolved_env {
                cmd.env(key, value);
            }
        }

        let output = cmd.output()?;

        if !output.stdout.is_empty() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }

        if !output.status.success() {
            return Err(format!(
                "Step '{}' failed with exit code: {:?}",
                step.id,
                output.status.code()
            )
            .into());
        }

        Ok(())
    }
}
