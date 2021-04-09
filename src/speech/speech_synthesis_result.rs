use crate::common::{PropertyCollection, ResultReason};
use crate::error::{convert_err, Result};
use crate::ffi::{
    synth_result_get_audio_data, synth_result_get_audio_length, synth_result_get_property_bag,
    synth_result_get_reason, synth_result_get_result_id, synthesizer_result_handle_release,
    SmartHandle, SPXPROPERTYBAGHANDLE, SPXRESULTHANDLE,
};
use std::convert::TryFrom;
use std::ffi::CStr;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_uint};

pub struct SpeechSynthesisResult {
    pub handle: SmartHandle<SPXRESULTHANDLE>,
    pub result_id: String,
    pub reason: ResultReason,
    pub audio_data: Vec<u8>,
    pub properties: PropertyCollection,
}

impl fmt::Debug for SpeechSynthesisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpeechRecognitionResult")
            .field("result_id", &self.result_id)
            .field("reason", &self.reason)
            .field("audio_data", &self.audio_data)
            .finish()
    }
}

impl SpeechSynthesisResult {
    pub fn from_handle(handle: SPXRESULTHANDLE) -> Result<Self> {
        unsafe {
            let mut audio_length: u32 = MaybeUninit::uninit().assume_init();
            let mut ret = synth_result_get_audio_length(handle, &mut audio_length);
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_audio_length) error",
            )?;

            // using max(1024, cAudioLength) as buffer size
            if audio_length < 1024 {
                audio_length = 1024;
            }

            let c_buf: *mut c_char = &mut [0u8; 1024] as *const _ as *mut c_char;
            ret = synth_result_get_result_id(handle, c_buf, 1024);
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_result_id) error",
            )?;
            let result_id = CStr::from_ptr(c_buf).to_str()?.to_owned();

            let mut reason: c_uint = MaybeUninit::uninit().assume_init();
            ret = synth_result_get_reason(handle, &mut reason);
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_reason) error",
            )?;

            let c_buf2: *mut u8 = &mut [0u8; 1024] as *const _ as *mut u8;
            let mut filled_size: u32 = MaybeUninit::uninit().assume_init();
            ret = synth_result_get_audio_data(handle, c_buf2, audio_length, &mut filled_size);
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_audio_data) error",
            )?;

            let converted_size = usize::try_from(filled_size)?;
            let slice_buffer = std::slice::from_raw_parts_mut(c_buf2, converted_size);

            let mut properties_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();
            ret = synth_result_get_property_bag(handle, &mut properties_handle);
            convert_err(
                ret,
                "SpeechSynthesisResult::from_handle(synth_result_get_property_bag) error",
            )?;
            let properties = PropertyCollection::from_handle(properties_handle);

            Ok(SpeechSynthesisResult {
                handle: SmartHandle::create(
                    "SpeechSynthesisResult",
                    handle,
                    synthesizer_result_handle_release,
                ),
                result_id,
                reason: ResultReason::from_u32(reason),
                audio_data: slice_buffer.to_vec(),
                properties,
            })
        }
    }
}
