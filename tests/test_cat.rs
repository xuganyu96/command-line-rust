use assert_cmd::Command;
use std::error::Error;
use predicates::str;

type TestResult = Result<(), Box<dyn Error>>;

/// Conatenate several files and check the output
#[test]
fn cat_regular_files() -> TestResult {
    Command::cargo_bin("cat")?
        .args(&["tests/cat/foobar.txt", "tests/cat/foobar.txt"])
        .assert()
        .try_success()?
        .try_stdout("foobar\nfoobar\n")?
        .try_stderr("")?;
    return Ok(());
}

/// Concatenate several files with stdin
#[test]
fn cat_files_and_stdin() -> TestResult {
    Command::cargo_bin("cat")?
        .args(&["tests/cat/foobar.txt", "-", "tests/cat/foobar.txt"])
        .write_stdin("baz\n")
        .assert()
        .try_success()?
        .try_stdout("foobar\nbaz\nfoobar\n")?
        .try_stderr("")?;
    return Ok(());
}

/// Concatenate with the -b flag, across oneor more files
#[test]
fn cat_counting_nonblank_lines() -> TestResult {
    Command::cargo_bin("cat")?
        .args(&["-b", "tests/cat/haiku.txt", "-", "tests/cat/haiku.txt"])
        .write_stdin("\n")
        .assert()
        .try_success()?
        .try_stdout("     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS

     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
")?
        .try_stderr("")?;
    return Ok(());
}

/// Concatenate with the -n flag, across one or more files
#[test]
fn cat_counting_all_lines() -> TestResult {
    Command::cargo_bin("cat")?
        .args(&["-n", "tests/cat/haiku.txt", "-", "tests/cat/haiku.txt"])
        .write_stdin("\n")
        .assert()
        .try_success()?
        .try_stdout("     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
     1	
     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn cat_does_not_exist() -> TestResult {
    Command::cargo_bin("cat")?
        .args(&["does-not-exist"])
        .assert()
        .try_failure()?
        .try_stdout("")?
        .try_stderr(str::contains("No such file or directory"))?;
    return Ok(());
}
