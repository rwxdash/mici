use crate::utils::traits::ExportAsHashMap;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Enums
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CommandSchemaStepRunArgsConfig {
    // Array format: ["environment", "service", "version"]
    List(Vec<String>),
    // Object format: { "target_env": "${inputs.environment}", ... }
    Map(HashMap<String, String>),
}

// Structs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchema {
    pub version: String,
    pub name: String,
    pub description: Option<String>,
    pub usage: Option<String>,
    pub inputs: Option<HashMap<String, CommandSchemaInput>>,
    pub configuration: CommandSchemaConfiguration,
    pub steps: Vec<CommandSchemaStep>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaInput {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub options: Option<Vec<String>>,
    #[serde(default = "default_schema_input_required")]
    pub required: Option<bool>,
    pub short: Option<String>,
    pub long: Option<String>,
    pub default: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaConfiguration {
    #[serde(default = "default_schema_configuration_confirm")]
    pub confirm: Option<bool>,
    pub environment: Option<HashMap<String, Option<String>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaStep {
    pub name: String,
    pub when: Option<String>,
    pub run: CommandSchemaStepRun,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaStepRun {
    pub shell: String,
    #[serde(default = "default_schema_step_run_always")]
    pub always: Option<bool>,
    pub environment: Option<HashMap<String, Option<String>>>,
    pub command: Option<String>,
    pub script: Option<String>,
    pub args: Option<CommandSchemaStepRunArgsConfig>,
}

// Traits
impl ExportAsHashMap for CommandSchema {
    fn as_hash_map(&self) -> HashMap<&str, &str> {
        let mut content: HashMap<&str, &str> = HashMap::new();

        content.insert("name", self.name.trim());
        content.insert("description", self.description.as_ref().unwrap().trim());
        content.insert("usage", self.usage.as_ref().unwrap().trim());

        return content;
    }
}

// Default Function
fn default_schema_configuration_confirm() -> Option<bool> {
    return Some(false);
}

fn default_schema_input_required() -> Option<bool> {
    return Some(false);
}

fn default_schema_step_run_always() -> Option<bool> {
    return Some(false);
}
