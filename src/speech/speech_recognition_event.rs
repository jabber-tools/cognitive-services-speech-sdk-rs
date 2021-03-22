use crate::error::{convert_err, Result};
use crate::ffi::{recognizer_recognition_event_get_result, SPXEVENTHANDLE, SPXRESULTHANDLE};
use crate::speech::{RecognitionEvent, SpeechRecognitionResult};
use log::*;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct SpeechRecognitionEvent {
    pub base: RecognitionEvent,
    pub result: SpeechRecognitionResult,
}

impl SpeechRecognitionEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<SpeechRecognitionEvent> {
        let base = RecognitionEvent::from_handle(handle)?;

        unsafe {
            let mut result_handle: SPXRESULTHANDLE = MaybeUninit::uninit().assume_init();
            trace!("calling recognizer_recognition_event_get_result");
            let ret = recognizer_recognition_event_get_result(handle, &mut result_handle);
            convert_err(ret, "SpeechRecognitionEvent::from_handle error")?;
            trace!("called recognizer_recognition_event_get_result");
            let result = SpeechRecognitionResult::from_handle(result_handle)?;
            Ok(SpeechRecognitionEvent {
                base,
                result: result,
            })
        }
    }
}
