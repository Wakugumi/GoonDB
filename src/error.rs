use std::fmt;
use std::io;

#[derive(Debug)]
pub enum GoonError {
    /// I/O errors (file operations, doors open mid goon session, etc)
    Io(io::Error),

    ParseError(String),

    KeyNotFound(String),

    DatabaseError(String),
}

impl fmt::Display for GoonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoonError::Io(err) => write!(f, "Gooning IO Error: {}", err),
            GoonError::ParseError(err) => write!(f, "Gooning Parse Error: {}", err),
            GoonError::KeyNotFound(key) => write!(f, "Goon Key not found: {}", key),
            GoonError::DatabaseError(err) => write!(f, "Goon error: {}", err),
        }
    }
}

impl std::error::Error for GoonError {}

impl From<io::Error> for GoonError {
    fn from(err: io::Error) -> Self {
        GoonError::Io(err)
    }
}

pub type GoonResult<T> = Result<T, GoonError>;
