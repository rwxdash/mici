use crate::{
    lib::maintenance::{
        base_command::ExportAsHashMap, init_command::INIT_COMMAND, seed_command::SEED_COMMAND,
    },
    utils::fs::get_commands_folder,
};

use colored::*;
use handlebars::*;
use pager::Pager;
use std::path::Path;
use std::process;

fn pager() {
    Pager::with_pager("less -r").setup();
}

pub fn print_individual_help(command: &String) {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("individual_help", "./templates/individual_help.hbs")
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
            let as_folder = format!("{}/{}", &get_commands_folder(), &command);
            let as_file = format!("{}/{}.yml", &get_commands_folder(), &command);
            let folder_exist = Path::new(&as_folder).exists();
            let command_exist = Path::new(&as_file).exists();

            if !folder_exist && !command_exist {
                println!(
                    "> {} {}",
                    "Couldn't find the given command at",
                    &as_file.bright_red()
                );
                println!(
                    "  {} {}",
                    "Try creating a new command or run",
                    "minici --help".bright_yellow()
                );
                println!(
                    "  {} {} {} {}",
                    "If you need help with creating a command, run",
                    "minici --doc".bright_yellow(),
                    "or visit",
                    "https://minici.rs".bright_blue(),
                );
                process::exit(1);
            }

            if folder_exist {
                println!(
                    "> {} {}",
                    &command, "isn't a valid command, it's a directory!"
                );
                println!(
                    "  {} {} {}",
                    "Run",
                    "minici --help".bright_yellow(),
                    "to see the available commands"
                );
                process::exit(1);
            }

            if command_exist {
                // parse yaml and print individual_help
            }

            // figure out path by
            // joining `&args[1..args.len() - 1]` with `/`
            // check if file exist
            // if so, print the usage
            // if not, warn and print general help
        }
    }
}
