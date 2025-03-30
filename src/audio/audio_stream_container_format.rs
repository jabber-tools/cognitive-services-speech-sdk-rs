#![allow(warnings)]
/// AudioStreamContainerFormat defines supported audio stream container format.
#[derive(Debug)]
pub enum AudioStreamContainerFormat {
    OggOpus = 257,
    Mp3 = 258,
    Flac = 259,
    Alaw = 260,
    Mulaw = 261,

    /// Currently not supported
    Amrnb = 262,

    /// Currently not supported
    Amrwb = 263,
}

impl From<AudioStreamContainerFormat> for u32 {
    fn from(format: AudioStreamContainerFormat) -> Self {
        format as u32
    }
}

impl From<AudioStreamContainerFormat> for i32 {
    fn from(format: AudioStreamContainerFormat) -> Self {
        format as i32
    }
}
