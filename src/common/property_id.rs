/// PropertyID defines speech property ids.
pub enum PropertyId {
    /// SpeechServiceConnectionKey is the Cognitive Services Speech Service subscription key. If you are using an
    /// intent recognizer, you need to specify the LUIS endpoint key for your particular LUIS app. Under normal
    /// circumstances, you shouldn't have to use this property directly.
    /// Instead, use NewSpeechConfigFromSubscription.
    SpeechServiceConnectionKey = 1000,

    /// SpeechServiceConnectionEndpoint is the Cognitive Services Speech Service endpoint (url).
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use NewSpeechConfigFromEndpoint.
    /// NOTE: This endpoint is not the same as the endpoint used to obtain an access token.
    SpeechServiceConnectionEndpoint = 1001,

    /// SpeechServiceConnectionRegion is the Cognitive Services Speech Service region. Under normal circumstances,
    /// you shouldn't have to use this property directly.
    /// Instead, use NewSpeechConfigFromSubscription, NewSpeechConfigFromEndpoint, NewSpeechConfigFromHost,
    /// NewSpeechConfigFromAuthorizationToken.
    SpeechServiceConnectionRegion = 1002,

    /// SpeechServiceAuthorizationToken is the Cognitive Services Speech Service authorization token (aka access token).
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use NewSpeechConfigFromAuthorizationToken,
    /// Recognizer.SetAuthorizationToken
    SpeechServiceAuthorizationToken = 1003,

    //// SpeechServiceAuthorizationType is the Cognitive Services Speech Service authorization type. Currently unused.
    SpeechServiceAuthorizationType = 1004,

    /// SpeechServiceConnectionEndpointID is the Cognitive Services Custom Speech Service endpoint id. Under normal
    /// circumstances, you shouldn't have to use this property directly.
    /// Instead use SpeechConfig.SetEndpointId.
    /// NOTE: The endpoint id is available in the Custom Speech Portal, listed under Endpoint Details.
    SpeechServiceConnectionEndpointId = 1005,

    /// SpeechServiceConnectionHost is the Cognitive Services Speech Service host (url). Under normal circumstances,
    /// you shouldn't have to use this property directly.
    /// Instead, use NewSpeechConfigFromHost.
    SpeechServiceConnectionHost = 1006,

    /// SpeechServiceConnectionProxyHostName is the host name of the proxy server used to connect to the Cognitive Services
    /// Speech Service. Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use SpeechConfig.SetProxy.
    SpeechServiceConnectionProxyHostName = 1100,

    /// SpeechServiceConnectionProxyPort is the port of the proxy server used to connect to the Cognitive Services Speech
    /// Service. Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use SpeechConfig.SetProxy.
    SpeechServiceConnectionProxyPort = 1101,

    /// SpeechServiceConnectionProxyUserName is the user name of the proxy server used to connect to the Cognitive Services
    /// Speech Service. Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use SpeechConfig.SetProxy.
    SpeechServiceConnectionProxyUserName = 1102,

    /// SpeechServiceConnectionProxyPassword is the password of the proxy server used to connect to the Cognitive Services
    /// Speech Service. Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use SpeechConfig.SetProxy.
    SpeechServiceConnectionProxyPassword = 1103,

    /// SpeechServiceConnectionURL is the URL string built from speech configuration. This property is intended to be read-only.
    /// The SDK is using it internally.
    SpeechServiceConnectionURL = 1104,

    /// SpeechServiceConnectionTranslationToLanguages is the list of comma separated languages used as target translation
    /// languages. Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead use SpeechTranslationConfig.AddTargetLanguage and SpeechTranslationConfig.GetTargetLanguages.
    SpeechServiceConnectionTranslationToLanguages = 2000,

    /// SpeechServiceConnectionTranslationVoice is the name of the Cognitive Service Text to Speech Service voice. Under normal
    /// circumstances, you shouldn't have to use this property directly.
    /// Instead use SpeechTranslationConfig.SetVoiceName.
    /// NOTE: Valid voice names can be found at https:///aka.ms/csspeech/voicenames.
    SpeechServiceConnectionTranslationVoice = 2001,

    /// SpeechServiceConnectionTranslationFeatures is the translation features. For internal use.
    SpeechServiceConnectionTranslationFeatures = 2002,

