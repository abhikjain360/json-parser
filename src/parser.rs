use crate::values::Value;

use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

pub struct Parser<'a> {
    src: Peekable<Chars<'a>>,
    pos: usize,
    len: usize,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedChar(char),
    ExpectedChar(char),
}

impl<'a> Parser<'a> {
    pub fn new<'b: 'a>(src: &'b str) -> Self {
        Parser {
            pos: 0,
            len: src.len(),
            src: src.chars().peekable(),
        }
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        self.skip_whitespaces();

        let ch = *self.peek()?;
        match ch {
            '{' => self.parse().map(|map| Value::Object(map)),
            '[' => {
                self.next().unwrap();
                self.skip_whitespaces();

                let mut v = Vec::new();

                while let Ok(ch) = self.peek() {
                    if *ch == ']' {
                        self.next().unwrap();
                        break;
                    } else {
                        v.push(self.parse_value()?);
                    }

                    self.skip_whitespaces();
                    if let Ok(ch) = self.peek() {
                        if *ch == ',' {
                            self.next().unwrap();
                        }
                    }
                }

                Ok(Value::Array(v))
            }
            '"' => {
                self.next().unwrap();
                let mut s = String::new();

                while let Ok(ch) = self.next() {
                    if ch == '"' {
                        break;
                    }
                    s += &ch.to_string();
                }

                Ok(Value::Str(s))
            }
            't' | 'f' | 'n' => {
                let mut s = String::from(self.next().unwrap());

                while let Ok(ch) = self.next() {
                    if !ch.is_ascii_alphabetic() {
                        break;
                    }
                    s += &ch.to_string();
                }

                match s.as_str() {
                    "true" => Ok(Value::Bool(true)),
                    "false" => Ok(Value::Bool(false)),
                    "null" => Ok(Value::Null),
                    _ => Err(ParseError::UnexpectedChar(ch)),
                }
            }
            '0'..='9' => {
                let mut s = String::from(self.next().unwrap());
                let mut is_float = false;

                while let Ok(ch) = self.peek() {
                    if *ch == '.' {
                        is_float = true;
                        s += &ch.to_string();
                    } else if ch.is_numeric() {
                        s += &ch.to_string();
                    } else {
                        break;
                    }
                    self.next().unwrap();
                }

                if is_float {
                    Ok(Value::Float(s.parse().unwrap()))
                } else {
                    Ok(Value::Number(s.parse().unwrap()))
                }
            }
            _ => Err(ParseError::UnexpectedChar(self.next().unwrap())),
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Ok(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.next().unwrap();
        }
    }

    pub fn parse(&mut self) -> Result<HashMap<String, Value>, ParseError> {
        let mut map = HashMap::new();

        self.skip_whitespaces();
        if self.next()? != '{' {
            return Err(ParseError::ExpectedChar('{'));
        }

        loop {
            self.skip_whitespaces();

            if self.next()? != '"' {
                return Err(ParseError::ExpectedChar('"'));
            }

            let ch = self.next()?;
            if !ch.is_ascii_alphabetic() && ch != '_' {
                return Err(ParseError::UnexpectedChar(ch));
            }
            let mut ident = String::from(ch);
            while let Ok(ch) = self.peek() {
                if ch.is_ascii_alphanumeric() || *ch == '_' {
                    ident += &ch.to_string();
                    self.next().unwrap();
                } else {
                    break;
                }
            }

            if self.next()? != '"' {
                return Err(ParseError::ExpectedChar('"'));
            }

            self.skip_whitespaces();

            if self.next()? != ':' {
                return Err(ParseError::ExpectedChar(':'));
            }

            map.insert(ident, self.parse_value()?);

            self.skip_whitespaces();

            if *self.peek()? == ',' {
                self.next().unwrap();
            }

            self.skip_whitespaces();

            if *self.peek()? == '}' {
                break;
            }
        }

        Ok(map)
    }

    fn peek(&mut self) -> Result<&char, ParseError> {
        self.src.peek().ok_or(ParseError::UnexpectedEOF)
    }

    fn next(&mut self) -> Result<char, ParseError> {
        self.src.next().ok_or(ParseError::UnexpectedEOF)
    }
}
