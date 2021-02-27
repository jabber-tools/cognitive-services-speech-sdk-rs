#[macro_use]
extern crate enum_primitive;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate futures;
#[macro_use]
extern crate log;
extern crate num;
extern crate tokio;

use std::ffi;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::os::raw::c_char;

pub use crate::async_handle::AsyncHandle;
pub use crate::async_handle::AsyncResultHandle;
pub use crate::config::{SpeechConfig, SpeechSynthesisOutputFormat};
pub use crate::property::PropertyBag;
pub use crate::property::PropertyId;
use crate::speech_api::*;

pub mod audio;
pub mod recognizer;
pub mod synthesizer;
mod async_handle;
mod speech_api;
mod property;
mod config;

const SPXHANDLE_INVALID: SPXHANDLE = 0 as SPXHANDLE;

#[derive(Fail, Debug)]
pub enum SpxError {
    #[fail(display = "Speech API error code: {}.", _0)]
    ApiError(usize),
    #[fail(display = "Invalid CString (NulError).")]
    StrNulError(#[cause] ffi::NulError),
    #[fail(display = "Failed to convert C string.")]
    IntoStringError(#[cause] ffi::IntoStringError),
    #[fail(display = "Failed to convert C string.")]
    InvalidCString,
    #[fail(display = "C string is not utf-8 encoded.")]
    FromUtf8Error(#[cause] std::string::FromUtf8Error),
    #[fail(display = "Stream is dropped.")]
    StreamDropped,
}

impl From<ffi::NulError> for SpxError {
    fn from(err: ffi::NulError) -> Self {
        return SpxError::StrNulError(err);
    }
}

impl From<ffi::IntoStringError> for SpxError {
    fn from(err: ffi::IntoStringError) -> Self {
        return SpxError::IntoStringError(err);
    }
}

impl From<std::string::FromUtf8Error> for SpxError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        return SpxError::FromUtf8Error(err);
    }
}

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum ResultReason {
    /// <summary>
    /// Indicates speech could not be recognized. More details can be found in the NoMatchDetails object.
    /// </summary>
    NoMatch = 0,

    /// <summary>
    /// Indicates that the recognition was canceled. More details can be found using the cancellation_details object.
    /// </summary>
    Canceled = 1,

    /// <summary>
    /// Indicates the speech result contains hypothesis text.
    /// </summary>
    RecognizingSpeech = 2,

    /// <summary>
    /// Indicates the speech result contains final text that has been recognized.
    /// Speech Recognition is now complete for this phrase.
    /// </summary>
    RecognizedSpeech = 3,

    /// <summary>
    /// Indicates the intent result contains hypothesis text and intent.
    /// </summary>
    RecognizingIntent = 4,

    /// <summary>
    /// Indicates the intent result contains final text and intent.
    /// Speech Recognition and Intent determination are now complete for this phrase.
    /// </summary>
    RecognizedIntent = 5,

    /// <summary>
    /// Indicates the translation result contains hypothesis text and its translation(s).
    /// </summary>
    TranslatingSpeech = 6,

    /// <summary>
    /// Indicates the translation result contains final text and corresponding translation(s).
    /// Speech Recognition and Translation are now complete for this phrase.
    /// </summary>
    TranslatedSpeech = 7,

    /// <summary>
    /// Indicates the synthesized audio result contains a non-zero amount of audio data
    /// </summary>
    SynthesizingAudio = 8,

    /// <summary>
    /// Indicates the synthesized audio is now complete for this phrase.
    /// </summary>
    SynthesizingAudioCompleted = 9,
}
}

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum CancellationReason
{
    Error = 1,
    EndOfStream = 2,
}
}

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum CancellationErrorCode
{
    NoError = 0,
    AuthenticationFailure = 1,
    BadRequest = 2,
    TooManyRequests = 3,
    Forbidden = 4,
    ConnectionFailure = 5,
    ServiceTimeout = 6,
    ServiceError = 7,
    ServiceUnavailable = 8,
    RuntimeError = 9
}
}

#[inline(always)]
fn convert_err(hr: usize) -> Result<(), SpxError> {
    if hr != SPX_NOERROR as usize {
        return Err(SpxError::ApiError(hr));
    }
    return Ok(());
}