    /// SpeechServiceConnectionIntentRegion is the Language Understanding Service region. Under normal circumstances, you
    /// shouldn't have to use this property directly.
    /// Instead use LanguageUnderstandingModel.
    SpeechServiceConnectionIntentRegion = 2003,

    /// SpeechServiceConnectionRecoMode is the Cognitive Services Speech Service recognition mode. Can be "INTERACTIVE",
    /// "CONVERSATION" or "DICTATION".
    /// This property is intended to be read-only. The SDK is using it internally.
    SpeechServiceConnectionRecoMode = 3000,

    /// SpeechServiceConnectionRecoLanguage is the spoken language to be recognized (in BCP-47 format). Under normal
    /// circumstances, you shouldn't have to use this property directly.
    /// Instead, use SpeechConfig.SetSpeechRecognitionLanguage.
    SpeechServiceConnectionRecoLanguage = 3001,

    /// SpeechSessionID is the session id. This id is a universally unique identifier (aka UUID) representing a specific
    /// binding of an audio input stream and the underlying speech recognition instance to which it is bound. Under normal
    /// circumstances, you shouldn't have to use this property directly.
    //// Instead use SessionEventArgs.SessionId.
    SpeechSessionId = 3002,

    /// SpeechServiceConnectionUserDefinedQueryParameters are the query parameters provided by users. They will be passed
    /// to the service as URL query parameters.
    SpeechServiceConnectionUserDefinedQueryParameters = 3003,

    /// The name of the model to be used for speech recognition.
    /// Under normal circumstances, you shouldn't use this property directly.
    /// Currently this is only valid when EmbeddedSpeechConfig is used.
    /// Added in version 1.19.0
    SpeechServiceConnectionRecoModelName = 3005,

    /// The decryption key of the model to be used for speech recognition.
    /// Under normal circumstances, you shouldn't use this property directly.
    /// Currently this is only valid when EmbeddedSpeechConfig is used.
    /// Added in version 1.19.0
    SpeechServiceConnectionRecoModelKey = 3006,

    /// The path to the ini file of the model to be used for speech recognition.
    /// Under normal circumstances, you shouldn't use this property directly.
    /// Currently this is only valid when EmbeddedSpeechConfig is used.
    /// Added in version 1.19.0
    SpeechServiceConnectionRecoModelIniFile = 3007,

    //// SpeechServiceConnectionSynthLanguage is the spoken language to be synthesized (e.g. en-US)
    SpeechServiceConnectionSynthLanguage = 3100,

    /// SpeechServiceConnectionSynthVoice is the name of the TTS voice to be used for speech synthesis
    SpeechServiceConnectionSynthVoice = 3101,

    /// SpeechServiceConnectionSynthOutputFormat is the string to specify TTS output audio format.
    SpeechServiceConnectionSynthOutputFormat = 3102,

    /// SpeechServiceConnectionSynthEnableCompressedAudioTransmission indicates if use compressed audio format
    /// for speech synthesis audio transmission.
    /// This property only affects when SpeechServiceConnectionSynthOutputFormat is set to a pcm format.
    /// If this property is not set and GStreamer is available, SDK will use compressed format for synthesized audio transmission,
    /// and decode it. You can set this property to "false" to use raw pcm format for transmission on wire.
    SpeechServiceConnectionSynthEnableCompressedAudioTransmission = 3103,

    /// The string to specify TTS backend; valid options are online and offline.
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="EmbeddedSpeechConfig::FromPath"/> or <see cref="EmbeddedSpeechConfig::FromPaths"/>
    /// to set the synthesis backend to offline.
    /// Added in version 1.19.0
    SpeechServiceConnectionSynthBackend = 3110,

    /// The data file path(s) for offline synthesis engine; only valid when synthesis backend is offline.
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="EmbeddedSpeechConfig::FromPath"/> or <see cref="EmbeddedSpeechConfig::FromPaths"/>.
    /// Added in version 1.19.0
    SpeechServiceConnectionSynthOfflineDataPath = 3112,

