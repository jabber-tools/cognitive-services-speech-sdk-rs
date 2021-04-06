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

    SpeechServiceConnectionEnableAudioLogging = 3202,

    SpeechServiceResponseRequestDetailedResultTrueFalse = 4000,
    SpeechServiceResponseRequestProfanityFilterTrueFalse = 4001,

    SpeechServiceResponseRequestWordLevelTimestamps = 4004,

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

    ConversationApplicationID = 10000,
    ConversationDialogType = 10001,
    ConversationInitialSilenceTimeout = 10002,
    ConversationFromID = 10003,
    ConversationConversationID = 10004,
    ConversationCustomVoiceDeploymentIDs = 10005,
    ConversationSpeechActivityTemplate = 10006,
    DataBufferTimeStamp = 11001,
    DataBufferUserID = 11002,
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

            PropertyId::SpeechServiceConnectionEnableAudioLogging => 3202,

            PropertyId::SpeechServiceResponseRequestDetailedResultTrueFalse => 4000,
            PropertyId::SpeechServiceResponseRequestProfanityFilterTrueFalse => 4001,
            PropertyId::SpeechServiceResponseRequestWordLevelTimestamps => 4004,

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

            PropertyId::ConversationApplicationID => 10000,
            PropertyId::ConversationDialogType => 10001,
            PropertyId::ConversationInitialSilenceTimeout => 10002,
            PropertyId::ConversationFromID => 10003,
            PropertyId::ConversationConversationID => 10004,
            PropertyId::ConversationCustomVoiceDeploymentIDs => 10005,
            PropertyId::ConversationSpeechActivityTemplate => 10006,
            PropertyId::DataBufferTimeStamp => 11001,
            PropertyId::DataBufferUserID => 11002,
        };
    }
}
