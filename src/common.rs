use crate::error::{convert_err, Result};
use crate::ffi::{property_bag_get_string, property_bag_set_string};
use crate::ffi::{SmartHandle, SPXPROPERTYBAGHANDLE};
use std::ffi::{CStr, CString};

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

    pub fn get_property<S>(&self, prop_id: PropertyId, default_val: S) -> Result<String>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            // TODO: see NULL_C_STR_PTR in orig solution
            let ret = property_bag_get_string(
                self.handle.get(),
                prop_id.to_i32(),
                std::ptr::null(),
                CString::new(default_val)?.into_raw(),
            );
            Ok(CStr::from_ptr(ret).to_str()?.to_owned())
        }
    }

    pub fn get_property_by_string(&self, prop_name: String, default_val: String) -> String {
        unimplemented!();
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

#[derive(Debug)]
pub enum CancellationReason {
    Error = 1,
    EndOfStream = 2,
}

impl CancellationReason {
    pub fn from_u32(code: u32) -> Self {
        return match code {
            1 => CancellationReason::Error,
            _ => CancellationReason::EndOfStream,
        };
    }
}

#[derive(Debug)]
pub enum CancellationErrorCode {
    NoError = 0,
    AuthenticationFailure = 1,
    BadRequest = 2,
    TooManyRequests = 3,
    Forbidden = 4,
    ConnectionFailure = 5,
    ServiceTimeout = 6,
    ServiceError = 7,
    ServiceUnavailable = 8,
    RuntimeError = 9,
}

impl CancellationErrorCode {
    pub fn from_u32(code: u32) -> Self {
        return match code {
            0 => CancellationErrorCode::NoError,
            1 => CancellationErrorCode::AuthenticationFailure,
            2 => CancellationErrorCode::BadRequest,
            3 => CancellationErrorCode::TooManyRequests,
            4 => CancellationErrorCode::Forbidden,
            5 => CancellationErrorCode::ConnectionFailure,
            6 => CancellationErrorCode::ServiceTimeout,
            7 => CancellationErrorCode::ServiceError,
            8 => CancellationErrorCode::ServiceUnavailable,
            _ => CancellationErrorCode::RuntimeError,
        };
    }
}

pub enum PropertyId {
    SpeechServiceConnectionKey = 1000,
    SpeechServiceConnectionEndpoint = 1001,
    SpeechServiceConnectionRegion = 1002,
    SpeechServiceAuthorizationToken = 1003,
    SpeechServiceAuthorizationType = 1004,
    SpeechServiceConnectionEndpointId = 1005,

    SpeechServiceConnectionProxyHostName = 1100,
    SpeechServiceConnectionProxyPort = 1101,
    SpeechServiceConnectionProxyUserName = 1102,
    SpeechServiceConnectionProxyPassword = 1103,

    SpeechServiceConnectionTranslationToLanguages = 2000,
    SpeechServiceConnectionTranslationVoice = 2001,
    SpeechServiceConnectionTranslationFeatures = 2002,
    SpeechServiceConnectionIntentRegion = 2003,

    SpeechServiceConnectionRecoMode = 3000,
    SpeechServiceConnectionRecoLanguage = 3001,
    SpeechSessionId = 3002,

    SpeechServiceConnectionSynthLanguage = 3100,
    SpeechServiceConnectionSynthVoice = 3101,
    SpeechServiceConnectionSynthOutputFormat = 3102,

    SpeechServiceResponseRequestDetailedResultTrueFalse = 4000,
    SpeechServiceResponseRequestProfanityFilterTrueFalse = 4001,

    SpeechServiceResponseJsonResult = 5000,
    SpeechServiceResponseJsonErrorDetails = 5001,
    SpeechServiceResponseRecognitionLatencyMs = 5002,

    CancellationDetailsReason = 6000,
    CancellationDetailsReasonText = 6001,
    CancellationDetailsReasonDetailedText = 6002,

    LanguageUnderstandingServiceResponseJsonResult = 7000,

    AudioConfigDeviceNameForCapture = 8000,
    AudioConfigNumberOfChannelsForCapture = 8001,
    AudioConfigSampleRateForCapture = 8002,
    AudioConfigBitsPerSampleForCapture = 8003,
    AudioConfigAudioSource = 8004,

    SpeechLogFilename = 9001,
}

impl PropertyId {
    pub fn to_i32(&self) -> i32 {
        return match self {
            PropertyId::SpeechServiceConnectionKey => 1000,
            PropertyId::SpeechServiceConnectionEndpoint => 1001,
            PropertyId::SpeechServiceConnectionRegion => 1002,
            PropertyId::SpeechServiceAuthorizationToken => 1003,
            PropertyId::SpeechServiceAuthorizationType => 1004,
            PropertyId::SpeechServiceConnectionEndpointId => 1005,

            PropertyId::SpeechServiceConnectionProxyHostName => 1100,
            PropertyId::SpeechServiceConnectionProxyPort => 1101,
            PropertyId::SpeechServiceConnectionProxyUserName => 1102,
            PropertyId::SpeechServiceConnectionProxyPassword => 1103,

            PropertyId::SpeechServiceConnectionTranslationToLanguages => 2000,
            PropertyId::SpeechServiceConnectionTranslationVoice => 2001,
            PropertyId::SpeechServiceConnectionTranslationFeatures => 2002,
            PropertyId::SpeechServiceConnectionIntentRegion => 2003,

            PropertyId::SpeechServiceConnectionRecoMode => 3000,
            PropertyId::SpeechServiceConnectionRecoLanguage => 3001,
            PropertyId::SpeechSessionId => 3002,

            PropertyId::SpeechServiceConnectionSynthLanguage => 3100,
            PropertyId::SpeechServiceConnectionSynthVoice => 3101,
            PropertyId::SpeechServiceConnectionSynthOutputFormat => 3102,

            PropertyId::SpeechServiceResponseRequestDetailedResultTrueFalse => 4000,
            PropertyId::SpeechServiceResponseRequestProfanityFilterTrueFalse => 4001,

            PropertyId::SpeechServiceResponseJsonResult => 5000,
            PropertyId::SpeechServiceResponseJsonErrorDetails => 5001,
            PropertyId::SpeechServiceResponseRecognitionLatencyMs => 5002,

            PropertyId::CancellationDetailsReason => 6000,
            PropertyId::CancellationDetailsReasonText => 6001,
            PropertyId::CancellationDetailsReasonDetailedText => 6002,

            PropertyId::LanguageUnderstandingServiceResponseJsonResult => 7000,

            PropertyId::AudioConfigDeviceNameForCapture => 8000,
            PropertyId::AudioConfigNumberOfChannelsForCapture => 8001,
            PropertyId::AudioConfigSampleRateForCapture => 8002,
            PropertyId::AudioConfigBitsPerSampleForCapture => 8003,
            PropertyId::AudioConfigAudioSource => 8004,

            PropertyId::SpeechLogFilename => 9001,
        };
    }
}
