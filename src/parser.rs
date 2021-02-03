use crate::json::Json;
use crate::lexer::Lexer;
use crate::tokens::Token;

use std::any::Any;
use std::collections::HashMap;

pub struct Parser<'lexer> {
    lexer: Lexer<'lexer>,
}

#[derive(Debug)]
pub enum ParseError {
    ExpectedIdent,
    ExpectedData,
    ExpectedChar(char),
    UnexpectedToken(Token),
    UnexpectedEOF,
}

impl<'lexer> Parser<'lexer> {
    pub fn new<'a>(src: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(src),
        }
    }

    pub fn parse(&mut self) -> Result<Json, ParseError> {
        let mut dict = HashMap::new();

        if let Some(token) = self.lexer.lex() {
            match token {
                Token::LParen => loop {
                    if let Some(token) = self.lexer.lex() {
                        match token {
                            Token::Ident(ident) => {
                                let pair = self.parse_pair(ident)?;
                                dict.insert(pair.0, pair.1);
                            }
                            Token::LParen => break,
                            token => return Err(ParseError::UnexpectedToken(token)),
                        }
                    } else {
                        return Err(ParseError::UnexpectedEOF);
                    }
                    match self.lexer.lex() {
                        Some(Token::Comma) => {}
                        Some(Token::RParen) => break,
                        _ => return Err(ParseError::ExpectedChar(',')),
                    }
                },
                _ => {}
            }
        }

        Ok(dict.into())
    }

    pub fn parse_pair(&mut self, ident: String) -> Result<(String, Box<dyn Any>), ParseError> {
        match self.lexer.lex() {
            Some(Token::Colon) => match self.lexer.lex() {
                Some(token) => {
                    let data = self.parse_data(token)?;
                    return Ok((ident, data));
                }
                _ => return Err(ParseError::ExpectedData),
            },
            _ => return Err(ParseError::ExpectedChar(':')),
        }
    }

    pub fn parse_data(&mut self, token: Token) -> Result<Box<dyn Any>, ParseError> {
        match token {
            Token::Str(s) => Ok(Box::new(s)),
            Token::Number(n) => Ok(Box::new(n)),
            Token::Float(f) => Ok(Box::new(f)),
            Token::Bool(b) => Ok(Box::new(b)),
            Token::Null => Ok(Box::new(Token::Null)),
            Token::ArrayLParen => {
                let mut arr: Box<Vec<Box<dyn Any>>> = Box::new(Vec::new());
                loop {
                    match self.lexer.lex() {
                        Some(Token::Str(x)) => arr.push(Box::new(x)),
                        Some(Token::Number(x)) => arr.push(Box::new(x)),
                        Some(Token::Float(x)) => arr.push(Box::new(x)),
                        Some(Token::Bool(x)) => arr.push(Box::new(x)),
                        Some(Token::ArrayRParen) => break,
                        _ => return Err(ParseError::ExpectedData),
                    }
                    match self.lexer.lex() {
                        Some(Token::ArrayRParen) => break,
                        Some(Token::Comma) => continue,
                        _ => return Err(ParseError::ExpectedChar(',')),
                    }
                }
                Ok(arr)
            }
            _ => Ok(Box::new(self.parse()?)),
        }
    }
}

// TODO: write some tests here.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_parse1() {
        let mut parser = Parser::new(r#"{ name: "Mr. John" }"#);
        let dict = parser.parse().unwrap();
        let x = dict.get::<String>("name").unwrap().clone();
        assert_eq!(x, "Mr. John".to_string());
    }
}