    /// The name of the offline TTS voice to be used for speech synthesis
    /// Under normal circumstances, you shouldn't use this property directly.
    /// Added in version 1.19.0
    SpeechServiceConnectionSynthOfflineVoice = 3113,

    /// The decryption key of the voice to be used for speech synthesis.
    /// Under normal circumstances, you shouldn't use this property directly.
    /// Added in version 1.19.0
    SpeechServiceConnectionSynthModelKey = 3114,

    /// The Cognitive Services Speech Service voices list api endpoint (url). Under normal circumstances,
    /// you don't need to specify this property, SDK will construct it based on the region/host/endpoint of <see cref="SpeechConfig"/>.
    /// Added in version 1.16.0
    SpeechServiceConnectionVoicesListEndpoint = 3130,

    /// SpeechServiceConnectionInitialSilenceTimeoutMs is the initial silence timeout value (in milliseconds) used by the
    /// service.
    SpeechServiceConnectionInitialSilenceTimeoutMs = 3200,

    /// SpeechServiceConnectionEndSilenceTimeoutMs is the end silence timeout value (in milliseconds) used by the service.
    SpeechServiceConnectionEndSilenceTimeoutMs = 3201,

    /// SpeechServiceConnectionEnableAudioLogging is a boolean value specifying whether audio logging is enabled in the service
    /// or not.
    SpeechServiceConnectionEnableAudioLogging = 3202,

    /// The speech service connection language identifier mode.
    /// Can be "AtStart" (the default), or "Continuous". See [Language
    /// Identification](https://aka.ms/speech/lid?pivots=programming-language-cpp) document.
    /// Added in 1.25.0
    SpeechServiceConnectionLanguageIdMode = 3205,

    /// The auto detect source languages
    /// Added in version 1.8.0
    SpeechServiceConnectionAutoDetectSourceLanguages = 3300,

    /// The auto detect source language result
    /// Added in version 1.8.0
    SpeechServiceConnectionAutoDetectSourceLanguageResult = 3301,

    /// SpeechServiceResponseRequestDetailedResultTrueFalse the requested Cognitive Services Speech Service response output
    /// format (simple or detailed). Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead use SpeechConfig.SetOutputFormat.
    SpeechServiceResponseRequestDetailedResultTrueFalse = 4000,

    /// SpeechServiceResponseRequestProfanityFilterTrueFalse is the requested Cognitive Services Speech Service response
    /// output profanity level. Currently unused.
    SpeechServiceResponseRequestProfanityFilterTrueFalse = 4001,

    /// SpeechServiceResponseProfanityOption is the requested Cognitive Services Speech Service response output profanity
    /// setting.
    /// Allowed values are "masked", "removed", and "raw".
    SpeechServiceResponseProfanityOption = 4002,

    /// SpeechServiceResponsePostProcessingOption a string value specifying which post processing option should be used
    /// by the service.
    /// Allowed values are "TrueText".
    SpeechServiceResponsePostProcessingOption = 4003,

    /// SpeechServiceResponseRequestWordLevelTimestamps is a boolean value specifying whether to include word-level
    /// timestamps in the response result.
    SpeechServiceResponseRequestWordLevelTimestamps = 4004,

    /// SpeechServiceResponseStablePartialResultThreshold is the number of times a word has to be in partial results
    /// to be returned.
    SpeechServiceResponseStablePartialResultThreshold = 4005,

    /// SpeechServiceResponseOutputFormatOption is a string value specifying the output format option in the response
    /// result. Internal use only.
    SpeechServiceResponseOutputFormatOption = 4006,

    /// A boolean value specifying whether to include SNR (signal to noise ratio) in the response result.
    /// Added in version 1.18.0
    SpeechServiceResponseRequestSnr = 4007,

    /// SpeechServiceResponseTranslationRequestStablePartialResult is a boolean value to request for stabilizing translation
    /// partial results by omitting words in the end.
    SpeechServiceResponseTranslationRequestStablePartialResult = 4100,

    /// A boolean value specifying whether to request WordBoundary events.
    /// Added in version 1.21.0.
    SpeechServiceResponseRequestWordBoundary = 4200,

    /// A boolean value specifying whether to request punctuation boundary in WordBoundary Events. Default is true.
    /// Added in version 1.21.0.
    SpeechServiceResponseRequestPunctuationBoundary = 4201,

