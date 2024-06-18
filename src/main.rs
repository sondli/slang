use std::{fs::File, io::{self, Read}, path::Path};
use clap::Parser;

/// Slang compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    filename: String
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let _exists = match Path::new(&args.filename).try_exists() {
        Ok(exists) => exists,
        Err(e) => return Err(e)
    };

    let mut file = File::open(args.filename).expect("File not found");
    let mut file_content = String::new();
    let _ = file.read_to_string(&mut file_content);

    println!("{}", file_content);
    Ok(())
}
