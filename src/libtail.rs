//! Library for the tail program
use std::{
    io::{ Read, Seek, SeekFrom, BufRead },
    error::Error,
};

type MyResult<T> = Result<T, Box<dyn Error>>;

// TODO: Implement Args
// TODO: parse Args into the appropriate TakeValue
// TODO: match TakeValue to the appropriate function call

/// Reader from the specified byte location using 0-based indexing. If the
/// starting location is such that no additional bytes can be read, then
/// return empty string
fn read_bytes_from<T>(
    reader: &mut T,
    loc: u64,
) -> MyResult<String> 
where T: Read + Seek {
    let mut buffer = String::new();
    reader.seek(SeekFrom::Start(loc))?;
    reader.read_to_string(&mut buffer)?;
    return Ok(buffer);
}

/// Read the last a few bytes based on the input number (which must not be
/// positive)
fn tail_n_bytes<T>(
    reader: &mut T,
    loc: i64,
) -> MyResult<String>
where T: Read + Seek {
    let mut buffer = String::new();
    reader.seek(SeekFrom::End(loc))?;
    reader.read_to_string(&mut buffer)?;

    return Ok(buffer);
}

/// Read lines from the specified location using 0-based indexing
fn read_lines_from<T>(
    reader: &mut T,
    n: usize,
) -> MyResult<String> 
where T: BufRead {
    let lines = reader.lines()
        .skip(n)
        .filter_map(|line_or_err| {
            if let Ok(line) = line_or_err {
                return Some(line);
            }
            return None;
        })
        .collect::<Vec<String>>()
        .join("\n");

    return Ok(lines);
}

/// Read the last N lines. This implementation relies on knowing the total
/// number of lines and calculating the number of lines to skip
fn tail_n_lines<T>(
    reader: &mut T,
    n: usize,
) -> MyResult<String> 
where T: BufRead + Seek {
    let n_lines = reader.lines().count();
    reader.seek(SeekFrom::Start(0))?;  // reset the reader's position
    if n > n_lines {
        return read_lines_from(reader, 0);
    } else {
        return read_lines_from(reader, n_lines - n);
    }
}

pub fn run() -> MyResult<i32> {
    return Ok(0);
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_read_bytes_from() {
        let mut cursor = Cursor::new("Hello, world");
        assert_eq!(read_bytes_from(&mut cursor, 0).unwrap(), "Hello, world");
        assert_eq!(read_bytes_from(&mut cursor, 1).unwrap(), "ello, world");
        assert_eq!(read_bytes_from(&mut cursor, 7).unwrap(), "world");
        assert_eq!(read_bytes_from(&mut cursor, 12).unwrap(), "");
        assert_eq!(read_bytes_from(&mut cursor, 14).unwrap(), "");
    }

    #[test]
    fn test_tail_n_bytes() {
        let mut cursor = Cursor::new("0123456789");
        assert_eq!(tail_n_bytes(&mut cursor, 0).unwrap(), "");
        assert_eq!(tail_n_bytes(&mut cursor, -1).unwrap(), "9");
        assert_eq!(tail_n_bytes(&mut cursor, -7).unwrap(), "3456789");
        assert_eq!(tail_n_bytes(&mut cursor, -10).unwrap(), "0123456789");
        assert!(tail_n_bytes(&mut cursor, -11).is_err());
    }

    #[test]
    fn test_read_line_froms() {
        let mut cursor = Cursor::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
        assert_eq!(read_lines_from(&mut cursor, 0).unwrap(), "0\n1\n2\n3\n4\n5\n6\n7\n8\n9");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(read_lines_from(&mut cursor, 1).unwrap(), "1\n2\n3\n4\n5\n6\n7\n8\n9");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(read_lines_from(&mut cursor, 9).unwrap(), "9");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(read_lines_from(&mut cursor, 10).unwrap(), "");
    }

    #[test]
    fn test_tail_n_lines() {
        let mut cursor = Cursor::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
        assert_eq!(tail_n_lines(&mut cursor, 0).unwrap(), "");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(tail_n_lines(&mut cursor, 1).unwrap(), "9");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(tail_n_lines(&mut cursor, 10).unwrap(), "0\n1\n2\n3\n4\n5\n6\n7\n8\n9");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(tail_n_lines(&mut cursor, 11).unwrap(), "0\n1\n2\n3\n4\n5\n6\n7\n8\n9");
    }
}
