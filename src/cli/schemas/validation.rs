use crate::cli::schemas::v1::*;
use crate::errors::command::{CommandError, ValidationError};
use miette::{NamedSource, SourceSpan};
use std::collections::{BTreeMap, HashSet};

pub struct SchemaValidator {
    yaml_content: String,
    source: NamedSource<String>,
    errors: Vec<ValidationError>,
}

impl SchemaValidator {
    pub fn new(yaml_content: String, filename: String) -> Self {
        let source = NamedSource::new(filename, yaml_content.clone());
        Self {
            yaml_content,
            source,
            errors: Vec::new(),
        }
    }

    pub fn validate(&mut self, schema: &CommandSchema) -> Result<(), CommandError> {
        self.validate_version(&schema.version);
        self.validate_name(&schema.name);
        self.validate_inputs(schema.inputs.as_ref());
        self.validate_steps(&schema.steps);

        if !self.errors.is_empty() {
            let error_count = self.errors.len();
            return Err(CommandError::ValidationErrors {
                src: self.source.clone(),
                errors: std::mem::take(&mut self.errors),
                error_count,
            });
        }

        Ok(())
    }

    fn validate_version(&mut self, version: &str) {
        if version != "1"
            && version != "1.0"
            && let Some(span) = self.find_field_span("version")
        {
            self.errors.push(ValidationError::VersionInvalid {
                src: self.source.clone(),
                found: version.to_string(),
                span,
            });
        }
    }

    fn validate_name(&mut self, name: &str) {
        if name.trim().is_empty()
            && let Some(span) = self.find_field_span("name")
        {
            self.errors.push(ValidationError::NameEmpty {
                src: self.source.clone(),
                span,
            });
        }
    }

