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

    /// The name of the offline TTS voice to be used for speech synthesis
    /// Under normal circumstances, you shouldn't use this property directly.
    /// Added in version 1.19.0
    SpeechServiceConnectionSynthOfflineVoice = 3113,

    /// The decryption key of the voice to be used for speech synthesis.
    /// Under normal circumstances, you shouldn't use this property directly.
    /// Added in version 1.19.0
    SpeechServiceConnectionSynthModelKey = 3114,

    /// SpeechServiceConnectionInitialSilenceTimeoutMs is the initial silence timeout value (in milliseconds) used by the
    /// service.
    SpeechServiceConnectionInitialSilenceTimeoutMs = 3200,

    /// SpeechServiceConnectionEndSilenceTimeoutMs is the end silence timeout value (in milliseconds) used by the service.
    SpeechServiceConnectionEndSilenceTimeoutMs = 3201,

    /// SpeechServiceConnectionEnableAudioLogging is a boolean value specifying whether audio logging is enabled in the service
    /// or not.
    SpeechServiceConnectionEnableAudioLogging = 3202,

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

    /// SpeechServiceResponseTranslationRequestStablePartialResult is a boolean value to request for stabilizing translation
    /// partial results by omitting words in the end.
    SpeechServiceResponseTranslationRequestStablePartialResult = 4100,

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

    /// SpeechLogFilename is the file name to write logs.
    SpeechLogFilename = 9001,

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

    /// DataBufferTimeStamp is the time stamp associated to data buffer written by client when using Pull/Push
    /// audio input streams.
    /// The time stamp is a 64-bit value with a resolution of 90 kHz. It is the same as the presentation timestamp
    /// in an MPEG transport stream. See https:///en.wikipedia.org/wiki/Presentation_timestamp
    DataBufferTimeStamp = 11001,

    /// DataBufferUserID is the user id associated to data buffer written by client when using Pull/Push audio
    /// input streams.
    DataBufferUserID = 11002,
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

            PropertyId::SpeechServiceConnectionSynthLanguage => 3100,
            PropertyId::SpeechServiceConnectionSynthVoice => 3101,
            PropertyId::SpeechServiceConnectionSynthOutputFormat => 3102,
            PropertyId::SpeechServiceConnectionSynthEnableCompressedAudioTransmission => 3103,
            PropertyId::SpeechServiceConnectionSynthOfflineVoice => 3113,
            PropertyId::SpeechServiceConnectionSynthModelKey => 3114,
            PropertyId::SpeechServiceConnectionInitialSilenceTimeoutMs => 3200,
            PropertyId::SpeechServiceConnectionEndSilenceTimeoutMs => 3201,
            PropertyId::SpeechServiceConnectionEnableAudioLogging => 3202,

            PropertyId::SpeechServiceResponseRequestDetailedResultTrueFalse => 4000,
            PropertyId::SpeechServiceResponseRequestProfanityFilterTrueFalse => 4001,
            PropertyId::SpeechServiceResponseProfanityOption => 4002,
            PropertyId::SpeechServiceResponsePostProcessingOption => 4003,
            PropertyId::SpeechServiceResponseRequestWordLevelTimestamps => 4004,
            PropertyId::SpeechServiceResponseStablePartialResultThreshold => 4005,
            PropertyId::SpeechServiceResponseOutputFormatOption => 4006,
            PropertyId::SpeechServiceResponseTranslationRequestStablePartialResult => 4100,

            PropertyId::SpeechServiceResponseJsonResult => 5000,
            PropertyId::SpeechServiceResponseJsonErrorDetails => 5001,
            PropertyId::SpeechServiceResponseRecognitionLatencyMs => 5002,

            PropertyId::SpeechServiceResponseSynthesisFirstByteLatencyMs => 5010,
            PropertyId::SpeechServiceResponseSynthesisFinishLatencyMs => 5011,
            PropertyId::SpeechServiceResponseSynthesisUnderrunTimeMs => 5012,
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
        }
    }
}
