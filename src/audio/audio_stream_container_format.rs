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

impl AudioStreamContainerFormat {
    /// Converts enum instance to respective u32 value.
    pub fn to_u32(&self) -> u32 {
        match self {
            AudioStreamContainerFormat::OggOpus => 257,
            AudioStreamContainerFormat::Mp3 => 258,
            AudioStreamContainerFormat::Flac => 259,
            AudioStreamContainerFormat::Alaw => 260,
            AudioStreamContainerFormat::Mulaw => 261,
            /// Currently not supported
            AudioStreamContainerFormat::Amrnb => 262,
            /// Currently not supported
            AudioStreamContainerFormat::Amrwb => 263,
        }
    }

    pub fn to_i32(&self) -> i32 {
        self.to_u32() as i32
    }
}
