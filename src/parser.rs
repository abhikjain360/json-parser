use crate::lexer::Lexer;
use crate::tokens::Tokens;

use std::collections::VecDeque;

struct Parser<'lexer> {
    lexer: Lexer<'lexer>,
    stack: VecDeque<Tokens>,
}

impl<'lexer> Parser<'lexer> {
    pub fn new<'a>(src: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(src),
            stack: VecDeque::new(),
        }
    }
}

// TODO: write some tests here.
