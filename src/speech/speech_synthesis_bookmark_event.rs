use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_free_string, synthesizer_bookmark_event_get_values, synthesizer_event_get_text,
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
            let mut audio_offset = MaybeUninit::uninit();
            let mut ret = synthesizer_bookmark_event_get_values(handle, audio_offset.as_mut_ptr());
            convert_err(ret, "SpeechSynthesisBookmarkEvent::from_handle error")?;

            let c_text = synthesizer_event_get_text(handle);
            let text = CStr::from_ptr(c_text).to_str()?.to_owned();

            ret = property_bag_free_string(c_text);
            convert_err(
                ret,
                "SpeechSynthesisBookmarkEvent::from_handle(property_bag_free_string) error",
            )?;

            let audio_offset = audio_offset.assume_init();
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
