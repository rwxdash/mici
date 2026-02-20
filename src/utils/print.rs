use crate::{
    EXECUTABLE,
    cli::core::{
        CORE_COMMANDS, config_command::CONFIG_COMMAND, edit_command::EDIT_COMMAND,
        fetch_command::FETCH_COMMAND, init_command::INIT_COMMAND, list_command::LIST_COMMAND,
        new_command::NEW_COMMAND, validate_command::VALIDATE_COMMAND,
    },
    utils::{
        fs::{get_command_file, get_commands_folder},
        traits::ExportAsHashMap,
        yaml::parse_command_file,
    },
};

use colored::*;
use handlebars::*;
use indoc::printdoc;
use std::collections::HashMap;
use std::path;
use std::sync::OnceLock;

#[cfg(not(target_os = "windows"))]
use pager::Pager;

#[cfg(not(target_os = "windows"))]
fn pager() {
    use std::process::Command;

    // Check if 'less' command exists
    let has_less = Command::new("which")
        .arg("less")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if has_less {
        Pager::with_pager("less -r").setup();
    } else {
        // Fallback to cat (always available on Unix)
        Pager::with_pager("cat").setup();
    }
}

#[cfg(target_os = "windows")]
fn pager() {
    // No-op
}

fn get_handlebars() -> &'static Handlebars<'static> {
    static HBS: OnceLock<Handlebars<'static>> = OnceLock::new();
    HBS.get_or_init(|| {
        let mut hbs = Handlebars::new();

        let general_template = include_bytes!("../../templates/general_help.hbs");
        let individual_template = include_bytes!("../../templates/individual_help.hbs");

        hbs.register_template_string(
            "general_help",
            std::str::from_utf8(general_template).unwrap(),
        )
        .unwrap();
        hbs.register_template_string(
            "individual_help",
            std::str::from_utf8(individual_template).unwrap(),
        )
        .unwrap();

        handlebars_helper!(bold: |p: String| p.bold().to_string());
        handlebars_helper!(pad_right: |s: String, width: u8| format!("{:<width$}", s, width = width as usize));
        handlebars_helper!(concat: |a: String, b: String| format!("{}{}", a, b));
        hbs.register_helper("bold", Box::new(bold));
        hbs.register_helper("pad_right", Box::new(pad_right));
        hbs.register_helper("concat", Box::new(concat));

        hbs
    })
}

/// Get the base hash map for a core command by name.
fn core_command_help_data(name: &str) -> Option<HashMap<&str, &str>> {
    match name {
        "init" => Some(INIT_COMMAND.base.as_hash_map()),
        "fetch" => Some(FETCH_COMMAND.base.as_hash_map()),
        "new" => Some(NEW_COMMAND.base.as_hash_map()),
        "edit" => Some(EDIT_COMMAND.base.as_hash_map()),
        "validate" => Some(VALIDATE_COMMAND.base.as_hash_map()),
        "list" => Some(LIST_COMMAND.base.as_hash_map()),
        "config" => Some(CONFIG_COMMAND.base.as_hash_map()),
        _ => None,
    }
}

pub fn print_general_help() {
    let handlebars = get_handlebars();

    use serde_json::json;

    let data = json!({
        "package_name": env!("CARGO_PKG_NAME"),
        "package_version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "executable": EXECUTABLE.get().unwrap(),
        "commands": [
            {
                "name": "init",
                "description": "Initializes or reconfigures a mici project"
            },
            {
                "name": "fetch",
                "description": "Synchronizes and updates local commands from remote"
            },
            {
                "name": "new",
                "description": "Creates a new command from a template"
            },
            {
                "name": "edit",
                "description": "Opens the given command file in the default editor"
            },
            {
                "name": "validate",
                "description": "Validates a given command's specification"
            },
            {
                "name": "list",
                "description": "Displays available commands"
            },
            {
                "name": "config",
                "description": "Opens the configuration file in the default editor"
            },
            {
                "name": "version",
                "description": "Display version information"
            },
            {
                "name": "help",
                "description": "Display help information"
            }
        ]
    });

    pager();
    println!("{}", handlebars.render("general_help", &data).unwrap());
}

