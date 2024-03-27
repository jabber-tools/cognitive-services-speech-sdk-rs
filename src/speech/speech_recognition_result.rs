use crate::common::{PropertyCollection, ResultReason};
use crate::error::{convert_err, Result};
use crate::ffi::{
    recognizer_result_handle_release, result_get_duration, result_get_offset,
    result_get_property_bag, result_get_reason, result_get_result_id, result_get_text, SmartHandle,
    SPXRESULTHANDLE,
};
use std::ffi::CStr;
use std::fmt;
use std::mem::MaybeUninit;

/// Represents speech recognition result contained within callback event *SpeechRecognitionEvent*.
pub struct SpeechRecognitionResult {
    pub handle: SmartHandle<SPXRESULTHANDLE>,
    pub result_id: String,
    pub reason: ResultReason,
    pub text: String,
    pub duration: String, //TBD: change to duration
    pub offset: String,   // TBD: change to duration
    pub properties: PropertyCollection,
}

impl fmt::Debug for SpeechRecognitionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpeechRecognitionResult")
            .field("result_id", &self.result_id)
            .field("reason", &self.reason)
            .field("text", &self.text)
            .field("duration", &self.duration)
            .field("offset", &self.offset)
            .finish()
    }
}

impl SpeechRecognitionResult {
    pub fn from_handle(handle: SPXRESULTHANDLE) -> Result<SpeechRecognitionResult> {
        unsafe {
            let mut c_buf = [0; 1024];
            let mut ret = result_get_result_id(handle, c_buf.as_mut_ptr(), c_buf.len() as u32);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_result_id) error",
            )?;
            let result_id = CStr::from_ptr(c_buf.as_ptr()).to_str()?.to_owned();

            let mut reason = MaybeUninit::uninit();
            ret = result_get_reason(handle, reason.as_mut_ptr());
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_reason) error",
            )?;

            let mut c_buf2 = [0; 1024];
            ret = result_get_text(handle, c_buf2.as_mut_ptr(), c_buf2.len() as u32);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_text) error",
            )?;
            let result_text = CStr::from_ptr(c_buf2.as_ptr()).to_str()?.to_owned();

            let mut duration = MaybeUninit::uninit();
            ret = result_get_duration(handle, duration.as_mut_ptr());
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_duration) error",
            )?;

            let mut offset = MaybeUninit::uninit();
            ret = result_get_offset(handle, offset.as_mut_ptr());
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_offset) error",
            )?;

            let mut properties_handle = MaybeUninit::uninit();
            ret = result_get_property_bag(handle, properties_handle.as_mut_ptr());
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_property_bag) error",
            )?;
            let properties = PropertyCollection::from_handle(properties_handle.assume_init());

            #[cfg(target_os = "windows")]
            let reason = ResultReason::from_i32(reason.assume_init());
            #[cfg(not(target_os = "windows"))]
            let reason = ResultReason::from_u32(reason.assume_init());

            Ok(SpeechRecognitionResult {
                handle: SmartHandle::create(
                    "SpeechRecognitionResult",
                    handle,
                    recognizer_result_handle_release,
                ),
                result_id,
                reason,
                text: result_text,
                duration: (duration.assume_init()).to_string(),
                offset: (offset.assume_init()).to_string(),
                properties,
            })
        }
    }
}
