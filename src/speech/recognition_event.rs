use crate::error::{convert_err, Result};
use crate::ffi::{recognizer_recognition_event_get_offset, SPXEVENTHANDLE};
use crate::speech::SessionEvent;
use log::*;
use std::mem::MaybeUninit;

/// Recognition event extending *SessionEvent* passed into callbacks *set_speech_start_detected_cb* and *set_speech_end_detected_cb*.
#[derive(Debug)]
pub struct RecognitionEvent {
    pub base: SessionEvent,
    pub offset: u64,
}

impl RecognitionEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<RecognitionEvent> {
        let base = SessionEvent::from_handle(handle)?;
        trace!("RecognitionEvent::from_handle got base event {:?}", base);
        unsafe {
            let mut offset = MaybeUninit::uninit();
            trace!("calling recognizer_recognition_event_get_offset");
            let ret = recognizer_recognition_event_get_offset(handle, offset.as_mut_ptr());
            convert_err(ret, "RecognitionEvent::from_handle error")?;
            let offset = offset.assume_init();
            trace!("recognizer_recognition_event_get_offset offset: {}", offset);
            Ok(RecognitionEvent { base, offset })
        }
    }
}
