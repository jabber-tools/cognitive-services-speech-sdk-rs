//! Package error contains struct Error used to wrap library errors.
use crate::ffi::{
    error_get_error_code, error_get_message, error_release, AZAC_HANDLE, SPX_NOERROR,
};
use std::ffi::{CStr, NulError};
use std::num::TryFromIntError;
use std::result;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

/// Enumeration of error root causes. Where appropriate
/// it wraps underlying error.
#[derive(Debug)]
pub enum ErrorRootCause {
    ApiError(usize),
    FfiNulError(NulError),
    InvalidCString,
    FromUtf8Error(FromUtf8Error),
    Utf8Error(Utf8Error),
    TryFromIntError(TryFromIntError),
}

/// Error struct represents error than can occur
/// during library processing/execution.
#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub caused_by: ErrorRootCause,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error {
    /// Creates new error from custom message and underlying root cause.
    pub fn new(message: String, caused_by: ErrorRootCause) -> Self {
        Error { message, caused_by }
    }

    /// Returns description based on API error code.
    pub fn api_error_desc(error_root_cause: &ErrorRootCause) -> Option<String> {
        match error_root_cause {
            ErrorRootCause::ApiError(api_code) => {
                let api_result_desc = match api_code {
                    0x000 => "SPX_NOERROR",
                    0xfff => "SPXERR_NOT_IMPL",
                    0x001 => "SPXERR_UNINITIALIZED",
                    0x002 => "SPXERR_ALREADY_INITIALIZED",
                    0x003 => "SPXERR_UNHANDLED_EXCEPTION",
                    0x004 => "SPXERR_NOT_FOUND",
                    0x005 => "SPXERR_INVALID_ARG",
                    0x006 => "SPXERR_TIMEOUT",
                    0x007 => "SPXERR_ALREADY_IN_PROGRESS",
                    0x008 => "SPXERR_FILE_OPEN_FAILED",
                    0x009 => "SPXERR_UNEXPECTED_EOF",
                    0x00a => "SPXERR_INVALID_HEADER",
                    0x00b => "SPXERR_AUDIO_IS_PUMPING",
                    0x00c => "SPXERR_UNSUPPORTED_FORMAT",
                    0x00d => "SPXERR_ABORT",
                    0x00e => "SPXERR_MIC_NOT_AVAILABLE",
                    0x00f => "SPXERR_INVALID_STATE",
                    0x010 => "SPXERR_UUID_CREATE_FAILED",
                    0x011 => "SPXERR_SETFORMAT_UNEXPECTED_STATE_TRANSITION",
                    0x012 => "SPXERR_PROCESS_AUDIO_INVALID_STATE",
                    0x013 => "SPXERR_START_RECOGNIZING_INVALID_STATE_TRANSITION",
                    0x014 => "SPXERR_UNEXPECTED_CREATE_OBJECT_FAILURE",
                    0x015 => "SPXERR_MIC_ERROR",
                    0x016 => "SPXERR_NO_AUDIO_INPUT",
                    0x017 => "SPXERR_UNEXPECTED_USP_SITE_FAILURE",
                    0x018 => "SPXERR_UNEXPECTED_UNIDEC_SITE_FAILURE",
                    0x019 => "SPXERR_BUFFER_TOO_SMALL",
                    0x01A => "SPXERR_OUT_OF_MEMORY",
                    0x01B => "SPXERR_RUNTIME_ERROR",
                    0x01C => "SPXERR_INVALID_URL",
                    0x01D => "SPXERR_INVALID_REGION",
                    0x01E => "SPXERR_SWITCH_MODE_NOT_ALLOWED",
                    0x01F => "SPXERR_CHANGE_CONNECTION_STATUS_NOT_ALLOWED",
                    0x020 => "SPXERR_EXPLICIT_CONNECTION_NOT_SUPPORTED_BY_RECOGNIZER",
                    0x021 => "SPXERR_INVALID_HANDLE",
                    0x022 => "SPXERR_INVALID_RECOGNIZER",
                    0x023 => "SPXERR_OUT_OF_RANGE",
                    0x024 => "SPXERR_EXTENSION_LIBRARY_NOT_FOUND",
                    0x025 => "SPXERR_UNEXPECTED_TTS_ENGINE_SITE_FAILURE",
                    0x026 => "SPXERR_UNEXPECTED_AUDIO_OUTPUT_FAILURE",
                    0x027 => "SPXERR_GSTREAMER_INTERNAL_ERROR",
                    0x028 => "SPXERR_CONTAINER_FORMAT_NOT_SUPPORTED_ERROR",
                    0x029 => "SPXERR_GSTREAMER_NOT_FOUND_ERROR",
                    0x02A => "SPXERR_INVALID_LANGUAGE",
                    0x02B => "SPXERR_UNSUPPORTED_API_ERROR",
                    0x02C => "SPXERR_RINGBUFFER_DATA_UNAVAILABLE",
                    0x030 => "SPXERR_UNEXPECTED_CONVERSATION_SITE_FAILURE",
                    0x031 => "SPXERR_UNEXPECTED_CONVERSATION_TRANSLATOR_SITE_FAILURE",
                    0x032 => "SPXERR_CANCELED",
                    _ => "UNKNOWN SPXERR",
                };
                Some(api_result_desc.to_owned())
            }
            _ => None,
        }
    }
}

/// Convenience type so that we can use *Result&lt;T&gt;*
/// instead of *Result<T, E>*.
pub type Result<T> = result::Result<T, Error>;

/// Converts underlying C API error represented by error code
/// into Error structure.
#[inline(always)]
pub fn convert_err(hr: usize, err_msg: &str) -> Result<()> {
    if hr != SPX_NOERROR as usize {
        unsafe {
            let error_handle = hr as AZAC_HANDLE;
            let code = error_get_error_code(error_handle);
            let message = CStr::from_ptr(error_get_message(error_handle))
                .to_str()
                .unwrap_or("");

            error_release(error_handle);

            Err(Error::new(
                err_msg.to_string() + ": " + message,
                ErrorRootCause::ApiError(code),
            ))
        }
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

impl From<TryFromIntError> for Error {
    fn from(error: TryFromIntError) -> Error {
        Error {
            message: format!("std::num::TryFromIntError: {}", error),
            caused_by: ErrorRootCause::TryFromIntError(error),
        }
    }
}
