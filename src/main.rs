use crate::archive::archive_files;

mod archive;
mod cli;
mod error;
mod extract;
mod format;
mod metadata;
mod compression;

fn main() {
    match run() {
        Ok(files_processed) => println!("{} Files Processed!", files_processed),
        Err(e) => {
            eprintln!("Error: {}", match e {
                error::MtarError::Usage(e)   => format!("Bad Usage! {e}"),
                error::MtarError::Archive(e) => format!("Archive Error: {e}"),
                error::MtarError::File(e)    => format!("File Error: {e}"),
                error::MtarError::Extract(e) => format!("Extract Error: {e}"),
                error::MtarError::Thread(e)  => format!("Thread Error: {e}"),
            });
        }
    }
}

fn run() -> Result<u32, error::MtarError>{
    let arguments: Vec<String> = std::env::args().collect(); // char ** argv
    let app_function= cli::parse_argv(arguments)?;

    match app_function {
        cli::AppFunction::Archive {archive, files} =>
            {
                Ok(archive_files(archive, files)?)
            }
        _ => Ok(0)
    }
}