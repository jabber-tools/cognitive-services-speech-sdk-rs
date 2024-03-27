use crate::audio::AudioConfig;
use crate::common::{PropertyCollection, PropertyId};
use crate::error::{convert_err, Result};
use crate::ffi::{
    synthesizer_bookmark_reached_set_callback, synthesizer_canceled_set_callback,
    synthesizer_completed_set_callback,
    synthesizer_create_speech_synthesizer_from_auto_detect_source_lang_config,
    synthesizer_create_speech_synthesizer_from_config, synthesizer_get_property_bag,
    synthesizer_get_voices_list, synthesizer_handle_release, synthesizer_speak_ssml,
    synthesizer_speak_text, synthesizer_start_speaking_ssml, synthesizer_start_speaking_text,
    synthesizer_started_set_callback, synthesizer_stop_speaking,
    synthesizer_synthesizing_set_callback, synthesizer_viseme_received_set_callback,
    synthesizer_word_boundary_set_callback, SmartHandle, SPXEVENTHANDLE, SPXSYNTHHANDLE,
};
use crate::speech::{
    AutoDetectSourceLanguageConfig, SpeechConfig, SpeechSynthesisBookmarkEvent,
    SpeechSynthesisEvent, SpeechSynthesisResult, SpeechSynthesisVisemeEvent,
    SpeechSynthesisWordBoundaryEvent, SynthesisVoicesResult,
};
use log::*;
use std::boxed::Box;
use std::ffi::CString;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::c_void;

/// SpeechSynthesizer struct holds functionality for text-to-speech synthesis.
pub struct SpeechSynthesizer {
    handle: SmartHandle<SPXSYNTHHANDLE>,
    properties: PropertyCollection,
    synthesizer_started_cb: Option<Box<dyn Fn(SpeechSynthesisEvent) + Send>>,
    synthesizer_synthesizing_cb: Option<Box<dyn Fn(SpeechSynthesisEvent) + Send>>,
    synthesizer_completed_cb: Option<Box<dyn Fn(SpeechSynthesisEvent) + Send>>,
    synthesizer_canceled_cb: Option<Box<dyn Fn(SpeechSynthesisEvent) + Send>>,
    synthesizer_word_boundary_cb: Option<Box<dyn Fn(SpeechSynthesisWordBoundaryEvent) + Send>>,
    synthesizer_viseme_cb: Option<Box<dyn Fn(SpeechSynthesisVisemeEvent) + Send>>,
    synthesizer_bookmark_cb: Option<Box<dyn Fn(SpeechSynthesisBookmarkEvent) + Send>>,
}

// to allow to move synthetizer to tokio::spawn
// see example speak_text_async_2
// not sure why it is needed, for recognizer
// and dialog connector this is not needed.
// TBD: find the root cause
unsafe impl Sync for SpeechSynthesizer {}

impl fmt::Debug for SpeechSynthesizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpeechSynthesizer")
            .field("handle", &self.handle)
            .field("properties", &self.properties)
            .finish()
    }
}

impl SpeechSynthesizer {
    fn from_handle(handle: SPXSYNTHHANDLE) -> Result<Self> {
        unsafe {
            let mut prop_bag_handle = MaybeUninit::uninit();
            let ret = synthesizer_get_property_bag(handle, prop_bag_handle.as_mut_ptr());
            convert_err(ret, "SpeechSynthesizer::from_handle error")?;

            let property_bag = PropertyCollection::from_handle(prop_bag_handle.assume_init());
            Ok(SpeechSynthesizer {
                handle: SmartHandle::create(
                    "SpeechSynthesizer",
                    handle,
                    synthesizer_handle_release,
                ),
                properties: property_bag,
                synthesizer_started_cb: None,
                synthesizer_synthesizing_cb: None,
                synthesizer_completed_cb: None,
                synthesizer_canceled_cb: None,
                synthesizer_word_boundary_cb: None,
                synthesizer_viseme_cb: None,
                synthesizer_bookmark_cb: None,
            })
        }
    }

