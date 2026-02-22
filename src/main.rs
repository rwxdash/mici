pub mod cli;
pub mod errors;
pub mod runner;
pub mod utils;
use crate::{
    cli::{
        core::{
            base_command::InitConfiguration, config_command::CONFIG_COMMAND,
            edit_command::EDIT_COMMAND, fetch_command::FETCH_COMMAND, init_command::INIT_COMMAND,
            list_command::LIST_COMMAND, new_command::NEW_COMMAND,
            validate_command::VALIDATE_COMMAND,
        },
        schemas::v1,
    },
    errors::cli::CliError,
    runner::{context::ExecutionContext, coordinator::Coordinator},
    utils::{checks::catch_help_and_version_commands, fs::*, yaml::parse_command_file},
};
use colored::Colorize;
use getopts::Options;
use indoc::printdoc;
use std::{
    env, fs,
    path::{self, Path},
    sync::OnceLock,
};

static PROJECT_DIR: &str = ".mici";
static EXECUTABLE: OnceLock<String> = OnceLock::new();

fn main() -> miette::Result<()> {
    run()
}

fn run() -> miette::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("mici=info".parse().unwrap()),
        )
        .without_time()
        .init();

    let args: Vec<String> = env::args().collect();

    // Set which executable is called the command
    let executable: String = Path::new(&args[0])
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("mici")
        .to_string();
    EXECUTABLE.set(executable).unwrap();

    // Read existing configuration file
    let config_file = get_config_file();
    if config_file.exists() {
        match fs::read_to_string(&config_file) {
            Ok(config_yaml_str) => {
                match serde_yaml::from_str::<InitConfiguration>(&config_yaml_str) {
                    Ok(config) => {
                        // Control terminal colors
                        match config.disable_cli_color {
                            Some(true) => {
                                colored::control::set_override(false);
                            }
                            _ => {
                                colored::control::set_override(true);
                            }
                        }

                        // Control pager
                        if let Some(true) = config.disable_pager {
                            unsafe {
                                std::env::set_var("NOPAGER", "1");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "{} Warning: Failed to parse config file: {}",
                            ">".bright_black(),
                            e
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "{} Warning: Failed to read config file: {}",
                    ">".bright_black(),
                    e
                );
            }
        }
    }

    catch_help_and_version_commands(&args);

    let mut opts = Options::new();

    match &args.get(1).map(String::as_ref) {
        Some("init") => {
            opts.optflag("", "clean", "");
            let matches = parse_opts(&opts, &args[1..])?;

            INIT_COMMAND
                .run(matches.opt_present("clean"))
                .map_err(CliError::from)?;
        }
        Some("fetch") => {
            opts.optopt("b", "branch", "", "");
            let matches = parse_opts(&opts, &args[1..])?;

            FETCH_COMMAND
                .run(matches.opt_str("b"))
                .map_err(CliError::from)?;
        }
        Some("new") => run_args_command(&opts, &args, |a| NEW_COMMAND.run(a))?,
        Some("edit") => run_args_command(&opts, &args, |a| EDIT_COMMAND.run(a))?,
        Some("validate") => run_args_command(&opts, &args, |a| VALIDATE_COMMAND.run(a))?,
        Some("list") => run_args_command(&opts, &args, |a| LIST_COMMAND.run(a))?,
        Some("config") => {
            CONFIG_COMMAND.run().map_err(CliError::from)?;
        }
        Some(_) => {
            run_dynamic_command(&args, &mut opts)?;
        }
        None => {
            let project_folder = get_project_folder();

            if project_folder.exists() {
                printdoc! {"
                    {} This is {}!
                      Found an existing configuration at {}
                      Try running {} {} to see what's available
                ",
                    ">".bright_black(),
                    EXECUTABLE.get().unwrap().underline().bold(),
                    project_folder.display().to_string().underline().bold(),
                    EXECUTABLE.get().unwrap().bright_yellow().bold(),
                    "--help".bright_yellow().bold(),
                };
            } else {
                printdoc! {"
                    {} This is {}!

                      I don't see any existing configuration at {}
                      Try running {} {} to initialize mici
                ",
                    ">".bright_black(),
                    EXECUTABLE.get().unwrap().underline().bold(),
                    project_folder.display().to_string().underline().bold(),
                    EXECUTABLE.get().unwrap().bright_yellow().bold(),
                    "init".bright_yellow().bold(),
                };
            }
        }
    }

    Ok(())
}

/// Parse args with the given options, returning CliError on failure.
fn parse_opts(opts: &Options, args: &[String]) -> miette::Result<getopts::Matches> {
    opts.parse(args)
        .map_err(|err| CliError::ArgParse(err.to_string()).into())
}

/// Run a core command that takes Vec<String> args.
fn run_args_command(
    opts: &Options,
    args: &[String],
    run: impl FnOnce(Vec<String>) -> Result<(), Box<dyn std::error::Error>>,
) -> miette::Result<()> {
    let matches = parse_opts(opts, &args[1..])?;
    let command_args = matches.free[1..].to_vec();
    run(command_args).map_err(CliError::from)?;
    Ok(())
}

fn run_dynamic_command(args: &[String], opts: &mut Options) -> miette::Result<()> {
    let command_args = &args[1..];
    let options_start = command_args.iter().position(|arg| arg.starts_with("-"));

    let (command_parts, option_args) = match options_start {
        Some(p) => (&command_args[..p], &command_args[p..]),
        None => (command_args, &[] as &[String]),
    };

    let (command_file_path, command_file) =
        match get_command_file(command_parts.join(path::MAIN_SEPARATOR_STR)) {
            Ok(result) => result,
            Err(err) => {
                return Err(CliError::General {
                    message: err.to_string(),
                }
                .into());
            }
        };

    if command_file.is_none() {
        let display_path = command_file_path.display();
        printdoc! {"
            {} Can't run command.

              Command doesn't exists at given path {}.
              Check the exact usage with {} {}
        ",
            ">".bright_black(),
            display_path.to_string().underline().bold(),
            EXECUTABLE.get().unwrap().bright_yellow().bold(),
            "edit --help".bright_yellow().bold(),
        };

        return Ok(());
    }

    let cmd = parse_command_file(&command_file_path)?;

    if let Some(inputs) = &cmd.inputs {
        for (name, input) in inputs {
            let strip_dashes = |s: &str| s.trim_start_matches('-').to_string();

            let short = input.short.as_deref().map(strip_dashes).unwrap_or_default();
            let long = input
                .long
                .as_deref()
                .map(strip_dashes)
                .unwrap_or_else(|| name.to_string());

            match input.r#type.as_str() {
                "boolean" | "bool" => {
                    opts.optflag(&short, &long, &input.description);
                }
                _ => {
                    opts.optopt(&short, &long, &input.description, "");
                }
            }
        }
    }

    let matches = parse_opts(opts, option_args)?;

    if let Some(inputs) = &cmd.inputs {
        v1::validate_inputs(inputs, &matches)?;
    }

    let context = ExecutionContext::new(&cmd, &matches);
    let coordinator = Coordinator::with_context(context);

    coordinator.run().map_err(CliError::from)?;

    Ok(())
}
