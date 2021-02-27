use std::ffi::CString;

use crate::convert_err;
use crate::SmartHandle;
use crate::speech_api::*;
use crate::SpxError;
use crate::SPXHANDLE_INVALID;

pub use self::stream::AudioInputStream;
pub use self::stream::AudioStreamSink;
pub use self::stream::PullAudioInputStreamCallback;
pub use self::stream_format::AudioStreamFormat;

mod stream;
mod stream_format;

pub struct AudioConfig {
    handle: SmartHandle<SPXAUDIOCONFIGHANDLE>,
    #[allow(unused)]
    stream: Option<Box<dyn AudioInputStream>>,
}

impl AudioConfig {
    pub fn from_stream_input(stream: Box<dyn AudioInputStream>) -> Result<AudioConfig, SpxError> {
        let mut handle = SPXHANDLE_INVALID;
        unsafe {
            convert_err(audio_config_create_audio_input_from_stream(
                &mut handle,
                stream.get_handle(),
            ))?;
        }
        let result = AudioConfig {
            handle: SmartHandle::create("AudioConfig", handle, audio_config_release),
            stream: Some(stream),
        };
        Ok(result)
    }

    pub fn from_wav_file_input<NM: AsRef<str>>(file_name: NM) -> Result<AudioConfig, SpxError> {
        let mut handle = SPXHANDLE_INVALID;
        let c_file_name = CString::new(file_name.as_ref())?;
        unsafe {
            convert_err(audio_config_create_audio_input_from_wav_file_name(
                &mut handle,
                c_file_name.as_ptr(),
            ))?;
        }
        let result = AudioConfig {
            handle: SmartHandle::create("AudioConfig", handle, audio_config_release),
            stream: None,
        };
        Ok(result)
    }

    pub fn output_from_default_speaker() -> Result<AudioConfig, SpxError> {
        let mut handle = SPXHANDLE_INVALID;
        unsafe {
            convert_err(audio_config_create_audio_output_from_default_speaker(&mut handle))?;
        }
        Ok(AudioConfig {
            handle: SmartHandle::create("AudioConfig", handle, audio_config_release),
            stream: None,
        })
    }

    #[inline]
    pub fn get_handle(&self) -> SPXAUDIOCONFIGHANDLE {
        self.handle.get()
    }
}
