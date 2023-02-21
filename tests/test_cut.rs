//! Integration tests for "cut"
use assert_cmd::Command;

#[test]
fn cut_bytes() {
    Command::new("cut")
        .args(&["-b", "1-2", "tests/cut/books.csv"])
        .assert()
        .success()
        .stdout("Au
É
Sa
Ju
");
}

#[test]
fn cut_chars() {
    Command::new("cut")
        .args(&["-c", "1,2,3-10", "tests/cut/books.csv"])
        .assert()
        .success()
        .stdout("Author,Yea
Émile Zola
Samuel Bec
Jules Vern
");
}

#[test]
fn cut_csv() {
    Command::new("cut")
        .args(&["-f", "2", "-d", ",", "tests/cut/books.csv"])
        .assert()
        .success()
        .stdout("Year
1865
1952
1870
");
}

#[test]
fn cut_outside_range() {
    Command::new("cut")
        .args(&["-c", "77-99", "tests/cut/books.csv"])
        .assert()
        .success()
        .stdout("



");
}
