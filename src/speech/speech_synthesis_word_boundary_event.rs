use crate::common::SpeechSynthesisBoundaryType;
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
    pub duration_ms: u64,
    pub text_offset: u32,
    pub word_length: u32,
    pub boundary_type: SpeechSynthesisBoundaryType,
}

impl SpeechSynthesisWordBoundaryEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_offset = MaybeUninit::uninit();
            let mut duration_ms = MaybeUninit::uninit();
            let mut text_offset = MaybeUninit::uninit();
            let mut word_length = MaybeUninit::uninit();
            let mut boundary_type = MaybeUninit::uninit();
            let ret = synthesizer_word_boundary_event_get_values(
                handle,
                audio_offset.as_mut_ptr(),
                duration_ms.as_mut_ptr(),
                text_offset.as_mut_ptr(),
                word_length.as_mut_ptr(),
                boundary_type.as_mut_ptr(),
            );
            convert_err(ret, "SpeechSynthesisWordBoundaryEvent::from_handle error")?;

            #[cfg(target_os = "windows")]
            let boundary_type = SpeechSynthesisBoundaryType::from_i32(boundary_type.assume_init());
            #[cfg(not(target_os = "windows"))]
            let boundary_type = SpeechSynthesisBoundaryType::from_u32(boundary_type.assume_init());

            let audio_offset = audio_offset.assume_init();
            let duration_ms = duration_ms.assume_init();
            let text_offset = text_offset.assume_init();
            let word_length = word_length.assume_init();

            Ok(SpeechSynthesisWordBoundaryEvent {
                handle: SmartHandle::create(
                    "SpeechSynthesisWordBoundaryEvent",
                    handle,
                    synthesizer_event_handle_release,
                ),
                audio_offset,
                duration_ms,
                text_offset,
                word_length,
                boundary_type,
            })
        }
    }
}