    /// A boolean value specifying whether to request sentence boundary in WordBoundary Events. Default is false.
    /// Added in version 1.21.0.
    SpeechServiceResponseRequestSentenceBoundary = 4202,

    /// A boolean value specifying whether the SDK should synchronize synthesis metadata events,
    /// (e.g. word boundary, viseme, etc.) to the audio playback. This only takes effect when the audio is played through the SDK.
    /// Default is true.
    /// If set to false, the SDK will fire the events as they come from the service, which may be out of sync with the audio playback.
    /// Added in version 1.31.0.
    SpeechServiceResponseSynthesisEventsSyncToAudio = 4210,

    /// SpeechServiceResponseJSONResult is the Cognitive Services Speech Service response output (in JSON format). This
    /// property is available on recognition result objects only.
    SpeechServiceResponseJsonResult = 5000,

    /// SpeechServiceResponseJSONErrorDetails is the Cognitive Services Speech Service error details (in JSON format).
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use CancellationDetails.ErrorDetails.
    SpeechServiceResponseJsonErrorDetails = 5001,

    /// SpeechServiceResponseRecognitionLatencyMs is the recognition latency in milliseconds. Read-only, available on final
    /// speech/translation/intent results. This measures the latency between when an audio input is received by the SDK, and
    /// the moment the final result is received from the service. The SDK computes the time difference between the last audio
    /// fragment from the audio input that is contributing to the final result, and the time the final result is received from
    /// the speech service.
    SpeechServiceResponseRecognitionLatencyMs = 5002,

    /// The recognition backend. Read-only, available on speech recognition results.
    /// This indicates whether cloud (online) or embedded (offline) recognition was used to produce the result.
    SpeechServiceResponseRecognitionBackend = 5003,

    /// SpeechServiceResponseSynthesisFirstByteLatencyMs is the speech synthesis first byte latency in milliseconds.
    /// Read-only, available on final speech synthesis results.
    /// This measures the latency between when the synthesis is started to be processed, and the moment the first byte audio is available.
    /// Added in version 1.17.0.
    SpeechServiceResponseSynthesisFirstByteLatencyMs = 5010,

    /// SpeechServiceResponseSynthesisFinishLatencyMs is the speech synthesis all bytes latency in milliseconds.
    /// Read-only, available on final speech synthesis results.
    /// This measures the latency between when the synthesis is started to be processed, and the moment the whole audio is synthesized.
    /// Added in version 1.17.0.
    SpeechServiceResponseSynthesisFinishLatencyMs = 5011,

    /// SpeechServiceResponseSynthesisUnderrunTimeMs is the underrun time for speech synthesis in milliseconds.
    /// Read-only, available on results in SynthesisCompleted events.
    /// This measures the total underrun time from AudioConfigPlaybackBufferLengthInMs is filled to synthesis completed.
    /// Added in version 1.17.0.
    SpeechServiceResponseSynthesisUnderrunTimeMs = 5012,

    /// The speech synthesis connection latency in milliseconds. Read-only, available on final speech synthesis results.
    /// This measures the latency between when the synthesis is started to be processed, and the moment the HTTP/WebSocket connection is established.
    /// Added in version 1.26.0.
    SpeechServiceResponseSynthesisConnectionLatencyMs = 5013,

    /// The speech synthesis network latency in milliseconds. Read-only, available on final speech synthesis results.
    /// This measures the network round trip time.
    /// Added in version 1.26.0.
    SpeechServiceResponseSynthesisNetworkLatencyMs = 5014,

    /// The speech synthesis service latency in milliseconds. Read-only, available on final speech synthesis results.
    /// This measures the service processing time to synthesize the first byte of audio.
    /// Added in version 1.26.0.
    SpeechServiceResponseSynthesisServiceLatencyMs = 5015,

    /// SpeechServiceResponseSynthesisBackend indicates which backend the synthesis is finished by.
    /// Read-only, available on speech synthesis results, except for the result in SynthesisStarted event
    /// Added in version 1.17.0.
    SpeechServiceResponseSynthesisBackend = 5020,

    /// CancellationDetailsReason is the cancellation reason. Currently unused.
    CancellationDetailsReason = 6000,

