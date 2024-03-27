use crate::audio::AudioConfig;
use crate::common::{PropertyCollection, PropertyId};
use crate::error::{convert_err, Result};
use crate::ffi::{
    recognizer_async_handle_release, recognizer_canceled_set_callback,
    recognizer_create_speech_recognizer_from_auto_detect_source_lang_config,
    recognizer_create_speech_recognizer_from_config,
    recognizer_create_speech_recognizer_from_source_lang_config, recognizer_get_property_bag,
    recognizer_handle_release, recognizer_recognize_once, recognizer_recognized_set_callback,
    recognizer_recognizing_set_callback, recognizer_session_started_set_callback,
    recognizer_session_stopped_set_callback, recognizer_speech_end_detected_set_callback,
    recognizer_speech_start_detected_set_callback, recognizer_start_continuous_recognition_async,
    recognizer_start_continuous_recognition_async_wait_for,
    recognizer_start_keyword_recognition_async,
    recognizer_start_keyword_recognition_async_wait_for,
    recognizer_stop_continuous_recognition_async,
    recognizer_stop_continuous_recognition_async_wait_for,
    recognizer_stop_keyword_recognition_async, recognizer_stop_keyword_recognition_async_wait_for,
    SmartHandle, SPXASYNCHANDLE, SPXEVENTHANDLE, SPXRECOHANDLE,
};
use crate::speech::{
    AutoDetectSourceLanguageConfig, EmbeddedSpeechConfig, KeywordRecognitionModel,
    RecognitionEvent, SessionEvent, SourceLanguageConfig, SpeechConfig,
    SpeechRecognitionCanceledEvent, SpeechRecognitionEvent, SpeechRecognitionResult,
};
use log::*;
use std::boxed::Box;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::c_void;

/// SpeechRecognizer struct holds functionality for speech-to-text recognition.
pub struct SpeechRecognizer {
    handle: SmartHandle<SPXRECOHANDLE>,
    properties: PropertyCollection,
    handle_async_start_continuous: Option<SmartHandle<SPXASYNCHANDLE>>,
    handle_async_stop_continuous: Option<SmartHandle<SPXASYNCHANDLE>>,
    handle_async_start_keyword: Option<SmartHandle<SPXASYNCHANDLE>>,
    handle_async_stop_keyword: Option<SmartHandle<SPXASYNCHANDLE>>,
    session_started_cb: Option<Box<dyn Fn(SessionEvent) + Send>>,
    session_stopped_cb: Option<Box<dyn Fn(SessionEvent) + Send>>,
    speech_start_detected_cb: Option<Box<dyn Fn(RecognitionEvent) + Send>>,
    speech_end_detected_cb: Option<Box<dyn Fn(RecognitionEvent) + Send>>,
    canceled_cb: Option<Box<dyn Fn(SpeechRecognitionCanceledEvent) + Send>>,
    recognizing_cb: Option<Box<dyn Fn(SpeechRecognitionEvent) + Send>>,
    recognized_cb: Option<Box<dyn Fn(SpeechRecognitionEvent) + Send>>,
}

impl fmt::Debug for SpeechRecognizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpeechRecognizer")
            .field("handle", &self.handle)
            .field("properties", &self.properties)
            .finish()
    }
}

impl SpeechRecognizer {
    fn from_handle(handle: SPXRECOHANDLE) -> Result<SpeechRecognizer> {
        unsafe {
            let mut prop_bag_handle = MaybeUninit::uninit();
            let ret = recognizer_get_property_bag(handle, prop_bag_handle.as_mut_ptr());
            convert_err(ret, "SpeechRecognizer::from_handle error")?;

            let property_bag = PropertyCollection::from_handle(prop_bag_handle.assume_init());

            let result = SpeechRecognizer {
                handle: SmartHandle::create("SpeechRecognizer", handle, recognizer_handle_release),
                properties: property_bag,
                handle_async_start_continuous: None,
                handle_async_stop_continuous: None,
                handle_async_start_keyword: None,
                handle_async_stop_keyword: None,
                session_started_cb: None,
                session_stopped_cb: None,
                speech_start_detected_cb: None,
                speech_end_detected_cb: None,
                canceled_cb: None,
                recognizing_cb: None,
                recognized_cb: None,
            };
            Ok(result)
        }
    }

