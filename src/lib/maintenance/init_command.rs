extern crate colored;
extern crate serde;

use crate::lib::maintenance::base_command::BaseCommand;
use crate::lib::maintenance::base_command::InitConfiguration;
use crate::utils::fs::*;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process;

#[allow(dead_code)]
pub struct InitCommand {
    pub base: BaseCommand,
}

impl InitCommand {
    pub const fn new() -> Self {
        Self {
            base: BaseCommand {
                name: "minici init",
                description: "Used for initializing the project",
                synopsis: "minici init [options]",
                options: "
    --clean (flag)
        This flag will remove the existing minici setup
        and do an empty setup.
                ",
                usage: "
    minici init
        [--clean]
                ",
            },
        }
    }

    pub fn run(&self, clean: bool) -> Result<(), Box<dyn Error>> {
        let minici_exist = Path::new(&get_project_folder()).exists();

        if minici_exist {
            println!("> Found existing minici setup");
            if clean {
                println!("> Doing the cleanup...");
                println!("  {}", "Removing ~/.minici".bright_yellow());
                if let Err(e) = fs::remove_dir_all(&get_project_folder()) {
                    println!("  {}", "Error while removing ~/.minici".on_red());
                    println!("  {}", e.to_string().on_red());
                    process::exit(1)
                } else {
                    println!("  {}", "Cleanup finished!".bright_green());
                }
            } else {
                println!("  Skipping minici setup...");
                println!(
                    "  {} {}",
                    "To do a clean setup, call this with".bright_black(),
                    "--clean".bright_yellow()
                );
                println!(
                    "  {} {}",
                    "For further information, call this with".bright_black(),
                    "--help".bright_yellow()
                );

                return Ok(());
            }
        }

        println!("> Setting up minici...");

        // ~/.minici
        create_folder_at(&get_project_folder());
        create_folder_at(&get_jobs_folder());
        create_folder_at(&get_commands_folder());
        create_folder_at(&get_scripts_folder());

        let upstream_url: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{}", "Upstream repository URL for your commands",))
            .interact_text()
            .unwrap();
        let upstream_cmd_path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{}", "Path for your commands in the repository",))
            .default("./".to_string())
            .interact_text()
            .unwrap();

        let init_configuration = InitConfiguration {
            upstream_url: upstream_url,
            upstream_cmd_path: upstream_cmd_path,
        };
        let mut config_yaml = fs::File::create(format!("{}/config.yml", &get_project_folder()))?;
        let config_yaml_as_string = serde_yaml::to_string(&init_configuration)?;
        config_yaml.write_all(&config_yaml_as_string.as_bytes())?;

        println!(
            "> {} {}",
            "Wrote the given configuration at",
            &get_project_folder()
        );
        println!(
            "  {} {}{}",
            "You can manually update this configuration at",
            &get_project_folder().blue().bold(),
            "/config.yml".blue().bold()
        );

        println!("  {}", "Successfully set up minici!".green());

        return Ok(());
    }
}

pub const INIT_COMMAND: InitCommand = InitCommand::new();
