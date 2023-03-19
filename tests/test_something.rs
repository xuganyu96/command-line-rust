use std::error::Error;
use std::process::Command;

type TestResult<T> = Result<T, Box<dyn Error>>;

#[test]
fn test_command() -> TestResult<()> {
    let output = Command::new("cat")
        .args(&["does-not-exist"])
        .output()?;

    assert_eq!(&output.stdout, "".as_bytes());
    let stderr_str = String::from_utf8(output.stderr)?;
    assert!(stderr_str.contains("No such file or directory"));
    return Ok(());
}
