//! Trivially correct tests meant for demonstrating the testing capabilities
//! of Rust
use std::process::Command;

#[test]
fn assert_true() {
    assert!(true);
}

#[should_panic]
#[test]
fn terrorize() {
    panic!("Oh my god!");
}

#[test]
fn runs() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}
