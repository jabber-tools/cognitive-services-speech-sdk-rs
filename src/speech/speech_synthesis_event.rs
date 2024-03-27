use crate::error::{convert_err, Result};
use crate::ffi::{
    synthesizer_event_handle_release, synthesizer_synthesis_event_get_result, SmartHandle,
    SPXEVENTHANDLE,
};
use crate::speech::SpeechSynthesisResult;
use std::mem::MaybeUninit;

/// Event passed into speech synthetizer callbacks.
#[derive(Debug)]
pub struct SpeechSynthesisEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub result: SpeechSynthesisResult,
}

impl SpeechSynthesisEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut result_handle = MaybeUninit::uninit();
            let ret = synthesizer_synthesis_event_get_result(handle, result_handle.as_mut_ptr());
            convert_err(ret, "SpeechSynthesisEvent::from_handle error")?;
            let result = SpeechSynthesisResult::from_handle(result_handle.assume_init())?;
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
