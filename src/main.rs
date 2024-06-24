use std::{fs::File, io::{self, Read}, path::Path};

use clap::Parser;
use lexer::lexer::scan_source;

mod lexer;

/// Slang compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    filename: String,
}


fn main() -> io::Result<()> {
    let args = Args::parse();
    let _exists = match Path::new(&args.filename).try_exists() {
        Ok(exists) => exists,
        Err(e) => return Err(e),
    };

    let mut file = File::open(args.filename).expect("File not found");
    let mut file_content = Vec::new();
    let _ = file.read_to_end(&mut file_content);
    let source_chars: Vec<char> = file_content.iter().map(|c| *c as char).collect();

    let tokens = scan_source(&source_chars);

    for token in tokens {
        println!(
            "lexeme {} with token {} on line {}",
            token.lexeme,
            token.token_type.to_string(),
            token.line
        );
    }

    Ok(())
}


