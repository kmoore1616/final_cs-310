use crate::error;
use crate::error::MtarError;

pub enum Mode{
    Archive,
    Extract,
}

/*
    AppFunction:
    Formatted user input into what is needed to archive or extract
 */
#[derive(Debug)]
pub enum AppFunction {
    Archive {
        archive_name: String, // Name of archive out
        files: Vec<String>, // filenames
    },
    Extract {
        archive: String, // Name of archive to unpack
    }
}

/*
    Parse Argument Variables: Determines how to parse user input 
    Input: arguments, a list of strings that were submitted when program was ran
    Output: AppFunction or MtarError
 */
pub fn parse_argv(arguments: Vec<String>) -> Result<AppFunction, MtarError> {
    if arguments.len() < 2 {
        return Err(MtarError::Usage(String::from("Not enough arguments")))
    }

    let usage_string = arguments[1].clone();

    match usage_string.as_str() {
        "-a" => parse_args(arguments, Mode::Archive),
        "-x" => parse_args(arguments, Mode::Extract),
        _ => Err(MtarError::Usage(usage_string))
    }
}

/*
    Parse arguments: 
    Input: Arguments, a list of strings (see above ^), and mode the mode that the application is in
 */
pub fn parse_args(arguments: Vec<String>, mode: Mode) -> Result<AppFunction, MtarError> {
    match mode {
        Mode::Archive => {
            // mtar -a file.mtar file1 [file2 ...]
            if arguments.len() < 4 {
                return Err(MtarError::Archive(String::from("Not enough arguments")));
            }

            let archive_name = arguments[2].clone();
            let files = arguments[3..].to_vec();

            Ok(AppFunction::Archive {
                archive_name,
                files,
            })
        }

        Mode::Extract => {
            // mtar -x file.mtar
            if arguments.len() != 3 {
                return Err(MtarError::Extract(String::from("Not enough arguments")));
            }

            let archive = arguments[2].clone();

            Ok(AppFunction::Extract {
                archive,
            })
        }
    }
}
