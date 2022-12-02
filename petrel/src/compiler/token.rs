use crate::compiler::compiler::Compiler;
use crate::compiler::compiler::ParseRule;
use crate::compiler::compiler::Precedence;

use super::Scanner;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    Comma,

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
    Const,
    Else,
    False,
    For,
    From,
    Fun,
    If,
    Impl,
    In,
    Null,
    Return,
    Struct,
    Super,
    This,
    Trait,
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

impl TokenType {
    /// Get the parser rule for the token
    pub fn get_rule(&self) -> ParseRule {
        use TokenType::*;
        match self {
            LeftParen => ParseRule {
                prefix: Some(Compiler::grouping),
                infix: None,
                precedence: Precedence::None,
            },
            Minus => ParseRule {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            Plus => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            Slash => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            Star => ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            Number => ParseRule {
                prefix: Some(Compiler::number),
                infix: None,
                precedence: Precedence::None,
            },
            False => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            True => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            Null => ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            Bang => ParseRule {
                prefix: Some(Compiler::unary),
                infix: None,
                precedence: Precedence::None,
            },
            _ => ParseRule::default(),
        }
    }
}

/// Represents a "word" in the program. Should be cheap to copy but I want to be explicit about when I'm copying
#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Token {
    /// Return the string of text the token represents in the source code
    pub fn contained_string(&self, scanner: &Scanner) -> String {
        scanner
            .source
            .get(self.start..(self.start + self.length))
            .expect("Text out of range")
            .iter()
            .collect::<String>()
    }
}
