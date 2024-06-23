use clap::Parser;
use core::panic;
use phf::{phf_map, Map};
use std::{
    borrow::Borrow,
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

    EOF,
}

static KEYWORDS: Map<&'static str, TokenTypes> = phf_map! {
    "let" => TokenTypes::Let,
    "print" => TokenTypes::Print
};

impl ToString for TokenTypes {
    fn to_string(&self) -> String {
        match self {
            TokenTypes::Minus => '-'.to_string(),
            TokenTypes::Plus => '+'.to_string(),
            TokenTypes::Semicolon => ';'.to_string(),
            TokenTypes::Star => '*'.to_string(),
            TokenTypes::Bang => '!'.to_string(),
            TokenTypes::Equal => '='.to_string(),
            TokenTypes::Identifier => "Identifier".to_string(),
            TokenTypes::Number => todo!(),
            TokenTypes::Print => "print".to_string(),
            TokenTypes::Let => "let".to_string(),
            TokenTypes::EOF => todo!(),
        }
    }
}

trait ToTokenType {
    fn to_token_type(&self) -> Result<TokenTypes, String>;
}

impl ToTokenType for String {
    fn to_token_type(&self) -> Result<TokenTypes, String> {
        match self.as_str() {
            "-" => Ok(TokenTypes::Minus),
            "+" => Ok(TokenTypes::Plus),
            ";" => Ok(TokenTypes::Semicolon),
            "*" => Ok(TokenTypes::Star),
            "!" => Ok(TokenTypes::Bang),
            "=" => Ok(TokenTypes::Equal),
            "print" => Ok(TokenTypes::Print),
            "let" => Ok(TokenTypes::Let),
            _ => Err("Ops".to_string()),
        }
    }
}

struct Token {
    lexeme: String,
    token_type: TokenTypes,
    line: usize,
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

trait TokenStore {
    fn add_token(&mut self, lexeme: String, line: usize);
}

impl TokenStore for Vec<Token> {
    fn add_token(&mut self, lexeme: String, line: usize) {
        let token_type = match lexeme.to_token_type() {
            Ok(t) => t,
            Err(error) => {
                print_error(line, "Unsupported lexeme: ".to_string() + &error);
                panic!();
            }
        };
        self.push(Token {
            lexeme,
            token_type,
            line,
        });
    }
}

fn print_error(line: usize, message: String) {
    println!("[line {}] Error: {}", line, message);
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\t'
}

fn scan_symbol(source: &Vec<char>, lexeme_start: usize, line: usize) -> Token {
    let token_type = match source.get(lexeme_start).expect("derp") {
        '-' => TokenTypes::Minus,
        '+' => TokenTypes::Plus,
        ';' => TokenTypes::Semicolon,
        '*' => TokenTypes::Star,
        '!' => TokenTypes::Bang,
        '=' => TokenTypes::Equal,
        _ => panic!("Unsupported symbol"),
    };

    Token {
        lexeme: token_type.to_string(),
        token_type,
        line,
    }
}

fn scan_token(source: &Vec<char>, lexeme_start: usize, line: &mut usize) -> Token {
    let start_char = source
        .get(lexeme_start)
        .expect("out of bounds when getting token char");
    if start_char.is_ascii_digit() {
        todo!();
    } else if start_char.is_ascii_alphabetic() || *start_char == '_' {
        let mut lexeme_len = 1;
        let mut next_char = source
            .get(lexeme_start + 1)
            .expect("out of bounds getting next char");
        while next_char.is_ascii_alphabetic() || next_char.is_ascii_digit() || *next_char == '_' {
            if *next_char == '\n' {
                *line += 1;
                break;
            }
            lexeme_len += 1;
            next_char = source
                .get(lexeme_start + lexeme_len)
                .expect("out of bounds when getting next char in lexeme");
        }
        let mut lexeme = String::new();
        let lexeme_end = lexeme_start + lexeme_len;
        for i in lexeme_start..lexeme_end {
            lexeme.push(*source.get(i).expect("wops"));
        }
        return match KEYWORDS.get(lexeme.as_str()) {
            Some(_) => {
                let token_type = lexeme.to_token_type().expect("ops");
                Token {
                    lexeme,
                    token_type,
                    line: *line,
                }
            }
            None => Token {
                lexeme,
                token_type: TokenTypes::Identifier,
                line: *line,
            },
        };
    } else if start_char.is_ascii() {
        return scan_symbol(source, lexeme_start, *line);
    }
    panic!("Unsupported character")
}

fn scan_source(source: &Vec<char>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut skip_n_iterations = 0;

    for (i, current) in source.iter().enumerate()  { 
        if skip_n_iterations > 0 {
            skip_n_iterations -= 1;
            continue;
        }
        let lexeme_start = i;
        if is_whitespace(*current) {
            continue;
        }
        if *current == '\n' {
            line += 1;
            continue;
        }
        let token = scan_token(source, lexeme_start, &mut line);
        skip_n_iterations = token.lexeme.len() - 1;
        tokens.push(token);
    }

    tokens
}
