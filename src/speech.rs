use crate::audio::AudioConfig;
use crate::common::{
    CancellationErrorCode, CancellationReason, PropertyCollection, PropertyId, ResultReason,
};
use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_release, recognizer_create_speech_recognizer_from_config,
    recognizer_event_handle_release, recognizer_get_property_bag, recognizer_handle_release,
    recognizer_recognition_event_get_offset, recognizer_recognition_event_get_result,
    recognizer_result_handle_release, recognizer_session_event_get_session_id,
    result_get_canceled_error_code, result_get_duration, result_get_offset,
    result_get_property_bag, result_get_reason, result_get_reason_canceled, result_get_result_id,
    result_get_text, speech_config_from_subscription, speech_config_get_property_bag,
    speech_config_release, Result_CancellationErrorCode, Result_CancellationReason, SmartHandle,
    SPXEVENTHANDLE, SPXHANDLE, SPXHANDLE_EMPTY, SPXPROPERTYBAGHANDLE, SPXRECOHANDLE,
    SPXRESULTHANDLE, SPXSPEECHCONFIGHANDLE,
};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_uint, c_void};

#[derive(Debug)]
pub struct SpeechConfig {
    pub handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    properties: PropertyCollection,
}

impl SpeechConfig {
    fn from_handle(handle: SPXHANDLE) -> Result<SpeechConfig> {
        let mut prop_bag_handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = speech_config_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "SpeechConfig::from_handle error")?;
        }
        let mut property_bag = PropertyCollection {
            handle: SmartHandle::create(
                "PropertyCollection",
                prop_bag_handle,
                property_bag_release,
            ),
        };

        property_bag.set_property_by_string("SPEECHSDK-SPEECH-CONFIG-SYSTEM-LANGUAGE", "Rust")?;

        let result = SpeechConfig {
            handle: SmartHandle::create("SpeechConfig", handle, speech_config_release),
            properties: property_bag,
        };
        Ok(result)
    }

    pub fn from_subscription<S>(subscription: S, region: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_sub = CString::new(subscription)?;
        let c_region = CString::new(region)?;
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret =
                speech_config_from_subscription(&mut handle, c_sub.as_ptr(), c_region.as_ptr());
            convert_err(ret, "SpeechConfig::from_subscription error")?
        }
        SpeechConfig::from_handle(handle)
    }
}

#[derive(Debug)]
pub struct SpeechRecognizer {
    handle: SmartHandle<SPXRECOHANDLE>,
    properties: PropertyCollection,
    speech_config: SpeechConfig,
    audio_config: AudioConfig,
}

