/// Tokens used to build Lex the input string.
#[derive(Debug)]
pub enum Token {
    /// String type.
    Str(String),
    /// Integer type.
    Number(i32),
    /// Floating type.
    Float(f32),
    /// Boolean type.
    Bool(bool),
    /// Null type.
    Null,
    /// Identifier.
    Ident(String),
    /// Left bracket used at the start of a block.
    LParen,
    /// Right bracket used at the end of a block.
    RParen,
    /// Used to denote start of an Array.
    ArrayLParen,
    /// Used to denote end of an Array.
    ArrayRParen,
    /// Used to differentiate between key and value.
    Colon,
    /// Comma.
    Comma,
    /// Unexpected token was encountered while lexing.
    Unknown(char),
}
