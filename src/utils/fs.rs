extern crate colored;
extern crate fs_extra;

use crate::PROJECT_DIR;
use colored::*;
use fs_extra::dir::CopyOptions;
use fs_extra::dir::copy;
use indoc::printdoc;
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

pub fn get_command_file(path: String) -> (String, Option<String>) {
    let yaml_path = format!("{}.yaml", path);
    let yml_path = format!("{}.yml", path);

    let commands_folder = get_commands_folder();
    let full_yaml_path = Path::new(&commands_folder)
        .join(&yaml_path)
        .to_string_lossy()
        .into_owned();
    let full_yml_path = Path::new(&commands_folder)
        .join(&yml_path)
        .to_string_lossy()
        .into_owned();

    let yaml_exists = Path::new(&full_yaml_path).exists();
    let yml_exists = Path::new(&full_yml_path).exists();

    if yaml_exists && yml_exists {
        printdoc! {"
                {} {}
                  {} Both .yaml and .yml files exist for the given path.
                  {}
                  Please choose a convention and delete one of the files.
            ",
                ">".bright_black(),
                "Error:".bright_red(),
                format!("  - {}", yaml_path).bright_yellow(),
                format!("  - {}", yml_path).bright_yellow(),
        }

        process::exit(1);
    } else if yaml_exists {
        (full_yaml_path, Some(yaml_path))
    } else if yml_exists {
        (full_yml_path, Some(yml_path))
    } else {
        // By default, we'll return the .yml path
        (full_yml_path, None)
    }
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
