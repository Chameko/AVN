use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmuError {
    #[error("file not found")]
    FileNotFound(#[from] std::io::Error),
    #[error("unexpected end of file")]
    UnexpectedEOF,
    #[error("unknown symbol {0}")]
    UnknownSymbol(char),
}
