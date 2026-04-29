use crate::archive::archive_files;

mod archive;
mod cli;
mod error;
mod extract;
mod format;
mod metadata;
fn main() {
    match run() {
        Ok(files_processed) => println!("{} Files Processed!", files_processed),
        Err(e) => { println!("Something Failed {:?}", e) }
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

    // This is the use case for the questiuon mark that just propogates the error up
    //  let returnval = match funct {
    //      Ok(_) => { Ok(0) },
    //      Err(e) => { Err(e) }
    //  };
    //
    // if returnval.is_err() {
    //     return Err(returnval)
    // }

}