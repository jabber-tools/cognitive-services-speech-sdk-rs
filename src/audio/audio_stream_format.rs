use crate::audio::AudioStreamContainerFormat;
use crate::error::{convert_err, Result};
use crate::ffi::{
    audio_stream_format_create_from_compressed_format,
    audio_stream_format_create_from_default_input, audio_stream_format_create_from_default_output,
    audio_stream_format_create_from_waveformat_pcm, audio_stream_format_release, SmartHandle,
    SPXAUDIOSTREAMFORMATHANDLE,
};
use std::mem::MaybeUninit;

/// AudioStreamFormat represents the audio stream format used for custom audio input configurations.
#[derive(Debug)]
pub struct AudioStreamFormat {
    /// Smart handle wrapping underlying SPXAUDIOSTREAMFORMATHANDLE
    pub handle: SmartHandle<SPXAUDIOSTREAMFORMATHANDLE>,
}

impl AudioStreamFormat {
    fn from_handle(handle: SPXAUDIOSTREAMFORMATHANDLE) -> Result<AudioStreamFormat> {
        Ok(AudioStreamFormat {
            handle: SmartHandle::create("AudioStreamFormat", handle, audio_stream_format_release),
        })
    }

    /// GetWaveFormatPCM creates an audio stream format object with the specified PCM waveformat characteristics.
    /// Note: Currently, only WAV / PCM with 16-bit samples, 16 kHz sample rate, and a single channel (Mono) is supported. When
    /// used with Conversation Transcription, eight channels are supported.
    pub fn get_wave_format_pcm(
        samples_per_second: u32,
        bits_per_sample: Option<u8>,
        channels: Option<u8>,
    ) -> Result<AudioStreamFormat> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = audio_stream_format_create_from_waveformat_pcm(
                handle.as_mut_ptr(),
                samples_per_second,
                bits_per_sample.unwrap_or(16),
                channels.unwrap_or(1),
            );
            convert_err(ret, "AudioStreamFormat::get_wave_format_pcm error")?;
            AudioStreamFormat::from_handle(handle.assume_init())
        }
    }

    /// GetDefaultInputFormat creates an audio stream format object representing the default audio stream format
    /// (16 kHz, 16 bit, mono PCM).
    pub fn get_default_input_format() -> Result<AudioStreamFormat> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = audio_stream_format_create_from_default_input(handle.as_mut_ptr());
            convert_err(ret, "AudioStreamFormat::get_default_format error")?;
            AudioStreamFormat::from_handle(handle.assume_init())
        }
    }

    /// GetDefaultOutputFormat creates an audio stream format object representing the default audio stream format
    /// (16 kHz, 16 bit, mono PCM).
    pub fn get_default_output_format() -> Result<AudioStreamFormat> {
        unsafe {
            let mut handle = MaybeUninit::<SPXAUDIOSTREAMFORMATHANDLE>::uninit();
            let ret = audio_stream_format_create_from_default_output(handle.as_mut_ptr());
            convert_err(ret, "AudioStreamFormat::get_default_output_format error")?;
            AudioStreamFormat::from_handle(handle.assume_init())
        }
    }

    /// GetCompressedFormat creates an audio stream format object with the specified compressed audio container format, to be
    /// used as input format.
    pub fn get_compressed_format(
        compressed_format: AudioStreamContainerFormat,
    ) -> Result<AudioStreamFormat> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            #[cfg(not(target_os = "windows"))]
            let compressed_format = compressed_format.to_u32();
            #[cfg(target_os = "windows")]
            let compressed_format = compressed_format.to_i32();

            let ret = audio_stream_format_create_from_compressed_format(
                handle.as_mut_ptr(),
                compressed_format,
            );
            convert_err(ret, "AudioStreamFormat::get_compressed_format error")?;
            AudioStreamFormat::from_handle(handle.assume_init())
        }
    }
}
