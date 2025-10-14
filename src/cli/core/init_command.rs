extern crate colored;
extern crate serde;

use crate::EXECUTABLE;
use crate::cli::core::base_command::BaseCommand;
use crate::cli::core::base_command::InitConfiguration;
use crate::utils::fs::*;
use colored::*;
use dialoguer::{Input, theme::ColorfulTheme};
use indoc::printdoc;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path;
use std::path::Path;
use std::process;

const MICI_REPOSITORY: &str = "git@github.com:rwxdash/mici.git";

#[cfg(unix)]
const MICI_EXAMPLES_PATH: &str = "./examples/unix";

#[cfg(windows)]
const MICI_EXAMPLES_PATH: &str = "./examples/windows";

#[allow(dead_code)]
pub struct InitCommand {
    pub base: BaseCommand,
}

impl InitCommand {
    pub const fn new() -> Self {
        Self {
            base: BaseCommand {
                name: "mici init",
                description: "Initializes a new mici project or reconfigures an existing setup.",
                synopsis: "mici init [options]",
                options: "
    --clean     (flag)
    Remove any existing mici configuration and perform a fresh setup.
    Use this to reset your environment.
                ",
                usage: "
    mici init            # Initialize a new project if it doesn't exist
    mici init --clean    # Initialize a new project from scratch
                ",
            },
        }
    }

    pub fn run(&self, clean: bool) -> Result<(), Box<dyn Error>> {
        let mici_exist = Path::new(&get_project_folder()).exists();

        if mici_exist {
            if clean {
                printdoc! {"
                    {} Found existing mici setup
                    {} Doing the cleanup...
                      {} {}
                ",
                    ">".bright_black(),
                    ">".bright_black(),
                    "Removing".bright_yellow(),
                    &get_project_folder().bright_yellow()
                }

                if let Err(e) = fs::remove_dir_all(&get_project_folder()) {
                    println!(
                        "  {}{}",
                        "Error while removing ".bright_red(),
                        &get_project_folder().bright_red().underline().bold()
                    );
                    println!("  {}", e.to_string().bright_red());
                    process::exit(1)
                } else {
                    println!("  {}", "Cleanup finished!\n".bright_green());
                }
            } else {
                printdoc! {"
                    {} Found existing mici setup
                      Skipping mici setup...
                      {} {}
                      {} {}
                ",
                    ">".bright_black(),
                    "To do a clean setup, call this with".bright_black(),
                    "--clean".bright_yellow(),
                    "For further information, call this with".bright_black(),
                    "--help".bright_yellow()
                }

                return Ok(());
            }
        }

        println!("{} Setting up mici...", ">".bright_black(),);

        let set_upstream = dialoguer::Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you keep your commands on a remote repository?")
            .item(format!(
                "{}   I haven't committed them anywhere yet",
                "No".bright_red().bold()
            ))
            .item(format!(
                "{}  They are already on a git repository",
                "Yes".bright_green().bold()
            ))
            .interact()
            .unwrap();

        let init_configuration: InitConfiguration;

        if set_upstream == 1 {
            let upstream_url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{}", "Upstream repository URL for your commands",))
                .default(MICI_REPOSITORY.to_string())
                .interact_text()
                .unwrap();
            let upstream_cmd_path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{}", "Path for your commands in the repository",))
                .default(MICI_EXAMPLES_PATH.to_string())
                .interact_text()
                .unwrap();

            init_configuration = InitConfiguration {
                upstream_url: Some(upstream_url),
                upstream_cmd_path: Some(upstream_cmd_path),
                disable_cli_color: Some(false),
                disable_pager: Some(false),
            };
        } else {
            init_configuration = InitConfiguration {
                upstream_url: None,
                upstream_cmd_path: None,
                disable_cli_color: Some(false),
                disable_pager: Some(false),
            };
        }

        // ~/.mici
        create_folder_at(&get_project_folder());
        create_folder_at(&get_jobs_folder());
        create_folder_at(&get_commands_folder());
        create_folder_at(&get_scripts_folder());

        let mut config_yaml = fs::File::create(&get_config_file())?;
        let config_yaml_as_string = serde_yaml::to_string(&init_configuration)?;
        config_yaml.write_all(&config_yaml_as_string.as_bytes())?;

        printdoc! {"
            {} Wrote the given configuration at {}{}{}
              You can update this configuration manually by editing this file
              Run {} {} to pull your commands from this repository
            ",
            ">".bright_black(),
            &get_project_folder().bright_cyan().bold(),
            path::MAIN_SEPARATOR_STR,
            "config.yml".bright_cyan().bold(),
            EXECUTABLE.get().unwrap(),
            "fetch".blue().bold(),
        }

        return Ok(());
    }
}

pub const INIT_COMMAND: InitCommand = InitCommand::new();
