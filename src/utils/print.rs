use crate::{
    EXECUTABLE,
    cli::core::{
        config_command::CONFIG_COMMAND, fetch_command::FETCH_COMMAND, init_command::INIT_COMMAND,
        list_command::LIST_COMMAND, new_command::NEW_COMMAND,
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
        "new" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &NEW_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        "init" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &INIT_COMMAND.base.as_hash_map())
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
        "fetch" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &FETCH_COMMAND.base.as_hash_map())
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
        _ => {
            let as_folder = Path::new(&get_commands_folder())
                .join(&command)
                .to_string_lossy()
                .into_owned();
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
                        let synopsis: String = format!(
                            "{} {} {}",
                            EXECUTABLE.get().unwrap(),
                            &command.replace(path::MAIN_SEPARATOR, " "),
                            "[options]".bright_black()
                        );
                        cmd_map.insert("synopsis", &synopsis);

                        let mut options: String = String::from("");
                        for opt in cmd.configuration.options.iter().flatten() {
                            let flag_type: &str;
                            if opt.flag.unwrap() {
                                flag_type = "(flag)";
                            } else {
                                flag_type = "(option)";
                            }

                            let mut flags: String = String::from("");
                            if opt.short.is_some() {
                                flags.push_str(
                                    format!("-{}, ", opt.short.as_ref().unwrap()).as_str(),
                                )
                            }
                            flags.push_str(format!("--{}", opt.long.as_str()).as_str());

                            options.push_str(
                                format!(
                                    "
    {:<16} {}
        {}
                                    ",
                                    flags,
                                    flag_type.bright_black(),
                                    opt.description.as_ref().unwrap().as_str()
                                )
                                .as_str(),
                            );
                        }
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
