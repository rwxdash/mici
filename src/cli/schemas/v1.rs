use crate::errors::command::CommandError;
use crate::utils::traits::ExportAsHashMap;

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;

// Enums
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CommandSchemaStepRunArgsConfig {
    // Array format: ["environment", "service", "version"]
    List(Vec<String>),
    // Object format: { "target_env": "@{inputs.environment}", ... }
    Map(BTreeMap<String, String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandSchemaStepRunExecution {
    Command { command: String },
    Script { script: String },
}

impl CommandSchemaStepRunExecution {
    pub fn is_command(&self) -> bool {
        matches!(self, CommandSchemaStepRunExecution::Command { .. })
    }

    pub fn is_script(&self) -> bool {
        matches!(self, CommandSchemaStepRunExecution::Script { .. })
    }

    pub fn get_command(&self) -> Option<&String> {
        if let CommandSchemaStepRunExecution::Command { command } = self {
            Some(command)
        } else {
            None
        }
    }

    pub fn get_script(&self) -> Option<&String> {
        if let CommandSchemaStepRunExecution::Script { script } = self {
            Some(script)
        } else {
            None
        }
    }
}

// Structs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchema {
    pub version: String,
    pub name: String,
    pub description: Option<String>,
    pub usage: Option<String>,
    pub inputs: Option<BTreeMap<String, CommandSchemaInput>>,
    pub configuration: CommandSchemaConfiguration,
    pub steps: Vec<CommandSchemaStep>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaInput {
    #[serde(rename = "type")]
    pub r#type: String,
    pub description: String,
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub secret: bool,
    pub short: Option<String>,
    pub long: Option<String>,
    pub default: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaConfiguration {
    #[serde(default)]
    pub confirm: bool,
    pub environment: Option<BTreeMap<String, Option<String>>>,
    pub working_directory: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaStep {
    pub id: String,
    pub name: Option<String>,
    pub when: Option<String>,
    pub run: CommandSchemaStepRun,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSchemaStepRun {
    #[serde(default = "default_schema_step_run_shell")]
    pub shell: Option<String>,
    pub environment: Option<BTreeMap<String, Option<String>>>,
    #[serde(flatten)]
    pub execution: CommandSchemaStepRunExecution,
    pub args: Option<CommandSchemaStepRunArgsConfig>,
    pub working_directory: Option<String>,
}

impl CommandSchema {
    pub fn inputs_or_empty(&self) -> &BTreeMap<String, CommandSchemaInput> {
        static EMPTY: OnceLock<BTreeMap<String, CommandSchemaInput>> = OnceLock::new();
        self.inputs
            .as_ref()
            .unwrap_or_else(|| EMPTY.get_or_init(BTreeMap::new))
    }
}

// Traits
impl ExportAsHashMap for CommandSchema {
    fn as_hash_map(&self) -> HashMap<&str, &str> {
        let mut content: HashMap<&str, &str> = HashMap::new();

        content.insert("name", self.name.trim());
        content.insert(
            "description",
            self.description.as_deref().unwrap_or("").trim(),
        );
        content.insert("usage", self.usage.as_deref().unwrap_or("").trim());

        content
    }
}

// Validation
/// Validates parsed CLI inputs against their schema definitions.
/// Checks: required inputs have values (or defaults), choice inputs match allowed options.
pub fn validate_inputs(
    inputs: &BTreeMap<String, CommandSchemaInput>,
    matches: &getopts::Matches,
) -> Result<(), CommandError> {
    for (name, input) in inputs {
        match input.r#type.as_str() {
            "boolean" | "bool" => continue,
            "choice" => {
                let value = matches.opt_str(name).or_else(|| input.default.clone());

                if input.required && value.is_none() {
                    return Err(CommandError::InputRequired {
                        input_name: name.clone(),
                    });
                }

                if let Some(ref val) = value
                    && let Some(ref options) = input.options
                    && !options.contains(val)
                {
                    return Err(CommandError::InputInvalidChoice {
                        input_name: name.clone(),
                        provided: val.clone(),
                        expected: options.join(", "),
                    });
                }
            }
            _ => {
                let value = matches.opt_str(name).or_else(|| input.default.clone());

                if input.required && value.is_none() {
                    return Err(CommandError::InputRequired {
                        input_name: name.clone(),
                    });
                }
            }
        }
    }
    Ok(())
}

// Default Functions
fn default_schema_step_run_shell() -> Option<String> {
    None
}
