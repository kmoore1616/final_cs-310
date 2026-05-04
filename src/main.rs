use crate::archive::archive_files;
use crate::extract::extract_files;

mod archive;
mod cli;
mod error;
mod extract;
mod format;
mod metadata;
mod compression;

// All main does is get program started and report errors along the way
fn main() {
    match run() {
        Ok(files_processed) => println!("{} Files Processed!", files_processed),
        Err(e) => {
            handle_error(e);
        }
    }
}

/*
Run: Organizes all module calls based on user input
Input: Nothing
Output: Either number of files processed, or Mtar Error
 */
fn run() -> Result<u32, error::MtarError>{
    let arguments: Vec<String> = std::env::args().collect(); // char ** argv
    let app_function= cli::parse_argv(arguments)?;


    match app_function {
        cli::AppFunction::Archive { archive_name: archive, files} =>
            {
                Ok(archive_files(archive, files)?)
            }
        cli::AppFunction::Extract {archive} =>
            {
                Ok(extract_files(archive)?)
            }
    }
}

/*
Helper function that breaks out error handling from main 
 */
fn handle_error(e: error::MtarError) {
    eprintln!("Error: {}", match e {
        error::MtarError::Usage(e)   => format!("Bad Usage! {e}"),
        error::MtarError::Archive(e) => format!("Archive Error: {e}"),
        error::MtarError::File(e)    => format!("File Error: {e}"),
        error::MtarError::Extract(e) => format!("Extract Error: {e}"),
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn archive_single_file_returns_one() {
        fs::write("test_file1.txt", "hello").unwrap();

        let result = archive_files(
            "test_one.mtar".to_string(),
            vec!["test_file1.txt".to_string()],
        );

        assert_eq!(result.unwrap(), 1);

        fs::remove_file("test_file1.txt").ok();
        fs::remove_file("test_one.mtar").ok();
    }

    #[test]
    fn archive_two_files_returns_two() {
        fs::write("test_file1.txt", "hello").unwrap();
        fs::write("test_file2.txt", "world").unwrap();

        let result = archive_files(
            "test_two.mtar".to_string(),
            vec![
                "test_file1.txt".to_string(),
                "test_file2.txt".to_string(),
            ],
        );

        assert_eq!(result.unwrap(), 2);

        fs::remove_file("test_file1.txt").ok();
        fs::remove_file("test_file2.txt").ok();
        fs::remove_file("test_two.mtar").ok();
    }

}