use crate::common::{PropertyCollection, StreamStatus};
use crate::error::{convert_err, Error, ErrorRootCause, Result};
use crate::ffi::{
    audio_data_stream_can_read_data, audio_data_stream_can_read_data_from_position,
    audio_data_stream_create_from_file, audio_data_stream_create_from_result,
    audio_data_stream_get_position, audio_data_stream_get_property_bag,
    audio_data_stream_get_status, audio_data_stream_read, audio_data_stream_read_from_position,
    audio_data_stream_save_to_wave_file, audio_data_stream_set_position, audio_stream_release,
    SmartHandle, SPXAUDIOSTREAMHANDLE,
};
use crate::speech::SpeechSynthesisResult;
use std::ffi::CString;
use std::mem::MaybeUninit;

/// AudioDataStream represents audio data retrieved either from file
/// or result of speech synthesis. Represents convenient option for
/// manipulating and storing of syntehtized audio data
/// Added in version 1.17.0
#[derive(Debug)]
pub struct AudioDataStream {
    pub handle: SmartHandle<SPXAUDIOSTREAMHANDLE>,
    pub properties: PropertyCollection,
}

impl AudioDataStream {
    fn from_handle(handle: SPXAUDIOSTREAMHANDLE) -> Result<Self> {
        unsafe {
            let mut prop_bag_handle = MaybeUninit::uninit();
            let ret = audio_data_stream_get_property_bag(handle, prop_bag_handle.as_mut_ptr());
            convert_err(ret, "AudioDataStream::from_handle error")?;

            let property_bag = PropertyCollection::from_handle(prop_bag_handle.assume_init());

            Ok(AudioDataStream {
                handle: SmartHandle::create("AudioDataStream", handle, audio_stream_release),
                properties: property_bag,
            })
        }
    }

    pub fn from_wav_file(filename: &str) -> Result<Self> {
        unsafe {
            let c_filename = CString::new(filename)?;
            let mut handle = MaybeUninit::uninit();
            let ret = audio_data_stream_create_from_file(handle.as_mut_ptr(), c_filename.as_ptr());
            convert_err(ret, "AudioDataStream::from_wav_file error")?;
            AudioDataStream::from_handle(handle.assume_init())
        }
    }

    pub fn from_speech_synthesis_result(
        speech_synthesis_result: SpeechSynthesisResult,
    ) -> Result<Self> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = audio_data_stream_create_from_result(
                handle.as_mut_ptr(),
                speech_synthesis_result.handle.inner(),
            );
            convert_err(ret, "AudioDataStream::from_speech_synthesis_result error")?;
            AudioDataStream::from_handle(handle.assume_init())
        }
    }

    pub fn get_status(&self) -> Result<StreamStatus> {
        unsafe {
            let mut status = MaybeUninit::uninit();
            let ret = audio_data_stream_get_status(self.handle.inner(), status.as_mut_ptr());
            convert_err(ret, "AudioDataStream.get_status error")?;

            #[cfg(target_os = "windows")]
            return Ok(StreamStatus::from_i32(status.assume_init()));
            #[cfg(not(target_os = "windows"))]
            return Ok(StreamStatus::from_u32(status.assume_init()));
        }
    }

    pub fn can_read_data(&self, requested_size: u32) -> bool {
        unsafe { audio_data_stream_can_read_data(self.handle.inner(), requested_size) }
    }

    pub fn can_read_data_at(&self, requested_size: u32, offset: u32) -> bool {
        unsafe {
            audio_data_stream_can_read_data_from_position(
                self.handle.inner(),
                requested_size,
                offset,
            )
        }
    }

    /// Read reads a chunk of the audio data stream and fill it to given buffer.
    /// It returns size of data filled to the buffer and any write error encountered.
    pub fn read(&self, buffer: &mut [u8]) -> Result<u32> {
        unsafe {
            #[allow(clippy::len_zero)]
            if buffer.len() == 0 {
                let rootc = ErrorRootCause::ApiError(0x005);
                return Err(Error::new(Error::api_error_desc(&rootc).unwrap(), rootc));
            }
            let mut filled_size = MaybeUninit::uninit();
            // let c_buffer: *mut u8 = buffer as *const _ as *mut u8;
            let c_buffer = buffer.as_mut_ptr();
            let ret = audio_data_stream_read(
                self.handle.inner(),
                c_buffer,
                buffer.len() as u32,
                filled_size.as_mut_ptr(),
            );
            convert_err(ret, "AudioDataStream.read error")?;
            Ok(filled_size.assume_init())
        }
    }

    /// ReadAt reads a chunk of the audio data stream and fill it to given buffer, at specified offset.
    /// It returns size of data filled to the buffer and any write error encountered.
    pub fn read_at(&self, buffer: &mut [u8], offset: u32) -> Result<u32> {
        unsafe {
            #[allow(clippy::len_zero)]
            if buffer.len() == 0 {
                let rootc = ErrorRootCause::ApiError(0x005);
                return Err(Error::new(Error::api_error_desc(&rootc).unwrap(), rootc));
            }
            let mut filled_size = MaybeUninit::uninit();
            // let c_buffer: *mut u8 = buffer as *const _ as *mut u8;
            let c_buffer = buffer.as_mut_ptr();
            let ret = audio_data_stream_read_from_position(
                self.handle.inner(),
                c_buffer,
                buffer.len() as u32,
                offset,
                filled_size.as_mut_ptr(),
            );
            convert_err(ret, "AudioDataStream.read_at error")?;
            Ok(filled_size.assume_init())
        }
    }

    pub async fn save_wav_file_async(&self, filename: &str) -> Result<()> {
        unsafe {
            let c_filename = CString::new(filename)?;
            let ret = audio_data_stream_save_to_wave_file(self.handle.inner(), c_filename.as_ptr());
            convert_err(ret, "AudioDataStream.save_wav_file_async error")?;
            Ok(())
        }
    }

    pub fn get_offset(&self) -> Result<u32> {
        unsafe {
            let mut offset = MaybeUninit::uninit();
            let ret = audio_data_stream_get_position(self.handle.inner(), offset.as_mut_ptr());
            convert_err(ret, "AudioDataStream.get_offset error")?;
            Ok(offset.assume_init())
        }
    }

    pub fn set_offset(&self, offset: u32) -> Result<()> {
        unsafe {
            let ret = audio_data_stream_set_position(self.handle.inner(), offset);
            convert_err(ret, "AudioDataStream.set_offset error")?;
            Ok(())
        }
    }
}
