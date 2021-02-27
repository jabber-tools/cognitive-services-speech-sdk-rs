use std::ffi::c_void;
use std::ops::Deref;
use std::os::raw::c_char;
use std::sync::Arc;
use std::time::Duration;

use futures::sync::mpsc::{channel, Receiver, Sender};
use num::FromPrimitive;

use crate::{AsyncHandle, AsyncResultHandle, convert_err, ResultHandleSupport};
use crate::async_handle::AsyncStart;
use crate::FromHandle;
use crate::recognizer::events::EventFactory;
use crate::recognizer::events::SessionEvent;
use crate::ResultReason;
use crate::SmartHandle;
use crate::speech_api::*;
use crate::SpxError;

pub use self::speech::*;

pub mod events;
mod speech;

const DEFAULT_CH_BUFF_SIZE: usize = 5;

const RESULT_ID_SIZE: usize = 32; // UUID

const MAX_TEXT_CHAR_COUNT: usize = 1024;

pub trait Recognizer: Send + Sync {
    fn is_enabled(&self) -> Result<bool, SpxError>;
    fn enable(&mut self) -> Result<(), SpxError>;
    fn disable(&mut self) -> Result<(), SpxError>;
    fn get_handle(&self) -> SPXRECOHANDLE;
}

pub trait AsyncRecognizer<R, E, C>: Deref<Target=dyn Recognizer> {
    fn start_continuous_recognition(&mut self) -> Result<AsyncHandle<StartContinuousRecognitionAsyncStart>, SpxError>;
    fn stop_continuous_recognition(&mut self) -> Result<AsyncHandle<StopContinuousRecognitionAsyncStart>, SpxError>;
    fn recognize_once_async(&mut self) -> Result<AsyncResultHandle<RecognizeOnceAsyncStart, R>, SpxError>
        where R: ResultHandleSupport;

    fn set_recognizing_channel(&mut self, v: Option<Box<Sender<E>>>);
    fn set_recognized_channel(&mut self, v: Option<Box<Sender<E>>>);
    fn set_session_started_channel(&mut self, v: Option<Box<Sender<SessionEvent>>>);
    fn set_session_stopped_channel(&mut self, v: Option<Box<Sender<SessionEvent>>>);
    fn set_canceled_channel(&mut self, v: Option<Box<Sender<C>>>);

    fn connect_recognizing(&mut self, buff_size: Option<usize>) -> Receiver<E> {
        let (s, r) = channel(buff_size.unwrap_or(DEFAULT_CH_BUFF_SIZE));
        self.set_recognizing_channel(Some(Box::new(s)));
        return r;
    }

    fn connect_recognized(&mut self, buff_size: Option<usize>) -> Receiver<E> {
        let (s, r) = channel(buff_size.unwrap_or(DEFAULT_CH_BUFF_SIZE));
        self.set_recognized_channel(Some(Box::new(s)));
        return r;
    }

    fn connect_session_started(&mut self, buff_size: Option<usize>) -> Receiver<SessionEvent> {
        let (s, r) = channel(buff_size.unwrap_or(DEFAULT_CH_BUFF_SIZE));
        self.set_session_started_channel(Some(Box::new(s)));
        return r;
    }

    fn connect_session_stopped(&mut self, buff_size: Option<usize>) -> Receiver<SessionEvent> {
        let (s, r) = channel(buff_size.unwrap_or(DEFAULT_CH_BUFF_SIZE));
        self.set_session_stopped_channel(Some(Box::new(s)));
        return r;
    }

    fn connect_canceled(&mut self, buff_size: Option<usize>) -> Receiver<C> {
        let (s, r) = channel(buff_size.unwrap_or(DEFAULT_CH_BUFF_SIZE));
        self.set_canceled_channel(Some(Box::new(s)));
        return r;
    }
}

struct BaseRecognizer {
    handle: SmartHandle<SPXRECOHANDLE>,
}

unsafe impl Sync for BaseRecognizer {}

impl BaseRecognizer {
    fn create(handle: SPXRECOHANDLE) -> Result<BaseRecognizer, SpxError> {
        Ok(BaseRecognizer {
            handle: SmartHandle::create("Recognizer", handle, recognizer_handle_release)
        })
    }
}

impl Recognizer for BaseRecognizer {
    fn is_enabled(&self) -> Result<bool, SpxError> {
        let mut result = false;
        unsafe {
            convert_err(recognizer_is_enabled(self.handle.get(), &mut result))?;
        }
        Ok(result)
    }

    fn enable(&mut self) -> Result<(), SpxError> {
        unsafe {
            convert_err(recognizer_enable(self.handle.get()))
        }
    }

