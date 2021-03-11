use crate::audio::AudioConfig;
use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_release, recognizer_create_speech_recognizer_from_config,
    recognizer_get_property_bag, recognizer_handle_release, speech_config_from_subscription,
    speech_config_get_property_bag, speech_config_release, SmartHandle, SPXEVENTHANDLE, SPXHANDLE,
    SPXHANDLE_EMPTY, SPXRECOHANDLE, SPXSPEECHCONFIGHANDLE,
};
use std::ffi::CString;

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

unsafe extern "C" fn cb_session_started(
    hreco: SPXRECOHANDLE,
    hevent: SPXEVENTHANDLE,
    pvContext: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

unsafe extern "C" fn cb_session_stopped(
    hreco: SPXRECOHANDLE,
    hevent: SPXEVENTHANDLE,
    pvContext: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

unsafe extern "C" fn cb_speech_start_detected(
    hreco: SPXRECOHANDLE,
    hevent: SPXEVENTHANDLE,
    pvContext: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

unsafe extern "C" fn cb_speech_end_detected(
    hreco: SPXRECOHANDLE,
    hevent: SPXEVENTHANDLE,
    pvContext: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

unsafe extern "C" fn cb_canceled(
    hreco: SPXRECOHANDLE,
    hevent: SPXEVENTHANDLE,
    pvContext: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

unsafe extern "C" fn cb_recognizing(
    hreco: SPXRECOHANDLE,
    hevent: SPXEVENTHANDLE,
    pvContext: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

unsafe extern "C" fn cb_recognized(
    hreco: SPXRECOHANDLE,
    hevent: SPXEVENTHANDLE,
    pvContext: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}
