pub mod lexer {
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
                TokenTypes::Number => "number".to_string(),
                TokenTypes::Print => "print".to_string(),
                TokenTypes::Let => "let".to_string(),
                TokenTypes::EOF => todo!(),
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

    fn scan_symbol(source: &Vec<char>, lexeme_start: usize, line: &mut usize) -> Token {
        let token_type = match source[lexeme_start] {
            '-' => TokenTypes::Minus,
            '+' => TokenTypes::Plus,
            ';' => TokenTypes::Semicolon,
            '*' => TokenTypes::Star,
            '!' => match source.get(lexeme_start + 1) {
                Some(char) => {
                    if *char == '=' {
                        *line += 1;
                        TokenTypes::BangEqual
                    } else {
                        TokenTypes::Bang
                    }
                }
                None => TokenTypes::Bang,
            }
            '=' => {
                if lexeme_start == source.len() - 1 || source[lexeme_start + 1] != '=' {
                    TokenTypes::Equal
                } else {
                    TokenTypes::EqualEqual
                }
            }
            '(' => TokenTypes::LeftParen,
            ')' => TokenTypes::RightParen,
            '>' => {
                if lexeme_start == source.len() - 1 || source[lexeme_start + 1] != '=' {
                    TokenTypes::Greater
                } else {
                    TokenTypes::GreaterEqual
                }
            }
            '<' => {
                if lexeme_start == source.len() - 1 || source[lexeme_start + 1] != '=' {
                    TokenTypes::Less
                } else {
                    TokenTypes::LessEqual
                }
            }

            _ => panic!("Unsupported symbol"),
        };

        Token {
            lexeme: token_type.to_string(),
            token_type,
            line: *line,
        }
    }

    fn scan_number(source: &Vec<char>, lexeme_start: usize, line: &mut usize) -> Token {
        let mut lexeme_len = 1;
        let mut next_char = source[lexeme_start + 1];
        while next_char.is_ascii_digit() {
            if next_char == '\n' {
                *line += 1;
                break;
            }
            lexeme_len += 1;
            next_char = source[lexeme_start + lexeme_len];
        }
        if next_char == '.' {
            lexeme_len += 1;
            next_char = source[lexeme_start + lexeme_len];
            while next_char.is_ascii_digit() {
                if next_char == '\n' {
                    *line += 1;
                    break;
                }
                lexeme_len += 1;
                next_char = source[lexeme_start + lexeme_len];
            }
        }
        let mut lexeme = String::new();
        let lexeme_end = lexeme_start + lexeme_len;
        for i in lexeme_start..lexeme_end {
            lexeme.push(source[i]);
        }
        return Token {
            lexeme,
            token_type: TokenTypes::Number,
            line: *line,
        };
    }

    fn scan_alphabetic(source: &Vec<char>, lexeme_start: usize, line: &mut usize) -> Token {
        let mut lexeme_len = 1;
        let mut next_char = source[lexeme_start + 1];
        while next_char.is_ascii_alphabetic() || next_char.is_ascii_digit() || next_char == '_' {
            if next_char == '\n' {
                *line += 1;
                break;
            }
            lexeme_len += 1;
            next_char = source[lexeme_start + lexeme_len];
        }
        let mut lexeme = String::new();
        let lexeme_end = lexeme_start + lexeme_len;
        for i in lexeme_start..lexeme_end {
            lexeme.push(source[i]);
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
    }

    fn scan_token(source: &Vec<char>, lexeme_start: usize, line: &mut usize) -> Token {
        let start_char = source[lexeme_start];
        if start_char.is_ascii_digit() {
            return scan_number(source, lexeme_start, line);
        } else if start_char.is_ascii_alphabetic() || start_char == '_' {
            return scan_alphabetic(source, lexeme_start, line);
        } else if start_char.is_ascii() {
            return scan_symbol(source, lexeme_start, line);
        }
        panic!("Unsupported character")
    }

    pub fn scan_source(source: &Vec<char>) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut line = 1;
        let mut skip_n_iterations = 0;

        for (i, current) in source.iter().enumerate() {
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
}
