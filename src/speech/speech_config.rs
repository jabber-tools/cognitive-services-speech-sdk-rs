use crate::common::PropertyCollection;
use crate::common::PropertyId;
use crate::error::{convert_err, Result};
use crate::ffi::{
    speech_config_from_subscription, speech_config_get_property_bag, speech_config_release,
    SmartHandle, SPXHANDLE, SPXPROPERTYBAGHANDLE, SPXSPEECHCONFIGHANDLE,
};
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct SpeechConfig {
    pub handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    properties: PropertyCollection,
}

impl SpeechConfig {
    fn from_handle(handle: SPXHANDLE) -> Result<SpeechConfig> {
        unsafe {
            let mut prop_bag_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();
            let ret = speech_config_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "SpeechConfig::from_handle error")?;

            let mut property_bag = PropertyCollection::from_handle(prop_bag_handle);

            property_bag
                .set_property_by_string("SPEECHSDK-SPEECH-CONFIG-SYSTEM-LANGUAGE", "Rust")?;

            let result = SpeechConfig {
                handle: SmartHandle::create("SpeechConfig", handle, speech_config_release),
                properties: property_bag,
            };
            Ok(result)
        }
    }

    pub fn from_subscription<S>(subscription: S, region: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_sub = CString::new(subscription)?;
        let c_region = CString::new(region)?;

        unsafe {
            let mut handle: SPXSPEECHCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let ret =
                speech_config_from_subscription(&mut handle, c_sub.as_ptr(), c_region.as_ptr());
            convert_err(ret, "SpeechConfig::from_subscription error")?;
            SpeechConfig::from_handle(handle)
        }
    }

    pub fn set_proxy(&mut self, hostname: String, port: u64) -> Result<()> {
        self.set_property(PropertyId::SpeechServiceConnectionProxyHostName, hostname)?;
        self.set_property(
            PropertyId::SpeechServiceConnectionProxyPort,
            port.to_string(),
        )
    }

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

    pub fn set_property(&mut self, id: PropertyId, value: String) -> Result<()> {
        self.properties.set_property(id, value)
    }
}
