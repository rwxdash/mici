use crate::PROJECT_DIR;
use fs_extra::dir::CopyOptions;
use fs_extra::dir::copy;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_home_dir() -> PathBuf {
    if let Ok(mici_home) = env::var("MICI_HOME") {
        return PathBuf::from(mici_home);
    }
    dirs::home_dir().expect("Could not determine home directory")
}

pub fn get_project_folder() -> PathBuf {
    get_home_dir().join(PROJECT_DIR)
}

pub fn get_config_file() -> PathBuf {
    get_home_dir().join(PROJECT_DIR).join("config.yml")
}

pub fn get_jobs_folder() -> PathBuf {
    get_home_dir().join(PROJECT_DIR).join("jobs")
}

pub fn get_commands_folder() -> PathBuf {
    get_home_dir()
        .join(PROJECT_DIR)
        .join("jobs")
        .join("commands")
}

pub fn get_scripts_folder() -> PathBuf {
    get_home_dir()
        .join(PROJECT_DIR)
        .join("jobs")
        .join("scripts")
}

pub fn get_command_file(path: String) -> Result<(PathBuf, Option<String>), String> {
    let yaml_path = format!("{}.yaml", path);
    let yml_path = format!("{}.yml", path);

    let commands_folder = get_commands_folder();
    let full_yaml_path = commands_folder.join(&yaml_path);
    let full_yml_path = commands_folder.join(&yml_path);

    let yaml_exists = full_yaml_path.exists();
    let yml_exists = full_yml_path.exists();

    if yaml_exists && yml_exists {
        Err(format!(
            "Both .yaml and .yml files exist for the given path.\n  - {}\n  - {}\nPlease choose a convention and delete one of the files.",
            yaml_path, yml_path,
        ))
    } else if yaml_exists {
        Ok((full_yaml_path, Some(yaml_path)))
    } else if yml_exists {
        Ok((full_yml_path, Some(yml_path)))
    } else {
        // By default, we'll return the .yml path
        Ok((full_yml_path, None))
    }
}

pub fn clear_jobs_folder() -> Result<(), std::io::Error> {
    fs::remove_dir_all(get_jobs_folder())
}

pub fn create_folder_at(path: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
}

pub fn create_tmp_folder() -> Result<PathBuf, std::io::Error> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let path = env::temp_dir().join(format!("mici-fetch-{}", timestamp));
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn copy_directory(from: &Path, to: &Path) -> Result<u64, fs_extra::error::Error> {
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
