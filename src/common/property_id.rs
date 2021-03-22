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
