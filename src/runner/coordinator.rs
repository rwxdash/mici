use crate::{cli::schemas::v1::CommandSchema, runner::context::ExecutionContext};
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::io::IsTerminal;

pub struct Coordinator<'a> {
    context: ExecutionContext<'a>,
    command: &'a CommandSchema,
}

impl<'a> Coordinator<'a> {
    pub fn new(command: &'a CommandSchema, context: ExecutionContext<'a>) -> Self {
        Self { context, command }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // println!("{:?}", self.context.inputs);
        // println!("{:?}", self.schema);

        println!("> Starting execution of: {}", self.command.name);

        if let Some(description) = &self.command.description {
            println!("  {}", description);
        }

        if self.context.configuration.confirm.unwrap() {
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

            if confirmation == false {
                println!("> Command execution cancelled...");
                return Ok(());
            }
        }

        println!("> Executing {} steps", self.command.steps.len());

        // TODO: check when/always conditions for steps
        // I'll need an expression evaluator for the full funcitonality
        // We'll skip this for now.

        for (index, step) in self.command.steps.iter().enumerate() {
            println!("> {}/{}: {}", index + 1, self.command.steps.len(), step.id);

            // Exec step

            println!("  Step completed: {}", step.id);
            println!()
        }

        println!("Done!");
        Ok(())
    }
}