    fn disable(&mut self) -> Result<(), SpxError> {
        unsafe {
            convert_err(recognizer_disable(self.handle.get()))
        }
    }

    #[inline]
    fn get_handle(&self) -> SPXRECOHANDLE {
        self.handle.get()
    }
}

struct AbstractAsyncRecognizer<E, C> {
    base: BaseRecognizer,
    recognizing_sender: Option<Box<Sender<E>>>,
    recognized_sender: Option<Box<Sender<E>>>,
    session_started_sender: Option<Box<Sender<SessionEvent>>>,
    session_stopped_sender: Option<Box<Sender<SessionEvent>>>,
    canceled_sender: Option<Box<Sender<C>>>,
}

impl<R, E, C> AsyncRecognizer<R, E, C> for AbstractAsyncRecognizer<E, C>
    where E: EventFactory, C: EventFactory {
    fn start_continuous_recognition(&mut self)
                                    -> Result<AsyncHandle<StartContinuousRecognitionAsyncStart>, SpxError> {
        self.set_callback(&self.canceled_sender, recognizer_canceled_set_callback)?;
        self.set_callback(&self.session_started_sender, recognizer_session_started_set_callback)?;
        self.set_callback(&self.session_stopped_sender, recognizer_session_stopped_set_callback)?;
        self.set_callback(&self.recognizing_sender, recognizer_recognizing_set_callback)?;
        self.set_callback(&self.recognized_sender, recognizer_recognized_set_callback)?;
        AsyncHandle::create(
            StartContinuousRecognitionAsyncStart(self.get_handle()),
            recognizer_async_handle_release,
            recognizer_start_continuous_recognition_async_wait_for,
        )
    }

    fn stop_continuous_recognition(&mut self)
                                   -> Result<AsyncHandle<StopContinuousRecognitionAsyncStart>, SpxError> {
        AsyncHandle::create(
            StopContinuousRecognitionAsyncStart(self.get_handle()),
            recognizer_async_handle_release,
            recognizer_stop_continuous_recognition_async_wait_for,
        )
    }

    fn recognize_once_async(&mut self) -> Result<AsyncResultHandle<RecognizeOnceAsyncStart, R>, SpxError>
        where R: ResultHandleSupport {
        AsyncResultHandle::create(
            RecognizeOnceAsyncStart(self.get_handle()),
            recognizer_async_handle_release,
        )
    }

    fn set_recognizing_channel(&mut self, v: Option<Box<Sender<E>>>) {
        self.recognizing_sender = v;
    }

    fn set_recognized_channel(&mut self, v: Option<Box<Sender<E>>>) {
        self.recognized_sender = v;
    }

    fn set_session_started_channel(&mut self, v: Option<Box<Sender<SessionEvent>>>) {
        self.session_started_sender = v;
    }

    fn set_session_stopped_channel(&mut self, v: Option<Box<Sender<SessionEvent>>>) {
        self.session_stopped_sender = v;
    }

    fn set_canceled_channel(&mut self, v: Option<Box<Sender<C>>>) {
        self.canceled_sender = v;
    }
}

impl<E, C> Deref for AbstractAsyncRecognizer<E, C> {
    type Target = dyn Recognizer;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<E, C> AbstractAsyncRecognizer<E, C> {
    fn create(handle: SPXRECOHANDLE) -> Result<AbstractAsyncRecognizer<E, C>, SpxError> {
        Ok(AbstractAsyncRecognizer {
            base: BaseRecognizer::create(handle)?,
            recognizing_sender: None,
            recognized_sender: None,
            session_started_sender: None,
            session_stopped_sender: None,
            canceled_sender: None,
        })
    }

    #[inline]
    fn set_callback<T>(&self,
                       sender: &Option<Box<Sender<T>>>,
                       f: unsafe extern "C" fn(SPXRECOHANDLE, PRECOGNITION_CALLBACK_FUNC, *mut c_void) -> SPXHR) -> Result<(), SpxError>
        where T: EventFactory {
        if let Some(s) = sender {
            let s = s.as_ref();
            unsafe {
                convert_err(f(self.get_handle(), Some(Self::cb_send::<T>), s as *const _ as *mut c_void))?;
            }
        } else {
            unsafe {
                convert_err(f(self.get_handle(), None, 0 as *mut c_void))?;
            }
        }
        Ok(())
    }

    unsafe extern "C" fn cb_send<T: EventFactory>(
        _hreco: SPXRECOHANDLE,
        h_evt: SPXEVENTHANDLE,
        p_sender: *mut ::std::os::raw::c_void,
    ) {
        let sender = &mut *(p_sender as *mut Sender<T>);
        let event = match T::create(h_evt) {
            Ok(x) => x,
            Err(e) => {
                error!("can not create event, err: {}", e);
                return;
            }
        };
        match sender.try_send(event) {
            Ok(()) => {}
            Err(e) => {
                error!("can not publish event, err: {}", e);
            }
        }
    }
}

pub struct RecognizeOnceAsyncStart(SPXRECOHANDLE);

impl AsyncStart for RecognizeOnceAsyncStart {
    fn name() -> &'static str {
        "RecognizeOnceAsyncHandle"
    }

