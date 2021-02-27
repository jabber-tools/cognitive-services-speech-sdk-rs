use std;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

use futures::prelude::*;
use tokio::timer::Interval;

use crate::{convert_err, ResultHandleSupport};
use crate::FromHandle;
use crate::SmartHandle;
use crate::speech_api::*;
use crate::SpxError;
use crate::SPXHANDLE_INVALID;

const ACTION_POLL_INTERVAL_MS: u64 = 30;
const RESULT_POLL_INTERVAL_MS: u64 = 100;

const SPXERR_TIMEOUT: SPXHR = 0x06;

pub trait AsyncStart {
    fn name() -> &'static str;

    unsafe fn async_start(&self, hasync: &mut SPXASYNCHANDLE) -> SPXHR;
}

pub trait AsyncWait {
    unsafe fn async_wait(&self, hasync: SPXASYNCHANDLE, timeout: u32) -> SPXHR;
}

pub struct AsyncWaitFn {
    wait_fn: unsafe extern "C" fn(SPXASYNCHANDLE, u32) -> SPXHR,
}

impl AsyncWait for AsyncWaitFn {
    unsafe fn async_wait(&self, hasync: SPXASYNCHANDLE, timeout: u32) -> SPXHR {
        (self.wait_fn)(hasync, timeout)
    }
}

pub struct BaseAsyncHandle<S, W> {
    handle: Option<SmartHandle<SPXASYNCHANDLE>>,
    release_fn: unsafe extern "C" fn(SPXASYNCHANDLE) -> SPXHR,
    timer: Interval,
    async_wait: W,
    // for lazy initialization
    async_start: S,
}

unsafe impl<S, W> Sync for BaseAsyncHandle<S, W> {}

unsafe impl<S, W> Send for BaseAsyncHandle<S, W> {}

impl<S: AsyncStart, W: AsyncWait> BaseAsyncHandle<S, W> {
    pub(crate)
    fn create(async_start: S,
              release_fn: unsafe extern "C" fn(SPXASYNCHANDLE) -> SPXHR,
              async_wait: W,
              poll_interval: Duration) -> Result<BaseAsyncHandle<S, W>, SpxError> {
        Ok(BaseAsyncHandle {
            handle: None,
            release_fn,
            timer: Interval::new(Instant::now(), poll_interval),
            async_wait,
            async_start,
        })
    }
}

impl<S: AsyncStart, W: AsyncWait> Future for BaseAsyncHandle<S, W> {
    type Item = ();
    type Error = SpxError;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        if self.handle.is_none() {
            let mut handle = SPXHANDLE_INVALID;
            unsafe {
                convert_err(self.async_start.async_start(&mut handle))?
            }
            self.handle = Some(SmartHandle::create(
                S::name(),
                handle,
                self.release_fn,
            ));
        }
        match self.timer.poll().expect("timer failure") {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(_) => {
                let hr = unsafe {
                    self.async_wait.async_wait(self.handle.as_ref().unwrap().get(), 0)
                };
                if hr == SPXERR_TIMEOUT {
                    self.poll()
                } else {
                    convert_err(hr)?;
                    Ok(Async::Ready(()))
                }
            }
        }
    }
}

pub struct AsyncHandle<S> {
    base: BaseAsyncHandle<S, AsyncWaitFn>,
}

impl<S: AsyncStart> AsyncHandle<S> {
    #[inline]
    pub(crate)
    fn create(async_start: S,
              release_fn: unsafe extern "C" fn(SPXASYNCHANDLE) -> SPXHR,
              wait_fn: unsafe extern "C" fn(SPXASYNCHANDLE, u32) -> SPXHR) -> Result<AsyncHandle<S>, SpxError> {
        Ok(AsyncHandle {
            base: BaseAsyncHandle::create(
                async_start,
                release_fn,
                AsyncWaitFn { wait_fn },
                Duration::from_millis(ACTION_POLL_INTERVAL_MS),
            )?
        })
    }
}

impl<S: AsyncStart> Future for AsyncHandle<S> {
    type Item = ();
    type Error = SpxError;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.base.poll()
    }
}

pub struct AsyncResultWait {
    wait_fn: unsafe extern "C" fn(SPXASYNCHANDLE, u32, *mut SPXRESULTHANDLE) -> SPXHR,
    result_handle_ptr: *mut SPXRESULTHANDLE,
}

impl AsyncWait for AsyncResultWait {
    unsafe fn async_wait(&self, hasync: SPXASYNCHANDLE, timeout: u32) -> SPXHR {
        (self.wait_fn)(hasync, timeout, self.result_handle_ptr)
    }
}

pub struct AsyncResultHandle<S, V: ResultHandleSupport> {
    base: BaseAsyncHandle<S, AsyncResultWait>,
    result_handle: Option<Box<SPXRESULTHANDLE>>,
    phantom_v: PhantomData<V>,
}

impl<S: AsyncStart, V: ResultHandleSupport> AsyncResultHandle<S, V> {
    #[inline]
    pub(crate)
    fn create(async_start: S,
              release_fn: unsafe extern "C" fn(SPXASYNCHANDLE) -> SPXHR)
              -> Result<AsyncResultHandle<S, V>, SpxError> {
        let mut result_handle = Box::new(SPXHANDLE_INVALID);
        let async_wait = AsyncResultWait {
            wait_fn: V::async_wait_fn(),
            result_handle_ptr: &mut *result_handle,
        };
        Ok(AsyncResultHandle {
            base: BaseAsyncHandle::create(
                async_start,
                release_fn,
                async_wait,
                Duration::from_millis(RESULT_POLL_INTERVAL_MS),
            )?,
            result_handle: Some(result_handle),
            phantom_v: PhantomData,
        })
    }
}

impl<S, V> Future for AsyncResultHandle<S, V>
    where S: AsyncStart,
          V: FromHandle<SPXRESULTHANDLE, SpxError>,
          V: ResultHandleSupport {
    type Item = V;
    type Error = SpxError;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.base.poll()? {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(_) => {
                let result_handle =
                    std::mem::replace(&mut self.result_handle, None);
                let v = V::from_handle(*result_handle.expect("result_handle is none"))?;
                Ok(Async::Ready(v))
            }
        }
    }
}

impl<S, V: ResultHandleSupport> Drop for AsyncResultHandle<S, V> {
    fn drop(&mut self) {
        if let Some(ref h) = self.result_handle {
            let h = **h;
            if h != SPXHANDLE_INVALID {
                unsafe {
                    (V::release_fn())(h);
                }
            }
        }
    }
}
