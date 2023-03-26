//! Integration tests for the comm program
//!
//! Some notes on how comm prints the lines:
//! At the beginning of the program, there are two pointers each starting at
//! first line of the each input. At each iteration, a line is produced from
//! each of the file that still has lines left. If the two lines are equal,
//! then print to the third column. If the two lines are not equal, then the
//! line that is lexicographically lesser is printed to the appropriate
//! column, and the appropriate file will advance by one line.
use assert_cmd::Command;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const FAANG: &str = "tests/comm/FAANG.txt";
const LOWER: &str = "tests/comm/lowercase.txt";
const MANGA: &str = "tests/comm/MANGA.txt";
const EMPTY: &str = "tests/comm/empty.txt";

#[test]
fn test_faang_empty() -> TestResult {
    Command::cargo_bin("comm")?
        .args(&[FAANG, EMPTY])
        .assert()
        .try_success()?
        .try_stdout(
            "Amazon
Apple
Facebook
Google
Netflix
",
        )?
        .try_stderr("")?;

    return Ok(());
}

#[test]
fn test_empty_faang() -> TestResult {
    Command::cargo_bin("comm")?
        .args(&[EMPTY, FAANG])
        .assert()
        .try_success()?
        .try_stdout(
            "	Amazon
	Apple
	Facebook
	Google
	Netflix
",
        )?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn test_faang_manga() -> TestResult {
    Command::cargo_bin("comm")?
        .args(&[FAANG, MANGA])
        .assert()
        .try_success()?
        .try_stdout(
            "		Amazon
		Apple
Facebook
		Google
	Meta
		Netflix
",
        )?
        .try_stderr("")?;

    return Ok(());
}

#[test]
fn test_faang_manga_1() -> TestResult {
    Command::cargo_bin("comm")?
        .args(&["-23", FAANG, MANGA])
        .assert()
        .try_success()?
        .try_stdout("Facebook\n")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn test_faang_manga_2() -> TestResult {
    Command::cargo_bin("comm")?
        .args(&["-13", FAANG, MANGA])
        .assert()
        .try_success()?
        .try_stdout("Meta\n")?
        .try_stderr("")?;
    return Ok(());
}

#[test]
fn test_faang_manga_3() -> TestResult {
    Command::cargo_bin("comm")?
        .args(&["-12", FAANG, MANGA])
        .assert()
        .try_success()?
        .try_stdout(
            "Amazon
Apple
Google
Netflix
",
        )?
        .try_stderr("")?;
    return Ok(());
}
