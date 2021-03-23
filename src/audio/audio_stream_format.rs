use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_format_create_from_waveformat_pcm, audio_stream_format_release, SmartHandle,
    SPXAUDIOSTREAMFORMATHANDLE,
};
use log::*;
use std::mem::MaybeUninit;

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
        unsafe {
            let mut handle: SPXAUDIOSTREAMFORMATHANDLE = MaybeUninit::uninit().assume_init();
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
