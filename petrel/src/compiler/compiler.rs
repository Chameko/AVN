use super::{Token, TokenType};
use crate::diagnostic::{Annotation, PetrelError, SyntaxError};

pub struct Compiler {
    /// The source as a list of lines
    pub source: Vec<String>,
    /// Tokens :)
    pub tokens: Vec<Token>,
    /// Index
    index: usize,
    /// Collected errors
    errors: Vec<SyntaxError>,
    /// Used to skip tokens until we can recover
    panicMode: bool,
}

impl Compiler {
    #[inline]
    fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index)
    }

    /// Advance the index by 1
    #[inline]
    fn advance(&mut self) {
        self.index += 1;
    }

    /// Peek at next char without consuming the character
    #[inline]
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    /// Get the current character. Should never fail so it panics if the index out of bounds
    #[inline]
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    /// Used to report error. Mostly exists so I don't forget to set panic mode to true.
    #[inline]
    fn report_error(&mut self, code_error: SyntaxError) -> Result<(), PetrelError> {
        self.panicMode = true;
        Err(PetrelError::SyntaxError(code_error))
    }

    fn consume(&mut self, tt: TokenType) -> Result<(), PetrelError> {
        let current = self
            .current()
            .ok_or_else(|| PetrelError::TokenOutOfBounds(self.index, self.tokens.len()))?;
        if tt == current.tt {
            self.advance();
            Ok(())
        } else {
            let a = Annotation::new(
                format!("expected {:#?}", tt),
                current.clone(),
                self.source[current.line].clone(),
            );
            self.report_error(SyntaxError::ExpectedToken(a))
        }
    }
}
