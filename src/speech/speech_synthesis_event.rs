use crate::error::{convert_err, Result};
use crate::ffi::{
    synthesizer_event_handle_release, synthesizer_synthesis_event_get_result, SmartHandle,
    SPXEVENTHANDLE, SPXRESULTHANDLE,
};
use crate::speech::SpeechSynthesisResult;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct SpeechSynthesisEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub result: SpeechSynthesisResult,
}

impl SpeechSynthesisEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut result_handle: SPXRESULTHANDLE = MaybeUninit::uninit().assume_init();
            let ret = synthesizer_synthesis_event_get_result(handle, &mut result_handle);
            convert_err(ret, "SpeechSynthesisEvent::from_handle error")?;
            let result = SpeechSynthesisResult::from_handle(result_handle)?;
            Ok(SpeechSynthesisEvent {
                handle: SmartHandle::create(
                    "SpeechSynthesisEvent",
                    handle,
                    synthesizer_event_handle_release,
                ),
                result,
            })
        }
    }
}
