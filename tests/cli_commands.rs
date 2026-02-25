mod common;

use assert_cmd::cargo::cargo_bin_cmd;
use common::setup_mici_home;
use predicates::prelude::*;

fn mici() -> assert_cmd::Command {
    cargo_bin_cmd!("mici")
}

// ─── Validation: success ───

#[test]
fn validate_valid_command() {
    let tmp = setup_mici_home(&["valid_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "valid_command"])
        .assert()
        .success();
}

#[test]
fn validate_minimal_command() {
    let tmp = setup_mici_home(&["minimal_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "minimal_command"])
        .assert()
        .success();
}

#[test]
fn validate_no_inputs() {
    let tmp = setup_mici_home(&["valid_no_inputs.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "valid_no_inputs"])
        .assert()
        .success();
}

#[test]
fn validate_multi_step() {
    let tmp = setup_mici_home(&["valid_multi_step.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "valid_multi_step"])
        .assert()
        .success();
}

#[test]
fn validate_choice_input() {
    let tmp = setup_mici_home(&["valid_choice_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "valid_choice_input"])
        .assert()
        .success();
}

#[test]
fn validate_bool_input() {
    let tmp = setup_mici_home(&["valid_bool_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "valid_bool_input"])
        .assert()
        .success();
}

#[test]
fn validate_env_vars() {
    let tmp = setup_mici_home(&["valid_env_vars.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "valid_env_vars"])
        .assert()
        .success();
}

#[test]
fn validate_input_resolution() {
    let tmp = setup_mici_home(&["valid_input_resolution.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "valid_input_resolution"])
        .assert()
        .success();
}

// ─── Validation: failures ───

#[test]
fn validate_invalid_version_name_steps() {
    let tmp = setup_mici_home(&["invalid_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_command"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("version_invalid"))
        .stderr(predicate::str::contains("name_empty"))
        .stderr(predicate::str::contains("steps_empty"));
}

#[test]
fn validate_invalid_input_type() {
    let tmp = setup_mici_home(&["invalid_input_type.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_input_type"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("input_type_invalid"));
}

#[test]
fn validate_invalid_empty_type() {
    let tmp = setup_mici_home(&["invalid_empty_type.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_empty_type"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("input_type_empty"));
}

#[test]
fn validate_invalid_secret_on_bool() {
    let tmp = setup_mici_home(&["invalid_secret_on_bool.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_secret_on_bool"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("secret_requires_string"));
}

#[test]
fn validate_invalid_choice_no_options() {
    let tmp = setup_mici_home(&["invalid_choice_no_options.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_choice_no_options"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("choice_requires_options"));
}

#[test]
fn validate_invalid_options_on_string() {
    let tmp = setup_mici_home(&["invalid_options_on_string.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_options_on_string"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("options_only_for_choice"));
}

#[test]
fn validate_invalid_step_empty_id() {
    let tmp = setup_mici_home(&["invalid_step_no_id.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_step_no_id"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("step_id_empty"));
}

#[test]
fn validate_invalid_step_whitespace_id() {
    let tmp = setup_mici_home(&["invalid_step_whitespace_id.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_step_whitespace_id"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("step_id_whitespace"));
}

#[test]
fn validate_invalid_duplicate_step_ids() {
    let tmp = setup_mici_home(&["invalid_duplicate_step_ids.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_duplicate_step_ids"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("step_id_duplicate"));
}

#[test]
fn validate_invalid_step_no_run() {
    let tmp = setup_mici_home(&["invalid_step_no_run.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_step_no_run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid_syntax"));
}

#[test]
fn validate_invalid_yaml_syntax() {
    let tmp = setup_mici_home(&["invalid_yaml_syntax.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_yaml_syntax"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid_syntax"));
}

#[test]
fn validate_invalid_step_command_and_script() {
    let tmp = setup_mici_home(&["invalid_step_command_and_script.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_step_command_and_script"])
        .assert()
        .failure();
}

#[test]
fn validate_invalid_multiple_errors() {
    let tmp = setup_mici_home(&["invalid_multiple_errors.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["validate", "invalid_multiple_errors"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("input_type_invalid"))
        .stderr(predicate::str::contains("choice_requires_options"))
        .stderr(predicate::str::contains("step_id_whitespace"))
        .stderr(predicate::str::contains("step_id_duplicate"));
}

// ─── Config validation ───

#[test]
fn config_warns_on_unknown_keys() {
    let tmp = setup_mici_home(&["valid_command.yml"], &[]);

    // Overwrite config with an unknown key
    std::fs::write(
        tmp.path().join(".mici/config.yml"),
        "disable_pager: true\ntypo_key: true\n",
    )
    .unwrap();

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Unknown config key 'typo_key'"));
}

#[test]
fn config_no_warning_on_valid_keys() {
    let tmp = setup_mici_home(&["valid_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Unknown config key").not());
}

// ─── List ───

#[test]
fn list_shows_commands() {
    let tmp = setup_mici_home(&["valid_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid_command"));
}

#[test]
fn list_shows_multiple_commands() {
    let tmp = setup_mici_home(&["valid_command.yml", "minimal_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid_command"))
        .stdout(predicate::str::contains("minimal_command"));
}

// ─── Run: success ───

#[test]
fn run_simple_command() {
    let tmp = setup_mici_home(&["minimal_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("minimal_command")
        .assert()
        .success()
        .stdout(predicate::str::contains("ok"));
}

#[test]
fn run_command_with_input() {
    let tmp = setup_mici_home(&["valid_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_command", "--name", "Rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Rust!"));
}

#[test]
fn run_command_with_default_input() {
    let tmp = setup_mici_home(&["valid_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_command")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn run_multi_step() {
    let tmp = setup_mici_home(&["valid_multi_step.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_multi_step")
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
    let tmp = setup_mici_home(&["valid_env_vars.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_env_vars")
        .assert()
        .success()
        .stdout(predicate::str::contains("global-value"))
        .stdout(predicate::str::contains("step-override"));
}

#[cfg(windows)]
#[test]
fn run_env_vars() {
    let tmp = setup_mici_home(&["valid_env_vars_windows.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_env_vars_windows")
        .assert()
        .success()
        .stdout(predicate::str::contains("global-value"))
        .stdout(predicate::str::contains("step-override"));
}

#[cfg(unix)]
#[test]
fn run_input_resolution_with_args() {
    let tmp = setup_mici_home(&["valid_input_resolution.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_input_resolution",
            "--greeting",
            "Hi",
            "--target",
            "Earth",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hi, Earth!"));
}

#[cfg(windows)]
#[test]
fn run_input_resolution_with_args() {
    let tmp = setup_mici_home(&["valid_input_resolution_windows.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_input_resolution_windows",
            "--greeting",
            "Hi",
            "--target",
            "Earth",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hi, Earth!"));
}

#[cfg(unix)]
#[test]
fn run_input_resolution_defaults() {
    let tmp = setup_mici_home(&["valid_input_resolution.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_input_resolution")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[cfg(windows)]
#[test]
fn run_input_resolution_defaults() {
    let tmp = setup_mici_home(&["valid_input_resolution_windows.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_input_resolution_windows")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn run_choice_input_with_value() {
    let tmp = setup_mici_home(&["valid_choice_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_choice_input", "--env", "staging"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deploying to staging"));
}

#[test]
fn run_choice_input_default() {
    let tmp = setup_mici_home(&["valid_choice_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_choice_input")
        .assert()
        .success()
        .stdout(predicate::str::contains("deploying to production"));
}

#[test]
fn run_bool_input_present() {
    let tmp = setup_mici_home(&["valid_bool_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_bool_input", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)dry_run=true").unwrap());
}

#[test]
fn run_bool_input_absent() {
    let tmp = setup_mici_home(&["valid_bool_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_bool_input")
        .assert()
        .success()
        .stdout(predicate::str::is_match("(?i)dry_run=false").unwrap());
}

#[test]
fn run_no_inputs_command() {
    let tmp = setup_mici_home(&["valid_no_inputs.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_no_inputs")
        .assert()
        .success()
        .stdout(predicate::str::contains("no inputs needed"));
}

// ─── Run: input validation ───

#[test]
fn run_choice_rejects_invalid_value() {
    let tmp = setup_mici_home(&["valid_choice_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_choice_input", "--env", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a valid option"))
        .stderr(predicate::str::contains("staging, production"));
}

#[test]
fn run_required_input_missing() {
    let tmp = setup_mici_home(&["valid_required_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_required_input")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not provided"));
}

#[test]
fn run_required_input_provided() {
    let tmp = setup_mici_home(&["valid_required_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_required_input", "--name", "Alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Alice!"));
}

#[test]
fn run_required_input_with_default_succeeds() {
    let tmp = setup_mici_home(&["valid_required_input_with_default.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_required_input_with_default")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

// ─── Run: failures ───

#[test]
fn run_step_failure_propagates() {
    let tmp = setup_mici_home(&["valid_step_failure.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_step_failure")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Step 'fail' failed"));
}

#[cfg(unix)]
#[test]
fn run_step_failure_forwards_exit_code() {
    let tmp = setup_mici_home(&["valid_step_exit_code.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_step_exit_code")
        .assert()
        .failure()
        .code(42)
        .stderr(predicate::str::contains(
            "Step 'specific-exit' failed with exit code: 42",
        ));
}

#[test]
fn run_nonexistent_command() {
    let tmp = setup_mici_home(&[], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("doesnotexist")
        .assert()
        .success()
        .stdout(predicate::str::contains("Can't run command"));
}

// ─── Run: warnings ───

#[cfg(unix)]
#[test]
fn run_warns_on_unknown_input_reference() {
    let tmp = setup_mici_home(&["valid_warn_unknown_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_warn_unknown_input")
        .assert()
        .success()
        .stderr(predicate::str::contains("Unknown input reference"))
        .stderr(predicate::str::contains("nonexistent"));
}

#[cfg(unix)]
#[test]
fn run_warns_on_unset_env_variable() {
    let tmp = setup_mici_home(&["valid_warn_unset_env.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_warn_unset_env")
        .assert()
        .success()
        .stderr(predicate::str::contains("Environment variable"))
        .stderr(predicate::str::contains(
            "MICI_TEST_UNSET_VAR_THAT_DOES_NOT_EXIST",
        ));
}

// ─── Run: scripts ───

#[cfg(unix)]
#[test]
fn run_script_step() {
    let tmp = setup_mici_home(
        &["valid_script.yml"],
        &[("hello.sh", "#!/bin/bash\necho \"hello from script\"")],
    );

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_script")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello from script"));
}

#[cfg(unix)]
#[test]
fn run_script_with_env_vars() {
    let tmp = setup_mici_home(
        &["valid_script_env.yml"],
        &[("show-env.sh", "#!/bin/bash\necho $MY_SCRIPT_VAR")],
    );

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_script_env")
        .assert()
        .success()
        .stdout(predicate::str::contains("script-env-value"));
}

#[cfg(windows)]
#[test]
fn run_script_step() {
    let tmp = setup_mici_home(
        &["valid_script_windows.yml"],
        &[("hello.ps1", "Write-Output \"hello from script\"")],
    );

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_script_windows")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello from script"));
}

#[cfg(windows)]
#[test]
fn run_script_with_env_vars() {
    let tmp = setup_mici_home(
        &["valid_script_env_windows.yml"],
        &[("show-env.ps1", "Write-Output $env:MY_SCRIPT_VAR")],
    );

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_script_env_windows")
        .assert()
        .success()
        .stdout(predicate::str::contains("script-env-value"));
}

// ─── Run: MICI_INPUT_* auto-injection ───

#[cfg(unix)]
#[test]
fn run_auto_input_env_in_command() {
    let tmp = setup_mici_home(&["valid_auto_input_env.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_auto_input_env", "--name", "injected-value"])
        .assert()
        .success()
        .stdout(predicate::str::contains("injected-value"));
}

#[cfg(windows)]
#[test]
fn run_auto_input_env_in_command() {
    let tmp = setup_mici_home(&["valid_auto_input_env_windows.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_auto_input_env_windows", "--name", "injected-value"])
        .assert()
        .success()
        .stdout(predicate::str::contains("injected-value"));
}

#[cfg(unix)]
#[test]
fn run_auto_input_env_in_script() {
    let tmp = setup_mici_home(
        &["valid_script_auto_inputs.yml"],
        &[(
            "check-inputs.sh",
            "#!/bin/bash\necho $MICI_INPUT_NAME\necho $MICI_INPUT_FORCE",
        )],
    );

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_script_auto_inputs", "--name", "test-value", "-f"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-value"))
        .stdout(predicate::str::contains("true"));
}

#[cfg(windows)]
#[test]
fn run_auto_input_env_in_script() {
    let tmp = setup_mici_home(
        &["valid_script_auto_inputs_windows.yml"],
        &[(
            "check-inputs.ps1",
            "Write-Output $env:MICI_INPUT_NAME\nWrite-Output $env:MICI_INPUT_FORCE",
        )],
    );

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_script_auto_inputs_windows",
            "--name",
            "test-value",
            "-f",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-value"))
        .stdout(predicate::str::contains("true"));
}

#[cfg(unix)]
#[test]
fn run_auto_input_env_uses_defaults() {
    let tmp = setup_mici_home(&["valid_auto_input_env.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_auto_input_env")
        .assert()
        .success()
        .stdout(predicate::str::contains("fallback"));
}

#[cfg(windows)]
#[test]
fn run_auto_input_env_uses_defaults() {
    let tmp = setup_mici_home(&["valid_auto_input_env_windows.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .arg("valid_auto_input_env_windows")
        .assert()
        .success()
        .stdout(predicate::str::contains("fallback"));
}

// ─── Run: working directory ───

#[cfg(unix)]
#[test]
fn run_with_working_directory_input_resolution() {
    let tmp = setup_mici_home(&["valid_working_dir.yml"], &[]);

    let target_dir = tmp.path().join("my-project");
    std::fs::create_dir_all(&target_dir).unwrap();

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_working_dir",
            "--target-dir",
            target_dir.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(target_dir.to_str().unwrap()));
}

#[cfg(windows)]
#[test]
fn run_with_working_directory_input_resolution() {
    let tmp = setup_mici_home(&["valid_working_dir_windows.yml"], &[]);

    let target_dir = tmp.path().join("my-project");
    std::fs::create_dir_all(&target_dir).unwrap();
    let canonical = target_dir
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_start_matches("\\\\?\\")
        .to_string();

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_working_dir_windows",
            "--target-dir",
            target_dir.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(&canonical));
}

#[cfg(unix)]
#[test]
fn run_with_step_working_directory_input_resolution() {
    let tmp = setup_mici_home(&["valid_step_working_dir.yml"], &[]);

    let target_dir = tmp.path().join("step-project");
    let other_dir = tmp.path().join("other-project");
    std::fs::create_dir_all(&target_dir).unwrap();
    std::fs::create_dir_all(&other_dir).unwrap();

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_step_working_dir",
            "--target-dir",
            target_dir.to_str().unwrap(),
            "--other-dir",
            other_dir.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(target_dir.to_str().unwrap()));
}

#[cfg(windows)]
#[test]
fn run_with_step_working_directory_input_resolution() {
    let tmp = setup_mici_home(&["valid_step_working_dir_windows.yml"], &[]);

    let target_dir = tmp.path().join("step-project");
    let other_dir = tmp.path().join("other-project");
    std::fs::create_dir_all(&target_dir).unwrap();
    std::fs::create_dir_all(&other_dir).unwrap();
    let canonical = target_dir
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_start_matches("\\\\?\\")
        .to_string();

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_step_working_dir_windows",
            "--target-dir",
            target_dir.to_str().unwrap(),
            "--other-dir",
            other_dir.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(&canonical));
}

#[cfg(unix)]
#[test]
fn run_with_invalid_working_directory() {
    let tmp = setup_mici_home(&["valid_working_dir.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_working_dir",
            "--target-dir",
            "/nonexistent/path/that/does/not/exist",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[cfg(windows)]
#[test]
fn run_with_invalid_working_directory() {
    let tmp = setup_mici_home(&["valid_working_dir_windows.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_working_dir_windows",
            "--target-dir",
            "C:\\nonexistent\\path\\that\\does\\not\\exist",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[cfg(unix)]
#[test]
fn run_with_invalid_step_working_directory_points_to_correct_step() {
    let tmp = setup_mici_home(&["valid_step_working_dir.yml"], &[]);

    let other_dir = tmp.path().join("other-project");
    std::fs::create_dir_all(&other_dir).unwrap();

    // first_step and third_step have valid other-dir, but print_cwd has invalid target-dir
    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_step_working_dir",
            "--target-dir",
            "/nonexistent/path",
            "--other-dir",
            other_dir.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"))
        .stderr(predicate::str::contains("@{inputs.target-dir}"));
}

#[cfg(windows)]
#[test]
fn run_with_invalid_step_working_directory_points_to_correct_step() {
    let tmp = setup_mici_home(&["valid_step_working_dir_windows.yml"], &[]);

    let other_dir = tmp.path().join("other-project");
    std::fs::create_dir_all(&other_dir).unwrap();

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_step_working_dir_windows",
            "--target-dir",
            "C:\\nonexistent\\path",
            "--other-dir",
            other_dir.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"))
        .stderr(predicate::str::contains("@{inputs.target-dir}"));
}

#[cfg(unix)]
#[test]
fn run_with_multiple_invalid_step_working_directories_reports_all() {
    let tmp = setup_mici_home(&["valid_step_working_dir.yml"], &[]);

    // Both target-dir and other-dir are invalid — should report errors for all 3 steps
    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_step_working_dir",
            "--target-dir",
            "/nonexistent/target",
            "--other-dir",
            "/nonexistent/other",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("3 working directory error(s)"))
        .stderr(predicate::str::contains("@{inputs.target-dir}"))
        .stderr(predicate::str::contains("@{inputs.other-dir}"));
}

#[cfg(windows)]
#[test]
fn run_with_multiple_invalid_step_working_directories_reports_all() {
    let tmp = setup_mici_home(&["valid_step_working_dir_windows.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args([
            "valid_step_working_dir_windows",
            "--target-dir",
            "C:\\nonexistent\\target",
            "--other-dir",
            "C:\\nonexistent\\other",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("3 working directory error(s)"))
        .stderr(predicate::str::contains("@{inputs.target-dir}"))
        .stderr(predicate::str::contains("@{inputs.other-dir}"));
}

// ─── Dynamic command help ───

#[test]
fn dynamic_command_help() {
    let tmp = setup_mici_home(&["valid_command.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_command", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-hello"))
        .stdout(predicate::str::contains("--name"));
}

#[test]
fn dynamic_command_help_masks_secret_default() {
    let tmp = setup_mici_home(&["valid_secret_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_secret_input", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("***"))
        .stdout(predicate::str::contains("my-token").not());
}

#[test]
fn dynamic_command_help_shows_choice_options() {
    let tmp = setup_mici_home(&["valid_choice_input.yml"], &[]);

    mici()
        .env("MICI_HOME", tmp.path())
        .args(["valid_choice_input", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--env"))
        .stdout(predicate::str::contains("staging"))
        .stdout(predicate::str::contains("production"));
}
