#![allow(warnings)]
include!("ffi/bindings.rs");

use log::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

// using std::mem::MaybeUninit::uninit().assume_init(); instead
// pub const SPXHANDLE_EMPTY: SPXHANDLE = 0 as SPXHANDLE;

#[derive(Debug)]
pub struct SmartHandle<T: Copy + Debug> {
    inner: T,
    release_fn: unsafe extern "C" fn(T) -> SPXHR,
    name: &'static str,
}

impl<T: Copy + Debug> SmartHandle<T> {
    #[inline(always)]
    pub fn create(
        name: &'static str,
        handle: T,
        release_fn: unsafe extern "C" fn(T) -> SPXHR,
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
    pub fn inner(&self) -> T {
        self.inner
    }
}

impl<T: Copy + Debug> Drop for SmartHandle<T> {
    fn drop(&mut self) {
        trace!("Drop SmartHandle {}.", self);
        let hr = unsafe { (self.release_fn)(self.inner) };
        if hr != SPX_NOERROR as usize {
            panic!("cannot release SmartHandle {}, err={}", self, hr);
        }
    }
}

impl<T: Copy + Debug> Display for SmartHandle<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{{{:?}}}", self.name, self.inner)
    }
}

unsafe impl<T: Copy + Debug> Send for SmartHandle<T> {}
