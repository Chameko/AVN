use thiserror::Error;

#[derive(Error, Debug)]
pub enum PetrelError {
    #[error("file not found")]
    FileNotFound(#[from] std::io::Error),
    #[error("unknown character {0}")]
    UnknownCharacter(char),
}
