use assert_cmd::Command;

#[test]
fn test_empty() {
    Command::new("grep")
        .args(&["fox", "tests/grep/empty.txt"])
        .assert()
        .failure()
        .stdout("")
        .stderr("");
}

#[test]
fn test_case_sensitivity() {
    Command::new("grep")
        .args(&["nobody", "tests/grep/nobody.txt"])
        .assert()
        .failure()
        .stdout("")
        .stderr("");

    Command::new("grep")
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
    Command::new("grep")
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
    Command::new("grep")
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
    Command::new("grep")
        .args(&["-c", "Nobody", "tests/grep/nobody.txt"])
        .assert()
        .success()
        .stdout("2\n")
        .stderr("");
}

#[test]
fn test_multiple_files() {
    Command::new("grep")
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
    Command::new("grep")
        .args(&["the", "tests/grep"])
        .assert()
        .failure()
        .stdout("")
        .stderr("grep: tests/grep: Is a directory\n");
}

#[test]
fn test_directory_recursive() {
    Command::new("grep")
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
