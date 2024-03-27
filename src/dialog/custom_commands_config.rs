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

/// CustomCommandsConfig defines configurations for the dialog service connector object for using a CustomCommands backend.
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
    /// Creates a Custom Commands config instance with the specified application id,
    /// subscription key and region.
    pub fn from_subscription<S>(
        application_id: S,
        subscription_key: S,
        region: S,
    ) -> Result<CustomCommandsConfig>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let c_sub_key = CString::new(subscription_key)?;
            let c_region = CString::new(region)?;
            let c_application_id = CString::new(application_id)?;
            let ret = custom_commands_config_from_subscription(
                handle.as_mut_ptr(),
                c_application_id.as_ptr(),
                c_sub_key.as_ptr(),
                c_region.as_ptr(),
            );
            convert_err(ret, "CustomCommandsConfig::from_subscription error")?;
            Ok(CustomCommandsConfig {
                config: SpeechConfig::from_handle(handle.assume_init())?,
            })
        }
    }

    /// Creates a Custom Commands config instance with the specified application id
    /// authorization token and region.
    /// Note: The caller needs to ensure that the authorization token is valid. Before the authorization token
    /// expires, the caller needs to refresh it by calling this setter with a new valid token.
    /// As configuration values are copied when creating a new connector, the new token value will not apply to connectors that have
    /// already been created.
    /// For connectors that have been created before, you need to set authorization token of the corresponding connector
    /// to refresh the token. Otherwise, the connectors will encounter errors during operation.
    pub fn from_auth_token<S>(
        application_id: S,
        authorization_token: S,
        region: S,
    ) -> Result<CustomCommandsConfig>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let c_authorization_token = CString::new(authorization_token)?;
            let c_region = CString::new(region)?;
            let c_application_id = CString::new(application_id)?;
            let ret = custom_commands_config_from_authorization_token(
                handle.as_mut_ptr(),
                c_application_id.as_ptr(),
                c_authorization_token.as_ptr(),
                c_region.as_ptr(),
            );
            convert_err(ret, "CustomCommandsConfig::from_auth_token error")?;
            Ok(CustomCommandsConfig {
                config: SpeechConfig::from_handle(handle.assume_init())?,
            })
        }
    }

    /// Gets the corresponding backend application identifier.
    pub fn get_application_id(&self) -> Result<String> {
        self.config
            .get_property(PropertyId::ConversationApplicationID)
    }

    /// Sets the corresponding backend application identifier.
    pub fn set_application_id(&mut self, app_id: String) -> Result<()> {
        self.config
            .set_property(PropertyId::ConversationApplicationID, app_id)
    }
}
