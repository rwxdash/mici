use crate::PROJECT_DIR;
use std::path;

extern crate colored;
use colored::*;

pub fn check_project() {
    if path::Path::new(&project_folder()).exists() {
        println!(
            "{} {}",
            "Found existing minici setup at".cyan(),
            PROJECT_DIR.cyan()
        )
    } else {
        println!("{} {}", "Couldn't find minici at", PROJECT_DIR.red());
        println!("{}", "Initializing basic project structure...".yellow());
        println!("{}", "Populating from the repository...".yellow());
        println!("{}", "Done!".green());
    }
}

pub fn project_folder() -> String {
    let home_dir: &str = &dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()[..];

    format!("{}/{}", home_dir, PROJECT_DIR)
}
