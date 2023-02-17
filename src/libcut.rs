use std::{
    error::Error,
    ops::Range,
};

type UniResult<T> = Result<T, Box<dyn Error>>;

/// Parse the input string into a range, where the generic <Idx> will be
/// passed to the appropriate parse method for parsing string into integers.
/// The start and stop indices are separated by a single "-". Preceding zeros
/// are acceptable, but preceding signs +/- are not
fn parse_range<Idx>(input: &str) -> UniResult<Range<Idx>> {
    todo!();
}

pub fn run() -> UniResult<()> {
    println!("Hello, world!");

    let range = Range{ start: 0, end: 10 };
    for num in range {
        println!("{num}");
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Note that while doc-test can also be appropriate, doc tests can only
    /// be applied to public functions/structs/enums since you need to import
    /// them from the top level like "command_line_rust::libcut::parse_range"
    #[test]
    fn test_parse_range() {
        assert!(parse_range::<usize>("0").is_err());
    }
}
