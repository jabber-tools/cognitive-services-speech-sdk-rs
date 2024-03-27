use crate::audio::AudioConfig;
use crate::common::{PropertyCollection, PropertyId};
use crate::dialog::{ActivityReceivedEvent, DialogServiceConfig};
use crate::error::{convert_err, Result};
use crate::ffi::{
    dialog_service_connector_activity_received_set_callback,
    dialog_service_connector_canceled_set_callback, dialog_service_connector_connect,
    dialog_service_connector_create_dialog_service_connector_from_config,
    dialog_service_connector_disconnect, dialog_service_connector_get_property_bag,
    dialog_service_connector_handle_release, dialog_service_connector_listen_once,
    dialog_service_connector_recognized_set_callback,
    dialog_service_connector_recognizing_set_callback, dialog_service_connector_send_activity,
    dialog_service_connector_session_started_set_callback,
    dialog_service_connector_session_stopped_set_callback,
    dialog_service_connector_start_keyword_recognition,
    dialog_service_connector_stop_keyword_recognition, SmartHandle, SPXEVENTHANDLE, SPXHANDLE,
    SPXRECOHANDLE,
};
use crate::speech::{
    KeywordRecognitionModel, SessionEvent, SpeechRecognitionCanceledEvent, SpeechRecognitionEvent,
    SpeechRecognitionResult,
};
use log::*;
use std::ffi::{CStr, CString};
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_void};

#[derive(Debug)]
pub struct SendActivityOutcome {
    pub interaction_id: String,
}

/// DialogServiceConnector connects to a speech enabled dialog backend.
pub struct DialogServiceConnector {
    pub properties: PropertyCollection,
    pub handle: SmartHandle<SPXRECOHANDLE>,
    session_started_cb: Option<Box<dyn Fn(SessionEvent) + Send>>,
    session_stopped_cb: Option<Box<dyn Fn(SessionEvent) + Send>>,
    canceled_cb: Option<Box<dyn Fn(SpeechRecognitionCanceledEvent) + Send>>,
    recognizing_cb: Option<Box<dyn Fn(SpeechRecognitionEvent) + Send>>,
    recognized_cb: Option<Box<dyn Fn(SpeechRecognitionEvent) + Send>>,
    activity_received_cb: Option<Box<dyn Fn(ActivityReceivedEvent) + Send>>,
}

impl fmt::Debug for DialogServiceConnector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DialogServiceConnector")
            .field("handle", &self.handle)
            .field("properties", &self.properties)
            .finish()
    }
}

impl DialogServiceConnector {
    fn from_handle(handle: SPXRECOHANDLE) -> Result<Self> {
        unsafe {
            let mut prop_bag_handle = MaybeUninit::uninit();
            let ret =
                dialog_service_connector_get_property_bag(handle, prop_bag_handle.as_mut_ptr());
            convert_err(ret, "DialogServiceConnector::from_handle error")?;
            Ok(DialogServiceConnector {
                properties: PropertyCollection::from_handle(prop_bag_handle.assume_init()),
                handle: SmartHandle::create(
                    "DialogServiceConnector",
                    handle,
                    dialog_service_connector_handle_release,
                ),
                session_started_cb: None,
                session_stopped_cb: None,
                canceled_cb: None,
                recognizing_cb: None,
                recognized_cb: None,
                activity_received_cb: None,
            })
        }
    }

