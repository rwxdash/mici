use std::path;

extern crate colored;
use colored::*;

pub fn check_project(project_path: &str) {
    if path::Path::new(&project_path).exists() {
        println!(
            "{} {}",
            "Found existing minici setup at".cyan(),
            project_path.cyan()
        )
    } else {
        println!("{} {}", "Couldn't find minici at", project_path.red());
        println!("{}", "Initializing basic project structure...".yellow());
        println!("{}", "Populating from the repository...".yellow());
        println!("{}", "Done!".green());
    }
}
