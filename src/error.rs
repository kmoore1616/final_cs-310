use std::error::Error;

#[derive(Debug)]
pub enum MtarError {
    Archive(String),
    Extract(String),
    Usage(String),
    Thread(String),
    File(String),
}

