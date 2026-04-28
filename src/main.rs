mod archive;
mod cli;
mod error;
mod extract;
mod format;
mod metadata;
fn main() {
    match run() {
        Ok(bytes) => println!("{} Bytes Processed", bytes),
        Err(e) => { println!("Something Failed {:?}", e) }
    }
}

fn run() -> Result<u32, error::MtarError>{
    let arguments: Vec<String> = std::env::args().collect(); // char ** argv
    let app_function= cli::parse_argv(arguments)?;

    println!("{:?}", app_function);

    // This is the use case for the questiuon mark that just propogates the error up
    //  let returnval = match funct {
    //      Ok(_) => { Ok(0) },
    //      Err(e) => { Err(e) }
    //  };
    //
    // if returnval.is_err() {
    //     return Err(returnval)
    // }

    Ok(0)
}