extern crate colored;

use crate::PROJECT_DIR;
use colored::*;
use std::env;
use std::fs;
use std::process;

pub fn get_home_dir() -> String {
    dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn get_project_folder() -> String {
    format!("{}/{}", &get_home_dir(), PROJECT_DIR)
}

pub fn get_config_file() -> String {
    format!("{}/{}/config.yml", &get_home_dir(), PROJECT_DIR)
}

pub fn get_jobs_folder() -> String {
    format!("{}/{}/jobs", &get_home_dir(), PROJECT_DIR)
}

pub fn get_commands_folder() -> String {
    format!("{}/{}/jobs/commands", &get_home_dir(), PROJECT_DIR)
}

pub fn get_scripts_folder() -> String {
    format!("{}/{}/jobs/scripts", &get_home_dir(), PROJECT_DIR)
}

pub fn create_folder_at(path: &str) {
    if let Err(e) = fs::create_dir_all(&path) {
        println!(
            "  {} {}",
            "Error while creating".bright_red(),
            &path.bright_red()
        );
        println!("  {}", e.to_string().on_red());

        process::exit(1)
    };
}

pub fn create_tmp_folder() -> String {
    let tmp_dir = env::temp_dir().into_os_string().into_string().unwrap();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let path = format!("{}/mci-seed-{}", tmp_dir, timestamp);

    if let Err(e) = fs::create_dir_all(&path) {
        println!(
            "  {} {}",
            "Error while creating".bright_red(),
            &path.bright_red()
        );
        println!("  {}", e.to_string().on_red());

        process::exit(1)
    };

    return path;
}
