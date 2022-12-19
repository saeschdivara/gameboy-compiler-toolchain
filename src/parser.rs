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
            token: None
        };

        p.next_token();

        return p;
    }

    pub fn next_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParsingError> {

        self.skip_spaces();

        if self.token.is_none() {
            return Err(ParsingError {
                error_message: "No token left"
            })
        }

        let token = self.token.as_ref().unwrap();

        match token.token_type {
            TokenType::Unknown => {}
            TokenType::Space => {}
            TokenType::LineBreak => {}
            TokenType::Slash => {}
            TokenType::DoubleQuote => {}
            TokenType::Comma => {}
            TokenType::Dot => {}
            TokenType::SemiColon => {}
            TokenType::Identifier => {
                if token.literal.to_lowercase() == "include".to_string() {
                    self.skip_spaces();

                    if let Some(k) = self.token.as_ref() {
                        if k.token_type != TokenType::DoubleQuote {
                            return Err(ParsingError {
                                error_message: "Missing \" after include"
                            })
                        }
                    }

                    // skip quote
                    let path = self.next_string();

                    return Ok(Box::new(ast::IncludeStatement {
                        path,
                    }))
                }
            }
            TokenType::EOF => {}
        }

        return Err(ParsingError {
            error_message: "Unsupported token found"
        })
    }

    fn skip_spaces(&mut self) {
        self.next_token();

        while let Some(tok) = self.token.as_ref() {
            if tok.token_type != TokenType::Space && tok.token_type != TokenType::LineBreak {
                break
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

        return data
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
        }
    }
}

pub fn parse_ast(tokens: Vec<lexer::Token>) -> Result<ast::Ast, ParsingError> {
    let mut parser = Parser::new(tokens);
    let mut statements = vec![];
    let mut stmt = parser.next_statement();
    while stmt.is_ok() {
        statements.push(stmt.unwrap());
        stmt = parser.next_statement();
    }

    return Ok(ast::Ast {
        statements,
    })
}
