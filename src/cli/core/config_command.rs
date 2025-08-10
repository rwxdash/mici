use crate::cli::core::base_command::BaseCommand;
use crate::utils::fs::*;
use std::env;
use std::error::Error;
use std::process::Command;

#[allow(dead_code)]
pub struct ConfigCommand {
    pub base: BaseCommand,
}

impl ConfigCommand {
    pub const fn new() -> Self {
        Self {
            base: BaseCommand {
                name: "minici config",
                description: "Opens the configuration file in the default editor.",
                synopsis: "minici config",
                options: "None",
                usage: "
    minici config
                ",
            },
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let config_file = get_config_file();

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

        // Execute the editor command
        let mut c = Command::new(&editor);
        c.arg(&config_file);
        let mut cmd = c;

        let status = cmd.status()?;

        if !status.success() {
            return Err(format!("Failed to open config file with editor '{}'", editor).into());
        }

        Ok(())
    }
}

pub const CONFIG_COMMAND: ConfigCommand = ConfigCommand::new();
