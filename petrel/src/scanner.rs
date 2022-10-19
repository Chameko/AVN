use crate::error::PetrelError;
use crate::token::{Token, TokenType};
use std::fs::File;
use std::io::Read;

/// Scanner used to conver the file into a vector of tokens
pub struct Scanner {
    /// The input for the scanner
    source: Vec<char>,
    /// The line number
    line: usize,
    /// The starting index
    start: usize,
    /// The length
    len: usize,
}

impl Scanner {
    /// Create a new scanner from string
    pub fn new(input: String) -> Scanner {
        // Create an iterator of said characters.
        // This is done as chars() references input, which is a function parameter
        let source = input.chars().collect::<Vec<char>>();
        Scanner {
            source,
            line: 1,
            start: 0,
            len: 0,
        }
    }

    /// Read from file
    pub fn from_file(path: &str) -> Result<Scanner, PetrelError> {
        // Read file
        let mut file = File::open(path)?;
        let mut input: String = "".into();
        file.read_to_string(&mut input)?;

        // Create an iterator of said characters.
        // This is done as chars() references input, which is a funcion parameter
        let source = input.chars().collect::<Vec<char>>();

        Ok(Scanner {
            source,
            line: 1,
            start: 0,
            len: 0,
        })
    }

    /// Creates a token
    #[inline]
    fn make_token(&mut self, tt: TokenType) -> Token {
        let t = Token {
            tt,
            line: self.line,
            start: self.start,
            length: self.len,
        };
        self.len = 0;
        t
    }

    /// Check if the end of file token has been generated
    fn end_of_file(tokens: &[Token]) -> bool {
        matches!(
            tokens.last(),
            Some(Token {
                tt: TokenType::EOF,
                ..
            })
        )
    }

    /// Move the start forward one, returning the next character
    #[inline]
    fn next(&mut self) -> Option<&char> {
        self.start += 1;
        self.source.get(self.start)
    }

    /// Advance the index by 1
    #[inline]
    fn advance(&mut self) {
        self.start += 1;
    }

    /// Peek at next char without consuming the character
    #[inline]
    fn peek(&mut self) -> Option<&char> {
        self.source.get(self.start + 1)
    }

    /// Get the current character
    #[inline]
    fn current(&self) -> Option<&char> {
        self.source.get(self.start)
    }

    /// Consume the character and peek at the next
    #[inline]
    fn consume_and_peek(&mut self) -> Option<&char> {
        self.next()?;
        self.peek()
    }

    /// Scan the input into the tokens
    pub fn scan(&mut self) -> Result<Vec<Token>, PetrelError> {
        // The tokens in the file
        let mut tokens: Vec<Token> = vec![];

        while !Self::end_of_file(&tokens) {
            tokens.push(self.scan_token()?);
        }

        Ok(tokens)
    }

    /// Scan a singular token
    pub fn scan_token(&mut self) -> Result<Token, PetrelError> {
        // The token is (usually) one character long
        self.len += 1;
        if let Some(c) = self.current() {
            // For convinience
            use crate::token::TokenType::*;
            match c {
                // Single character tokens
                '.' => Ok(self.make_token(Dot)),
                '?' => Ok(self.make_token(QuestionMark)),
                '+' => Ok(self.make_token(Plus)),
                '-' => Ok(self.make_token(Minus)),
                '/' => Ok(self.make_token(Slash)),
                '*' => Ok(self.make_token(Star)),
                '>' => Ok(self.make_token(Greater)),
                '<' => Ok(self.make_token(Less)),
                '!' => Ok(self.make_token(Bang)),
                '=' => Ok(self.make_token(Equal)),
                ':' => Ok(self.make_token(Colon)),

                // Various brackets
                '(' => Ok(self.make_token(LeftParen)),
                ')' => Ok(self.make_token(RightParen)),
                '{' => Ok(self.make_token(LeftBrace)),
                '}' => Ok(self.make_token(RightBrace)),
                '[' => Ok(self.make_token(LeftBracket)),
                ']' => Ok(self.make_token(RightBracket)),

                _ => Err(PetrelError::UnknownCharacter(*c)),
            }
        } else {
            // End of file has no length
            self.len = 0;
            Ok(self.make_token(TokenType::EOF))
        }
    }
}
