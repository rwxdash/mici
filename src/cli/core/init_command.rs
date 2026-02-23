use crate::EXECUTABLE;
use crate::cli::core::base_command::BaseCommand;
use crate::cli::core::base_command::{InitConfiguration, LogTimer};
use crate::utils::fs::*;
use colored::*;
use dialoguer::{Input, theme::ColorfulTheme};
use indoc::printdoc;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path;

const MICI_REPOSITORY: &str = "git@github.com:rwxdash/mici.git";

#[cfg(unix)]
const MICI_EXAMPLES_PATH: &str = "./examples/unix";

#[cfg(windows)]
const MICI_EXAMPLES_PATH: &str = "./examples/windows";

#[allow(dead_code)]
pub struct InitCommand {
    pub base: BaseCommand,
}

impl Default for InitCommand {
    fn default() -> Self {
        Self::new()
    }
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
        let project_folder = get_project_folder();

        if project_folder.exists() {
            if clean {
                printdoc! {"
                    {} Found existing mici setup
                    {} Doing the cleanup...
                      {} {}
                ",
                    ">".bright_black(),
                    ">".bright_black(),
                    "Removing".bright_yellow(),
                    project_folder.display().to_string().bright_yellow()
                }

                fs::remove_dir_all(&project_folder).map_err(|e| {
                    format!("Error while removing {}: {}", project_folder.display(), e)
                })?;

                println!("  {}", "Cleanup finished!\n".bright_green());
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
            .interact()?;

        let init_configuration: InitConfiguration = if set_upstream == 1 {
            let upstream_url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Upstream repository URL for your commands")
                .default(MICI_REPOSITORY.to_string())
                .interact_text()?;
            let upstream_cmd_path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Path for your commands in the repository")
                .default(MICI_EXAMPLES_PATH.to_string())
                .interact_text()?;

            InitConfiguration {
                upstream_url: Some(upstream_url),
                upstream_cmd_path: Some(upstream_cmd_path),
                disable_cli_color: Some(false),
                disable_pager: Some(false),
                log_timer: Some(LogTimer::Wallclock),
            }
        } else {
            InitConfiguration {
                upstream_url: None,
                upstream_cmd_path: None,
                disable_cli_color: Some(false),
                disable_pager: Some(false),
                log_timer: Some(LogTimer::Wallclock),
            }
        };

        // ~/.mici
        create_folder_at(&get_project_folder())?;
        create_folder_at(&get_jobs_folder())?;
        create_folder_at(&get_commands_folder())?;
        create_folder_at(&get_scripts_folder())?;

        let config_file = get_config_file();
        let mut config_yaml = fs::File::create(&config_file)?;
        let config_yaml_as_string = self.format_config_yaml(&init_configuration);
        config_yaml.write_all(config_yaml_as_string.as_bytes())?;

        printdoc! {"
            {} Wrote the given configuration at {}{}{}
              You can update this configuration manually by editing this file
              Run {} {} to pull your commands from this repository
            ",
            ">".bright_black(),
            project_folder.display().to_string().bright_cyan().bold(),
            path::MAIN_SEPARATOR_STR,
            "config.yml".bright_cyan().bold(),
            EXECUTABLE.get().unwrap(),
            "fetch".blue().bold(),
        }

        Ok(())
    }

    fn format_config_yaml(&self, config: &InitConfiguration) -> String {
        let format_optional = |val: &Option<String>| match val {
            Some(v) => format!("\"{}\"", v),
            None => "null".to_string(),
        };

        let format_bool = |val: &Option<bool>| match val {
            Some(true) => "true".to_string(),
            Some(false) => "false".to_string(),
            None => "false".to_string(),
        };

        format!(
            r#"##  ==================================================
##  mici Configuration
##  Global settings for all mici commands
##  ==================================================

##  Upstream Repository
#
#   upstream_url: String
#         [Optional]  default: null
#         Git repository URL where your commands are stored
#         Used by `mici fetch` to pull commands from a remote
#   upstream_cmd_path: String
#         [Optional]  default: null
#         Path to the commands directory within the repository
#
upstream_url: {upstream_url}
upstream_cmd_path: {upstream_cmd_path}

##  Terminal Settings
#
#   disable_cli_color: bool
#         [Optional]  default: false
#         Disable colored output in the terminal
#   disable_pager: bool
#         [Optional]  default: false
#         Disable the pager for long output (e.g., help text)
#
disable_cli_color: {disable_cli_color}
disable_pager: {disable_pager}

##  Logging
#
#   log_timer: String
#         [Optional]  default: "wallclock"
#         Timer style for tracing log output
#         Options:
#           "wallclock" - Full timestamps (e.g., 2026-02-23T14:30:00Z)
#           "uptime"    - Time since process start (e.g., 0.003s)
#           "none"      - No timestamps in log output
#
log_timer: {log_timer}
"#,
            upstream_url = format_optional(&config.upstream_url),
            upstream_cmd_path = format_optional(&config.upstream_cmd_path),
            disable_cli_color = format_bool(&config.disable_cli_color),
            disable_pager = format_bool(&config.disable_pager),
            log_timer = config
                .log_timer
                .as_ref()
                .map_or("wallclock".to_string(), |t| t.to_string()),
        )
    }
}

pub const INIT_COMMAND: InitCommand = InitCommand::new();
