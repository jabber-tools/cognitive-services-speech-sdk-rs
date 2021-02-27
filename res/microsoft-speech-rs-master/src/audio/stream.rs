use std::ffi::c_void;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice;
use std::sync::Arc;
use std::sync::Weak;

use crate::audio::AudioStreamFormat;
use crate::convert_err;
use crate::SmartHandle;
use crate::speech_api::*;
use crate::SpxError;
use crate::SPXHANDLE_INVALID;

pub trait AudioStreamSink: Send {
    fn write(&mut self, buf: impl AsRef<[u8]>) -> Result<(), SpxError>;

    fn close(&mut self) -> Result<(), SpxError>;
}

pub trait AudioInputStream: Send {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE;
}

impl AudioInputStream {
    pub fn create_push_stream(format: Option<AudioStreamFormat>) -> Result<(Box<dyn AudioInputStream>, impl AudioStreamSink), SpxError> {
        let stream = PushAudioInputStream::create(format)?;
        let sink = PushAudioInputStreamSink {
            handle: Arc::downgrade(&stream.handle),
        };
        Ok((Box::new(stream), sink))
    }

    pub fn create_pull_stream<CB>(callback: CB, format: Option<AudioStreamFormat>) -> Result<Box<dyn AudioInputStream>, SpxError>
        where CB: PullAudioInputStreamCallback + 'static {
        Ok(Box::new(PullAudioInputStream::create(format, callback)?))
    }
}

#[derive(Debug)]
struct BaseAudioInputStream {
    handle: Arc<SmartHandle<SPXAUDIOSTREAMHANDLE>>,
    format: AudioStreamFormat,
}

impl BaseAudioInputStream {
    fn create(name: &'static str,
              format: Option<AudioStreamFormat>,
              create_fn: unsafe extern "C" fn(*mut SPXAUDIOSTREAMHANDLE, SPXAUDIOSTREAMFORMATHANDLE) -> SPXHR) -> Result<BaseAudioInputStream, SpxError> {
        let format = format
            .map(|x| Ok(x))
            .unwrap_or_else(|| { AudioStreamFormat::get_default_input_format() })?;
        let mut handle = SPXHANDLE_INVALID;
        unsafe {
            convert_err(create_fn(&mut handle, format.get_handle()))?;
        }
        let result = BaseAudioInputStream {
            handle: Arc::new(SmartHandle::create(name, handle, audio_stream_release)),
            format,
        };
        Ok(result)
    }
}

// PushAudioInputStream

#[derive(Debug)]
struct PushAudioInputStream {
    base: BaseAudioInputStream,
}

impl PushAudioInputStream {
    fn create(format: Option<AudioStreamFormat>) -> Result<PushAudioInputStream, SpxError> {
        Ok(PushAudioInputStream {
            base: BaseAudioInputStream::create("PushAudioInputStream", format, audio_stream_create_push_audio_input_stream)?,
        })
    }
}

impl AudioInputStream for PushAudioInputStream {
    #[inline]
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE {
        self.base.handle.get()
    }
}

impl Deref for PushAudioInputStream {
    type Target = BaseAudioInputStream;

    fn deref(&self) -> &BaseAudioInputStream {
        &self.base
    }
}

impl DerefMut for PushAudioInputStream {
    fn deref_mut(&mut self) -> &mut BaseAudioInputStream {
        &mut self.base
    }
}

impl Drop for PushAudioInputStream {
    fn drop(&mut self) {
        unsafe {
            if audio_stream_is_handle_valid(self.handle.get()) {
                push_audio_input_stream_close(self.handle.get());
            }
        }
    }
}

unsafe impl Send for PushAudioInputStream {}

pub struct PushAudioInputStreamSink {
    handle: Weak<SmartHandle<SPXAUDIOSTREAMHANDLE>>,
}

impl AudioStreamSink for PushAudioInputStreamSink {
    fn write(&mut self, buf: impl AsRef<[u8]>) -> Result<(), SpxError> {
        match self.handle.upgrade() {
            None => Err(SpxError::StreamDropped),
            Some(handle) => unsafe {
                let buf = buf.as_ref();
                let ptr = buf.as_ptr() as *mut u8;
                convert_err(push_audio_input_stream_write(handle.get(), ptr, buf.len() as u32))
            }
        }
    }

    fn close(&mut self) -> Result<(), SpxError> {
        match self.handle.upgrade() {
            None => Err(SpxError::StreamDropped),
            Some(handle) => unsafe {
                convert_err(push_audio_input_stream_close(handle.get()))
            }
        }
    }
}

impl Drop for PushAudioInputStreamSink {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        if let Some(handle) = self.handle.upgrade() {
            unsafe {
                if audio_stream_is_handle_valid(handle.get()) {
                    push_audio_input_stream_close(handle.get());
                }
            }
        }
    }
}

unsafe impl Send for PushAudioInputStreamSink {}

// PullAudioInputStream

pub trait PullAudioInputStreamCallback: Send {
    fn read(&mut self, data_buffer: &mut [u8]) -> usize;
    fn close(&mut self);
}

struct PullAudioInputStream<CB> {
    base: BaseAudioInputStream,
    callback: Box<CB>,
}

impl<CB> PullAudioInputStream<CB> where CB: PullAudioInputStreamCallback + 'static {
    fn create(format: Option<AudioStreamFormat>, callback: CB) -> Result<PullAudioInputStream<CB>, SpxError> {
        let mut result = PullAudioInputStream {
            base: BaseAudioInputStream::create("PullAudioInputStream", format, audio_stream_create_pull_audio_input_stream)?,
            callback: Box::new(callback),
        };

        unsafe {
            let cb_ptr = &mut *result.callback as *mut _ as *mut c_void;
            convert_err(pull_audio_input_stream_set_callbacks(
                result.get_handle(),
                cb_ptr,
                Some(Self::cb_read),
                Some(Self::cb_close),
            ))?;
        }

        Ok(result)
    }

    extern "C" fn cb_read(
        pv_ctx: *mut ::std::os::raw::c_void,
        buff: *mut u8,
        size: u32,
    ) -> ::std::os::raw::c_int {
        let cb = unsafe { &mut *(pv_ctx as *mut CB) };
        let buff = unsafe { slice::from_raw_parts_mut(buff, size as usize) };
        cb.read(buff) as i32
    }

    extern "C" fn cb_close(pv_ctx: *mut ::std::os::raw::c_void) {
        let cb = unsafe { &mut *(pv_ctx as *mut CB) };
        cb.close();
    }
}

impl<CB> Deref for PullAudioInputStream<CB> {
    type Target = BaseAudioInputStream;

    fn deref(&self) -> &BaseAudioInputStream {
        &self.base
    }
}

impl<CB: Send> AudioInputStream for PullAudioInputStream<CB> {
    #[inline(always)]
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE {
        self.base.handle.get()
    }
}

unsafe impl<CB: Send> Send for PullAudioInputStream<CB> {}
