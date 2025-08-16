use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};

use crate::cli::schemas::v1::{CommandSchema, CommandSchemaConfiguration, CommandSchemaInput};

pub struct ExecutionContext<'a> {
    pub inputs: &'a BTreeMap<String, CommandSchemaInput>,
    pub os_environment: BTreeMap<OsString, OsString>,
    pub current_directory: PathBuf,
    pub configuration: &'a CommandSchemaConfiguration,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(command: &'a CommandSchema, matches: &getopts::Matches) -> Self {
        let os_environment = std::env::vars_os().collect();
        let current_directory = std::env::current_dir().expect("Failed to get working directory");

        let inputs = command.inputs.as_ref().expect("Inputs must be present");

        Self {
            inputs,
            os_environment,
            current_directory,
            configuration: &command.configuration,
        }
    }
}