    pub fn from_config(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            convert_err(
                recognizer_create_speech_recognizer_from_config(
                    handle.as_mut_ptr(),
                    speech_config.handle.inner(),
                    audio_config.handle.inner(),
                ),
                "SpeechRecognizer.from_config error",
            )?;
            SpeechRecognizer::from_handle(handle.assume_init())
        }
    }

    pub fn from_embedded_config(
        speech_config: EmbeddedSpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        SpeechRecognizer::from_config(speech_config.into(), audio_config)
    }

    pub fn from_auto_detect_source_lang_config(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
        lang_config: AutoDetectSourceLanguageConfig,
    ) -> Result<SpeechRecognizer> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            convert_err(
                recognizer_create_speech_recognizer_from_auto_detect_source_lang_config(
                    handle.as_mut_ptr(),
                    speech_config.handle.inner(),
                    lang_config.handle.inner(),
                    audio_config.handle.inner(),
                ),
                "SpeechRecognizer.from_auto_detect_source_lang_config error",
            )?;
            SpeechRecognizer::from_handle(handle.assume_init())
        }
    }

    pub fn from_source_lang_config(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
        source_lang_config: SourceLanguageConfig,
    ) -> Result<SpeechRecognizer> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            convert_err(
                recognizer_create_speech_recognizer_from_source_lang_config(
                    handle.as_mut_ptr(),
                    speech_config.handle.inner(),
                    source_lang_config.handle.inner(),
                    audio_config.handle.inner(),
                ),
                "SpeechRecognizer.from_source_lang_config error",
            )?;
            SpeechRecognizer::from_handle(handle.assume_init())
        }
    }

    pub fn from_source_lang(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
        source_lang: &str,
    ) -> Result<SpeechRecognizer> {
        let source_lang_config = SourceLanguageConfig::from_language(source_lang)?;
        SpeechRecognizer::from_source_lang_config(speech_config, audio_config, source_lang_config)
    }

    pub fn set_session_started_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SessionEvent) + 'static + Send,
    {
        self.session_started_cb = Some(Box::new(f));
        unsafe {
            let ret = recognizer_session_started_set_callback(
                self.handle.inner(),
                Some(Self::cb_session_started),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechRecognizer.set_session_started_cb error")?;
            Ok(())
        }
    }

    pub fn set_session_stopped_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SessionEvent) + 'static + Send,
    {
        self.session_stopped_cb = Some(Box::new(f));
        unsafe {
            let ret = recognizer_session_stopped_set_callback(
                self.handle.inner(),
                Some(Self::cb_session_stopped),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechRecognizer.set_session_stopped_cb error")?;
            Ok(())
        }
    }

    pub fn set_speech_start_detected_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(RecognitionEvent) + 'static + Send,
    {
        self.speech_start_detected_cb = Some(Box::new(f));
        unsafe {
            let ret = recognizer_speech_start_detected_set_callback(
                self.handle.inner(),
                Some(Self::cb_speech_start_detected),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechRecognizer.set_speech_start_detected_cb error")?;
            Ok(())
        }
    }

    pub fn set_speech_end_detected_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(RecognitionEvent) + 'static + Send,
    {
        self.speech_end_detected_cb = Some(Box::new(f));
        unsafe {
            let ret = recognizer_speech_end_detected_set_callback(
                self.handle.inner(),
                Some(Self::cb_speech_end_detected),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechRecognizer.set_speech_end_detected_cb error")?;
            Ok(())
        }
    }

    /// Canceled signals for events containing canceled recognition results
    /// (indicating a recognition attempt that was canceled as a result or a direct cancellation request
    /// or, alternatively, a transport or protocol failure).
    pub fn set_canceled_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechRecognitionCanceledEvent) + 'static + Send,
    {
        self.canceled_cb = Some(Box::new(f));
        unsafe {
            let ret = recognizer_canceled_set_callback(
                self.handle.inner(),
                Some(Self::cb_canceled),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechRecognizer.set_canceled_cb error")?;
            Ok(())
        }
    }

    pub fn set_recognizing_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechRecognitionEvent) + 'static + Send,
    {
        self.recognizing_cb = Some(Box::new(f));
        unsafe {
            trace!("calling recognizer_recognizing_set_callback");
            let ret = recognizer_recognizing_set_callback(
                self.handle.inner(),
                Some(Self::cb_recognizing),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechRecognizer.set_recognizing_cb error")?;
            trace!("called recognizer_recognizing_set_callback");
            Ok(())
        }
    }

    pub fn set_recognized_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechRecognitionEvent) + 'static + Send,
    {
        self.recognized_cb = Some(Box::new(f));
        unsafe {
            trace!("calling recognizer_recognized_set_callback");
            let ret = recognizer_recognized_set_callback(
                self.handle.inner(),
                Some(Self::cb_recognized),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechRecognizer.set_recognized_cb error")?;
            trace!("called recognizer_recognized_set_callback");
            Ok(())
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_session_started(
        hreco: SPXRECOHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechRecognizer::cb_session_started called");
        let speech_recognizer = &mut *(pvContext as *mut SpeechRecognizer);
        trace!("speech_recognizer {:?}", speech_recognizer);
        if let Some(cb) = &speech_recognizer.session_started_cb {
            trace!("session_started_cb defined");
            match SessionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("SpeechRecognizer::cb_session_started error {:?}", err);
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_session_stopped(
        hreco: SPXRECOHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechRecognizer::cb_session_stopped called");
        let speech_recognizer = &mut *(pvContext as *mut SpeechRecognizer);
        if let Some(cb) = &speech_recognizer.session_stopped_cb {
            trace!("cb_session_stopped defined");
            match SessionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("SpeechRecognizer::cb_session_stopped error {:?}", err);
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_speech_start_detected(
        hreco: SPXRECOHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechRecognizer::cb_speech_start_detected called");
        let speech_recognizer = &mut *(pvContext as *mut SpeechRecognizer);
        trace!("speech_recognizer {:?}", speech_recognizer);
        if let Some(cb) = &speech_recognizer.speech_start_detected_cb {
            trace!("speech_start_detected_cb defined");
            match RecognitionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("SpeechRecognizer::cb_speech_start_detected error {:?}", err);
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_speech_end_detected(
        hreco: SPXRECOHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechRecognizer::cb_speech_end_detected called");
        let speech_recognizer = &mut *(pvContext as *mut SpeechRecognizer);
        if let Some(cb) = &speech_recognizer.speech_end_detected_cb {
            trace!("speech_end_detected_cb defined");
            match RecognitionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("SpeechRecognizer::cb_speech_end_detected error {:?}", err);
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_canceled(
        hreco: SPXRECOHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechRecognizer::cb_canceled called");
        let speech_recognizer = &mut *(pvContext as *mut SpeechRecognizer);
        if let Some(cb) = &speech_recognizer.canceled_cb {
            trace!("canceled_cb defined");
            match SpeechRecognitionCanceledEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("SpeechRecognizer::cb_canceled error {:?}", err);
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_recognizing(
        hreco: SPXRECOHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechRecognizer::cb_recognizing called");
        let speech_recognizer = &mut *(pvContext as *mut SpeechRecognizer);
        trace!("speech_recognizer {:?}", speech_recognizer);
        if let Some(cb) = &speech_recognizer.recognizing_cb {
            trace!("recognizing_cb defined");
            match SpeechRecognitionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("SpeechRecognizer::cb_recognizing error {:?}", err);
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_recognized(
        hreco: SPXRECOHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechRecognizer::cb_recognized called");
        let speech_recognizer = &mut *(pvContext as *mut SpeechRecognizer);
        if let Some(cb) = &speech_recognizer.recognized_cb {
            trace!("recognized_cb defined");
            match SpeechRecognitionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("SpeechRecognizer::cb_recognized error {:?}", err);
                }
            }
        }
    }

    /// Starts speech recognition, and returns after a single utterance is recognized.
    /// The end of a single utterance is determined by listening for silence at the end or until a maximum
    /// of 15 seconds of audio is processed.  The task returns the recognition text as result.
    /// Note: Since RecognizeOnceAsync() returns only a single utterance, it is suitable only for single
    /// shot recognition like command or query.
    /// For long-running multi-utterance recognition, use StartContinuousRecognitionAsync() instead.
    pub async fn recognize_once_async(&mut self) -> Result<SpeechRecognitionResult> {
        unsafe {
            let mut handle_result = MaybeUninit::uninit();
            let ret = recognizer_recognize_once(self.handle.inner(), handle_result.as_mut_ptr());
            convert_err(ret, "SpeechRecognizer.recognize_once_async error")?;
            SpeechRecognitionResult::from_handle(handle_result.assume_init())
        }
    }

    /// Asynchronously initiates continuous speech recognition operation.
    pub async fn start_continuous_recognition_async(&mut self) -> Result<()> {
        unsafe {
            let mut handle_async_start_continuous = MaybeUninit::uninit();
            let mut ret = recognizer_start_continuous_recognition_async(
                self.handle.inner(),
                handle_async_start_continuous.as_mut_ptr(),
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_start_continuous_recognition_async error",
            )?;
            let handle_async_start_continuous = handle_async_start_continuous.assume_init();
            self.handle_async_start_continuous = Some(SmartHandle::create(
                "handle_async_start_continuous",
                handle_async_start_continuous,
                recognizer_async_handle_release,
            ));

            ret = recognizer_start_continuous_recognition_async_wait_for(
                handle_async_start_continuous,
                u32::MAX,
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_start_continuous_recognition_async_wait_for error",
            )?;
        }
        Ok(())
    }

    /// Asynchronously terminates ongoing continuous speech recognition operation.
    pub async fn stop_continuous_recognition_async(&mut self) -> Result<()> {
        unsafe {
            let mut handle_async_stop_continuous = MaybeUninit::uninit();
            let mut ret = recognizer_stop_continuous_recognition_async(
                self.handle.inner(),
                handle_async_stop_continuous.as_mut_ptr(),
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_stop_continuous_recognition_async error",
            )?;
            let handle_async_stop_continuous = handle_async_stop_continuous.assume_init();
            self.handle_async_stop_continuous = Some(SmartHandle::create(
                "handle_async_stop_continuous",
                handle_async_stop_continuous,
                recognizer_async_handle_release,
            ));

            ret = recognizer_stop_continuous_recognition_async_wait_for(
                handle_async_stop_continuous,
                u32::MAX,
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_stop_continuous_recognition_async_wait_for error",
            )?;
        }
        Ok(())
    }

    /// Asynchronously initiates keyword recognition operation.
    pub async fn start_keyword_recognition_async(
        &mut self,
        model: KeywordRecognitionModel,
    ) -> Result<()> {
        unsafe {
            let mut handle_async_start_keyword = MaybeUninit::uninit();
            let mut ret = recognizer_start_keyword_recognition_async(
                self.handle.inner(),
                model.handle.inner(),
                handle_async_start_keyword.as_mut_ptr(),
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_start_keyword_recognition_async error",
            )?;

            let handle_async_start_keyword = handle_async_start_keyword.assume_init();
            self.handle_async_start_keyword = Some(SmartHandle::create(
                "handle_async_start_keyword",
                handle_async_start_keyword,
                recognizer_async_handle_release,
            ));

            ret = recognizer_start_keyword_recognition_async_wait_for(
                handle_async_start_keyword,
                u32::MAX,
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_start_keyword_recognition_async_wait_for error",
            )?;
        }
        Ok(())
    }

    /// Asynchronously terminates keyword recognition operation.
    pub async fn stop_keyword_recognition_async(&mut self) -> Result<()> {
        unsafe {
            let mut handle_async_stop_keyword = MaybeUninit::uninit();
            let mut ret = recognizer_stop_keyword_recognition_async(
                self.handle.inner(),
                handle_async_stop_keyword.as_mut_ptr(),
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_stop_keyword_recognition_async error",
            )?;
            let handle_async_stop_keyword = handle_async_stop_keyword.assume_init();
            self.handle_async_stop_keyword = Some(SmartHandle::create(
                "handle_async_stop_keyword",
                handle_async_stop_keyword,
                recognizer_async_handle_release,
            ));

            ret = recognizer_stop_keyword_recognition_async_wait_for(
                handle_async_stop_keyword,
                u32::MAX,
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_stop_keyword_recognition_async_wait_for error",
            )?;
        }
        Ok(())
    }

    pub fn get_endpoint_id(&self) -> Result<String> {
        self.properties
            .get_property(PropertyId::SpeechServiceConnectionEndpointId, "")
    }

    pub fn get_auth_token(&self) -> Result<String> {
        self.properties
            .get_property(PropertyId::SpeechServiceAuthorizationToken, "")
    }

    /// Sets the authorization token that will be used for connecting to the service.
    /// Note: The caller needs to ensure that the authorization token is valid. Before the authorization token
    /// expires, the caller needs to refresh it by calling this setter with a new valid token.
    /// Otherwise, the recognizer will encounter errors during recognition.
    pub fn set_auth_token(&mut self, token: &str) -> Result<()> {
        self.properties
            .set_property(PropertyId::SpeechServiceAuthorizationToken, token)
    }
}
