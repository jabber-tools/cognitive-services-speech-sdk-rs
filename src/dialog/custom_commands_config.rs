use crate::common::PropertyId;
use crate::dialog::DialogServiceConfig;
use crate::error::{convert_err, Result};
use crate::ffi::{
    custom_commands_config_from_authorization_token, custom_commands_config_from_subscription,
    SPXSPEECHCONFIGHANDLE,
};
use crate::speech::SpeechConfig;
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct CustomCommandsConfig {
    pub config: SpeechConfig,
}

impl DialogServiceConfig for CustomCommandsConfig {
    fn get_speech_config(&mut self) -> &mut SpeechConfig {
        &mut self.config
    }

    fn get_handle(&self) -> SPXSPEECHCONFIGHANDLE {
        self.config.handle.inner()
    }
}

impl CustomCommandsConfig {
    pub fn from_subscription<S>(
        application_id: S,
        subscription_key: S,
        region: S,
    ) -> Result<CustomCommandsConfig>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let mut handle: SPXSPEECHCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_sub_key = CString::new(subscription_key)?;
            let c_region = CString::new(region)?;
            let c_application_id = CString::new(application_id)?;
            let ret = custom_commands_config_from_subscription(
                &mut handle,
                c_application_id.as_ptr(),
                c_sub_key.as_ptr(),
                c_region.as_ptr(),
            );
            convert_err(ret, "CustomCommandsConfig::from_subscription error")?;
            Ok(CustomCommandsConfig {
                config: SpeechConfig::from_handle(handle)?,
            })
        }
    }

    pub fn from_auth_token<S>(
        application_id: S,
        authorization_token: S,
        region: S,
    ) -> Result<CustomCommandsConfig>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let mut handle: SPXSPEECHCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_authorization_token = CString::new(authorization_token)?;
            let c_region = CString::new(region)?;
            let c_application_id = CString::new(application_id)?;
            let ret = custom_commands_config_from_authorization_token(
                &mut handle,
                c_application_id.as_ptr(),
                c_authorization_token.as_ptr(),
                c_region.as_ptr(),
            );
            convert_err(ret, "CustomCommandsConfig::from_auth_token error")?;
            Ok(CustomCommandsConfig {
                config: SpeechConfig::from_handle(handle)?,
            })
        }
    }

    pub fn get_application_id(&self) -> Result<String> {
        self.config
            .get_property(PropertyId::ConversationApplicationID)
    }

    pub fn set_application_id(&mut self, app_id: String) -> Result<()> {
        self.config
            .set_property(PropertyId::ConversationApplicationID, app_id)
    }
}
