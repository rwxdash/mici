use crate::{
    EXECUTABLE,
    cli::core::{
        config_command::CONFIG_COMMAND, edit_command::EDIT_COMMAND, fetch_command::FETCH_COMMAND,
        init_command::INIT_COMMAND, list_command::LIST_COMMAND, new_command::NEW_COMMAND,
    },
    utils::{fs::get_commands_folder, traits::ExportAsHashMap, yaml::parse_command_file},
};

use colored::*;
use handlebars::*;
use indoc::printdoc;
use std::path::{self, Path};

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

pub fn print_general_help() {
    let mut handlebars = Handlebars::new();

    let template_asset = include_bytes!("../../templates/general_help.hbs");

    handlebars
        .register_template_string("general_help", std::str::from_utf8(template_asset).unwrap())
        .unwrap();

    handlebars_helper!(bold: |p: String| p.bold().to_string());
    handlebars_helper!(pad_right: |s: String, width: u8| format!("{:<width$}", s, width = width as usize));
    handlebars_helper!(concat: |a: String, b: String| format!("{}{}", a, b));
    handlebars.register_helper("bold", Box::new(bold));
    handlebars.register_helper("pad_right", Box::new(pad_right));
    handlebars.register_helper("concat", Box::new(concat));

    use serde_json::json;

    let data = json!({
        "package_name": env!("CARGO_PKG_NAME"),
        "package_version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "executable": EXECUTABLE.get().unwrap(),
        "commands": [
            {
                "name": "init",
                "description": "Initializes or reconfigures a minici project"
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
    let mut handlebars = Handlebars::new();

    let template_asset = include_bytes!("../../templates/individual_help.hbs");

    handlebars
        .register_template_string(
            "individual_help",
            std::str::from_utf8(template_asset).unwrap(),
        )
        .unwrap();

    handlebars_helper!(bold: |p: String| p.bold().to_string());
    handlebars.register_helper("bold", Box::new(bold));

    match command.as_ref() {
        "init" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &INIT_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        "fetch" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &FETCH_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        "new" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &NEW_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        "edit" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &EDIT_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        "list" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &LIST_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        "config" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &CONFIG_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        _ => {
            let as_folder = Path::new(&get_commands_folder())
                .join(&command)
                .to_string_lossy()
                .into_owned();
            // TODO: This needs to support both .yml and .yaml
            let as_file: String = Path::new(&get_commands_folder())
                .join(format!("{}.yml", &command))
                .to_string_lossy()
                .into_owned();
            let folder_exist: bool = Path::new(&as_folder).exists();
            let command_exist: bool = Path::new(&as_file).exists();

            if command_exist {
                match parse_command_file(&as_file) {
                    Ok(cmd) => {
                        let cmd_map: &mut std::collections::HashMap<&str, &str> =
                            &mut cmd.as_hash_map();

                        let mut options: String = String::new();
                        let synopsis: String;

                        if let Some(inputs) = &cmd.inputs {
                            synopsis = format!(
                                "{} {} {}",
                                EXECUTABLE.get().unwrap(),
                                &command.replace(path::MAIN_SEPARATOR, " "),
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

                                let required_marker = if input_def.required.unwrap_or(false) {
                                    " (required)".bright_red()
                                } else {
                                    "".normal()
                                };

                                options.push_str(&format!(
                                    "\n    {:<16} {}{}\n        {}",
                                    flags,
                                    flag_type.bright_black(),
                                    required_marker,
                                    input_def.description
                                ));

                                if let Some(default) = &input_def.default {
                                    options.push_str(&format!(
                                        " (default: {})",
                                        default.bright_blue()
                                    ));
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
                                &command.replace(path::MAIN_SEPARATOR, " "),
                            );
                        }
                        cmd_map.insert("synopsis", &synopsis.trim());
                        cmd_map.insert("options", &options.trim());

                        // println!("{}", &options);
                        pager();
                        println!("{}", handlebars.render("individual_help", cmd_map).unwrap());
                    }
                    Err(err) => {
                        println!("{}", err.to_string());
                        println!("yaml invalid");
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
                printdoc! {"
                    {} Couldn't find the given command at {}
                      Try creating a new command with {} {}
                      or run {} {} to see the available commands
                ",
                    ">".bright_black(),
                    &as_file.underline().bold(),
                    EXECUTABLE.get().unwrap(),
                    "new".bright_yellow(),
                    EXECUTABLE.get().unwrap(),
                    "--help".bright_yellow(),
                }
            }
        }
    }
}
