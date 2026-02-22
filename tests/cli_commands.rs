mod common;

use assert_cmd::cargo::cargo_bin_cmd;
use common::{fixture, setup_mici_home};
use predicates::prelude::*;

fn mici() -> assert_cmd::Command {
    cargo_bin_cmd!("mici")
}

#[test]
fn validate_valid_command() {
    let yaml = fixture("valid_command.yml");
    let tmp = setup_mici_home(&[("hello.yml", &yaml)]);

    mici()
        .env("HOME", tmp.path())
        .args(["validate", "hello"])
        .assert()
        .success();
}

#[test]
fn validate_invalid_command() {
    let yaml = fixture("invalid_command.yml");
    let tmp = setup_mici_home(&[("bad.yml", &yaml)]);

    mici()
        .env("HOME", tmp.path())
        .args(["validate", "bad"])
        .assert()
        .failure();
}

#[test]
fn list_shows_commands() {
    let yaml = fixture("valid_command.yml");
    let tmp = setup_mici_home(&[("hello.yml", &yaml)]);

    mici()
        .env("HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn run_simple_command() {
    let yaml = fixture("minimal_command.yml");
    let tmp = setup_mici_home(&[("minimal.yml", &yaml)]);

    mici()
        .env("HOME", tmp.path())
        .arg("minimal")
        .assert()
        .success()
        .stdout(predicate::str::contains("ok"));
}

#[test]
fn run_command_with_input() {
    let yaml = fixture("valid_command.yml");
    let tmp = setup_mici_home(&[("hello.yml", &yaml)]);

    mici()
        .env("HOME", tmp.path())
        .args(["hello", "--name", "Rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Rust!"));
}

#[test]
fn run_command_with_default_input() {
    let yaml = fixture("valid_command.yml");
    let tmp = setup_mici_home(&[("hello.yml", &yaml)]);

    mici()
        .env("HOME", tmp.path())
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn nonexistent_command_shows_error() {
    let tmp = setup_mici_home(&[]);

    mici()
        .env("HOME", tmp.path())
        .arg("doesnotexist")
        .assert()
        .success()
        .stdout(predicate::str::contains("Can't run command"));
}

#[test]
fn dynamic_command_help() {
    let yaml = fixture("valid_command.yml");
    let tmp = setup_mici_home(&[("hello.yml", &yaml)]);

    mici()
        .env("HOME", tmp.path())
        .args(["hello", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-hello"))
        .stdout(predicate::str::contains("--name"));
}
