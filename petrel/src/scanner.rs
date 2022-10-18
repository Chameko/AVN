use crate::error::EmuError;
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
}
