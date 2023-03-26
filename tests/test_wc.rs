use assert_cmd::Command;
use predicates::str;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

/// Test various flag combinations against some simple strings fed through
/// stdin
#[test]
fn count_stdin() -> TestResult {
    Command::cargo_bin("wc")?
        .write_stdin("Hello, world!\n")
        .assert()
        .try_success()?
        .try_stdout("       1       2      14 \n")?
        .try_stderr("")?;

    Command::cargo_bin("wc")?
        .args(&["-lm"])
        .write_stdin("Hello, world!\n")
        .assert()
        .try_success()?
        .try_stdout("       1      14 \n")?
        .try_stderr("")?;

    Command::cargo_bin("wc")?
        .args(&["-wc"])
        .write_stdin("Hello, world!\n")
        .assert()
        .try_success()?
        .try_stdout("       2      14 \n")?
        .try_stderr("")?;

    return Ok(());
}

/// Count the words of multiple files, summing the results at the end
#[test]
fn count_multiple_files() -> TestResult {
    Command::cargo_bin("wc")?
        .args(&["tests/wc/haiku.txt", "tests/wc/haiku.txt"])
        .assert()
        .try_success()?
        .try_stdout(
            "       3      11      48 tests/wc/haiku.txt
       3      11      48 tests/wc/haiku.txt
       6      22      96 total\n",
        )?
        .try_stderr("")?;

    return Ok(());
}

/// Count the words of files that contain multi-byte characters
#[test]
fn count_nonascii() -> TestResult {
    Command::cargo_bin("wc")?
        .args(&["-lwm", "tests/wc/nonascii.txt"])
        .assert()
        .try_success()?
        .try_stdout("       4       4      16 tests/wc/nonascii.txt\n")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn count_does_not_exist() -> TestResult {
    Command::cargo_bin("wc")?
        .args(&["does-not-exist"])
        .assert()
        .try_failure()?
        .try_stderr(str::contains("No such file or directory"))?;
    return Ok(());
}
