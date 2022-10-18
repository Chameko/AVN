#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    Hash,
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
    Fun,
    Use,
    Const,
    Var,
    Try,
    Class,
    If,
    For,
    Else,
    While,
    Return,
    Fail,
    Override,
    Promise,
    From,
    In,
    Super,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    Bool(bool),
    // End of file
    EOF,
    // New Line
    NL,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    /// Token Type
    pub tt: TokenType,
    /// Which line the token is on
    pub line: usize,
    /// Which column the start of the token is in
    pub column: usize,
}
