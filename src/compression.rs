use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::{Read, Write};
use flate2::read::GzDecoder;
use crate::error::MtarError;

/* 
    Compress data: Function that applies compression algo to a byte array
    Input: Byte array to compress
    Output: Resulting compressed byte array on success, mtar error on failure
 */
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

/* 
    DeCompress data: Function that applies decompression algo to a byte array
    Input: Byte array to decompress
    Output: Resulting decompressed byte array on success, mtar error on failure
 */
pub fn decompress_data(data: &[u8]) -> Result<Vec<u8>, MtarError> {
    let mut decoder = GzDecoder::new(data);
    let mut data = Vec::new();
    match decoder.read_to_end(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(MtarError::Extract(e.to_string())),
    }
}