use crate::common::{
    OutputFormat, ProfanityOption, PropertyCollection, PropertyId, ServicePropertyChannel,
};
use crate::error::{convert_err, Result};
use crate::ffi::{
    speech_config_from_authorization_token, speech_config_from_endpoint, speech_config_from_host,
    speech_config_from_subscription, speech_config_get_property_bag, speech_config_release,
    speech_config_set_profanity, speech_config_set_service_property, SmartHandle, SPXHANDLE,
    SPXSPEECHCONFIGHANDLE,
};
use crate::speech::EmbeddedSpeechConfig;
use std::ffi::CString;
use std::mem::MaybeUninit;

/// SpeechConfig is the class that defines configurations for speech / intent recognition, or speech synthesis.
#[derive(Debug)]
pub struct SpeechConfig {
    pub handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    properties: PropertyCollection,
}

impl From<EmbeddedSpeechConfig> for SpeechConfig {
    fn from(esc: EmbeddedSpeechConfig) -> SpeechConfig {
        esc.config
    }
}

impl SpeechConfig {
    /// Creates a SpeechConfig instance from a valid handle. This is for internal use only.
    pub fn from_handle(handle: SPXHANDLE) -> Result<SpeechConfig> {
        unsafe {
            let mut prop_bag_handle = MaybeUninit::uninit();
            let ret = speech_config_get_property_bag(handle, prop_bag_handle.as_mut_ptr());
            convert_err(ret, "SpeechConfig::from_handle error")?;

            let mut property_bag = PropertyCollection::from_handle(prop_bag_handle.assume_init());

            property_bag
                .set_property_by_string("SPEECHSDK-SPEECH-CONFIG-SYSTEM-LANGUAGE", "Rust")?;

            let result = SpeechConfig {
                handle: SmartHandle::create("SpeechConfig", handle, speech_config_release),
                properties: property_bag,
            };
            Ok(result)
        }
    }

