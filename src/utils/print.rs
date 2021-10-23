use crate::lib::maintenance::{
    base_command::ExportAsHashMap, init_command::INIT_COMMAND, seed_command::SEED_COMMAND,
};

use handlebars::*;
use pager::Pager;

fn pager() {
    Pager::with_pager("less -r").setup();
}

pub fn print_individual_help(command: &String) {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("individual_help", "./templates/individual_help.hbs")
        .unwrap();

    pager();
    match command.as_ref() {
        "init" => {
            println!(
                "{}",
                handlebars
                    .render("individual_help", &INIT_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        "seed" => {
            println!(
                "{}",
                handlebars
                    .render("individual_help", &SEED_COMMAND.base.as_hash_map())
                    .unwrap(),
            );
        }
        _ => {
            println!("custom cmd help")
            // figure out path by
            // joining `&args[1..args.len() - 1]` with `/`
            // check if file exist
            // if so, print the usage
            // if not, warn and print general help
        }
    }
}
