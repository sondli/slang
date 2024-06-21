use clap::Parser;
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
    usize,
};

/// Slang compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    filename: String,
}

enum TokenTypes {
    // Single-character tokens.
    Minus,
    Plus,
    Semicolon,
    Star,

    // One or two character tokens.
    Bang,
    Equal,

    // Literals.
    Identifier,
    Number,

    // Keywords.
    Print,
    Let,
}

impl ToString for TokenTypes {
    fn to_string(&self) -> String {
        match self {
            TokenTypes::Minus => '-'.to_string(),
            TokenTypes::Plus => '+'.to_string(),
            TokenTypes::Semicolon => ';'.to_string(),
            TokenTypes::Star => '*'.to_string(),
            TokenTypes::Bang => '!'.to_string(),
            TokenTypes::Equal => '='.to_string(),
            TokenTypes::Identifier => todo!(),
            TokenTypes::Number => todo!(),
            TokenTypes::Print => "print".to_string(),
            TokenTypes::Let => "let".to_string(),
        }
    }
}

trait ToTokenType {
    fn to_token_type(&self) -> TokenTypes;
}

impl ToTokenType for char {
    fn to_token_type(&self) -> TokenTypes {
        match self {
            '-' => TokenTypes::Minus,
            '+' => TokenTypes::Plus,
            ';' => TokenTypes::Semicolon,
            '*' => TokenTypes::Star,
            '!' => TokenTypes::Bang,
            '=' => TokenTypes::Equal,
            _ => panic!()
        }
    }
}

struct Token {
    raw: char,
    token_type: TokenTypes,
    line: usize
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
        println!("char {} with token {} on line {}", token.raw, token.token_type.to_string(), token.line);
    };


    Ok(())
}

trait TokenStore {
    fn add_token(&mut self, c: char, line: usize);
}

impl TokenStore for Vec<Token> {
    fn add_token(&mut self, c: char, line: usize) {
        self.push(Token {
            raw: c,
            token_type: c.to_token_type(),
            line
        })
    }
}

fn is_at_end(current: usize, source: &Vec<u8>) -> bool {
    current >= source.len()
}

fn next_char(source: &Vec<char>, current: &mut usize) -> char {
    let next = source[*current];
    *current += 1;
    next
}

fn scan_source(source: &Vec<char>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line = 1;

    for c in source {
        if *c == ' ' {
            continue;
        }
        if *c == '\n' {
            line += 1;
            continue;
        }
        tokens.add_token(*c, line);
    }

    tokens
}
