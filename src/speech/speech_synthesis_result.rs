use crate::common::{PropertyCollection, ResultReason};
use crate::error::{convert_err, Result};
use crate::ffi::{
    synth_result_get_audio_data, synth_result_get_audio_length_duration,
    synth_result_get_property_bag, synth_result_get_reason, synth_result_get_result_id,
    synthesizer_result_handle_release, SmartHandle, SPXRESULTHANDLE,
};
use std::convert::TryFrom;
use std::ffi::CStr;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::c_char;

/// Represents speech synthetis result contained in SpeechSynthesisEvent callback event.
pub struct SpeechSynthesisResult {
    pub handle: SmartHandle<SPXRESULTHANDLE>,
    pub result_id: String,
    pub reason: ResultReason,
    pub audio_data: Vec<u8>,
    pub audio_duration_ms: u64,
    pub properties: PropertyCollection,
}

impl fmt::Debug for SpeechSynthesisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let audio_data_truncated = if self.audio_data.len() > 10 {
            &self.audio_data[..10]
        } else {
            &self.audio_data[..]
        };
        f.debug_struct("SpeechRecognitionResult")
            .field("result_id", &self.result_id)
            .field("reason", &self.reason)
            .field(
                "audio_data",
                &format!("(Truncated): {:?}", &audio_data_truncated),
            )
            // .field("audio_data", &self.audio_data)
            .finish()
    }
}

impl SpeechSynthesisResult {
    pub fn from_handle(handle: SPXRESULTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_length = MaybeUninit::uninit();
            let mut audio_duration = MaybeUninit::uninit();
            let mut ret = synth_result_get_audio_length_duration(
                handle,
                audio_length.as_mut_ptr(),
                audio_duration.as_mut_ptr(),
            );
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_audio_length) error",
            )?;

            let audio_duration = audio_duration.assume_init();
            let mut audio_length = audio_length.assume_init();
            // we will allocate at least 1024 bytes
            // in reality audio_length returned by
            // synth_result_get_audio_length might be much bigger
            if audio_length < 1024 {
                audio_length = 1024;
            }

            let c_buf: *mut c_char = &mut [0u8; 37] as *const _ as *mut c_char;
            ret = synth_result_get_result_id(handle, c_buf, 37);
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_result_id) error",
            )?;
            let result_id = CStr::from_ptr(c_buf).to_str()?.to_owned();

            let mut reason = MaybeUninit::uninit();
            ret = synth_result_get_reason(handle, reason.as_mut_ptr());
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_reason) error",
            )?;

            let mut c_buf2_vec = vec![0u8; audio_length as usize];
            let c_buf2: *mut u8 = &mut c_buf2_vec[..] as *const _ as *mut u8;
            let mut filled_size = MaybeUninit::uninit();
            ret =
                synth_result_get_audio_data(handle, c_buf2, audio_length, filled_size.as_mut_ptr());
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_audio_data) error",
            )?;

            let converted_size = usize::try_from(filled_size.assume_init())?;
            let slice_buffer = std::slice::from_raw_parts_mut(c_buf2, converted_size);

            let mut properties_handle = MaybeUninit::uninit();
            ret = synth_result_get_property_bag(handle, properties_handle.as_mut_ptr());
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_property_bag) error",
            )?;
            let properties = PropertyCollection::from_handle(properties_handle.assume_init());

            #[cfg(target_os = "windows")]
            let reason = ResultReason::from_i32(reason.assume_init());
            #[cfg(not(target_os = "windows"))]
            let reason = ResultReason::from_u32(reason.assume_init());

            let speech_synthesis_result = SpeechSynthesisResult {
                handle: SmartHandle::create(
                    "SpeechSynthesisResult",
                    handle,
                    synthesizer_result_handle_release,
                ),
                result_id,
                reason,
                audio_data: slice_buffer.to_vec(),
                audio_duration_ms: audio_duration,
                properties,
            };
            Ok(speech_synthesis_result)
        }
    }
}
