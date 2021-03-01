use crate::ffi::SPX_NOERROR;
use std::result;

#[derive(Debug)]
pub enum ErrorRootCause {
    ApiError(usize),
    FfiNulError,
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

#[inline(always)]
pub fn convert_err(hr: usize, err_msg: &str) -> Result<()> {
    if hr != SPX_NOERROR as usize {
        Err(Error::new(err_msg.into(), ErrorRootCause::ApiError(hr)))
    } else {
        Ok(())
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Error {
        Error {
            message: format!("std::ffi::NulError: {}", error),
            caused_by: ErrorRootCause::FfiNulError,
        }
    }
}
