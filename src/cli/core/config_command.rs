use crate::cli::core::base_command::{BaseCommand, InitConfiguration};
use crate::utils::fs::*;
use std::env;
use std::error::Error;
use std::process::Command;

#[allow(dead_code)]
pub struct ConfigCommand {
    pub base: BaseCommand,
}

impl Default for ConfigCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigCommand {
    pub const fn new() -> Self {
        Self {
            base: BaseCommand {
                name: "mici config",
                description: "Opens the configuration file in the default editor.",
                synopsis: "mici config",
                options: "None",
                usage: "
    mici config          # Opens the config.yml in the default editor
                ",
            },
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let config_file = get_config_file();

        // Auto-create config.yml with defaults if it doesn't exist
        if !config_file.exists() {
            if let Some(parent) = config_file.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let default_config = InitConfiguration::default();
            std::fs::write(&config_file, default_config.format_config_yaml())?;
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

        let status = Command::new(&editor).arg(&config_file).status()?;

        if !status.success() {
            return Err(format!("Failed to open config file with editor '{}'", editor).into());
        }

        Ok(())
    }
}

pub const CONFIG_COMMAND: ConfigCommand = ConfigCommand::new();
