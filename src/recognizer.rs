use crate::config::{AudioConfig, SpeechConfig};
use crate::error::{convert_err, Error, ErrorRootCause, Result};
use crate::ffi::{
    recognizer_create_speech_recognizer_from_config, recognizer_handle_release, SPXRECOHANDLE,
    SPX_NOERROR,
};
use crate::{SmartHandle, SPXHANDLE_EMPTY};

#[derive(Debug)]
pub struct SpeechRecognizer {
    handle: SmartHandle<SPXRECOHANDLE>,
    speech_config: SpeechConfig,
    audio_config: AudioConfig,
}

impl SpeechRecognizer {
    pub fn from_config(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            convert_err(
                recognizer_create_speech_recognizer_from_config(
                    &mut handle,
                    speech_config.handle.get(),
                    audio_config.handle.get(),
                ),
                "SpeechRecognizer.from_config error",
            )?;
        }

        Ok(SpeechRecognizer {
            handle: SmartHandle::create("SpeechRecognizer", handle, recognizer_handle_release),
            speech_config,
            audio_config,
        })
    }
}
