/// ResultReason specifies the possible reasons a recognition result might be generated.
#[derive(Debug)]
pub enum ResultReason {
    /// NoMatch indicates speech could not be recognized. More details can be found in the NoMatchDetails object.
    NoMatch = 0,

    /// Canceled indicates that the recognition was canceled. More details can be found using the CancellationDetails object.
    Canceled = 1,

    /// RecognizingSpeech indicates the speech result contains hypothesis text.
    RecognizingSpeech = 2,

    /// RecognizedSpeech indicates the speech result contains final text that has been recognized.
    /// Speech Recognition is now complete for this phrase.
    RecognizedSpeech = 3,

    /// RecognizingIntent indicates the intent result contains hypothesis text and intent.
    RecognizingIntent = 4,

    /// RecognizedIntent indicates the intent result contains final text and intent.
    /// Speech Recognition and Intent determination are now complete for this phrase.
    RecognizedIntent = 5,

    /// TranslatingSpeech indicates the translation result contains hypothesis text and its translation(s).
    TranslatingSpeech = 6,

    /// TranslatedSpeech indicates the translation result contains final text and corresponding translation(s).
    /// Speech Recognition and Translation are now complete for this phrase.
    TranslatedSpeech = 7,

    /// SynthesizingAudio indicates the synthesized audio result contains a non-zero amount of audio data
    SynthesizingAudio = 8,

    /// SynthesizingAudioCompleted indicates the synthesized audio is now complete for this phrase.
    SynthesizingAudioCompleted = 9,

    /// RecognizingKeyword indicates the speech result contains (unverified) keyword text.
    RecognizingKeyword = 10,

    /// RecognizedKeyword indicates that keyword recognition completed recognizing the given keyword.
    RecognizedKeyword = 11,

    /// SynthesizingAudioStarted indicates the speech synthesis is now started
    SynthesizingAudioStarted = 12,

    /// VoicesListRetrieved indicates the voices list has been retrieved successfully.
    VoicesListRetrieved = 23,
}

impl ResultReason {
    pub fn from_u32(reason: u32) -> Self {
        match reason {
            0 => ResultReason::NoMatch,
            1 => ResultReason::Canceled,
            2 => ResultReason::RecognizingSpeech,
            3 => ResultReason::RecognizedSpeech,
            4 => ResultReason::RecognizingIntent,
            5 => ResultReason::RecognizedIntent,
            6 => ResultReason::TranslatingSpeech,
            7 => ResultReason::TranslatedSpeech,
            8 => ResultReason::SynthesizingAudio,
            9 => ResultReason::SynthesizingAudioCompleted,
            10 => ResultReason::RecognizingKeyword,
            11 => ResultReason::RecognizedKeyword,
            12 => ResultReason::SynthesizingAudioStarted,
            _ => ResultReason::VoicesListRetrieved,
        }
    }
}

impl From<u32> for ResultReason {
    fn from(value: u32) -> Self {
        ResultReason::from_u32(value)
    }
}

impl From<i32> for ResultReason {
    fn from(value: i32) -> Self {
        ResultReason::from_u32(value as u32)
    }
}
