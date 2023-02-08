//! Integration tests for uniq, including the following variations:
//! input can come from stdin or some file
//! output can go to stdout or some file
//! use the "count" flag or not
//! several different inputs
use std::{
    fs::File,
    io::{ BufReader, BufRead },
    error::Error
};
use tempfile::NamedTempFile;
use assert_cmd::Command;

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

#[test]
fn cities() -> TestResult {
    let outfile = NamedTempFile::new()?;
    let cmd = Command::new("uniq");

    test_bin(
        cmd,
        &["-", outfile.path().to_str().unwrap()],
        "Madrid\nMadrid\nLisbon\n",
        true,
        "",
        "",
        Some(outfile.path().to_str().unwrap()),
        Some("Madrid\nLisbon\n"),
    )?;

    return Ok(());
}
