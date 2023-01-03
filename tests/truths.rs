//! Test the programs "true", "false", and "hello-world"
use assert_cmd::Command;

#[test]
fn test_true() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn test_false() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_hello_world() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().stdout("Hello, world!\n");
}