    /// NewDialogServiceConnectorFromConfig creates a dialog service connector from a dialog service config and an audio config.
    /// Users should use this function to create a dialog service connector.
    // using static dispatch, see https://joshleeb.com/posts/rust-traits-and-trait-objects/
    pub fn from_config(
        dialog_service_config: impl DialogServiceConfig,
        audio_config: Option<AudioConfig>,
    ) -> Result<Self> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let speech_config_handle = dialog_service_config.get_handle();
            let ret;
            if let Some(audio_cfg) = audio_config {
                ret = dialog_service_connector_create_dialog_service_connector_from_config(
                    handle.as_mut_ptr(),
                    speech_config_handle,
                    audio_cfg.handle.inner(),
                );
            } else {
                let spxhandle_null: SPXHANDLE = 0 as SPXHANDLE;
                ret = dialog_service_connector_create_dialog_service_connector_from_config(
                    handle.as_mut_ptr(),
                    speech_config_handle,
                    spxhandle_null,
                );
            }
            convert_err(ret, "DialogServiceConnector::from_config error")?;
            DialogServiceConnector::from_handle(handle.assume_init())
        }
    }

    pub async fn connect_async(&self) -> Result<()> {
        unsafe {
            let ret = dialog_service_connector_connect(self.handle.inner());
            convert_err(ret, "DialogServiceConnector.connect_async error")?;
            Ok(())
        }
    }

    pub async fn disconnect_async(&self) -> Result<()> {
        unsafe {
            let ret = dialog_service_connector_disconnect(self.handle.inner());
            convert_err(ret, "DialogServiceConnector.disconnect_async error")?;
            Ok(())
        }
    }

    /// Sends an activity to the backing dialog.
    pub async fn send_activity_async(&self, message: String) -> Result<SendActivityOutcome> {
        unsafe {
            let c_buf: *mut c_char = &mut [0u8; 37] as *const _ as *mut c_char;
            let c_message = CString::new(message)?;
            let ret = dialog_service_connector_send_activity(
                self.handle.inner(),
                c_message.as_ptr(),
                c_buf,
            );
            convert_err(ret, "DialogServiceConnector.send_activity_async error")?;
            let interaction_id = CStr::from_ptr(c_buf).to_str()?.to_owned();
            Ok(SendActivityOutcome { interaction_id })
        }
    }

    /// ListenOnceAsync starts a listening session that will terminate after the first utterance.
    pub async fn listen_once_async(&self) -> Result<SpeechRecognitionResult> {
        unsafe {
            let mut result_handle = MaybeUninit::uninit();
            let ret = dialog_service_connector_listen_once(
                self.handle.inner(),
                result_handle.as_mut_ptr(),
            );
            convert_err(ret, "DialogServiceConnector.listen_once_async error")?;
            SpeechRecognitionResult::from_handle(result_handle.assume_init())
        }
    }

    /// StartKeywordRecognitionAsync initiates keyword recognition.
    pub async fn start_keyword_recognition_async(
        &self,
        model: &KeywordRecognitionModel,
    ) -> Result<()> {
        unsafe {
            let ret = dialog_service_connector_start_keyword_recognition(
                self.handle.inner(),
                model.handle.inner(),
            );
            convert_err(
                ret,
                "DialogServiceConnector.start_keyword_recognition_async error",
            )?;
            Ok(())
        }
    }

    /// StopKeywordRecognitionAsync stops keyword recognition.
    pub async fn stop_keyword_recognition_async(&self) -> Result<()> {
        unsafe {
            let ret = dialog_service_connector_stop_keyword_recognition(self.handle.inner());
            convert_err(
                ret,
                "DialogServiceConnector.stop_keyword_recognition_async error",
            )?;
            Ok(())
        }
    }

    pub fn get_auth_token(&self) -> Result<String> {
        self.properties
            .get_property(PropertyId::SpeechServiceAuthorizationToken, "")
    }

    /// Sets the authorization token that will be used for connecting to the service.
    /// Note: The caller needs to ensure that the authorization token is valid. Before the authorization token
    /// expires, the caller needs to refresh it by calling this setter with a new valid token.
    /// Otherwise, the connector will encounter errors during its operation.
    pub fn set_auth_token(&mut self, auth_token: String) -> Result<()> {
        self.properties
            .set_property(PropertyId::SpeechServiceAuthorizationToken, auth_token)
    }

    pub fn get_speech_activity_template(&self) -> Result<String> {
        self.properties
            .get_property(PropertyId::ConversationSpeechActivityTemplate, "")
    }

    /// Sets the speech activity template. It is used to stamp properties from the template on the service generated
    /// activty for speech.
    pub fn set_speech_activity_template(&mut self, speech_activity_template: String) -> Result<()> {
        self.properties.set_property(
            PropertyId::ConversationSpeechActivityTemplate,
            speech_activity_template,
        )
    }

    pub fn set_session_started_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SessionEvent) + 'static + Send,
    {
        self.session_started_cb = Some(Box::new(f));
        unsafe {
            let ret = dialog_service_connector_session_started_set_callback(
                self.handle.inner(),
                Some(Self::cb_session_started),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "DialogServiceConnector.set_session_started_cb error")?;
            Ok(())
        }
    }

    pub fn set_session_stopped_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SessionEvent) + 'static + Send,
    {
        self.session_stopped_cb = Some(Box::new(f));
        unsafe {
            let ret = dialog_service_connector_session_stopped_set_callback(
                self.handle.inner(),
                Some(Self::cb_session_stopped),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "DialogServiceConnector.set_session_stopped_cb error")?;
            Ok(())
        }
    }

    pub fn set_canceled_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechRecognitionCanceledEvent) + 'static + Send,
    {
        self.canceled_cb = Some(Box::new(f));
        unsafe {
            let ret = dialog_service_connector_canceled_set_callback(
                self.handle.inner(),
                Some(Self::cb_canceled),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "DialogServiceConnector.set_canceled_cb error")?;
            Ok(())
        }
    }

    pub fn set_recognizing_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechRecognitionEvent) + 'static + Send,
    {
        self.recognizing_cb = Some(Box::new(f));
        unsafe {
            let ret = dialog_service_connector_recognizing_set_callback(
                self.handle.inner(),
                Some(Self::cb_recognizing),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "DialogServiceConnector.set_recognizing_cb error")?;
            Ok(())
        }
    }

    pub fn set_recognized_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechRecognitionEvent) + 'static + Send,
    {
        self.recognized_cb = Some(Box::new(f));
        unsafe {
            let ret = dialog_service_connector_recognized_set_callback(
                self.handle.inner(),
                Some(Self::cb_recognized),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "DialogServiceConnector.set_recognized_cb error")?;
            Ok(())
        }
    }

    pub fn set_activity_received_cb<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(SpeechRecognitionEvent) + 'static + Send,
    {
        self.recognized_cb = Some(Box::new(f));
        unsafe {
            let ret = dialog_service_connector_activity_received_set_callback(
                self.handle.inner(),
                Some(Self::cb_activity_received),
                self as *const _ as *mut c_void,
            );
            convert_err(ret, "DialogServiceConnector.set_activity_received_cb error")?;
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
        trace!("DialogServiceConnector::cb_session_started called");
        let dialog_service_connector = &mut *(pvContext as *mut DialogServiceConnector);
        trace!("dialog_service_connector {:?}", dialog_service_connector);
        if let Some(cb) = &dialog_service_connector.session_started_cb {
            trace!("session_started_cb defined");
            match SessionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("DialogServiceConnector::cb_session_started error {:?}", err);
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
        trace!("DialogServiceConnector::cb_session_stopped called");
        let dialog_service_connector = &mut *(pvContext as *mut DialogServiceConnector);
        if let Some(cb) = &dialog_service_connector.session_stopped_cb {
            trace!("cb_session_stopped defined");
            match SessionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("DialogServiceConnector::cb_session_stopped error {:?}", err);
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
        trace!("DialogServiceConnector::cb_canceled called");
        let dialog_service_connector = &mut *(pvContext as *mut DialogServiceConnector);
        if let Some(cb) = &dialog_service_connector.canceled_cb {
            trace!("canceled_cb defined");
            match SpeechRecognitionCanceledEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("DialogServiceConnector::cb_canceled error {:?}", err);
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
        trace!("DialogServiceConnector::cb_recognizing called");
        let dialog_service_connector = &mut *(pvContext as *mut DialogServiceConnector);
        if let Some(cb) = &dialog_service_connector.recognizing_cb {
            trace!("recognizing_cb defined");
            match SpeechRecognitionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("DialogServiceConnector::cb_recognizing error {:?}", err);
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
        trace!("DialogServiceConnector::cb_recognized called");
        let dialog_service_connector = &mut *(pvContext as *mut DialogServiceConnector);
        if let Some(cb) = &dialog_service_connector.recognized_cb {
            trace!("recognized_cb defined");
            match SpeechRecognitionEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!("DialogServiceConnector::cb_recognized error {:?}", err);
                }
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    unsafe extern "C" fn cb_activity_received(
        hreco: SPXRECOHANDLE,
        hevent: SPXEVENTHANDLE,
        pvContext: *mut c_void,
    ) {
        trace!("DialogServiceConnector::cb_activity_received called");
        let dialog_service_connector = &mut *(pvContext as *mut DialogServiceConnector);
        if let Some(cb) = &dialog_service_connector.activity_received_cb {
            trace!("cb_activity_received defined");
            match ActivityReceivedEvent::from_handle(hevent) {
                Ok(event) => {
                    trace!("calling cb with event {:?}", event);
                    cb(event);
                }
                Err(err) => {
                    error!(
                        "DialogServiceConnector::cb_activity_received error {:?}",
                        err
                    );
                }
            }
        }
    }
}
