use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::path::PathBuf;

use bzip2::read::BzDecoder;
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

        if tree_magic_mini::match_filepath("text/plain", &path) {
            Ok(Box::new(BufReader::new(f)))
        } else {
            // Note that a buffer of 512 bytes will fail to correctly detect
            // plain text of less than 512 bytes. I don't expect a less than
            // 512 bytes text file to be compressed...
            let mut magic_buffer: [u8; 512] = [0; 512];
            // Safe unwrap because the file is already tested.
            let mut mime_type  = tree_magic_mini::from_filepath(&path).unwrap();
            let mut compressed = false;

            // TODO: Clean all that...

            if tree_magic_mini::match_filepath("application/gzip", &path) {
                compressed = true;

                let mut d = GzDecoder::new(f);
                d.read(&mut magic_buffer)?;

                if tree_magic_mini::match_u8("text/plain", &magic_buffer) {
                    // We re-open the file and the decompress stream because
                    // we want to go back to the file start. If we use
                    // d.get_mut().seek(), we corrupt the stream.
                    let f = File::open(&path)?;
                    let d = GzDecoder::new(f);
                    return Ok(Box::new(BufReader::new(d)));
                } else {
                    mime_type = tree_magic_mini::from_u8(&magic_buffer);
                }
            } else if tree_magic_mini::match_filepath("application/x-bzip2", &path) {
                compressed = true;

                let mut d = BzDecoder::new(f);
                d.read(&mut magic_buffer)?;

                if tree_magic_mini::match_u8("text/plain", &magic_buffer) {
                    // Idem here
                    let f = File::open(&path)?;
                    let d = BzDecoder::new(f);
                    return Ok(Box::new(BufReader::new(d)));
                } else {
                    mime_type = tree_magic_mini::from_u8(&magic_buffer);
                }
            }

            // If we're here, then neither the file nor its decompressed
            // content is supported.

            let err_msg = if compressed {
                format!("Unsupported compressed content: {}", mime_type)
            } else {
                format!("Unsupported file type: {}", mime_type)
            };

            Err(Error::new(
                ErrorKind::InvalidData,
                err_msg,
            ))
        }
    }
}
