#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    Dot,
    QuestionMark,
    Plus,
    Minus,
    Slash,
    Star,
    Greater,
    Less,
    Bang,
    Equal,
    Colon,

    // Brackets -> []
    LeftBracket,
    RightBracket,
    // Brace -> {}
    LeftBrace,
    RightBrace,
    // Parenthases -> ()
    LeftParen,
    RightParen,

    // Double character symbols
    Arrow,
    GreaterEqual,
    LessEqual,
    DoubleEqual,
    DoubleColon,
    BangEqual,

    // Keywords
    Class,
    Const,
    Else,
    False,
    For,
    From,
    Fun,
    If,
    In,
    Override,
    Promise,
    Return,
    Super,
    True,
    Use,
    Var,
    While,

    // Literals
    Identifier,
    String,
    Number,
    // End of file
    EOF,
    // New Line
    NL,
}

/// Represents a "word" in the program
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    /// Token Type
    pub tt: TokenType,
    /// Which line the token is on
    pub line: usize,
    /// Which column the start of the token is in
    pub start: usize,
    /// The length of the token
    pub length: usize,
}
