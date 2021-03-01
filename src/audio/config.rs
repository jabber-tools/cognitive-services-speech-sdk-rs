use super::input_stream::AudioInputStream;
use crate::error::{Error, ErrorRootCause, Result};
use crate::ffi::{
    audio_config_create_audio_input_from_stream, audio_config_release, SPXAUDIOCONFIGHANDLE,
    SPX_NOERROR,
};
use crate::{SmartHandle, SPXHANDLE_EMPTY};
use log::*;

#[derive(Debug)]
pub struct AudioConfig {
    handle: SmartHandle<SPXAUDIOCONFIGHANDLE>,
    stream: AudioInputStream,
}

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