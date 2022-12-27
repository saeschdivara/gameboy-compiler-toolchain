#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenType {
    Unknown,

    Space,
    Tab,
    LineBreak,

    Slash,
    DoubleQuote,
    Comma,
    Dot,
    SemiColon,

    Number,
    Identifier,

    EOF,
}

#[derive(Debug, Clone)]
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
        let characters = c.chars().collect::<Vec<char>>();
        let input_size = characters.len();

        let mut l = Self {
            input: characters,
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
                '\t' => Ok(Token {
                    literal: c.to_string(),
                    token_type: TokenType::Tab,
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
                '$' => Ok(Token {
                    literal: self.read_number(),
                    token_type: TokenType::Number,
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
                            literal: c.to_string(),
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
        let mut changed = false;

        while let Some(c) = self.peek_char() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }

            changed = true;
            self.read_char();
            identifier += &*self.ch.unwrap().to_string();
        }

        if changed {
            self.read_char();

            if self.ch.is_some() {
                identifier += &*self.ch.unwrap().to_string();
            }
        }

        return identifier;
    }

    fn read_number(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(c) = self.peek_char() {
            if !c.is_ascii_hexdigit() {
                break;
            }

            self.read_char();

            identifier += &*self.ch.unwrap().to_string();
        }

        self.read_char();

        if self.ch.is_some() {
            identifier += &*self.ch.unwrap().to_string();
        }

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

    for x in content {
        c += &*x;
        c += "\n";
    }

    let mut lexer = Lexer::new(c);
    let mut tokens: Vec<Token> = vec![];

    while let Ok(token) = lexer.retrieve_next_token() {
        tokens.push(token);
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexing_content() {
        let mut l = Lexer::new(concat!("INCLUDE \"foo.asm\"\n", "; simple comment").to_string());

        let expected_tokens = vec![
            Token {
                literal: "INCLUDE".to_string(),
                token_type: TokenType::Identifier,
            },
            Token {
                literal: " ".to_string(),
                token_type: TokenType::Space,
            },
            Token {
                literal: "\"".to_string(),
                token_type: TokenType::DoubleQuote,
            },
            Token {
                literal: "foo".to_string(),
                token_type: TokenType::Identifier,
            },
            Token {
                literal: ".".to_string(),
                token_type: TokenType::Dot,
            },
            Token {
                literal: "asm".to_string(),
                token_type: TokenType::Identifier,
            },
            Token {
                literal: "\"".to_string(),
                token_type: TokenType::DoubleQuote,
            },
            Token {
                literal: "\n".to_string(),
                token_type: TokenType::LineBreak,
            },
            Token {
                literal: ";".to_string(),
                token_type: TokenType::SemiColon,
            },
            Token {
                literal: " ".to_string(),
                token_type: TokenType::Space,
            },
            Token {
                literal: "simple".to_string(),
                token_type: TokenType::Identifier,
            },
            Token {
                literal: " ".to_string(),
                token_type: TokenType::Space,
            },
            Token {
                literal: "comment".to_string(),
                token_type: TokenType::Identifier,
            },
        ];

        let mut output_tokens = vec![];
        let mut r = l.retrieve_next_token();

        while r.is_ok() {
            output_tokens.push(r.unwrap());
            r = l.retrieve_next_token();
        }

        assert_eq!(expected_tokens.len(), output_tokens.len());
        for i in 0..expected_tokens.len() {
            let exp_tok: Token = expected_tokens[i].clone();
            let output_tok: Token = expected_tokens[i].clone();

            assert_eq!(exp_tok.literal, output_tok.literal);
            assert_eq!(exp_tok.token_type, output_tok.token_type);
        }
    }
}
