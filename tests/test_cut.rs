//! Integration tests for "cut"
use std::error::Error;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn cut_bytes() -> TestResult {
    Command::cargo_bin("cut")?
        .args(&["-b", "1-2", "tests/cut/books.csv"])
        .assert()
        .success()
        .stdout("Au
É
Sa
Ju
");

    return Ok(());
}

#[test]
fn cut_chars() -> TestResult {
    Command::cargo_bin("cut")?
        .args(&["-c", "1,2,3-10", "tests/cut/books.csv"])
        .assert()
        .success()
        .stdout("Author,Yea
Émile Zola
Samuel Bec
Jules Vern
");

    return Ok(());
}

#[test]
fn cut_csv() -> TestResult {
    Command::cargo_bin("cut")?
        .args(&["-f", "2", "-d", ",", "tests/cut/books.csv"])
        .assert()
        .success()
        .stdout("Year
1865
1952
1870
");
    return Ok(());
}

#[test]
fn cut_outside_range() -> TestResult {
    Command::cargo_bin("cut")?
        .args(&["-c", "77-99", "tests/cut/books.csv"])
        .assert()
        .success()
        .stdout("



");

    return Ok(());
}
