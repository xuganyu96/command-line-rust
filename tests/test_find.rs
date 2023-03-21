//! Integration tests for find
use std::error::Error;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

fn test_bin(
    mut cmd: Command,
    args: &[&str],
    success: bool,
    stdout: &'static str,
    stderr: &'static str,
) -> TestResult {
    let mut assertion = cmd.args(args)
        .assert();

    if success {
        assertion = assertion.try_success()?;
    } else {
        assertion = assertion.try_failure()?;
    }

    assertion = assertion.try_stdout(stdout)?;
    let _ = assertion.try_stderr(stderr)?;

    return Ok(());
}


#[test]
fn find_all() -> TestResult {
    Command::cargo_bin("find")?
        .args(&["tests/find"])
        .assert()
        .try_success()?
        .try_stdout("tests/find
tests/find/g.csv
tests/find/a
tests/find/a/a.txt
tests/find/a/b
tests/find/a/b/b.csv
tests/find/a/b/c
tests/find/a/b/c/c.mp3
tests/find/f
tests/find/f/f.txt
tests/find/d
tests/find/d/b.csv
tests/find/d/d.txt
tests/find/d/d.tsv
tests/find/d/e
tests/find/d/e/e.mp3
")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn find_files() -> TestResult {
    Command::cargo_bin("find")?
        .args(&["tests/find", "--type", "f"])
        .assert()
        .try_success()?
        .try_stdout("tests/find/g.csv
tests/find/a/a.txt
tests/find/a/b/b.csv
tests/find/a/b/c/c.mp3
tests/find/f/f.txt
tests/find/d/d.txt
tests/find/d/d.tsv
tests/find/d/e/e.mp3
")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn find_dirs() -> TestResult {
    Command::cargo_bin("find")?
        .args(&["tests/find", "--type", "d"])
        .assert()
        .try_success()?
        .try_stdout("tests/find
tests/find/a
tests/find/a/b
tests/find/a/b/c
tests/find/f
tests/find/d
tests/find/d/e
")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn find_links() -> TestResult {
    Command::cargo_bin("find")?
        .args(&["tests/find", "--type", "l"])
        .assert()
        .try_success()?
        .try_stdout("tests/find/d/b.csv\n")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn find_txts() -> TestResult {
    Command::cargo_bin("find")?
        .args(&["tests/find", "--regex", ".*\\.txt"])
        .assert()
        .try_success()?
        .try_stdout("tests/find/a/a.txt
tests/find/f/f.txt
tests/find/d/d.txt
")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn find_csv() -> TestResult {
    Command::cargo_bin("find")?
        .args(&["tests/find", "--regex", ".*\\.csv"])
        .assert()
        .try_success()?
        .try_stdout("tests/find/g.csv
tests/find/a/b/b.csv
tests/find/d/b.csv
")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn find_mp3() -> TestResult {
    Command::cargo_bin("find")?
        .args(&["tests/find", "--regex", ".*\\.mp3"])
        .assert()
        .try_success()?
        .try_stdout("tests/find/a/b/c/c.mp3
tests/find/d/e/e.mp3
")?
        .try_stderr("")?;
    return Ok(());
}
