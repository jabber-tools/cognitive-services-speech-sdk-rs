use crate::audio::AudioInputStream;
use crate::error::{convert_err, Error, ErrorRootCause, Result};
use crate::ffi::{
    audio_config_create_audio_input_from_stream,
    audio_config_create_audio_input_from_wav_file_name, audio_config_release,
    speech_config_from_subscription, speech_config_get_property_bag, speech_config_release,
    SmartHandle, SPXAUDIOCONFIGHANDLE, SPXHANDLE, SPXHANDLE_EMPTY, SPXSPEECHCONFIGHANDLE,
    SPX_NOERROR,
};
use crate::property::PropertyBag;
use log::*;
use std::ffi::CString;

#[derive(Debug)]
pub struct AudioConfig {
    pub handle: SmartHandle<SPXAUDIOCONFIGHANDLE>,
    stream: Option<AudioInputStream>,
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
                    stream: Some(stream),
                };
                Ok(result)
            }
        }
    }

    pub fn from_wav_file_input<NM: AsRef<str>>(file_name: NM) -> Result<AudioConfig> {
        let mut handle = SPXHANDLE_EMPTY;
        let c_file_name = CString::new(file_name.as_ref())?;
        unsafe {
            convert_err(
                audio_config_create_audio_input_from_wav_file_name(
                    &mut handle,
                    c_file_name.as_ptr(),
                ),
                "AudioConfig.from_wav_file_input error",
            )?;
        }
        let result = AudioConfig {
            handle: SmartHandle::create("AudioConfig", handle, audio_config_release),
            stream: None,
        };
        Ok(result)
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
