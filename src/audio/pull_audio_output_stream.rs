use crate::audio::AudioOutputStream;
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_create_pull_audio_output_stream, audio_stream_release,
    pull_audio_output_stream_read, SmartHandle, SPXAUDIOSTREAMHANDLE,
};
use std::convert::TryFrom;
use std::mem::MaybeUninit;

/// PullAudioOutputStream represents audio output stream with audio data pulled (read) by service calling Speech Synthetizer via *read* method.
/// Speech Synthetizer's caller is activelly pulling (reading) already synthetized audio data.
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
            let mut handle = MaybeUninit::uninit();
            let ret = audio_stream_create_pull_audio_output_stream(handle.as_mut_ptr());
            convert_err(ret, "PullAudioOutputStream::create_pull_stream error")?;
            PullAudioOutputStream::from_handle(handle.assume_init())
        }
    }

    /// Read reads audio from the stream.
    /// The maximal number of bytes to be read is determined from the size parameter.
    /// If there is no data immediately available, read() blocks until the next data becomes available.
    pub fn read(&self, size: u32) -> Result<Vec<u8>> {
        unsafe {
            let mut buf_vec = vec![0u8; size as usize];
            let c_buf: *mut u8 = &mut buf_vec[..] as *const _ as *mut u8;

            let mut filled_size = MaybeUninit::uninit();
            let ret = pull_audio_output_stream_read(
                self.handle.inner(),
                c_buf,
                size,
                filled_size.as_mut_ptr(),
            );
            convert_err(ret, "PullAudioOutputStream.read error")?;

            let converted_size = usize::try_from(filled_size.assume_init())?;
            let slice_buffer = std::slice::from_raw_parts_mut(c_buf, converted_size);
            Ok(slice_buffer.to_vec())
        }
    }
}
