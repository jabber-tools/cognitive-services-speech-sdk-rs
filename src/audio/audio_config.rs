use crate::audio::AudioInputStream;
use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_config_create_audio_input_from_default_microphone,
    audio_config_create_audio_input_from_stream,
    audio_config_create_audio_input_from_wav_file_name, audio_config_get_property_bag,
    audio_config_release, property_bag_release, SmartHandle, SPXAUDIOCONFIGHANDLE,
    SPXAUDIOSTREAMHANDLE, SPXHANDLE, SPXPROPERTYBAGHANDLE,
};
use log::*;
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct AudioConfig {
    pub handle: SmartHandle<SPXAUDIOCONFIGHANDLE>,
    stream: Option<AudioInputStream>,
    pub properties: PropertyCollection,
}

impl AudioConfig {
    // passing also stream, need to solve this more elegantly, do not want to use Option
    fn from_handle(handle: SPXHANDLE, stream: Option<AudioInputStream>) -> Result<AudioConfig> {
        unsafe {
            let mut prop_bag_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();
            let ret = audio_config_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "AudioConfig::from_handle error")?;

            let property_bag = PropertyCollection {
                handle: SmartHandle::create(
                    "PropertyCollection",
                    prop_bag_handle,
                    property_bag_release,
                ),
            };

            let result = AudioConfig {
                handle: SmartHandle::create("AudioConfig", handle, audio_config_release),
                stream: stream,
                properties: property_bag,
            };
            Ok(result)
        }
    }

    pub fn from_stream_input(stream: AudioInputStream) -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOSTREAMHANDLE = MaybeUninit::uninit().assume_init();
            let ret =
                audio_config_create_audio_input_from_stream(&mut handle, stream.handle.inner());
            convert_err(ret, "AudioConfig::from_stream_input error")?;
            info!("from_stream_input ok");
            AudioConfig::from_handle(handle, Some(stream))
        }
    }

    pub fn from_wav_file_input<NM: AsRef<str>>(file_name: NM) -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_file_name = CString::new(file_name.as_ref())?;
            convert_err(
                audio_config_create_audio_input_from_wav_file_name(
                    &mut handle,
                    c_file_name.as_ptr(),
                ),
                "AudioConfig::from_wav_file_input error",
            )?;
            AudioConfig::from_handle(handle, None)
        }
    }

    pub fn from_default_microphone_input() -> Result<AudioConfig> {
        unsafe {
            let mut handle: SPXAUDIOCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            convert_err(
                audio_config_create_audio_input_from_default_microphone(&mut handle),
                "AudioConfig::from_default_microphone_input",
            )?;
            AudioConfig::from_handle(handle, None)
        }
    }
}
