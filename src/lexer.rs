use crate::tokens::Token;

use std::iter::Peekable;
use std::str::Chars;

/// The Lexer struct. Keeps a immutable refernce to the string that is to
/// be parsed.
pub struct Lexer<'chars> {
    // Invariant : The lexer consumes all the characters it has parsed.
    /// Immutable iterator over the chars of `str` being parsed.
    src: Peekable<Chars<'chars>>,
    // Invariant : `pos` always next char to to be parsed.
    /// To keep track of position. Useful for getting error positions.
    pos: usize,
}

impl<'chars> Lexer<'chars> {
    /// Create a new lexer from given `str`.
    pub fn new<'a>(src: &'a str) -> Lexer<'a> {
        Lexer {
            src: src.chars().peekable(),
            pos: 0,
        }
    }

    /// Lex the next token. May parse multiple characters as well.
    pub fn lex(&mut self) -> Option<Token> {
        // get the next character
        let mut ch = self.src.next()?;
        self.pos += 1;

        // skip whitespaces
        while ch.is_whitespace() {
            ch = self.src.next()?;
            self.pos += 1;
        }

        // finally, find the token in next character(s)
        match ch {
            '{' => Some(Token::LParen),
            '}' => Some(Token::RParen),
            '[' => Some(Token::ArrayRParen),
            ']' => Some(Token::ArrayLParen),
            ':' => Some(Token::Colon),
            ',' => Some(Token::Comma),
            '"' => {
                let mut s = String::new();
                while let Some(ch) = self.src.next() {
                    if ch == '"' {
                        break;
                    }
                    s += &ch.to_string();
                    self.pos += 1;
                }
                Some(Token::Str(s))
            }
            'a'..='z' | 'A'..='Z' => {
                let mut s = String::from(ch);
                while let Some(ch) = self.src.peek() {
                    if ch.is_alphanumeric() || *ch == '_' {
                        s += &ch.to_string();
                    } else {
                        break;
                    }
                    self.pos += 1;
                    self.src.next();
                }
                // unwrap always works because we only chose valid chars
                if s == "true" || s == "false" {
                    Some(Token::Bool(s.parse().ok()?))
                } else if s == "null" {
                    Some(Token::Null)
                } else {
                    Some(Token::Ident(s))
                }
            }
            '0'..='9' => {
                let mut n = String::from(ch);
                let mut is_float = false;
                while let Some(ch) = self.src.peek() {
                    if ch.is_numeric() {
                        n += &ch.to_string();
                    } else if *ch == '.' {
                        n += &ch.to_string();
                        is_float = true;
                    } else {
                        break;
                    }
                    self.pos += 1;
                    self.src.next();
                }
                // unwrap always works because we only chose valid chars
                if is_float {
                    Some(Token::Float(n.parse().unwrap()))
                } else {
                    Some(Token::Number(n.parse().unwrap()))
                }
            }
            _ => Some(Token::Unknown(ch)),
        }
    }
}

impl<'chars> Iterator for Lexer<'chars> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_str() {
        let mut x = Lexer {
            src: "\"okay\"".chars().peekable(),
            pos: 0,
        };
        let x: String = match x.lex() {
            Some(Token::Str(s)) => s.into(),
            _ => "".into(),
        };
        assert_eq!(x, "okay".to_string());
    }

    #[test]
    fn lex_num() {
        let mut x = Lexer {
            src: "12".chars().peekable(),
            pos: 0,
        };
        let x = match x.lex() {
            Some(Token::Number(s)) => s,
            _ => 0,
        };
        assert_eq!(x, 12);
    }

    #[test]
    fn lex_float() {
        let mut x = Lexer {
            src: "12.3".chars().peekable(),
            pos: 0,
        };
        let x = match x.lex() {
            Some(Token::Float(s)) => s,
            _ => 0.0,
        };
        assert!(x > 12.2 && x < 12.4);
    }

    #[test]
    fn lex_bool() {
        let mut x = Lexer {
            src: "true false".chars().peekable(),
            pos: 0,
        };
        let t1 = match x.lex() {
            Some(Token::Bool(s)) => s,
            _ => false,
        };
        let t2 = match x.lex() {
            Some(Token::Bool(s)) => s,
            _ => true,
        };
        assert_eq!(t1, true);
        assert_eq!(t2, false);
    }

    #[test]
    fn lex_null() {
        let mut x = Lexer {
            src: "null".chars().peekable(),
            pos: 0,
        };
        let x: Option<()> = match x.lex() {
            Some(Token::Null) => Some(()),
            _ => None,
        };
        assert_eq!(x, Some(()));
    }

    #[test]
    fn lex_ident() {
        let mut x = Lexer {
            src: "okay".chars().peekable(),
            pos: 0,
        };
        let x: String = match x.lex() {
            Some(Token::Ident(s)) => s.into(),
            _ => "".into(),
        };
        assert_eq!(x, "okay".to_string());
    }

    #[test]
    fn json_lex1() {
        let mut x = Lexer {
            src: r#"
{
    name: "Mr. John",
    age: 25,
    cars: ["ferrari", "bmw"],
    others: {
        lucky_numbers: [1, 2, 2.5],
    }
}
"#
            .chars()
            .peekable(),
            pos: 0,
        };

        match x.lex() {
            Some(Token::LParen) => {}
            _ => panic!(),
        }
        match x.lex() {
            Some(Token::Ident(ident)) => assert_eq!(ident, "name"),
            _ => panic!(),
        }
        match x.lex() {
            Some(Token::Colon) => {}
            err => panic!("got this {:?}", err),
        }
        match x.lex() {
            Some(Token::Str(s)) => assert_eq!(s, "Mr. John"),
            _ => panic!(),
        }
        match x.lex() {
            Some(Token::Comma) => {}
            _ => panic!(),
        }
        match x.lex() {
            Some(Token::Ident(ident)) => assert_eq!(ident, "age"),
            _ => panic!(),
        }
        match x.lex() {
            Some(Token::Colon) => {}
            _ => panic!(),
        }
        match x.lex() {
            Some(Token::Number(n)) => assert_eq!(n, 25),
            _ => panic!(),
        }
        // match x.lex() {
        //     Some(Token::LParen) => {},
        //     _ => panic!(),
        // }

        // TODO: check the entire json
    }
}
