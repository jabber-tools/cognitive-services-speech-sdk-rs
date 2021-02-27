use std::result;

#[derive(Debug)]
pub enum ErrorRootCause {
    ApiError(usize),
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub caused_by: ErrorRootCause,
}

impl Error {
    pub fn new(message: String, caused_by: ErrorRootCause) -> Self {
        Error { message, caused_by }
    }
}

pub type Result<T> = result::Result<T, Error>;
