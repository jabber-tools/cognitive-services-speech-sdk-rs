use log::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::MaybeUninit;
use std::os::raw::c_char;

pub mod audio;
pub mod config;
pub mod error;
pub mod events;
pub mod ffi;
pub mod property;
pub mod recognizer;

const SPXHANDLE_EMPTY: ffi::SPXHANDLE = 0 as ffi::SPXHANDLE;

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

#[derive(Debug)]
pub struct SmartHandle<T: Copy + Debug> {
    inner: T,
    release_fn: unsafe extern "C" fn(T) -> ffi::SPXHR,
    name: &'static str,
}

impl<T: Copy + Debug> SmartHandle<T> {
    #[inline(always)]
    fn create(
        name: &'static str,
        handle: T,
        release_fn: unsafe extern "C" fn(T) -> ffi::SPXHR,
    ) -> SmartHandle<T> {
        let result = SmartHandle {
            inner: handle,
            release_fn,
            name,
        };
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
        debug!("Drop SmartHandle {}.", self);
        let hr = unsafe { (self.release_fn)(self.inner) };
        if hr != ffi::SPX_NOERROR as usize {
            panic!("cannot release SmartHandle {}, err={}", self, hr);
        }
    }
}

impl<T: Copy + Debug> Display for SmartHandle<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{{{:?}}}", self.name, self.inner)
    }
}

#[inline(always)]
fn spx_populate<T>(
    handle: ffi::SPXHANDLE,
    f: unsafe extern "C" fn(ffi::SPXHANDLE, *mut T) -> ffi::SPXHR,
) -> error::Result<T> {
    unsafe {
        // let mut result: T = std::mem::uninitialized();
        let mut result: T = MaybeUninit::uninit().assume_init();
        error::convert_err(f(handle, &mut result), "spx_populate error")?;
        Ok(result)
    }
}

#[inline(always)]
fn spx_populate_string(
    handle: ffi::SPXHANDLE,
    max_chars: usize,
    f: unsafe extern "C" fn(ffi::SPXHANDLE, *mut c_char, u32) -> ffi::SPXHR,
) -> error::Result<String> {
    #[inline(always)]
    unsafe fn find_nul_char(ptr: *mut c_char, size: usize) -> error::Result<usize> {
        for i in 0..size {
            if *ptr.offset(i as isize) == 0 {
                return Ok(i);
            }
        }

        Err(error::Error::new(
            "spx_populate_string error".into(),
            error::ErrorRootCause::InvalidCString,
        ))
    }

    let buff = FfiObject::new_uninitialized(max_chars + 1);
    let len = unsafe {
        let ptr = buff.ptr as *mut c_char;
        error::convert_err(
            f(handle, ptr, buff.size as u32),
            "spx_populate_string error",
        )?;
        find_nul_char(ptr, buff.size)
    }?;
    let vec = buff.into_vec(len);
    Ok(String::from_utf8(vec)?)
}

#[derive(Debug, PartialEq)]
pub enum CancellationReason {
    Error,
    EndOfStream,
    Unknown,
}

impl CancellationReason {
    pub fn from_u32(code: u32) -> Self {
        return match code {
            1 => CancellationReason::Error,
            2 => CancellationReason::EndOfStream,
            _ => CancellationReason::Unknown,
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum CancellationErrorCode {
    NoError,
    AuthenticationFailure,
    BadRequest,
    TooManyRequests,
    Forbidden,
    ConnectionFailure,
    ServiceTimeout,
    ServiceError,
    ServiceUnavailable,
    RuntimeError,
    Unknown,
}

impl CancellationErrorCode {
    pub fn from_u32(code: u32) -> Self {
        return match code {
            0 => CancellationErrorCode::NoError,
            1 => CancellationErrorCode::AuthenticationFailure,
            2 => CancellationErrorCode::BadRequest,
            3 => CancellationErrorCode::TooManyRequests,
            4 => CancellationErrorCode::Forbidden,
            5 => CancellationErrorCode::ConnectionFailure,
            6 => CancellationErrorCode::ServiceTimeout,
            7 => CancellationErrorCode::ServiceError,
            8 => CancellationErrorCode::ServiceUnavailable,
            9 => CancellationErrorCode::RuntimeError,
            _ => CancellationErrorCode::Unknown,
        };
    }
}

pub trait FromHandle<H>: Sized {
    fn from_handle(handle: H) -> error::Result<Self>;
}
