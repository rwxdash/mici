extern crate colored;

use crate::PROJECT_DIR;
use colored::*;
use std::fs;
use std::process;

pub fn get_project_folder() -> String {
    let home_dir: &str = &dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()[..];

    format!("{}/{}", home_dir, PROJECT_DIR)
}

pub fn get_jobs_folder() -> String {
    let home_dir: &str = &dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()[..];

    format!("{}/{}/jobs", home_dir, PROJECT_DIR)
}

pub fn get_commands_folder() -> String {
    let home_dir: &str = &dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()[..];

    format!("{}/{}/jobs/commands", home_dir, PROJECT_DIR)
}

pub fn get_scripts_folder() -> String {
    let home_dir: &str = &dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()[..];

    format!("{}/{}/jobs/scripts", home_dir, PROJECT_DIR)
}

pub fn create_folder_at(path: &str) {
    if let Err(e) = fs::create_dir_all(&path) {
        println!(
            "  {} {}",
            "Error while creating".bright_red(),
            &PROJECT_DIR.bright_red()
        );
        println!("  {}", e.to_string().on_red());

        process::exit(1)
    };
}
