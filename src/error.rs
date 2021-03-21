use crate::ffi::SPX_NOERROR;
use std::ffi::NulError;
use std::result;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ErrorRootCause {
    ApiError(usize),
    FfiNulError(NulError),
    InvalidCString,
    FromUtf8Error(FromUtf8Error),
    Utf8Error(Utf8Error),
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

impl From<NulError> for Error {
    fn from(error: NulError) -> Error {
        Error {
            message: format!("std::ffi::NulError: {}", error),
            caused_by: ErrorRootCause::FfiNulError(error),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error {
            message: format!("std::string::FromUtf8Error: {}", error),
            caused_by: ErrorRootCause::FromUtf8Error(error),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Error {
        Error {
            message: format!("std::str::Utf8Error: {}", error),
            caused_by: ErrorRootCause::Utf8Error(error),
        }
    }
}
