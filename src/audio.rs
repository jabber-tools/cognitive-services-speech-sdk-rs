//! Package audio provides the audio configuration, input/output streams, and related utilities for audio interactions.
mod audio_config;
mod audio_input_stream;
mod audio_output_stream;
mod audio_stream_container_format;
mod audio_stream_format;
mod pull_audio_input_stream;
mod pull_audio_output_stream;
mod push_audio_input_stream;
mod push_audio_output_stream;

// re-export structs directly under audio module
pub use self::audio_config::AudioConfig;
pub use self::audio_input_stream::AudioInputStream;
pub use self::audio_output_stream::AudioOutputStream;
pub use self::audio_stream_container_format::AudioStreamContainerFormat;
pub use self::audio_stream_format::AudioStreamFormat;
pub use self::pull_audio_input_stream::PullAudioInputStream;
pub use self::pull_audio_input_stream::PullAudioInputStreamCallbacks;
pub use self::pull_audio_output_stream::PullAudioOutputStream;
pub use self::push_audio_input_stream::PushAudioInputStream;
pub use self::push_audio_output_stream::PushAudioOutputStream;
pub use self::push_audio_output_stream::PushAudioOutputStreamCallbacks;
