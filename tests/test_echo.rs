//! Integration tests for "echo"
use assert_cmd::Command;
use predicates;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn echo_empty_input() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert().stdout("\n").stderr("");

    return Ok(());
}

#[test]
fn echo_nonempty_input() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("Hello, world")
        .assert()
        .success()
        .stdout("Hello, world\n")
        .stderr("");
    return Ok(());
}

#[test]
fn echo_empty_no_new_line() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("-n")
        .arg("Hello, world")
        .assert()
        .success()
        .stdout("Hello, world")
        .stderr("");

    return Ok(());
}

#[test]
fn echo_invalid_flag() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("-x")
        .assert()
        .failure()
        .stdout("")
        .stderr(predicates::str::contains("error:"));

    return Ok(());
}
