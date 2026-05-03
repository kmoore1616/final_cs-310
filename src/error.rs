#[derive(Debug)]
pub enum MtarError {
    Archive(String),
    Extract(String),
    Usage(String),
    File(String),
}

