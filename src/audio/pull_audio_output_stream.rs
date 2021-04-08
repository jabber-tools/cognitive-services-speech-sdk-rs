use crate::audio::AudioOutputStream;
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_create_pull_audio_output_stream, audio_stream_release, SmartHandle,
    SPXAUDIOSTREAMHANDLE,
};
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct PullAudioOutputStream {
    pub handle: SmartHandle<SPXAUDIOSTREAMHANDLE>,
}

impl AudioOutputStream for PullAudioOutputStream {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE {
        self.handle.inner()
    }
}

impl PullAudioOutputStream {
    pub fn from_handle(handle: SPXAUDIOSTREAMHANDLE) -> Result<Self> {
        Ok(PullAudioOutputStream {
            handle: SmartHandle::create("PullAudioOutputStream", handle, audio_stream_release),
        })
    }

    pub fn create_pull_stream() -> Result<Self> {
        unsafe {
            let mut handle: SPXAUDIOSTREAMHANDLE = MaybeUninit::uninit().assume_init();
            let ret = audio_stream_create_pull_audio_output_stream(&mut handle);
            convert_err(ret, "PullAudioOutputStream::create_pull_stream error")?;
            PullAudioOutputStream::from_handle(handle)
        }
    }

    /// Read reads audio from the stream.
    /// The maximal number of bytes to be read is determined from the size parameter.
    /// If there is no data immediately available, read() blocks until the next data becomes available.
    pub fn read(_size: u32, _buffer: &mut [u8]) -> Result<()> {
        unimplemented!();
        // crate::ffi::pull_audio_output_stream_read

        // no idea how this should work. Go impl does not expose
        // this function as C function to speech synthetizer
        // and it is not called in official C# examples.
    }
}
