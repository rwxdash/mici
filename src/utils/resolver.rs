use regex::Regex;
use std::{collections::BTreeMap, sync::OnceLock};

use crate::cli::schemas::v1::CommandSchemaInput;

const MAX_ITERATIONS: usize = 10;

static INPUTS_RE: OnceLock<Regex> = OnceLock::new();
static ENV_RE: OnceLock<Regex> = OnceLock::new();

pub fn resolve_environment_variables(
    environment: &BTreeMap<String, Option<String>>,
    inputs: &BTreeMap<String, CommandSchemaInput>,
    matches: &getopts::Matches,
) -> BTreeMap<String, String> {
    use regex::Regex;

    let inputs_re =
        INPUTS_RE.get_or_init(|| Regex::new(r"@\{inputs\.([a-zA-Z_][a-zA-Z0-9_]*)\}").unwrap());
    let env_re = ENV_RE.get_or_init(|| Regex::new(r"\$\{([A-Z_][A-Z0-9_]*)\}").unwrap());

    let mut resolved: BTreeMap<String, String> = BTreeMap::new();
    let mut pending: BTreeMap<String, String> = BTreeMap::new();

    // Separate variables that need substitution from those that don't
    for (key, value) in environment {
        if let Some(val) = value {
            if val.contains("${") || val.contains("@{") {
                // Has substitution patterns - add to pending
                pending.insert(key.clone(), val.clone());
            } else {
                // No substitution needed - directly resolved
                resolved.insert(key.clone(), val.clone());
            }
        }
    }

    let mut iterations = 0;

    while !pending.is_empty() && iterations < MAX_ITERATIONS {
        let mut progress_made = false;
        let mut keys_to_remove = Vec::new();

        for (key, value) in &pending {
            let mut result = value.clone();
            let mut variable_resolved = true;

            // Handle @{inputs.xxx}
            // Skip if iteration is > 0 since `inputs` will resolve on first go
            if iterations == 0 {
                result = inputs_re
                    .replace_all(&result, |caps: &regex::Captures| {
                        let variable_name = &caps[1];

                        if let Some(input) = inputs.get(variable_name) {
                            let resolved_value = match input.r#type.as_str() {
                                "boolean" | "bool" => {
                                    if matches.opt_present(variable_name) {
                                        "true".to_string()
                                    } else {
                                        input
                                            .default
                                            .as_ref()
                                            .unwrap_or(&"false".to_string())
                                            .clone()
                                    }
                                }
                                _ => matches
                                    .opt_str(variable_name)
                                    .or_else(|| input.default.clone())
                                    .unwrap_or_else(|| "".to_string()),
                            };

                            resolved_value
                        } else {
                            "".to_string()
                        }
                    })
                    .to_string();
            }

            // Handle ${ENV_VAR}
            result = env_re
                .replace_all(&result, |caps: &regex::Captures| {
                    let variable_name = &caps[1];

                    if let Some(resolved_val) = resolved.get(variable_name) {
                        resolved_val.clone()
                    } else if pending.contains_key(variable_name) {
                        // Still pending
                        variable_resolved = false;
                        caps[0].to_string() // Keep ${VAR} unchanged
                    } else {
                        std::env::var(variable_name).unwrap_or_default()
                    }
                })
                .to_string();

            if variable_resolved || !result.contains("${") {
                resolved.insert(key.clone(), result);
                keys_to_remove.push(key.clone());

                progress_made = true
            }
        }

        for key in keys_to_remove {
            pending.remove(&key);
        }

        if !progress_made {
            // No progress made - likely circular references
            // Just resolve remaining with OS env vars
            for (key, value) in pending {
                let result = env_re
                    .replace_all(&value, |caps: &regex::Captures| {
                        std::env::var(&caps[1]).unwrap_or_default()
                    })
                    .to_string();
                resolved.insert(key, result);
            }
            break;
        }

        iterations += 1;
    }

    resolved.into_iter().collect()
}

pub fn resolve_input_variables(
    text: &str,
    inputs: &BTreeMap<String, CommandSchemaInput>,
    matches: &getopts::Matches,
) -> String {
    let inputs_re =
        INPUTS_RE.get_or_init(|| Regex::new(r"@\{inputs\.([a-zA-Z_][a-zA-Z0-9_]*)\}").unwrap());

    inputs_re
        .replace_all(text, |caps: &regex::Captures| {
            let variable_name = &caps[1];

            if let Some(input) = inputs.get(variable_name) {
                match input.r#type.as_str() {
                    "boolean" | "bool" => {
                        if matches.opt_present(variable_name) {
                            "true".to_string()
                        } else {
                            input
                                .default
                                .as_ref()
                                .unwrap_or(&"false".to_string())
                                .clone()
                        }
                    }
                    _ => matches
                        .opt_str(variable_name)
                        .or_else(|| input.default.clone())
                        .unwrap_or_else(|| "".to_string()),
                }
            } else {
                "".to_string()
            }
        })
        .to_string()
}
