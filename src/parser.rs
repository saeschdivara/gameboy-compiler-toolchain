use std::result::Result::Ok;
use crate::lexer;
use crate::ast;
use crate::lexer::TokenType;

pub struct Parser {
    tokens: Vec<lexer::Token>,
    tokens_number: usize,
    position: usize,
    read_position: usize,
    token: Option<lexer::Token>,
}

#[derive(Debug)]
pub struct ParsingError {
    pub error_message: &'static str,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Self {
        let tokens_number = tokens.len();
        let mut p = Self {
            tokens,
            tokens_number,
            position: 0,
            read_position: 0,
            token: None,
        };

        p.next_token();

        return p;
    }

    pub fn next_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParsingError> {
        self.skip_spaces();

        if self.token.is_none() {
            return Err(ParsingError {
                error_message: "No token left"
            });
        }

        let token = self.token.as_ref().unwrap();

        match token.token_type {
            TokenType::SemiColon => {
                self.skip_comment();
                return self.next_statement();
            }
            TokenType::Identifier => {
                if token.literal.to_lowercase() == "include".to_string() {
                    return self.parse_include();
                } else if token.literal.to_lowercase() == "section".to_string() {
                    return self.parse_section();
                } else if token.literal.to_lowercase() == "if".to_string() {
                    return self.parse_if();
                } else if token.literal.to_lowercase() == "newcharmap".to_string() {
                    return self.parse_new_char_map();
                } else if token.literal.to_lowercase() == "charmap".to_string() {
                    return self.parse_char_map();
                } else if let Some(f) = self.peek_token() {
                    if f.literal.to_lowercase() == "equ" {
                        return self.parse_def()
                    }
                }
            }
            _ => {}
        }

        return Err(ParsingError {
            error_message: "Unsupported token found"
        });
    }

    fn parse_include(&mut self) -> Result<Box<dyn ast::Statement>, ParsingError> {
        self.skip_spaces();

        if let Some(k) = self.token.as_ref() {
            if k.token_type != TokenType::DoubleQuote {
                return Err(ParsingError {
                    error_message: "Missing \" after include"
                });
            }
        }

        let path = self.next_string();

        return Ok(Box::new(ast::IncludeStatement {
            path,
        }));
    }

    fn parse_section(&mut self) -> Result<Box<dyn ast::Statement>, ParsingError> {
        self.skip_spaces();

        if let Some(k) = self.token.as_ref() {
            if k.token_type != TokenType::DoubleQuote {
                return Err(ParsingError {
                    error_message: "Missing \" after section"
                });
            }
        }

        let name = self.next_string();

        if let Some(k) = self.token.as_ref() {
            if k.token_type != TokenType::Comma {
                return Err(ParsingError {
                    error_message: "Missing , after include name"
                });
            }
        }

        // skip comma
        self.next_token();

        self.skip_spaces();

        let section_type = self.token.as_ref().unwrap();

        return Ok(Box::new(ast::SectionStatement {
            name,
            section_type: section_type.literal.clone(),
        }));
    }

    fn parse_if(&mut self) -> Result<Box<dyn ast::Statement>, ParsingError> {
        self.skip_spaces();

        while let Some(tok) = self.token.as_ref() {
            if tok.token_type == TokenType::Identifier && tok.literal.to_lowercase() == "endc" {
                break
            }

            self.next_token();
        }

        self.next_token();

        return Ok(Box::new(
            ast::IfStatement{}
        ));
    }

    fn parse_new_char_map(&mut self) -> Result<Box<dyn ast::Statement>, ParsingError> {
        self.skip_spaces();

        if let Some(tok) = self.token.as_ref() {
            if tok.token_type != TokenType::Identifier {
                return Err(ParsingError {
                    error_message: "No identifier after newcharmap"
                })
            }

            let char_map_name = tok.clone().literal;

            return Ok(Box::new(
                ast::NewCharMapStatement{
                    name: char_map_name
                }
            ));
        }

        return Err(ParsingError {
            error_message: "Invalid token found"
        })
    }

    fn parse_char_map(&mut self) -> Result<Box<dyn ast::Statement>, ParsingError> {
        self.skip_spaces();

        let value = self.next_string();

        if let Some(k) = self.token.as_ref() {
            if k.token_type != TokenType::Comma {
                return Err(ParsingError {
                    error_message: "Missing , after charmap value"
                });
            }
        }

        // skip comma
        self.next_token();

        self.skip_spaces();

        if let Some(tok) = self.token.as_ref() {
            if tok.token_type != TokenType::Number {
                return Err(ParsingError {
                    error_message: "Missing number after charmap value"
                });
            }

            return Ok(Box::new(
                ast::CharMapStatement{
                    value,
                    number: i32::from_str_radix(tok.literal.clone().as_str(), 16).unwrap()
                }
            ));
        }

        return Err(ParsingError {
            error_message: "Invalid token found"
        })
    }

    fn parse_def(&mut self) -> Result<Box<dyn ast::Statement>, ParsingError> {
        let def_name = self.token.as_ref().unwrap().literal.clone();
        self.skip_spaces();

        // skip equ
        self.next_token();
        self.skip_spaces();

        let mut value = String::new();

        while let Some(tok) = self.token.as_ref() {
            if tok.token_type == TokenType::LineBreak {
                break;
            }

            value.push_str(tok.literal.clone().as_str());

            self.next_token();
        }

        return Ok(Box::new(ast::DefStatement {
            name: def_name,
            value,
        }))
    }

    fn skip_spaces(&mut self) {
        self.next_token();

        while let Some(tok) = self.token.as_ref() {
            if tok.token_type != TokenType::Space && tok.token_type != TokenType::LineBreak && tok.token_type != TokenType::Tab {
                break;
            }

            self.next_token();
        }
    }

    fn skip_comment(&mut self) {
        self.next_token();

        while let Some(tok) = self.token.as_ref() {
            if tok.token_type == TokenType::LineBreak {
                break;
            }

            self.next_token();
        }
    }

    fn next_string(&mut self) -> String {
        // skip quote
        self.next_token();

        let mut data = "".to_string();

        while let Some(tok) = self.token.as_ref() {
            if tok.token_type == TokenType::DoubleQuote {
                break;
            }

            data += &*tok.literal;
            self.next_token();
        }

        // skip quote
        self.next_token();

        return data;
    }

    fn next_token(&mut self) {
        if self.read_position >= self.tokens_number {
            self.token = None
        } else {
            self.token = Some(self.tokens[self.position].clone());
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_token(&self) -> Option<lexer::Token> {
        return if self.read_position >= self.tokens_number {
            None
        } else {
            Some(self.tokens[self.read_position].clone())
        };
    }
}

pub fn parse_ast(tokens: Vec<lexer::Token>) -> Result<ast::Ast, ParsingError> {
    let mut parser = Parser::new(tokens);
    let mut statements = vec![];
    let mut stmt = parser.next_statement();
    while stmt.is_ok() && parser.token.is_some() {
        statements.push(stmt.unwrap());
        stmt = parser.next_statement();
    }

    if stmt.is_err() {
        println!("Error: {}", stmt.err().unwrap().error_message)
    }

    return Ok(ast::Ast {
        statements,
    });
}
