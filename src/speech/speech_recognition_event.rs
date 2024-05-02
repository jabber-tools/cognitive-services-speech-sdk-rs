use crate::error::{convert_err, Result};
use crate::ffi::{recognizer_recognition_event_get_result, SPXEVENTHANDLE, SPXRESULTHANDLE};
use crate::speech::{RecognitionEvent, SpeechRecognitionResult};
use log::*;
use std::mem::MaybeUninit;

/// Recognition event extending *RecognitionEvent* passed into callbacks *set_recognizing_cb* and *set_recognized_cb*.
#[derive(Debug)]
pub struct SpeechRecognitionEvent {
    pub base: RecognitionEvent,
    pub result: SpeechRecognitionResult,
}

impl SpeechRecognitionEvent {
    /// # Safety
    /// `handle` must be a valid handle to a live speech recognition event.
    pub unsafe fn from_handle(handle: SPXEVENTHANDLE) -> Result<SpeechRecognitionEvent> {
        unsafe {
            let base = RecognitionEvent::from_handle(handle)?;
            let mut result_handle: MaybeUninit<SPXRESULTHANDLE> = MaybeUninit::uninit();
            trace!("calling recognizer_recognition_event_get_result");
            let ret = recognizer_recognition_event_get_result(handle, result_handle.as_mut_ptr());
            convert_err(ret, "SpeechRecognitionEvent::from_handle error")?;
            trace!("called recognizer_recognition_event_get_result");
            let result = SpeechRecognitionResult::from_handle(result_handle.assume_init())?;
            Ok(SpeechRecognitionEvent { base, result })
        }
    }
}
