use std::fs::File;
use std::io::{BufReader, ErrorKind, Read, Write};
use crate::compression::decompress_data;
use crate::error::MtarError;
use crate::format::MyFile;

/*
    write_section(&(name_bytes.len() as u64).to_le_bytes(), archive_file)?;
    write_section(name_bytes, archive_file)?;
    write_section(&file.size.to_le_bytes(), archive_file)?;
    write_section(&(compressed_data.len() as u64).to_le_bytes(), archive_file)?;
    write_section(&compressed_data, archive_file)?;
enum Stages{
    ReadNameLen,
    ReadName,
    ReadSize,
    ReadData,
    WriteToOriginal
}
 */

pub fn extract_files(archive_name: String) -> Result<u32, MtarError>{
    let archive = match File::open(archive_name.clone()){
        Ok(f) => f,
        Err(e) => return Err(MtarError::File(e.to_string()))
    };
    decode_mtar(archive)?;

    Ok(1)
}

pub fn decode_mtar(file: File) -> Result<(), MtarError>{
    let mut reader = BufReader::new(file);

    loop {
        let mut buf = [0u8; 8];
        match reader.read_exact(&mut buf) {
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(MtarError::Extract(format!("Error on read namelen {e}")))
        }

        let name_len= u64::from_le_bytes(buf) as usize;

        let mut namebuf = vec![0u8; name_len];
        match reader.read_exact(&mut namebuf) {
            Ok(_) => {}
            Err(e) => return Err(MtarError::Extract(format!("Error on read name {e}")))
        }

        let filename = match String::from_utf8(namebuf){
            Ok(f) => f,
            Err(e) => return Err(MtarError::Extract(e.to_string()))
        };

        match reader.read_exact(&mut buf) {
            Ok(_) => {}
            Err(e) => return Err(MtarError::Extract(format!("Error on read filesize {e}")))
        }

        let compressed_len = u64::from_le_bytes(buf) as usize;

        let mut compressed = vec![0u8; compressed_len];
        match reader.read_exact(&mut compressed) {
            Ok(_) => {}
            Err(e) => return Err(MtarError::Extract(format!("Error on read file {e}")))
        };

        let data = decompress_data(&*compressed)?; // My favorite symbol

        let mut out = match File::create(filename) {
            Ok(f) => f,
            Err(e) => return Err(MtarError::Extract(e.to_string()))
        };

        match out.write_all(&data) {
            Ok(_) => {}
            Err(e) => return Err(MtarError::Extract(e.to_string()))
        }
    }
    Ok(())

    /*
    An Exhibit in antagonistic rust programming :p
    let mut stage = Stages::ReadNameLen;
    let mut i = 0;
    let mut name_len = 0;
    let mut data_len = 0;
    let mut filename = String::new();
    let mut data: Vec<u8> = Vec::new();

    loop {
        match reader.bytes().next() {
            Some(Ok(byte)) => {
                match stage {
                    Stages::ReadNameLen => {
                        println!("Name Len {byte}");
                        name_len = byte;
                        stage = Stages::ReadName;
                    },
                    Stages::ReadName => {
                        println!("Name: {byte}");
                        i += 1;
                        if i > name_len {
                            i = 0;
                            stage = Stages::ReadSize;
                        } else {
                            filename.push_str(byte.to_string().as_str());
                        }
                    },
                    Stages::ReadSize => {
                        println!("Size: {byte}");
                        data_len = byte;
                        stage = Stages::ReadData;
                    },
                    Stages::ReadData => {
                        println!("Data: {byte}");
                        i += 1;
                        if i > name_len {
                            i = 0;
                            stage = Stages::WriteToOriginal;
                        } else {
                            data.push(byte);
                        }
                    },
                    Stages::WriteToOriginal => {
                        let mut file = match File::create(filename) {
                            Ok(f) => f,
                            Err(e) => return Err(MtarError::File(e.to_string()))
                        };
                        match file.write_all(data.as_slice()) {
                            Ok(_) => (),
                            Err(e) => return Err(MtarError::File(e.to_string()))
                        };
                        stage = Stages::ReadNameLen;
                    }
                }
            },
            None => return Ok(files),
            Some(Err(e)) => { return Err(MtarError::Extract(format!("Error on extract {e}"))) }
        }
    }

     */
}