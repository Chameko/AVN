use crate::error::EmuError;
use crate::token::{Token, TokenType};
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;

/// The type for source
type Source = Peekable<std::vec::IntoIter<char>>;

pub struct Scanner {
    /// The input for the scanner
    source: Source,
    /// The line no
    line: usize,
    /// The column no
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
    pub fn from_file(path: &str) -> Result<Scanner, EmuError> {
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
            column: 1,
        })
    }

    /// Scan the input into the tokens
    pub fn scan(&mut self) -> Result<Vec<Token>, EmuError> {
        // The tokens in the file
        let mut tokens: Vec<Token> = vec![];

        while !Self::end_of_file(&tokens) {
            tokens.push(self.scan_token()?);
        }

        Ok(tokens)
    }

    /// Scan the source for tokens
    fn scan_token(&mut self) -> Result<Token, EmuError> {
        // Get the next character
        if let Some(c) = self.source.peek() {
            use TokenType::*;

            match c {
                // This effectivly translates everything on the opposite side to a string
                '-' => {
                    let dialogue = self.consume_until_char('\n');
                    Ok(self.make_token(String(dialogue)))
                }
                '[' => {
                    self.next();
                    Ok(self.make_token(LeftBracket))
                }
                ']' => {
                    self.next();
                    Ok(self.make_token(RightBracket))
                }
                // { } acts as a *insert petrel block* and hence is effectivly a string but with petrel code instead
                '{' => Ok(self.petrel()),
                // String
                '"' => Ok(self.string()),
                '@' => {
                    self.next();
                    Ok(self.make_token(At))
                }
                '#' => {
                    // If there is a comment consume that line
                    self.comment();
                    // Return the next token
                    Ok(self.scan_token()?)
                }
                _ => {
                    // Check if its whitespace
                    if c.is_whitespace() {
                        // If its a new line update
                        if *c == '\n' {
                            self.line += 1;
                        }
                        // Return the next token
                        self.next().expect("Unreachable");
                        self.scan_token()
                    // Check for keywords/identifiers
                    } else if c.is_alphanumeric() || *c == '_' {
                        self.keyword()
                    } else {
                        Err(EmuError::UnknownSymbol(self.next().expect("Unreachable")))
                    }
                }
            }
        } else {
            // We've reached the end of the file
            Ok(self.make_token(TokenType::EOF))
        }
    }

    fn keyword(&mut self) -> Result<Token, EmuError> {
        // Look at the first character
        let current = self.source.peek().expect("Unreachable");
        // Check if its a key word
        match current {
            's' => {
                self.source.next().expect("Unreachable");
                if let Some(next) = self.source.peek() {
                    match next {
                        't' => Ok(self.test_keyword("tart", "s", TokenType::Start)),
                        'c' => Ok(self.test_keyword("cript", "s", TokenType::Script)),
                        _ => Ok(self.identifier("s".to_string())),
                    }
                } else {
                    // We've reached the end of file. The only possible token is the identifier s
                    Ok(self.make_token(TokenType::Identifier("s".to_string())))
                }
            }
            'c' => Ok(self.test_keyword("call", "", TokenType::Call)),
            'l' => Ok(self.test_keyword("let", "", TokenType::Let)),
            'j' => Ok(self.test_keyword("jump", "", TokenType::Jump)),
            _ => Ok(self.identifier("".to_string())),
        }
    }

    /// Get the next token, incramenting the column
    fn next(&mut self) -> Option<char> {
        self.column += 1;
        self.source.next()
    }

    /// Test if a series of character is a key word. If it isn't it returns the corresponding identifier
    fn test_keyword(&mut self, to_test: &str, prefix: &str, tt: TokenType) -> Token {
        // Get the current character
        let mut current = self.source.peek().expect("Unreachable");
        // Keep track of what we've consumed so far
        let mut consumed = prefix.to_string();

        for c in to_test.chars() {
            // If its a part of the key word
            if *current == c {
                // Add it to the consumed pile
                consumed.push(self.next().expect("Unreachable"));
                // Check for end of file
                if let Some(c) = self.source.peek() {
                    // Set current to the next char
                    current = c;
                } else {
                    // If it is return end of file
                    return self.make_token(TokenType::EOF);
                }
            } else {
                // Otherwise its an identifier
                return self.identifier(consumed);
            }
        }
        // Check if the token continues
        if current.is_alphanumeric() || *current == '_' {
            // Its an identifier
            self.identifier(consumed)
        } else {
            // Its our key word
            self.make_token(tt)
        }
    }

    /// Create an identifier, with the consumed string being the prefix
    fn identifier(&mut self, consumed: String) -> Token {
        // Get the identifier name
        let name = consumed + &self.consume_to_whitespace();
        self.make_token(TokenType::Identifier(name))
    }

    /// Consume characters until whitespace is encountered
    fn consume_to_whitespace(&mut self) -> String {
        let mut result = String::from("");
        if let Some(mut next) = self.source.peek() {
            // While it isn't whitespace
            while !next.is_whitespace() {
                // Add the next token
                result.push(self.next().expect("Unreachable"));
                // Check if next token is valid
                if let Some(c) = self.source.peek() {
                    next = c
                } else {
                    // If it isn't (end of file) break.
                    break;
                }
            }
            result
        } else {
            result
        }
    }

    /// Helper function that consumes character until we hit a delimiter and pops out the consumed characters
    fn consume_until_char(&mut self, end: char) -> String {
        // Go past the opening character
        self.next().expect("Unreachable");
        // Consume all the characters until we reach the char
        let mut consumed = String::from("");
        if let Some(mut next) = self.source.peek() {
            while *next != end {
                consumed.push(*next);
                if let Some(c) = self.source.peek() {
                    next = c;
                } else {
                    break;
                }
            }
            consumed
        } else {
            consumed
        }
    }

    /// Create a comment
    fn comment(&mut self) {
        // Consume all the characters until we reach a new line
        self.consume_until_char('\n');
    }

    /// Create a petrel block
    fn petrel(&mut self) -> Token {
        // Consume until the closing }
        let petrel = self.consume_until_char('}');
        self.make_token(TokenType::Petrel(petrel))
    }

    fn string(&mut self) -> Token {
        // Consume until the closing "
        let string = self.consume_until_char('"');
        self.make_token(TokenType::String(string))
    }

    /// Make a token
    fn make_token(&self, tt: TokenType) -> Token {
        Token {
            tt,
            line: self.line,
            column: self.column,
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
}

#[cfg(test)]
mod tests {
    #[test]
    /// Test for the correct interpretation if single character tokens
    fn single_character_tokens() {
        let mut scanner = super::Scanner::new("@ [ ]".to_string());
        let tokens = scanner.scan().expect("failed to scan tokens");
        use crate::token::TokenType;
        let token_types: Vec<TokenType> = tokens.into_iter().map(|t| t.tt).collect();
        use crate::token::TokenType::*;
        let result = vec![At, LeftBracket, RightBracket, LeftBrace, RightBrace, EOF];
        assert_eq!(token_types, result);
    }

    #[test]
    fn identifiers_and_keywords() {
        let mut scanner = super::Scanner::new(
            "start script awesome s scri script_1 jump call let jum jumped ca called le letgo _script"
                .to_string(),
        );
        let tokens = scanner.scan().expect("failed to scan tokens");
        use crate::token::TokenType;
        let token_types: Vec<TokenType> = tokens.into_iter().map(|t| t.tt).collect();
        use crate::token::TokenType::*;
        let result = vec![
            Start,
            Script,
            Identifier("awesome".to_string()),
            Identifier("s".to_string()),
            Identifier("scri".to_string()),
            Identifier("script_1".to_string()),
            Jump,
            Call,
            Let,
            Identifier("jum".to_string()),
            Identifier("jumped".to_string()),
            Identifier("ca".to_string()),
            Identifier("called".to_string()),
            Identifier("le".to_string()),
            Identifier("letgo".to_string()),
            Identifier("_script".to_string()),
            EOF,
        ];
        assert_eq!(token_types, result);
    }
}
