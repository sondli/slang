pub mod lexer {
    use core::panic;
    use std::process::exit;
    use phf::{phf_map, Map};

    pub enum TokenTypes {
        // Single-character tokens.
        Minus,
        Plus,
        Semicolon,
        Star,
        LeftParen,
        RightParen,

        // One or two character tokens.
        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,

        // Literals.
        Identifier,
        Number,

        // Keywords.
        Print,
        Let,
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
                TokenTypes::Number => "number".to_string(),
                TokenTypes::Print => "print".to_string(),
                TokenTypes::Let => "let".to_string(),
                TokenTypes::LeftParen => '('.to_string(),
                TokenTypes::RightParen => ')'.to_string(),
                TokenTypes::BangEqual => "!=".to_string(),
                TokenTypes::EqualEqual => "==".to_string(),
                TokenTypes::Greater => '>'.to_string(),
                TokenTypes::GreaterEqual => ">=".to_string(),
                TokenTypes::Less => '<'.to_string(),
                TokenTypes::LessEqual => "<=".to_string(),
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

    pub struct Token {
        pub lexeme: String,
        pub token_type: TokenTypes,
        pub line: usize,
    }

    fn print_error(line: usize, message: String) {
        println!("[line {}] Error: {}", line, message);
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\r' || c == '\t'
    }

    fn scan_symbol(
        source: &Vec<char>,
        lexeme_start: usize,
    ) -> Result<(String, TokenTypes), String> {
        let mut lexeme = String::new();
        let char = match source.get(lexeme_start) {
            Some(char) => char,
            None => return Err(String::from("Out of bounds trying to scan symbol")),
        };
        let next_char_is_equal = match source.get(lexeme_start + 1) {
            Some(next_char) => *next_char == '=',
            None => false,
        };
        let token_type = match char {
            '-' => TokenTypes::Minus,
            '+' => TokenTypes::Plus,
            ';' => TokenTypes::Semicolon,
            '*' => TokenTypes::Star,
            '!' => {
                if next_char_is_equal {
                    TokenTypes::BangEqual
                } else {
                    TokenTypes::Bang
                }
            }
            '=' => {
                if next_char_is_equal {
                    TokenTypes::EqualEqual
                } else {
                    TokenTypes::Equal
                }
            }
            '(' => TokenTypes::LeftParen,
            ')' => TokenTypes::RightParen,
            '>' => {
                if next_char_is_equal {
                    TokenTypes::GreaterEqual
                } else {
                    TokenTypes::Greater
                }
            }
            '<' => {
                if next_char_is_equal {
                    TokenTypes::LessEqual
                } else {
                    TokenTypes::Less
                }
            }
            _ => return Err(format!("Unsupported character: {}", char)),
        };

        lexeme.push(*char);
        if next_char_is_equal {
            lexeme.push('=');
        }

        Ok((lexeme, token_type))
    }

    fn scan_number(
        source: &Vec<char>,
        lexeme_start: usize,
    ) -> Result<(String, TokenTypes), String> {
        let mut lexeme = String::new();
        let mut next_char = match source.get(lexeme_start) {
            Some(char) => *char,
            None => return Err(String::from("Out of bounds trying to scan number")),
        };
        while next_char.is_ascii_digit() {
            lexeme.push(next_char);
            next_char = match source.get(lexeme_start + lexeme.len()) {
                Some(char) => *char,
                None => return Err(String::from("Out of bounds trying to scan number")),
            };
        }
        if next_char == '.' {
            lexeme.push(next_char);
            next_char = match source.get(lexeme_start + lexeme.len()) {
                Some(char) => *char,
                None => return Err(String::from("Out of bounds trying to scan number")),
            };
            while next_char.is_ascii_digit() {
                lexeme.push(next_char);
                next_char = match source.get(lexeme_start + lexeme.len()) {
                    Some(char) => *char,
                    None => return Err(String::from("Out of bounds trying to scan number")),
                };
            }
        }
        return Ok((lexeme, TokenTypes::Number));
    }

    fn scan_alphabetic(
        source: &Vec<char>,
        lexeme_start: usize,
    ) -> Result<(String, TokenTypes), String> {
        let mut lexeme = String::new();
        let mut next_char = match source.get(lexeme_start) {
            Some(next_char) => next_char,
            None => return Err("Out of bounds trying to scan alphabetic token".to_string()),
        };
       while next_char.is_ascii_alphabetic() || next_char.is_ascii_digit() || *next_char == '_' {
            lexeme.push(*next_char);
            next_char = match source.get(lexeme_start + lexeme.len()) {
                Some(next_char) => next_char,
                None => return Err("Out of bounds trying to scan alphabetic token".to_string()),
            };
        }
        return match KEYWORDS.get(lexeme.as_str()) {
            Some(_) => match lexeme.to_token_type() {
                Ok(token_type) => Ok((lexeme, token_type)),
                Err(e) => return Err(e),
            },
            None => Ok((lexeme, TokenTypes::Identifier)),
        };
    }

    fn scan_token(source: &Vec<char>, lexeme_start: usize) -> Result<(String, TokenTypes), String> {
        let start_char = match source.get(lexeme_start) {
            Some(char) => *char,
            None => return Err("Out of bounds trying to scan token".to_string())
        };
        if start_char.is_ascii_digit() {
            return scan_number(source, lexeme_start);
        } else if start_char.is_ascii_alphabetic() || start_char == '_' {
            return scan_alphabetic(source, lexeme_start);
        } else if start_char.is_ascii() {
            return scan_symbol(source, lexeme_start);
        }   
        panic!()
    }

    pub fn scan_source(source: &Vec<char>) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut line = 1;
        let mut skip_n_iterations = 0;

        for (i, current) in source.iter().enumerate() {
            if *current == '\n' {
                line += 1;
                continue;
            }
            if skip_n_iterations > 0 {
                skip_n_iterations -= 1;
                continue;
            }
            if is_whitespace(*current) {
                continue;
            }
            let (lexeme, token_type) = match scan_token(source, i) {
                Ok(tuple) => tuple,
                Err(e) => {
                    print_error(line, e);
                    exit(1);
                }
            };
            skip_n_iterations = lexeme.len() - 1;
            tokens.push(Token {
                lexeme,
                token_type,
                line,
            });
        }

        tokens
    }
}
