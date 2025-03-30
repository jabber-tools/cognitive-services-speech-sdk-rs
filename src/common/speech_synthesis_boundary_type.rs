/// SpeechSynthesisBoundaryType defines the boundary type of speech synthesis boundary event.
#[derive(Debug)]
pub enum SpeechSynthesisBoundaryType {
    /// WordBoundary indicates word boundary.
    WordBoundary = 0,

    /// PunctuationBoundary indicates punctuation boundary.
    PunctuationBoundary = 1,

    /// SentenceBoundary indicates sentence boundary.
    SentenceBoundary = 2,
}

impl SpeechSynthesisBoundaryType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => SpeechSynthesisBoundaryType::WordBoundary,
            1 => SpeechSynthesisBoundaryType::PunctuationBoundary,
            _ => SpeechSynthesisBoundaryType::SentenceBoundary,
        }
    }
}

impl From<u32> for SpeechSynthesisBoundaryType {
    fn from(value: u32) -> Self {
        SpeechSynthesisBoundaryType::from_u32(value)
    }
}

impl From<i32> for SpeechSynthesisBoundaryType {
    fn from(value: i32) -> Self {
        SpeechSynthesisBoundaryType::from_u32(value as u32)
    }
}
