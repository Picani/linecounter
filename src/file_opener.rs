use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::path::PathBuf;

use flate2::read::GzDecoder;


/// Open the file at *path* and return it as a BufReader.
///
/// Automatically detect the file type, and open/decompress it or return
/// an *InvalidData* error.
pub fn open(path: &PathBuf) -> std::io::Result<Box<dyn BufRead>> {
    if !path.exists() {
        Err(Error::new(ErrorKind::NotFound, "File not found"))
    } else if !path.is_file() {
        Err(Error::new(ErrorKind::InvalidInput, "Not a regular file"))
    } else {
        let f = File::open(&path)?;

        if tree_magic::match_filepath("text/plain", &path) {
            Ok(Box::new(BufReader::new(f)))
        } else if tree_magic::match_filepath("application/gzip", &path) {
            let d = GzDecoder::new(f);
            Ok(Box::new( BufReader::new(d)))
        } else {
            let mime_type = tree_magic::from_filepath(&path);
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unsupported file type: {}", mime_type),
            ))
        }
    }
}
