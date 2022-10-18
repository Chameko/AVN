use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmuError {
    #[error("file not found")]
    FileNotFound(#[from] std::io::Error),
}
