use crate::audio::{AudioInputStream, AudioStreamFormat};
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_create_pull_audio_input_stream, audio_stream_release,
    pull_audio_input_stream_set_callbacks, pull_audio_input_stream_set_getproperty_callback,
    SmartHandle, SPXAUDIOSTREAMHANDLE,
};
use log::*;
use std::convert::TryFrom;
use std::ffi::CString;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};

/// Trait that must be implemented by pull audio input stream
/// Methods of this trait will be called by Speech Recognizer.
/// When Speech recognizer is ready to process more data it
/// will call read method. Structs implementing
/// PullAudioInputStreamCallbacks must also implement Send trait
pub trait PullAudioInputStreamCallbacks: Send {
    /// Reads (pulls) data from this audio stream instance
    /// data_buffer - data buffer to populate
    /// size - maximum number of bytes to read and return
    /// returns - number of actual bytes populated or 0 in case
    /// stream hits end and there is no more data. If there
    /// is no data available immediatelly read should block
    /// until next data becomes available.
    fn read(&mut self, data_buffer: &mut [u8], size: u32) -> u32;

    /// Closes this audio stream instance
    fn close(&mut self);

    // retrieves specific property from this audio stream instances
    // property id is provided by underlying C api as u32
    fn get_property(&mut self, id: u32) -> Result<String>;
}

pub struct PullAudioInputStream {
    pub handle: SmartHandle<SPXAUDIOSTREAMHANDLE>,
    callbacks: Box<dyn PullAudioInputStreamCallbacks>,
}

impl fmt::Debug for PullAudioInputStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PullAudioInputStream")
            .field("handle", &self.handle)
            .finish()
    }
}

impl AudioInputStream for PullAudioInputStream {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE {
        self.handle.inner()
    }
}

impl PullAudioInputStream {
    pub fn from_format(
        callbacks: Box<dyn PullAudioInputStreamCallbacks>,
        format: &AudioStreamFormat,
    ) -> Result<Self> {
        unsafe {
            let mut handle: SPXAUDIOSTREAMHANDLE = MaybeUninit::uninit().assume_init();
            let mut ret =
                audio_stream_create_pull_audio_input_stream(&mut handle, format.handle.inner());
            convert_err(ret, "PullAudioInputStream::from_format(audio_stream_create_pull_audio_input_stream) error")?;

            let mut pull_stream = PullAudioInputStream {
                handle: SmartHandle::create("PullAudioInputStream", handle, audio_stream_release),
                callbacks,
            };

            ret = pull_audio_input_stream_set_callbacks(
                handle,
                // as a context provide pointer to callbacks
                &mut pull_stream.callbacks as *const _ as *mut c_void,
                Some(Self::cb_read),
                Some(Self::cb_close),
            );
            convert_err(
                ret,
                "PullAudioInputStream::from_format(pull_audio_input_stream_set_callbacks) error",
            )?;

            ret = pull_audio_input_stream_set_getproperty_callback(
                handle,
                &mut pull_stream.callbacks as *const _ as *mut c_void,
                Some(Self::cb_get_property),
            );
            convert_err(ret, "PullAudioInputStream::from_format(pull_audio_input_stream_set_getproperty_callback) error")?;

            Ok(pull_stream)
        }
    }

    pub fn from_default_format(callbacks: Box<dyn PullAudioInputStreamCallbacks>) -> Result<Self> {
        let default_format = AudioStreamFormat::get_default_input_format()?;
        PullAudioInputStream::from_format(callbacks, &default_format)
    }

    // https://doc.rust-lang.org/std/primitive.pointer.html
    // https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_read(pvContext: *mut c_void, buffer: *mut u8, size: u32) -> c_int {
        let callbacks = &mut *(pvContext as *mut Box<dyn PullAudioInputStreamCallbacks>);

        let converted_size = usize::try_from(size);
        if let Err(conv_err) = converted_size {
            error!(
                "PullAudioInputStream::cb_read errror when converting size to usize: {}",
                conv_err
            );
            0 // return 0 as we did not read anything
        } else {
            let slice_buffer = std::slice::from_raw_parts_mut(buffer, converted_size.unwrap());
            let bytes_read = callbacks.read(slice_buffer, size);
            bytes_read as i32
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_close(pvContext: *mut c_void) {
        let callbacks = &mut *(pvContext as *mut Box<dyn PullAudioInputStreamCallbacks>);
        callbacks.close();
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_get_property(
        pvContext: *mut c_void,
        id: c_int,
        value: *mut u8,
        size: u32,
    ) {
        let callbacks = &mut *(pvContext as *mut Box<dyn PullAudioInputStreamCallbacks>);
        if let Ok(prop_value) = callbacks.get_property(size) {
            if let Ok(c_prop_value) = CString::new(prop_value) {
                std::ptr::copy_nonoverlapping(
                    c_prop_value.as_ptr(),
                    value as *mut i8,
                    std::mem::size_of::<u32>(),
                );
            } else {
                // TBD: log error (use match to get error details)
            }
        } else {
            // TBD: log error (use match to get error details)
        }
    }
}
