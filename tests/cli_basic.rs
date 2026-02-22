use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

fn mici() -> assert_cmd::Command {
    cargo_bin_cmd!("mici")
}

#[test]
fn version_flag_long() {
    mici()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn version_flag_short() {
    mici()
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn version_subcommand() {
    mici()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_flag_long() {
    mici()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("fetch"))
        .stdout(predicate::str::contains("new"))
        .stdout(predicate::str::contains("edit"))
        .stdout(predicate::str::contains("validate"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("config"));
}

#[test]
fn help_flag_short() {
    mici()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("init"));
}

#[test]
fn help_subcommand() {
    mici()
        .arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains("init"));
}

#[test]
fn no_args_without_config() {
    let tmp = tempfile::TempDir::new().unwrap();

    mici()
        .env("HOME", tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("init"));
}

#[test]
fn init_help() {
    mici()
        .args(["init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("--clean"));
}

#[test]
fn fetch_help() {
    mici()
        .args(["fetch", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fetch"))
        .stdout(predicate::str::contains("--branch"));
}

#[test]
fn new_help() {
    mici()
        .args(["new", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("new"));
}

#[test]
fn edit_help() {
    mici()
        .args(["edit", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("edit"));
}

#[test]
fn validate_help() {
    mici()
        .args(["validate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("validate"));
}

#[test]
fn list_help() {
    mici()
        .args(["list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"));
}

#[test]
fn config_help() {
    mici()
        .args(["config", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("config"));
}
