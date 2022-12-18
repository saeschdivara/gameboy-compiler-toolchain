use std::process::exit;
use std::time::Instant;

#[derive(Debug)]
pub enum TokenType {
    Unknown,

    Space,
    LineBreak,

    Slash,
    DoubleQuote,
    Comma,
    Dot,
    SemiColon,

    Identifier,
}

#[derive(Debug)]
pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

#[derive(Debug)]
pub struct LexingError {}

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub input_size: usize,
    // current position in input (points to current char)
    pub position: usize,
    // current reading position in input (after current char)
    pub read_position: usize,
    // current char under examination
    pub ch: Option<char>,
}

impl Lexer {
    pub fn new(c: String) -> Self {
        let input_size = c.len();

        let mut l = Self {
            input: c.chars().collect::<Vec<char>>(),
            input_size,
            position: 0,
            read_position: 0,
            ch: None,
        };


        l.read_char();

        return l;
    }

    pub fn retrieve_next_token(&mut self) -> Result<Token, LexingError> {
        self.read_char();

        if let Some(c) = self.ch {
            return match c {
                '\n' => Ok(Token {
                    literal: c.to_string(),
                    token_type: TokenType::LineBreak,
                }),
                ' ' => Ok(Token {
                    literal: c.to_string(),
                    token_type: TokenType::Space,
                }),
                '"' => Ok(Token {
                    literal: c.to_string(),
                    token_type: TokenType::DoubleQuote,
                }),
                '.' => Ok(Token {
                    literal: c.to_string(),
                    token_type: TokenType::Dot,
                }),
                ',' => Ok(Token {
                    literal: c.to_string(),
                    token_type: TokenType::Comma,
                }),
                '/' => Ok(Token {
                    literal: c.to_string(),
                    token_type: TokenType::Slash,
                }),
                ';' => Ok(Token {
                    literal: c.to_string(),
                    token_type: TokenType::SemiColon,
                }),
                _ => {
                    return if c.is_alphabetic() {
                        let identifier = self.read_identifier();

                        Ok(Token {
                            literal: identifier,
                            token_type: TokenType::Identifier,
                        })
                    } else {
                        Ok(Token {
                            literal: "".to_string(),
                            token_type: TokenType::Unknown,
                        })
                    };
                }
            };
        }
        return Err(LexingError {});
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = self.ch.unwrap().to_string();

        while let Some(c) = self.peek_char() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }

            self.read_char();

            identifier += &*self.ch.unwrap().to_string();
        }

        self.read_char();
        identifier += &*self.ch.unwrap().to_string();

        return identifier;
    }

    fn peek_char(&self) -> Option<char> {
        let result = if self.read_position >= self.input_size {
            None
        } else {
            Some(self.input[self.read_position])
        };

        return result;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input_size {
            self.ch = None
        } else {
            self.ch = Some(self.input[self.position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

pub fn lex_content(content: Vec<String>) -> Vec<Token> {
    let mut c: String = "".to_string();

    let start = Instant::now();
    for x in content {
        c += &*x;
        c += "\n";
    }
    let duration = start.elapsed();
    println!("Concat strings: {:?}", duration);

    let mut lexer = Lexer::new(c);
    let mut tokens: Vec<Token> = vec![];

    while let Ok(token) = lexer.retrieve_next_token() {
        tokens.push(token);
    }

    return tokens;
}