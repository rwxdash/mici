mod common;

use assert_cmd::cargo::cargo_bin_cmd;
use common::{fixture, setup_mici_home};
use predicates::prelude::*;

fn mici() -> assert_cmd::Command {
    cargo_bin_cmd!("mici")
}

// ─── Validation: success ───

#[test]
fn validate_valid_command() {
    let tmp = setup_mici_home(&[("hello.yml", &fixture("valid_command.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "hello"])
        .assert()
        .success();
}

#[test]
fn validate_minimal_command() {
    let tmp = setup_mici_home(&[("minimal.yml", &fixture("minimal_command.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "minimal"])
        .assert()
        .success();
}

#[test]
fn validate_no_inputs() {
    let tmp = setup_mici_home(&[("no-inputs.yml", &fixture("valid_no_inputs.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "no-inputs"])
        .assert()
        .success();
}

#[test]
fn validate_multi_step() {
    let tmp = setup_mici_home(&[("multi.yml", &fixture("valid_multi_step.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "multi"])
        .assert()
        .success();
}

#[test]
fn validate_choice_input() {
    let tmp = setup_mici_home(&[("choice.yml", &fixture("valid_choice_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "choice"])
        .assert()
        .success();
}

#[test]
fn validate_bool_input() {
    let tmp = setup_mici_home(&[("booltest.yml", &fixture("valid_bool_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "booltest"])
        .assert()
        .success();
}

#[test]
fn validate_env_vars() {
    let tmp = setup_mici_home(&[("env-vars.yml", &fixture("valid_env_vars.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "env-vars"])
        .assert()
        .success();
}

#[test]
fn validate_input_resolution() {
    let tmp = setup_mici_home(&[("input-res.yml", &fixture("valid_input_resolution.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "input-res"])
        .assert()
        .success();
}

// ─── Validation: failures ───

#[test]
fn validate_invalid_version_name_steps() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_command.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("version_invalid"))
        .stderr(predicate::str::contains("name_empty"))
        .stderr(predicate::str::contains("steps_empty"));
}

#[test]
fn validate_invalid_input_type() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_input_type.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("input_type_invalid"));
}

#[test]
fn validate_invalid_empty_type() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_empty_type.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("input_type_empty"));
}

#[test]
fn validate_invalid_secret_on_bool() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_secret_on_bool.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("secret_requires_string"));
}

#[test]
fn validate_invalid_choice_no_options() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_choice_no_options.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("choice_requires_options"));
}

#[test]
fn validate_invalid_options_on_string() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_options_on_string.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("options_only_for_choice"));
}

#[test]
fn validate_invalid_step_empty_id() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_step_no_id.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("step_id_empty"));
}

#[test]
fn validate_invalid_step_whitespace_id() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_step_whitespace_id.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("step_id_whitespace"));
}

#[test]
fn validate_invalid_duplicate_step_ids() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_duplicate_step_ids.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("step_id_duplicate"));
}

#[test]
fn validate_invalid_step_no_run() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_step_no_run.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid_syntax"));
}

#[test]
fn validate_invalid_yaml_syntax() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_yaml_syntax.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid_syntax"));
}

#[test]
fn validate_invalid_step_command_and_script() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_step_command_and_script.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure();
}

#[test]
fn validate_invalid_multiple_errors() {
    let tmp = setup_mici_home(&[("bad.yml", &fixture("invalid_multiple_errors.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("input_type_invalid"))
        .stderr(predicate::str::contains("choice_requires_options"))
        .stderr(predicate::str::contains("step_id_whitespace"))
        .stderr(predicate::str::contains("step_id_duplicate"));
}

// ─── List ───

#[test]
fn list_shows_commands() {
    let tmp = setup_mici_home(&[("hello.yml", &fixture("valid_command.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn list_shows_multiple_commands() {
    let tmp = setup_mici_home(&[
        ("hello.yml", &fixture("valid_command.yml")),
        ("minimal.yml", &fixture("minimal_command.yml")),
    ]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"))
        .stdout(predicate::str::contains("minimal"));
}

// ─── Run: success ───

#[test]
fn run_simple_command() {
    let tmp = setup_mici_home(&[("minimal.yml", &fixture("minimal_command.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("minimal")
        .assert()
        .success()
        .stdout(predicate::str::contains("ok"));
}

#[test]
fn run_command_with_input() {
    let tmp = setup_mici_home(&[("hello.yml", &fixture("valid_command.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["hello", "--name", "Rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Rust!"));
}

#[test]
fn run_command_with_default_input() {
    let tmp = setup_mici_home(&[("hello.yml", &fixture("valid_command.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn run_multi_step() {
    let tmp = setup_mici_home(&[("multi.yml", &fixture("valid_multi_step.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("multi")
        .assert()
        .success()
        .stdout(predicate::str::contains("step-1"))
        .stdout(predicate::str::contains("step-2"))
        .stdout(predicate::str::contains("step-3"));
}

// Env var and input resolution tests use shell-specific syntax:
//   Unix (bash):       echo $VAR
//   Windows (PowerShell): echo $env:VAR

#[cfg(unix)]
#[test]
fn run_env_vars() {
    let tmp = setup_mici_home(&[("env-vars.yml", &fixture("valid_env_vars.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("env-vars")
        .assert()
        .success()
        .stdout(predicate::str::contains("global-value"))
        .stdout(predicate::str::contains("step-override"));
}

#[cfg(windows)]
#[test]
fn run_env_vars() {
    let tmp = setup_mici_home(&[("env-vars.yml", &fixture("valid_env_vars_windows.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("env-vars")
        .assert()
        .success()
        .stdout(predicate::str::contains("global-value"))
        .stdout(predicate::str::contains("step-override"));
}

#[cfg(unix)]
#[test]
fn run_input_resolution_with_args() {
    let tmp = setup_mici_home(&[("input-res.yml", &fixture("valid_input_resolution.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["input-res", "--greeting", "Hi", "--target", "Earth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hi, Earth!"));
}

#[cfg(windows)]
#[test]
fn run_input_resolution_with_args() {
    let tmp = setup_mici_home(&[(
        "input-res.yml",
        &fixture("valid_input_resolution_windows.yml"),
    )]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["input-res", "--greeting", "Hi", "--target", "Earth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hi, Earth!"));
}

#[cfg(unix)]
#[test]
fn run_input_resolution_defaults() {
    let tmp = setup_mici_home(&[("input-res.yml", &fixture("valid_input_resolution.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("input-res")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[cfg(windows)]
#[test]
fn run_input_resolution_defaults() {
    let tmp = setup_mici_home(&[(
        "input-res.yml",
        &fixture("valid_input_resolution_windows.yml"),
    )]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("input-res")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn run_choice_input_with_value() {
    let tmp = setup_mici_home(&[("choice.yml", &fixture("valid_choice_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["choice", "--env", "staging"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deploying to staging"));
}

#[test]
fn run_choice_input_default() {
    let tmp = setup_mici_home(&[("choice.yml", &fixture("valid_choice_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("choice")
        .assert()
        .success()
        .stdout(predicate::str::contains("deploying to production"));
}

#[test]
fn run_bool_input_present() {
    let tmp = setup_mici_home(&[("booltest.yml", &fixture("valid_bool_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["booltest", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)dry_run=true").unwrap());
}

#[test]
fn run_bool_input_absent() {
    let tmp = setup_mici_home(&[("booltest.yml", &fixture("valid_bool_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("booltest")
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)dry_run=false").unwrap());
}

#[test]
fn run_no_inputs_command() {
    let tmp = setup_mici_home(&[("no-inputs.yml", &fixture("valid_no_inputs.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("no-inputs")
        .assert()
        .success()
        .stdout(predicate::str::contains("no inputs needed"));
}

// ─── Run: input validation ───

#[test]
fn run_choice_rejects_invalid_value() {
    let tmp = setup_mici_home(&[("choice.yml", &fixture("valid_choice_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["choice", "--env", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a valid option"))
        .stderr(predicate::str::contains("staging, production"));
}

#[test]
fn run_required_input_missing() {
    let tmp = setup_mici_home(&[("required.yml", &fixture("valid_required_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("required")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not provided"));
}

#[test]
fn run_required_input_provided() {
    let tmp = setup_mici_home(&[("required.yml", &fixture("valid_required_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["required", "--name", "Alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Alice!"));
}

#[test]
fn run_required_input_with_default_succeeds() {
    let tmp = setup_mici_home(&[(
        "required-default.yml",
        &fixture("valid_required_input_with_default.yml"),
    )]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("required-default")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

// ─── Run: failures ───

#[test]
fn run_step_failure_propagates() {
    let tmp = setup_mici_home(&[("step-fail.yml", &fixture("valid_step_failure.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("step-fail")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Step 'fail' failed"));
}

#[test]
fn run_nonexistent_command() {
    let tmp = setup_mici_home(&[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("doesnotexist")
        .assert()
        .success()
        .stdout(predicate::str::contains("Can't run command"));
}

// ─── Dynamic command help ───

#[test]
fn dynamic_command_help() {
    let tmp = setup_mici_home(&[("hello.yml", &fixture("valid_command.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["hello", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-hello"))
        .stdout(predicate::str::contains("--name"));
}

#[test]
fn dynamic_command_help_masks_secret_default() {
    let tmp = setup_mici_home(&[("secret.yml", &fixture("valid_secret_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["secret", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("***"))
        .stdout(predicate::str::contains("my-token").not());
}

#[test]
fn dynamic_command_help_shows_choice_options() {
    let tmp = setup_mici_home(&[("choice.yml", &fixture("valid_choice_input.yml"))]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["choice", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--env"))
        .stdout(predicate::str::contains("staging"))
        .stdout(predicate::str::contains("production"));
}
