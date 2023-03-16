pub mod libcat;
pub mod libcomm;
pub mod libcut;
pub mod libfind;
pub mod libgrep;
pub mod libhead;
pub mod libtail;
pub mod libuniq;
pub mod libwc;

pub mod common {
    use std::{
        error::Error,
        fs::File,
        io::{self, BufRead, BufReader},
    };

    pub type MyResult<T> = Result<T, Box<dyn Error>>;

    /// Open a file or a stdin and return a buffered reader against it
    /// Upon encountering error, the error will be pre-pended with the path
    pub fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
        let reader: Box<dyn BufRead> = match path {
            "" | "-" => Box::new(BufReader::new(io::stdin())),
            _ => {
                let file = File::open(path).map_err(|e| format!("{path}: {e}"))?;
                Box::new(BufReader::new(file))
            }
        };

        return Ok(reader);
    }
}
