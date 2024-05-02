//! Package ffi contains bindings to underlying C API and thin abstraction for managing C handles.
#![allow(warnings)]
#![allow(unaligned_references)]
include!("ffi/bindings.rs");

// manual entry as API v1.21.0 is using this types as #define so bindings.rs does not contains them
pub type SPXPROPERTYBAGHANDLE = AZAC_HANDLE;
pub type SPXHANDLE = AZAC_HANDLE;
pub type SPXHR = AZACHR;

use log::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::os::raw::c_char;
pub const NULL_C_STR_PTR: *const c_char = 0 as *const c_char;

// using std::mem::MaybeUninit::uninit(); instead
// pub const SPXHANDLE_EMPTY: SPXHANDLE = 0 as SPXHANDLE;

/// Wrapper struct around underlying native handles
/// Apart from handle it wraps release function
/// that is automatically called when SmartHandle
/// is dropped (see Drop trait implementation).
#[derive(Debug)]
pub struct SmartHandle<T: Copy + Debug> {
    inner: T,
    release_fn: unsafe extern "C" fn(T) -> SPXHR,
    name: &'static str,
}

/// Creates SmartHandle from underlying native
/// handle and respective release function.
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

/// Calls release function when handle is being dropped
/// This ensures underlying native resources are released properly.
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

/// Send implementation so that we can send SmartHandles
/// accross threads.
unsafe impl<T: Copy + Debug> Send for SmartHandle<T> {}
