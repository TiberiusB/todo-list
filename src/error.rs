use std::{error, fmt, io};

pub type TodoResult<T> = Result<T, TodoError>;

#[derive(Debug)]
pub enum TodoError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl error::Error for TodoError {}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::Io(error) => write!(f, "IO error: {}", error),
            TodoError::Json(error) => write!(f, "JSON error: {}", error),
        }
    }
}

impl From<io::Error> for TodoError {
    fn from(error: io::Error) -> Self {
        TodoError::Io(error)
    }
}

impl From<serde_json::Error> for TodoError {
    fn from(error: serde_json::Error) -> Self {
        TodoError::Json(error)
    }
}
