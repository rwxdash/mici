use crate::errors::command::CommandError;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum CliError {
    #[error("{message}")]
    General { message: String },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Command(#[from] CommandError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),

    #[error("Argument error: {0}")]
    ArgParse(String),

    #[error("Step '{step_id}' failed with exit code: {exit_code}")]
    StepFailed { step_id: String, exit_code: i32 },
}

impl From<String> for CliError {
    fn from(s: String) -> Self {
        CliError::General { message: s }
    }
}

impl From<&str> for CliError {
    fn from(s: &str) -> Self {
        CliError::General {
            message: s.to_string(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for CliError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        CliError::General {
            message: e.to_string(),
        }
    }
}
