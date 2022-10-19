use crate::error::PetrelError;
use crate::token::{Token, TokenType};
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;

/// The type for source
type Source = Peekable<std::vec::IntoIter<char>>;

/// Scanner used to conver the file into a vector of tokens
pub struct Scanner {
    /// The input for the scanner
    source: Source,
    /// The line number
    line: usize,
    /// The column number
    column: usize,
}

impl Scanner {
    /// Create a new scanner from string
    pub fn new(input: String) -> Scanner {
        // Create an iterator of said characters.
        // This is done as chars() references input, which is a function parameter
        let source = input.chars().collect::<Vec<char>>().into_iter().peekable();
        Scanner {
            source,
            line: 1,
            column: 1,
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
        let source = input.chars().collect::<Vec<char>>().into_iter().peekable();

        Ok(Scanner {
            source,
            line: 1,
            column: 0,
        })
    }

    /// Creates a token
    fn make_token(&self, tt: TokenType, start: usize) -> Token {
        Token {
            tt,
            line: self.line,
            column: start,
            length: self.column - start,
        }
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

    /// Move the iterator forward one, consuming the character and returning the next
    #[inline]
    fn next(&mut self) -> Option<char> {
        // Done so we don't incrament the column if the next token doesn't exist
        let next = self.source.next()?;
        self.column += 1;
        Some(next)
    }

    /// Peek at next char without consuming token
    #[inline]
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
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
        todo!()
    }
}
