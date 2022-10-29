use std::fmt::{Debug, Display};

use crate::compiler::token::Token;
use colored::*;
use thiserror::Error;

use crate::runtime::vm::Opcode;

/// General error types
#[derive(Error, Debug)]
pub enum PetrelError {
    #[error("file {0} not found")]
    FileNotFound(#[from] std::io::Error),
    #[error("unknown character {0}")]
    UnknownCharacter(char),
    #[error("missing double quote (\")")]
    MissingDoubleQuote,
    #[error("virtual machine ran into problem {0}")]
    VMError(#[from] VMError),
    #[error("tried to use token {0} in array of length {1}")]
    TokenOutOfBounds(usize, usize),
    #[error("{}{0}", "Syntax error\n".blue().bold())]
    SyntaxError(#[from] SyntaxError),
}

/// Errors that can occur in the VM
#[derive(Debug, Error)]
pub enum VMError {
    #[error("opcode {0:?} unsupported")]
    UnsupportedOpcode(Opcode),
    #[error("cannot turn byte {0} into opcode")]
    InvalidOpcodeConversion(u8),
    #[error("attempted to perform operation on empty stack")]
    EmptyStack,
    #[error("encountered end of instructions with no return")]
    NoReturn,
}

#[derive(Debug, Error, PartialEq)]
pub enum BlockError {
    #[error("Block size requested wasn't power of 2")]
    BadRequest,
    #[error("Insufficient memory, couldn't allocate block")]
    OOM,
}

/// Errors due to actual problems in the code
#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("{0}")]
    MissingDoubleQuote(Annotation),
    #[error("{0}")]
    ExpectedToken(Annotation),
}

#[derive(Debug)]
pub struct Annotation {
    token: Token,
    source: String,
    info: String,
}

impl Display for Annotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print_error())
    }
}

impl Annotation {
    pub fn new(message: String, tk: Token, src: String) -> Self {
        Self {
            token: tk,
            source: src,
            info: message,
        }
    }

    pub fn print_error(&self) -> String {
        let l1 = format!("{}{}\n", "error: ".red().bold(), self.info.bold());
        let l2 = format!(" from --> {:-<50}\n", self.source);
        let l3 = format!(" {:<3} | {}\n", self.token.line, self.source);
        let l4 = format!(
            " {:<3} | {}{}\n",
            "",
            " ".repeat(self.token.start),
            "^".repeat(self.token.length)
        );
        l1 + &l2 + &l3 + &l4
    }
}
