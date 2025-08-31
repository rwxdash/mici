use crate::cli::schemas::v1::CommandSchema;
use std::path::Path;

pub fn parse_command_file(path: &String) -> Result<CommandSchema, serde_yaml::Error> {
    let command_yaml: String = std::fs::read_to_string(Path::new(&path)).unwrap();
    let command: CommandSchema = serde_yaml::from_str(&command_yaml)?;

    return Ok(command);
}

pub fn validate_command_file(path: &String) -> Result<CommandSchema, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err(format!("Command file not found: {}", path).into());
    }

    let command_yaml = std::fs::read_to_string(Path::new(path))?;
    let command: CommandSchema = serde_yaml::from_str(&command_yaml)?;

    // Basic validation
    // TODO: Do more...
    if command.version.is_empty() {
        return Err("Command schema version cannot be empty".into());
    }

    if command.version != "1" {
        return Err("Command schema version must be 1".into());
    }

    if command.name.is_empty() {
        return Err("Command name cannot be empty".into());
    }

    Ok(command)
}
