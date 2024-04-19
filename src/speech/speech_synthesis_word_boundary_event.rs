use crate::common::SpeechSynthesisBoundaryType;
use crate::error::{convert_err, Result};
use crate::ffi::{
    synthesizer_event_handle_release, synthesizer_word_boundary_event_get_values, SmartHandle,
    SpeechSynthesis_BoundaryType, SPXEVENTHANDLE,
};

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
    /// # Safety
    /// `handle` must be a valid handle to a live speech synthesis word boundary event.
    pub unsafe fn from_handle(handle: SPXEVENTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_offset: u64 = 0;
            let mut duration_ms: u64 = 0;
            let mut text_offset: u32 = 0;
            let mut word_length: u32 = 0;
            let mut boundary_type: SpeechSynthesis_BoundaryType = 0;
            let ret = synthesizer_word_boundary_event_get_values(
                handle,
                &mut audio_offset,
                &mut duration_ms,
                &mut text_offset,
                &mut word_length,
                &mut boundary_type,
            );
            convert_err(ret, "SpeechSynthesisWordBoundaryEvent::from_handle error")?;

            #[cfg(target_os = "windows")]
            let boundary_type = SpeechSynthesisBoundaryType::from_i32(boundary_type);
            #[cfg(not(target_os = "windows"))]
            let boundary_type = SpeechSynthesisBoundaryType::from_u32(boundary_type);

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
