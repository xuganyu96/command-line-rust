//! Structs and functions commonly used by integration tests
use assert_cmd::assert::IntoOutputPredicate;
use assert_cmd::Command;
use predicates::Predicate;
use std::error::Error;

pub type TestResult = Result<(), Box<dyn Error>>;

/// Execute the input cargo binary once using the specified inputs and compare
/// with the specified outputs
pub fn test_cargo_bin<I, J, P>(
    setup: Box<dyn FnOnce() -> TestResult>,
    cleanup: Box<dyn FnOnce()>,
    bin: &str,
    args: &[&str],
    stdin: &str,
    success: bool,
    stdout_pred: I,
    stderr_pred: J,
) -> TestResult
where
    I: IntoOutputPredicate<P>,
    J: IntoOutputPredicate<P>,
    P: Predicate<[u8]>,
{
    setup()?;
    let mut assertion = Command::cargo_bin(bin)?
        .args(args)
        .write_stdin(stdin)
        .assert();
    if success {
        assertion = assertion.try_success()?;
    } else {
        assertion = assertion.try_failure()?;
    }

    assertion = assertion.try_stdout(stdout_pred)?;
    _ = assertion.try_stderr(stderr_pred)?;

    cleanup();
    return Ok(());
}