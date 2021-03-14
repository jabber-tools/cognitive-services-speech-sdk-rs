use crate::error::{convert_err, Result};
use crate::ffi::property_bag_set_string;
use crate::ffi::{SmartHandle, SPXPROPERTYBAGHANDLE};
use std::ffi::CString;

// TODO: rename to PropertyBag
#[derive(Debug)]
pub struct PropertyCollection {
    pub handle: SmartHandle<SPXPROPERTYBAGHANDLE>,
}

impl PropertyCollection {
    pub fn set_property_by_string<S>(&mut self, prop_nam: S, prop_val: S) -> Result<()>
    where
        S: Into<Vec<u8>>,
    {
        let c_name = CString::new(prop_nam)?;
        let c_val = CString::new(prop_val)?;
        unsafe {
            let ret =
                property_bag_set_string(self.handle.get(), -1, c_name.as_ptr(), c_val.as_ptr());
            convert_err(ret, "PropertyCollection.set_property_by_string error")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ResultReason {
    // NoMatch indicates speech could not be recognized. More details can be found in the NoMatchDetails object.
    NoMatch = 0,

    // Canceled indicates that the recognition was canceled. More details can be found using the CancellationDetails object.
    Canceled = 1,

    // RecognizingSpeech indicates the speech result contains hypothesis text.
    RecognizingSpeech = 2,

    // RecognizedSpeech indicates the speech result contains final text that has been recognized.
    // Speech Recognition is now complete for this phrase.
    RecognizedSpeech = 3,

    // RecognizingIntent indicates the intent result contains hypothesis text and intent.
    RecognizingIntent = 4,

    // RecognizedIntent indicates the intent result contains final text and intent.
    // Speech Recognition and Intent determination are now complete for this phrase.
    RecognizedIntent = 5,

    // TranslatingSpeech indicates the translation result contains hypothesis text and its translation(s).
    TranslatingSpeech = 6,

    // TranslatedSpeech indicates the translation result contains final text and corresponding translation(s).
    // Speech Recognition and Translation are now complete for this phrase.
    TranslatedSpeech = 7,

    // SynthesizingAudio indicates the synthesized audio result contains a non-zero amount of audio data
    SynthesizingAudio = 8,

    // SynthesizingAudioCompleted indicates the synthesized audio is now complete for this phrase.
    SynthesizingAudioCompleted = 9,

    // RecognizingKeyword indicates the speech result contains (unverified) keyword text.
    RecognizingKeyword = 10,

    // RecognizedKeyword indicates that keyword recognition completed recognizing the given keyword.
    RecognizedKeyword = 11,

    // SynthesizingAudioStarted indicates the speech synthesis is now started
    SynthesizingAudioStarted = 12,
}

impl ResultReason {
    pub fn from_u32(reason: u32) -> Self {
        return match reason {
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
            _ => ResultReason::SynthesizingAudioStarted,
        };
    }
}
