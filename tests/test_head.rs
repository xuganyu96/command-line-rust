//! Test head:
use assert_cmd::Command;
use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader };

type TestResult = Result<(), Box<dyn Error>>;

/// Execute the input Command (system binary or cargo binary) once using the
/// specified inputs and compare with the specified outputs. If filepath is
/// supplied, then the content will be checked against fileout.
///
/// The caller is responsible for supplying the path and the content of the
/// output file in both "args" and "output_path"
fn test_bin(
    mut bin: Command,
    args: &[&str],
    stdin: &str,
    success: bool,
    stdout: &'static str,
    stderr: &'static str,
    output_path: Option<&str>,
    output_str: Option<&'static str>,
) -> TestResult {
    let mut assertion = bin.args(args)
        .write_stdin(stdin)
        .assert();
    if success {
        assertion = assertion.try_success()?;
    } else {
        assertion = assertion.try_failure()?;
    }

    assertion = assertion.try_stdout(stdout)?;
    let _ = assertion.try_stderr(stderr)?;

    // Check file output
    if let Some(output_path) = output_path {
        let output_str = output_str.unwrap(); // assume they are both Some()
        let mut reader = BufReader::new(File::open(output_path)?);
        let mut output = String::new();
        while let Ok(bytes) = reader.read_line(&mut output) {
            if bytes == 0 { break; }
        }
        assert_eq!(output, output_str);
    }

    return Ok(());
}

/// Without any inputs, the head command should print the first ten lines
#[test]
fn single_file() -> TestResult {
    let cmd = Command::cargo_bin("head")?;
    return test_bin(
        cmd,
        &["-n", "4", "tests/head/manylines.txt"],
        "",
        true,
        "0000\n0001\n0002\n0003\n",
        "",
        None, None
    );
}

#[test]
fn byte_flag() -> TestResult {
    let cmd = Command::cargo_bin("head")?;
    return test_bin(
        cmd,
        &["-c", "10", "tests/head/manylines.txt"],
        "",
        true,
        "0000\n0001\n",
        "",
        None, None
    );
}

/// Without any file specified, the head command will listen for inputs
/// from stdin.
#[test]
fn head_stdin() -> TestResult {
    let cmd = Command::cargo_bin("head")?;
    return test_bin(
        cmd,
        &["-n", "4"],
        "0\n1\n2\n3\n4\n5\n6\n7\n8\n9",
        true,
        "0\n1\n2\n3\n",
        "",
        None, None
    );
}

/// When heading multiple files, the content of each file is prefixed with a
/// header and separated by a line break.
#[test]
fn multiple_files() -> TestResult {
    let cmd = Command::cargo_bin("head")?;
    return test_bin(
        cmd,
        &[
            "-n",
            "1",
            "tests/head/manylines.txt",
            "tests/head/manylines.txt",
        ],
        "",
        true,
        "==> tests/head/manylines.txt <==
0000

==> tests/head/manylines.txt <==
0000
",
        "",
        None, None
    );
}

/// If the input file does not exist, output to stderr:
/// "head: does-not-exist: No such file or directory"
#[test]
fn does_not_exist() -> TestResult {
    let cmd = Command::cargo_bin("head")?;
    return test_bin(
        cmd,
        &["does-not-exist"],
        "",
        false,
        "",
        "head: does-not-exist: No such file or directory (os error 2)\n",
        None, None
    );
}