    pub fn from_config(speech_config: SpeechConfig, audio_config: AudioConfig) -> Result<Self> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            convert_err(
                synthesizer_create_speech_synthesizer_from_config(
                    handle.as_mut_ptr(),
                    speech_config.handle.inner(),
                    audio_config.handle.inner(),
                ),
                "SpeechSynthesizer.from_config error",
            )?;
            SpeechSynthesizer::from_handle(handle.assume_init())
        }
    }

    pub fn from_auto_detect_source_lang_config(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
        lang_config: AutoDetectSourceLanguageConfig,
    ) -> Result<Self> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            convert_err(
                synthesizer_create_speech_synthesizer_from_auto_detect_source_lang_config(
                    handle.as_mut_ptr(),
                    speech_config.handle.inner(),
                    lang_config.handle.inner(),
                    audio_config.handle.inner(),
                ),
                "SpeechSynthesizer.from_auto_detect_source_lang_config error",
            )?;
            SpeechSynthesizer::from_handle(handle.assume_init())
        }
    }

    /// Executes the speech synthesis on plain text, asynchronously.
    pub async fn speak_text_async(&self, text: &str) -> Result<SpeechSynthesisResult> {
        unsafe {
            let c_text = CString::new(text)?;
            let text_len = c_text.as_bytes().len();
            let mut result_handle = MaybeUninit::uninit();
            let ret = synthesizer_speak_text(
                self.handle.inner(),
                c_text.as_ptr(),
                text_len as u32,
                result_handle.as_mut_ptr(),
            );
            convert_err(ret, "SpeechSynthesizer.speak_text_async error")?;
            SpeechSynthesisResult::from_handle(result_handle.assume_init())
        }
    }

    /// Executes the speech synthesis on SSML, asynchronously.
    pub async fn speak_ssml_async(&self, ssml: &str) -> Result<SpeechSynthesisResult> {
        unsafe {
            let c_ssml = CString::new(ssml)?;
            let ssml_len = c_ssml.as_bytes().len();
            let mut result_handle = MaybeUninit::uninit();
            let ret = synthesizer_speak_ssml(
                self.handle.inner(),
                c_ssml.as_ptr(),
                ssml_len as u32,
                result_handle.as_mut_ptr(),
            );
            convert_err(ret, "SpeechSynthesizer.speak_ssml_async error")?;
            SpeechSynthesisResult::from_handle(result_handle.assume_init())
        }
    }

    /// Starts the speech synthesis on plain text, asynchronously.
    /// It returns when the synthesis request is started to process
    /// (the result reason is SynthesizingAudioStarted).
    pub async fn start_speaking_text_async(&self, text: &str) -> Result<SpeechSynthesisResult> {
        unsafe {
            let c_text = CString::new(text)?;
            let text_len = c_text.as_bytes().len();
            let mut result_handle = MaybeUninit::uninit();
            let ret = synthesizer_start_speaking_text(
                self.handle.inner(),
                c_text.as_ptr(),
                text_len as u32,
                result_handle.as_mut_ptr(),
            );
            convert_err(ret, "SpeechSynthesizer.start_speaking_text_async error")?;
            SpeechSynthesisResult::from_handle(result_handle.assume_init())
        }
    }

    /// Starts the speech synthesis on SSML, asynchronously.
    /// It returns when the synthesis request is started to process
    ///(the result reason is SynthesizingAudioStarted).
    pub async fn start_speaking_ssml_async(&self, ssml: &str) -> Result<SpeechSynthesisResult> {
        unsafe {
            let c_ssml = CString::new(ssml)?;
            let ssml_len = c_ssml.as_bytes().len();
            let mut result_handle = MaybeUninit::uninit();
            let ret = synthesizer_start_speaking_ssml(
                self.handle.inner(),
                c_ssml.as_ptr(),
                ssml_len as u32,
                result_handle.as_mut_ptr(),
            );
            convert_err(ret, "SpeechSynthesizer.start_speaking_ssml_async error")?;
            SpeechSynthesisResult::from_handle(result_handle.assume_init())
        }
    }

    /// Stops the speech synthesis, asynchronously.
    /// It stops audio speech synthesis and discards any unread data in audio.PullAudioOutputStream.
    pub async fn stop_speaking_async(&self) -> Result<()> {
        unsafe {
            let ret = synthesizer_stop_speaking(self.handle.inner());
            convert_err(ret, "SpeechSynthesizer.stop_speaking_async error")?;
            Ok(())
        }
    }

    /// Gets the available voices, asynchronously.
    /// The parameter locale specifies the locale of voices, in BCP-47 format; or leave it empty to get all available voices.
    pub async fn get_voices_async(&self, locale: &str) -> Result<SynthesisVoicesResult> {
        unsafe {
            let c_locale_str = CString::new(locale)?;
            let mut v_result = MaybeUninit::uninit();
            let ret = synthesizer_get_voices_list(
                self.handle.inner(),
                c_locale_str.as_ptr(),
                v_result.as_mut_ptr(),
            );
            convert_err(ret, "SpeechSynthesizer.get_voices_async error")?;
            SynthesisVoicesResult::from_handle(v_result.assume_init())
        }
    }

    pub fn get_auth_token(&self) -> Result<String> {
        self.properties
            .get_property(PropertyId::SpeechServiceAuthorizationToken, "")
    }

    /// Sets the authorization token that will be used for connecting to the service.
    /// Note: The caller needs to ensure that the authorization token is valid. Before the authorization token
    /// expires, the caller needs to refresh it by calling this setter with a new valid token.
    /// Otherwise, the synthesizer will encounter errors during synthesizing.
    pub fn set_auth_token(&mut self, token: &str) -> Result<()> {
        self.properties
            .set_property(PropertyId::SpeechServiceAuthorizationToken, token)
    }

    pub fn set_synthesizer_started_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechSynthesisEvent) + 'static + Send,
    {
        self.synthesizer_started_cb = Some(Box::new(f));
        unsafe {
            let ret = synthesizer_started_set_callback(
                self.handle.inner(),
                Some(Self::cb_synthesizer_started),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechSynthesizer.set_synthesizer_started_cb error")?;
            Ok(())
        }
    }

    pub fn set_synthesizer_synthesizing_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechSynthesisEvent) + 'static + Send,
    {
        self.synthesizer_synthesizing_cb = Some(Box::new(f));
        unsafe {
            let ret = synthesizer_synthesizing_set_callback(
                self.handle.inner(),
                Some(Self::cb_synthesizer_synthesizing),
                self as *const _ as *mut c_void,
            );
            convert_err(
                ret,
                "SpeechSynthesizer.set_synthesizer_synthesizing_cb error",
            )?;
            Ok(())
        }
    }

    pub fn set_synthesizer_completed_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechSynthesisEvent) + 'static + Send,
    {
        self.synthesizer_completed_cb = Some(Box::new(f));
        unsafe {
            let ret = synthesizer_completed_set_callback(
                self.handle.inner(),
                Some(Self::cb_synthesizer_completed),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechSynthesizer.set_synthesizer_completed_cb error")?;
            Ok(())
        }
    }

    pub fn set_synthesizer_canceled_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechSynthesisEvent) + 'static + Send,
    {
        self.synthesizer_canceled_cb = Some(Box::new(f));
        unsafe {
            let ret = synthesizer_canceled_set_callback(
                self.handle.inner(),
                Some(Self::cb_synthesizer_canceled),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechSynthesizer.set_synthesizer_canceled_cb error")?;
            Ok(())
        }
    }

    pub fn set_synthesizer_word_boundary_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechSynthesisWordBoundaryEvent) + 'static + Send,
    {
        self.synthesizer_word_boundary_cb = Some(Box::new(f));
        unsafe {
            let ret = synthesizer_word_boundary_set_callback(
                self.handle.inner(),
                Some(Self::cb_synthesizer_word_boundary),
                self as *const _ as *mut c_void,
            );
            convert_err(
                ret,
                "SpeechSynthesizer.set_synthesizer_word_boundary_cb error",
            )?;
            Ok(())
        }
    }

    pub fn set_synthesizer_viseme_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechSynthesisVisemeEvent) + 'static + Send,
    {
        self.synthesizer_viseme_cb = Some(Box::new(f));
        unsafe {
            let ret = synthesizer_viseme_received_set_callback(
                self.handle.inner(),
                Some(Self::cb_synthesizer_viseme),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechSynthesizer.set_synthesizer_viseme_cb error")?;
            Ok(())
        }
    }

    pub fn set_synthesizer_bookmark_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechSynthesisBookmarkEvent) + 'static + Send,
    {
        self.synthesizer_bookmark_cb = Some(Box::new(f));
        unsafe {
            let ret = synthesizer_bookmark_reached_set_callback(
                self.handle.inner(),
                Some(Self::cb_synthesizer_bookmark),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechSynthesizer.set_synthesizer_bookmark_cb error")?;
            Ok(())
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_synthesizer_started(
        hsynth: SPXSYNTHHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechSynthesizer::cb_synthesizer_started called");
        let speech_synthesizer = &mut *(pvContext as *mut SpeechSynthesizer);
        if let Some(cb) = &speech_synthesizer.synthesizer_started_cb {
            trace!("synthesizer_started_cb defined");
            match SpeechSynthesisEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!(
                        "SpeechSynthesisEvent::cb_synthesizer_started error {:?}",
                        err
                    );
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_synthesizer_synthesizing(
        hsynth: SPXSYNTHHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechSynthesizer::cb_synthesizer_synthesizing called");
        let speech_synthesizer = &mut *(pvContext as *mut SpeechSynthesizer);
        if let Some(cb) = &speech_synthesizer.synthesizer_synthesizing_cb {
            trace!("synthesizer_synthesizing_cb defined");
            match SpeechSynthesisEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!(
                        "SpeechSynthesisEvent::cb_synthesizer_synthesizing error {:?}",
                        err
                    );
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_synthesizer_completed(
        hsynth: SPXSYNTHHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechSynthesizer::cb_synthesizer_completed called");
        let speech_synthesizer = &mut *(pvContext as *mut SpeechSynthesizer);
        if let Some(cb) = &speech_synthesizer.synthesizer_completed_cb {
            trace!("synthesizer_completed_cb defined");
            match SpeechSynthesisEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!(
                        "SpeechSynthesisEvent::cb_synthesizer_completed error {:?}",
                        err
                    );
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_synthesizer_canceled(
        hsynth: SPXSYNTHHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechSynthesizer::cb_synthesizer_canceled called");
        let speech_synthesizer = &mut *(pvContext as *mut SpeechSynthesizer);
        if let Some(cb) = &speech_synthesizer.synthesizer_canceled_cb {
            trace!("synthesizer_canceled_cb defined");
            match SpeechSynthesisEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!(
                        "SpeechSynthesisEvent::cb_synthesizer_canceled error {:?}",
                        err
                    );
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_synthesizer_word_boundary(
        hsynth: SPXSYNTHHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechSynthesizer::cb_synthesizer_word_boundary called");
        let speech_synthesizer = &mut *(pvContext as *mut SpeechSynthesizer);
        if let Some(cb) = &speech_synthesizer.synthesizer_word_boundary_cb {
            trace!("synthesizer_word_boundary_cb defined");
            match SpeechSynthesisWordBoundaryEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!(
                        "SpeechSynthesisEvent::cb_synthesizer_word_boundary error {:?}",
                        err
                    );
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_synthesizer_viseme(
        hsynth: SPXSYNTHHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechSynthesizer::cb_synthesizer_viseme called");
        let speech_synthesizer = &mut *(pvContext as *mut SpeechSynthesizer);
        if let Some(cb) = &speech_synthesizer.synthesizer_viseme_cb {
            trace!("synthesizer_viseme_cb defined");
            match SpeechSynthesisVisemeEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!(
                        "SpeechSynthesisEvent::cb_synthesizer_viseme error {:?}",
                        err
                    );
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_synthesizer_bookmark(
        hsynth: SPXSYNTHHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("SpeechSynthesizer::cb_synthesizer_bookmark called");
        let speech_synthesizer = &mut *(pvContext as *mut SpeechSynthesizer);
        if let Some(cb) = &speech_synthesizer.synthesizer_bookmark_cb {
            trace!("synthesizer_bookmark_cb defined");
            match SpeechSynthesisBookmarkEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!(
                        "SpeechSynthesisEvent::cb_synthesizer_bookmark error {:?}",
                        err
                    );
                }
            }
        }
    }
}
