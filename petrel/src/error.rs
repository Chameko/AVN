use thiserror::Error;

#[derive(Error, Debug)]
pub enum PetrelError {
    #[error("file not found")]
    FileNotFound(#[from] std::io::Error),
}
