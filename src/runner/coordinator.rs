use crate::{
    cli::schemas::v1::{CommandSchemaStep, CommandSchemaStepRunExecution},
    errors::cli::CliError,
    runner::context::ExecutionContext,
    utils::{
        fs::get_scripts_folder,
        resolver::{resolve_environment_variables, resolve_input_variables},
    },
};
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::{io::IsTerminal, process::Command};

pub struct Coordinator<'a> {
    context: ExecutionContext<'a>,
}

impl<'a> Coordinator<'a> {
    pub fn with_context(context: ExecutionContext<'a>) -> Self {
        Self { context }
    }

    pub fn run(&self) -> Result<(), CliError> {
        tracing::info!("Starting execution of: {}", self.context.command.name);

        if let Some(description) = &self.context.command.description {
            tracing::info!("  {}", description);
        }

        if self.context.command.configuration.confirm {
            let confirmation = if std::io::stdin().is_terminal() {
                println!("> This command requires your confirmation!");

                Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you want to continue with the execution?")
                    .wait_for_newline(true)
                    .interact()
                    .map_err(|e| CliError::General {
                        message: e.to_string(),
                    })?
            } else {
                tracing::info!("Command confirmation is piped into the command");

                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let trimmed = input.trim().to_lowercase();

                        match trimmed.as_str() {
                            "y" | "yes" | "true" | "1" => {
                                tracing::debug!("Command confirmed with {}", &trimmed);
                                true
                            }
                            "n" | "no" | "false" | "0" => {
                                tracing::debug!("Command not confirmed with {}", &trimmed);
                                false
                            }
                            _ => {
                                tracing::warn!(
                                    "Piped value '{}' is invalid. Acceptable values: y|yes|true|1 or n|no|false|0",
                                    &trimmed
                                );
                                false
                            }
                        }
                    }
                    Err(_) => false,
                }
            };

            if !confirmation {
                tracing::info!("Command execution cancelled");
                return Ok(());
            }
        }

        tracing::info!("Executing {} steps", self.context.command.steps.len());

        for (index, step) in self.context.command.steps.iter().enumerate() {
            tracing::info!(
                "Step {}/{}: {}",
                index + 1,
                self.context.command.steps.len(),
                step.id
            );

            self.execute_step(step)?;

            tracing::info!("Step completed: {}", step.id);
        }

        tracing::info!("Done!");
        Ok(())
    }

    fn execute_step(&self, step: &CommandSchemaStep) -> Result<(), CliError> {
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

        let inputs = self.context.command.inputs_or_empty();

        let mut cmd = match &step.run.execution {
            CommandSchemaStepRunExecution::Command { command } => {
                let mut c = Command::new(shell);

                let flag = match shell {
                    "bash" | "sh" | "zsh" | "fish" => "-c",
                    "powershell" | "pwsh" => "-Command",
                    "cmd" => "/c",
                    _ => "-c",
                };

                let resolved_command =
                    resolve_input_variables(command, inputs, self.context.matches);

                c.arg(flag).arg(&resolved_command);
                c
            }
            CommandSchemaStepRunExecution::Script { script } => {
                let mut c = Command::new(shell);

                let resolved_script = resolve_input_variables(script, inputs, self.context.matches);

                let script_path = get_scripts_folder().join(&resolved_script);

                c.arg(&script_path);
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
            let resolved_env = resolve_environment_variables(
                command_environment_variables,
                inputs,
                self.context.matches,
            );

            for (key, value) in resolved_env {
                cmd.env(key, value);
            }
        }

        if let Some(step_environment_variables) = &step.run.environment {
            let resolved_env = resolve_environment_variables(
                step_environment_variables,
                inputs,
                self.context.matches,
            );

            for (key, value) in resolved_env {
                cmd.env(key, value);
            }
        }

        // Auto-inject all inputs as MICI_INPUT_* environment variables
        for (name, input) in inputs {
            let value = match input.r#type.as_str() {
                "boolean" | "bool" => {
                    if self.context.matches.opt_present(name) {
                        "true".to_string()
                    } else {
                        input.default.as_deref().unwrap_or("false").to_string()
                    }
                }
                _ => self
                    .context
                    .matches
                    .opt_str(name)
                    .or_else(|| input.default.clone())
                    .unwrap_or_default(),
            };

            let env_key = format!("MICI_INPUT_{}", name.to_uppercase());
            cmd.env(env_key, value);
        }

        let output = cmd.output().map_err(CliError::from)?;

        if !output.stdout.is_empty() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }

        if !output.status.success() {
            let exit_code = output.status.code().unwrap_or(1);
            tracing::error!("Step '{}' failed with exit code: {}", step.id, exit_code);
            return Err(CliError::StepFailed {
                step_id: step.id.clone(),
                exit_code,
            });
        }

        Ok(())
    }
}
