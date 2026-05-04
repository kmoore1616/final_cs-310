use std::time::SystemTime;
use std::fs::{Permissions};


/*
    MyFile Object: 
    Organizes all relavant information into a struct
 */
pub struct MyFile {
    pub name: String,
    pub permissions: Permissions,
    pub modified: SystemTime,
    pub size: u64,
    pub data: Vec<u8>,
}

impl MyFile {
    pub fn new(name: String, permissions: Permissions, modified: SystemTime, size: u64, data: Vec<u8>) -> MyFile {
        MyFile {
            name,
            permissions,
            modified,
            size,
            data
        }
    }
}