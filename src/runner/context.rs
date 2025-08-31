use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};

use crate::cli::schemas::v1::CommandSchema;

#[derive(Debug)]
pub struct ExecutionContext<'a> {
    pub os_environment: BTreeMap<OsString, OsString>,
    pub current_directory: PathBuf,
    pub matches: &'a getopts::Matches,
    pub command: &'a CommandSchema,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(command: &'a CommandSchema, matches: &'a getopts::Matches) -> Self {
        let os_environment = std::env::vars_os().collect();
        let current_directory = std::env::current_dir().unwrap();

        Self {
            os_environment,
            current_directory,
            matches,
            command,
        }
    }
}
