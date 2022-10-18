#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    At,
    // Brackets -> []
    LeftBracket,
    RightBracket,
    // Brace -> {}
    LeftBrace,
    RightBrace,
    // Keywords
    Start,
    Script,
    Jump,
    Call,
    Let,

    // Literals
    Identifier(String),
    String(String),
    Petrel(String),
    // End of file
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    /// Token Type
    pub tt: TokenType,
    /// Which line the token is on
    pub line: usize,
    /// Which column the start of the token is in
    pub column: usize,
}
