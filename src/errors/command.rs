#![allow(unused_assignments)]

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum CommandError {
    #[error("Failed to parse YAML file")]
    #[diagnostic(
        code(mici::yaml::invalid_syntax),
        help(
            "Check your YAML syntax - common issues include incorrect indentation, missing colons, or unclosed quotes"
        )
    )]
    YamlSyntaxError {
        #[source_code]
        src: NamedSource<String>,
        #[label("syntax error here")]
        span: SourceSpan,
        #[source]
        err: serde_yaml::Error,
    },

    #[error("Command schema has {error_count} validation error(s)")]
    #[diagnostic(code(mici::schema::validation_errors))]
    ValidationErrors {
        #[source_code]
        src: NamedSource<String>,

        #[related]
        errors: Vec<ValidationError>,

        error_count: usize,
    },

    #[error("Cannot find command file at '{path}'")]
    #[diagnostic(
        code(mici::io::not_found),
        help("Check if the file exists or create one using 'mici new'")
    )]
    FileNotFound {
        path: String,
        #[source]
        err: std::io::Error,
    },

    #[error("Permission denied reading command file at '{path}'")]
    #[diagnostic(
        code(mici::io::permission_denied),
        help("Check the file permissions and ensure you have read access")
    )]
    FilePermissionDenied {
        path: String,
        #[source]
        err: std::io::Error,
    },

    #[error("Failed to read command file at '{path}'")]
    #[diagnostic(
        code(mici::io::read_error),
        help("Check that the file is accessible and not corrupted")
    )]
    FileReadError {
        path: String,
        #[source]
        err: std::io::Error,
    },
}

#[derive(Error, Debug, Diagnostic)]
pub enum ValidationError {
    #[error("Version must be '1' or '1.0', found '{found}'")]
    #[diagnostic(
        code(mici::schema::version_invalid),
        help("Set 'version' to '\"1\"' or '\"1.0\"' in your command schema")
    )]
    VersionInvalid {
        #[source_code]
        src: NamedSource<String>,

        found: String,
        #[label("invalid version here")]
        span: SourceSpan,
    },

    #[error("Command name cannot be empty")]
    #[diagnostic(
        code(mici::schema::name_empty),
        help("Add a name with at least one character")
    )]
    NameEmpty {
        #[source_code]
        src: NamedSource<String>,

        #[label("empty name here")]
        span: SourceSpan,
    },

    #[error("Input '{input_name}' has empty type '{found}'")]
    #[diagnostic(
        code(mici::schema::input_type_empty),
        help("Valid input types are: string, choice, and bool or boolean")
    )]
    InputTypeEmpty {
        #[source_code]
        src: NamedSource<String>,

        input_name: String,
        found: String,

        #[label("empty type here")]
        span: SourceSpan,
    },

    #[error("Input '{input_name}' has invalid type '{found}'")]
    #[diagnostic(
        code(mici::schema::input_type_invalid),
        help("Valid input types are: string, choice, and bool or boolean")
    )]
    InputTypeInvalid {
        #[source_code]
        src: NamedSource<String>,

        input_name: String,
        found: String,

        #[label("invalid type here")]
        span: SourceSpan,
    },

    #[error(
        "Input '{input_name}' has 'secret' set to true but type '{input_type}' doesn't allow it"
    )]
    #[diagnostic(
        code(mici::schema::secret_requires_string),
        help(
            "Only 'string' inputs can be marked as secret. Change the type to 'string' or remove 'secret: true'"
        )
    )]
    SecretRequiresString {
        #[source_code]
        src: NamedSource<String>,

        input_name: String,
        input_type: String,

        #[label("secret is set to true")]
        secret_span: SourceSpan,

        #[label("type is '{input_type}'")]
        type_span: SourceSpan,
    },

    #[error("Input '{input_name}' has type 'choice' but no 'options' provided")]
    #[diagnostic(
        code(mici::schema::choice_requires_options),
        help(
            "Add an 'options' array with available choices, e.g., 'options: [dev, staging, prod]'"
        )
    )]
    ChoiceRequiresOptions {
        #[source_code]
        src: NamedSource<String>,

        input_name: String,

        #[label("'choice' type requires 'options'")]
        span: SourceSpan,
    },

    #[error(
        "Input '{input_name}' has type '{input_type}' but 'options' are only valid for type 'choice'"
    )]
    #[diagnostic(
        code(mici::schema::options_only_for_choice),
        help("Remove the 'options' field or change the type to 'choice'")
    )]
    OptionsOnlyForChoice {
        #[source_code]
        src: NamedSource<String>,

        input_name: String,
        input_type: String,

        #[label("'options' should not be set for type '{input_type}'")]
        span: SourceSpan,
    },

    #[error("Steps cannot be empty - at least one step is required")]
    #[diagnostic(
        code(mici::schema::steps_empty),
        help("Add at least one step to the 'steps' array")
    )]
    StepsEmpty {
        #[source_code]
        src: NamedSource<String>,

        #[label("steps array is empty")]
        span: SourceSpan,
    },

    #[error("Step #{index} is missing an 'id' field")]
    #[diagnostic(
        code(mici::schema::step_id_missing),
        help("Add a valid 'id' field to identify this step")
    )]
    StepIdMissing {
        #[source_code]
        src: NamedSource<String>,

        index: usize,

        #[label("step missing id")]
        span: SourceSpan,
    },

    #[error("Step '{step_id}' has an id with whitespace")]
    #[diagnostic(
        code(mici::schema::step_id_whitespace),
        help(
            "Remove spaces from the step id - use hyphens or underscores instead (e.g., 'build-app' or 'build_app')"
        )
    )]
    StepIdWhitespace {
        #[source_code]
        src: NamedSource<String>,

        step_id: String,

        #[label("id contains whitespace")]
        span: SourceSpan,
    },

    #[error("Step id cannot be empty")]
    #[diagnostic(
        code(mici::schema::step_id_empty),
        help("Provide a meaningful id for this step")
    )]
    StepIdEmpty {
        #[source_code]
        src: NamedSource<String>,

        index: usize,

        #[label("empty id")]
        span: SourceSpan,
    },

    #[error("Duplicate step id for '{step_id}'")]
    #[diagnostic(
        code(mici::schema::step_id_duplicate),
        help("Each step must have a unique id")
    )]
    StepIdDuplicate {
        #[source_code]
        src: NamedSource<String>,

        step_id: String,

        #[label("first occurrence here")]
        first_span: SourceSpan,

        #[label("duplicate id occurred here")]
        second_span: SourceSpan,

        first_index: usize,
        second_index: usize,
    },

    #[error("Step '{step_id}' is missing a 'run' field")]
    #[diagnostic(
        code(mici::schema::step_run_missing),
        help("Add a 'run' field with either 'command' or 'script'")
    )]
    StepRunMissing {
        #[source_code]
        src: NamedSource<String>,

        step_id: String,

        #[label("run field required")]
        span: SourceSpan,
    },

    #[error(
        "Step '{step_id}' has both 'command' and 'script' in its 'run' - they are mutually exclusive"
    )]
    #[diagnostic(
        code(mici::schema::step_run_mutually_exclusive),
        help("Only one of 'command' or 'script' may be present in a step's 'run' block")
    )]
    StepRunMutuallyExclusive {
        #[source_code]
        src: NamedSource<String>,

        step_id: String,

        #[label("'command' is set here")]
        command_span: SourceSpan,

        #[label("'script' is set here")]
        script_span: SourceSpan,
    },
}
