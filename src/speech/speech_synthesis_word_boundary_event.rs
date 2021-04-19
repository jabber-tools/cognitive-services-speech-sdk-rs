use crate::error::{convert_err, Result};
use crate::ffi::{
    synthesizer_event_handle_release, synthesizer_word_boundary_event_get_values, SmartHandle,
    SPXEVENTHANDLE,
};
use std::mem::MaybeUninit;

/// Event passed into speech synthetizer's callback set_synthesizer_word_boundary_cb.
#[derive(Debug)]
pub struct SpeechSynthesisWordBoundaryEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub audio_offset: u64,
    pub text_offset: u32,
    pub word_length: u32,
}

impl SpeechSynthesisWordBoundaryEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_offset: u64 = MaybeUninit::uninit().assume_init();
            let mut text_offset: u32 = MaybeUninit::uninit().assume_init();
            let mut word_length: u32 = MaybeUninit::uninit().assume_init();
            let ret = synthesizer_word_boundary_event_get_values(
                handle,
                &mut audio_offset,
                &mut text_offset,
                &mut word_length,
            );
            convert_err(ret, "SpeechSynthesisWordBoundaryEvent::from_handle error")?;

            Ok(SpeechSynthesisWordBoundaryEvent {
                handle: SmartHandle::create(
                    "SpeechSynthesisWordBoundaryEvent",
                    handle,
                    synthesizer_event_handle_release,
                ),
                audio_offset,
                text_offset,
                word_length,
            })
        }
    }
}
