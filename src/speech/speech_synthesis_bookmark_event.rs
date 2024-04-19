use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_free_string, synthesizer_bookmark_event_get_values, synthesizer_event_get_text,
    synthesizer_event_handle_release, SmartHandle, SPXEVENTHANDLE,
};
use std::ffi::CStr;

/// Event passed into speech synthetizer's callback set_synthesizer_bookmark_cb.
#[derive(Debug)]
pub struct SpeechSynthesisBookmarkEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub audio_offset: u64,
    pub text: String,
}

impl SpeechSynthesisBookmarkEvent {
    /// # Safety
    /// `handle` must be a valid handle to a live speech synthesis bookmark event.
    pub unsafe fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_offset: u64 = 0;
            let mut ret = synthesizer_bookmark_event_get_values(handle, &mut audio_offset);
            convert_err(ret, "SpeechSynthesisBookmarkEvent::from_handle error")?;

            let c_text = synthesizer_event_get_text(handle);
            let text = CStr::from_ptr(c_text).to_str()?.to_owned();

            ret = property_bag_free_string(c_text);
            convert_err(
                ret,
                "SpeechSynthesisBookmarkEvent::from_handle(property_bag_free_string) error",
            )?;

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
