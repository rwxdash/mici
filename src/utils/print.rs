use crate::{
    cli::maintenance::{init_command::INIT_COMMAND, seed_command::SEED_COMMAND},
    utils::{fs::get_commands_folder, traits::ExportAsHashMap, yaml::parse_command_file},
};

use colored::*;
use handlebars::*;
use indoc::printdoc;
use pager::Pager;
use std::path::Path;
use std::process;

fn pager() {
    Pager::with_pager("less -r").setup();
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
        "seed" => {
            pager();
            println!(
                "{}",
                handlebars
                    .render("individual_help", &SEED_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        _ => {
            let as_folder: String = format!("{}/{}", &get_commands_folder(), &command);
            let as_file: String = format!("{}/{}.yml", &get_commands_folder(), &command);
            let folder_exist: bool = Path::new(&as_folder).exists();
            let command_exist: bool = Path::new(&as_file).exists();

            if command_exist {
                match parse_command_file(&as_file) {
                    Ok(cmd) => {
                        let cmd_map: &mut std::collections::HashMap<&str, &str> =
                            &mut cmd.as_hash_map();
                        let synopsis: String = format!(
                            "minici {} {}",
                            &command.replace("/", " "),
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
                      Run {} to see the available commands
                ",
                    ">".bright_black(),
                    &command,
                    "minici --help".bright_yellow(),
                }
            } else {
                printdoc! {"
                    {} Couldn't find the given command at {}
                      Try creating a new command with {}
                      or run {} to see the available commands
                ",
                    ">".bright_black(),
                    &as_file.underline().bold(),
                    "minici new".bright_yellow(),
                    "minici --help".bright_yellow(),
                }
            }
            process::exit(1);
        }
    }
}
