use crate::utils::traits::ExportAsHashMap;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchema {
    pub version: Option<String>,
    pub name: String,
    pub description: String,
    pub usage: String,
    pub configuration: CommandSchemaConfiguration,
    pub steps: Vec<CommandSchemaStep>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaConfiguration {
    #[serde(default = "default_schema_configuration_confirm")]
    pub confirm: Option<bool>,
    #[serde(default = "default_schema_configuration_orderly")]
    pub orderly: Option<bool>,
    pub environment: Option<HashMap<String, Option<String>>>,
    pub options: Option<Vec<CommandSchemaOption>>,
    pub group: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaOption {
    pub long: String,
    pub short: Option<String>,
    #[serde(default = "default_schema_option_required")]
    pub required: Option<bool>,
    #[serde(default = "default_schema_option_flag")]
    pub flag: Option<bool>,
    pub default: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaStep {
    pub name: String,
    pub run: CommandSchemaStepRun,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaStepRun {
    pub shell: String,
    #[serde(default = "default_schema_step_run_always")]
    pub always: Option<bool>,
    pub environment: Option<HashMap<String, Option<String>>>,
    pub command: String,
}

impl ExportAsHashMap for CommandSchema {
    fn as_hash_map(&self) -> HashMap<&str, &str> {
        let mut content: HashMap<&str, &str> = HashMap::new();

        content.insert("name", self.name.trim());
        content.insert("description", self.description.trim());
        content.insert("usage", self.usage.trim());

        return content;
    }
}

fn default_schema_configuration_confirm() -> Option<bool> {
    return Some(false);
}

fn default_schema_configuration_orderly() -> Option<bool> {
    return Some(true);
}

fn default_schema_option_required() -> Option<bool> {
    return Some(false);
}

fn default_schema_option_flag() -> Option<bool> {
    return Some(false);
}

fn default_schema_step_run_always() -> Option<bool> {
    return Some(false);
}
