//! Test head:
use assert_cmd::Command;
use predicates::str;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type TestResult = Result<(), Box<dyn Error>>;

/// Without any inputs, the head command should print the first ten lines
#[test]
fn single_file() -> TestResult {
    Command::cargo_bin("head")?
        .args(&["-n", "4", "tests/head/manylines.txt"])
        .assert()
        .success()
        .stdout("0000\n0001\n0002\n0003\n")
        .stderr("");
    return Ok(());
}

#[test]
fn byte_flag() -> TestResult {
    Command::cargo_bin("head")?
        .args(&["-c", "10", "tests/head/manylines.txt"])
        .assert()
        .success()
        .stdout("0000\n0001\n")
        .stderr("");
    return Ok(());
}

/// Without any file specified, the head command will listen for inputs
/// from stdin.
#[test]
fn head_stdin() -> TestResult {
    Command::cargo_bin("head")?
        .args(&["-n", "4"])
        .write_stdin("0\n1\n2\n3\n4\n5\n6\n7\n8\n9")
        .assert()
        .success()
        .stdout("0\n1\n2\n3\n")
        .stderr("");
    return Ok(());
}

/// When heading multiple files, the content of each file is prefixed with a
/// header and separated by a line break.
#[test]
fn multiple_files() -> TestResult {
    Command::cargo_bin("head")?
        .args(&[
            "-n",
            "1",
            "tests/head/manylines.txt",
            "tests/head/manylines.txt",
        ])
        .assert()
        .success()
        .stdout(
            "==> tests/head/manylines.txt <==
0000

==> tests/head/manylines.txt <==
0000
",
        )
        .stderr("");
    return Ok(());
}

/// If the input file does not exist, output to stderr:
/// "head: does-not-exist: No such file or directory"
#[test]
fn does_not_exist() -> TestResult {
    Command::cargo_bin("head")?
        .args(&["does-not-exist"])
        .assert()
        .failure()
        .stdout("")
        .stderr(str::contains("No such file or directory (os error 2)\n"));
    return Ok(());
}
