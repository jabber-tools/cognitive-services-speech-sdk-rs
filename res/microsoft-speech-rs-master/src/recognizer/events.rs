use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use num::FromPrimitive;

use crate::CancellationErrorCode;
use crate::CancellationReason;
use crate::FromHandle;
use crate::SmartHandle;
use crate::speech_api::*;
use crate::SpxError;

const SESSION_ID_SIZE: usize = 32; // UUID

pub trait EventFactory: Sized {
    fn create(handle: SPXEVENTHANDLE) -> Result<Self, SpxError>;
}

// Event

pub struct Event {
    handle: SmartHandle<SPXEVENTHANDLE>,
}

impl EventFactory for Event {
    #[inline]
    fn create(handle: SPXEVENTHANDLE) -> Result<Event, SpxError> {
        Ok(Event {
            handle: SmartHandle::create("Event", handle, recognizer_event_handle_release),
        })
    }
}

impl Event {
    #[inline]
    pub fn get_handle(&self) -> SPXEVENTHANDLE {
        self.handle.get()
    }
}

// SessionEvent

pub struct SessionEvent {
    base: Event,
}

impl Deref for SessionEvent {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl EventFactory for SessionEvent {
    #[inline]
    fn create(handle: SPXEVENTHANDLE) -> Result<SessionEvent, SpxError> {
        Ok(SessionEvent {
            base: Event::create(handle)?,
        })
    }
}

impl SessionEvent {
    pub fn session_id(&self) -> Result<String, SpxError> {
        crate::spx_populate_string(
            self.get_handle(),
            SESSION_ID_SIZE,
            recognizer_session_event_get_session_id,
        )
    }
}

// RecognitionEvent

pub struct RecognitionEvent {
    base: SessionEvent,
}

impl Deref for RecognitionEvent {
    type Target = SessionEvent;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl EventFactory for RecognitionEvent {
    #[inline]
    fn create(handle: SPXEVENTHANDLE) -> Result<RecognitionEvent, SpxError> {
        Ok(RecognitionEvent {
            base: SessionEvent::create(handle)?,
        })
    }
}

impl RecognitionEvent {
    pub fn offset(&self) -> Result<u64, SpxError> {
        crate::spx_populate(self.get_handle(), recognizer_recognition_event_get_offset)
    }
}

// BaseRecognitionResultEvent

pub struct BaseRecognitionResultEvent {
    base: RecognitionEvent,
    result_handle: Arc<SmartHandle<SPXRESULTHANDLE>>,
}

unsafe impl Send for BaseRecognitionResultEvent {}

impl Deref for BaseRecognitionResultEvent {
    type Target = RecognitionEvent;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl EventFactory for BaseRecognitionResultEvent {
    #[inline]
    fn create(handle: SPXEVENTHANDLE) -> Result<BaseRecognitionResultEvent, SpxError> {
        Ok(BaseRecognitionResultEvent {
            base: RecognitionEvent::create(handle)?,
            result_handle: Self::get_result_handle(handle)?,
        })
    }
}

impl BaseRecognitionResultEvent {
    #[inline(always)]
    fn get_result_handle(event_handle: SPXEVENTHANDLE) -> Result<Arc<SmartHandle<SPXRESULTHANDLE>>, SpxError> {
        let handle = crate::spx_populate(event_handle, recognizer_recognition_event_get_result)?;
        Ok(Arc::new(SmartHandle::create("RecognitionResult", handle, recognizer_result_handle_release)))
    }
}

// RecognitionResultEvent

pub struct RecognitionResultEvent<R> {
    base: BaseRecognitionResultEvent,
    phantom_r: PhantomData<R>,
}

impl<R> Deref for RecognitionResultEvent<R> {
    type Target = BaseRecognitionResultEvent;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<R> EventFactory for RecognitionResultEvent<R> {
    #[inline]
    fn create(handle: SPXEVENTHANDLE) -> Result<RecognitionResultEvent<R>, SpxError> {
        Ok(RecognitionResultEvent {
            base: BaseRecognitionResultEvent::create(handle)?,
            phantom_r: PhantomData,
        })
    }
}

impl<R> RecognitionResultEvent<R>
    where R: FromHandle<Arc<SmartHandle<SPXRESULTHANDLE>>, SpxError> {
    pub fn result(&self) -> Result<R, SpxError> {
        R::from_handle(self.result_handle.clone())
    }
}

// RecognitionCanceledEvent

pub struct RecognitionCanceledEvent {
    base: BaseRecognitionResultEvent,
}


impl Deref for RecognitionCanceledEvent {
    type Target = BaseRecognitionResultEvent;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl EventFactory for RecognitionCanceledEvent {
    #[inline]
    fn create(handle: SPXEVENTHANDLE) -> Result<RecognitionCanceledEvent, SpxError> {
        Ok(RecognitionCanceledEvent {
            base: BaseRecognitionResultEvent::create(handle)?,
        })
    }
}

impl RecognitionCanceledEvent {
    pub fn reason(&self) -> Result<CancellationReason, SpxError> {
        let code = crate::spx_populate(self.result_handle.get(), result_get_reason_canceled)?;
        return Ok(CancellationReason::from_u32(code).expect("unknown reason"));
    }

    pub fn err_code(&self) -> Result<CancellationErrorCode, SpxError> {
        let code = crate::spx_populate(self.result_handle.get(), result_get_canceled_error_code)?;
        return Ok(CancellationErrorCode::from_u32(code).expect("unknown code"));
    }
}
