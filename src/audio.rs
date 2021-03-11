use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_config_create_audio_input_from_stream,
    audio_config_create_audio_input_from_wav_file_name, audio_config_release,
    audio_stream_create_push_audio_input_stream, audio_stream_format_create_from_waveformat_pcm,
    audio_stream_format_release, audio_stream_release, SmartHandle, SPXAUDIOCONFIGHANDLE,
    SPXAUDIOSTREAMFORMATHANDLE, SPXAUDIOSTREAMHANDLE, SPXHANDLE_EMPTY,
};
use log::*;
use std::ffi::CString;

#[derive(Debug)]
pub struct AudioInputStream {
    pub handle: SmartHandle<SPXAUDIOSTREAMHANDLE>,
    // we need to keep format handle in this struct
    // otherwise it would be dropped after call of
    // create_push_stream_from_format!
    format: AudioStreamFormat,
}

impl AudioInputStream {
    pub fn create_push_stream_from_format(format: AudioStreamFormat) -> Result<AudioInputStream> {
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = audio_stream_create_push_audio_input_stream(&mut handle, format.handle.get());
            convert_err(ret, "create_push_stream_from_format error")?;
            info!("create_push_stream_from_format ok");
            let result = AudioInputStream {
                handle: SmartHandle::create("AudioInputStream", handle, audio_stream_release),
                format,
            };
            Ok(result)
        }
    }
}

#[derive(Debug)]
pub struct AudioStreamFormat {
    pub handle: SmartHandle<SPXAUDIOSTREAMFORMATHANDLE>,
}

impl AudioStreamFormat {
    pub fn get_wave_format_pcm(
        samples_per_second: u32,
        bits_per_sample: Option<u8>,
        channels: Option<u8>,
    ) -> Result<AudioStreamFormat> {
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = audio_stream_format_create_from_waveformat_pcm(
                &mut handle,
                samples_per_second,
                bits_per_sample.unwrap_or(16),
                channels.unwrap_or(1),
            );
            convert_err(ret, "get_wave_format_pcm error")?;
            info!("get_wave_format_pcm ok");
            let result = AudioStreamFormat {
                handle: SmartHandle::create(
                    "AudioStreamFormat",
                    handle,
                    audio_stream_format_release,
                ),
            };
            Ok(result)
        }
    }
}

#[derive(Debug)]
pub struct AudioConfig {
    pub handle: SmartHandle<SPXAUDIOCONFIGHANDLE>,
    stream: Option<AudioInputStream>,
}

impl AudioConfig {
    pub fn from_stream_input(stream: AudioInputStream) -> Result<AudioConfig> {
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = audio_config_create_audio_input_from_stream(&mut handle, stream.handle.get());
            convert_err(ret, "from_stream_input error")?;
            info!("from_stream_input ok");
            let result = AudioConfig {
                handle: SmartHandle::create("AudioConfig", handle, audio_config_release),
                stream: Some(stream),
            };
            Ok(result)
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