    fn validate_inputs(&mut self, inputs: Option<&BTreeMap<String, CommandSchemaInput>>) {
        let Some(inputs) = inputs else { return };

        for (input_name, input) in inputs.iter() {
            self.validate_input_type(input_name, &input.r#type);
            self.validate_input_secret(input_name, &input.r#type, input.secret);
            self.validate_input_options(input_name, &input.r#type, &input.options);
        }
    }

    fn validate_input_type(&mut self, input_name: &str, input_type: &str) {
        let valid_types = ["string", "choice", "bool", "boolean"];

        if input_type.is_empty() {
            if let Some(span) = self.find_nested_field_span(&["inputs", input_name, "type"]) {
                self.errors.push(ValidationError::InputTypeEmpty {
                    src: self.source.clone(),
                    input_name: input_name.to_string(),
                    found: input_type.to_string(),
                    span,
                });
            }
        } else if !valid_types.contains(&input_type)
            && let Some(span) = self.find_nested_field_span(&["inputs", input_name, "type"])
        {
            self.errors.push(ValidationError::InputTypeInvalid {
                src: self.source.clone(),
                input_name: input_name.to_string(),
                found: input_type.to_string(),
                span,
            });
        }
    }

    fn validate_input_secret(&mut self, input_name: &str, input_type: &str, secret: bool) {
        if secret && input_type != "string" {
            let secret_span = self.find_nested_field_span(&["inputs", input_name, "secret"]);
            let type_span = self.find_nested_field_span(&["inputs", input_name, "type"]);

            if let (Some(secret_span), Some(type_span)) = (secret_span, type_span) {
                self.errors.push(ValidationError::SecretRequiresString {
                    src: self.source.clone(),
                    input_name: input_name.to_string(),
                    input_type: input_type.to_string(),
                    secret_span,
                    type_span,
                });
            }
        }
    }

    fn validate_input_options(
        &mut self,
        input_name: &str,
        input_type: &str,
        options: &Option<Vec<String>>,
    ) {
        match (input_type, options) {
            ("choice", None) => {
                if let Some(span) = self.find_nested_field_span(&["inputs", input_name, "type"]) {
                    self.errors.push(ValidationError::ChoiceRequiresOptions {
                        src: self.source.clone(),
                        input_name: input_name.to_string(),
                        span,
                    });
                }
            }
            (t, Some(_)) if !t.is_empty() && t != "choice" => {
                if let Some(span) = self.find_nested_field_span(&["inputs", input_name, "options"])
                {
                    self.errors.push(ValidationError::OptionsOnlyForChoice {
                        src: self.source.clone(),
                        input_name: input_name.to_string(),
                        input_type: t.to_string(),
                        span,
                    });
                }
            }
            _ => {}
        }
    }

    fn validate_steps(&mut self, steps: &[CommandSchemaStep]) {
        if steps.is_empty() {
            if let Some(span) = self.find_field_span("steps") {
                self.errors.push(ValidationError::StepsEmpty {
                    src: self.source.clone(),
                    span,
                });
            }
            return;
        }

        let mut seen_ids: HashSet<&str> = HashSet::new();
        let mut id_positions: Vec<(&str, usize)> = Vec::new();

        for (index, step) in steps.iter().enumerate() {
            if step.id.is_empty() {
                if let Some(span) = self.find_step_field_span(index, "id") {
                    self.errors.push(ValidationError::StepIdEmpty {
                        src: self.source.clone(),
                        index,
                        span,
                    });
                }
            } else if step.id.contains(char::is_whitespace)
                && let Some(span) = self.find_step_field_span(index, "id")
            {
                self.errors.push(ValidationError::StepIdWhitespace {
                    src: self.source.clone(),
                    step_id: step.id.clone(),
                    span,
                });
            }

            if !seen_ids.insert(&step.id) {
                if let Some((_, first_index)) = id_positions.iter().find(|(id, _)| *id == step.id) {
                    let first_span = self.find_step_field_span(*first_index, "id");
                    let second_span = self.find_step_field_span(index, "id");

                    if let (Some(first_span), Some(second_span)) = (first_span, second_span) {
                        self.errors.push(ValidationError::StepIdDuplicate {
                            src: self.source.clone(),
                            step_id: step.id.clone(),
                            first_span,
                            second_span,
                            first_index: *first_index,
                            second_index: index,
                        });
                    }
                }
            } else {
                id_positions.push((&step.id, index));
            }

            let command_span = self.find_step_field_span(index, "command");
            let script_span = self.find_step_field_span(index, "script");

            match (command_span, script_span) {
                (None, None) => {
                    if let Some(span) = self.find_step_field_span(index, "run") {
                        self.errors.push(ValidationError::StepRunMissing {
                            src: self.source.clone(),
                            step_id: step.id.clone(),
                            span,
                        });
                    }
                }
                (Some(_), Some(_)) => {
                    if let (Some(command_span), Some(script_span)) = (
                        self.find_step_field_span(index, "command"),
                        self.find_step_field_span(index, "script"),
                    ) {
                        self.errors.push(ValidationError::StepRunMutuallyExclusive {
                            src: self.source.clone(),
                            step_id: step.id.clone(),
                            command_span,
                            script_span,
                        });
                    }
                }
                _ => {
                    // Valid. Noop.
                }
            }
        }
    }

    fn find_field_span(&self, field_name: &str) -> Option<SourceSpan> {
        let pattern = format!("{}:", field_name);
        for (line_num, line) in self.yaml_content.lines().enumerate() {
            if line.trim_start().starts_with(&pattern) {
                let offset = self
                    .yaml_content
                    .lines()
                    .take(line_num)
                    .map(|l| l.len() + 1)
                    .sum::<usize>();

                if let Some(col) = line.find(&pattern) {
                    return Some((offset + col + pattern.len() - 1, 1).into());
                }
            }
        }
        None
    }

    fn find_nested_field_span(&self, path: &[&str]) -> Option<SourceSpan> {
        let mut path_index = 0;

        for (line_num, line) in self.yaml_content.lines().enumerate() {
            let trimmed = line.trim_start();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if path_index < path.len() {
                let pattern = format!("{}:", path[path_index]);
                if trimmed.starts_with(&pattern) {
                    if path_index == path.len() - 1 {
                        let offset = self
                            .yaml_content
                            .lines()
                            .take(line_num)
                            .map(|l| l.len() + 1)
                            .sum::<usize>();
                        if let Some(col) = line.find(&pattern) {
                            return Some((offset + col + pattern.len() - 1, 1).into());
                        }
                    } else {
                        path_index += 1;
                    }
                }
            }
        }

        None
    }

    fn find_step_field_span(&self, step_index: usize, field_name: &str) -> Option<SourceSpan> {
        let mut in_steps_block = false;
        let mut steps_indent: Option<usize> = None;

        let mut current_step = 0usize;
        let mut target_start_line: Option<usize> = None;
        let mut target_end_line: Option<usize> = None;

        let lines: Vec<&str> = self.yaml_content.lines().collect();

        for (i, raw_line) in lines.iter().enumerate() {
            let line = *raw_line;
            let trimmed = line.trim_start();

            if !in_steps_block {
                if trimmed.starts_with("steps:") {
                    in_steps_block = true;
                    steps_indent = Some(line.len() - trimmed.len());
                    continue;
                }
            } else {
                let indent = line.len() - trimmed.len();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                } else if let Some(si) = steps_indent
                    && indent <= si
                    && !trimmed.starts_with('-')
                {
                    break;
                }

                if trimmed.starts_with('-') {
                    let indent = line.len() - trimmed.len();
                    if let Some(si) = steps_indent
                        && indent >= si
                    {
                        if current_step == step_index {
                            target_start_line = Some(i);
                        } else if target_start_line.is_some() && target_end_line.is_none() {
                            target_end_line = Some(i - 1);
                            break;
                        }

                        current_step += 1;
                    }
                }
            }
        }

        if target_start_line.is_some() && target_end_line.is_none() {
            target_end_line = Some(lines.len() - 1);
        }

        let (start, end) = match (target_start_line, target_end_line) {
            (Some(s), Some(e)) => (s, e),
            _ => return None,
        };

        let name_with_colon = format!("{}:", field_name);
        let name_with_dash = format!("- {}:", field_name);

        for line_num in start..=end {
            let line = lines[line_num];
            let trimmed = line.trim_start();

            if trimmed.starts_with(&name_with_dash) || trimmed.starts_with(&name_with_colon) {
                let offset = lines
                    .iter()
                    .take(line_num)
                    .map(|l| l.len() + 1)
                    .sum::<usize>();
                if let Some(col) = line.find(field_name) {
                    return Some((offset + col + field_name.len(), 1).into());
                }
            }

            if trimmed.starts_with(&format!("{}:", field_name)) {
                let offset = lines
                    .iter()
                    .take(line_num)
                    .map(|l| l.len() + 1)
                    .sum::<usize>();
                if let Some(col) = line.find(&format!("{}:", field_name)) {
                    return Some((offset + col + field_name.len(), 1).into());
                }
            }
        }

        None
    }
}