    /// Creates an instance of the speech config with specified subscription key and region.
    pub fn from_subscription<S>(subscription: S, region: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_sub = CString::new(subscription)?;
        let c_region = CString::new(region)?;

        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = speech_config_from_subscription(
                handle.as_mut_ptr(),
                c_sub.as_ptr(),
                c_region.as_ptr(),
            );
            convert_err(ret, "SpeechConfig::from_subscription error")?;
            SpeechConfig::from_handle(handle.assume_init())
        }
    }

    /// Creates an instance of the speech config with specified authorization token and
    /// region.
    /// Note: The caller needs to ensure that the authorization token is valid. Before the authorization token expires, the
    /// caller needs to refresh it by calling this setter with a new valid token.
    /// As configuration values are copied when creating a new recognizer, the new token value will not apply to recognizers
    /// that have already been created.
    /// For recognizers that have been created before, you need to set authorization token of the corresponding recognizer
    /// to refresh the token. Otherwise, the recognizers will encounter errors during recognition.
    pub fn from_auth_token<S>(auth_token: S, region: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_auth_token = CString::new(auth_token)?;
        let c_region = CString::new(region)?;

        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = speech_config_from_authorization_token(
                handle.as_mut_ptr(),
                c_auth_token.as_ptr(),
                c_region.as_ptr(),
            );
            convert_err(ret, "SpeechConfig::from_auth_token error")?;
            SpeechConfig::from_handle(handle.assume_init())
        }
    }

    // Creates an instance of the speech config with specified endpoint
    /// and subscription.
    /// This method is intended only for users who use a non-standard service endpoint.
    /// Note: The query parameters specified in the endpoint URI are not changed, even if they are set by any other APIs.
    /// For example, if the recognition language is defined in URI as query parameter "language=de-DE", and also set by
    /// SetSpeechRecognitionLanguage("en-US"), the language setting in URI takes precedence, and the effective language
    /// is "de-DE".
    /// Only the parameters that are not specified in the endpoint URI can be set by other APIs.
    /// Note: To use an authorization token with endoint, use FromEndpoint,
    /// and then call SetAuthorizationToken() on the created SpeechConfig instance.
    pub fn from_endpoint_with_subscription<S>(endpoint: S, subscription: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_endpoint = CString::new(endpoint)?;
        let c_subscription = CString::new(subscription)?;

        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = speech_config_from_endpoint(
                handle.as_mut_ptr(),
                c_endpoint.as_ptr(),
                c_subscription.as_ptr(),
            );
            convert_err(ret, "SpeechConfig::from_endpoint_with_subscription error")?;
            SpeechConfig::from_handle(handle.assume_init())
        }
    }

    /// Creates an instance of SpeechConfig with specified endpoint.
    /// This method is intended only for users who use a non-standard service endpoint.
    /// Note: The query parameters specified in the endpoint URI are not changed, even if they are set by any other APIs.
    /// For example, if the recognition language is defined in URI as query parameter "language=de-DE", and also set by
    /// SetSpeechRecognitionLanguage("en-US"), the language setting in URI takes precedence, and the effective language is
    /// "de-DE".
    /// Only the parameters that are not specified in the endpoint URI can be set by other APIs.
    /// Note: If the endpoint requires a subscription key for authentication, use NewSpeechConfigFromEndpointWithSubscription
    /// to pass the subscription key as parameter.
    /// To use an authorization token with FromEndpoint, use this method to create a SpeechConfig instance, and then
    /// call SetAuthorizationToken() on the created SpeechConfig instance.
    pub fn from_endpoint<S>(endpoint: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_endpoint = CString::new(endpoint)?;

        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = speech_config_from_endpoint(
                handle.as_mut_ptr(),
                c_endpoint.as_ptr(),
                std::ptr::null(),
            );
            convert_err(ret, "SpeechConfig::from_endpoint error")?;
            SpeechConfig::from_handle(handle.assume_init())
        }
    }

    /// Creates an instance of the speech config with specified host and subscription.
    /// This method is intended only for users who use a non-default service host. Standard resource path will be assumed.
    /// For services with a non-standard resource path or no path at all, use FromEndpoint instead.
    /// Note: Query parameters are not allowed in the host URI and must be set by other APIs.
    /// Note: To use an authorization token with host, use NewSpeechConfigFromHost,
    /// and then call SetAuthorizationToken() on the created SpeechConfig instance.
    pub fn from_host_with_subscription<S>(host: S, subscription: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_host = CString::new(host)?;
        let c_subscription = CString::new(subscription)?;

        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = speech_config_from_host(
                handle.as_mut_ptr(),
                c_host.as_ptr(),
                c_subscription.as_ptr(),
            );
            convert_err(ret, "SpeechConfig::from_host_with_subscription error")?;
            SpeechConfig::from_handle(handle.assume_init())
        }
    }

    /// Creates an instance of SpeechConfig with specified host.
    /// This method is intended only for users who use a non-default service host. Standard resource path will be assumed.
    /// For services with a non-standard resource path or no path at all, use FromEndpoint instead.
    /// Note: Query parameters are not allowed in the host URI and must be set by other APIs.
    /// Note: If the host requires a subscription key for authentication, use NewSpeechConfigFromHostWithSubscription to pass
    /// the subscription key as parameter.
    /// To use an authorization token with FromHost, use this method to create a SpeechConfig instance, and then
    /// call SetAuthorizationToken() on the created SpeechConfig instance.    
    pub fn from_host<S>(host: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_host = CString::new(host)?;

        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret =
                speech_config_from_host(handle.as_mut_ptr(), c_host.as_ptr(), std::ptr::null());
            convert_err(ret, "SpeechConfig::from_host error")?;
            SpeechConfig::from_handle(handle.assume_init())
        }
    }

    /// Sets proxy configuration
    /// Note: Proxy functionality is not available on macOS. This function will have no effect on this platform.
    pub fn set_proxy(&mut self, hostname: String, port: u64) -> Result<()> {
        self.set_property(PropertyId::SpeechServiceConnectionProxyHostName, hostname)?;
        self.set_property(
            PropertyId::SpeechServiceConnectionProxyPort,
            port.to_string(),
        )
    }

    /// Sets proxy configuration with username and password
    ///  Note: Proxy functionality is not available on macOS. This function will have no effect on this platform.
    pub fn set_proxy_with_usrname_and_pwd(
        &mut self,
        hostname: String,
        port: u64,
        username: String,
        password: String,
    ) -> Result<()> {
        self.set_proxy(hostname, port)?;
        self.set_property(PropertyId::SpeechServiceConnectionProxyUserName, username)?;
        self.set_property(PropertyId::SpeechServiceConnectionProxyPassword, password)
    }

    pub fn set_service_property(
        &mut self,
        name: String,
        value: String,
        channel: ServicePropertyChannel,
    ) -> Result<()> {
        unsafe {
            let c_name = CString::new(name)?;
            let c_value = CString::new(value)?;
            #[cfg(target_os = "windows")]
            let ret = speech_config_set_service_property(
                self.handle.inner(),
                c_name.as_ptr(),
                c_value.as_ptr(),
                channel as i32,
            );
            #[cfg(not(target_os = "windows"))]
            let ret = speech_config_set_service_property(
                self.handle.inner(),
                c_name.as_ptr(),
                c_value.as_ptr(),
                channel as u32,
            );

            convert_err(ret, "SpeechConfig.set_service_property error")?;
            Ok(())
        }
    }

    pub fn set_profanity_option(&mut self, profanity_option: ProfanityOption) -> Result<()> {
        unsafe {
            #[cfg(target_os = "windows")]
            let ret = speech_config_set_profanity(self.handle.inner(), profanity_option as i32);
            #[cfg(not(target_os = "windows"))]
            let ret = speech_config_set_profanity(self.handle.inner(), profanity_option as u32);

            convert_err(ret, "SpeechConfig.set_profanity_option error")?;
            Ok(())
        }
    }

    pub fn enable_audio_logging(&mut self) -> Result<()> {
        self.set_property(
            PropertyId::SpeechServiceConnectionEnableAudioLogging,
            "true".into(),
        )
    }

    /// Includes word-level timestamps in response result.
    pub fn request_word_level_timestamps(&mut self) -> Result<()> {
        self.set_property(
            PropertyId::SpeechServiceResponseRequestWordLevelTimestamps,
            "true".into(),
        )
    }

    /// Enables dictation mode. Only supported in speech continuous recognition.
    pub fn enable_dictation(&mut self) -> Result<()> {
        self.set_property(
            PropertyId::SpeechServiceConnectionRecoMode,
            "DICTATION".into(),
        )
    }

    pub fn set_property(&mut self, id: PropertyId, value: String) -> Result<()> {
        self.properties.set_property(id, value)
    }

    pub fn get_property(&self, id: PropertyId) -> Result<String> {
        self.properties.get_property(id, "")
    }

    pub fn set_property_by_string(&mut self, name: String, value: String) -> Result<()> {
        self.properties.set_property_by_string(name, value)
    }

    pub fn get_property_by_string(&self, name: String) -> Result<String> {
        self.properties.get_property_by_string(name, "".into())
    }

    /// Subscription key that is used to create Speech Recognizer or Intent Recognizer or Translation
    /// Recognizer or Speech Synthesizer
    pub fn get_subscription_key(&self) -> Result<String> {
        self.get_property(PropertyId::SpeechServiceConnectionKey)
    }

    /// Region key that used to create Speech Recognizer or Intent Recognizer or Translation Recognizer or
    /// Speech Synthesizer.
    pub fn get_region(&self) -> Result<String> {
        self.get_property(PropertyId::SpeechServiceConnectionRegion)
    }

    /// Authorization token to connect to the service.
    pub fn get_auth_token(&self) -> Result<String> {
        self.get_property(PropertyId::SpeechServiceAuthorizationToken)
    }

    /// Sets the authorization token to connect to the service.
    /// Note: The caller needs to ensure that the authorization token is valid. Before the authorization token
    /// expires, the caller needs to refresh it by calling this setter with a new valid token.
    /// As configuration values are copied when creating a new recognizer, the new token value will not apply to
    /// recognizers that have already been created.
    /// For recognizers that have been created before, you need to set authorization token of the corresponding recognizer
    /// to refresh the token. Otherwise, the recognizers will encounter errors during recognition.
    pub fn set_auth_token(&mut self, auth_token: String) -> Result<()> {
        self.set_property(PropertyId::SpeechServiceAuthorizationToken, auth_token)
    }

    /// Gets input language to the speech recognition.
    /// The language is specified in BCP-47 format.
    pub fn get_speech_recognition_language(&self) -> Result<String> {
        self.get_property(PropertyId::SpeechServiceConnectionRecoLanguage)
    }

    pub fn set_speech_recognition_language(&mut self, reco_lang: String) -> Result<()> {
        self.set_property(PropertyId::SpeechServiceConnectionRecoLanguage, reco_lang)
    }

    pub fn get_output_format(&self) -> Result<OutputFormat> {
        let output_format =
            self.get_property(PropertyId::SpeechServiceResponseRequestDetailedResultTrueFalse)?;
        match output_format.as_str() {
            "true" => Ok(OutputFormat::Detailed),
            _ => Ok(OutputFormat::Simple),
        }
    }

    // Sets the speech synthesis output format (e.g. Riff16Khz16BitMonoPcm).
    pub fn set_get_output_format(&mut self, output_format: OutputFormat) -> Result<()> {
        match output_format {
            OutputFormat::Simple => self.set_property(
                PropertyId::SpeechServiceResponseRequestDetailedResultTrueFalse,
                "false".into(),
            ),
            OutputFormat::Detailed => self.set_property(
                PropertyId::SpeechServiceResponseRequestDetailedResultTrueFalse,
                "true".into(),
            ),
        }
    }

    pub fn get_endpoint_id(&self) -> Result<String> {
        self.get_property(PropertyId::SpeechServiceConnectionEndpointId)
    }

    pub fn set_endpoint_id(&mut self, endpoint_id: String) -> Result<()> {
        self.set_property(PropertyId::SpeechServiceConnectionEndpointId, endpoint_id)
    }

    pub fn get_speech_synthesis_language(&self) -> Result<String> {
        self.get_property(PropertyId::SpeechServiceConnectionSynthLanguage)
    }

    pub fn set_get_speech_synthesis_language(
        &mut self,
        speech_synthesis_language: String,
    ) -> Result<()> {
        self.set_property(
            PropertyId::SpeechServiceConnectionSynthLanguage,
            speech_synthesis_language,
        )
    }

    pub fn get_speech_synthesis_voice_name(&self) -> Result<String> {
        self.get_property(PropertyId::SpeechServiceConnectionSynthVoice)
    }

    pub fn set_get_speech_synthesis_voice_name(
        &mut self,
        speech_synthesis_voice_name: String,
    ) -> Result<()> {
        self.set_property(
            PropertyId::SpeechServiceConnectionSynthVoice,
            speech_synthesis_voice_name,
        )
    }

    pub fn get_speech_synthesis_output_format(&self) -> Result<String> {
        self.get_property(PropertyId::SpeechServiceConnectionSynthOutputFormat)
    }

    pub fn set_get_speech_synthesis_output_format(
        &mut self,
        speech_synthesis_output_format: String,
    ) -> Result<()> {
        self.set_property(
            PropertyId::SpeechServiceConnectionSynthOutputFormat,
            speech_synthesis_output_format,
        )
    }
}
