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

/// Test various flag combinations against some simple strings fed through
/// stdin
#[test]
fn count_stdin() -> TestResult {
    test_bin(
        Command::cargo_bin("wc")?,
        &[],
        "Hello, world!\n",
        true,
        "       1       2      14 \n", // WHY?
        "", None, None
    )?;

    test_bin(
        Command::cargo_bin("wc")?,
        &["-lm"],
        "Hello, world!\n",
        true,
        "       1      14 \n",
        "", None, None
    )?;

    test_bin(
        Command::cargo_bin("wc")?,
        &["-wc"],
        "Hello, world!\n",
        true,
        "       2      14 \n",
        "", None, None
    )?;

    return Ok(());
}

/// Count the words of multiple files, summing the results at the end
#[test]
fn count_multiple_files() -> TestResult {
    test_bin(
        Command::cargo_bin("wc")?,
        &["tests/wc/haiku.txt", "tests/wc/haiku.txt"],
        "",
        true,
        "       3      11      48 tests/wc/haiku.txt
       3      11      48 tests/wc/haiku.txt
       6      22      96 total\n",
        "", None, None
    )?;
    return Ok(());
}

/// Count the words of files that contain multi-byte characters
#[test]
fn count_nonascii() -> TestResult {
    return test_bin(
        Command::cargo_bin("wc")?,
        &["-lwm", "tests/wc/nonascii.txt"],
        "",
        true,
        "       4       4      16 tests/wc/nonascii.txt\n",
        "", None, None
    );
}

/// Input is a file that does not exist
/// TODO: when there are multiple files with one that's missing, the rest of
/// the input should still be counted
#[test]
fn count_does_not_exist() -> TestResult {
    return test_bin(
        Command::cargo_bin("wc")?,
        &["does-not-exist"],
        "",
        false,
        "",
        "wc: does-not-exist: No such file or directory (os error 2)\n",
        None, None,
    );
}