#[derive(Debug)]
pub struct SmartHandle<T: Copy + Debug> {
    inner: T,
    release_fn: unsafe extern "C" fn(T) -> SPXHR,
    name: &'static str,
}

impl<T: Copy + Debug> SmartHandle<T> {
    #[inline(always)]
    fn create(name: &'static str, handle: T, release_fn: unsafe extern "C" fn(T) -> SPXHR) -> SmartHandle<T> {
        let result = SmartHandle { inner: handle, release_fn, name };
        trace!("Create SmartHandle {}.", result);
        return result;
    }

    #[inline(always)]
    fn get(&self) -> T {
        self.inner
    }
}

impl<T: Copy + Debug> Drop for SmartHandle<T> {
    fn drop(&mut self) {
        trace!("Drop SmartHandle {}.", self);
        let hr = unsafe { (self.release_fn)(self.inner) };
        if hr != SPX_NOERROR as usize {
            panic!("can not release SmartHandle {}, err={}", self, hr);
        }
    }
}

impl<T: Copy + Debug> Display for SmartHandle<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{{{:?}}}", self.name, self.inner)
    }
}

unsafe impl<T: Copy + Debug> Send for SmartHandle<T> {}

pub trait FromHandle<H, E>: Sized {
    fn from_handle(handle: H) -> Result<Self, E>;
}

pub trait ResultHandleSupport {
    fn async_wait_fn() -> unsafe extern "C" fn(SPXASYNCHANDLE, u32, *mut SPXRESULTHANDLE) -> SPXHR;

    fn release_fn() -> unsafe extern "C" fn(SPXRESULTHANDLE) -> SPXHR;
}

pub struct FfiObject {
    pub ptr: *mut u8,
    pub size: usize,
}

impl FfiObject {
    // allocate and zero memory
    pub fn new(size: usize) -> FfiObject {
        FfiObject::_from_vec(vec![0u8; size], size)
    }

    // allocate memory without zeroing
    pub fn new_uninitialized(size: usize) -> FfiObject {
        FfiObject::_from_vec(Vec::with_capacity(size), size)
    }

    pub fn into_vec(self, length: usize) -> Vec<u8> {
        unsafe {
            let v = Vec::from_raw_parts(self.ptr, length, self.size);
            std::mem::forget(self);
            return v;
        }
    }

    fn _from_vec(mut v: Vec<u8>, size: usize) -> FfiObject {
        assert!(size > 0);
        let ptr = v.as_mut_ptr();
        std::mem::forget(v);
        FfiObject { ptr, size }
    }
}

impl Drop for FfiObject {
    fn drop(&mut self) {
        unsafe { std::mem::drop(Vec::from_raw_parts(self.ptr, 0, self.size)) };
    }
}

#[inline(always)]
fn spx_populate_string(handle: SPXHANDLE, max_chars: usize,
                       f: unsafe extern "C" fn(SPXHANDLE, *mut c_char, u32) -> SPXHR) -> Result<String, SpxError> {
    #[inline(always)]
    unsafe fn find_nul_char(ptr: *mut c_char, size: usize) -> Result<usize, SpxError> {
        for i in 0..size {
            if *ptr.offset(i as isize) == 0 {
                return Ok(i);
            }
        }
        Err(SpxError::InvalidCString)
    }

    let buff = FfiObject::new_uninitialized(max_chars + 1);
    let len = unsafe {
        let ptr = buff.ptr as *mut c_char;
        convert_err(f(handle, ptr, buff.size as u32))?;
        find_nul_char(ptr, buff.size)
    }?;
    let vec = buff.into_vec(len);
    Ok(String::from_utf8(vec)?)
}

#[inline(always)]
fn spx_populate<T>(handle: SPXHANDLE,
                   f: unsafe extern "C" fn(SPXHANDLE, *mut T) -> SPXHR) -> Result<T, SpxError> {
    unsafe {
        let mut result: T = std::mem::uninitialized();
        convert_err(f(handle, &mut result))?;
        return Ok(result);
    }
}