    /// CancellationDetailsReasonText the cancellation text. Currently unused.
    CancellationDetailsReasonText = 6001,

    /// CancellationDetailsReasonDetailedText is the cancellation detailed text. Currently unused.
    CancellationDetailsReasonDetailedText = 6002,

    /// LanguageUnderstandingServiceResponseJSONResult is the Language Understanding Service response output (in JSON format).
    /// Available via IntentRecognitionResult.Properties.
    LanguageUnderstandingServiceResponseJsonResult = 7000,

    /// AudioConfigDeviceNameForCapture is the device name for audio capture. Under normal circumstances, you shouldn't have
    /// to use this property directly.
    /// Instead, use AudioConfig.FromMicrophoneInput.
    AudioConfigDeviceNameForCapture = 8000,

    /// AudioConfigNumberOfChannelsForCapture is the number of channels for audio capture. Internal use only.
    AudioConfigNumberOfChannelsForCapture = 8001,

    /// AudioConfigSampleRateForCapture is the sample rate (in Hz) for audio capture. Internal use only.
    AudioConfigSampleRateForCapture = 8002,

    /// AudioConfigBitsPerSampleForCapture is the number of bits of each sample for audio capture. Internal use only.
    AudioConfigBitsPerSampleForCapture = 8003,

    /// AudioConfigAudioSource is the audio source. Allowed values are "Microphones", "File", and "Stream".
    AudioConfigAudioSource = 8004,

    // AudioConfigDeviceNameForRender indicates the device name for audio render. Under normal circumstances,
    /// you shouldn't have to use this property directly. Instead, use NewAudioConfigFromDefaultSpeakerOutput.
    /// Added in version 1.17.0
    AudioConfigDeviceNameForRender = 8005,

    /// AudioConfigPlaybackBufferLengthInMs indicates the playback buffer length in milliseconds, default is 50 milliseconds.
    AudioConfigPlaybackBufferLengthInMs = 8006,

    /// Audio processing options in JSON format.
    AudioConfigAudioProcessingOptions = 8007,

    /// SpeechLogFilename is the file name to write logs.
    SpeechLogFilename = 9001,

    /// A duration of detected silence, measured in milliseconds, after which speech-to-text will determine a spoken
    /// phrase has ended and generate a final Recognized result. Configuring this timeout may be helpful in situations
    /// where spoken input is significantly faster or slower than usual and default segmentation behavior consistently
    /// yields results that are too long or too short. Segmentation timeout values that are inappropriately high or low
    /// can negatively affect speech-to-text accuracy; this property should be carefully configured and the resulting
    /// behavior should be thoroughly validated as intended.
    ///
    /// For more information about timeout configuration that includes discussion of default behaviors, please visit
    /// https://aka.ms/csspeech/timeouts.
    SpeechSegmentationSilenceTimeoutMs = 9002,

    /// ConversationApplicationID is the identifier used to connect to the backend service.
    ConversationApplicationID = 10000,

    /// ConversationDialogType is the type of dialog backend to connect to.
    ConversationDialogType = 10001,

    /// ConversationInitialSilenceTimeout is the silence timeout for listening.
    ConversationInitialSilenceTimeout = 10002,

    /// ConversationFromID is the FromId to be used on speech recognition activities.
    ConversationFromID = 10003,

    /// ConversationConversationID is the ConversationId for the session.
    ConversationConversationID = 10004,

    /// ConversationCustomVoiceDeploymentIDs is a comma separated list of custom voice deployment ids.
    ConversationCustomVoiceDeploymentIDs = 10005,

    /// ConversationSpeechActivityTemplate is use to stamp properties in the template on the activity generated by the service for speech.
    ConversationSpeechActivityTemplate = 10006,

    /// Your participant identifier in the current conversation.
    /// Added in version 1.13.0
    ConversationParticipantId = 10007,

    // If specified as true, request that the service send MessageStatus payloads via the ActivityReceived event
    // handler. These messages communicate the outcome of ITurnContext resolution from the dialog system.
    // Added in version 1.14.0.
    ConversationRequestBotStatusMessages = 10008,

