use crate::audio::input_stream::AudioInputStream;
use crate::error::{convert_err, Error, ErrorRootCause, Result};
use crate::ffi::{
    audio_config_create_audio_input_from_stream, audio_config_release,
    speech_config_from_subscription, speech_config_get_property_bag, speech_config_release,
    SPXAUDIOCONFIGHANDLE, SPXHANDLE, SPXSPEECHCONFIGHANDLE, SPX_NOERROR,
};
use crate::property::PropertyBag;
use crate::{SmartHandle, SPXHANDLE_EMPTY};
use log::*;
use std::ffi::CString;

#[derive(Debug)]
pub struct AudioConfig {
    pub handle: SmartHandle<SPXAUDIOCONFIGHANDLE>,
    stream: AudioInputStream,
}

#[derive(Debug)]
pub struct SpeechConfig {
    pub handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    properties: PropertyBag,
}

impl AudioConfig {
    pub fn from_stream_input(stream: AudioInputStream) -> Result<AudioConfig> {
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = audio_config_create_audio_input_from_stream(&mut handle, stream.handle.get());
            if ret != SPX_NOERROR as usize {
                error!("from_stream_input error {}", ret);
                Err(Error::new(
                    "from_stream_input error".into(),
                    ErrorRootCause::ApiError(ret),
                ))
            } else {
                info!("from_stream_input ok");
                let result = AudioConfig {
                    handle: SmartHandle::create("AudioConfig", handle, audio_config_release),
                    stream,
                };
                Ok(result)
            }
        }
    }
}

impl SpeechConfig {
    pub fn from_subscription<S1, S2>(subscription: S1, region: S2) -> Result<SpeechConfig>
    where
        S1: Into<Vec<u8>>,
        S2: Into<Vec<u8>>,
    {
        let c_sub = CString::new(subscription)?;
        let c_region = CString::new(region)?;
        Self::create(|handle| unsafe {
            convert_err(
                speech_config_from_subscription(handle, c_sub.as_ptr(), c_region.as_ptr()),
                "SpeechConfig.from_subscription error",
            )
        })
    }

    #[inline(always)]
    fn create(f: impl FnOnce(&mut SPXHANDLE) -> Result<()>) -> Result<SpeechConfig> {
        let mut handle = SPXHANDLE_EMPTY;
        f(&mut handle)?;
        let result = SpeechConfig {
            handle: SmartHandle::create("SpeechConfig", handle, speech_config_release),
            properties: PropertyBag::create(handle, speech_config_get_property_bag)?,
        };
        Ok(result)
    }
}
