use crate::audio::AudioStreamFormat;
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_create_push_audio_input_stream, audio_stream_release, SmartHandle,
    SPXAUDIOSTREAMHANDLE, SPXHANDLE_EMPTY,
};
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
            let ret =
                audio_stream_create_push_audio_input_stream(&mut handle, format.handle.inner());
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
