use crate::error::{Error, ErrorRootCause, Result};
use crate::ffi::{
    audio_stream_create_push_audio_input_stream, audio_stream_format_create_from_waveformat_pcm,
    audio_stream_format_release, audio_stream_release, SPXAUDIOSTREAMFORMATHANDLE,
    SPXAUDIOSTREAMHANDLE, SPX_NOERROR,
};
use crate::{SmartHandle, SPXHANDLE_EMPTY};
use log::*;

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
            if ret != SPX_NOERROR as usize {
                error!("create_push_stream_from_format error {}", ret);
                Err(Error::new(
                    "create_push_stream_from_format error".into(),
                    ErrorRootCause::ApiError(ret),
                ))
            } else {
                info!("create_push_stream_from_format ok");
                let result = AudioInputStream {
                    handle: SmartHandle::create("AudioInputStream", handle, audio_stream_release),
                    format,
                };
                Ok(result)
            }
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
            if ret != SPX_NOERROR as usize {
                error!("get_wave_format_pcm error {}", ret);
                Err(Error::new(
                    "get_wave_format_pcm error".into(),
                    ErrorRootCause::ApiError(ret),
                ))
            } else {
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
}
