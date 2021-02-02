use crate::lexer::Lexer;
use crate::tokens::Tokens;

use std::any::Any;
use std::collections::{HashMap, VecDeque};

struct Parser<'lexer> {
    lexer: Lexer<'lexer>,
    stack: VecDeque<Tokens>,
}

enum ParseError {
    ExpectedIdent,
    ExpectedData,
    ExpectedToken(char),
}

impl<'lexer> Parser<'lexer> {
    pub fn new<'a>(src: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(src),
            stack: VecDeque::new(),
        }
    }

    pub fn parse_pair(&mut self, ident: String) -> Result<(String, Box<dyn Any>), ParseError> {
        match self.lexer.lex() {
            Some(Tokens::Colon) => match self.lexer.lex() {
                Some(token) => {
                    let data = self.parse_data(token)?;
                    return Ok((ident, data));
                }
                _ => return Err(ParseError::ExpectedData),
            },
            _ => return Err(ParseError::ExpectedToken(':')),
        }
    }

    pub fn parse_data(&mut self, token: Tokens) -> Result<Box<dyn Any>, ParseError> {
        match token {
            Tokens::Str(s) => Ok(Box::new(s)),
            Tokens::Number(n) => Ok(Box::new(n)),
            Tokens::Float(f) => Ok(Box::new(f)),
            Tokens::Bool(b) => Ok(Box::new(b)),
            Tokens::Null => Ok(Box::new(Tokens::Null)),
            Tokens::ArrayLParen => {

            }
            _ => Err(ParseError::ExpectedData),
        }
    }
}

// TODO: write some tests here.