impl SpeechRecognizer {
    fn from_handle(
        handle: SPXHANDLE,
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        let mut prop_bag_handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = recognizer_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "SpeechRecognizer::from_handle error")?;
        }
        let property_bag = PropertyCollection {
            handle: SmartHandle::create(
                "PropertyCollection",
                prop_bag_handle,
                property_bag_release,
            ),
        };

        let result = SpeechRecognizer {
            handle: SmartHandle::create("SpeechRecognizer", handle, recognizer_handle_release),
            properties: property_bag,
            speech_config,
            audio_config,
        };
        Ok(result)
    }

    pub fn from_config(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            convert_err(
                recognizer_create_speech_recognizer_from_config(
                    &mut handle,
                    speech_config.handle.get(),
                    audio_config.handle.get(),
                ),
                "SpeechRecognizer.from_config error",
            )?;
        }
        SpeechRecognizer::from_handle(handle, speech_config, audio_config)
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_session_started(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_session_stopped(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_speech_start_detected(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _vpvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_speech_end_detected(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_canceled(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_recognizing(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_recognized(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[derive(Debug)]
pub struct SessionEvent {
    session_id: String,
    handle: SmartHandle<SPXEVENTHANDLE>,
}

impl SessionEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<SessionEvent> {
        //allocate zeroed buffer and convert to unsafe mutable ptr
        let buffer: *mut u8 = vec![0u8; 37].as_mut_ptr();
        unsafe {
            let ret = recognizer_session_event_get_session_id(handle, buffer as *mut c_char, 37);
            convert_err(ret, "SessionEvent::from_handle error")?;
            // TBD:not sure whether recognizer_session_event_get_session_id will allocate
            // 37 bytes exactly or it might allocate less? In that case we would have to
            // offset pointer byte by byte until we find terminating nul char(0) of C string
            // see also https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr
            let session_id_slice = std::slice::from_raw_parts(buffer, 37);
            let session_id = String::from_utf8(session_id_slice.to_vec())?;
            Ok(SessionEvent {
                session_id,
                handle: SmartHandle::create(
                    "SessionEvent",
                    handle,
                    recognizer_event_handle_release,
                ),
            })
        }
    }
}

#[derive(Debug)]
pub struct RecognitionEvent {
    base: SessionEvent,
    offset: u64,
}

impl RecognitionEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<RecognitionEvent> {
        let base = SessionEvent::from_handle(handle)?;
        unsafe {
            let offset: *mut u64 = MaybeUninit::uninit().assume_init();
            let ret = recognizer_recognition_event_get_offset(handle, offset);
            convert_err(ret, "RecognitionEvent::from_handle error")?;
            Ok(RecognitionEvent {
                base,
                offset: *offset,
            })
        }
    }
}

#[derive(Debug)]
pub struct SpeechRecognitionResult {
    handle: SmartHandle<SPXRESULTHANDLE>,
    result_id: String,
    reason: ResultReason,
    text: String,
    duration: String, //TBD: change to duration
    offset: String,   // TBD: change to duration
    properties: PropertyCollection,
}

impl SpeechRecognitionResult {
    pub fn from_handle(handle: SPXRESULTHANDLE) -> Result<SpeechRecognitionResult> {
        unsafe {
            let buffer: *mut u8 = vec![0u8; 1024].as_mut_ptr();
            let mut ret = result_get_result_id(handle, buffer as *mut c_char, 1024);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_result_id) error",
            )?;
            let result_id_slice = std::slice::from_raw_parts(buffer, 1024);
            let result_id = String::from_utf8(result_id_slice.to_vec())?;

            let reason: *mut c_uint = MaybeUninit::uninit().assume_init();
            ret = result_get_reason(handle, reason);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_reason) error",
            )?;

            let buffer2: *mut u8 = vec![0u8; 1024].as_mut_ptr();
            ret = result_get_text(handle, buffer2 as *mut c_char, 1024);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_text) error",
            )?;
            let result_text_slice = std::slice::from_raw_parts(buffer, 1024);
            let result_text = String::from_utf8(result_text_slice.to_vec())?;

            let duration: *mut u64 = MaybeUninit::uninit().assume_init();
            ret = result_get_duration(handle, duration);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_duration) error",
            )?;

            let offset: *mut u64 = MaybeUninit::uninit().assume_init();
            ret = result_get_offset(handle, offset);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_offset) error",
            )?;

            let properties_handle: *mut SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();
            ret = result_get_property_bag(handle, properties_handle);
            convert_err(
                ret,
                "SpeechRecognitionResult::from_handle(result_get_property_bag) error",
            )?;
            let properties = PropertyCollection {
                handle: SmartHandle::create(
                    "PropertyCollection",
                    *properties_handle,
                    property_bag_release,
                ),
            };

            Ok(SpeechRecognitionResult {
                handle: SmartHandle::create(
                    "SpeechRecognitionResult",
                    handle,
                    recognizer_result_handle_release,
                ),
                result_id,
                reason: ResultReason::from_u32(*reason),
                text: result_text,
                duration: (*duration).to_string(),
                offset: (*offset).to_string(),
                properties,
            })
        }
    }
}

#[derive(Debug)]
pub struct SpeechRecognitionEvent {
    base: RecognitionEvent,
    result: SpeechRecognitionResult,
}

impl SpeechRecognitionEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<SpeechRecognitionEvent> {
        let base = RecognitionEvent::from_handle(handle)?;

        unsafe {
            let result_handle: *mut SPXRESULTHANDLE = MaybeUninit::uninit().assume_init();
            let ret = recognizer_recognition_event_get_result(handle, result_handle);
            convert_err(ret, "SpeechRecognitionEvent::from_handle error")?;
            let result = SpeechRecognitionResult::from_handle(*result_handle)?;
            Ok(SpeechRecognitionEvent {
                base,
                result: result,
            })
        }
    }
}

#[derive(Debug)]
pub struct SpeechRecognitionCanceledEvent {
    base: SpeechRecognitionEvent,
    reason: CancellationReason,
    error_code: CancellationErrorCode,
    error_details: String,
}

impl SpeechRecognitionCanceledEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<SpeechRecognitionCanceledEvent> {
        let base = SpeechRecognitionEvent::from_handle(handle)?;
        unsafe {
            let reason_ptr: *mut Result_CancellationReason = MaybeUninit::uninit().assume_init();
            let ret = result_get_reason_canceled(base.result.handle.get(), reason_ptr);
            convert_err(
                ret,
                "SpeechRecognitionCanceledEvent::from_handle(result_get_reason_canceled) error",
            )?;

            let error_code_ptr: *mut Result_CancellationErrorCode =
                MaybeUninit::uninit().assume_init();
            let ret = result_get_canceled_error_code(base.result.handle.get(), error_code_ptr);
            convert_err(
                ret,
                "SpeechRecognitionCanceledEvent::from_handle(result_get_canceled_error_code) error",
            )?;

            let error_details = base.result.properties.get_property(
                PropertyId::SpeechServiceResponseJsonErrorDetails,
                "".to_string(),
            )?;
            Ok(SpeechRecognitionCanceledEvent {
                base,
                reason: CancellationReason::from_u32(*reason_ptr),
                error_code: CancellationErrorCode::from_u32(*error_code_ptr),
                error_details,
            })
        }
    }
}
