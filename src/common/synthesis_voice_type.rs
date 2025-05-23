/// SynthesisVoiceType defines the type of a synthesis voice.
#[derive(Debug)]
pub enum SynthesisVoiceType {
    /// OnlineNeural indicates online neural voice.
    OnlineNeural = 1,

    /// OnlineStandard indicates online standard voice.
    OnlineStandard = 2,

    /// OfflineNeural indicates offline neural voice.
    OfflineNeural = 3,

    /// OfflineStandard indicates offline started voice.
    OfflineStandard = 4,
}

impl SynthesisVoiceType {
    pub fn from_u32(reason: u32) -> Self {
        match reason {
            1 => SynthesisVoiceType::OnlineNeural,
            2 => SynthesisVoiceType::OnlineStandard,
            3 => SynthesisVoiceType::OfflineNeural,
            _ => SynthesisVoiceType::OfflineStandard,
        }
    }
}

impl From<u32> for SynthesisVoiceType {
    fn from(value: u32) -> Self {
        SynthesisVoiceType::from_u32(value)
    }
}

impl From<i32> for SynthesisVoiceType {
    fn from(value: i32) -> Self {
        SynthesisVoiceType::from_u32(value as u32)
    }
}
