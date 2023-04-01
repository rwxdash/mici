use crate::cli::schemas::v1::CommandSchema;
use std::path::Path;

pub fn parse_command_file(path: &String) -> Result<CommandSchema, serde_yaml::Error> {
    let command_yaml: String = std::fs::read_to_string(Path::new(&path)).unwrap();
    let command: CommandSchema = serde_yaml::from_str(&command_yaml)?;

    return Ok(command);
}
