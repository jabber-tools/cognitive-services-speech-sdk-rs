use crate::audio::AudioStreamFormat;
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_create_push_audio_input_stream, audio_stream_release,
    push_audio_input_stream_write, SmartHandle, SPXAUDIOSTREAMHANDLE,
};
use log::*;
use std::mem::MaybeUninit;

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
        unsafe {
            let mut handle: SPXAUDIOSTREAMHANDLE = MaybeUninit::uninit().assume_init();
            let ret =
                audio_stream_create_push_audio_input_stream(&mut handle, format.handle.inner());
            convert_err(
                ret,
                "AudioInputStream::create_push_stream_from_format error",
            )?;
            info!("create_push_stream_from_format ok");
            let result = AudioInputStream {
                handle: SmartHandle::create("AudioInputStream", handle, audio_stream_release),
                format,
            };
            Ok(result)
        }
    }

    // impl to use static dispatch, see https://joshleeb.com/posts/rust-traits-and-trait-objects/
    pub fn write(&mut self, buffer: impl AsRef<[u8]>) -> Result<()> {
        unsafe {
            let buf = buffer.as_ref();
            let ptr = buf.as_ptr() as *mut u8;
            let ret = push_audio_input_stream_write(self.handle.inner(), ptr, buf.len() as u32);
            convert_err(ret, "AudioInputStream.write error")?;
            Ok(())
        }
    }
}
