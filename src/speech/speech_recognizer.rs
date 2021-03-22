use crate::audio::AudioConfig;
use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_release, recognizer_async_handle_release, recognizer_canceled_set_callback,
    recognizer_create_speech_recognizer_from_config, recognizer_get_property_bag,
    recognizer_handle_release, recognizer_recognize_once, recognizer_recognized_set_callback,
    recognizer_recognizing_set_callback, recognizer_session_started_set_callback,
    recognizer_session_stopped_set_callback, recognizer_speech_end_detected_set_callback,
    recognizer_speech_start_detected_set_callback, recognizer_start_continuous_recognition_async,
    recognizer_start_continuous_recognition_async_wait_for, SmartHandle, SPXASYNCHANDLE,
    SPXEVENTHANDLE, SPXHANDLE, SPXHANDLE_EMPTY, SPXRECOHANDLE, SPXRESULTHANDLE,
};
use crate::speech::{
    RecognitionEvent, SessionEvent, SpeechConfig, SpeechRecognitionCanceledEvent,
    SpeechRecognitionEvent, SpeechRecognitionResult,
};
use log::*;
use std::boxed::Box;
use std::fmt;
use std::os::raw::c_void;

pub struct SpeechRecognizer {
    handle: SmartHandle<SPXRECOHANDLE>,
    properties: PropertyCollection,
    speech_config: SpeechConfig,
    audio_config: AudioConfig,
    handle_async_start_continuous: Option<SmartHandle<SPXASYNCHANDLE>>,
    _handle_async_stop_continuous: Option<SmartHandle<SPXASYNCHANDLE>>,
    _handle_async_start_keyword: Option<SmartHandle<SPXASYNCHANDLE>>,
    _handle_async_stop_keyword: Option<SmartHandle<SPXASYNCHANDLE>>,
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
            .field("speech_config", &self.speech_config)
            .field("audio_config", &self.audio_config)
            .finish()
    }
}

impl SpeechRecognizer {
    fn from_handle(
        handle: SPXHANDLE,
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        let mut prop_bag_handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = recognizer_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "SpeechRecognizer::from_handle error")?;
        }
        let property_bag = PropertyCollection {
            handle: SmartHandle::create(
                "PropertyCollection",
                prop_bag_handle,
                property_bag_release,
            ),
        };

        let result = SpeechRecognizer {
            handle: SmartHandle::create("SpeechRecognizer", handle, recognizer_handle_release),
            properties: property_bag,
            speech_config,
            audio_config,
            handle_async_start_continuous: None,
            _handle_async_stop_continuous: None,
            _handle_async_start_keyword: None,
            _handle_async_stop_keyword: None,
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

    pub fn from_config(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            convert_err(
                recognizer_create_speech_recognizer_from_config(
                    &mut handle,
                    speech_config.handle.get(),
                    audio_config.handle.get(),
                ),
                "SpeechRecognizer.from_config error",
            )?;
        }
        SpeechRecognizer::from_handle(handle, speech_config, audio_config)
    }

    pub fn set_session_started_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SessionEvent) + 'static + Send,
    {
        self.session_started_cb = Some(Box::new(f));
        unsafe {
            let ret = recognizer_session_started_set_callback(
                self.handle.get(),
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
                self.handle.get(),
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
                self.handle.get(),
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
                self.handle.get(),
                Some(Self::cb_speech_end_detected),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "SpeechRecognizer.set_speech_end_detected_cb error")?;
            Ok(())
        }
    }

    pub fn set_canceled_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechRecognitionCanceledEvent) + 'static + Send,
    {
        self.canceled_cb = Some(Box::new(f));
        unsafe {
            let ret = recognizer_canceled_set_callback(
                self.handle.get(),
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
                self.handle.get(),
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
                self.handle.get(),
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

    pub async fn recognize_once_async(&mut self) -> Result<SpeechRecognitionResult> {
        // TBD:
        // does it make sense to tokio:spawn whole inner implementation?
        // does it make sense to have this func async at all?
        // similar design is used by golang implementation
        // not really sure about fundamental reasons behind it
        unsafe {
            let mut handle_result: SPXRESULTHANDLE = SPXHANDLE_EMPTY;
            let ret = recognizer_recognize_once(self.handle.get(), &mut handle_result);
            convert_err(ret, "SpeechRecognizer.recognize_once_async error")?;
            SpeechRecognitionResult::from_handle(handle_result)
        }
    }

    pub async fn start_continuous_recognition_async(&mut self) -> Result<()> {
        // TBD:
        // does it make sense to tokio:spawn whole inner implementation?
        // does it make sense to have this func async at all?
        // similar design is used by golang implementation
        // not really sure about fundamental reasons behind it
        unsafe {
            let mut handle_async_start_continuous: SPXASYNCHANDLE = SPXHANDLE_EMPTY;
            trace!("calling recognizer_start_continuous_recognition_async");
            let mut ret = recognizer_start_continuous_recognition_async(
                self.handle.get(),
                &mut handle_async_start_continuous,
            );
            convert_err(
                ret,
                "SpeechRecognizer.start_continuous_recognition_async error",
            )?;
            trace!("called recognizer_start_continuous_recognition_async");
            self.handle_async_start_continuous = Some(SmartHandle::create(
                "handle_async_start_continuous",
                handle_async_start_continuous,
                recognizer_async_handle_release,
            ));

            trace!("calling recognizer_start_continuous_recognition_async_wait_for");
            ret = recognizer_start_continuous_recognition_async_wait_for(
                handle_async_start_continuous,
                u32::MAX,
            );
            convert_err(
                ret,
                "SpeechRecognizer.recognizer_start_continuous_recognition_async_wait_for error",
            )?;
            trace!("called recognizer_start_continuous_recognition_async_wait_for");
        }
        Ok(())
    }
}
