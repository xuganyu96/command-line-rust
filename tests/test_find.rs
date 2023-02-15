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
    return test_bin(
        Command::new("find"),
        &["tests/find"],
        true,
        "tests/find
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
",
        "",
    );
}

#[test]
fn find_files() -> TestResult {
    return test_bin(
        Command::new("find"),
        &["tests/find", "-type", "f"],
        true,
        "tests/find/g.csv
tests/find/a/a.txt
tests/find/a/b/b.csv
tests/find/a/b/c/c.mp3
tests/find/f/f.txt
tests/find/d/d.txt
tests/find/d/d.tsv
tests/find/d/e/e.mp3
",
        "",
    );
}

#[test]
fn find_dirs() -> TestResult {
    return test_bin(
        Command::new("find"),
        &["tests/find", "-type", "d"],
        true,
        "tests/find
tests/find/a
tests/find/a/b
tests/find/a/b/c
tests/find/f
tests/find/d
tests/find/d/e
",
        "",
    );
}

#[test]
fn find_links() -> TestResult {
    return test_bin(
        Command::new("find"),
        &["tests/find", "-type", "l"],
        true,
        "tests/find/d/b.csv
",
        "",
    );
}

#[test]
fn find_txts() -> TestResult {
    return test_bin(
        Command::new("find"),
        &["tests/find", "-regex", ".*\\.txt"],
        true,
        "tests/find/a/a.txt
tests/find/f/f.txt
tests/find/d/d.txt
",
        "",
    );
}

#[test]
fn find_csv() -> TestResult {
    return test_bin(
        Command::new("find"),
        &["tests/find", "-regex", ".*\\.csv"],
        true,
        "tests/find/g.csv
tests/find/a/b/b.csv
tests/find/d/b.csv
",
        "",
    );
}

#[test]
fn find_mp3() -> TestResult {
    return test_bin(
        Command::new("find"),
        &["tests/find", "-regex", ".*\\.mp3"],
        true,
        "tests/find/a/b/c/c.mp3
tests/find/d/e/e.mp3
",
        "",
    );
}