    // Additional identifying information, such as a Direct Line token, used to authenticate with the backend service.
    // Added in version 1.16.0.
    ConversationConnectionId = 10009,

    /// DataBufferTimeStamp is the time stamp associated to data buffer written by client when using Pull/Push
    /// audio input streams.
    /// The time stamp is a 64-bit value with a resolution of 90 kHz. It is the same as the presentation timestamp
    /// in an MPEG transport stream. See https:///en.wikipedia.org/wiki/Presentation_timestamp
    DataBufferTimeStamp = 11001,

    /// DataBufferUserID is the user id associated to data buffer written by client when using Pull/Push audio
    /// input streams.
    DataBufferUserID = 11002,

    /// The reference text of the audio for pronunciation evaluation.
    /// For this and the following pronunciation assessment parameters, see the table
    /// [Pronunciation assessment parameters](/azure/cognitive-services/speech-service/rest-speech-to-text-short#pronunciation-assessment-parameters).
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::Create"/> or <see cref="PronunciationAssessmentConfig::SetReferenceText"/>.
    /// Added in version 1.14.0
    PronunciationAssessmentReferenceText = 12001,

    /// The point system for pronunciation score calibration (FivePoint or HundredMark).
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::Create"/>.
    /// Added in version 1.14.0
    PronunciationAssessmentGradingSystem = 12002,

    /// The pronunciation evaluation granularity (Phoneme, Word, or FullText).
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::Create"/>.
    /// Added in version 1.14.0
    PronunciationAssessmentGranularity = 12003,

    /// Defines if enable miscue calculation.
    /// With this enabled, the pronounced words will be compared to the reference text,
    /// and will be marked with omission/insertion based on the comparison. The default setting is False.
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::Create"/>.
    /// Added in version 1.14.0
    PronunciationAssessmentEnableMiscue = 12005,

    /// The pronunciation evaluation phoneme alphabet. The valid values are "SAPI" (default) and "IPA"
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::SetPhonemeAlphabet"/>.
    /// Added in version 1.20.0
    PronunciationAssessmentPhonemeAlphabet = 12006,

    /// The pronunciation evaluation nbest phoneme count.
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::SetNBestPhonemeCount"/>.
    /// Added in version 1.20.0
    PronunciationAssessmentNBestPhonemeCount = 12007,

    /// Whether to enable prosody assessment.
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::EnableProsodyAssessment"/>.
    /// Added in version 1.33.0
    PronunciationAssessmentEnableProsodyAssessment = 12008,

    /// The json string of pronunciation assessment parameters
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::Create"/>.
    /// Added in version 1.14.0
    PronunciationAssessmentJson = 12009,

    /// Pronunciation assessment parameters.
    /// This property is intended to be read-only. The SDK is using it internally.
    /// Added in version 1.14.0
    PronunciationAssessmentParams = 12010,

    /// The content topic of the pronunciation assessment.
    /// Under normal circumstances, you shouldn't have to use this property directly.
    /// Instead, use <see cref="PronunciationAssessmentConfig::EnableContentAssessmentWithTopic"/>.
    /// Added in version 1.33.0
    PronunciationAssessmentContentTopic = 12020,

    /// Speaker Recognition backend API version.
    /// This property is added to allow testing and use of previous versions of Speaker Recognition APIs, where applicable.
    /// Added in version 1.18.0
    SpeakerRecognitionApiVersion = 13001,

    /// The name of a model to be used for speech translation.
    /// Do not use this property directly.
    /// Currently this is only valid when EmbeddedSpeechConfig is used.
    SpeechTranslationModelName = 13100,

    /// The decryption key of a model to be used for speech translation.
    /// Do not use this property directly.
    /// Currently this is only valid when EmbeddedSpeechConfig is used.
    SpeechTranslationModelKey = 13101,

    /// The name of a model to be used for keyword recognition.
    /// Do not use this property directly.
    /// Currently this is only valid when EmbeddedSpeechConfig is used.
    KeywordRecognitionModelName = 13200,

    /// The decryption key of a model to be used for keyword recognition.
    /// Do not use this property directly.
    /// Currently this is only valid when EmbeddedSpeechConfig is used.
    KeywordRecognitionModelKey = 13201,

