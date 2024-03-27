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
use std::os::raw::{c_char, c_int, c_void};

/// This trait that must be implemented by callback struct
/// passed into  pull audio input stream during initialization.
/// Methods of this trait will be called by Speech Recognizer.
/// When Speech recognizer is ready to process more data it
/// will call **read** method. <br/>Structs implementing
/// **PullAudioInputStreamCallbacks** must also implement **Send** trait.<br/>
/// To see how to use see example: **recognizer/continuous_recognition_pull_stream**.
pub trait PullAudioInputStreamCallbacks: Send {
    /// Reads (pulls) data from this audio stream instance
    /// data_buffer - data buffer to populate. It is prepolutated
    /// with zeros and its length represents maximum number of
    /// bytes to read and return.
    /// returns - number of actual bytes populated or 0 in case
    /// stream hits end and there is no more data. If there
    /// is no data available immediatelly read should block
    /// until next data becomes available.
    fn read(&mut self, data_buffer: &mut [u8]) -> u32;

    /// Closes underlying resources of struct implementing this trait.
    fn close(&mut self);

    /// retrieves specific property related to read audio frame
    /// recognizer will be trying to retrieve following properties:<br/>
    /// **ConversationTranscribingService_DataBufferTimeStamp (11001)**<br/>
    /// **ConversationTranscribingService_DataBufferUserId (11002)**<br/>
    /// For mor details see:
    /// [PropertyId Enum definition](https://docs.microsoft.com/en-us/dotnet/api/microsoft.cognitiveservices.speech.propertyid?view=azure-dotnet).
    fn get_property(&mut self, id: i32) -> Result<String>;
}

/// PullAudioInputStream represents audio input stream with audio data pulled (read) by Speech Recognizer when needed via *read* method.
/// Passing audio input is controlled by receiver.
pub struct PullAudioInputStream {
    pub handle: SmartHandle<SPXAUDIOSTREAMHANDLE>,
    callbacks: Option<Box<dyn PullAudioInputStreamCallbacks>>,
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
    pub fn from_format(format: &AudioStreamFormat) -> Result<Self> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = audio_stream_create_pull_audio_input_stream(
                handle.as_mut_ptr(),
                format.handle.inner(),
            );
            convert_err(ret, "PullAudioInputStream::from_format error")?;

            Ok(PullAudioInputStream {
                handle: SmartHandle::create(
                    "PullAudioInputStream",
                    handle.assume_init(),
                    audio_stream_release,
                ),
                callbacks: None,
            })
        }
    }

    pub fn from_default_format() -> Result<Self> {
        let default_format = AudioStreamFormat::get_default_input_format()?;
        PullAudioInputStream::from_format(&default_format)
    }

    /// Registers callbacks for speech recognizer. Callback for pull read
    /// and close are mandatory. Optionally (register_get_prop_cb = true)
    /// get property callback can be registered as well.
    pub fn set_callbacks(
        &mut self,
        callbacks: Box<dyn PullAudioInputStreamCallbacks>,
        register_get_prop_cb: bool,
    ) -> Result<()> {
        self.callbacks = Some(callbacks);
        unsafe {
            let mut ret = pull_audio_input_stream_set_callbacks(
                self.handle.inner(),
                self as *const _ as *mut c_void,
                Some(Self::cb_read),
                Some(Self::cb_close),
            );
            convert_err(
                ret,
                "PullAudioInputStream.set_callbacks (pull_audio_input_stream_set_callbacks) error",
            )?;

            if register_get_prop_cb {
                ret = pull_audio_input_stream_set_getproperty_callback(
                    self.handle.inner(),
                    self as *const _ as *mut c_void,
                    Some(Self::cb_get_property),
                );
                convert_err(
                    ret,
                    "PullAudioInputStream.set_callbacks (pull_audio_input_stream_set_getproperty_callback) error",
                )?;
            }

            Ok(())
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_read(pvContext: *mut c_void, buffer: *mut u8, size: u32) -> c_int {
        let pullstream = &mut *(pvContext as *mut PullAudioInputStream);
        let callbacks: &mut Option<Box<(dyn PullAudioInputStreamCallbacks)>> =
            &mut pullstream.callbacks;
        let callbacks = callbacks.as_mut().unwrap();

        let converted_size = usize::try_from(size);
        if let Err(conv_err) = converted_size {
            error!(
                "PullAudioInputStream::cb_read errror when converting size to usize: {}",
                conv_err
            );
            0 // return 0 as we did not read anything
        } else {
            let slice_buffer = std::slice::from_raw_parts_mut(buffer, converted_size.unwrap());
            let bytes_read = callbacks.read(slice_buffer);
            bytes_read as i32
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_close(pvContext: *mut c_void) {
        let pullstream = &mut *(pvContext as *mut PullAudioInputStream);
        let callbacks: &mut Option<Box<(dyn PullAudioInputStreamCallbacks)>> =
            &mut pullstream.callbacks;
        let callbacks = callbacks.as_mut().unwrap();

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
        let pullstream = &mut *(pvContext as *mut PullAudioInputStream);
        let callbacks: &mut Option<Box<(dyn PullAudioInputStreamCallbacks)>> =
            &mut pullstream.callbacks;
        let callbacks = callbacks.as_mut().unwrap();

        let converted_size = usize::try_from(size);
        if let Err(conv_err) = converted_size {
            error!(
                "PullAudioInputStream::cb_get_property errror when converting size to usize: {}",
                conv_err
            );
            return;
        }
        let converted_size = converted_size.unwrap();

        match callbacks.get_property(id) {
            Ok(prop_value) => match CString::new(prop_value) {
                Ok(c_prop_value) => {
                    let c_prop_value_bytes_count = c_prop_value.as_bytes().len();
                    let bytes_count_to_copy = if c_prop_value_bytes_count < converted_size {
                        c_prop_value_bytes_count
                    } else {
                        converted_size
                    };
                    std::ptr::copy_nonoverlapping(
                        c_prop_value.as_ptr(),
                        value as *mut c_char,
                        bytes_count_to_copy,
                    );
                }
                Err(cstr_err) => {
                    error!(
                        "PullAudioInputStream.cb_get_property error(CString::new): {:?}",
                        cstr_err
                    );
                }
            },
            Err(get_prop_err) => {
                error!(
                    "PullAudioInputStream.cb_get_property error(callbacks.get_property): {:?}",
                    get_prop_err
                );
            }
        }
    }
}
