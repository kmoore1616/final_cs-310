use std::fs::{File, Permissions};
use std::time::SystemTime;
use crate::error::MtarError;

pub fn get_permissions(file:&File) -> Result<Permissions, MtarError>{
    match file.metadata() {
        Ok(metadata) => { Ok(metadata.permissions()) },
        Err(e) => { Err(MtarError::File("Failed to get permissions".to_string())) }
    }
}

pub fn get_modified(file:&File) -> Result<SystemTime, MtarError>{
    match file.metadata() {
        Ok(metadata) => {match metadata.modified() {
            Ok(modified) => Ok(modified),
            Err(e) => { Err(MtarError::File(format!("Failed to get modified {e}"))) }
        }},
        Err(e) => { Err(MtarError::File(format!("Failed to get metadata {e}"))) }
    }
}

pub fn get_size(file:&File) -> Result<u64, MtarError>{
    match file.metadata() {
        Ok(metadata) => { Ok(metadata.len()) },
        Err(e) => { Err(MtarError::File(format!("Failed to get size of file{e}"))) }
    }
}