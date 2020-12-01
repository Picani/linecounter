use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::path::PathBuf;

use flate2::read::GzDecoder;

/// Count the number of lines in *input*.
///
/// Return it or an error if the data are not valid UTF-8.
fn nb_lines(input: impl BufRead) -> std::io::Result<usize> {
    let mut res = 0;

    for line in input.lines() {
        match line {
            Ok(_) => res += 1,
            Err(e) => return Err(e)
        }
    }

    Ok(res)
}

/// Count the number of lines in the file at *path*.
///
/// Automatically detect the file type, and open/decompress it or return
/// an *InvalidData* error.
pub fn count_lines(path: &PathBuf) -> std::io::Result<usize> {
    if !path.exists() {
        Err(Error::new(ErrorKind::NotFound, "File not found"))
    } else if !path.is_file() {
        Err(Error::new(ErrorKind::InvalidInput, "Not a regular file"))
    } else {
        let f = File::open(&path)?;

        if tree_magic::match_filepath("text/plain", &path) {
            nb_lines(BufReader::new(f))
        } else if tree_magic::match_filepath("application/gzip", &path) {
            let d = GzDecoder::new(f);
            nb_lines(BufReader::new(d))
        } else {
            let mime_type = tree_magic::from_filepath(&path);
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unsupported file type: {}", mime_type),
            ))
        }
    }
}