    /// Enable the collection of embedded speech performance metrics which can
    /// be used to evaluate the capability of a device to use embedded speech.
    /// The collected data is included in results from specific scenarios like
    /// speech recognition.
    /// The default setting is "false". Note that metrics may not be available
    /// from all embedded speech scenarios.
    EmbeddedSpeechEnablePerformanceMetrics = 13300,
}

impl PropertyId {
    pub fn to_i32(&self) -> i32 {
        match self {
            PropertyId::SpeechServiceConnectionKey => 1000,
            PropertyId::SpeechServiceConnectionEndpoint => 1001,
            PropertyId::SpeechServiceConnectionRegion => 1002,
            PropertyId::SpeechServiceAuthorizationToken => 1003,
            PropertyId::SpeechServiceAuthorizationType => 1004,
            PropertyId::SpeechServiceConnectionEndpointId => 1005,
            PropertyId::SpeechServiceConnectionHost => 1006,

            PropertyId::SpeechServiceConnectionProxyHostName => 1100,
            PropertyId::SpeechServiceConnectionProxyPort => 1101,
            PropertyId::SpeechServiceConnectionProxyUserName => 1102,
            PropertyId::SpeechServiceConnectionProxyPassword => 1103,
            PropertyId::SpeechServiceConnectionURL => 1104,

            PropertyId::SpeechServiceConnectionTranslationToLanguages => 2000,
            PropertyId::SpeechServiceConnectionTranslationVoice => 2001,
            PropertyId::SpeechServiceConnectionTranslationFeatures => 2002,
            PropertyId::SpeechServiceConnectionIntentRegion => 2003,

            PropertyId::SpeechServiceConnectionRecoMode => 3000,
            PropertyId::SpeechServiceConnectionRecoLanguage => 3001,
            PropertyId::SpeechSessionId => 3002,
            PropertyId::SpeechServiceConnectionUserDefinedQueryParameters => 3003,
            PropertyId::SpeechServiceConnectionRecoModelName => 3005,
            PropertyId::SpeechServiceConnectionRecoModelKey => 3006,
            PropertyId::SpeechServiceConnectionRecoModelIniFile => 3007,

            PropertyId::SpeechServiceConnectionSynthLanguage => 3100,
            PropertyId::SpeechServiceConnectionSynthVoice => 3101,
            PropertyId::SpeechServiceConnectionSynthOutputFormat => 3102,
            PropertyId::SpeechServiceConnectionSynthEnableCompressedAudioTransmission => 3103,
            PropertyId::SpeechServiceConnectionSynthBackend => 3110,
            PropertyId::SpeechServiceConnectionSynthOfflineDataPath => 3112,
            PropertyId::SpeechServiceConnectionSynthOfflineVoice => 3113,
            PropertyId::SpeechServiceConnectionSynthModelKey => 3114,
            PropertyId::SpeechServiceConnectionVoicesListEndpoint => 3130,

            PropertyId::SpeechServiceConnectionInitialSilenceTimeoutMs => 3200,
            PropertyId::SpeechServiceConnectionEndSilenceTimeoutMs => 3201,
            PropertyId::SpeechServiceConnectionEnableAudioLogging => 3202,
            PropertyId::SpeechServiceConnectionLanguageIdMode => 3205,

            PropertyId::SpeechServiceConnectionAutoDetectSourceLanguages => 3300,
            PropertyId::SpeechServiceConnectionAutoDetectSourceLanguageResult => 3301,

            PropertyId::SpeechServiceResponseRequestDetailedResultTrueFalse => 4000,
            PropertyId::SpeechServiceResponseRequestProfanityFilterTrueFalse => 4001,
            PropertyId::SpeechServiceResponseProfanityOption => 4002,
            PropertyId::SpeechServiceResponsePostProcessingOption => 4003,
            PropertyId::SpeechServiceResponseRequestWordLevelTimestamps => 4004,
            PropertyId::SpeechServiceResponseStablePartialResultThreshold => 4005,
            PropertyId::SpeechServiceResponseOutputFormatOption => 4006,
            PropertyId::SpeechServiceResponseRequestSnr => 4007,

            PropertyId::SpeechServiceResponseTranslationRequestStablePartialResult => 4100,

            PropertyId::SpeechServiceResponseRequestWordBoundary => 4200,
            PropertyId::SpeechServiceResponseRequestPunctuationBoundary => 4201,
            PropertyId::SpeechServiceResponseRequestSentenceBoundary => 4202,
            PropertyId::SpeechServiceResponseSynthesisEventsSyncToAudio => 4210,

            PropertyId::SpeechServiceResponseJsonResult => 5000,
            PropertyId::SpeechServiceResponseJsonErrorDetails => 5001,
            PropertyId::SpeechServiceResponseRecognitionLatencyMs => 5002,
            PropertyId::SpeechServiceResponseRecognitionBackend => 5003,
            PropertyId::SpeechServiceResponseSynthesisFirstByteLatencyMs => 5010,
            PropertyId::SpeechServiceResponseSynthesisFinishLatencyMs => 5011,
            PropertyId::SpeechServiceResponseSynthesisUnderrunTimeMs => 5012,
            PropertyId::SpeechServiceResponseSynthesisConnectionLatencyMs => 5013,
            PropertyId::SpeechServiceResponseSynthesisNetworkLatencyMs => 5014,
            PropertyId::SpeechServiceResponseSynthesisServiceLatencyMs => 5015,
            PropertyId::SpeechServiceResponseSynthesisBackend => 5020,

            PropertyId::CancellationDetailsReason => 6000,
            PropertyId::CancellationDetailsReasonText => 6001,
            PropertyId::CancellationDetailsReasonDetailedText => 6002,

            PropertyId::LanguageUnderstandingServiceResponseJsonResult => 7000,

            PropertyId::AudioConfigDeviceNameForCapture => 8000,
            PropertyId::AudioConfigNumberOfChannelsForCapture => 8001,
            PropertyId::AudioConfigSampleRateForCapture => 8002,
            PropertyId::AudioConfigBitsPerSampleForCapture => 8003,
            PropertyId::AudioConfigAudioSource => 8004,
            PropertyId::AudioConfigDeviceNameForRender => 8005,
            PropertyId::AudioConfigPlaybackBufferLengthInMs => 8006,
            PropertyId::AudioConfigAudioProcessingOptions => 8007,

            PropertyId::SpeechLogFilename => 9001,
            PropertyId::SpeechSegmentationSilenceTimeoutMs => 9002,

            PropertyId::ConversationApplicationID => 10000,
            PropertyId::ConversationDialogType => 10001,
            PropertyId::ConversationInitialSilenceTimeout => 10002,
            PropertyId::ConversationFromID => 10003,
            PropertyId::ConversationConversationID => 10004,
            PropertyId::ConversationCustomVoiceDeploymentIDs => 10005,
            PropertyId::ConversationSpeechActivityTemplate => 10006,
            PropertyId::ConversationParticipantId => 10007,
            PropertyId::ConversationRequestBotStatusMessages => 10008,
            PropertyId::ConversationConnectionId => 10009,

            PropertyId::DataBufferTimeStamp => 11001,
            PropertyId::DataBufferUserID => 11002,

            PropertyId::PronunciationAssessmentReferenceText => 12001,
            PropertyId::PronunciationAssessmentGradingSystem => 12002,
            PropertyId::PronunciationAssessmentGranularity => 12003,
            PropertyId::PronunciationAssessmentEnableMiscue => 12005,
            PropertyId::PronunciationAssessmentPhonemeAlphabet => 12006,
            PropertyId::PronunciationAssessmentNBestPhonemeCount => 12007,
            PropertyId::PronunciationAssessmentEnableProsodyAssessment => 12008,
            PropertyId::PronunciationAssessmentJson => 12009,
            PropertyId::PronunciationAssessmentParams => 12010,
            PropertyId::PronunciationAssessmentContentTopic => 12020,

            PropertyId::SpeakerRecognitionApiVersion => 13001,

            PropertyId::SpeechTranslationModelName => 13100,
            PropertyId::SpeechTranslationModelKey => 13101,

            PropertyId::KeywordRecognitionModelName => 13200,
            PropertyId::KeywordRecognitionModelKey => 13201,

            PropertyId::EmbeddedSpeechEnablePerformanceMetrics => 13300,
        }
    }
}