    unsafe fn async_start(&self, hasync: &mut SPXASYNCHANDLE) -> SPXHR {
        recognizer_recognize_once_async(self.0, hasync)
    }
}

pub struct StartContinuousRecognitionAsyncStart(SPXRECOHANDLE);

impl AsyncStart for StartContinuousRecognitionAsyncStart {
    fn name() -> &'static str {
        "StartContinuousRecognitionAsyncHandle"
    }

    unsafe fn async_start(&self, hasync: &mut SPXASYNCHANDLE) -> SPXHR {
        recognizer_start_continuous_recognition_async(self.0, hasync)
    }
}

pub struct StopContinuousRecognitionAsyncStart(SPXRECOHANDLE);

impl AsyncStart for StopContinuousRecognitionAsyncStart {
    fn name() -> &'static str {
        "StopContinuousRecognitionAsyncHandle"
    }

    unsafe fn async_start(&self, hasync: &mut SPXASYNCHANDLE) -> SPXHR {
        recognizer_stop_continuous_recognition_async(self.0, hasync)
    }
}

pub struct RecognitionResult {
    handle: Arc<SmartHandle<SPXRESULTHANDLE>>,
}

impl RecognitionResult {
    fn create(handle: Arc<SmartHandle<SPXRESULTHANDLE>>) -> Result<RecognitionResult, SpxError> {
        Ok(RecognitionResult {
            handle,
        })
    }

    #[inline(always)]
    pub fn get_handle(&self) -> SPXRESULTHANDLE {
        self.handle.get()
    }

    pub fn id(&self) -> Result<String, SpxError> {
        self.populate_string(RESULT_ID_SIZE, result_get_result_id)
    }

    pub fn text(&self) -> Result<String, SpxError> {
        self.populate_string(MAX_TEXT_CHAR_COUNT, result_get_text)
    }

    pub fn reason(&self) -> Result<ResultReason, SpxError> {
        let code = crate::spx_populate(self.get_handle(), result_get_reason)?;
        return Ok(ResultReason::from_u32(code).expect("unknown reason"));
    }

    pub fn offset(&self) -> Result<u64, SpxError> {
        self.populate(result_get_offset)
    }

    pub fn duration(&self) -> Result<Duration, SpxError> {
        self.populate(result_get_duration).map(|v| Duration::from_nanos(v * 100))
    }

    #[inline(always)]
    fn populate_string(&self, max_chars: usize,
                       f: unsafe extern "C" fn(SPXRESULTHANDLE, *mut c_char, u32) -> SPXHR) -> Result<String, SpxError> {
        crate::spx_populate_string(self.get_handle(), max_chars, f)
    }

    #[inline(always)]
    fn populate<T>(&self,
                   f: unsafe extern "C" fn(SPXRESULTHANDLE, *mut T) -> SPXHR) -> Result<T, SpxError> {
        crate::spx_populate(self.get_handle(), f)
    }
}

impl FromHandle<Arc<SmartHandle<SPXRESULTHANDLE>>, SpxError> for RecognitionResult {
    fn from_handle(handle: Arc<SmartHandle<SPXRESULTHANDLE>>) -> Result<RecognitionResult, SpxError> {
        RecognitionResult::create(handle)
    }
}

impl FromHandle<SPXRESULTHANDLE, SpxError> for RecognitionResult {
    fn from_handle(handle: SPXRESULTHANDLE) -> Result<RecognitionResult, SpxError> {
        RecognitionResult::create(Arc::new(SmartHandle::create(
            "RecognitionResult",
            handle,
            RecognitionResult::release_fn(),
        )))
    }
}

impl ResultHandleSupport for RecognitionResult {
    fn async_wait_fn() -> unsafe extern "C" fn(SPXASYNCHANDLE, u32, *mut SPXRESULTHANDLE) -> SPXHR {
        recognizer_recognize_once_async_wait_for
    }

    fn release_fn() -> unsafe extern "C" fn(SPXRESULTHANDLE) -> SPXHR {
        recognizer_result_handle_release
    }
}

unsafe impl Sync for RecognitionResult {}

unsafe impl Send for RecognitionResult {}
