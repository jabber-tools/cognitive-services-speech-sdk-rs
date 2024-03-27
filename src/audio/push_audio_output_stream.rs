use crate::audio::AudioOutputStream;
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_create_push_audio_output_stream, audio_stream_release,
    push_audio_output_stream_set_callbacks, SmartHandle, SPXAUDIOSTREAMHANDLE,
};
use log::*;
use std::convert::TryFrom;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};

/// This trait that must be implemented by callback struct
/// passed into  push audio output stream during initialization.
/// Methods of this trait will be called by Speech Synthetizer.
/// When Synthetizer has syntehtized data it
/// will call **write** method. <br/>Structs implementing
/// **PushAudioOutputStreamCallbacks** must also implement **Send** trait.<br/>
/// To see how to use see example: **synthesizer/speak_ssml_async**.
pub trait PushAudioOutputStreamCallbacks: Send {
    /// called by synthetizer when new data are synthetized
    /// callback implementation should then write data to
    /// target sink (whatever it is) as appropriate
    /// It should return number of received
    /// bytes (in most cases it will return data_buffer.len())
    fn write(&mut self, data_buffer: &[u8]) -> u32;

    /// Closes underlying resources of struct implementing this trait.
    fn close(&mut self);
}

/// PushAudioOutputStream represents audio output stream with audio data pushed by Speech Synthetizer via *write* method.
/// Speech Synthetizer's caller is passivelly receiving already synthetized audio data via registered *write* callback.
pub struct PushAudioOutputStream {
    pub handle: SmartHandle<SPXAUDIOSTREAMHANDLE>,
    callbacks: Option<Box<dyn PushAudioOutputStreamCallbacks>>,
}

impl fmt::Debug for PushAudioOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PushAudioOutputStream")
            .field("handle", &self.handle)
            .finish()
    }
}

impl AudioOutputStream for PushAudioOutputStream {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE {
        self.handle.inner()
    }
}

impl PushAudioOutputStream {
    pub fn from_handle(handle: SPXAUDIOSTREAMHANDLE) -> Result<Self> {
        Ok(PushAudioOutputStream {
            handle: SmartHandle::create("PushAudioOutputStream", handle, audio_stream_release),
            callbacks: None,
        })
    }

    pub fn create_push_stream() -> Result<Self> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = audio_stream_create_push_audio_output_stream(handle.as_mut_ptr());
            convert_err(ret, "PushAudioOutputStream::create_push_stream error")?;
            PushAudioOutputStream::from_handle(handle.assume_init())
        }
    }

    /// Registers callbacks for speech synthetizer.
    pub fn set_callbacks(
        &mut self,
        callbacks: Box<dyn PushAudioOutputStreamCallbacks>,
    ) -> Result<()> {
        self.callbacks = Some(callbacks);
        unsafe {
            let ret = push_audio_output_stream_set_callbacks(
                self.handle.inner(),
                self as *const _ as *mut c_void,
                Some(Self::cb_write),
                Some(Self::cb_close),
            );
            convert_err(ret, "PushAudioOutputStream.set_callbacks error")?;
            Ok(())
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_write(pvContext: *mut c_void, buffer: *mut u8, size: u32) -> c_int {
        let pushstream = &mut *(pvContext as *mut PushAudioOutputStream);
        let callbacks: &mut Option<Box<(dyn PushAudioOutputStreamCallbacks)>> =
            &mut pushstream.callbacks;
        let callbacks = callbacks.as_mut().unwrap();

        let converted_size = usize::try_from(size);
        if let Err(conv_err) = converted_size {
            error!(
                "PushAudioOutputStream::cb_write errror when converting size to usize: {}",
                conv_err
            );
            0 // return 0 as we did not write anything
        } else {
            let slice_buffer = std::slice::from_raw_parts_mut(buffer, converted_size.unwrap());
            let bytes_written = callbacks.write(slice_buffer);
            bytes_written as i32
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_close(pvContext: *mut c_void) {
        let pushstream = &mut *(pvContext as *mut PushAudioOutputStream);
        let callbacks: &mut Option<Box<(dyn PushAudioOutputStreamCallbacks)>> =
            &mut pushstream.callbacks;
        let callbacks = callbacks.as_mut().unwrap();

        callbacks.close();
    }
}
