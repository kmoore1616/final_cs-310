use crate::format::MyFile;
use std::fs::{metadata, File};
use std::io::{Read, Write};
use crate::error::MtarError;
use crate::metadata;
use crate::metadata::*;



pub fn archive_files(archive_name: String, names: Vec<String>) -> Result<u32, MtarError> {
    let mut archive_file = match File::create(archive_name) {
        Ok(file) => file,
        Err(e) => return Err(MtarError::File(format!("Failed to create file{e}")))
    };
    let mut files_processed = 0;
    for file_name in names{
        let mut file =  match File::open(file_name.clone()){
            Ok(f) => f,
            Err(e) => return Err(MtarError::Archive(format!("{}", e))) // Returns from entire function
        };

        let perms = metadata::get_permissions(&file)?;
        let modified = metadata::get_modified(&file)?;
        let size = metadata::get_size(&file)?;
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(_) => (),
            Err(e) => return Err(MtarError::Archive(format!("{}", e)))
        }


        let file_to_write = MyFile::new(
            file_name,
            perms,
            modified,
            size,
            data
        );

        files_processed +=1;
    }
    Ok(files_processed)
}

pub fn write_to_archive(file: MyFile, archive_file: &mut File) -> Result<(), MtarError> {
    let name_bytes = file.name.as_bytes();
    let mut bytes_written = 0;
    match archive_file.write_all(&(name_bytes.len() as u64).to_le_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(MtarError::Archive(format!("{}", e)))
    }
    match archive_file.write_all(name_bytes) {
        Ok(_) => (),
        Err(e) => return Err(MtarError::Archive(format!("{}", e)))
    }
    match archive_file.write_all(&file.size.to_le_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(MtarError::Archive(format!("{}", e)))
    }
    match archive_file.write_all(&file.data) {
        Ok(_) => (),
        Err(e) => return Err(MtarError::Archive(format!("{}", e)))
    }

    Ok(())

}