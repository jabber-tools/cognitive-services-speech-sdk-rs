use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_free_string, synthesizer_event_handle_release,
    synthesizer_viseme_event_get_animation, synthesizer_viseme_event_get_values, SmartHandle,
    SPXEVENTHANDLE,
};
use std::ffi::CStr;

/// Event passed into speech synthetizer's callback set_synthesizer_viseme_cb.
#[derive(Debug)]
pub struct SpeechSynthesisVisemeEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub audio_offset: u64,
    pub viseme_id: u32,
    pub animation: String,
}

impl SpeechSynthesisVisemeEvent {
    /// # Safety
    /// `handle` must be a valid handle to a live speech synthesis viseme event.
    pub unsafe fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_offset: u64 = 0;
            let mut viseme_id: u32 = 0;
            let mut ret =
                synthesizer_viseme_event_get_values(handle, &mut audio_offset, &mut viseme_id);
            convert_err(ret, "SpeechSynthesisVisemeEvent::from_handle error")?;

            let c_animation = synthesizer_viseme_event_get_animation(handle);
            let animation = CStr::from_ptr(c_animation).to_str()?.to_owned();

            ret = property_bag_free_string(c_animation);
            convert_err(
                ret,
                "SpeechSynthesisVisemeEvent::from_handle(property_bag_free_string) error",
            )?;

            Ok(SpeechSynthesisVisemeEvent {
                handle: SmartHandle::create(
                    "SpeechSynthesisVisemeEvent",
                    handle,
                    synthesizer_event_handle_release,
                ),
                audio_offset,
                viseme_id,
                animation,
            })
        }
    }
}
