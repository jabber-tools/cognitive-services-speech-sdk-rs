use super::stream_format::AudioStreamFormat;
use crate::error::{Error, ErrorRootCause, Result};
use crate::ffi::{
    audio_stream_create_push_audio_input_stream, audio_stream_release, SPXAUDIOSTREAMHANDLE,
    SPX_NOERROR,
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
