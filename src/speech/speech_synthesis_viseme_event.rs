use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_free_string, synthesizer_event_handle_release,
    synthesizer_viseme_event_get_animation, synthesizer_viseme_event_get_values, SmartHandle,
    SPXEVENTHANDLE,
};
use std::ffi::CStr;
use std::mem::MaybeUninit;

/// Event passed into speech synthetizer's callback set_synthesizer_viseme_cb.
#[derive(Debug)]
pub struct SpeechSynthesisVisemeEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub audio_offset: u64,
    pub viseme_id: u32,
    pub animation: String,
}

impl SpeechSynthesisVisemeEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_offset = MaybeUninit::uninit();
            let mut viseme_id = MaybeUninit::uninit();
            let mut ret = synthesizer_viseme_event_get_values(
                handle,
                audio_offset.as_mut_ptr(),
                viseme_id.as_mut_ptr(),
            );
            convert_err(ret, "SpeechSynthesisVisemeEvent::from_handle error")?;

            let c_animation = synthesizer_viseme_event_get_animation(handle);
            let animation = CStr::from_ptr(c_animation).to_str()?.to_owned();

            ret = property_bag_free_string(c_animation);
            convert_err(
                ret,
                "SpeechSynthesisVisemeEvent::from_handle(property_bag_free_string) error",
            )?;

            let audio_offset = audio_offset.assume_init();
            let viseme_id = viseme_id.assume_init();
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
