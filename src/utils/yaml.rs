use crate::cli::schemas::{v1::CommandSchema, validation::SchemaValidator};
use crate::errors::command::CommandError;
use miette::NamedSource;
use std::path::Path;

pub fn parse_command_file(path: &String) -> Result<CommandSchema, CommandError> {
    let yaml_content = std::fs::read_to_string(Path::new(&path))
        .map_err(|_| CommandError::FileNotFound { path: path.clone() })?;

    let schema: CommandSchema = serde_yaml::from_str(&yaml_content).map_err(|err| {
        let span = if let Some(location) = err.location() {
            let line = location.line();
            let column = location.column();

            let offset = yaml_content
                .lines()
                .take(line.saturating_sub(1))
                .map(|l| l.len() + 1)
                .sum::<usize>()
                + column;

            let length = 1;

            (offset, length).into()
        } else {
            (0, 1).into()
        };

        CommandError::YamlSyntaxError {
            src: NamedSource::new(path.clone(), yaml_content.clone()),
            span,
            err,
        }
    })?;

    let mut validator = SchemaValidator::new(yaml_content, path.clone());
    validator.validate(&schema)?;

    Ok(schema)
}