pub fn print_individual_help(command: &String) {
    let handlebars = get_handlebars();

    // Handle core commands
    if CORE_COMMANDS.contains(&command.as_str()) {
        if let Some(data) = core_command_help_data(command) {
            pager();
            println!("{}", handlebars.render("individual_help", &data).unwrap(),);
            return;
        }
    }

    // Handle dynamic (user-defined) commands
    let commands_folder = get_commands_folder();
    let as_folder = commands_folder.join(command);
    let folder_exist: bool = as_folder.exists();

    let (command_file_path, command_file) = match get_command_file(command.to_string()) {
        Ok(result) => result,
        Err(err) => {
            eprintln!(
                "{} {}\n  {}",
                ">".bright_black(),
                "Error:".bright_red(),
                err
            );
            return;
        }
    };
    let command_exist: bool = command_file.is_some();

    if command_exist {
        match parse_command_file(&command_file_path) {
            Ok(cmd) => {
                let cmd_map: &mut HashMap<&str, &str> = &mut cmd.as_hash_map();

                let mut options: String = String::new();
                let synopsis: String;

                if let Some(inputs) = &cmd.inputs {
                    synopsis = format!(
                        "{} {} {}",
                        EXECUTABLE.get().unwrap(),
                        &command.replace(path::MAIN_SEPARATOR_STR, " "),
                        "[options]".bright_black()
                    );

                    for (input_name, input_def) in inputs {
                        let flag_type = match input_def.r#type.as_str() {
                            "boolean" => "(flag)",
                            _ => "(option)",
                        };

                        let mut flags: String = String::from("");
                        if let Some(short) = &input_def.short {
                            flags.push_str(&format!("{}, ", short));
                        }
                        if let Some(long) = &input_def.long {
                            flags.push_str(long);
                        } else {
                            flags.push_str(&format!("--{}", input_name));
                        }

                        let secret_marker = if input_def.secret {
                            " (secret)".bright_black()
                        } else {
                            "".normal()
                        };

                        let required_marker = if input_def.required {
                            " (required)".bright_red()
                        } else {
                            "".normal()
                        };

                        options.push_str(&format!(
                            "\n    {:<16} {}{}{}\n        {}",
                            flags,
                            flag_type.bright_black(),
                            secret_marker,
                            required_marker,
                            input_def.description
                        ));

                        if let Some(default) = &input_def.default {
                            options.push_str(&format!(" (default: {})", default.bright_blue()));
                        }

                        if let Some(choices) = &input_def.options {
                            options.push_str(&format!(
                                " (choices: {})",
                                choices.join(", ").bright_cyan()
                            ));
                        }
                    }
                } else {
                    // If there are no options...
                    synopsis = format!(
                        "{} {}",
                        EXECUTABLE.get().unwrap(),
                        &command.replace(path::MAIN_SEPARATOR_STR, " "),
                    );
                }
                cmd_map.insert("synopsis", &synopsis.trim());
                cmd_map.insert("options", &options.trim());

                pager();
                println!("{}", handlebars.render("individual_help", cmd_map).unwrap());
            }
            Err(err) => {
                let report = miette::Report::new(err);
                eprintln!("{:?}", report);
                std::process::exit(1);
            }
        }
    } else if folder_exist {
        printdoc! {"
            {} {} isn't a valid command, it's a directory!
              Run {} {} to see the available commands
        ",
            ">".bright_black(),
            &command,
            EXECUTABLE.get().unwrap(),
            "--help".bright_yellow(),
        }
    } else {
        let display_path = command_file_path.display();
        printdoc! {"
            {} Couldn't find the given command at {}
              Try creating a new command with {} {}
              or run {} {} to see the available commands
        ",
            ">".bright_black(),
            display_path.to_string().underline().bold(),
            EXECUTABLE.get().unwrap(),
            "new".bright_yellow(),
            EXECUTABLE.get().unwrap(),
            "--help".bright_yellow(),
        }
    }
}
