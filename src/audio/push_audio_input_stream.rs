use crate::audio::AudioStreamFormat;
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_create_push_audio_input_stream, audio_stream_release,
    push_audio_input_stream_close, push_audio_input_stream_write, SmartHandle,
    SPXAUDIOSTREAMHANDLE,
};
use log::*;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct PushAudioInputStream {
    pub handle: SmartHandle<SPXAUDIOSTREAMHANDLE>,
}

impl PushAudioInputStream {
    pub fn create_push_stream_from_format(
        format: AudioStreamFormat,
    ) -> Result<PushAudioInputStream> {
        unsafe {
            let mut handle: SPXAUDIOSTREAMHANDLE = MaybeUninit::uninit().assume_init();
            let ret =
                audio_stream_create_push_audio_input_stream(&mut handle, format.handle.inner());
            convert_err(
                ret,
                "PushAudioInputStream::create_push_stream_from_format error",
            )?;
            info!("create_push_stream_from_format ok");
            Ok(PushAudioInputStream {
                handle: SmartHandle::create("PushAudioInputStream", handle, audio_stream_release),
            })
        }
    }

    pub fn create_push_stream() -> Result<PushAudioInputStream> {
        unsafe {
            let mut handle: SPXAUDIOSTREAMHANDLE = MaybeUninit::uninit().assume_init();

            let default_format = AudioStreamFormat::get_default_input_format()?;

            let ret = audio_stream_create_push_audio_input_stream(
                &mut handle,
                default_format.handle.inner(),
            );
            convert_err(
                ret,
                "PushAudioInputStream::create_push_stream_from_format error",
            )?;
            info!("create_push_stream_from_format ok");
            Ok(PushAudioInputStream {
                handle: SmartHandle::create("PushAudioInputStream", handle, audio_stream_release),
            })
        }
    }

    // impl to use static dispatch, see https://joshleeb.com/posts/rust-traits-and-trait-objects/
    pub fn write(&mut self, buffer: impl AsRef<[u8]>) -> Result<()> {
        unsafe {
            let buf = buffer.as_ref();
            let ptr = buf.as_ptr() as *mut u8;
            let ret = push_audio_input_stream_write(self.handle.inner(), ptr, buf.len() as u32);
            convert_err(ret, "PushAudioInputStream.write error")?;
            Ok(())
        }
    }

    pub fn close_stream(&self) -> Result<()> {
        unsafe {
            let ret = push_audio_input_stream_close(self.handle.inner());
            convert_err(ret, "PushAudioInputStream.close_stream error")?;
            Ok(())
        }
    }
}
