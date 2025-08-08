extern crate colored;
extern crate fs_extra;

use crate::PROJECT_DIR;
use colored::*;
use fs_extra::dir::CopyOptions;
use fs_extra::dir::copy;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

pub fn get_home_dir() -> String {
    dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn get_project_folder() -> String {
    Path::new(&get_home_dir())
        .join(PROJECT_DIR)
        .to_string_lossy()
        .into_owned()
}

pub fn get_config_file() -> String {
    Path::new(&get_home_dir())
        .join(PROJECT_DIR)
        .join("config.yml")
        .to_string_lossy()
        .into_owned()
}

pub fn get_jobs_folder() -> String {
    Path::new(&get_home_dir())
        .join(PROJECT_DIR)
        .join("jobs")
        .to_string_lossy()
        .into_owned()
}

pub fn get_commands_folder() -> String {
    Path::new(&get_home_dir())
        .join(PROJECT_DIR)
        .join("jobs")
        .join("commands")
        .to_string_lossy()
        .into_owned()
}

pub fn get_scripts_folder() -> String {
    Path::new(&get_home_dir())
        .join(PROJECT_DIR)
        .join("jobs")
        .join("scripts")
        .to_string_lossy()
        .into_owned()
}

pub fn clear_jobs_folder() -> Result<(), std::io::Error> {
    fs::remove_dir_all(&get_jobs_folder())
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
    let path = Path::new(&tmp_dir)
        .join(format!("mci-fetch-{}", timestamp))
        .to_string_lossy()
        .into_owned();

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

pub fn copy_directory(from: &str, to: &str) -> Result<u64, fs_extra::error::Error> {
    let options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000, // 64kb
        copy_inside: true,
        content_only: true,
        depth: 0,
    };

    copy(from, to, &options)
}
