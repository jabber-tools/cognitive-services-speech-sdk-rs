use log::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

pub mod audio;
pub mod error;
pub mod ffi;

const SPXHANDLE_EMPTY: ffi::SPXHANDLE = 0 as ffi::SPXHANDLE;

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
    #[allow(dead_code)]
    fn get(&self) -> T {
        self.inner
    }
}

impl<T: Copy + Debug> Drop for SmartHandle<T> {
    fn drop(&mut self) {
        trace!("Drop SmartHandle {}.", self);
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
