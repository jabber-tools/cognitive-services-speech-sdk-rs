use crate::error::{Error, ErrorRootCause, Result};
use crate::ffi::{
    audio_stream_format_create_from_waveformat_pcm, audio_stream_format_release,
    SPXAUDIOSTREAMFORMATHANDLE, SPX_NOERROR,
};
use crate::{SmartHandle, SPXHANDLE_EMPTY};
use log::*;

#[derive(Debug)]
pub struct AudioStreamFormat {
    handle: SmartHandle<SPXAUDIOSTREAMFORMATHANDLE>,
}

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
