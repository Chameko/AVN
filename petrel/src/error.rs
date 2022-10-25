use thiserror::Error;

#[derive(Error, Debug)]
pub enum PetrelError {
    #[error("file not found")]
    FileNotFound(#[from] std::io::Error),
    #[error("unknown character {0}")]
    UnknownCharacter(char),
    #[error("missing double quote (\")")]
    MissingDoubleQuote,
    #[error("Interpreter failed :(")]
    InterpretError,
    #[error("Opcode is not supported yet.")]
    UnsupportedOpcode,
    #[error("Byte doesn't have associated codde")]
    NoPotentialOpcode,
    #[error("Stack is empty when performing operation")]
    EmptyStack,
}
