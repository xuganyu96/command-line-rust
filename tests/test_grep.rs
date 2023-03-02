use assert_cmd::Command;

#[test]
fn test_empty() {
    Command::cargo_bin("grep").unwrap()
        .args(&["fox", "tests/grep/empty.txt"])
        .assert()
        .failure()
        .stdout("")
        .stderr("");
}

#[test]
fn test_case_sensitivity() {
    Command::cargo_bin("grep").unwrap()
        .args(&["nobody", "tests/grep/nobody.txt"])
        .assert()
        .failure()
        .stdout("")
        .stderr("");

    Command::cargo_bin("grep").unwrap()
        .args(&["Nobody", "tests/grep/nobody.txt"])
        .assert()
        .success()
        .stdout("I'm Nobody! Who are you?
Are you—Nobody—too?
")
        .stderr("");
}

#[test]
fn test_ignore_case() {
    Command::cargo_bin("grep").unwrap()
        .args(&["-i", "nobody", "tests/grep/nobody.txt"])
        .assert()
        .success()
        .stdout("I'm Nobody! Who are you?
Are you—Nobody—too?
")
        .stderr("");
}

#[test]
fn test_invert_match() {
    Command::cargo_bin("grep").unwrap()
        .args(&["-v", "Nobody", "tests/grep/nobody.txt"])
        .assert()
        .success()
        .stdout("Then there's a pair of us!
Don't tell! they'd advertise—you know!

How dreary—to be—Somebody!
How public—like a Frog—
To tell one's name—the livelong June—
To an admiring Bog!
")
        .stderr("");
}

#[test]
fn test_count() {
    Command::cargo_bin("grep").unwrap()
        .args(&["-c", "Nobody", "tests/grep/nobody.txt"])
        .assert()
        .success()
        .stdout("2\n")
        .stderr("");
}

#[test]
fn test_multiple_files() {
    Command::cargo_bin("grep").unwrap()
        .args(&["the", "tests/grep/nobody.txt", "tests/grep/bustle.txt"])
        .assert()
        .success()
        .stdout("tests/grep/nobody.txt:Then there's a pair of us!
tests/grep/nobody.txt:Don't tell! they'd advertise—you know!
tests/grep/nobody.txt:To tell one's name—the livelong June—
tests/grep/bustle.txt:The sweeping up the heart,
")
        .stderr("");
}

#[test]
fn test_directory_nonrecursive() {
    Command::cargo_bin("grep").unwrap()
        .args(&["the", "tests/grep"])
        .assert()
        .failure()
        .stdout("")
        .stderr("grep: tests/grep: Is a directory\n");
}

#[test]
fn test_directory_recursive() {
    Command::cargo_bin("grep").unwrap()
        .args(&["-r", "the", "tests/grep"])
        .assert()
        .success()
        .stdout("tests/grep/nobody.txt:Then there's a pair of us!
tests/grep/nobody.txt:Don't tell! they'd advertise—you know!
tests/grep/nobody.txt:To tell one's name—the livelong June—
tests/grep/bustle.txt:The sweeping up the heart,
tests/grep/fox.txt:The quick brown fox jumps over the lazy dog.
")
        .stderr("");
}
