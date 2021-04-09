use crate::common::{PropertyCollection, ResultReason};
use crate::error::{convert_err, Result};
use crate::ffi::{
    recognizer_result_handle_release, result_get_duration, result_get_offset,
    result_get_property_bag, result_get_reason, result_get_result_id, result_get_text, SmartHandle,
    SPXPROPERTYBAGHANDLE, SPXRESULTHANDLE,
};
use std::ffi::CStr;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_uint};

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
            let c_buf: *mut c_char = &mut [0u8; 1024] as *const _ as *mut c_char;
            let mut ret = result_get_result_id(handle, c_buf, 1024);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_result_id) error",
            )?;
            let result_id = CStr::from_ptr(c_buf).to_str()?.to_owned();

            let mut reason: c_uint = MaybeUninit::uninit().assume_init();
            ret = result_get_reason(handle, &mut reason);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_reason) error",
            )?;

            let c_buf2: *mut c_char = &mut [0u8; 1024] as *const _ as *mut c_char;
            ret = result_get_text(handle, c_buf2, 1024);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_text) error",
            )?;
            let result_text = CStr::from_ptr(c_buf2).to_str()?.to_owned();

            let mut duration: u64 = MaybeUninit::uninit().assume_init();
            ret = result_get_duration(handle, &mut duration);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_duration) error",
            )?;

            let mut offset: u64 = MaybeUninit::uninit().assume_init();
            ret = result_get_offset(handle, &mut offset);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_offset) error",
            )?;

            let mut properties_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();
            ret = result_get_property_bag(handle, &mut properties_handle);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_property_bag) error",
            )?;
            let properties = PropertyCollection::from_handle(properties_handle);

            Ok(SpeechRecognitionResult {
                handle: SmartHandle::create(
                    "SpeechRecognitionResult",
                    handle,
                    recognizer_result_handle_release,
                ),
                result_id,
                reason: ResultReason::from_u32(reason),
                text: result_text,
                duration: (duration).to_string(),
                offset: (offset).to_string(),
                properties,
            })
        }
    }
}
