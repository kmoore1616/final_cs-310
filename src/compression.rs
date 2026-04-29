use std::fmt::format;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;
use crate::error::MtarError;

pub fn compress_data(data: &[u8]) -> Result<Vec<u8>, MtarError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    match encoder.write_all(data) {
        Ok(_) => (),
        Err(e) => {
            return Err(MtarError::Archive(format!("Compression failed: {e}")));
        }
    }

    let compressed_data = match encoder.finish() {
        Ok(data) => data,
        Err(e) => {
            return Err(MtarError::Archive(format!("Compression failed: {e}")));
        }
    };

    Ok(compressed_data)
}