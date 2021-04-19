use crate::error::{convert_err, Result};
use crate::ffi::{
    synthesizer_bookmark_event_get_text, synthesizer_bookmark_event_get_values,
    synthesizer_event_handle_release, SmartHandle, SPXEVENTHANDLE,
};
use std::ffi::CStr;
use std::mem::MaybeUninit;

/// Event passed into speech synthetizer's callback set_synthesizer_bookmark_cb.
#[derive(Debug)]
pub struct SpeechSynthesisBookmarkEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub audio_offset: u64,
    pub text: String,
}

impl SpeechSynthesisBookmarkEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_offset: u64 = MaybeUninit::uninit().assume_init();
            let ret = synthesizer_bookmark_event_get_values(handle, &mut audio_offset);
            convert_err(ret, "SpeechSynthesisEvent::from_handle error")?;

            let c_text = synthesizer_bookmark_event_get_text(handle);
            let text = CStr::from_ptr(c_text).to_str()?.to_owned();

            Ok(SpeechSynthesisBookmarkEvent {
                handle: SmartHandle::create(
                    "SpeechSynthesisBookmarkEvent",
                    handle,
                    synthesizer_event_handle_release,
                ),
                audio_offset,
                text,
            })
        }
    }
}
