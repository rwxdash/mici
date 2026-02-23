use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Create a temporary .mici directory structure with a config file and optional command files.
pub fn setup_mici_home(commands: &[(&str, &str)]) -> TempDir {
    let tmp = TempDir::new().expect("Failed to create temp dir");
    let mici_dir = tmp.path().join(".mici");
    let commands_dir = mici_dir.join("jobs").join("commands");
    let scripts_dir = mici_dir.join("jobs").join("scripts");

    fs::create_dir_all(&commands_dir).unwrap();
    fs::create_dir_all(&scripts_dir).unwrap();

    // Write a minimal config
    let config_path = mici_dir.join("config.yml");
    fs::write(
        &config_path,
        "upstream_url: null\nupstream_cmd_path: null\ndisable_cli_color: false\ndisable_pager: true\n",
    )
    .unwrap();

    // Write command files
    for (name, content) in commands {
        let cmd_path = commands_dir.join(name);
        if let Some(parent) = cmd_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&cmd_path, content).unwrap();
    }

    tmp
}

/// Create a temporary .mici directory with command files and script files.
pub fn setup_mici_home_with_scripts(
    commands: &[(&str, &str)],
    scripts: &[(&str, &str)],
) -> TempDir {
    let tmp = setup_mici_home(commands);
    let scripts_dir = tmp.path().join(".mici").join("jobs").join("scripts");

    for (name, content) in scripts {
        let script_path = scripts_dir.join(name);
        if let Some(parent) = script_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&script_path, content).unwrap();
    }

    tmp
}

/// Read a fixture file from tests/fixtures/
pub fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read fixture: {}", name))
}
